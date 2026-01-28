mod log;
mod metrics;

use std::time::Duration;

use anyhow::anyhow;
use clap::{Args, Subcommand};
use tonic::{
    codec::CompressionEncoding,
    service::interceptor::InterceptedService,
    transport::{Endpoint, Uri},
};

use crate::{
    cli::ResourceOptions,
    otlp::grpc::{CustomHeaderInterceptor, Header, LogsClient, MetricsClient},
};

use self::{log::ExportLogsContext, metrics::ExportMetricsContext};

/// Export telemetry data.
#[derive(Args, Debug)]
pub(super) struct ExportCommand {
    /// to which telemetry data exported
    #[arg(long, default_value = "http://localhost:4317", global = true)]
    endpoint: Uri,

    /// custom headers (format: key:value)
    /// example: -H "authorization:basic xxx" -H "organization:default"
    #[arg(long, short = 'H', alias = "header", global = true, value_parser = parse_header)]
    headers: Vec<Header>,

    /// Resources
    #[command(flatten)]
    resources: ResourceOptions,
    /// telemetry
    #[command(subcommand)]
    command: ExportTelemetryCommand,
}

fn parse_header(s: &str) -> anyhow::Result<Header> {
    use tonic::metadata::{MetadataKey, MetadataValue};

    match s.split_once(':') {
        Some((key, value)) => Ok(Header {
            key: MetadataKey::from_bytes(key.as_bytes())?,
            value: MetadataValue::try_from(value)?,
        }),
        _ => Err(anyhow!("expect key:value format for header")),
    }
}

#[derive(Subcommand, Debug)]
enum ExportTelemetryCommand {
    #[command(alias = "metric")]
    Metrics(metrics::ExportMetricsCommand),
    #[command(alias = "log")]
    Logs(log::ExportLogsCommand),
}

impl ExportCommand {
    pub(super) async fn run(self) -> anyhow::Result<()> {
        let Self {
            command,
            endpoint,
            resources,
            headers,
        } = self;

        let channel = {
            let endpoint = Endpoint::from(endpoint)
                .connect_timeout(Duration::from_secs(10))
                .timeout(Duration::from_secs(10));
            endpoint.connect().await?
        };

        let interceptor = CustomHeaderInterceptor::new(headers);

        macro_rules! configure_compression {
            ($client:expr) => {
                $client
                    .accept_compressed(CompressionEncoding::Gzip)
                    .accept_compressed(CompressionEncoding::Zstd)
            };
        }

        match command {
            ExportTelemetryCommand::Metrics(metrics) => {
                let client = configure_compression!(MetricsClient::new(InterceptedService::new(
                    channel,
                    interceptor,
                )));
                let ctx = ExportMetricsContext { client, resources };
                metrics.run(ctx).await
            }
            ExportTelemetryCommand::Logs(logs) => {
                let client = configure_compression!(LogsClient::new(InterceptedService::new(
                    channel,
                    interceptor,
                )));
                let ctx = ExportLogsContext { client, resources };
                logs.run(ctx).await
            }
        }
    }
}
