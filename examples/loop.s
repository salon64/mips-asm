  .set noreorder
  .text
label:
  addi  $t0, $t0, 1
  bne   $zero, $t0, label
loop:
  b loop
  .data
