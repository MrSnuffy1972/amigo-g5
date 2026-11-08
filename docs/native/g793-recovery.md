# Game 793 (settlers, the (1993)(blue byte)(disk 1 of 3)(intro)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00793). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910432 top-level bytes in 39 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 57 of 57 named (0 author symbols), 57 of 57 described (5 by rule, 49 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 553 of 553 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 69041 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2117 of 2136 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 732068 |
| code | 6558 |
| data | 161585 |
| padding | 9709 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:Install | 8 | 45 | 2 |
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:mcp | 5 | 253 | 7 |
| run | 2 | 117 | 2 |
| endcli | 2 | 117 | 2 |
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:Introecho | 1 | 45 | 2 |
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:Loader | 1 | 43 | 3 |
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1236 | 34 |
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:PIB1 (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:DISP (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:SOUN (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:USER (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:EFEC (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:SD00 (raw code file, load address unknown) | 1 | 0 | 0 |
| Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:FUNC (raw code file, load address unknown) | 1 | 270 | 4 |

## Cases handled

- runtime segment an address: 318 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- settlers, the (1993)(blue byte)(disk 1 of 3)(intro)/Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:PIB1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)(disk 1 of 3)(intro)/Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:DISP: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)(disk 1 of 3)(intro)/Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:SOUN: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)(disk 1 of 3)(intro)/Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:USER: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)(disk 1 of 3)(intro)/Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:EFEC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)(disk 1 of 3)(intro)/Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:SD00: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)(disk 1 of 3)(intro)/Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:FUNC: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- settlers, the (1993)(blue byte)(disk 1 of 3)(intro)/Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:Install: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- settlers, the (1993)(blue byte)(disk 1 of 3)(intro)/Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:mcp: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 6 never-loaded region(s), 68291 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:Install

- `initialize_system_memory`: Performs a series of function calls, decrements a counter in a loop, and checks a specific memory pattern to update a global variable.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:mcp

- `init_and_check_pattern`: Invokes several routines and loops three times, then clears a memory location and, when a value matches a specific pattern, stores it and zeroes a global variable.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `expand_and_copy_data_2`: Calls a routine via a pointer in a4 multiple times, performs arithmetic on d0, and uses the result to add values to a destination buffer at an offset determined by d2.
- `load_address_and_set_d0`: Loads a constant into a1 using the program counter and sets d0 to two.
- `expand_and_copy_data`: Expands a byte stream using a variable bit-shift multiplier and copies the resulting bytes into a destination buffer.
- `push_registers_to_stack`: Performs bitwise operations on d0, stores d0, d1, and d2 onto the stack, and branches to another routine.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves the contents of d1 into d0 and branches to another routine.

### endcli

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, then writes a header and a sequence of words into a buffer.
- `transfer_register_and_branch`: Moves the contents of data register 1 into data register 0 and then performs a short branch to another routine.

### Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:Introecho

- `process_system_messages`: Interacts with system structures through multiple function pointers to manage task-related data and nested callbacks.
- `perform_logical_or`: Performs a logical OR operation on d0 with zero.

### Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:Loader

