#include "play.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "scrabble.h"

struct play *create_play() {
    struct play *p = malloc(sizeof(struct play));
    if (p == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    memset(p, 0, sizeof(struct play));

    p->word = malloc(BOARD_SIZE + 1);
    if (p->word == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    memset(p->word, 0, BOARD_SIZE + 1);

    p->tiles = malloc(sizeof(struct play_tile) * HAND_SIZE);
    if (p->tiles == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    memset(p->tiles, 0, sizeof(struct play_tile) * HAND_SIZE);

    return p;
}

void free_play(struct play *p) {
    free(p->word);
    free(p->tiles);
    free(p);
}
