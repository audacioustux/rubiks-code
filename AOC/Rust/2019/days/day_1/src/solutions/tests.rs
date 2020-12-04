use super::Puzzle;
use aoclib::*;

pub mod part_one {
    use super::*;

    const EXAMPLE_DATA_1: &'static str = "
    12
    14
    1969
    100756";

    #[test]
    fn example_1() {
        assert_eq!(
            Puzzle::solve(&EXAMPLE_DATA_1).unwrap().0,
            Some(2 + 2 + 654 + 33583)
        )
    }
}

pub mod part_two {
    use super::*;

    const EXAMPLE_DATA_2: &'static str = "
    14
    1969
    100756";

    #[test]
    fn example_2() {
        assert_eq!(
            Puzzle::solve(&EXAMPLE_DATA_2).unwrap().1,
            Some(2 + 966 + 50346)
        )
    }
}
