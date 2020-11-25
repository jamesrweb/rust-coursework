#[derive(Debug)]
enum IPAddrKind {
  V4,
  V6
}

fn route(ip: IPAddrKind) {
  println!("Routing via {:?}", ip);
}

fn main() {
  let four = IPAddrKind::V4;
  let six = IPAddrKind::V6;
  route(four);
  route(six);
}