use std::cmp::{min, Ordering};
use std::fs;
use itertools::Itertools;

pub(crate) fn day13() {
    let input = fs::read_to_string("input/day13/input.txt").unwrap();
    part_a(input.trim());
}

fn part_a(input: &str) {
    let ans: usize = input.split("\n\n").into_iter()
        .map(|pair| pair.split_once("\n").unwrap())
        .enumerate()
        .filter(|(_, (first, second))| is_ordered(first, second))
        .map(|(i, _)| i + 1)
        .sum();
    println!("{}", ans);
}

fn is_ordered(first: &str, second: &str) -> bool {
    let first_tree = ListTree::new(first);
    let second_tree = ListTree::new(second);
    println!("{}", first_tree.print());
    println!("{}", second_tree.print());
    first_tree.cmp(&second_tree) != Ordering::Greater
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ListTree {
    Node(Vec<ListTree>),
    Leaf(u32),
}

impl ListTree {
    fn new(s: &str) -> Self {
        let mut stack: Vec<Vec<Self>> = vec!();
        let mut children: Vec<Self> = vec!();

        for c in s.chars() {
            match c {
                '[' => {
                    stack.push(children);
                    children = vec!();
                }
                ']' => {
                    let node = Self::Node(children.clone());
                    if let Some(parent)= stack.pop() {
                        children = parent;
                        children.push(node);
                    }
                }
                ',' => { }
                _ => {
                    children.push(Self::Leaf(c.to_digit(10).unwrap()))
                }
            }
        }

        return Self::Node(children);
    }

    fn print(&self) -> String {
        match self {
            Self::Leaf(v) => { char::from_digit(*v, 10).unwrap().to_string() }
            Self::Node(children) => { format!("[{}]", children.into_iter().map(|c| c.print()).join(",")) }
        }
    }
}

impl PartialOrd<Self> for ListTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ListTree {
    fn cmp(&self, other: &Self) -> Ordering {
        return match self {
            Self::Leaf(left) => {
                match other {
                    // If both values are integers
                    Self::Leaf(right) => { left.cmp(right) }
                    // If exactly one value is an integer
                    Self::Node(_) => { Self::Node(vec!(Self::Leaf(*left))).cmp(other) }
                }
            }
            Self::Node(lchildren) => {
                match other {
                    // If exactly one value is an integer
                    Self::Leaf(right) => { self.cmp(&Self::Node(vec!(Self::Leaf(*right)))) }
                    // If both values are lists
                    Self::Node(rchildren) => {
                        for i in 0..min(lchildren.len(), rchildren.len()) {
                            let left = &lchildren[i];
                            let right = &rchildren[i];
                            if left < right { return Ordering::Less }
                            if left > right { return Ordering::Greater }
                        }
                        return lchildren.len().cmp(&rchildren.len())
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod day13_tests {
    use crate::day13::is_ordered;

    #[test]
    fn is_ordered_works_1() {
        assert_eq!(true, is_ordered("[1,1,3,1,1]", "[1,1,5,1,1]"));
    }

    #[test]
    fn is_ordered_works_2() {
        assert_eq!(true, is_ordered("[[1],[2,3,4]]", "[[1],4]"));
    }

    #[test]
    fn is_ordered_works_3() {
        assert_eq!(false, is_ordered("[9]", "[[8,7,6]]"));
    }

    #[test]
    fn is_ordered_works_4() {
        assert_eq!(true, is_ordered("[[4,4],4,4]", "[[4,4],4,4,4]"));
    }

    #[test]
    fn is_ordered_works_5() {
        assert_eq!(false, is_ordered("[7,7,7,7]", "[7,7,7]"));
    }

    #[test]
    fn is_ordered_works_6() {
        assert_eq!(true, is_ordered("[]", "[3]"));
    }

    #[test]
    fn is_ordered_works_7() {
        assert_eq!(false, is_ordered("[[[]]]", "[[]]"));
    }

    #[test]
    fn is_ordered_works_8() {
        assert_eq!(false, is_ordered("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"));
    }
}
