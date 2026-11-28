# Game 3117 (castle master ii - the crypt (1991)(domark)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03117). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903342 top-level bytes in 17 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 60 of 60 named (0 author symbols), 60 of 60 described (14 by rule, 39 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 343 of 343 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 117388 bytes, 24 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2352 of 2352 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 184168 |
| code | 8478 |
| data | 328049 |
| padding | 2743 |
| unused | 379904 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Text | 2 | 24 | 4 |
| Wait | 2 | 395 | 13 |
| Castle Master II - The Crypt (1991)(Domark)[cr QTX].adf:loader | 1 | 18 | 2 |
| Castle Master II - The Crypt (1991)(Domark)[cr QTX].adf:CM | 1 | 99 | 2 |
| Castle Master II - The Crypt (1991)(Domark)[cr QTX].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 260 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 33 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 81 | 5 |
| Castle Master II - The Crypt (1991)(Domark)[cr QTX].adf:intro (raw code file, load address unknown) | 1 | 1415 | 24 |

## Cases handled

- castle master ii - the crypt (1991)(domark)[cr qtx]/Castle Master II - The Crypt (1991)(Domark)[cr QTX].adf:crypt.com: high-entropy data file (7.95 bits/byte), classified as packed data
- runtime segment an address: 159 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- RAM snapshot an address: 24 executed PCs whose RAM was overwritten before any snapshot match its bytes 525232-525360: recovered as a unit run at an address
- 52 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 96 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- castle master ii - the crypt (1991)(domark)[cr qtx]/Castle Master II - The Crypt (1991)(Domark)[cr QTX].adf:intro: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 117388 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Castle Master II - The Crypt (1991)(Domark)[cr QTX].adf:intro (raw code file, load address unknown)

## Copy protection found

- none found

## Functions


### Text

- `call_library_entry`: Calls one routine and, if the zero flag is clear, calls a second routine and then jumps through a function pointer taken from a global table.
- `frag_movea_lea_jsr_move_tst_rts`: Short code fragment that copies a fixed memory address to a6, then loads the address of a fixed memory address into a1, then calls another routine, then copies d0 to a fixed memory address, then tests d0, then returns.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Wait

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_position`: Initializes position-related data by clearing three longword fields, performing arithmetic on a pointer, and setting a flag based on conditional tests.
- `copy_shifted_longs_to_planes`: Reads four source longwords per step from a table, shifts them left by the low four bits of a position value, and writes each one to a different plane buffer.
- `frag_move_lsl_tst_beq_bra`: Short code fragment that copies the memory at a1 to d1, then shifts d1 left by 2, then tests an indexed entry from a0, then branches if the last result was zero, then copies the memory at an offset from a1 to the memory at an offset from a1, then jumps to another point in the code.
- `calculate_and_store_differences`: Calculates differences between values in a structure and stores results in multiple fields, performing bit shifts and arithmetic operations on data from memory locations.
- `process_character_data`: Reads character data from a buffer using table lookups and arithmetic operations, storing results in structure fields at offsets 224, 228.
- `validate_character_range`: Checks if a character from a buffer falls within specific numeric ranges and updates flags accordingly.
- `update_position_data`: Processes position data from a buffer using table lookups and arithmetic operations, storing results in structure fields at offsets 224, 228.
- `update_position_values`: Processes position data from a buffer using table lookups and arithmetic operations, storing results in structure fields at offsets 224, 228, and 212.
- `transfer_data_blocks`: Transfers data blocks between source and destination buffers using nested loops, copying values from a table to multiple memory locations while incrementing pointers and decrementing counters.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a6, so the target is only known at run time.

### Castle Master II - The Crypt (1991)(Domark)[cr QTX].adf:loader

- `frag_lea_move_dbra_moveq_rts`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then copies 52 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then loads 0 into d0, then returns.
- `setup_video_registers`: Sets up video register values at specific memory offsets relative to a base pointer, then jumps to another routine using an address stored at a fixed offset from the same base pointer.

### Castle Master II - The Crypt (1991)(Domark)[cr QTX].adf:CM

- `initialize_game`: Allocates memory, opens a library, and sets up global variables for game initialization.
- `print_null_terminated_string`: Measures a null-terminated string and passes its length through the library entry at offset -60.

### Castle Master II - The Crypt (1991)(Domark)[cr QTX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sets_global_pointer_returns_minus_one`: Stores a constant into a global variable, removes 4 bytes from the stack, restores registers from memory, and returns the value -1 in d0.
- `frag_bra_move`: Short code fragment that jumps to another point in the code, then copies d0 to a fixed memory address, then copies d1 to a fixed memory address, then jumps to another point in the code.
- `decrement_and_fill_buffer_2`: Decrements a global counter and fills a buffer with values from a register, using loop counters and conditional branches to control the operation, then performs additional memory operations based on the filled buffer.
- `decrement_and_fill_buffer`: Decrements a global counter and fills a buffer with values from a register, using loop counters and conditional branches to control the operation.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `decrement_timer`: Decrements a global counter and performs bit manipulation on a memory location when the counter reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `runtime_segment_data_table`: The sixteen decoded instructions are treated as a data table entered from outside static flow, not as reliable executable code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_coprocessor_data`: Loads data from a global variable into the coprocessor register COP1LCH and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_system`: Initializes several global variables with different offsets from a base address, calls the OpenLibrary function to load a library, and performs arithmetic operations on values loaded from memory locations.
- `update_coprocessor`: Reads a value from a global variable, writes it to the COP1LCH hardware register, and then returns.
- `frag_beq_move_jsr`: Short code fragment that branches if the last result was zero, then copies a fixed memory address to d1, then copies a fixed memory address to d2, then copies 32000 to d3, then calls another routine.
- `frag_bmi_move_jsr_ori`: Short code fragment that branches if the last result was negative, then copies a fixed memory address to d1, then calls another routine, then merges 112 into d0.
- `copy_palette_data`: Copies word values from one memory location to another in a loop that iterates fifteen times, advancing both pointers by two bytes after each copy operation.

