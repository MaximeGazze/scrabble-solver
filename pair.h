#ifndef PAIR_H
#define PAIR_H

struct pair {
    void *first;
    void *second;
};

struct pair *create_pair(void *first, void *second);

#endif
