use std::{fs, collections::HashMap};
use regex::Regex;

fn main() {
    let line_re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();

    let text = fs::read_to_string("src/15-input.txt").unwrap();
    let y = 2000000;
    // let y = 10;

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

    let mut count = 0;
    sensors.iter()
        .for_each(|((x1, y1), (x2, y2))| {
            let d = (x1 - x2).abs() + (y1 - y2).abs();
            let dd = d - (y - y1).abs();
            for x in x1 - dd..x1 + dd + 1 {
                map.entry((x, y)).or_insert_with(|| {
                    count += 1;
                    "empty"
                });
            }
        });

    println!("{}", count);
}
