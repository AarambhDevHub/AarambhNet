# AarambhNet

AarambhNet is a Rust library providing networking capabilities with HTTP, TCP, and UDP support. This library aims to simplify the process of creating networking applications in Rust.

## Features

- **HTTP Client**: Easily make HTTP requests with support for custom headers and endpoints.
- **TCP Server/Client**: Set up TCP servers and clients to handle connection-based communication.
- **UDP Server/Client**: Implement lightweight UDP communication for fast, connectionless data transfer.
- 

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
aarambh-net = "0.1.3"
```

## Usage

### HTTP Client Example

    use aarambh_net::http::HttpClient;

    #[tokio::main]
    async fn main() {
        let base_url = "https://example.com";
        HttpClient::new(base_url, None).unwrap()
        let response = client.get("/endpoint", None).await.unwrap();
        println!("Response: {:?}", response);
    }

### TCP Server and Client Example

    use aarambh_net::tcp::{TcpServer, TcpClient};

    #[tokio::main]
    async fn main() {
        let addr = "127.0.0.1:8000";
        let server = TcpServer::bind(addr).await?;
        let server_task = tokio::spawn(async move {
            server.run().await.unwarp();
        });

        // Allow some time for the server to start
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Create a TCP client and send a message
        let mut client = TcpClient::connect(server_addr).await?;
        let message = "Hello, TCP Server!";
        client.send_message(message).await?;

        // Receive and print the response
        let response = client.receive_response().await?;
        println!("Received from server: {}", response);

        // Wait for user input to shut down the server
        println!("Press Enter to stop the server...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Shutdown the server
        server.shutdown().await;

        // Wait for the server task to complete
        let _ = server_task.await;

        Ok(())
    }

### UDP Server and Client Example

    use aarambh_net::udp::UdpClient;

    #[tokio::main]
    async fn main() {
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


## Contributing
Contributions are welcome! Please open an issue or submit a pull request if you'd like to help improve the library.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

## Donations

If you find this project useful and would like to support its continued development, you can make a donation via [Buy Me a Coffee](https://buymeacoffee.com/aarambhdevhub).

Thank you for your support!