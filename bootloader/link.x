/* Memory layout of the bl602 microcontroller */
/* 1K = 1 KiBi = 1024 bytes */
MEMORY
{
  RAM : ORIGIN = 0x20080000, LENGTH = 272K
  FLASH : ORIGIN = 0x23000000, LENGTH = 2M
}

ENTRY(reset);

SECTIONS
{
  .text :
  {
    KEEP(*(.init));
    *(.text .text.*);
  } > FLASH
}
