use crate::core::job::Job;
use crate::core::jobmanager::JobManager;
use crate::remote::*;

use tokio::task;

use log::debug;

pub struct Parallel {
    job_manager: JobManager,
}

impl Parallel {
    pub fn new() -> Parallel {
        let job_manager: JobManager = JobManager::new();

        Parallel { job_manager }
    }

    pub fn new_cmd(&mut self, args: Vec<String>) {
        self.job_manager.add_job(Job::new(args))
    }

    pub fn start(&mut self) {
        debug!("Parallel start with => {}", self.job_manager);
        self.job_manager.exec_all();
    }
}

pub async fn remote_exec_test(client_side: bool) {
    if client_side {
        client::test().await;
    } else {
        server::test().await;
    }
}
