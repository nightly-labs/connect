use server::router::get_router;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::mpsc::channel;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");

    let router = get_router().await;
    let socket = SocketAddr::from_str("127.0.0.1:6969").unwrap();

    let server = axum::Server::bind(&socket).serve(
        router
            .clone()
            .into_make_service_with_connect_info::<SocketAddr>(),
    );
    tokio::spawn(server);

    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    rx.recv().expect("Could not receive from channel.");
    println!("Got it! Exiting...");
}
