#[cfg(feature = "logger")]
use std::fs;
#[cfg(feature = "logger")]
use tracing_subscriber::EnvFilter;
#[cfg(feature = "logger")]
use tracing_appender::rolling;
#[cfg(feature = "logger")]
use std::sync::Once;

#[cfg(feature = "logger")]
static INIT: Once = Once::new();

/// Initializes logging (only when `logger` feature is enabled)
#[cfg(feature = "logger")]
pub fn init_logger() {
    INIT.call_once(|| {
        fs::create_dir_all("logs").ok();

        let file_appender = rolling::daily("logs", "aarambh-net.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        let subscriber = tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(EnvFilter::from_default_env())
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global subscriber");
    });
}

#[cfg(not(feature = "logger"))]
pub fn init_logger() {

}