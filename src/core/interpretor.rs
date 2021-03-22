use super::jobmanager::JobManager;
//use log::debug;

// To see the avaible Rules & Pairs:
#[derive(Parser)]
#[grammar = "core/parallel.pest"]
pub struct ParallelParser;
use pest::iterators::Pairs;

struct JobValues {
    value: String,
    next_value: Option<Box<JobValues>>,
}

fn orderer (vector : Vec<Vec<&str>>) -> Vec<usize> {
    let mut vec_new = vector.clone();
    let mut vec_ordered = Vec::new();
    let mut max = 0;
    let mut index = 0; // index memorized inside the returned vector
    let mut j = 0;

    while vec_ordered.len() != vector.len() {
        for elem in &vec_new {
            if elem.len() > max {
                max = elem.len();
                index = j;
            }
            j = j + 1;
        }
        // j and max are reinitialized
        j = 0;
        max = 0;
        // we add the index of the vector with the biggest size
        vec_ordered.push(index);
        // we substitute the vector with the biggest size with an empty vector
        vec_new.remove(index);
        vec_new.insert(index, Vec::new());
    }
    return vec_ordered;
}



pub fn interpret(job_man : &mut JobManager , inputs: Pairs<Rule> ) {
    let mut nb_thread: Option<usize> = None;
    let mut dry_run : bool = false;
    let mut keep_order : bool = false;
    let mut vec_separator_values = Vec::new(); // contient les valeurs de chaque séparateur

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
            Rule::separators => { 
                // compter le nombre de séparateurs, on note s
                let mut vec_separator = Vec::new();
                for input in pairs.into_inner().skip(1) {
                    vec_separator.push(input.as_str());
                }
                vec_separator_values.push(vec_separator);
            }
            // some rules are not reachable from main rule, 
            // that is totaly normal according to the grammar.
            // this part of the code is unreachable.
            _ => unreachable!(),
        }
    }

    // vec_ordered will contain the indexes of the vectors inside vec_separator_values classified from the biggest vector to the 
    // smalest vector according to their sizes

    let mut vec_ordered = orderer(vec_separator_values);
                
                // construire_jobs(o, 0 /*indice actuel dans o, pour récuperer le numéro de séparateur*/, 
                //                 v, None /*liste des values pour un job*/)
                // <- to remove
                // TODO : build all possible combinations

                // println!("separator : {}", pairs.as_str());
                // for input in pairs.into_inner().skip(1) {
                //     println!("input : {}", input.as_str());
                // }

    job_man.set_exec_env(nb_thread, dry_run,keep_order);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orderer_test() {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut vec_main = Vec::new();
        vec_main.push(vec!["hi", "hi", "hi"]);
        vec_main.push(vec!["hi", "hi"]);
        vec_main.push(vec!["hi", "hi", "hi", "hi", "hi"]);
        let order_vector = orderer(vec_main);
        assert_eq!(order_vector[0], 2);
        assert_eq!(order_vector[1], 0);
        assert_eq!(order_vector[2], 1);
    }
}