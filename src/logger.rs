#[cfg(feature = "logger")]
use std::fs;
#[cfg(feature = "logger")]
use std::sync::Once;
#[cfg(feature = "logger")]
use tracing::{info, Level};
#[cfg(feature = "logger")]
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, Registry};
#[cfg(feature = "logger")]
use tracing_appender::rolling;
#[cfg(feature = "logger")]
use tracing_appender::non_blocking::WorkerGuard;

#[cfg(feature = "logger")]
static INIT: Once = Once::new();
#[cfg(feature = "logger")]
static mut LOG_GUARD: Option<WorkerGuard> = None;

/// Initializes logging (only when `logger` feature is enabled)
#[cfg(feature = "logger")]
pub fn init_logger() {
    INIT.call_once(|| {
        fs::create_dir_all("logs").expect("Failed to create logs directory");

        let file_appender = rolling::daily("logs", "aarambh-net.log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        // Store the guard globally to prevent logs from being dropped
        unsafe {
            LOG_GUARD = Some(guard);
        }

        let subscriber = Registry::default()
            .with(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
            .with(fmt::layer().with_writer(non_blocking));

        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global subscriber");

        info!("Logger initialized successfully!");
    });
}

#[cfg(not(feature = "logger"))]
pub fn init_logger() {}

