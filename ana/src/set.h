#ifndef ANA_SET_H
#define ANA_SET_H
#include <stdlib.h>
#include <stdbool.h>

typedef struct AnaStrSet AnaStrSet;
struct AnaStrSet {
    char **strings;
    size_t size;
    size_t capacity;
};

AnaStrSet *set_create();
bool set_includes(AnaStrSet *set, char word[]);
void set_load(AnaStrSet *set, char word[]);
void set_add(AnaStrSet *set, char word[]);
void set_print(AnaStrSet *set);
void set_free(AnaStrSet *set);

#endif