/* Memory layout of the bl602 microcontroller */
/* 1K = 1 KiBi = 1024 bytes */
MEMORY
{
  FLASH (rx!w) : ORIGIN = 0x23000000, LENGTH = 2M

  /* Tightly-Coupled Memory bank 0 - ITCM is for program data and DTCM is for data.
   * NOTE: These are overlapping physical memory!
   */
  ITCM0          : ORIGIN = 0x22008000, LENGTH = 48K
  DTCM0          : ORIGIN = 0x42008000, LENGTH = 48K
  /*
   * Tightly-Coupled Memory bank 1 - ITCM is for program data and DTCM is for data.
   * NOTE: These are overlapping physical memory!
   */
  ITCM1          : ORIGIN = 0x22014000, LENGTH = 48K
  DTCM1          : ORIGIN = 0x42014000, LENGTH = 48K

  /* Wireless SRAM */
  WRAM           : ORIGIN = 0x42030000, LENGTH = 112K
  /* Deep sleep retention RAM */
  RETRAM         : ORIGIN = 0x40010000, LENGTH = 4K
}

ENTRY(main);

SECTIONS
{
  .text :
  {
    KEEP(*(.init));
    *(.text .text.*);
  } > FLASH
}
