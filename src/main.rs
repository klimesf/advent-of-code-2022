extern crate cached;
extern crate itertools;
extern crate priority_queue;
extern crate regex;

use crate::day01::day01;
use crate::day02::day02;
use crate::day03::day03;
use crate::day04::day04;
use crate::day05::day05;

mod io;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

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
}
