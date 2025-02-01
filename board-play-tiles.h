#ifndef BOARD_PLAYS_H
#define BOARD_PLAYS_H

#include "scrabble.h"

struct board_play_tiles {
    struct board_play_tiles_node **table;
};

struct board_play_tiles_node {
    struct play_tile *pt;
    struct board_play_tiles_node *next;
};

struct board_play_tiles *create_board_play_tiles();

void board_play_tiles_insert(struct board_play_tiles *bpt, struct play_tile *pt);

struct board_play_tiles_node *board_play_tiles_at(struct board_play_tiles *bpt, int i, int j);

void free_board_play_tiles(struct board_play_tiles *bpt);

#endif
