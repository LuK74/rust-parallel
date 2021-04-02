use rust_parallel::parallel::Parallel;
use std::env;

fn main() {
    #[cfg(debug_assertions)]
    env_logger::init();

    let shell = match env::var("SHELL") {
        Ok(val) => val,
        Err(e) => {
            eprintln!("couldn't interpret environment variable SHELL: {}", e);
            return;
        }
    };

    let args: Vec<String> = env::args().skip(1).collect();

    let mut prg = Parallel::new(shell, args);
    prg.start();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn lib_test() {
        let _ = env_logger::builder().is_test(true).try_init();

        let args: Vec<String> = vec![
            String::from("echo"),
            String::from(":::"),
            String::from("Hello"),
            String::from("World"),
        ];

        let mut prg = Parallel::new(String::from("/bin/bash"), args);

        prg.start();
    }
}
