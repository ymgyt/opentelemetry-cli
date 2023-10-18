use opentelemetry_proto::tonic::collector::metrics::v1::metrics_service_client::MetricsServiceClient;
use tonic;

pub type MetricsClient = MetricsServiceClient<tonic::transport::channel::Channel>;
