use hashmaprs::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Running server on http://127.0.0.1:8080");

    run(listener)?.await
}
