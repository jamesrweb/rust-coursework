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

fn even_or_odd(num: i64) -> String {
  if num % 2 == 0 {
    return String::from("even");
  }

  return String::from("odd");
}

fn main() {
  let start = user_value("Want to answer some questions?");

  if start.starts_with("n") {
    std::process::exit(0x0100);
  }

  let num: i64 = user_value("Enter a number to check if it is even or odd:").parse().expect("Must be a number");
  println!("{} is {}", num, even_or_odd(num));
  let movies = user_value("Do you like movies?");

  if movies.starts_with("y") {
    println!("Awesome, me too, my favourite is the one with the guy who does the thing!")
  } else {
    println!("That's a shame. I love them personally.");
  }

  let interests = user_value("What interests do you have?");
  println!("Cool!");
}