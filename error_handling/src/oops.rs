use std::io::{Error, Read};
use std::fs::File;

pub fn get_user_name(path: &str) -> Result<String, Error> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e),
    };
    let mut name = String::with_capacity(16);
    match file.read_to_string(&mut name) {
        Ok(s) => {
            println!(" > Total bytes read from \"{}\": {}", path, s);
            Ok(name)
        },
        Err(e) => Err(e),
    }
}
