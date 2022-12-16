use std::collections::{HashMap, VecDeque};
use std::fs;
use regex::{Regex};

pub(crate) fn day16() {
    let input = fs::read_to_string("input/day16/input.txt").unwrap();
    let mut pressures: HashMap<&str, i32> = HashMap::new();
    let mut adjacency_list: HashMap<&str, Vec<&str>> = HashMap::new();

    let re = Regex::new(r"^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? (.+)$").unwrap();
    for line in input.lines().into_iter() {
        let g = re.captures(line).unwrap();
        let from = g.get(1).unwrap().as_str();
        let flow: i32 = g.get(2).unwrap().as_str().parse().unwrap();
        pressures.insert(from, flow);

        let neighbors: Vec<&str> = g.get(3).unwrap().as_str().split(", ").into_iter().collect();
        adjacency_list.insert(from, neighbors);
    }

    let mut adjacency_list_with_time: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    for from in adjacency_list.keys() {
        let mut distance_map = HashMap::new();
        let mut stack = VecDeque::new();
        stack.push_back((from, 0));
        while !stack.is_empty() {
            let (valve, time) = stack.pop_front().unwrap();

            if *distance_map.entry(*valve).or_insert(i32::MAX) > time {
                distance_map.insert(*valve, time);
            } else {
                continue;
            }

            adjacency_list.get(valve).unwrap().iter()
                .for_each(|neighbor| stack.push_back((neighbor, time + 1)))
        }
        adjacency_list_with_time.insert(from, distance_map);
    }

    let pressured_valves: Vec<&str> = pressures.iter()
        .filter(|(_, pressure)| **pressure > 0)
        .map(|(valve, _)| *valve)
        .collect();

    part_a(pressures.clone(), adjacency_list_with_time.clone(), pressured_valves.clone());
    part_b(pressures.clone(), adjacency_list_with_time.clone(), pressured_valves.clone());
}

fn part_a(
    pressures: HashMap<&str, i32>,
    adjacency_list_with_time: HashMap<&str, HashMap<&str, i32>>,
    pressured_valves: Vec<&str>
) {
    let mut stack: Vec<(&str, Vec<&str>, i32, i32)> = vec!();
    stack.push(("AA", vec!["AA"], 30, 0));

    let mut max = i32::MIN;
    while !stack.is_empty() {
        let (from, visited, time, total) = stack.pop().unwrap();
        pressured_valves.iter()
            .filter(|to| !visited.contains(to))
            .for_each(|to| {
                let dist = adjacency_list_with_time.get(from).unwrap().get(to).unwrap();
                let mut new_visited = visited.clone();
                new_visited.push(*to);
                let next_time = time - dist - 1;

                if next_time <= 0 || visited.len() > pressured_valves.len() {
                    if total > max { max = total }
                    return;
                }
                let pressure = pressures.get(to).unwrap();
                stack.push((to, new_visited, next_time, total + (next_time * pressure)));
            })
    }
    println!("{}", max);
}

fn part_b(
    pressures: HashMap<&str, i32>,
    adjacency_list_with_time: HashMap<&str, HashMap<&str, i32>>,
    pressured_valves: Vec<&str>
) {
    let mut stack: Vec<(&str, Vec<&str>, i32, i32, bool)> = vec!();
    stack.push(("AA", vec!["AA"], 26, 0, false));
    let mut max = i32::MIN;
    while !stack.is_empty() {
        let (from, visited, time, total, elephant) = stack.pop().unwrap();
        pressured_valves.iter()
            .filter(|to| !visited.contains(to))
            .for_each(|to| {
                let dist = adjacency_list_with_time.get(from).unwrap().get(to).unwrap();
                let mut new_visited = visited.clone();
                new_visited.push(*to);
                let next_time = time - dist - 1;

                if next_time <= 0 || visited.len() > pressured_valves.len() {
                    if !elephant {
                        stack.push(("AA", new_visited, 26, total, true));
                        return;
                    }
                    if total > max { println!("{}", total); max = total }
                    return;
                }
                let pressure = pressures.get(to).unwrap();
                stack.push((to, new_visited, next_time, total + (next_time * pressure), elephant));
            })
    }
    println!("{}", max);
}
