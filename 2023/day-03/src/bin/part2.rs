fn part2(input: &str) -> u64 {
    let mut answer = 0;
    let mut nums = vec![];
    let mut gears = vec![];

    for (line_num, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

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

                if ch == '*' {
                    gears.push((line_num, char_num));
                }
            }
        }
        if let Some(n) = num {
            nums.push((n, line_num, num_starting, line.len()));
        }
    }

    for (gear_line, gear_indx) in gears {
        let adj_nums = nums
            .iter()
            .filter_map(|(num, line_num, start, end)| {
                if *line_num == gear_line && (gear_indx == *end || gear_indx + 1 == *start) {
                    return Some(num);
                }

                if (gear_line + 1 == *line_num || *line_num + 1 == gear_line)
                    && (gear_indx + 1 >= *start && gear_indx <= *end)
                {
                    return Some(num);
                }

                None
            })
            .collect::<Vec<_>>();

        if adj_nums.len() == 2 {
            answer += adj_nums[0] * adj_nums[1];
        }
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
        assert_eq!(part2(input), 467835)
    }
}
