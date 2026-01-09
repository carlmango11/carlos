CC := x86_64-unknown-linux-gnu-gcc
LD := x86_64-elf-ld
CFLAGS := -Wall -Wextra -fno-optimize-sibling-calls -fno-omit-frame-pointer -O0 -g
BUILDDIR := build

SRCS := $(wildcard src/c/*.c)
OBJS := $(SRCS:src/c/%.c=$(BUILDDIR)/%.o)

ASM_SRCS := $(wildcard src/asm/*.asm)
ASM_OBJS := $(ASM_SRCS:src/asm/%.asm=$(BUILDDIR)/%_asm.o)

BUNDLED_BIN_SRCS := $(wildcard src/programs/*.c)
BUNDLED_BIN_OBJS := $(BUNDLED_BIN_SRCS:src/programs/%.c=$(BUILDDIR)/bin/%.o)
BUNDLED_BIN_ELFS := $(BUNDLED_BIN_SRCS:src/programs/%.c=$(BUILDDIR)/bin/%.elf)

all: build/kernel.iso

$(BUILDDIR)/kernel.elf: $(OBJS) $(ASM_OBJS) $(BUNDLED_BIN_OBJS) target/x86_64-unknown-none/debug/libcarlos.a
	$(LD) -T linker.ld -o $(BUILDDIR)/kernel.elf $(BUILDDIR)/*.o $(BUILDDIR)/bin/*.o target/x86_64-unknown-none/debug/libcarlos.a

$(BUILDDIR)/bin/%.o: $(BUILDDIR)/bin/%.elf | $(BUILDDIR)/bin
	objcopy --input-target binary \
			--output-target elf64-x86-64 \
			--binary-architecture i386:x86-64 \
			$< $@

$(BUILDDIR)/bin/%.elf: src/programs/%.c | $(BUILDDIR)/bin
	$(CC) $(CFLAGS) -static -ffreestanding -nostdlib $< -o $@

$(BUILDDIR)/%_asm.o: src/asm/%.asm | $(BUILDDIR)
	nasm -f elf64 $< -o $@

$(BUILDDIR)/%.o: src/c/%.c | $(BUILDDIR)
	$(CC) $(CFLAGS) -c $< -o $@

$(BUILDDIR)/bin:
	mkdir -p $@

$(BUILDDIR):
	mkdir -p $@

test:
	#gcc -Iinclude -Isrc/c tests/test_exec.c $(shell pkg-config --cflags --libs criterion) -o $(BUILDDIR)/run_tests; $(BUILDDIR)/run_tests
	gcc -Iinclude -Isrc/c tests/test_exec.c $(shell pkg-config --cflags --libs criterion) -o $(BUILDDIR)/run_tests

PHONY: clean

clean:
	rm -rf build
	rm -rf target

target/x86_64-unknown-none/debug/libcarlos.a: src/rust/*
	cargo build

target/x86_64-unknown-none/debug/libcarlos.a: src/rust/*
	RUSTFLAGS="--emit asm" cargo build

build/kernel.iso: $(BUILDDIR)/kernel.elf
	docker build -t kernel-build .
	docker create --name kernel-build kernel-build
	docker cp kernel-build:/app/kernel.iso ./build
	docker rm kernel-build