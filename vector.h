#ifndef VECTOR_H
#define VECTOR_H

#include <stdlib.h>

#define VEC_INITIAL_SIZE 2

struct vec {
    void **array;
    size_t size;
    size_t mem_size;
};

struct vec *vec_initialize();

void vec_free(struct vec *vector);

void vec_add(struct vec *vector, void *address);

void *vec_get(struct vec *vector, size_t index);

size_t vec_size(struct vec *vector);

#endif
