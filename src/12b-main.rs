use std::fs;
use std::collections::{VecDeque, HashSet};

fn main() {
    let text = fs::read_to_string("src/12-input.txt").unwrap();

    let mut starts = Vec::new();
    let mut end = (0, 0);
    let map = text.split('\n')
        .enumerate()
        .map(|(j, line)| line.chars()
            .enumerate()
            .map(|(i, c)| {
                match c {
                    'S' | 'a' => {
                        starts.push((i, j));
                        0
                    },
                    'E' => {
                        end = (i, j);
                        'z' as usize - 'a' as usize
                    },
                    _ => {
                        c as usize - 'a' as usize
                    }
                }
            }).collect::<Vec<usize>>()
        ).collect::<Vec<Vec<usize>>>();
    
    let steps = starts.iter().map(|start| {
        let mut deq = VecDeque::from([Vec::from([*start])]);
        let mut visited = HashSet::new();
    
        while let Some(path) = deq.pop_front() {
            let (x, y) = path[path.len() - 1];
            if (x, y) == end {
                return path.len() - 1;
            }
            let c = map[y][x];
            if y > 0 && !visited.contains(&(x, y - 1)) && map[y - 1][x] <= c + 1 {
                let mut new_path = path.to_vec();
                new_path.push((x, y - 1));
                deq.push_back(new_path);
                visited.insert((x, y - 1));
            }
            if y < map.len() - 1 && !visited.contains(&(x, y + 1)) && map[y + 1][x] <= c + 1 {
                let mut new_path = path.to_vec();
                new_path.push((x, y + 1));
                deq.push_back(new_path);
                visited.insert((x, y + 1));
            }
            if x > 0 && !visited.contains(&(x - 1, y)) && map[y][x - 1] <= c + 1 {
                let mut new_path = path.to_vec();
                new_path.push((x - 1, y));
                deq.push_back(new_path);
                visited.insert((x - 1, y));
            }
            if x < map[0].len() - 1 && !visited.contains(&(x + 1, y)) && map[y][x + 1] <= c + 1 {
                let mut new_path = path.to_vec();
                new_path.push((x + 1, y));
                deq.push_back(new_path);
                visited.insert((x + 1, y));
            }
        }
        999999
    });
    println!("{}", steps.min().unwrap());
}
