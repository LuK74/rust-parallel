/**
    Test structure
*/
struct InputOption {
    m_inputData : Vec<(String, Vec<String>)>,
    m_currentIndex : usize,
}

impl InputOption {
    fn add_input_data(self : &mut Self, data : String) {
        self.m_inputData[self.m_currentIndex].1.push(data);
    }

    fn add_new_uplet(self : &mut Self, inputOption : String) {
        self.m_inputData.push((inputOption, Vec::new()));
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
    If not, we'll return an Error containing a 2-uplet
    - Left part : Vec<String>, correspond to the words that couldn't be
    parsed by this function. By doing so we'll allow the upper function
    (the one who called this one) to rebuild the Vector
    - Right part : String, just a standard error message
*/
fn parse_next_input(words : &mut Vec<String>, inputOption : Option<InputOption>) -> Result<InputOption, (Vec<String>, String)> {

    /****
     * This case need to return Ok when this parsing function
     * can parse an "Empty token"
     * If not, we'll return an Error
     */
    if words.len() <= 0 {
        //// don't know what to return yet
        return Ok(());
    }

    //// Remove we'll get the first word and remove it from the vector
    let mut firstWord = words.remove(0);

    //// First rule define in the grammar linked to this function
    let res = parse_input_option(words, inputOption);

    //// If the first rule failed, we can retrieve the words used in the
    //// previous function, in order to use it in the next one
    if let Err((remaining, err)) = res {   
        //// Second rule define in the grammar linked to this function   
        let res = parse_next_input(&mut remaining);
        //// If the second rule fail too, we'll rebuild the vector in order to
        //// return it to the upper function
        if let Err((remainingNext, errNext)) = res {
            //// Rebuild the vector
            remainingNext.insert(0, firstWord);
            return Err((remainingNext, errNext));
        } else {
            // insert return success case
        }
    } else {
        // insert return success case
    }
    

    return Err(());
}

fn parse_input_option(words : &mut Vec<String>, inputOption : Option<InputOption>) -> Result<InputOption, (Vec<String>, String)> {
    if words.len() <= 0 {
        return Err((words.clone(), String::from("No more words to parse")));
    }

    let mut firstWord = words.remove(0);
   
    match firstWord.as_str() {
        ":::" => return Ok(()),
        _ => {
            words.insert(0, firstWord);
            return Err((*words, String::from("Couldn't parse this input option")));
        },
    }

    return Ok(());
}

pub fn test(){
    println!("Hello, I am the parser controler.");
}