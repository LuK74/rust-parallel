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
fn test_echo1() {
    let _ = env_logger::builder().is_test(true).try_init();

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
    let _ = env_logger::builder().is_test(true).try_init();

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
    let _ = env_logger::builder().is_test(true).try_init();

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
