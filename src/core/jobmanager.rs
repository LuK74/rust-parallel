use super::job::Job;
use log::debug;
use std::fmt;
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;
use std::thread;
use std::process;
use tokio::sync::mpsc;
use futures::future;

pub struct JobManager {
    cmds: Vec<Job>,
    nb_thread: Option<usize>,
    dry_run : bool,
    keep_order : bool
}

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
    pub fn new() -> JobManager {
        JobManager {
            cmds: vec![],
            nb_thread: None,
            dry_run : false,
            keep_order : false
        }
    }

    pub fn add_job(&mut self, job: Job) {
        self.cmds.push(job);
    }

    pub fn set_exec_env (&mut self, nb : Option<usize>, d_r : bool, k_o : bool) {
        self.nb_thread = nb;
        self.dry_run = d_r;
        self.keep_order = k_o;
    }

    pub fn exec(&mut self) {
        if self.dry_run {
            self.dry_run();
        }else{
            self.exec_all();
        }
    }

    fn dry_run(&mut self){
        for i in 0..self.cmds.len() {
            println!("{}", self.cmds[i]);
        }
    }

    fn exec_all(&mut self){
        debug!("{} {:?}",process::id(), thread::current().id());
        
        let mut runtime_builder: Builder = Builder::new_multi_thread();
        runtime_builder.enable_all();
        let runtime: Runtime = match self.nb_thread {
            None => runtime_builder.build().unwrap(),
            Some(n) => runtime_builder.worker_threads(n).build().unwrap(),
        };

        let nb_cmd = self.cmds.len();

        runtime.block_on(async {
            debug!("{} {:?}",process::id(), thread::current().id());
            debug!("start block_on");

            let mut tasks : Vec<JoinHandle<_>> = vec![];
            let (tx, mut rx) = mpsc::channel::<(i32,process::Output)>(1);
            let mut order : i32 = 0;
            for mut cmd in self.cmds.drain(..){
                let tx_task = tx.clone();
                let task = tokio::spawn(async move {
                    let res = cmd.exec().await.unwrap();
                    tx_task.send((order,res)).await.unwrap();
                });
                tasks.push(task);
                order+=1;
            }

            let mut counter : usize = 0;
            let mut messages = vec![Default::default(); nb_cmd];
            while counter < nb_cmd {
                let (order, message) = rx.recv().await.unwrap();
                if self.keep_order {
                    debug!("order {}", order);
                    messages.insert(order as usize, String::from_utf8(message.stdout.clone()).unwrap());
                }else{
                    messages.insert(counter, String::from_utf8(message.stdout.clone()).unwrap());
                }
                counter += 1;
            }

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

    fn init_jm (nb : Option<usize>, d_r : bool, k_o : bool) -> JobManager {
        let mut jobmanager = JobManager::new();
        jobmanager.set_exec_env(nb, d_r, k_o);

        jobmanager
    }

    #[test]
    fn test_echo1() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut jobmanager = init_jm(Some(1), false, false);

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

        let mut jobmanager = init_jm(Some(1), false, true);

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

    static NB_THREAD : Option<usize> = Some(5);

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
}
