ENTRY(start)

SECTIONS {
    . = 1M;

    .boot :
    {
        /* ensures multiboot header is at the start */
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }
}
