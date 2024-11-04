struct Rect {
    width: f32,
    height: f32,
}

impl Rect {
    fn area(&self) -> f32 {
        self.width * self.height
    }
    fn increase(&mut self, amount: f32) {
        self.width *= 1.0 + amount;
        self.height *= 1.0 + amount;
    }
    fn print(&self) {
        println!("Rectangle: {}m x {}m ({}m²)", self.width, self.height, self.area());
    }
}

fn main() {
    let width: f32 = 2.5;
    let height: f32 = 3.5;
    let mut rect = Rect {
        width,
        height,
    };
    rect.print();
    rect.increase(0.2);
    rect.print();
}
