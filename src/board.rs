use crate::{
    score_letter, Coordinates, CoordinatesIterator, Hand, Orientation, OrientationValue, Play,
    SpecialTile, Tile, TileIterator, BOARD_SIZE, SPECIAL_TILE_BOARD,
};
use colored::Colorize;
use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, Sub},
};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BoardWord {
    word: String,
    tiles: Vec<Tile>,
    orientation: Orientation,
}

impl BoardWord {
    pub const fn new(word: String, tiles: Vec<Tile>, orientation: Orientation) -> Self {
        Self {
            word,
            tiles,
            orientation,
        }
    }

    pub fn len(&self) -> usize {
        self.word.len()
    }
}

pub struct Board {
    pub tiles: Vec<Vec<Option<Tile>>>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: vec![vec![None; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    pub fn insert_tile(&mut self, tile: Tile) {
        let i = tile.coordinates.i;
        let j = tile.coordinates.j;

        self.tiles[i][j] = Some(tile);
    }

    pub fn tile_at(&self, coordinates: Coordinates) -> Option<&Tile> {
        self.tiles.get(coordinates.i)?.get(coordinates.j)?.as_ref()
    }

    fn tile_above(&self, coordinates: Coordinates) -> Option<&Tile> {
        self.tile_at((coordinates - OrientationValue::Vertical(1))?)
    }

    fn tile_below(&self, coordinates: Coordinates) -> Option<&Tile> {
        self.tile_at((coordinates + OrientationValue::Vertical(1))?)
    }

    fn tile_left(&self, coordinates: Coordinates) -> Option<&Tile> {
        self.tile_at((coordinates - OrientationValue::Horizontal(1))?)
    }

    fn tile_right(&self, coordinates: Coordinates) -> Option<&Tile> {
        self.tile_at((coordinates + OrientationValue::Horizontal(1))?)
    }

    fn tile_before(&self, coordinates: Coordinates, orientation: Orientation) -> Option<&Tile> {
        self.tile_at((coordinates - OrientationValue::new(orientation, 1))?)
    }

    fn tile_after(&self, coordinates: Coordinates, orientation: Orientation) -> Option<&Tile> {
        self.tile_at((coordinates + OrientationValue::new(orientation, 1))?)
    }

    fn board_word_at(
        &self,
        mut coordinates: Coordinates,
        orientation: Orientation,
    ) -> Option<BoardWord> {
        let mut board_word = BoardWord::new(String::new(), Vec::new(), orientation);

        while self.tile_before(coordinates, orientation).is_some() {
            coordinates.sub_mut(1, orientation);
        }

        while let Some(current_tile) = self.tile_at(coordinates) {
            board_word.word.push(current_tile.letter);
            board_word.tiles.push(*current_tile);

            if !coordinates.add_mut(1, orientation) {
                break;
            }
        }

        if board_word.len() > 1 {
            Some(board_word)
        } else {
            None
        }
    }

    pub fn tiles(&self) -> TileIterator<'_> {
        TileIterator::new(self)
    }

    fn board_words(&self) -> Vec<BoardWord> {
        let mut board_words = Vec::new();

        for tile in self.tiles() {
            let top_has_tile = self.tile_above(tile.coordinates).is_some();
            let bottom_has_tile = self.tile_below(tile.coordinates).is_some();
            let left_has_tile = self.tile_left(tile.coordinates).is_some();
            let right_has_tile = self.tile_right(tile.coordinates).is_some();

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
        self.score_play_with_crosswords_check(play, play.start_coordinates, play.orientation, true)
    }

    fn score_play_with_crosswords_check(
        &self,
        play: &Play,
        mut coordinates: Coordinates,
        orientation: Orientation,
        check_for_crosswords: bool,
    ) -> u32 {
        let mut score = 0;
        let mut main_word_score = 0;
        let mut main_word_multiplier = 1;

        while coordinates
            .sub(OrientationValue::new(orientation, 1))
            .map_or(false, |new_coordinates| {
                self.tile_at(new_coordinates).is_some() || play.tile_at(new_coordinates).is_some()
            })
        {
            coordinates.sub_mut(1, orientation);
        }

        loop {
            let board_tile = self.tile_at(coordinates);
            let play_tile = play.tile_at(coordinates);

            assert!(board_tile.is_none() || play_tile.is_none());

            if board_tile.is_none() && play_tile.is_none() {
                break;
            }

            if let Some(tile) = board_tile {
                if !tile.wildcard {
                    main_word_score += score_letter(tile.letter);
                }
            }

            if let Some(tile) = play_tile {
                let mut letter_multiplier = 1;

                match SPECIAL_TILE_BOARD[coordinates.i][coordinates.j] {
                    SpecialTile::Empty => {}
                    SpecialTile::DoubleLetter => letter_multiplier = 2,
                    SpecialTile::TripleLetter => letter_multiplier = 3,
                    SpecialTile::DoubleWord => main_word_multiplier = 2,
                    SpecialTile::TripleWord => main_word_multiplier = 3,
                }

                if !tile.wildcard {
                    main_word_score += letter_multiplier * score_letter(tile.letter);
                }
            }

            if play_tile.is_some()
                && check_for_crosswords
                && (self.tile_before(coordinates, !orientation).is_some()
                    || self.tile_after(coordinates, !orientation).is_some())
            {
                score +=
                    self.score_play_with_crosswords_check(play, coordinates, !orientation, false);
            }

            if !coordinates.add_mut(1, orientation) {
                break;
            }
        }

        score += main_word_multiplier * main_word_score;

        if play.len() == 7 {
            score += 50;
        }

        score
    }

    fn validate_tile(
        &self,
        tile: &Tile,
        orientation: Orientation,
        wordlist: &HashSet<String>,
    ) -> bool {
        let mut new_word = String::new();

        if let Some(tile_before) = self.tile_before(tile.coordinates, orientation) {
            match self.board_word_at(tile_before.coordinates, orientation) {
                None => new_word.push(tile_before.letter),
                Some(board_word) => new_word.push_str(&board_word.word),
            }
        }

        new_word.push(tile.letter);

        if let Some(tile_after) = self.tile_after(tile.coordinates, orientation) {
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
        wordlist: &HashSet<String>,
        allow_skewer: bool,
        allow_long_skewer: bool,
    ) -> HashSet<Play> {
        let mut plays = HashSet::new();
        let mut play_stack = vec![play];

        while let Some(current_play) = play_stack.pop() {
            let orientation = current_play.orientation;

            let prepend_out_of_bounds = current_play
                .start_coordinates
                .sub(OrientationValue::new(orientation, 1))
                .is_none();
            let append_out_of_bounds = current_play
                .end_coordinates
                .add(OrientationValue::new(orientation, 1))
                .is_none();

            let prepend_would_skewer = current_play
                .start_coordinates
                .sub(OrientationValue::new(orientation, 2))
                .map_or(false, |coordinates| self.tile_at(coordinates).is_some());
            let append_would_skewer = current_play
                .end_coordinates
                .add(OrientationValue::new(orientation, 2))
                .map_or(false, |coordinates| self.tile_at(coordinates).is_some());

            let prepend_would_long_skewer = prepend_would_skewer
                && current_play
                    .start_coordinates
                    .sub(OrientationValue::new(orientation, 3))
                    .map_or(false, |coordinates| self.tile_at(coordinates).is_some());
            let append_would_long_skewer = append_would_skewer
                && current_play
                    .end_coordinates
                    .add(OrientationValue::new(orientation, 3))
                    .map_or(false, |coordinates| self.tile_at(coordinates).is_some());

            let allow_prepend = !prepend_out_of_bounds
                && (allow_skewer || !prepend_would_skewer)
                && (allow_long_skewer || !prepend_would_long_skewer);
            let allow_append = !append_out_of_bounds
                && (allow_skewer || !append_would_skewer)
                && (allow_long_skewer || !append_would_long_skewer);

            if allow_prepend || allow_append {
                for hand_letter in current_play.hand.iter() {
                    let letter = hand_letter.letter;
                    let wildcard = hand_letter.wildcard;

                    if allow_prepend {
                        if let Some(new_coordinates) = current_play
                            .start_coordinates
                            .sub(OrientationValue::new(orientation, 1))
                        {
                            let new_tile = Tile::new(letter, new_coordinates, wildcard);

                            if self.validate_tile(&new_tile, !orientation, wordlist) {
                                let new_hand = current_play.hand.clone_without(&hand_letter);

                                let mut new_play = Play {
                                    word: format!("{}{}", letter, current_play.word),
                                    tiles: [vec![new_tile], current_play.tiles.clone()].concat(),
                                    hand: new_hand,
                                    orientation,
                                    start_coordinates: new_coordinates,
                                    end_coordinates: current_play.end_coordinates,
                                };

                                while let Some(tile) =
                                    self.tile_before(new_play.start_coordinates, orientation)
                                {
                                    new_play.word.insert(0, tile.letter);
                                    new_play.start_coordinates.sub_mut(1, orientation);
                                }

                                play_stack.push(new_play);
                            }
                        }
                    }

                    if allow_append {
                        if let Some(new_coordinates) = current_play
                            .end_coordinates
                            .add(OrientationValue::new(orientation, 1))
                        {
                            let new_tile = Tile::new(letter, new_coordinates, wildcard);

                            if self.validate_tile(&new_tile, !orientation, wordlist) {
                                let new_hand = current_play.hand.clone_without(&hand_letter);

                                let mut new_play = Play {
                                    word: format!("{}{}", current_play.word, letter),
                                    tiles: [current_play.tiles.clone(), vec![new_tile]].concat(),
                                    hand: new_hand,
                                    orientation,
                                    start_coordinates: current_play.start_coordinates,
                                    end_coordinates: new_coordinates,
                                };

                                while let Some(tile) =
                                    self.tile_after(new_play.end_coordinates, orientation)
                                {
                                    new_play.word.push(tile.letter);
                                    new_play.end_coordinates.add_mut(1, orientation);
                                }

                                play_stack.push(new_play);
                            }
                        }
                    }
                }

                if current_play.len() > 0 && wordlist.contains(&current_play.word) {
                    plays.insert(current_play);
                }
            }
        }

        plays
    }

    fn find_starting_plays(&self, hand: Hand, wordlist: &HashSet<String>) -> HashSet<Play> {
        let mut plays = HashSet::new();

        let center_coordinates = Coordinates::new(7, 7);

        for hand_letter in hand.iter() {
            let letter = hand_letter.letter;
            let wildcard = hand_letter.wildcard;

            let hand_clone = hand.clone_without(&hand_letter);

            let play = Play {
                word: letter.to_string(),
                tiles: vec![Tile::new(letter, center_coordinates, wildcard)],
                hand: hand_clone,
                orientation: Orientation::Horizontal,
                start_coordinates: center_coordinates,
                end_coordinates: center_coordinates,
            };

            plays.extend(self.build_possible_plays(play, wordlist, false, false));
        }

        plays
    }

    fn find_extension_plays(
        &self,
        board_word: &BoardWord,
        hand: Hand,
        wordlist: &HashSet<String>,
    ) -> HashSet<Play> {
        let start_coordinates = board_word
            .tiles
            .first()
            .expect("board word tiles should not be empty")
            .coordinates;
        let end_coordinates = board_word
            .tiles
            .last()
            .expect("board word tiles should not be empty")
            .coordinates;

        let play = Play {
            word: board_word.word.clone(),
            tiles: vec![],
            hand,
            orientation: board_word.orientation,
            start_coordinates,
            end_coordinates,
        };

        self.build_possible_plays(play, wordlist, true, true)
    }

    fn find_hook_plays(&self, play: &Play, wordlist: &HashSet<String>) -> HashSet<Play> {
        assert!(play.len() == 1);

        let tile = play.tiles.first().expect("word tiles should not be empty");
        let orientation = !play.orientation;

        let tile_before = self.tile_before(tile.coordinates, orientation).is_some();
        let tile_after = self.tile_after(tile.coordinates, orientation).is_some();

        if tile_before || tile_after {
            return HashSet::new();
        }

        let init_play = Play {
            word: tile.letter.to_string(),
            tiles: play.tiles.clone(),
            hand: play.hand.clone(),
            orientation,
            start_coordinates: tile.coordinates,
            end_coordinates: tile.coordinates,
        };

        self.build_possible_plays(init_play, wordlist, false, false)
    }

    fn find_perpendicular_plays(
        &self,
        board_word: &BoardWord,
        hand: &Hand,
        wordlist: &HashSet<String>,
    ) -> HashSet<Play> {
        let mut plays = HashSet::new();
        let orientation = !board_word.orientation;

        for tile in &board_word.tiles {
            let tile_before = self.tile_before(tile.coordinates, orientation).is_some();
            let tile_after = self.tile_after(tile.coordinates, orientation).is_some();

            if !tile_before && !tile_after {
                let init_play = Play {
                    word: tile.letter.to_string(),
                    tiles: vec![],
                    hand: hand.clone(),
                    orientation,
                    start_coordinates: tile.coordinates,
                    end_coordinates: tile.coordinates,
                };

                plays.extend(self.build_possible_plays(init_play, wordlist, true, false));
            }
        }

        plays
    }

    fn find_parallel_plays(&self, play: &Play, wordlist: &HashSet<String>) -> HashSet<Play> {
        assert!(play.len() == 1);

        let tile = play.tiles.first().expect("word tiles should not be empty");
        let orientation = !play.orientation;

        let tile_before = self.tile_before(tile.coordinates, orientation).is_some();
        let tile_after = self.tile_after(tile.coordinates, orientation).is_some();

        if tile_before || tile_after {
            return HashSet::new();
        }

        let init_play = Play {
            word: tile.letter.to_string(),
            tiles: play.tiles.clone(),
            hand: play.hand.clone(),
            orientation,
            start_coordinates: tile.coordinates,
            end_coordinates: tile.coordinates,
        };

        self.build_possible_plays(init_play, wordlist, false, false)
    }

    pub fn find_possible_plays(&self, wordlist: &HashSet<String>, hand: Hand) -> HashSet<Play> {
        let mut plays: HashSet<Play> = HashSet::new();
        let current_board_words = self.board_words();

        if current_board_words.is_empty() {
            self.find_starting_plays(hand, wordlist)
        } else {
            for current_board_word in current_board_words.iter() {
                let extension_plays =
                    self.find_extension_plays(current_board_word, hand.clone(), wordlist);

                plays.extend(extension_plays.clone());

                let hook_plays = extension_plays
                    .iter()
                    .filter(|play| play.len() == 1)
                    .flat_map(|play| self.find_hook_plays(play, wordlist));

                plays.extend(hook_plays);

                let perpendicular_plays =
                    self.find_perpendicular_plays(current_board_word, &hand, wordlist);

                plays.extend(perpendicular_plays.clone());

                let parallel_plays = perpendicular_plays
                    .iter()
                    .filter(|play| play.len() == 1)
                    .flat_map(|play| self.find_parallel_plays(play, wordlist));

                plays.extend(parallel_plays);
            }

            plays
        }
    }

    pub fn find_best_play(&self, wordlist: &HashSet<String>, hand: Hand) -> Option<(u32, Play)> {
        let mut best_play: Option<Play> = None;
        let mut best_play_score = 0;

        let current_board_words = self.board_words();

        if current_board_words.is_empty() {
            let starting_plays = self.find_starting_plays(hand, wordlist);

            for play in starting_plays {
                let score = self.score_play(&play);
                if score > best_play_score {
                    best_play = Some(play);
                    best_play_score = score;
                }
            }
        } else {
            for current_board_word in current_board_words.iter() {
                let extension_plays =
                    self.find_extension_plays(current_board_word, hand.clone(), wordlist);

                for play in extension_plays.iter() {
                    let score = self.score_play(&play);
                    if score > best_play_score {
                        best_play = Some(play.clone());
                        best_play_score = score;
                    }
                }

                let hook_plays = extension_plays
                    .iter()
                    .filter(|play| play.len() == 1)
                    .flat_map(|play| self.find_hook_plays(play, wordlist));

                for play in hook_plays {
                    let score = self.score_play(&play);
                    if score > best_play_score {
                        best_play = Some(play);
                        best_play_score = score;
                    }
                }

                let perpendicular_plays =
                    self.find_perpendicular_plays(current_board_word, &hand, wordlist);

                for play in perpendicular_plays.iter() {
                    let score = self.score_play(&play);
                    if score > best_play_score {
                        best_play = Some(play.clone());
                        best_play_score = score;
                    }
                }

                let parallel_plays = perpendicular_plays
                    .iter()
                    .filter(|play| play.len() == 1)
                    .flat_map(|play| self.find_parallel_plays(play, wordlist));

                for play in parallel_plays {
                    let score = self.score_play(&play);
                    if score > best_play_score {
                        best_play = Some(play);
                        best_play_score = score;
                    }
                }
            }
        }

        if let Some(play) = best_play {
            Some((best_play_score, play))
        } else {
            None
        }
    }

    pub fn print_play(&self, play: &Play) {
        let first_tile_coordinates = play
            .tiles
            .first()
            .expect("play tiles should not be empty")
            .coordinates;
        let last_tile_coordinates = play
            .tiles
            .last()
            .expect("play tiles should not be empty")
            .coordinates;

        for coordinates in CoordinatesIterator::new() {
            if let Some(tile) = play.tile_at(coordinates) {
                if tile.wildcard {
                    print!("{}", tile.letter.to_string().white().on_red());
                } else {
                    print!("{}", tile.letter.to_string().red());
                }
            } else if let Some(tile) = self.tile_at(coordinates) {
                let mut is_connected = false;
                if coordinates.i >= first_tile_coordinates.i
                    && coordinates.i <= last_tile_coordinates.i
                {
                    if coordinates.j < first_tile_coordinates.j {
                        let mut current_coordinates = coordinates;
                        while current_coordinates.add_mut(1, Orientation::Horizontal) {
                            if play.tile_at(current_coordinates).is_some() {
                                is_connected = true;
                                break;
                            } else if self.tile_at(current_coordinates).is_none() {
                                break;
                            }
                        }
                    } else {
                        let mut current_coordinates = coordinates;
                        while current_coordinates.sub_mut(1, !play.orientation) {
                            if play.tile_at(current_coordinates).is_some() {
                                is_connected = true;
                                break;
                            } else if self.tile_at(current_coordinates).is_none() {
                                break;
                            }
                        }
                    }
                }

                if coordinates.j >= first_tile_coordinates.j
                    && coordinates.j <= last_tile_coordinates.j
                {
                    if coordinates.i < first_tile_coordinates.i {
                        let mut current_coordinates = coordinates;
                        while current_coordinates.add_mut(1, Orientation::Vertical) {
                            if play.tile_at(current_coordinates).is_some() {
                                is_connected = true;
                                break;
                            } else if self.tile_at(current_coordinates).is_none() {
                                break;
                            }
                        }
                    } else {
                        let mut current_coordinates = coordinates;
                        while current_coordinates.sub_mut(1, !play.orientation) {
                            if play.tile_at(current_coordinates).is_some() {
                                is_connected = true;
                                break;
                            } else if self.tile_at(current_coordinates).is_none() {
                                break;
                            }
                        }
                    }
                };

                if is_connected {
                    if tile.wildcard {
                        print!("{}", tile.letter.to_string().black().on_yellow());
                    } else {
                        print!("{}", tile.letter.to_string().yellow());
                    }
                } else {
                    if tile.wildcard {
                        print!("{}", tile.letter.to_string().black().on_white());
                    } else {
                        print!("{}", tile.letter);
                    }
                }
            } else {
                print!("_");
            }

            if coordinates.j < BOARD_SIZE - 1 {
                print!(" ");
            }

            if coordinates.j == BOARD_SIZE - 1 {
                print!("\n");
            }
        }
    }
}

impl TryFrom<String> for Board {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(&value[..])
    }
}

impl TryFrom<&str> for Board {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut board = Board::new();

        let uppercase = value.to_uppercase();
        let mut chars = uppercase.chars();
        let mut coordinates_iterator = CoordinatesIterator::new();

        while let Some(c) = chars.next() {
            match c {
                ' ' | '\n' => {}
                '_' => {
                    coordinates_iterator.next();
                }
                'A'..='Z' => {
                    let Some(coordinates) = coordinates_iterator.next() else {
                        return Err("board is too long".to_string());
                    };

                    board.insert_tile(Tile::new(c, coordinates, false))
                }
                '*' => {
                    let Some(coordinates) = coordinates_iterator.next() else {
                        return Err("board is too long".to_string());
                    };

                    let next_c = match chars.next() {
                        Some(next_c) if ('A'..='Z').contains(&next_c) => next_c,
                        Some(c) => return Err(format!("invalid character for wildcard {}", c)),
                        None => return Err("board is too short".to_string()),
                    };

                    board.insert_tile(Tile::new(next_c, coordinates, true))
                }
                _ => {
                    return Err(format!("board: invalid character {}", c));
                }
            }
        }

        if coordinates_iterator.next().is_some() {
            return Err("board is too short".to_string());
        }

        Ok(board)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                match self.tiles[i][j] {
                    None => write!(f, "_")?,
                    Some(tile) => write!(f, "{}", tile.letter)?,
                };

                if j < BOARD_SIZE - 1 {
                    write!(f, " ")?;
                }
            }

            if i < BOARD_SIZE - 1 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}
