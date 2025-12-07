#![allow(dead_code)]

use aoc25::d1::{Dial, DialRotation};
use aoc25::d2::{IdRange, NoDigitSequenceRepeated, NoDigitSequenceRepeatedTwice};
use aoc25::d3::BatteriesBank;
use aoc25::d4::PaperRollsMap;
use aoc25::d5::Database;
use aoc25::d6::{BadCephalopodMathSolver, cephalopod_grand_total};
use aoc25::d7::{count_beam_splits, count_timelines};

fn main() {
    d7_part1();
    d7_part2();
}

fn d7_part2() {
    let input = std::fs::read_to_string("inputs/d7.txt").unwrap();
    println!("D7 Part 2 Answer: {}", count_timelines(&input));
}

fn d7_part1() {
    let input = std::fs::read_to_string("inputs/d7.txt").unwrap();
    println!("D7 Part 1 Answer: {}", count_beam_splits(&input));
}

fn d6_part2() {
    let input = std::fs::read_to_string("inputs/d6.txt").unwrap();
    println!("D6 Part 2 Answer: {}", cephalopod_grand_total(&input));
}

fn d6_part1() {
    let input = std::fs::read_to_string("inputs/d6.txt").unwrap();
    let solver = input.parse::<BadCephalopodMathSolver>().unwrap();
    println!("D6 Part 1 Answer: {}", solver.grand_total());
}

fn d5_part2() {
    let input = std::fs::read_to_string("inputs/d5.txt").unwrap();
    let db = input.parse::<Database>().unwrap();
    println!("D5 Part 2 Answer: {}", db.count_fresh_ids());
}

fn d5_part1() {
    let input = std::fs::read_to_string("inputs/d5.txt").unwrap();
    let db = input.parse::<Database>().unwrap();
    println!("D5 Part 1 Answer: {}", db.count_fresh_products());
}

fn d4_part2() {
    let input = std::fs::read_to_string("inputs/d4.txt").unwrap();
    let map = input.parse::<PaperRollsMap>().unwrap();
    let accessible_count = map.count_accessible_paper_rolls_with_removal();
    println!("D4 Part 2 Answer: {}", accessible_count);
}

fn d4_part1() {
    let input = std::fs::read_to_string("inputs/d4.txt").unwrap();
    let map = input.parse::<PaperRollsMap>().unwrap();
    let accessible_count = map.count_accessible_paper_rolls();
    println!("D4 Part 1 Answer: {}", accessible_count);
}

fn d3_part2() {
    let input = std::fs::read_to_string("inputs/d3.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let bank = line.parse::<BatteriesBank>().unwrap();
        let max_joltage = bank.max_joltage(12);
        total += max_joltage;
    }
    println!("D3 Part 2 Answer: {}", total)
}

fn d3_part1() {
    let input = std::fs::read_to_string("inputs/d3.txt").unwrap();
    let mut total = 0;
    for line in input.lines() {
        let bank = line.parse::<BatteriesBank>().unwrap();
        let max_joltage = bank.max_joltage(2);
        total += max_joltage;
    }
    println!("D3 Part 1 Answer: {}", total)
}

fn d2_part2() {
    let input = std::fs::read_to_string("inputs/d2.txt").unwrap();
    let mut invalid = vec![];
    for str in input.split(",") {
        let range = str.parse::<IdRange>().unwrap();
        invalid.extend(range.collect_invalid::<NoDigitSequenceRepeated>());
    }
    let answer = invalid.iter().fold(0, |acc, n| acc + n);
    println!("D2 Part 2 Answer: {}", answer);
}

fn d2_part1() {
    let input = std::fs::read_to_string("inputs/d2.txt").unwrap();
    let mut invalid = vec![];
    for str in input.split(",") {
        let range = str.parse::<IdRange>().unwrap();
        invalid.extend(range.collect_invalid::<NoDigitSequenceRepeatedTwice>());
    }
    let answer = invalid.iter().fold(0, |acc, n| acc + n);
    println!("D2 Part 1 Answer: {}", answer);
}

fn d1_part2() {
    let input = std::fs::read_to_string("inputs/d1.txt").unwrap();
    let mut dial = Dial::<99>::new(50);
    let mut n_zeros = 0;
    for line in input.lines() {
        let rotation = line.parse::<DialRotation>().unwrap();
        let start_pos = dial.points_at();
        let n_wraps = dial.rotate_with_wraps_count(rotation);
        match rotation {
            DialRotation::Left(_) => {
                let mut count = n_wraps;
                if start_pos == 0 {
                    count -= 1;
                }
                if dial.points_at() == 0 {
                    count += 1;
                }
                n_zeros += count
            }
            DialRotation::Right(_) => n_zeros += n_wraps,
        };
    }
    println!("D1 Part 2 Answer: {}", n_zeros);
}

fn d1_part1() {
    let input = std::fs::read_to_string("inputs/d1.txt").unwrap();
    let mut dial = Dial::<99>::new(50);
    let mut n_zeros = 0;
    for line in input.lines() {
        let rotation = line.parse::<DialRotation>().unwrap();
        dial.rotate(rotation);
        if dial.points_at() == 0 {
            n_zeros += 1
        }
    }
    println!("D1 Part 1 Answer: {}", n_zeros);
}
