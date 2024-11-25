use std::ffi::OsString;

mod oops;

fn main() {
    let default_path = OsString::from("user.txt");
    let mut args = std::env::args_os();
    let path = if args.len() > 1 {
        args.nth(1).unwrap_or(default_path)
    } else  {
        default_path
    };
    match oops::get_user_name(&path[..]) {
        Ok(s) => println!("The user name is: {}", s),
        Err(e) => println!("Error retrieving user name: {}", e),
    }
}
