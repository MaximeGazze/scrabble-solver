use crate::{Board, Coordinates, CoordinatesIterator};

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Tile {
    pub letter: char,
    pub coordinates: Coordinates,
    pub wildcard: bool,
}

impl Tile {
    pub const fn new(letter: char, coordinates: Coordinates, wildcard: bool) -> Self {
        Self {
            letter,
            coordinates,
            wildcard,
        }
    }
}

pub struct TileIterator<'a> {
    board: &'a Board,
    coordinates: CoordinatesIterator,
}

impl<'a> TileIterator<'a> {
    pub const fn new(board: &'a Board) -> Self {
        Self {
            board,
            coordinates: CoordinatesIterator::new(),
        }
    }
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = &'a Tile;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(coordinates) = self.coordinates.next() {
            if let Some(tile) = self.board.tile_at(coordinates) {
                return Some(tile);
            }
        }

        None
    }
}
