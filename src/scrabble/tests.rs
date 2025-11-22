use super::*;

#[test]
fn coordinates_add_center_horizontal() {
    let result = Coordinates { i: 5, j: 7 }.add(1, Orientation::Horizontal);
    assert_eq!(result, Some(Coordinates { i: 5, j: 8 }));
}

#[test]
fn coordinates_add_center_vertical() {
    let result = Coordinates { i: 5, j: 7 }.add(1, Orientation::Vertical);
    assert_eq!(result, Some(Coordinates { i: 6, j: 7 }));
}

#[test]
fn coordinates_add_top_left_horizontal() {
    let result = Coordinates { i: 0, j: 0 }.add(1, Orientation::Horizontal);
    assert_eq!(result, Some(Coordinates { i: 0, j: 1 }));
}

#[test]
fn coordinates_add_top_left_vertical() {
    let result = Coordinates { i: 0, j: 0 }.add(1, Orientation::Vertical);
    assert_eq!(result, Some(Coordinates { i: 1, j: 0 }));
}

#[test]
fn coordinates_add_top_right_horizontal() {
    let result = Coordinates {
        i: 0,
        j: Board::BOARD_SIZE,
    }
    .add(1, Orientation::Horizontal);
    assert_eq!(result, None);
}

#[test]
fn coordinates_add_top_right_vertical() {
    let result = Coordinates {
        i: 0,
        j: Board::BOARD_SIZE,
    }
    .add(1, Orientation::Vertical);
    assert_eq!(
        result,
        Some(Coordinates {
            i: 1,
            j: Board::BOARD_SIZE
        })
    );
}

#[test]
fn coordinates_add_bottom_left_horizontal() {
    let result = Coordinates {
        i: Board::BOARD_SIZE,
        j: 0,
    }
    .add(1, Orientation::Horizontal);
    assert_eq!(
        result,
        Some(Coordinates {
            i: Board::BOARD_SIZE,
            j: 1,
        })
    );
}

#[test]
fn coordinates_add_bottom_left_vertical() {
    let result = Coordinates {
        i: Board::BOARD_SIZE,
        j: 0,
    }
    .add(1, Orientation::Vertical);
    assert_eq!(result, None);
}

#[test]
fn coordinates_add_bottom_right_horizontal() {
    let result = Coordinates {
        i: Board::BOARD_SIZE,
        j: Board::BOARD_SIZE,
    }
    .add(1, Orientation::Horizontal);
    assert_eq!(result, None);
}

