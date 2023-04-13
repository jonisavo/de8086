bits 16

; Register/memory to/from register
mov cx, bx
mov ch, ah
mov dx, bx
mov si, bx
mov bx, di
mov al, cl
mov ch, ch
mov bx, ax
mov bx, si
mov sp, di
mov bp, ax

mov bx, [bp + 4h]
mov [4h], bx
mov si, [bx + si + 785h]
mov bx, [di]
mov [si], cx
mov [bx + di], cx
mov bx, [bp + 1h]
mov bx, [bx + 5h]
mov [bp + di + 6f69h], di
mov bp, [bp + si + 2h]
mov bx, [5b77h]
mov ax, [bx + di - 37h]
mov [si - 300h], cx
mov dx, [bx - 32h]
mov bp, [5]

; Immediate to register / memory

mov [bp + di], byte 7
mov [bx + si - 1025], byte 1
mov [di + 901], word 347

; Immediate to register
mov ax, 0x4f02
mov bx, 0x01
mov cl, 0x85
mov dl, 0x05

; Memory-to-accumulator
mov ax, [2555]
mov ax, [16]

; Accumulator-to-memory
mov [2554], ax
mov [15], ax
