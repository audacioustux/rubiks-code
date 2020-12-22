use super::Puzzle;
use aoclib::*;

const EXAMPLE_DATA_1: &str = "
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

pub mod part_one {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve(&EXAMPLE_DATA_1).unwrap().0, Some(71))
    }
}

pub mod part_two {

    // #[test]
    // fn example_1() {
    //     assert_eq!(Puzzle::solve(&EXAMPLE_DATA_1).unwrap().1, Some(1068781))
    // }
}
