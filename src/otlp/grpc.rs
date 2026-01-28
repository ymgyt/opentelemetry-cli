use opentelemetry_proto::tonic::collector::{
    logs::v1::logs_service_client::LogsServiceClient,
    metrics::v1::metrics_service_client::MetricsServiceClient,
};
use tonic::{
    self,
    metadata::{Ascii, MetadataKey, MetadataValue},
    service::interceptor::InterceptedService,
    transport::Channel,
    Request, Status,
};

/// tonic metrics client alias
pub type MetricsClient = MetricsServiceClient<InterceptedService<Channel, CustomHeaderInterceptor>>;
/// tonic logs client alias
pub type LogsClient = LogsServiceClient<InterceptedService<Channel, CustomHeaderInterceptor>>;

#[derive(Debug, Clone)]
pub struct Header {
    pub key: MetadataKey<Ascii>,
    pub value: MetadataValue<Ascii>,
}

#[derive(Clone)]
pub struct CustomHeaderInterceptor {
    headers: Vec<Header>,
}

impl CustomHeaderInterceptor {
    pub fn new(headers: Vec<Header>) -> Self {
        Self { headers }
    }
}

impl tonic::service::Interceptor for CustomHeaderInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        for header in &self.headers {
            request
                .metadata_mut()
                .insert(header.key.clone(), header.value.clone());
        }
        Ok(request)
    }
}
