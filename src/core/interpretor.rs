use super::jobmanager::JobManager;
use log::debug;

// To see the avaible Rules & Pairs:
#[derive(Parser)]
#[grammar = "core/parallel.pest"]
pub struct ParallelParser;
use pest::iterators::Pairs;

pub fn interpret(job_man : &mut JobManager , inputs: Pairs<Rule> ) {
    let mut nb_thread: Option<usize> = None;
    let mut dry_run : bool = false;
    let mut keep_order : bool = false;

    for pairs in inputs.next().unwrap().into_inner() {
        match pairs.as_rule() {
            Rule::options => {
                let opt = pairs.as_str();
                match opt {
                    "--keep-order" => keep_order = true,
                    "--dry-run" => dry_run = true,
                    "--jobs" | "-j" => nb_thread = Some(pairs.into_inner().next().unwrap().as_rule().as_str().parse::<usize>().unwrap()), // never fails
                    "--pipe" => /*TODO*/() ,
                    "--help" => /*TODO return*/() ,
                    _ => unreachable!(),
                }
            }
            Rule::commands => { () // <- to remove
                // println!("command : {}", pairs.as_str());
                // for arg in pairs.into_inner().skip(1) /*skip the command name*/ {
                //     print!("argument : {} (of type", arg.as_str());
                //     let rule = arg.into_inner().next().unwrap().as_rule();
                //     match rule {
                //         Rule::target => println!(" target)"),
                //         Rule::quoted_char => println!(" quoted char)"),
                //         Rule::string => println!(" string)"),
                //         _ => unreachable!(),
                //     }
                // }
            }
            Rule::separators => { () // <- to remove
                // TODO : build all possible combinations

                // println!("separator : {}", pairs.as_str());
                // for input in pairs.into_inner().skip(1) {
                //     println!("input : {}", input.as_str());
                // }
            }
            // some rules are not reachable from main rule, 
            // that is totaly normal according to the grammar.
            // this part of the code is unreachable.
            _ => unreachable!(),
        }
    }

    job_man.set_exec_env(nb_thread, dry_run,keep_order);
}

// pub fn interpret(job_man : &mut JobManager , s: Vec<String> ) {
//     let mut nb_thread: Option<usize> = None;
//     let mut dry_run : bool = false;
//     let mut keep_order : bool = false;
    
//     let mut optionInterpretation : bool = true;
//     let mut commandInterpretation : bool = false;
//     let mut valuesInterpretation : bool = false;
    
//     let mut jobVector : Vec<String> = Vec::new();
//     let mut values : Vec<String> = Vec::new();
    
//     for i in 0..s.len() {
//     let arg = s[i].clone();
//     // Commande not yet interpreted
//     if optionInterpretation {
//     if arg == "--keep-order" {
//     keep_order = true;
//     } else if arg == "--dry-run" {
//     dry_run = true;
//     } else if arg == "--jobs" || arg == "-j" {
//     let nb_str = s[i+1].clone();
//     let result = nb_str.parse::<usize>();
//     match result {
//     Ok(u) => nb_thread = Some(u),
//     Err(e) => {debug!("{}", e)}
//     }
//     } else if arg == ":::" {
//     panic!("Syntax error ! Parsed ::: before command");
//     } else {
//     optionInterpretation = false;
//     commandInterpretation = true;
//     jobVector.push(arg); // Push of the command's name
//     }
//     // One command at least interpreted
//     } else if commandInterpretation {
//     if arg == ";" || arg == "|" {
//     job_man.add_job(Job::new(&jobVector));
//     jobVector = Vec::new();
//     } else if arg == ":::" {
//     job_man.add_job(Job::new(&jobVector));
//     jobVector = Vec::new();
//     commandInterpretation = false;
//     valuesInterpretation= true;
//     } else {
//     jobVector.push(arg); // Push of the command's arguments
//     }
//     } else {
//     values.push(arg);
//     }
//     }
    
//     job_man.set_exec_env(nb_thread, dry_run,keep_order);
// }