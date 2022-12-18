use std::fs;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge};
use regex::{Match, Regex};
use rayon::iter::ParallelIterator;

pub(crate) fn day19() {
    let input = fs::read_to_string("input/day19/input.txt").unwrap();
    let re = Regex::new(r"^Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.$").unwrap();
    let mut blueprints = vec!();
    for line in input.lines().into_iter() {
        let g = re.captures(line).unwrap();
        let costs = (
            parse_i32(g.get(1)),
            (parse_i32(g.get(2)), 0, 0),
            (parse_i32(g.get(3)), 0, 0),
            (parse_i32(g.get(4)), parse_i32(g.get(5)), 0),
            (parse_i32(g.get(6)), 0, parse_i32(g.get(7))),
        );
        blueprints.push(costs);
    }

    let ans = blueprints.par_iter()
        .map(|(id, ore_cost, clay_cost, obsidian_cost, geode_cost)|
            part_a(*id, *ore_cost, *clay_cost, *obsidian_cost, *geode_cost) * id)
        .sum::<i32>();
    println!("{}", ans);

    // let ans = blueprints.iter()
    //     .take(3)
    //     .par_bridge()
    //     .map(|(id, ore_cost, clay_cost, obsidian_cost, geode_cost)|
    //         part_b(*id, *ore_cost, *clay_cost, *obsidian_cost, *geode_cost))
    //     .product::<i32>();
    // println!("{}", ans);
}

fn parse_i32(g: Option<Match>) -> i32 {
    return g.map_or(0, |m| m.as_str().parse().unwrap());
}

type Cost = (i32, i32, i32);

#[allow(dead_code)]
fn part_a(id: i32, ore_cost: Cost, clay_cost: Cost, obsidian_cost: Cost, geode_cost: Cost) -> i32 {
    let robot_ctr = [1, 0, 0, 0];
    let resource_ctr = [0; 4];
    let robot_factory = [0, 0, 0, 0];
    let mut states = vec!();
    states.push((0, robot_ctr.clone(), resource_ctr.clone(), robot_factory.clone()));

    let mut max = i32::MIN;
    while !states.is_empty() {
        let (minute, mut robot_ctr, mut resource_ctr, mut robot_factory) = states.pop().unwrap();
        if minute == 24 {
            if resource_ctr[3] > max {
                max = resource_ctr[3];
            }
            continue;
        }

        let available_res = resource_ctr.clone();
        for i in 0..4 {
            robot_ctr[i] += robot_factory[i];
            robot_factory[i] = 0;
        }
        for i in 0..4 { resource_ctr[i] += robot_ctr[i]; }
        if (24 - minute) * robot_ctr[3].max(1) * 2 < max - resource_ctr[3] {
            continue;
        }

        if geode_cost.0 <= available_res[0] && geode_cost.1 <= available_res[1] && geode_cost.2 <= available_res[2] {
            let mut res_clone = resource_ctr.clone();
            res_clone[0] -= geode_cost.0;
            res_clone[1] -= geode_cost.1;
            res_clone[2] -= geode_cost.2;
            states.push((minute + 1, robot_ctr.clone(), res_clone, [0, 0, 0, 1]));
            continue; // Greedy - if you can build a geode robot, just do it
        }

        states.push((minute + 1, robot_ctr.clone(), resource_ctr.clone(), [0, 0, 0, 0]));
        if minute < 12 && ore_cost.0 <= available_res[0] && ore_cost.1 <= available_res[1] && ore_cost.2 <= available_res[2] {
            let mut res_clone = resource_ctr.clone();
            res_clone[0] -= ore_cost.0;
            res_clone[1] -= ore_cost.1;
            res_clone[2] -= ore_cost.2;
            states.push((minute + 1, robot_ctr.clone(), res_clone, [1, 0, 0, 0]));
        }
        if minute < 20 && clay_cost.0 <= available_res[0] && clay_cost.1 <= available_res[1] && clay_cost.2 <= available_res[2] {
            let mut res_clone = resource_ctr.clone();
            res_clone[0] -= clay_cost.0;
            res_clone[1] -= clay_cost.1;
            res_clone[2] -= clay_cost.2;
            states.push((minute + 1, robot_ctr.clone(), res_clone, [0, 1, 0, 0]));
        }
        if obsidian_cost.0 <= available_res[0] && obsidian_cost.1 <= available_res[1] && obsidian_cost.2 <= available_res[2] {
            let mut res_clone = resource_ctr.clone();
            res_clone[0] -= obsidian_cost.0;
            res_clone[1] -= obsidian_cost.1;
            res_clone[2] -= obsidian_cost.2;
            states.push((minute + 1, robot_ctr.clone(), res_clone, [0, 0, 1, 0]));
        }
    }
    return max;
}

