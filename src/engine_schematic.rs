use nom::IResult;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;

pub struct PartNumber {
    pub number: i32,
    len: i32,
    pos: (i32,i32)
}

struct Symbol {
    symbol: String,
    pos: (i32,i32)
}

pub struct Gear {
    pos: (i32,i32),
    pub ratio: i32
}

fn parse_part_number(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_digit(10))(input)
}

fn parse_ignored_char(input: &str) -> IResult<&str, &str> {
    tag(".")(input)
}

fn parse_symbol(input: &str) -> IResult<&str, &str> {
    is_a("!@#$%^&*()-=_+`~[]{}\\|;:'\",<>/?")(input)
}

fn parse_line(input: &str, y: i32) -> (Vec<PartNumber>, Vec<Symbol>) {
    let mut input = input;
    let mut part_numbers = Vec::new();
    let mut symbols = Vec::new();
    let mut x = 0;
    
    while !input.is_empty() {
        if let Ok((updated_input, pn)) = parse_part_number(input) {
            part_numbers.push(PartNumber {
                number: pn.parse().unwrap(),
                len: pn.len() as i32,
                pos: (x,y)
            });
            x += pn.len() as i32;
            input = updated_input;
        } else if let Ok((updated_input, ignored)) = parse_ignored_char(input) {
            x += ignored.len() as i32;
            input = updated_input;
        } else if let Ok((updated_input, symbol)) = parse_symbol(input) {
            symbols.push(Symbol { symbol: symbol.into(), pos: (x,y) });
            x += symbol.len() as i32;
            input = updated_input;
        } else {
            println!("failed to parse: {:?}", input);
            break;
        }
    }

    (part_numbers, symbols)
}

fn parse_corpus(input: &str) -> (Vec<PartNumber>, Vec<Symbol>) {
    let mut part_numbers = Vec::new();
    let mut symbols = Vec::new();
    let mut y = 0;

    for line in input.trim().lines() {
        let (pns, ss) = parse_line(line.trim(), y);
        part_numbers.extend(pns);
        symbols.extend(ss);
        y += 1;
    }

    (part_numbers, symbols)
}

fn adjacent(part_number: &PartNumber, symbol: &Symbol) -> bool {
    let test_range_x = (
        part_number.pos.0 - 1,
        part_number.pos.0 + part_number.len + 1
    );

    let test_range_y = (
        part_number.pos.1 - 1,
        part_number.pos.1 + 1
    );

    let x_bound_match = |x: i32| -> bool {
        x >= test_range_x.0 && x < test_range_x.1
    };

    let y_bound_match = |y: i32| -> bool {
        y >= test_range_y.0 && y <= test_range_y.1
    };

    x_bound_match(symbol.pos.0) && y_bound_match(symbol.pos.1)
}

fn is_valid_part_number(part_number: &PartNumber, symbols: &Vec<Symbol>) -> bool {
    for symbol in symbols {
        if adjacent(part_number, symbol) {
            return true;
        }
    }

    false
}

pub fn part_numbers_in_corpus(input: &str) -> Vec<PartNumber> {
    let (part_numbers, symbols) = parse_corpus(input);
    let mut valid_part_numbers = Vec::new();

    for part_number in part_numbers {
        if is_valid_part_number(&part_number, &symbols) {
            valid_part_numbers.push(part_number);
        }
    }

    valid_part_numbers
}

pub fn gears_in_corpus(input: &str) -> Vec<Gear> {
    let (part_numbers, symbols) = parse_corpus(input);

    let potential_gears: Vec<&Symbol> = symbols.iter()
        .filter(|s| s.symbol == "*")
        .collect();

    let mut gears = Vec::new();

    for potential_gear in potential_gears {
        let adjacent_part_numbers: Vec<&PartNumber> = part_numbers.iter()
            .filter(|pn| adjacent(pn, potential_gear))
            .collect();

        if adjacent_part_numbers.len() == 2 {
            let gear = Gear {
                pos: potential_gear.pos,
                ratio: adjacent_part_numbers[0].number * adjacent_part_numbers[1].number
            };

            gears.push(gear);
        }
    }

    gears
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_lines() {
        let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ";

        let (part_numbers, symbols) = parse_corpus(input);

        assert_eq!(part_numbers.len(), 10);
        assert_eq!(symbols.len(), 6);

        assert_eq!(symbols[0].symbol, "*");
        assert_eq!(symbols[0].pos, (3,1));

        assert_eq!(symbols[1].symbol, "#");
        assert_eq!(symbols[1].pos, (6,3));

        assert_eq!(symbols[2].symbol, "*");
        assert_eq!(symbols[2].pos, (3,4));

        assert_eq!(symbols[3].symbol, "+");
        assert_eq!(symbols[3].pos, (5,5));

        assert_eq!(symbols[4].symbol, "$");
        assert_eq!(symbols[4].pos, (3,8));

        assert_eq!(symbols[5].symbol, "*");
        assert_eq!(symbols[5].pos, (5,8));
    }

    #[test]
    fn test_valid_part_numbers() {
        let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ";

        let part_numbers = part_numbers_in_corpus(input);
        assert_eq!(part_numbers.len(), 8);

        assert_eq!(part_numbers[0].number, 467);
        assert_eq!(part_numbers[0].pos, (0,0));

        assert_eq!(part_numbers[1].number, 35);
        assert_eq!(part_numbers[1].pos, (2, 2));

        assert_eq!(part_numbers[2].number, 633);
        assert_eq!(part_numbers[2].pos, (6, 2));

        assert_eq!(part_numbers[3].number, 617);
        assert_eq!(part_numbers[3].pos, (0,4));

        assert_eq!(part_numbers[4].number, 592);
        assert_eq!(part_numbers[4].pos, (2,6));

        assert_eq!(part_numbers[5].number, 755);
        assert_eq!(part_numbers[5].pos, (6,7));

        assert_eq!(part_numbers[6].number, 664);
        assert_eq!(part_numbers[6].pos, (1,9));

        assert_eq!(part_numbers[7].number, 598);
        assert_eq!(part_numbers[7].pos, (5,9));
    }

    #[test]
    fn test_valid_gears() {
        let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ";

        let gears = gears_in_corpus(input);
        assert_eq!(gears.len(), 2);

        assert_eq!(gears[0].ratio, 467 * 35);
        assert_eq!(gears[1].ratio, 755 * 598);
    }
}