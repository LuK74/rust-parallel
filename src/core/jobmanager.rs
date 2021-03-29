use super::job::Job;
use log::debug;
use std::fmt;
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;
use std::thread;
use std::process;
use tokio::sync::mpsc;
use futures::future;

/**
 * Representation of the command execution environment :
 * - `cmds : Vec<Job>` - the list of commands to be executed 
 * - `nb_thread : Option<usize>` - the number of threads to be used in the execution environment
 * - `dry_run : bool` - execution parameter allowing only to display the commands without executing them
 * - `keep_order : bool` - execution parameter allowing to display the returns of the commands in the execution order given in input
 * # Example
 * ```rust
 * use rust_parallel::core::jobmanager::JobManager;
 * let mut jobmanager : JobManager = Jobmanager::new();
 * jobmanager.set_exec_env(Some(5), false, true); //5 threads, no "dry run", keep order
 * let args: Vec<String> = vec![
 *             String::from("echo"),
 *             String::from("Hello"),
 *             String::from("World"),
 *         ];
 * jobmanager.add_job(Job::new(args));
 * jobmanager.exec()
 * ```
 */
pub struct JobManager {
    cmds: Vec<Job>,
    nb_thread: Option<usize>,
    dry_run : bool,
    keep_order : bool
}

/***
 * Allow to display all information about the current job manager. 
 */
impl fmt::Display for JobManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _r = write!(f, "Runtime with {:?} threads", self.nb_thread);
        for i in 0..self.cmds.len() {
            let _r = write!(f, "\n\t{}", self.cmds[i]);
        }
        Ok(())
    }
}

impl JobManager {
    /**
     * Return a new job manager with default values of the execution parameters.
     * # Attributs
     * - `cmds` - is initialized and empty
     * - `nb_thread` - None
     * - `dry_run` - false
     * - `keep_order` - false
     */
    pub fn new() -> JobManager {
        JobManager {
            cmds: vec![],
            nb_thread: None,
            dry_run : false,
            keep_order : false
        }
    }

    /**
     * Allows to add the given job to the current job manager.
     * # Arguments 
     * - `job` - A Job you want to add
     */
    pub fn add_job(&mut self, job: Job) {
        self.cmds.push(job);
    }

    /**
     * Allows to change the default values of the execution parameters.
     * # Arguments
     * - `nb` - to change the number of thread
     * - `d_r` - to change *dry run* value
     * - `k_o` - to change *keep order* value
     * # Additional information
     * `d_r` and `k_o` can take true|false values.
     * `nb` can take Some(uzise)|None values but in case of Some(0) the value realy given to the manager is None
     */
    pub fn set_exec_env (&mut self, nb : Option<usize>, d_r : bool, k_o : bool) {
        self.nb_thread = match nb {
            None|Some(0) => None,
            Some(_) => nb
        };
        self.dry_run = d_r;
        self.keep_order = k_o;
    }

    /**
     * Allows to execute all the commands in according to the execution parameters.
     * 
     * In case dry run is requested, then the other parameters are not very useful, we only display the commands.
     */
    pub fn exec(&mut self) {
        if self.dry_run {
            self.dry_run();
        }else{
            self.exec_all();
        }
    }

    /**
     * Private function.
     * 
     * Display the list of command.
     */
    fn dry_run(&mut self){
        for i in 0..self.cmds.len() {
            println!("{}", self.cmds[i]);
        }
    }