#[test]
fn coordinates_add_bottom_right_vertical() {
    let result = Coordinates {
        i: Board::BOARD_SIZE,
        j: Board::BOARD_SIZE,
    }
    .add(1, Orientation::Vertical);
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_center_horizontal() {
    let result = Coordinates { i: 5, j: 7 }.sub(1, Orientation::Horizontal);
    assert_eq!(result, Some(Coordinates { i: 5, j: 6 }));
}

#[test]
fn coordinates_sub_center_vertical() {
    let result = Coordinates { i: 5, j: 7 }.sub(1, Orientation::Vertical);
    assert_eq!(result, Some(Coordinates { i: 4, j: 7 }));
}

#[test]
fn coordinates_sub_top_left_horizontal() {
    let result = Coordinates { i: 0, j: 0 }.sub(1, Orientation::Horizontal);
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_top_left_vertical() {
    let result = Coordinates { i: 0, j: 0 }.sub(1, Orientation::Vertical);
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_top_right_horizontal() {
    let result = Coordinates {
        i: 0,
        j: Board::BOARD_SIZE,
    }
    .sub(1, Orientation::Horizontal);
    assert_eq!(
        result,
        Some(Coordinates {
            i: 0,
            j: Board::BOARD_SIZE - 1,
        })
    );
}

#[test]
fn coordinates_sub_top_right_vertical() {
    let result = Coordinates {
        i: 0,
        j: Board::BOARD_SIZE,
    }
    .sub(1, Orientation::Vertical);
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_bottom_left_horizontal() {
    let result = Coordinates {
        i: Board::BOARD_SIZE,
        j: 0,
    }
    .sub(1, Orientation::Horizontal);
    assert_eq!(result, None);
}

#[test]
fn coordinates_sub_bottom_left_vertical() {
    let result = Coordinates {
        i: Board::BOARD_SIZE,
        j: 0,
    }
    .sub(1, Orientation::Vertical);
    assert_eq!(
        result,
        Some(Coordinates {
            i: Board::BOARD_SIZE - 1,
            j: 0,
        })
    );
}

#[test]
fn coordinates_sub_bottom_right_horizontal() {
    let result = Coordinates {
        i: Board::BOARD_SIZE,
        j: Board::BOARD_SIZE,
    }
    .sub(1, Orientation::Horizontal);
    assert_eq!(
        result,
        Some(Coordinates {
            i: Board::BOARD_SIZE,
            j: Board::BOARD_SIZE - 1,
        })
    );
}

#[test]
fn coordinates_sub_bottom_right_vertical() {
    let result = Coordinates {
        i: Board::BOARD_SIZE,
        j: Board::BOARD_SIZE,
    }
    .sub(1, Orientation::Vertical);
    assert_eq!(
        result,
        Some(Coordinates {
            i: Board::BOARD_SIZE - 1,
            j: Board::BOARD_SIZE,
        })
    );
}

#[test]
fn tile_iterator() {
    let mut board = Board::new();

    let tiles = [
        Tile {
            letter: 'F',
            coordinates: Coordinates { i: 3, j: 2 },
            wildcard: false,
        },
        Tile {
            letter: 'U',
            coordinates: Coordinates { i: 3, j: 3 },
            wildcard: false,
        },
        Tile {
            letter: 'C',
            coordinates: Coordinates { i: 3, j: 4 },
            wildcard: false,
        },
        Tile {
            letter: 'K',
            coordinates: Coordinates { i: 3, j: 5 },
            wildcard: false,
        },
        Tile {
            letter: 'Y',
            coordinates: Coordinates { i: 4, j: 0 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 4, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'U',
            coordinates: Coordinates { i: 4, j: 2 },
            wildcard: false,
        },
        Tile {
            letter: 'N',
            coordinates: Coordinates { i: 5, j: 2 },
            wildcard: false,
        },
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let mut tile_iter = TileIterator::from_board(&board);

    for tile in tiles {
        assert_eq!(tile_iter.next(), Some(tile).as_ref());
    }

    assert_eq!(tile_iter.next(), None);
}

// TODO test tile before after etc

#[test]
fn validate_tile_standalone() {
    let board = Board::new();

    let tile = Tile {
        letter: 'A',
        coordinates: Coordinates { i: 0, j: 0 },
        wildcard: false,
    };

    let wordlist = HashSet::new();

    assert!(board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
    assert!(board.validate_tile(&tile, Orientation::Vertical, &wordlist));
}

#[test]
fn validate_tile_word_before_valid() {
    let mut board = Board::new();

    let tiles = [
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 3, j: 2 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 3 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 3, j: 4 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 3, j: 5 },
            wildcard: false,
        },
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile {
        letter: 'S',
        coordinates: Coordinates { i: 3, j: 6 },
        wildcard: false,
    };

    let wordlist = HashSet::from([String::from("WORDS")]);

    assert!(board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn validate_tile_word_before_invalid() {
    let mut board = Board::new();

    let tiles = [
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 3, j: 2 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 3 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 3, j: 4 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 3, j: 5 },
            wildcard: false,
        },
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile {
        letter: 'Z',
        coordinates: Coordinates { i: 3, j: 6 },
        wildcard: false,
    };

    let wordlist = HashSet::from([String::from("WORDS")]);

    assert!(!board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn validate_tile_word_after_valid() {
    let mut board = Board::new();

    let tiles = [
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 3, j: 2 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 3 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 3, j: 4 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 3, j: 5 },
            wildcard: false,
        },
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile {
        letter: 'S',
        coordinates: Coordinates { i: 3, j: 1 },
        wildcard: false,
    };

    let wordlist = HashSet::from([String::from("SWORD")]);

    assert!(board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn validate_tile_word_after_invalid() {
    let mut board = Board::new();

    let tiles = [
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 3, j: 2 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 3 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 3, j: 4 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 3, j: 5 },
            wildcard: false,
        },
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile {
        letter: 'Z',
        coordinates: Coordinates { i: 3, j: 1 },
        wildcard: false,
    };

    let wordlist = HashSet::from([String::from("SWORD")]);

    assert!(!board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn validate_tile_word_before_and_after_valid() {
    let mut board = Board::new();

    let tiles = [
        Tile {
            letter: 'B',
            coordinates: Coordinates { i: 3, j: 2 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 3, j: 3 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 4 },
            wildcard: false,
        },
        Tile {
            letter: 'A',
            coordinates: Coordinates { i: 3, j: 5 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 3, j: 6 },
            wildcard: false,
        },
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 3, j: 8 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 9 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 3, j: 10 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 3, j: 11 },
            wildcard: false,
        },
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile {
        letter: 'S',
        coordinates: Coordinates { i: 3, j: 7 },
        wildcard: false,
    };

    let wordlist = HashSet::from([String::from("BROADSWORD")]);

    assert!(board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn validate_tile_word_before_and_after_invalid() {
    let mut board = Board::new();

    let tiles = [
        Tile {
            letter: 'B',
            coordinates: Coordinates { i: 3, j: 2 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 3, j: 3 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 4 },
            wildcard: false,
        },
        Tile {
            letter: 'A',
            coordinates: Coordinates { i: 3, j: 5 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 3, j: 6 },
            wildcard: false,
        },
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 3, j: 8 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 9 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 3, j: 10 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 3, j: 11 },
            wildcard: false,
        },
    ];

    for tile in tiles {
        board.insert_tile(tile);
    }

    let tile = Tile {
        letter: 'Z',
        coordinates: Coordinates { i: 3, j: 7 },
        wildcard: false,
    };

    let wordlist = HashSet::from([String::from("BROADSWORD")]);

    assert!(!board.validate_tile(&tile, Orientation::Horizontal, &wordlist));
}

#[test]
fn find_extension_plays_horizontal() {
    let mut board = Board::new();

    let tiles = vec![
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 3, j: 4 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 5 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 3, j: 6 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 3, j: 7 },
            wildcard: false,
        },
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let board_word = BoardWord {
        word: String::from("WORD"),
        tiles,
        orientation: Orientation::Horizontal,
    };

    let hand = vec!['P', 'A', 'S', 'S', 'M', 'L', 'L'];

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
            tiles: vec![Tile {
                letter: 'S',
                coordinates: Coordinates { i: 3, j: 3 },
                wildcard: false,
            }],
            hand: vec!['P', 'A', 'S', 'M', 'L', 'L'],
            orientation: Orientation::Horizontal,
        },
        Play {
            word: String::from("PASSWORD"),
            tiles: vec![
                Tile {
                    letter: 'P',
                    coordinates: Coordinates { i: 3, j: 0 },
                    wildcard: false,
                },
                Tile {
                    letter: 'A',
                    coordinates: Coordinates { i: 3, j: 1 },
                    wildcard: false,
                },
                Tile {
                    letter: 'S',
                    coordinates: Coordinates { i: 3, j: 2 },
                    wildcard: false,
                },
                Tile {
                    letter: 'S',
                    coordinates: Coordinates { i: 3, j: 3 },
                    wildcard: false,
                },
            ],
            hand: vec!['M', 'L', 'L'],
            orientation: Orientation::Horizontal,
        },
        Play {
            word: String::from("WORDS"),
            tiles: vec![Tile {
                letter: 'S',
                coordinates: Coordinates { i: 3, j: 8 },
                wildcard: false,
            }],
            hand: vec!['P', 'A', 'S', 'M', 'L', 'L'],
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
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 4, j: 3 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 5, j: 3 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 6, j: 3 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 7, j: 3 },
            wildcard: false,
        },
    ];

    for tile in &tiles {
        board.insert_tile(*tile);
    }

    let board_word = BoardWord {
        word: String::from("WORD"),
        tiles,
        orientation: Orientation::Vertical,
    };

    let hand = vec!['P', 'A', 'S', 'S', 'M', 'L', 'L'];

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
            tiles: vec![Tile {
                letter: 'S',
                coordinates: Coordinates { i: 3, j: 3 },
                wildcard: false,
            }],
            hand: vec!['P', 'A', 'S', 'M', 'L', 'L'],
            orientation: Orientation::Vertical,
        },
        Play {
            word: String::from("PASSWORD"),
            tiles: vec![
                Tile {
                    letter: 'P',
                    coordinates: Coordinates { i: 0, j: 3 },
                    wildcard: false,
                },
                Tile {
                    letter: 'A',
                    coordinates: Coordinates { i: 1, j: 3 },
                    wildcard: false,
                },
                Tile {
                    letter: 'S',
                    coordinates: Coordinates { i: 2, j: 3 },
                    wildcard: false,
                },
                Tile {
                    letter: 'S',
                    coordinates: Coordinates { i: 3, j: 3 },
                    wildcard: false,
                },
            ],
            hand: vec!['M', 'L', 'L'],
            orientation: Orientation::Vertical,
        },
        Play {
            word: String::from("WORDS"),
            tiles: vec![Tile {
                letter: 'S',
                coordinates: Coordinates { i: 8, j: 3 },
                wildcard: false,
            }],
            hand: vec!['P', 'A', 'S', 'M', 'L', 'L'],
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
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 3, j: 4 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 5 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 3, j: 6 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 3, j: 7 },
            wildcard: false,
        },
    ];

    let whale_tiles = vec![
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 1, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'H',
            coordinates: Coordinates { i: 2, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'A',
            coordinates: Coordinates { i: 3, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'L',
            coordinates: Coordinates { i: 4, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'E',
            coordinates: Coordinates { i: 5, j: 1 },
            wildcard: false,
        },
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

    let hand = vec!['P', 'A', 'S', 'S', 'K', 'E', 'Y'];

    let wordlist = HashSet::from([
        String::from("WORD"),
        String::from("PASSWORD"),
        String::from("KEYWORD"),
    ]);

    let expected = HashSet::from([Play {
        word: String::from("PASSWORD"),
        tiles: vec![
            Tile {
                letter: 'P',
                coordinates: Coordinates { i: 3, j: 0 },
                wildcard: false,
            },
            Tile {
                letter: 'S',
                coordinates: Coordinates { i: 3, j: 2 },
                wildcard: false,
            },
            Tile {
                letter: 'S',
                coordinates: Coordinates { i: 3, j: 3 },
                wildcard: false,
            },
        ],
        hand: vec!['A', 'K', 'E', 'Y'],
        orientation: Orientation::Horizontal,
    }]);

    let result = board.find_extension_plays(&board_word, hand, &wordlist);

    assert_eq!(result, expected);
}

#[test]
fn find_extension_plays_skewer_after() {
    let mut board = Board::new();

    let word_tiles = vec![
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 3, j: 4 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 5 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 3, j: 6 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 3, j: 7 },
            wildcard: false,
        },
    ];

    let whale_tiles = vec![
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 1, j: 10 },
            wildcard: false,
        },
        Tile {
            letter: 'H',
            coordinates: Coordinates { i: 2, j: 10 },
            wildcard: false,
        },
        Tile {
            letter: 'A',
            coordinates: Coordinates { i: 3, j: 10 },
            wildcard: false,
        },
        Tile {
            letter: 'L',
            coordinates: Coordinates { i: 4, j: 10 },
            wildcard: false,
        },
        Tile {
            letter: 'E',
            coordinates: Coordinates { i: 5, j: 10 },
            wildcard: false,
        },
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

    let hand = vec!['P', 'L', 'A', 'Y', 'I', 'N', 'G'];

    let wordlist = HashSet::from([
        String::from("WORD"),
        String::from("WORDPLAY"),
        String::from("WORDING"),
    ]);

    let expected = HashSet::from([Play {
        word: String::from("WORDPLAY"),
        tiles: vec![
            Tile {
                letter: 'P',
                coordinates: Coordinates { i: 3, j: 8 },
                wildcard: false,
            },
            Tile {
                letter: 'L',
                coordinates: Coordinates { i: 3, j: 9 },
                wildcard: false,
            },
            Tile {
                letter: 'Y',
                coordinates: Coordinates { i: 3, j: 11 },
                wildcard: false,
            },
        ],
        hand: vec!['A', 'I', 'N', 'G'],
        orientation: Orientation::Horizontal,
    }]);

    let result = board.find_extension_plays(&board_word, hand, &wordlist);

    assert_eq!(result, expected);
}

#[test]
fn find_extension_plays_combine() {
    let mut board = Board::new();

    let short_tiles = vec![
        Tile {
            letter: 'S',
            coordinates: Coordinates { i: 1, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'H',
            coordinates: Coordinates { i: 2, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 3, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 4, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'T',
            coordinates: Coordinates { i: 5, j: 1 },
            wildcard: false,
        },
    ];

    let word_tiles = vec![
        Tile {
            letter: 'W',
            coordinates: Coordinates { i: 7, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'O',
            coordinates: Coordinates { i: 8, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'R',
            coordinates: Coordinates { i: 9, j: 1 },
            wildcard: false,
        },
        Tile {
            letter: 'D',
            coordinates: Coordinates { i: 10, j: 1 },
            wildcard: false,
        },
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

    let hand = vec!['A', 'B', 'C', 'D', 'S', 'E', 'F'];

    let wordlist = HashSet::from([
        String::from("SHORT"),
        String::from("WORD"),
        String::from("SHORTSWORD"),
    ]);

    let expected = HashSet::from([Play {
        word: String::from("SHORTSWORD"),
        tiles: vec![Tile {
            letter: 'S',
            coordinates: Coordinates { i: 6, j: 1 },
            wildcard: false,
        }],
        hand: vec!['A', 'B', 'C', 'D', 'E', 'F'],
        orientation: Orientation::Vertical,
    }]);

    let result = board.find_extension_plays(&board_word, hand, &wordlist);

    assert_eq!(result, expected);
}
