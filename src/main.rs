use opentelemetry_cli::cli;
use tracing::error;

fn init_tracing() {
    use tracing_subscriber::{
        filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt as _, Registry,
    };

    Registry::default()
        .with(
            fmt::Layer::new()
                .with_ansi(true)
                .with_timer(fmt::time::UtcTime::rfc_3339())
                .with_file(false)
                .with_line_number(false)
                .with_target(false),
        )
        .with(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap(),
        )
        .init();
}

#[tokio::main]
async fn main() {
    init_tracing();

    let _app = cli::parse();
    if let Err(err) = cli::parse().run().await {
        error!("{err:?}");
        std::process::exit(1);
    }
}
