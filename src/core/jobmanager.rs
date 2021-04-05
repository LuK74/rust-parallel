use super::job::Job;
use crate::remote::client::ParallelClient;
use crate::remote::server::ParallelServer;
use futures::future;
use log::debug;
use std::fmt;
use std::process;
use std::thread;
use tokio::runtime::Handle;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

/**
 * Representation of the command execution environment :
 * - `cmds : Vec<Job>` - the list of commands to be executed
 * - `nb_thread : Option<usize>` - the number of threads to be used in the execution environment
 * - `dry_run : bool` - execution parameter allowing only to display the commands without executing them
 * - `keep_order : bool` - execution parameter allowing to display the returns of the commands in the execution order given in input
 * # Example
 * ```rust
 * use rust_parallel::core::jobmanager::JobManager;
 * use rust_parallel::core::job::Job;
 * let mut jobmanager : JobManager = JobManager::new(String::from("/bin/bash"));
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
    pub shell: String, //the shell used to launch jobs
    cmds: Vec<Job>,
    nb_thread: Option<usize>,
    dry_run: bool,
    keep_order: bool,
    local_port: Option<usize>,
    remote_addr: Option<(String, usize)>,
    request: String,
}

/***
 * Allow to display all information about the current job manager.
 */
impl fmt::Display for JobManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _r = write!(
            f,
            "JobManager with \n\t{:?} threads \n\tdry-run : {} \n\tkeep-order : {}",
            self.nb_thread, self.dry_run, self.keep_order
        );
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
    pub fn new(shell: String) -> JobManager {
        JobManager {
            shell: shell,
            cmds: vec![],
            nb_thread: None,
            dry_run: false,
            keep_order: false,
            remote_addr: None,
            local_port: None,
            request: String::new(),
        }
    }

    /**
     * Set the request field with the given string
     */
    pub fn set_request(&mut self, request: String) {
        self.request = request;
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
    pub fn set_exec_env(
        &mut self,
        nb: Option<usize>,
        d_r: bool,
        k_o: bool,
        src_port: Option<usize>,
        dst_addr: Option<(String, usize)>,
    ) {
        self.nb_thread = match nb {
            None | Some(0) => None,
            Some(_) => nb,
        };
        self.dry_run = d_r;
        self.keep_order = k_o;
        self.local_port = src_port;
        self.remote_addr = dst_addr;
    }

    /**
     * Allows to execute all the commands in according to the execution parameters.
     *
     * In case dry run is requested, then the other parameters are not very useful, we only display the commands.
     */
    pub fn exec(mut self) -> Option<Vec<String>> {
        if self.dry_run {
            self.dry_run();
            return None;
        } else if let Some(ip_addr) = &self.remote_addr {
            // This case corresponds to the Client side of a remote execution
            let port_string = ip_addr.1.to_string();
            let mut address = ip_addr.0.clone();
            address.push(':');
            address.push_str(&port_string);


            // After we've collected all the information needed to launch
            // the connection, we can remove the remote execution arguments
            // from the request in order to send it to the Server
            let mut tokens: Vec<&str> = self.request.split_whitespace().collect();
            let mut index: usize = 0;

            for token in &tokens {
                if token.eq_ignore_ascii_case("--client") {
                    break;
                }
                index += 1;
            }
            tokens.remove(index);
            tokens.remove(index);
            tokens.remove(index);

            let mut new_request: String = String::new();
            for token in tokens {
                new_request.push_str(token);
                new_request.push(' ');
            }

            let rt = Runtime::new().unwrap();

            rt.block_on(async {
                let mut client: ParallelClient =
                    ParallelClient::new(String::from(address), new_request);
                let res = client.start_client().await;
                println!("Result of the request :");
                println!("{}", res.unwrap());
            });

            return None;
        } else if let Some(port_number) = self.local_port {
            // This case corresponds to the Server side of a remote execution
            let port_string = port_number.to_string();
            let mut address = String::from("127.0.0.1:");
            address.push_str(&port_string);

            let mut runtime_builder: Builder = Builder::new_multi_thread();
            runtime_builder.enable_all();
            let runtime = runtime_builder.build().unwrap();

            runtime.block_on(async {
                let mut server: ParallelServer = ParallelServer::new(address).await;
                server.waiting_request().await;
            });

            return None;
        } else {
            let messages = self.exec_all();
            return Some(messages);
        }
    }

    /**
     * Private function.
     *
     * Display the list of command.
     */
    fn dry_run(&mut self) {
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
    fn exec_all(mut self) -> Vec<String> {
        debug!("{} {:?}", process::id(), thread::current().id());



        // Check if a runtime already exists
        if let Err(_e) = Handle::try_current() {

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
            let result = futures::executor::block_on(runtime.spawn(async move {
                debug!("{} {:?}", process::id(), thread::current().id());
                debug!("start block_on");

                // Allows to keep access to the different tasks given to the threads.
                let mut tasks: Vec<JoinHandle<_>> = vec![];

                // mpsc = multi producer single consumer
                // allow to the main thread to retrieve the output of child threads
                let (tx, mut rx) =
                    mpsc::channel::<(i32, Result<process::Output, std::io::Error>)>(1);

                // allows to keep the execution order
                let mut order: i32 = 0;

                // for each command/job
                for mut cmd in self.cmds.drain(..) {
                    // create new producer
                    let tx_task = tx.clone();

                    // gives a new task to the runtime which executes the command and get output asynchronoulsy
                    let task = tokio::spawn(async move {
                        let output = cmd.exec().await;
                        tx_task.send((order, output)).await.unwrap();
                    });
                    tasks.push(task);
                    order += 1;
                }

                // allows to wait for the output of all commands and to store them
                // either in the order of arrival or in the order of execution (if requested => keep order)
                let mut counter: usize = 0;
                let mut messages = vec![Default::default(); nb_cmd]; // create Vector with default value
                while counter < nb_cmd {
                    let (order, result) = rx.recv().await.unwrap();
                    let message: String = match result {
                        // if the command is correct
                        Ok(output) => {
                            // if the command was executed successfully
                            if output.status.success() {
                                String::from_utf8(output.stdout.clone()).unwrap()
                            } else {
                                String::from_utf8(output.stderr.clone()).unwrap()
                            }
                        }
                        // the command is uncorrect
                        Err(e) => {
                            let mut msg = String::from(e.to_string());
                            msg.push_str("\n");
                            msg
                        }
                    };

                    if self.keep_order {
                        messages[order as usize] = message;
                    } else {
                        messages[counter] = message;
                    }
                    counter += 1;
                }

                // display output message
                for i in 0..messages.len() {
                    print!("{}", messages[i]);
                }

                future::join_all(tasks).await;

                debug!("stop block_on");
                return messages;
            }));

            return result.unwrap();
        } else {

            let nb_cmd = self.cmds.len();

            // Run all command into the runtime previously created.
            let res = tokio::task::spawn_blocking(move || {
                debug!("{} {:?}", process::id(), thread::current().id());
                debug!("start block_on");

                // Allows to keep access to the different tasks given to the threads.
                let mut tasks: Vec<JoinHandle<_>> = vec![];

                // mpsc = multi producer single consumer
                // allow to the main thread to retrieve the output of child threads
                let (tx, mut rx) =
                    mpsc::channel::<(i32, Result<process::Output, std::io::Error>)>(1);

                // allows to keep the execution order
                let mut order: i32 = 0;

                // for each command/job
                for mut cmd in self.cmds.drain(..) {
                    // create new producer
                    let tx_task = tx.clone();

                    // gives a new task to the runtime which executes the command and get output asynchronoulsy
                    let task = tokio::spawn(async move {
                        let output = cmd.exec().await;
                        tx_task.send((order, output)).await.unwrap();
                    });
                    tasks.push(task);
                    order += 1;
                }
                // allows to wait for the output of all commands and to store them
                // either in the order of arrival or in the order of execution (if requested => keep order)
                let mut counter: usize = 0;
                let mut messages = vec![Default::default(); nb_cmd]; // create Vector with default value
                while counter < nb_cmd {
                    let (order, result) = futures::executor::block_on(rx.recv()).unwrap();
                    let message: String = match result {
                        // if the command is correct
                        Ok(output) => {
                            // if the command was executed successfully
                            if output.status.success() {
                                String::from_utf8(output.stdout.clone()).unwrap()
                            } else {
                                String::from_utf8(output.stderr.clone()).unwrap()
                            }
                        }
                        // the command is uncorrect
                        Err(e) => {
                            let mut msg = String::from(e.to_string());
                            msg.push_str("\n");
                            msg
                        }
                    };

                    if self.keep_order {
                        messages[order as usize] = message;
                    } else {
                        messages[counter] = message;
                    }
                    counter += 1;
                }


                // display output message
                for i in 0..messages.len() {
                    print!("{}", messages[i]);
                }

                futures::executor::block_on(future::join_all(tasks));

                debug!("stop block_on");
                return messages;
            });
            // Now we wait for the task previously created to end
            return futures::executor::block_on(res).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    static NB_THREAD: Option<usize> = Some(5);

    fn init_jm(nb: Option<usize>, d_r: bool, k_o: bool) -> JobManager {
        let mut jobmanager = JobManager::new(String::from("/bin/bash"));
        jobmanager.set_exec_env(nb, d_r, k_o, None, None);

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

        let args: Vec<String> = vec![String::from("sleep"), String::from("5")];
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

    fn init(nb_thread: Option<usize>) -> (JobManager, Runtime) {
        let jobmanager = init_jm(nb_thread, false, false);

        let mut runtime_builder: Builder = Builder::new_multi_thread();
        runtime_builder.enable_all();
        let runtime: Runtime = match nb_thread {
            None => runtime_builder.build().unwrap(),
            Some(n) => runtime_builder.worker_threads(n).build().unwrap(),
        };

        (jobmanager, runtime)
    }

    #[test]
    fn jobmanager_thread_worker() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (_jobmanager, runtime) = init(NB_THREAD);

        runtime.block_on(async {
            debug!("start block_on");

            for i in 0..10 {
                debug!("{} {:?}", process::id(), thread::current().id());
                let _task = tokio::spawn(async move {
                    debug!("{} {} {:?}", i, process::id(), thread::current().id());
                });
            }

            debug!("stop block_on");
        });

        debug!("end test");
    }

    #[test]
    fn jobmanager_thread_worker2() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (_jobmanager, runtime) = init(NB_THREAD);

        runtime.block_on(async {
            debug!("start block_on");

            for i in 0..5 {
                let _task = tokio::spawn(async move {
                    debug!("{}.{} | {:?}", i, 1, thread::current().id());
                    sleep(Duration::from_secs(2));
                    debug!("{}.{}", i, 2);
                });
            }

            debug!("stop block_on");
        });

        debug!("end test");
    }

    #[test]
    fn jobmanager_thread_worker3() {
        let _ = env_logger::builder().is_test(true).try_init();

        let (_jobmanager, runtime) = init(NB_THREAD);

        runtime.block_on(async {
            debug!("start block_on");

            for i in 0..5 {
                let task = tokio::spawn(async move {
                    debug!("{}.{} | {:?}", i, 1, thread::current().id());
                    sleep(Duration::from_secs(3));
                    debug!("{}.{}", i, 2);
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

        let (_jobmanager, runtime) = init(NB_THREAD);

        runtime.block_on(async {
            debug!("start block_on");

            let mut tasks: Vec<tokio::task::JoinHandle<()>> = vec![];

            for i in 0..5 {
                let task = tokio::spawn(async move {
                    debug!("{}.{} | {:?}", i, 1, thread::current().id());
                    sleep(Duration::from_secs(3));
                    debug!("{}.{}", i, 2);
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

        let args: Vec<String> = vec![String::from("unknown")];
        jobmanager.add_job(Job::new(args));

        jobmanager.exec();
    }
}
