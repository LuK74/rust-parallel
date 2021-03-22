extern crate tokio;
use log::debug;
use std::fmt;
use tokio::process::Command;
use std::thread;
use std::process;

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

    pub async fn exec(&mut self) -> Result<process::Output, Box<dyn std::error::Error>> {
        debug!("{} {:?}",process::id(), thread::current().id());
        let mut command: Command = Command::new(self.cmd.clone());
        for arg in &self.parameter {
            command.arg(&arg.clone());
        }

        let future = command.output();
        debug!("<{}> spawn", self);
        let output : process::Output = future.await?;

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::{Builder, Runtime};
    use tokio::process::Child;

    fn init (nb_thread : Option<usize>) -> Runtime {
        let mut runtime_builder: Builder = Builder::new_multi_thread();
        runtime_builder.enable_all();
        let runtime: Runtime = match nb_thread {
            None => runtime_builder.build().unwrap(),
            Some(n) => runtime_builder.worker_threads(n).build().unwrap(),
        };

        runtime
    }

    #[test]
    fn job_cmd1() {
        let _ = env_logger::builder().is_test(true).try_init();

        let runtime = init(Some(5));

        runtime.block_on(async {
            let _cmd : Child = Command::new(String::from("echo"))
                .arg(String::from("Hello World"))
                .spawn()
                .unwrap();
        });
    }

    #[test]
    #[should_panic]
    fn job_cmd2() {
        let _ = env_logger::builder().is_test(true).try_init();

        let runtime = init(Some(5));

        runtime.block_on(async {
            let _cmd : Child = Command::new(String::from("echo Hello World"))
                .spawn()
                .unwrap();
        });

    }
}