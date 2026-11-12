# Game 1298 (virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01298). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (914608 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 118 of 118 named (0 author symbols), 118 of 118 described (5 by rule, 111 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1110 of 1110 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 880 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2222 of 2239 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11360 |
| code | 12122 |
| data | 703664 |
| padding | 70 |
| unused | 187392 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf:bootblock | 1 | 269 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 847 | 58 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 794 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 141 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 80 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| logged disk load from Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf | 1 | 15 | 2 |
| Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf bytes 29696-30208 (code never loaded by a reference run) | 1 | 92 | 1 |
| Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf bytes 144-512 beside a logged load (code never loaded by a reference run, load address unknown) | 1 | 0 | 0 |

## Cases handled

- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: custom-format (non-DOS) disk, 1392 non-empty blocks: matched against track-loaded segments
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-lgpwide): 636 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 32 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 113 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 72 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: 15 executed PCs whose RAM was overwritten before any snapshot match its bytes 0-144: recovered as a unit run at an address
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: bytes 29696-30208 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate+address
- Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf bytes 144-512 beside a logged load (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 0 of 368 bytes decode)
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: 3 logged loads cover 656 bytes of the image
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: 20 track-loaded blocks found in recovered runtime segments and classified through them
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: blocks 2-26 (bytes 1024-13824) outside the filesystem hold data, not code (opcode density 0.015, entropy 4.19)
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: blocks 29-30 (bytes 14848-15872) outside the filesystem hold data, not code (opcode density 0.018, entropy 5.20)
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: blocks 46-46 (bytes 23552-24064) outside the filesystem hold data, not code (opcode density 0.027, entropy 3.23)
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: blocks 64-68 (bytes 32768-35328) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.70)
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: blocks 70-72 (bytes 35840-37376) outside the filesystem hold data, not code (opcode density 0.003, entropy 2.74)
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: blocks 74-123 (bytes 37888-63488) outside the filesystem hold data, not code (opcode density 0.006, entropy 4.51)
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: blocks 132-167 (bytes 67584-86016) outside the filesystem hold data, not code (opcode density 0.011, entropy 6.40)
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: blocks 499-608 (bytes 255488-311808) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.62)
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: blocks 616-719 (bytes 315392-368640) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.90)
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: blocks 724-849 (bytes 370688-435200) outside the filesystem hold data, not code (opcode density 0.037, entropy 6.22)
- virus (1988)(d. braben)[cr shockwave][t +6 axxess 1992]/Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf: blocks 851-1759 (bytes 435712-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.14)
- 1 never-loaded region(s), 368 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf:bootblock

