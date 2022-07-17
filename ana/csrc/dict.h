#ifndef ANA_DICT_H
#define ANA_DICT_H
#include <stdlib.h>
#include <stdbool.h>

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