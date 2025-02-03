#ifndef SCRABBLE_H
#define SCRABBLE_H

#include <stdlib.h>
#include <stdbool.h>

#include "board-word.h"
#include "play.h"
#include "board-play-tiles.h"
#include "vector.h"
#include "hashset.h"

#define BOARD_SIZE 15
#define HAND_SIZE 7
#define SET_CAPACITY 256
#define WORDLIST_PATH "./wordlist.txt"

enum board_tile {
    Empty,
    DoubleLetter,
    TripleLetter,
    DoubleWord,
    TripleWord,
};

struct pair {
    void *first;
    void *second;
};

extern const enum board_tile bonus_board[BOARD_SIZE][BOARD_SIZE];

/**
   This function takes a wordlist, in other terms, a string of words seperated
   by newline characters and creates a hashset containing all the words found.
*/
struct hashset *build_wordlist_set(const char *wordlist);

/**
   This function creates a new `board_word` using the chars from the given board
   starting at index [i,j] in the orientation is_horizontal. 
*/
struct board_word *create_board_word_from_index(const char board[BOARD_SIZE][BOARD_SIZE],
                                                int i,
                                                int j,
                                                bool is_horizontal);

/**
   This function finds all words in the given board. Words are sequences of 2 or
   more contiguous characters horizontally or vertically. This function makes no
   validation with regards to the words found. If they are present on the
   board, they are assumed to be valid and will be collected. The result is a
   vector of `board_word` structs.
*/
struct vector *find_board_words(const char board[BOARD_SIZE][BOARD_SIZE]);

/**
   This function takes a `board_word` bw and returns a regex string that can be
   used against a wordlist to find a word extension. Word extensions are new
   valid words built by adding characters to the begining, end or both ends of
   a word. The extensions built from the returned regex string are not
   necessarily valid and need to be validated to make sure they do not create
   new invalid words perpendicular to the newly added characters.
*/
char *build_extension_regex_string(const char board[BOARD_SIZE][BOARD_SIZE],
                                   const char *hand,
                                   struct board_word *bw);

/**
   TODO
*/
char *build_hook_regex_string(const char board[BOARD_SIZE][BOARD_SIZE],
                              const char *hand,
                              struct play *play);

/**
   TODO
*/
char *build_perpendicular_regex_string(const char board[BOARD_SIZE][BOARD_SIZE],
                                       const char *hand,
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
                        const char *hand,
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
                   const char *hand,
                   struct play *play);

/**
   This function finds all extension plays on a board for the board_word bw. An
   extension play is a new word containing the word bw as a substring. The
   extension can be a prefix, a suffix or both at the same time. A vector of plays
   is returned containing all plays found.
*/
struct vector *find_extension_plays(const char board[BOARD_SIZE][BOARD_SIZE],
                                    const char *wordlist,
                                    struct hashset *wordlist_set,
                                    const char *hand,
                                    struct board_word *bw);

/**
   TODO
*/
struct vector *find_hook_plays(const char board[BOARD_SIZE][BOARD_SIZE],
                               const char *wordlist,
                               struct hashset *wordlist_set,
                               const char *hand,
                               struct play *play);

/**
   TODO
*/
struct vector *find_perpendicular_plays(const char board[BOARD_SIZE][BOARD_SIZE],
                                        const char *wordlist,
                                        struct hashset *wordlist_set,
                                        const char *hand,
                                        struct board_word *bw);

/**
   TODO
*/
struct vector *find_parallel_plays(const char board[BOARD_SIZE][BOARD_SIZE],
                                   struct hashset *wordlist_set,
                                   const char *hand,
                                   struct board_word *bw,
                                   struct board_play_tiles *one_tile_plays);

/**
   TODO
*/
struct pair *create_pair(void *first, void *second);

/**
   TODO
*/
int hand_index_of(const char *hand, char letter);

/**
   TODO
*/
void hand_remove_at(char *hand, int index);

/**
   TODO
*/
struct vector *find_plays(const char board[BOARD_SIZE][BOARD_SIZE],
                          const char *wordlist,
                          struct hashset *wordlist_set,
                          const char *hand,
                          struct vector *words);

#endif
