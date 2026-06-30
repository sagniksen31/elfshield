#include <stdio.h>

void vuln(char *s) {
    char buf[64];
    sprintf(buf, "%s", s);
}

int main(int argc, char **argv) {
    vuln(argv[1]);
}
