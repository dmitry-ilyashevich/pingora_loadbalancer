mod settings;

use settings::Settings;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::prelude::*;

use async_trait::async_trait;
use pingora_core::services::background::background_service;
use std::{sync::Arc, time::Duration};

use pingora_core::server::configuration::Opt;
use pingora_core::server::Server;
use pingora_core::upstreams::peer::HttpPeer;
use pingora_core::Result;
use pingora_load_balancing::{health_check, selection::RoundRobin, LoadBalancer};
use pingora_proxy::{ProxyHttp, Session};

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self
            .0
            .select(b"", 256) // hash doesn't matter
            .unwrap();

        info!("upstream peer is: {:?}", upstream);

        let peer = Box::new(HttpPeer::new(upstream, true, "lvh.me".to_string()));
        Ok(peer)
    }

    async fn upstream_request_filter(
        &self,
        _session: &mut Session,
        upstream_request: &mut pingora_http::RequestHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()> {
        upstream_request
            .insert_header("Host", "lvh.me")
            .unwrap();
        Ok(())
    }
}

fn main() {
    let settings = Settings::new();

    let fmt_layer = fmt::layer()
        .with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(settings.get_log_level()))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    let opt = Opt::parse_args();
    let mut my_server = Server::new(opt).unwrap();
    my_server.bootstrap();

    let mut upstreams = LoadBalancer::try_from_iter(settings.get_upstream_addresses()).unwrap();

    // We add health check in the background so that the bad server is never selected.
    let hc = health_check::TcpHealthCheck::new();
    upstreams.set_health_check(hc);
    upstreams.health_check_frequency = Some(Duration::from_secs(settings.get_health_check_interval_secs()));

    let background = background_service("health check", upstreams);

    let upstreams = background.task();

    let mut lb = pingora_proxy::http_proxy_service(&my_server.configuration, LB(upstreams));

    lb.add_tcp(&settings.get_server_addr());

    let cert_file = settings.get_cert_file();
    let key_file = settings.get_key_file();

    let cert_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), cert_file);
    let key_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), key_file);

    let mut tls_settings =
        pingora_core::listeners::tls::TlsSettings::intermediate(&cert_path, &key_path).unwrap();
    tls_settings.enable_h2();
    lb.add_tls_with_settings(&settings.get_tls_server_addr(), None, tls_settings);

    my_server.add_service(lb);
    my_server.add_service(background);
    my_server.run_forever();
}
