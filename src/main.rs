use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    println!(
        "Listening on http://127.0.0.1:{}",
        listener.local_addr().unwrap().port()
    );
    run(listener)?.await
}
