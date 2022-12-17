extern crate itertools;
extern crate regex;

use colored::Colorize;
use std::env;
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
use crate::day16::day16;
use crate::day17::day17;

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
mod day16;
mod day17;
mod toolbox;

macro_rules! measure {
    ($s:stmt) => {
        let timer = std::time::Instant::now();
        $s
        println!("{}", format!("Elapsed: {:?}", timer.elapsed()).italic().dimmed());
    };
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"all".to_string()) {
        println!();
        println!("{}", format!("Advent of Code").red());
        println!("{}", format!("        //2022").red());
        println!();

        println!("{}", format!("--- day01:").underline().green());
        measure!(day01());

        println!("{}", format!("--- day02:").underline().green());
        measure!(day02());

        println!("{}", format!("--- day03:").underline().green());
        measure!(day03());

        println!("{}", format!("--- day04:").underline().green());
        measure!(day04());

        println!("{}", format!("--- day05:").underline().green());
        measure!(day05());

        println!("{}", format!("--- day06:").underline().green());
        measure!(day06());

        println!("{}", format!("--- day07:").underline().green());
        measure!(day07());

        println!("{}", format!("--- day08:").underline().green());
        measure!(day08());

        println!("{}", format!("--- day09:").underline().green());
        measure!(day09());

        println!("{}", format!("--- day10:").underline().green());
        measure!(day10());

        println!("{}", format!("--- day11:").underline().green());
        measure!(day11());

        println!("{}", format!("--- day12:").underline().green());
        measure!(day12());

        println!("{}", format!("--- day13:").underline().green());
        measure!(day13());

        println!("{}", format!("--- day14:").underline().green());
        measure!(day14());

        println!("{}", format!("--- day15:").underline().green());
        measure!(day15());

        println!("{}", format!("--- day16:").underline().green());
        measure!(day16());
    }

    println!("{}", format!("--- day17:").underline().green());
    measure!(day17());
}
