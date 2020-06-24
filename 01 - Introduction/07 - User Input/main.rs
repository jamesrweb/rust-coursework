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

fn main() {
  let name: String = user_value("Enter your name");
  let age = user_value("Enter your age").parse::<i32>().expect("Age must be a number");
  // OR: let age: i32 = user_value("Enter your age").parse().expect("Age must be a number");
  let nationality: String = user_value("Enter your nationality");
  let job: String = user_value("Enter your job title");
  println!("\n{} is {}, {} years old and works as a {}.", name, nationality, age, job);
}