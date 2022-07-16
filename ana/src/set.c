#include "set.h"
#include <stdio.h>
#include <string.h>

AnaStrSet *set_create() {
    AnaStrSet *set = malloc(sizeof(AnaStrSet));
    if (!set) return NULL;
    set->size = 0;
    set->capacity = 5;
    set->strings = calloc(set->capacity, sizeof(char *));
    if (!set->strings) return NULL;
    return set;
}

bool set_includes(AnaStrSet *set, char word[]) {
    // assuming strings are added alphabetically b/c words.txt is currently alphabetical
    for (size_t i = 0; i < set->size; ++i) {
        int cmp = strcmp(set->strings[i], word);
        if (!cmp) return true;
        if (cmp > 1) return false;
    }
    return false;
}

// skips check for pre-existing elements
void set_load(AnaStrSet *set, char word[]) {
    if (set->size == set->capacity) {
        set->capacity *= 2;
        set->strings = realloc(set->strings, set->capacity * sizeof(char *));
        if (!set->strings) {
            fprintf(stderr, "set_add: failed realloc\n");
            exit(EXIT_FAILURE);
        }
    }

    set->strings[set->size] = strdup(word);
    if (!set->strings[set->size]) {
        fprintf(stderr, "set_add: failed strdup\n");
        exit(EXIT_FAILURE);
    }
    ++set->size;
}

void set_add(AnaStrSet *set, char word[]) {
    if (set_includes(set, word)) return;
    set_load(set, word);
}

void set_print(AnaStrSet *set) {
    for (size_t i = 0; i < set->size; ++i)
        printf("%s\n", set->strings[i]);
}

void set_free(AnaStrSet *set) {
    for (size_t i = 0; i < set->size; ++i)
        free(set->strings[i]);
    free(set->strings);
}