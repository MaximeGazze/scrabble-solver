#ifndef HASHSET_H
#define HASHSET_H

#include <stdlib.h>
#include <stdbool.h>

struct hashset_node {
    char *value;
    struct hashset_node *next;
};

struct hashset {
    struct hashset_node **table;
    size_t capacity;
};

struct hashset *hashset_create(size_t capacity);

void hashset_insert(struct hashset *hashset, const char *value);

bool hashset_contains(const struct hashset *hashset, const char *value);

void free_hashset(struct hashset *hashset);

#endif
