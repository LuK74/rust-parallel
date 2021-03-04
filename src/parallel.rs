use crate::core::jobmanager::JobManager;
use crate::core::job::Job;

pub struct Parallel {
    job_manager : JobManager
}

impl Parallel {
    pub fn new() -> Parallel {  
        let job_manager: JobManager = JobManager::new();

        Parallel {job_manager}
    }

    pub fn new_cmd(&mut self, args : Vec<String>){
        // println!("{:?}", args);
        self.job_manager.add_job(Job::new(args))
    }

    pub fn start(&mut self) {
        println!("{}", self.job_manager);
        self.job_manager.exec_all();
    }
}