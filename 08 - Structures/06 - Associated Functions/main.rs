#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn build(width: u32, height: u32) -> Rectangle {
    return Rectangle { width: width, height: height };
  }
}

fn main() {
  let rect = Rectangle::build(100, 50);
  println!("{:?}", rect);
}