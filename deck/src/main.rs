use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let ranks = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];

        let mut cards = vec![];

        for suit in suits {
            for rank in ranks {
                let card = format!("{} of {}", rank, suit);
                cards.push(card)
            }
        }

        return Deck { cards };
    }

    fn shuffle(&mut self){
        let mut random_number_generator = rng();
        self.cards.shuffle(&mut random_number_generator);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        return self.cards.split_off(self.cards.len() - num_cards);
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();
    println!("Your Deck: {:#?}", deck);

    let cards = deck.deal(5);

    println!("Dealt Cards: {:#?}", cards);

    println!("Your Deck: {:#?}", deck);
}
