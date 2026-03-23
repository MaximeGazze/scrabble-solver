use crate::{Orientation, OrientationValue, BOARD_SIZE};
use std::ops::{Add, Sub};

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coordinates {
    pub i: usize,
    pub j: usize,
}

impl Coordinates {
    /// Constructs a new Coordinates struct from the given i and j values.
    ///
    /// The i value denotates the row index and the j value denotates the column index.
    ///
    /// # Examples
    ///
    /// ```
    /// use scrabble_solver::Coordinates;
    ///
    /// let coordinates = Coordinates::new(4, 5);
    /// ```
    pub const fn new(i: usize, j: usize) -> Self {
        assert!(i < BOARD_SIZE && j < BOARD_SIZE);

        Self { i, j }
    }

    pub fn add_mut(&mut self, value: usize, orientation: Orientation) -> bool {
        match orientation {
            Orientation::Vertical => {
                if self.i + value < BOARD_SIZE {
                    self.i += value;
                    true
                } else {
                    false
                }
            }
            Orientation::Horizontal => {
                if self.j + value < BOARD_SIZE {
                    self.j += value;
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn sub_mut(&mut self, value: usize, orientation: Orientation) -> bool {
        match orientation {
            Orientation::Vertical => {
                if value <= self.i {
                    self.i -= value;
                    true
                } else {
                    false
                }
            }
            Orientation::Horizontal => {
                if value <= self.j {
                    self.j -= value;
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl Add<OrientationValue> for Coordinates {
    type Output = Option<Coordinates>;

    fn add(self, rhs: OrientationValue) -> Self::Output {
        match rhs {
            OrientationValue::Horizontal(value) => {
                if self.j + value >= BOARD_SIZE {
                    None
                } else {
                    Some(Coordinates::new(self.i, self.j + value))
                }
            }
            OrientationValue::Vertical(value) => {
                if self.i + value >= BOARD_SIZE {
                    None
                } else {
                    Some(Coordinates::new(self.i + value, self.j))
                }
            }
        }
    }
}

impl Sub<OrientationValue> for Coordinates {
    type Output = Option<Coordinates>;

    fn sub(self, rhs: OrientationValue) -> Self::Output {
        match rhs {
            OrientationValue::Horizontal(value) => {
                if self.j < value {
                    None
                } else {
                    Some(Coordinates::new(self.i, self.j - value))
                }
            }
            OrientationValue::Vertical(value) => {
                if self.i < value {
                    None
                } else {
                    Some(Coordinates::new(self.i - value, self.j))
                }
            }
        }
    }
}

pub struct CoordinatesIterator {
    coordinates: Coordinates,
}

impl CoordinatesIterator {
    pub const fn new() -> Self {
        Self {
            coordinates: Coordinates::new(0, 0),
        }
    }
}

impl Iterator for CoordinatesIterator {
    type Item = Coordinates;

    fn next(&mut self) -> Option<Self::Item> {
        while self.coordinates.i < BOARD_SIZE {
            let coordinates = self.coordinates;

            if self.coordinates.j < BOARD_SIZE - 1 {
                self.coordinates.j += 1;
            } else {
                self.coordinates.j = 0;
                self.coordinates.i += 1;
            }

            return Some(coordinates);
        }

        None
    }
}
