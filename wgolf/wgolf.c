// strpool - what's an allocation?
#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#define BUFSIZE 19333
#define max(a,b) (((a)>(b))?(a):(b))
typedef struct strpool strpool;
struct strpool {
    char buf[BUFSIZE];
    size_t sp; // should always be pointing at '\0'
    size_t size;
};

typedef struct dict dict;
struct dict {
    strpool pool;
    size_t size;
};

// stringview type
typedef struct strv strv;
struct strv {
    size_t ptr;
    size_t len;
};

// potential buffer overflow
strv strpool_new(strpool *pool, char *s) {
    size_t saved = pool->sp;
    size_t len = 0;
    while (*s) {
        pool->buf[pool->sp] = *s;
        pool->sp++;
        len++;
        s++;
    }
    pool->buf[pool->sp++] = '\0';

    if (pool->sp >= BUFSIZE) {
        fprintf(stderr, "strpool buffer overflow\n");
        exit(EXIT_FAILURE);
    } 

    pool->size++;
    return (strv){ .ptr = saved, .len = len };
}

char *strpool_get(strpool *pool, strv s) {
    return pool->buf+s.ptr;
}

void strv_print_debug(strpool *pool, strv s) {
    printf("{.ptr=%ld, .len=%ld} ", s.ptr, s.len);
    size_t sp = s.ptr;
    while (pool->buf[sp]) {
        printf("'%c'", pool->buf[sp]);
        sp++;
    }
    printf("\n");
}

int strv_cmp(strpool *pool, strv s1, strv s2) {
    size_t sp1 = s1.ptr;
    size_t sp2 = s2.ptr;
    int a = 0, b = 0;

    while (pool->buf[sp1]) {
        a += pool->buf[sp1];
        sp1++;
    }
    while (pool->buf[sp2]) {
        b += pool->buf[sp2];
        sp2++;
    }
    return !(a-b) ? 0 : a-b > 0 ? 1 : -1;
}

void test_strv_cmp(strpool *p) {
    strv a = strpool_new(p, "canoe");
    strv b = strpool_new(p, "canon");
    assert(-1 == strv_cmp(p, a, b));

    a = strpool_new(p, "");
    b = strpool_new(p, "a");
    assert(-1 == strv_cmp(p, a, b));

    a = strpool_new(p, "A");
    b = strpool_new(p, "a");
    assert(-1 == strv_cmp(p, a, b));

    a = strpool_new(p, "cil");
    b = strpool_new(p, "cml");
    assert(-1 == strv_cmp(p, a, b));
}

// f('lewis',3) -> 'lews'
void strv_delete_at(strpool *p, strv s, size_t i) {
    assert(i < s.len);
    memcpy(strpool_get(p,s)+i, strpool_get(p,s)+i+1, s.len-i+1);
    s.len--;
}

void test_strv_delete_at(strpool *p) {
    // delete in middle
    strv a = strpool_new(p, "lewis");
    strv_delete_at(p, a, 3);
    assert(!strcmp("lews", strpool_get(p, a)));

    // delete at end
    a = strpool_new(p, "abc");
    strv_delete_at(p, a, 2);
    assert(!strcmp("ab", strpool_get(p, a)));

    // delete at the begining
    a = strpool_new(p, "ed");
    strv_delete_at(p, a, 0);
    assert(!strcmp("d", strpool_get(p, a)));
}

void strv_replace_at(strpool *p, strv s, size_t i, char c) {
    assert(i < s.len);
    *(strpool_get(p, s)+i) = c;
}

void test_strv_replace_at(strpool *p) {
    strv a = strpool_new(p, "repl");
    strv_replace_at(p, a, 2, 'a');
    assert(!strcmp("real", strpool_get(p, a)));
}

strv strv_prepend(strpool *p, strv s, char c) {
    // cheat by making new str in pool
    char new[s.len+1];
    new[0] = c;
    memcpy(new+1, strpool_get(p,s), s.len+1);
    return strpool_new(p, new);
}

void test_strv_prepend(strpool *p) {
    strv a = strpool_new(p, "end");
    strv res = strv_prepend(p, a, 'r');
    assert(!strcmp("rend", strpool_get(p, res)));
    assert(!strcmp("end", strpool_get(p, a)));
}

// TODO: what is the point of this?
strv strv_cpymov(strpool *p, strv dest, strv src) {
    if (dest.len == src.len) {
        strcpy(strpool_get(p, dest), strpool_get(p, src));
        dest.len = src.len;
    } else if (dest.len > src.len) {
        memcpy(strpool_get(p, dest), strpool_get(p, src), src.len+1);
        dest.len = src.len;
    } else {
        strv new = strpool_new(p, strpool_get(p, src));
        dest.ptr = new.ptr;
        dest.len = new.len;
    }
    return dest;
}
void test_strv_cpymov(strpool *p) {
    // x.len == y.len
    strv x = strpool_new(p, "tweedledee");
    strv y = strpool_new(p, "tweedledum");
    strv res = strv_cpymov(p, y, x);
    assert(!strcmp("tweedledee", strpool_get(p, res)));

    // x.len < y.len
    x = strpool_new(p, "rel");
    y = strpool_new(p, "relo");
    res = strv_cpymov(p, y, x);
    assert(!strcmp("rel", strpool_get(p, res)));

    // x.len > y.len
    x = strpool_new(p, "relo");
    y = strpool_new(p, "rel");
    res = strv_cpymov(p, y, x);
    assert(!strcmp("relo", strpool_get(p, res)));
}


