bits 16

; Data transfer
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

mov [bp + di], byte 7
mov [bx + si - 1025], byte 1
mov [di + 901], word 347

mov ax, 0x4f02
mov bx, 0x01
mov cl, 0x85
mov dl, 0x05

mov ax, [2555]
mov al, [16]

mov [2554], ax
mov [15], al

mov cs, ax
mov es, [1234]

mov [1234], ss
mov ax, ds

push word [bp + si]
push word [3000]
push word [bx + di - 30]
push cx
push ax
push dx
push cs

pop word [bp + si]
pop word [3]
pop word [bx + di - 3000]
pop sp
pop di
pop si
pop ds

xchg ax, [bp - 1000]
xchg [bx + 50], bp

xchg ax, ax
xchg ax, dx
xchg ax, sp
xchg ax, si
xchg ax, di

xchg cx, dx
xchg si, cx
xchg cl, ah

in al, 200
in al, dx
in ax, dx

out 44, ax
out dx, al

xlat
lea ax, [bx + di + 1420]
lea bx, [bp - 50]
lea sp, [bp - 1003]
lea di, [bx + si - 7]

lds ax, [bx + di + 1420]
lds bx, [bp - 50]
lds sp, [bp - 1003]
lds di, [bx + si - 7]

les ax, [bx + di + 1420]
les bx, [bp - 50]
les sp, [bp - 1003]
les di, [bx + si - 7]

lahf
sahf
pushf
popf

; Arithmetic
add bx, [bx+si]
add bx, [bp]
add si, 2
add bp, 2
add cx, 8
add bx, [bp + 0]
add cx, [bx + 2]
add bh, [bp + si + 4]
add di, [bp + di + 6]
add [bx+si], bx
add [bp], bx
add [bp + 0], bx
add [bx + 2], cx
add [bp + si + 4], bh
add [bp + di + 6], di
add byte [bx], 34
add word [bp + si + 1000], 29
add ax, [bp]
add al, [bx + si]
add ax, bx
add al, ah
add ax, 1000
add al, -30
add al, 9

adc cx, [bp]
adc dx, [bx + si]
adc [bp + di + 5000], ah
adc [bx], al
adc sp, 392
adc si, 5
adc ax, 1000
adc ah, 30
adc al, 9
adc cx, bx
adc ch, al

inc ax
inc cx
inc dh
inc al
inc ah
inc sp
inc di
inc byte [bp + 1002]
inc word [bx + 39]
inc byte [bx + si + 5]
inc word [bp + di - 10044]
inc word [9349]
inc byte [bp]

aaa
daa

sub bx, [bx+si]
sub bx, [bp]
sub si, 2
sub bp, 2
sub cx, 8
sub bx, [bp + 0]
sub cx, [bx + 2]
sub bh, [bp + si + 4]
sub di, [bp + di + 6]
sub [bx+si], bx
sub [bp], bx
sub [bp + 0], bx
sub [bx + 2], cx
sub [bp + si + 4], bh
sub [bp + di + 6], di
sub byte [bx], 34
sub word [bx + di], 29
sub ax, [bp]
sub al, [bx + si]
sub ax, bx
sub al, ah
sub ax, 1000
sub al, -30
sub al, 9

sbb cx, [bp]
sbb dx, [bx + si]
sbb [bp + di + 5000], ah
sbb [bx], al
sbb sp, 392
sbb si, 5
sbb ax, 1000
sbb ah, 30
sbb al, 9
sbb cx, bx
sbb ch, al

dec ax
dec cx
dec dh
dec al
dec ah
dec sp
dec di
dec byte [bp + 1002]
dec word [bx + 39]
dec byte [bx + si + 5]
dec word [bp + di - 10044]
dec word [9349]
dec byte [bp]

neg ax
neg cx
neg dh
neg al
neg ah
neg sp
neg di
neg byte [bp + 1002]
neg word [bx + 39]
neg byte [bx + si + 5]
neg word [bp + di - 10044]
neg word [9349]
neg byte [bp]

