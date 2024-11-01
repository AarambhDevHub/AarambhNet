mod client;
mod server;

pub use client::TcpClient;
pub use server::TcpServer;


mod test {
    use super::*;

    async fn test_tcp() {

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    }
}