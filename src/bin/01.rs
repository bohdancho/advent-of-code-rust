advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));

        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);

        sum += first * 10 + last;
    }
    Some(sum)
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;

        for i in 0..line.len() {
            let digit = parse_digit_or_word(&line[i..]);
            if let Some(digit) = digit {
                first.get_or_insert(digit);
                last = Some(digit);
            }
        }

        sum += first.unwrap() * 10 + last.unwrap();
    }
    Some(sum)
}

fn parse_digit_or_word(str: &str) -> Option<u32> {
    let first_char = str.chars().next().unwrap();
    if first_char.is_numeric() {
        return first_char.to_digit(10);
    }

    DIGITS
        .iter()
        .enumerate()
        .filter(|(_, word)| word.len() <= str.len())
        .find_map(|(idx, &word)| {
            if *word == str[0..word.len()] {
                return Some(idx as u32 + 1);
            }
            None
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
