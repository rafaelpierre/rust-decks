
use emojis;

#[derive(Debug)]
struct Deck {
    //Vectors can grow and shrink, as opposed to arrays
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let hearts = emojis::get("❤️").unwrap();
        let spades = emojis::get("♠️").unwrap();
        let diamonds = emojis::get("♦️").unwrap();
        let clubs = emojis::get("♣️").unwrap();

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

        //Create a "deck" variable (or "binding", in Rust)
        //Alternatively, you can use Vec::new()
        //let deck: Deck = Deck { cards: Vec::new() };

        let deck: Deck = Deck { cards };
        return deck;
    }
}

fn main() {

    let deck = Deck::new();

    println!("Here's your deck: {:#?}", deck);
}
