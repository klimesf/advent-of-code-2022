use std::collections::{HashMap, HashSet};
use std::fs;

pub(crate) fn day23() {
    let input = fs::read_to_string("input/day23/input.txt").unwrap();
    let mut map = HashSet::new();
    input.lines().enumerate().for_each(|(x, line)| line.chars().enumerate()
        .for_each(|(y, c)| {
            match c {
                '#' => { map.insert((x as i32, y as i32)); },
                _ => { }
            };
        }));

    let all_dirs = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, -1), (1, 1), (-1, 1), (-1, -1)];
    let north_dirs = ([(-1, 0), (-1, -1), (-1, 1)], (-1, 0));
    let south_dirs = ([(1, 0), (1, -1), (1, 1)], (1, 0));
    let west_dirs = ([(0, -1), (1, -1), (-1, -1)], (0, -1));
    let east_dirs = ([(0, 1), (1, 1), (-1, 1)], (0, 1));
    let ordnung = [north_dirs, south_dirs, west_dirs, east_dirs];

    let mut i = 0;
    loop {
        let mut move_ctr = 0;
        let mut proposed_dest: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        let mut proposed_dest_ctr: HashMap<(i32, i32), i32> = HashMap::new();
        let mut new_map = HashSet::new();
        'outer: for elf in &map {
            if all_dirs.iter().all(|(dx, dy)| !map.contains(&(elf.0 + dx, elf.1 + dy))) {
                new_map.insert(*elf);
                continue;
            }
            for j in i..i+4 {
                let (dirs, mv) = ordnung[j % 4];
                if dirs.iter().all(|(dx, dy)| !map.contains(&(elf.0 + dx, elf.1 + dy))) {
                    proposed_dest.insert(*elf, (elf.0 + mv.0, elf.1 + mv.1));
                    *proposed_dest_ctr.entry((elf.0 + mv.0, elf.1 + mv.1)).or_insert(0) += 1;
                    continue 'outer;
                }
            }
            new_map.insert(*elf);
        }

        proposed_dest.iter().for_each(|(elf, dest)| {
            if *proposed_dest_ctr.get(dest).unwrap() <= 1 {
                new_map.insert(*dest);
                move_ctr += 1;
            } else {
                new_map.insert(*elf);
            }
        });
        map = new_map;

        if i == 9 {
            get_empty_space(&map);
        }

        i += 1;
        if move_ctr == 0 {
            println!("{}", i);
            break;
        }
    }
}

fn get_empty_space(map: &HashSet<(i32, i32)>) {
    let max_x = map.iter().map(|(x, _)| *x).max().unwrap();
    let min_x = map.iter().map(|(x, _)| *x).min().unwrap();
    let max_y = map.iter().map(|(_, y)| *y).max().unwrap();
    let min_y = map.iter().map(|(_, y)| *y).min().unwrap();
    let mut ans = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !map.contains(&(x, y)) { ans += 1 };
        }
    }
    println!("{}", ans);
}

#[allow(dead_code)]
fn print_map(map: &HashSet<(i32, i32)>) {
    for x in -2..=9 {
        for y in -3..=10 {
            if !map.contains(&(x, y)) { print!(".") } else { print!("#") };
        }
        println!()
    }
    println!();
}

