use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use tracing_appender::{rolling};

pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let log_output = std::env::var("LOG_OUTPUT")
        .unwrap_or_else(|_| "stdout".into());

    if log_output == "file" {
        let file_appender = rolling::daily("logs", "app.log");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .with_writer(non_blocking)
                    .with_ansi(false)   
            );
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .with_writer(std::io::stdout)
                    .with_ansi(true)   
            );
    }
}