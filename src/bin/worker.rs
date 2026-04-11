use backend_core::features::newsletter::infrastructure::worker::build_worker;
use backend_core::runtime::initialize_process;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = initialize_process("worker")?;
    let worker = build_worker(&configuration);
    worker.run_until_stopped().await?;
    Ok(())
}
