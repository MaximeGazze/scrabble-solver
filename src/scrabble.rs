use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashSet},
    hash::{Hash, Hasher},
    ops::Not,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coordinates {
    pub i: usize,
    pub j: usize,
}

impl Coordinates {
    pub fn add(&self, value: usize, orientation: Orientation) -> Option<Self> {
        match orientation {
            Orientation::Vertical => {
                if self.i + value >= Board::BOARD_SIZE {
                    return None;
                }

                Some(Self {
                    i: self.i + value,
                    j: self.j,
                })
            }
            Orientation::Horizontal => {
                if self.j + value >= Board::BOARD_SIZE {
                    return None;
                }

                Some(Self {
                    i: self.i,
                    j: self.j + value,
                })
            }
        }
    }

    pub fn sub(&self, value: usize, orientation: Orientation) -> Option<Self> {
        match orientation {
            Orientation::Vertical => {
                if value > self.i {
                    return None;
                }

                Some(Self {
                    i: self.i - value,
                    j: self.j,
                })
            }
            Orientation::Horizontal => {
                if value > self.j {
                    return None;
                }

                Some(Self {
                    i: self.i,
                    j: self.j - value,
                })
            }
        }
    }
}

impl Hash for Coordinates {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.i.hash(state);
        self.j.hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Not for Orientation {
    type Output = Orientation;

    fn not(self) -> Self::Output {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

impl Hash for Orientation {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Orientation::Horizontal => state.write_u8(0),
            Orientation::Vertical => state.write_u8(1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tile {
    pub letter: char,
    pub coordinates: Coordinates,
    pub wildcard: bool,
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.letter.hash(state);
        self.coordinates.hash(state);
        self.wildcard.hash(state);
    }
}

pub struct TileIterator<'a> {
    board: &'a Board,
    coordinates: Coordinates,
}

impl<'a> TileIterator<'a> {
    pub fn from_board(board: &'a Board) -> Self {
        TileIterator {
            board,
            coordinates: Coordinates { i: 0, j: 0 },
        }
    }
}

impl<'a> Iterator for TileIterator<'a> {
    type Item = &'a Tile;

    fn next(&mut self) -> Option<Self::Item> {
        while self.coordinates.i < Board::BOARD_SIZE {
            let current_tile = self.board.tiles[self.coordinates.i][self.coordinates.j].as_ref();

            if self.coordinates.j < Board::BOARD_SIZE - 1 {
                self.coordinates.j += 1;
            } else {
                self.coordinates.j = 0;
                self.coordinates.i += 1;
            }

            if let Some(tile) = current_tile {
                return Some(tile);
            }
        }

        None
    }
}

#[derive(Clone, Debug)]
pub struct BoardWord {
    pub word: String,
    pub tiles: Vec<Tile>,
    pub orientation: Orientation,
}

impl BoardWord {
    pub fn len(&self) -> usize {
        self.word.len()
    }
}

impl PartialEq for BoardWord {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
            && self.tiles == other.tiles
            && self.orientation == other.orientation
    }
}

impl Eq for BoardWord {}

impl Hash for BoardWord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.word.hash(state);
        self.tiles.hash(state);
        self.orientation.hash(state);
    }
}

#[derive(Clone, Debug)]
pub struct Play {
    pub word: String,
    pub tiles: Vec<Tile>,
    pub hand: Vec<char>,
    pub orientation: Orientation,
}

impl Play {
    pub fn len(&self) -> usize {
        self.tiles.len()
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
            && self.tiles == other.tiles
            && self.hand == other.hand
            && self.orientation == other.orientation
    }
}

impl Eq for Play {}

impl Hash for Play {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.word.hash(state);
        self.tiles.hash(state);
        self.hand.hash(state);
        self.orientation.hash(state);
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
        self.word.cmp(&other.word)
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Board {
    pub tiles: Vec<Vec<Option<Tile>>>,
}

impl Board {
    pub const BOARD_SIZE: usize = 15;

    pub fn new() -> Self {
        Self {
            tiles: vec![vec![None; Self::BOARD_SIZE]; Self::BOARD_SIZE],
        }
    }

