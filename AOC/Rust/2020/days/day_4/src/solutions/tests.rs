use super::Puzzle;
use aoclib::*;

const EXAMPLE_DATA_1: &'static str = "
    ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm
    
    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929
    
    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm
    
    hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in
    ";

pub mod part_one {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Puzzle::solve(&EXAMPLE_DATA_1).unwrap().0, Some(2))
    }
}

// pub mod part_two {
//     use super::*;
//
//     #[test]
//     fn example_2() {
//         assert_eq!(
//             Puzzle::solve(&EXAMPLE_DATA_1).unwrap().1,
//             Some(2 * 7 * 3 * 4 * 2)
//         )
//     }
// }
