use tokio::io::Ready;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::error::Error;

use crate::remote::channel::*;

pub struct ParallelServer {
    listener : TcpListener,
}

impl ParallelServer {
    
    pub async fn new(server_address : String) -> Self {
        let res_bind =  TcpListener::bind(server_address.clone()).await;
        let listener : TcpListener;

        if let Ok(l) = res_bind {
            listener = l; 
        } else {
            panic!("Couldn't bind the server to the address {}", server_address);
        }


        ParallelServer {
            listener
        }
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
    let mut server : ParallelServer = ParallelServer::new(String::from("127.0.0.1:4000")).await;
    println!("Server opened on port 4000");
    
    server.waiting_request().await;
    Ok(())
}

pub struct ParallelWorker {
    request_result : String,
    request : String,
}

impl ParallelWorker {
    pub fn new() -> Self {
        ParallelWorker {
           request_result : String::new(),
           request : String::new(),
        }
    }

    pub fn process(socket : TcpStream) -> Result<String, String> {
        let mut worker = ParallelWorker::new();
        let result_work = worker.start_worker(socket);

        if let Ok(res) = result_work {
            return Ok(res);
        } else {
            println!("Error occured during worker job");
            return result_work;
        }
    }

    pub fn start_worker(&mut self, socket : TcpStream) -> Result<String, String> {
        let mut channel : Channel = Channel::new(socket);

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

        self.request = String::from_utf8(buffer).unwrap();
        
        // handle the request and send it
        
        // put the request result in this variable
        println!("Server : request received : {}", self.request);
        self.request_result = self.request.clone();
        println!("Server : result of request : {}", self.request_result);

        Some(self.request_result.bytes().collect())
    }

    fn sent(&mut self) -> Option<()> {
        None
    }
}
