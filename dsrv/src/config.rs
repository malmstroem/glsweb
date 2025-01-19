use crate::envs::get_env;
use std::sync::OnceLock;

pub fn web_config() -> &'static WebConfig {
    static INSTANCE: OnceLock<WebConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        WebConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct WebConfig {
    pub DB_URL: String,
}

impl WebConfig {
    fn load_from_env() -> crate::envs::Result<WebConfig> {
        Ok(WebConfig {
            DB_URL: get_env("SERVICE_DB_URL")?,
        })
    }
}
