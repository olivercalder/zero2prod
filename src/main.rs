use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8000").expect("Failed to bind address.");
    zero2prod::startup::run(listener)?.await
}
