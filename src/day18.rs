use std::collections::{HashMap, HashSet};
use std::fs;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

pub(crate) fn day18() {
    let input = fs::read_to_string("input/day18/input.txt").unwrap();
    let droplets: HashSet<(i32, i32, i32)> = HashSet::from_iter(input.lines().into_iter()
        .map(|line| {
            let coords: Vec<&str> = line.split(",").collect();
            let x = coords[0].parse::<i32>().unwrap();
            let y = coords[1].parse::<i32>().unwrap();
            let z = coords[2].parse::<i32>().unwrap();
            (x, y, z)
        }));

    let dirs = vec!(
        (-1, 0, 0), // Left
        (1, 0, 0), // Right
        (0, -1, 0), // Down
        (0, 1, 0), // Up
        (0, 0, -1), // Back
        (0, 0, 1), // Forth
    );

    let mut possible_air_gaps: HashMap<(i32, i32, i32), usize> = HashMap::new();
    let ans: usize = droplets.iter()
        .map(|(x, y, z)| {
            dirs.iter().filter(|(dx, dy, dz)| {
                *possible_air_gaps.entry((x + dx, y + dy, z + dz)).or_insert(0) += 1;
                !droplets.contains(&(x + dx, y + dy, z + dz))
            }).count()
        })
        .sum();
    println!("{}", ans);

    let max_x = *droplets.iter().map(|(x, _, _)| x).max().unwrap();
    let max_y = *droplets.iter().map(|(_, y, _)| y).max().unwrap();
    let max_z = *droplets.iter().map(|(_, _, z)| z).max().unwrap();
    let min_x = *droplets.iter().map(|(x, _, _)| x).min().unwrap();
    let min_y = *droplets.iter().map(|(_, y, _)| y).min().unwrap();
    let min_z = *droplets.iter().map(|(_, _, z)| z).min().unwrap();

    let air_gaps: Vec<(i32, i32, i32, usize)> = possible_air_gaps.iter()
        .filter(|((x, y, z), _)|
            !droplets.contains(&(*x, *y, *z))
                && min_x < *x && *x < max_x
                && min_y < *y && *y < max_y
                && min_z < *z && *z < max_z
        )
        .map(|((x, y, z), ctr)| (*x, *y, *z, *ctr))
        .collect();

    let closed = air_gaps.par_iter()
        .filter(|air_gap| {
            let mut stack = vec!();
            let mut visited = HashSet::new();
            stack.push((air_gap.0, air_gap.1, air_gap.2));

            // Run DFS from each air gap and see if we can reach outer bounds
            while !stack.is_empty() {
                let (x, y, z) = stack.pop().unwrap();
                if x <= min_x || x >= max_x
                    || y <= min_y || y >= max_y
                    || z <= min_z || z >= max_z {
                    return false;
                }
                if visited.contains(&(x, y, z)) || droplets.contains(&(x, y, z)) { continue; }
                visited.insert((x, y, z));
                dirs.iter().for_each(|(dx, dy, dz)| { stack.push((x + dx, y + dy, z + dz)); });
            }
            return true;
        })
        .map(|(_, _, _, ctr)| ctr)
        .sum::<usize>();
    println!("{}", ans - closed);
}
