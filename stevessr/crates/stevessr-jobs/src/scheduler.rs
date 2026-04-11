use tokio::time::{interval, Duration};

pub struct JobScheduler;

impl JobScheduler {
    pub async fn run(runner: &super::runner::JobRunner) {
        let mut ticker = interval(Duration::from_secs(60));
        loop {
            ticker.tick().await;
            // TODO: check scheduled jobs and enqueue those whose time has come
            let _ = runner;
        }
    }
}
