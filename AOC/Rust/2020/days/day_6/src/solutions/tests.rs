use super::Puzzle;
use aoclib::*;

const EXAMPLE_INPUT_1: &'static str = "abc

a
b
c

ab
ac

a
a
a
a

b";

pub mod part_one {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve(&EXAMPLE_INPUT_1).unwrap().0, Some(11))
    }
}

pub mod part_two {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve(&EXAMPLE_INPUT_1).unwrap().1, Some(6))
    }
}
