use std::collections::HashSet;
use std::fs;

pub(crate) fn day09() {
    part_a();
    part_b();
}

fn part_a() {
    let input = fs::read_to_string("input/day09/input.txt").unwrap();
    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(tail_pos);

    for line in input.lines() {
        let parts = line.split_once(' ').unwrap();
        let dir = parts.0;
        let amount: i32 = parts.1.parse().unwrap();

        for i in 0..amount {
            match dir {
                "R" => {
                    head_pos.0 += 1;
                }
                "L" => {
                    head_pos.0 -= 1;
                }
                "U" => {
                    head_pos.1 += 1;
                }
                "D" => {
                    head_pos.1 -= 1;
                }
                _ => panic!("Unknown dir {}", dir)
            }
            tail_pos = update_tail_pos(head_pos, tail_pos);
            visited.insert(tail_pos);
            // println!("{} {} & {} {}", head_pos.0, head_pos.1, tail_pos.0, tail_pos.1);
        }
    }

    println!("{}", visited.len())
}

fn update_tail_pos(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    if head_pos.0 == tail_pos.0 && head_pos.1 == tail_pos.1 {
        return head_pos;
    }

    if head_pos.0 == tail_pos.0 {
        if (head_pos.1 - tail_pos.1).abs() <= 1 {
            return tail_pos;
        } else if head_pos.1 > tail_pos.1 {
            return (tail_pos.0, tail_pos.1 + 1);
        } else {
            return (tail_pos.0, tail_pos.1 - 1);
        }
    } else if head_pos.1 == tail_pos.1 {
        if (head_pos.0 - tail_pos.0).abs() <= 1 {
            return tail_pos;
        } else if head_pos.0 > tail_pos.0 {
            return (tail_pos.0 + 1, tail_pos.1);
        } else {
            return (tail_pos.0 - 1, tail_pos.1);
        }
    } else {
        if (head_pos.0 - tail_pos.0).abs() <= 1 && (head_pos.1 - tail_pos.1).abs() <= 1 {
            return tail_pos;
        } else if head_pos.0 > tail_pos.0 {
            if head_pos.1 > tail_pos.1 {
                return (tail_pos.0 + 1, tail_pos.1 + 1)
            } else {
                return (tail_pos.0 + 1, tail_pos.1 - 1)
            }
        } else {
            if head_pos.1 > tail_pos.1 {
                return (tail_pos.0 - 1, tail_pos.1 + 1)
            } else {
                return (tail_pos.0 - 1, tail_pos.1 - 1)
            }
        }
    }
}

fn part_b() {
    let input = fs::read_to_string("input/day09/input.txt").unwrap();
    let mut positions: Vec<(i32, i32)> = vec!((0, 0); 10);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(positions[9]);

    for line in input.lines() {
        let parts = line.split_once(' ').unwrap();
        let dir = parts.0;
        let amount: i32 = parts.1.parse().unwrap();

        for i in 0..amount {
            match dir {
                "R" => {
                    positions[0].0 += 1;
                }
                "L" => {
                    positions[0].0 -= 1;
                }
                "U" => {
                    positions[0].1 += 1;
                }
                "D" => {
                    positions[0].1 -= 1;
                }
                _ => panic!("Unknown dir {}", dir)
            }

            for j in 1..10 {
                positions[j] = update_tail_pos(positions[j - 1], positions[j]);
            }
            visited.insert(positions[9]);
            // println!("{} {} & {} {}", head_pos.0, head_pos.1, tail_pos.0, tail_pos.1);
        }
    }

    println!("{}", visited.len())
}

#[cfg(test)]
mod day09_tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
