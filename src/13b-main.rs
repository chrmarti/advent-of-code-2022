use std::{fs, cmp::Ordering};
use json::{JsonValue};

fn main() {
    let text = fs::read_to_string("src/13-input.txt").unwrap() + "\n[[2]]\n[[6]]";

    let mut vec = text.split_whitespace()
        .map(|s| json::parse(s).unwrap())
        .collect::<Vec<_>>();
    vec.sort_by(|a, b| {
        let cmp = compare_packets(a.clone(), b.clone());
        match cmp {
            0 => Ordering::Equal,
            -1 => Ordering::Less,
            1 => Ordering::Greater,
            _ => panic!("Something went wrong!")
        }
    });

    let one = vec.iter().position(|x| json::stringify(x.clone()) == "[[2]]").unwrap() + 1;
    let two = vec.iter().position(|x| json::stringify(x.clone()) == "[[6]]").unwrap() + 1;
    println!("{} {} {}", one, two, one * two);
}

fn compare_packets(j_left: JsonValue, j_right: JsonValue) -> isize {
    match j_left.clone() {
        JsonValue::Array(left) => {
            match j_right {
                JsonValue::Array(right) => {
                    let mut left_iter = left.iter();
                    let mut right_iter = right.iter();
                    loop {
                        let l = left_iter.next();
                        let r = right_iter.next();
                        if l.is_none() {
                            if r.is_none() {
                                println!("Equal: {:?} {:?}", left, right);
                                return 0;
                            } else {
                                println!("Left is shorter: {:?} {:?}", left, right);
                                return -1;
                            }
                        } else if r.is_none() {
                            println!("Right is shorter: {:?} {:?}", left, right);
                            return 1;
                        }
                        let result = compare_packets(l.unwrap().clone(), r.unwrap().clone());
                        if result != 0 {
                            return result;
                        }
                    }
                },
                JsonValue::Number(right) => {
                    compare_packets(j_left, JsonValue::Array(Vec::from([right.into()])))
                },
                _ => {
                    panic!("Something went wrong!");
                }
            }
        },
        JsonValue::Number(left) => {
            match j_right {
                JsonValue::Array(_) => {
                    compare_packets(JsonValue::Array(Vec::from([left.into()])), j_right)
                },
                JsonValue::Number(right) => {
                    let r: f32 = right.into();
                    let l: f32 = left.into();
                    let res = if l > r {
                        1
                    } else if l < r {
                        -1
                    } else {
                        0
                    };
                    println!("Compare: {} {} {}", l, r, res);
                    res
                },
                _ => {
                    panic!("Something went wrong!");
                }
            }
        },
        _ => {
            panic!("Something went wrong!");
        }
    }
}