use std::io;
use tokio::io::Ready;
use tokio::net::TcpStream;

use log::debug;

pub trait ChannelListener {
    /// Method invoked when a message has been fully read
    /// If this returns an option Some the Channel will change his
    /// interest to Write
    fn received(&mut self, buffer: Vec<u8>) -> Option<Vec<u8>>;

    /// Method invoked when a message has been fully sent
    /// If this returns an option Some the Channel will change his
    /// interest to Read
    fn sent(&mut self) -> Option<()>;
}

/**
 * Designed to be used with 0-knowledge of what the application does
 * Will only send and receive data, and call the associate listener
 * when those actions complete
 **/
pub struct Channel<'a> {
    /// Buffer used by the Channel to read and write
    read_buf: Vec<u8>,
    write_buf: Vec<u8>,

    /// Ready struct, used to know if we're looking to read or write
    ready: Ready,

    /// Socket linked to this channel
    socket: TcpStream,

    /// Channel Listener
    listener: Option<&'a mut dyn ChannelListener>,

    /// Running state of the Channel
    running: bool,
}

impl<'a> Channel<'a> {
    /// Default Channel Constructor
    pub fn new(socket: TcpStream) -> Self {
        Channel {
            read_buf: Vec::new(),
            write_buf: Vec::new(),

            ready: Ready::EMPTY,

            socket: socket,

            listener: None,

            running: false,
        }
    }

    /// Modify the current interest of the Channel
    pub fn set_interest(&mut self, interest: Ready) {
        self.ready = interest;
    }

    /// Set the listener of this Channel
    pub fn set_listener(&mut self, listener: &'a mut dyn ChannelListener) {
        self.listener = Some(listener);
    }

    /// Close the channel
    pub fn close(&mut self) {
        debug!("Closing channel");
        self.running = false;
        self.ready = Ready::EMPTY;
    }

    /// Prepare the write_buf
    /// Clone the given buffer
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
                // Vec size can be modified to read more bytes
                // during each iteration
                let mut data = vec![0; 1024];
                let mut size = vec![0; 8];

                let read_result;

                // If size_request is equal to 0 it means
                // that we haven't read the value yet
                if size_request == 0 {
                    read_result = self.socket.try_read(&mut size);
                } else {
                    read_result = self.socket.try_read(&mut data);
                }

                match read_result {
                    Ok(n) => {
                        // We're assuming that we always read the 8 bytes for the size
                        // in one try
                        if size_request == 0 && n == 8 {
                            // Retrieve the 8 bytes, and convert it to an u64
                            // using the native endianness
                            let mut array = [0; 8];
                            for i in 0..8 {
                                array[i] = size[i];
                            }
                            size_request = u64::from_ne_bytes(array);
                        } else if size_request > 0 {
                            // Retrieve each bytes and put it the reader buffer
                            for i in 0..n {
                                self.read_buf.push(data[i]);
                            }

                            size_request = size_request - (n as u64);
                            data.clear();

                            if size_request <= 0 {
                                // Call the listener to tell him that we received the message
                                // and we give it to him
                                if let Some(next_msg) = self
                                    .listener
                                    .as_mut()
                                    .unwrap()
                                    .received(self.read_buf.clone())
                                {
                                    // The listener returned an Option Some containing
                                    // the next msg, we need to change the interest of our channel
                                    // to WRITABLE in order for him to send the next message
                                    self.ready = Ready::WRITABLE;
                                    self.send(next_msg);
                                } else {
                                    // The listener returned an Option None, it means that
                                    // the communication is over, we can close our channel
                                    self.close();
                                    break;
                                }

                                // Clear the reader buffer
                                self.read_buf.clear();
                                size_request = 0;
                            }
                        }

                        // If a try_read return an Ok(n) value with n equal to 0
                        // it means that the communication has been ended
                        if n == 0 {
                            if self.running != false {
                                return Err(String::from("Channel has been closed unexpectedly"));
                            }
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                        size_request = 0;
                        continue;
                    }
                    Err(e) => {
                        debug!("Error occured during read : {:?}", e);
                        return Err(String::from("Unknown Error"));
                    }
                }
            }

            // Write part
            if self.ready.is_writable() {
                // If size_response is equal to 0, it means
                // that we haven't read the size yet
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

                // If size_response isn't equal to 0, we can skip to
                // reading data part
                } else {
                    match self.socket.try_write(&mut self.write_buf) {
                        Ok(n) => {
                            size_response -= n as u64;
                            self.write_buf = self.write_buf.split_off(n);

                            if size_response <= 0 {
                                size_response = 0;
                                // After the Channel sent the entire message we need to
                                // tell it to the listener using the sent() method
                                // If this method returns an Option Some it means that our
                                // channel to be prepared to read a reponse
                                // If not we can close this channel
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
