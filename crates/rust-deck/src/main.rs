use rust_deck::cards::Deck;

fn main() {
    // Setup
    let mut deck = Deck::new(); // We defined "new()" in the impl Deck block

    deck.shuffle();
    // Probably need to add error handling!!
    let cards = deck.deal(3);

    println!("Heres your hand: {:#?}", cards);
}
