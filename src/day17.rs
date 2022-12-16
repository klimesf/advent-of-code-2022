use std::collections::HashMap;
use std::fs;

pub(crate) fn day17() {
    let input = fs::read_to_string("input/day17/input.txt").unwrap();
    println!("{}", tetrisize(input.as_str(), 2022));
    println!("{}", tetrisize(input.as_str(), 1000000000000));
}

fn tetrisize(input: &str, rocks: usize) -> i64 {
    let shapes: Vec<(char, Vec<(i64, i64)>)> = vec!(
        ('-', vec!((0, 0), (1, 0), (2, 0), (3, 0))),
        ('+', vec!((1, 0), (0, 1), (1, 1), (2, 1), (1, 2))),
        ('⅃', vec!((0, 0), (1, 0), (2, 0), (2, 1), (2, 2))),
        ('|', vec!((0, 0), (0, 1), (0, 2), (0, 3))),
        ('□', vec!((0, 0), (1, 0), (0, 1), (1, 1))),
    );

    let jets: Vec<char> = input.trim().chars().collect();
    let mut rows: HashMap<(i64, i64), char> = HashMap::new();
    let mut rock = 0;
    let mut jet_ctr = 0;
    let mut pos = (2, 4);
    let mut cache: HashMap<(usize, usize, [i64; 7]), (usize, i64)> = HashMap::new();
    let mut skipped = 0;

    while rock < rocks {
        let current_height = *rows.keys().map(|(_, y)| y).max().unwrap_or(&0);
        let cache_key = (jet_ctr % jets.len(), rock % 5, birdseye_view(current_height, &rows));
        if cache.contains_key(&cache_key) {
            let (old_rock, old_height) = cache.get(&cache_key).unwrap();
            let repeat = (rocks - rock) / (rock - old_rock);
            rock += (rock - old_rock) * repeat;
            skipped += (current_height - old_height) * repeat as i64;
        } else {
            cache.insert(cache_key, (rock, current_height));
        }

        let shape = &shapes[rock % 5].1;
        let shape_letter = &shapes[rock % 5].0;
        loop {
            match jets[jet_ctr % jets.len()] {
                '<' => {
                    let new_pos = (pos.0 - 1, pos.1);
                    if check_collision(new_pos, shape, &rows) { pos = new_pos; }
                }
                '>' => {
                    let new_pos = (pos.0 + 1, pos.1);
                    if check_collision(new_pos, shape, &rows) { pos = new_pos; }
                }
                c => { panic!("Unknown jet {}", c) }
            }
            jet_ctr += 1;

            let new_pos = (pos.0, pos.1 - 1);
            if check_collision(new_pos, shape, &rows) {
                pos = new_pos;
            } else {
                break;
            }
        }

        insert_shape(pos, *shape_letter, shape, &mut rows);
        let current_height = *rows.keys().map(|(_, y)| y).max().unwrap();
        pos = (2, current_height + 4);
        rock += 1;
    }

    *rows.keys().map(|(_, y)| y).max().unwrap() + skipped
}

#[allow(dead_code)]
fn print(rows: &HashMap<(i64, i64), char>, current_height: i64) {
    for y in (1..=current_height).rev() {
        print!("|");
        for x in 0..7 {
            print!("{}", rows.get(&(x, y)).unwrap_or(&'.'));
        }
        print!("|");
        println!();
    }
    println!("+-------+");
}

fn check_collision(pos: (i64, i64), shape: &Vec<(i64, i64)>, rows: &HashMap<(i64, i64), char>) -> bool {
    pos.0 >= 0 && pos.1 > 0 && shape.iter()
        .all(|(dx, dy)| pos.0 + dx < 7 && !rows.contains_key(&(pos.0 + dx, pos.1 + dy)))
}

fn insert_shape(pos: (i64, i64), shape_letter: char, shape: &Vec<(i64, i64)>, rows: &mut HashMap<(i64, i64), char>) {
    shape.iter().for_each(|(dx, dy)| {
        rows.insert((pos.0 + dx, pos.1 + dy), shape_letter);
    });
}

fn birdseye_view(current_height: i64, map: &HashMap<(i64, i64), char>) -> [i64; 7] {
    let mut ans = [0; 7];
    map.keys().for_each(|(x, y)| {
        if *y > ans[*x as usize] {
            ans[*x as usize] = *y;
        }
    });
    for i in 0..7 { ans[i] = current_height - ans[i] }
    return ans;
}

#[cfg(test)]
mod day17_tests {
    use std::collections::HashMap;
    use crate::day17::{birdseye_view, tetrisize};

    #[test]
    fn birdseye_view_works() {
        let mut map= HashMap::new();
        map.insert((0, 1), 'x');
        map.insert((1, 2), 'x');
        map.insert((2, 3), 'x');
        map.insert((4, 3), 'x');
        map.insert((5, 2), 'x');
        map.insert((6, 1), 'x');

        assert_eq!([2, 1, 0, 3, 0, 1, 2], birdseye_view(3, &map));
    }

    #[test]
    fn part_a_works() {
        assert_eq!(3068, tetrisize(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 2022));
    }

    #[test]
    fn part_b_works() {
        assert_eq!(1514285714288, tetrisize(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>", 1000000000000));
    }
}
