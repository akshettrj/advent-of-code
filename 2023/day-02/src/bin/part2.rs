fn part2(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let mut parts = line.split(": ");
            let game_part = parts.next().unwrap();
            let cubes_part = parts.next().unwrap();

            let game_id: u64 = game_part
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            let mut min_red = 0;
            let mut min_blue = 0;
            let mut min_green = 0;

            for trial in cubes_part.split("; ") {
                for cube in trial.split(", ") {
                    let mut parts = cube.split_whitespace();
                    let cube_count: u64 = parts.next().unwrap().parse().unwrap();
                    let cube_type = parts.next().unwrap();

                    if cube_type == "blue" {
                        if cube_count > min_blue {
                            min_blue = cube_count;
                        }
                    } else if cube_type == "green" {
                        if cube_count > min_green {
                            min_green = cube_count;
                        }
                    } else {
                        // red
                        if cube_count > min_red {
                            min_red = cube_count;
                        }
                    }
                }
            }

            Some(min_green * min_blue * min_red)
        })
        .sum()
}

fn main() {
    let input = include_str!("./input2.txt");
    println!("{}", part2(input));
}

#[cfg(test)]
mod test {
    use super::part2;

    #[test]
    fn case1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(input), 2286)
    }
}
