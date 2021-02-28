use std::env;
mod entrycmd;

fn main() {
    let args: Vec<String> = env::args().collect();

    entrycmd::entrymod(&args);
}
