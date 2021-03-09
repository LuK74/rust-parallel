use tokio::io::AsyncWriteExt;
use tokio::io::Interest;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::ReadBuf;

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
                handle_client(s);
            });

            continue;
        } else {
            panic!("Error while accepting connection request");
        }
    }

    Ok(())
}

pub async fn handle_client(socket: TcpStream) -> Result<(), Box<dyn Error>> {
    
    let mut size_request : u64 = 0;
    let mut readBuf : Vec<u8> = Vec::new();
    let mut ready = socket.ready(Interest::READABLE).await.unwrap();
    
    loop {

        // Read part
        if ready.is_readable() {
            let mut data = vec![0; 1024];

            match socket.try_read(&mut data) {
                Ok(n) => {
                    println!("read {} bytes", n);
                    if size_request == 0 && n == 4 {
                        for i in 0..4 {
                            let value : u64 = data[i] as u64;
                            size_request = size_request * (0xFF as u64) + value;
                        }
                    } else if size_request > 0 {
                        for i in 0..(n+1) {
                            readBuf.push(data[i]);
                        } 
                        size_request = size_request - (n as u64);

                        // insert function which will handle the request

                        ready = socket.ready(Interest::WRITABLE).await.unwrap();
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


        }
    }
    Ok(())
}
