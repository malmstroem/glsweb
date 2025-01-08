use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(feature = "server")]
use crate::auth::{Session, User};
#[cfg(feature = "server")]
use axum::http::Method;
#[cfg(feature = "server")]
use axum_session_auth::{Auth, Rights};
#[cfg(feature = "server")]
use sqlx::PgPool;

#[cfg(feature = "server")]
#[derive(sqlx::FromRow, Debug)]
pub struct DbUser {
    id: i32,
    username: String,
    password: String,
}

#[server(Login)]
pub async fn login(username: String, password: String) -> Result<String, ServerFnError> {
    let session: Session = extract().await?;
    let auth: Session = extract().await?;
    let dbc = session.1;
    let user: DbUser = match sqlx::query_as::<_, DbUser>(
        "SELECT id,username,password FROM users where username = $1",
    )
    .bind(username)
    .fetch_one(dbc.as_ref())
    .await
    {
        Ok(user) => user,
        Err(err) => return Ok("user not found".into()), // FIXME: replace with error
    };
    info!("{user:?}");
    if user.password != password {
        return Ok("incorrect password".into()); // FIXME: replace with error
    }
    info!("Match");
    auth.login_user(user.id as i64);
    Ok("login successful".into())
}

#[server(Logout)]
pub async fn logout() -> Result<(), ServerFnError> {
    let auth: Session = extract().await?;
    auth.logout_user();
    Ok(())
}

#[server(GetUserName)]
pub async fn get_user_name() -> Result<String, ServerFnError> {
    let session: Session = extract().await?;
    Ok(session.0.current_user.unwrap().username.to_string())
}

#[server(Permissions)]
pub async fn get_permissions() -> Result<String, ServerFnError> {
    let method: Method = extract().await?;
    let auth: Session = extract().await?;
    let curr_user = auth.current_user.clone().unwrap_or_default();

    if !Auth::<User, i64, PgPool>::build([Method::POST], false)
        .requires(Rights::any([
            Rights::permission("Category::View"),
            Rights::permission("Admin::View"),
        ]))
        .validate(&curr_user, &method, None)
        .await
    {
        return Ok(format!(
            "User '{}' has no permissions. Please login.",
            curr_user.username
        ));
    }

    Ok(format!(
        "User: '{}'; permissions: {:?}",
        curr_user.username, curr_user.permissions
    ))
}
