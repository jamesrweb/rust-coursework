fn main() {
  let tuple_inferred = (220, true, 8.5);
  println!("{:?}", tuple_inferred);

  println!("Items by key:");
  println!("First item: {}", tuple_inferred.0);
  println!("Second item: {}", tuple_inferred.1);
  println!("Third item: {}", tuple_inferred.2);

  let tuple_typed: (i32, bool, f64) = (123, false, 22.1);
  let (first, second, third) = tuple_typed;
  println!("Items by destructuring:");
  println!("First item: {}", first);
  println!("Second item: {}", second);
  println!("Third item: {}", third);
}