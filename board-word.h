#ifndef BOARD_WORD_H
#define BOARD_WORD_H

#include <stdbool.h>

struct board_word {
    char *word;
    int length;
    int i;
    int j;
    bool is_horizontal;
};

struct board_word *create_board_word();

void free_board_word(struct board_word *bw);

#endif
