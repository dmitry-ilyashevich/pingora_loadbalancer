use std::collections::HashMap;

use config::Config;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Settings {
    general: GeneralSettings,
    server: ServerSettings,
    upstreams: HashMap<String, UpstreamSettings>,
}

#[derive(Debug, serde::Deserialize)]
struct GeneralSettings {
    log_level: String,
    health_check_interval_secs: u64,
    cert_file: String,
    key_file: String,
}

#[derive(Debug, serde::Deserialize)]
struct UpstreamSettings {
    address: String,
    port: u16,
}

#[derive(Debug, serde::Deserialize)]
struct ServerSettings {
    address: String,
    port: u16,
    tls_port: u16,
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

    pub(crate) fn get_health_check_interval_secs(&self) -> u64 {
        self.general.health_check_interval_secs
    }

    pub(crate) fn get_upstream_addresses(&self) -> Vec<String> {
        self.upstreams
            .values()
            .map(|upstream| format!("{}:{}", upstream.address, upstream.port))
            .collect()
    }

    pub(crate) fn get_server_addr(&self) -> String {
        format!("{}:{}", &self.server.address, &self.server.port)
    }

    pub(crate) fn get_tls_server_addr(&self) -> String {
        format!("{}:{}", &self.server.address, &self.server.tls_port)
    }

    pub(crate) fn get_cert_file(&self) -> &str {
        &self.general.cert_file
    }

    pub(crate) fn get_key_file(&self) -> &str {
        &self.general.key_file
    }
}
