fn plus_one(value: Option<i32>) -> Option<i32> {
  match value {
    Some(num) => Some(num + 1),
    None => None
    // alternatively we could use a catch all like the default keyword in switch statements
    // _ => None
  }
}

fn main() {
  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  println!("{:?}, {:?}, {:?}", five, six, none);
}