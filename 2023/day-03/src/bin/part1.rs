use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> u64 {
    let mut answer = 0;
    let mut nums = vec![];
    let mut symbols = HashMap::new();

    for (line_num, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let mut line_symbols = HashSet::new();
        let mut num_starting = 0;
        let mut num: Option<u64> = None;

        for (char_num, ch) in line.chars().enumerate() {
            if ch.is_digit(10) {
                match num {
                    Some(n) => {
                        num = Some(n * 10 + ch.to_digit(10).unwrap() as u64);
                    }
                    None => {
                        num_starting = char_num;
                        num = Some(ch.to_digit(10).unwrap() as u64);
                    }
                }
            } else {
                if let Some(n) = num {
                    nums.push((n, line_num, num_starting, char_num));
                    num = None;
                }

                if ch != '.' {
                    line_symbols.insert(char_num);
                }
            }
        }
        if let Some(n) = num {
            nums.push((n, line_num, num_starting, line.len()));
        }

        symbols.insert(line_num, line_symbols);
    }

    'outer: for (num, line_num, start, end) in nums {
        if (start > 0 && symbols[&line_num].contains(&(start - 1)))
            || (symbols[&line_num].contains(&end))
        {
            answer += num;
            continue 'outer;
        }
        for indx in (if start > 0 { start - 1 } else { 0 })..=end {
            if line_num >= 1 && symbols.contains_key(&(line_num - 1)) {
                if symbols[&(line_num - 1)].contains(&indx) {
                    answer += num;
                    continue 'outer;
                }
            }
            if symbols.contains_key(&(line_num + 1)) {
                if symbols[&(line_num + 1)].contains(&indx) {
                    answer += num;
                    continue 'outer;
                }
            }
        }
    }

    answer
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
        assert_eq!(part1(input), 4361)
    }
}
