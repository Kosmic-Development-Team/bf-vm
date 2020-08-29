# BrainFuck Virtual Machine

This is a virtual machine for an augmented version of BrainFuck.

## Specifications

16 bit addressing for memory (tapes) and pages (of takes).
Tapes for peripheral interface.
Register.

## Language

`>` Moves data pointer to the right.
`<` Moves data pointer to the left.
`+` Increment at data pointer.
`-` Decrement at data pointer.
`[` Jump to matching `]` if value at pointer is `0`.
`]` Jump to matching `[` if value at pointer is not `0`.
`.` Write from data pointer to WO peripheral tape at address in register.
`,` Read from the RO peripheral tape at address in register to data pointer.

`@` Move to address on current page from data pointer.
`^` Copies from data pointer to register.
`*` Copies from register to data pointer.
`~` Rotate right shift.
`&` Bitwise NAND between data pointer and register onto the data pointer.
`#` Jump to specified page from data pointer.
`}` Go to next memory page.
`{` Previous memory page.
