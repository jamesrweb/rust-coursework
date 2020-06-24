const MIN_VALUE: i32 = 0; // global scope allowed with const

fn main() {
  // Let variables
  // let [name]: [optional type] = [value]
  // Local scope only
  let a = 10; // a = 20; - Throws
  let mut b = 10; // b = 20; - Doesn't throw
  println!("a: {}, b: {}", a, b);

  // Const variables
  // const [name]: [type] = [value]
  // Local or global scope allowed
  const MAX_VALUE: i32 = 10; // MAX_VALUE = 20; - Throws
  // const mut VALUE: f32 = 10.0; - Using mut is not possible with constants
  println!("min: {}, max: {}", MIN_VALUE, MAX_VALUE);
}