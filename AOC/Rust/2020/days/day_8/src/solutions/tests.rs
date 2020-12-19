use super::Puzzle;
use aoclib::*;

const EXAMPLE_DATA_1: &str = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

pub mod part_one {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve(&EXAMPLE_DATA_1).unwrap().0, Some(5))
    }
}

pub mod part_two {
    
}
