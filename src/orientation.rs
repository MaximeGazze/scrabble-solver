use std::ops::Not;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Not for Orientation {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum OrientationValue {
    Horizontal(usize),
    Vertical(usize),
}

impl OrientationValue {
    pub const fn new(orientation: Orientation, value: usize) -> Self {
        match orientation {
            Orientation::Horizontal => Self::Horizontal(value),
            Orientation::Vertical => Self::Vertical(value),
        }
    }
}
