fn print(value: &String) {
  println!("{}", value);
}

fn add_world(s: &mut String) {
  s.push_str(" World!");
}

fn main() {
  // Even though this is a stored value, we can allow other functions to "borrow" the value without worrying about the ownership rules causing issues like those that happened previously. By explicitly saying that the variable is to be handed back, we don't need any reassignments like before either. As an example this works just fine:
  let greeting = String::from("Hello there!");
  print(&greeting);
  println!("Greeting: {}", greeting);

  // We can even allow the scope that is borrowing our variable to alter that variable as long as we explicitly mark it as mutable, for example:
  let mut string = String::from("Hello");
  add_world(&mut string);
  println!("{}", string);

  // Slices allow us to reference a contiguous sequence of elements
  let character = String::from("Spongebob");
  let slices = (&character[0..5], &character[0..=5], &character[..5], &character[0..]);
  println!("Slices: {:?}", slices);

  // Another example could be shown using arrays like so:
  let list = [1, 2, 3, 4, 5];
  println!("List: {:?}, Slice: {:?}", list, &list[0..3])
}