use std::io;

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

  match io::stdin().read_line(&mut value) {
    Ok(_bytes) => trim_newline(&mut value),
    Err(error) => println!("error: {}", error)
  }

  return value.trim().to_string();
}

fn vowel_checker() {
  let input: char = user_value("Enter a character").parse().expect("Must be a single character");

  if ['a', 'e', 'i', 'o', 'u'].contains(&input) {
    println!("You entered a vowel, thanks!");
  } else {
    println!("That wasn't a vowel... oh well, still a nice character eh?")
  }
}

fn calculator() {
  println!("Let's do a calculation:");
  let left: i32 = user_value("Enter a number").parse().expect("Must be a number");
  let right: i32 = user_value("Enter another number").parse().expect("Must be a number");
  let operation: String = user_value("Enter an operation to run on these numbers (+, -, /, *)");

  if operation == "+" {
    println!("{} + {} = {}", left, right, left + right);
  } else if operation == "-" {
    println!("{} - {} = {}", left, right, left - right);
  } else if operation == "/" {
    println!("{} / {} = {}", left, right, left / right);
  } else if operation == "*" {
    println!("{} * {} = {}", left, right, left * right);
  } else {
    println!("Invalid operation entered.. whoops!");
  }
}

fn grade_converter() {
  let score: i32 = user_value("What percentage did you score on your test?").parse().expect("Must be a number");

  if score >= 90 {
    println!("You got an A");
  } else if score >= 80 {
    println!("You got a B");
  } else if score >= 70 {
    println!("You got a C");
  } else if score >= 60 {
    println!("You got a D");
  } else {
    println!("Looks like you failed the test it would seem, better luck next time!")
  }
}

fn main() {
  vowel_checker();
  calculator();
  grade_converter();
}