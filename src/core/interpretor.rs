use super::jobmanager::JobManager;
use log::debug;

pub fn interpret(job_man : &mut JobManager , s: Vec<String> ) {
    let mut nb_thread: Option<usize> = None;
    let mut dry_run : bool = false;
    let mut keep_order : bool = false;

    for i in 0..s.len() {
        let arg = s[i].clone();
        if arg == "--keep-order" {
            keep_order = true;
        } else if arg == "--dry-run"  {
            dry_run = true;
        } else if arg == "--jobs" || arg == "-j" {
            let nb_str = s[i+1].clone();
            let result = nb_str.parse::<usize>();
            match result {
                Ok(u) => nb_thread = Some(u),
                Err(e) => {debug!("{}", e)}
            }
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