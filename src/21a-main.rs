use std::{fs, collections::HashMap};
use regex::Regex;

#[derive(Debug)]
struct Monkey {
    id: String,
    left_id: Option<String>,
    op: Option<String>,
    right_id: Option<String>,
    number: Option<isize>,
}

fn main() {
    let line_re = Regex::new(r"(\S+): (\S+)( (\S) (\S+))?").unwrap();

    let text = fs::read_to_string("src/21-input.txt").unwrap();
    // root: pppw + sjmn
    // dbpl: 5

    let mut monkeys = Vec::new();
    let mut id_to_index = HashMap::new();
    let mut propagate = Vec::new();
    let mut id_to_back_refs: HashMap::<String, Vec<String>> = HashMap::new();

    for line_cap in line_re.captures_iter(&text) {
        let id = &line_cap[1];
        let monkey: Monkey;
        if line_cap.get(3).is_some() {
            let left_id = &line_cap[2];
            let op = &line_cap[4];
            let right_id = &line_cap[5];
            monkey = Monkey {
                id: id.to_string(),
                left_id: Some(left_id.to_string()),
                op: Some(op.to_string()),
                right_id: Some(right_id.to_string()),
                number: None,
            };
            id_to_back_refs.entry(left_id.to_string()).or_default().push(id.to_string());
            id_to_back_refs.entry(right_id.to_string()).or_default().push(id.to_string());
        } else {
            let number = line_cap[2].parse::<isize>().unwrap();
            monkey = Monkey {
                id: id.to_string(),
                number: Some(number),
                left_id: None,
                op: None,
                right_id: None,
            };
            propagate.push(id.to_string());
        }
        monkeys.push(monkey);
        id_to_index.insert(id.to_string(), monkeys.len() - 1);
    }

    while !propagate.is_empty() {
        let mut new_propagate = Vec::new();
        for monkey_id in propagate {
            if let Some(back_refs) = id_to_back_refs.get(&monkey_id) {
                for back_ref in back_refs {
                    let monkey = &mut monkeys[id_to_index[back_ref]];
                    let left_index = id_to_index[&monkey.left_id.clone().unwrap()];
                    let right_index = id_to_index[&monkey.right_id.clone().unwrap()];
                    let left_monkey = &monkeys[left_index];
                    let right_monkey = &monkeys[right_index];
                    if let Some(left_number) = left_monkey.number {
                        if let Some(right_number) = right_monkey.number {
                            let monkey = &mut monkeys[id_to_index[back_ref]];
                            let new_number = match monkey.op.clone().unwrap().as_str() {
                                "+" => left_number + right_number,
                                "*" => left_number * right_number,
                                "-" => left_number - right_number,
                                "/" => left_number / right_number,
                                _ => panic!("Unknown op"),
                            };
                            monkey.number = Some(new_number);
                            new_propagate.push(monkey.id.clone());
                        }
                    }
                }
            }
        }
        propagate = new_propagate;
    }

    println!("{:?}", monkeys[id_to_index["root"]].number.unwrap());
}
