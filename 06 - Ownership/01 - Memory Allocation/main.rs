fn main() {
  // The stack holds data that won't change
  let _a = 10;
  let _b = 3.14;
  let _c = true;
  let _d = 'a';
  let _e = [1, 2, 3, 4, 5];

  // The heap holds data that can possibly be changed
  let _f = (1, true, "woo");
  let _g = String::from("testing");
  let _h = Box::new('a');
}