use std::{fs, collections::HashSet};

fn main() {
    let text = fs::read_to_string("src/18-input.txt").unwrap();

    let cubes = text.split('\n')
        .map(|line| {
            let mut iter = line.split(',').map(|i| i.parse::<isize>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
        });
    let set = cubes.clone().collect::<HashSet<_>>();
    let mut open = 0;
    for cube in cubes {
        if !set.contains(&(cube.0 + 1, cube.1, cube.2)) {
            open += 1;
        }
        if !set.contains(&(cube.0 - 1, cube.1, cube.2)) {
            open += 1;
        }
        if !set.contains(&(cube.0, cube.1 + 1, cube.2)) {
            open += 1;
        }
        if !set.contains(&(cube.0, cube.1 - 1, cube.2)) {
            open += 1;
        }
        if !set.contains(&(cube.0, cube.1, cube.2 + 1)) {
            open += 1;
        }
        if !set.contains(&(cube.0, cube.1, cube.2 - 1)) {
            open += 1;
        }
    }
    println!("{}", open);
}
