
begin   :
  top :
  loop
  :
lb x2, x1, data
beq x0, x2, end
sb x2, x1, to
addi x1, x1 , 1
sub x1, x1 , 1
console:
jal x0, loop sd
addi x1, x0, 0
addi x4, x0, 0x20
    lb x2, x1, console
    beq x0, x2, end
    beq x4, x2, change_case_end
    xori x2, x2, 0x4f4f
    sb x2, x1, console

change_case_end:
end:
jalr x0, x3, 0

