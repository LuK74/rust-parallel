extern crate tokio;
use log::debug;
use std::fmt;
use tokio::process::{Child, Command};
use std::thread;
use std::process;
use std::process::Stdio;

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
        debug!("{} {:?}",process::id(), thread::current().id());
        let mut command: Command = Command::new(self.cmd.clone());
        for arg in &self.parameter {
            command.arg(&arg.clone());
        }

        // let mut _child: Child = command.spawn()?;
        // debug!("<{}> spawn", self);

        let mut child : Child = command.stdout(Stdio::piped()).spawn()?;
        let mut _stdout = child.stdout.take().unwrap();
        // let result: Vec<_> = io::BufReader::new(stdout)
        // .lines
        // .inspect(|s| println!("> {:?}", s))
        // .collect();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::{Builder, Runtime};

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