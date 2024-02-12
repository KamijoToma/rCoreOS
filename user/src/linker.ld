OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80400000;

SECTIONS
{
    . = BASE_ADDRESS;
    slib = .;

    lib_stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    lib_etext = .;
    lib_srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    lib_erodata = .;
    lib_sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    lib_edata = .;
    .bss : {
        *(.bss.stack)
        lib_sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    lib_ebss = .;
    elib = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}