#ifndef ANA_DICT_H
#define ANA_DICT_H
#include <stdlib.h>
#include <stdbool.h>

typedef struct DictEntry DictEntry;
struct DictEntry {
    char *word;
    char *alphabetical;
    size_t key;
    size_t len;
};

typedef struct Dict Dict;
struct Dict {
    char **strings;
    size_t size;
    size_t capacity;
};

Dict *dict_create();
void dict_push(Dict *dict, char word[]);
bool dict_includes(Dict *dict, char word[]);
void dict_print(Dict *dict);
void dict_free(Dict *dict);

#endif