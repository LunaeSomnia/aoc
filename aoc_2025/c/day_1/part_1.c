#include "stdlib.h"
#include "stdio.h"
#include "string.h"
#include "lib.h"

#define MAX_LINE_LENGTH 100

int main(void) {
    int dial = 50;
    int times_pointing_at_0 = 0;

    char line[MAX_LINE_LENGTH];
    while (fgets(line, MAX_LINE_LENGTH, stdin))
    {
        int line_length = strlen(line);
        line[line_length-1] = '\0';

        enum Rotation rotation_dir = LEFT;
        if (line[0] == 'R') {
            rotation_dir = RIGHT;
        }

        char amount_buffer[4];
        int copy_amount = MIN(line_length - 1, 4);
        strncpy(amount_buffer, line + 1, copy_amount);
        amount_buffer[3] = '\0';
        int amount = atoi(amount_buffer);

        rotate_dial(&dial, &times_pointing_at_0, rotation_dir, amount);
    }

    printf("Password is %d.\n", times_pointing_at_0);

    return 0;
}
