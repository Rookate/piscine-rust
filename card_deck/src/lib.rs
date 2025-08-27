use rand::Rng;
#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}
#[derive(Debug)]

pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::rng();
        let n = rng.random_range(1..4);
        Suit::translate(n)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid rank"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::rng();
        let n: u8 = rng.random_range(1..12);
        Rank::translate(n)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            value if value >= 2 && value < 11 => Rank::Number(value),
            _ => panic!("invalid Rank!!"),
        }
    }
}
#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    matches!(
        card,
        Card {
            suit: Suit::Spade,
            rank: Rank::Ace
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let your_card = Card {
            rank: Rank::random(),
            suit: Suit::random(),
        };

        println!("Your card is {:?}", &your_card);

        if winner_card(&your_card) {
            println!("You are the winner!");
        }
    }
}
