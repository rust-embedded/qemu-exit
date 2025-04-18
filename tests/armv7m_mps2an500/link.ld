/* SPDX-License-Identifier: MIT OR Apache-2.0
 *
 * Copyright (c) 2024 Philipp Schulz <schulz.phil@gmx.de>
 */
ENTRY(_vectors)

SECTIONS
{
    . = 0x0;
    .boot :
    {
        KEEP(*(.text.boot))
        KEEP(*(.data.boot))
    }

    .text :
    {
        *(.vectors)
        *(.text._start)
        *(.text)
    }
}
