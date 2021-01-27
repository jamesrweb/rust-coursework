fn inferred_types_lazy() -> Vec<i32> {
    let mut v = Vec::new();
    v.push(20);
    v.push(30);
    v.push(40);
    return v;
}

fn inferred_types_eager() -> Vec<i32> {
    let v = vec![20, 30, 40];
    return v;
}

fn pre_typed() -> Vec<i32> {
    let v: Vec<i32> = vec![20, 30, 40];
    return v;
}

// Panics at runtime if index out of range
fn get_by_index(vector: &Vec<i32>, index: i32) -> i32 {
    return vector[index as usize];
}

// Does not panic but instead returns option of Some(T) or None
fn get_by_method(vector: &Vec<i32>, index: i32) -> Option<&i32> {
    return vector.get(index as usize);
}

#[derive(Debug)]
enum SpreadSheet {
    Integer(i32),
    Float(f64),
    Text(String),
}

fn generate_sheet() -> Vec<[SpreadSheet; 3]> {
    return vec![
        [
            SpreadSheet::Integer(30),
            SpreadSheet::Float(3.14175),
            SpreadSheet::Text(String::from("Hello world!")),
        ],
        [
            SpreadSheet::Integer(20),
            SpreadSheet::Float(42.875634),
            SpreadSheet::Text(String::from("Hi there!")),
        ],
    ];
}

fn main() {
    println!("Inferred lazy: {:?}", inferred_types_lazy());
    println!("Inferred eager: {:?}", inferred_types_eager());
    println!("Typed: {:?}", pre_typed());

    let vector = vec![1, 2, 3];
    println!("Value by index: {:?}", get_by_index(&vector, 0));
    println!("Value by method: {:?}", get_by_method(&vector, 0));
    println!("Values: {:?}", &vector);

    let sheet = generate_sheet();
    println!("Multiple types via an enum: {:#?}", &sheet);
}
