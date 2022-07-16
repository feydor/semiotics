#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

unsigned fact(unsigned n) {
    if (!n) return 1;
    else return n * fact(n-1);
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
char *shr(char *word, size_t i, size_t len) {
    assert(i < len);
    char *res = strdup(word);
    char ch = *(word+i);

    if (i+1 == len) {
        // rotate right
        for (size_t i = 1; i < len; ++i) {
            *(res + i) = *(word + i -1);
        }
        *res = ch;
    } else {
        // swap
        *(res + i) = *(word + i + 1);
        *(res + i + 1) = ch;
    }

    return res;
}

// swap word[i] with word[j]
char *swapch(char *word, size_t i, size_t j, size_t len) {
    assert(i < len && j < len);
    char *res = strdup(word);
    char tmp = word[i];
    res[i] = word[j];
    res[j] = tmp;
    return res;
}



size_t dict_index = 0;
size_t dict_capacity = 0;

bool dict_includes(char *dict[], char word[]) {
    assert(dict[0]);
    for (size_t i = 0; i < dict_index; ++i) {
        if (!strcmp(dict[i], word)) {
            return true;
        }
    }
    return false;
}

// assumes dict is preallocated to hold N words of known length
void dict_add(char *dict[], char word[]) {
    dict[dict_index] = strdup(word);
    ++dict_index;
}

bool dict_full() {
    return dict_index == dict_capacity;
}

void swap_all(char *anagrams[], char *word, size_t len) {
    if (dict_full()) return;
    for (size_t i = 0; i < len; ++i) {
        for (size_t j = i+1; j < len; ++j) {
            char *gram = swapch(word, i, j, len);
            
            if (!dict_includes(anagrams, gram)) {
                dict_add(anagrams, gram);
                swap_all(anagrams, gram, len);
            }
        }
    }
}

int main(int argc, char *argv[]) {
    // there are n! permutations for a word of len n
    char *word = "abcd";
    size_t len = strlen(word);
    size_t n = fact(len);
    char *permutations[n];
    dict_capacity = n;
    for (size_t i = 0; i < n; ++i) {
        permutations[i] = word;
    }

    swap_all(permutations, word, len);

    printf("printing permutations...\n");
    printf("# of permutations: %lu\n", n);
    for (size_t i = 0; i < n; ++i) {
        printf("%s\n", permutations[i]);
    }
}