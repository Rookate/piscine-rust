#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        // génère 0..=3
        let n = pseudo_random_u8(4);
        Suit::translate(n)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            0 => Suit::Heart,
            1 => Suit::Diamond,
            2 => Suit::Spade,
            3 => Suit::Club,
            _ => panic!("Invalid suit"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let n = 1 + pseudo_random_u8(13);
        Rank::translate(n)
    }

    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            v @ 2..=10 => Rank::Number(v),
            _ => panic!("Invalid rank"),
        }
    }
}

#[derive(Debug, PartialEq)]
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

// Ton petit générateur pseudo-aléatoire basé sur l’horloge
fn pseudo_random_u8(max: u8) -> u8 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos % max as u32) as u8
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
