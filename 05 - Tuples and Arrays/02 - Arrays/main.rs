fn print_array(array: [i32;5], tag: &str) {
  println!("{} array: {:?}", tag, array);
}

fn print_elements(array: [i32;5]) {
  for (index, item) in array.iter().enumerate() {
    println!("Item {}: {}", index + 1, item);
  }
}

fn main() {
  let array = [22, 34, 65, 3, 2];
  print_array(array, "Default");
  print_elements(array);

  let sized_array: [i32;5] = [1, 2, 3, 4, 5];
  print_array(sized_array, "Sized");
  print_elements(sized_array);

  let mut mutable_array = [0;5];
  mutable_array[2] = 1;
  print_array(mutable_array, "Mutable");
  print_elements(mutable_array);
}