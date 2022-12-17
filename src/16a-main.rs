use std::{fs, collections::HashMap, collections::HashSet};
use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    _name: String,
    flow: usize,
    tunnels: Vec<String>,
}

#[derive(Debug, Clone)]
struct State {
    valve: String,
    sum: usize,
    flow: usize,
    open_valves: HashSet<String>,
    came_from: Option<String>,
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

    let mut states = vec![
        State {
            valve: "AA".to_string(),
            sum: 0,
            flow: 0,
            open_valves: HashSet::new(),
            came_from: None,
        }
    ];

    for i in 1..=30 {
        println!("{}: {}", i, states.len());
        let mut new_states = Vec::new();
        for mut state in states {
            state.sum += state.flow;
            let valve = &valves[&state.valve];
            if valve.flow > 0 && !state.open_valves.contains(&state.valve) {
                let mut new_state = State {
                    valve: state.valve.clone(),
                    sum: state.sum,
                    flow: state.flow + valve.flow,
                    open_valves: state.open_valves.clone(),
                    came_from: None,
                };
                new_state.open_valves.insert(state.valve.clone());
                new_states.push(new_state);
            }
            for tunnel in &valve.tunnels {
                if state.came_from.is_some() && state.came_from.as_ref().unwrap() == tunnel {
                    continue;
                }
                let new_state = State {
                    valve: tunnel.clone(),
                    sum: state.sum,
                    flow: state.flow,
                    open_valves: state.open_valves.clone(),
                    came_from: Some(state.valve.clone()),
                };
                new_states.push(new_state);
            }
        }
        states = new_states.clone().into_iter().filter(|new_state| {
            !new_states.iter().any(|x| {
                x.valve == new_state.valve && x.sum > new_state.sum && !new_state.open_valves.iter().any(|y| !x.open_valves.contains(y))
            })
        }).collect::<Vec<_>>();
    }

    let max = states.iter().map(|x| x.sum).max().unwrap();
    println!("{}", max);
}
