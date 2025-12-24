use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

// Struct example case
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[test]
fn test_rectangle_set_width_height() {
    let mut rectangle = Rectangle::new(2, 5);
    assert_eq!(rectangle.width, 2);
    assert_eq!(rectangle.height, 5);
}

// Always Cap first letter of struct
#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self { // Associated function
        // List of 'suits' - 'hearts', 'spades', etc. (immutable)
        let suits = ["Hearts", "Spades", "Diamonds"];
        // List of 'values'- 'ace', 'two, etc. (immuntable)
        let values = ["Ace", "Two", "Three"];
        let mut cards = vec![];
        
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards } // Same as return Deck { cards } -- implicit return    
    }

    fn shuffle(&mut self) {  // Method; the "&mut self" means the method is called on a mutable instance of the struct
        let mut rng = ThreadRng::default();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> { // Method
        self.cards.split_off(self.cards.len() - num_cards) // Implicit return
    }
}

#[test]
fn test_deck_new() {
    let deck = Deck::new();
    assert_eq!(deck.cards.len(), 9); // 3 suits * 3 values = 9 cards
}

#[test]
fn test_deck_new_cards() {
    let deck = Deck::new();
    assert_eq!(deck.cards, vec!["Ace of Hearts", "Two of Hearts", "Three of Hearts", "Ace of Spades", "Two of Spades", "Three of Spades", "Ace of Diamonds", "Two of Diamonds", "Three of Diamonds"]);
}

#[test]
fn test_deck_new_cards_length() {
    let deck = Deck::new();
    assert_eq!(deck.cards.len(), 9);
}

#[test]
fn test_deck_shuffle() {
    let mut deck = Deck::new();
    deck.shuffle();
    assert_ne!(deck.cards, vec!["Ace of Hearts", "Two of Hearts", "Three of Hearts", "Ace of Spades", "Two of Spades", "Three of Spades", "Ace of Diamonds", "Two of Diamonds", "Three of Diamonds"]);
}

#[test]
fn test_deck_deal() {
    let mut deck = Deck::new();
    deck.shuffle();
    let dealt_cards = deck.deal(3);
    assert_eq!(dealt_cards.len(), 3);
}

fn main() {
    // Setup
    let mut deck = Deck::new(); // We defined "new()" in the impl Deck block

    deck.shuffle();
    // Probably need to add error handling!!
    let cards = deck.deal(3);

    println!("Heres your hand: {:#?}", cards);
}
