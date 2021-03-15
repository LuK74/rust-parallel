use tokio::net::TcpStream;
use tokio::io::Ready;

use crate::remote::channel::*;

pub async fn test_exchange(args : Vec<String>) {
    let mut client : ParallelClient = ParallelClient::new(String::from("127.0.0.1:4000"), args[0].clone());
    client.start_client().await;
}

struct ParallelClient {
    request: String,
    request_response: String,
    server_address : String,
}

impl ParallelClient {

    pub fn new(server_address: String, request: String) -> Self {
        ParallelClient {
            request, 
            request_response : String::new(),
            server_address,
        }
    }

    pub async fn start_client(&mut self) {
        let res_connection = TcpStream::connect(self.server_address.clone()).await;

        let mut channel : Channel;

        if let Ok(s) = res_connection {
            println!("Connection was a success");
            channel = Channel::new(s);
        } else {
            panic!("Couldn't connect to the server");
        }

        let request = self.request.clone();

        channel.set_listener(self);
        channel.send(request.into_bytes());
        channel.set_interest(Ready::WRITABLE);

        // Should spawn a thread to do this work
        // Result isn't correctly handled yet
        channel.exchange_loop().unwrap();
        
        //self.send_request(&mut channel);
    }

}

impl ChannelListener for ParallelClient {
    
    fn received(&mut self, buffer : Vec<u8>) -> Option<Vec<u8>> {
        println!("Result :");
        println!("{:?}", buffer);

        self.request_response = String::from_utf8(buffer).unwrap();

        println!("Message : {}", self.request_response);
        None
    }

    fn sent(&mut self) -> Option<()> {
        println!("Message has been sent");
        Some(())
    }
}
