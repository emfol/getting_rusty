struct Rect {
    width: f32,
    height: f32,
}

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

fn main() {
    let (width, height, increase): (f32, f32, f32) = (4.0, 3.0, 0.25);
    let mut rect = Rect { width, height };
    let other_rects: [Rect; 2] = [
        Rect { width: 1.5, height: 2.1 },
        Rect { width: 8.0, height: 5.2 },
    ];
    rect.print();
    println!("Increasing rectangle by {}%...", increase * 100.0);
    rect.increase(increase);
    rect.print();
    for (i, e) in other_rects.iter().enumerate() {
        println!("Rectangle #{}", i);
        e.print();
        println!("Fits in main? {}", rect.can_hold(e));
    }
}
