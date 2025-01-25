#include "scrabble.h"

#include <stdio.h>
#include <string.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <regex.h>
#include <assert.h>

extern const enum board_tile bonus_board[BOARD_SIZE][BOARD_SIZE];

struct hashset *build_wordlist_set(const char *wordlist) {
    struct hashset *set = hashset_create(SET_CAPACITY);

    char buffer[BOARD_SIZE + 1];
    size_t buffer_index = 0;
    const char *current_char = wordlist;

    while (*current_char) {
        if (*current_char == '\n') {
            buffer[buffer_index] = '\0';
            hashset_insert(set, buffer);
            buffer_index = 0;
        } else {
            buffer[buffer_index++] = *current_char;
        }

        current_char++;
    }

    return set;
}

struct vec *find_all_words(const char board[BOARD_SIZE][BOARD_SIZE]) {
    struct vec *words = vec_initialize();

    for (size_t i = 0; i < BOARD_SIZE; i++) {
        for (size_t j = 0; j < BOARD_SIZE; j++) {
            if (board[i][j] != '\0') {
                // check for new vertical word
                if ((i == 0 && board[i + 1][j] != '\0') ||
                    (i != 0 && i < BOARD_SIZE - 1 && board[i - 1][j] == '\0' && board[i + 1][j] != '\0')) {
                    struct board_word *bw = malloc(sizeof(struct board_word));
                    if (bw == NULL) {
                        perror("malloc");
                        exit(EXIT_FAILURE);
                    }

                    bw->word = malloc(MAX_WORD_LENGTH + 1);
                    if (bw->word == NULL) {
                        perror("malloc");
                        exit(EXIT_FAILURE);
                    }

                    bw->length = 0;
                    bw->i = i;
                    bw->j = j;
                    bw->is_horizontal = false;

                    size_t temp_i = i;
                    while (temp_i < BOARD_SIZE) {
                        if (board[temp_i][j] == '\0') {
                            bw->word[bw->length] = '\0';
                            break;
                        }

                        bw->word[temp_i - i] = board[temp_i][j];
                        temp_i++;
                        bw->length++;
                    }

                    vec_add(words, bw);
                }

                // check for new horizontal word
                if ((j == 0 && board[i][j + 1] != '\0') ||
                    (j != 0 && j < BOARD_SIZE - 1 && board[i][j - 1] == '\0' && board[i][j + 1] != '\0')) {
                    struct board_word *bw = malloc(sizeof(struct board_word));
                    if (bw == NULL) {
                        perror("malloc");
                        exit(EXIT_FAILURE);
                    }

                    bw->word = malloc(MAX_WORD_LENGTH + 1);
                    if (bw->word == NULL) {
                        perror("malloc");
                        exit(EXIT_FAILURE);
                    }

                    bw->length = 0;
                    bw->i = i;
                    bw->j = j;
                    bw->is_horizontal = true;

                    size_t temp_j = j;
                    while (temp_j < BOARD_SIZE) {
                        if (board[i][temp_j] == '\0') {
                            bw->word[bw->length] = '\0';
                            break;
                        }

                        bw->word[temp_j - j] = board[i][temp_j];
                        temp_j++;
                        bw->length++;
                    }

                    vec_add(words, bw);
                }
            }
        }
    }

    return words;
}

