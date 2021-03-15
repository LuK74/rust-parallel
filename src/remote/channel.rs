use std::io;
use tokio::io::Interest;
use tokio::io::Ready;
use tokio::net::TcpStream;

pub trait ChannelListener {
    // Method invoked when a message has been fully read
    fn received(&self, buffer: Vec<u8>, channel : &mut Channel);

    // Method invoked when a message has been fully sent
    fn sent(&self, channel : &mut Channel);
}

pub struct Channel<'a> {
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,

    ready: Ready,

    socket: TcpStream,

    listener: Option<&'a dyn ChannelListener>,
}

impl<'a> Channel<'a> {
    pub fn new(socket: TcpStream) -> Self {
        Channel {
            read_buf: Vec::new(),
            write_buf: Vec::new(),

            ready: Ready::EMPTY,

            socket: socket,

            listener: None,
        }
    }

    pub async fn set_interest(&mut self, interest: Interest) {
        self.ready = self.socket.ready(interest).await.unwrap();
    }

    pub fn set_listener(&mut self, listener: &'a dyn ChannelListener) {
        self.listener = Some(listener);
    }

    pub fn close(&mut self) {
        self.ready = Ready::EMPTY;
    }

    pub fn send(&mut self, buf: Vec<u8>) {
        self.read_buf = buf.clone();
    }

    pub async fn exchange_loop(&mut self) -> Result<(), String> {
        let mut size_response: u64 = 0;
        let mut size_request: u64 = 0;

        if self.ready == Ready::EMPTY {
            panic!("Interest hasn't been set");
        }

        loop {
            if self.ready.is_readable() {
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
                            self.listener.as_ref().unwrap().received(self.read_buf.clone(), self);
                            self.ready = self.socket.ready(Interest::WRITABLE).await.unwrap();
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        size_request = 0;
                        continue;
                    }
                    Err(e) => {
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
                        Err(e) => {
                            return Err(String::from("Unknown Error"));
                        }
                    }
                } else {
                    match self.socket.try_write(&mut self.write_buf) {
                        Ok(n) => {
                            size_response -= n as u64;
                            if size_response <= 0 {
                                size_response = 0;
                                self.ready = self.socket.ready(Interest::READABLE).await.unwrap();
                                self.listener.as_ref().unwrap().sent(self);
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                            continue;
                        }
                        Err(e) => {
                            return Err(String::from("Unknown Error"));
                        }
                    }
                }
            }
        }
    }
}
