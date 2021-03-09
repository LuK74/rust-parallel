use super::job::Job;
use log::debug;
use std::fmt;
use tokio::runtime::{Builder, Runtime};

pub struct JobManager {
    cmds: Vec<Job>,
    nb_thread: Option<usize>,
    dry_run : bool,
    keep_order : bool
}

impl fmt::Display for JobManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _r = write!(f, "JobManager with \n\t{:?} threads \n\tdry-run : {} \n\tkeep-order : {}", self.nb_thread, self.dry_run, self.keep_order);
        for i in 0..self.cmds.len() {
            let _r = write!(f, "\n\t{}", self.cmds[i]);
        }
        Ok(())
    }
}

impl JobManager {
    pub fn new() -> JobManager {
        JobManager {
            cmds: vec![],
            nb_thread: None,
            dry_run : false,
            keep_order : false
        }
    }

    pub fn add_job(&mut self, job: Job) {
        self.cmds.push(job);
    }

    pub fn set_exec_env (&mut self, nb : Option<usize>, d_r : bool, k_o : bool) {
        self.nb_thread = nb;
        self.dry_run = d_r;
        self.keep_order = k_o;
    }

    pub fn exec(&mut self) {
        if self.dry_run {
            self.dry_run();
        }else{
            self.exec_all();
        }
    }

    fn dry_run(&mut self){
        for i in 0..self.cmds.len() {
            println!("{}", self.cmds[i]);
        }
    }

    fn exec_all(&mut self){
        let mut runtime_builder: Builder = Builder::new_multi_thread();
        runtime_builder.enable_all();
        let runtime: Runtime = match self.nb_thread {
            Some(n) => runtime_builder.worker_threads(n).build().unwrap(),
            None => runtime_builder.build().unwrap(),
        };

        runtime.block_on(async {
            debug!("start block_on");

            if self.keep_order {
                for i in 0..self.cmds.len() {
                    let _r = self.cmds[i].exec().await;
                }
            }else{
                for i in 0..self.cmds.len() {
                    let _r = self.cmds[i].exec();
                }
            }

            debug!("stop block_on");
        });
    }
}
