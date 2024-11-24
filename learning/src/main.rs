use std::{mem, collections::HashMap};

struct Rect {
  width: f64,
  height: f64,
}

impl Rect {
  fn new(width: f64, height: f64) -> Self {
    Self { width, height }
  }
  fn square(side: f64) -> Self {
    Self::new(side, side)
  }
}

fn main() {
  let s: String = String::from("Josiane");
  let mut v: Vec<String> = Vec::with_capacity(4);
  let mut d: HashMap<String, String> = HashMap::new();
  let c = 'a';
  let r: Rect = Rect::square(2.0);
  let rp: &Rect = &r;
  let x: [u8; 4] = [ b'H', b'I', b'G', b'H' ];
  let y: [i64; 4] = [1, 2, 3, 4];
  let xs: &[u8] = &x[..];
  let ys: &[i64] = &y[..];
  let a: [&str; 4] = [
    "Rust",
    "Programming",
    "Language",
    std::str::from_utf8(xs).unwrap_or(""),
  ];
  d.insert(String::from("name"), String::from("Emanuel"));
  d.insert(String::from("last_name"), String::from("Oliveira"));
  v.push(String::from("Josiane"));
  v.push(String::from("Eric"));
  println!(" - \"s\" is {} bytes", mem::size_of_val(&s));
  println!(" - \"v\" is {} bytes (length: {}, capacity: {})", mem::size_of_val(&v), v.len(), v.capacity());
  println!(" - \"d\" is {} bytes", mem::size_of_val(&d));
  println!(" - \"c\" is {} bytes", mem::size_of_val(&c));
  println!(" - \"r\" ({} x {}) is {} bytes", r.width, r.height, mem::size_of_val(&r));
  println!(" - \"rp\" is {} bytes", mem::size_of_val(&rp));
  println!(" - \"x\" is {} bytes", mem::size_of_val(&x));
  println!(" - \"y\" is {} bytes", mem::size_of_val(&y));
  println!(" - Slice of \"x\" is {} bytes", mem::size_of_val(&xs));
  println!(" - Slice of \"y\" is {} bytes", mem::size_of_val(&ys));
  println!(" - \"a\" is {} bytes", mem::size_of_val(&a));
  for n in x {
    println!(" > {} ({} bytes)", n, mem::size_of_val(&n));
  }
  for (i, s) in a.iter().enumerate() {
    println!(
      " - String #{} is \"{}\" ({}/{} bytes)",
      i + 1,
      s,
      mem::size_of_val(&s),
      mem::size_of_val(&(*s))
    )
  }
}

