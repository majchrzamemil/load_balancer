use async_trait::async_trait;
use pingora::{lb::health_check::HttpHealthCheck, prelude::*};
use std::sync::Arc;

pub struct LB(Arc<LoadBalancer<RoundRobin>>);

#[async_trait]
impl ProxyHttp for LB {
    /// For this small example, we don't need context storage
    type CTX = ();
    fn new_ctx(&self) -> () {
        ()
    }

    async fn upstream_peer(&self, _session: &mut Session, _ctx: &mut ()) -> Result<Box<HttpPeer>> {
        let upstream = self
            .0
            .select(b"", 256) // hash doesn't matter for round robin
            .unwrap();

        println!("upstream peer is: {:?}", upstream);

        // Set SNI to one.one.one.one
        let peer = Box::new(HttpPeer::new(
            upstream,
            false,
            "one.one.one.one".to_string(),
        ));
        Ok(peer)
    }
}

fn main() {
    let mut my_server = Server::new(Some(Opt::default())).unwrap();
    my_server.bootstrap();
    let mut upstreams = LoadBalancer::try_from_iter(["127.0.0.1:3001", "127.0.0.1:3000"]).unwrap();

    // For mine use HttpHealthCheck
    let hc = HttpHealthCheck::new("one.one.one.one", false);
    upstreams.set_health_check(Box::new(hc));
    upstreams.health_check_frequency = Some(std::time::Duration::from_secs(1));

    let background = background_service("health check", upstreams);
    let upstreams = background.task();

    let mut lb = http_proxy_service(&my_server.configuration, LB(upstreams));
    lb.add_tcp("0.0.0.0:6188");

    my_server.add_service(background);
    my_server.add_service(lb);
    my_server.run_forever();
}
