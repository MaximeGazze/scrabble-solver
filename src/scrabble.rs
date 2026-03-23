pub const BOARD_SIZE: usize = 15;

pub enum SpecialTile {
    Empty,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
}

#[rustfmt::skip]
pub const SPECIAL_TILE_BOARD: [[SpecialTile; BOARD_SIZE]; BOARD_SIZE] = {
    use SpecialTile::*;
    [
        [TripleWord, Empty, Empty, DoubleLetter, Empty, Empty, Empty, TripleWord, Empty, Empty, Empty, DoubleLetter, Empty, Empty, TripleWord],
        [Empty, DoubleWord, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, DoubleWord, Empty],
        [Empty, Empty, DoubleWord, Empty, Empty, Empty, DoubleLetter, Empty, DoubleLetter, Empty, Empty, Empty, DoubleWord, Empty, Empty],
        [DoubleLetter, Empty, Empty, DoubleWord, Empty, Empty, Empty, DoubleLetter, Empty, Empty, Empty, DoubleWord, Empty, Empty, DoubleLetter],
        [Empty, Empty, Empty, Empty, DoubleWord, Empty, Empty, Empty, Empty, Empty, DoubleWord, Empty, Empty, Empty, Empty],
        [Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty],
        [Empty, Empty, DoubleLetter, Empty, Empty, Empty, DoubleLetter, Empty, DoubleLetter, Empty, Empty, Empty, DoubleLetter, Empty, Empty],
        [TripleWord, Empty, Empty, DoubleLetter, Empty, Empty, Empty, DoubleWord, Empty, Empty, Empty, DoubleLetter, Empty, Empty, TripleWord],
        [Empty, Empty, DoubleLetter, Empty, Empty, Empty, DoubleLetter, Empty, DoubleLetter, Empty, Empty, Empty, DoubleLetter, Empty, Empty],
        [Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty],
        [Empty, Empty, Empty, Empty, DoubleWord, Empty, Empty, Empty, Empty, Empty, DoubleWord, Empty, Empty, Empty, Empty],
        [DoubleLetter, Empty, Empty, DoubleWord, Empty, Empty, Empty, DoubleLetter, Empty, Empty, Empty, DoubleWord, Empty, Empty, DoubleLetter],
        [Empty, Empty, DoubleWord, Empty, Empty, Empty, DoubleLetter, Empty, DoubleLetter, Empty, Empty, Empty, DoubleWord, Empty, Empty],
        [Empty, DoubleWord, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, DoubleWord, Empty],
        [TripleWord, Empty, Empty, DoubleLetter, Empty, Empty, Empty, TripleWord, Empty, Empty, Empty, DoubleLetter, Empty, Empty, TripleWord],
    ]
};

pub fn score_letter(letter: char) -> u32 {
    match letter {
        'A' | 'E' | 'I' | 'L' | 'N' | 'O' | 'R' | 'S' | 'T' | 'U' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => panic!("invalid letter {}", letter),
    }
}
