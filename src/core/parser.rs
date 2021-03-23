extern crate pest;

use pest::Parser;
use pest::error::Error;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "core/parallel.pest"]
pub struct ParallelParser;

pub fn parse(raw_string: &str) -> Result<Pairs<Rule>, Error<Rule>> {   
    let parse_result = ParallelParser::parse(Rule::main, &raw_string);
    let inputs = match parse_result {
        Ok(pairs) => pairs,
        Err(error) => {
            eprintln!("The arguments entered are incorrect.");
            return Err(error);
        }
    };
    return Ok(inputs);

    // let inputs = inputs.next().unwrap(); // get and unwrap the `main` rule; never fails
    // for pairs in inputs.into_inner() {
    //     match pairs.as_rule() {
    //         Rule::options => println!("option : {}", pairs.as_str()),
    //         Rule::commands => {
    //             println!("command : {}", pairs.as_str());
    //             for arg in pairs.into_inner().skip(1) /*skip the first item*/ {
    //                 print!("argument : {} (of type", arg.as_str());
    //                 let rule = arg.into_inner().next().unwrap().as_rule();
    //                 match rule {
    //                     Rule::target => println!(" target)"),
    //                     Rule::quoted_char => println!(" quoted char)"),
    //                     Rule::string => println!(" string)"),
    //                     _ => unreachable!(),
    //                 }
    //             }
    //         }
    //         Rule::separators => {
    //             println!("separator : {}", pairs.as_str());
    //             for input in pairs.into_inner().skip(1) {
    //                 println!("input : {}", input.as_str());
    //             }
    //         }
    //         // some rules are not reachable from main rule, 
    //         // that is totaly normal according to the grammar.
    //         _ => unreachable!(),
    //     }
    // }
    // return Ok(inputs);
}