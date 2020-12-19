use super::Puzzle;
use aoclib::*;

const EXAMPLE_DATA_1: &str = "
    ..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#";

pub mod part_one {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve(&EXAMPLE_DATA_1).unwrap().0, Some(7))
    }
}

pub mod part_two {
    use super::*;

    #[test]
    fn example_2() {
        assert_eq!(
            Puzzle::solve(&EXAMPLE_DATA_1).unwrap().1,
            Some(2 * 7 * 3 * 4 * 2)
        )
    }
}
