#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

size_t fact(size_t n) {
    if (!n) return 1;
    else return n * fact(n-1);
}

void swap(char *a, char *b) {
    char tmp = *a;
    *a = *b;
    *b = tmp;
}

void reverse(char *word, size_t len) {
    for (size_t i = 0; i < len; ++i) {
        swap(word + len-i-1, word + i);
    }
}

// a string with everything but the char at i
char *rest(char word[static 1], size_t i, size_t len) {
    assert(i < len);
    char *rest = malloc(len-1 * sizeof(char));
    if (!rest) return NULL;

    // before i
    for (size_t j = 0; j != i; ++j) {
        rest[j] = word[j];
    }

    // after
    for (size_t k = i+1; k <= len-1; ++k) {
        rest[k-1] = word[k];
    }
    rest[len-1] = '\0';
    return rest;
}

// shifts the char at i to i+1, wrapping if necessary
void shr(char *word, size_t i, size_t len) {
    assert(i < len);

    if (i+1 == len) {
        // rotate right
        char *res = strdup(word);
        char *last = word + i;
        for (size_t j = 1; j < len; ++j) {
            *(res + j) = *(word + j - 1);
        }
        *res = *last;
        strcpy(word, res);
        free(res);
    } else {
        swap(word+i, word+i+1);
    }
}

char *reversed(char *word, size_t len) {
    char *res = strdup(word);
    for (size_t i = 0; i < len; ++i) {
        res[len-i-1] = word[i];
    }
    return res;
}

typedef struct Dict Dict;
struct Dict {
    char **strings;
    size_t size;
    size_t capacity;
};

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

bool str_includes_ch(char *str, size_t len, char ch) {
    for (size_t i = 0; i < len; ++i) {
        if (str[i] == ch) return true;
    }
    return false;
}

size_t unique_letters(char *word, size_t len) {
    size_t unique = 0;
    for (size_t i = 0; i < len; ++i) {
        char *rest_word = rest(word, i, len);
        unique += !str_includes_ch(rest_word, len-1, word[i]);
    }
    return unique;
}

void permutate(Dict *dict, char word[], size_t curr, size_t end) {
    if (curr == end) {
        if (!dict_includes(dict, word))
            dict_push(dict, word);
    } else {
        for (size_t i = curr; i <= end; ++i) {
            swap(word+curr, word+i);
            permutate(dict, word, curr+1, end);
            swap(word+curr, word+i);
        }
    }
}

int main(int argc, char *argv[]) {
    (void)argc;
    (void)argv;
    
    // there are n! permutations for a word of len n
    char word[] = "mother";
    size_t len = strlen(word);
    printf("# of unique letters: %lu\n", unique_letters(word, len));
    Dict *permutations = dict_create();

    permutate(permutations, word, 0, len-1);
    
    printf("printing unique permutations...\n");
    printf("# of unique permutations: %lu\n", permutations->size);
    dict_print(permutations);
    dict_free(permutations);
}