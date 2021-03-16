use std::io;
use tokio::io::Ready;
use tokio::net::TcpStream;

pub trait ChannelListener {
    // Method invoked when a message has been fully read
    fn received(&mut self, buffer: Vec<u8>) -> Option<Vec<u8>>;

    // Method invoked when a message has been fully sent
    fn sent(&mut self) -> Option<()>;
}

pub struct Channel<'a> {
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,

    ready: Ready,

    socket: TcpStream,

    listener: Option<&'a mut dyn ChannelListener>,

    running: bool,
}

impl<'a> Channel<'a> {
    pub fn new(socket: TcpStream) -> Self {
        Channel {
            read_buf: Vec::new(),
            write_buf: Vec::new(),

            ready: Ready::EMPTY,

            socket: socket,

            listener: None,

            running : false,
        }
    }

    pub fn set_interest(&mut self, interest: Ready) {
        self.ready = interest;
    }

    pub fn set_listener(&mut self, listener: &'a mut dyn ChannelListener) {
        self.listener = Some(listener);
    }

    pub fn close(&mut self) {
        println!("Closing channel");
        self.running = false;
        self.ready = Ready::EMPTY;
    }

    pub fn send(&mut self, buf: Vec<u8>) {
        self.write_buf = buf.clone();
    }

    pub fn exchange_loop(&mut self) -> Result<(), String> {
        let mut size_response: u64 = 0;
        let mut size_request: u64 = 0;

        self.running = true;

        if self.ready == Ready::EMPTY {
            panic!("Interest hasn't been set");
        }

        while self.running {

            if self.ready.is_readable() {
                let mut data = vec![0; 1024];
                let mut size = vec![0; 8];

                let read_result;

                if size_request == 0 {
                    read_result = self.socket.try_read(&mut size);
                } else {
                    read_result = self.socket.try_read(&mut data);
                }

                match read_result {
                    Ok(n) => {
                        if size_request == 0 && n == 8 {
                            for i in 0..8 {
                                let value: u64 = size[7-i] as u64;
                                size_request = size_request * (0xFF as u64) + value;
                            }
                        } else if size_request > 0 {
                            for i in 0..n  {
                                self.read_buf.push(data[i]);
                            }
                            size_request = size_request - (n as u64);
                            
                            if size_request <= 0 {
                                // insert function which will handle the request
                                if let Some(next_msg) = self.listener.as_mut().unwrap().received(self.read_buf.clone()) {
                                    self.ready = Ready::WRITABLE;
                                    self.send(next_msg);
                                } else {
                                    self.close();
                                }
                                //self.ready = self.socket.ready(Interest::WRITABLE).await.unwrap();
                                
                                
                                size_request = 0;
                            }
                        }

                        if n == 0 {
                            return Err(String::from("Channel has been closed unexpectedly"));
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        size_request = 0;
                        continue;
                    }
                    Err(_e) => {
                        return Err(String::from("Unknown Error"));
                    }
                }
            }

            // Write part
            if self.ready.is_writable() {
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
                        Err(_e) => {
                            return Err(String::from("Unknown Error"));
                        }
                    }
                } else {
                    match self.socket.try_write(&mut self.write_buf) {
                        Ok(n) => {
                            size_response -= n as u64;
                            if size_response <= 0 {
                                size_response = 0;
                                //self.ready = self.socket.ready(Interest::READABLE).await.unwrap();
                                //self.ready = Ready::READABLE;
                                if let Some(_) = self.listener.as_mut().unwrap().sent() {
                                    self.ready = Ready::READABLE;
                                } else {
                                    self.close();
                                }
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(_e) => {
                            return Err(String::from("Unknown Error"));
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
