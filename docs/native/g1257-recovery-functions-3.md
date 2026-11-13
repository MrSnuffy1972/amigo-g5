# Game 1257 (quest for glory 2 - trial by fire (1992)(sierra)(disk 1 of 8)): code recovery summary

## Functions (part 3 of 3)

### Quest for Glory 2 - Trial by Fire (1992)(Sierra)(Disk 1 of 8).adf:amigasnd.drv
(continued)

- `check_index_return`: Swaps d2 and compares it with 192; when they differ it restores d2 and jumps to the return routine, otherwise it indexes a table using d2 and jumps to the code that returns 65535.
- `index_byte_from_table_d0_2`: Compares the value in d2 with 192 and returns immediately when they differ, otherwise swaps d2, clears d0, reads a byte from a table indexed by d2 into d0, and jumps to another routine.
- `index_byte_from_table_d0`: Compares the value in d2 with 176 and returns immediately when they differ.
- `load_byte_indexed_d0`: Branches away when d2 is not 64, otherwise loads a pointer from offset 536 of the structure in a4, masks d0 with 255, reads a byte from that table indexed by d0, and branches to a routine that stores the result.
- `compare_index_load_byte`: Compares register d2 against the constant 1 and branches away when they differ.
- `compare_index_read_byte`: Compares d2 against the constant 75, then, when they are equal, masks d0 to its low byte and reads a byte from a table at offset 548 of the structure in a4 indexed by d0.
- `compare_and_advance_index`: Compares the byte in d0 against entries in a table at offset 1416 of the structure in a4, incrementing an index while they match until reaching four, then restores d3 from the stack and returns.
- `return_65535_value`: Pops two long values off the stack, discards them, and returns the 16-bit value 65535.
- `allocate_and_store_memory`: Allocates 512 bytes of memory and stores the returned pointer at offset 1204 of the structure in a4, then writes that pointer and four incremented copies into the long fields at offsets 1540 through 1568.
- `initialize_audio_channels`: Stores a passed pointer at offset 1200 of a4 and increments that field by four, sets a byte at offset 9 of a structure to 127, and fills a second structure with a pointer and the value one.
- `initialize_audio_interrupts`: Initializes four audio channel structures with common fields and handler pointers, reads a pointer from a global variable to fill each slot, writes a value to the interrupt enable register, and calls another routine before returning.
- `return_constant_64bit`: Loads four 16-bit constants (65, 133, 6, 4) into the data registers d0 and d1, swapping the upper and lower halves of each, then returns.

### Quest for Glory 2 - Trial by Fire (1992)(Sierra)(Disk 1 of 8).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_a0_to_d0`: Copies the value in a0 into d0 and applies a bitwise OR with zero, repeating this operation for the entire body of the routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `return_constant_8`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `find_table_entry`: Loads a table pointer, calls a helper routine, then walks a sorted table of 32-bit entries looking for the largest value not greater than 149 and stores that value into a global variable.
