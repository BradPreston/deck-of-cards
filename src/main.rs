use std::vec;

fn main() {
    let deck = create_deck();
    print_deck(deck);
}

fn create_deck() -> Vec<String> {
    let mut deck = vec![];
    let suits = ["Diamonds", "Clubs", "Hearts", "Spades"];
    let faces = [
        "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
    ];
    for suit in suits {
        for face in faces {
            let card = format!("{face} of {suit}");
            deck.push(card);
        }
    }
    deck
}

fn print_deck(deck: Vec<String>) {
    for card in deck {
        println!("{}", card);
    }
}
