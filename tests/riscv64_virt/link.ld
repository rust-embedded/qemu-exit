/* SPDX-License-Identifier: MIT OR Apache-2.0
 *
 * Copyright (c) 2020-2022 Esteban Blanc <estblcsk@gmail.com>
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
    .rodata : { *(.rodata*) }

    PROVIDE(_stack = . + 1M);
}
