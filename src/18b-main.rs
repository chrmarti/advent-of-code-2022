use std::{fs, collections::HashMap};

fn main() {
    let text = fs::read_to_string("src/18-input.txt").unwrap();

    let cubes = text.split('\n')
        .map(|line| {
            let mut iter = line.split(',').map(|i| i.parse::<isize>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
        });
    let mut map = cubes.clone().map(|cube| (cube, "droplet")).collect::<HashMap<_, _>>();
    let mut open = 0;
    for cube in cubes {
        for other in neighbors(&cube) {
            if is_open(&mut map, &other) {
                open += 1;
            }
        }
    }
    println!("{}", open);
}

fn is_open(map: &mut HashMap<(isize, isize, isize), &str>, cube: &(isize, isize, isize)) -> bool {
    println!("is_open({:?})", cube);
    let mut seen = Vec::new();
    trace(map, &mut seen, cube);
    let mut res = map[cube];
    println!("is_open({:?}) = {:?}", cube, res);
    if res == "tracing" {
        res = "closed";
    }
    for cube in seen {
        map.insert(cube, res);
    }
    map[cube] == "open"
}

fn trace(map: &mut HashMap<(isize, isize, isize), &str>, seen: &mut Vec<(isize, isize, isize)>, cube: &(isize, isize, isize)) {
    println!("{:?}", cube);
    if map.contains_key(cube) {
        return;
    }
    if cube.0.abs() > 100 || cube.1.abs() > 100 || cube.2.abs() > 100 {
        map.insert(*cube, "open");
        return;
    }
    map.insert(*cube, "tracing");
    seen.push(*cube);
    for other in neighbors(cube) {
        trace(map, seen, &other);
        if map[&other] == "open" {
            map.insert(*cube, "open");
            return;
        }
        if map[&other] == "closed" {
            map.insert(*cube, "closed");
            return;
        }
    }
}

fn neighbors(cube: &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    vec![
        (cube.0 + 1, cube.1, cube.2),
        (cube.0 - 1, cube.1, cube.2),
        (cube.0, cube.1 + 1, cube.2),
        (cube.0, cube.1 - 1, cube.2),
        (cube.0, cube.1, cube.2 + 1),
        (cube.0, cube.1, cube.2 - 1),
    ]
}