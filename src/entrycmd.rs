use std::io::stdin;
// use std::io::SplitWhitespace;

pub fn entrymod(args: &[String]){
    if args.len() == 1 {
        shell();
    }else{
        no_shell(args);
    }
}

fn shell(){
    println!("rust-parallel: Warning: Input is read from the terminal.");
    println!("parallel: Warning: Enter 'exit' exec to exit.");

    let stdin = stdin();
    let mut buffer;
    let mut tab = vec![];

    loop {
        buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(_line) => {
                let tmp: Vec<String> = buffer.split_whitespace().map(String::from).collect();
                if tmp[0].eq("exit") {
                    break;
                }
                tab.push(tmp);
            }, 
            Err(e) => {
                panic!("Error : {:?}",e);
            }
        }
    }

    println!("Commands : ");
    for cmd in tab {
        println!("\t-{:?}", cmd);
    }
}

fn no_shell(args: &[String]){
    #[cfg(debug_assertions)]
    println!("{:?}", args);
}