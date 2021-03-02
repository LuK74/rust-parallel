extern crate tokio;
use tokio::process::{Command, Child};


pub struct Job {
    cmd : String,
    parameter : Vec<String>,
}

impl Job {
    pub fn new(args : Vec<String>) -> Job{
        let cmd = args[0].clone();
        let mut parameter :  Vec<String> = vec!();
        for i in 1..args.len() {
            parameter.push(args[i].clone());
        }

        Job {
            cmd,
            parameter
        }
    }

    pub async fn exec(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("test");
        let mut command : Command = Command::new(self.cmd.clone());
        for arg in &self.parameter {
            command.arg(&arg.clone());
        }
        
        let mut child : Child = command.spawn().unwrap();
        let stdout = child.stdout.take().unwrap();

        println!("stderr of ls: {:?}", stdout);

        Ok(())
    }
}