#include "board-word.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "scrabble.h"

struct board_word *create_board_word() {
    struct board_word *bw = malloc(sizeof(struct board_word));
    if (bw == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    memset(bw, 0, sizeof(struct board_word));

    bw->word = malloc(BOARD_SIZE + 1);
    if (bw->word == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    memset(bw->word, 0, BOARD_SIZE + 1);

    return bw;
}

void free_board_word(struct board_word *bw) {
    free(bw->word);
    free(bw);
}
