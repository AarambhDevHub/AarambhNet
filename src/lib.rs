mod http;
mod tcp;
mod udp;

#[cfg(feature = "logger")]
mod logger;

pub use http::HttpClient;
pub use tcp::TcpClient;
pub use tcp::TcpServer;
pub use udp::UdpServer;
pub use reqwest::header;

#[cfg(feature = "logger")]
pub use logger::init_logger;

pub fn init() {
    #[cfg(feature = "logger")]
    init_logger();
}