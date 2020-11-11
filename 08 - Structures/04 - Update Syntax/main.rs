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
  let user_one = build_user(String::from("user"), 26, String::from("user@email.com"));
  let user_two = build_user(String::from("user"), 22, String::from("user@email.com"));

  let updated_user_two = User {
    email: user_two.email,
    username: user_two.username,
    ..user_one
  };

  println!("{:?}", updated_user_two);
}