use rust_parallel::*;

fn main() {
    remote::server::test();
    parallel::test();
    core::parser::display_test();
}
