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