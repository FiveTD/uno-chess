use uno_chess::uno_deck::*;

fn main() {
    for c in UnoCard::all_cards() {
        println!("{}", c.to_string());
    }
}
