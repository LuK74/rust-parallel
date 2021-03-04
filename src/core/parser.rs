use std::fmt;

/**
    Test structure
*/
struct InputOption {
    //// On the left the input type like ":::"
    //// On the right the input file or data like "1 2 3 4" for example
    m_inputData : Vec<(String, Vec<String>)>,

    m_currentIndex : usize,
}

impl fmt::Display for InputOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.m_inputData.len() {
            let _r = write!(f, "\n{} ", self.m_inputData[i].0);
            for j in 0..(self.m_inputData[i].1).len() {
                let _r = write!(f, "{} ", (self.m_inputData[i].1)[j]);
            }
        }
        Ok(())
    }
}

impl InputOption {
    fn new() -> Self { 
        InputOption {
            m_inputData : Vec::new(),
            m_currentIndex : 0,
        }
    }

    fn clone(self : &mut Self) -> Self {
        InputOption {
            m_inputData : self.m_inputData.clone(),
            m_currentIndex : self.m_currentIndex,
        }
    }

    fn add_input_data(self : &mut Self, data : &String) {
        self.m_inputData[self.m_currentIndex-1].1.push(data.clone());
    }

    fn add_new_uplet(self : &mut Self, inputOption : &String) {
        self.m_inputData.push((inputOption.clone(), Vec::new()));
        self.m_currentIndex += 1;
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
fn parse_next_input(words : &mut Vec<String>, inputOption : &mut Option<&mut InputOption>) -> Result<InputOption, String> {

    /****
     * This case need to return Ok when this parsing function
     * can parse an "Empty token"
     * If not, we'll return an Error
     */
    if words.len() <= 0 {
        //// don't know what to return yet
        if let Some(inputOpt) = inputOption {
            return Ok(inputOpt.clone());
        } else {
            return Err(String::from("Something went wrong during the parsing"));
        }
    }

    //// Remove we'll get the first word and remove it from the vector
    let firstWord = words.remove(0);
    let mut inputOpt;

    if let Some(currentInputOpt) = inputOption {
        inputOpt = currentInputOpt;
        inputOpt.add_input_data(&firstWord);
    } else {
        return Err(String::from("Something went wrong during the parsing"));
    }

    //// First rule define in the grammar linked to this function
    let mut res = parse_input_option(words, &mut Some(&mut inputOpt.clone()));

    //// If the first rule failed, we can retrieve the words used in the
    //// previous function, in order to use it in the next one
    if let Err(err) = res {   
        //// Second rule define in the grammar linked to this function   
        
        res = parse_next_input(words, &mut Some(&mut inputOpt.clone()));
        //// If the second rule fail too, we'll rebuild the vector in order to
        //// return it to the upper function
        if let Err(errNext) = res {
            //// Rebuild the vector
            words.insert(0, firstWord);
            return Err(errNext);
        } else {
            return res;
        }
    } else {
        return res;
    }
    

    return Err(String::from("Something unexpected happened"));
}

fn parse_input_option(words : &mut Vec<String>, inputOption : &mut Option<&mut InputOption>) -> Result<InputOption, String> {
    if words.len() <= 0 {
        return Err(String::from("No more words to parse"));
    }

    let firstWord = words.remove(0);
    let mut newInputOpt;

    if let Some(inputOpt) = inputOption {
        newInputOpt = inputOpt.clone();
        newInputOpt.add_new_uplet(&firstWord);
    } else {
        newInputOpt = InputOption::new();
        newInputOpt.add_new_uplet(&firstWord);
    }
   
    match firstWord.as_str() {
        ":::" => {
            let parseResult = parse_next_input(words, &mut Some(&mut newInputOpt.clone()));
            if let Err(err) = parseResult {
                //// in this one we only have one rule, so we'll return the error
                words.insert(0, firstWord);
                return Err(err);
            } else {
                return parseResult;
            }
        }
        _ => {
            words.insert(0, firstWord);
            return Err(String::from("Couldn't parse this input option"));
        },
    }

    return Err(String::from("Something unexpected happened"));
}

pub fn display_test() {
    let testStr = String::from("::: 1 2 3 4 ::: *.txt ::: 123 4 *.txt");
    let mut words : Vec<String> = testStr.split_whitespace().map(|x| x.to_string()).collect();

    let res = parse_input_option(&mut words, &mut None);
    if let Ok(inputOpt) = res {
        println!("{}", inputOpt);
    } else if let Err(err) = res{
        println!("{}", err);
    }
}

pub fn test(){
    println!("Hello, I am the parser controler.");
}