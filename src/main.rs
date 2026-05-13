use uno_chess::uno_deck::*;

fn main() {
    for c in UnoCard::standard_deck() {
        println!("{}", c);
    }
}
