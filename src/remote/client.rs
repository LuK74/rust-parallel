use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use std::net::Ipv4Addr;
use tokio::io::Interest;

use std::error::Error;
use std::io;

pub async fn test() {
    // Connection
    let mut stream = TcpStream::connect("127.0.0.1:4000").await.unwrap();

    // write some data
    stream.write_all(b"Hello world !").await;

    println!("Hello, I am the client.");
}


// Could be useful to

pub async fn client_exec(server_adress : String , request : String) {
    let res_connect = connect_to_server(server_adress).await;

    if let Ok(socket) = res_connect {
        exchange_loop(socket, request).await.unwrap();
    } else {
        panic!("Client couldn't connect");
    }
}

pub async fn connect_to_server(server_adress : String) -> Result<TcpStream , String>{
    let mut socket :TcpStream;
    let res_connection = TcpStream::connect(server_adress).await;

    if let Ok(s) = res_connection {
        socket = s;
        return Ok(socket);
    } else {
        return Err(String::from("Couldn't connected to the Server"));
    }
}

pub async fn exchange_loop(socket : TcpStream, request : String) -> Result<(), Box<dyn Error>> {
    let mut ready = socket.ready(Interest::WRITABLE).await.unwrap();

    let mut size_response : u64 = 0;

    let mut readBuf : Vec<u8> = Vec::new();

    loop {
        
        if ready.is_readable() {
            let mut data = vec![0; 1024];

            match socket.try_read(&mut data) {
                Ok(n) => {
                    println!("read {} bytes", n);
                    if size_response == 0 && n == 4 {
                        for i in 0..4 {
                            let value : u64 = data[i] as u64;
                            size_response = size_response * (0xFF as u64) + value;
                        }
                    } else if size_response > 0 {
                        for i in 0..(n+1) {
                            readBuf.push(data[i]);
                        } 
                        size_response = size_response - (n as u64);

                        // insert function which will handle the request

                        ready = socket.ready(Interest::WRITABLE).await.unwrap();
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


        }

    }

}

pub async fn send_request() {

}