fn add(left: i32, right: i32) -> i32 {
  left + right
}

fn sub_add(left: i32, right: i32) -> (i32, i32) {
  (left + right, left - right)
}

fn main() {
  let sum = add(1, 3);
  println!("1 + 3 = {}", sum);

  let sum_diff = sub_add(2, 8);
  println!("2 + 8 = {}, 2 - 8 = {}", sum_diff.0, sum_diff.1);
}