    pub fn tile_at(&self, coordinates: &Coordinates) -> Option<&Tile> {
        self.tiles.get(coordinates.i)?.get(coordinates.j)?.as_ref()
    }

    fn tile_above(&self, coordinates: &Coordinates) -> Option<&Tile> {
        self.tile_at(&coordinates.sub(1, Orientation::Vertical)?)
    }

    fn tile_below(&self, coordinates: &Coordinates) -> Option<&Tile> {
        self.tile_at(&coordinates.add(1, Orientation::Vertical)?)
    }

    fn tile_left(&self, coordinates: &Coordinates) -> Option<&Tile> {
        self.tile_at(&coordinates.sub(1, Orientation::Horizontal)?)
    }

    fn tile_right(&self, coordinates: &Coordinates) -> Option<&Tile> {
        self.tile_at(&coordinates.add(1, Orientation::Horizontal)?)
    }

    fn tile_before(&self, coordinates: &Coordinates, orientation: Orientation) -> Option<&Tile> {
        self.tile_at(&coordinates.sub(1, orientation)?)
    }

    fn tile_after(&self, coordinates: &Coordinates, orientation: Orientation) -> Option<&Tile> {
        self.tile_at(&coordinates.add(1, orientation)?)
    }

    pub fn board_word_at(
        &self,
        mut coordinates: Coordinates,
        orientation: Orientation,
    ) -> Option<BoardWord> {
        let mut board_word = BoardWord {
            word: String::new(),
            tiles: Vec::new(),
            orientation,
        };

        match orientation {
            Orientation::Vertical => {
                while self.tile_above(&coordinates).is_some() {
                    coordinates.i -= 1;
                }
            }
            Orientation::Horizontal => {
                while self.tile_left(&coordinates).is_some() {
                    coordinates.j -= 1;
                }
            }
        }

        while let Some(current_tile) = self.tile_at(&coordinates) {
            board_word.word.push(current_tile.letter);
            board_word.tiles.push(current_tile.clone());

            match orientation {
                Orientation::Vertical => coordinates.i += 1,
                Orientation::Horizontal => coordinates.j += 1,
            }
        }

        if board_word.len() > 1 {
            Some(board_word)
        } else {
            None
        }
    }

    pub fn tiles(&self) -> TileIterator<'_> {
        TileIterator::from_board(self)
    }

    pub fn board_words(&self) -> Vec<BoardWord> {
        let mut board_words = Vec::new();

        for tile in self.tiles() {
            let top_has_tile = self.tile_above(&tile.coordinates).is_some();
            let bottom_has_tile = self.tile_below(&tile.coordinates).is_some();
            let left_has_tile = self.tile_left(&tile.coordinates).is_some();
            let right_has_tile = self.tile_right(&tile.coordinates).is_some();

            if !top_has_tile && bottom_has_tile {
                board_words.push(
                    self.board_word_at(tile.coordinates, Orientation::Vertical)
                        .expect("a board word should be present"),
                )
            }

            if !left_has_tile && right_has_tile {
                board_words.push(
                    self.board_word_at(tile.coordinates, Orientation::Horizontal)
                        .expect("a board word should be present"),
                )
            }
        }

        board_words
    }

    pub fn score_play(&self, play: &Play) -> u32 {
        // TODO
        0
    }

    pub fn validate_tile(
        &self,
        tile: &Tile,
        orientation: Orientation,
        wordlist: &HashSet<String>,
    ) -> bool {
        let mut new_word = String::new();

        if let Some(tile_before) = self.tile_before(&tile.coordinates, orientation) {
            match self.board_word_at(tile_before.coordinates, orientation) {
                None => new_word.push(tile_before.letter),
                Some(board_word) => new_word.push_str(&board_word.word),
            }
        }

        new_word.push(tile.letter);

        if let Some(tile_after) = self.tile_after(&tile.coordinates, orientation) {
            match self.board_word_at(tile_after.coordinates, orientation) {
                None => new_word.push(tile_after.letter),
                Some(board_word) => new_word.push_str(&board_word.word),
            }
        }

        new_word.len() == 1 || wordlist.contains(&new_word)
    }

