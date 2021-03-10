use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
use tokio::io::ReadBuf;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::Ready;

use std::error::Error;
use std::io;

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
                worker.exchange_loop();
            });

            continue;
        } else {
            panic!("Error while accepting connection request");
        }
    }

    Ok(())
}

struct ParallelWorker {
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,

    request: String,

    socket: TcpStream,
}

impl ParallelWorker {
    pub fn new(socket: TcpStream) -> Self {
        ParallelWorker {
            read_buf: Vec::new(),
            write_buf: Vec::new(),

            request: String::new(),

            socket: socket,
        }
    }

    pub async fn exchange_loop(&mut self) -> Result<(), Box<dyn Error>> {
        let mut ready = self.socket.ready(Interest::READABLE).await.unwrap();

        let mut size_response: u64 = 0;
        let mut size_request: u64 = 0;

        loop {
            if ready.is_readable() {
                let mut data = vec![0; 1024];

                match self.socket.try_read(&mut data) {
                    Ok(n) => {
                        println!("read {} bytes", n);
                        if size_request == 0 && n == 4 {
                            for i in 0..4 {
                                let value: u64 = data[i] as u64;
                                size_request = size_request * (0xFF as u64) + value;
                            }
                        } else if size_request > 0 {
                            for i in 0..(n + 1) {
                                self.read_buf.push(data[i]);
                            }
                            size_request = size_request - (n as u64);

                            // insert function which will handle the request
                            ready = Ready::EMPTY;
                            self.handle_request();
                            return Ok(());
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        size_request = 0;
                        //continue;
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }

            // Write part
            if ready.is_writable() {
                if size_response == 0 {
                    size_response = self.write_buf.len() as u64;
                    match self.socket.try_write(&mut size_response.to_ne_bytes()) {
                        Ok(n) => {
                            if n != 8 {
                                panic!("NYI when size isn't sent entirely");
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            size_response = 0;
                            continue;
                        }
                        Err(e) => {
                            return Err(e.into());
                        }
                    }
                } else {
                    match self.socket.try_write(&mut self.write_buf) {
                        Ok(n) => {
                            size_response -= n as u64;
                            if size_response == 0 {
                                ready = Ready::EMPTY;
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

    pub fn send_response(&mut self) {
       // put something in the writeBuf
    }

    pub fn handle_request(&mut self) {
        // call parallel app
        // apply parallel
        // get result and send it
        println!("Request data : {:?}", self.read_buf);

        self.send_response();
    }
}
