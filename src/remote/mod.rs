pub mod channel;
pub mod client;
pub mod server;

#[cfg(test)]
mod tests {
    use crate::remote::client::*;
    use crate::remote::server::*;
    use std::fs;
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::Read;
    use std::io::Write;
    use std::sync::Arc;
    use tokio::runtime::Runtime;
    use tokio::sync::Semaphore;

    // Test a basic "Hello World !" exchange
    #[test]
    fn test_exchange() {
        let rt = Runtime::new().unwrap();

        let metadata = fs::metadata(String::from("tmp").clone());

        if let Ok(m) = metadata {
            if m.is_dir() {
                fs::remove_dir_all("tmp").unwrap();
            }
        }

        rt.block_on(async {
            let mutex_server = Arc::new(Semaphore::new(0));
            let mutex_main = Arc::clone(&mutex_server);

            tokio::spawn(async move {
                let mut server: ParallelServer =
                    ParallelServer::new(String::from("127.0.0.1:8888")).await;
                mutex_server.add_permits(1);
                server.waiting_request().await;
            });

            let _ = mutex_main.acquire().await.unwrap();

            let mut client: ParallelClient =
                ParallelClient::new(String::from("127.0.0.1:8888"), "Hello World!".to_string());
            let res = client.start_client().await;

            if let Ok(_s) = res {
                //assert_eq!(s, "Hello World!");
            } else if let Err(e) = res {
                println!("Error caught : {}", e);
                panic!("Shouldn't caught an error");
            }
        });

        fs::remove_dir_all("tmp").unwrap();
    }

    // Test a basic "Hello World !" exchange
    // And the transfer of 2 files, "toto" and "toto2" which needs
    // to exists in the crate directory
    // Could be modified to create and delete the files needed for
    // test
    #[test]
    fn test_file_exchange() {
        let rt = Runtime::new().unwrap();

        let metadata = fs::metadata(String::from("tmp").clone());

        if let Ok(m) = metadata {
            if m.is_dir() {
                fs::remove_dir_all("tmp").unwrap();
            }
        }

        // First file
        let result = OpenOptions::new().write(true).create_new(true).open("toto");

        if let Err(e) = result {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                panic!("Couldn't create the file needed")
            }
        } else {
            let mut file = result.unwrap();
            let buf: &[u8] = &"Hello World !".bytes().collect::<Vec<u8>>()[..];
            file.write_all(buf).unwrap();
            file.sync_data().unwrap();
        }

        // Second file
        let result = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("toto2");

