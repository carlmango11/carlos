PHONY: clean

clean:
	rm -rf build
	rm -rf target
	mkdir build

build/boot.o: asm/boot.asm
	nasm -f elf64 asm/boot.asm -o build/boot.o

build/multiboot_header.o: asm/multiboot_header.asm
	nasm -f elf64 asm/multiboot_header.asm -o build/multiboot_header.o

build/main.o: src/main.c
	x86_64-unknown-linux-gnu-gcc -Wall -Wextra -c src/main.c -o build/main.o

build/idt.o: src/idt.c
	x86_64-unknown-linux-gnu-gcc -Wall -Wextra -c src/idt.c -o build/idt.o

build/main64.o: asm/main64.asm
	nasm -f elf64 asm/main64.asm -o build/main64.o

build/lib.o: src/lib.rs
	rustc --target=x86_64-unknown-none -C opt-level=z -C relocation-model=static --emit=obj src/main.rs -o build/lib.o

build/kernel.elf: build/multiboot_header.o build/main64.o build/boot.o build/idt.o build/main.o
	cargo build
	x86_64-elf-ld -T linker.ld -o build/kernel.elf build/multiboot_header.o build/main64.o build/main.o build/idt.o build/boot.o target/x86_64-unknown-none/debug/libcarlos.a

build/kernel.iso: build/kernel.elf
	docker build -t kernel-build .
	docker create --name kernel-build kernel-build
	docker cp kernel-build:/app/kernel.iso ./build
	docker rm kernel-build
