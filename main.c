#define true 1
#define false 0

#define AAA

typedef _Bool bool;

#ifdef AAA

bool and(bool a, bool b) {
    return a && b;
}

#endif
