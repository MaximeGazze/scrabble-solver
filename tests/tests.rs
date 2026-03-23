use scrabble_solver::{read_wordlist, Board, Coordinates, Hand, Orientation, Play, Tile};

#[test]
fn puzzle() {
    let board_string = r###"
_ _ _ _ _ _ _ _ _ _ _ _ _ _ E
_ _ _ _ _ _ _ _ _ _ _ R _ _ A
_ _ _ _ _ _ _ _ _ _ _ U M _ S
_ _ _ _ _ _ _ _ _ _ _ B A H T
_ _ _ _ _ _ _ _ _ _ _ B _ A _
_ _ _ _ _ J A M _ _ _ E _ Z A
_ _ _ _ _ _ L O X _ _ R _ A W
_ _ _ _ _ _ _ P I N K Y _ R E
_ _ I G N I T E S _ I _ _ D _
_ _ _ _ _ _ _ _ _ _ N _ E _ _
_ _ _ _ _ _ _ _ _ _ G _ A _ _
_ _ _ _ _ _ _ _ _ _ L _ G _ _
_ _ _ _ _ _ _ _ _ _ I _ L _ _
_ R O O F E D _ _ _ E _ E _ _
_ _ _ _ _ R E Q U I R E D _ _
    "###;

    let board = Board::try_from(board_string).unwrap();

    let wordlist = read_wordlist(format!(
        "{}/wordlists/NWL2020.txt",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    ))
    .unwrap();

    let hand = Hand::try_from(['H', 'A', 'P', 'L', 'Y', 'E', 'T']).unwrap();

    let result = board.find_best_play(&wordlist, hand).unwrap();

    let expected_score = 94;
    let expected_play = Play {
        word: String::from("HAPHAZARDLY"),
        tiles: vec![
            Tile::new('H', Coordinates::new(0, 13), false),
            Tile::new('A', Coordinates::new(1, 13), false),
            Tile::new('P', Coordinates::new(2, 13), false),
            Tile::new('L', Coordinates::new(9, 13), false),
            Tile::new('Y', Coordinates::new(10, 13), false),
        ],
        hand: Hand::try_from(['E', 'T']).unwrap(),
        orientation: Orientation::Vertical,
        start_coordinates: Coordinates::new(0, 13),
        end_coordinates: Coordinates::new(10, 13),
    };

    let expected = (expected_score, expected_play);

    assert_eq!(result, expected);
}

#[test]
fn puzzle2() {
    let board_string = r###"
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ B _ _ _ _ _ _ _ _ _ _ _ _ _
_ I F _ _ _ _ _ _ _ _ _ _ _ _
_ L I _ _ _ _ _ _ _ _ _ _ _ _
_ L A _ _ _ _ _ _ _ _ _ _ _ _
_ E T _ _ _ _ _ _ _ _ _ _ _ _
I T _ _ _ _ _ _ _ _ _ _ _ _ _
N _ _ Q U I C H E _ _ _ _ _ _
D _ _ A _ _ _ _ _ _ _ _ _ _ _
I _ _ N _ Z _ _ _ _ _ _ _ _ _
C R O A K I N G _ _ _ _ _ _ _
T _ _ T _ N _ U _ _ _ _ _ _ _
_ _ _ _ _ G _ M _ _ _ _ J A W
_ A L L E Y _ M E G A P O D _
_ _ _ _ _ _ _ Y _ _ _ _ _ _ _
    "###;

    let board = Board::try_from(board_string).unwrap();

    let wordlist = read_wordlist(format!(
        "{}/wordlists/NWL2020.txt",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    ))
    .unwrap();

    let hand = Hand::try_from(['E', 'E', 'I', 'N', 'R', 'V', 'V']).unwrap();

    let result = board.find_best_play(&wordlist, hand).unwrap();

    let expected_score = 75;
    let expected_play = Play {
        word: String::from("VINDICTIVE"),
        tiles: vec![
            Tile::new('V', Coordinates::new(5, 0), false),
            Tile::new('I', Coordinates::new(12, 0), false),
            Tile::new('V', Coordinates::new(13, 0), false),
            Tile::new('E', Coordinates::new(14, 0), false),
        ],
        hand: Hand::try_from(['E', 'N', 'R']).unwrap(),
        orientation: Orientation::Vertical,
        start_coordinates: Coordinates::new(5, 0),
        end_coordinates: Coordinates::new(14, 0),
    };

    let expected = (expected_score, expected_play);

    assert_eq!(result, expected);
}

#[test]
fn puzzle3() {
    let board_string = r###"
_ _ _ _ _ _ _ M E A D O W _ _
_ _ _ _ _ B _ _ _ _ _ _ A _ _
_ _ _ _ _ E _ _ _ _ _ _ I _ _
_ _ _ _ _ G _ _ _ _ F A V O R
_ _ _ _ Y A _ _ _ _ L _ E _ _
_ _ _ _ U N _ _ _ J O _ R _ _
_ _ _ _ A _ _ _ _ O W T S _ _
_ _ _ Q U I V E R Y _ H _ _ _
_ _ _ U _ _ _ _ _ _ _ I _ _ _
_ _ R O M A N C E D _ N _ _ _
_ _ _ T _ _ _ _ _ _ _ N _ _ _
_ _ _ E _ _ _ _ _ _ _ E _ _ _
_ _ _ _ _ _ _ _ _ _ _ R _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
    "###;

    let board = Board::try_from(board_string).unwrap();

    let wordlist = read_wordlist(format!(
        "{}/wordlists/NWL2020.txt",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    ))
    .unwrap();

    let hand = Hand::try_from(['A', 'D', 'H', 'O', 'R', 'T', 'X']).unwrap();

    let result = board.find_best_play(&wordlist, hand).unwrap();

    let expected_score = 68;
    let expected_play = Play {
        word: String::from("THORAX"),
        tiles: vec![
            Tile::new('T', Coordinates::new(1, 8), false),
            Tile::new('H', Coordinates::new(1, 9), false),
            Tile::new('O', Coordinates::new(1, 10), false),
            Tile::new('R', Coordinates::new(1, 11), false),
            Tile::new('X', Coordinates::new(1, 13), false),
        ],
        hand: Hand::try_from(['A', 'D']).unwrap(),
        orientation: Orientation::Horizontal,
        start_coordinates: Coordinates::new(1, 8),
        end_coordinates: Coordinates::new(1, 13),
    };

    let expected = (expected_score, expected_play);

    assert_eq!(result, expected);
}