- `store_next_entry_trap`: Loads the address of the next routine into a register, stores it into a global variable, and executes a trap before falling through into the following routine.
- `setup_vectors_and_jump`: Disables interrupts and DMA channels, sets up a new stack pointer, and copies a vector table from read-only memory into a memory region in a loop before jumping to it.
- `load_sector_then_jump`: Loads the address of a disk sector into a register, calls a routine, then loads a memory address and a byte count and calls another routine before jumping to that address.
- `store_masked_selector_word`: Computes a masked selector from a routine's result, stores its low two bits plus three as a word at the start of a structure in a3, stores a longword at offset three of that structure, writes two control values to global variables, and calls another routine before returning.
- `load_global_address`: Loads the address of a global variable into a register and returns.
- `load_interface_addresses`: Loads the addresses of the two parallel interface data registers into registers and returns without touching them.
- `step_units_in_rows_eleven`: Derives a unit count from one input and a position from another, calling a routine for each unit.
- `pulse_control_bit_on_status`: Initializes a control byte through a pointer, calls a routine that busy-waits on the beam position registers, then waits on a status bit and repeatedly pulses a control bit while a second status bit remains set.
- `validate_range_then_pulse_control`: Returns minus one when its input lies outside the zero-to-seventy-nine range.
- `alternate_flag_and_pulse_control`: Checks a stored flag and, when it is zero, increments it and clears a control bit, otherwise clears it, increments another counter, pulses a control bit, and returns the flag value.
- `set_control_bit_from_input`: Sets bit two of a control byte through a pointer when a register is zero and clears it otherwise.
- `set_selected_bit_high_flag`: Calls a routine that busy-waits on the beam position registers until it returns zero, reads a 16-bit value from a memory location, sets the bit selected by its low three bits in a control byte, sets the top bit of that byte, and returns.
- `repeat_raster_wait_until_zero`: Loads a counter with eleven and repeatedly calls a routine that busy-waits on the beam position registers until that routine returns zero, then restores the registers and returns.
- `repeat_raster_poll_until_zero`: Loads a counter with one hundred and twenty repeatedly calls a routine that busy-waits on the beam position registers until that routine returns zero, then restores the registers and returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `decode_interleaved_marker_blocks`: Waits for the vertical retrace, then scans a data structure for the 17721 marker and, for each occurrence, interleaves the bits of two adjacent words into a new buffer.
- `interleave_structure_bytes`: Reads two bytes from a structure, interleaves their bits, and stores the result in a register before returning.
- `copy_block_by_modular_index`: Computes an index from the low nibble of a register minus a stored byte, adding eleven when the result is negative, then copies 512 bytes from a table at that index into the destination and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_constant_long`: Stores the constant value 4144 in hex into a global variable at offset 64.
- `accumulate_counters_periodically`: Adds three 16-bit values from a structure in d0 to three global variables, then increments a counter at offset 98 of that structure.
- `skip_negative_words`: Skips past the leading negative 16-bit words in the table pointed to by a0, then stores a pointer one word past the first non-negative word into the global at offset 16704 and returns that word in d0.
- `pack_table_entries_into_buffer`: Stores an address in a global variable, then reads words from a table until a negative word is found, writing a packed high nibble and low nibble pair into a destination buffer at offset 49808, and returns.
- `copy_multiple_blocks`: Subtracts d7 from a value in a global variable, then performs three consecutive loops to copy 17 words from three different source locations into a destination buffer at offset 48398, appending a zero word after each block.
- `program_blitter_scan_sequence`: Disables blitters and clears several custom registers, then loops 65 times setting up blitter channels with addresses derived from a table pointer and a destination buffer.
- `branch_to_jump_vector_update`: Calls another routine and then updates a global variable with a value from a local address before replacing that global variable with the address of a different routine.
- `branch_to_address`: Performs a relative branch to another location.
- `reorganize_and_initialize_audio_data_2`: Iterates through a list of data structures to reorder and link pointers based on specific offsets, then checks a magic value to potentially perform a linked-list traversal.
- `branch_to_instruction_stream`: Performs a short branch to another routine.
- `clear_audio_volumes_2`: Performs a branch to another routine.
- `set_table_values_2`: Performs a branch to another routine.
- `xor_word_from_table_2`: Loads a 16-bit word from a table using a pointer in a6, performs an eor with 15 on it, and returns.
- `update_dma_control_and_table`: Writes a masked version of a 16-bit register to the DMA control register and stores the bitwise XOR of that masked value with 15 into a table at an offset of 44 from a6, then returns immediately.
- `branch_to_distant_address`: Performs a short relative branch to a different location in the code.
- `copy_data_to_structure_2`: Copies several longwords and words from various global locations into a destination structure pointed to by a0.
- `update_jump_vector_2`: Calls another routine, then updates a global word with a value from a local address and replaces that global word with the address of another routine.
- `update_jump_vector`: Loads a longword from a table into a global variable, then calls another routine.
- `check_interrupt_request`: Saves d0 to the stack, reads the interrupt request register, masks bit 5, and if the result is zero, calls another routine before restoring d0.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `reorganize_and_initialize_audio_data`: Iterates through a list of data structures to reorder and link pointers based on specific offsets, then checks a magic value to potentially perform a linked-list update on another set of structures.
- `clear_audio_volumes`: Clears a 16-bit value in a structure, sets a value at a fixed address, and clears the four audio volume registers and the DMA controller.
- `xor_word_from_table`: Loads a word from a table using a pointer in a6, performs an eor with 15 on it, and returns.
- `update_dma_and_table`: Writes a masked version of d0 to the DMACON register, then stores the eor of that value with 15 into a table offset 44 from a6.
- `load_word_from_table`: Loads a word from a table at offset 52 from a6 and returns.
- `copy_data_to_structure`: Copies several longwords and words from various offsets of a table into a destination structure pointed to by a0.
- `set_table_values`: Masks d0 with 63 and stores the result in two locations at offsets 34 and 36 from a6, sets a 16-bit value at offset 30 to 65535, and clears a word at offset 32 from a6.
- `process_instruction_stream`: Checks a 16-bit value at offset 30 of a structure in a6, calls another routine if non-zero, then checks a 16-bit value at offset 52 of the structure.
- `parse_instruction_stream`: Resets several fields in a structure, then parses a byte-stream to update state variables, including a counter, a pointer, and various data fields.
- `update_structure_field_bits`: If a 16-bit register is non-zero, it writes the lower 4 bits and the upper 12 bits to offsets 38 and 40 of the structure in a0, while setting offsets 34 and 42 to specific constant values.
- `update_structure_fields_2`: Negates a 16-bit value and stores it at offset 34 of a structure in a0, masks a byte from a register with 63 and stores it at offset 18, then calls another routine if the masked value is not 63.
- `update_object_coordinate_index`: Masks a byte from a register with 63 and stores it at offset 18 of a structure in a0, then calls update_object_coordinates to update the structure's position using a table lookup.
- `branch_to_sub_1ebc`: Performs a short branch to another routine.
- `branch_to_offset`: Performs a short branch to a specific offset within another routine.
- `branch_to_address_3`: Performs a relative branch to a specific offset.
- `branch_to_address_4`: Performs a relative branch to a specific offset.
- `branch_to_middle_of_routine_2`: Performs a short branch into the middle of another routine.
- `branch_to_audio_update`: Returns immediately by performing a short branch to another location.
- `branch_to_address_2`: Performs a relative branch to another location.
- `branch_to_offset_2`: Performs a short branch to a specific offset within another routine.
- `update_structure_and_recurse`: Writes a 16-bit value to offset 2a of a structure in a6, increments a 16-bit value at offset 36 of the same structure, and performs a recursive call if a masked byte from a pointer in a5 does not equal 63.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `branch_to_distant_address_2`: Performs a short relative branch to a location outside the current routine's immediate range.
- `dispatch_jump_table_entry`: Calculates an offset by shifting a 16-bit value in register d1 right by 4 bits, multiplies it by 4, and performs an indirect jump to a location in a table using that offset from a global variable.
- `branch_to_offset_3`: Performs a short relative branch to a location within another routine.
- `update_structure_fields`: Tests a 16-bit register, and if non-zero, writes its value and a modified version of itself to offsets 26 and 28 of a structure in a0, then branches to another location.
- `write_negated_value`: Negates a 16-bit register, stores the result at offset 22 of a structure in a0, and branches to another location.
- `update_object_coordinates`: Updates a structure in a0 by extracting a byte from a5, masking it, and using it as an index into a table via a4 to calculate a new offset.
- `write_multiple_values`: Writes a word of all ones to offset 54 of a structure in a0 and branches; otherwise it writes a word of all ones to offset 44 of the structure in a0 and branches; otherwise it writes the value 2 to offset 50 of a structure in a6, clears a word at offset 24 of the structure in a0, and branches.
- `update_audio_parameters`: Calls another routine and branches to another location; otherwise it performs comparisons on a 16-bit register to write a value and a constant to offsets 1e and 20 of a structure in a0.
- `modify_structure_values`: Writes a 16-bit register to offset 2a of a structure in a6, increments a value at offset 36 of the same structure, and branches to another location; otherwise it compares a register to 64 and writes a value to offset 1c of a structure in a0.
- `dispatch_indirect_jump_2`: Writes a word to offset 40 of a structure in a6, then performs a series of comparisons on d1 to either branch to a fixed location or write a new value to offset 26 of a structure in a6 before branching to a fixed location.
- `dispatch_indirect_jump`: Calculates an offset by shifting a value in d1 and uses it to jump to a location in a jump table via an indirect jump.
- `store_and_set_bit`: Writes d0 to the word at offset 16 of the structure in a0 and to a table entry indexed by d0 in the table at a4, then sets bit 5 of the byte at offset 44 of the structure in a6 if the corresponding bit at offset 12 is set.
- `disable_audio_channels`: Reads and clears a 4-bit flag in the structure at a6, disables DMA via DMACON, then for each of four channels waits for a raster line and writes its length and pointer registers, setting the disable bit after the first pass.
- `wait_for_raster_line_2`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `update_object_physics_and_state`: Updates an object's position and velocity by applying acceleration and clamping values, manages a three-state animation cycle by swapping a 16-bit value and fetching a new coordinate from a table, calculates a division-based value from a table entry, and updates a secondary structure's coordinates and flags.
- `reset_audio_volumes`: Checks a 16-bit value at offset 36 of a structure in a6; if it is not 65, increments a 16-bit value at offset 32 of the structure, and if that value reaches 65, sets four audio volume registers to zero and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_globals_enable_dma`: Clears four 32-bit global variables to zero and writes a value of 15 to the DMA controller register to enable the DMA channels, then returns.
- `modify_global_value`: Checks a byte at a global address; if non-zero, it subtracts 8192 from d1, negates d1, shifts it right by 2, and stores the result in a long word at a global address; otherwise it returns.
- `update_structure_from_table`: Uses a value in d0 to index into a table and retrieves a structure, then performs a series of comparisons against four constant values to determine if the structure's field at offset 4 falls within specific ranges.
- `modify_struct_flags_copy`: Loads eight 16-bit values from the structure pointed to by a0 into registers, adds a value from a global table to one of them, sets bit 2 of a flag in another register, stores the results into the structure pointed to by a1, then restores the registers and returns.
- `check_struct_bit`: Tests bit 6 of a byte at offset 6 of a structure in a0 and, if not set, adds a constant to a word at offset 8 of the structure and stores the result at offset 8 of a structure in a1.
- `compare_and_branch_sequence`: Compares a word in d1 with d2 and branches if d2 is greater; otherwise it compares d1 with d3 and branches if d3 is greater; otherwise it compares d1 with a word at offset 4 of a structure in a0 and branches if that value is greater; otherwise it loads an address into a1 and branches.
- `compare_and_branch_2`: Compares d2 with d3, and if d2 is not greater, compares d2 with a word at offset 4 of a structure in a0; if d2 is not greater than that value, it loads a pointer into a1 and branches.
- `compare_and_branch`: Compares a word at offset 4 of a structure in a0 with d3 and, if d3 is not greater, performs a branch to another routine.
- `update_audio_channels`: Loads four addresses into registers a1 and calls another routine to update DMA parameters for each address.
- `update_dma_parameters`: Decrements a 16-bit value in a structure at a0; if it is non-zero, it either copies data from a table into the structure or updates the structure with new values and sets bit 15 of the DMA controller register, then loops if a counter is still positive.
- `apply_bitmask_or_3`: Calculates a destination address from a table and a source address from a buffer, then loops eight times to OR a shifted byte from the source into the destination and four specific offsets.
- `apply_bitmask_or`: Performs a loop eight times, taking a byte from a1, shifting it, and performing an OR operation with the results into four specific offsets of a structure pointed to by a0.
- `apply_bitmask_and_or_2`: Calculates a destination address from a table and a source address from a buffer, then loops eight times to perform a combination of AND and OR operations using a shifted byte from the source into the destination and four specific offsets.
- `apply_bitmask_and_or`: Performs a loop eight times, taking a byte from a1, shifting it, and performing an AND operation with a bitwise NOT of the value into one offset and an OR operation into others within a structure pointed to by a0.
- `apply_bitmask_and_or_alt_2`: Calculates a destination address from a table and a source address from a buffer, then loops eight times to perform a combination of AND and OR operations using a shifted byte from the source into the destination and three specific offsets.
- `apply_bitmask_and_or_alt`: Performs a loop eight times, taking a byte from a1, shifting it, and performing an AND operation with a bitwise NOT of the value into one offset and an OR operation into others within a structure pointed to by a0, using different offsets than R6.
- `apply_bitmask_to_planar_buffer`: Calculates a destination address in a bitplane buffer based on two coordinates and performs an eight-iteration loop that modifies word values at specific offsets using bitwise shifts, ANDs, and ORs derived from a source byte table.
- `apply_bitmask_and_or_mixed`: Performs a loop eight times, taking a byte from a1, shifting it, and performing an AND operation with a bitwise NOT of the value into one offset, an OR into another, and an AND into a third offset within a structure pointed to by a0.
- `apply_bitmask_or_and_2`: Calculates a destination address from a table and a source address from a buffer, then loops eight times to perform a combination of OR and AND operations using a shifted byte from the source into the destination and three specific offsets.
- `apply_bitmask_or_and`: Performs a loop eight times, taking a byte from a1, shifting it, and performing an OR operation into one offset and an AND operation with a bitwise NOT of the value into others within a structure pointed to by a0.
- `apply_bitmask_to_buffer_3`: Calculates a destination address based on two input values and a global pointer, then enters a loop that iterates eight times.
- `apply_bitmask_to_buffer`: Iterates eight times, reading a byte from a source pointer and applying it as a bit-shift amount to modify values in a destination buffer.
- `apply_bitmask_to_planar_buffer_2`: Calculates a destination address in a planar bitplane buffer based on coordinates and a source address in a table, then performs an eight-iteration loop to bitwise OR or AND shifted bytes into specific bitplane offsets.
- `apply_bitmask_or_2`: Performs a loop of eight iterations where it takes a byte from a source, shifts it left or right based on a value in a register, and then uses the result to bitwise OR it into a destination word at specific offsets, while also bitwise ANDing the complement of that value into other offsets in the destination.
- `apply_bitmask_to_planar_buffer_3`: Calculates a destination address in a planar graphics buffer based on coordinates and performs an eight-iteration loop that applies a bitwise mask to specific words at fixed offsets within a structure.
- `apply_bitmask_and`: Performs a loop of eight iterations where it takes a byte from a source, shifts it left or right based on a value in a register, and then uses the result to bitwise AND it into a destination word at specific offsets, while also bitwise ANDing the complement of that value into other offsets in the destination.
- `apply_bitmask_to_buffer_4`: Calculates a destination address based on two input values and a global pointer, then enters a loop that runs eight times to modify a buffer.
- `apply_bitmask_to_buffer_2`: Performs a loop eight times to modify a buffer by applying bitwise OR and AND operations using values derived from a source byte stream.
- `wait_clear_then_store_counters`: Waits until a 16-bit global variable becomes zero, then stores all ones into that variable and the following 16-bit variable, copies the 32-bit vertical blank counter value into a 32-bit global at offset 8, clears the 16-bit variable at offset 14, stores the value 4 in the 16-bit variable at offset 16, and stores all ones in the 16-bit variable at offset 18.
- `wait_for_zero_variable`: Calls another routine, then tests a 16-bit global variable and returns when it is zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_interrupt_handlers`: Sets the interrupt mask, assigns several routine addresses to a table of pointers, writes specific bytes to CIAA and CIAB interrupt acknowledge registers, tests those registers, and updates the interrupt mask and enable register before returning.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `handle_interrupt_and_input`: Updates hardware registers for Blitter and DMA, processes joystick and mouse movement data, and copies a block of data from a pointer to a destination before calling another routine.
- `handle_keyboard_command_dispatch`: Saves two registers and reads a keyboard status byte, returning early when bit 3 is clear.
- `set_global_shorts`: Stores the value 128 into two consecutive 16-bit words at a specific memory location.
- `load_global_shorts`: Loads two 16-bit words from a specific memory location into registers and returns the second word masked with three.
- `get_masked_byte`: Reads a byte from a specific memory location and returns it masked with three.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_hardware_and_display`: Initializes several memory locations with values, configures various hardware registers including DMA, Copper, and Blitter settings, and makes four calls to other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### logged disk load from Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf

- `load_address_to_global`: Loads the address of the instruction following the trap into a 32-bit global variable and executes a trap.
- `configure_dma_interrupts_and_colour`: Writes the DMA and interrupt enable registers and the background colour register, then makes two calls to unnamed routines.

### Virus (1988)(D. Braben)[cr Shockwave][t +6 Axxess 1992].adf bytes 29696-30208 (code never loaded by a reference run)

- `copy_tables_configure_blitter`: Returns immediately when called with a nonzero value in a1, otherwise it copies several tables into fast memory, configures the blitter and playfield registers based on the current display position, and returns.
