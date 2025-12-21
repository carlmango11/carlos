#include <stdint.h>
#include <stddef.h>

#include "exec.h"
#include "io.h"

void load_program(uint8_t *data, size_t size) {
    print_char(0, 0, TEXT_FORMAT, data[1]);
    print_char(1, 0, TEXT_FORMAT, data[2]);
    print_char(2, 0, TEXT_FORMAT, data[3]);
}