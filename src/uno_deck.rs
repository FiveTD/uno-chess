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
}

impl fmt::Display for UnoColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnoColor::Red => write!(f, "Red"),
            UnoColor::Yellow => write!(f, "Yellow"),
            UnoColor::Green => write!(f, "Green"),
            UnoColor::Blue => write!(f, "Blue"),
        }
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

impl UnoValue {
    pub fn number(n: u8) -> Option<Self> {
        if n <= 9 { Some(Self::Number(n)) } else { None }
    }
}

impl fmt::Display for UnoValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnoValue::Number(n) => write!(f, "{n}"),
            UnoValue::PlusTwo => write!(f, "+2"),
            UnoValue::PlusFour => write!(f, "Wild +4"),
            UnoValue::Wild => write!(f, "Wild"),
            UnoValue::Skip => write!(f, "Skip"),
            UnoValue::Reverse => write!(f, "Reverse"),
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct UnoCard {
    color: Option<UnoColor>,
    value: UnoValue,
}

const STANDARD_DECK: [UnoCard; 108] = UnoCard::build_deck();

impl UnoCard {
    pub const fn new(color: Option<UnoColor>, value: UnoValue) -> Self {
        Self { color, value }
    }

    /// Array slice of a complete standard Uno deck
    pub fn standard_deck() -> &'static [UnoCard] {
        &STANDARD_DECK
    }

    pub fn can_play_on(&self, other: &UnoCard) -> bool {
        self.value == other.value
            || self.color == other.color
            || self.color == None
            || other.color == None
    }

    const fn build_deck() -> [UnoCard; 108] {
        let mut cards = [UnoCard::new(None, UnoValue::Wild); 108];
        let mut i = 0;

        // Colors
        i = Self::add_color(&mut cards, i, UnoColor::Red);
        i = Self::add_color(&mut cards, i, UnoColor::Yellow);
        i = Self::add_color(&mut cards, i, UnoColor::Green);
        i = Self::add_color(&mut cards, i, UnoColor::Blue);

        // Wilds
        const_for!(_ in 0..4 => {
            cards[i] = UnoCard::new(None, UnoValue::Wild);
            cards[i+4] = UnoCard::new(None, UnoValue::PlusFour);
            i += 1;
        });

        cards
    }

    const fn add_color(cards: &mut [UnoCard; 108], mut i: usize, color: UnoColor) -> usize {
        // One zero
        cards[i] = UnoCard::new(Some(color), UnoValue::Number(0));
        i += 1;

        // Two of 1..=9
        const_for!(n in 1..10 => {
            cards[i] = UnoCard::new(Some(color), UnoValue::Number(n));
            cards[i+1] = UnoCard::new(Some(color), UnoValue::Number(n));
            i += 2;
        });

        // Two skips, reverses, +2s
        const_for!(_ in 0..2 => {
            cards[i] = UnoCard::new(Some(color), UnoValue::Skip);
            cards[i+2] = UnoCard::new(Some(color), UnoValue::Reverse);
            cards[i+4] = UnoCard::new(Some(color), UnoValue::PlusTwo);
            i += 1;
        });

        i + 4
    }

    fn sort_key(&self) -> (bool, Option<UnoColor>, &UnoValue) {
        (self.color.is_none(), self.color, &self.value)
    }
}

impl Ord for UnoCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sort_key().cmp(&other.sort_key())
    }
}

impl PartialOrd for UnoCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for UnoCard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(color) = self.color {
            write!(f, "{} {}", color, self.value)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

/// Shared functionality between hands and deck (sorting, etc.)
pub trait Cards {
    /// Get cards as a slice
    fn cards(&self) -> &[UnoCard];

    /// Get cards as a mut slice
    fn cards_mut(&mut self) -> &mut [UnoCard];

    /// Shuffle the cards using a Knuth shuffle
    fn shuffle(&mut self) {
        let cards = self.cards_mut();
        let mut rng = rand::rng();
        cards.shuffle(&mut rng);
    }

    /// Sort cards
    fn sort(&mut self) {
        self.cards_mut().sort();
    }
}

#[derive(Clone, Debug)]
pub struct UnoDeck {
    deck: Vec<UnoCard>,
    dealt: Vec<UnoCard>,
    discard: Option<UnoCard>,
    active_color: Option<UnoColor>, // None if draw to discard is wild
}

impl UnoDeck {
    pub fn new() -> Self {
        Self::from_cards(UnoCard::standard_deck())
    }

    pub fn from_cards(cards: &[UnoCard]) -> Self {
        Self {
            deck: cards.to_vec(),
            dealt: Vec::with_capacity(cards.len()),
            discard: None,
            active_color: None,
        }
    }

    pub fn peek(&self) -> Option<&UnoCard> {
        self.deck.last()
    }

    /// Draws one card from the top of the deck.
    pub fn draw(&mut self) -> Option<UnoCard> {
        let card = self.deck.pop()?;
        self.dealt.push(card);
        Some(card)
    }

    /// Draws the top card to the discard pile. Returns the drawn card.
    pub fn draw_discard(&mut self) -> Option<UnoCard> {
        self.discard = self.draw();
        self.discard
    }

    /// Draws up to `count` cards.
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
        let mut dealt = 0;
        for _ in 0..count {
            if let Some(card) = self.draw() {
                hand.add_card(card);
                dealt += 1;
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

    fn cards_mut(&mut self) -> &mut [UnoCard] {
        self.deck.as_mut_slice()
    }
}

#[derive(Clone, Default, Debug)]
pub struct Hand {
    cards: Vec<UnoCard>,
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

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn remove(&mut self, index: usize) -> UnoCard {
        self.cards.remove(index)
    }

    /// Returns cards that can be played on an `UnoColor`.
    pub fn matches_color(&self, color: &UnoColor) -> Vec<&UnoCard> {
        self.cards
            .iter()
            .filter(|c| {
                if let Some(card_color) = c.color {
                    card_color == *color
                } else {
                    true
                }
            })
            .collect()
    }

    /// Returns cards that can be played on an `UnoCard`.
    pub fn matches_card(&self, card: &UnoCard) -> Vec<&UnoCard> {
        self.cards.iter().filter(|c| c.can_play_on(card)).collect()
    }
}

impl Cards for Hand {
    fn cards(&self) -> &[UnoCard] {
        self.cards.as_slice()
    }

    fn cards_mut(&mut self) -> &mut [UnoCard] {
        self.cards.as_mut_slice()
    }
}