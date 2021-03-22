use super::jobmanager::JobManager;
use log::debug;

// To see the avaible Rules & Pairs:
#[derive(Parser)]
#[grammar = "core/parallel.pest"]
pub struct ParallelParser;
use pest::iterators::Pairs;

fn create_job(command : String) {
    //TODO
}

pub fn interpret(job_man : &mut JobManager , inputs: &mut Pairs<Rule> ) {
    let mut nb_thread: Option<usize> = None;
    let mut dry_run : bool = false;
    let mut keep_order : bool = false;
    let mut command_pattern : Option<Pairs<Rule>> = None;
    let mut jobs : Vec<Vec<String>>;

    for pair in inputs.next()/*we skip "parallel"*/.unwrap().into_inner() {
        match pair.as_rule() {
            Rule::options => {
                let opt = pair.as_str();
                match opt {
                    "--keep-order" => keep_order = true,
                    "--dry-run" => dry_run = true,
                    "--jobs" | "-j" => nb_thread = Some(pair.into_inner().next().unwrap().as_str().parse::<usize>().unwrap()), // never fails
                    "--pipe" => /*TODO*/() ,
                    "--help" => /*TODO return*/() ,
                    _ => unreachable!(),
                }
            }
            Rule::commands => command_pattern = Some(pair.into_inner()),
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

    //Create all jobs here
    let mut pattern = command_pattern.unwrap(); // should never panic
    for job in jobs {
        let mut command : String = String::from("");
        for arg in &mut pattern {
            let raw = String::from(arg.as_str());
            command.push_str(match arg.into_inner().next().unwrap().as_rule() {
                Rule::target => "", //TODO: get the job[i] corresponding to {i} 
                Rule::quoted_char => "", //TODO: unquote the char
                Rule::string => raw.as_str(),
                _ => unreachable!(),
            })
        }
        create_job(command);
    }

    job_man.set_exec_env(nb_thread, dry_run, keep_order);
}