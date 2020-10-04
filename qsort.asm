default rel

section .text
global swap
global choose_pivot
global partition
global qsort

swap:
    ; <- tab (rdi): int array
    ; <- i   (rsi): int
    ; <- j   (rdx): int
    mov rax, [rdi+rsi*8]      ; tmpj <- tab[i]
    mov rbx, [rdi+rdx*8]      ; tmpi <- tab[j]
    mov [rdi+rsi*8], rbx      ; tab[i] <- tmpi
    mov [rdi+rdx*8], rax      ; tab[j] <- tmpj
    ret

choose_pivot:
    ; <- tab (rdi): int array
    ; <- lo  (rsi): int
    ; <- hi  (rdx): int
    ; -> int (eax)
    mov rax, [rdi+rsi*8]      ; ret <- tab[lo]
    ret

partition:
    ; <- tab (rdi): int array
    ; <- lo  (rsi): int
    ; <- sm  (rdx): int*        (r11)
    ; <- eq  (rcx): int*
    ; <- hi   (r8): int
    ; <- pv   (r9): int
    ; == gt  (r10): int
    mov r10, rsi              ; gt <- lo
    mov r11, [rdx]            ; copy sm
    push rdx                  ; save &sm
    push rcx                  ; save &eq
    mov rcx, [rcx]            ; copy eq
  .while:
    cmp r10, r8               ; gt <=> hi
    jge .exit
    mov rax, [rdi+r10*8]      ;     read tab[gt]
    cmp rax, r9               ;     tab[gt] <=> pv
    jl .case_sm
    je .case_eq
    jg .case_gt
  .case_sm:                   ; --- when tab[gt] < pv
    mov rsi, rcx              ;     load eq
    mov rdx, r10              ;     load gt
    call swap
    mov rsi, r11              ;     load sm
    mov rdx, rcx              ;     load eq
    call swap
    inc r10                   ;     gt++
    inc rcx                   ;     eq++
    inc r11                   ;     sm++
    jmp .endif
  .case_eq:                   ; --- when tab[gt] = pv
    mov rsi, rcx              ;     load eq
    mov rdx, r10              ;     load gt
    call swap
    inc r10                   ;     gt++
    inc rcx                   ;     eq++
    jmp .endif
  .case_gt:                   ; --- when tab[gt] > pv
    inc r10                   ;     gt++
    jmp .endif
  .endif:
    jmp .while
  .exit:
    pop rax                   ; reload eq to its address
    mov [rax], rcx
    pop rax                   ; reload sm to its address
    mov [rax], r11
    ret

qsort:
    ; <- tab (rdi)
    ; <- lo  (rsi)
    ; <- hi  (rdx)
    cmp rsi, rdx              ; lo <=> hi
    je .end
    push rdx                  ; save hi
    call choose_pivot         ; (rax) <- pv
    push rsi                  ; create local sm
    push rsi                  ; create local eq
    push rsi                  ; save lo
    mov r8, rdx               ; load hi
    lea rdx, [rsp+8]          ; load &sm
    lea rcx, [rsp+16]         ; load &eq
    mov r9, rax               ; load pv
    call partition            ; partition(tab, lo, &sm, &eq, hi, pv)
    pop rsi                   ; load lo
    pop rdx                   ; load sm
    call qsort                ; qsort_aux(tab, lo, sm)
    pop rsi                   ; load eq
    pop rdx                   ; load hi
    call qsort                ; qsort_aux(tab, eq, hi)
  .end:
    ret
