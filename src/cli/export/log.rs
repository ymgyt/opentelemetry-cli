use chrono::Utc;
use clap::Args;
use opentelemetry_proto::tonic::{
    collector::logs::v1::ExportLogsServiceRequest,
    common::v1::{any_value, AnyValue},
    logs::v1::{LogRecord, ResourceLogs, ScopeLogs},
};
use tracing::{debug, info};

use crate::{
    cli::{instrumentation_scope, parse_attribute, Attribute, ResourceOptions},
    otlp::grpc::LogsClient,
};

/// Export log data
#[derive(Args, Debug)]
pub struct ExportLogsCommand {
    #[command(flatten)]
    log_record: LogRecordOptions,
}

#[derive(Args, Debug)]
#[command(next_help_heading = "LogRecord")]
struct LogRecordOptions {
    /// Request trace id as defined in W3C Trace Context. Can be set for logs that are part of request processing and have an assigned trace id.
    // trace_id: Option<u32>,
    /// Span id. Can be set for logs that are part of a particular processing span. If SpanId is present TraceId SHOULD be also present.
    // span_id: Option<i32>,
    /// Trace flag as defined in W3C Trace Context specification. At the time of writing the specification defines one flag - the SAMPLED flag.
    #[arg(long)]
    trace_flags: Option<u32>,
    /// (also known as log level) original string representation of the severity as it is known at the source
    #[arg(long)]
    severity_text: String,
    /// numerical value of the severity
    /// (https://opentelemetry.io/docs/specs/otel/logs/data-model/#field-severitynumber)
    /// TRACE(1), INFO(9), WARN(13), ERROR(17), FATAL(21)
    #[arg(long)]
    severity_number: i32,
    /// The set of key/value pairs that uniquely identify the timeseries from where this data point belongs
    #[arg(long, value_parser = parse_attribute)]
    attributes: Vec<Attribute>,
    /// A value containing the body of the log record
    #[arg(long)]
    body: String,
}

/// Common input for export logs operation
pub struct ExportLogsContext {
    pub client: LogsClient,
    pub resources: ResourceOptions,
}

impl ExportLogsCommand {
    pub async fn run(self, mut ctx: ExportLogsContext) -> anyhow::Result<()> {
        let Self { log_record } = self;

        let schema_url = ctx.resources.schema_url.take().unwrap_or_default();

        let log_record = LogRecord {
            time_unix_nano: Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64,
            observed_time_unix_nano: 0,
            severity_number: log_record.severity_number,
            severity_text: log_record.severity_text,
            body: Some(AnyValue {
                value: Some(any_value::Value::StringValue(log_record.body)),
            }),
            attributes: log_record.attributes.into_iter().map(From::from).collect(),
            dropped_attributes_count: 0,
            flags: log_record.trace_flags.unwrap_or(0),
            // TODO: how to convert to Vec<u8> ?
            trace_id: Vec::new(),
            // TODO: how to convert to Vec<u8> ?
            span_id: Vec::new(),
        };

        let scope_logs = ScopeLogs {
            scope: Some(instrumentation_scope().clone()),
            log_records: vec![log_record],
            schema_url: "".into(),
        };

        let resource_logs = ResourceLogs {
            resource: Some(ctx.resources.resource()),
            scope_logs: vec![scope_logs],
            schema_url,
        };

        let response = ctx
            .client
            .export(ExportLogsServiceRequest {
                resource_logs: vec![resource_logs],
            })
            .await?;

        debug!("{response:?}");

        // should we check partial success?
        info!("Logs successfully exported");

        Ok(())
    }
}
