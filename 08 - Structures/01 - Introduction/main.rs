#[derive(Debug)] // allows us to print instances of this structure
struct User { // name must uppercase the first letter
  username: String,
  email: String,
  age: u8
}

fn main() {
  let user = User { // Creates a User instance
    username: String::from("user"),
    email: String::from("user@email.com"),
    age: 50
  };

  println!("{:?}", user);
}