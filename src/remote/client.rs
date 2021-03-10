use std::net::Ipv4Addr;
use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
use tokio::io::Ready;
use tokio::net::TcpStream;

use std::error::Error;
use std::io;

pub async fn test() {
    // Connection
    let mut stream = TcpStream::connect("127.0.0.1:4000").await.unwrap();

    // write some data
    stream.write_all(b"Hello world !").await;

    println!("Hello, I am the client.");
}

struct ParallelClient {
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,

    request: String,

    socket: TcpStream,
}

impl ParallelClient {
    pub async fn new(server_adress: String, request: String) -> Self {
        let socket: TcpStream;
        let res_connection = TcpStream::connect(server_adress).await;

        if let Ok(s) = res_connection {
            socket = s;
            return ParallelClient {
                read_buf: Vec::new(),
                write_buf: Vec::new(),

                request,

                socket: socket,
            };
        }

        panic!("Couldn't create the socket");
    }

    pub async fn exchange_loop(&mut self) -> Result<(), Box<dyn Error>> {
        self.send_request();

        let mut ready = self.socket.ready(Interest::WRITABLE).await.unwrap();

        let mut size_response: u64 = 0;
        let mut size_request: u64 = 0;

        loop {
            if ready.is_readable() {
                let mut data = vec![0; 1024];

                match self.socket.try_read(&mut data) {
                    Ok(n) => {
                        println!("read {} bytes", n);
                        if size_response == 0 && n == 4 {
                            for i in 0..4 {
                                let value: u64 = data[i] as u64;
                                size_response = size_response * (0xFF as u64) + value;
                            }
                        } else if size_response > 0 {
                            for i in 0..(n + 1) {
                                self.read_buf.push(data[i]);
                            }
                            size_response = size_response - (n as u64);

                            // insert function which will handle the request
                            ready = Ready::EMPTY;
                            self.handle_response();
                            return Ok(());
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        size_response = 0;
                        //continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }

            // Write part
            if ready.is_writable() {
                if size_request == 0 {
                    size_request = self.write_buf.len() as u64;
                    match self.socket.try_write(&mut size_response.to_ne_bytes()) {
                        Ok(n) => {
                            if n != 8 {
                                panic!("NYI when size isn't sent entirely");
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            size_request = 0;
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                } else {
                    match self.socket.try_write(&mut self.write_buf) {
                        Ok(n) => {
                            size_request -= n as u64;
                            if size_request == 0 {
                                ready = self.socket.ready(Interest::READABLE).await.unwrap();
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                }
            }
        }
    }

    pub async fn send_request(&mut self) {
        self.write_buf = self.request.clone().into_bytes();
    }

    pub async fn handle_response(&mut self) {
        println!("Reponse data : {:?}", self.read_buf);
    }
}
