mod scrabble;

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
    // let mut board = Board::new();
    // let hand = vec!['A', 'D', 'N', 'D', 'E', 'I', 'G'];
    //
    // let tiles = [
    //     Tile::new('F', Coordinates::new(3, 2), false),
    //     Tile::new('U', Coordinates::new(3, 3), false),
    //     Tile::new('C', Coordinates::new(3, 4), false),
    //     Tile::new('K', Coordinates::new(3, 5), false),
    //     Tile::new('Y', Coordinates::new(4, 0), false),
    //     Tile::new('O', Coordinates::new(4, 1), false),
    //     Tile::new('U', Coordinates::new(4, 2), false),
    //     Tile::new('N', Coordinates::new(5, 2), false),
    // ];
    //
    // tiles.into_iter().for_each(|tile| board.insert_tile(tile));
    //
    // let wordlist = read_wordlist("wordlist.txt");

    // board.tiles().for_each(|it| println!("{:?}", it));

    // board.plays().iter().for_each(|it| println!("{:?}", it));

    // let board_words = board.board_words();
    // let board_word = board_words.get(0).unwrap();

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

    // let perpendicular_plays = board.find_perpendicular_plays(board_word, hand, &wordlist);
    // perpendicular_plays
    //     .iter()
    //     .for_each(|it| println!("{}", it.word));
    // perpendicular_plays
    //     .iter()
    //     .for_each(|it| println!("{:?}", it));

    // let parallel_plays: Vec<_> = perpendicular_plays
    //     .iter()
    //     .filter(|play| play.len() == 1)
    //     .flat_map(|play| board.find_parallel_plays(play, &wordlist))
    //     .collect();
    // parallel_plays.iter().for_each(|it| println!("{}", it.word));
    // parallel_plays.iter().for_each(|it| println!("{:?}", it));

    // let plays = board.find_possible_plays(&wordlist, &hand);
    //
    // let mut score_plays: Vec<_> = plays
    //     .iter()
    //     .map(|play| (play, board.score_play(play)))
    //     .collect();
    //
    // score_plays.sort_by(|(_, score), (_, other_score)| score.cmp(other_score));
    //
    // score_plays.into_iter().for_each(|x| println!("{:?}", x));

    // plays.iter().for_each(|it| println!("{}", it.word));
    // plays.iter().for_each(|it| println!("{:?}", it));

    // println!("{}", board);
}
