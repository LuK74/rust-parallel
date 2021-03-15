use std::rc::Rc;
use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
use tokio::io::ReadBuf;
use tokio::io::Ready;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::error::Error;
use std::io;

use crate::remote::channel::*;

pub async fn test() -> Result<(), Box<dyn Error>> {
    // Connection
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("Server opened on port 4000");

    let (socket, _) = listener.accept().await?;

    let mut msg = vec![0; 1024];

    loop {
        socket.readable().await.unwrap();

        match socket.try_read(&mut msg) {
            Ok(n) => {
                msg.truncate(n);
                break;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    println!("GOT = {:?}", msg);
    Ok(())
}

pub async fn test_exchange() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("Server opened on port 4000");

    waiting_request(listener).await;
    Ok(())
}

pub async fn waiting_request(listener: TcpListener) -> Result<(), ()> {
    loop {
        //let (mut socket, _) : (tokio::net::TcpStream, std::net::SocketAddr);

        if let Ok((s, _)) = listener.accept().await {
            //socket = s;
            tokio::spawn(async move {
                let mut worker = ParallelWorker::new();
                worker.start_worker(s);
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

    pub async fn start_worker(&mut self, socket : TcpStream) {
        let mut channel : Channel = Channel::new(socket);

        channel.set_listener(self);
        channel.set_interest(Interest::READABLE).await;

        // Should spawn a thread to do this work
        // Result isn't correctly handled yet
        channel.exchange_loop().await.unwrap();
        
        //self.send_request(&mut channel);
    }
}

impl ChannelListener for ParallelWorker {
    fn received(&self, buffer: Vec<u8>, channel : &mut Channel) {
        println!("Request data : {:?}", buffer);
        // Here we'll need to call the parralel application
    }

    fn sent(&self, channel : &mut Channel) {
        println!("Result of request has been sent");
        channel.close();
    }
}
