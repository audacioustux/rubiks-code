use super::Puzzle;
use aoclib::*;

const EXAMPLE_DATA_1: &str = "
939
7,13,x,x,59,x,31,19
";

pub mod part_one {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve(&EXAMPLE_DATA_1).unwrap().0, Some(295))
    }
}

pub mod part_two {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve(&EXAMPLE_DATA_1).unwrap().1, Some(1068781))
    }
}
