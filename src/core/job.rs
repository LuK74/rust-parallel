extern crate tokio;
use tokio::process::Command;
use tokio::runtime;

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

        println!("job cmd : {:?}", cmd);

        Job {
            cmd,
            parameter
        }
    }

    pub fn exec(&mut self) {
        let mut command : Command = Command::new(self.cmd.clone());
        for arg in &self.parameter {
            command.arg(&arg.clone());
        }

        runtime::Runtime::new().unwrap().block_on(async {
            command.spawn().expect("failed to spawn");
        });
    }
}