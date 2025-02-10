#include "pair.h"

#include <stdio.h>
#include <stdlib.h>

struct pair *create_pair(void *first, void *second) {
    struct pair *p = malloc(sizeof(struct pair));
    if (p == NULL) {
        perror("malloc");
        exit(EXIT_FAILURE);
    }

    p->first = first;
    p->second = second;

    return p;
}
