use std::{collections::HashMap, io::stdin};

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn user_value(request_text: &str) -> String {
    let mut value = String::new();
    println!("{}", request_text);

    match stdin().read_line(&mut value) {
        Ok(_bytes) => trim_newline(&mut value),
        Err(error) => println!("error: {}", error),
    }

    return value.trim().to_string();
}

fn main() {
    let mut contacts = HashMap::new();
    let mut counter = 1;
    let to_store = user_value("How may contacts do you want to store?")
        .parse::<i32>()
        .expect("Failed conversion");

    while counter <= to_store {
        let name = user_value("Who should we add to your contacts?");
        let phone_number = user_value("What is their phone number?");
        contacts.insert(name, phone_number);
        counter += 1;
    }

    let choice = user_value("What would you like to do now? (options: search, view, quit)")
        .to_ascii_lowercase();

    if choice == "search" {
        let name = user_value("Who would you like to view?");

        match contacts.get_key_value(&name) {
            Some((name, number)) => println!("{}:\n{}", name, number),
            None => println!("Contact not found!"),
        }
    } else if choice == "view" {
        println!("Contacts: {:#?}", contacts);
    } else if choice == "quit" {
        println!("Goodbye!")
    } else {
        panic!("Invalid choice submitted!");
    }
}
