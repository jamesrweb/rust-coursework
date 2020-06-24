fn main() {
  // Strings
  let character: char = 'a';
  let slice: &str = "hello";
  let string: String = String::from("hello");
  let mut mutable_string: String = String::new();
  mutable_string.push_str("hello");

  println!(
    "char: {:?},\nslice: {:?},\nstring: {:?},\nmutable string: {:?}",
    character,
    slice,
    string,
    mutable_string
  );

  // Signed integers
  let signed_8: i8 = -1;
  let signed_16: i16 = 2;
  let signed_32: i32 = -3;
  let signed_64: i64 = 4;
  let signed_128: i128 = -5;

  println!(
    "\nsigned_8: {:?},\nsigned_16: {:?},\nsigned_32: {:?},\nsigned_64: {:?},\nsigned_128: {:?}",
    signed_8,
    signed_16,
    signed_32,
    signed_64,
    signed_128
  );

  // Unsigned integers
  let unsigned_8: u8 = 1;
  let unsigned_16: u16 = 2;
  let unsigned_32: u32 = 3;
  let unsigned_64: u64 = 4;
  let unsigned_128: u128 = 5;

  println!(
    "\nunsigned 8: {:?},\nunsigned 16: {:?},\nunsigned 32: {:?},\nunsigned 64: {:?},\nunsigned 128: {:?}",
    unsigned_8,
    unsigned_16,
    unsigned_32,
    unsigned_64,
    unsigned_128
  );

  // Floating point numbers
  let float_32: f32 = 1.0;
  let float_64: f64 = 1.0;

  println!(
    "\nfloat 32: {:?},\nfloat 64: {:?}",
    float_32,
    float_64
  );

  // Booleans
  let boolean_false: bool = false;
  let boolean_true: bool = true;

  println!(
    "\nBoolean false: {:?},\nBoolean true: {:?}",
    boolean_false,
    boolean_true
  );
}