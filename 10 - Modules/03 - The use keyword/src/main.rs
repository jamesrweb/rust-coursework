extern crate traffic;

// Way one (Specific import for current scope)
use traffic::TrafficLight;

// Way two (Destructuring into the current scope)
fn one() {
    use TrafficLight::{Red, Yellow};

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
    println!("{:?}, {:?}, {:?}", red, green, yellow);
}

// Way three (Dump all into the current scope)
fn two() {
    use TrafficLight::*;

    let red = Red;
    let yellow = Yellow;
    let green = Green;
    println!("{:?}, {:?}, {:?}", red, green, yellow);
}

fn main() {
    one();
    two();
}
