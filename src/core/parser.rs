extern crate pest;

use pest::Parser;
use pest::error::Error;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "core/parallel.pest"]
pub struct ParallelParser;

pub fn parse(raw_string: &String) -> Result<Pairs<Rule>, Error<Rule>> {   
    let parse_result = ParallelParser::parse(Rule::main, &raw_string);
    let inputs = match parse_result {
        Ok(pairs) => pairs,
        Err(error) => {
            eprintln!("The arguments entered are incorrect.");
            return Err(error);
        }
    };
    return Ok(inputs);
}