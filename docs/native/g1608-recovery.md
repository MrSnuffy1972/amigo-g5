# Game 1608 (rainbow islands (1987)(ocean)[cr scx][t +3 scx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01608). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (915616 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 179 of 179 named (0 author symbols), 179 of 179 described (76 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 188 of 188 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 98816 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1858 of 1858 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 34201 |
| code | 5964 |
| data | 684471 |
| padding | 4 |
| unused | 190976 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf:bootblock | 1 | 15 | 1 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load | 1 | 34 | 1 |
| logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load | 1 | 90 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 377 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1150 | 97 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 40 | 40 |
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf bytes 34816-88576 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf bytes 91648-120832 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf bytes 250880-255488 (code never loaded by a reference run) | 1 | 132 | 1 |
| Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf bytes 256000-267264 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: custom-format (non-DOS) disk, 1359 non-empty blocks: matched against track-loaded segments
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: the boot block reads 22528 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- runtime segment an address: 134 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 42 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: bytes 34816-88576 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: bytes 91648-120832 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: bytes 250880-255488 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: bytes 256000-267264 look like code no run loaded: recovered position-independently (load address unknown)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: 6 logged loads cover 132608 bytes of the image
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 22-29 (bytes 11264-15360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.12)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 32-39 (bytes 16384-20480) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.95)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 65-66 (bytes 33280-34304) outside the filesystem hold data, not code (opcode density 0.002, entropy 1.29)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 242-488 (bytes 123904-250368) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.32)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 523-523 (bytes 267776-268288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 525-525 (bytes 268800-269312) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.48)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 527-551 (bytes 269824-282624) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.26)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 553-727 (bytes 283136-372736) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.76)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 729-823 (bytes 373248-421888) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.17)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 825-925 (bytes 422400-474112) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.53)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 927-1037 (bytes 474624-531456) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.90)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1039-1149 (bytes 531968-588800) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.03)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1151-1273 (bytes 589312-652288) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.38)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1275-1336 (bytes 652800-684544) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.86)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1347-1347 (bytes 689664-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.79)
- rainbow islands (1987)(ocean)[cr scx][t +3 scx]/Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf: blocks 1349-1400 (bytes 690688-717312) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.33)
- 4 never-loaded region(s), 98816 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf:bootblock

- `load_boot_data_then_jump`: Disables all interrupts and DMA channels, issues two exec DoIO requests, clears a 16-long global area, traps once, copies 256 longwords into low memory and calls a routine through a pointer.

### logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load

- `scale_palette_per_frame`: Waits for and acknowledges the vertical blank request bit, then scales a table of 16 color values by a level that rises each frame and writes them into the palette buffer.

