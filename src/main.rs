use rust_parallel::parallel::Parallel;
use std::env;

fn main() {
    #[cfg(debug_assertions)]
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mut prg = Parallel::new();
    prg.new_cmd(args[1..args.len()].to_vec());
    prg.start();
}

#[test]
fn test_echo1(){
    let args: Vec<String> = vec![
        String::from("echo"), 
        String::from("Hello"),
        String::from("World")
    ];
    let mut prg = Parallel::new();
    prg.new_cmd(args);
    prg.start();
}

#[test]
fn test_echo2(){
    let args: Vec<String> = vec![
        String::from("echo"), 
        String::from("-e"), 
        String::from("'Hello\nWorld'"),
    ];
    let mut prg = Parallel::new();
    prg.new_cmd(args);
    prg.start();
}

