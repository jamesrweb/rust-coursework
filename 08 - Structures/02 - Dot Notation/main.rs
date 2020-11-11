#[derive(Debug)]
struct User {
  username: String,
  email: String,
  age: u8
}

fn main() {
  let mut user = User {
    username: String::from("user"),
    email: String::from("user@email.com"),
    age: 50
  };

  println!("User information");
  println!("Username: {}", user.username);
  println!("Email: {}", user.email);
  println!("Age: {}", user.age);

  user.username = String::from("test");
  user.email = String::from("test@email.com");
  user.age = 22;

  println!("Updated values");
  println!("Username: {}", user.username);
  println!("Email: {}", user.email);
  println!("Age: {}", user.age);
}