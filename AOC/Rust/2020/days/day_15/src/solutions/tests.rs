use super::Puzzle;
use aoclib::*;

pub mod part_one {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve("0,3,6").unwrap().0, Some(436))
    }
    #[test]
    fn example_2() {
        assert_eq!(Puzzle::solve("1,3,2").unwrap().0, Some(1))
    }
    #[test]
    fn example_3() {
        assert_eq!(Puzzle::solve("2,1,3").unwrap().0, Some(10))
    }
    #[test]
    fn example_4() {
        assert_eq!(Puzzle::solve("1,2,3").unwrap().0, Some(27))
    }
    #[test]
    fn example_5() {
        assert_eq!(Puzzle::solve("2,3,1").unwrap().0, Some(78))
    }
    #[test]
    fn example_6() {
        assert_eq!(Puzzle::solve("3,2,1").unwrap().0, Some(438))
    }
    #[test]
    fn example_7() {
        assert_eq!(Puzzle::solve("3,1,2").unwrap().0, Some(1836))
    }
}

pub mod part_two {
    
    // #[test]
    // fn example_1() {
    //     assert_eq!(Puzzle::solve(&EXAMPLE_DATA_1).unwrap().1, Some(1068781))
    // }
}
