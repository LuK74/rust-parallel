use std::fmt;

/**
    Test structure
*/
struct InputOption {
    //// On the left the input type like ":::"
    //// On the right the input file or data like "1 2 3 4" for example
    input_data : Vec<(String, Vec<String>)>,

    current_index : usize,
}

impl fmt::Display for InputOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.input_data.len() {
            let _r = write!(f, "\n{} ", self.input_data[i].0);
            for j in 0..(self.input_data[i].1).len() {
                let _r = write!(f, "{} ", (self.input_data[i].1)[j]);
            }
        }
        Ok(())
    }
}

impl InputOption {
    fn new() -> Self { 
        InputOption {
            input_data : Vec::new(),
            current_index : 0,
        }
    }

    fn clone(self : &mut Self) -> Self {
        InputOption {
            input_data : self.input_data.clone(),
            current_index : self.current_index,
        }
    }

    fn add_input_data(self : &mut Self, data : &String) {
        self.input_data[self.current_index-1].1.push(data.clone());
    }

    fn add_new_uplet(self : &mut Self, input_option : &String) {
        self.input_data.push((input_option.clone(), Vec::new()));
        self.current_index += 1;
    }
}

/**
    Standard number parser
*/ 
fn parse_number(word : String) -> Result<u32, String> {
    let mut iter = word.chars();
    let mut res = 0;
    while let Some(c) = iter.next() {
        if c.is_numeric() {
            if let Some(n) = c.to_digit(10) {
                res = res * 10 + n;
            } else {
                return Err(String::from("Error converting char to u32"));
            }
        } else {
            return Err(String::from("Error word contains a non numerical char"));
        }
    }
    return Ok(res);
}

/**
    Example of how we could make our parsing functions
    If the input words follow the grammar linked to this function
    we'll return an Ok() containing some structure [need to be define]
    If not, we'll return an Error containing a string, just a standard error message
*/
fn parse_next_input(words : &mut Vec<String>, input_option : &mut Option<&mut InputOption>) -> Result<InputOption, String> {

    /****
     * This case need to return Ok when this parsing function
     * can parse an "Empty token"
     * If not, we'll return an Error
     */
    if words.len() <= 0 {
        //// don't know what to return yet
        if let Some(input_opt) = input_option {
            return Ok(input_opt.clone());
        } else {
            return Err(String::from("Something went wrong during the parsing"));
        }
    }

    //// Remove we'll get the first word and remove it from the vector
    let first_word = words.remove(0);
    let input_opt;

    if let Some(current_input_opt) = input_option {
        input_opt = current_input_opt;
        input_opt.add_input_data(&first_word);
    } else {
        return Err(String::from("Something went wrong during the parsing"));
    }

    //// First rule define in the grammar linked to this function
    let mut res = parse_input_option(words, &mut Some(&mut input_opt.clone()));

    //// If the first rule failed, we can retrieve the words used in the
    //// previous function, in order to use it in the next one
    if let Err(_err) = res {   
        //// Second rule define in the grammar linked to this function   
        
        res = parse_next_input(words, &mut Some(&mut input_opt.clone()));
        //// If the second rule fail too, we'll rebuild the vector in order to
        //// return it to the upper function
        if let Err(err_next) = res {
            //// Rebuild the vector
            words.insert(0, first_word);
            return Err(err_next);
        } else {
            return res;
        }
    } else {
        return res;
    }
    

}

fn parse_input_option(words : &mut Vec<String>, input_option : &mut Option<&mut InputOption>) -> Result<InputOption, String> {
    if words.len() <= 0 {
        return Err(String::from("No more words to parse"));
    }

    let first_word = words.remove(0);
    let mut new_input_opt;

    if let Some(input_opt) = input_option {
        new_input_opt = input_opt.clone();
        new_input_opt.add_new_uplet(&first_word);
    } else {
        new_input_opt = InputOption::new();
        new_input_opt.add_new_uplet(&first_word);
    }
   
    match first_word.as_str() {
        ":::" => {
            let parse_result = parse_next_input(words, &mut Some(&mut new_input_opt.clone()));
            if let Err(err) = parse_result {
                //// in this one we only have one rule, so we'll return the error
                words.insert(0, first_word);
                return Err(err);
            } else {
                return parse_result;
            }
        }
        _ => {
            words.insert(0, first_word);
            return Err(String::from("Couldn't parse this input option"));
        },
    }

}

pub fn display_test() {
    let test_str = String::from("::: 1 2 3 4 ::: *.txt ::: 123 4 *.txt");
    let mut words : Vec<String> = test_str.split_whitespace().map(|x| x.to_string()).collect();

    let res = parse_input_option(&mut words, &mut None);
    if let Ok(input_opt) = res {
        println!("{}", input_opt);
    } else if let Err(err) = res{
        println!("{}", err);
    }
}

pub fn test(){
    println!("Hello, I am the parser controler.");
}