    fn build_possible_plays(
        &self,
        play: Play,
        first_tile_coordinates: Coordinates,
        last_tile_coordinates: Coordinates,
        wordlist: &HashSet<String>,
        allow_skewer: bool,
    ) -> Vec<Play> {
        let mut plays = BTreeSet::new();
        // let mut plays = HashSet::new();
        let mut play_stack: Vec<(Play, Coordinates, Coordinates)> = Vec::new();

        play_stack.push((play, first_tile_coordinates, last_tile_coordinates));

        while let Some(play_context) = play_stack.pop() {
            let (current_play, first_tile_coordinates, last_tile_coordinates) = play_context;

            let prepend_out_of_bounds = first_tile_coordinates
                .sub(1, current_play.orientation)
                .is_none();
            let append_out_of_bounds = last_tile_coordinates
                .add(1, current_play.orientation)
                .is_none();

            let prepend_would_skewer = match first_tile_coordinates.sub(2, current_play.orientation)
            {
                None => false,
                Some(coordinates) => self.tile_at(&coordinates).is_some(),
            };

            let append_would_skewer = match last_tile_coordinates.add(2, current_play.orientation) {
                None => false,
                Some(coordinates) => self.tile_at(&coordinates).is_some(),
            };

            let allow_prepend = !prepend_out_of_bounds && (allow_skewer || !prepend_would_skewer);
            let allow_append = !append_out_of_bounds && (allow_skewer || !append_would_skewer);

            if current_play.len() > 0 && wordlist.contains(&current_play.word) {
                plays.insert(current_play.clone());
            }

            let current_hand: Vec<(usize, char, bool)> = current_play
                .hand
                .iter()
                .enumerate()
                .flat_map(|(i, hand_letter)| match hand_letter {
                    '*' => ('A'..='Z').map(|it| (i, it, true)).collect(),
                    letter => vec![(i, *letter, false)],
                })
                .collect();

            for (i, letter, wildcard) in current_hand.into_iter() {
                let mut new_hand = current_play.hand.clone();
                new_hand.remove(i);

                if allow_prepend {
                    if let Some(new_coordinates) =
                        first_tile_coordinates.sub(1, current_play.orientation)
                    {
                        let new_tile = Tile {
                            letter,
                            coordinates: new_coordinates,
                            wildcard,
                        };

                        if self.validate_tile(&new_tile, !current_play.orientation, wordlist) {
                            let mut new_play = Play {
                                word: format!("{}{}", letter, current_play.word),
                                tiles: [vec![new_tile], current_play.tiles.clone()].concat(),
                                hand: new_hand.clone(),
                                orientation: current_play.orientation,
                            };

                            let mut first_tile_coordinates = new_coordinates;
                            while let Some(tile) =
                                self.tile_before(&new_coordinates, current_play.orientation)
                            {
                                new_play.word.insert(0, tile.letter);
                                first_tile_coordinates = tile.coordinates;
                            }

                            play_stack.push((
                                new_play,
                                first_tile_coordinates,
                                last_tile_coordinates,
                            ));
                        }
                    }
                }

                if allow_append {
                    if let Some(new_coordinates) =
                        last_tile_coordinates.add(1, current_play.orientation)
                    {
                        let new_tile = Tile {
                            letter,
                            coordinates: new_coordinates,
                            wildcard,
                        };

                        if self.validate_tile(&new_tile, !current_play.orientation, wordlist) {
                            let mut new_play = Play {
                                word: format!("{}{}", current_play.word, letter),
                                tiles: [current_play.tiles.clone(), vec![new_tile]].concat(),
                                hand: new_hand.clone(),
                                orientation: current_play.orientation,
                            };

                            let mut last_tile_coordinates = new_coordinates;
                            while let Some(tile) =
                                self.tile_after(&new_coordinates, current_play.orientation)
                            {
                                new_play.word.push(tile.letter);
                                last_tile_coordinates = tile.coordinates;
                            }

                            play_stack.push((
                                new_play,
                                first_tile_coordinates,
                                last_tile_coordinates,
                            ));
                        }
                    }
                }
            }
        }

        plays.into_iter().collect()
    }

