#include "dict.h"
#include "set.h"
#include <assert.h>
#include <ctype.h>
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

bool is_anagram(char a[], char b[]) {
    int first_freq[26] = {0};
    int second_freq[26] = {0};
    for (int i = 0; a[i]; ++i)
        first_freq[a[i] - 'a']++;

    for (int i = 0; b[i]; ++i)
        second_freq[b[i] - 'a']++;

    for (int i = 0; i < 26; ++i)
        if (first_freq[i] != second_freq[i])
            return false;
    return true;
}

void load_dictionary(AnaStrSet *dict) {
    char *src = "../res/words.txt";
    char *line = NULL;
    size_t len = 0;
    ssize_t nread;
    FILE *fp = fopen(src, "r");
    if (!fp) exit(EXIT_FAILURE);
    while ((nread = getline(&line, &len, fp)) != -1) {
        line[strcspn(line, "\n")] = 0;
        for (int i = 0; line[i]; i++) {
            line[i] = (char)tolower(line[i]);
        }
        set_load(dict, line);
    }

    free(line);
    fclose(fp);
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        printf("program: ana [word]\n");
        exit(EXIT_FAILURE);
    }

    AnaStrSet *dict = set_create();
    load_dictionary(dict);
    AnaStrSet *results = set_create();

    for (size_t i = 0; i < dict->size; ++i) {
        char *curr = dict->strings[i];
        if (is_anagram(curr, argv[1])) set_add(results, curr);
    }

    set_print(results);
    
    set_free(dict);
    set_free(results);
    exit(EXIT_SUCCESS);
}