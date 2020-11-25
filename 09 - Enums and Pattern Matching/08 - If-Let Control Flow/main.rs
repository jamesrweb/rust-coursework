fn main() {
  let value = Some(3);

  if let Some(3) = value {
    println!("It's a three alright!");
  } else {
    println!("It's not a three i'm afraid..");
  }
}