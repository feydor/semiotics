#include "dict.h"
#include <string.h>
#include <stdio.h>

Dict *dict_create() {
    Dict *dict = malloc(sizeof(Dict));
    if (!dict) return NULL;
    dict->size = 0;
    dict->capacity = 5;
    dict->strings = calloc(dict->capacity, sizeof(char *));
    if (!dict->strings) return NULL;
    for (size_t i = 0; i < dict->capacity; ++i) {
        dict->strings[i] = strdup("\0");
        if (!dict->strings[i]) return NULL;
    }
    return dict;
}

void dict_push(Dict *dict, char word[]) {
    if (dict->size == dict->capacity) {
        dict->capacity *= 2;
        dict->strings = realloc(dict->strings, dict->capacity * sizeof(char *));
        if (!dict->strings) {
            fprintf(stderr, "dict_push: failed realloc\n");
            exit(EXIT_FAILURE);
        }
    }
    dict->strings[dict->size] = strdup(word);
    if (!dict->strings[dict->size]) {
        fprintf(stderr, "dict_push: failed strdup\n");
        exit(EXIT_FAILURE);
    }
    ++dict->size;
}

bool dict_includes(Dict *dict, char word[]) {
    for (size_t i = 0; i < dict->size; ++i) {
        if (!strcmp(dict->strings[i], word)) {
            return true;
        }
    }
    return false;
}

void dict_print(Dict *dict) {
    for (size_t i = 0; i < dict->size; ++i) {
        printf("%s\n", dict->strings[i]);
    }
}

void dict_free(Dict *dict) {
    for (size_t i = 0; i < dict->size; ++i) {
        free(dict->strings[i]);
        dict->strings[i] = NULL;
    }
    free(dict->strings);
    dict->strings = NULL;
}
