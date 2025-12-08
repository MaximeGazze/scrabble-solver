mod scrabble;

use clap::Parser;
use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    panic,
    path::{Path, PathBuf},
};

use scrabble::{Board, CoordinatesIterator, Hand, Tile};

fn read_board_string(s: String) -> Board {
    let mut board = Board::new();

    let uppercase = s.to_uppercase();
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
                    panic!("board is too long");
                };

                board.insert_tile(Tile::new(c, coordinates, false))
            }
            '*' => {
                let Some(coordinates) = coordinates_iterator.next() else {
                    panic!("board is too long");
                };

                let Some(next_c) = chars.next() else {
                    panic!("wilcard is missing it's letter");
                };

                if !('A'..='Z').contains(&next_c) {
                    panic!("board: invalid character {}", c);
                }

                board.insert_tile(Tile::new(next_c, coordinates, true))
            }
            _ => {
                panic!("board: invalid character {}", c);
            }
        }
    }

    board
}

fn read_board_file<P>(path: P) -> Result<Board, io::Error>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut file_contents = String::new();

    let _ = file.read_to_string(&mut file_contents);

    Ok(read_board_string(file_contents))
}

fn read_wordlist<P>(path: P) -> Result<HashSet<String>, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;

    BufReader::new(file).lines().into_iter().collect()
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(flatten)]
    board: BoardGroup,

    /// Wordlist file path
    #[clap(short, long)]
    wordlist: PathBuf,

    /// String describing a hand
    #[clap(short = 'H', long)]
    hand: String,
}

#[derive(Debug, clap::Args)]
#[group(required = true, multiple = false)]
struct BoardGroup {
    /// String describing a board
    #[arg(short = 's', long)]
    board_string: Option<String>,

    /// File containing a board
    #[arg(short, long)]
    board_file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let board = if let Some(s) = args.board.board_string {
        read_board_string(s)
    } else if let Some(path) = args.board.board_file {
        read_board_file(path).unwrap_or_else(|error| panic!("board: {}", error))
    } else {
        panic!("missing board argument");
    };

    let wordlist =
        read_wordlist(args.wordlist).unwrap_or_else(|error| panic!("wordlist: {}", error));

    let hand = Hand::try_from(args.hand).unwrap_or_else(|error| panic!("hand: {}", error));

    let plays = board.find_possible_plays(&wordlist, hand);

    let mut scores = plays
        .iter()
        .map(|play| (board.score_play(play), play.word.clone()))
        .collect::<Vec<_>>();

    scores.sort_by(|a, b| a.0.cmp(&b.0));

    for (word, score) in scores {
        println!("{} {}", word, score);
    }
}
