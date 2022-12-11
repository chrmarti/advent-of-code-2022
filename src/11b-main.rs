use std::fs;
use num_bigint::BigUint;
use num_bigint::ToBigUint;

#[derive(Debug, Clone)]
struct Monkey {
    activity_counter: usize,
    queue: Vec<BigUint>,
    operation: String,
    divisible_by: usize,
    next_monkey_true: usize,
    next_monkey_false: usize,
}

fn main() {
    let text = fs::read_to_string("src/11-input.txt").unwrap();

    let mut monkeys = text.split("\n\n").map(|monkey| {
        let mut lines = monkey.split('\n');
        lines.next();
        let activity_counter = 0;
        let queue = lines.next().unwrap()
            .split(": ").nth(1).unwrap()
            .split(", ").map(|x| x.parse::<usize>().unwrap().to_biguint().unwrap())
            .collect();
        let operation = lines.next().unwrap()
            .split(": ").nth(1).unwrap()
            .to_string();
        let divisible_by = lines.next().unwrap()
            .split("by ").nth(1).unwrap()
            .parse::<usize>().unwrap();
        let next_monkey_true = lines.next().unwrap()
            .split("monkey ").nth(1).unwrap()
            .parse::<usize>().unwrap();
        let next_monkey_false = lines.next().unwrap()
            .split("monkey ").nth(1).unwrap()
            .parse::<usize>().unwrap();
        Monkey {
            activity_counter,
            queue,
            operation,
            divisible_by,
            next_monkey_true,
            next_monkey_false,
        }
    }).collect::<Vec<Monkey>>();
    
    let mut d = 1;
    for monkey in &monkeys {
        d *= monkey.divisible_by;
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut apply = Vec::new();
            for item in &monkey.queue {
                monkey.activity_counter += 1;
                let o = &monkey.operation;
                let new;
                if o == "new = old * old" {
                    new = item * item;
                } else if let Some(s) = o.strip_prefix("new = old * ") {
                    new = item * s.parse::<usize>().unwrap();
                } else if let Some(s) = o.strip_prefix("new = old + ") {
                    new = item + s.parse::<usize>().unwrap();
                } else {
                    panic!("Unknown operation: {}", o);
                }
                let new2 = new % d;
                let next_monkey = if &new2 % monkey.divisible_by == 0.to_biguint().unwrap() {
                    monkey.next_monkey_true
                } else {
                    monkey.next_monkey_false
                };
                apply.push((next_monkey, new2));
            }
            monkey.queue.clear();
            for (next_monkey, new) in apply {
                monkeys[next_monkey].queue.push(new);
            }
        }
    }

    monkeys.iter().enumerate().for_each(|(i, monkey)| {
        println!("Monkey #{}:", i);
        println!("  Activity counter: {}", monkey.activity_counter);
        println!("  Queue: {:?}", monkey.queue);
        // println!("  Operation: {}", monkey.operation);
        // println!("  Divisible by: {}", monkey.divisible_by);
        // println!("  Next monkey (true): {}", monkey.next_monkey_true);
        // println!("  Next monkey (false): {}", monkey.next_monkey_false);
    });

    let mut act = monkeys.iter().map(|monkey| monkey.activity_counter).collect::<Vec<_>>();
    act.sort_unstable();
    act.reverse();
    println!("{:?}", act);
    println!("{:?}", act[0] * act[1]);
}
