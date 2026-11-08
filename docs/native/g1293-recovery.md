# Game 1293 (space harrier (1989)(grandstand)[cr band][t +2 band]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01293). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902432 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 72 of 72 named (0 author symbols), 72 of 72 described (1 by rule, 71 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 848 of 848 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 512 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3760 of 7685 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 196596 |
| code | 28388 |
| data | 676935 |
| padding | 1 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf:bootblock | 1 | 152 | 7 |
| logged disk load from Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf boot-block load | 1 | 97 | 2 |
| logged disk load from Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf boot-block load | 1 | 0 | 0 |
| logged disk load from Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf boot-block load | 1 | 7134 | 49 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 302 | 14 |
| Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf bytes 196096-196608 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- space harrier (1989)(grandstand)[cr band][t +2 band]/Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf: custom-format (non-DOS) disk, 1728 non-empty blocks: matched against track-loaded segments
- space harrier (1989)(grandstand)[cr band][t +2 band]/Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf: the boot block reads 52224 bytes from disk offset 664576 to an address (found statically); recovered as a unit
- space harrier (1989)(grandstand)[cr band][t +2 band]/Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf: the boot block reads 135680 bytes from disk offset 22528 to an address (found statically); recovered as a unit
- space harrier (1989)(grandstand)[cr band][t +2 band]/Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf: the boot block reads 37888 bytes from disk offset 158208 to an address (found statically); recovered as a unit
- runtime segment an address: 223 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- space harrier (1989)(grandstand)[cr band][t +2 band]/Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf: bytes 196096-196608 look like code no run loaded: recovered position-independently (load address unknown)
- space harrier (1989)(grandstand)[cr band][t +2 band]/Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf: 4 logged loads cover 226304 bytes of the image
- space harrier (1989)(grandstand)[cr band][t +2 band]/Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- space harrier (1989)(grandstand)[cr band][t +2 band]/Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf: blocks 2-43 (bytes 1024-22528) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.35)
- space harrier (1989)(grandstand)[cr band][t +2 band]/Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf: blocks 385-973 (bytes 197120-498688) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.16)
- space harrier (1989)(grandstand)[cr band][t +2 band]/Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf: blocks 975-1297 (bytes 499200-664576) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.23)
- space harrier (1989)(grandstand)[cr band][t +2 band]/Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf: blocks 1400-1759 (bytes 716800-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.31)
- 1 never-loaded region(s), 512 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `write_cia_registers` (Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf boot-block load +an address to an address (37888 bytes)): Writes the contents of d3 and d4 to the CIAA Data Register and then checks a bit in the CIAB Port B register. Kind: disk hardware/sync check. Pass path: sub_271E (the branch following the call to the routine).
- `configure_dma_and_process_data` (Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf boot-block load +an address to an address (37888 bytes)): Sets several custom chip registers, performs a loop that bitwise manipulates data from a table and stores the results in a destination pointed to by a2, and finally calculates a new address in a0. Kind: bit-manipulation/data-scrambling check. Pass path: the loop completes and execution continues to the next instruction in the caller (bsr.w an address).

## Functions


### Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf:bootblock

- `initialize_boot_parameters`: Calls another routine, writes the value 32767 to offset 154 of a structure in a0, writes 288 to offset 150 of the structure, loads the address of a local label into a1, and stores a1 at offset 32 of a structure.
- `copy_data_loop_and_jump`: Sets the processor status register, initializes a stack pointer, and enters a loop that copies words from a source to a destination at offset 80 before jumping to the address stored at that location.
- `initialize_hardware_parameters_2`: Checks bit 5 of a word at offset 30 of the structure in a0; if set, it stores a word at offset 156 and writes a descending sequence of words starting from 4095 to a destination at offset 384.
- `initialize_hardware_parameters`: Copies sixteen words from a table to a destination pointed to by a0, writes several constant values to offsets of a0, stores two addresses into a0, and performs a memory comparison loop before jumping to an address.
- `perform_io_operations`: Performs three consecutive I/O operations by writing specific values to offsets 28, 36, 40, and 44 of a structure pointed to by a1, then calls a routine via a pointer from a global variable and calls another routine at a fixed address between the first and second operations.
- `update_hardware_status`: Loads the address of the custom chip into a0, writes the value 112 to offset 156 of that address, performs a bitwise NOT on a value at an offset relative to the program counter, and branches to an earlier instruction if the result is not zero.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### logged disk load from Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf boot-block load

- `extract_and_shift_long_word`: Loads a long word from the address in a0, stores its lower byte into a 16-bit word at the address in a5, sets the carry flag, and returns a long word that has been rotated right by one bit.
- `process_bit_stream`: Decrements a counter in d1 and loops, calling another routine and performing bit shifts until d1 reaches zero.

### logged disk load from Space Harrier (1989)(Grandstand)[cr Band][t +2 Band].adf boot-block load