- `load_segment_data`: Retrieves a pointer from a global variable and executes functions through offsets to process data segments.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_relocate_amiga_hunks`: Parses a linked list of AmigaDOS hunk structures to allocate memory and perform relocations.
- `parse_amiga_dos_hunks`: Iterates through a linked list of data structures to identify and process AmigaDOS hunk types.
- `traverse_linked_list`: Iterates through a linked list of structures and executes two function pointers stored within each element.
- `process_hunk_data`: Checks for specific values in a data structure and executes function pointers if certain conditions are met, returning zero or a specific error code.
- `tiny_move`: Small routine of 1 instruction that copies a3 to d7.
- `search_and_jump_table`: Loops through a list of values, compares a byte to a value in a structure, and performs an indirect jump using a calculated offset if they match.
- `branch_to_offset_2`: Performs a short relative branch to a location within the code segment.
- `branch_to_offset_routine`: Performs a short branch to a location within another routine.
- `branch_to_offset_3`: Performs a short branch to a specific location within another routine.
- `decrement_and_check_structure_counter`: Traverses a linked list of structures to find an entry where a value at offset 14 matches a register, then decrements a 16-bit counter at offset 18 of that structure.
- `branch_to_offset`: Performs a short branch to an offset within another routine.
- `reset_two_state_words`: Sets one state word to zero and the word beside it to three, then returns zero.
- `check_structure_values`: Saves registers to the stack, loads a pointer from offset 26 of a6 into a5, and if the longword at that pointer is not zero, compares d0 with a longword at offset 14 of the structure pointed to by a5, or loads a longword from offset 4 of that structure into d1 and branches.
- `restore_registers_and_return`: Sets register d1 to negative one and restores d1 and registers a0 through a5 from the stack before returning.
- `decrement_countdown_and_release`: Decrements a 16-bit countdown word in a structure at offset 18 of a4, and if the value reaches zero, calls a library function through a pointer in a6 and calls two other routines.
- `branch_on_pointer_match`: Compares a pointer in a0 with a value from a global variable, and if they match, calls another routine and moves d0 and d1 into d2 and d3 before branching.
- `clear_registers_and_branch`: Clears registers d2 and d3, loads a pointer from the stack into a0, calls another routine, and then performs a short branch.
- `traverse_linked_list_3`: Loads a longword from an offset of 12 in a0 into d0, sets a1 to a pointer from a0, compares a longword at offset 4 of a1 with zero, and if the result is negative, loads that same longword into d1; otherwise, it loads a longword from a1 into d5, and if d5 is not zero, sets a1 to d5 and branches back to an earlier instruction in this routine.
- `restore_registers_and_return_2`: Restores d5 and registers a0 and a1 from the stack and then returns.
- `remove_elements_from_linked_list`: Removes a specified number of elements from a linked list by traversing the nodes to find the appropriate split point, updating the list pointers and counters, and optionally calls another routine or adjusts node sizes if specific bits in a control register are set.
- `update_color_and_raster`: Checks a value at a specific hardware register and, depending on the result, enters a loop to write a value to the color register and the vertical position register a specific number of times.
- `adjust_linked_list_offsets`: Calculates and updates offsets and counters within a linked list structure based on comparison results.
- `insert_into_linked_list`: Inserts a new node into a memory-aligned linked list structure by traversing the list and updating pointers.
- `shrink_linked_list_block`: Iterates through a linked list of memory blocks to find a block containing a specific address range and reduces its size or splits it to accommodate a removal.
- `traverse_linked_list_2`: Iterates through a linked list until it reaches a null pointer or a specific value.
- `fill_memory_with_blt`: Sets up the Amiga Blitter hardware registers to perform a block fill operation and then manually fills the remaining memory range using unrolled 68000 move instructions.
- `copy_variable_length_block`: Copies a block of memory from a source at a0 to a destination at a1 using a bitmask in d0 to determine the size of each incremental transfer.
- `manage_linked_list`: Calls another routine, then checks bit 0 of a byte at offset 9 of a structure in a1.
- `remove_linked_item`: Removes an item from a linked list by updating pointers and decrementing a counter in a structure, or handles list manipulation based on a bit test in a structure at offset 9 of a1.
- `copy_memory_block`: Copies a block of memory from a source to a destination, handling overlapping ranges by copying in reverse order if necessary.
- `compare_pointer_value`: Compares a longword at the address pointed to by a1 with d0, and if the value is not equal to zero, sets a1 to that value and branches.
- `load_registers_from_stack`: Loads the value 255 into d1 and restores d0 and d1 from the stack before returning.
- `branch_on_memory_value`: Loads a 32-bit word from offset 8 of a structure in a1 into d1, then branches to a different location if the value is not zero, otherwise it performs an unconditional branch to another location.
- `compare_pointer_value_and_return`: Loads a longword from offset 12 of a structure in a1 into d1, and if it is not zero, calls compare_pointer_value; otherwise it calls load_registers_from_stack.

### Settlers, The (1993)(Blue Byte)(Disk 1 of 3)(Intro).adf:FUNC (raw code file, load address unknown)

- `initialize_data_buffer`: Stores a byte with a bitwise OR operation and clears a series of word-sized memory locations.
- `populate_data_array`: Checks if a 16-bit value is non-zero and not negative 32768; if so, it stores that value at offsets 0, 2, and 8 of a structure pointed to by a6, and clears offsets 4 and 6, otherwise it stores 1 at offset 0 and returns -1.
- `convert_number_to_string`: Converts a value into a string by performing repeated division and storing the resulting characters into a buffer pointed to by a2.
- `perform_division_math`: Performs division and modulo operations on input values and returns the results concatenated into a single value.
