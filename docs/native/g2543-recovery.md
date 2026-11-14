# Game 2543 (nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02543). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906480 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 39 of 39 named (0 author symbols), 39 of 39 described (2 by rule, 37 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 429 of 429 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 975 of 975 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21445 |
| code | 3110 |
| data | 602885 |
| unused | 279040 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf:bootblock | 1 | 7 | 1 |
| logged disk load from Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf boot-block load | 1 | 127 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 841 | 36 |

## Cases handled

- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a2]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf: custom-format (non-DOS) disk, 1190 non-empty blocks: matched against track-loaded segments
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a2]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf: the boot block reads 20480 bytes from disk offset 606208 to an address (found statically); recovered as a unit
- runtime segment an address: 429 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- nitro (1990)(psygnosis)[cr hz][t +5 full - evil][a2]/Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf: 1 logged loads cover 20480 bytes of the image
- 1 packed track region(s), 600576 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `copy_interleaved_data` (runtime segment an address): Modifies a hardware register, enters a loop to copy bytes from a source table to a destination buffer using calculations derived from d0, and restores the hardware register before returning. Kind: interleaved data copy/reconstruction. Pass path: The routine completes the memory copy of the non-contiguous/interleaved sectors into a linear buffer, allowing the subsequent loader to access the data normally..

## Functions


### Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf:bootblock

- `init_and_copy_table`: Initializes a set of fixed values in a data structure, then invokes a startup routine.

### logged disk load from Nitro (1990)(Psygnosis)[cr HZ][t +5 Full - Evil][a2].adf boot-block load

- `tiny_clr`: Small routine of 1 instruction that clears d0.
- `xor_byte_pattern`: Walks through a 32-bit value one byte at a time, flipping the matching byte of a global variable with an XOR whenever that byte is zero, then decrements the value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_copper_and_registers`: Waits for the copper to reach a specific line, then programs several hardware registers.
- `process_byte_table`: Copies a block of words from a source to a destination in chip memory, then walks a table of byte values, dispatching to different routines for each value and copying a variable-length run of bytes for the ones it recognizes.
- `zero_memory_range`: It zeroes a run of consecutive memory words by writing 0 to each location in turn, advancing one word at a time until it reaches the end address.
- `process_table_and_increment`: Calculates an offset using a value from a structure in a5, accesses a table at a specific memory location, and calls another routine if a byte from a0 is positive; otherwise, it increments a 16-bit value at offset 34 of the structure in a5 and returns.
- `load_table_values`: Uses an index to fetch a byte from a table and then copies several specific bytes from a memory range into a destination buffer at fixed offsets.
- `count_null_bytes`: Iterates through a string until a null byte is found and then stores a calculated value into a destination offset.
- `update_string_index`: Checks a character value and iterates through a string to find a non-zero byte, then updates a global table with calculated values based on the resulting index.
- `decrement_and_wrap_counter`: Decrements a byte in a structure and, if it reaches zero, performs logic to increment or decrement a counter and update a status byte.
- `process_ascii_data`: Loads three pointers from a structure in a5, writes the values 18 and 2 to offsets 34 and 32 of that structure, and then loops through a data block to subtract 48 from each byte and store the result in a second structure until a zero byte is encountered, at which point it calls another routine and branches to another location.
- `update_counters_and_call_table`: If d0 equals 2, it copies a byte from a0 to a4 and calls another routine; it then updates several byte values at specific offsets within a structure in a5 and calls another routine with a pointer in a0.
- `manage_timer_and_mode`: Decrements a countdown timer and, when it reaches zero, resets it and updates a status byte.
- `process_data_table`: Loads a byte from a table at offset 43 of the structure in a5, then enters a loop that modifies d3 and d2 based on the byte value, calling another routine if the byte is 1 or 2.
- `transfer_and_call_routine`: Loads a pointer from a5 into a0, stores d3 and d4 into offsets 34 and 32 of the structure in a5, calls another routine, and then branches to another location.
- `load_pointer_and_branch`: Loads a pointer from offset 74 of the structure in a5 into a0 and branches to another routine.
- `copy_interleaved_data`: Modifies a hardware register, enters a loop to copy bytes from a source table to a destination buffer using calculations derived from d0, and restores the hardware register before returning.
- `update_hardware_and_loop`: Saves registers, performs calculations involving division and subtraction, modifies bits in a5, writes several words to a structure in a4, and enters a loop that calls other routines and waits for a bit to be set in a4.
- `clear_bit_and_branch`: Clears bit 2 of a byte in the structure in a5 and branches to another location.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `process_interleaved_data_table`: Searches a table for a specific marker, then calculates a range based on values extracted from the table and compares it against two input parameters.
- `restore_registers_and_branch`: Restores d2 through d7 and a1 through a3 from the stack, increments the stack pointer, sets bit 1 of a byte in a6, and branches to another location.
- `compare_masked_values`: Performs bitwise AND operations on two values from a structure in a1 using a mask in d6, executes a loop 255 times to XOR a value from a1 with d3, and compares the result with the previously calculated value.
- `call_toggle_status_bit_routine`: Calls a routine to toggle the first bit of a byte located at the address stored in register a5.
- `set_bit_and_branch`: Sets bit 1 of a byte in a5, tests bit 4 of a byte in a6, and branches to another routine if bit 4 is set, otherwise it returns.
- `toggle_status_bit`: Clears bit 0 of a byte at the address in a5, sets bit 0 of the same byte, and then branches to another location.
- `increment_and_loop`: Clears bit 1 of a byte in a5, adds d3 to a word at the address in a1, and then enters a loop that calls another routine while decrementing d3 if d3 is not zero.
- `push_and_branch`: Pushes the contents of d0 onto the stack, loads the word 45056 into d0, and branches to another location.
- `test_bit_after_call`: Pushes d0 to the stack, calls another routine, and then branches to another location if bit 5 of a byte in a6 is set.
- `write_and_wait_for_bit`: Stores a word into two separate memory locations and then waits in a loop until a specific bit in a third memory location is cleared.
- `mask_status_byte`: Performs several bitwise operations on a byte at the address in a5 to clear bits 3 and 7 and set bits 4, 5, and 6, then calls another routine and returns.
- `modify_status_and_branch`: Performs bitwise OR and AND operations on a byte in the structure in a5 and then branches to another location.
- `decode_command_sequence_2`: Decrements a counter in a global structure and, when it reaches zero, decodes a sequence of command bytes from a table by repeatedly decoding length-prefixed strings.
- `decode_length_prefixed_string`: Decrements a counter stored in a data structure and then decodes a length-prefixed string from a table, splitting it into a text pointer and a byte count while advancing past the encoded bytes.
- `decode_command_sequence`: Decrements a counter stored in a global structure and, when it underflows, decodes a sequence of command bytes from a table.
- `apply_movement_update`: Reads a movement table entry selected by an offset and updates a structure's coordinates and a timer value, then applies a direction derived from a nibble of a table byte.
- `update_channel_period_and_vibrato`: Updates a channel's period and vibrato state from its instrument entry, applying a counter-driven sweep and a sign flip, then stores the new period and volume.
- `clamp_and_store_value`: Compares a value in d0 against a constant, clamps it to a maximum of 64 if it exceeds it, and stores the result and a masked version of d1 into a table or structure via a pointer in a0.