cmp bx, [bx+si]
cmp bx, [bp]
cmp si, 2
cmp bp, 2
cmp cx, 8
cmp bx, [bp + 0]
cmp cx, [bx + 2]
cmp bh, [bp + si + 4]
cmp di, [bp + di + 6]
cmp [bx+si], bx
cmp [bp], bx
cmp [bp + 0], bx
cmp [bx + 2], cx
cmp [bp + si + 4], bh
cmp [bp + di + 6], di
cmp byte [bx], 34
cmp word [4834], 29
cmp ax, [bp]
cmp al, [bx + si]
cmp ax, bx
cmp al, ah
cmp ax, 1000
cmp al, -30
cmp al, 9

aas
das

mul al
mul cx
mul word [bp]
mul byte [bx + di + 500]

imul ch
imul dx
imul byte [bx]
imul word [9483]

aam

div bl
div sp
div byte [bx + si + 2990]
div word [bp + di + 1000]

idiv ax
idiv si
idiv byte [bp + si]
idiv word [bx + 493]

aad
cbw
cwd

; Logic
not ah
not bl
not sp
not si
not word [bp]
not byte [bp + 9905]

shl ah, 1
shr ax, 1
sar bx, 1
rol cx, 1
ror dh, 1
rcl sp, 1
rcr bp, 1

shl word [bp + 5], 1
shr byte [bx + si - 199], 1
sar byte [bx + di - 300], 1
rol word [bp], 1
ror word [4938], 1
rcl byte [3], 1
rcr word [bx], 1

shl ah, cl
shr ax, cl
sar bx, cl
rol cx, cl
ror dh, cl
rcl sp, cl
rcr bp, cl

shl word [bp + 5], cl
shr word [bx + si - 199], cl
sar byte [bx + di - 300], cl
rol byte [bp], cl
ror byte [4938], cl
rcl byte [3], cl
rcr word [bx], cl

and al, ah
and ch, cl
and bp, si
and di, sp
and al, 93
and ax, 20392
and [bp + si + 10], ch
and [bx + di + 1000], dx
and bx, [bp]
and cx, [4384]
and byte [bp - 39], 239
and word [bx + si - 4332], 10328

test bx, cx
test dh, [bp + 390]
test [bp + 2], si
test bl, 20
test byte [bx], 34
test ax, 23909

or al, ah
or ch, cl
or bp, si
or di, sp
or al, 93
or ax, 20392
or [bp + si + 10], ch
or [bx + di + 1000], dx
or bx, [bp]
or cx, [4384]
or byte [bp - 39], 239
or word [bx + si - 4332], 10328

xor al, ah
xor ch, cl
xor bp, si
xor di, sp
xor al, 93
xor ax, 20392
xor [bp + si + 10], ch
xor [bx + di + 1000], dx
xor bx, [bp]
xor cx, [4384]
xor byte [bp - 39], 239
xor word [bx + si - 4332], 10328

rep movsb
rep cmpsb
rep scasb
rep lodsb
rep movsw
rep cmpsw
rep scasw
rep lodsw

rep stosb
rep stosw

; Control transfer

; why is this not working? nasm outputs something else than 11500.
; call 11500
call [39201]
call [bp - 100]
call sp
call ax
call 123:456
call [bp+si-0x3a]
call far [bp+si-0x3a]

jmp ax
jmp di
jmp [12]
jmp [4395]
jmp 789:34
; Same thing as "call 11500" above. Nasm outputs a different get_disp_value
; for the direct within segment operations.
; jmp 2620
jmp [di]
jmp far [di]
jmp 21862:30600

test_label0:
jnz test_label1
jnz test_label0
test_label1:
jnz test_label0
jnz test_label1

label:
je label
jl label
jle label
jb label
jbe label
jp label
jo label
js label
jne label
jnl label
jg label
jnb label
ja label
jnp label
jno label
jns label
loop label
loopz label
loopnz label
jcxz label
