#![feature(iter_array_chunks)]
use core::panic;
use std::fs;


fn main() {
    let text = fs::read_to_string("src/03-input.txt")
        .unwrap();
    let sum = text
        .split('\n')
        .array_chunks::<3>()
        .map(|[one, two, three]| {
            for c in one.chars() {
                if two.contains(c) && three.contains(c) {
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
