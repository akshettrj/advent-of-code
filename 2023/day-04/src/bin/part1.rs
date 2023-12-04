use std::collections::HashSet;

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let card_info = line.split(':').nth(1).unwrap();
            let mut lists_iter = card_info.split('|');

            let winning_numbers: HashSet<_> = lists_iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();
            let card_numbers: HashSet<_> = lists_iter
                .next()
                .unwrap()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            let common_numbers = winning_numbers.intersection(&card_numbers).count();

            if common_numbers == 0 {
                0u64
            } else {
                1 << (common_numbers as u64 - 1)
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn test_case() {
        let input = include_str!("test_case.txt");
        assert_eq!(part1(input), 13)
    }
}
