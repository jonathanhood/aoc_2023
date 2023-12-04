use std::collections::BinaryHeap;

use nom::IResult;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::multispace0;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::multi::many_till;
use nom::sequence::tuple;

#[derive(Clone)]
pub struct ScratchCard {
    id: usize,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>
}

impl Ord for ScratchCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id).reverse()
    }
}

impl Eq for ScratchCard {}

impl PartialOrd for ScratchCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    } 
}

impl PartialEq for ScratchCard {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl ScratchCard {
    fn parse(input: &str) -> IResult<&str, ScratchCard> {
        let number_parser = || take_while1(|c: char| c.is_digit(10));
        let (input, (_, _, _, id, _, _)) = tuple((multispace0, tag("Card"), multispace0, number_parser(), tag(":"), multispace0))(input)?;
        let id = id.parse::<usize>().unwrap();
        
        let (input, (winning_numbers, _)) = many_till(
            map_res(
                tuple((multispace0, number_parser(), multispace0)),
                |(_, number, _)| number.parse::<u32>()
            ),
            tag("|")
        )(input)?;

        let (input, my_numbers) = many1(
            map_res(
                tuple((multispace0, number_parser(), multispace0)),
                |(_, number, _)| number.parse::<u32>()
            )
        )(input)?;

        Ok((input, ScratchCard { id, winning_numbers, my_numbers }))
    }

    pub fn num_matching(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|n| self.my_numbers.contains(n))
            .count()
    }

    pub fn score(&self) -> u32 {
        let matching_count = self.num_matching();

        if matching_count == 0 {
            0
        } else {
            let base: u32 = 2;
            base.pow(matching_count as u32 - 1)
        }
    }
}

pub fn cards_in_corpus(input: &str) -> Vec<ScratchCard> {
    let mut result = Vec::new();
    for line in input.lines() {
        if let Ok((_, card)) = ScratchCard::parse(line) {
            result.push(card);
        }
    }
    result
}

pub fn play_game(input: &str) -> usize {
    let original_cards = cards_in_corpus(input);
    let mut count: usize = 0;
    let mut working_heap: BinaryHeap<&ScratchCard> = BinaryHeap::new();

    working_heap.extend(original_cards.iter());

    while let Some(card) = working_heap.pop() {
        let matching_count = card.num_matching();
        count += 1;

        if matching_count > 0 {
            let mut original_card_index = card.id;
            let end_index = original_card_index + matching_count;

            while original_card_index < original_cards.len() && original_card_index < end_index {
                let original_card = original_cards.get(original_card_index as usize).expect("index should be valid");
                working_heap.push(original_card);
                original_card_index += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_scratch_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (_, card) = ScratchCard::parse(input).expect("Should successfully parse");
        assert_eq!(card.id, 1);
        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(card.my_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_parse_example_input() {
        let input = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";
        
        let cards = cards_in_corpus(input);
        assert_eq!(cards.len(), 6);


        assert_eq!(cards[0].id, 1);
        assert_eq!(cards[0].winning_numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(cards[0].my_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);

        assert_eq!(cards[1].id, 2);
        assert_eq!(cards[1].winning_numbers, vec![13, 32, 20, 16, 61]);
        assert_eq!(cards[1].my_numbers, vec![61, 30, 68, 82, 17, 32, 24, 19]);

        assert_eq!(cards[2].id, 3);
        assert_eq!(cards[2].winning_numbers, vec![1, 21, 53, 59, 44]);
        assert_eq!(cards[2].my_numbers, vec![69, 82, 63, 72, 16, 21, 14, 1]);

        assert_eq!(cards[3].id, 4);
        assert_eq!(cards[3].winning_numbers, vec![41, 92, 73, 84, 69]);
        assert_eq!(cards[3].my_numbers, vec![59, 84, 76, 51, 58, 5, 54, 83]);

        assert_eq!(cards[4].id, 5);
        assert_eq!(cards[4].winning_numbers, vec![87, 83, 26, 28, 32]);
        assert_eq!(cards[4].my_numbers, vec![88, 30, 70, 12, 93, 22, 82, 36]);

        assert_eq!(cards[5].id, 6);
        assert_eq!(cards[5].winning_numbers, vec![31, 18, 13, 56, 72]);
        assert_eq!(cards[5].my_numbers, vec![74, 77, 10, 23, 35, 67, 36, 11]);
    }

    #[test]
    fn test_example_scores() {
        let input = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";
        
        let cards = cards_in_corpus(input);
        assert_eq!(cards.len(), 6);

        assert!(cards[0].score() == 8);
        assert!(cards[1].score() == 2);
        assert!(cards[2].score() == 2);
        assert!(cards[3].score() == 1);
        assert!(cards[4].score() == 0);
        assert!(cards[4].score() == 0);
    }

    #[test]
    fn play_game_example() {
        let input = "
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        ";
        
        let cards = play_game(input);
        assert_eq!(cards, 30);
    }
}