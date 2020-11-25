enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25
  }
}

fn main() {
  println!("1 Penny = {} cents", value_in_cents(Coin::Penny));
  println!("1 Nickel = {} cents", value_in_cents(Coin::Nickel));
  println!("1 Dime = {} cents", value_in_cents(Coin::Dime));
  println!("1 Quarter = {} cents", value_in_cents(Coin::Quarter));
}