// TODO: invalidates src
void strv_move(strpool *p, strv *dest, strv *src) {
    dest->ptr = src->ptr;
    dest->len = src->len;
    // TODO: Let the strpool know that src is "freed"
}

void test_strv_move(strpool *p) {
    strv dest = strpool_new(p, "tweedledee");
    strv src = strpool_new(p, "tweedledum");
    strv_move(p, &dest, &src);
    assert(!strcmp("tweedledum", strpool_get(p, dest)));
}

// f(s1: "dipple", s2: "dapple") -> s1: "dippledapple", s2: "dapple"
void strv_cat(strpool *p, strv *s1, strv *s2) {
    char temp[s1->len + s2->len+1];
    memcpy(temp, strpool_get(p, *s1), s1->len);
    memcpy(temp+s1->len, strpool_get(p, *s2), s2->len+1);
    strv svt = strpool_new(p, temp);
    strv_move(p, s1, &svt);
}

void test_strv_cat(strpool *p) {
    strv x = strpool_new(p, "con");
    strv y = strpool_new(p, "cat");
    strv_cat(p, &x, &y);
    assert(!strcmp("concat", strpool_get(p, x)));
    x = strpool_new(p, "");
    y = strpool_new(p, "cat");
    strv_cat(p, &x, &y);
    assert(!strcmp("cat", strpool_get(p, x)));
    x = strpool_new(p, "cat");
    y = strpool_new(p, "");
    strv_cat(p, &x, &y);
    assert(!strcmp("cat", strpool_get(p, x)));
    x = strpool_new(p, "");
    y = strpool_new(p, "");
    strv_cat(p, &x, &y);
    assert(!strcmp("", strpool_get(p, x)));
}

dict load_dict(size_t wordlen) {
    FILE *fp = fopen("/usr/share/dict/words", "r");
    if (!fp) {
        fprintf(stderr, "Missing '/usr/share/dict/words'.\n");
        exit(EXIT_FAILURE);
    }

    dict d = {0};
    char *line = NULL;
    size_t len = 0;
    ssize_t nread;
    while ((nread = getline(&line, &len, fp)) != -1) {
        line[strcspn(line, "\n")] = 0;
        if (strlen(line) != wordlen) continue;
        for (int i = 0; line[i]; ++i)
            line[i] = (char)tolower(line[i]);
        d.size++;
        strpool_new(&d.pool, line);
    }
    return d;
}

// TODO: change all memcpy to use sizeof
bool strpool_alphabetical_search(strpool p, char *s) {
    size_t sp = 0;
    while (sp != p.sp) {
        size_t len = 0;
        while (p.buf[len++]);
        char found[len+1];
        memcpy(found, p.buf+sp, sizeof(found));
        if (!strcmp(found, s)) return true;
        // if (found[0] > s[0]) return false;
        sp += len;
    }
    return false;
}

void test_strpool_alphabetical_search(dict d) {
    assert(strpool_alphabetical_search(d.pool, "zoos"));
    assert(strpool_alphabetical_search(d.pool, "abby"));
    assert(!strpool_alphabetical_search(d.pool, "zzzzzz"));
    assert(!strpool_alphabetical_search(d.pool, ""));
}

// dict is in alphabetical order
bool dictionary_word(dict d, char *s) {
    return strpool_alphabetical_search(d.pool, s);
}


char ALPHABET[] = "abcdefghijklmnopqrstuvwxyz";

void wgolf(strpool *p, dict *d, strv dest, strv src, size_t start, size_t *n) {
    assert(dest.len == src.len);
    if (!strv_cmp(p, dest, src)) return;
    char *src_word = strpool_get(p, src);
    char *dest_word = strpool_get(p, dest);
    for (size_t i = start; i < src.len; ++i) {
        char a;
        size_t ac = 0;
        char temp = src_word[i];
        while ((a = ALPHABET[ac++])) {
            src_word[i] = a;
            if (dictionary_word(*d, src_word)) {
                strcpy(strpool_get(p, src), src_word);
                strcpy(strpool_get(p, dest), dest_word);
                wgolf(p, d, dest, src, start+1, n);
            }
            src_word[i] = temp;
        }
    }
}

void test_strv() {
    strpool p = {0};
    test_strv_cmp(&p);
    test_strv_delete_at(&p);
    test_strv_replace_at(&p);
    test_strv_prepend(&p);
    test_strv_move(&p);
    test_strv_cat(&p);
    printf("all tests passed on strv\n");
}

int main(void) {
    assert(27 == sizeof(ALPHABET));
    printf("wgolf 0.1.0\n");
    test_strv();
    dict d = load_dict(4);
    test_strpool_alphabetical_search(d);
    
    strpool p = {0};
    strv src = strpool_new(&p, "head");
    strv dest = strpool_new(&p, "tail");
    size_t n = 0;
    wgolf(&p, &d, dest, src, 0, &n);
    printf("%s -> %s in %lu steps.\n", strpool_get(&p, src), strpool_get(&p, dest), n);

    // print steps
    size_t sp = dest.ptr;
    while (sp != p.sp) {
        size_t len = 0;
        while (p.buf[len++]);
        char result[len+1];
        memcpy(result, p.buf+sp, sizeof(result));
        printf("%s\n", result);
        sp += len;
    }
}
