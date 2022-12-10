use std::fs;

fn main() {
    let text = fs::read_to_string("src/10-input.txt").unwrap();

    let mut ic = 1;
    let mut x = 1;
    for instr in text.split('\n') {
        if x-1 <= (ic-1)%40 && (ic-1)%40 <= x+1 {
            print!("#");
        } else {
            print!(".");
        }
        if instr == "noop" {
            ic += 1;
        } else {
            let v = instr.split(' ').nth(1).unwrap().parse::<isize>().unwrap();
            ic += 1;
            if (ic-1)%40 == 0 {
                println!();
            }
            if x-1 <= (ic-1)%40 && (ic-1)%40 <= x+1 {
                print!("#");
            } else {
                print!(".");
            }
            x += v;
            ic += 1;
        }
        if (ic-1)%40 == 0 {
            println!();
        }
    }
}
