use const_for::const_for;
use rand::prelude::*;
use std::cmp::Ordering;
use std::fmt;
use std::vec::Vec;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum UnoColor {
    Red,
    Yellow,
    Green,
    Blue,
    Wild,
}

impl fmt::Display for UnoColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum UnoValue {
    Number(u8),
    Skip,
    Reverse,
    PlusTwo,
    Wild,
    PlusFour,
}

impl fmt::Display for UnoValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self {
            UnoValue::Number(n) => n.to_string(),
            UnoValue::PlusTwo => "+2".to_string(),
            UnoValue::PlusFour => "+4".to_string(),
            UnoValue::Wild => "".to_string(),
            _ => format!("{:?}", self)
        })
    }
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
        let mut cards = [UnoCard::new(UnoColor::Wild, UnoValue::Wild); 108];
        let mut i = 0;

        // Colors
        i = Self::add_color(&mut cards, i, UnoColor::Red);
        i = Self::add_color(&mut cards, i, UnoColor::Yellow);
        i = Self::add_color(&mut cards, i, UnoColor::Green);
        i = Self::add_color(&mut cards, i, UnoColor::Blue);

        // Wilds
        const_for!(_ in 0..4 => {
            cards[i] = UnoCard::new(UnoColor::Wild, UnoValue::Wild);
            cards[i+4] = UnoCard::new(UnoColor::Wild, UnoValue::PlusFour);
            i += 1;
        });

        cards
    }

    const fn add_color(cards: &mut [UnoCard; 108], mut i: usize, color: UnoColor) -> usize {
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

        i + 4
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

impl fmt::Display for UnoCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.color.to_string(), self.value.to_string())
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
        let cards = self.mut_cards();
        let l = cards.len();
        let mut rng = rand::rng();
        for n in 0..l {
            let i = rng.random_range(0..l - n);
            cards.swap(i, l - n - 1);
        }
    }

    /// Sort cards
    fn sort(&mut self) {
        self.mut_cards().sort();
    }
}

#[derive(Clone)]
pub struct UnoDeck {
    deck: Vec<UnoCard>,
    dealt: Vec<UnoCard>,
    discard: Option<UnoCard>,
}

impl UnoDeck {
    pub fn new() -> Self {
        Self::from_cards(UnoCard::all_cards())
    }

    pub fn from_cards(cards: &[UnoCard]) -> Self {
        Self {
            deck: cards.to_vec(),
            dealt: Vec::with_capacity(cards.len()),
            discard: None,
        }
    }

    pub fn peek(&self) -> Option<&UnoCard> {
        self.deck.last()
    }

    /// Draws one card from the top of the deck.
    pub fn draw(&mut self) -> Option<UnoCard> {
        self.deck.pop().inspect(|card| {
            self.dealt.push(*card);
        })
    }

    /// Draws the top card to the discard pile. Returns the drawn card.
    pub fn draw_discard(&mut self) -> Option<UnoCard> {
        self.discard = self.draw();
        self.discard
    }

    /// Draws up to `count` cards and returns them as an array.
    pub fn deal(&mut self, count: usize) -> Vec<UnoCard> {
        let mut result = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(card) = self.draw() {
                result.push(card);
            } else {
                break;
            }
        }
        result
    }

    /// Draws up to `count` cards directly to the `Hand`.
    /// Returns the number of cards actually drawn.
    pub fn draw_to_hand(&mut self, hand: &mut Hand, count: usize) -> usize {
        let mut dealt: usize = 0;
        for _ in 0..count {
            if let Some(card) = self.draw() {
                dealt += 1;
                hand.add_card(card);
            } else {
                break;
            }
        }
        dealt
    }

    /// Returns dealt cards back to the end of the undealt pile.
    /// Order is preserved from the last shuffle.
    pub fn reset(&mut self) {
        self.deck.extend(self.dealt.iter().rev());
        self.dealt.clear();
        self.discard = None;
    }

    pub fn deck_count(&self) -> usize {
        self.deck.len()
    }

    pub fn dealt_count(&self) -> usize {
        self.dealt.len()
    }
}

impl Cards for UnoDeck {
    fn cards(&self) -> &[UnoCard] {
        self.deck.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [UnoCard] {
        self.deck.as_mut_slice()
    }
}

#[derive(Clone, Default)]
pub struct Hand {
    pub cards: Vec<UnoCard>,
}

impl Hand {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_cards(cards: &[UnoCard]) -> Self {
        Self {
            cards: cards.to_vec(),
        }
    }

    pub fn add_card(&mut self, card: UnoCard) {
        self.cards.push(card);
    }

    pub fn add_cards(&mut self, other: &[UnoCard]) {
        self.cards.extend_from_slice(other);
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn remove(&mut self, index: usize) -> UnoCard {
        self.cards.remove(index)
    }

    /// Returns cards that can be played on a `Card`.
    pub fn matches_card(&self, card: &UnoCard) -> Vec<UnoCard> {
        self.cards
            .iter()
            .filter(|c| 
                c.value == card.value ||
                c.color == card.color ||
                c.color == UnoColor::Wild ||
                card.color == UnoColor::Wild)
            .cloned()
            .collect()
    }
}

impl Cards for Hand {
    fn cards(&self) -> &[UnoCard] {
        self.cards.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [UnoCard] {
        self.cards.as_mut_slice()
    }
}