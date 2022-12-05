use std::fs;

fn main() {
    let text = fs::read_to_string("src/04-input.txt")
        .unwrap();
    let sum = text
        .split('\n')
        .map(|line| line.split(',').map(|e| e.split('-').map(|e| e.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>())
        .map(|line| {
            if let [e1, e2] = &line[..] {
                if let [from1, to1] = &e1[..] {
                    if let [from2, to2] = &e2[..] {
                        let res = if (from1 <= to2 && from2 <= to1) || (from2 <= to1 && from1 <= to2) { 1 } else { 0 }; // two
                        // let res = if (from1 <= from2 && to2 <= to1) || (from2 <= from1 && to1 <= to2) { 1 } else { 0 }; // one
                        // println!("{:?}-{:?},{:?}-{:?} {:?}", from1, to1, from2, to2, res);
                        return res
                    }
                }
            }
            panic!("ooops");
        })
        .sum::<i32>();
    println!("sum: {:?}", sum);
}
