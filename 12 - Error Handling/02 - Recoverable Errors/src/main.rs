use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::process;

fn main() -> Result<(), Error> {
    let mut file = match File::open("./src/input.txt") {
        Ok(handler) => handler,
        Err(_error) => {
            println!("Unable to read file!");
            process::exit(1);
        }
    };
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("{:?}", content);
    return Ok(());
}