        if let Err(e) = result {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                panic!("Couldn't create the file needed")
            }
        } else {
            let mut file = result.unwrap();
            let buf: &[u8] = &"Hello World 2 !".bytes().collect::<Vec<u8>>()[..];
            file.write_all(buf).unwrap();
            file.sync_data().unwrap();
        }

        rt.block_on(async {
            let mutex_server = Arc::new(Semaphore::new(0));
            let mutex_main = Arc::clone(&mutex_server);

            tokio::spawn(async move {
                let mut server: ParallelServer =
                    ParallelServer::new(String::from("127.0.0.1:8889")).await;
                mutex_server.add_permits(1);
                server.waiting_request().await;
            });

            let _ = mutex_main.acquire().await.unwrap();

            let mut client: ParallelClient =
                ParallelClient::new(String::from("127.0.0.1:8889"), "Hello World!".to_string());
            client.add_files(vec!["toto".to_string(), "toto2".to_string()]);
            let res = client.start_client().await;

            if let Ok(_s) = res {
                //assert_eq!(s, "Hello World!");
                assert!(fs::metadata(String::from("tmp/toto").clone())
                    .unwrap()
                    .is_file());
                assert!(fs::metadata(String::from("tmp/toto2").clone())
                    .unwrap()
                    .is_file());
            } else if let Err(e) = res {
                println!("Error caught : {}", e);
                panic!("Shouldn't caught an error");
            }
        });

        fs::remove_dir_all("tmp").unwrap();
        fs::remove_file("toto").unwrap();
        fs::remove_file("toto2").unwrap();
    }

    // Test a basic "Hello World !" exchange
    // And the transfer of 1 file but 2 times
    // This should generate to temporary files "toto" and "toto(1)"
    // Could be modified to create and delete the files needed for
    // test
    #[test]
    fn test_duplicate_file_exchange() {
        let rt = Runtime::new().unwrap();

        let metadata = fs::metadata(String::from("tmp").clone());

        if let Ok(m) = metadata {
            if m.is_dir() {
                fs::remove_dir_all("tmp").unwrap();
            }
        }

        // File creation
        let result = OpenOptions::new().write(true).create_new(true).open("titi");

        if let Err(e) = result {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                panic!("Couldn't create the file needed")
            }
        } else {
            let mut file = result.unwrap();
            let buf: &[u8] = &"Hello World !".bytes().collect::<Vec<u8>>()[..];
            file.write_all(buf).unwrap();
            file.sync_data().unwrap();
        }

        rt.block_on(async {
            let mutex_server = Arc::new(Semaphore::new(0));
            let mutex_main = Arc::clone(&mutex_server);

            println!("Lauching server thread");
            tokio::spawn(async move {
                let mut server: ParallelServer =
                    ParallelServer::new(String::from("127.0.0.1:8890")).await;
                mutex_server.add_permits(1);
                println!("Server going in waiting loop");
                server.waiting_request().await;
            });

            let _ = mutex_main.acquire().await.unwrap();

            println!("Lauching client thread");

            let mut client: ParallelClient =
                ParallelClient::new(String::from("127.0.0.1:8890"), "Hello World!".to_string());
            client.add_files(vec!["titi".to_string(), "titi".to_string()]);
            println!("Client going to start");
            let res = client.start_client().await;
            println!("Client finish");

            if let Ok(_s) = res {
                //assert_eq!(s, "Hello World!");
                assert!(fs::metadata(String::from("tmp/titi").clone())
                    .unwrap()
                    .is_file());
                assert!(fs::metadata(String::from("tmp/titi(1)").clone())
                    .unwrap()
                    .is_file());
            } else if let Err(e) = res {
                println!("Error caught : {}", e);
                panic!("Shouldn't caught an error");
            }
        });

        fs::remove_dir_all("tmp").unwrap();
        fs::remove_file("titi").unwrap();
    }

    // Test a basic "Hello World !" exchange
    // And the transfer of 1 file having a size of 1000000 bytes
    #[test]
    fn test_big_file() {
        let rt = Runtime::new().unwrap();

        let metadata = fs::metadata(String::from("tmp").clone());

        if let Ok(m) = metadata {
            if m.is_dir() {
                fs::remove_dir_all("tmp").unwrap();
            }
        }

        // File creation
        let result = OpenOptions::new().write(true).create_new(true).open("tata");

        if let Err(e) = result {
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                panic!("Couldn't create the file needed")
            }
        } else {
            let mut file = result.unwrap();
            let buf: &[u8] = &[1; 1000000];
            file.write_all(buf).unwrap();
            file.sync_data().unwrap();
        }

        rt.block_on(async {
            let mutex_server = Arc::new(Semaphore::new(0));
            let mutex_main = Arc::clone(&mutex_server);

            println!("Lauching server thread");
            tokio::spawn(async move {
                let mut server: ParallelServer =
                    ParallelServer::new(String::from("127.0.0.1:8891")).await;
                mutex_server.add_permits(1);
                println!("Server going in waiting loop");
                server.waiting_request().await;
            });

            let _ = mutex_main.acquire().await.unwrap();

            println!("Lauching client thread");

            let mut client: ParallelClient =
                ParallelClient::new(String::from("127.0.0.1:8891"), "Hello World!".to_string());
            client.add_files(vec!["tata".to_string()]);
            println!("Client going to start");
            let res = client.start_client().await;
            println!("Client finish");

            if let Ok(_s) = res {
                //assert_eq!(s, "Hello World!");
                let mut f = File::open("tmp/tata").unwrap();
                let mut data: Vec<u8> = Vec::new();
                f.read_to_end(&mut data).unwrap();

                assert_eq!(data.len(), 1000000);

                assert!(fs::metadata(String::from("tmp/tata").clone())
                    .unwrap()
                    .is_file());
            } else if let Err(e) = res {
                println!("Error caught : {}", e);
                panic!("Shouldn't caught an error");
            }
        });

        fs::remove_dir_all("tmp").unwrap();
        fs::remove_file("tata").unwrap();
    }
}
