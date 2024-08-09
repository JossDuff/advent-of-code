use std::{cmp::Ordering, fmt, str::FromStr};

pub struct Hand {
    hand_values: [u8; 5],
    hand_type: Type,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Convert hand_values to a string
        let values_str: Vec<&str> = self
            .hand_values
            .iter()
            .map(|v| match v {
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                7 => "7",
                8 => "8",
                9 => "9",
                10 => "T",
                11 => "J",
                12 => "Q",
                13 => "K",
                14 => "A",
                _ => panic!("Number doesn't match a card"),
            })
            .collect();
        let values_str = values_str.join("");

        // Write the output using the hand type and values
        write!(f, "{values_str}")
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_values == other.hand_values && self.hand_type == other.hand_type
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        // First, compare by hand_type
        match self.hand_type.cmp(&other.hand_type) {
            // if hands are equal, compare each value
            Ordering::Equal => {
                for i in 0..5 {
                    match self.hand_values[i].cmp(&other.hand_values[i]) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        _ => continue,
                    }
                }
                Ordering::Equal
            }
            other => other,
        }
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hand_values: Vec<u8> = s
            .chars()
            .map(|c| match c {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("card not found"),
            })
            .collect();
        let slice = hand_values.as_slice();
        let mut hand_values = [0; 5];
        hand_values.copy_from_slice(slice);

        let hand_type = determine_type(hand_values);
        Ok(Hand {
            hand_values,
            hand_type,
        })
    }
}
#[derive(Eq, PartialEq, Debug)]
pub enum Type {
    FiveOfKind,  // only 1 card
    FourOfKind,  // 2 cards
    FullHouse,   // 2 cards
    ThreeOfKind, // 3 cards
    TwoPair,     // 3 cards
    OnePair,     // 4 cards
    HighCard,    // 5 cards
}

impl Type {
    fn rank(&self) -> u8 {
        match self {
            Type::FiveOfKind => 7,
            Type::FourOfKind => 6,
            Type::FullHouse => 5,
            Type::ThreeOfKind => 4,
            Type::TwoPair => 3,
            Type::OnePair => 2,
            Type::HighCard => 1,
        }
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank().cmp(&other.rank()).reverse() // reverse for greatest to least
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn determine_type(hand_values: [u8; 5]) -> Type {
    // (card value, count)
    let mut unique_cards: Vec<(u8, u8)> = Vec::new();

    for val in hand_values.into_iter() {
        if let Some((_, count)) = unique_cards
            .iter_mut()
            .find(|(card_value, _)| card_value == &val)
        {
            *count += 1;
        } else {
            unique_cards.push((val, 1));
        }
    }

    match unique_cards.len() {
        1 => Type::FiveOfKind,
        2 => {
            // 2 different cards can either be four of a kind with counts of 4 and 1
            // or full house with counts of 3 and 2
            if unique_cards[0].1 == 4 || unique_cards[0].1 == 1 {
                Type::FourOfKind
            } else {
                Type::FullHouse
            }
        }
        3 => {
            // 3 different cards can either be three of a kind with counts of 3, 1, 1
            // or two pair with counts 2, 2, 1
            if unique_cards[0].1 == 3 || unique_cards[1].1 == 3 || unique_cards[2].1 == 3 {
                Type::ThreeOfKind
            } else {
                Type::TwoPair
            }
        }
        4 => Type::OnePair,
        5 => Type::HighCard,
        _ => panic!("invalid unique card length {}", unique_cards.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_five_of_a_kind() {
        assert_eq!(Type::FiveOfKind, determine_type([2; 5]))
    }

    #[test]
    fn test_four_of_a_kind() {
        let hand = [2, 2, 2, 2, 3];
        assert_eq!(Type::FourOfKind, determine_type(hand));
    }

    #[test]
    fn test_full_house() {
        let hand = [2, 2, 2, 3, 3];
        assert_eq!(Type::FullHouse, determine_type(hand));
    }

    #[test]
    fn test_three_of_a_kind() {
        let hand = [2, 2, 2, 6, 3];
        assert_eq!(Type::ThreeOfKind, determine_type(hand));
    }

    #[test]
    fn test_two_pair() {
        let hand = [2, 2, 6, 3, 3];
        assert_eq!(Type::TwoPair, determine_type(hand));
    }

    #[test]
    fn test_one_pair() {
        let hand = [2, 7, 6, 3, 3];
        assert_eq!(Type::OnePair, determine_type(hand));
    }

    #[test]
    fn test_high_card() {
        let hand = [2, 3, 4, 5, 6];
        assert_eq!(Type::HighCard, determine_type(hand));
    }

    #[test]
    fn test_ordering() {
        let mut hands = vec!["557T5", "A777A", "363Q6", "63J35", "TKKKK"];
        let ordered_hands = vec!["TKKKK", "A777A", "557T5", "363Q6", "63J35"];
        hands.sort();
        assert_eq!(ordered_hands, hands);
    }
}
