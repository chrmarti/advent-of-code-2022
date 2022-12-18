use std::{fs, collections::HashMap};

fn main() {
    let jets = fs::read_to_string("src/17-input.txt").unwrap();

    let shapes = vec![
        vec![
            "####"
        ],
        vec![
            ".#.",
            "###",
            ".#.",
        ],
        vec![
            "###",
            "..#",
            "..#",
        ],
        vec![
            "#",
            "#",
            "#",
            "#",
        ],
        vec![
            "##",
            "##",
        ],
    ];

    let mut grid: HashMap<(isize, isize), bool> = HashMap::new();

    let mut max_y = -1;

    let mut j = 0;

    let mut next = std::time::Instant::now() + std::time::Duration::from_secs(10);

    // let n = 2022; // 1
    let n = 1000000000000; // 2
    let mut i = 0;
    let mut add_to_max_y = 0;
    while i < n {
        let now = std::time::Instant::now();
        if now > next {
            println!("i: {}", i);
            println!("Current max y: {}", max_y + 1);
            next = now + std::time::Duration::from_secs(10);
        }

        let shape_i = i % shapes.len();
        if shape_i == 0 && j % jets.len() == 52 {
            println!("(Re)starting at i: {}, max_y + 1: {}", i, max_y + 1);
            // 1715 , 2677
            let k = (n - i) / 1715;
            i += k * 1715;
            add_to_max_y += k as isize * 2677;
            println!("Cont at i: {}, max_y + 1: {}, k: {}", i, add_to_max_y + 1, k);
        }

        let shape = &shapes[shape_i];
        let mut x: isize = 2;
        let mut y: isize = max_y + 4;

        loop {
            let jet_j = j % jets.len();
            let jet = &jets[jet_j..jet_j+1];
            j += 1;
            match jet {
                ">" => {
                    if x + (shape[0].len() as isize) < 7 && !collide(&grid, shape, x + 1, y) {
                        x += 1;
                    }
                },
                "<" => {
                    if x > 0 && !collide(&grid, shape, x - 1, y) {
                        x -= 1;
                    }
                },
                _ => panic!(),
            }
            if y > 0 && !collide(&grid, shape, x, y - 1) {
                y -= 1;
            } else {
                for (i, row) in shape.iter().enumerate() {
                    for (j, c) in row.chars().enumerate() {
                        if c == '#' {
                            grid.insert((x + j as isize, y + i as isize), true);
                        }
                    }
                }
                max_y = max_y.max(y + shape.len() as isize - 1);
                break;
            }
        }

        if grid.len() > 1000000 {
            grid = grid.iter()
                .filter(|((_, y), _)| *y > max_y - 100)
                .map(|(k, v)| (*k, *v))
                .collect();
        }

        i += 1;
    }

    println!("Max y: {}", max_y + add_to_max_y + 1);

}

fn collide(grid: &HashMap<(isize, isize), bool>, shape: &[&str], x: isize, y: isize) -> bool {
    for (i, row) in shape.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '#' && *grid.get(&(x + j as isize, y + i as isize)).unwrap_or(&false) {
                return true;
            }
        }
    }
    false
}
