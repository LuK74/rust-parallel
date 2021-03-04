use super::job::Job;
use std::fmt;
use tokio::runtime::{Runtime,Builder};

pub struct JobManager {
    cmds : Vec<Job>,
    nb_thread : Option<usize>
}

impl fmt::Display for JobManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime with {:?} threads", self.nb_thread);
        for i in 0..self.cmds.len() {
            write!(f, "\n\t{}", self.cmds[i]);
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
        let runtime : Runtime = match self.nb_thread {
            Some(n) => runtime_builder.worker_threads(n).build().unwrap(),
            None => runtime_builder.build().unwrap()
        };

        runtime.block_on(async {
            println!("start block_on");
            for i in 0..self.cmds.len() {
                let _r = self.cmds[i]
                    .exec();
            }
            println!("stop block_on");
        });
    }
}