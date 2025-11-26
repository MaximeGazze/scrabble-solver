use super::*;

#[test]
fn coordinates_add_center_horizontal() {
    let result = Coordinates::new(5, 7).add(1, Orientation::Horizontal);
    assert_eq!(result, Some(Coordinates::new(5, 8)));
}

#[test]
fn coordinates_add_center_vertical() {
    let result = Coordinates::new(5, 7).add(1, Orientation::Vertical);
    assert_eq!(result, Some(Coordinates::new(6, 7)));
}

#[test]
fn coordinates_add_top_left_horizontal() {
    let result = Coordinates::new(0, 0).add(1, Orientation::Horizontal);
    assert_eq!(result, Some(Coordinates::new(0, 1)));
}

#[test]
fn coordinates_add_top_left_vertical() {
    let result = Coordinates::new(0, 0).add(1, Orientation::Vertical);
    assert_eq!(result, Some(Coordinates::new(1, 0)));
}

#[test]
fn coordinates_add_top_right_horizontal() {
    let result = Coordinates::new(0, Board::BOARD_SIZE - 1).add(1, Orientation::Horizontal);
    assert_eq!(result, None);
}

#[test]
fn coordinates_add_top_right_vertical() {
    let result = Coordinates::new(0, Board::BOARD_SIZE - 1).add(1, Orientation::Vertical);
    assert_eq!(result, Some(Coordinates::new(1, Board::BOARD_SIZE - 1)));
}

#[test]
fn coordinates_add_bottom_left_horizontal() {
    let result = Coordinates::new(Board::BOARD_SIZE - 1, 0).add(1, Orientation::Horizontal);
    assert_eq!(result, Some(Coordinates::new(Board::BOARD_SIZE - 1, 1)));
}

#[test]
fn coordinates_add_bottom_left_vertical() {
    let result = Coordinates::new(Board::BOARD_SIZE - 1, 0).add(1, Orientation::Vertical);
    assert_eq!(result, None);
}

#[test]
fn coordinates_add_bottom_right_horizontal() {
    let result = Coordinates::new(Board::BOARD_SIZE - 1, Board::BOARD_SIZE - 1)
        .add(1, Orientation::Horizontal);
    assert_eq!(result, None);
}

