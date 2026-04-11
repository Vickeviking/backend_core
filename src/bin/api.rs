use backend_core::runtime::initialize_process;
use backend_core::startup::Application;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = initialize_process("api")?;
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
