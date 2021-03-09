extern crate tokio;
use log::debug;
use std::fmt;
use tokio::process::{Child, Command};

pub struct Job {
    cmd: String,
    parameter: Vec<String>,
}

impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _r = write!(
            f,
            r"Job : {} {}",
            self.cmd,
            self.parameter
                .iter()
                .fold(String::new(), |acc, arg| acc + " " + arg)
        );
        Ok(())
    }
}

impl Job {
    pub fn new(args: Vec<String>) -> Job {
        let cmd = args[0].clone();
        let mut parameter: Vec<String> = vec![];
        for i in 1..args.len() {
            parameter.push(args[i].clone());
        }

        Job { cmd, parameter }
    }

    pub async fn exec(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut command: Command = Command::new(self.cmd.clone());
        for arg in &self.parameter {
            command.arg(&arg.clone());
        }

        let mut _child: Child = command.spawn()?;
        debug!("<{}> spawn", self);
        // let stdout = child.stdout.take().unwrap();

        // debug!("stderr of {:?} : {:?}", self.cmd, stdout);

        Ok(())
    }
}
