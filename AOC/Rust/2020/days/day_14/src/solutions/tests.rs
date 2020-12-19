use super::Puzzle;
use aoclib::*;

const EXAMPLE_DATA_1: &str = "
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
";

const EXAMPLE_DATA_2: &str = "
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1
";

pub mod part_one {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve(&EXAMPLE_DATA_1).unwrap().0, Some(165))
    }
}

pub mod part_two {
    use super::*;
    #[test]
    fn example_2() {
        assert_eq!(Puzzle::solve(&EXAMPLE_DATA_2).unwrap().1, Some(208))
    }
}
