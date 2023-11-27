pub mod tracing_utils {
    pub use tracing::{debug, error, info, subscriber, trace, warn, Level};
    use tracing_subscriber;
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, EnvFilter};

    pub fn tracing() {
        let subscriber = tracing_subscriber::fmt()
            .with_target(false)
            .without_time()
            .with_max_level(Level::INFO)
            .with_writer(std::io::stderr)
            .compact()
            .finish();
        let _ = tracing::subscriber::set_global_default(subscriber)
            .map_err(|_err| eprintln!("Unable to set global default subscriber"));
    }

    pub fn tracing_env() {
        let fmt_layer = fmt::layer().compact().with_target(false).without_time();

        let filter_layer = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("info"))
            .unwrap();

        tracing_subscriber::registry()
            .with(filter_layer)
            .with(fmt_layer)
            .init();
    }
}
