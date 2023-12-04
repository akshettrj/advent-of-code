fn part2(input: &str) -> u64 {
    todo!()
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
        assert_eq!(part2(input), -1)
    }
}
