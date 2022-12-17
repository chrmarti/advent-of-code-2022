use std::{fs, collections::HashMap, collections::{BTreeSet, HashSet, VecDeque}};
use regex::Regex;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow: usize,
    tunnels: HashMap<String, usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    team: Vec<Player>,
    open_valves: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Player {
    valve: String,
    time: usize,
    sum: usize,
    flow: usize,
}

fn main() {
    let line_re = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();

    let text = fs::read_to_string("src/16-input.txt").unwrap();

    let mut valves = HashMap::new();

    for line_cap in line_re.captures_iter(&text) {
        let name = &line_cap[1];
        let flow = line_cap[2].parse::<usize>().unwrap();
        let mut tunnels = HashMap::new();
        line_cap[3].split(", ")
            .map(|x| x.to_string())
            .for_each(|valve| {
                tunnels.insert(valve, 1);
            });

        valves.insert(name.to_string(), Valve {
            name: name.to_string(),
            flow,
            tunnels,
        });
    }

    loop {
        let tmp_valves = valves.clone();
        let maybe_remove = tmp_valves.values().find(|valve| valve.flow == 0);
        if maybe_remove.is_none() {
            break;
        }
        let remove = maybe_remove.unwrap();
        let tmp_valves = valves.clone();
        for valve in tmp_valves.values().filter(|v| v.tunnels.contains_key(&remove.name)) {
            let mut new_tunnels = valve.tunnels.clone();
            let old_time = new_tunnels[&remove.name];
            new_tunnels.remove(&remove.name);
            for (tunnel2, time2) in &remove.tunnels {
                let new_time = old_time + time2;
                if valve.name != *tunnel2 && (!new_tunnels.contains_key(tunnel2) || new_time < new_tunnels[tunnel2]) {
                    new_tunnels.insert(tunnel2.clone(), new_time);
                }
            }
            valves.insert(valve.name.clone(), Valve {
                name: valve.name.clone(),
                flow: valve.flow,
                tunnels: new_tunnels,
            });
            valves.remove(&remove.name);
        }
    }

    // println!("{:?}", valves);

    let mut states = VecDeque::from(vec![
        State {
            team: vec![
                Player {
                    valve: "AA".to_string(),
                    time: 0,
                    sum: 0,
                    flow: 0,
                },
                Player {
                    valve: "AA".to_string(),
                    time: 0,
                    sum: 0,
                    flow: 0,
                },
            ],
            open_valves: BTreeSet::new(),
        }
    ]);

    let mut seen = HashSet::new();
    seen.insert(states[0].clone());

    let mut max_final_sum = 0;

    loop {
        let state = states.pop_front().unwrap();
        for player in state.team {
            if player.time < 26 {
                let valve = &valves[&player.valve];
                if !state.open_valves.contains(&player.valve) {
                    let mut new_state = State {
                        team: vec![
                            
                        ], open_valves: () };
                    new_state.open_valves.insert(player.valve.clone());
                    new_state.team.push(Player {
                        valve: player.valve.clone(),
                        time: player.time + 1,
                        sum: player.sum + player.flow,
                        flow: player.flow + valve.flow,
                    });
                    if !seen.contains(&new_state) {
                        seen.insert(new_state.clone());
                        states.push_back(new_state);
                    }
                }
            }
        }
    }

    // for i in 1..=26 {
    //     println!("{}: {}", i, states.len());
    //     let mut my_new_states = Vec::new();
    //     for mut state in states {
    //         state.sum += state.flow;
    //         let valve = &valves[&state.my_valve];
    //         if valve.flow > 0 && !state.open_valves.contains(&state.my_valve) {
    //             let mut new_state = State {
    //                 my_valve: state.my_valve.clone(),
    //                 elephant_valve: state.elephant_valve.clone(),
    //                 sum: state.sum,
    //                 flow: state.flow + valve.flow,
    //                 open_valves: state.open_valves.clone(),
    //                 my_came_from: None,
    //                 elephant_came_from: state.elephant_came_from.clone(),
    //             };
    //             new_state.open_valves.insert(state.my_valve.clone());
    //             if new_state.open_valves.len() == max_open {
    //                 let final_sum = (26-i) * new_state.flow + new_state.sum;
    //                 if final_sum > max_final_sum {
    //                     max_final_sum = final_sum;
    //                     println!("All opened: {}", final_sum);
    //                 }
    //             }
    //             my_new_states.push(new_state);
    //         }
    //         for tunnel in &valve.tunnels {
    //             if state.my_came_from.is_some() && state.my_came_from.as_ref().unwrap() == tunnel {
    //                 continue;
    //             }
    //             let new_state = State {
    //                 my_valve: tunnel.clone(),
    //                 elephant_valve: state.elephant_valve.clone(),
    //                 sum: state.sum,
    //                 flow: state.flow,
    //                 open_valves: state.open_valves.clone(),
    //                 my_came_from: Some(state.my_valve.clone()),
    //                 elephant_came_from: state.elephant_came_from.clone(),
    //             };
    //             my_new_states.push(new_state);
    //         }
    //     }
    //     let mut new_states = Vec::new();
    //     for state in my_new_states {
    //         let valve = &valves[&state.elephant_valve];
    //         if valve.flow > 0 && !state.open_valves.contains(&state.elephant_valve) {
    //             let mut new_state = State {
    //                 my_valve: state.my_valve.clone(),
    //                 elephant_valve: state.elephant_valve.clone(),
    //                 sum: state.sum,
    //                 flow: state.flow + valve.flow,
    //                 open_valves: state.open_valves.clone(),
    //                 my_came_from: state.my_came_from.clone(),
    //                 elephant_came_from: None,
    //             };
    //             new_state.open_valves.insert(state.elephant_valve.clone());
    //             if new_state.open_valves.len() == max_open {
    //                 let final_sum = (26-i) * new_state.flow + new_state.sum;
    //                 if final_sum > max_final_sum {
    //                     max_final_sum = final_sum;
    //                     println!("All opened: {}", final_sum);
    //                 }
    //             }
    //             new_states.push(new_state);
    //         }
    //         for tunnel in &valve.tunnels {
    //             if state.elephant_came_from.is_some() && state.elephant_came_from.as_ref().unwrap() == tunnel {
    //                 continue;
    //             }
    //             let new_state = State {
    //                 my_valve: state.my_valve.clone(),
    //                 elephant_valve: tunnel.clone(),
    //                 sum: state.sum,
    //                 flow: state.flow,
    //                 open_valves: state.open_valves.clone(),
    //                 my_came_from: state.my_came_from.clone(),
    //                 elephant_came_from: Some(state.elephant_valve.clone()),
    //             };
    //             new_states.push(new_state);
    //         }
    //     }
    //     states = new_states.clone().into_iter().enumerate().filter(|(i, new_state)| {
    //         !new_states.iter().enumerate().any(|(j, x)| {
    //             let can_compare = x.my_valve == new_state.my_valve && x.elephant_valve == new_state.elephant_valve && !new_state.open_valves.iter().any(|y| !x.open_valves.contains(y));
    //             can_compare && (x.sum > new_state.sum || (x.sum == new_state.sum && j < *i))
    //         })
    //     })
    //     .map(|x| x.1)
    //     .collect::<Vec<_>>();
    // }

    // let max = states.iter().map(|x| x.sum).max().unwrap();
    // println!("{}", max);
}
