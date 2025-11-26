mod scrabble;

use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use scrabble::{Board, Hand};

fn read_wordlist<P>(path: P) -> Result<HashSet<String>, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;

    BufReader::new(file).lines().into_iter().collect()
}

fn main() {
    let board = Board::new();
    let wordlist = read_wordlist("wordlist.txt").unwrap();
    let hand = Hand::new();
    board.find_possible_plays(&wordlist, hand);
}
