fn should_water_plants(raining: bool) -> bool {
  if raining {
    println!("It is raining, no need to water the plants");
    return false;
  }

  println!("It hasn't rained, you should water the plants");
  return true;
}

fn main() {
  let is_raining = true;
  let should_water = should_water_plants(is_raining);

  if should_water {
    println!("Go water them when you get the chance");
  } else {
    println!("Sit back and relax");
  }
}