
fn main() {
    let tuple: (i32, f32, char) = (2, 3.14, '🦀');
    let array: [i32; 4] = [1, 2, 3, 4];
    let needle: i32 = 3;
    // Copy
    let e1: i32 = 11;
    let e2: i32 = e1;
    // Move
    let s1: String = String::from("Emanuel\tis a programmer...");
    let s2: String = s1.clone();
    let ls1 = get_len(&s1);
    let ls2 = get_len(&s2);
    println!("Tuple: ({}, {}, {})", tuple.0, tuple.1, tuple.2);
    println!("Array: [{}, {}, {}, {}]", array[0], array[1], array[2], array[3]);
    println!("E1: {}, E2: {}", e1, e2);
    println!("S1: {} ({}), S2: {} ({})", s1, ls1, s2, ls2);
    println!("First word from S1 ends at: {}", first_word_ends_at(&s1));
    println!("First word from S1: {}", first_word(&s1));
    println!("The entry {} is at {}", needle, find_index(&array, needle).1)
}

fn is_space(c: u8) -> bool {
    c == b' ' || c == b'\t'
}

fn get_len(s: &String) -> usize {
    s.len()
}

fn find_index(a: &[i32], x: i32) -> (bool, usize) {
    for (i, &n) in a.iter().enumerate() {
        if n == x {
            return (true, i);
        }
    }
    (false, 0)
}

fn first_word_ends_at(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if is_space(c) {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &c) in bytes.iter().enumerate() {
        if is_space(c) {
            return &s[0..i];
        }
    }
    &s[..]
}
