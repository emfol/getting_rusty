
struct Rect {
  width: f32,
  height: f32,
}

impl Rect {
  fn area(&self) -> f32 {
    self.width * self.height
  }
  fn print(&self) {
    println!(" > Rect: {}mm x {}mm ({} mm²)", self.width, self.height, self.area());
  }
  fn fits_in(&self, rect: &Self) -> bool {
    self.width <= rect.width && self.height <= rect.height
  }
  fn new(width: f32, height: f32) -> Self {
    Self { width, height }
  }
}

fn main() {
  let mut rect = Rect::new(4.0, 3.0);
  let other_rects = [
    Rect::new(2.0, 1.0),
    Rect::new(16.0, 8.0),
  ];
  let args: Vec<String> = std::env::args().collect();

  if args.len() > 1 {
    rect.width = args[1].parse().unwrap_or(rect.width);
  }

  if args.len() > 2 {
    rect.height = args[2].parse().unwrap_or(rect.height);
  }

  println!("Main Rect:");
  rect.print();

  for (i, r) in other_rects.iter().enumerate() {
    println!("Other Rect #{}:", i + 1);
    r.print();
    println!("Fits in main? {}", r.fits_in(&rect));
  }
}
