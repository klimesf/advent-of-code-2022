extern crate cached;
extern crate itertools;
extern crate priority_queue;
extern crate regex;

use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;
use crate::day04::day04;
use crate::day05::day05;
use crate::day06::day06;
use crate::day07::day07;
use crate::day08::day08;
use crate::day09::day09;
use crate::day10::day10;
use crate::day11::day11;
use crate::day12::day12;
use crate::day13::day13;
use crate::day14::day14;
use crate::day15::day15;

mod io;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod toolbox;

fn main() {
    println!();
    println!("Advent of Code");
    println!("        //2022");
    println!();

    println!("--- day01:");
    day01();

    println!("--- day02:");
    day02();

    println!("--- day03:");
    day03();

    println!("--- day04:");
    day04();

    println!("--- day05:");
    day05();

    println!("--- day06:");
    day06();

    println!("--- day07:");
    day07();

    println!("--- day08:");
    day08();

    println!("--- day09:");
    day09();

    println!("--- day10:");
    day10();

    println!("--- day11:");
    day11();

    println!("--- day12:");
    day12();

    println!("--- day13:");
    day13();

    println!("--- day14:");
    day14();

    println!("--- day15:");
    day15();
}
