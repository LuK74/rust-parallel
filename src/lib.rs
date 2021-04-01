pub mod core;
pub mod parallel;
pub mod remote;

// an 'extern crate' loading macros must be at the crate root
// according to rust rules because macros are handled early 
// enough in the compilation stage that order matters.
#[macro_use] extern crate pest_derive; // for parsing with pest