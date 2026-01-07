#include <stdint.h>
#include <stddef.h>

#include "internal/io.h"
#include "internal/exec.h"

extern const uint8_t _binary_build_bin_hello_elf_start[];
extern const size_t  _binary_build_bin_hello_elf_size;

int main(int loc, char **c) {
    print("welcome to CarlOS");

//    const uint8_t *data = _binary_build_bin_hello_elf_start;
//    load_program(data, _binary_build_bin_hello_elf_size);

    main_rust(); // RUST
    for (;;) {}

//    while(1) {
//        for (int i =0;i<300000000;i++) {}
//        print(".");
//    }
    return 0;
}