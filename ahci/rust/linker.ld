/* LoongArch64 Linker Script for AHCI Driver with Alignment Control */

SECTIONS
{
    /* Ensure all sections are properly aligned */
    .text : ALIGN(0x1000) {
        *(.text*)
    }
    
    .rodata : ALIGN(0x1000) {
        *(.rodata*)
    }
    
    .data : ALIGN(0x1000) {
        *(.data*)
    }
    
    .bss : ALIGN(0x1000) {
        *(.bss*)
    }
    
    /* Ensure all global symbols are 8-byte aligned */
    . = ALIGN(8);
    
    /* Debug sections */
    .debug_info     0 : { *(.debug_info) }
    .debug_abbrev   0 : { *(.debug_abbrev) }
    .debug_line     0 : { *(.debug_line) }
    .debug_frame    0 : { *(.debug_frame) }
    .debug_str      0 : { *(.debug_str) }
    .debug_loc      0 : { *(.debug_loc) }
    .debug_ranges   0 : { *(.debug_ranges) }
}
