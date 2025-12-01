#![allow(dead_code)]

use aoc25::{Dial, DialRotation};

fn main() {
    d1_part1();
    d1_part2();
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
    println!("D1 Part 2 Password: {}", n_zeros)
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
    println!("D1 Part 1 Password: {}", n_zeros)
}
