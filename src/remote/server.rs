use tokio::io::Ready;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::error::Error;

use crate::remote::channel::*;

pub async fn test_exchange() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("Server opened on port 4000");

    waiting_request(listener).await;
    Ok(())
}

pub async fn waiting_request(listener: TcpListener) {
    loop {
        //let (mut socket, _) : (tokio::net::TcpStream, std::net::SocketAddr);

        if let Ok((s, _)) = listener.accept().await {
            //socket = s;
            tokio::spawn(async move {
                println!("New worker has been created");
                ParallelWorker::process(s);
            });

            continue;
        } else {
            panic!("Error while accepting connection request");
        }
    }
}

struct ParallelWorker {
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

    pub fn process(socket : TcpStream) {
        let mut worker = ParallelWorker::new();
        worker.start_worker(socket);
    }

    pub fn start_worker(&mut self, socket : TcpStream) {
        let mut channel : Channel = Channel::new(socket);

        channel.set_listener(self);
        channel.set_interest(Ready::READABLE);

        println!("Worker is going to enter the main loop");

        // Should spawn a thread to do this work
        // Result isn't correctly handled yet
        channel.exchange_loop().unwrap();
        
        //self.send_request(&mut channel);
    }
}

impl ChannelListener for ParallelWorker {
    fn received(&mut self, buffer: Vec<u8>) -> Option<Vec<u8>> {
        println!("Request data : {:?}", buffer);

        self.request = String::from_utf8(buffer).unwrap();
        
        // handle the request and send it
        
        // put the request result in this variable
        self.request_result = String::from("Received !");
        
        println!("Message : {}", self.request);
        Some(self.request_result.bytes().collect())
    }

    fn sent(&mut self) -> Option<()> {
        println!("Result of request has been sent");
        None
    }
}
