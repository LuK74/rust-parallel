use super::jobmanager::JobManager;
use log::debug;

// pub struct Interpretor {
//     jobMan: &JobManager,
// }

// impl Interpretor {

//     pub fn new(jm: &JobManager) -> Interpretor {
//         Interpretor { jobMan: jm }
//     }

//     pub fn interpret(&self , s: Vec<String> ) {
//         let mut nb_thread: Option<usize> = None;
//         let mut dry_run : bool = false;
//         let mut keep_order : bool = false;
//         let iter = s.iter();

//         for arg in iter {
//             match arg {
//                 String::from("--keep-order") => {
//                     keep_order = true;
//                 } 
//                 String::from("--dry-run") => {
//                     dry_run = true;
//                 }
//                 String::from("--jobs") | String::from("-j") => {
//                     let nb_str = iter.next();
//                     let nb = from_str<usize>(&nb_str);
//                     nb_thread = Some(nb);
//                 } 
//             }
//         }

//         self.jobMan.set_exec_env(nb_thread, dry_run,keep_order);
//     }


// }


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