fn main() {
  // If references to variables that are on the heap are used, the original reference will be lost due to the changed ownership and an error thrown when used. For example:
  let s1: String = String::from("hello");
  let s2 = s1;
  // println!("s1: {}, s2: {}", s1, s2);

  // If variables reference others that are on the stack however, that works just fine since the compiler knows the variable size at compile time and can just copy the value. For example:
  let s1: &str = "hello";
  let s2: &str = s1;
  println!("s1: {}, s2: {}", s1, s2);
}