### Castle Master II - The Crypt (1991)(Domark)[cr QTX].adf:intro (raw code file, load address unknown)

- `setup_video_buffers`: Initializes video buffer pointers and clears memory regions, then sets up color and coprocessor registers for display rendering.
- `clear_and_fill_buffer`: Clears a global counter and fills a buffer with values from a register, using loop counters and conditional branches to control the operation.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `update_global_counter`: Decrements a global counter and sets a bit in a memory location when the counter reaches zero.
- `clear_planes_data`: Clears memory locations in four planes by writing zero values at specific offsets from a base address, repeating this process for 189 iterations.
- `draw_shifted_planes_four`: Draws bit-shifted data into four destination planes using a mask, with the specific plane update logic determined by a table of four source words.
- `copy_shifted_longs_to_planes_2`: Reads four longwords from a table, shifts them left by bits from a global variable, and writes each shifted value to a different plane buffer.
- `transfer_data_blocks_2`: Copies data from a source table to multiple memory locations through nested loops, incrementing pointers and decrementing counters while transferring values from offsets 0, 40, 80, and 120 of the source to destinations at offsets 8000, 16000, 24000, and 0 of the destination buffer.
- `copy_table_values`: Reads word values from a source table and writes them into multiple destination buffers with specific offsets in a repeating pattern.
- `update_pixel_data`: Processes pixel data through multiple memory locations using bitwise operations on word-sized values, then clears a block of memory with zero values.
- `process_display_row_2`: Processes display row calculations using multiplication, bit shifting, and conditional logic to determine when to clear or update display data based on input parameters.
- `process_display_row`: Calculates row positions and processes display data using multiplication, bit shifting, and conditional logic to determine when to clear or update values.
- `shift_and_draw_planes`: Performs bit-shifted data drawing into four destination planes using a mask, with plane update logic determined by source words.
- `draw_wide_clipped_masked_block`: Draws a bit-shifted, masked four-plane block like the narrower variant, but limits the column count to eight and reads its source planes further apart.
- `process_bit_pattern`: Processes a bit pattern from a source buffer through a series of bitwise operations and writes modified values to memory locations relative to a base pointer, using a global variable to determine the operation type.
- `draw_shifted_row_with_spill`: Processes up to eight source rows from a buffer, writing them into four memory planes with bit shifting controlled by low four bits of an x position.
- `fill_pattern_with_shifted_bits`: Reads pattern data from a fixed memory location and writes it to video memory planes, applying bitwise AND and OR operations with inverted mask values to control pixel rendering across multiple rows and planes.
- `fill_pattern_with_shifted_bits_2`: Reads pattern data from another fixed memory location and writes it to video memory planes, applying bitwise AND and OR operations with inverted mask values to control pixel rendering across multiple rows and planes.
- `loop_pattern_update`: Processes video pattern data by writing to four memory planes using bitwise operations, repeating four times with shifted bit patterns.
- `update_video_planes`: Writes pattern data to four video memory planes using bitwise AND and OR operations with shifted bit patterns, processing multiple words per plane and handling row-based updates.
- `update_graphics_state`: Updates graphics-related variables based on conditional checks and pointer arithmetic, then performs a final state adjustment before returning.
- `process_color_deltas`: Processes color component differences between two sets of RGB values and adjusts them based on a threshold.
- `initialize_render_data`: Initializes rendering data by setting up pointers and writing specific values to memory locations.
- `clear_planes_data_2`: Clears memory locations in four planes by writing zero values at specific offsets from a base address, repeating this process for 199 iterations with a loop counter and conditional branches.
