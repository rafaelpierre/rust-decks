
#[derive(Debug)]
struct Deck {
    //Vectors can grow and shrink, as opposed to arrays
    cards: Vec<String>,
}

fn main() {

    //List of suits
    let suits = ["hearts", "spades", "diamonds", "clubs"];

    //List of values
    let values = ["ace", "two", "three"];

    //List of cards
    let mut cards = vec![];

    //Double nested for loop
    for suit in suits.iter() {
        for value in values.iter() {
            let card = format!("{} of {}", value, suit);
            cards.push(card);
        }
    }

    //Create a "deck" variable (or "binding", in Rust)
    //Alternatively, you can use Vec::new()
    //let deck: Deck = Deck { cards: Vec::new() };

    let mut deck: Deck = Deck { cards: vec![] };
    deck.cards = cards;

    println!("Here's your deck: {:?}", deck);
}
