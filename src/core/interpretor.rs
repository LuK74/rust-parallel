/////////////////////////////////////////////////////////////////////////////////////
/// Disclaimer : this file works very closely with the grammar of rust-parallel,  ///
/// if the grammar changes the code below should be maintained accordingly.       ///
/////////////////////////////////////////////////////////////////////////////////////

use super::jobmanager::JobManager;

// To see the avaible Rules & Pairs from the grammar:
use super::parser::Rule;
use pest::iterators::Pairs;

fn create_job(command : &str) {
    println!("job created : '{}'", command);
    //TODO : create the job.
}

/// Builds all the possible combinations according to sep_val values.
/// 
/// ## PARAMS
/// - builds: the list containing all separators combinations
/// - sep_ord: vector of separators' order according to the number of elements
/// - depth: the depth in sep_ord (up to sep_val.len() - 1)
/// - sep_val: contains all the values of each separator
/// - curr_build: the current building combination of values
fn build_combinations<'a>(builds: &mut Vec<Vec<&'a str>>, sep_ord: & Vec<usize>, depth: usize, sep_val: & Vec<Vec<&'a str>>, curr_build: Vec<&'a str>) {
    for i_value in 0..sep_val[sep_ord[depth]].len() {
        let mut build = curr_build.clone();
        let value = sep_val[sep_ord[depth]][i_value];
        build.push(value);
        if depth == sep_ord.len() - 1 { // we are at the smallest separator here
            //the iteration stops here (no recursive call)
            builds.push(build); // we had the final combination.
        } else { //recursive call here
            build_combinations(builds, sep_ord, depth + 1, sep_val, build);
        }
    }
}

/// returned vector will contain the indexes of the vectors inside the given "vector"
/// classified from the longest vector to the smalest vector according to their sizes
fn orderly_sorter(vector: & Vec<Vec<&str>>) -> Vec<usize> {
    struct IndexAndValue {
        i: usize, //index
        v: usize, //value
    }

    let mut orderly_indexes = Vec::with_capacity(vector.len()); // contains the orderly indexes
    let mut max = IndexAndValue{i: 0, v: 0}; // the current max observed value at some index

    for _ /*discard value*/ in 0..vector.len() {
        for (index, vec) in vector.iter().enumerate() {
            let len = vec.len();
            if max.v < len && !orderly_indexes.contains(&index) {
                max.i = index;
                max.v = len;
            }
        }
        orderly_indexes.push(max.i);
        max = IndexAndValue{i: 0, v: 0}
    }

    orderly_indexes
}

