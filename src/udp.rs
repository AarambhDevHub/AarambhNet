use std::{error::Error, sync::Arc};
use tokio::{net::UdpSocket, sync::Notify};

#[cfg(feature = "logger")]
use tracing::{info, error, warn};
/// The `UdpServer` struct in Rust contains a UDP socket and an Arc-wrapped notification mechanism.
/// 
/// # Properties:
/// 
/// * `socket`: The `socket` property in the `UdpServer` struct represents a UDP socket that the server
/// uses to send and receive data over the network.
/// * `notify`: The `notify` property in the `UdpServer` struct is of type `Arc<Notify>`. `Arc` stands
/// for "atomic reference counting" and is a thread-safe reference-counting pointer. `Notify` is a
/// synchronization primitive that allows threads to wait until a condition is satisfied.
pub struct UdpServer {
    socket: UdpSocket,
    notify: Arc<Notify>,
}

impl UdpServer {
    /// The function `bind` creates a UDP server bound to a specified address and returns a result with
    /// the server instance or an error.
    /// 
    /// # Arguments:
    /// 
    /// * `addr`: The `addr` parameter in the `bind` function is a reference to a string that represents
    /// the address to bind the UDP socket to. This address typically includes the IP address and port
    /// number on which the socket will listen for incoming connections.
    /// 
    /// # Returns:
    /// 
    /// The `bind` function is returning a `Result` containing an instance of `UdpServer` if the binding
    /// operation is successful. The `UdpServer` struct contains a `UdpSocket` and an `Arc<Notify>`
    /// instance.
    pub async fn bind(addr: &str) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(addr).await?;
        let notify = Arc::new(Notify::new());

        #[cfg(feature = "logger")]
        info!("UDP server bound to {}", addr);
        Ok(UdpServer { socket, notify })
    }

    /// The function `run` is an asynchronous method in Rust that continuously listens for incoming data
    /// on a UDP socket, processes the data, and echoes it back to the sender while also checking for a
    /// shutdown signal.
    /// 
    /// # Arguments:
    /// 
    /// * ``: The code you provided is a Rust asynchronous function that runs a UDP server using Tokio.
    /// Here's a breakdown of the key components:
    /// 
    /// # Returns:
    /// 
    /// The `run` function returns a `Result` with an `Ok(())` value if the UDP server is shut down
    /// successfully.
    pub async fn run(self: Arc<Self>) -> Result<(), Box<dyn Error>> {
        let mut buf = [0; 1024]; // Buffer to store incoming data
        #[cfg(feature = "logger")]
        info!("UDP server is running...");
        loop {
            tokio::select! {
                // Wait for incoming data
                Ok((len, addr)) = self.socket.recv_from(&mut buf) => {
                    // Process the incoming data
                    let received_message = String::from_utf8_lossy(&buf[..len]);
                    #[cfg(feature = "logger")]
                    info!("Received from {}: {}", addr, received_message);

                    // Echo the message back to the sender
                    if let Err(e) = self.socket.send_to(&buf[..len], addr).await {
                        #[cfg(feature = "logger")]
                        error!("Failed to send data: {}", e);
                    }
                },
                // Check for shutdown signal
                _ = self.notify.notified() => {
                    #[cfg(feature = "logger")]
                    warn!("Shutting down the UDP server...");
                    return Ok(()); // Exit the loop if notified
                }
            }
        }
    }

    /// The `shutdown` function in Rust asynchronously notifies the server to shut down.
    pub async fn shutdown(&self) {
        #[cfg(feature = "logger")]
        warn!("Shutdown signal received.");
        self.notify.notify_one(); // Notify the server to shut down
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{error::Error, time::Duration};

    #[tokio::test]
    async fn test_udp_server() -> Result<(), Box<dyn Error>> {
        let server_addr = "127.0.0.1:8000";
        let server = Arc::new(UdpServer::bind(server_addr).await?);
        let server_task = {
            let server_clone = Arc::clone(&server);
            tokio::spawn(async move {
                server_clone.run().await.unwrap();
            })
        };

        tokio::time::sleep(Duration::from_secs(1)).await;

        let client_socket = UdpSocket::bind("127.0.0.1:0").await?;
        let message = b"Hello, UDP server!";

        client_socket.send_to(message, server_addr).await?;

        let mut buf = [0; 1024];
        match client_socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                let response = String::from_utf8_lossy(&buf[..len]);
                assert_eq!(response, "Hello, UDP server!"); // Assert that the response matches the sent message
                println!("Received from {}: {}", addr, response);
            }
            Err(e) => {
                eprintln!("Failed to receive response: {}", e);
            }
        }

        // Shutdown the server
        server.shutdown().await;

        // Wait for the server task to complete
        let _ = server_task.await;

        Ok(())
    }
}
