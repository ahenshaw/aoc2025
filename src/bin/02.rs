advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    for range in input.trim().split(",") {

        let (start, end) = range.split_once("-").unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();
        for id in start..=end {
            let id_str = id.to_string();
            if id_str.len() % 2 == 0 {
                let (a, b) = id_str.split_at(id_str.len()/2);
                if a == b {
                    total += id;
                }
            }

        }
    }
    Some(total as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    for range in input.trim().split(",") {

        let (start, end) = range.split_once("-").unwrap();
        let start: usize = start.parse().unwrap();
        let end: usize = end.parse().unwrap();
        for id in start..=end {
            let id_str = id.to_string();
            let length = id_str.len();
            for index in 1..length {
                if length % index == 0 {
                    let base = &id_str[..index];
                    if base.repeat(length/index) == id_str {
                        total += id;
                        break;
                    }
                }
            }
        }
    }
    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
