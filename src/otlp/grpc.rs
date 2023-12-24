use opentelemetry_proto::tonic::collector::{
    logs::v1::logs_service_client::LogsServiceClient,
    metrics::v1::metrics_service_client::MetricsServiceClient,
};
use tonic;

/// tonic metrics client alias
pub type MetricsClient = MetricsServiceClient<tonic::transport::channel::Channel>;
/// tonic logs client alias
pub type LogsClient = LogsServiceClient<tonic::transport::channel::Channel>;
