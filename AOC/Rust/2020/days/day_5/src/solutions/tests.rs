use super::Puzzle;
use aoclib::*;

pub mod part_one {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve("FBFBBFFRLR").unwrap().0, Some(357))
    }
}

pub mod part_two {}
