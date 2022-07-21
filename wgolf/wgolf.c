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
strview strview_from(strpool *pool, char *s) {
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

// f('lewis',3) -> 'lews'
void strview_delete_at(strpool *p, strview s, size_t i) {
    assert(i < s.len);
    memcpy(strpool_get(p,s)+i, strpool_get(p,s)+i+1, s.len-i+1);
    s.len--;
}

void strview_replace_at(strpool *p, strview s, char c, size_t i) {
    assert(i < s.len);
    *(strpool_get(p, s)+i) = c;
}

strview strview_prepend_at(strpool *p, strview s, char c, size_t i) {
    // cheat by making new str in pool
    char new[s.len+1];
    new[0] = c;
    memcpy(new+1, strpool_get(p,s), s.len+1);
    return strview_from(p, new);
}

// src is invalidated
strview strview_mov(strpool *p, strview dest, strview src) {
    if (dest.len == src.len) {
        strcpy(strpool_get(p, dest), strpool_get(p, src));
        dest.len = src.len;
    } else if (dest.len > src.len) {
        memcpy(strpool_get(p, dest), strpool_get(p, src), src.len+1);
        dest.len = src.len;
    } else {
        strview new = strview_from(p, strpool_get(p, src));
        dest.ptr = new.ptr;
        dest.len = new.len;
    }
    return dest;
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
    printf("%s\n", strpool_get(&sp, s));
    test_strview_cmp(&sp);

    dict d = load_dict(4);
    assert(27 == sizeof(ALPHABET));
}
