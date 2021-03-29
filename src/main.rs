use rust_parallel::parallel::Parallel;
use std::env;
use rust_parallel::core::jobmanager::JobManager;
use rust_parallel::core::interpretor;

fn main() {
    #[cfg(debug_assertions)]
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    let mut prg = Parallel::new();
    prg.new_cmd(args[1..args.len()].to_vec());
    prg.start();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn lib_test() {
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
}