    pub fn find_extension_plays(
        &self,
        board_word: &BoardWord,
        hand: Vec<char>,
        wordlist: &HashSet<String>,
    ) -> Vec<Play> {
        let play = Play {
            word: board_word.word.clone(),
            tiles: vec![],
            hand,
            orientation: board_word.orientation,
        };

        let first_tile_coordinates = board_word
            .tiles
            .first()
            .expect("board word tiles should not be empty")
            .coordinates;
        let last_tile_coordinates = board_word
            .tiles
            .last()
            .expect("board word tiles should not be empty")
            .coordinates;

        self.build_possible_plays(
            play,
            first_tile_coordinates,
            last_tile_coordinates,
            wordlist,
            true,
        )
    }

    pub fn find_hook_plays(&self, play: &Play, wordlist: &HashSet<String>) -> Vec<Play> {
        assert!(play.len() == 1);

        let orientation = !play.orientation;

        let tile = play.tiles.first().expect("word tiles should not be empty");

        let tile_before = self.tile_before(&tile.coordinates, orientation).is_some();
        let tile_after = self.tile_after(&tile.coordinates, orientation).is_some();

        if tile_before || tile_after {
            return vec![];
        }

        let init_play = Play {
            word: tile.letter.to_string(),
            tiles: play.tiles.clone(),
            hand: play.hand.clone(),
            orientation,
        };

        self.build_possible_plays(
            init_play,
            tile.coordinates,
            tile.coordinates,
            wordlist,
            false,
        )
    }

    pub fn find_perpendicular_plays(
        &self,
        board_word: &BoardWord,
        hand: Vec<char>,
        wordlist: &HashSet<String>,
    ) -> Vec<Play> {
        let mut plays = vec![];

        let orientation = !board_word.orientation;

        for tile in &board_word.tiles {
            let tile_before = self.tile_before(&tile.coordinates, orientation).is_some();
            let tile_after = self.tile_after(&tile.coordinates, orientation).is_some();

            if !tile_before && !tile_after {
                let init_play = Play {
                    word: tile.letter.to_string(),
                    tiles: vec![],
                    hand: hand.clone(),
                    orientation,
                };

                // FIXME add long skewer, this function should not be able to long skewer, only
                // extension_plays
                plays.extend(self.build_possible_plays(
                    init_play,
                    tile.coordinates,
                    tile.coordinates,
                    wordlist,
                    true,
                ));
            }
        }

        plays
    }

    // #[allow(dead_code)] // TODO
    // pub fn find_parallel_plays(
    //     &self,
    //     play: &Play,
    //     hand: &Vec<char>,
    //     wordlist: &HashSet<String>,
    //     one_tile_plays: &Vec<Play>,
    // ) -> Vec<Play> {
    //     Vec::new()
    // }

    pub fn find_possible_plays(&self, wordlist: &HashSet<String>, hand: &Vec<char>) -> Vec<Play> {
        let mut plays: Vec<Play> = Vec::new();

        let current_board_words = self.board_words();

        for current_board_word in current_board_words.iter() {
            let extension_plays =
                self.find_extension_plays(current_board_word, hand.clone(), wordlist);

            plays.extend(extension_plays.clone());

            let hook_plays: Vec<Play> = extension_plays
                .iter()
                .filter(|play| play.len() == 1)
                .flat_map(|play| self.find_hook_plays(play, wordlist))
                .collect();

            plays.extend(hook_plays.clone());

            let perpendicular_plays =
                self.find_perpendicular_plays(current_board_word, hand.clone(), wordlist);

            plays.extend(perpendicular_plays.clone());
        }

        // let one_tile_plays = plays
        //     .iter()
        //     .filter(|play| play.len() == 1)
        //     .cloned()
        //     .collect();

        // for current_board_word in current_board_words.iter() {
        //     let parallel_plays: Vec<Play> =
        //         self.find_parallel_plays(current_board_word, hand, wordlist, &one_tile_plays);
        //
        //     plays.extend(parallel_plays.clone());
        // }

        plays
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..Self::BOARD_SIZE {
            for j in 0..Self::BOARD_SIZE {
                match &self.tiles[i][j] {
                    None => write!(f, "_")?,
                    Some(tile) => write!(f, "{}", tile.letter)?,
                };

                if j < Self::BOARD_SIZE - 1 {
                    write!(f, " ")?;
                }
            }

            if i < Self::BOARD_SIZE - 1 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}
