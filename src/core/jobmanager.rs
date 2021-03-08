use super::job::Job;
use std::fmt;
use tokio::runtime::{Runtime,Builder};
use log::debug;

pub struct JobManager {
    cmds : Vec<Job>,
    nb_thread : Option<usize>
}

impl fmt::Display for JobManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _r = write!(f, "Runtime with {:?} threads", self.nb_thread);
        for i in 0..self.cmds.len() {
            let _r = write!(f, "\n\t{}", self.cmds[i]);
        }
        Ok(())
    }
}

impl JobManager {
    pub fn new() -> JobManager {
        JobManager{
            cmds : vec!(),
            nb_thread : None
        }
    }

    pub fn add_job (&mut self, job : Job) {
        self.cmds.push(job);
    }

    pub fn exec_all(&mut self) {
        let mut runtime_builder : Builder = Builder::new_multi_thread();
        runtime_builder.enable_all();
        let runtime : Runtime = match self.nb_thread {
            Some(n) => runtime_builder.worker_threads(n).build().unwrap(),
            None => runtime_builder.build().unwrap()
        };

        runtime.block_on(async {
            debug!("start block_on");
            
            for i in 0..self.cmds.len() {
                let _r = self.cmds[i]
                    .exec().await;
            }
            debug!("stop block_on");
        });
    }
}