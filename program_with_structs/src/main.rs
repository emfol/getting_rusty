
struct Point(f32, f32);
struct Rect(Point, Point);

struct User {
    name: String,
    email: String,
    login_count: u64,
    active: bool,
}

fn print_user(u: &User) {
    println!("User: {}, {}, {}, {}", u.name, u.email, u.login_count, u.active);
}

fn main() {
    let rect = Rect(
        Point(1.0, 2.0),
        Point(4.0, 5.0),
    );
    let user1 = User {
        name: String::from("Emanuel"),
        email: String::from("emfolg@gmail.com"),
        login_count: 1,
        active: true,
    };
    let mut user2 = User {
        name: String::from("Josiane"),
        email: String::from("josyrecin@gmail.com"),
        ..user1
    };
    println!("Rect: {}, {}, {}, {}", rect.0.0, rect.0.1, rect.1.0, rect.1.1 );
    print_user(&user1);
    print_user(&user2);
    user2.login_count += 1;
    print_user(&user2);
}
