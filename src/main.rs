mod calibration;
mod stone_game;

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
    }
}
