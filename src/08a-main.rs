use std::fs;

fn main() {
    let text = fs::read_to_string("src/08-input.txt")
        .unwrap();
    
    const WIDTH: usize = 99;
    const HEIGHT: usize = 99;
    // const WIDTH: usize = 5;
    // const HEIGHT: usize = 5;

    let mut heights = [[0; WIDTH]; HEIGHT];

    for (i, line) in text.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            heights[i][j] = ch.to_string().parse::<isize>().unwrap();
        }
    }
    
    let mut visible = 0;
    let mut visibility = [[false; WIDTH]; HEIGHT];

    for i in 0..HEIGHT {
        let mut heighest = -1;
        for j in 0..WIDTH {
            let current = heights[i][j];
            if current > heighest {
                heighest = current;
                if !visibility[i][j] {
                    visibility[i][j] = true;
                    visible += 1;
                }
            }
        }
        let mut heighest = -1;
        for j in (0..=WIDTH-1).rev() {
            let current = heights[i][j];
            if current > heighest {
                heighest = current;
                if !visibility[i][j] {
                    visibility[i][j] = true;
                    visible += 1;
                }
            }
        }
    }

    for j in 0..WIDTH {
        let mut heighest = -1;
        for i in 0..HEIGHT {
            let current = heights[i][j];
            if current > heighest {
                heighest = current;
                if !visibility[i][j] {
                    visibility[i][j] = true;
                    visible += 1;
                }
            }
        }
        let mut heighest = -1;
        for i in (0..=HEIGHT-1).rev() {
            let current = heights[i][j];
            if current > heighest {
                heighest = current;
                if !visibility[i][j] {
                    visibility[i][j] = true;
                    visible += 1;
                }
            }
        }
    }

    println!("{:?}", heights);
    println!("{:?}", visibility);
    println!("{:?}", visible);
}
