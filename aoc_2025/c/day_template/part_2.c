#include "stdlib.h"
#include "stdio.h"
#include "lib.h"

int main() {
    char *year = getenv("YEAR");
    char *day = getenv("DAY");
    printf("%s %s\n", year, day);
}
