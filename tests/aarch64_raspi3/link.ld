/* SPDX-License-Identifier: MIT OR Apache-2.0
 *
 * Copyright (c) 2020-2022 Andre Richter <andre.o.richter@gmail.com>
 */

SECTIONS
{
    /* Set current address to the value from which the RPi starts execution */
    . = 0x80000;

    .text :
    {
        *(.text._start)
        *(.text)
    }
}
