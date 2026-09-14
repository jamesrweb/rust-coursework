fn method_mutation() -> String {
    let mut value = String::new();
    value.push_str("Hello"); // add a slice
    value.push(' '); // add a char
    value.push_str("World!"); // add a slice
    value
}

fn concatenation() -> String {
    let first = String::from("Hello");
    let second = String::from(" world!");
    first + &second
}

fn formatted() -> String {
    let first = String::from("Hello");
    let second = String::from("world!");
    format!("{} {}", first, second)
}

fn formatted_named() -> String {
    let first = String::from("Hello");
    let second = String::from("world!");
    format!("{first} {second}", second = second, first = first)
}

fn main() {
    println!("Int to string: {}", 1);
    println!("Str to string: string");
    println!("Float to string: {}", (1.02));
    println!("Mutated string: {}", method_mutation());
    println!("Concatenated strings: {}", concatenation());
    println!("Formatted strings: {}", formatted());
    println!("Named formatted strings: {}", formatted_named());
    println!(
        "String characters: {:?}",
        String::from("Hello world!").chars()
    );
}
