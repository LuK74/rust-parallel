use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
use tokio::net::TcpStream;

use crate::remote::channel::*;

pub async fn test() {
    // Connection
    let mut stream = TcpStream::connect("127.0.0.1:4000").await.unwrap();

    // write some data
    stream.write_all(b"Hello world !").await;

    println!("Hello, I am the client.");
}

pub async fn test_exchange(args : Vec<String>) {
    let mut client : ParallelClient = ParallelClient::new(String::from("127.0.0.1:4000"), args[0].clone()).await;
    client.start_client().await;
}

struct ParallelClient {
    request: String,
    server_address : String,
}

impl ParallelClient {

    pub async fn new(server_address: String, request: String) -> Self {
        ParallelClient {
            request, 
            server_address,
        }
    }

    pub async fn start_client(&mut self) {
        let res_connection = TcpStream::connect(self.server_address.clone()).await;

        let mut channel : Channel;

        if let Ok(s) = res_connection {
            channel = Channel::new(s);
        } else {
            panic!("Couldn't connect to the server");
        }

        channel.set_listener(self);
        channel.send(self.request.clone().into_bytes());
        channel.set_interest(Interest::WRITABLE).await;

        // Should spawn a thread to do this work
        // Result isn't correctly handled yet
        channel.exchange_loop().await.unwrap();
        
        //self.send_request(&mut channel);
    }

}

impl ChannelListener for ParallelClient {
    
    fn received(&self, buffer : Vec<u8>, channel : &mut Channel) {
        println!("Result :");
        println!("{:?}", buffer);
        channel.close();
    }

    fn sent(&self, channel : &mut Channel) {
        println!("Message has been sent");
    }
}
