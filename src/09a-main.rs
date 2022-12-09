use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs;

fn main() {
    let text = fs::read_to_string("src/09-input.txt").unwrap();

    let mut visits = 0;
    let mut visited = HashSet::new();

    let mut head = [0, 0];
    let mut tail = [0, 0];

    let s = format!("({},{})", tail[0], tail[1]);
    if visited.insert(s) {
        visits += 1;
    }

    let mut move_by = |head_dir: [isize; 2], distance: usize| {
        for _ in 0..distance {
            for i in 0..2 {
                head[i] += head_dir[i];
            }

            let mut tail_dir = [0, 0];
            if (head[0] - tail[0]).abs() > 1 || (head[1] - tail[1]).abs() > 1 {
                for i in 0..2 {
                    match head[i].cmp(&tail[i]) {
                        Ordering::Greater => tail_dir[i] = 1,
                        Ordering::Less => tail_dir[i] = -1,
                        Ordering::Equal => tail_dir[i] = 0,
                    }
                }
            }

            for i in 0..2 {
                tail[i] += tail_dir[i];
            }

            let s = format!("({},{})", tail[0], tail[1]);
            if visited.insert(s) {
                visits += 1;
            }

            // println!("Head: {:?}", head);
            // println!("Tail: {:?}", tail);
            // println!("Visits: {}", visits);
        }
        // println!();
    };

    let instrs = text.split('\n').map(|line| line.split(' '));
    for mut instr in instrs {
        let direction = instr.next().unwrap();
        let distance = instr.next().unwrap().parse::<usize>().unwrap();
        match direction {
            "U" => move_by([0, 1], distance),
            "D" => move_by([0, -1], distance),
            "L" => move_by([-1, 0], distance),
            "R" => move_by([1, 0], distance),
            _ => panic!("Unknown instruction: {}", direction),
        }
    }

    println!("Visits: {}", visits);
}
