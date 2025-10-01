FROM --platform=linux/amd64 debian:latest

WORKDIR /app

RUN apt-get update
RUN DEBIAN_FRONTEND=noninteractive apt-get install  -y --force-yes grub2-common grub-pc-bin
RUN DEBIAN_FRONTEND=noninteractive apt-get install  -y --force-yes xorriso mtools file genisoimage

COPY iso ./iso/
COPY build/kernel.elf ./iso/boot/kernel.elf
RUN grub-mkrescue -o kernel.iso iso