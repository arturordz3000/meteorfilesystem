pub use tracing;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use tracing_appender::{rolling};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogOutput {
    File,
    Stdout,
}

pub fn log_output_from_env() -> LogOutput {
    match std::env::var("LOG_OUTPUT") {
        Ok(val) if val.eq_ignore_ascii_case("file") => LogOutput::File,
        _ => LogOutput::Stdout,
    }
}

pub fn env_filter_from_env() -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))
}

pub fn init_logging() {
    let filter = env_filter_from_env();
    let log_output = log_output_from_env();

    match log_output {
        LogOutput::File => {
            let file_appender = rolling::daily("logs", "app.log");
            let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .with_writer(non_blocking)
                        .with_ansi(false)   
                )
                .init();
        }
        LogOutput::Stdout => {
            tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .with_writer(std::io::stdout)
                    .with_ansi(true)   
            )
            .init();
        }
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use crate::{LogOutput, env_filter_from_env, init_logging, log_output_from_env};

    #[test]
    #[serial]
    fn default_log_output_is_stdout() {
        unsafe {
            std::env::remove_var("LOG_OUTPUT");
        }
        assert_eq!(log_output_from_env(), LogOutput::Stdout);
    }

    #[test]
    #[serial]
    fn file_log_output_when_env_is_file() {
        unsafe  {
            std::env::set_var("LOG_OUTPUT", "file");
        }
        assert_eq!(log_output_from_env(), LogOutput::File);
    }

    #[test]
    #[serial]
    fn log_output_env_ignores_case() {
        unsafe  {
            std::env::set_var("LOG_OUTPUT", "FILE");
        }
        assert_eq!(log_output_from_env(), LogOutput::File);

        unsafe  {
            std::env::set_var("LOG_OUTPUT", "File");
        }
        assert_eq!(log_output_from_env(), LogOutput::File);
    }

    #[test]
    #[serial]
    fn default_env_filter_is_info() {
        unsafe  {
            std::env::remove_var("RUST_LOG");
        }

        let filter = env_filter_from_env();
        let filter_name = format!("{:?}", filter);

        assert!(filter_name.contains("LevelFilter::INFO"));
    }

    #[test]
    #[serial]
    fn env_filter_uses_env_var() {
        unsafe  {
            std::env::set_var("RUST_LOG", "debug");
        }

        let filter = env_filter_from_env();
        let filter_name = format!("{:?}", filter);
        
        assert!(filter_name.contains("LevelFilter::DEBUG"));
    }

    #[test]
    #[serial]
    fn init_logging_does_not_panic_with_no_env_vars() {
        unsafe {
            std::env::remove_var("LOG_OUTPUT");
            std::env::remove_var("RUST_LOG");
        }

        init_logging();
    }

    #[test]
    #[serial]
    fn init_logging_does_not_panic_with_stdout() {
        unsafe {
            std::env::set_var("LOG_OUTPUT", "stdout");
        }

        init_logging();
    }

    #[test]
    #[serial]
    fn init_logging_does_not_panic_with_file() {
        unsafe {
            std::env::set_var("LOG_OUTPUT", "file");
        }

        init_logging();
    }
}