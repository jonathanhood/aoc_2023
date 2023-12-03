#![allow(dead_code)]

mod calibration;
mod engine_schematic;
mod stone_game;
mod stone_game_nom;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Day1 {
        input_path: String,
    },
    Day2 {
        #[arg(short)]
        red: i32,

        #[arg(short)]
        green: i32,

        #[arg(short)]
        blue: i32,
        input_path: String,
    },
    Day2Part2 {
        input_path: String,
    },
    Day3 {
        input_path: String,
    },
    Day3Part2 {
        input_path: String,
    }
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Day1 { input_path } => {
            let input = std::fs::read_to_string(input_path).unwrap();
            let calibration = calibration::value_for_corpus(&input);

            println!("Calibration: {}", calibration);
        }
        Command::Day2 {
            red,
            green,
            blue,
            input_path,
        } => {
            let input = std::fs::read_to_string(input_path).unwrap();
            let games = stone_game::StoneGame::parse_lines(&input);
            let mut id_sum = 0;

            for game in games {
                if game.possible_for(red, blue, green) {
                    id_sum += game.id;
                }
            }

            println!("Sum of Valid IDs: {}", id_sum);
        }
        Command::Day2Part2 { input_path } => {
            let input = std::fs::read_to_string(input_path).unwrap();
            let games = stone_game::StoneGame::parse_lines(&input);
            let mut power_sum = 0;

            for game in games {
                power_sum += game.power();
            }

            println!("Sum of Power: {}", power_sum);
        }
        Command::Day3 { input_path } => {
            let input = std::fs::read_to_string(input_path).unwrap();
            let part_numbers = engine_schematic::part_numbers_in_corpus(&input);

            let mut pn_sum = 0;
            for pn in part_numbers {
                pn_sum += pn.number;
            }

            println!("Sum of Part Numbers: {}", pn_sum);
        }

        Command::Day3Part2 { input_path } => {
            let input = std::fs::read_to_string(input_path).unwrap();
            let gears = engine_schematic::gears_in_corpus(&input);

            let mut gear_ratio_sum = 0;
            for gear in gears {
                gear_ratio_sum += gear.ratio;
            }

            println!("Sum of Gear Ratios: {}", gear_ratio_sum);
        }
    }
}
