default rel

section .text
global swap
global choose_pivot
global partition

swap:
    ; <- tab (rdi): int array
    ; <- i   (rsi): int
    ; <- j   (rdx): int
    mov rax, qword [rdi+rsi*8]    ; tmpj <- tab[i]
    mov rbx, qword [rdi+rdx*8]    ; tmpi <- tab[j]
    mov qword [rdi+rsi*8], rbx    ; tab[i] <- tmpi
    mov qword [rdi+rdx*8], rax    ; tab[j] <- tmpj
    ret

