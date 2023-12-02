use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
number = _{ ('0' .. '9')+ }
color = { "red" | "green" | "blue" }
num_pulled = { number }
pull = {  WHITE_SPACE? ~ num_pulled ~ WHITE_SPACE ~ color ~ ","? }
turn = { pull+ ~ ";"? }
id = { number }
game = { "Game " ~ id ~ ":" ~ turn+ ~ NEWLINE? }
"#]
struct StoneGameParser;

#[derive(Debug, PartialEq)]
pub struct StoneGamePull {
    color: String,
    num_pulled: i32,
}

impl StoneGamePull {
    fn parse_rule(rule: Pair<'_, Rule>) -> StoneGamePull {
        let mut color = String::new();
        let mut num_pulled = 0;
        for pull_inner in rule.into_inner() {
            match pull_inner.as_rule() {
                Rule::color => {
                    color = pull_inner.as_str().to_string();
                }
                Rule::num_pulled => {
                    num_pulled = pull_inner.as_str().parse().unwrap();
                }
                _ => {}
            }
        }

        StoneGamePull { color, num_pulled }
    }
}

#[derive(Debug, PartialEq)]
pub struct StoneGameTurn {
    pulls: Vec<StoneGamePull>,
}

impl StoneGameTurn {
    fn parse_rule(rule: Pair<'_, Rule>) -> StoneGameTurn {
        let mut pulls = Vec::new();

        for rule in rule.into_inner() {
            if rule.as_rule() == Rule::pull {
                pulls.push(StoneGamePull::parse_rule(rule));
            }
        }

        StoneGameTurn { pulls }
    }

    fn pulls_for_color(&self, color: &str) -> Option<i32> {
        for pull in &self.pulls {
            if pull.color == color {
                return Some(pull.num_pulled);
            }
        }

        None
    }
}

#[derive(Debug, PartialEq)]
pub struct StoneGame {
    pub id: i32,
    turns: Vec<StoneGameTurn>,
}

impl StoneGame {
    pub fn parse_lines(input: &str) -> Vec<StoneGame> {
        let mut games = Vec::new();
        for line in input.trim().lines() {
            let line = line.trim();
            if let Some(game) = StoneGame::parse(line) {
                games.push(game);
            } else {
                println!("failed to parse line: |{}|", line);
            }
        }
        games
    }

    pub fn parse(input: &str) -> Option<StoneGame> {
        let parsed = StoneGameParser::parse(Rule::game, input).ok()?;
        let mut turns = Vec::new();
        let mut id = 0;
        for game in parsed {
            for inner in game.into_inner() {
                match inner.as_rule() {
                    Rule::id => {
                        id = inner.as_str().parse().unwrap();
                    }
                    Rule::turn => {
                        turns.push(StoneGameTurn::parse_rule(inner));
                    }
                    _ => {}
                }
            }
        }
        Some(StoneGame { id, turns })
    }

    pub fn max_pulls_for_color(&self, color: &str) -> Option<i32> {
        let mut pull_found = false;
        let mut pulls = 0;
        for turn in &self.turns {
            if let Some(turn_pulls) = turn.pulls_for_color(color) {
                pull_found = true;
                if turn_pulls > pulls {
                    pulls = turn_pulls;
                }
            }
        }

        if pull_found {
            Some(pulls)
        } else {
            None
        }
    }

    pub fn possible_for(&self, red: i32, blue: i32, green: i32) -> bool {
        let game_reds = self.max_pulls_for_color("red").unwrap_or(0);
        let game_blues = self.max_pulls_for_color("blue").unwrap_or(i32::MAX);
        let game_greens = self.max_pulls_for_color("green").unwrap_or(0);

        red >= game_reds && blue >= game_blues && green >= game_greens
    }

    pub fn power(&self) -> i32 {
        let game_reds = self.max_pulls_for_color("red").unwrap_or(1);
        let game_blues = self.max_pulls_for_color("blue").unwrap_or(1);
        let game_greens = self.max_pulls_for_color("green").unwrap_or(1);
        game_reds * game_blues * game_greens
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn possible_game1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let stonegame = StoneGame::parse(input).unwrap();
        assert!(stonegame.possible_for(12, 13, 14));
    }

