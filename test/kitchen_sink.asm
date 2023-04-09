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

; Immediate to register
mov bx, [5b77h]
mov ax, 0x4f02
mov bx, 0x01
mov cl, 0x85
mov dl, 0x05
