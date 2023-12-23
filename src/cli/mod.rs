mod export;

use clap::{Args, Parser, Subcommand};
use opentelemetry_proto::tonic::{
    common::v1::{any_value, AnyValue, KeyValue},
    resource::v1::Resource,
};
use tracing::debug;

#[derive(Parser, Debug)]
#[command(
    version,
    propagate_version = true,
    disable_help_subcommand = true,
    help_expected = true,
    infer_subcommands = true,
    bin_name = "otel"
)]
pub struct App {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Export(export::ExportCommand),
}

#[derive(Args, Debug, Clone)]
#[command(next_help_heading = "Resource")]
struct ResourceOptions {
    /// Entity from which telemetry data produced.
    /// key:value format expected (ex. service.name:foo)
    /// For a more detailed explanation, see https://opentelemetry.io/docs/instrumentation/js/resources/
    #[arg(long, short = 'r', global = true, value_parser = parse_attribute)]
    resources: Vec<Attribute>,

    /// Telemetry schema url
    /// For a more detailed explanation, see https://opentelemetry.io/docs/specs/otel/schemas/
    #[arg(long, global = true)]
    schema_url: Option<String>,
}

/// Cli attribute representation
#[derive(Debug, Clone)]
struct Attribute {
    key: String,
    value: String,
}

impl From<Attribute> for KeyValue {
    fn from(attr: Attribute) -> Self {
        KeyValue {
            key: attr.key,
            value: Some(AnyValue {
                value: Some(any_value::Value::StringValue(attr.value)),
            }),
        }
    }
}

impl ResourceOptions {
    fn resource(&self) -> Resource {
        Resource {
            attributes: self.resources.iter().cloned().map(From::from).collect(),

            dropped_attributes_count: 0,
        }
    }
}

fn parse_attribute(s: &str) -> Result<Attribute, String> {
    match s.splitn(2, ':').collect::<Vec<&str>>().as_slice() {
        [key, value] => Ok(Attribute {
            key: key.to_string(),
            value: value.to_string(),
        }),
        _ => Err("expect key:value format for attribute".to_string()),
    }
}

impl App {
    pub async fn run(self) -> anyhow::Result<()> {
        let App { command } = self;

        debug!("Running...");

        match command {
            Command::Export(export) => export.run().await,
        }
    }
}

pub fn parse() -> App {
    App::parse()
}
