use aoclib::*;
use day_11::solutions::naive::Puzzle;
use lazy_static::lazy_static;
use std::fs::read_to_string;

lazy_static! {
    static ref INPUT: String = read_to_string("input.txt").unwrap();
}

#[test]
#[ignore]
fn part_one() {
    if let Some(solution) = Puzzle::solve(&INPUT).unwrap().0 {
        println!("Part One: {}", solution);
        return;
    }
    panic!()
}

#[test]
#[ignore]
fn part_two() {
    if let Some(solution) = Puzzle::solve(&INPUT).unwrap().1 {
        println!("Part Two: {}", solution);
        return;
    }
    panic!()
}
