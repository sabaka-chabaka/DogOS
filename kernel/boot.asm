section .multiboot
align 4

MAGIC equ 0x1BADB002
FLAGS equ 0x00000003
CHECKSUM equ -(MAGIC + FLAGS)

dd MAGIC
dd FLAGS
dd CHECKSUM

section .bss
align 16

stack_bottom:
    resb 16384
stack_top:

section .text
global _start
extern kernel_main

_start:
    mov esp, stack_top

    call kernel_main

.hang:
    cli
    hlt
    jmp .hang