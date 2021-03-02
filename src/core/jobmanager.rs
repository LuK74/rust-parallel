use super::job::Job;

pub struct JobManager {
    cmds : Vec<Job>
}

impl JobManager {
    pub fn new() -> JobManager {
        JobManager{cmds : vec!()}
    }

    pub fn add_job (&mut self, job : Job) {
        self.cmds.push(job);
    }

    pub fn exec_all(&mut self) {
        for i in 0..self.cmds.len() {
            self.cmds[i].exec();
        }
    }
}