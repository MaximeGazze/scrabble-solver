use crate::{Coordinates, Hand, Orientation, Tile};

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Play {
    pub word: String,
    pub tiles: Vec<Tile>,
    pub hand: Hand,
    pub orientation: Orientation,
    pub start_coordinates: Coordinates,
    pub end_coordinates: Coordinates,
}

impl Play {
    pub fn len(&self) -> usize {
        self.tiles.len()
    }

    pub fn tile_at(&self, coordinates: Coordinates) -> Option<&Tile> {
        self.tiles
            .iter()
            .find(|tile| tile.coordinates == coordinates)
    }
}
