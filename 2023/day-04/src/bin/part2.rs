use std::collections::{HashSet, VecDeque};

fn part2(input: &str) -> u64 {
    let card_matches: Vec<_> = input
        .lines()
        .map(|line| {
            let mut lists_iter = line.split(':').nth(1).unwrap().split('|');

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

            winning_numbers.intersection(&card_numbers).count()
        })
        .collect();

    let mut answer = 0;

    let mut cards = VecDeque::from_iter(0..card_matches.len());

    while !cards.is_empty() {
        answer += 1;
        let card = cards.pop_front().unwrap();
        let matches = card_matches[card];
        let mut copies = ((card + 1)..=(card + matches))
            .take_while(|card_num| *card_num < card_matches.len())
            .collect();
        cards.append(&mut copies);
    }

    answer
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

#[cfg(test)]
mod test {
    use super::part2;

    #[test]
    fn test_case() {
        let input = include_str!("test_case.txt");
        assert_eq!(part2(input), 30)
    }
}
