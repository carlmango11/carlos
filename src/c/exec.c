#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>

#include "exec.h"
#include "io.h"

struct elf {
    uint64_t e_entry;
    uint16_t e_shnum;
};



void int_to_string(uint64_t value, char *buf) {
    char *p = buf;

    if (value == 0) {
        buf[0] = '0';
        buf[1] = '\0';
        return;
    }

    // Work with negative to handle INT_MIN safely
    int v = value;
    if (v > 0) {
        v = -v;
    }

    char *start = p;

    while (v != 0) {
        *p++ = '0' - (v % 10);
        v /= 10;
    }

    *p = '\0';

    // Reverse digits
    for (char *l = start, *r = p - 1; l < r; l++, r--) {
        char tmp = *l;
        *l = *r;
        *r = tmp;
    }
}

uint32_t read_32(uint8_t *data, size_t i) {
    uint32_t d1 = data[i];
    uint32_t d2 = data[i+1];
    uint32_t d3 = data[i+2];
    uint32_t d4 = data[i+3];

    return d1 | (d2 << 8) | (d3 << 16) | (d4 << 24);
}

uint64_t read_64(uint8_t *data, size_t x) {
    uint32_t val = 0;

    for (int i = 0; i < 8; i++) {
        uint64_t b = data[x+i];
        b <<= 8 * i;

        val |= b;
    }

    return val;
}

void load_program(uint8_t *data, size_t size) {
//    uint8_t header = data[0x20];

    uint64_t e_entry = read_64(data, 0x18);

    uint16_t e_shnum_lo = data[0x3C];
    uint16_t e_shnum_hi = data[0x3D];

    uint16_t section_c = e_shnum_lo | (e_shnum_hi << 8);

    char buf[12];
    int_to_string(section_c, buf);

    char buf2[12];
    int_to_string(e_entry, buf2);

    print_str(1, 0, TEXT_FORMAT, buf);
    print_str(2, 0, TEXT_FORMAT, buf2);
}