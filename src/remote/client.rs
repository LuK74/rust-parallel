use tokio::io::Ready;
use tokio::net::TcpStream;

use std::fs::File;
use std::io::Read;

use crate::remote::channel::*;

pub async fn test_exchange(args: Vec<String>) {
    let mut client: ParallelClient =
        ParallelClient::new(String::from("127.0.0.1:4000"), args[0].clone());
    client.start_client().await.unwrap();
}

pub struct ParallelClient {
    request: String,

    files: Vec<String>,

    request_response: String,
    server_address: String,
}

impl ParallelClient {
    pub fn new(server_address: String, request: String) -> Self {
        ParallelClient {
            request,

            files: Vec::new(),

            request_response: String::new(),
            server_address,
        }
    }

    pub fn add_files(&mut self, f: Vec<String>) {
        for file in f.iter() {
            self.files.push(file.clone());
        }
    }

    pub async fn start_client(&mut self) -> Result<String, String> {
        let res_connection = TcpStream::connect(self.server_address.clone()).await;

        let mut channel: Channel;

        if let Ok(s) = res_connection {
            println!("Connection was a success");
            channel = Channel::new(s);
        } else {
            panic!("Couldn't connect to the server");
        }

        let request = self.request.clone();

        if self.files.len() == 0 {
            channel.send("Sending request".bytes().collect());
        } else {
            channel.send("Sending files".bytes().collect());
        }

        channel.set_listener(self);

        channel.set_interest(Ready::WRITABLE);

        channel.exchange_loop().unwrap();

        Ok(self.request_response.clone())
    }
}

impl ChannelListener for ParallelClient {
    fn received(&mut self, buffer: Vec<u8>) -> Option<Vec<u8>> {
        let response: String = String::from_utf8(buffer).unwrap();
        //println!("-- Client received : {}", response);

        let mut next_msg: Vec<u8> = Vec::new();

        if response.eq("Ready for next file") {
            let mut file = File::open(self.files.remove(0)).unwrap();

            if let Err(_e) = file.read_to_end(&mut next_msg) {
                println!("Error reading the file");
            }

            return Some(next_msg);
        } else if response.eq("Waiting for new file") {
            if self.files.len() == 0 {
                return Some("No more files".bytes().collect());
            }

            return Some(self.files.get(0).unwrap().bytes().collect());
        } else if response.eq("Ready for request") {
            return Some(self.request.bytes().collect());
        } else {
            self.request_response = response;
            //println!("Client : message received : {:?}", self.request_response);
            None
        }
    }

    fn sent(&mut self) -> Option<()> {
        Some(())
    }
}
