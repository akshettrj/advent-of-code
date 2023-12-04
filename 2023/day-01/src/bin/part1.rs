fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u64 {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut digits = line.chars().filter(|c| c.is_digit(10));
            let first_digit = digits.next().unwrap();
            let last_digit = digits.last().unwrap_or(first_digit);

            let number: u64 = format!("{first_digit}{last_digit}").parse().unwrap();
            number
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
        );
        assert_eq!(result, 142)
    }
}
