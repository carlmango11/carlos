PHONY: clean

clean:
	rm -rf build
	rm -rf target
	mkdir build

build/multiboot_header.o: asm/multiboot_header.asm
	nasm -f elf64 asm/multiboot_header.asm -o build/multiboot_header.o

build/kernel.o: src/main.rs
	rustc --target=x86_64-unknown-none -C opt-level=z -C relocation-model=static --emit=obj src/main.rs -o build/kernel.o

build/kernel.asm: src/main.rs
	rustc -C panic=abort --target=x86_64-unknown-none -C opt-level=z -C relocation-model=static --emit=asm src/main.rs -o build/kernel.asm

build/kernel.elf: build/multiboot_header.o build/kernel.o
	x86_64-elf-ld -m64 -T linker.ld -o build/kernel.elf build/multiboot_header.o build/kernel.o

kernel.iso: build/kernel.elf
	docker build -t kernel-build .
	docker create --name kernel-build kernel-build
	docker cp kernel-build:/app/kernel.iso ./build
	docker rm kernel-build