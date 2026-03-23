# Scrabble Solver

A command line scrabble solver used to find the best possible play.

## Usage

To run the program, you will need a text board file, a wordlist and a hand provided as command line arguments.

```txt
Usage: scrabble-solver [OPTIONS] --wordlist <WORDLIST> --hand <HAND>

Options:
  -s, --board-string <BOARD_STRING>  String describing a board
  -b, --board-file <BOARD_FILE>      File containing a board
  -w, --wordlist <WORDLIST>          Wordlist file path
  -H, --hand <HAND>                  String describing a hand
  -h, --help                         Print help
  -V, --version                      Print version
```

When no board is provided, the program tries to find the best possible opening on an empty board.

Board text files must have the following format where `_` describes an empty tile, letters from `A` to `Z` describe a played tile and `*` followed by a letter from `A` to `Z` describes a wildcard tile with its associated letter. Note that the empty spaces and newlines are optional in board files.

```txt
_ _ _ _ _ _ _ W _ _ _ _ _ _ _
_ _ _ _ _ _ _ I _ _ _ _ _ _ _
_ _ _ _ _ _ _ N _ _ _ _ _ _ _
_ _ _ _ _ _ _ K _ _ _ _ _ _ _
_ _ _ _ _ _ _ E _ _ _ _ _ _ _
_ _ _ _ _ J U R E L _ _ _ _ _
_ _ _ _ _ *I_ _ _ _ _ _ _ _ _
_ _ _ _ W A R N _ _ _ _ _ _ _
_ _ _ _ _ O _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
```

## Example

Here is an example of running the program on the board shown previously:

```sh
./scrabble-solver -H "abcdefg" -b board.txt -w wordlists/NWL2020.txt
```

And here is the output you would receive:

```txt
Word: FACED - Score: 28
_ _ _ _ _ _ _ W _ _ _ _ _ _ _
_ _ _ _ _ _ _ I _ _ _ _ _ _ _
_ _ _ _ _ _ _ N _ _ _ _ _ _ _
_ _ _ _ _ _ _ K _ _ _ _ _ _ _
_ _ _ _ _ _ _ E _ _ _ _ _ _ _
_ _ _ _ _ J U R E L _ _ _ _ _
_ _ _ _ _ I _ _ F A C E D _ _
_ _ _ _ W A R N _ _ _ _ _ _ _
_ _ _ _ _ O _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
_ _ _ _ _ _ _ _ _ _ _ _ _ _ _
```
