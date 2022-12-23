use std::{fs, collections::{HashMap, hash_map::Entry}, ops::Add};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point(isize, isize);
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self  {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

fn main() {
    let text = fs::read_to_string("src/23-input.txt").unwrap();

    let mut map = text.lines().enumerate()
        .flat_map(|(y, line)| line.chars().enumerate()
            .map(move |(x, c)| (Point(x as isize, y as isize), c)))
        .filter(|(_, c)| *c != '.')
        .collect::<HashMap<_, _>>();

    let directions = vec![
        Point(0, -1),
        Point(1, -1),
        Point(1, 0),
        Point(1, 1),
        Point(0, 1),
        Point(-1, 1),
        Point(-1, 0),
        Point(-1, -1),
    ];

    let moves = vec![
        vec![
            Point(0, -1),
            Point(-1, -1),
            Point(1, -1),
        ],
        vec![
            Point(0, 1),
            Point(-1, 1),
            Point(1, 1),
        ],
        vec![
            Point(-1, 0),
            Point(-1, -1),
            Point(-1, 1),
        ],
        vec![
            Point(1, 0),
            Point(1, -1),
            Point(1, 1),
        ],
    ];

    let mut round = 0;
    loop {
        let mut proposals = HashMap::new();
        for (point, _) in map.iter() {
            if directions.iter().all(|m| !map.contains_key(&(*point + *m))) {
                continue;
            }
            for move_i in 0..4 {
                let moves = &moves[(round + move_i) % 4];
                if moves.iter().all(|m| !map.contains_key(&(*point + *m))) {
                    let new_point = *point + moves[0];
                    match proposals.entry(new_point) {
                        Entry::Vacant(e) => {
                            e.insert(Some(*point));
                        },
                        Entry::Occupied(mut e) => {
                            e.insert(None);
                        }
                    }
                    break;
                }
            }
        }
        if proposals.is_empty() {
            println!("Part 2: {}", round + 1);
            break;
        }
        for (new_point, old_point_or_none) in proposals {
            if let Some(old_point) = old_point_or_none {
                map.insert(new_point, '#');
                map.remove(&old_point);
            }
        }
        if round == 9 {

            let min_x = map.keys().map(|p| p.0).min().unwrap();
            let max_x = map.keys().map(|p| p.0).max().unwrap();
            let min_y = map.keys().map(|p| p.1).min().unwrap();
            let max_y = map.keys().map(|p| p.1).max().unwrap();
        
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    print!("{}", map.get(&Point(x, y)).unwrap_or(&'.'));
                }
                println!();
            }
        
            println!("{} {} {} {}", min_x, max_x, min_y, max_y);
            println!("Part 1: {}", (max_x - min_x + 1) * (max_y - min_y + 1) - map.len() as isize);
        }
        round += 1;
    }
}
