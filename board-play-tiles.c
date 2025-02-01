#include "board-play-tiles.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct board_play_tiles *create_board_play_tiles() {
    struct board_play_tiles *bpt = malloc(sizeof(struct board_play_tiles));
    if (bpt == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    size_t table_size = sizeof(struct board_play_tiles_node *) * BOARD_SIZE * BOARD_SIZE;

    bpt->table = malloc(table_size);
    if (bpt->table == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    memset(bpt->table, 0, table_size);

    return bpt;
}

void board_play_tiles_insert(struct board_play_tiles *bpt, struct play_tile *pt) {
    struct board_play_tiles_node *new_bptn = malloc(sizeof(struct board_play_tiles_node));
    if (new_bptn == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    new_bptn->pt = pt;
    new_bptn->next = bpt->table[pt->i * BOARD_SIZE + pt->j];

    bpt->table[pt->i * BOARD_SIZE + pt->j] = new_bptn;
}

struct board_play_tiles_node *board_play_tiles_at(struct board_play_tiles *bpt, int i, int j) {
    return bpt->table[i * BOARD_SIZE + j];
}

void free_board_play_tiles(struct board_play_tiles *bpt) {
    for (int i = 0; i < BOARD_SIZE; i++) {
        for (int j = 0; j < BOARD_SIZE; j++) {
            struct board_play_tiles_node *current_bptn = bpt->table[i * BOARD_SIZE + j];

            while (current_bptn != NULL) {
                struct board_play_tiles_node *temp_bptn = current_bptn;
                current_bptn = current_bptn->next;
                free(temp_bptn);
            }
        }
    }
}
