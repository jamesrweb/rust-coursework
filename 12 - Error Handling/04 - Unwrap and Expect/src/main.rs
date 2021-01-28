use std::fs::File;

fn main() {
    // Calls panic!() for us with a default error message if something goes wrong
    // File::open("input.txt").unwrap();

    // Calls panic for us with a custom error message if something goes wrong
    File::open("input.txt").expect("A custom error message.");
}
