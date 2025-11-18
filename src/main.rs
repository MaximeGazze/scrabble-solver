mod scrabble;

use scrabble::{Board, Coordinates, Tile};
use std::{collections::HashSet, fs, path::Path};

fn read_wordlist<P>(path: P) -> HashSet<String>
where
    P: AsRef<Path>,
{
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn main() {
    let mut board = Board::new();
    let hand = vec!['A', 'D', 'N', 'D', 'E', 'I', 'G'];

    board.tiles[3][2] = Some(Tile {
        letter: 'F',
        coordinates: Coordinates { i: 3, j: 2 },
        wildcard: false,
    });

    board.tiles[3][3] = Some(Tile {
        letter: 'U',
        coordinates: Coordinates { i: 3, j: 3 },
        wildcard: false,
    });

    board.tiles[3][4] = Some(Tile {
        letter: 'C',
        coordinates: Coordinates { i: 3, j: 4 },
        wildcard: false,
    });

    board.tiles[3][5] = Some(Tile {
        letter: 'K',
        coordinates: Coordinates { i: 3, j: 5 },
        wildcard: false,
    });

    board.tiles[4][0] = Some(Tile {
        letter: 'Y',
        coordinates: Coordinates { i: 4, j: 0 },
        wildcard: false,
    });

    board.tiles[4][1] = Some(Tile {
        letter: 'O',
        coordinates: Coordinates { i: 4, j: 1 },
        wildcard: false,
    });

    board.tiles[4][2] = Some(Tile {
        letter: 'U',
        coordinates: Coordinates { i: 4, j: 2 },
        wildcard: false,
    });

    board.tiles[5][2] = Some(Tile {
        letter: 'N',
        coordinates: Coordinates { i: 5, j: 2 },
        wildcard: false,
    });

    let wordlist = read_wordlist("wordlist.txt");

    // board.tiles().for_each(|it| println!("{:?}", it));

    // board.plays().iter().for_each(|it| println!("{:?}", it));

    let board_words = board.board_words();
    let board_word = board_words.get(0).unwrap();

    // let extension_plays = board.find_extension_plays(board_word, hand, &wordlist);
    // extension_plays
    //     .iter()
    //     .for_each(|it| println!("{}", it.word));
    // extension_plays.iter().for_each(|it| println!("{:?}\n", it));

    // let play = extension_plays.get(7).unwrap();
    // let play = scrabble::Play {
    //     word: String::from("FUNG"),
    //     tiles: vec![Tile {
    //         letter: 'G',
    //         coordinates: Coordinates { i: 6, j: 2 },
    //         wildcard: false,
    //     }],
    //     hand: vec!['A', 'D', 'N', 'D', 'E', 'I'],
    //     orientation: scrabble::Orientation::Vertical,
    // };
    // println!("{:?}", play);

    // let hook_plays = board.find_hook_plays(&play, &wordlist);
    // hook_plays.iter().for_each(|it| println!("{}", it.word));
    // hook_plays
    //     .iter()
    //     .for_each(|it| println!("{} {:?}", it.word, it.tiles.first().unwrap().coordinates));
    // hook_plays.iter().for_each(|it| println!("{:?}\n", it));

    let perpendicular_plays = board.find_perpendicular_plays(board_word, hand, &wordlist);
    // perpendicular_plays
    //     .iter()
    //     .for_each(|it| println!("{}", it.word));
    perpendicular_plays
        .iter()
        .for_each(|it| println!("{:?}", it));

    // let plays = board.find_possible_plays(&wordlist, &hand);

    // plays.iter().for_each(|it| println!("{}", it.word));
    // plays.iter().for_each(|it| println!("{:?}", it));

    println!("{}", board);
}
