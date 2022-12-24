use core::panic;
use std::{fs, mem::swap};

fn main() {
    let text = fs::read_to_string("src/24-input.txt").unwrap();

    let mut map = text.lines()
        .map(|line| line.chars()
            .map(move |c| if c == '.' { vec![] } else { vec![c] })
            .collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    map[0][1].push('E');

    let mut new_map = map.clone();

    let mut round = 0;
    loop {
        new_map.iter_mut().for_each(|v| v.iter_mut().for_each(|v| v.clear()));

        for (y, line) in map.iter().enumerate() {
            for (x, v) in line.iter().enumerate() {
                for c in v.iter() {
                    match c {
                        '>' => {
                            if x + 2 < line.len() {
                                new_map[y][x + 1].push('>');
                            } else {
                                new_map[y][1].push('>');
                            }
                        },
                        '<' => {
                            if x >= 2 {
                                new_map[y][x - 1].push('<');
                            } else {
                                new_map[y][line.len() - 2].push('<');
                            }
                        },
                        'v' => {
                            if y + 2 < map.len() {
                                new_map[y + 1][x].push('v');
                            } else {
                                new_map[1][x].push('v');
                            }
                        },
                        '^' => {
                            if y >= 2 {
                                new_map[y - 1][x].push('^');
                            } else {
                                new_map[map.len() - 2][x].push('^');
                            }
                        },
                        '#' => {
                            new_map[y][x].push('#');
                        },
                        'E' => {
                        },
                        _ => panic!(),
                    }
                }
            }
        }

        for (y, line) in map.iter().enumerate() {
            for (x, v) in line.iter().enumerate() {
                for c in v.iter() {
                    match c {
                        '>' => {
                        },
                        '<' => {
                        },
                        'v' => {
                        },
                        '^' => {
                        },
                        '#' => {
                        },
                        'E' => {
                            if new_map[y][x].is_empty() {
                                new_map[y][x].push('E');
                            }
                            if x + 1 < line.len() && new_map[y][x + 1].is_empty() {
                                new_map[y][x + 1].push('E');
                            }
                            if x > 0 && new_map[y][x - 1].is_empty() {
                                new_map[y][x - 1].push('E');
                            }
                            if y + 1 < map.len() && new_map[y + 1][x].is_empty() {
                                new_map[y + 1][x].push('E');
                            }
                            if y > 0 && new_map[y - 1][x].is_empty() {
                                new_map[y - 1][x].push('E');
                            }
                        },
                        _ => panic!(),
                    }
                }
            }
        }

        swap(&mut map, &mut new_map);
        round += 1;
        
        println!("Round {}", round);
        for line in map.iter() {
            for v in line.iter() {
                if v.is_empty() {
                    print!(".");
                } else if v.len() > 1 {
                    print!("{}", v.len());
                } else {
                    print!("{}", v[0]);
                }
            }
            println!();
        }
        println!();

        if map[map.len() - 1][map[0].len() - 2].contains(&'E') {
            break;
        }
    }
}
