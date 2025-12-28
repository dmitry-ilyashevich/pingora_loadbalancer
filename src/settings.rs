use std::collections::HashMap;

use config::Config;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Settings {
    general: GeneralSettings,
    server: ServerSettings,
    upstreams: HashMap<String, UpstreamSettings>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct GeneralSettings {
    log_level: String,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct UpstreamSettings {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct ServerSettings {
    pub address: String,
    pub port: u16,
    pub tls_port: u16,
}

impl Settings {
    pub(crate) fn new() -> Self {
        Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build()
            .unwrap_or_else(|e| panic!("Failed to load configuration: {}", e))
            .try_deserialize()
            .unwrap_or_else(|e| panic!("Failed to deserialize configuration: {}", e))
    }

    pub(crate) fn get_log_level(&self) -> &str {
        &self.general.log_level
    }

    pub(crate) fn get_server(&self) -> &ServerSettings {
        &self.server
    }

    pub(crate) fn get_upstreams(&self) -> &HashMap<String, UpstreamSettings> {
        &self.upstreams
    }
}
