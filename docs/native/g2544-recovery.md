# Game 2544 (nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02544). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906480 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 38 of 38 named (0 author symbols), 38 of 38 described (1 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 429 of 429 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 904 of 904 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11647 |
| code | 3056 |
| data | 607617 |
| unused | 284160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf:bootblock | 1 | 63 | 2 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 841 | 36 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf: custom-format (non-DOS) disk, 1203 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf: the boot block reads 10752 bytes from disk offset 716800 to an address (found statically); recovered as a unit
- runtime segment an address: 429 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf: 1 logged loads cover 10752 bytes of the image
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.02)
- 1 packed track region(s), 604672 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_interleaved_data` (runtime segment an address): Modifies a hardware register, enters a loop to copy bytes from a source table to a destination buffer using calculations derived from d0, and restores the hardware register before returning. Kind: interleaved data loader/reconstruction. Pass path: the routine completes the memory copy and returns to the caller to continue execution.

## Functions


### Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a].adf:bootblock

- `setup_display_hardware`: Configures various display registers including bitplane modes, copper control, and video timings, then enables DMA. It performs three library calls and swaps the high and low bytes of each word in a table.
- `initialize_and_swap_data`: Performs three library calls to set up parameters and then loops through a table to swap the high and low bytes of each word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_main_loop`: Enters a loop that repeatedly calls four different routines based on pointers stored in a structure.
- `copy_memory_block`: Copies a large block of data from a global variable to a destination starting at a fixed address.
- `clear_memory_range`: Iterates through a memory range starting at a given pointer and sets each word to zero until a specified limit is reached.
- `process_table_and_increment`: Calculates an offset using a value from a structure in a5, accesses a table at a specific memory location, and calls another routine if a byte from a0 is positive; otherwise, it increments a 16-bit value at offset 34 of the structure in a5 and returns.
- `load_table_values`: Uses an index to fetch a byte from a table and then copies several specific bytes from a memory range into a destination buffer at fixed offsets.
- `count_null_bytes`: Iterates through a string until a null byte is found and then stores a calculated value into a destination offset.
- `update_string_index`: Checks a character value and iterates through a string to find a non-zero byte, then updates a global table with calculated values based on the resulting index.
- `decrement_and_wrap_counter`: Decrements a byte in a structure and, if it reaches zero, performs logic to increment or decrement a counter and update a status byte.
- `process_ascii_data`: Loads three pointers from a structure in a5, writes the values 18 and 2 to offsets 34 and 32 of that structure, and then loops through a data block to subtract 48 from each byte and store the result in a second structure until a zero byte is encountered, at which point it calls another routine and branches to another location.
- `update_counters_and_call_table`: If d0 equals 2, it copies a byte from a0 to a4 and calls another routine; it then updates several byte values at specific offsets within a structure in a5 and calls another routine with a pointer in a0.
- `update_score_display`: Decrements a counter and, upon reaching zero, updates a numeric value in a data structure and converts it into a three-character ASCII string for display.
- `process_data_table`: Loads a byte from a table at offset 43 of the structure in a5, then enters a loop that modifies d3 and d2 based on the byte value, calling another routine if the byte is 1 or 2.
- `update_structure_and_call`: Loads an address into a0 from an offset of 82 in a structure in a5, stores words from d4 and d3 into offsets 32 and 34 of that structure, calls another routine, and then branches to another location.
- `load_address_and_branch`: Loads an address into a0 from an offset of 74 in a structure in a5 and branches to another location.
- `copy_interleaved_data`: Modifies a hardware register, enters a loop to copy bytes from a source table to a destination buffer using calculations derived from d0, and restores the hardware register before returning.
- `update_hardware_and_loop`: Saves registers, performs calculations involving division and subtraction, modifies bits in a5, writes several words to a structure in a4, and enters a loop that calls other routines and waits for a bit to be set in a4.
- `clear_bit_and_branch`: Clears bit 2 of a byte in a structure in a5 and branches to another location.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `process_interleaved_data_table`: Searches a table for a specific marker, then calculates a range based on values extracted from the table and compares it against two input parameters.
- `restore_registers_and_branch`: Restores d2 through d7 and a1 through a3 from the stack, increments the stack pointer by 4, sets bit 1 of a byte in a structure in a6, and branches to another location.
- `compare_masked_values`: Performs bitwise AND operations on two values from a structure in a1 using a mask in d6, executes a loop 255 times to XOR a value from a1 with d3, and compares the result with the previously calculated value.
- `call_toggle_bit_and_branch_wrapper`: Calls the toggle_bit_and_branch routine and then continues execution into the next routine.
- `set_bit_and_test_condition`: Sets bit 1 of a byte at the address in a5 and tests bit 4 of a byte at the address in a6, branching to a previous instruction if the bit is set; otherwise it returns.
- `toggle_bit_and_branch`: Clears bit 0 of a byte in a structure in a5, sets bit 0 of that same byte, and branches to another location.
- `increment_and_loop`: Clears bit 1 of a byte in a5, adds d3 to a word at the address in a1, and then enters a loop that calls another routine while decrementing d3 if d3 is not zero.
- `push_and_load_constant`: Pushes the value in d0 onto the stack, loads the constant 45056 into d0, and branches to another location.
- `test_bit_after_call`: Pushes d0 to the stack, calls another routine, and then branches to another location if bit 5 of a byte in a6 is set.
- `write_and_wait_for_bit`: Stores a word into two separate memory locations and then waits in a loop until a specific bit in a third memory location is cleared.
- `mask_status_byte`: Performs several bitwise operations on a byte at the address in a5 to clear bits 3 and 7 and set bits 4, 5, and 6, then calls another routine and returns.
- `modify_byte_and_branch`: Sets bits 3, 4, 5, and 6 of a byte in a structure in a5, clears bit 3 of that same byte, and then sets bits 3, 4, 5, and 6 of that byte again before branching to another location.
- `update_dma_control_register`: Updates a global variable and modifies a DMA control register to enable DMA channels.
- `parse_compressed_data_stream`: Decodes a custom bitstream or compressed data format by reading bytes and extracting multi-bit values.
- `parse_sample_sequence`: Reads and processes a sequence of sample data from a table to update object properties.
- `update_object_kinematics`: Calculates and updates the position and velocity of an object based on data retrieved from a table.
- `update_channel_period_and_vibrato`: Updates a channel's period and vibrato state from its instrument entry, applying a counter-driven sweep and a sign flip, then stores the new period and volume.
- `clamp_and_store_value`: Compares a value in d0 against a constant, clamps it to a maximum of 64 if it exceeds it, and stores the result and a masked version of d1 into a table or structure via a pointer in a0.
