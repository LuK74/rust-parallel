extern crate pest;

use pest::Parser;

#[derive(Parser)]
#[grammar = "core/parallel.pest"]
pub struct ParallelParser;

use std::fs;

pub fn parse() {
    //dirty error handling here, just testing.
    let unparsed_file = fs::read_to_string("inputexample.txt").expect("cannot read file");
    
    let inputs = ParallelParser::parse(Rule::main, &unparsed_file)
    .expect("unsuccessful parse") // unwrap the parse result
    .next().unwrap(); // get and unwrap the `main` rule; never fails

    for token in inputs.into_inner() {
        match token.as_rule() {
            // some rules are not reachable from main rule, 
            // that is totaly normal according to the grammar.
            Rule::options => println!("option : {}", token.as_str()),
            Rule::commands => {
                println!("command : {}", token.as_str());
                for arg in token.into_inner().skip(1) /*skip the first item*/ {
                    print!("argument : {} (of type", arg.as_str());
                    let rule = arg.into_inner().next().unwrap().as_rule();
                    match rule {
                        Rule::target => println!(" target)"),
                        Rule::quoted_char => println!(" quoted char)"),
                        Rule::string => println!(" string)"),
                        _ => unreachable!(),
                    }
                }
            }
            Rule::separators => {
                println!("separator : {}", token.as_str());
                for input in token.into_inner().skip(1) {
                    println!("input : {}", input.as_str());
                }
            }
            _ => unreachable!(),
        }
    }

    println!("fin parse");
}