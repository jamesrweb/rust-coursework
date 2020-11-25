#[derive(Debug)]
enum IPAddrKind {
  V4,
  V6
}

#[derive(Debug)]
struct IPAddr {
  kind: IPAddrKind,
  address: String
}

fn main() {
  let home = IPAddr {
    kind: IPAddrKind::V4,
    address: String::from("127.0.0.1")
  };

  let loopback = IPAddr {
    kind: IPAddrKind::V6,
    address: String::from("::1")
  };

  println!("home: {:?}", home);
  println!("loopback: {:?}", loopback);
}