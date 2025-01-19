use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
};
use http::{request::Parts, StatusCode};
use sqlx::{PgPool, Pool, Postgres};

#[derive(Clone)]
pub struct ServerState(pub std::sync::Arc<sqlx::Pool<sqlx::Postgres>>);

#[async_trait]
impl<S> FromRequestParts<S> for ServerState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
