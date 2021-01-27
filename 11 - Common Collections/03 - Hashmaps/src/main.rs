use std::collections::HashMap;

fn create_map() -> HashMap<String, i32> {
    let mut scores = HashMap::new();
    scores.insert("x".to_owned(), 10);
    scores.insert("y".to_owned(), 20);
    return scores;
}

fn print_pairs(hashmap: &HashMap<String, i32>) {
    for (key, value) in hashmap {
        println!("{} = {}", key, value);
    }
}

fn main() {
    let mut map = create_map();
    println!("Simple: {:?}", &map);

    map.insert(String::from("z"), 1);
    println!("After update: {:?}", &map);

    map.entry("t".to_owned()).or_insert(30);

    print_pairs(&map);
}
