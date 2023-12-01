mod calibration;

use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Day1 { input_path: String },
    Day2 {},
}

fn main() {
    let args = Args::parse();

    if let Command::Day1 { input_path } = args.command {
        let input = std::fs::read_to_string(input_path).unwrap();
        let calibration = calibration::value_for_corpus(&input);

        println!("Calibration: {}", calibration);
    }
}
