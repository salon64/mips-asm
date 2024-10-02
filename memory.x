SECTIONS
{
  . = 0x0;
  .text :
  {
    KEEP(*(.text)); 
  }
  . = 0x1000;
  .rodata : {
    *(.srodata .srodata.*);
    *(.rodata .rodata.*);
  }
  .data :
  {
    KEEP(*(.data));  
  }
  /DISCARD/ : {
   *(.MIPS.abiflags)
   *(.reginfo)
  }
}
PROVIDE(_stack_start = 0x80000000);
