use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

/// The `TcpClient` struct represents a TCP client with a `stream` field of type `TcpStream`.
/// 
/// # Properties:
/// 
/// * `stream`: The `stream` property in the `TcpClient` struct represents the TCP stream that is used
/// for communication with the server. It allows data to be sent and received over the network
/// connection.
pub struct TcpClient {
    stream: TcpStream,
}

impl TcpClient {
    /// The function `connect` establishes a TCP connection to the specified address asynchronously in
    /// Rust.
    /// 
    /// # Arguments:
    /// 
    /// * `addr`: The `addr` parameter in the `connect` function is a reference to a string (`&str`)
    /// which represents the address to which the TCP client will connect. This address typically
    /// includes the IP address and port number of the server to establish the connection with.
    /// 
    /// # Returns:
    /// 
    /// The `connect` function is returning a `Result` containing either an instance of `TcpClient` if
    /// the connection is successful, or a boxed `Error` trait object if an error occurs during the
    /// connection process.
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn Error>> {
        let stream = TcpStream::connect(addr).await?;
        Ok(TcpClient { stream })
    }

    /// The function `send_message` sends a message over a stream in Rust asynchronously.
    /// 
    /// # Arguments:
    /// 
    /// * `message`: The `message` parameter in the `send_message` function is a reference to a string
    /// (`&str`) that represents the message to be sent.
    /// 
    /// # Returns:
    /// 
    /// The `send_message` function returns a `Result` enum with the success type `()` (unit type) if
    /// the message is successfully sent, or an error wrapped in a `Box<dyn Error>` if an error occurs
    /// during the process.
    pub async fn send_message(&mut self, message: &str) -> Result<(), Box<dyn Error>> {
        self.stream.write_all(message.as_bytes()).await?;
        Ok(())
    }

    /// The function `receive_response` reads data from a stream and returns it as a string.
    /// 
    /// # Returns:
    /// 
    /// The `receive_response` function returns a `Result` containing a `String` or a `Box<dyn Error>`.
    pub async fn receive_response(&mut self) -> Result<String, Box<dyn Error>> {
        let mut buffer = vec![0; 1024];
        let n = self.stream.read(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]).to_string();
        Ok(response)
    }
}