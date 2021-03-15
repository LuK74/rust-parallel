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
    let mut listener = TcpListener::bind("127.0.0.1:4000").await?;
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

pub async fn waiting_request(listener: TcpListener) -> Result<(), ()> {
    loop {
        //let (mut socket, _) : (tokio::net::TcpStream, std::net::SocketAddr);

        if let Ok((s, _)) = listener.accept().await {
            //socket = s;
            tokio::spawn(async move {
                let mut worker = ParallelWorker::new(s);
                //worker.exchange_loop();
            });

            continue;
        } else {
            panic!("Error while accepting connection request");
        }
    }

    Ok(())
}

struct ParallelWorker {
    request: String,

    channel: Channel,
}

impl ParallelWorker {
    pub fn new(socket: TcpStream) -> Self {
        let worker : ParallelWorker = ParallelWorker{
            request: String::new(),

            channel: Channel::new(socket),
        };
        worker.channel.setListener(Rc::new(worker));
        return worker;
    }
}

impl ChannelListener for ParallelWorker {
    fn received(&self, buffer: Vec<u8>) {
        println!("Request data : {:?}", buffer);
        // Here we'll need to call the parralel application
    }

    fn sent(&self) {
        println!("Result of request has been sent");
    }
}