pub fn interpret(job_man : &mut JobManager , inputs: &mut Pairs<Rule> ) /*TODO : return type*/ {
    let mut nb_thread: Option<usize> = None;
    let mut dry_run : bool = false;
    let mut keep_order : bool = false;
    let mut separators : Vec<Vec<&str>> = Vec::new();
    let mut combinations : Vec<Vec<&str>> = Vec::new();
    let mut command_pattern : String = String::from("");

    for pair in inputs.next().unwrap()/*safe to unwrap the main rule as it will never fail*/.into_inner() {
        match pair.as_rule() {
            Rule::options => {
                let mut opt_iter = pair.as_str().split_whitespace();
                match opt_iter.next().unwrap() {
                    "--keep-order" => keep_order = true,
                    "--dry-run" => dry_run = true,
                    // Never fails because the parse succeeded.
                    "--jobs" | "-j" => nb_thread = Some(opt_iter.next().unwrap().parse::<usize>().unwrap()),
                    "--pipe" => /*TODO*/(),
                    "--help" => /*TODO return*/(),
                    _ => unreachable!(),
                }
            }
            Rule::commands => command_pattern = String::from(pair.into_inner().as_str()),
            Rule::separators => {
                let mut separator = Vec::new();
                for sep_values in pair.into_inner().skip(1) /*we skip the "separator" rule*/ {
                    // here we are on "input" rule
                    match sep_values.as_rule() {
                        Rule::string => separator.push(sep_values.as_str()),
                        _ /*some digits*/ => {
                            for sep_value in sep_values.as_str().split_whitespace() {
                                separator.push(sep_value);
                            }
                        }
                    }
                }
                separators.push(separator);
            }
            // some rules are not reachable from main rule, 
            // that is totaly normal according to the grammar.
            // this part of the code is unreachable.
            _ => unreachable!(),
        }
    }

    // TODO : explain why we need this
    let vec_ordered = orderly_sorter(&separators);

    // build all possible combinations from separators values
    build_combinations(&mut combinations, &vec_ordered, 0, &separators, Vec::new());

    // Create all jobs here from the command's pattern
    for combination in combinations {
        // we un-quote special characters.
        let mut command = command_pattern.replace("'", "");

        // in parallel, having *no targets* or a *"{}"* target while having 
        // one or multiple seprators has the same behaviour has "{1}" for 
        // one separator, "{1} {2}" for two separators, "{1} {2} {3}" for
        // three separators, etc.
        let mut combo = String::from("");
        for value in &combination {
            combo.push_str(value);
            if combination.last().unwrap() != value { combo.push(' '); }
        }

        // we check if actual targets exist
        let mut target_exists = false;
        let open_braces = command.find('{').unwrap_or(0);
        let close_braces = command.find('}').unwrap_or(0);
        if open_braces < close_braces {
            let braces_content = command[open_braces+1..close_braces].parse::<usize>();
            match braces_content {
                Ok(_)  => target_exists = true,
                _ => target_exists = false,
            }
        }

        if !target_exists {
            command.push(' ');
            command.push_str(combo.as_str());
        } else {
            command = command.replace("{}", combo.as_str());
            // If we encounter special targets like "{1}" we replace them with
            // the right value of the combination.
            while target_exists {
                let open_braces = command.find('{').unwrap_or(0);
                let close_braces = command.find('}').unwrap_or(0);
                if open_braces < close_braces {
                    let braces_content = command[open_braces+1..close_braces].parse::<usize>();
                    match braces_content {
                        Ok(value)  => {
                            if value <= combination.len() {
                                command.replace_range(open_braces..=close_braces, combination[value - 1]);
                            } else {
                                target_exists = false;
                            }
                        }
                        _ => target_exists = false,
                    }
                } else {
                    target_exists = false;
                }
            }
        }
        create_job(&command);
    }

    job_man.set_exec_env(nb_thread, dry_run, keep_order);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orderer_test() {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut vec_main = Vec::new();
        vec_main.push(vec!["hi", "hi", "hi"]);
        vec_main.push(vec!["hi"]);
        vec_main.push(vec!["hi", "hi", "hi"]);
        vec_main.push(vec!["hi", "hi"]);
        vec_main.push(vec!["hi", "hi", "hi", "hi", "hi"]);
        let order_vector = orderly_sorter(&vec_main);
        assert_eq!(order_vector[0], 4);
        assert_eq!(order_vector[1], 0);
        assert_eq!(order_vector[2], 2);
        assert_eq!(order_vector[3], 3);
        assert_eq!(order_vector[4], 1);
    }

    #[test]
    fn builder_test1() {
        let mut jm = JobManager::new();
        let mut parsing_result1 = super::super::parser::parse("parallel echo ::: 1 2 3").unwrap();
        let mut parsing_result2 = super::super::parser::parse("parallel echo -i {2}{1} ok';'wc -l ::: 1 2 3 ::: 4 5 6").unwrap();
        let mut parsing_result3 = super::super::parser::parse("parallel --jobs 5 --dry-run echo -i {1}{} ok';'wc -l ::: 1 2 3 ::: 4 5 6").unwrap();
        let mut parsing_result4 = super::super::parser::parse("parallel --jobs 5 --dry-run echo -i {1}{} ok';'wc -l").unwrap();
        let mut parsing_result5 = super::super::parser::parse("parallel --keep-order --dry-run echo -i {2}{} ok';'wc -l ::: 1 2 3").unwrap();
        interpret(&mut jm, &mut parsing_result1);
        interpret(&mut jm, &mut parsing_result2);
        interpret(&mut jm, &mut parsing_result3);
        interpret(&mut jm, &mut parsing_result4);
        interpret(&mut jm, &mut parsing_result5);
    }

    #[test]
    fn builder_test2() {
    }
}