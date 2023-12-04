const MAX_RED_CUBES: u64 = 12;
const MAX_BLUE_CUBES: u64 = 14;
const MAX_GREEN_CUBES: u64 = 13;

fn part1(input: &str) -> u64 {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(line)
            }
        })
        .filter_map(|line| {
            let mut parts = line.split(": ");
            let game_part = parts.next().unwrap();
            let cubes_part = parts.next().unwrap();

            let game_id: u64 = game_part
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .unwrap();

            let mut okay = true;

            'outer: for trial in cubes_part.split("; ") {
                for cube in trial.split(", ") {
                    let mut parts = cube.split_whitespace();
                    let cube_count: u64 = parts.next().unwrap().parse().unwrap();
                    let cube_type = parts.next().unwrap();

                    if cube_type == "blue" {
                        if cube_count > MAX_BLUE_CUBES {
                            okay = false;
                            break 'outer;
                        }
                    } else if cube_type == "green" {
                        if cube_count > MAX_GREEN_CUBES {
                            okay = false;
                            break 'outer;
                        }
                    } else {
                        // red
                        if cube_count > MAX_RED_CUBES {
                            okay = false;
                            break 'outer;
                        }
                    }
                }
            }

            if okay {
                Some(game_id)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("./input1.txt");
    println!("{}", part1(input));
}

#[cfg(test)]
mod test {
    use super::part1;

    #[test]
    fn case1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8)
    }
}
