use std::fs;
use std::collections::HashSet;

fn main() {
    let text = fs::read_to_string("src/06-input.txt")
        .unwrap();
    
    // let n = 4; // a
    let n = 14; // b
    for i in n..text.len() {
        let mut set = HashSet::new();
        for ch in text.chars().skip(i - n).take(n) {
            set.insert(ch);
        }
        if set.len() == n {
            println!("{}: {}", i, text.chars().skip(i - n).take(n).collect::<String>());
            break;
        }
    }
}
