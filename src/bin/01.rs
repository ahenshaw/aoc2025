advent_of_code::solution!(1);
use aoc_parse::{parser, prelude::*};

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zeros = 0;
    let p = parser!(lines(upper i32));
    let lines = p.parse(input).unwrap();
    for (dir, clicks) in lines {
        dial += if dir == 'L' {-clicks} else {clicks};
        dial %= 100;
        if dial == 0 {
            zeros += 1;
        }
    }
    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial = 50;
    let mut zeros = 0;
    let p = parser!(lines(upper i32));
    let lines = p.parse(input).unwrap();
    for (dir, clicks) in lines {
        dial += if dir == 'L' {-clicks} else {clicks};
        while dial < 0 {
            dial += 100;
            zeros += 1;
        }
        while dial >= 100 {
            dial -= 100;
            zeros += 1;
        }
        if dial == 0 {
            zeros += 1;
        }
        println!("{dial} {dir} {clicks} {zeros}");
    }
    Some(zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
