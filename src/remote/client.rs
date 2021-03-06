use tokio::io::Interest;
use tokio::net::TcpStream;

use std::fs::File;
use std::io::Read;

use log::debug;

use crate::remote::channel::*;

/**
 * Client side of a Parallel Client-Server exchange
 * - `request : String` - Request that we want to execute
 * - `files : Vec<String>` - List of file needed for the request's execution
 * - `request_response : String` - Use to store the request's result sent
 * by the Server
 * - `server_address : String` - Server address, example : "127.0.0.1:8080"
 */
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

/**
 * ParallelClient functions implementation
 */
impl ParallelClient {
    /**
     * Create a new parallel client using the given arguments
     * # Arguments
     * - `server_address` - Server address
     * - `request` - Request
     */
    pub fn new(server_address: String, request: String) -> Self {
        ParallelClient {
            request,

            files: Vec::new(),

            request_response: String::new(),
            server_address,
        }
    }

    /**
     * Modify the current interest of the channel
     * # Arguments
     * - `interest` - New interest of the Channel
     */
    pub fn add_files(&mut self, f: Vec<String>) {
        self.files = f;
    }

    /**
     * Main function for the Client
     * Will try to connect to server and then prepare the Channel
     * in order to launch the exchange_loop()
     *
     * Will return an Ok() result containing the request's execution
     * result
     *
     * If an error occured, will return an Err() result containing
     * a String describing the kind of error
     */
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
            panic!(
                "Couldn't connect to the server, server address : {}",
                self.server_address
            );
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

/**
 * ChannelListener implementation for the Client
 */
impl ChannelListener for ParallelClient {
    /** Method invoked when a message has been fully read
     * If this returns an option Some the Channel will change his
     * interest to Write
     */
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

    /** Method invoked when a message has been fully sent
     * If this returns an option Some the Channel will change his
     * interest to Read
     */
    fn sent(&mut self) -> Option<()> {
        Some(())
    }
}
