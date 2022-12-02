use std::fs;

fn main() {
    let text = fs::read_to_string("src/01-input.txt")
        .unwrap();
    let mut vec = text
        .split("\n\n")
        .map(|s| s.split('\n')
            .map(|s| s.parse::<i32>().unwrap())
            .sum::<i32>())
        .collect::<Vec<_>>();
    
    let max = vec.iter()
        .max()
        .unwrap();
    println!("max: {:?}", max);
    // 75501

    vec.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    let sum = vec.iter()
        .take(3)
        .sum::<i32>();
    println!("sum: {:?}", sum);
}
