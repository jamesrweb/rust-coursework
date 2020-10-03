use std::io;

fn evens(from: i32, to: i32) {
  let mut counter = from;
  while counter <= to {
    if counter % 2 == 1 {
      counter += 1;
      continue;
    }

    println!("{} is even", start);
    counter += 2;
  }
}

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

fn quiz(question: String, answer: String, tries: i32) {
  let display = format!("{} ({} tries remaining)", question, tries);
  let guess: String = user_value(&display);

  if guess == answer {
    println!("Congratulations, you got it right!")
  } else if guess != answer && tries - 1 > 0 {
    quiz(question, answer, tries - 1);
  } else {
    println!("Better luck next time");
  }
}

fn count_digits(num: f32) -> f32 {
  return (num.log10() + 1.).floor();
}

fn main() {
  evens(1, 100);
  quiz(
    String::from("What is the meaning of life?"),
    String::from("42"),
    3
  );
  count_digits(1034829102.);
}
