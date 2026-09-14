use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    println!("i32: {}, u32: {}", rng.random::<i32>(), rng.random::<u32>())
}
