/* SPDX-License-Identifier: MIT OR Apache-2.0
 *
 * Copyright (c) 2020 Esteban <estblcsk@gmail.com>
 */

ENTRY(_start)

SECTIONS
{
    /* Set current address to the address where OpenSBI will jump */
    . = 0x80200000;

    .text : {
        *(.text._start)
        *(.text)
    }
    .data : { *(.data*) }
    .sdata : { *(.sdata*) }
    .rodata : { *(.rodata*) }
    .bss (NOLOAD) : { *(.bss*) }

    PROVIDE(_stack = . + 1M);
}
