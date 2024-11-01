# AarambhNet

AarambhNet is a Rust library providing networking capabilities with HTTP, TCP, and UDP support. This library aims to simplify the process of creating networking applications in Rust.

## Features

- **HTTP Client**: Easily make HTTP requests with support for custom headers and endpoints.
- **TCP Server/Client**: Set up TCP servers and clients to handle connection-based communication.
- **UDP Server/Client**: Implement lightweight UDP communication for fast, connectionless data transfer.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
aarambh-net = "0.1.0"
```

## Usage

### HTTP Client Example

    ```
    use aarambh_net::http::HttpClient;

    #[tokio::main]
    async fn main() {
        let client = HttpClient::new(Some("https://api.example.com".to_string()), None);
        let response = client.get("/endpoint", None).await.unwrap();
        println!("Response: {:?}", response);
    }
    ```

### TCP Server Example

    ```
    use aarambh_net::tcp::{TcpServer, TcpClient};

    #[tokio::main]
    async fn main() {
        let server = TcpServer::new("127.0.0.1:8080").await.unwrap();
        server.start().await.unwrap();
    }
    ```

### UDP Client Example

    ```
    use aarambh_net::udp::UdpClient;

    #[tokio::main]
    async fn main() {
        let client = UdpClient::new("127.0.0.1:8080").await.unwrap();
        let response = client.query("example.com").await.unwrap();
        println!("Response: {:?}", response);
    }
    ```

## Contributing
Contributions are welcome! Please open an issue or submit a pull request if you'd like to help improve the library.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for more details.

## Donations

If you find this project useful and would like to support its continued development, you can make a donation via [Buy Me a Coffee](https://buymeacoffee.com/aarambhdevhub).

Thank you for your support!