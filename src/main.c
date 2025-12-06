#include <stdint.h>
#include <string.h>

extern void disable_interrupts();
extern void halt();

#define NO_KEY 0

int cursor_row = 0;
int cursor_col = 0;

extern void exec(int);

static inline uint8_t port_inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

static inline void port_outb(uint16_t port, uint8_t val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

static char scancode_to_ascii[128] = {
        [0x02] = '1',
        [0x03] = '2',
        [0x04] = '3',
        [0x05] = '4',
        [0x06] = '5',
        [0x07] = '6',
        [0x08] = '7',
        [0x09] = '8',
        [0x0A] = '9',
        [0x0B] = '0',

        [0x10] = 'q',
        [0x11] = 'w',
        [0x12] = 'e',
        [0x13] = 'r',
        [0x14] = 't',
        [0x15] = 'y',
        [0x16] = 'u',
        [0x17] = 'i',
        [0x18] = 'o',
        [0x19] = 'p',

        [0x1E] = 'a',
        [0x1F] = 's',
        [0x20] = 'd',
        [0x21] = 'f',
        [0x22] = 'g',
        [0x23] = 'h',
        [0x24] = 'j',
        [0x25] = 'k',
        [0x26] = 'l',

        [0x2C] = 'z',
        [0x2D] = 'x',
        [0x2E] = 'c',
        [0x2F] = 'v',
        [0x30] = 'b',
        [0x31] = 'n',
        [0x32] = 'm',

        [0x39] = ' ',     // space bar
        [0x1C] = '\n',    // Enter
        [0x0E] = '\b',    // Backspace
};

void print_char(int row, int col, int format, const char c) {
    volatile uint16_t *vga_buffer = (volatile uint16_t *)0xb8000;

    vga_buffer[col + (80 * row)] = format | c;
}

void print_str(int row, int col, int format, const char* msg) {
    int i = 0;
    while (msg[i] != '\0') {
        print_char(row, col, format, msg[i]);
        i++;
        col++;
    }
}

void print(const char *msg) {
    print_str(cursor_row, 0, 0x0f00, msg);
    cursor_row++;
}

void panic(const char *msg) {
    disable_interrupts();

    volatile uint16_t *vga_buffer = (volatile uint16_t *)0xb8000;

    for (int i = 0; i < 80 * 25; i++) {
        vga_buffer[i] = 0xfc00;
    }

    print_str(0, 0, 0xfc00, msg);

    halt();
    return;
}

char scancode_to_char(uint8_t code) {
    if (code & 0x80)
        return NO_KEY;  // ignore break codes

    if (code < 128)
        return scancode_to_ascii[code];

    panic("no key");
    return NO_KEY;
}

void keyboard_irt_handler() {
    uint8_t sc = port_inb(0x60);      // read keyboard scancode (ack IRQ1)

    char ch = scancode_to_char(sc);
    if (ch == NO_KEY) {
        return;
    }

    char c = scancode_to_ascii[sc];

    if (c == '\n') {
        cursor_row++;
        cursor_col = 0;
        return;
    }

    print_char(cursor_row, cursor_col, 0x0f00, c);
    cursor_col++;
}

int main(int loc, char **c) {
    print("welcome to CarlOS");

    exec(1);
    for (;;) {}

//    while(1) {
//        for (int i =0;i<300000000;i++) {}
//        print(".");
//    }
    return 0;
}