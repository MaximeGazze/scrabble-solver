use std::{
    collections::HashSet,
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
pub struct Play {
    pub word: String,
    pub tiles: Vec<Tile>,
    pub orientation: Orientation,
}

impl Play {
    pub fn len(&self) -> usize {
        self.word.len()
    }
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

    pub fn play_at(&self, mut coordinates: Coordinates, orientation: Orientation) -> Option<Play> {
        let mut play = Play {
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
            play.word.push(current_tile.letter);
            play.tiles.push(current_tile.clone());

            match orientation {
                Orientation::Vertical => coordinates.i += 1,
                Orientation::Horizontal => coordinates.j += 1,
            }
        }

        if play.len() > 1 {
            Some(play)
        } else {
            None
        }
    }

    pub fn tiles(&self) -> TileIterator<'_> {
        TileIterator::from_board(self)
    }

    pub fn plays(&self) -> Vec<Play> {
        let mut plays = Vec::new();

        for tile in self.tiles() {
            let top_has_tile = self.tile_above(&tile.coordinates).is_some();
            let bottom_has_tile = self.tile_below(&tile.coordinates).is_some();
            let left_has_tile = self.tile_left(&tile.coordinates).is_some();
            let right_has_tile = self.tile_right(&tile.coordinates).is_some();

            if !top_has_tile && bottom_has_tile {
                plays.push(
                    self.play_at(tile.coordinates, Orientation::Vertical)
                        .expect("a play should be present"),
                )
            }

            if !left_has_tile && right_has_tile {
                plays.push(
                    self.play_at(tile.coordinates, Orientation::Horizontal)
                        .expect("a play should be present"),
                )
            }
        }

        plays
    }

    pub fn score_play(&self, play: &Play) -> u32 {
        // TODO
        0
    }

    fn validate_tile(
        &self,
        tile: &Tile,
        orientation: Orientation,
        wordlist: &HashSet<String>,
    ) -> bool {
        let word_before = self
            .tile_before(&tile.coordinates, orientation)
            .and_then(|tile_before| self.play_at(tile_before.coordinates, orientation))
            .map(|play| play.word);

        let word_after = self
            .tile_after(&tile.coordinates, orientation)
            .and_then(|tile_after| self.play_at(tile_after.coordinates, orientation))
            .map(|play| play.word);

        println!("{:?} : {:?}", word_before, word_after);

        let mut new_word = String::new();

        if let Some(ref word) = word_before {
            new_word += &word;
        }

        new_word.push(tile.letter);

        if let Some(ref word) = word_after {
            new_word += &word;
        }

        (word_before.is_none() && word_after.is_none()) || wordlist.contains(&new_word)
    }

    fn build_possible_plays(
        &self,
        play: &Play,
        wordlist: &HashSet<String>,
        hand: &Vec<char>,
        allow_skewer: bool,
        include_init_tiles: bool,
    ) -> Vec<Play> {
        let mut plays = HashSet::new();
        let mut play_stack: Vec<(Play, Vec<(char, bool)>, Coordinates, Coordinates)> = Vec::new();

        let init_play = if include_init_tiles {
            play.clone()
        } else {
            Play {
                word: play.word.clone(),
                tiles: Vec::new(),
                orientation: play.orientation,
            }
        };

        let init_hand: Vec<(char, bool)> = hand
            .into_iter()
            .flat_map(|hand_letter| match hand_letter {
                '*' => ('A'..='Z').map(|it| (it, true)).collect(),
                default => vec![(*default, false)],
            })
            .collect();

        let first_tile_coordinates = play
            .tiles
            .first()
            .expect("word tiles should not be empty")
            .coordinates;
        let last_tile_coordinates = play
            .tiles
            .last()
            .expect("word tiles should not be empty")
            .coordinates;

        play_stack.push((
            init_play,
            init_hand,
            first_tile_coordinates,
            last_tile_coordinates,
        ));

        while let Some(play_context) = play_stack.pop() {
            let (current_play, current_hand, first_tile_coordinates, last_tile_coordinates) =
                play_context;

            let prepend_out_of_bounds = first_tile_coordinates.sub(1, play.orientation).is_none();
            let append_out_of_bounds = last_tile_coordinates.add(1, play.orientation).is_none();

            let prepend_would_skewer = match first_tile_coordinates.sub(2, play.orientation) {
                None => false,
                Some(coordinates) => self.tile_at(&coordinates).is_some(),
            };

            let append_would_skewer = match last_tile_coordinates.add(2, play.orientation) {
                None => false,
                Some(coordinates) => self.tile_at(&coordinates).is_some(),
            };

            let allow_prepend = !prepend_out_of_bounds && (allow_skewer || !prepend_would_skewer);
            let allow_append = !append_out_of_bounds && (allow_skewer || !append_would_skewer);

            if wordlist.contains(&current_play.word) {
                plays.insert(current_play.clone());
            }

            for (i, (letter, wildcard)) in current_hand.iter().enumerate() {
                let mut new_hand = current_hand.clone();
                new_hand.remove(i);

                if allow_prepend {
                    if let Some(new_coordinates) = first_tile_coordinates.sub(1, play.orientation) {
                        let mut new_play = current_play.clone();

                        let new_tile = Tile {
                            letter: *letter,
                            coordinates: new_coordinates,
                            wildcard: *wildcard,
                        };

                        if self.validate_tile(&new_tile, !play.orientation, wordlist) {
                            new_play.word.insert(0, *letter);
                            new_play.tiles.insert(0, new_tile);

                            let mut first_tile_coordinates = new_coordinates;
                            while let Some(tile) =
                                self.tile_before(&new_coordinates, play.orientation)
                            {
                                new_play.word.insert(0, tile.letter);
                                first_tile_coordinates = tile.coordinates;
                            }

                            play_stack.push((
                                new_play,
                                new_hand.clone(),
                                first_tile_coordinates,
                                last_tile_coordinates,
                            ));
                        }
                    }
                }

                if allow_append {
                    if let Some(new_coordinates) = last_tile_coordinates.add(1, play.orientation) {
                        let mut new_play = current_play.clone();

                        let new_tile = Tile {
                            letter: *letter,
                            coordinates: new_coordinates,
                            wildcard: *wildcard,
                        };

                        if self.validate_tile(&new_tile, !play.orientation, wordlist) {
                            new_play.word.push(*letter);
                            new_play.tiles.push(new_tile);

                            let mut last_tile_coordinates = new_coordinates;
                            while let Some(tile) =
                                self.tile_after(&new_coordinates, play.orientation)
                            {
                                new_play.word.push(tile.letter);
                                last_tile_coordinates = tile.coordinates;
                            }

                            play_stack.push((
                                new_play,
                                new_hand.clone(),
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
        play: &Play,
        wordlist: &HashSet<String>,
        hand: &Vec<char>,
    ) -> Vec<Play> {
        self.build_possible_plays(play, wordlist, hand, true, false)
    }

    pub fn find_hook_plays(
        &self,
        play: &Play,
        wordlist: &HashSet<String>,
        hand: &Vec<char>,
    ) -> Vec<Play> {
        let mut plays = vec![];

        let first_tile = play.tiles.first().expect("word tiles should not be empty");
        let last_tile = play.tiles.last().expect("word tiles should not be empty");

        let hook_before_coordinates = first_tile.coordinates.sub(1, play.orientation);
        let hook_after_coordinates = last_tile.coordinates.add(1, play.orientation);

        let init_hand: Vec<(usize, char, bool)> = hand
            .into_iter()
            .enumerate()
            .flat_map(|(i, hand_letter)| match hand_letter {
                '*' => ('A'..='Z').map(|it| (i, it, true)).collect(),
                default => vec![(i, *default, false)],
            })
            .collect();

        for (i, letter, wildcard) in init_hand {
            let mut hand_copy = hand.clone();
            hand_copy.remove(i);

            if let Some(coordinates) = hook_before_coordinates {
                let init_play = Play {
                    word: String::from(letter),
                    tiles: vec![Tile {
                        letter,
                        coordinates,
                        wildcard,
                    }],
                    orientation: !play.orientation,
                };

                plays.extend(
                    self.build_possible_plays(&init_play, wordlist, &hand_copy, false, true),
                );
            }

            if let Some(coordinates) = hook_after_coordinates {
                let init_play = Play {
                    word: String::from(letter),
                    tiles: vec![Tile {
                        letter,
                        coordinates,
                        wildcard,
                    }],
                    orientation: !play.orientation,
                };

                plays.extend(
                    self.build_possible_plays(&init_play, wordlist, &hand_copy, false, true),
                );
            }
        }

        plays
    }

    #[allow(dead_code)] // TODO
    pub fn find_perpendicular_plays(
        &self,
        play: &Play,
        wordlist: &HashSet<String>,
        hand: &Vec<char>,
    ) -> Vec<Play> {
        Vec::new()
    }

    #[allow(dead_code)] // TODO
    pub fn find_parallel_plays(
        &self,
        play: &Play,
        wordlist: &HashSet<String>,
        hand: &Vec<char>,
        one_tile_plays: &Vec<Play>,
    ) -> Vec<Play> {
        Vec::new()
    }

    pub fn find_possible_plays(&self, wordlist: &HashSet<String>, hand: &Vec<char>) -> Vec<Play> {
        let mut plays: Vec<Play> = Vec::new();

        let current_plays = self.plays();

        for current_play in current_plays.iter() {
            let extension_plays = self.find_extension_plays(current_play, wordlist, hand);

            plays.extend(extension_plays.clone());

            let hook_plays: Vec<Play> = extension_plays
                .iter()
                .filter(|play| play.len() == 1)
                .flat_map(|play| self.find_hook_plays(play, wordlist, hand))
                .collect();

            plays.extend(hook_plays.clone());

            let perpendicular_plays = self.find_perpendicular_plays(current_play, wordlist, hand);

            plays.extend(perpendicular_plays.clone());
        }

        let one_tile_plays = plays
            .iter()
            .filter(|play| play.len() == 1)
            .cloned()
            .collect();

        for current_play in current_plays.iter() {
            let parallel_plays: Vec<Play> =
                self.find_parallel_plays(current_play, wordlist, hand, &one_tile_plays);

            plays.extend(parallel_plays.clone());
        }

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
