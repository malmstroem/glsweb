use dioxus::dioxus_core::Element;

pub fn server_start(app_fn: fn() -> Element) {
    use crate::{auth::*, ServerState};
    use axum::{routing::*, Extension};
    use axum_session::{SessionAnyPool, SessionConfig, SessionStore};
    use axum_session_auth::{AuthConfig, AuthSessionLayer};
    use axum_session_sqlx::{SessionPgPool, SessionPgSessionStore};
    use dioxus::prelude::*;
    use log::{debug, error};
    use sqlx::PgPool;
    use std::sync::Arc;

    dioxus::logger::initialize_default();

    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            debug!("Connecting to the database ...");
            let pg_pool = connect_to_db().await;
            if pg_pool.is_err() {
                error!("Failed to connect to the database! Exiting now.");
                return;
            }
            let pg_pool = pg_pool.unwrap();
            debug!("Connected to the database.");
            debug!("pg_pool={:?}", &pg_pool);

            let session_config = SessionConfig::default().with_table_name("users_sessions");
            let auth_config = AuthConfig::<i64>::default().with_anonymous_user_id(Some(1));
            let session_store =
                SessionStore::<SessionPgPool>::new(Some(pg_pool.clone().into()), session_config)
                    .await
                    .unwrap();

            let state = ServerState(Arc::new(pg_pool.clone()));

            let web_api_router = Router::new()
                .serve_dioxus_application(ServeConfig::builder().build().unwrap(), app_fn)
                .layer(
                    AuthSessionLayer::<User, i64, SessionPgPool, PgPool>::new(Some(
                        pg_pool.clone(),
                    ))
                    .with_config(auth_config),
                )
                .layer(axum_session::SessionLayer::new(session_store))
                .layer(Extension(state));

            let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

            axum::serve(listener, web_api_router.into_make_service())
                .await
                .unwrap();
        });
}
