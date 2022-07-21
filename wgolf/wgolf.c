// strpool - what's an allocation?
#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <fcntl.h>
#define BUFSIZE 999999
#define max(a,b) (((a)>(b))?(a):(b))
static struct {
    char buf[BUFSIZE];
    size_t i; // should always point to '\0'
    size_t size;
} strpool = { .i = 0, .size = 0, .buf = {[0] = '\0'} };


typedef struct strview strview;
struct strview {
    size_t ptr;
    size_t len;
};

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
    strpool.buf[strpool.i++] = '\0';

    if (strpool.i >= BUFSIZE) {
        fprintf(stderr, "strpool buffer overflow\n");
        exit(EXIT_FAILURE);
    } 
    
    strpool.size++;
    return (strview){ .ptr = saved, .len = len };
}

char *strview_get(strview s) {
    return strpool.buf+s.ptr;
}

// print ith string in pool
void strpool_print(size_t i) {
    assert(i < strpool.size);
    size_t count = 0;
    size_t sp = 0;
    while (count < i) {
        while (strpool.buf[sp++]);
        count++;
    }

    printf("sp:%lu\n", sp);

    // sp points to specified string
    char c;
    while ((c = strpool.buf[sp++]))
        putc(c, stdout); 
    putc('\n', stdout);
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
    int a = 0, b = 0;

    while (strpool.buf[sp1]) {
        a += strpool.buf[sp1];
        sp1++;
    }
    while (strpool.buf[sp2]) {
        b += strpool.buf[sp2];
        sp2++;
    }
    return !(a-b) ? 0 : a-b > 0 ? 1 : -1;
}

void test_strview_cmp() {
    strview a = strview_from("canoe");
    strview b = strview_from("canon");
    assert(-1 == strview_cmp(a, b));

    a = strview_from("");
    b = strview_from("a");
    assert(-1 == strview_cmp(a, b));

    a = strview_from("A");
    b = strview_from("a");
    assert(-1 == strview_cmp(a, b));
    
    a = strview_from("cil");
    b = strview_from("cml");
    assert(-1 == strview_cmp(a, b));
    printf("tests passed on strview_cmp\n");
}

size_t load_dict() {
    FILE *fp = fopen("/usr/share/dict/words", "r");
    if (!fp) {
        fprintf(stderr, "Missing '/usr/share/dict/words'.\n");
        exit(EXIT_FAILURE);
    }
    
    char *line = NULL;
    size_t len = 0;
    ssize_t nread;
    nread = getline(&line, &len, fp);
    size_t first = strview_from(line).len;
    while ((nread = getline(&line, &len, fp)) != -1) {
        line[strcspn(line, "\n")] = 0;
        for (int i = 0; line[i]; ++i)
            line[i] = (char)tolower(line[i]);
        strview_from(line);
    }
    return first;
}

char ALPHABET[] = "abcdefghijklmnopqrstuvwxyz";
/*void wgolf(strview dest, strview src, size_t start, size_t *n) {
    if (!strview_cmp(dest, src)) return;
    int ac = 0;
    for (char a = ALPHABET[ac]; a; ++ac) {
        for (int i = 0; i < src.len; ++i) {
            strview s = strview_remove_at(src, i);
            strview word = strview_prependc(s, dest.ptr);
            if in_dict(word) {
                strview_copy(src, word);
                *n++;
            }
        }
    }

    if (strview_cmp(dest, src))
        wgolf(dest, src, start, n);
}
*/

int main(void) {
    strview s = strview_from("wgolf 0.1.0");
    printf("%s\n", strview_get(s));
    size_t dp = load_dict();
    test_strview_cmp();
    assert(27 == sizeof(ALPHABET));
}