#[allow(dead_code)]
fn part_b(id: i32, ore_cost: Cost, clay_cost: Cost, obsidian_cost: Cost, geode_cost: Cost) -> i32 {
    let robot_ctr = [1, 0, 0, 0];
    let resource_ctr = [0; 4];
    let robot_factory = [0, 0, 0, 0];
    let mut states = vec!();
    states.push((0, robot_ctr.clone(), resource_ctr.clone(), robot_factory.clone()));

    let mut max = i32::MIN;
    while !states.is_empty() {
        let state = states.pop().unwrap();
        let (minute, mut robot_ctr, mut resource_ctr, mut robot_factory) = state;
        if minute == 32 {
            if resource_ctr[3] > max {
                max = resource_ctr[3];
                println!("blueprint {}: {:?}, robots {:?}", id, resource_ctr, robot_ctr);
            }
            continue;
        }

        let available_res = resource_ctr.clone();
        for i in 0..4 {
            robot_ctr[i] += robot_factory[i];
            robot_factory[i] = 0;
        }
        for i in 0..4 { resource_ctr[i] += robot_ctr[i]; }
        if (32 - minute) * robot_ctr[3].max(1) * 5 < max - resource_ctr[3] {
            continue;
        }

        if geode_cost.0 <= available_res[0] && geode_cost.1 <= available_res[1] && geode_cost.2 <= available_res[2] {
            let mut res_clone = resource_ctr.clone();
            res_clone[0] -= geode_cost.0;
            res_clone[1] -= geode_cost.1;
            res_clone[2] -= geode_cost.2;
            states.push((minute + 1, robot_ctr.clone(), res_clone, [0, 0, 0, 1]));
            continue; // Greedy - if you can build a geode robot, just do it
        }

        states.push((minute + 1, robot_ctr.clone(), resource_ctr.clone(), [0, 0, 0, 0]));
        if ore_cost.0 <= available_res[0] && ore_cost.1 <= available_res[1] && ore_cost.2 <= available_res[2] {
            let mut res_clone = resource_ctr.clone();
            res_clone[0] -= ore_cost.0;
            res_clone[1] -= ore_cost.1;
            res_clone[2] -= ore_cost.2;
            states.push((minute + 1, robot_ctr.clone(), res_clone, [1, 0, 0, 0]));
        }
        if clay_cost.0 <= available_res[0] && clay_cost.1 <= available_res[1] && clay_cost.2 <= available_res[2] {
            let mut res_clone = resource_ctr.clone();
            res_clone[0] -= clay_cost.0;
            res_clone[1] -= clay_cost.1;
            res_clone[2] -= clay_cost.2;
            states.push((minute + 1, robot_ctr.clone(), res_clone, [0, 1, 0, 0]));
        }
        if obsidian_cost.0 <= available_res[0] && obsidian_cost.1 <= available_res[1] && obsidian_cost.2 <= available_res[2] {
            let mut res_clone = resource_ctr.clone();
            res_clone[0] -= obsidian_cost.0;
            res_clone[1] -= obsidian_cost.1;
            res_clone[2] -= obsidian_cost.2;
            states.push((minute + 1, robot_ctr.clone(), res_clone, [0, 0, 1, 0]));
        }
    }
    println!("blueprint {} finished! ans: {}", id, max);
    return max;
}
