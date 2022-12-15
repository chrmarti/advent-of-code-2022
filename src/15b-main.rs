use std::{fs, collections::HashMap, process::exit};
use regex::Regex;

fn main() {
    let line_re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    let text = fs::read_to_string("src/15-input.txt").unwrap();
    let max = 4000000;
    // let max = 20;

    let mut map = HashMap::new();
    let mut sensors = Vec::new();

    for line_cap in line_re.captures_iter(&text) {
        let x1 = line_cap[1].parse::<isize>().unwrap();
        let y1 = line_cap[2].parse::<isize>().unwrap();
        let x2 = line_cap[3].parse::<isize>().unwrap();
        let y2 = line_cap[4].parse::<isize>().unwrap();

        map.insert((x1, y1), "sensor");
        map.insert((x2, y2), "beacon");

        sensors.push(((x1, y1), (x2, y2)));
    }

    println!("{:?}", sensors);

    let mut next = std::time::Instant::now() + std::time::Duration::from_secs(10);
    
    for y in 0..max + 1 {
        let mut x = 0;
        while x < max + 1 {
            let now = std::time::Instant::now();
            if now > next {
                println!("{} {}", x, y);
                next = now + std::time::Duration::from_secs(10);
            }
            if !map.contains_key(&(x, y)) {
                let seen = sensors.iter()
                    .find(|((x1, y1), (x2, y2))| {
                        let d1 = (x1 - x2).abs() + (y1 - y2).abs();
                        let d2 = (x1 - x).abs() + (y1 - y).abs();
                        d2 <= d1
                    });
                if seen.is_none() {
                    println!("{}", x * 4000000 + y);
                    exit(0);
                }
                let ((x1, y1), (x2, y2)) = seen.unwrap();
                let d = (x1 - x2).abs() + (y1 - y2).abs();
                let dd = d - (y - y1).abs();
                x = x1 + dd;
            }
            x += 1;
        }
    }
}
