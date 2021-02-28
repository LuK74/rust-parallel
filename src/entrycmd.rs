use std::io::{stdin, stdout, Write};
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
        print!("$> ");
        let _r = stdout().flush();
        buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(_line) => {
                let tmp: Vec<String> = buffer.split_whitespace().map(String::from).collect();

                // if the user puts an enter
                // then do not consider the line 
                if tmp.len() == 0 {
                    continue;
                }

                // if the first word of the line is "exit"
                // then close the terminal 
                if tmp[0].eq("exit") {
                    break;
                }
                tab.push(tmp);
            }, 
            Err(e) => {
                println!("Error : {:?}",e);
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