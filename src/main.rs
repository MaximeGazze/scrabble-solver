use clap::Parser;
use scrabble_solver::{read_board_file, read_wordlist, Board, Hand};

use std::{panic, path::PathBuf};

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
#[group(required = false, multiple = false)]
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
        Board::try_from(s).unwrap_or_else(|error| panic!("board: {}", error))
    } else if let Some(path) = args.board.board_file {
        read_board_file(path).unwrap_or_else(|error| panic!("board: {}", error))
    } else {
        Board::new()
    };

    let wordlist =
        read_wordlist(args.wordlist).unwrap_or_else(|error| panic!("wordlist: {}", error));

    let hand = Hand::try_from(args.hand).unwrap_or_else(|error| panic!("hand: {}", error));

    // let plays = board.find_possible_plays(&wordlist, hand);
    //
    // let best_play = plays
    //     .iter()
    //     .map(|play| (board.score_play(play), play))
    //     .max_by(|a, b| a.0.cmp(&b.0));

    let best_play = board.find_best_play(&wordlist, hand);

    if let Some((score, play)) = best_play {
        println!("Word: {} - Score: {}", play.word, score);
        board.print_play(&play);
    }
}
