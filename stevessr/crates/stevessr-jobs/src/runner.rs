use std::collections::HashMap;
use std::sync::Arc;
use crate::traits::Job;
use redis::aio::ConnectionManager;

pub struct JobRunner {
    jobs: HashMap<String, Arc<dyn Job>>,
    redis: ConnectionManager,
}

impl JobRunner {
    pub fn new(redis: ConnectionManager) -> Self {
        Self { jobs: HashMap::new(), redis }
    }

    pub fn register(&mut self, job: Arc<dyn Job>) {
        self.jobs.insert(job.name().to_string(), job);
    }

    pub async fn enqueue(&self, job_name: &str, payload: serde_json::Value) -> Result<(), anyhow::Error> {
        let data = serde_json::json!({ "job": job_name, "payload": payload });
        let _: () = redis::cmd("LPUSH")
            .arg("stevessr:jobs:default")
            .arg(serde_json::to_string(&data)?)
            .query_async(&mut self.redis.clone())
            .await?;
        Ok(())
    }

    pub async fn run_loop(&self) {
        loop {
            match self.process_next().await {
                Ok(true) => {} // processed a job
                Ok(false) => tokio::time::sleep(tokio::time::Duration::from_secs(1)).await,
                Err(e) => {
                    tracing::error!("job processing error: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn process_next(&self) -> Result<bool, anyhow::Error> {
        let result: Option<String> = redis::cmd("RPOP")
            .arg("stevessr:jobs:default")
            .query_async(&mut self.redis.clone())
            .await?;

        if let Some(data) = result {
            let parsed: serde_json::Value = serde_json::from_str(&data)?;
            let job_name = parsed["job"].as_str().unwrap_or("");
            let payload = parsed["payload"].clone();

            if let Some(job) = self.jobs.get(job_name) {
                if let Err(e) = job.execute(payload).await {
                    tracing::error!(job = job_name, "job execution failed: {}", e);
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
