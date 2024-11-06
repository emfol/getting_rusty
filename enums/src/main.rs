struct User {
  id: u64,
  email: Option<String>,
  name: Option<String>,
}

impl User {
  fn new(id: u64, email: Option<String>, name: Option<String>) -> Self {
    Self { id, email, name }
  }
  fn print(&self) {
    let email: &str = if let Some(s) = &self.email { &s[..] } else { "" };
    let name: &str = if let Some(s) = &self.name { &s[..] } else { "" };
    println!(
      "@ User #{}:\n - Email: \"{}\"\n - Name: \"{}\"",
      self.id,
      email,
      name,
    );
  }
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  let id: u64 = if args.len() > 1 { args[1].parse().unwrap_or(1) } else { 1 };
  let email: Option<String> = if args.len() > 2 { Some(args[2].clone()) } else { None };
  let name: Option<String> = if args.len() > 3 { Some(args[3].clone()) } else { None };
  let user = User::new(id, email, name);
  user.print();
}
