use std::{error::Error, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener, sync::Notify,
};

/// The `TcpServer` struct represents a TCP server with a listener and a notification mechanism.
/// 
/// # Properties:
/// 
/// * `listener`: The `listener` property in the `TcpServer` struct is of type `TcpListener`. It is used
/// to listen for incoming TCP connections on a specific port.
/// * `notify`: The `notify` property in the `TcpServer` struct is of type `Arc<Notify>`. `Arc` stands
/// for "Atomically Reference Counted" and is a thread-safe reference-counting pointer. `Notify` is a
/// synchronization primitive that allows threads to wait until a condition is satisfied
pub struct TcpServer {
    listener: TcpListener,
    notify: Arc<Notify>,
}

impl TcpServer {
    /// The function `bind` asynchronously binds a TCP listener to a specified address and returns a
    /// `TcpServer` instance wrapped in a `Result`.
    /// 
    /// # Arguments:
    /// 
    /// * `addr`: The `addr` parameter in the `bind` function is a reference to a string that represents
    /// the address to which the TCP listener will bind. This address typically includes the IP address
    /// and port number on which the server will listen for incoming connections.
    /// 
    /// # Returns:
    /// 
    /// The `bind` function returns a `Result` containing an instance of `TcpServer` if the operation is
    /// successful, or a boxed `dyn Error` trait object if an error occurs during the process.
    pub async fn bind(addr: &str) -> Result<Self, Box<dyn Error>> {
        let listener = TcpListener::bind(addr).await?;
        let notify = Arc::new(Notify::new());
        Ok(TcpServer { listener, notify })
    }

    /// The function `run` is an asynchronous Rust function that continuously accepts incoming
    /// connections, reads data from the socket, echoes it back, and can be shut down upon notification.
    /// 
    /// # Returns:
    /// 
    /// The `run` function is returning a `Result` with an empty tuple `()` on success or a `Box`
    /// containing any type that implements the `Error` trait on failure.
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        loop {
            tokio::select! {
                Ok((mut socket, _)) = self.listener.accept() => {
                    let notify = self.notify.clone();
                    tokio::spawn(async move {
                        let mut buffer = vec![0; 1024]; // Buffer to read data
                        loop {
                            tokio::select! {
                                result = socket.read(&mut buffer) => {
                                    match result {
                                        Ok(0) => return, // Connection closed
                                        Ok(n) => {
                                            // Echo the message back
                                            if let Err(e) = socket.write_all(&buffer[..n]).await {
                                                eprintln!("Failed to write to socket: {}", e);
                                                return;
                                            }
                                        }
                                        Err(_) => {
                                            eprintln!("Failed to read from socket");
                                            return;
                                        }
                                    }
                                },
                                // Check for shutdown signal
                                _ = notify.notified() => {
                                    println!("Shutting down the server...");
                                    return; // Exit the loop if notified
                                }
                            }
                        }
                    });
                }
                // You can include other handling or a timeout here if needed
            }
        }
    }

    /// The `shutdown` function in Rust asynchronously notifies one waiting task to shut down.
    pub async fn shutdown(&self) {
        self.notify.notify_one();
    }

}
