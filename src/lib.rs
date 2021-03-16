pub mod core;
pub mod parallel;
pub mod remote;

// an `extern crate` loading macros must be at the crate root
#[macro_use] extern crate pest_derive;