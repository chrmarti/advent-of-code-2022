use std::fs;

fn main() {
    let text = fs::read_to_string("src/20-input.txt").unwrap();

    let id_to_value = text.split('\n')
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut id_to_position = id_to_value.iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    
    let mut position_to_id = id_to_value.iter()
        .enumerate()
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    
    let n = id_to_value.len();
    let mut offset = 0;

    for moving_id in 0..n {
        let move_by = id_to_value[moving_id] % (n as isize - 1);
        let mut new_position = id_to_position[moving_id] as isize;
        if move_by >= 0 {
            for position in new_position + 1..=new_position + move_by {

                let id = position_to_id[modulo(position, n)];
                let new_pos = modulo(new_position, n);
                position_to_id[new_pos] = id;
                id_to_position[id] = new_pos;

                if modulo(position - offset, n) == n - 1 {
                    offset -= 1;
                }
                
                new_position = position;
            }
    
            let new_pos = modulo(new_position, n);
            position_to_id[new_pos] = moving_id;
            id_to_position[moving_id] = new_pos;
    } else {
            for position in (new_position + move_by..new_position).rev() {

                let id = position_to_id[modulo(position, n)];
                let new_pos = modulo(new_position, n);
                position_to_id[new_pos] = id;
                id_to_position[id] = new_pos;

                if modulo(position - offset, n) == 0 {
                    offset += 1;
                }
             
                new_position = position;
            }
    
            let new_pos = modulo(new_position, n);
            position_to_id[new_pos] = moving_id;
            id_to_position[moving_id] = new_pos;
        }

        // let new_order = (0..n)
        //     .map(|position| modulo(position as isize + offset, n))
        //     .map(|pos| position_to_id[pos])
        //     .map(|id| id_to_value[id])
        //     .collect::<Vec<_>>();
        // println!("{:?}", offset);
        // println!("{:?}", new_order);
    }

    let zero_position = id_to_position[id_to_value.iter().position(|&value| value == 0).unwrap()] as isize;
    let value1 = id_to_value[position_to_id[modulo(1000 + zero_position, n)]];
    let value2 = id_to_value[position_to_id[modulo(2000 + zero_position, n)]];
    let value3 = id_to_value[position_to_id[modulo(3000 + zero_position, n)]];
    println!("{}, {}, {}", value1, value2, value3);
    let sum = value1 + value2 + value3;
    println!("{}", sum);
}

fn modulo(a: isize, b: usize) -> usize {
    (((a % b as isize) + b as isize) as usize % b) as usize
}