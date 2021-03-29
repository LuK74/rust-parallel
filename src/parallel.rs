use crate::core::jobmanager::JobManager;
use crate::core::interpretor;
use crate::core::parser;
use log::debug;

pub struct Parallel {
    job_manager: JobManager,
    command: String,
}

fn print_usage() {
    println!("usage below");
    //TODO
}

impl Parallel {
    pub fn new(args: Vec<String>) -> Parallel {
        let job_manager: JobManager = JobManager::new();
        let mut command = String::from("parallel");
        for arg in args {
            command.push(' ');
            command.push_str(&arg);
        }

        Parallel { job_manager, command }
    }

    pub fn start(&mut self) {
        // let's try to parse our command
        let mut result = match parser::parse(self.command.as_str()) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Error : {}", error);
                return;
            }, 
        };

        // now that the command has been parse, we give the result to create jobs
        match interpretor::interpret(&mut self.job_manager, &mut result) {
            Err(error) => {
                match error {
                    interpretor::InterpretError::Help => print_usage(),
                    interpretor::InterpretError::NoData(string) => eprintln!("{}", string),
                }
                return;
            },
            _ => (),
        }

        // If everything is OK, we tell the jobmanager to start the execution.
        debug!("Parallel starts with => {}", self.job_manager);
        self.job_manager.exec();
    }
}
