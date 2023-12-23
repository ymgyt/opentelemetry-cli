use clap::Args;

/// Export log data
#[derive(Args, Debug)]
pub struct ExportLogsCommand {}

impl ExportLogsCommand {
    pub async fn run(self) -> anyhow::Result<()> {
        Ok(())
    }
}
