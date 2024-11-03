fn main() {
    let width: f32 = 2.5;
    let height: f32 = 3.5;
    let rect: (f32, f32) = (width, height);
    print_rectangle(rect);
}

fn area(rect: (f32, f32)) -> f32 {
    // width * height
    rect.0 * rect.1
}

fn print_rectangle(rect: (f32, f32)) {
    println!("Rectangle: {}m x {}m ({}m²)", rect.0, rect.1, area(rect));
}
