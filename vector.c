#include "vector.h"
#include <stdlib.h>
#include <assert.h>

struct vector *create_vector() {
    struct vector *vector = malloc(sizeof(struct vector));

    vector->array = malloc(sizeof(void *) * VEC_INITIAL_SIZE);
    vector->size = 0;
    vector->mem_size = VEC_INITIAL_SIZE;
    
    return vector;
}

void free_vector(struct vector *vector) {
    free(vector->array);
    free(vector);
}

void vector_resize(struct vector *vector) {
    size_t new_size = vector->mem_size * 2;

    void *new_array = realloc(vector->array, sizeof(void *) * new_size);

    vector->array = new_array;
    vector->mem_size = new_size;
}

void vector_push_back(struct vector *vector, void *address) {
    if (vector->size == vector->mem_size) {
        vector_resize(vector);
    }

    vector->array[vector->size++] = address;
}

void *vector_at(struct vector *vector, size_t index) {
    assert(index < vector->size);
    
    return vector->array[index];
}

void *vector_back(struct vector *vector) {
    assert(vector->size > 0);
    
    return vector->array[vector->size - 1];
}

void vector_pop_back(struct vector *vector) {
    assert(vector->size > 0);

    vector->size--;
}

size_t vector_size(struct vector *vector) {
    return vector->size;
}