- `update_blitter_display_parameters`: Calculates and sets Blitter parameters including source, destination, and size, while performing a series of memory clears and updating a table of values using division.
- `process_graphics_updates`: Performs several calls to routines that process data blocks and configure blitter parameters, updates values in a structure pointed to by a6, and returns.
- `process_data_block_with_loop`: Calculates an offset into a structure, enters a loop that processes bytes from a source pointer, uses a table to determine values, and calls another routine within the loop.
- `render_high_score_screen`: Copies a block of data into a structure, then enters a loop that processes character input to display high score names.
- `copy_and_clear_data`: Compares a value from a global variable against a list, copies four words and three bytes from two source locations to a destination, and then clears three bytes at a destination offset if a condition is met.
- `dispatch_interrupt_handler`: Checks bit 2 of a global variable and, if set, jumps to an address stored in a global variable; otherwise, it writes 4 to a hardware register and returns from interrupt.
- `update_interrupt_parameters`: Checks a word at offset 38 of a structure in a6, if non-zero calculates two new values based on other structure members and stores them at offsets 104 and 106, calls another routine, checks offset 166 and calls another routine, checks offset 194 and calls another routine, and finally increments and masks a value at offset 172.
- `update_interrupt_vector`: Compares two values on the stack, and if the difference exceeds three, updates a stack value, performs a bitwise NOT on another stack value, and calls dispatch_interrupt_handler to set a new interrupt vector and a global value.
- `adjust_bcd_values`: Performs four BCD adjustments by adding bytes from a source table to a destination table at specific offsets.
- `fill_table_entry`: Clears a longword at a destination, then loops up to 16 times to check if a word in a table is zero; if so, it copies five words from a source to that table entry and calls another routine.
- `call_graphics_routine`: Sets d4 to zero and d3 to seven, then calls another routine.
- `update_object_graphics`: Sets d4 to zero and d3 to one, clears d2 after performing a swap and shift operation, and then calls another routine.
- `update_graphics_buffer`: Performs bitwise operations to update a destination buffer using data from a source table, looping through multiple rows and columns based on values in registers.
- `setup_blitter_copy_rows`: Waits for the blitter to be free, then programs its control, modulo, first and last word mask, pointer and size registers for each pair of rows, decrementing a count until done.
- `initialize_hardware_and_interrupt_vectors`: Configures several hardware registers, clears multiple fields in a structure, copies data from two sources into a destination using a loop, and populates several global pointers with the addresses of other routines.
- `process_joystick_input`: Checks hardware status bits, modifies CIAA registers, and writes a calculated byte to a stack offset based on a lookup table.
- `invert_and_update_status`: Writes 112 to a hardware register, inverts a word in a global variable, and returns if the inverted value is not zero.
- `update_display_and_interrupt_state`: Sets custom hardware registers, updates a global variable based on bitwise operations on hardware registers, copies a block of data from a pointer in a structure to a destination, and conditionally updates a byte in a global variable before calling another routine and returning via an exception return.
- `configure_audio_channels`: Sets audio parameters for two channels using registers, then checks a bitmask to decide whether to update the channel addresses and period values.
- `wait_for_bit_toggle`: Waits for bit 6 of a global variable to become zero, then waits for it to become one, and performs two empty loops before returning.
- `wait_for_joystick_bit_and_fill_table`: Polls a bit in a hardware register until it changes state, performing two empty loops based on the bit status, then populates a table with interleaved 16-bit values derived from a register and its byte-swapped counterpart.
- `process_joystick_input_and_update_player`: Polls the CIAA port to update directional counters and a status byte in a structure, then applies smoothing and clamping logic to update player position and velocity values based on the input.
- `calculate_blitter_parameters`: Performs complex fixed-point arithmetic to calculate coordinates and dimensions, then updates several bitplane and blitter control registers in the hardware customization registers.
- `configure_blitter_parameters`: Waits for the blitter, then writes various control words, masks, and pointers to the blitter registers based on values in a1, a4, and d1, using d7 to offset pointers.
- `program_blitter_fixed_control`: Waits for the blitter, then writes fixed control words and the first and last word masks, along with source and destination modulo values of 42, and sets the pointer and size registers.
- `initialize_and_expand_data_structures`: Clears a large block of memory starting at a specific address, then performs a series of nested loops to copy, scale, and rotate data from a source table into various structures within a primary data block.
- `transform_color_palette_data`: Reads bytes from a source table in a6 and performs arithmetic operations including additions, bit shifts, and rotations to calculate new 16-bit values.
- `expand_and_resample_input_table`: Reads two 16-bit values from a table, performs bounds checking and calls another routine if values exceed specific limits, then calculates scaling factors to expand the input data into a larger destination table using a loop.
- `process_nested_data_tables`: Performs a nested loop that manipulates bits in a table using logical operations and bitwise inversions, while updating several values in a structure at a6.
- `load_data_and_call_routine`: Pops registers to the stack, loads a pointer and two 32-bit values from a table pointed to by a1, extracts two bytes from the table into d0 and d7, calls another routine, and returns.
- `clear_global_byte`: Sets a byte at offset 106 of a structure in a6 to zero and returns.
- `configure_hardware_and_status`: Sets bit 14 of the INTENA register, calls two other routines, calculates a value by dividing a sum of a pointer and a register by 1600 decimal, calls a third routine, writes two bytes to the CIAA Data Register via another routine, sets bit 15 of the INTENA register, and writes a byte to a global variable.
- `loop_hardware_calls`: Pushes a0 to the stack, calls another routine, increments d0, calls another routine, loops based on the value in d1, restores a0, and returns.
- `write_cia_registers`: Writes the contents of d3 and d4 to the CIAA Data Register and then checks a bit in the CIAB Port B register.
- `initialize_cia_registers`: Calls a routine to write values to the CIAA Data Register, sets three bytes in memory, and waits for a bit in a global variable to be set before returning.
- `setup_hardware_and_branch`: Writes two bytes to registers, calls two different routines, and then branches to another routine.
- `load_registers_and_branch`: Stores the values 114 and 115 into registers d3 and d4, then branches to another location.
- `configure_dma_and_process_data`: Sets several custom chip registers, performs a loop that bitwise manipulates data from a table and stores the results in a destination pointed to by a2, and finally calculates a new address in a0.
- `search_and_calculate_offset`: Searches a list for a specific word, performs bitwise manipulation on two longwords, and returns a calculated offset in a2.
- `check_input_and_call_handlers`: Reads a byte from a hardware register, checks bit 4, and if not set, enters a loop that calls two different routines before branching.
- `set_hardware_and_check_bit`: Writes specific values to two hardware registers and waits for bit 0 of a register to be set.
- `handle_input_and_status`: Saves registers, compares a byte at a specific address to 255, clears that byte if they are equal, calls another routine, performs arithmetic on a value from a table, compares the result to a value in d1, calls another routine or branches, modifies bit 2 of a hardware register, and updates a global byte before returning.
- `toggle_dma_control`: Writes a value to the DMACON register, calls another routine, and then writes a different value to the DMACON register before returning.
- `update_bitmapped_data`: Converts a character to a numeric value and uses it to perform a series of bitwise operations on a target memory buffer using a loop.
- `process_null_terminated_string`: Reads bytes from a pointer in a0 and calls another routine for each non-zero byte encountered.
- `call_subroutine_with_stack_save`: Saves registers d0, d2, and d3 to the stack, calls another routine, restores the registers, and returns.
- `convert_nibble_and_call`: Performs a bitwise rotation on a byte in d2, converts the lower nibble to an ASCII character, and calls another routine if the character is greater than 57.
- `process_bit_nibbles`: Rotates a longword in d5, performs bitwise operations to extract nibbles, and calls another routine in a loop based on the value of d3.
- `perform_empty_operations`: Performs a long sequence of logical OR operations on d0 but does nothing else.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_counters_and_table_data`: Sets a hardware bit, performs arithmetic on values from a table, and enters loops that call other routines to update various fields in a structure based on table lookups and bitwise shifts.
- `advance_table_stepped_counter`: Decrements a counter at offset fourteen of the addressed structure and reloads it from a lookup table when it reaches zero, then adds a table entry to a byte at offset fifteen.
- `update_hardware_and_structure_counters`: Calculates a value using multiplication and bit shifts, stores it in a hardware register, and performs complex decrements and updates on several multi-byte counters and values within a structure pointed to by a0.
- `clear_hardware_and_globals`: Clears five specific bytes in memory, sets two hardware registers to specific values, and enables DMA channels.
- `update_structure_field_via_table`: Increments a byte from a structure, compares it to another byte in the structure, and if they match, uses the value to index into a table to update a field in the structure before branching.
- `set_status_bits`: Sets bit 5 and bit 1 of a byte at the address pointed to by a0, then branches to another routine.
- `set_status_bit`: Sets bit 5 of a byte at the address pointed to by a0, then branches to another routine.
- `update_object_data`: Copies a byte from a1 to offset 22 of a structure in a0, copies the next byte from a1 to d0, stores d0 at offset 23 of the structure, doubles d0, stores the result at offset 21 of the structure, sets bit 6 of the byte at a0, and branches to another routine.
- `copy_byte_to_structure`: Copies a byte from the address pointed to by a1 to offset 16 of the structure in a0, then branches to another routine.
- `set_multiple_bits`: Sets bit 7 and bit 3 of a byte at the address pointed to by a0, then branches to another routine.
- `process_object_timer_and_data`: Decrements a byte at offset 17 of a structure, and if it reaches zero, performs bitwise operations and uses a value from a table to update various offsets in the structure or calls another routine via a jump table.
- `reset_object_timer_from_input`: If a byte at offset 48 of the structure in a0 is zero, it clears a byte at offset 28, sets a byte at offset 49 to 2, and writes a 16-bit value derived from a byte at offset 51 to the DMA control register.
- `load_sequence_entry_fields`: Looks up a byte from a table indexed by an input value and stores it in an object field together with a second table-derived byte, then sets a countdown to one.
- `update_value_from_table_record`: When a mode byte is set, it looks up a ten-byte table record by index, decrements a counter, and stores a difference and an offset in the output fields.
