use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::http::StatusCode;
use salvo::prelude::*;

#[handler]
async fn greet(req: &mut Request) -> String {
    let name = req
        .params()
        .get("name")
        .cloned()
        .unwrap_or("World".to_string());
    format!("Hello, {}!", &name)
}

#[handler]
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    let cert = include_bytes!("/etc/letsencrypt/live/showdown.monster/cert.pem").to_vec();
    let key = include_bytes!("/etc/letsencrypt/live/showdown.monster/privkey.pem").to_vec();

    let router = Router::with_path("<name>").get(greet);
    let config = RustlsConfig::new(Keycert::new().cert(cert.as_slice()).key(key.as_slice()));

    let listener = TcpListener::new(("0.0.0.0", 5800)).rustls(config.clone());
    let acceptor = QuinnListener::new(config, ("0.0.0.0", 5800))
        .join(listener)
        .bind()
        .await;

    Server::new(acceptor).serve(router).await;
}
