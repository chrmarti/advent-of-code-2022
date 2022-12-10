use std::fs;

fn main() {
    let text = fs::read_to_string("src/10-input.txt").unwrap();

    let mut sum = 0;

    let mut ic = 1;
    let mut x = 1;
    for instr in text.split('\n') {
        if (ic - 20) % 40 == 0 {
            println!("{}: {}", ic, x);
            sum += ic * x;
        }
        if instr == "noop" {
            ic += 1;
        } else {
            let v = instr.split(' ').nth(1).unwrap().parse::<isize>().unwrap();
            ic += 1;
            if (ic - 20) % 40 == 0 {
                println!("{}: {}", ic, x);
                sum += ic * x;
            }
            x += v;
            ic += 1;
        }

    }

    println!("Sum: {}", sum);
}
