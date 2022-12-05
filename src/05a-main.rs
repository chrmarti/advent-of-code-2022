use std::fs;
use regex::Regex;


fn main() {
    let stack_re = Regex::new(r"(?m)^(   |\[[A-Z]\])( (   |\[[A-Z]\]))*").unwrap();
    let cargo_re = Regex::new(r"(    ?|\[[A-Z]\] ?)").unwrap();
    let instr_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let text = fs::read_to_string("src/05-input.txt")
        .unwrap();
    
    let mut stacks = Vec::new();
    for stack_cap in stack_re.captures_iter(&text).collect::<Vec<_>>().iter().rev() {
        let cargo_caps = cargo_re.captures_iter(&stack_cap[0]).collect::<Vec<_>>();
        if stacks.is_empty() {
            for _ in 0..cargo_caps.len() {
                stacks.push(Vec::new());
            }
        }
        for i in 0..cargo_caps.len() {
            print!("{:?}", &cargo_caps[i][0]);
            let ch = cargo_caps[i][0].chars().nth(1).unwrap();
            if ch != ' ' {
                stacks[i].push(ch);
            }
        }
        println!();
    }
    println!("{:?}", stacks);

    for instr_cap in instr_re.captures_iter(&text) {
        let n = instr_cap[1].parse::<usize>().unwrap();
        let from = instr_cap[2].parse::<usize>().unwrap();
        let to = instr_cap[3].parse::<usize>().unwrap();
        println!("move {} from {} to {}", n, from, to);
        for _ in 0..n {
            let cargo = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(cargo);
        }
    }
    println!("{:?}", stacks);
    let tops = stacks.iter().map(|stack| stack.last().unwrap().to_string()).collect::<Vec<_>>();
    let res = tops.join("");
    println!("{:?}", res);
}
