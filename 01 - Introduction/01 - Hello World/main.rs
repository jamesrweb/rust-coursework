fn greet(name: Option<&str>) -> String {
  match name {
    Some(name) => format!("{} {}!", "Hello", name),
    None => String::from("Hello World!")
  }
}

fn main() {
  println!("{}", greet(None));
  println!("{}", greet(Some("James")));
}