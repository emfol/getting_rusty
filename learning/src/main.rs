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
  let r: Rect = Rect::square(2.0);
  let rp: &Rect = &r;
  let x: [u8; 4] = [ b'H', b'I', b'G', b'H' ];
  let xs: &[u8] = &x[..];
  let a: [&str; 4] = [
    "Rust",
    "Programming",
    "Language",
    std::str::from_utf8(xs).unwrap_or(""),
  ];
  println!(" - \"r\" ({} x {}) is {} bytes", r.width, r.height, std::mem::size_of_val(&r));
  println!(" - \"rp\" is {} bytes", std::mem::size_of_val(&rp));
  println!(" - \"x\" is {} bytes", std::mem::size_of_val(&x));
  println!(" - Slice of \"x\" is {} bytes", std::mem::size_of_val(&xs));
  println!(" - \"a\" is {} bytes", std::mem::size_of_val(&a));
  for n in x {
    println!(" > {} ({} bytes)", n, std::mem::size_of_val(&n));
  }
  for (i, s) in a.iter().enumerate() {
    println!(
      " - String #{} is \"{}\" ({}/{} bytes)",
      i + 1,
      s,
      std::mem::size_of_val(&s),
      std::mem::size_of_val(&(*s))
    )
  }
}

