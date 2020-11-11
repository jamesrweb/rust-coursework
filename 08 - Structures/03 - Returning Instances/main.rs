#[derive(Debug)]
struct User {
  username: String,
  email: String,
  age: u8
}

fn build_user(name: String, age: u8, email: String) -> User {
  return User {
    username: name,
    age: age,
    email: email
  };
}

fn main() {
  let user = build_user(String::from("user"), 26, String::from("user@email.com"));
  println!("{:?}", user);
}