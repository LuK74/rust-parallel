use crate::parallel::Parallel;
use tokio::io::Interest;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use std::env;
use std::fs::OpenOptions;
use std::fs::{self, DirBuilder};
use std::io::Write;

use log::debug;

use crate::remote::channel::*;

/**
 * Server side of a Parallel Client-Server exchange
 * - `listener : TcpListener` - Passive Socket of the Server side
 * - `tmp_dir : String` - Path of the temporary directory used
 * during the exchange files phase (if needed)
 */
pub struct ParallelServer {
    // Passive socket listening for Client request
    listener: TcpListener,

    // Name of the temporary directory used to store
    // the files needed by the server to execute the requests
    tmp_dir: String,
}

/**
 * ParallelServer functions implementation
 */
impl ParallelServer {
    /**
     * ParralelServer constructor
     * Will launch the passive socket and create the temporary directory
     * # Arguments
     * - `server_adresse : String` - Server address of the server we want
     * to create, need to follow this format : [serverAddress:serverPort]
     * Example : 127.0.0.1:8080
     */
    pub async fn new(server_address: String) -> Self {
        // Creation of the passive Socket
        let res_bind = TcpListener::bind(server_address.clone()).await;
        let listener: TcpListener;

        let tmp_dir = String::from("tmp/");

        // If the TcpListener::bind() method return an Ok result
        // it means that the bind succeed and that we can retrieve
        // the TcpListener
        if let Ok(l) = res_bind {
            listener = l;
        } else {
            panic!("Couldn't bind the server to the address {}", server_address);
        }

        // Try to create the temporary directory
        // It is not an "actual" error if this returns an error AlreadyExists
        if let Err(e) = DirBuilder::new().create(tmp_dir.clone()) {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                debug!("{:?}", e);
                panic!("Couldn't create temporary directory");
            } else {
                debug!("Temporary dir already exists, {}", e);
            }
        }

        // Check that the tmp_dir is a directory
        assert!(fs::metadata(tmp_dir.clone()).unwrap().is_dir());

        ParallelServer { listener, tmp_dir }
    }

    /** Listening loop
     * Will listening to every client request and create a
     * ParallelWorker for each one
     * Each of this ParallelWorker will do his job in his own
     * thread
     */
    pub async fn waiting_request(&mut self) {
        loop {
            let tmp_dir = self.tmp_dir.clone();
            if let Ok((s, _)) = self.listener.accept().await {
                tokio::spawn(async move {
                    if let Ok(_) = ParallelWorker::process(s, tmp_dir).await {
                    } else {
                    }
                });
                continue;
            } else {
                // Panic isn't necessarily needed here
                // We don't want our Server to stop only because
                // of one Client that may have kill his Client before
                // the accept process ended (for more information we
                // would need to look the TcpListener implementation)
                // Using the macro panic!() could result in an
                // Denial Of Service exploit
                println!("Error while accepting connection request");
            }
        }
    }
}

/**
* Enum used to know the current state of a Worker
* Due to the fact that a Worker will need to retrieve several
* types of informations and not necessarily in the same way each time,
* the use of a state is well suited
*/
pub enum WorkerState {
    Idle,
    WaitingFileName,
    WaitingFileData,
    WaitingRequest,
    SendingResult,
}

/**
 * Client side of a Parallel Client-Server exchange
 * - `request : String` - Request that we want to execute
 * - `request_result : String` - Use to store the request's result
 * - `tmp_dir : String` - Path of the temporary directory used
 * during the exchange files phase (if needed)
 * - `files : Vec<(String, String)>` - Name of the files sent by the Client
 * and their actual path
 * Example : File toto.txt sent by the Client : (toto.txt, "/tmp/toto.txt")
 * if the tmp_dir variable is set to "/tmp/"
 * - `state : WorkerState` - Current state of the Worker
 * - `current_file : String` - Field used for the file exchange phase
 */
pub struct ParallelWorker {
    // Result of the request execution
    request_result: String,
    // Request asked by the Client
    request: String,

    // Path of the temporary directory
    tmp_dir: String,
    // Name of the files sent by the Client
    // and their actual path
    // Exemple : File toto.txt sent by the Client
    // (toto.txt, "/tmp/toto.txt")
    // if the tmp_dir variable is set to "/tmp/"
    files: Vec<(String, String)>,

    // State of the worker
    state: WorkerState,
    // Name of the file currently transfered
    current_file: String,
}

impl ParallelWorker {
    /**
     * Default constructor for a Parallel Worker
     * # Arguments
     * - `tmp_dir : String` - Path of the temporary directory used
     * during the exchange files phase (if needed)
     */
    pub fn new(tmp_dir: String) -> Self {
        ParallelWorker {
            request_result: String::new(),
            request: String::new(),
            tmp_dir,
            files: Vec::new(),
            state: WorkerState::Idle,
            current_file: String::new(),
        }
    }

    /**
     * Given a TcpStream and the path to the temporary directory
     * This method will create and start a ParallelWorker
     * It will also retrieve the result and return it
     * # Arguments
     * - `socket : TcpStream` : Socket created by the Server passive
     * socket
     * - `tmp_dir : String` - Path of the temporary directory used
     * during the exchange files phase (if needed)
     */
    pub async fn process(socket: TcpStream, tmp_dir: String) -> Result<String, String> {
        let mut worker = ParallelWorker::new(tmp_dir);
        let result_work = worker.start_worker(socket).await;

        if let Ok(res) = result_work {
            return Ok(res);
        } else {
            debug!("Error occured during worker job");
            return result_work;
        }
    }

