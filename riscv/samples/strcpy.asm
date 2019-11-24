;;
;; stringcopy
;;

.section .data

data: .byte "RISC V Instruction Set"

.equ  tx80, 0xTFAB80    ; = 128
.equ  t800, 800     ;
.equ  t0800, 0800   ;

.section .text
loop:
    lb x2, x1, data
    beq x0, x2, end
    sb x2, x1, to
    addi x1,x1,1
    jal x0, loop

end:
