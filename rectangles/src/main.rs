struct Rect {
    width: f32,
    height: f32,
}

// Methods
impl Rect {
    fn area(&self) -> f32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rect) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
    fn increase(&mut self, amount: f32) {
        self.width *= amount + 1.0;
        self.height *= amount + 1.0;
    }
    fn print(&self) {
        println!("Rectangle: {}m x {}m ({}m²)", self.width, self.height, self.area());
    }
}

// Associated Functions
impl Rect {
    fn from(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    fn square(side: f32) -> Rect {
        Self::from(side, side)
    }
}

fn main() {
    let (width, height, increase): (f32, f32, f32) = (4.0, 3.0, 0.25);
    let mut rect = Rect::from(width, height);
    let other_rects: [Rect; 3] = [
        Rect::from(1.5, 2.1),
        Rect::from(8.0, 5.2),
        Rect::square(2.0),
    ];
    rect.print();
    println!("Increasing main rectangle by {}%...", increase * 100.0);
    rect.increase(increase);
    rect.print();
    for (i, e) in other_rects.iter().enumerate() {
        println!("Rectangle #{}", i + 1);
        e.print();
        println!("Fits in main? {}", rect.can_hold(e));
    }
}
