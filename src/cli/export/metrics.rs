use crate::{
    cli::{parse_attribute, Attribute, ResourceOptions},
    otlp::grpc::MetricsClient,
};
use chrono::{DateTime, FixedOffset, Utc};
use clap::{Args, Subcommand};
use opentelemetry_proto::tonic::{
    collector::metrics::v1::ExportMetricsServiceRequest,
    metrics::v1::{
        metric, number_data_point, Gauge, Metric, NumberDataPoint, ResourceMetrics, ScopeMetrics,
    },
};
use tracing::{debug, info};

/// Export metrics data
#[derive(Args, Debug)]
pub struct ExportMetricsCommand {
    #[command(subcommand)]
    command: ExportMetricsDataCommand,
}

#[derive(Subcommand, Debug)]
enum ExportMetricsDataCommand {
    Gauge(ExportGaugeCommand),
}

#[derive(Args, Debug)]
#[command(next_help_heading = "Metrics")]
struct MetricOptions {
    /// Metrics name
    #[arg(long)]
    name: String,
    /// Metrics description
    #[arg(long)]
    description: Option<String>,
    /// Metrics unit
    #[arg(long)]
    unit: Option<String>,
}

#[derive(Args, Debug)]
#[command(next_help_heading = "DataPoint")]
struct DataPointOptions {
    /// The set of key/value pairs that uniquely identify the timeseries from where this data point belongs
    #[arg(long, value_parser = parse_attribute)]
    attributes: Vec<Attribute>,
    /// Metrics data point start timestamp
    #[arg(long, value_parser = chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339, value_name = "RFC3339", visible_alias = "start-time-unix")]
    start_time: Option<DateTime<FixedOffset>>,
    /// Metrics data point timestamp
    /// default current time
    #[arg(long, value_parser = chrono::DateTime::<chrono::FixedOffset>::parse_from_rfc3339, value_name = "RFC3339",visible_alias = "time-unix")]
    time: Option<DateTime<FixedOffset>>,
    /// examplars
    #[arg(long)]
    examplers: Vec<String>,
}

/// Export gauge data
#[derive(Args, Debug)]
struct ExportGaugeCommand {
    #[command(flatten)]
    metric: MetricOptions,
    #[command(flatten)]
    data_point: DataPointOptions,
    #[command(flatten)]
    value: NumberDataPointValue,
}

#[derive(Args, Debug)]
#[group(required = true)]
struct NumberDataPointValue {
    /// Metrics data point f64 value
    #[arg(long, value_name = "f64")]
    value_as_double: Option<f64>,
    // #[arg(long)]
    /// Metrics data point i64 value
    #[arg(long, value_name = "i64")]
    value_as_int: Option<i64>,
}

pub struct ExportMetricsContext {
    pub client: MetricsClient,
    pub resources: ResourceOptions,
}

impl ExportMetricsCommand {
    pub async fn run(self, ctx: ExportMetricsContext) -> anyhow::Result<()> {
        let Self { command, .. } = self;

        match command {
            ExportMetricsDataCommand::Gauge(gauge) => gauge.run(ctx).await,
        }
    }
}

impl ExportGaugeCommand {
    async fn run(self, mut ctx: ExportMetricsContext) -> anyhow::Result<()> {
        let Self {
            metric,
            data_point,
            value,
        } = self;

        let schema_url = ctx.resources.schema_url.take().unwrap_or_default();
        let value = match value {
            NumberDataPointValue {
                value_as_double: Some(v),
                ..
            } => number_data_point::Value::AsDouble(v),
            NumberDataPointValue {
                value_as_int: Some(v),
                ..
            } => number_data_point::Value::AsInt(v),
            _ => unreachable!(),
        };

        let resource_metrics = ResourceMetrics {
            resource: Some(ctx.resources.resource()),
            scope_metrics: vec![ScopeMetrics {
                scope: None,
                schema_url: schema_url.clone(),
                metrics: vec![Metric {
                    name: metric.name.clone(),
                    description: metric.description.unwrap_or_default(),
                    unit: metric.unit.unwrap_or_default(),
                    data: Some(metric::Data::Gauge(Gauge {
                        data_points: vec![NumberDataPoint {
                            attributes: data_point.attributes.into_iter().map(From::from).collect(),
                            start_time_unix_nano: data_point
                                .start_time
                                .unwrap_or_else(|| Utc::now().into())
                                .timestamp_nanos_opt()
                                .unwrap_or(0)
                                as u64,
                            time_unix_nano: data_point
                                .time
                                .unwrap_or_else(|| Utc::now().into())
                                .timestamp_nanos_opt()
                                .unwrap_or(0) as u64,
                            exemplars: vec![],
                            flags: 0,
                            value: Some(value),
                        }],
                    })),
                }],
            }],
            schema_url,
        };

        let response = ctx
            .client
            .export(ExportMetricsServiceRequest {
                resource_metrics: vec![resource_metrics],
            })
            .await?;

        debug!("{response:?}");

        // should we check partial success?
        info!(
            instrument = "gauge",
            name = metric.name,
            "Metrics successfully exported"
        );

        Ok(())
    }
}
