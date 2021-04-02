extern crate tokio;
use log::debug;
use std::fmt;
use tokio::process::Command;
use std::thread;
use std::process;

/**
 * Representation of the command execution environment :
 * - `cmd : String` - linux command name
 * - `parameter: Vec<String>` - list of command parameters
 * # Example
 * ```rust
 * use rust_parallel::core::job::Job;
 * let args: Vec<String> = vec![
 *             String::from("echo"),
 *             String::from("Hello"),
 *             String::from("World"),
 *         ];
 * let job : Job = Job::new(args)
 * job.exec();
 * ```
 */
pub struct Job {
    cmd: String,
    parameter: Vec<String>,
}

/***
 * Allow to display all information about the current job. 
 */
impl fmt::Display for Job {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _r = write!(
            f,
            r"{}{}",
            self.cmd,
            self.parameter
                .iter()
                .fold(String::new(), |acc, arg| acc + " " + arg)
        );
        Ok(())
    }
}

impl Job {
    /**
     * Return a new job set with the given parameter.
     * # Attributs
     * - `args: Vec<String>` - list of word use in the command
     */
    pub fn new(args: Vec<String>) -> Job {
        // the first element of the list is the linux command name
        let cmd = args[0].clone();

        // the others element of the list are the command parameters
        let mut parameter: Vec<String> = vec![];
        for i in 1..args.len() {
            parameter.push(args[i].clone());
        }

        Job { cmd, parameter }
    }

    /**
     * Execute the current job.
     * # Return
     * either the standard output if the execution was successful or an error.
     */
    pub async fn exec(&mut self) -> Result<process::Output, std::io::Error> {
        debug!("{} {:?}",process::id(), thread::current().id());

        // Create a new tokio command with the linux command name
        let mut command: Command = Command::new(self.cmd.clone());

        // Add parameters to the command
        for arg in &self.parameter {
            command.arg(&arg.clone());
        }

        // A future is a value that may not have finished computing yet. 
        // This kind of "asynchronous value" makes it possible for a thread to continue doing useful work 
        // while it waits for the value to become available.
        let future = command.output();
        debug!("<{}> spawn", self);

        // Wait for the result of the command execution
        let future_result = future.await;
        match future_result {
            Err(e) => return Err(e),
            Ok(o) => return Ok(o),
        }

        // if there was no error during execution then the output is returned
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

    #[test]
    #[should_panic]
    fn job_cmd3() {
        let _ = env_logger::builder().is_test(true).try_init();

        let runtime = init(Some(5));

        runtime.block_on(async {
            let _cmd : Child = Command::new(String::from("unknown_cmd"))
                .spawn()
                .unwrap();
        });
    }
}