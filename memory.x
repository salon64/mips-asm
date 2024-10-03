SECTIONS
{
  . = 0x0;
  .text :
  {
    KEEP(*(.text)); 
  }
  . = ALIGN(4);
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
   *(.comment)
   *(.pdr)
   *(.mdebug.abi32)
  }
}
PROVIDE(_stack_start = 0x80000000);
