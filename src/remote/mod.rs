pub mod channel;
pub mod client;
pub mod server;

#[cfg(test)]
mod tests {
    use crate::remote::client::*;
    use crate::remote::server::*;
    use std::sync::Arc;
    use tokio::runtime::Runtime;
    use tokio::sync::oneshot;
    use tokio::sync::Semaphore;

    #[test]
    fn test_exchange() {
        let rt = Runtime::new().unwrap();

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

    #[test]
    #[should_panic]
    fn test_exchange2() {
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            let mutex_server = Arc::new(Semaphore::new(0));
            let mutex_main = Arc::clone(&mutex_server);

            let (tx, rx) = oneshot::channel();

            tokio::spawn(async move {
                let mut server: ParallelServer =
                    ParallelServer::new(String::from("127.0.0.1:8889")).await;
                mutex_server.add_permits(1);
                server.waiting_request().await;
            });

            mutex_main.acquire().await.unwrap();

            tokio::spawn(async move {
                let mut client: ParallelClient =
                    ParallelClient::new(String::from("127.0.0.1:8889"), "Hello World!".to_string());
                let server_response = client.start_client().await;

                tx.send(server_response).unwrap();
            });

            let res = rx.await.unwrap();

            if let Ok(s) = res {
                assert_eq!(s, "Hello W!".to_string());
            } else if let Err(e) = res {
                println!("Error caught : {}", e);
                panic!("Shouldn't caught an error");
            }
        });
    }

    #[test]
    fn test_file_exchange() {
        let rt = Runtime::new().unwrap();

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
                client.add_files(vec!("toto".to_string()));
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
}
