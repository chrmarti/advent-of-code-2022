use std::{fs, collections::HashMap};

fn main() {
    let text = fs::read_to_string("src/07-input.txt")
        .unwrap();
    
    let mut dirs = HashMap::new();
    let mut cwd = Vec::new();

    for instr0 in text.split("$ ").skip(1) {
        let instr = instr0.trim();
        // println!("'{}'", instr);
        if instr.starts_with("ls") {
            let mut sum = 0;
            for entry in instr.split('\n').skip(1) {
                let mut parts = entry.split(' ');
                let first = parts.next().unwrap();
                if first != "dir" {
                    let size = first.parse::<usize>().unwrap();
                    sum += size;
                }
            }
            for i in 0..cwd.len() {
                let dir = cwd[..=i].join("/");
                // println!("{}: {}", dir, sum);
                let entry = dirs.entry(dir).or_insert(0);
                *entry += sum; // modifies entry
            }
        } else if let Some(stripped) = instr.strip_prefix("cd ") {
            match stripped.trim() {
                "/" => { cwd.clear(); cwd.push(""); },
                ".." => { cwd.pop(); },
                name => cwd.push(name),
            }
        } else {
            panic!("Unexpected instruction: {}", instr);
        }
    }
    // println!("{:?}", dirs);

    let sum = dirs.values().filter(|&&size| size <= 100000).sum::<usize>();
    println!("Sum: {}", sum);

    let needed = 30000000 - (70000000 - dirs[""]);
    let free = dirs.values().filter(|&&size| size >= needed).min().unwrap();
    println!("Free: {}", free);
}
