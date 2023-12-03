use anyhow::{Error, Result};
use std::collections::HashMap;

#[derive(PartialEq, Eq, Debug, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    id: u32,
    rounds: Vec<HashMap<Color, u32>>,
}

impl TryFrom<&str> for Game {
    type Error = anyhow::Error;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let id = line
            .split(':')
            .next()
            .ok_or(Error::msg("Input string malformed!"))?
            .split(' ')
            .nth(1)
            .ok_or(Error::msg("Input string malformed!"))?
            .parse()?;
        let rounds = line
            .split(':')
            .nth(1)
            .ok_or(Error::msg("Input string malformed!"))?
            .split(';')
            .map(|raw_round| -> Result<HashMap<Color, u32>> {
                let mut round = HashMap::new();
                for sub_round in raw_round.split(',') {
                    let num = sub_round
                        .chars()
                        .filter(char::is_ascii_digit)
                        .collect::<String>()
                        .parse::<u32>()?;

                    if sub_round.contains("blue") {
                        round.insert(Color::Blue, num);
                    } else if sub_round.contains("red") {
                        round.insert(Color::Red, num);
                    } else if sub_round.contains("green") {
                        round.insert(Color::Green, num);
                    }
                }
                Ok(round)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { id, rounds })
    }
}

impl Game {
    fn is_possible(&self, bag: &HashMap<Color, u32>) -> bool {
        for round in &self.rounds {
            for (color, n) in round.iter() {
                match bag.get(color) {
                    Some(x) => {
                        if x < n {
                            return false;
                        }
                    }
                    None => return false,
                };
            }
        }
        true
    }

    fn lowest_power(&self) -> u64 {
        let mut lowest_possible = HashMap::new();
        for round in &self.rounds {
            for (key, val) in round.iter() {
                let entry = lowest_possible.entry(key).or_insert(0);
                *entry = (*entry).max(*val as u64);
            }
        }
        lowest_possible.values().product()
    }
}

fn part_a(games: &[Game], bag: &HashMap<Color, u32>) -> u32 {
    games
        .iter()
        .filter(|g| g.is_possible(bag))
        .map(|g| g.id)
        .sum()
}

fn part_b(games: &[Game]) -> u64 {
    games
        .iter()
        .map(|g| g.lowest_power())
        .sum()
}

fn main() {
    let in_str = include_str!("../input.txt");
    let bag = {
        let mut m = HashMap::new();
        m.insert(Color::Blue, 14);
        m.insert(Color::Green, 13);
        m.insert(Color::Red, 12);
        m
    };
    let games = in_str.lines()
        .filter_map(|l| Game::try_from(l).ok())
        .collect::<Vec<Game>>();
    println!("Part A: {}", part_a(&games, &bag));
    println!("Part B: {}", part_b(&games));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_parse() {
        let round_1 = {
            let mut m = HashMap::new();
            m.insert(Color::Blue, 3);
            m.insert(Color::Red, 4);
            m
        };
        let round_2 = {
            let mut m = HashMap::new();
            m.insert(Color::Green, 2);
            m.insert(Color::Red, 1);
            m.insert(Color::Blue, 6);
            m
        };
        let round_3 = {
            let mut m = HashMap::new();
            m.insert(Color::Green, 2);
            m
        };
        let expected = Game {
            id: 1,
            rounds: vec![round_1, round_2, round_3],
        };
        let actual = Game::try_from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }

    #[test]
    fn test_possible_round() {
        let bag = {
            let mut m = HashMap::new();
            m.insert(Color::Blue, 14);
            m.insert(Color::Green, 13);
            m.insert(Color::Red, 12);
            m
        };
        let game = Game::try_from("Game 1: 3 blue, 4 red, 5 green");
        assert!(game.unwrap().is_possible(&bag));
    }

    #[test]
    fn test_impossible_round() {
        let bag = {
            let mut m = HashMap::new();
            m.insert(Color::Blue, 14);
            m.insert(Color::Green, 13);
            m.insert(Color::Red, 12);
            m
        };
        let game = Game::try_from("Game 1: 3 blue, 4 red, 15 green");
        assert!(!game.unwrap().is_possible(&bag));
    }

    #[test]
    fn test_part_a() {
        let bag = {
            let mut m = HashMap::new();
            m.insert(Color::Blue, 14);
            m.insert(Color::Green, 13);
            m.insert(Color::Red, 12);
            m
        };
        let s = include_str!("../test.txt");
        let games = s.lines()
            .filter_map(|l| Game::try_from(l).ok())
            .collect::<Vec<Game>>();
        assert_eq!(part_a(&games, &bag), 8);
    }

    #[test]
    fn test_lowest_power() {
        let puzzle = "Game 1: 4 red, 2 green, 3 blue; 2 red, 5 green, 1 blue";
        let game = Game::try_from(puzzle).unwrap();
        // 4 * 5 * 3 = 60
        assert_eq!(game.lowest_power(), 60);
    }

    #[test]
    fn test_part_b() {
        let s = include_str!("../test.txt");
        let games = s.lines()
            .filter_map(|l| Game::try_from(l).ok())
            .collect::<Vec<Game>>();
        assert_eq!(part_b(&games), 2286);
    }
}
