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
    size_t i;
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
strview strview_from(strpool *pool, char *s) {
    size_t saved = pool->i;
    size_t len = 0;
    while (*s) {
        pool->buf[pool->i] = *s;
        pool->i++;
        len++;
        s++;
    }
    pool->buf[pool->i++] = '\0';

    if (pool->i >= BUFSIZE) {
        fprintf(stderr, "strpool buffer overflow\n");
        exit(EXIT_FAILURE);
    } 
    
    pool->size++;
    return (strview){ .ptr = saved, .len = len };
}

char *strview_get(strpool *pool, strview s) {
    return pool->buf+s.ptr;
}

// print ith string in pool
void strpool_print(strpool *pool, size_t i) {
    assert(i < pool->size);
    size_t count = 0;
    size_t sp = 0;
    while (count < i) {
        while (pool->buf[sp++]);
        count++;
    }
    
    // sp points to specified string
    printf("'%s'\n", &pool->buf[sp]);
}

void strview_pushc(strpool *pool, strview s, char c) {
    pool->buf[s.ptr] = c;
    s.len++;
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
    strview a = strview_from(p, "canoe");
    strview b = strview_from(p, "canon");
    assert(-1 == strview_cmp(p, a, b));

    a = strview_from(p, "");
    b = strview_from(p, "a");
    assert(-1 == strview_cmp(p, a, b));

    a = strview_from(p, "A");
    b = strview_from(p, "a");
    assert(-1 == strview_cmp(p, a, b));
    
    a = strview_from(p, "cil");
        b = strview_from(p, "cml");
    assert(-1 == strview_cmp(p, a, b));
    printf("tests passed on strview_cmp\n");
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
        strview_from(&d.pool, line);
    }
    return d;
}

char ALPHABET[] = "abcdefghijklmnopqrstuvwxyz";
int main(void) {
    strpool sp = {0};
    strview s = strview_from(&sp, "wgolf 0.1.0");
    printf("%s\n", strview_get(&sp, s));
    test_strview_cmp(&sp);

    dict d = load_dict(4);
    strpool_print(&d.pool, 0);
    assert(27 == sizeof(ALPHABET));
}
