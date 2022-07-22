// strpool - what's an allocation?
#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
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

typedef struct strview strview;
struct strview {
    size_t ptr;
    size_t len;
};

// potential buffer overflow
strview strpool_new(strpool *pool, char *s) {
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
    return (strview){ .ptr = saved, .len = len };
}

char *strpool_get(strpool *pool, strview s) {
    return pool->buf+s.ptr;
}

void strview_print_debug(strpool *pool, strview s) {
    printf("{.ptr=%ld, .len=%ld} ", s.ptr, s.len);
    size_t sp = s.ptr;
    while (pool->buf[sp]) {
       printf("'%c'", pool->buf[sp]);
       sp++;
    }
    printf("\n");
}

int strview_cmp(strpool *pool, strview s1, strview s2) {
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

void test_strview_cmp(strpool *p) {
    strview a = strpool_new(p, "canoe");
    strview b = strpool_new(p, "canon");
    assert(-1 == strview_cmp(p, a, b));

    a = strpool_new(p, "");
    b = strpool_new(p, "a");
    assert(-1 == strview_cmp(p, a, b));

    a = strpool_new(p, "A");
    b = strpool_new(p, "a");
    assert(-1 == strview_cmp(p, a, b));
    
    a = strpool_new(p, "cil");
        b = strpool_new(p, "cml");
    assert(-1 == strview_cmp(p, a, b));
}

// f('lewis',3) -> 'lews'
void strview_delete_at(strpool *p, strview s, size_t i) {
    assert(i < s.len);
    memcpy(strpool_get(p,s)+i, strpool_get(p,s)+i+1, s.len-i+1);
    s.len--;
}

void test_strview_delete_at(strpool *p) {
    // delete in middle
    strview a = strpool_new(p, "lewis");
    strview_delete_at(p, a, 3);
    assert(0 == strcmp("lews", strpool_get(p, a)));

    // delete at end
    a = strpool_new(p, "abc");
    strview_delete_at(p, a, 2);     
    assert(0 == strcmp("ab", strpool_get(p, a)));
    
    // delete at the begining
    a = strpool_new(p, "ed");
    strview_delete_at(p, a, 0);     
    assert(0 == strcmp("d", strpool_get(p, a)));
}

void strview_replace_at(strpool *p, strview s, size_t i, char c) {
    assert(i < s.len);
    *(strpool_get(p, s)+i) = c;
}

void test_strview_replace_at(strpool *p) {
    strview a = strpool_new(p, "repl");
    strview_replace_at(p, a, 2, 'a');
    assert(!strcmp("real", strpool_get(p, a)));
}

strview strview_prepend(strpool *p, strview s, char c) {
    // cheat by making new str in pool
    char new[s.len+1];
    new[0] = c;
    memcpy(new+1, strpool_get(p,s), s.len+1);
    return strpool_new(p, new);
}

void test_strview_prepend(strpool *p) {
    strview a = strpool_new(p, "end");
    strview res = strview_prepend(p, a, 'r');
    assert(!strcmp("rend", strpool_get(p, res)));
    assert(!strcmp("end", strpool_get(p, a)));
}

strview strview_cpymov(strpool *p, strview dest, strview src) {
    if (dest.len == src.len) {
        strcpy(strpool_get(p, dest), strpool_get(p, src));
        dest.len = src.len;
    } else if (dest.len > src.len) {
        memcpy(strpool_get(p, dest), strpool_get(p, src), src.len+1);
        dest.len = src.len;
    } else {
        strview new = strpool_new(p, strpool_get(p, src));
        dest.ptr = new.ptr;
        dest.len = new.len;
    }
    return dest;
}
void test_strview_cpymov(strpool *p) {
    // x.len == y.len
    strview x = strpool_new(p, "tweedledee");
    strview y = strpool_new(p, "tweedledum");
    strview res = strview_cpymov(p, y, x);
    assert(!strcmp("tweedledee", strpool_get(p, res)));

    // x.len < y.len
    x = strpool_new(p, "rel");
    y = strpool_new(p, "relo");
    res = strview_cpymov(p, y, x);
    assert(!strcmp("rel", strpool_get(p, res)));
    
    // x.len > y.len
    x = strpool_new(p, "relo");
    y = strpool_new(p, "rel");
    res = strview_cpymov(p, y, x);
    assert(!strcmp("relo", strpool_get(p, res)));
}


// TODO: invalidates src
void strview_move(strpool *p, strview *dest, strview *src) {
    dest->ptr = src->ptr;
    dest->len = src->len;
    // TODO: Let the strpool know that src is "freed"
}

void test_strview_move(strpool *p) {
    strview dest = strpool_new(p, "tweedledee");
    strview src = strpool_new(p, "tweedledum");
    strview_move(p, &dest, &src);
    assert(!strcmp("tweedledum", strpool_get(p, dest)));
}

// f(s1: "dipple", s2: "dapple") -> s1: "dippledapple", s2: "dapple"
void strview_cat(strpool *p, strview *s1, strview *s2) {
    char temp[s1->len + s2->len+1];
    memcpy(temp, strpool_get(p, *s1), s1->len);
    memcpy(temp+s1->len, strpool_get(p, *s2), s2->len+1);
    strview svt = strpool_new(p, temp);
    strview_move(p, s1, &svt);
}

void test_strview_cat(strpool *p) {
    strview x = strpool_new(p, "con");
    strview y = strpool_new(p, "cat");
    strview_cat(p, &x, &y);
    assert(!strcmp("concat", strpool_get(p, x)));
    x = strpool_new(p, "");
    y = strpool_new(p, "cat");
    strview_cat(p, &x, &y);
    assert(!strcmp("cat", strpool_get(p, x)));
    x = strpool_new(p, "cat");
    y = strpool_new(p, "");
    strview_cat(p, &x, &y);
    assert(!strcmp("cat", strpool_get(p, x)));
    x = strpool_new(p, "");
    y = strpool_new(p, "");
    strview_cat(p, &x, &y);
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

char ALPHABET[] = "abcdefghijklmnopqrstuvwxyz";
/*void wgolf(strpool sp, dict d, strview dest, strview src, size_t start, size_t *n) {
    if (!strview_cmp(dest, src)) return;
    int ac = 0;
    for (char a = ALPHABET[ac]; a; ++ac) {
        for (int i = 0; i < src.len; ++i) {
            strview_delete_at(sp, src, i);
            strview word = strview_prependc(sp, s, dest.ptr);
            if (in_dict(strpool_get(sp, word))) {
                src = strview_move(src, word);
                *n++;
            }
        }
    }

    if (strview_cmp(dest, src))
        wgolf(dest, src, start, n);
}
*/

void test_strview() {
    strpool p = {0};
    test_strview_cmp(&p);
    test_strview_delete_at(&p);
    test_strview_replace_at(&p);
    test_strview_prepend(&p);
    test_strview_move(&p);
    test_strview_cat(&p);
    printf("all tests passed on strview\n");
}

int main(void) {
    strpool sp = {0};
    strview s = strpool_new(&sp, "wgolf 0.1.0");
    printf("%s\n", strpool_get(&sp, s));
    test_strview();

    dict d = load_dict(4);
    assert(27 == sizeof(ALPHABET));
}
