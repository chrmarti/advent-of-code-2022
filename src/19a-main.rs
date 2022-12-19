use std::{fs, collections::{BTreeMap, HashSet}};
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    resources: BTreeMap<ResourceType, usize>,
    robots: BTreeMap<ResourceType, usize>
}

#[derive(Debug, Clone)]
struct AnnotatedState {
    state: State,
    did_nothing_instead: HashSet<ResourceType>,
    previous: Option<Box<AnnotatedState>>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Blueprint {
    id: usize,
    robot_costs: BTreeMap<ResourceType, BTreeMap<ResourceType, usize>>
}

fn main() {
    let line_re = Regex::new(r"Blueprint (\d+): (.*)").unwrap();
    let robot_re = Regex::new(r"Each (\w+) robot costs (\d+) (\w+)( and (\d+) (\w+))?.").unwrap();

    let text = fs::read_to_string("src/19-input.txt").unwrap();

    let mut blueprints = Vec::new();

    for line_cap in line_re.captures_iter(&text) {
        let id = line_cap[1].parse::<usize>().unwrap();
        let mut robot_costs = BTreeMap::new();
        for robot_cap in robot_re.captures_iter(&line_cap[2]) {
            let robot_type = resource_from_string(&robot_cap[1]);
            let mut this_robot_costs = BTreeMap::new();
            let cost_one = robot_cap[2].parse::<usize>().unwrap();
            let type_one = resource_from_string(&robot_cap[3]);
            this_robot_costs.insert(type_one, cost_one);
            if robot_cap.get(4).is_some() {
                let cost_two = robot_cap[5].parse::<usize>().unwrap();
                let type_two = resource_from_string(&robot_cap[6]);
                this_robot_costs.insert(type_two, cost_two);
            }
            robot_costs.insert(robot_type, this_robot_costs);
        }
        blueprints.push(Blueprint {
            id,
            robot_costs,
        });
    }

    let sum: usize = blueprints.iter().map(|blueprint| {
        
        let mut states = vec![AnnotatedState {
            state: State {
                resources: BTreeMap::new(),
                robots: BTreeMap::new()
            },
            did_nothing_instead: HashSet::new(),
            previous: None
        }];
        states[0].state.robots.insert(ResourceType::Ore, 1);

        let mut seen = HashSet::new();
        seen.insert(states[0].state.clone());

        let resource_types = resource_types();

        for i in 0..24 {
            println!("Iteration {}", i);
            println!("States {}", states.len());
            let mut new_states = Vec::new();
            for annotated in states.iter() {
                let state = &annotated.state;
                // println!("State: {:?}", state);
                let mut inner_new_states = Vec::new();
                let can_build = resource_types.iter().filter(|resource_type| {
                    blueprint.robot_costs[resource_type].iter().all(|(cost_type, cost)| {
                        state.resources.get(cost_type).unwrap_or(&0) >= cost
                    })
                }).collect::<Vec<_>>();

                let can_build_geode = can_build.iter().any(|t| **t == ResourceType::Geode);
                if can_build.len() < resource_types.len() && !can_build_geode {
                    let mut new_state = state.clone();
                    for (resource_type, count) in new_state.robots.iter() {
                        new_state.resources.entry(*resource_type)
                            .and_modify(|v| *v += count)
                            .or_insert(*count);
                    }
                    inner_new_states.push(AnnotatedState {
                        state: new_state,
                        did_nothing_instead: can_build.clone().into_iter().cloned().collect::<HashSet<_>>(),
                        previous: None
                        // previous: Some(Box::new(annotated.clone()))
                    });
                }

                for resource_type in can_build {
                    if annotated.did_nothing_instead.contains(resource_type) {
                        continue;
                    }
                    if can_build_geode && *resource_type != ResourceType::Geode {
                        continue;
                    }
                    // if state.robots.get(resource_type).unwrap_or(&0) >= blueprint.robot_costs.values().map(|costs| costs.get(resource_type).unwrap_or(&0)).max().unwrap() {
                    //     continue;
                    // }
                    let mut new_state = state.clone();
                    for (cost_type, cost) in blueprint.robot_costs[resource_type].iter() {
                        new_state.resources.entry(*cost_type).and_modify(|v| *v -= cost);
                    }
                    for (resource_type, count) in new_state.robots.iter() {
                        new_state.resources.entry(*resource_type)
                            .and_modify(|v| *v += count)
                            .or_insert(*count);
                    }
                    new_state.robots.entry(*resource_type).and_modify(|v| *v += 1).or_insert(1);
                    inner_new_states.push(AnnotatedState {
                        state: new_state,
                        did_nothing_instead: HashSet::new(),
                        previous: None
                        // previous: Some(Box::new(annotated.clone()))
                    });
                }

                for new_state in inner_new_states {
                    if seen.insert(new_state.state.clone()) {
                        new_states.push(new_state);
                    }
                }
            }
            // let new_states2 = new_states.clone();
            // states = new_states.into_iter().filter(|state| {
            //     !new_states2.iter().any(|other_state| {
            //         other_state != state &&
            //         resource_types.iter().all(|resource_type| {
            //             other_state.resources.get(resource_type).unwrap_or(&0) >= state.resources.get(resource_type).unwrap_or(&0)
            //         }) && resource_types.iter().all(|resource_type| {
            //             other_state.robots.get(resource_type).unwrap_or(&0) >= state.robots.get(resource_type).unwrap_or(&0)
            //         })
            //     })
            // }).collect();
            states = new_states;
        }

        let max_state = states.iter().max_by_key(|x| x.state.resources.get(&ResourceType::Geode).unwrap_or(&0)).unwrap();
        let mut current = max_state;
        loop {
            println!("Current: {:?}", current.state);
            if let Some(previous) = &current.previous {
                current = previous;
            } else {
                break;
            }
        }
        let max = max_state.state.resources.get(&ResourceType::Geode).unwrap_or(&0);
        println!("{}: {}", blueprint.id, max);

        blueprint.id * *max

    }).sum();
    println!("Sum: {}", sum);
}

fn resource_from_string(s: &str) -> ResourceType {
    match s {
        "ore" => ResourceType::Ore,
        "clay" => ResourceType::Clay,
        "obsidian" => ResourceType::Obsidian,
        "geode" => ResourceType::Geode,
        _ => panic!("Unknown resource: {}", s),
    }
}

fn resource_types() -> Vec<ResourceType> {
    vec![ResourceType::Ore, ResourceType::Clay, ResourceType::Obsidian, ResourceType::Geode]
}