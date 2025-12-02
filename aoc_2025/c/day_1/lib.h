#ifndef LIB_H
#define LIB_H

#define MIN(X,Y) ((X) < (Y) ? (X) : (Y))

enum Rotation {
    LEFT,
    RIGHT,
};

void rotate_dial(int *dial, int *times_pointing_at_0, enum Rotation direction, int amount);

#endif
