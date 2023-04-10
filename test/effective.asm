bits 16

mov cx, bx
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
mov ax, [bx + di - 37h]
mov [si - 18205], cx
mov dx, [bx - 32]
mov bx, [5b77h]
