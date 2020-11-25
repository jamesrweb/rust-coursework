#[derive(Debug)]
enum Fruits {
  Apple = 0,
  Mango = 10,
  Watermelon = 20
}

fn main() {
  let mango = Fruits::Mango;
  // Wrong - returns type label
  println!("A mango costs £{:?}", mango);
  // Right - returns associated value
  println!("A mango costs £{:?}", mango as u8);
}