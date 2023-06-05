use axum_server::tls_rustls::RustlsConfig;
use server::router::get_router;
use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::mpsc::channel;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let router = get_router().await;
    let socket = SocketAddr::from_str("127.0.0.1:6969").unwrap();
    if env::var("ENV") == Ok("PROD".to_string()) {
        let config = RustlsConfig::from_pem_file(
            PathBuf::from(env!("HOME")).join(".cert/nc2.nightly.app/fullchain.pem"),
            PathBuf::from(env!("HOME")).join(".cert/nc2.nightly.app/privkey.pem"),
        )
        .await
        .expect("no certificates at expected location");
        println!("Running in production mode.");

        let server = axum_server::bind_rustls(socket, config).serve(
            router
                .clone()
                .into_make_service_with_connect_info::<SocketAddr>(),
        );
        tokio::spawn(server);
    } else {
        println!("Running in development mode.");

        let server = axum::Server::bind(&socket).serve(
            router
                .clone()
                .into_make_service_with_connect_info::<SocketAddr>(),
        );
        tokio::spawn(server);
    }

    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");
    println!("Got it! Exiting...");
}
