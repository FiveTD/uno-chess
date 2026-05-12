use const_for::const_for;
use std::cmp::Ordering;
use std::vec::Vec;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum UnoColor {
    Red,
    Yellow,
    Green,
    Blue,
    Wild,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum UnoValue {
    None,
    Number(u8),
    Skip,
    Reverse,
    PlusTwo,
    PlusFour,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd)]
pub struct UnoCard {
    color: UnoColor,
    value: UnoValue,
}

const ALL_CARDS: [UnoCard; 108] = UnoCard::build_deck();

impl UnoCard {
    pub const fn new(color: UnoColor, value: UnoValue) -> Self {
        Self { color, value }
    }

    /// Array slice of a complete standard Uno deck
    pub fn all_cards() -> &'static [UnoCard] {
        &ALL_CARDS
    }

    const fn build_deck() -> [UnoCard; 108] {
        let mut cards = [UnoCard::new(UnoColor::Wild, UnoValue::None); 108];
        let mut i = 0;

        // Colors
        i = Self::add_color(&mut cards, i, UnoColor::Red);
        i = Self::add_color(&mut cards, i, UnoColor::Yellow);
        i = Self::add_color(&mut cards, i, UnoColor::Green);
        i = Self::add_color(&mut cards, i, UnoColor::Blue);

        // Wilds
        const_for!(_ in 0..4 => {
            cards[i] = UnoCard::new(UnoColor::Wild, UnoValue::None);
            cards[i+4] = UnoCard::new(UnoColor::Wild, UnoValue::PlusFour);
            i += 1;
        });

        cards
    }

    const fn add_color(
        cards: &mut [UnoCard; 108],
        mut i: usize,
        color: UnoColor,
    ) -> usize {
        // One zero
        cards[i] = UnoCard::new(color, UnoValue::Number(0));
        i += 1;

        // Two of 1..=9
        const_for!(n in 1..10 => {
            cards[i] = UnoCard::new(color, UnoValue::Number(n));
            cards[i+1] = UnoCard::new(color, UnoValue::Number(n));
            i += 2;
        });

        // Two skips, reverses, +2s
        const_for!(_ in 0..2 => {
            cards[i] = UnoCard::new(color, UnoValue::Skip);
            cards[i+2] = UnoCard::new(color, UnoValue::Reverse);
            cards[i+4] = UnoCard::new(color, UnoValue::PlusTwo);
            i += 1;
        });

        i
    }
}

impl Ord for UnoCard {
    /// Sort by color, then value
    fn cmp(&self, other: &UnoCard) -> Ordering {
        let result = self.color.cmp(&other.color);
        if result == Ordering::Equal {
            self.value.cmp(&other.value)
        } else {
            result
        }
    }
}

/// Shared functionality between hands and deck (sorting, etc.)
pub trait Cards {
    /// Get cards as a slice
    fn cards(&self) -> &[UnoCard];

    /// Get cards as a mut slice
    fn mut_cards(&mut self) -> &mut [UnoCard];

    /// Shuffle the cards using a Knuth shuffle
    fn shuffle(&mut self) {
        todo!("Shuffle")
    }

    /// Sort cards
    fn sort(&mut self) {
        todo!("Sort")
    }
}

#[derive(Clone)]
pub struct UnoDeck {
    deck: Vec<UnoCard>,
    discard: Vec<UnoCard>,
}

impl UnoDeck {
    pub fn new() -> Self {
        Self::from_cards(UnoCard::all_cards())
    }

    pub fn from_cards(cards: &[UnoCard]) -> Self {
        Self {
            deck: cards.to_vec(),
            discard: Vec::with_capacity(cards.len())
        }
    }
}

pub struct Hand {

}