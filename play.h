#ifndef PLAY_H
#define PLAY_H

#include <stdlib.h>
#include <stdbool.h>

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

struct play *create_play();

void free_play(struct play *p);

#endif
