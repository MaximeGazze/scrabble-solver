#include "hashset.h"

#include <stdio.h>
#include <string.h>

struct hashset *hashset_create(size_t capacity) {
    struct hashset *hashset = malloc(sizeof(struct hashset));
    if (hashset == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    hashset->table = malloc(sizeof(struct hashset_node *) * capacity);
    if (hashset->table == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    memset(hashset->table, 0, sizeof(struct hashset_node *) * capacity);

    hashset->capacity = capacity;

    return hashset;
}

unsigned long hash(const char *string) {
    unsigned long hash = 5381;

    for (const char *current_char = string; *current_char; current_char++) {
        hash = ((hash << 5) + hash) * *current_char;
    }

    return hash;
}

void hashset_insert(struct hashset *hashset, const char *value) {
    size_t index = hash(value) % hashset->capacity;

    struct hashset_node *node = malloc(sizeof(struct hashset_node));
    if (node == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    node->value = strdup(value);
    node->next = hashset->table[index];
    
    hashset->table[index] = node;
}

bool hashset_contains(const struct hashset *hashset, const char *value) {
    size_t index = hash(value) % hashset->capacity;

    struct hashset_node *node = hashset->table[index];

    while (node != NULL) {
        if (!strcmp(node->value, value)) {
            return true;
        }

        node = node->next;
    }

    return false;
}

void free_hashset(struct hashset *hashset) {
    for (size_t index = 0; index < hashset->capacity; index++) {
        struct hashset_node *node = hashset->table[index];

        while (node != NULL) {
            struct hashset_node *temp_node = node;

            node = node->next;

            free(temp_node->value);
            free(temp_node);
        }
    }

    free(hashset->table);
    free(hashset);
}
