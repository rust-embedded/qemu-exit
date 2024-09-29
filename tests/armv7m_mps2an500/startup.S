.section ".text.boot","ax"

.thumb
.thumb_func
.globl _vectors
_vectors:
.word 0x2000
.word reset

.thumb_func
reset:
    bl entry
