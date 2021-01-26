extern crate assignment;

use assignment::{facebook, food};

fn main() {
  println!("Fruit: {:?}", food::fruits::mango());
  println!("Vegetable: {:?}", food::vegetables::carrot());
  println!("Logged in status: {:?}", facebook::login("username"));
  println!("Posted status: {:?}", facebook::post("username", "my post"));
  println!("Logged out status: {:?}", facebook::logout("username"));
}
