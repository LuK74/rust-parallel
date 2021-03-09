



pub struct Interpretor {
    jobMan: &JobManager,
}

impl Interpretor {

    pub fn new(jm: &JobManager) -> Interpretor {
        Interpretor { jobMan: jm }
    }

    pub fn interpret(&self , s: Vec<String> ) {
        let mut nb_thread: Option<usize> = None;
        let mut dry_run : bool = false;
        let mut keep_order : bool = false;
        let iter = s.iter();

        for arg in iter {
            match arg {
                String::from("--keep-order") => {
                    keep_order = true;
                } 
                String::from("--dry-run") => {
                    dry_run = true;
                }
                String::from("--jobs") | String::from("-j") => {
                    let nb_str = iter.next();
                    let nb = from_str<usize>(&nb_str);
                    nb_thread = Some(nb);
                } 
            }
        }

        self.jobMan.set_exec_env(nb_thread, dry_run,keep_order);
    }


}

#[test]
fn test_interpretor() {
    let mut jm = JobManager::new();
    let mut int = Interpretor::new(&jm);

    let args: Vec<String> = vec![
        String::from("--keep-order"),
        String::from("--dry-run"),
        String::from("-j"),
        String::from("3"),
    ];

    int.interpret(args);
    println!("{}", jm.nb_thread);
    println!("{}", jm.keep_order);
    println!("{}", jm.dry_run);
}