fn part1(input: &str) -> u64 {
    todo!()
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
        assert_eq!(part1(input), -1)
    }
}
