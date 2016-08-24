global outport_b
global inport_b
section .text
outport_b:
        mov dx, si
        mov ax, di
        out dx, al
        ret
inport_b:
        mov dx, di
        in al, dx
        ret
