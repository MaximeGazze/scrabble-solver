use crate::{orientation::Orientation, tile::Tile};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct Play {
    pub word: String,
    pub tiles: Vec<Tile>,
    pub orientation: Orientation,
}

impl Play {
    pub fn len(&self) -> usize {
        self.word.len()
    }

    // fn copy_and_prepend_letter(&self, letter: char, wildcard: bool) -> Option<Self> {
    //     let coordinates = self
    //         .tiles
    //         .first()
    //         .unwrap()
    //         .coordinates
    //         .sub(1, self.orientation);

    //     match coordinates {
    //         None => None,
    //         Some(coordinates) => {
    //             let new_tile = Tile {
    //                 letter,
    //                 coordinates,
    //                 wildcard,
    //             };

    //             let mut new_play = self.clone();

    //             new_play.word.insert(0, letter);
    //             new_play.tiles.insert(0, new_tile);

    //             Some(new_play)
    //         }
    //     }
    // }

    // fn copy_and_append_letter(&self, letter: char, wildcard: bool) -> Option<Self> {
    //     let coordinates = self
    //         .tiles
    //         .last()
    //         .unwrap()
    //         .coordinates
    //         .add(1, self.orientation);

    //     match coordinates {
    //         None => None,
    //         Some(coordinates) => {
    //             let new_tile = Tile {
    //                 letter,
    //                 coordinates,
    //                 wildcard,
    //             };

    //             let mut new_play = self.clone();

    //             new_play.word.push(letter);
    //             new_play.tiles.push(new_tile);

    //             Some(new_play)
    //         }
    //     }
    // }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
            && self.tiles == other.tiles
            && self.orientation == other.orientation
    }
}

impl Eq for Play {}

impl Hash for Play {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.word.hash(state);
        self.tiles.hash(state);
        self.orientation.hash(state);
    }
}
