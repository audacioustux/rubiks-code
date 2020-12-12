use super::Puzzle;
use aoclib::*;

const EXAMPLE_DATA_1: &'static str = "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

pub mod part_one {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Puzzle::find_weakness(&Puzzle::parse_input(&EXAMPLE_DATA_1).unwrap(), 5),
            127
        )
    }
}

pub mod part_two {
    use super::*;
}
