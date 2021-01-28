use std::io::stdin;

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

fn documents() -> bool {
    let done = user_value("Did you submit your documents? (y/n)").to_ascii_lowercase();
    return done == "y";
}

fn fees() -> bool {
    let done = user_value("Did you submit your fees? (y/n)").to_ascii_lowercase();
    return done == "y";
}

fn main() {
    println!("Welcome to the application checker!\n");
    if documents() {
        println!("Thank You for submitting your documents.\n");
    } else {
        println!(
            "Sorry you have not submitted your documents, do this before using this service again."
        );
        std::process::exit(1);
    }

    if fees() {
        println!("\nThank You for submitting your fees.");
        println!("Admission Confirmed");
    } else {
        println!("Sorry You have not Submitted fees");
        println!("Admission Cancelled.");
        std::process::exit(1);
    }
}
