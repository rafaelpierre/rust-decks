
use emojis;
use rand::{thread_rng, seq::SliceRandom, rngs::OsRng};

#[derive(Debug)]
struct Deck {
    //Vectors can grow and shrink, as opposed to arrays
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let hearts = emojis::get("😍").unwrap();
        let spades = emojis::get("🔪").unwrap();
        let diamonds = emojis::get("💎").unwrap();
        let clubs = emojis::get("🏏").unwrap();

        //List of suits
        let suits = [
            hearts.as_str(),
            spades.as_str(),
            diamonds.as_str(),
            clubs.as_str()
        ];

        //List of values
        let values = ["A", "2", "3"];

        //List of cards
        let mut cards = vec![];

        //Double nested for loop
        for suit in suits.iter() {
            for value in values.iter() {
                let card = format!("{}{}", value, suit);
                cards.push(card);
            }
        }

        //Implicit return
        Deck { cards }

    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(
            self.cards.len() - num_cards
        )
    }


}

fn main() {

    let mut deck = Deck::new();

    deck.shuffle();
    //TODO: add error handling
    let cards = deck.deal(3);

    println!("Here's your hand: {:#?}", cards);
}
