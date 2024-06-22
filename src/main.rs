use tokio::sync::broadcast;
use screenmonitor::core::manager;
use screenmonitor::utils::logger;
use screenmonitor::core::Scheduler;
use screenmonitor::{register, Task};
use tokio::signal;
use tracing::{error, info};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init logger
    let _guard = logger::init("./logs".to_string()).unwrap();

    match register() {
        Ok(_) => info!("Screen Monitor Registered..."),
        Err(e) => {
            error!("Failed to register Screen Monitor: {:?}", e);
            return Err(anyhow::anyhow!("Failed to register Screen Monitor"));
        }
    }

    let (tx, rx) = broadcast::channel(1);
    let mut m = manager::Manager::new(rx);

    let _ = m.start(Task::default()).await;

    tokio::spawn(async move {
        m.monitor().await;
    });

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                info!("Received Ctrl-C signal");
                tx.send("stop".to_string()).unwrap();
                break;
            }
        }
    }

    info!("Screen Monitor Stopped...");
    Ok(())
}
