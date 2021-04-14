/* If having stack corruption errors, do a full clean and rebuild */
/* since every binary must use the same memory layout scheme. */

MEMORY
{
  FLASH (rx)      : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM (xrw)       : ORIGIN = 0x20000000, LENGTH = 128K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
