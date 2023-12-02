use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::multispace0;
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn parse(input: &str) -> IResult<&str, Color> {
        map_res(alt((tag("red"), tag("green"), tag("blue"))), |s| match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        })(input)
    }
}

#[derive(Debug, PartialEq)]
struct Pull {
    color: Color,
    count: i32,
}

impl Pull {
    fn parse(input: &str) -> IResult<&str, Pull> {
        let (input, count) = map_res(take_while1(|c: char| c.is_digit(10)), |count_str: &str| {
            count_str.parse::<i32>()
        })(input)?;
        let (input, _) = multispace0(input)?;
        let (input, color) = Color::parse(input)?;

        Ok((input, Pull { color, count }))
    }
}

#[derive(Debug, PartialEq)]
struct Turn {
    pulls: Vec<Pull>,
}

impl Turn {
    fn parse(input: &str) -> IResult<&str, Turn> {
        let (input, pulls) = separated_list1(tuple((tag(","), multispace0)), Pull::parse)(input)?;
        Ok((input, Turn { pulls }))
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    turns: Vec<Turn>,
}

impl Game {
    fn parse_line(input: &str) -> IResult<&str, Game> {
        let (input, _) = tuple((multispace0, tag("Game"), multispace0))(input)?;
        let (input, id) = map_res(take_while1(|c: char| c.is_digit(10)), |count_str: &str| {
            count_str.parse::<i32>()
        })(input)?;
        let (input, _) = tuple((tag(":"), multispace0))(input)?;
        let (input, turns) = separated_list1(tuple((tag(";"), multispace0)), Turn::parse)(input)?;
        Ok((input, Game { id, turns }))
    }

    fn parse(input: &str) -> Vec<Game> {
        let mut games = Vec::new();
        for line in input.trim().lines() {
            if let Ok((_, game)) = Game::parse_line(line) {
                games.push(game);
            }
        }
        games
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn parse_color() {
        assert_eq!(Color::parse("red"), Ok(("", Color::Red)));
    }

    #[test]
    fn parse_pull() {
        assert_eq!(
            Pull::parse("10 red"),
            Ok((
                "",
                Pull {
                    color: Color::Red,
                    count: 10
                }
            ))
        );
    }

    #[test]
    fn parse_turn() {
        assert_eq!(
            Turn::parse("10 red, 5 blue"),
            Ok((
                "",
                Turn {
                    pulls: vec![
                        Pull {
                            color: Color::Red,
                            count: 10
                        },
                        Pull {
                            color: Color::Blue,
                            count: 5
                        }
                    ]
                }
            ))
        );
    }

    #[test]
    fn parse_game_line() {
        assert_eq!(
            Game::parse_line("Game 10: 10 red, 5 blue; 5 red; 6 blue"),
            Ok((
                "",
                Game {
                    id: 10,
                    turns: vec![
                        Turn {
                            pulls: vec![
                                Pull {
                                    color: Color::Red,
                                    count: 10
                                },
                                Pull {
                                    color: Color::Blue,
                                    count: 5
                                }
                            ]
                        },
                        Turn {
                            pulls: vec![Pull {
                                color: Color::Red,
                                count: 5
                            }]
                        },
                        Turn {
                            pulls: vec![Pull {
                                color: Color::Blue,
                                count: 6
                            }]
                        }
                    ]
                }
            ))
        );
    }

    #[test]
    fn parse_game_lines() {
        assert_eq!(
            Game::parse(
                "
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 10: 10 red, 5 blue; 5 red; 6 blue
            "
            ),
            vec![
                Game {
                    id: 2,
                    turns: vec![
                        Turn {
                            pulls: vec![
                                Pull {
                                    color: Color::Blue,
                                    count: 1
                                },
                                Pull {
                                    color: Color::Green,
                                    count: 2
                                }
                            ]
                        },
                        Turn {
                            pulls: vec![
                                Pull {
                                    color: Color::Green,
                                    count: 3
                                },
                                Pull {
                                    color: Color::Blue,
                                    count: 4
                                },
                                Pull {
                                    color: Color::Red,
                                    count: 1
                                }
                            ]
                        },
                        Turn {
                            pulls: vec![
                                Pull {
                                    color: Color::Green,
                                    count: 1
                                },
                                Pull {
                                    color: Color::Blue,
                                    count: 1
                                }
                            ]
                        }
                    ]
                },
                Game {
                    id: 10,
                    turns: vec![
                        Turn {
                            pulls: vec![
                                Pull {
                                    color: Color::Red,
                                    count: 10
                                },
                                Pull {
                                    color: Color::Blue,
                                    count: 5
                                }
                            ]
                        },
                        Turn {
                            pulls: vec![Pull {
                                color: Color::Red,
                                count: 5
                            }]
                        },
                        Turn {
                            pulls: vec![Pull {
                                color: Color::Blue,
                                count: 6
                            }]
                        }
                    ]
                }
            ]
        );
    }
}
