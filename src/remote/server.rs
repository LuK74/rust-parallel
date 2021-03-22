use tokio::io::Ready;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::error::Error;

use std::fs::{self, DirBuilder, File};
use std::io::Write;
use std::fs::OpenOptions;



use crate::remote::channel::*;

pub struct ParallelServer {
    listener: TcpListener,
}

impl ParallelServer {
    pub async fn new(server_address: String) -> Self {
        let res_bind = TcpListener::bind(server_address.clone()).await;
        let listener: TcpListener;

        if let Ok(l) = res_bind {
            listener = l;
        } else {
            panic!("Couldn't bind the server to the address {}", server_address);
        }

        if let Err(e) = DirBuilder::new().create("tmp") {
            println!("{:?}", e);
        } else {
            assert!(fs::metadata("tmp").unwrap().is_dir());
        }

        ParallelServer { listener }
    }

    pub async fn waiting_request(&mut self) {
        loop {
            if let Ok((s, _)) = self.listener.accept().await {
                tokio::spawn(async {
                    ParallelWorker::process(s).unwrap();
                });

                continue;
            } else {
                panic!("Error while accepting connection request");
            }
        }
    }
}

pub async fn test_exchange() -> Result<(), Box<dyn Error>> {
    let mut server: ParallelServer = ParallelServer::new(String::from("127.0.0.1:4000")).await;
    println!("Server opened on port 4000");

    server.waiting_request().await;
    Ok(())
}

pub enum WorkerState {
    Idle,
    WaitingFileName,
    WaitingFileData,
    WaitingRequest,
    SendingResult,
}

pub struct ParallelWorker {
    request_result: String,
    request: String,

    tmp_dir: String,
    files: Vec<(String, String)>,

    state: WorkerState,
    current_file: String,
}

impl ParallelWorker {
    pub fn new() -> Self {
        ParallelWorker {
            request_result: String::new(),
            request: String::new(),
            tmp_dir: "tmp/".to_string(),
            files: Vec::new(),
            state: WorkerState::Idle,
            current_file: String::new(),
        }
    }

    pub fn process(socket: TcpStream) -> Result<String, String> {
        let mut worker = ParallelWorker::new();
        let result_work = worker.start_worker(socket);

        if let Ok(res) = result_work {
            return Ok(res);
        } else {
            println!("Error occured during worker job");
            return result_work;
        }
    }

    pub fn start_worker(&mut self, socket: TcpStream) -> Result<String, String> {
        let mut channel: Channel = Channel::new(socket);

        channel.set_listener(self);
        channel.set_interest(Ready::READABLE);

        // Should spawn a thread to do this work
        // Result isn't correctly handled yet
        channel.exchange_loop().unwrap();

        Ok(self.request_result.clone())
    }
}

impl ChannelListener for ParallelWorker {
    fn received(&mut self, buffer: Vec<u8>) -> Option<Vec<u8>> {
        let response = String::from_utf8(buffer).unwrap();
        //println!("-- Server received : {}", response);

        if response.eq("Sending files") {
            self.state = WorkerState::WaitingFileName;
            return Some("Waiting for new file".bytes().collect());
        } else if response.eq("No more files") || response.eq("Sending request") {
            self.state = WorkerState::WaitingRequest;
            return Some("Ready for request".bytes().collect());
        } else {
            println!("changing State");
            match &self.state {
                WorkerState::Idle => {
                    println!("Unexpected behaviour");
                }
                WorkerState::WaitingFileName => {
                    self.current_file = response;
                    self.state = WorkerState::WaitingFileData;
                    return Some("Ready for next file".bytes().collect());
                }
                WorkerState::WaitingFileData => {
                    let mut extension = String::from("");
                    let mut current_number = 0;

                    let mut result = OpenOptions::new().write(true)
                    .create_new(true)
                    .open(self.tmp_dir.clone() + &self.current_file + &extension[..]);

                    while let Err(e) = &result {
                        if e.kind() == std::io::ErrorKind::AlreadyExists {
                            current_number += 1;
                            extension = format!("({})", current_number);
                            result = OpenOptions::new().write(true)
                    .create_new(true)
                    .open(self.tmp_dir.clone() + &self.current_file + &extension[..]);
                        } else {
                            println!("{:?}",e);
                            panic!("Unexpected behaviour");
                        }
                    }

                    let mut file = result.unwrap();

                    let buf : &[u8] = &response.bytes().collect::<Vec<u8>>()[..];

                    file.write_all(buf).unwrap();
                    file.sync_data().unwrap();

                    self.files.push((
                        self.current_file.clone(),
                        self.tmp_dir.clone() + "/" + &self.current_file + &extension[..],
                    ));
                    self.state = WorkerState::WaitingFileName;
                    return Some("Waiting for new file".bytes().collect());
                }
                WorkerState::WaitingRequest => {
                    self.request = response;
                    // handle the request and send the result

                    // put the request result in this variable
                    println!("Server : request received : {}", self.request);
                    self.request_result = self.request.clone();
                    println!("Server : result of request : {}", self.request_result);

                    self.state = WorkerState::SendingResult;
                    return Some(self.request_result.bytes().collect());
                },
                _ => panic!("Shoudln't happened"),
            }

            return None;
        }
    }

    fn sent(&mut self) -> Option<()> {
        match &self.state {
            WorkerState::SendingResult => return None,
            _ => return Some(())
        }
    }
}
