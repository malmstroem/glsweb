use crate::web_config;
use async_trait::async_trait;
use axum::{
    extract::{FromRef, State},
    http::Method,
    response::{IntoResponse, Response},
    routing::get,
    RequestPartsExt, Router,
};
use axum_session::{SessionConfig, SessionLayer, SessionStore};
use axum_session_auth::*;
use axum_session_sqlx::SessionPgPool;
use core::pin::Pin;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{
    collections::HashSet, error::Error, future::Future, net::SocketAddr, str::FromStr, sync::Arc,
};

use crate::ServerState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub anonymous: bool,
    pub username: String,
    pub permissions: HashSet<String>,
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlPermissionTokens {
    pub token: String,
}

impl Default for User {
    fn default() -> Self {
        let mut permissions = HashSet::new();

        permissions.insert("Category::View".to_owned());

        Self {
            id: 1,
            anonymous: true,
            username: "Guest".into(),
            permissions,
        }
    }
}

#[async_trait]
impl Authentication<User, i64, PgPool> for User {
    async fn load_user(userid: i64, pool: Option<&PgPool>) -> Result<User, anyhow::Error> {
        let pool = pool.unwrap();

        User::get_user(userid, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Could not load user"))
    }

    fn is_authenticated(&self) -> bool {
        !self.anonymous
    }

    fn is_active(&self) -> bool {
        !self.anonymous
    }

    fn is_anonymous(&self) -> bool {
        self.anonymous
    }
}

#[async_trait]
impl HasPermission<PgPool> for User {
    async fn has(&self, perm: &str, _pool: &Option<&PgPool>) -> bool {
        self.permissions.contains(perm)
    }
}

impl User {
    pub async fn get_user(id: i64, pool: &PgPool) -> Option<Self> {
        let sqluser = sqlx::query_as::<_, SqlUser>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .ok()?;

        let sql_user_perms = sqlx::query_as::<_, SqlPermissionTokens>(
            "SELECT token FROM users_permissions WHERE user_id = $1;",
        )
        .bind(id)
        .fetch_all(pool)
        .await
        .ok()?;

        Some(sqluser.into_user(Some(sql_user_perms)))
    }
}

#[derive(sqlx::FromRow, Clone)]
pub struct SqlUser {
    pub id: i32,
    pub anonymous: bool,
    pub username: String,
}

impl SqlUser {
    pub fn into_user(self, sql_user_perms: Option<Vec<SqlPermissionTokens>>) -> User {
        User {
            id: self.id,
            anonymous: self.anonymous,
            username: self.username,
            permissions: if let Some(user_perms) = sql_user_perms {
                user_perms
                    .into_iter()
                    .map(|x| x.token)
                    .collect::<HashSet<String>>()
            } else {
                HashSet::<String>::new()
            },
        }
    }
}

pub async fn connect_to_db() -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&web_config().DB_URL)
        .await?;
    Ok(pool)
}

pub struct Session(
    pub AuthSession<crate::auth::User, i64, SessionPgPool, sqlx::PgPool>,
    pub Arc<sqlx::Pool<sqlx::Postgres>>,
);

impl std::ops::Deref for Session {
    type Target = AuthSession<crate::auth::User, i64, SessionPgPool, sqlx::PgPool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Session {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct AuthSessionLayerNotFound;

impl std::fmt::Display for AuthSessionLayerNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AuthSessionLayer was not found")
    }
}

impl std::error::Error for AuthSessionLayerNotFound {}

impl IntoResponse for AuthSessionLayerNotFound {
    fn into_response(self) -> Response {
        (
            http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "AuthSessionLayer was not found",
        )
            .into_response()
    }
}

#[async_trait]
impl<S> axum::extract::FromRequestParts<S> for Session
where
    S: std::marker::Sync + std::marker::Send,
{
    type Rejection = AuthSessionLayerNotFound;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        AuthSession::<User, i64, SessionPgPool, PgPool>::from_request_parts(parts, state)
            .await
            .map(|auth_session| {
                let ss = parts.extensions.get::<ServerState>().unwrap();
                let dbp = ss.0.clone();
                Session(auth_session, dbp)
            })
            .map_err(|_| AuthSessionLayerNotFound)
    }
}
