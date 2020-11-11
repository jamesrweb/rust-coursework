struct Rectangle {
  width: u32,
  height: u32
}

impl Rectangle {
  fn area(&self) -> u32 {
    return self.width * self.height;
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    return self.width > other.width && self.height > other.height;
  }
}

fn main() {
  let rect_one = Rectangle { width: 100, height: 50 };
  let rect_two = Rectangle { width: 95, height: 27 };

  println!("Rect 1 area: {}", rect_one.area());
  println!("Rect 2 area: {}", rect_two.area());
  println!("Rect 1 can hold Rect 2: {:?}", rect_one.can_hold(&rect_two));
  println!("Rect 2 can hold Rect 1: {:?}", rect_two.can_hold(&rect_one));
}