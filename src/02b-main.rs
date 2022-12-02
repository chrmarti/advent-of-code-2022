use std::fs;

fn main() {
    let text = fs::read_to_string("src/02-input.txt")
        .unwrap();
    let sum = text
        .split('\n')
        .map(|s| s.split(' ')
            .map(|s| s.chars().next().unwrap() as i32 % 'X' as i32 % 'A' as i32))
        .map(|mut s| {
            let opp = s.next().unwrap();
            let cmp = s.next().unwrap(); // 0: loose, 1: draw, 2: win
            let win = if cmp == 0 { 0 } else if cmp == 1 { 3 } else { 6 };
            let own = (opp + cmp + 2) % 3;
            let mov = own + 1;
            win + mov
        })
        .sum::<i32>();
    println!("sum: {:?}", sum);
}
