extern crate pest;

use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "core/parallel.pest"]
pub struct ParallelParser;

pub fn parse(raw_string: &str) -> Result<Pairs<Rule>, Error<Rule>> {
    let parse_result = ParallelParser::parse(Rule::main, raw_string);
    let inputs = match parse_result {
        Ok(pairs) => pairs,
        Err(error) => return Err(error),
    };
    return Ok(inputs);
}