    /**
     * Private function.
     * 
     * Execute the list of command (with the requested number of threads), gives them an order 
     * and asynchronously retrieve the standard output of the threads in order to display them 
     * (using the order if requested)
     */
    fn exec_all(&mut self){
        debug!("{} {:?}",process::id(), thread::current().id());
        
        // Create a asynchronous tokio runtime with the given number of thread.
        // Threads work as consumer producers
        let mut runtime_builder: Builder = Builder::new_multi_thread();
        runtime_builder.enable_all();
        let runtime: Runtime = match self.nb_thread {
            None => runtime_builder.build().unwrap(),
            Some(n) => runtime_builder.worker_threads(n).build().unwrap(),
        };

        let nb_cmd = self.cmds.len();

        // Run all command into the runtime previously created.
        runtime.block_on(async {
            debug!("{} {:?}",process::id(), thread::current().id());
            debug!("start block_on");

            // Allows to keep access to the different tasks given to the threads.
            let mut tasks : Vec<JoinHandle<_>> = vec![];

            // mpsc = multi producer single consumer
            // allow to the main thread to retrieve the output of child threads
            let (tx, mut rx) = mpsc::channel::<(i32,Result<process::Output, std::io::Error>)>(1);

            // allows to keep the execution order
            let mut order : i32 = 0;

            // for each command/job
            for mut cmd in self.cmds.drain(..){
                // create new producer
                let tx_task = tx.clone();

                // gives a new task to the runtime which executes the command and get output asynchronoulsy
                let task = tokio::spawn(async move {
                    let output = cmd.exec().await;
                    tx_task.send((order,output)).await.unwrap();
                });
                tasks.push(task);
                order+=1;
            }

            // allows to wait for the output of all commands and to store them 
            // either in the order of arrival or in the order of execution (if requested => keep order)
            let mut counter : usize = 0;
            let mut messages = vec![Default::default(); nb_cmd]; // create Vector with default value
            while counter < nb_cmd {
                let (order, result) = rx.recv().await.unwrap();
                let message: String = match result {
                    // if the command is correct
                    Ok(output) => {
                        // if the command was executed successfully
                        if output.status.success() {
                            String::from_utf8(output.stdout.clone()).unwrap()
                        }else{
                            String::from_utf8(output.stderr.clone()).unwrap()
                        }
                    },
                    // the command is uncorrect
                    Err(e) => e.to_string()
                };

                if self.keep_order {
                    messages[order as usize] = message;
                }else{
                    messages[counter] = message;
                }
                counter += 1;
            }

            // display output message
            for i in 0..messages.len() {
                println!("GOT = {}", messages[i]);
            }

            future::join_all(tasks).await;

            debug!("stop block_on");
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    static NB_THREAD : Option<usize> = Some(5);

    fn init_jm (nb : Option<usize>, d_r : bool, k_o : bool) -> JobManager {
        let mut jobmanager = JobManager::new();
        jobmanager.set_exec_env(nb, d_r, k_o);

        jobmanager
    }

    #[test]
    fn test_echo1() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut jobmanager = init_jm(NB_THREAD, false, false);

        let args: Vec<String> = vec![
            String::from("echo"),
            String::from("-e"),
            String::from("'Hello\nWorld'"),
        ];
        jobmanager.add_job(Job::new(args));

        jobmanager.exec();
    }

    #[test]
    fn test_echo2() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut jobmanager = init_jm(NB_THREAD, false, true);

        let args: Vec<String> = vec![
            String::from("sleep"),
            String::from("5"),
        ];
        jobmanager.add_job(Job::new(args));

        let args: Vec<String> = vec![
            String::from("echo"),
            String::from("Hello"),
            String::from("World"),
        ];
        jobmanager.add_job(Job::new(args));


        let args: Vec<String> = vec![
            String::from("echo"),
            String::from("-e"),
            String::from("'Hello\nWorld'"),
        ];
        jobmanager.add_job(Job::new(args));

        jobmanager.exec();
    }

    fn init (nb_thread : Option<usize>) -> (JobManager, Runtime) {
        let jobmanager = init_jm(nb_thread, false, false);

        let mut runtime_builder: Builder = Builder::new_multi_thread();
        runtime_builder.enable_all();
        let runtime: Runtime = match nb_thread {
            None => runtime_builder.build().unwrap(),
            Some(n) => runtime_builder.worker_threads(n).build().unwrap(),
        };

        (jobmanager,runtime)
    }

    #[test]
    fn jobmanager_thread_worker() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (_jobmanager,runtime) = init(NB_THREAD);

        runtime.block_on(async {
            debug!("start block_on");
            
            for i in 0..10 {
                debug!("{} {:?}", process::id(), thread::current().id());
                let _task = tokio::spawn(async move {
                    debug!("{} {} {:?}",i,process::id(), thread::current().id());
                });
            }

            debug!("stop block_on");
        });

        debug!("end test");
    }

    #[test]
    fn jobmanager_thread_worker2() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (_jobmanager,runtime) = init(NB_THREAD);

        runtime.block_on(async {
            debug!("start block_on");
            
            for i in 0..5 {
                let _task = tokio::spawn(async move {
                    debug!("{}.{} | {:?}",i,1, thread::current().id());
                    sleep(Duration::from_secs(2));
                    debug!("{}.{}",i,2);
                });
            }

            debug!("stop block_on");
        });

        debug!("end test");
    }

    #[test]
    fn jobmanager_thread_worker3() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (_jobmanager,runtime) = init(NB_THREAD);

        runtime.block_on(async {
            debug!("start block_on");
            
            for i in 0..5 {
                let task = tokio::spawn(async move {
                    debug!("{}.{} | {:?}",i,1, thread::current().id());
                    sleep(Duration::from_secs(3));
                    debug!("{}.{}",i,2);
                });
                task.await.unwrap();
            }

            debug!("stop block_on");
        });

        debug!("end test");
    }

    #[test]
    fn jobmanager_thread_worker4() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (_jobmanager,runtime) = init(NB_THREAD);

        runtime.block_on(async {
            debug!("start block_on");

            let mut tasks : Vec<tokio::task::JoinHandle<()>> = vec![];
            
            for i in 0..5 {
                let task = tokio::spawn(async move {
                    debug!("{}.{} | {:?}",i,1, thread::current().id());
                    sleep(Duration::from_secs(3));
                    debug!("{}.{}",i,2);
                });
                tasks.push(task);
            }

            // for task in tasks.drain(..){
            //     task.await.unwrap();
            // }

            futures::future::join_all(tasks).await;

            debug!("stop block_on");
        });

        debug!("end test");
    }

    #[test]
    fn test_notpanick() {
        let _ = env_logger::builder().is_test(true).try_init();
    
        let mut jobmanager = init_jm(NB_THREAD, false, false);
    
        let args: Vec<String> = vec![
            String::from("echo"),
            String::from("-e"),
            String::from("'Hello\nWorld'"),
        ];
        jobmanager.add_job(Job::new(args));
    
        let args: Vec<String> = vec![
            String::from("unknown"),
        ];
        jobmanager.add_job(Job::new(args));
    
        jobmanager.exec();
    }
}
