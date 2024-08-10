use crate::hand::Hand;
use std::{cmp::Ordering, fmt, str::FromStr};

pub struct HandWager {
    pub hand: Hand,
    pub wager: u64,
}

impl PartialEq for HandWager {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.wager == other.wager
    }
}

impl Eq for HandWager {}

impl PartialOrd for HandWager {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandWager {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl FromStr for HandWager {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(" ").collect();
        assert!(split.len() == 2);
        let hand: Hand = Hand::from_str(split[0]).unwrap();
        let wager = u64::from_str(split[1]).unwrap();
        Ok(HandWager { hand, wager })
    }
}
