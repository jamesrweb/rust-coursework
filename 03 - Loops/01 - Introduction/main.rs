fn count(from: i32, to: i32) {
  if from == to + 1 {
    return;
  }

  println!("{}", from);
  return count(from + 1, to);
}

fn main() {
  println!("Using the loop keyword:");
  let mut i = 0;
  loop {
    i += 1;
    println!("{}", i);

    if i == 5 {
      break;
    }
  }

  println!("Using a while loop:");
  let mut j = 0;
  while j < 5 {
    j += 1;
    println!("{}", j);
  }

  println!("Using a for loop:");
  for k in 1..6 {
    println!("{}", k);
  }

  println!("Using recursion:");
  count(1, 5);
}