pub mod other;

pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut result = add(2, 2);
        assert_eq!(result, 4);
        result = other::sub(4, 16);
        assert_eq!(result, 12);
        result = other::oops::mul(3, 128);
        assert_eq!(result, 384);
    }
}