    /**
     * Method used to start a worker
     * It will create and set the Channel used for the exchange
     * # Arguments
     * - `tmp_dir : String` - Path of the temporary directory used
     * during the exchange files phase (if needed)
     */
    pub async fn start_worker(&mut self, socket: TcpStream) -> Result<String, String> {
        // Channel creation
        let mut channel: Channel = Channel::new(socket);

        // Preparation of the channel
        channel.set_listener(self);
        channel.set_interest(Interest::READABLE);

        // Start of the exchange loop
        channel.exchange_loop().await.unwrap();

        Ok(self.request_result.clone())
    }
}

impl ChannelListener for ParallelWorker {
    /**
     * Method invoked when a message has been fully read
     * If this returns an option Some the Channel will change his
     * interest to Write
     */
    fn received(&mut self, buffer: Vec<u8>) -> Option<Vec<u8>> {
        let response = String::from_utf8(buffer).unwrap();
        debug!("-- Server received : {}", response);

        match &self.state {
            WorkerState::Idle => {
                if response.eq("Sending files") {
                    // Response "Sending files" means that the Client is going to send
                    // us files needed for the request execution
                    self.state = WorkerState::WaitingFileName;
                    return Some("Waiting for new file".bytes().collect());
                } else if response.eq("Sending request") {
                    // Response "Sending request" means that the Client is going to send
                    // us the request
                    self.state = WorkerState::WaitingRequest;
                    return Some("Ready for request".bytes().collect());
                } else {
                    debug!("Unexepected behaviour, shouldn't be in a IDLE state");
                    return None;
                }
            }
            WorkerState::WaitingFileName => {
                if response.eq("No more files") {
                    // Response "No more files" means that the Client has no more
                    // files to send us, we can now ask for the request
                    self.state = WorkerState::WaitingRequest;
                    return Some("Ready for request".bytes().collect());
                } else {
                    // If Response isn't equals to any pre-defined message
                    // We can assume that he sent us a filename
                    // We'll save this filename and ask for the file data
                    self.current_file = response;
                    self.state = WorkerState::WaitingFileData;
                    return Some("Ready for next file".bytes().collect());
                }
            }
            WorkerState::WaitingFileData => {
                // In this state we're supposed to retrieve the file data
                // and to create a temporary file, in order for the server to use
                // those data for the request execution
                let mut extension = String::from("");
                let mut current_number = 0;

                // We start by tring to create a temporary file with the same name
                // as the initial file
                let mut result = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(self.tmp_dir.clone() + &self.current_file + &extension[..]);

                while let Err(e) = &result {
                    // If the file creation didn't work due to an AlreadyExists error
                    // we'll try to add en version extension to the file
                    // Example : Client sent "toto.txt" which already exists in our
                    // temporary directory
                    // So we'll try to save the data in a file named "toto.txt(1)"
                    // If this still doesn't work, will increment the version number by one
                    // until it works
                    if e.kind() == std::io::ErrorKind::AlreadyExists {
                        current_number += 1;
                        extension = format!("({})", current_number);
                        result = OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(self.tmp_dir.clone() + &self.current_file + &extension[..]);
                    // If the error isn't an AlreadyExists error, we'll panic
                    } else {
                        println!("{:?}", e);
                        panic!(
                            "Unexpected behaviour, couldn't create the temporary file for {}",
                            self.current_file
                        );
                    }
                }

                // The use of unwrap here is safe due to our previous while loop
                let mut file = result.unwrap();

                // Write the data on the file and sync it
                let buf: &[u8] = &response.bytes().collect::<Vec<u8>>()[..];

                if let Err(_) = file.write_all(buf) {
                    panic!("Couldn't write the data in the temporary file");
                }

                if let Err(_) = file.sync_data() {
                    panic!("Couldn't save the data in the temporary file");
                }

                // Push the couple (Filename, Filepath) in the vec contained in the
                // ParallelWorker struct
                self.files.push((
                    self.current_file.clone(),
                    self.tmp_dir.clone() + "/" + &self.current_file + &extension[..],
                ));

                // Return in the state waiting for a filename or for a message telling
                // that there is no more file to transfer
                self.state = WorkerState::WaitingFileName;
                return Some("Waiting for new file".bytes().collect());
            }
            WorkerState::WaitingRequest => {
                self.request = response;
                // handle the request and send the result

                // !!! insert here the method executing the request !!! //

                // put the request result in this variable
                debug!("Server : request received : {}", self.request);

                let shell = match env::var("SHELL") {
                    Ok(val) => val,
                    Err(e) => {
                        eprintln!("Couldn't interpret environment variable SHELL: {}", e);
                        return None;
                    }
                };

                let args: Vec<String> = self
                    .request
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();

                let prg = Parallel::new(shell, args);
                self.request_result = String::new();
                if let Some(results) = prg.start() {
                    for result in results {
                        self.request_result.push_str(&result);
                    }
                }

                debug!("Server : result of request : {}", self.request_result);

                self.state = WorkerState::SendingResult;
                return Some(self.request_result.bytes().collect());
            }
            _ => panic!("Shoudln't happened"),
        }
    }

    /** Method invoked when a message has been fully sent
     * If this returns an option Some the Channel will change his
     * interest to Read
     */
    fn sent(&mut self) -> Option<()> {
        match &self.state {
            // If we were in the SendingResult it means that
            // we sent the last message that our ParallelWorker was
            // supposed to send
            // So we can return None, in order to end the Channel
            WorkerState::SendingResult => return None,
            _ => return Some(()),
        }
    }
}
