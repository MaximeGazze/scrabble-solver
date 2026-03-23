use std::{
    collections::HashSet,
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
    path::Path,
};

use crate::Board;

pub fn read_board_file<P>(path: P) -> Result<Board, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut file = File::open(path)?;
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents)?;

    Ok(Board::try_from(file_contents)?)
}

pub fn read_wordlist<P>(path: P) -> Result<HashSet<String>, io::Error>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;

    BufReader::new(file).lines().into_iter().collect()
}
