use std::env;
use rust_parallel::entrycmd;

fn main() {
    let args: Vec<String> = env::args().collect();

    entrycmd::entrymod(&args);
}
