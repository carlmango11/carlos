#include <criterion/criterion.h>
#include "internal/exec.h"

Test(exec, parse_elf) {
    load_elf(0, 0);
}