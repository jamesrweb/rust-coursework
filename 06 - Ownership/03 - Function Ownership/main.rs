fn say(value: String) {
  println!("{}", value);
}

fn bark(dog: String) -> String {
  println!("The {} says woof!", dog);
  return dog;
}

fn squared(value: i32) -> i32 {
  return value.pow(2);
}

fn main() {
  // Add heap variable to scope
  let greeting = String::from("hello world!");

  // The say function takes ownership of the greeting variable since it is moved into it
  say(greeting);

  // Because the greeting variable has been moved into the say function, we cannot use it here anymore. This line, if used would throw, if used, due to this.
  // println!("{}", greeting);

  // If however we "pass back" the variable, this can work since ownership really just means giving access to the memory based on scope and so we can just pass the memory pointer back to the original variable if we really want to, for example:
  let mut dog = String::from("Staffie");
  dog = bark(dog);
  println!("{}", dog);

  // Add stack variable to scope
  let x = 5;

  // Instead of taking ownership, the function just copies in the value
  let answer = squared(x);

  // Thanks to this, we can still use the x variable here
  println!("{}^2 = {:?}", x, answer);
}