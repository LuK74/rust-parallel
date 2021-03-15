use rust_parallel::parallel;
use rust_parallel::parallel::Parallel;

use std::env;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args[1].eq("-c") {
        println!("Remote execution on Client Side");
        parallel::remote_exec_test(true, args[2..].to_vec()).await;
    } else if args[1].eq("-s") {
        println!("Remote execution on Server Side");
        parallel::remote_exec_test(false, args[2..].to_vec()).await;
    }

    /*let mut prg = Parallel::new();
    prg.new_cmd(args[1..args.len()].to_vec());
    prg.start();*/
}

#[test]
fn test_echo1() {
    // env_logger::init();

    let mut prg = Parallel::new();

    let args: Vec<String> = vec![
        String::from("echo"),
        String::from("Hello"),
        String::from("World"),
    ];
    prg.new_cmd(args);

    prg.start();
}

#[test]
fn test_echo2() {
    // env_logger::init();

    let mut prg = Parallel::new();

    let args: Vec<String> = vec![
        String::from("echo"),
        String::from("-e"),
        String::from("'Hello\nWorld'"),
    ];
    prg.new_cmd(args);

    prg.start();
}

#[test]
fn test_multi_echo() {
    // env_logger::init();

    let mut prg = Parallel::new();

    let args: Vec<String> = vec![
        String::from("echo"),
        String::from("Hello"),
        String::from("World"),
    ];
    prg.new_cmd(args);

    let args: Vec<String> = vec![
        String::from("echo"),
        String::from("-e"),
        String::from("'Hello\nWorld'"),
    ];
    prg.new_cmd(args);

    prg.start();
}
