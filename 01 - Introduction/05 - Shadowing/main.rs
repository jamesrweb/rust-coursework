fn main() {
  let a = 10;
  println!("{:?}", a);
  let a = "a"; // shadow (redeclare) the variable
  println!("{:?}", a);

  let mut b = 20;
  println!("{:?}", b);
  b = 20; // change the value of the variable
  // b = "b"; - throws since mut only allows the same type as the original value to be used
  println!("{:?}", b);

  let mut c = 30;
  println!("{:?}", c);
  let c = "c"; // shadow (redeclare) the variable - totally fine since we are shadowing
  println!("{:?}", c);
}