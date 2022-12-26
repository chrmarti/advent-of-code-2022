use std::fs;

fn main() {
    let text = fs::read_to_string("src/25-input.txt").unwrap();

    let sum = text.lines()
        .map(|line| line.chars().enumerate()
            .map(move |(i, c)| {
                let m = 5_isize.pow((line.len() - i - 1) as u32);
                match c {
                    '=' => -2 * m,
                    '-' => -m,
                    '0' => 0,
                    '1' => m,
                    '2' => 2 * m,
                    _ => panic!(),
                }
            }).sum::<isize>()
        ).sum::<isize>();
    println!("{}", sum);

    let mut rem = sum;
    let n = (rem as f64).log(5.0).ceil();
    let mut s = String::new();
    for i in 0..n as usize {
        let m = 5_isize.pow((n as usize - i - 1) as u32);
        let c  = (rem / m).to_string();
        s += &c;
        rem %= m;
    }

    println!("{}", s);

    let mut r = String::new();
    let mut over = 0;
    for c in s.chars().rev() {
        let i = c.to_string().parse::<usize>().unwrap() + over;
        match i {
            0..=2 => {
                r = i.to_string() + &r;
                over = 0;
            },
            3 => {
                r = "=".to_owned() + &r;
                over = 1;
            },
            4 => {
                r = "-".to_owned() + &r;
                over = 1;
            },
            5 => {
                r = "0".to_owned() + &r;
                over = 1;
            },
            _ => panic!(),
        }
    }
    if over == 1 {
        r = "1".to_owned() + &r;
    }

    println!("{}", r);
}
