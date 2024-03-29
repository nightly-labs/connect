use server::env::ONLY_RELAY_SERVICE;
use server::routes::router::get_router;
use std::net::SocketAddr;
use std::sync::mpsc::channel;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    let filter: EnvFilter = "debug,sqlx=warn,tower_http=trace,hyper=warn"
        .parse()
        .expect("filter should parse");

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let router = get_router(ONLY_RELAY_SERVICE()).await;
    let listener = tokio::net::TcpListener::bind(&"127.0.0.1:6969")
        .await
        .expect("Failed to bind socket");

    let server = axum::serve(
        listener,
        router
            .clone()
            .into_make_service_with_connect_info::<SocketAddr>(),
    );

    tokio::spawn(async move {
        server.await.unwrap();
    });

    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");
    println!("Got it! Exiting...");
}
