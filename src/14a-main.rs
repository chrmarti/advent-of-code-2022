use std::{fs, collections::HashMap};

fn main() {
    let text = fs::read_to_string("src/14-input.txt").unwrap();

    let mut map = HashMap::new();
    let mut max_y = 0;

    text.split('\n')
        .for_each(|s| {
            let points = s.split(" -> ")
                .map(|s| {
                    let mut iter = s.split(',');
                    let x = iter.next().unwrap().parse::<isize>().unwrap();
                    let y = iter.next().unwrap().parse::<isize>().unwrap();
                    (x, y)
                })
                .collect::<Vec<_>>();

            for i in 0..points.len() - 1 {
                let (x1, y1) = points[i];
                let (x2, y2) = points[i + 1];
                if x1 == x2 {
                    let from = y1.min(y2);
                    let to = y1.max(y2);
                    for j in from..to + 1 {
                        map.insert((x1, j), "stone");
                    }
                } else if y1 == y2 {
                    let from = x1.min(x2);
                    let to = x1.max(x2);
                    for j in from..to + 1 {
                        map.insert((j, y1), "stone");
                    }
                } else {
                    panic!("Something went wrong! {}", s);
                }
            }
            max_y = *points.iter().map(|(_, y)| y).max().unwrap().max(&max_y);
        });

    let mut count = 0;
    'outer: loop {
        let mut sand = (500, 0);
        'inner: loop {
            let (x, y) = sand;
            if y >= max_y {
                break 'outer;
            }
            if !map.contains_key(&(x, y + 1)) {
                sand = (x, y + 1);
            } else if !map.contains_key(&(x - 1, y + 1)) {
                sand = (x - 1, y + 1);
            } else if !map.contains_key(&(x + 1, y + 1)) {
                sand = (x + 1, y + 1);
            } else {
                count += 1;
                map.insert(sand, "sand");
                break 'inner;
            }
        }
    }

    println!("{}", count);
}