#[tokio::main]
async fn main() {

    let listener = tokio::net::TcpListener::bind(":7888").await.expect("Cannot open socket");

    'listener: loop {

        let (socket, _) = listener.accept().await.expect("Cannot accept connection");

    }

}