#ifndef SCRABBLE_H
#define SCRABBLE_H

#include <stdlib.h>
#include <stdbool.h>

#include "vector.h"
#include "hashset.h"

#define BOARD_SIZE 15
#define HAND_SIZE 7
#define MAX_WORD_LENGTH 16
#define SET_CAPACITY 256
#define WORDLIST_PATH "./wordlist.txt"

enum board_tile {
    Empty,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
};

struct board_word {
    char *word;
    int length;
    int i;
    int j;
    bool is_horizontal;
};

struct play {
    char *word;
    struct play_tile *tiles;
    size_t size;
    bool is_horizontal;
};

struct play_tile {
    char letter;
    int i;
    int j;
    bool is_wildcard;
};

const enum board_tile bonus_board[BOARD_SIZE][BOARD_SIZE] = {
    {TripleWord, Empty, Empty, DoubleLetter, Empty, Empty, Empty, TripleWord, Empty, Empty, Empty, DoubleLetter, Empty, Empty, TripleWord},
    {Empty, DoubleWord, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, DoubleWord, Empty},
    {Empty, Empty, DoubleWord, Empty, Empty, Empty, DoubleLetter, Empty, DoubleLetter, Empty, Empty, Empty, DoubleWord, Empty, Empty},
    {DoubleLetter, Empty, Empty, DoubleWord, Empty, Empty, Empty, DoubleLetter, Empty, Empty, Empty, DoubleWord, Empty, Empty, DoubleLetter},
    {Empty, Empty, Empty, Empty, DoubleWord, Empty, Empty, Empty, Empty, Empty, DoubleWord, Empty, Empty, Empty, Empty},
    {Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty},
    {Empty, Empty, DoubleLetter, Empty, Empty, Empty, DoubleLetter, Empty, DoubleLetter, Empty, Empty, Empty, DoubleLetter, Empty, Empty},
    {TripleWord, Empty, Empty, DoubleLetter, Empty, Empty, Empty, DoubleWord, Empty, Empty, Empty, DoubleLetter, Empty, Empty, TripleWord},
    {Empty, Empty, DoubleLetter, Empty, Empty, Empty, DoubleLetter, Empty, DoubleLetter, Empty, Empty, Empty, DoubleLetter, Empty, Empty},
    {Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty},
    {Empty, Empty, Empty, Empty, DoubleWord, Empty, Empty, Empty, Empty, Empty, DoubleWord, Empty, Empty, Empty, Empty},
    {DoubleLetter, Empty, Empty, DoubleWord, Empty, Empty, Empty, DoubleLetter, Empty, Empty, Empty, DoubleWord, Empty, Empty, DoubleLetter},
    {Empty, Empty, DoubleWord, Empty, Empty, Empty, DoubleLetter, Empty, DoubleLetter, Empty, Empty, Empty, DoubleWord, Empty, Empty},
    {Empty, DoubleWord, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, TripleLetter, Empty, Empty, Empty, DoubleWord, Empty},
    {TripleWord, Empty, Empty, DoubleLetter, Empty, Empty, Empty, TripleWord, Empty, Empty, Empty, DoubleLetter, Empty, Empty, TripleWord},
};

/**
   This function takes a wordlist, in other terms, a string of words seperated
   by newline characters and creates a hashset containing all the words found.
 */
struct hashset *build_wordlist_set(const char *wordlist);

/**
   This function finds all words in the given board. Words are sequences of 2 or
   more contiguous characters horizontally or vertically. This function makes no
   validation with regards to the words found. If they are present on the
   board, they are assumed to be valid and will be collected.
 */
struct vec *find_all_words(const char board[BOARD_SIZE][BOARD_SIZE]);

/**
   This function takes a board_word bw and returns a regex string that can be
   used against a wordlist to find a word extension. Word extensions are new
   valid words built by adding characters to the begining, end or both ends of
   a word. The extensions built from the returned regex string are not
   necessarily valid and need to be validated to make sure they do not create
   new invalid words perpendicular to the newly added characters.
 */
char *build_extension_regex_string(const char board[BOARD_SIZE][BOARD_SIZE],
                                   const char hand[HAND_SIZE],
                                   struct board_word *bw);

/**
   TODO
 */
char *build_hook_regex_string(const char board[BOARD_SIZE][BOARD_SIZE],
                              const char hand[HAND_SIZE],
                              struct play *play);

/**
   TODO
*/
char *build_perpendicular_regex_string(const char board[BOARD_SIZE][BOARD_SIZE],
                                       const char hand[HAND_SIZE],
                                       struct board_word *bw,
                                       int index);

/**
   This function takes a board_word bw not present on the board and and creates
   a play that contains the tiles needed for this word to be present on the
   board. The word is also validated against the wordlist_set. If the play is
   invalid due to using too many tiles or creating an invalid word, a NULL
   pointer is returned.
*/
struct play *build_play(const char board[BOARD_SIZE][BOARD_SIZE],
                        struct hashset *wordlist_set,
                        const char hand[HAND_SIZE],
                        struct board_word *bw);

/**
   This function is used to free any plays created with malloc.
*/
void free_play(struct play *p);

/**
   This function is used to free any board_words created with malloc.
*/
void free_board_word(struct board_word *bw);

/**
   TODO
*/
bool validate_play_tile_horizontally(const char board[BOARD_SIZE][BOARD_SIZE],
                                     struct hashset *wordlist_set,
                                     struct play_tile *tile);

/**
   TODO
*/
bool validate_play_tile_vertically(const char board[BOARD_SIZE][BOARD_SIZE],
                                   struct hashset *wordlist_set,
                                   struct play_tile *tile);

/**
   TODO
*/
bool validate_play_horizontally(const char board[BOARD_SIZE][BOARD_SIZE],
                                struct hashset *wordlist_set,
                                struct play *play);

/**
   TODO
*/
bool validate_play_vertically(const char board[BOARD_SIZE][BOARD_SIZE],
                              struct hashset *wordlist_set,
                              struct play *play);

/**
   This function takes a wordlist, a hand, a play and validates that the words
   created in all orientations on the board by the play are contained in the
   wordlist. If the play is valid, true is returned, otherwise false.
*/
bool validate_play(const char board[BOARD_SIZE][BOARD_SIZE],
                   struct hashset *wordlist_set,
                   const char hand[HAND_SIZE],
                   struct play *play);

/**
   This function finds all extension plays on a board for the board_word bw. An
   extension play is a new word containing the word bw as a substring. The
   extension can be a prefix, a suffix or both at the same time. A vec of plays
   is returned containing all plays found.
*/
struct vec *find_all_extensions(const char board[BOARD_SIZE][BOARD_SIZE],
                                const char *wordlist,
                                struct hashset *wordlist_set,
                                const char hand[HAND_SIZE],
                                struct board_word *bw);

/**
   TODO
*/
struct vec *find_all_hooks(const char board[BOARD_SIZE][BOARD_SIZE],
                           const char *wordlist,
                           struct hashset *wordlist_set,
                           const char hand[HAND_SIZE],
                           struct play *play);

/**
   TODO
*/
struct vec *find_all_perpendiculars(const char board[BOARD_SIZE][BOARD_SIZE],
                                    const char *wordlist,
                                    struct hashset *wordlist_set,
                                    const char hand[HAND_SIZE],
                                    struct board_word *bw);

/**
   TODO
*/
struct vec *find_all_parallels(const char board[BOARD_SIZE][BOARD_SIZE],
                               const char *wordlist);

#endif
