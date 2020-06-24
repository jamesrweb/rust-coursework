fn add(left: f32, right: f32) -> f32 {
  return left + right;
}

fn subtract(left: f32, right: f32) -> f32 {
  return left - right;
}

fn divide(left: f32, right: f32) -> f32 {
  return left / right;
}

fn multiply(left: f32, right: f32) -> f32 {
  return left * right;
}

fn remainder(left: f32, right: f32) -> f32 {
  return left % right;
}

fn greater_than(left: f32, right: f32) -> bool {
  return left > right;
}

fn less_than(left: f32, right: f32) -> bool {
  return left < right;
}

fn and(left: bool, right: bool) -> bool {
  return left && right;
}

fn or(left: bool, right: bool) -> bool {
  return left || right;
}

fn not(left: bool, right: bool) -> bool {
  return left != right;
}

fn is(left: bool, right: bool) -> bool {
  return left == right;
}

fn main() {
  println!("1 + 2 = {:?}", add(1.0, 2.0));
  println!("2 - 3 = {:?}", subtract(2.0, 3.0));
  println!("3 / 4 = {:?}", divide(3.0, 4.0));
  println!("4 * 5 = {:?}", multiply(4.0, 5.0));
  println!("5 % 6 = {:?}", remainder(5.0, 6.0));
  println!("6 < 7 = {:?}", less_than(5.0, 6.0));
  println!("7 > 8 = {:?}", greater_than(5.0, 6.0));
  println!("true && false = {:?}", and(true, false));
  println!("true || false = {:?}", or(true, false));
  println!("true != false = {:?}", not(true, false));
  println!("true == false = {:?}", is(true, false));
}