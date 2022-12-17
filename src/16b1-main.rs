use std::{fs, collections::{HashMap, HashSet}, collections::BTreeSet};
use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    _name: String,
    flow: usize,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    my_valve: String,
    elephant_valve: String,
    sum: usize,
    flow: usize,
    open_valves: BTreeSet<String>,
    my_came_from: Option<String>,
    elephant_came_from: Option<String>,
}

fn main() {
    let line_re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    let text = fs::read_to_string("src/16-input.txt").unwrap();

    let mut valves = HashMap::new();

    for line_cap in line_re.captures_iter(&text) {
        let name = &line_cap[1];
        let flow = line_cap[2].parse::<usize>().unwrap();
        let tunnels = line_cap[3].split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();

        valves.insert(name.to_string(), Valve {
            _name: name.to_string(),
            flow,
            tunnels,
        });
    }

    let max_open = valves.values().filter(|v| v.flow > 0).count();

    let mut states = vec![
        State {
            my_valve: "AA".to_string(),
            elephant_valve: "AA".to_string(),
            sum: 0,
            flow: 0,
            open_valves: BTreeSet::new(),
            my_came_from: None,
            elephant_came_from: None,
        }
    ];

    let mut seen = HashSet::new();
    seen.insert(states[0].clone());

    let mut max_final_sum = 0;

    for i in 1..=26 {
        println!("{}: {}", i, states.len());
        let mut my_new_states = Vec::new();
        for mut state in states {
            state.sum += state.flow;
            let valve = &valves[&state.my_valve];
            if valve.flow > 0 && !state.open_valves.contains(&state.my_valve) {
                let mut new_state = State {
                    my_valve: state.my_valve.clone(),
                    elephant_valve: state.elephant_valve.clone(),
                    sum: state.sum,
                    flow: state.flow + valve.flow,
                    open_valves: state.open_valves.clone(),
                    my_came_from: None,
                    elephant_came_from: state.elephant_came_from.clone(),
                };
                new_state.open_valves.insert(state.my_valve.clone());
                if new_state.open_valves.len() == max_open {
                    let final_sum = (26-i) * new_state.flow + new_state.sum;
                    if final_sum > max_final_sum {
                        max_final_sum = final_sum;
                        println!("All opened: {}", final_sum);
                    }
                }
                my_new_states.push(new_state);
            }
            for tunnel in &valve.tunnels {
                if state.my_came_from.is_some() && state.my_came_from.as_ref().unwrap() == tunnel {
                    continue;
                }
                let new_state = State {
                    my_valve: tunnel.clone(),
                    elephant_valve: state.elephant_valve.clone(),
                    sum: state.sum,
                    flow: state.flow,
                    open_valves: state.open_valves.clone(),
                    my_came_from: Some(state.my_valve.clone()),
                    elephant_came_from: state.elephant_came_from.clone(),
                };
                my_new_states.push(new_state);
            }
        }
        let mut new_states = Vec::new();
        for state in my_new_states {
            let valve = &valves[&state.elephant_valve];
            if valve.flow > 0 && !state.open_valves.contains(&state.elephant_valve) {
                let mut new_state = State {
                    my_valve: state.my_valve.clone(),
                    elephant_valve: state.elephant_valve.clone(),
                    sum: state.sum,
                    flow: state.flow + valve.flow,
                    open_valves: state.open_valves.clone(),
                    my_came_from: state.my_came_from.clone(),
                    elephant_came_from: None,
                };
                new_state.open_valves.insert(state.elephant_valve.clone());
                if new_state.open_valves.len() == max_open {
                    let final_sum = (26-i) * new_state.flow + new_state.sum;
                    if final_sum > max_final_sum {
                        max_final_sum = final_sum;
                        println!("All opened: {}", final_sum);
                    }
                }
                new_states.push(new_state);
            }
            for tunnel in &valve.tunnels {
                if state.elephant_came_from.is_some() && state.elephant_came_from.as_ref().unwrap() == tunnel {
                    continue;
                }
                let new_state = State {
                    my_valve: state.my_valve.clone(),
                    elephant_valve: tunnel.clone(),
                    sum: state.sum,
                    flow: state.flow,
                    open_valves: state.open_valves.clone(),
                    my_came_from: state.my_came_from.clone(),
                    elephant_came_from: Some(state.elephant_valve.clone()),
                };
                new_states.push(new_state);
            }
        }
        states = new_states.clone().into_iter().enumerate().filter(|(i, new_state)| {
            !seen.contains(new_state) &&
            !new_states.iter().enumerate().any(|(j, x)| {
                let can_compare = x.my_valve == new_state.my_valve && x.elephant_valve == new_state.elephant_valve && !new_state.open_valves.iter().any(|y| !x.open_valves.contains(y));
                can_compare && (x.sum > new_state.sum || (x.sum == new_state.sum && j < *i))
            })
        })
        .map(|x| x.1)
        .collect::<Vec<_>>();
        states.clone().into_iter().for_each(|x| { seen.insert(x); });
    }

    let max = states.iter().map(|x| x.sum).max().unwrap();
    println!("{}", max);
}
