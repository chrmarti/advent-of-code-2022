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
    
    let mut max_score = 0;
    let mut scores = [[1; WIDTH]; HEIGHT];

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let current = heights[i][j];

            let mut k = j + 1;
            let mut steps = 0;
            while k < WIDTH {
                steps += 1;
                if heights[i][k] >= current {
                    break;
                }
                k += 1;
            }
            scores[i][j] *= steps;

            let mut k = j as isize - 1;
            let mut steps = 0;
            while k >= 0 {
                steps += 1;
                if heights[i][k as usize] >= current {
                    break;
                }
                k -= 1;
            }
            scores[i][j] *= steps;

            let mut k = i + 1;
            let mut steps = 0;
            while k < HEIGHT {
                steps += 1;
                if heights[k][j] >= current {
                    break;
                }
                k += 1;
            }
            scores[i][j] *= steps;

            let mut k = i as isize - 1;
            let mut steps = 0;
            while k >= 0 {
                steps += 1;
                if heights[k as usize][j] >= current {
                    break;
                }
                k -= 1;
            }
            scores[i][j] *= steps;

            if scores[i][j] > max_score {
                max_score = scores[i][j];
            }
        }
    }

    println!("{:?}", heights);
    println!("{:?}", scores);
    println!("{:?}", max_score);
}
