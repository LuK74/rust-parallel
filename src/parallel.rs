// #![feature(external_doc)] // for #[doc(include="../README.md")] line 59

use crate::core::jobmanager::JobManager;
use crate::core::interpretor;
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
     * - `shell : String` - the shell from the given environment, will be used to launch jobs
     * - `args: Vec<String>` - word table representing the command to be executed
     */
    pub fn new(shell: String, args: Vec<String>) -> Parallel {
        if args.len() == 0 {
            Parallel::print_usage();
            process::exit(0);
        }

        let job_manager: JobManager = JobManager::new(shell);
        let mut command = String::from("");
        for arg in args {
            command.push_str(&arg);
            command.push(' ');
        }

        Parallel { job_manager, command }
    }

    /**
     * Display help.
     */
    // #[doc(include="../README.md")] // currently not available
    fn print_usage() {
        println!("RUST PARALLEL");
        println!("rust_parallel [options] [command [arguments | {spe} | {spe}]] (:::+ arguments)", spe="{}");
        
        println!("\nOptions :");
            println!("\t--help ");
            println!("\t-h ");
            println!("\t\tTo get more information");
            print!("\n");

            println!("\t--dry-run");
            println!("\t\tAllow to display the commands without executing them");
            print!("\n");

            println!("\t--keep-order ");
            println!("\t\tAllow to display the returns of the commands in the execution order given in input");
            print!("\n");

            println!("\t--jobs NB");
            println!("\t-j NB");
            println!("\t\tthe number of threads (NB) to be used in the execution environment");
            print!("\n");

            println!("\t--pipe ");
            println!("\t\tis not yet implemented");
            print!("\n");

        println!("\nExample :");
            println!("\tparallel echo ::: a b c ::: 1 2 3");
            println!("\tparallel echo {} {}::: a b c ::: 1 2 3", "{0}", "{1}");
            print!("\n\n");
    }

    /**
     * Parse the input command and configure the job manager with all the commands and execution options.
     */
    pub fn start(&mut self) {
        // let's try to parse our command
        let mut result = match parser::parse(self.command.as_str()) {
            Ok(result) => result,
            Err(error) => {
                eprintln!("Error : {}", error);
                Parallel::print_usage();
                process::exit(0);
            }, 
        };

        // now that the command has been parse, we give the result to create jobs
        match interpretor::interpret(&mut self.job_manager, &mut result) {
            Err(error) => {
                match error {
                    interpretor::InterpretError::Help => Parallel::print_usage(),
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
