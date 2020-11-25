enum Coin {
  Penny(USState),
  Nickel(USState),
  Dime(USState),
  Quarter(USState)
}

#[derive(Debug)]
enum USState {
  Alaska,
  Arizona
}


fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny(state) => {
      println!("State: {:?}", state);
      1
    },
    Coin::Nickel(state) => {
      println!("State: {:?}", state);
      5
    },
    Coin::Dime(state) => {
      println!("State: {:?}", state);
      10
    },
    Coin::Quarter(state) => {
      println!("State: {:?}", state);
      25
    }
  }
}

fn main() {
  println!("1 Penny = {} cents", value_in_cents(Coin::Penny(USState::Alaska)));
  println!("1 Nickel = {} cents", value_in_cents(Coin::Nickel(USState::Arizona)));
  println!("1 Dime = {} cents", value_in_cents(Coin::Dime(USState::Alaska)));
  println!("1 Quarter = {} cents", value_in_cents(Coin::Quarter(USState::Arizona)));
}