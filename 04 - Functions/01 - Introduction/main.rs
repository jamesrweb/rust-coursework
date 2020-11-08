fn fibonacci(n: u32) -> u32 {
  match n {
    0 => 1,
    1 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2)
  }
}

fn main() {
  println!("Fibonacci 0: {}", fibonacci(0));
  println!("Fibonacci 1: {}", fibonacci(1));
  println!("Fibonacci 2: {}", fibonacci(2));
  println!("Fibonacci 3: {}", fibonacci(3));
  println!("Fibonacci 4: {}", fibonacci(4));
  println!("Fibonacci 5: {}", fibonacci(5));
  println!("Fibonacci 6: {}", fibonacci(6));
  println!("Fibonacci 7: {}", fibonacci(7));
  println!("Fibonacci 8: {}", fibonacci(8));
  println!("Fibonacci 9: {}", fibonacci(9));
}