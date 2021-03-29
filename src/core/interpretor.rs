/////////////////////////////////////////////////////////////////////////////////////
/// Disclaimer : this file works very closely with the grammar of rust-parallel,  ///
/// if the grammar changes the code below should be maintained accordingly.       ///
/////////////////////////////////////////////////////////////////////////////////////

use super::jobmanager::JobManager;
use super::job::Job;

// To see the avaible Rules & Pairs from the grammar:
use super::parser::Rule;
use pest::iterators::Pairs;

/// All interpretation errors that can be created by the complexity of
/// parallel that the parser can not see. Returned by using the function
/// interpret.
pub enum InterpretError {
    Help,
    NoData(String),
}

/// Builds all the possible combinations according to sep_val values.
/// 
/// ## PARAMS
/// - builds: the list containing all separators combinations
/// - sep_ord: vector of separators' order according to the number of elements
/// - depth: the depth in sep_ord (up to sep_val.len() - 1)
/// - sep_val: contains all the values of each separator
/// - curr_build: the current building combination of values
fn build_combinations<'a>(builds: &mut Vec<Vec<&'a str>>, sep_ord: & Vec<usize>, 
                          depth: usize, sep_val: & Vec<Vec<&'a str>>, curr_build: Vec<&'a str>) {
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

pub fn interpret(job_man : &mut JobManager , inputs: &mut Pairs<Rule> ) -> Result<(), InterpretError> {
    let mut nb_thread: Option<usize> = None;
    let mut dry_run : bool = false;
    let mut keep_order : bool = false;

    let mut separators : Vec<Vec<&str>> = Vec::new();
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
                    "--pipe" => /*has no effect yet.*/(),
                    "--help" => return Err(InterpretError::Help),
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

    // TODO : add input in separators values.

    if separators.len() > 0 {
        // to properly imbricate the loops that will be used to create 
        // all the possible combinations, we must sort a vector to know 
        // which separator to look first, then second, and so on ... 
        let vec_ordered = orderly_sorter(&separators);

        // a vector that will contain all possible combinations
        let mut combinations : Vec<Vec<&str>> = Vec::new();
        // build all possible combinations from separators values
        build_combinations(&mut combinations, &vec_ordered, 0, &separators, Vec::new());

        // Create all jobs here from the command's pattern
        create_all_jobs(job_man, &combinations, command_pattern);
    } else {
        return Err(InterpretError::NoData(String::from("you forgot ::: or to pipe data into parallel")));
    }

    job_man.set_exec_env(nb_thread, dry_run, keep_order);
    Ok(())
}

fn create_job(job_man : &mut JobManager, command : &str) {
    let job = Job::new(command.split_whitespace().map(String::from).collect());
    job_man.add_job(job);
}

fn create_all_jobs(job_man : &mut JobManager, combinations : &Vec<Vec<&str>>, command_pattern : String) {
    for combination in combinations {
        // we un-quote special characters.
        let mut command = command_pattern.replace("'", "");

        // in parallel, having no targets or a "{}" target while having 
        // one or multiple seprators has the same behaviour has "{1}" for 
        // one separator, "{1} {2}" for two separators, "{1} {2} {3}" for
        // three separators, etc.
        let mut combo = String::from("");
        for value in combination {
            combo.push_str(value);
            if combination.last().unwrap() != value { combo.push(' '); }
        }

        // we check if actual targets exist
        let mut target_exists = false;
        let open_braces = command.find('{').unwrap_or(0);
        let close_braces = command.find('}').unwrap_or(0);
        if open_braces < close_braces {
            // braces exists in a good order, but the 
            // content will be checked after in the while loop
            target_exists = true; 
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
                                // The value is above the separator's index
                                // ex : specifying target {3} while only two dimensions were specified.
                                // So we ignore the target and erase it as parallel would do the same.
                                command.replace_range(open_braces..=close_braces, "");
                            }
                        }
                        // an error occured while parsing the usize
                        _ => target_exists = false,
                    }
                } else {
                    // no valid braces found !
                    target_exists = false;
                }
            }
        }
        create_job(job_man, &command);
    }
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
        // All parses below should succeed !
        let mut parsing_result1 = super::super::parser::parse("parallel echo ::: 1 2 3").unwrap();
        let mut parsing_result2 = super::super::parser::parse("parallel echo -i{2} -{2}{1} ok';'wc -l ::: 1 2 3 ::: 4 5 6").unwrap();
        let mut parsing_result3 = super::super::parser::parse("parallel --jobs 5 --dry-run echo -i {1}{} ok';'wc -l ::: 1 2 3 ::: 4 5 6").unwrap();
        let mut parsing_result4 = super::super::parser::parse("parallel --jobs 5 --dry-run echo -i {1}{} ok';'wc -l").unwrap();
        // With parallel there are no issues specifying multiple times the same option
        let mut parsing_result5 = super::super::parser::parse("parallel --keep-order --dry-run --jobs 5 --jobs 3 echo -i {2}{} ok';'wc -l ::: 1 2 3").unwrap();
        // And so the interpretation must not panic !
        let _ = interpret(&mut jm, &mut parsing_result1);
        let _ = interpret(&mut jm, &mut parsing_result2);
        let _ = interpret(&mut jm, &mut parsing_result3);
        let _ = interpret(&mut jm, &mut parsing_result4);
        let _ = interpret(&mut jm, &mut parsing_result5);
    }

    #[test]
    fn builder_test2() {
        let mut jm = JobManager::new();
        // "parallel echo :::" Must pass throught parsing but not interpretation
        let mut parsing_result6 = super::super::parser::parse("parallel echo :::").unwrap();
        match interpret(&mut jm, &mut parsing_result6) {
            Err(InterpretError::NoData(_)) => (),
            _ => panic!()
        }
    }

    #[test]
    fn builder_test3() {
        let mut jm = JobManager::new();
        // "parallel --help echo ::: 1 2 3" Must pass throught parsing but not interpretation
        let mut parsing_result6 = super::super::parser::parse("parallel --help echo ::: 1 2 3").unwrap();
        match interpret(&mut jm, &mut parsing_result6) {
            Err(InterpretError::Help) => (),
            _ => panic!()
        }
    }
}