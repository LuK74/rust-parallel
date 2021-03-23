pub mod channel;
pub mod client;
pub mod server;

#[cfg(test)]
mod tests {
    use crate::remote::client::*;
    use crate::remote::server::*;
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::sync::Arc;
    use tokio::runtime::Runtime;
    use tokio::sync::oneshot;
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

            let (tx, rx) = oneshot::channel();

            tokio::spawn(async move {
                let mut server: ParallelServer =
                    ParallelServer::new(String::from("127.0.0.1:8888")).await;
                mutex_server.add_permits(1);
                server.waiting_request().await;
            });

            mutex_main.acquire().await.unwrap();

            tokio::spawn(async move {
                let mut client: ParallelClient =
                    ParallelClient::new(String::from("127.0.0.1:8888"), "Hello World!".to_string());
                let server_response = client.start_client().await;

                tx.send(server_response).unwrap();
            });

            let res = rx.await.unwrap();

            if let Ok(s) = res {
                assert_eq!(s, "Hello World!");
            } else if let Err(e) = res {
                println!("Error caught : {}", e);
                panic!("Shouldn't caught an error");
            }
        });
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
        let result = OpenOptions::new().write(true).create_new(true).open("toto2");

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

            let (tx, rx) = oneshot::channel();

            tokio::spawn(async move {
                let mut server: ParallelServer =
                    ParallelServer::new(String::from("127.0.0.1:8888")).await;
                mutex_server.add_permits(1);
                server.waiting_request().await;
            });

            mutex_main.acquire().await.unwrap();

            tokio::spawn(async move {
                let mut client: ParallelClient =
                    ParallelClient::new(String::from("127.0.0.1:8888"), "Hello World!".to_string());
                client.add_files(vec!["toto".to_string(), "toto2".to_string()]);
                let server_response = client.start_client().await;

                tx.send(server_response).unwrap();
            });

            let res = rx.await.unwrap();

            if let Ok(s) = res {
                assert_eq!(s, "Hello World!");
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

        rt.block_on(async {
            let mutex_server = Arc::new(Semaphore::new(0));
            let mutex_main = Arc::clone(&mutex_server);

            let (tx, rx) = oneshot::channel();

            tokio::spawn(async move {
                let mut server: ParallelServer =
                    ParallelServer::new(String::from("127.0.0.1:8888")).await;
                mutex_server.add_permits(1);
                server.waiting_request().await;
            });

            mutex_main.acquire().await.unwrap();

            tokio::spawn(async move {
                let mut client: ParallelClient =
                    ParallelClient::new(String::from("127.0.0.1:8888"), "Hello World!".to_string());
                client.add_files(vec!["toto".to_string(), "toto".to_string()]);
                let server_response = client.start_client().await;

                tx.send(server_response).unwrap();
            });

            let res = rx.await.unwrap();

            if let Ok(s) = res {
                assert_eq!(s, "Hello World!");
                assert!(fs::metadata(String::from("tmp/toto").clone())
                    .unwrap()
                    .is_file());
                assert!(fs::metadata(String::from("tmp/toto(1)").clone())
                    .unwrap()
                    .is_file());
            } else if let Err(e) = res {
                println!("Error caught : {}", e);
                panic!("Shouldn't caught an error");
            }
        });
    }
}
