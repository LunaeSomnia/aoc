#include "lib.h"

void rotate_dial(int *dial, int *times_pointing_at_0, enum Rotation direction, int amount) {
    int temp = *dial;
    if (direction == LEFT) {
        temp -= amount;
    } else {
        temp += amount;
    }

    *dial = temp % 100;

    if (*dial == 0) {
        *times_pointing_at_0 += 1;
    }
}
