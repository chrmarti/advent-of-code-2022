use std::{fs, ops::Add};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point(isize, isize);
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self  {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

fn main() {
    let text = fs::read_to_string("src/22-input.txt").unwrap();

    let mut input = text.split("\n\n");
    let board_text = input.next().unwrap();
    let instructions = input.next().unwrap();
    let instr_chars = instructions.chars().collect::<Vec<_>>();

    let mut board = board_text.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let dirs = [
        Point(1, 0),
        Point(0, 1),
        Point(-1, 0),
        Point(0, -1),
    ];

    let mut pos = Point(board[0].iter().position(|c| *c == '.').unwrap() as isize, 0);
    let mut dir_i = 0;
    let mut dir = Point(1, 0);
    board[pos.1 as usize][pos.0 as usize] = '>';

    let mut ip = 0;
    while ip < instructions.len() {
        let from = ip;
        while ip < instr_chars.len() && '0' <= instr_chars[ip] && instr_chars[ip] <= '9' {
            ip += 1;
        }
        if ip > from {
            let mov = instructions[from..ip].parse::<usize>().unwrap();
            for _ in 0..mov {
                let mut next = pos + dir;
                if next.1 < 0 || next.1 >= board.len() as isize || next.0 < 0 || next.0 >= board[next.1 as usize].len() as isize || board[next.1 as usize][next.0 as usize] == ' ' {
                    next = match dir {
                        Point(1, 0) => Point(board[next.1 as usize].iter().position(|c| *c != ' ').unwrap() as isize, next.1),
                        Point(-1, 0) => Point(board[next.1 as usize].iter().enumerate().rev().find(|(_, c)| **c != ' ').unwrap().0 as isize, next.1),
                        Point(0, 1) => Point(next.0, board.iter().position(|row| next.0 < row.len() as isize && row[next.0 as usize] != ' ').unwrap() as isize),
                        Point(0, -1) => Point(next.0, board.iter().enumerate().rev().find(|(_, row)| next.0 < row.len() as isize && row[next.0 as usize] != ' ').unwrap().0 as isize),
                        _ => unreachable!(),
                    };
                }
                if board[next.1 as usize][next.0 as usize] == '#' {
                    break; // wall
                }
                pos = next;
                board[pos.1 as usize][pos.0 as usize] = match dir {
                    Point(1, 0) => '>',
                    Point(-1, 0) => '<',
                    Point(0, 1) => 'v',
                    Point(0, -1) => '^',
                    _ => unreachable!(),
                };
            }
        }

        while ip < instr_chars.len() && (instr_chars[ip] == 'R' || instr_chars[ip] == 'L') {
            match instr_chars[ip] {
                'R' => dir_i = (dir_i + 1) % dirs.len(),
                'L' => dir_i = (dir_i + dirs.len() - 1) % dirs.len(),
                _ => unreachable!(),
            }
            dir = dirs[dir_i];
            ip += 1;
        }
        board[pos.1 as usize][pos.0 as usize] = match dir {
            Point(1, 0) => '>',
            Point(-1, 0) => '<',
            Point(0, 1) => 'v',
            Point(0, -1) => '^',
            _ => unreachable!(),
        };
    }

    for row in board {
        println!("{}", row.iter().collect::<String>());
    }

    println!("({}, {}) {}", pos.0, pos.1, dir_i);
    println!("{}", 1000 * (pos.1 + 1) + 4 * (pos.0 + 1) + dir_i as isize);
}
