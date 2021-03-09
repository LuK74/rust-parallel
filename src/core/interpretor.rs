



pub struct Interpretor {
    jobMan: JobManager,
}

impl Interpretor {

    pub fn new(jm: JobManager) -> Interpretor {
        Interpretor { jobMan: jm }
    }
}