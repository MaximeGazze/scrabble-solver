#include "vector.h"
#include <stdlib.h>

struct vec *vec_initialize() {
    struct vec *vector = malloc(sizeof(struct vec));

    vector->array = malloc(sizeof(void *) * VEC_INITIAL_SIZE);
    vector->size = 0;
    vector->mem_size = VEC_INITIAL_SIZE;
    
    return vector;
}

void vec_free(struct vec *vector) {
    free(vector->array);
    free(vector);
}

void vec_resize(struct vec *vector) {
    size_t new_size = vector->mem_size * 2;

    void *new_array = realloc(vector->array, sizeof(void *) * new_size);

    vector->array = new_array;
    vector->mem_size = new_size;
}

void vec_add(struct vec *vector, void *address) {
    if (vector->size == vector->mem_size) {
        vec_resize(vector);
    }

    vector->array[vector->size++] = address;
}

void *vec_get(struct vec *vector, size_t index) {
    return vector->array[index];
}

size_t vec_size(struct vec *vector) {
    return vector->size;
}
