// #![feature(external_doc)] // for #[doc(include="../README.md")] line 59
use crate::core::interpretor;
use crate::core::jobmanager::JobManager;
use crate::core::parser;
use log::debug;
use std::process;

/**
 * Entry point of the parallel program :
 * - `job_manager: JobManager` - allows you to manage the execution of the command
 * - `command: String` - list of parameters that we concatenate into a single String
 * # Example
 * ```rust
 * use rust_parallel::parallel::Parallel;
 * let args: Vec<String> = vec![
 *           String::from("echo"),
 *           String::from(":::"),
 *           String::from(":::"),
 *           String::from("Hello"),
 *           String::from("World"),];
 * let mut prg = Parallel::new(String::from("/bin/bash"),args);
 * prg.start();
 * ```
 */
pub struct Parallel {
    job_manager: JobManager,
    command: String,
}

impl Parallel {
    /**
     * Returns the structure allowing the parsing and the execution of the command.
     * Initialize the job manager.
     * # Attributs
     * - `shell: String` - the shell from the given environment, will be used to launch jobs
     * - `args: Vec<String>` - word table representing the command to be executed
     */
    pub fn new(shell: String, args: Vec<String>) -> Parallel {
        if args.len() == 0 {
            Parallel::print_usage();
            process::exit(1);
        }

        let job_manager: JobManager = JobManager::new(shell);
        let mut command = String::from("");
        for arg in args {
            command.push_str(&arg);
            command.push(' ');
        }

        Parallel {
            job_manager,
            command,
        }
    }

    /**
     * Display help.
     */
    // #[doc(include="../README.md")] // currently not available
    fn print_usage() {
        println!("RUST PARALLEL");
        println!("\nUSAGE:");
        println!("\trust_parallel [options] [command [arguments | {{[n]}}]] ::: values");

        println!("\nOPTIONS :");
        print!("\t--help ");
        println!("\t\t\tdisplay this message");

        print!("\t--dry-run");
        println!("\t\tdisplay the jobs without executing them");

        print!("\t--server PORT");
        println!("\t\tlaunch as a remote executor machine listening on PORT");

        print!("\t--client IP_DST PORT");
        println!("\tlaunch all the jobs remotly on machine IP_DST:PORT");

        print!("\t--keep-order ");
        println!("\t\tallow to display the returns of the commands in the execution order given in input");

        print!("\t--jobs NB / -j NB");
        println!("\tthe number of threads (NB) to be used (0 = unlimited)");

        print!("\t--pipe ");
        println!("\t\t\tis not yet implemented");

        println!("\nEXAMPLES :");
        println!("\tparallel echo ::: a b c ::: 1 2 3");
        println!("\tparallel echo {} {}::: a b c ::: 1 2 3", "{2}", "{1}");
        print!("\n\n");
    }

    /**
     * Parse the input command and configure the job manager with all the commands and execution options.
     */
    pub fn start(mut self) -> Option<Vec<String>> {
        // first let's store our request
        self.job_manager.set_request(self.command.clone());

        // let's try to parse our command
        let mut result = match parser::parse(self.command.as_str()) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Error : {}", error);
                Parallel::print_usage();
                process::exit(1);
            }
        };

        // now that the command has been parse, we give the result to create jobs
        match interpretor::interpret(&mut self.job_manager, &mut result) {
            Err(error) => {
                match error {
                    interpretor::InterpretError::Help => (),
                    interpretor::InterpretError::NoData(string) => eprintln!("{}", string),
                    interpretor::InterpretError::BothSourceAndRemote(string) => {
                        eprintln!("{}", string)
                    }
                }
                Parallel::print_usage();
                process::exit(1);
            }
            _ => (),
        }

        // If everything is OK, we tell the jobmanager to start the execution.
        debug!("Parallel starts with => {}", self.job_manager);
        return self.job_manager.exec();
    }
}
