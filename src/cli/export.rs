mod log;
mod metrics;

use crate::otlp::grpc::{LogsClient, MetricsClient};
use clap::{Args, Subcommand};

use crate::cli::ResourceOptions;

use self::{log::ExportLogsContext, metrics::ExportMetricsContext};

/// Export telemetry data.
#[derive(Args, Debug)]
pub(super) struct ExportCommand {
    /// to which telemetry data exported
    #[arg(long, default_value = "http://localhost:4317", global = true)]
    endpoint: String,
    /// Resources
    #[command(flatten)]
    resources: ResourceOptions,
    /// telemetry
    #[command(subcommand)]
    command: ExportTelemetryCommand,
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
        } = self;

        match command {
            ExportTelemetryCommand::Metrics(metrics) => {
                let client = MetricsClient::connect(endpoint).await?;
                let ctx = ExportMetricsContext { client, resources };
                metrics.run(ctx).await
            }
            ExportTelemetryCommand::Logs(logs) => {
                let client = LogsClient::connect(endpoint).await?;
                let ctx = ExportLogsContext { client, resources };
                logs.run(ctx).await
            }
        }
    }
}
