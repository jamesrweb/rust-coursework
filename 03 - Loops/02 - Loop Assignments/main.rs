use std::io;

fn evens(from: i32, to: i32) {
  let mut start = from;
  while start <= to {
    if start % 2 == 0 {
      println!("{} is even", start);
      start += 2;
    } else {
      start += 1;
    }
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

fn count_digits(number: i32) {
  println!("{} has {} digits", number, number.to_string().len());
}

fn main() {
  evens(1, 100);
  quiz(
    String::from("What is the meaning of life?"),
    String::from("42"),
    3
  );
  count_digits(1034829102);
}