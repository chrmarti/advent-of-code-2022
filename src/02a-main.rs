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
            let own = s.next().unwrap();
            let cmp = (own - opp + 2) % 3;
            let win = if cmp == 0 { 6 } else if cmp == 1 { 0 } else { 3 };
            let mov = own + 1;
            win + mov
        })
        .sum::<i32>();
    println!("sum: {:?}", sum);
}
