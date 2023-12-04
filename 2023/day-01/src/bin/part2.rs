use std::cmp::Ordering;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // The digit number, the starting position, length of the digit
            let mut digits: Vec<(u32, usize, usize)> = vec![];
            digits.append(
                &mut line
                    .chars()
                    .enumerate()
                    .filter_map(|(index, ch)| {
                        if ch.is_digit(10) {
                            Some((ch.to_digit(10).unwrap(), index, 1))
                        } else {
                            None
                        }
                    })
                    .collect(),
            );
            for (index, word) in DIGITS.iter().enumerate() {
                let digit = (index + 1) as u32;
                digits.append(
                    &mut line
                        .match_indices(*word)
                        .map(|index| (digit, index.0, word.len()))
                        .collect(),
                );
            }

            digits.sort_by(|a, b| {
                if a.1 < b.1 {
                    Ordering::Less
                } else if a.1 == b.1 {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            });

            let first_digit = digits[0];
            let last_digit = if digits[digits.len() - 1].1 < first_digit.1 + first_digit.2 {
                first_digit
            } else {
                digits[digits.len() - 1]
            };

            ((first_digit.0 * 10) + last_digit.0) as u64
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
",
        );
        assert_eq!(result, 281)
    }
}
