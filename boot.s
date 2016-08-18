global start
section .text
bits 64
start:
        extern main
        mov esp, stack_top
        call main       
        hlt

error:
        mov dword [0xb8000], 0x4f524f45
        mov dword [0xb8004], 0x4f3a4f52
        mov dword [0xb8008], 0x4f204f20
        mov byte  [0xb800a], al
        hlt

check_multiboot:
        cmp eax, 0x36d76289
        jne .no_multiboot
        ret

        .no_multiboot:
        mov al, "0"
        jmp error
        
section .bss
stack_bottom:
        resb 64
stack_top:
        
