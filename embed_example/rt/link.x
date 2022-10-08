/* Memory layout of the LM3S6965 uC */
/* 1K = 1KiBi = 1024 bytes */
MEMORY
{
    FLASH : ORIGIN = 0x00000000, LENGTH = 256K
    RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

/* Entry point is the reset handler */
ENTRY(Reset);
/* Reset vector */
EXTERN(RESET_VECTOR);
/* Exceptions vector */
EXTERN(EXCEPTIONS);

SECTIONS
{
    .vector_table ORIGIN(FLASH) :
    {
        /* First entry: initial stack pointer value */
        LONG(ORIGIN(RAM) + LENGTH(RAM));

        /* Second entry: reset vector */
        KEEP(*(.vector_table.reset_vector));
    } > FLASH

    /* code section */
    .text :
    {
        *(.text .text.*);
    } > FLASH

    /* const */
    .rodata :
    {
        *(.rodata .rodata.*);
    } > FLASH

    /* var */
    .bss :
    {
        /* start address of bss */
        _sbss = .;
        *(.bss .bss.*);
        /* end address of bss */
        _ebss = .;
    } > RAM

    /* Set LMA of .data section at the end of .rodata section */
    .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
    {
        /* start address of data */
        _sdata = .;
        *(.data .data.*)v
        /* end address of data */
        _edata = .;
    } > RAM

    _sidata = LOADADDR(.data);

    /DISCARD/ :
    {
        *(.ARM.exidx .ARM.exidx.*);
    }
}

/* Default exeption handler */
PROVIDE(NMI = DefaultExceptionHandler);
PROVIDE(HardFault = DefaultExceptionHandler);
PROVIDE(MemManage = DefaultExceptionHandler);
PROVIDE(BusFault = DefaultExceptionHandler);
PROVIDE(UsageFault = DefaultExceptionHandler);
PROVIDE(SVCall = DefaultExceptionHandler);
PROVIDE(PendSV = DefaultExceptionHandler);
PROVIDE(SysTick = DefaultExceptionHandler);
