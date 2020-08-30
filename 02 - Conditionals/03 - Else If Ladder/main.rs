fn largest(a: i32, b: i32, c: i32) {
  if a > b && a > c {
    println!("A is the largest number");
  } else if b > a && b > c {
    println!("B is the largest number");
  } else {
    println!("C is the largest number");
  }
}

fn main() {
  largest(10, 20, 5);
  largest(100, 60, -30);
  largest(40, -20, 80);
}