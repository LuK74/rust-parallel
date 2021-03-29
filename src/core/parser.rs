extern crate pest;

use pest::Parser;
use pest::error::Error;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "core/parallel.pest"]
pub struct ParallelParser;

pub fn parse(raw_string: &str) -> Result<Pairs<Rule>, Error<Rule>> {   
    let parse_result = ParallelParser::parse(Rule::main, raw_string);
    let inputs = match parse_result {
        Ok(pairs) => pairs,
        Err(error) => {
            eprintln!("The arguments entered are incorrect.");
            return Err(error);
        }
    };
    return Ok(inputs);
}

#[test]
#[should_panic]
fn builder_test_panic0() {
    parse("paralll echo ::: 1 2 3").unwrap();
    //          ^- error here
}

#[test]
#[should_panic]
fn builder_test_panic1() {
    parse("parallel ::: 1 2 3").unwrap();
}

#[test]
#[should_panic]
fn builder_test_panic2() {
    // asking for help make the parser yelling anyway because
    // of the awaited command to be specified, and in fine, the
    // behaviour remains the same : display usage.
    parse("parallel --help").unwrap();
}

#[test]
#[should_panic]
fn builder_test_panic3() {
    //command missing in a complex string
    parse("parallel --dry-run --jobs 5 ::: 1 2 3 ::: A B C DE").unwrap();
}