use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let file = match File::open("./src/input.txt") {
        Ok(handler) => handler,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("./src/input.txt") {
                Ok(handler) => handler,
                Err(error) => panic!("Cannot create file: {:?}", error),
            },
            error => panic!("Unable to open file: {:?}", error),
        },
    };

    println!("{:?}", file);
}
