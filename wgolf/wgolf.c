// strpool - what's an allocation?
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#define BUFSIZE 333
#define max(a,b) (((a)>(b))?(a):(b))
static struct {
    char buf[BUFSIZE];
    size_t i; // should always point to '\0'
} strpool = { .i = 0, .buf = {[0] = '\0'} };

typedef struct strview strview;
struct strview {
    size_t ptr;
    size_t len;
};

strview strview_default() {
    strpool.buf[strpool.i] = '\0';
    return (strview){ .ptr = max(0, strpool.i-1), .len = 0 };
}

// potential buffer overflow
strview strview_from(char *s) {
    size_t saved = strpool.i;
    size_t len = 0;
    while (*s) {
        strpool.buf[strpool.i] = *s;
        strpool.i++;
        len++;
        s++;
    }
    strpool.buf[strpool.i] = '\0';

    return (strview){ .ptr = saved, .len = len };
}

char *strview_get(strview s) {
    return strpool.buf+s.ptr;
}

void strview_pushc(strview s, char c) {
    strpool.buf[s.ptr] = c;
    s.len++;
}

void strview_print_debug(strview s) {
    printf("{.ptr=%ld, .len=%ld} ", s.ptr, s.len);
    size_t sp = s.ptr;
    while (strpool.buf[sp]) {
       printf("'%c'", strpool.buf[sp]);
       sp++;
    }
    printf("\n");
}

int strview_cmp(strview s1, strview s2) {
    size_t sp1 = s1.ptr;
    size_t sp2 = s2.ptr;
    size_t a = 0, b = 0;

    while (!strpool.buf[sp1]) {
        a += strpool.buf[sp1];
        sp1++;
    }
    while (!strpool.buf[sp2]) {
        b += strpool.buf[sp2];
        sp2++;
    }
    return a-b == 0 ? 0 : a-b > 0 ? 1 : -1;
}

int main(void) {
    char bytes[33];
    int urandom = open("/dev/urandom", O_RDONLY);
    for (int i = 0; i < 10; ++i) {
        read(urandom, bytes, sizeof(bytes));
        strview s = strview_from(bytes);
        printf("%s\n", strview_get(s));
    }
}
