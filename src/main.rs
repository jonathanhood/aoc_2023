use clap::Parser;
use clap::Subcommand;
use regex::Regex;

fn parse_value(value: &str) -> Option<i32> {
    match value {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        numeric => numeric.parse::<i32>().ok()
    }
}

fn calibration_value_for_line(line: String) -> Option<i32> {
    let re: Regex = Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut line = line;
    let mut numbers = Vec::<i32>::new();

    while !line.is_empty() {
        if let Some(number) = re.find(&line) {
            if let Some(number) = parse_value(number.as_str()) {
                numbers.push(number);
            }            
        }
        line.drain(..1);
    }

    if let Some(first_number) = numbers.first() {
        if let Some(last_number) = numbers.last() {
            let number = format!("{}{}", first_number, last_number);
            return number.parse::<i32>().ok();
        }
    }

    None
}

fn calibration_value_for_corpus(corpus: &str) -> i32 {
    let mut calibrations = Vec::<i32>::new();

    for line in corpus.lines() {
        if let Some(calibration) = calibration_value_for_line(line.into()) {
            calibrations.push(calibration);
        }
    }

    calibrations.iter().sum::<i32>()
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
        let calibration = calibration_value_for_corpus(&input);

        println!("Calibration: {}", calibration);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day1_example() {
        let example = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ";

        let calibration = calibration_value_for_corpus(&example);
        assert_eq!(calibration, 142);
    }

    #[test]
    fn day1_part2_example() {
        let example = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        ";

        let calibration = calibration_value_for_corpus(&example);
        assert_eq!(calibration, 281);
    }

    #[test]
    fn day1_test_value_parsing() {
        assert_eq!(parse_value("zero"), Some(0));
        assert_eq!(parse_value("one"), Some(1));
        assert_eq!(parse_value("two"), Some(2));
        assert_eq!(parse_value("three"), Some(3));
        assert_eq!(parse_value("four"), Some(4));
        assert_eq!(parse_value("five"), Some(5));
        assert_eq!(parse_value("six"), Some(6));
        assert_eq!(parse_value("seven"), Some(7));
        assert_eq!(parse_value("eight"), Some(8));
        assert_eq!(parse_value("nine"), Some(9));

        assert_eq!(parse_value("0"), Some(0));
        assert_eq!(parse_value("1"), Some(1));
        assert_eq!(parse_value("2"), Some(2));
        assert_eq!(parse_value("3"), Some(3));
        assert_eq!(parse_value("4"), Some(4));
        assert_eq!(parse_value("5"), Some(5));
        assert_eq!(parse_value("6"), Some(6));
        assert_eq!(parse_value("7"), Some(7));
        assert_eq!(parse_value("8"), Some(8));
        assert_eq!(parse_value("9"), Some(9));
    }

    #[test]
    fn day1_solution_debug() {
        assert_eq!(calibration_value_for_line("jvhhrkrnhfivenineonethree3sixninegplzthbxj".into()), Some(59));
        assert_eq!(calibration_value_for_line("qkqgptwotvjkctgsbmsxvmssdpteightlxlkfqv46".into()), Some(26));
        assert_eq!(calibration_value_for_line("one".into()), Some(11));
        assert_eq!(calibration_value_for_line("sevenine".into()), Some(79));
        assert_eq!(calibration_value_for_line("".into()), None);
    }
}
