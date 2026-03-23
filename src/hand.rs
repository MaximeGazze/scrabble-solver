use std::{collections::BTreeMap, error::Error, fmt::Display};

#[derive(Debug)]
pub struct HandLetter {
    pub letter: char,
    pub wildcard: bool,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    letters: BTreeMap<char, u32>,
}

impl Hand {
    pub const fn new() -> Self {
        Self {
            letters: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, letter: char) {
        assert!(letter.is_ascii_uppercase() || letter == '*');

        self.letters
            .entry(letter)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn remove(&mut self, letter: &char) {
        if let Some(count) = self.letters.get_mut(letter) {
            *count -= 1;

            if *count <= 0 {
                self.letters.remove(letter);
            }
        }
    }

    pub fn remove_handletter(&mut self, letter: &HandLetter) {
        let letter_char = if letter.wildcard { '*' } else { letter.letter };

        self.remove(&letter_char)
    }

    pub fn iter(&self) -> HandIterator<'_> {
        HandIterator::new(self)
    }

    pub fn clone_without(&self, letter: &HandLetter) -> Self {
        let mut new_hand = self.clone();
        new_hand.remove_handletter(&letter);
        new_hand
    }
}

impl TryFrom<String> for Hand {
    type Error = HandError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut hand = Hand::new();

        for c in value.to_uppercase().chars() {
            match c {
                'A'..='Z' | '*' => hand.insert(c),
                _ => return Err(HandError::InvalidLetter(c)),
            }
        }

        Ok(hand)
    }
}

impl<const N: usize> TryFrom<[char; N]> for Hand {
    type Error = HandError;

    fn try_from(value: [char; N]) -> Result<Self, Self::Error> {
        let mut hand = Hand::new();

        for c in value {
            match c {
                'A'..='Z' | '*' => hand.insert(c),
                _ => return Err(HandError::InvalidLetter(c)),
            }
        }

        Ok(hand)
    }
}

#[derive(Debug)]
pub enum HandError {
    InvalidLetter(char),
}

impl Display for HandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLetter(c) => write!(f, "invalid letter {}", c),
        }
    }
}

impl Error for HandError {}

pub struct HandIterator<'a> {
    chars: std::collections::btree_map::Iter<'a, char, u32>,
    current_char: char,
    current_char_count: u32,
    wildcard_char: Option<char>,
}

impl<'a> HandIterator<'a> {
    pub fn new(hand: &'a Hand) -> Self {
        HandIterator {
            chars: hand.letters.iter(),
            current_char: ' ',
            current_char_count: 0,
            wildcard_char: None,
        }
    }
}

impl<'a> Iterator for HandIterator<'a> {
    type Item = HandLetter;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(wildcard_char) = self.wildcard_char {
            let next_wildcard_char =
                std::char::from_u32(wildcard_char as u32 + 1).expect("char should be valid");

            if next_wildcard_char >= 'Z' {
                self.wildcard_char = None;
            } else {
                self.wildcard_char = Some(next_wildcard_char);
                return Some(HandLetter {
                    letter: next_wildcard_char,
                    wildcard: true,
                });
            }
        }

        let current_letter = if self.current_char_count > 0 {
            self.current_char_count -= 1;

            self.current_char
        } else {
            match self.chars.next() {
                None => return None,
                Some((letter, count)) => {
                    self.current_char = *letter;
                    self.current_char_count = count - 1;

                    *letter
                }
            }
        };

        match current_letter {
            '*' => {
                self.wildcard_char = Some('A');

                Some(HandLetter {
                    letter: 'A',
                    wildcard: true,
                })
            }
            letter => Some(HandLetter {
                letter,
                wildcard: false,
            }),
        }
    }
}
