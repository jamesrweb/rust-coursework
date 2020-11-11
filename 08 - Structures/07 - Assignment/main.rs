#[derive(Debug)]
struct Student {
  name: String,
  marks: Vec<u8>
}

impl Student {
  fn build(name: String, marks: Vec<u8>) -> Student {
    return Student {
      name: name,
      marks: marks
    }
  }
  fn highest_mark(&self) -> Option<&u8> {
    return self.marks.iter().min();
  }
}

fn main() {
  let student = Student::build(
    String::from("user"),
    vec![4, 5, 2]
  );

  println!("Student: {:?}", student);
  println!("Highest marks: {:?}", student.highest_mark());
}