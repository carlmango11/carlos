#include <stdint.h>
#include <string.h>

// Type/Attribute flags
#define IDT_PRESENT     0x80
#define IDT_INT_GATE    0x0E  // 64-bit interrupt gate
#define IDT_TRAP_GATE   0x0F  // 64-bit trap gate
#define IDT_DPL_0       0x00  // Ring 0
#define IDT_DPL_3       0x60  // Ring 3

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
extern void page_fault_routine(void);

static inline uint8_t port_inb(uint16_t port) {
    uint8_t ret;
    __asm__ volatile ("inb %1, %0" : "=a"(ret) : "Nd"(port));
    return ret;
}

void idt_install() {
//return;
    idt_set_gate(0x21, (uint64_t)isr1, 0x08, 0, IDT_PRESENT | IDT_INT_GATE);
    idt_set_gate(0x0E, (uint64_t)page_fault_routine, 0x08, 0, IDT_PRESENT | IDT_INT_GATE);

    idtr.limit = sizeof(idt) - 1;
    idtr.base = (uint64_t)&idt;
    load_idt(&idtr);
}

static inline void port_outb(uint16_t port, uint8_t val) {
    __asm__ volatile ("outb %0, %1" : : "a"(val), "Nd"(port));
}
