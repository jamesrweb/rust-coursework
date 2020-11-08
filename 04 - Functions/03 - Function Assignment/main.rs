fn factorial_imperative(n: u32) -> u32 {
  let mut result = 1;
  for n in 1..n + 1 { result = result * n; }
  return result;
}

fn factorial_recursive(n: u32) -> u32 {
  if n == 0 { return 1; }
  return n * factorial_recursive(n - 1);
}

fn main() {
  println!("Factorial 0: {}", factorial_recursive(0));
  println!("Factorial 1: {}", factorial_recursive(1));
  println!("Factorial 2: {}", factorial_recursive(2));
  println!("Factorial 3: {}", factorial_recursive(3));
  println!("Factorial 4: {}", factorial_recursive(4));
  println!("Factorial 5: {}", factorial_imperative(5));
  println!("Factorial 6: {}", factorial_imperative(6));
  println!("Factorial 7: {}", factorial_imperative(7));
  println!("Factorial 8: {}", factorial_imperative(8));
  println!("Factorial 9: {}", factorial_imperative(9));
}