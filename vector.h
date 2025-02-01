#ifndef VECTOR_H
#define VECTOR_H

#include <stdlib.h>

#define VEC_INITIAL_SIZE 2

struct vector {
    void **array;
    size_t size;
    size_t mem_size;
};

struct vector *create_vector();

void free_vector(struct vector *vector);

void vector_push_back(struct vector *vector, void *address);

void *vector_at(struct vector *vector, size_t index);

void *vector_back(struct vector *vector);

void vector_pop_back(struct vector *vector);

size_t vector_size(struct vector *vector);

#endif
