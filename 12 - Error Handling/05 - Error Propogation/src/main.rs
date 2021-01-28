use std::fs::File;
use std::io::{Error, Read};

fn read_without_question_mark(path: &str) -> Result<String, Error> {
    let mut file = match File::open(path) {
        Ok(handler) => handler,
        Err(error) => return Err(error),
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(error) => Err(error),
    }
}

fn read_with_question_mark(path: &str) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    return Ok(content);
}

fn main() {
    match read_without_question_mark("./src/input.txt") {
        Ok(content) => println!("{:?}", content),
        Err(error) => println!("{:?}", error),
    }

    match read_with_question_mark("./src/input.txt") {
        Ok(content) => println!("{:?}", content),
        Err(error) => println!("{:?}", error),
    }
}
