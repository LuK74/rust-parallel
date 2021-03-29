use tokio::io::Interest;
use tokio::net::TcpStream;

use std::fs::File;
use std::io::Read;

use log::debug;

use crate::remote::channel::*;

pub async fn test_exchange(args: Vec<String>) {
    let mut client: ParallelClient =
        ParallelClient::new(String::from("127.0.0.1:4000"), args[0].clone());
    client.start_client().await.unwrap();
}

pub struct ParallelClient {
    // Request that our Client is going to send
    request: String,

    // Files needed by the server to execute the request
    files: Vec<String>,

    // Request response sent by the server
    request_response: String,

    // Server addresse using this format "[address:port]"
    server_address: String,
}

impl ParallelClient {
    // Default Parralel Client constructor
    pub fn new(server_address: String, request: String) -> Self {
        ParallelClient {
            request,

            files: Vec::new(),

            request_response: String::new(),
            server_address,
        }
    }

    // Set the list of files needed by the Server
    pub fn add_files(&mut self, f: Vec<String>) {
        self.files = f;
    }

    // Main function for Client
    // Will try to connect to server and then prepare the Channel
    // in order to launch the exchange_loop()
    pub async fn start_client(&mut self) -> Result<String, String> {
        // Try to connect to the Server
        let res_connection = TcpStream::connect(self.server_address.clone()).await;

        let mut channel: Channel;

        if let Ok(s) = res_connection {
            debug!("Connection was a success");
            // Creation of the channel with the socket returned
            // by the TcpStream::connect method
            channel = Channel::new(s);
        } else {
            panic!("Couldn't connect to the server");
        }

        // If the list of files needed by the server is empty we can
        // directly prepare our channel to send the request
        // If not we have to go through a "Sending files" phase
        if self.files.len() == 0 {
            channel.send("Sending request".bytes().collect());
        } else {
            channel.send("Sending files".bytes().collect());
        }

        // Preparation of the Channel
        channel.set_listener(self);
        channel.set_interest(Interest::WRITABLE);
        channel.exchange_loop().await.unwrap();

        Ok(self.request_response.clone())
    }
}

impl ChannelListener for ParallelClient {
    fn received(&mut self, buffer: Vec<u8>) -> Option<Vec<u8>> {
        // Convert the received buffer into a String using an utf-8 format
        let response: String = String::from_utf8(buffer).unwrap();
        debug!("-- Client received : {}", response);

        let mut next_msg: Vec<u8> = Vec::new();

        // Response "Ready for next file" means that we can now
        // send the file data
        if response.eq("Ready for next file") {
            let mut file = File::open(self.files.remove(0)).unwrap();

            if let Err(_e) = file.read_to_end(&mut next_msg) {
                println!("Error reading the file");
            }

            return Some(next_msg);

        // Response "Waiting for new file" means that the server
        // want to know the name of the next file if there is one
        } else if response.eq("Waiting for new file") {
            if self.files.len() == 0 {
                // If there is no more file to send
                // We send a "No more files" to the Server
                return Some("No more files".bytes().collect());
            }
            // If there is still files to send, we send the filename
            // Here the use of unwrap is safe due to the previous test
            return Some(self.files.get(0).unwrap().bytes().collect());
        // Response "Ready for request" means that the server
        // is ready to received and execute the request
        } else if response.eq("Ready for request") {
            return Some(self.request.bytes().collect());
        } else {
            // If the response is not equal to any of the above test
            // we can assume that the server sent us the result of the
            // request execution
            self.request_response = response;
            None
        }
    }

    fn sent(&mut self) -> Option<()> {
        Some(())
    }
}
