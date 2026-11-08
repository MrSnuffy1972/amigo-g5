# Game 2306 (strider (1989)(u. s. gold)[cr hz]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02306). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (904896 top-level bytes in 33 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 34 of 34 named (0 author symbols), 34 of 34 described (2 by rule, 30 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1112 of 1112 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 203883 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1431 of 1435 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 479324 |
| code | 4532 |
| data | 286458 |
| padding | 8630 |
| unused | 125952 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Strider (1989)(U. S. Gold)[cr HZ].adf:Horizon | 6 | 233 | 7 |
| Strider (1989)(U. S. Gold)[cr HZ].adf:stboot | 1 | 48 | 2 |
| Strider (1989)(U. S. Gold)[cr HZ].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1127 | 23 |
| Strider (1989)(U. S. Gold)[cr HZ].adf:CHR1.CHR (raw code file, load address unknown) | 1 | 0 | 0 |
| Strider (1989)(U. S. Gold)[cr HZ].adf:LEVEL4.PC1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Strider (1989)(U. S. Gold)[cr HZ].adf:TORGRID1.PC1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Strider (1989)(U. S. Gold)[cr HZ].adf:tor.prg (raw code file, load address unknown) | 1 | 17 | 1 |

## Cases handled

- runtime segment an address: 894 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- strider (1989)(u. s. gold)[cr hz]/Strider (1989)(U. S. Gold)[cr HZ].adf:CHR1.CHR: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- strider (1989)(u. s. gold)[cr hz]/Strider (1989)(U. S. Gold)[cr HZ].adf:LEVEL4.PC1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- strider (1989)(u. s. gold)[cr hz]/Strider (1989)(U. S. Gold)[cr HZ].adf:TORGRID1.PC1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- strider (1989)(u. s. gold)[cr hz]/Strider (1989)(U. S. Gold)[cr HZ].adf:tor.prg: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- strider (1989)(u. s. gold)[cr hz]/Strider (1989)(U. S. Gold)[cr HZ].adf: blocks 1730-1737 (bytes 885760-889856) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.17)
- strider (1989)(u. s. gold)[cr hz]/Strider (1989)(U. S. Gold)[cr HZ].adf:Horizon: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 4 never-loaded region(s), 203883 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Strider (1989)(U. S. Gold)[cr HZ].adf:Horizon

- `execute_init_and_free`: Calls an initial routine and a loop that executes a function pointer from a global variable, then clears a value at an offset from a pointer and returns a concatenated register value.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `expand_data_stream`: Calls a routine three times to load data, then enters a loop that calls another routine to expand a data stream and add values to a destination buffer at an offset determined by the expanded data.
- `load_address_and_constant`: Loads a constant into register d0 and loads an address into register a1.
- `expand_and_copy_data_stream`: Reads a sequence of bytes from a source buffer and expands them into a destination buffer using a variable-length encoding scheme involving bit-shifting and repeated additions.
- `push_registers_and_branch`: Stores the values of d0, d1, and d2 onto the stack, increments a pointer in a6, and branches to a relative offset.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Strider (1989)(U. S. Gold)[cr HZ].adf:stboot

- `load_boot_sequence`: Copies a string from a source to a destination, calls two library functions to allocate memory and obtain a pointer, and then uses a loop to copy a block of data to a fixed address before performing a trap.
- `copy_memory_block`: Copies a block of words from a source pointed to by register d6 to a destination at a fixed address, then jumps to a long address and returns.

### Strider (1989)(U. S. Gold)[cr HZ].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_system_trap`: Pushes the current address onto the stack, moves a value from the stack to an offset of 128, and executes a trap.
- `process_system_state`: Disables interrupts, calls another routine, restores interrupts, checks several bytes in a structure to conditionally call other routines, decrements values in a structure, and potentially calls more routines before returning from interrupt.
- `initialize_structure_fields`: Checks bit 14 of a byte at an offset from a pointer in a6; if not set, it writes several long and word values to specific offsets within that structure and returns.
- `process_input_buffer`: Increments a pointer in a table, checks for specific byte values, and if a match is found, populates a structure in a6 with several values and returns.
- `populate_string_data_table`: Iterates through a list of values, performing bitwise rotations and calculating offsets to store a sequence of derived values into a destination table.
- `decrement_and_update_list`: Decrements a byte in a structure at an offset from a5; if the result is negative, it enters a loop that copies values from a table into a destination list and updates a pointer.
- `increment_table_entries`: Iterates 24 times, incrementing a byte at offset 0, a byte at offset 8, and a byte at offset 16 of a structure pointed to by a modified address.
- `update_multiple_tables`: Checks a byte in a global variable and, if not zero, calls another routine three times with different source and destination pointers and counts to update data tables.
- `limit_values_in_loop`: Iterates through a block of words, subtracting a constant from each and ensuring the result does not fall below the value in register a0.
- `multiply_table_elements`: Performs a loop 259 times, multiplying each word in a table by 40 and storing the result back in the table.
- `format_numeric_string`: Runs a loop 100 times to write formatted characters and a 16-bit value into a buffer, where the 16-bit value is updated using a swap and XOR operation.
- `populate_data_tables`: Fills multiple tables with long and word values using values from other memory locations and handles pointer wrapping within a loop.
- `setup_system_parameters`: Modifies hardware registers, populates a table with ten derived values using a loop, updates two global pointers, writes several bytes to hardware registers, and calls six different routines before updating two values in a structure and clearing a global variable.
- `setup_complex_data_structure`: Performs multiple loops to write swapped word values into various memory locations and populates a large structure with several word and long values before returning.
- `initialize_structure_values`: Sets two 16-bit values at offsets 150 and 154 of a structure in a6 to 32767, updates two global variables using values from a table, writes 1 and 0 to specific hardware register addresses, restores the two 16-bit values from the table, and finally writes a value from a pointer in a5 to offset 128 of a6.
- `copy_table_data`: Performs a nested loop to copy eleven bytes from a source table to a destination structure, where the source offset is determined by multiplying a byte from a table by 11, and repeats this for 40 destination entries and 8 source entries.
- `swap_and_store_values`: Swaps two 32-bit values from a table, stores the results back into that table, calculates a new value by subtracting 40 from one of the original values, and writes the resulting 16-bit components and their swapped high-word counterparts to a different table before writing 32 to a hardware register and returning from an interrupt.
- `handle_interrupt_return`: Saves registers, reads a byte from a hardware register, calls another routine, restores registers, writes a value to a hardware register, and returns from interrupt.
- `branch_to_offset`: Performs a relative branch to a different location.
- `branch_to_locations`: Performs two relative branches to different locations.
- `initialize_audio_and_sprites`: Clears several audio registers and a global variable, sets a bit in a hardware register, and performs multiple loops to copy and manipulate data from a source table into various structures and hardware registers.
- `update_sprite_data_structure`: Updates a data structure by reading bytes from a source table using an index and bit-shifted offset, then writes these values into various offsets of the structure.
- `parse_command_stream`: Iterates through a data stream to interpret command bytes, updating a structure in a0 with pointer offsets, sprite/object data, and hardware register values.

### Strider (1989)(U. S. Gold)[cr HZ].adf:tor.prg (raw code file, load address unknown)

- `initialize_hardware_and_call_subroutines`: Sets the DMA control and interrupt enable registers, then calls three subroutines via long addresses before branching to another location.