    #[test]
    fn power_game1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let stonegame = StoneGame::parse(input).unwrap();
        assert_eq!(stonegame.power(), 48);
    }

    #[test]
    fn possible_game2() {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let stonegame = StoneGame::parse(input).unwrap();
        assert!(stonegame.possible_for(12, 13, 14));
    }

    #[test]
    fn power_game2() {
        let input = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let stonegame = StoneGame::parse(input).unwrap();
        assert_eq!(stonegame.power(), 12);
    }

    #[test]
    fn max_pulls_for_color() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let stonegame = StoneGame::parse(input).unwrap();
        assert!(stonegame.max_pulls_for_color("red") == Some(20));
        assert!(stonegame.max_pulls_for_color("blue") == Some(6));
        assert!(stonegame.max_pulls_for_color("green") == Some(13));
    }

    #[test]
    fn reads_input() {
        let input = std::fs::read_to_string("inputs/day2.txt").unwrap();
        let games = StoneGame::parse_lines(&input);
        assert_eq!(games.len(), 100);
    }

    #[test]
    fn parse_game_line() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let parsed = StoneGame::parse(input);

        assert_eq!(
            parsed,
            Some(StoneGame {
                id: 1,
                turns: vec![
                    StoneGameTurn {
                        pulls: vec![
                            StoneGamePull {
                                color: "blue".to_string(),
                                num_pulled: 3
                            },
                            StoneGamePull {
                                color: "red".to_string(),
                                num_pulled: 4
                            }
                        ]
                    },
                    StoneGameTurn {
                        pulls: vec![
                            StoneGamePull {
                                color: "red".to_string(),
                                num_pulled: 1
                            },
                            StoneGamePull {
                                color: "green".to_string(),
                                num_pulled: 2
                            },
                            StoneGamePull {
                                color: "blue".to_string(),
                                num_pulled: 6
                            }
                        ]
                    },
                    StoneGameTurn {
                        pulls: vec![StoneGamePull {
                            color: "green".to_string(),
                            num_pulled: 2
                        }]
                    }
                ]
            })
        );
    }

    #[test]
    fn parse_game_lines() {
        let input = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        ";
        let parsed = StoneGame::parse_lines(input);

        assert_eq!(
            parsed,
            vec![
                StoneGame {
                    id: 1,
                    turns: vec![
                        StoneGameTurn {
                            pulls: vec![
                                StoneGamePull {
                                    color: "blue".to_string(),
                                    num_pulled: 3
                                },
                                StoneGamePull {
                                    color: "red".to_string(),
                                    num_pulled: 4
                                }
                            ]
                        },
                        StoneGameTurn {
                            pulls: vec![
                                StoneGamePull {
                                    color: "red".to_string(),
                                    num_pulled: 1
                                },
                                StoneGamePull {
                                    color: "green".to_string(),
                                    num_pulled: 2
                                },
                                StoneGamePull {
                                    color: "blue".to_string(),
                                    num_pulled: 6
                                }
                            ]
                        },
                        StoneGameTurn {
                            pulls: vec![StoneGamePull {
                                color: "green".to_string(),
                                num_pulled: 2
                            }]
                        }
                    ]
                },
                StoneGame {
                    id: 2,
                    turns: vec![
                        StoneGameTurn {
                            pulls: vec![
                                StoneGamePull {
                                    color: "blue".to_string(),
                                    num_pulled: 1
                                },
                                StoneGamePull {
                                    color: "green".to_string(),
                                    num_pulled: 2
                                }
                            ]
                        },
                        StoneGameTurn {
                            pulls: vec![
                                StoneGamePull {
                                    color: "green".to_string(),
                                    num_pulled: 3
                                },
                                StoneGamePull {
                                    color: "blue".to_string(),
                                    num_pulled: 4
                                },
                                StoneGamePull {
                                    color: "red".to_string(),
                                    num_pulled: 1
                                }
                            ]
                        },
                        StoneGameTurn {
                            pulls: vec![
                                StoneGamePull {
                                    color: "green".to_string(),
                                    num_pulled: 1
                                },
                                StoneGamePull {
                                    color: "blue".to_string(),
                                    num_pulled: 1
                                }
                            ]
                        }
                    ]
                }
            ]
        );
    }
}
