use core::panic;
use std::fs;

fn main() {
    let text = fs::read_to_string("src/03-input.txt")
        .unwrap();
    let sum = text
        .split('\n')
        .map(|s| {
            let one = &s[0..s.len() / 2];
            let two = &s[s.len() / 2..s.len()];
            for c in one.chars() {
                if two.contains(c) {
                    return c;
                }
            }
            panic!("no match");
        })
        .map(|s| {
            if s as i32 >= 'a' as i32 { s as i32 - 'a' as i32 + 1 } else { s as i32 - 'A' as i32 + 27 }
        })
        .sum::<i32>();
    println!("sum: {:?}", sum);
}
