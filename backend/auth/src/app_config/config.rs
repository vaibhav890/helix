use serde::Deserialize;

use super::ServerConfig;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn load() -> Self {
        Self::load_environment();

        let server = envy::prefixed("SERVER_")
            .from_env::<ServerConfig>()
            .expect("server config missing");

        tracing::info!("config loaded successfully");

        Self { server }
    }

    fn load_environment() {
        match dotenvy::dotenv() {
            Ok(path) => tracing::info!("loaded .env from {:?}", path),
            Err(error) => {
                tracing::warn!(".env not found: {}", error);

                if cfg!(not(debug_assertions)) {
                    panic!(".env is required in production");
                }
            }
        }
    }
}
