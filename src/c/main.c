#include <stdint.h>
#include <stddef.h>

#include "internal/io.h"
#include "internal/exec.h"

int main(int loc, char **c) {
    print("welcome to CarlOS");

    main_rust(); // RUST
    for (;;) {}
    return 0;
}