char *build_extension_regex_string(const char board[BOARD_SIZE][BOARD_SIZE],
                                   const char hand[HAND_SIZE],
                                   struct board_word *bw) {
    char buffer[512];
    size_t buffer_index = 0;
    bool is_horizontal = bw->is_horizontal;
    bool do_prefix = is_horizontal ? bw->j != 0 : bw->i != 0;
    bool do_suffix = is_horizontal ? (bw->j + bw->length < BOARD_SIZE) : (bw->i + bw->length < BOARD_SIZE);
    bool hand_has_wildcard = strchr(hand, '*') != NULL;

    buffer[buffer_index++] = '^';

    if (do_prefix) {
        if (is_horizontal) {
            int i = bw->i;
            size_t start_index = bw->j > (HAND_SIZE - 1) ? bw->j - (HAND_SIZE - 1) : 0;

            // add the opening parentheses for optional blocks
            for (int j = start_index; j < bw->j; j++) {
                // if the character is null, we know an optional block will be needed
                if (board[i][j] == '\0') {
                    buffer[buffer_index++] = '(';
                }
            }

            // add characters to regex
            for (int j = start_index; j < bw->j; j++) {
                // if the character is null, we add a wildcard with the current hand
                if (board[i][j] == '\0') {
                    if (hand_has_wildcard) {
                        buffer[buffer_index++] = '.';
                    } else {
                        buffer[buffer_index++] = '[';
                        strncpy(buffer + buffer_index, hand, HAND_SIZE);
                        buffer_index += HAND_SIZE;
                        buffer[buffer_index++] = ']';
                    }

                    buffer[buffer_index++] = ')';
                    buffer[buffer_index++] = '?';
                } else {
                    buffer[buffer_index++] = board[i][j];
                }
            }
        } else {
            int j = bw->j;
            int start_index = bw->i > (HAND_SIZE - 1) ? bw->i - (HAND_SIZE - 1) : 0;

            // add the opening parentheses for optional blocks
            for (int i = start_index; i < bw->i; i++) {
                // if the character is null, we know an optional block will be needed
                if (board[i][j] == '\0') {
                    buffer[buffer_index++] = '(';
                }
            }

            // add characters to regex
            for (int i = start_index; i < bw->i; i++) {
                // if the character is null, we add a wildcard with the current hand
                if (board[i][j] == '\0') {
                    if (hand_has_wildcard) {
                        buffer[buffer_index++] = '.';
                    } else {
                        buffer[buffer_index++] = '[';
                        strncpy(buffer + buffer_index, hand, HAND_SIZE);
                        buffer_index += HAND_SIZE;
                        buffer[buffer_index++] = ']';
                    }

                    buffer[buffer_index++] = ')';
                    buffer[buffer_index++] = '?';
                } else {
                    buffer[buffer_index++] = board[i][j];
                }
            }
        }
    }

    strncpy(buffer + buffer_index, bw->word, bw->length);
    buffer_index += bw->length;

    if (do_suffix) {
        if (is_horizontal) {
            int i = bw->i;
            int start_index = bw->j + bw->length;
            int end_index = start_index + HAND_SIZE - 1 < BOARD_SIZE ? start_index + HAND_SIZE - 1 : BOARD_SIZE - 1;

            // add characters to regex
            for (int j = start_index; j <= end_index; j++) {
                if (board[i][j] == '\0') {
                    buffer[buffer_index++] = '(';
                    if (hand_has_wildcard) {
                        buffer[buffer_index++] = '.';
                    } else {
                        buffer[buffer_index++] = '[';
                        strncpy(buffer + buffer_index, hand, HAND_SIZE);
                        buffer_index += HAND_SIZE;
                        buffer[buffer_index++] = ']';
                    }
                } else {
                    buffer[buffer_index++] = board[i][j];
                }
            }

            // add closing parentheses and '?' for optional blocks
            for (int j = start_index; j <= end_index; j++) {
                if (board[i][j] == '\0') {
                    buffer[buffer_index++] = ')';
                    buffer[buffer_index++] = '?';
                }
            }
        } else {
            int j = bw->j;
            int start_index = bw->i + bw->length;
            int end_index = start_index + HAND_SIZE - 1 < BOARD_SIZE ? start_index + HAND_SIZE - 1 : BOARD_SIZE - 1;

            // add characters to regex
            for (int i = start_index; i <= end_index; i++) {
                if (board[i][j] == '\0') {
                    buffer[buffer_index++] = '(';
                    if (hand_has_wildcard) {
                        buffer[buffer_index++] = '.';
                    } else {
                        buffer[buffer_index++] = '[';
                        strncpy(buffer + buffer_index, hand, HAND_SIZE);
                        buffer_index += HAND_SIZE;
                        buffer[buffer_index++] = ']';
                    }
                } else {
                    buffer[buffer_index++] = board[i][j];
                }
            }

            // add closing parentheses and '?' for optional blocks
            for (int i = start_index; i <= end_index; i++) {
                if (board[i][j] == '\0') {
                    buffer[buffer_index++] = ')';
                    buffer[buffer_index++] = '?';
                }
            }
        }
    }

    buffer[buffer_index++] = '$';
    buffer[buffer_index++] = '\0';

    char *regex_string = malloc(buffer_index);
    if (regex_string == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    strncpy(regex_string, buffer, buffer_index);

    return regex_string;
}

char *build_hook_regex_string(const char board[BOARD_SIZE][BOARD_SIZE],
                              const char hand[HAND_SIZE],
                              struct play *play) {
    char buffer[512];
    size_t buffer_index = 0;
    bool is_horizontal = !play->is_horizontal;
    bool hand_has_wildcard = strchr(hand, '*') != NULL;
    struct play_tile pt = play->tiles[0];

    int i = pt.i;
    int j = pt.j;

    int start_index = is_horizontal ? j : i;
    int end_index = is_horizontal ? j : i;

    // rewind as far back as possible
    while (start_index > 0) {
        // if we rewinded HAND_SIZE -1 times, we break
        if ((is_horizontal && j - start_index == HAND_SIZE - 1)
            || (!is_horizontal && i - start_index == HAND_SIZE - 1)) {
            break;
        }

        // if we encounter a character that is non null, we add a padding of 1 between it and break
        if ((is_horizontal && board[i][start_index] != '\0')
            || (!is_horizontal && board[start_index][j] != '\0')) {
            start_index += 2;
            break;
        }

        start_index--;
    }

    // we go forward as far as possible
    while (end_index < BOARD_SIZE - 1) {
        // if we went forward HAND_SIZE - 1 times, we break
        if ((is_horizontal && end_index - j == HAND_SIZE - 1)
            || (!is_horizontal && end_index - i == HAND_SIZE - 1)) {
            break;
        }

        // if we encounter a character that is non null, we add a padding of 1 between it and break
        if ((is_horizontal && board[i][end_index] != '\0')
            || (!is_horizontal && board[end_index][j] != '\0')) {
            end_index -= 2;
            break;
        }

        end_index++;
    }

    int before_count = (is_horizontal ? j : i) - start_index;
    int after_count = end_index - (is_horizontal ? j : i);

    buffer[buffer_index++] = '^';

    // write prefix
    for (int count = 0; count < before_count; count++) {
        buffer[buffer_index++] = '(';
    }

    for (int count = 0; count < before_count; count++) {
        if (hand_has_wildcard) {
            buffer[buffer_index++] = '.';
        } else {
            buffer[buffer_index++] = '[';
            strncpy(buffer + buffer_index, hand, HAND_SIZE);
            buffer_index += HAND_SIZE;
            buffer[buffer_index++] = ']';
        }

        buffer[buffer_index++] = ')';
        buffer[buffer_index++] = '?';
    }

    buffer[buffer_index++] = pt.letter;

    // write suffix
    for (int count = 0; count < after_count; count++) {
        buffer[buffer_index++] = '(';
    }

    for (int count = 0; count < after_count; count++) {
        if (hand_has_wildcard) {
            buffer[buffer_index++] = '.';
        } else {
            buffer[buffer_index++] = '[';
            strncpy(buffer + buffer_index, hand, HAND_SIZE);
            buffer_index += HAND_SIZE;
            buffer[buffer_index++] = ']';
        }

        buffer[buffer_index++] = ')';
        buffer[buffer_index++] = '?';
    }

    buffer[buffer_index++] = '$';
    buffer[buffer_index++] = '\0';

    char *regex_string = malloc(buffer_index);
    if (regex_string == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    strncpy(regex_string, buffer, buffer_index);

    return regex_string;
}

char *build_perpendicular_regex_string(const char board[BOARD_SIZE][BOARD_SIZE],
                                       const char hand[HAND_SIZE],
                                       struct board_word *bw,
                                       int index) {
    char buffer[512];
    size_t buffer_index = 0;
    bool is_horizontal = !bw->is_horizontal;
    bool hand_has_wildcard = strchr(hand, '*') != NULL;

    int letter_i = bw->is_horizontal ? bw->i : bw->i + index;
    int letter_j = bw->is_horizontal ? bw->j + index : bw->j;

    /* int i = bw->is_horizontal ? bw->i : bw->i + index; */
    /* int j = bw->is_horizontal ? bw->j + index : bw->j; */

    int start_index = is_horizontal ? letter_j : letter_i;
    int end_index = is_horizontal ? letter_j : letter_i;

    // rewind as far back as possible
    while (start_index > 0) {
        // if we rewinded HAND_SIZE times, we break
        if ((is_horizontal && letter_j - start_index == HAND_SIZE)
            || (!is_horizontal && letter_i - start_index == HAND_SIZE)) {
            break;
        }

        // if we encounter another word in the same orientation, we add a padding of 1 between it and break
        if (start_index > 0
            && ((is_horizontal && board[letter_i][start_index] != '\0' && board[letter_i][start_index - 1] != '\0')
                || (!is_horizontal && board[start_index][letter_j] != '\0' && board[start_index - 1][letter_j] != '\0'))) {
            start_index += 2;
            break;
        }

        start_index--;
    }

    // we go forward as far as possible
    while (end_index < BOARD_SIZE - 1) {
        // if we went forward HAND_SIZE - 1 times, we break
        if ((is_horizontal && end_index - letter_j == HAND_SIZE - 1)
            || (!is_horizontal && end_index - letter_i == HAND_SIZE - 1)) {
            break;
        }

        // if we encounter a character that is non null, we add a padding of 1 between it and break
        if (end_index < BOARD_SIZE - 1
            && ((is_horizontal && board[letter_i][end_index] != '\0' && board[letter_i][end_index + 1] != '\0')
                || (!is_horizontal && board[end_index][letter_j] != '\0' && board[end_index + 1][letter_j] != '\0'))) {
            end_index -= 2;
            break;
        }

        end_index++;
    }

    int start_i = is_horizontal ? letter_i : start_index;
    int start_j = is_horizontal ? start_index : letter_j;
    int i = start_i;
    int j = start_j;

    buffer[buffer_index++] = '^';

    // write prefix
    while ((!is_horizontal && i < letter_i) || (is_horizontal && j < letter_j)) {
        if (board[i][j] == '\0') {
            buffer[buffer_index++] = '(';
        }

        if (is_horizontal) {
            j++;
        } else {
            i++;
        }
    }

    i = start_i;
    j = start_j;

    // write prefix
    while ((!is_horizontal && i < letter_i) || (is_horizontal && j < letter_j)) {
        // if the character is null, we add a wildcard with the current hand
        if (board[i][j] == '\0') {
            if (hand_has_wildcard) {
                buffer[buffer_index++] = '.';
            } else {
                buffer[buffer_index++] = '[';
                strncpy(buffer + buffer_index, hand, HAND_SIZE);
                buffer_index += HAND_SIZE;
                buffer[buffer_index++] = ']';
            }

            buffer[buffer_index++] = ')';
            buffer[buffer_index++] = '?';
        } else {
            buffer[buffer_index++] = board[i][j];
        }

        if (is_horizontal) {
            j++;
        } else {
            i++;
        }
    }

    // write letter
    buffer[buffer_index++] = bw->word[index];

    i = is_horizontal ? letter_i : letter_i + 1;
    j = is_horizontal ? letter_j + 1 : letter_i;

    // write suffix
    while (i <= end_index && j <= end_index) {
        if (board[i][j] == '\0') {
            buffer[buffer_index++] = '(';
            if (hand_has_wildcard) {
                buffer[buffer_index++] = '.';
            } else {
                buffer[buffer_index++] = '[';
                strncpy(buffer + buffer_index, hand, HAND_SIZE);
                buffer_index += HAND_SIZE;
                buffer[buffer_index++] = ']';
            }
        } else {
            buffer[buffer_index++] = board[i][j];
        }

        if (is_horizontal) {
            j++;
        } else {
            i++;
        }
    }

    i = is_horizontal ? letter_i : letter_i + 1;
    j = is_horizontal ? letter_j + 1 : letter_i;

    while (i <= end_index && j <= end_index) {
        if (board[i][j] == '\0') {
            buffer[buffer_index++] = ')';
            buffer[buffer_index++] = '?';
        }

        if (is_horizontal) {
            j++;
        } else {
            i++;
        }
    }

    buffer[buffer_index++] = '$';
    buffer[buffer_index++] = '\0';

    char *regex_string = malloc(buffer_index);
    if (regex_string == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    strncpy(regex_string, buffer, buffer_index);

    return regex_string;
}

/* bool validate_word(const char *wordlist, const char *word) { */
/*     printf("Now validating word %s\n", word); // DEBUG */
/*     regex_t regex; */
/*     regmatch_t pmatch[1]; */

/*     char regex_string[BOARD_SIZE + 3]; */
/*     /\* char *regex_string = malloc(strlen(word) + 4); *\/ */
/*     /\* if (regex_string == NULL) { *\/ */
/*     /\*     perror("malloc"); *\/ */
/*     /\*     exit(EXIT_FAILURE); *\/ */
/*     /\* } *\/ */

/*     regex_string[0] = '^'; */
/*     strncpy(regex_string + 1, word, strlen(word)); */
/*     regex_string[strlen(word) + 1] = '$'; */
/*     regex_string[strlen(word) + 2] = '\0'; */

/*     printf("regex_string: %s\n", regex_string); // DEBUG */

/*     if (regcomp(&regex, regex_string, REG_NEWLINE | REG_EXTENDED)) { */
/*         fprintf(stderr, "could not compile regex\n"); */
/*         exit(EXIT_FAILURE); */
/*     } */

/*     bool is_valid; */

/*     int result = regexec(&regex, wordlist, 1, pmatch, 0); */
/*     if (result == 0) { */
/*         is_valid = true; */
/*         printf("Match found: %.*s\n", pmatch[0].rm_eo - pmatch[0].rm_so, wordlist + pmatch[0].rm_so); // DEBUG */
/*     } else if (result == REG_NOMATCH) { */
/*         is_valid = false; */
/*         printf("No match found\n"); // DEBUG */
/*     } else { */
/*         char msgbuf[100]; */
/*         regerror(result, &regex, msgbuf, sizeof(msgbuf)); */
/*         fprintf(stderr, "Regex match failed: %s\n", msgbuf); */
/*         exit(EXIT_FAILURE); */
/*     } */

/*     /\* free(regex_string); *\/ */
/*     regfree(&regex); */

/*     return is_valid; */
/* } */

struct play *build_play(const char board[BOARD_SIZE][BOARD_SIZE],
                        struct hashset *wordlist_set,
                        const char hand[HAND_SIZE],
                        struct board_word *bw) {
    /* printf("Now building play with bw: %s %d %d %d %c\n", bw->word, bw->length, bw->i, bw->j, bw->is_horizontal ? 'H' : 'V'); // DEBUG */

    struct play *play = malloc(sizeof(struct play));
    if (play == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    play->word = strdup(bw->word);

    play->tiles = malloc(sizeof(struct play_tile) * HAND_SIZE);
    if (play->tiles == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    play->is_horizontal = bw->is_horizontal;

    int play_index = 0;
    int i = bw->i;
    int j = bw->j;
    for (int bw_index = 0; bw_index < bw->length; bw_index++) {
        // if the tile is not present on the board, we add it to our play
        if (board[i][j] == '\0') {
            /* printf("Adding tile %c at board[%ld][%ld] play_index=%ld\n", bw->word[index], i, j, play_index); // DEBUG */
            if (play_index == HAND_SIZE) {
                free_play(play);
                return NULL;
            }

            play->tiles[play_index].letter = bw->word[bw_index];
            play->tiles[play_index].i = i;
            play->tiles[play_index].j = j;
            play->tiles[play_index].is_wildcard = false;

            play_index++;
        }

        if (bw->is_horizontal) {
            j++;
        } else {
            i++;
        }
    }

    play->size = play_index;

    if (!validate_play(board, wordlist_set, hand, play)) {
        free_play(play);
        return NULL;
    }

    return play;
}

void free_play(struct play *p) {
    free(p->word);
    free(p->tiles);
    free(p);
}

void free_board_word(struct board_word *bw) {
    free(bw->word);
    free(bw);
}

bool validate_play_tile_horizontally(const char board[BOARD_SIZE][BOARD_SIZE],
                                     struct hashset *wordlist_set,
                                     struct play_tile *tile) {
    char buffer[BOARD_SIZE + 1];
    int i = tile->i;
    int j = tile->j;

    // rewind to the start of the word on the board
    while (j != 0 && board[i][j - 1] != '\0') {
        j--;
    }

    // if there are no new words created by this tile, the tile is valid
    if (j == BOARD_SIZE - 1 || board[i][j + 1] == '\0') {
        return true;
    }

    size_t index = 0;

    // copy the word in the buffer
    while ((board[i][j] != '\0' || tile->j == j) && j < BOARD_SIZE) {
        if (tile->j == j) {
            buffer[index++] = tile->letter;
        } else {
            buffer[index++] = board[i][j];
        }

        j++;
    }

    buffer[index] = '\0';

    printf("validate_play_tile_horizontally word copied is: %s\n", buffer); // DEBUG

    return hashset_contains(wordlist_set, buffer);
}

bool validate_play_tile_vertically(const char board[BOARD_SIZE][BOARD_SIZE],
                                   struct hashset *wordlist_set,
                                   struct play_tile *tile) {
    char buffer[BOARD_SIZE + 1];
    int i = tile->i;
    int j = tile->j;

    // rewind to the start of the word on the board
    while (i != 0 && board[i - 1][j] != '\0') {
        i--;
    }

    // if there are no new words created by this tile, the tile is valid
    if (i == BOARD_SIZE - 1 || board[i + 1][j] == '\0') {
        return true;
    }

    size_t index = 0;

    // copy the word in the buffer
    while ((board[i][j] != '\0' || tile->i == i) && i < BOARD_SIZE) {
        if (tile->i == i) {
            buffer[index++] = tile->letter;
        } else {
            buffer[index++] = board[i][j];
        }

        i++;
    }

    buffer[index] = '\0';

    printf("validate_play_tile_vertically word copied is: %s\n", buffer); // DEBUG

    return hashset_contains(wordlist_set, buffer);
}

bool validate_play_horizontally(const char board[BOARD_SIZE][BOARD_SIZE],
                                struct hashset *wordlist_set,
                                struct play *play) {
    if (play->size == 0) {
        return false;
    }
    
    char buffer[BOARD_SIZE + 1];
    int i = play->tiles[0].i;
    int j = play->tiles[0].j;

    // rewind to the start of the word on the board
    while (j != 0) {
        if (board[i][j - 1] == '\0') {
            bool found_tile = false;

            for (size_t tile_index = 0; tile_index < play->size; tile_index++) {
                struct play_tile pt = play->tiles[tile_index];

                if (pt.i == i && pt.j == j) {
                    found_tile = true;
                    break;
                }
            }

            if (!found_tile) {
                break;
            }
        }

        j--;
    }

    size_t index = 0;

    // copy the word in the buffer
    while (j < BOARD_SIZE) {
        if (board[i][j] != '\0') {
            buffer[index++] = board[i][j];
        } else {
            bool found_tile = false;

            for (size_t tile_index = 0; tile_index < play->size; tile_index++) {
                struct play_tile pt = play->tiles[tile_index];

                if (pt.i == i && pt.j == j) {
                    buffer[index++] = pt.letter;
                    found_tile = true;
                    continue;
                }
            }

            if (!found_tile) {
                break;
            }
        }

        j++;
    }

    buffer[index] = '\0';

    printf("validate_play_horizontally word copied is: %s\n", buffer); // DEBUG

    return hashset_contains(wordlist_set, buffer);
}

bool validate_play_vertically(const char board[BOARD_SIZE][BOARD_SIZE],
                              struct hashset *wordlist_set,
                              struct play *play) {
    printf("Now validating with validate_play_vertically for word %s\n", play->word);
    
    if (play->size == 0) {
        return false;
    }
    
    char buffer[BOARD_SIZE + 1];
    int i = play->tiles[0].i;
    int j = play->tiles[0].j;

    // rewind to the start of the word on the board
    while (i != 0) {
        if (board[i - 1][j] == '\0') {
            bool found_tile = false;
            
            for (size_t tile_index = 0; tile_index < play->size; tile_index++) {
                struct play_tile pt = play->tiles[tile_index];

                if (pt.i == i - 1 && pt.j == j) {
                    found_tile = true;
                    break;
                }
            }

            if (!found_tile) {
                break;
            }
        }

        i--;
    }

    size_t index = 0;
    
    // copy the word in the buffer
    while (i < BOARD_SIZE) {
        if (board[i][j] != '\0') {
            buffer[index++] = board[i][j];
        } else {
            bool found_tile = false;

            for (size_t tile_index = 0; tile_index < play->size; tile_index++) {
                struct play_tile pt = play->tiles[tile_index];

                if (pt.i == i && pt.j == j) {
                    buffer[index++] = pt.letter;
                    found_tile = true;
                    break;
                }
            }

            if (!found_tile) {
                break;
            }
        }

        i++;
    }

    buffer[index] = '\0';

    printf("validate_play_vertically word copied is: %s\n", buffer); // DEBUG

    return hashset_contains(wordlist_set, buffer);
}

bool validate_play(const char board[BOARD_SIZE][BOARD_SIZE],
                   struct hashset *wordlist_set,
                   const char hand[HAND_SIZE],
                   struct play *play) {
    printf("Now validating play for word: %s\n", play->word);
    
    if (play->size == 0) {
        printf("play size is 0\n"); // DEBUG
        return false;
    }

    bool is_horizontal = true;
    bool is_vertical = true;
    int first_i = play->tiles[0].i;
    int first_j = play->tiles[0].j;

    // check if the tiles are in bounds and placed on a row or column
    for (size_t index = 0; index < play->size; index++) {
        struct play_tile pt = play->tiles[index];

        if (pt.i < 0 || pt.i > BOARD_SIZE - 1 || pt.j < 0 || pt.j > BOARD_SIZE - 1) {
            printf("play has out of bound tiles\n"); // DEBUG
            return false;
        }

        if (first_j != pt.j) {
            is_vertical = false;
        }

        if (first_i != pt.i) {
            is_horizontal = false;
        }
    }

    // if play is_horizontal doesn't match with our values found, the play is invalid
    if (play->is_horizontal != is_horizontal && !play->is_horizontal != is_vertical) {
        printf("play is_horizontal is invalid %b %b %b %b\n", play->is_horizontal, is_horizontal, play->is_horizontal, is_vertical); // DEBUG
        return false;
    }

    char hand_copy[HAND_SIZE];

    strcpy(hand_copy, hand);

    // validate that hand contains all tiles necessary for the play
    for (size_t i = 0; i < play->size; i++) {
        struct play_tile *pt = &play->tiles[i];
        bool pt_removed = false;
        bool has_wildcard = false;

        // remove play tile from hand to validate 
        for (size_t index = 0; index < HAND_SIZE; index++) {
            if (hand_copy[index] == '*') {
                has_wildcard = true;
            }

            if (hand_copy[index] == pt->letter) {
                hand_copy[index] = '_';
                pt_removed = true;
                break;
            }
        }

        // if no tile was found and a wildcard tile is present, it is used for the play
        if (has_wildcard) {
            for (size_t index = 0; index < HAND_SIZE; index++) {
                if (hand_copy[index] == '*') {
                    hand_copy[index] = '_';
                    pt->is_wildcard = true;
                    pt_removed = true;
                    break;
                }
            }
        }

        if (!pt_removed) {
            printf("play tile %c was not present in the hand %.*s\n", pt->letter, HAND_SIZE, hand); // DEBUG
            return false;
        }

        // check that all tiles are placed on unplayed board tiles
        if (board[pt->i][pt->j] != '\0') {
            printf("tile is played on invalid square\n"); // DEBUG
            return false;
        }
    }

    if (play->is_horizontal) {
        for (size_t i = 0; i < play->size; i++) {
            if (!validate_play_tile_vertically(board, wordlist_set, &play->tiles[i])) {
                printf("validate_play_tile_vertically failed\n"); // DEBUG
                return false;
            }
        }

        if (!validate_play_horizontally(board, wordlist_set, play)) {
            printf("validate_play_horizontally failed\n"); // DEBUG
            return false;
        }
    } else {
        for (size_t i = 0; i < play->size; i++) {
            if (!validate_play_tile_horizontally(board, wordlist_set, &play->tiles[i])) {
                printf("validate_play_tile_horizontally failed\n"); // DEBUG
                return false;
            }
        }

        if (!validate_play_vertically(board, wordlist_set, play)) {
            printf("validate_play_vertically failed\n"); // DEBUG
            return false;
        }
    }

    return true;
}

struct vec *find_all_extensions(const char board[BOARD_SIZE][BOARD_SIZE],
                                const char *wordlist,
                                struct hashset *wordlist_set,
                                const char hand[HAND_SIZE],
                                struct board_word *bw) {
    struct vec *plays = vec_initialize();

    char *regex_string = build_extension_regex_string(board, hand, bw);

    regex_t regex;
    regmatch_t pmatch[1];

    if (regcomp(&regex, regex_string, REG_NEWLINE | REG_EXTENDED)) {
        fprintf(stderr, "could not compile regex\n");
        exit(EXIT_FAILURE);
    }

    for (char const *regex_wordlist_ptr = wordlist;
         !regexec(&regex, regex_wordlist_ptr, 1, pmatch, 0);
         regex_wordlist_ptr += pmatch[0].rm_eo) {
        regoff_t regex_offset_start = pmatch[0].rm_so;
        regoff_t regex_offset_end = pmatch[0].rm_eo;
        regoff_t regex_length = regex_offset_end - regex_offset_start;

        printf("regex: %d %d \n", regex_offset_start, regex_offset_end); // DEBUG

        assert(regex_length > 0 && regex_length < BOARD_SIZE);

        // if the tile count for the play is over the hand size, we discard it
        if (regex_length - bw->length != 0 && regex_length - bw->length <= HAND_SIZE) {
            // copy the word to a buffer
            char extended_word[BOARD_SIZE];
            strncpy(extended_word, regex_wordlist_ptr + regex_offset_start, regex_length);
            extended_word[regex_length] = '\0';

            char *word_occurence_ptr = strstr(extended_word, bw->word);

            assert(word_occurence_ptr != NULL);

            while (word_occurence_ptr != NULL) {
                int extension_offset = extended_word - word_occurence_ptr;
            
                struct board_word ext_bw = {
                    .word = extended_word,
                    .length = regex_length,
                    .i = bw->is_horizontal ? bw->i : bw->i + extension_offset,
                    .j = bw->is_horizontal ? bw->j + extension_offset : bw->j,
                    .is_horizontal = bw->is_horizontal,
                };

                struct play *play = build_play(board, wordlist_set, hand, &ext_bw);
                if (play != NULL) {
                    vec_add(plays, play);
                }

                word_occurence_ptr = strstr(word_occurence_ptr + 1, bw->word);
            }
        }
    }

    free(regex_string);
    regfree(&regex);

    return plays;
}

struct vec *find_all_hooks(const char board[BOARD_SIZE][BOARD_SIZE],
                           const char *wordlist,
                           struct hashset *wordlist_set,
                           const char hand[HAND_SIZE],
                           struct play *play) {
    assert(play->size == 1);

    struct play_tile pt = play->tiles[0];

    struct vec *plays = vec_initialize();
    
    // for horizontal, check tiles above and under on the board
    // for vertical, check tiles left and right on the board
    // if they are non empty the play is not considered a hook
    if ((play->is_horizontal &&
         ((pt.i != 0 && board[pt.i - 1][pt.j] != '\0') || (pt.i != BOARD_SIZE - 1 && board[pt.i + 1][pt.j] != '\0'))) ||
        (!play->is_horizontal &&
         ((pt.j != 0 && board[pt.i][pt.j - 1] != '\0') || (pt.j != BOARD_SIZE - 1 && board[pt.i][pt.j + 1] != '\0')))) {
        printf("Detected interference for hook on play: %s\n", play->word); // DEBUG
        return plays;
    }

    char play_char = play->tiles[0].letter;

    char *regex_string = build_hook_regex_string(board, hand, play);

    regex_t regex;
    regmatch_t pmatch[1];

    if (regcomp(&regex, regex_string, REG_NEWLINE | REG_EXTENDED)) {
        fprintf(stderr, "could not compile regex\n");
        exit(EXIT_FAILURE);
    }

    for (char const *regex_wordlist_ptr = wordlist;
         !regexec(&regex, regex_wordlist_ptr, 1, pmatch, 0);
         regex_wordlist_ptr += pmatch[0].rm_eo) {
        regoff_t regex_offset_start = pmatch[0].rm_so;
        regoff_t regex_offset_end = pmatch[0].rm_eo;
        regoff_t regex_length = regex_offset_end - regex_offset_start;

        assert(regex_length > 0 && regex_length < BOARD_SIZE);

        // if the tile count for the play is over the hand size, we discard it
        if (regex_length - 1 <= HAND_SIZE) {
            // copy the word to a buffer
            char hook_word[BOARD_SIZE];
            strncpy(hook_word, regex_wordlist_ptr + regex_offset_start, regex_length);
            hook_word[regex_length] = '\0';

            char *char_occurence_ptr = strchr(hook_word, play_char);

            assert(char_occurence_ptr != NULL);

            while (char_occurence_ptr != NULL) {
                int extension_offset = hook_word - char_occurence_ptr;
                bool is_horizontal = !play->is_horizontal;

                struct board_word hook_bw = {
                    .word = hook_word,
                    .length = regex_length,
                    .i = is_horizontal ? play->tiles[0].i : play->tiles[0].i + extension_offset,
                    .j = is_horizontal ? play->tiles[0].j + extension_offset : play->tiles[0].j,
                    .is_horizontal = is_horizontal,
                };

                if (hook_bw.length <= HAND_SIZE) {
                    struct play *play = build_play(board, wordlist_set, hand, &hook_bw);
                    if (play != NULL) {
                        vec_add(plays, play);
                    }
                }

                char_occurence_ptr = strchr(char_occurence_ptr + 1, play_char);
            }
        }
    }
  
    free(regex_string);
    regfree(&regex);
    
    return plays;
}

struct vec *find_all_perpendiculars(const char board[BOARD_SIZE][BOARD_SIZE],
                                    const char *wordlist,
                                    struct hashset *wordlist_set,
                                    const char hand[HAND_SIZE],
                                    struct board_word *bw) {
    struct vec *plays = vec_initialize();

    bool is_horizontal = !bw->is_horizontal;
    int i = bw->i;
    int j = bw->j;

    // go over each character in the word
    for (int char_index = 0; char_index < bw->length; char_index++) {
        char play_char = bw->word[char_index];

        bool has_char_above = i != 0 && board[i - 1][j] != '\0';
        bool has_char_below = i != BOARD_SIZE - 1 && board[i + 1][j] != '\0';
        bool has_char_left = j != 0 && board[i][j - 1] != '\0';
        bool has_char_right = j != BOARD_SIZE - 1 && board[i][j + 1] != '\0';

        if ((bw->is_horizontal && (has_char_above || has_char_below))
            || (!bw->is_horizontal && (has_char_left || has_char_right))) {
            continue;
        }

        char *regex_string = build_perpendicular_regex_string(board, hand, bw, char_index);

        regex_t regex;
        regmatch_t pmatch[1];

        if (regcomp(&regex, regex_string, REG_NEWLINE | REG_EXTENDED)) {
            fprintf(stderr, "could not compile regex\n");
            exit(EXIT_FAILURE);
        }

        for (char const *regex_wordlist_ptr = wordlist;
             !regexec(&regex, regex_wordlist_ptr, 1, pmatch, 0);
             regex_wordlist_ptr += pmatch[0].rm_eo) {
            regoff_t regex_offset_start = pmatch[0].rm_so;
            regoff_t regex_offset_end = pmatch[0].rm_eo;
            regoff_t regex_length = regex_offset_end - regex_offset_start;

            assert(regex_length > 0 && regex_length < BOARD_SIZE);

            // if the tile count for the play is over the hand size, we discard it
            if (regex_length - bw->length != 0 && regex_length - bw->length <= HAND_SIZE) {
                // copy the word to a buffer
                char perpendicular_word[BOARD_SIZE];
                strncpy(perpendicular_word, regex_wordlist_ptr + regex_offset_start, regex_length);
                perpendicular_word[regex_length] = '\0';

                char *char_occurence_ptr = strchr(perpendicular_word, play_char);
                /* char *word_occurence_ptr = strstr(extended_word, bw->word); */

                assert(char_occurence_ptr != NULL);

                while (char_occurence_ptr != NULL) {
                    int extension_offset = perpendicular_word - char_occurence_ptr;

                    int i = is_horizontal ? bw->i + char_index : bw->i + extension_offset;
                    int j = is_horizontal ? bw->j + extension_offset : bw->j + char_index;

                    if (bw->is_horizontal) {
                        j += char_index;
                    } else {
                        i += char_index;
                    }

                    struct board_word perp_bw = {
                        .word = perpendicular_word,
                        .length = regex_length,
                        .i = i,
                        .j = j,
                        .is_horizontal = is_horizontal,
                    };

                    printf(" ===> bw: %s %d %d %d %c\n", perp_bw.word, perp_bw.length, perp_bw.i, perp_bw.j, is_horizontal ? 'H' : 'V'); // DEBUG

                    struct play *play = build_play(board, wordlist_set, hand, &perp_bw);
                    if (play != NULL) {
                        vec_add(plays, play);
                    }

                    char_occurence_ptr = strstr(char_occurence_ptr + 1, bw->word);
                }
            }
        }

        free(regex_string);
        regfree(&regex);

        if (bw->is_horizontal) {
            j++;
        } else {
            i++;
        }
    }

    return plays;
}

struct vec *find_all_parallels(const char board[BOARD_SIZE][BOARD_SIZE],
                               const char *wordlist) {
    // TODO
}

int main(int argc, char **argv) {
    /* char board[BOARD_SIZE][BOARD_SIZE]; */

    /* memset(board, 0, sizeof(board)); */

    int wordlist_fd = open(WORDLIST_PATH, O_RDONLY);
    if (wordlist_fd == -1) {
        perror("open wordlist");
        exit(EXIT_FAILURE);
    }

    struct stat file_stat;
    if (fstat(wordlist_fd, &file_stat)) {
        perror("stat wordlist");
        exit(EXIT_FAILURE);
    }

    off_t file_size = file_stat.st_size;

    char *wordlist = malloc(file_size + 1);
    if (wordlist == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    wordlist[file_size] = '\0';
    
    ssize_t read_size = read(wordlist_fd, wordlist, file_size);
    if (read_size == -1) {
        perror("read wordlist");
        exit(EXIT_FAILURE);
    }

    if (close(wordlist_fd)) {
        perror("close wordlist");
        exit(EXIT_FAILURE);
    }

    struct hashset *wordlist_set = build_wordlist_set(wordlist);

    /* char board[15][15] = { */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     { 'S',  'H',  'A', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0',  'N', '\0', '\0', '\0', '\0',  'C', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0',  'I',  'R',  'I',  'S', '\0',  'E', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0',  'M', '\0', '\0',  'L', '\0',  'N', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0',  'A', '\0', '\0',  'I',  'N',  'T', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0',  'L', '\0', '\0',  'M',  'O',  'R',  'E', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0',  'E',  'N',  'I',  'G',  'M',  'A', '\0', '\0', '\0', '\0'}, */
    /*     {'\0',  'S',  'C',  'A',  'R',  'Y', '\0',  'C',  'G', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0',  'E', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /* }; */

    /* char board[15][15] = { */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0',  'W', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0',  'A', '\0',  'A', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0',  'L', '\0',  'N', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0',  'E',  'N',  'D', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0',  'O', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0',  'N', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /* }; */

    /* char board[15][15] = { */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'P',  'O',  'O', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'A', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'R', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'T', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /* }; */

    /* // RANGER IN ALL CORNERS WITH STRING AND STRINGS IN THE MIDDLE */
    /* char board[15][15] = { */
    /*     { 'R',  'A',  'N',  'G',  'E',  'R', '\0', '\0', '\0',  'R',  'A',  'N',  'G',  'E',  'R'}, */
    /*     { 'A', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'A'}, */
    /*     { 'N', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'N'}, */
    /*     { 'G', '\0', '\0', '\0', '\0', '\0', '\0',  'S', '\0', '\0', '\0', '\0', '\0', '\0',  'G'}, */
    /*     { 'E', '\0', '\0', '\0', '\0', '\0', '\0',  'T', '\0', '\0', '\0', '\0', '\0', '\0',  'E'}, */
    /*     { 'R', '\0', '\0', '\0', '\0', '\0', '\0',  'R', '\0', '\0', '\0', '\0', '\0', '\0',  'R'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0',  'I', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0',  'S',  'T',  'R',  'I',  'N',  'G',  'S', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0',  'G', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     { 'R', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'R'}, */
    /*     { 'A', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'A'}, */
    /*     { 'N', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'N'}, */
    /*     { 'G', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'G'}, */
    /*     { 'E', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'E'}, */
    /*     { 'R',  'A',  'N',  'G',  'E',  'R', '\0', '\0', '\0',  'R',  'A',  'N',  'G',  'E',  'R'}, */
    /* }; */

    // string in middle
    /* char board[15][15] = { */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0',  'S', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0',  'T', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0',  'S',  'T',  'R',  'I',  'N',  'G', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0',  'I', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0',  'N', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0',  'G', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', 'A', '\0', '\0', '\0', 'A', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /* }; */

    // special hook
    char board[15][15] = {
        {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0',  'Z', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0',  'E', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0',  'S',  'H', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        { 'T',  'I',  'T', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0',  'Y', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
        {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'},
    };

    // TEST REGEX FOR HOOK BEGIN
    /* char board[15][15] = { */
    /*     {'\0', '\0', '\0',  'A', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0',  'N', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0',  'I', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0',  'M', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0',  'A', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     { 'Z', '\0', '\0',  'L', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     { 'Y', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     { 'M', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     { 'U', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     { 'R', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     { 'G', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     { 'Y', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /* }; */

    // TEST REGEX FOR HOOK END
    /* char board[15][15] = { */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'A', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'N', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'I', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'M', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'A', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'L', '\0', '\0',  'Z'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'Y'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'M'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'U'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'R'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'G'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0',  'Y'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /*     {'\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0', '\0'}, */
    /* }; */

    // DEBUG PRINT BOARD
    printf("   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4\n");
    for (size_t i = 0; i < BOARD_SIZE; i++) {
        printf("%2ld ", i);
        for (size_t j = 0; j < BOARD_SIZE; j++) {
            printf("%c ", board[i][j] == '\0' ? '_' : board[i][j]);
        }
        printf("\n");
    }
    printf("\n");
    // DEBUG PRINT BOARD END
    
    /* char hand[HAND_SIZE] = "ZYMU*GY"; */
    char hand[HAND_SIZE] = "DAEB*GY";

    struct vec *words = find_all_words(board);

    struct vec *plays = vec_initialize();

    for (size_t word_index = 0; word_index < words->size; word_index++) {
        struct board_word *bw = vec_get(words, word_index);
        struct vec *extension_plays = find_all_extensions(board, wordlist, wordlist_set, hand, bw);

        // add all word extension plays
        for (size_t ext_index = 0; ext_index < extension_plays->size; ext_index++) {
            struct play *extension_play = vec_get(extension_plays, ext_index);

            // add all hook plays
            if (extension_play->size == 1) {
                struct vec *hook_plays = find_all_hooks(board, wordlist, wordlist_set, hand, extension_play);

                for (size_t hook_index = 0; hook_index < hook_plays->size; hook_index++) {
                    struct play *hook_play = vec_get(hook_plays, hook_index);
   
                    vec_add(plays, hook_play);
                }

                vec_free(hook_plays);
            }

            vec_add(plays, extension_play);
        }

        struct vec *perpendicular_plays = find_all_perpendiculars(board, wordlist, wordlist_set, hand, bw);

        for (size_t perp_index = 0; perp_index < perpendicular_plays->size; perp_index++) {
            struct play *perpendicular_play = vec_get(perpendicular_plays, perp_index);

            vec_add(plays, perpendicular_play);
        }

        // TODO get all parallel plays

        vec_free(extension_plays);
        vec_free(perpendicular_plays);
    }

    // DEBUG PRINT BOARD
    printf("   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4\n");
    for (size_t i = 0; i < BOARD_SIZE; i++) {
        printf("%2ld ", i);
        for (size_t j = 0; j < BOARD_SIZE; j++) {
            printf("%c ", board[i][j] == '\0' ? '_' : board[i][j]);
        }
        printf("\n");
    }
    printf("\n");
    // DEBUG PRINT BOARD END

    printf("WORDS:\n"); // DEBUG
    for (size_t i = 0; i < words->size; i++) {
        struct board_word *bw = vec_get(words, i);
        
        printf("%d %d %c %s\n", bw->i, bw->j, bw->is_horizontal ? 'H' : 'V', bw->word);
    }
    printf("\n"); // DEBUG

    printf("PLAYS:\n"); // DEBUG
    for (size_t i = 0; i < plays->size; i++) {
        struct play *play = vec_get(plays, i);

        printf(" %ld %s %c { ", play->size, play->word, play->is_horizontal ? 'H' : 'V');

        for (size_t j = 0; j < play->size; j++) {
            struct play_tile pt = play->tiles[j];

            printf("%c %d %d, ", pt.is_wildcard ? '*' : pt.letter, pt.i, pt.j);
        }

        printf("}\n");
    }
    printf("\n"); // DEBUG

    for (size_t i = 0; i < plays->size; i++) {
        struct play *play = vec_get(plays, i);

        free_play(play);
    }

    for (size_t i = 0; i < words->size; i++) {
        struct board_word *bw = vec_get(words, i);

        free_board_word(bw);
    }

    vec_free(plays);
    vec_free(words);
    free(wordlist);
    free_hashset(wordlist_set);

    return 0;
}