#[test]
fn coordinates_add_bottom_right_vertical() {
    let result = Coordinates::new(Board::BOARD_SIZE - 1, Board::BOARD_SIZE - 1)
        .add(1, Orientation::Vertical);
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_center_horizontal() {
    let result = Coordinates::new(5, 7).sub(1, Orientation::Horizontal);
    assert_eq!(result, Some(Coordinates::new(5, 6)));
}

#[test]
fn coordinates_sub_center_vertical() {
    let result = Coordinates::new(5, 7).sub(1, Orientation::Vertical);
    assert_eq!(result, Some(Coordinates::new(4, 7)));
}

#[test]
fn coordinates_sub_top_left_horizontal() {
    let result = Coordinates::new(0, 0).sub(1, Orientation::Horizontal);
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_top_left_vertical() {
    let result = Coordinates::new(0, 0).sub(1, Orientation::Vertical);
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_top_right_horizontal() {
    let result = Coordinates::new(0, Board::BOARD_SIZE - 1).sub(1, Orientation::Horizontal);
    assert_eq!(result, Some(Coordinates::new(0, Board::BOARD_SIZE - 2)));
}

#[test]
fn coordinates_sub_top_right_vertical() {
    let result = Coordinates::new(0, Board::BOARD_SIZE - 1).sub(1, Orientation::Vertical);
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_bottom_left_horizontal() {
    let result = Coordinates::new(Board::BOARD_SIZE - 1, 0).sub(1, Orientation::Horizontal);
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_bottom_left_vertical() {
    let result = Coordinates::new(Board::BOARD_SIZE - 1, 0).sub(1, Orientation::Vertical);
    assert_eq!(result, Some(Coordinates::new(Board::BOARD_SIZE - 2, 0)));
}

#[test]
fn coordinates_sub_bottom_right_horizontal() {
    let result = Coordinates::new(Board::BOARD_SIZE - 1, Board::BOARD_SIZE - 1)
        .sub(1, Orientation::Horizontal);
    assert_eq!(
        result,
        Some(Coordinates::new(
            Board::BOARD_SIZE - 1,
            Board::BOARD_SIZE - 2
        )),
    );
}

#[test]
fn coordinates_sub_bottom_right_vertical() {
    let result = Coordinates::new(Board::BOARD_SIZE - 1, Board::BOARD_SIZE - 1)
        .sub(1, Orientation::Vertical);
    assert_eq!(
        result,
        Some(Coordinates::new(
            Board::BOARD_SIZE - 2,
            Board::BOARD_SIZE - 1
        )),
    );
}

#[test]
fn tile_iterator() {
    let mut board = Board::new();

    let tiles = [
        Tile::new('F', Coordinates::new(3, 2), false),
        Tile::new('U', Coordinates::new(3, 3), false),
        Tile::new('C', Coordinates::new(3, 4), false),
        Tile::new('K', Coordinates::new(3, 5), false),
        Tile::new('Y', Coordinates::new(4, 0), false),
        Tile::new('O', Coordinates::new(4, 1), false),
        Tile::new('U', Coordinates::new(4, 2), false),
        Tile::new('N', Coordinates::new(5, 2), false),
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let mut tile_iter = TileIterator {
        board: &board,
        coordinates: Coordinates::new(0, 0),
    };

    for tile in tiles {
        assert_eq!(tile_iter.next(), Some(tile).as_ref());
    }

    assert_eq!(tile_iter.next(), None);
}

#[test]
fn validate_tile_standalone() {
    let board = Board::new();

    let tile = Tile::new('A', Coordinates::new(0, 0), false);

    let wordlist = HashSet::new();

    assert!(board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
    assert!(board.validate_tile(&tile, Orientation::Vertical, &wordlist));
}

#[test]
fn validate_tile_word_before_valid() {
    let mut board = Board::new();

    let tiles = [
        Tile::new('W', Coordinates::new(3, 2), false),
        Tile::new('O', Coordinates::new(3, 3), false),
        Tile::new('R', Coordinates::new(3, 4), false),
        Tile::new('D', Coordinates::new(3, 5), false),
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile::new('S', Coordinates::new(3, 6), false);

    let wordlist = HashSet::from([String::from("WORDS")]);

    assert!(board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn validate_tile_word_before_invalid() {
    let mut board = Board::new();

    let tiles = [
        Tile::new('W', Coordinates::new(3, 2), false),
        Tile::new('O', Coordinates::new(3, 3), false),
        Tile::new('R', Coordinates::new(3, 4), false),
        Tile::new('D', Coordinates::new(3, 5), false),
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile::new('Z', Coordinates::new(3, 6), false);

    let wordlist = HashSet::from([String::from("WORDS")]);

    assert!(!board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn validate_tile_word_after_valid() {
    let mut board = Board::new();

    let tiles = [
        Tile::new('W', Coordinates::new(3, 2), false),
        Tile::new('O', Coordinates::new(3, 3), false),
        Tile::new('R', Coordinates::new(3, 4), false),
        Tile::new('D', Coordinates::new(3, 5), false),
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile::new('S', Coordinates::new(3, 1), false);

    let wordlist = HashSet::from([String::from("SWORD")]);

    assert!(board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn validate_tile_word_after_invalid() {
    let mut board = Board::new();

    let tiles = [
        Tile::new('W', Coordinates::new(3, 2), false),
        Tile::new('O', Coordinates::new(3, 3), false),
        Tile::new('R', Coordinates::new(3, 4), false),
        Tile::new('D', Coordinates::new(3, 5), false),
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile::new('Z', Coordinates::new(3, 1), false);

    let wordlist = HashSet::from([String::from("SWORD")]);

    assert!(!board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn validate_tile_word_before_and_after_valid() {
    let mut board = Board::new();

    let tiles = [
        Tile::new('B', Coordinates::new(3, 2), false),
        Tile::new('R', Coordinates::new(3, 3), false),
        Tile::new('O', Coordinates::new(3, 4), false),
        Tile::new('A', Coordinates::new(3, 5), false),
        Tile::new('D', Coordinates::new(3, 6), false),
        Tile::new('W', Coordinates::new(3, 8), false),
        Tile::new('O', Coordinates::new(3, 9), false),
        Tile::new('R', Coordinates::new(3, 10), false),
        Tile::new('D', Coordinates::new(3, 11), false),
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile::new('S', Coordinates::new(3, 7), false);

    let wordlist = HashSet::from([String::from("BROADSWORD")]);

    assert!(board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn validate_tile_word_before_and_after_invalid() {
    let mut board = Board::new();

    let tiles = [
        Tile::new('B', Coordinates::new(3, 2), false),
        Tile::new('R', Coordinates::new(3, 3), false),
        Tile::new('O', Coordinates::new(3, 4), false),
        Tile::new('A', Coordinates::new(3, 5), false),
        Tile::new('D', Coordinates::new(3, 6), false),
        Tile::new('W', Coordinates::new(3, 8), false),
        Tile::new('O', Coordinates::new(3, 9), false),
        Tile::new('R', Coordinates::new(3, 10), false),
        Tile::new('D', Coordinates::new(3, 11), false),
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile::new('Z', Coordinates::new(3, 7), false);

    let wordlist = HashSet::from([String::from("BROADSWORD")]);

    assert!(!board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn find_extension_plays_horizontal() {
    let mut board = Board::new();

    let tiles = vec![
        Tile::new('W', Coordinates::new(3, 4), false),
        Tile::new('O', Coordinates::new(3, 5), false),
        Tile::new('R', Coordinates::new(3, 6), false),
        Tile::new('D', Coordinates::new(3, 7), false),
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let board_word = BoardWord {
        word: String::from("WORD"),
        tiles,
        orientation: Orientation::Horizontal,
    };

    let hand = Hand::from(['P', 'A', 'S', 'S', 'M', 'L', 'L']);

    let wordlist = HashSet::from([
        String::from("WORD"),
        String::from("WORDS"),
        String::from("SWORD"),
        String::from("PASSWORD"),
        String::from("SMALLSWORD"),
    ]);

    let expected = HashSet::from([
        Play {
            word: String::from("SWORD"),
            tiles: vec![Tile::new('S', Coordinates::new(3, 3), false)],
            hand: Hand::from(['P', 'A', 'S', 'M', 'L', 'L']),
            orientation: Orientation::Horizontal,
        },
        Play {
            word: String::from("PASSWORD"),
            tiles: vec![
                Tile::new('P', Coordinates::new(3, 0), false),
                Tile::new('A', Coordinates::new(3, 1), false),
                Tile::new('S', Coordinates::new(3, 2), false),
                Tile::new('S', Coordinates::new(3, 3), false),
            ],
            hand: Hand::from(['M', 'L', 'L']),
            orientation: Orientation::Horizontal,
        },
        Play {
            word: String::from("WORDS"),
            tiles: vec![Tile::new('S', Coordinates::new(3, 8), false)],
            hand: Hand::from(['P', 'A', 'S', 'M', 'L', 'L']),
            orientation: Orientation::Horizontal,
        },
    ]);

    let result = board.find_extension_plays(&board_word, hand, &wordlist);

    assert_eq!(result, expected);
}

#[test]
fn find_extension_plays_vertical() {
    let mut board = Board::new();

    let tiles = vec![
        Tile::new('W', Coordinates::new(4, 3), false),
        Tile::new('O', Coordinates::new(5, 3), false),
        Tile::new('R', Coordinates::new(6, 3), false),
        Tile::new('D', Coordinates::new(7, 3), false),
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let board_word = BoardWord {
        word: String::from("WORD"),
        tiles,
        orientation: Orientation::Vertical,
    };

    let hand = Hand::from(['P', 'A', 'S', 'S', 'M', 'L', 'L']);

    let wordlist = HashSet::from([
        String::from("WORD"),
        String::from("WORDS"),
        String::from("SWORD"),
        String::from("PASSWORD"),
        String::from("SMALLSWORD"),
    ]);

    let expected = HashSet::from([
        Play {
            word: String::from("SWORD"),
            tiles: vec![Tile::new('S', Coordinates::new(3, 3), false)],
            hand: Hand::from(['P', 'A', 'S', 'M', 'L', 'L']),
            orientation: Orientation::Vertical,
        },
        Play {
            word: String::from("PASSWORD"),
            tiles: vec![
                Tile::new('P', Coordinates::new(0, 3), false),
                Tile::new('A', Coordinates::new(1, 3), false),
                Tile::new('S', Coordinates::new(2, 3), false),
                Tile::new('S', Coordinates::new(3, 3), false),
            ],
            hand: Hand::from(['M', 'L', 'L']),
            orientation: Orientation::Vertical,
        },
        Play {
            word: String::from("WORDS"),
            tiles: vec![Tile::new('S', Coordinates::new(8, 3), false)],
            hand: Hand::from(['P', 'A', 'S', 'M', 'L', 'L']),
            orientation: Orientation::Vertical,
        },
    ]);

    let result = board.find_extension_plays(&board_word, hand, &wordlist);

    assert_eq!(result, expected);
}

#[test]
fn find_extension_plays_skewer_before() {
    let mut board = Board::new();

    let word_tiles = vec![
        Tile::new('W', Coordinates::new(3, 4), false),
        Tile::new('O', Coordinates::new(3, 5), false),
        Tile::new('R', Coordinates::new(3, 6), false),
        Tile::new('D', Coordinates::new(3, 7), false),
    ];

    let whale_tiles = vec![
        Tile::new('W', Coordinates::new(1, 1), false),
        Tile::new('H', Coordinates::new(2, 1), false),
        Tile::new('A', Coordinates::new(3, 1), false),
        Tile::new('L', Coordinates::new(4, 1), false),
        Tile::new('E', Coordinates::new(5, 1), false),
    ];

    for tile in &word_tiles {
        board.insert_tile(*tile);
    }

    for tile in &whale_tiles {
        board.insert_tile(*tile);
    }

    let board_word = BoardWord {
        word: String::from("WORD"),
        tiles: word_tiles,
        orientation: Orientation::Horizontal,
    };

    let hand = Hand::from(['P', 'A', 'S', 'S', 'K', 'E', 'Y']);

    let wordlist = HashSet::from([
        String::from("WORD"),
        String::from("PASSWORD"),
        String::from("KEYWORD"),
    ]);

    let expected = HashSet::from([Play {
        word: String::from("PASSWORD"),
        tiles: vec![
            Tile::new('P', Coordinates::new(3, 0), false),
            Tile::new('S', Coordinates::new(3, 2), false),
            Tile::new('S', Coordinates::new(3, 3), false),
        ],
        hand: Hand::from(['A', 'K', 'E', 'Y']),
        orientation: Orientation::Horizontal,
    }]);

    let result = board.find_extension_plays(&board_word, hand, &wordlist);

    assert_eq!(result, expected);
}

#[test]
fn find_extension_plays_skewer_after() {
    let mut board = Board::new();

    let word_tiles = vec![
        Tile::new('W', Coordinates::new(3, 4), false),
        Tile::new('O', Coordinates::new(3, 5), false),
        Tile::new('R', Coordinates::new(3, 6), false),
        Tile::new('D', Coordinates::new(3, 7), false),
    ];

    let whale_tiles = vec![
        Tile::new('W', Coordinates::new(1, 10), false),
        Tile::new('H', Coordinates::new(2, 10), false),
        Tile::new('A', Coordinates::new(3, 10), false),
        Tile::new('L', Coordinates::new(4, 10), false),
        Tile::new('E', Coordinates::new(5, 10), false),
    ];

    for tile in &word_tiles {
        board.insert_tile(*tile);
    }

    for tile in &whale_tiles {
        board.insert_tile(*tile);
    }

    let board_word = BoardWord {
        word: String::from("WORD"),
        tiles: word_tiles,
        orientation: Orientation::Horizontal,
    };

    let hand = Hand::from(['P', 'L', 'A', 'Y', 'I', 'N', 'G']);

    let wordlist = HashSet::from([
        String::from("WORD"),
        String::from("WORDPLAY"),
        String::from("WORDING"),
    ]);

    let expected = HashSet::from([Play {
        word: String::from("WORDPLAY"),
        tiles: vec![
            Tile::new('P', Coordinates::new(3, 8), false),
            Tile::new('L', Coordinates::new(3, 9), false),
            Tile::new('Y', Coordinates::new(3, 11), false),
        ],
        hand: Hand::from(['A', 'I', 'N', 'G']),
        orientation: Orientation::Horizontal,
    }]);

    let result = board.find_extension_plays(&board_word, hand, &wordlist);

    assert_eq!(result, expected);
}

#[test]
fn find_extension_plays_combine() {
    let mut board = Board::new();

    let short_tiles = vec![
        Tile::new('S', Coordinates::new(1, 1), false),
        Tile::new('H', Coordinates::new(2, 1), false),
        Tile::new('O', Coordinates::new(3, 1), false),
        Tile::new('R', Coordinates::new(4, 1), false),
        Tile::new('T', Coordinates::new(5, 1), false),
    ];

    let word_tiles = vec![
        Tile::new('W', Coordinates::new(7, 1), false),
        Tile::new('O', Coordinates::new(8, 1), false),
        Tile::new('R', Coordinates::new(9, 1), false),
        Tile::new('D', Coordinates::new(10, 1), false),
    ];

    for tile in &short_tiles {
        board.insert_tile(*tile);
    }

    for tile in &word_tiles {
        board.insert_tile(*tile);
    }

    let board_word = BoardWord {
        word: String::from("WORD"),
        tiles: word_tiles,
        orientation: Orientation::Vertical,
    };

    let hand = Hand::from(['A', 'B', 'C', 'D', 'S', 'E', 'F']);

    let wordlist = HashSet::from([
        String::from("SHORT"),
        String::from("WORD"),
        String::from("SHORTSWORD"),
    ]);

    let expected = HashSet::from([Play {
        word: String::from("SHORTSWORD"),
        tiles: vec![Tile::new('S', Coordinates::new(6, 1), false)],
        hand: Hand::from(['A', 'B', 'C', 'D', 'E', 'F']),
        orientation: Orientation::Vertical,
    }]);

    let result = board.find_extension_plays(&board_word, hand, &wordlist);

    assert_eq!(result, expected);
}

#[test]
fn find_hook_plays() {
    let mut board = Board::new();

    let tiles = vec![
        Tile::new('W', Coordinates::new(3, 8), false),
        Tile::new('O', Coordinates::new(4, 8), false),
        Tile::new('R', Coordinates::new(5, 8), false),
        Tile::new('D', Coordinates::new(6, 8), false),
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let play = Play {
        word: String::from("WORDS"),
        tiles: vec![Tile::new('S', Coordinates::new(7, 8), false)],
        hand: Hand::from(['L', 'A', 'S', 'O', 'A', 'F']),
        orientation: Orientation::Vertical,
    };

    let wordlist = HashSet::from([
        String::from("WORD"),
        String::from("WORDS"),
        String::from("LASSO"),
        String::from("LOAF"),
        String::from("LOAFS"),
    ]);

    let expected = HashSet::from([
        Play {
            word: String::from("LASSO"),
            tiles: vec![
                Tile::new('L', Coordinates::new(7, 6), false),
                Tile::new('A', Coordinates::new(7, 7), false),
                Tile::new('S', Coordinates::new(7, 8), false),
                Tile::new('S', Coordinates::new(7, 9), false),
                Tile::new('O', Coordinates::new(7, 10), false),
            ],
            hand: Hand::from(['A', 'F']),
            orientation: Orientation::Horizontal,
        },
        Play {
            word: String::from("LASSO"),
            tiles: vec![
                Tile::new('L', Coordinates::new(7, 5), false),
                Tile::new('A', Coordinates::new(7, 6), false),
                Tile::new('S', Coordinates::new(7, 7), false),
                Tile::new('S', Coordinates::new(7, 8), false),
                Tile::new('O', Coordinates::new(7, 9), false),
            ],
            hand: Hand::from(['A', 'F']),
            orientation: Orientation::Horizontal,
        },
        Play {
            word: String::from("LOAFS"),
            tiles: vec![
                Tile::new('L', Coordinates::new(7, 4), false),
                Tile::new('O', Coordinates::new(7, 5), false),
                Tile::new('A', Coordinates::new(7, 6), false),
                Tile::new('F', Coordinates::new(7, 7), false),
                Tile::new('S', Coordinates::new(7, 8), false),
            ],
            hand: Hand::from(['S', 'A']),
            orientation: Orientation::Horizontal,
        },
    ]);

    let result = board.find_hook_plays(&play, &wordlist);

    assert_eq!(result, expected);
}

#[test]
fn find_hook_plays_skewer() {
    let mut board = Board::new();

    let word_tiles = vec![
        Tile::new('W', Coordinates::new(0, 6), false),
        Tile::new('O', Coordinates::new(1, 6), false),
        Tile::new('R', Coordinates::new(2, 6), false),
        Tile::new('D', Coordinates::new(3, 6), false),
    ];

    let bag_tiles = vec![
        Tile::new('B', Coordinates::new(4, 5), false),
        Tile::new('A', Coordinates::new(5, 5), false),
        Tile::new('G', Coordinates::new(6, 5), false),
    ];

    for tile in &word_tiles {
        board.insert_tile(*tile);
    }

    for tile in &bag_tiles {
        board.insert_tile(*tile);
    }

    let play = Play {
        word: String::from("WORDS"),
        tiles: vec![Tile::new('S', Coordinates::new(4, 6), false)],
        hand: Hand::from(['L', 'A', 'S', 'O', 'A', 'F']),
        orientation: Orientation::Vertical,
    };

    let wordlist = HashSet::from([
        String::from("WORD"),
        String::from("WORDS"),
        String::from("LASSO"),
        String::from("LOAF"),
        String::from("LOAFS"),
    ]);

    let result = board.find_hook_plays(&play, &wordlist);

    assert_eq!(result, HashSet::new());
}

#[test]
fn find_hook_plays_long_skewer() {
    let mut board = Board::new();

    let word_tiles = vec![
        Tile::new('W', Coordinates::new(0, 6), false),
        Tile::new('O', Coordinates::new(1, 6), false),
        Tile::new('R', Coordinates::new(2, 6), false),
        Tile::new('D', Coordinates::new(3, 6), false),
    ];

    let bag_tiles = vec![
        Tile::new('B', Coordinates::new(4, 3), false),
        Tile::new('A', Coordinates::new(4, 4), false),
        Tile::new('G', Coordinates::new(4, 5), false),
    ];

    for tile in &word_tiles {
        board.insert_tile(*tile);
    }

    for tile in &bag_tiles {
        board.insert_tile(*tile);
    }

    let play = Play {
        word: String::from("WORDS"),
        tiles: vec![Tile::new('S', Coordinates::new(4, 6), false)],
        hand: Hand::from(['L', 'A', 'S', 'O', 'A', 'F']),
        orientation: Orientation::Vertical,
    };

    let wordlist = HashSet::from([
        String::from("WORD"),
        String::from("WORDS"),
        String::from("LASSO"),
        String::from("LOAF"),
        String::from("LOAFS"),
    ]);

    let result = board.find_hook_plays(&play, &wordlist);

    assert_eq!(result, HashSet::new());
}

#[test]
fn find_perpendicular_plays() {
    let mut board = Board::new();

    let tiles = vec![
        Tile::new('W', Coordinates::new(0, 6), false),
        Tile::new('O', Coordinates::new(1, 6), false),
        Tile::new('R', Coordinates::new(2, 6), false),
        Tile::new('D', Coordinates::new(3, 6), false),
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let board_word = BoardWord {
        word: String::from("WORD"),
        tiles,
        orientation: Orientation::Vertical,
    };

    let hand = Hand::from(['W', 'I', 'R', 'A', 'L', 'M', 'Z']);

    let wordlist = HashSet::from([String::from("WORD"), String::from("WORM")]);

    let result = board.find_perpendicular_plays(&board_word, hand, &wordlist);

    let expected = HashSet::from([Play {
        word: String::from("WORM"),
        tiles: vec![
            Tile::new('W', Coordinates::new(1, 5), false),
            Tile::new('R', Coordinates::new(1, 7), false),
            Tile::new('M', Coordinates::new(1, 8), false),
        ],
        hand: Hand::from(['I', 'A', 'L', 'Z']),
        orientation: Orientation::Horizontal,
    }]);

    assert_eq!(result, expected);
}

#[test]
fn find_parallel_plays() {
    let mut board = Board::new();

    let tiles = vec![
        Tile::new('W', Coordinates::new(0, 6), false),
        Tile::new('O', Coordinates::new(1, 6), false),
        Tile::new('R', Coordinates::new(2, 6), false),
        Tile::new('D', Coordinates::new(3, 6), false),
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let play = Play {
        word: String::from("OF"),
        tiles: vec![Tile::new('F', Coordinates::new(1, 7), false)],
        hand: Hand::from(['E', 'A', 'R', 'A', 'B', 'C']),
        orientation: Orientation::Horizontal,
    };

    let wordlist = HashSet::from([
        String::from("WORD"),
        String::from("FEAR"),
        String::from("OF"),
        String::from("RE"),
        String::from("DA"),
    ]);

    let result = board.find_parallel_plays(&play, &wordlist);

    let expected = HashSet::from([Play {
        word: String::from("FEAR"),
        tiles: vec![
            Tile::new('F', Coordinates::new(1, 7), false),
            Tile::new('E', Coordinates::new(2, 7), false),
            Tile::new('A', Coordinates::new(3, 7), false),
            Tile::new('R', Coordinates::new(4, 7), false),
        ],
        hand: Hand::from(['A', 'B', 'C']),
        orientation: Orientation::Vertical,
    }]);

    assert_eq!(result, expected);
}

#[test]
fn score_play() {
    let mut board = Board::new();

    let tiles = vec![
        Tile::new('W', Coordinates::new(0, 6), false),
        Tile::new('O', Coordinates::new(1, 6), false),
        Tile::new('R', Coordinates::new(2, 6), false),
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let play = Play {
        word: String::from("WORD"),
        tiles: vec![Tile::new('D', Coordinates::new(3, 6), false)],
        hand: Hand::from(['A', 'B', 'C', 'D', 'E', 'F']),
        orientation: Orientation::Vertical,
    };

    assert_eq!(8, board.score_play(&play));
}

#[test]
fn score_play_with_double_letter_bonus() {
    let mut board = Board::new();

    let tiles = vec![
        Tile::new('W', Coordinates::new(2, 3), false),
        Tile::new('O', Coordinates::new(2, 4), false),
        Tile::new('R', Coordinates::new(2, 5), false),
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let play = Play {
        word: String::from("WORD"),
        tiles: vec![Tile::new('D', Coordinates::new(2, 6), false)],
        hand: Hand::from(['A', 'B', 'C', 'D', 'E', 'F']),
        orientation: Orientation::Horizontal,
    };

    assert_eq!(10, board.score_play(&play));
}

#[test]
fn score_play_with_triple_letter_bonus() {
    let mut board = Board::new();

    let tiles = vec![
        Tile::new('W', Coordinates::new(5, 6), false),
        Tile::new('O', Coordinates::new(5, 7), false),
        Tile::new('R', Coordinates::new(5, 8), false),
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let play = Play {
        word: String::from("WORD"),
        tiles: vec![Tile::new('D', Coordinates::new(5, 9), false)],
        hand: Hand::from(['A', 'B', 'C', 'D', 'E', 'F']),
        orientation: Orientation::Horizontal,
    };

    assert_eq!(12, board.score_play(&play));
}

#[test]
fn score_play_with_double_word_bonus() {
    let mut board = Board::new();

    let tiles = vec![
        Tile::new('W', Coordinates::new(11, 4), false),
        Tile::new('O', Coordinates::new(11, 5), false),
        Tile::new('R', Coordinates::new(11, 6), false),
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let play = Play {
        word: String::from("WORD"),
        tiles: vec![Tile::new('D', Coordinates::new(11, 3), false)],
        hand: Hand::from(['A', 'B', 'C', 'D', 'E', 'F']),
        orientation: Orientation::Horizontal,
    };

    assert_eq!(16, board.score_play(&play));
}

#[test]
fn score_play_with_triple_word_bonus() {
    let mut board = Board::new();

    let tiles = vec![
        Tile::new('W', Coordinates::new(14, 11), false),
        Tile::new('O', Coordinates::new(14, 12), false),
        Tile::new('R', Coordinates::new(14, 13), false),
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let play = Play {
        word: String::from("WORD"),
        tiles: vec![Tile::new('D', Coordinates::new(14, 14), false)],
        hand: Hand::from(['A', 'B', 'C', 'D', 'E', 'F']),
        orientation: Orientation::Horizontal,
    };

    assert_eq!(24, board.score_play(&play));
}

#[test]
fn score_play_with_crosswords() {
    let mut board = Board::new();

    let cave_tiles = vec![
        Tile::new('C', Coordinates::new(0, 7), false),
        Tile::new('A', Coordinates::new(0, 8), false),
        Tile::new('V', Coordinates::new(0, 9), false),
        Tile::new('E', Coordinates::new(0, 10), false),
    ];

    let van_tiles = vec![
        Tile::new('V', Coordinates::new(0, 9), false),
        Tile::new('A', Coordinates::new(1, 9), false),
        Tile::new('N', Coordinates::new(2, 9), false),
    ];

    let treason_tiles = vec![
        Tile::new('T', Coordinates::new(2, 7), false),
        Tile::new('R', Coordinates::new(3, 7), false),
        Tile::new('E', Coordinates::new(4, 7), false),
        Tile::new('A', Coordinates::new(5, 7), false),
        Tile::new('S', Coordinates::new(6, 7), false),
        Tile::new('O', Coordinates::new(7, 7), false),
        Tile::new('N', Coordinates::new(8, 7), false),
    ];

    let yen_tiles = vec![
        Tile::new('Y', Coordinates::new(2, 10), false),
        Tile::new('E', Coordinates::new(3, 10), true),
        Tile::new('N', Coordinates::new(4, 10), false),
    ];

    let evil_tiles = vec![
        Tile::new('E', Coordinates::new(3, 10), true),
        Tile::new('V', Coordinates::new(3, 11), false),
        Tile::new('I', Coordinates::new(3, 12), false),
        Tile::new('L', Coordinates::new(3, 13), false),
    ];

    for tile in &cave_tiles {
        board.insert_tile(*tile);
    }

    for tile in &van_tiles {
        board.insert_tile(*tile);
    }

    for tile in &treason_tiles {
        board.insert_tile(*tile);
    }

    for tile in &yen_tiles {
        board.insert_tile(*tile);
    }

    for tile in &evil_tiles {
        board.insert_tile(*tile);
    }

    let play = Play {
        word: String::from("TINY"),
        tiles: vec![Tile::new('I', Coordinates::new(2, 8), false)],
        hand: Hand::from(['A', 'B', 'C', 'D', 'E']),
        orientation: Orientation::Horizontal,
    };

    assert_eq!(26, board.score_play(&play));
}
