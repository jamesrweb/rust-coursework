// Options are defined in the standard library as:
// enum Option<T> {
//   Some(T),
//   None
// }

fn main() {
  // For an option to hold a value it must be Some value
  let num = Some(5);
  // For an Option to be None, we must say what it should have been or will be, in this case, it is a non-existant String. Heed the type annotation in this case.
  let text: Option<String> = None;

  println!("num: {:?}, text: {:?}", num, text);
}