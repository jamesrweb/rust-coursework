fn main() {
  let a: i32 = 10;
  // let b: i64 = a; - throws since the types are mismatched
  let b: i64 = a.into(); // convert the type
  let c: i64 = a as i64; // convert the type like above
  // let d: i64 = a + 10; - throws since the types are mismatched
  // let d: i64 = a.into() + 10; - throws since the into() cannot infer the type
  let d: i64 = a as i64 + 10;

  println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
}