### logged disk load from Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a decrementing 16-bit value into a global cell in a loop that ends when the counter wraps past zero, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_movem_rts`: Short code fragment that copies 16574 to a fixed memory address, then restores saved registers from the stack, then returns.
- `frag_move_rts`: Short code fragment that copies 32768 to a fixed memory address, then copies 9 to a fixed memory address, then returns.
- `update_game_state`: Updates game state variables including counters and flags, performs conditional logic based on bit tests, and handles data movement between memory locations.
- `frag_move_moveq_bra`: Short code fragment that copies d0 to the stack, then loads 3 into d0, then jumps to another point in the code.
- `save_processor_context`: Saves the processor context including registers and stack pointers to a structure, then performs a series of checks and updates on a data pointer before returning.
- `frag_move_moveq_bra_2`: Short code fragment that copies d0 to the stack, then loads 4 into d0, then jumps to another point in the code.
- `frag_move_moveq_bra_3`: Short code fragment that copies d0 to the stack, then loads 7 into d0, then jumps to another point in the code.
- `frag_move_moveq_bra_4`: Short code fragment that copies d0 to the stack, then loads 5 into d0, then jumps to another point in the code.
- `frag_move_moveq_bra_5`: Short code fragment that copies d0 to the stack, then loads 2 into d0, then jumps to another point in the code.
- `save_context_and_call`: Saves processor context to a memory structure, calls another routine through a function pointer, then restores the context and returns via rte.
- `frag_move_lea_moveq_dbra_bra`: Short code fragment that copies 32767 to a fixed memory address (2 times in a row), then loads the address of a fixed memory address into a0, then loads 15 into d0, then copies 1792 to the memory at a0 (advancing a0), then loops back while counting down d0, then jumps to another point in the code.
- `call_vtable_method_2`: Reads a method offset from a structure and calls the corresponding function through a virtual table pointer, then returns.
- `call_table_entry`: Reads a word value from a table entry at offset 64 of a structure, uses it as an index to call a routine from another table, then returns.
- `frag_addq_rte`: Short code fragment that adds 1 to a fixed memory address, then returns from the exception.
- `frag_subq_bne_tst_beq_move_rte_bra`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then tests a fixed memory address, then branches if the last result was zero, then copies 9 to a fixed memory address, then returns from the exception, then jumps to another point in the code.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_display_state`: Calls a routine through a pointer table, stores a pointer in a global variable, sets bit 5 of a word at offset 322, and writes a specific value to a hardware register.
- `increment_counter_and_call`: Increments a global counter and calls a routine whose address is read from a pointer plus a 16-bit offset stored 38 bytes into the structure it points to.
- `call_offset_eight`: Loads a pointer from a table at offset 2338 of a structure in a5, then calls a routine at an offset of 8 bytes from the second pointer in that table.
- `call_offset_hundred`: Loads a pointer from a table at offset 2338 of a structure in a5, then calls a routine at an offset of 100 bytes from the second pointer in that table.
- `compare_and_trap`: Compares a 16-bit value at offset 2344 of a structure in a5 with itself, and if they are not equal, executes a trap and returns.
- `frag_move_bsr_rts_3`: Short code fragment that copies the memory at an offset from a5 to d0, then calls another routine, then returns.
- `fill_table_with_pseudo_random`: Fills a table starting at offset 2352 of a structure in a5 with 128 16-bit values calculated using a multiplication and division algorithm, then stores the last value at offset 2608 of the structure in a5.
- `update_structure_with_math`: Increments three bytes at specific offsets in a structure in a5, calculates a new value using a multiplication and division algorithm based on a value at offset 2608, and adds this to two values retrieved from a table to store the result in another table location.
- `increment_and_return_word`: Increments a byte at offset 2613 of a structure in a5 and returns a 16-bit word from the address stored at offset 2610 of the same structure.
- `process_interrupts_and_call`: Reads interrupt enable and request registers, masks the interrupt signals, and conditionally calls routines from a table based on the masked interrupt bits.
- `handle_interrupt_mask`: Reads interrupt status from a hardware register, updates global interrupt masks, and conditionally calls up to five different routines based on bit shifts of a computed value.
- `handle_low_priority_interrupts`: Handles low priority interrupts by reading interrupt status, masking specific bits, writing result back to interrupt request register, and conditionally calling up to three different routines based on shifted bit values.
- `frag_movea_jsr_bra`: Short code fragment that copies the memory at an offset from a5 to a0, then calls the routine whose address is in a0, then jumps to another point in the code.
- `frag_movea_jsr_bra_2`: Short code fragment that copies the memory at an offset from a5 to a0, then calls the routine whose address is in a0, then jumps to another point in the code.
- `frag_movea_jsr_bra_3`: Short code fragment that copies the memory at an offset from a5 to a0, then calls the routine whose address is in a0, then jumps to another point in the code.
- `frag_move_addq_rte`: Short code fragment that copies a fixed memory address to a fixed memory address, then adds 1 to a fixed memory address, then returns from the exception.
- `decrement_global_counter_2`: Decrements a global counter by one and, when the result is zero, sets the top bit of that global.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `manage_game_timer`: Decrements a timer value and executes a function when the timer reaches zero, while also clearing and setting specific bits in a global flag variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movea_jsr_rts_2`: Short code fragment that copies a6 to a0, then calls another routine, then returns.
- `initialize_and_update_display_list`: Initializes display list pointers and updates hardware registers with calculated values from a structure, then stores the result in a global variable.
- `frag_move_movea_jsr_rts_2`: Short code fragment that copies a6 to the stack, then copies the memory at an offset from a6 to a6, then copies the memory at a6 to a4, then copies the memory at an offset from a4 to d7, then calls another routine, then copies the top of the stack to a6, then copies the memory at a6 to a4, then returns.
- `decrement_and_validate_counter`: Decrements a 16-bit counter value at a specific memory location and performs signed overflow validation using trap instructions.
- `initialize_structure_2_2`: Writes a series of predefined 16-bit values to a memory region starting at address in a0, then performs a loop that repeats based on a count.
- `decrement_counter`: Decrements a signed 16-bit value located at offset 4 of a structure pointed to by a global variable, then checks for signed overflow and triggers an exception if detected.
- `initialize_structure_2`: Writes a series of predefined 16-bit values to a memory region starting at address in a0, then performs a loop that repeats based on a count.
- `fill_memory_block`: Fills a memory block with predefined 16-bit values and then calls another routine through a function pointer retrieved from a table.
- `conditional_execute`: Compares a 16-bit value with another value from a global structure, and if they differ, executes a function call through a virtual table; then executes another function call regardless of the comparison result.
- `process_object_data_2`: Reads object data from a table, updates object properties such as position and status flags, and processes the data through multiple iterations until a condition is met.
- `frag_move_add_movea_lea_bra`: Short code fragment that copies d6 to the stack, then adds d0 to the memory at an offset from a6 (3 times in a row), then copies the memory at an offset from a6 to d1, then copies the memory at an offset from a6 to a0, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `add_and_advance`: Adds a byte value to memory location pointed by a0, then advances a0 by 24 bytes, repeating this process for a specified number of iterations controlled by d1.
- `store_coordinates`: Stores the value of a0 register into a memory location at offset 34 of a structure pointed by a6, then stores the value of d0 register into a memory location at offset 38 of that structure.
- `frag_move_movea_jsr_rts_5`: Short code fragment that copies a6 to the stack, then copies a1 to a6, then copies the memory at a6 to a4, then copies the memory at an offset from a4 to d7, then calls another routine, then copies the top of the stack to a6, then copies the memory at a6 to a4, then returns.
- `frag_addq_jsr_movea_move_rts`: Short code fragment that adds 1 to d0, then calls another routine, then copies the memory at an offset from a6 to a0, then copies an address to the memory at a0, then returns.
- `read_offset_from_pointer`: Reads a 32-bit value from ten bytes after the address held in an uninitialized pointer register and returns that value minus one.
- `frag_move_movea_jsr_rts_6`: Short code fragment that copies a6 to the stack, then copies the memory at an offset from a6 to a6, then copies the memory at a6 to a4, then copies the memory at an offset from a4 to d7, then calls another routine, then copies the top of the stack to a6, then copies the memory at a6 to a4, then copies the memory at an offset from a4 to d7, then calls another routine, then returns.
- `decrement_and_call`: Decrements a word value at offset 4 of a global variable, checks for overflow and triggers an exception if needed, then conditionally calls another routine based on the value in a global variable.
- `update_position_and_call_2`: Copies data from a buffer to structure fields, calls a function through a table lookup using a register value, and stores the result into multiple memory locations of a structure pointed by a.
- `process_data_and_call`: Calls a method on a structure pointed to by a1, stores the result in a structure, clears a memory location, then calls another routine through a function pointer retrieved from a table.
- `store_word_triple`: Copies three consecutive 16-bit values from a source pointer into three consecutive 16-bit fields of a record at offset 30.
- `reset_three_word_fields`: Zeroes three consecutive 16-bit fields of a record at offset 30 of a6 and then calls another routine.
- `frag_move_swap_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then swaps the halves of d0, then copies the memory at an offset from a6 to d0, then copies the memory at an offset from a6 to d1, then returns.
- `update_tile_data`: Updates tile data by checking a bit in memory, calculating new values, and storing results to memory locations when certain conditions are met.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `frag_move_rts_2`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a6, then returns.
- `frag_clr_jsr_rts`: Short code fragment that clears the memory at an offset from a6 (2 times in a row), then calls another routine, then returns.
- `compare_counters`: Compares two word values located at offsets 48 and 50 of a structure pointed to by a6, and sets the return value to -1 if they are equal, otherwise sets it to 0.
- `frag_moveq_move_bmi_and_lsr_btst_beq_bs`: Short code fragment that loads 127 into d1, then copies the memory at a2 to d0, then branches if the last result was negative, then masks d1 with d0, then shifts d1 right by 3, then tests bit d0 of an indexed entry from a6, then branches if the last result was zero, then sets bit d0 of an indexed entry from a6, then tests the memory at an offset from a6, then returns.
- `frag_and_lsr_bclr_moveq_rts`: Short code fragment that masks d1 with d0, then shifts d1 right by 3, then clears bit d0 of an indexed entry from a6, then loads 0 into d0, then returns.
- `clear_four_long_fields`: Zeroes four consecutive 32-bit fields at offset 48 of the record in a6 and then calls another routine.
- `test_bit_from_scaled_index`: Shifts a 16-bit value in a register right by three bits to calculate an index, tests a bit at that index within a table located at an offset of 48 from a pointer in a6, and returns.
- `frag_movem_moveq_clr_bra`: Short code fragment that saves registers on the stack, then loads 0 into d0, then clears d3, then jumps to another point in the code.
- `frag_move_lsr_btst_beq_bset_addq_bpl_mo`: Short code fragment that copies d2 to d1, then shifts d1 right by 3, then tests bit d2 of an indexed entry from a6, then branches if the last result was zero, then sets bit d3 of d0, then adds 1 to d3, then copies the memory at a0 (advancing a0) to d2, then branches if the last result was not negative, then restores saved registers from the stack, then returns.
- `frag_move_bmi_tst_beq_moveq_lsr_btst_rt`: Short code fragment that copies the memory at a2 to d0, then branches if the last result was negative, then tests the memory at an offset from a6, then branches if the last result was zero, then loads 0 into d1, then copies d0 to d1, then shifts d1 right by 3, then tests bit d0 of an indexed entry from a6, then returns.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `store_and_branch`: Stores a byte from d1 to an offset 30 of the structure in a6, then stores a pointer from a0 to d1, and if d1 is not zero, branches to another location.
- `frag_move_lsr_bchg_bpl_jsr_rts`: Short code fragment that copies d0 to d1, then shifts d1 right by 3, then flips bit d0 of an indexed entry from a6, then copies the memory at a0 (advancing a0) to d0, then branches if the last result was not negative, then copies the top of the stack to d0, then calls another routine, then returns.
- `frag_tst_bne_move_bra`: Short code fragment that tests d1, then branches if the last result was not zero, then copies 16104 to the memory at an offset from a6, then copies 16152 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_jsr_rts`: Short code fragment that copies 16116 to the memory at an offset from a6, then copies 16158 to the memory at an offset from a6, then calls another routine, then returns.
- `decrement_and_call_2_2`: Decrements a 16-bit value at offset 4 of a structure pointed to by a0, checks for overflow, calls another routine, then decrements a 16-bit value at offset 30 of a structure.
- `decrement_and_call_2`: Decrements a 16-bit value at offset 4 of a structure pointed to by a0, checks for overflow, calls another routine, then decrements a 16-bit value at offset 30 of a structure in a6.
- `calculate_division_result`: Performs division calculations using multiplication and subtraction operations, then processes results through a loop that loads values from a table and tests counter conditions.
- `decrement_and_branch`: Decrements a counter and branches back to itself while loading values from a table, then tests the counter and branches based on its value, ultimately calling another routine and updating memory locations.
- `frag_move_bra`: Short code fragment that copies d4 to an indexed entry from a0, then jumps to another point in the code.
- `frag_tst_bne_moveq_bra`: Short code fragment that tests d2, then branches if the last result was not zero, then loads 0 into d5, then jumps to another point in the code.
- `frag_move_bsr_movea_bra`: Short code fragment that copies 65535 to an indexed entry from a0, then calls another routine, then copies the memory at an offset from a6 to a0, then copies the memory at an offset from a2 to d2, then copies the memory at a2 to d3, then copies the memory at an offset from a2 to d4, then jumps to another point in the code.
- `iterate_and_call`: Iterates a specified number of times, calling a routine through a table lookup with parameters from the stack, and returns early if the routine returns a negative value.
- `frag_move_subq_lea_moveq_bra`: Short code fragment that copies the memory at an offset from a2 to d3, then subtracts 2 from d3, then loads the address of the memory at an offset from a0 into a1, then loads 255 into d4, then jumps to another point in the code.
- `conditional_copy_loop`: Loops while a 16-bit counter is non-zero, conditionally copying data based on a comparison result, and updates pointers during each iteration.
- `call_routine_if_nonnegative`: Calls another routine only when a 16-bit value passed in a data register is nonnegative, and otherwise returns without doing anything.
- `clear_linked_cell_chain`: Follows a chain of 16-bit links starting from a table index, zeroing each visited cell, and stops when it reaches a negative link.
- `frag_movea_move_jsr_rts_2`: Short code fragment that copies the memory at an offset from a6 to a0, then copies a6 to the stack, then copies the memory at an offset from a6 to a6, then copies the memory at a6 to a4, then copies the memory at an offset from a4 to d7, then calls another routine, then copies the top of the stack to a6, then copies the memory at a6 to a4, then returns.
- `decrement_two_counters`: Decrements two 16-bit values located at offsets 4 and 26 of structures pointed to by offsets 14 and 26 of a structure in a register, calling a trap if either subtraction causes a borrow.
- `update_and_call_pointers`: Stores a byte from d0 into a structure at offset 36, stores a pointer from a0 into offset 14, increments a 16-bit value at offset 4 of the pointer in a0, updates a pointer in a6 using a value from a5, increments a 16-bit value at offset 4 of that new pointer, clears a 16-bit value at offset 30, and calls a routine at the address in a0.
- `compare_byte_to_stored_flag`: Compares the byte in a data register with the flag byte stored at offset 36 in the record.
- `update_and_check_2`: Updates two 16-bit values in a structure and checks a condition, decrementing a counter until it reaches zero.
- `process_object_operations`: Processes object operations by calling multiple routines through function pointers, updating structure values, handling conditional branches, and managing counters for object state transitions.
- `increment_and_call`: Calls a routine through a table lookup, increments two 16-bit values in a structure, and returns zero.
- `frag_move_bsr_rts`: Short code fragment that copies d6 to the stack, then copies 88 to d6, then calls another routine, then copies the top of the stack to d6, then returns.
- `call_with_table_lookup`: Performs a table lookup using a value from a structure to call a routine, then returns without modifying any data.
- `frag_move_bsr_rts_2`: Short code fragment that copies d6 to the stack, then copies 138 to d6, then calls another routine, then copies the top of the stack to d6, then returns.
- `calculate_and_iterate_2`: Calculates values based on data in tables and structures, performs iterative operations with conditional exits, and calls routines through tables with parameters from the stack.
- `calculate_and_iterate`: Calculates values based on data in tables and structures, performs iterative operations with conditional exits, and calls routines through table lookups.
- `execute_iterative_callback_sequence`: Iterates through data, calculates offsets based on values in a structure, calls functions through table lookups, and continues looping until conditions are met or lists are exhausted.
- `decrement_and_call_if_one`: Checks if a 16-bit value at offset 44 of a structure in a6 is equal to 1.
- `execute_multi_calls`: Executes multiple routine calls through table lookups with different offsets, updates values in structures, handles conditional branches based on return values, and manages object state transitions.
- `multi_call_sequence`: Performs multiple routine calls through table lookups with different offsets, updates values in structures, and handles conditional branches based on return values from these calls.
- `frag_clr_jsr_rts_2`: Short code fragment that clears the memory at an offset from a6, then calls another routine, then returns.
- `release_secondary_object_then_call`: Decrements the 16-bit counter of the object referenced at offset 50, trapping on signed overflow, and then calls another routine.
- `decrement_held_object_then_call`: Decrements the 16-bit counter of the object referenced at offset 46, trapping on signed overflow, and then calls another routine.
- `decrement_structure_field`: Decrements a short integer at an offset within a structure and triggers a trap if an arithmetic overflow occurs.
- `frag_jsr_clr_rts`: Short code fragment that calls another routine, then clears the memory at an offset from a6, then returns.
- `frag_tst_bne_move_jsr_addq_rts_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not zero, then copies the memory at an offset from a4 to d7, then calls another routine, then adds 1 to the memory at an offset from a6, then returns.
- `execute_callback_chain`: Walks a singly linked list starting from a pointer held in a structure, calling the routine at offset 16 in each node until a node's next pointer is zero.
- `frag_movea_jsr_rts`: Short code fragment that copies a6 to a0, then copies the memory at an offset from a6 to a1, then calls the routine whose address is in a1, then returns.
- `frag_move_jsr_beq_rts`: Short code fragment that copies d0 to the memory at an offset from a6, then copies a0 to the memory at an offset from a6, then copies 38 to d0, then copies the memory at an offset from a4 to d7, then calls another routine, then copies a0 to d0, then branches if the last result was zero, then copies a0 to the memory at an offset from a6, then returns.
- `frag_movea_jsr_rts_3`: Short code fragment that copies a6 to a0, then copies the memory at an offset from a6 to a1, then calls the routine whose address is in a1, then returns.
- `frag_addq_move_movea_bra`: Short code fragment that adds 1 to the memory at an offset from a0, then copies the memory at an offset from a0 to d1, then copies the memory at an offset from a6 to a1, then jumps to another point in the code.
- `update_links`: Traverses a linked list structure by following pointers and updates links between nodes.
- `insert_node_at_front`: Increments the counter of the node in an address register and inserts it at the front of the list headed in this record, rewriting the neighbour links.
- `append_node_at_stored_tail`: Increments the counter of the node in an address register, links it after the tail node stored at offset 14 in this record, and makes it the new tail.
- `swap_linked_list_nodes`: The routine swaps the next and previous pointers of two linked list nodes referenced through a pointer in A0 and a global variable.
- `frag_movem_movea_subq_bra`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a6 to a6, then subtracts 4 from a7, then jumps to another point in the code.
- `execute_function_table_entry`: Loads values from the stack, uses them to index into a function table, executes the function at that index, and continues looping until a null pointer is encountered.
- `frag_movem_movea_subq_moveq_bra`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a6 to a6, then subtracts 4 from a7, then loads 0 into d0, then jumps to another point in the code.
- `execute_and_check_result`: Loads values from the stack, uses them to index into a function table, executes the function at that index, checks if the result is non-zero, and returns immediately if it is.
- `frag_movem_movea_subq_moveq_bra_2`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a6 to a6, then subtracts 4 from a7, then loads 0 into d0, then jumps to another point in the code.
- `execute_with_loop_check`: Loads values from the stack, uses them to index into a function table, executes the function at that index, checks if the result is non-zero, and continues looping until a null pointer is encountered.
- `frag_movea_addq_lea_movem_rts`: Short code fragment that copies a6 to a0, then adds 1 to the memory at an offset from a0, then loads the address of the memory at an offset from a7 into a7, then restores saved registers from the stack, then returns.
- `frag_movea_bra`: Short code fragment that copies the memory at an offset from a6 to a0, then jumps to another point in the code.
- `release_chain_and_reset_list`: Walks a chain of nodes starting at the node in a data register, decrementing the counter of each visited node.
- `frag_movea_bra_2`: Short code fragment that copies the memory at an offset from a6 to a0, then jumps to another point in the code.
- `decrement_linked_counter_chain`: The routine walks a singly linked list via a pointer in D0 and a node pointer in A0.
- `increment_counter_if_nonzero`: The routine reads an integer from a global variable via a pointer stored in a register.
- `initialize_list_head`: Points the list head link and the tail link of the record at the record's own fields and clears the 32-bit field at offset 10.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_11`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_12`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_13`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_14`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_15`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_16`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_17`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_18`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_19`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_20`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_21`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_22`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_23`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_24`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_25`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_26`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_27`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_28`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_29`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_30`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_31`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_32`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_33`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_34`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_35`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_36`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_37`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_38`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_39`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_40`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Rainbow Islands (1987)(Ocean)[cr SCX][t +3 SCX].adf bytes 250880-255488 (code never loaded by a reference run)

- `clear_object_fields`: Clears three 16-bit fields at offsets 30, 32 and 34 of a structure pointed to by a6, then returns.
