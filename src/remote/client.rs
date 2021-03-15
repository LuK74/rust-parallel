use std::rc::Weak;
use std::rc::Rc;
use std::net::Ipv4Addr;
use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
use tokio::io::Ready;
use tokio::net::TcpStream;

use std::error::Error;
use std::io;

use crate::remote::channel::*;

pub async fn test() {
    // Connection
    let mut stream = TcpStream::connect("127.0.0.1:4000").await.unwrap();

    // write some data
    stream.write_all(b"Hello world !").await;

    println!("Hello, I am the client.");
}

struct ParallelClient {
    request: String,

    channel: Channel,
}

impl ParallelClient {
    pub async fn new(server_adress: String, request: String) -> Self {
        let socket: TcpStream;
        let res_connection = TcpStream::connect(server_adress).await;

        if let Ok(s) = res_connection {
            socket = s;
            let mut client : ParallelClient = ParallelClient {
                request,

                channel : Channel::new(socket),
            };

            let mut reference : Rc<dyn ChannelListener> = Rc::new(client);

            *(reference).channel.setListener(reference);

            client.send_request();
            return client;
        }

        panic!("Couldn't create the socket");
    }

    pub async fn send_request(&mut self) {
        self.channel.send(self.request.clone().into_bytes());
    }
}

impl ChannelListener for ParallelClient {
    fn received(&self, buffer: std::vec::Vec<u8>) {
        println!("Result :");
        println!("{:?}", buffer);
    }

    fn sent(&self) {
        println!("Message has been sent");
    }
}
