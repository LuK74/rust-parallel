use rust_parallel::parallel::Parallel;
use std::env;
use rust_parallel::core::jobmanager::JobManager;
use rust_parallel::core::interpretor;
use rust_parallel::core::parser;

fn main() {
    #[cfg(debug_assertions)]
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mut prg = Parallel::new();
    prg.new_cmd(args[1..args.len()].to_vec());
    prg.start();
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

#[test]
fn test_interpretor() {
    let mut jm = JobManager::new();

    let args: Vec<String> = vec![
        String::from("--keep-order"),
        String::from("--dry-run"),
        String::from("-j"),
        String::from("3"),
    ];

    interpretor::interpret(&mut jm, args);

    println!("{}", jm);
}

#[test]
fn test_parser() {
    parser::parse();
}
