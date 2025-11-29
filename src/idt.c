#include <stdint.h>
#include <string.h>

// Type/Attribute flags
#define IDT_PRESENT     0x80
#define IDT_INT_GATE    0x0E  // 64-bit interrupt gate
#define IDT_TRAP_GATE   0x0F  // 64-bit trap gate
#define IDT_DPL_0       0x00  // Ring 0
#define IDT_DPL_3       0x60  // Ring 3

extern disable_interrupts();
extern halt();

// IDT Entry Structure for Long Mode (16 bytes)
typedef struct {
    uint16_t offset_low;    // Offset bits 0-15
    uint16_t selector;      // Code segment selector
    uint8_t  ist;           // Interrupt Stack Table (bits 0-2)
    uint8_t  type_attr;     // Type and attributes
    uint16_t offset_mid;    // Offset bits 16-31
    uint32_t offset_high;   // Offset bits 32-63
    uint32_t zero;          // Reserved, must be zero
} __attribute__((packed)) idt_entry_t;

// IDTR Structure
typedef struct {
    uint16_t limit;         // Size of IDT - 1
    uint64_t base;          // Base address of IDT
} __attribute__((packed)) idtr_t;

// IDT with 256 entries
static idt_entry_t idt[256];
static idtr_t idtr;

// Set an IDT entry
void idt_set_gate(uint8_t num, uint64_t handler, uint16_t selector,
                         uint8_t ist, uint8_t type_attr) {
    idt[num].offset_low = handler & 0xFFFF;
    idt[num].selector = selector;
    idt[num].ist = ist & 0x07;
    idt[num].type_attr = type_attr;
    idt[num].offset_mid = (handler >> 16) & 0xFFFF;
    idt[num].offset_high = (handler >> 32) & 0xFFFFFFFF;
    idt[num].zero = 0;
}

extern void idt_flush(uint64_t);
extern void isr1(void);

void main(int loc, char c) {
    init_scan_codes();
    print("welcome to CarlOS");

    while(1) {
        for (int i =0;i<300000000;i++) {}
        print(".");
    }
}

int line = 0;

static inline uint8_t port_inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

void idt_install() {
    idt_set_gate(0x21, (uint64_t)isr1, 0x08, 0, IDT_PRESENT | IDT_INT_GATE);

    idtr.limit = sizeof(idt) - 1;
    idtr.base = (uint64_t)&idt;
    load_idt(&idtr);
}

static inline void port_outb(uint16_t port, uint8_t val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}

#define NO_KEY 0

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

void init_scan_codes() {
    return;
    for (int i = 0; i < 128; i++) {
        if (scancode_to_ascii[i] != '\0') {
            scancode_to_ascii[i] = '\0';
        }
    }
}

char scancode_to_char(uint8_t code) {
    if (code & 0x80)
        return NO_KEY;  // ignore break codes

    if (code < 128)
        return scancode_to_ascii[code];

    panic("no key");
    return NO_KEY;
}

void isr_handler() {
    uint8_t sc = port_inb(0x60);      // read keyboard scancode (ack IRQ1)

    char ch = scancode_to_char(sc);
    if (ch == NO_KEY || ch == 0) {
        return;
    }

    char dd = scancode_to_ascii[sc];
    char str[2];
    str[0] = dd;
    str[1] = '\0';
    print(&str);
}

void print(const char *msg) {
    print_str(line, 0, 0x0f00, msg);
    line++;
}

void print_str(int row, int col, int format, const char *msg) {
    volatile uint16_t *vga_buffer = (volatile uint16_t *)0xb8000;

    int i = 0;
    while (msg[i] != '\0') {
        if (msg[i] >= 128) {
            panic("invalid char");
        }

        vga_buffer[i + (80 * row)] = format | msg[i];
        i++;
    }
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