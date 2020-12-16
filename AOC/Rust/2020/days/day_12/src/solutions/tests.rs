use super::Puzzle;
use aoclib::*;

const EXAMPLE_DATA_1: &'static str = "
F10
N3
F7
R90
F11
";

pub mod part_one {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve(&EXAMPLE_DATA_1).unwrap().0, Some(25))
    }
}

pub mod part_two {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve(&EXAMPLE_DATA_1).unwrap().1, Some(286))
    }
}
