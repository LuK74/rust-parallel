use super::jobmanager::JobManager;

// To see the avaible Rules & Pairs from the grammar:
#[derive(Parser)]
#[grammar = "core/parallel.pest"]
pub struct ParallelParser;
use pest::iterators::Pairs;

// fn orderer (vector : Vec<Vec<&str>>) -> Vec<usize> {
//     let mut vec_new = vector.clone();
//     let mut vec_ordered = Vec::new();
//     let mut max = 0;
//     let mut index = 0; // index memorized inside the returned vector
//     let mut j = 0;
//     while vec_ordered.len() != vector.len() {
//         for elem in &vec_new {
//             if elem.len() > max {
//                 max = elem.len();
//                 index = j;
//             }
//             j = j + 1;
//         }
//         // j and max are reinitialized
//         j = 0;
//         max = 0;
//         // we add the index of the vector with the biggest size
//         vec_ordered.push(index);
//         // we substitute the vector with the biggest size with an empty vector
//         vec_new.remove(index);
//         vec_new.insert(index, Vec::new());
//     }
//     return vec_ordered;
// }

fn create_job(_command : String) {
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

pub fn interpret(job_man : &mut JobManager , inputs: &mut Pairs<Rule> ) {
    let mut nb_thread: Option<usize> = None;
    let mut dry_run : bool = false;
    let mut keep_order : bool = false;
    let mut separators : Vec<Vec<&str>> = Vec::new();
    let mut combinations : Vec<Vec<&str>> = Vec::new();
    let mut command_pattern : Option<Pairs<Rule>> = None;

    for pair in inputs.next()/*we skip "parallel" terminal term*/.unwrap().into_inner() {
        match pair.as_rule() {
            Rule::options => {
                let opt = pair.as_str();
                match opt {
                    "--keep-order" => keep_order = true,
                    "--dry-run" => dry_run = true,
                    "--jobs" | "-j" => nb_thread = Some(pair.into_inner().next().unwrap().as_str().parse::<usize>().unwrap()), // never fails
                    "--pipe" => /*TODO*/(),
                    "--help" => /*TODO return*/(),
                    _ => unreachable!(),
                }
            }
            Rule::commands => command_pattern = Some(pair.into_inner()),
            Rule::separators => {
                let mut separator = Vec::new();
                for sep_value in pair.into_inner().skip(1)/*we skip the ":::" term*/ {
                    separator.push(sep_value.as_str());
                }
                separators.push(separator);
            }
            // some rules are not reachable from main rule, 
            // that is totaly normal according to the grammar.
            // this part of the code is unreachable.
            _ => unreachable!(),
        }
    }

    let vec_ordered = orderly_sorter(&separators);

    // build all possible combinations
    build_combinations(&mut combinations, &vec_ordered, 0, &separators, Vec::new());

    // Create all jobs here
    let mut pattern = command_pattern.unwrap(); // never fails
    for _combination in combinations {
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
    }

    #[test]
    fn builder_test2() {
    }
}