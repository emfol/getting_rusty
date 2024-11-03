struct Rect {
    width: f32,
    height: f32,
}

fn main() {
    let width: f32 = 2.5;
    let height: f32 = 3.5;
    let rect = Rect {
        width,
        height,
    };
    print_rectangle(&rect);
}

fn area(rect: &Rect) -> f32 {
    rect.width * rect.height
}

fn print_rectangle(rect: &Rect) {
    println!("Rectangle: {}m x {}m ({}m²)", rect.width, rect.height, area(rect));
}
