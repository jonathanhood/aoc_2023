use clap::Parser;
use clap::Subcommand;

fn calibration_value_for(msg: &str) -> Option<i32> {
    if let Some(left_index) = msg.find(|c: char| c.is_numeric()) {
        if let Some(right_index) = msg.rfind(|c: char| c.is_numeric()) {
            let left = msg.chars().nth(left_index).unwrap();
            let right = msg.chars().nth(right_index).unwrap();
            let calibration = format!("{}{}", left, right);
            let calibration = calibration.parse::<i32>().unwrap();
            return Some(calibration);
        }
    }

    None
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Day1 { input_path: String },
    Day2 {}
}

fn main() {
    let args = Args::parse();

    if let Command::Day1 { input_path } = args.command {
        let input = std::fs::read_to_string(input_path).unwrap();
        let mut calibrations = Vec::<i32>::new();

        for line in input.lines() {
            if let Some(calibration) = calibration_value_for(line) {
                calibrations.push(calibration);
            }
        }

        println!("Calibration: {}", calibrations.iter().sum::<i32>());
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn day1_example() {
        let example = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ";

        let mut calibrations = Vec::<i32>::new();

        for line in example.lines() {
            if let Some(calibration) = super::calibration_value_for(line) {
                calibrations.push(calibration);
            }
        }

        assert_eq!(calibrations, [12, 38, 15, 77])
    }
}
