# Game 144 (Alfred Chicken): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00144). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907248 top-level bytes in 31 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 118 raw-word lines |
| Functions named and described | pass, 292 of 292 named (0 author symbols), 292 of 292 described (13 by rule, 278 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1017 of 1017 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1210880 bytes, 257 functions |
| Independent decoder cross-check (Ghidra against capstone) | 13034 of 15160 instructions identical in start and length, 9 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12540 |
| code | 10832 |
| data | 618602 |
| padding | 5690 |
| unused | 259584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Alfred Chicken.adf:TRAINER | 2 | 202 | 2 |
| Alfred Chicken.adf:alfred | 1 | 78 | 9 |
| Alfred Chicken.adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 549 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 815 | 12 |
| Alfred Chicken.adf:loader.cod (raw code file, load address unknown) | 1 | 1408 | 74 |
| Alfred Chicken.adf:HIALL.COD#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Alfred Chicken.adf:AT80000.COD#unpacked (raw code file, load address unknown) | 1 | 3273 | 49 |
| Alfred Chicken.adf:FRONTEND.COD#unpacked (raw code file, load address unknown) | 1 | 1759 | 31 |
| Alfred Chicken.adf:LOWER.COD#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Alfred Chicken.adf:ATC00000.COD#unpacked (raw code file, load address unknown) | 1 | 3273 | 49 |
| Alfred Chicken.adf:AT200000.COD#unpacked (raw code file, load address unknown) | 1 | 3785 | 54 |

## Cases handled

- Alfred Chicken/Alfred Chicken.adf:GLASS.PAK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (11289 -> 61440 bytes)
- Alfred Chicken/Alfred Chicken.adf:HIALL.COD: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (86878 -> 319774 bytes)
- Alfred Chicken/Alfred Chicken.adf:AT80000.COD: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (49824 -> 107134 bytes)
- Alfred Chicken/Alfred Chicken.adf:CHEESE.PAK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (5453 -> 36352 bytes)
- Alfred Chicken/Alfred Chicken.adf:BOOK.PAK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10967 -> 58880 bytes)
- Alfred Chicken/Alfred Chicken.adf:BABY.PAK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (14821 -> 61440 bytes)
- Alfred Chicken/Alfred Chicken.adf:SCIFI.PAK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (10856 -> 40960 bytes)
- Alfred Chicken/Alfred Chicken.adf:FRONTEND.COD: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (145756 -> 226956 bytes)
- Alfred Chicken/Alfred Chicken.adf:LOWER.COD: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (121200 -> 338052 bytes)
- Alfred Chicken/Alfred Chicken.adf:ATC00000.COD: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (49826 -> 107134 bytes)
- Alfred Chicken/Alfred Chicken.adf:WOOD.PAK: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (15051 -> 56320 bytes)
- Alfred Chicken/Alfred Chicken.adf:AT200000.COD: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (49822 -> 107134 bytes)
- runtime segment an address: 331 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 487 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Alfred Chicken/Alfred Chicken.adf:loader.cod: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Alfred Chicken/Alfred Chicken.adf:HIALL.COD#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Alfred Chicken/Alfred Chicken.adf:AT80000.COD#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Alfred Chicken/Alfred Chicken.adf:FRONTEND.COD#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Alfred Chicken/Alfred Chicken.adf:LOWER.COD#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Alfred Chicken/Alfred Chicken.adf:ATC00000.COD#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Alfred Chicken/Alfred Chicken.adf:AT200000.COD#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 884782 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Alfred Chicken.adf:TRAINER

## Copy protection found

- `configure_hardware_and_loop` (loader.cod (raw code file, load address unknown)): Enters a loop that runs up to three times; in each iteration, it calls three different routines and checks bit 3 of a hardware register; if the bit is set, it writes several values to custom hardware registers and calls another routine, otherwise it continues the loop. Kind: disk hardware/track validation check. Pass path: bne.b an address.
- `configure_dma_registers` (loader.cod (raw code file, load address unknown)): Writes several 16-bit values to specific offsets of a structure pointed to by a1, including a pointer from a0, and modifies d0 before writing it twice to offset 36 of the structure. Kind: disk track/sync validation. Pass path: the branch taken when bne.b is NOT triggered (the fall-through path).
- `check_hardware_status` (loader.cod (raw code file, load address unknown)): Calls another routine, checks bit 1 of a hardware register, and either calls another routine or writes specific values to hardware registers at offsets a fixed address and a fixed address. Kind: disk hardware/integrity check. Pass path: beq.b an address (at BA4) and bne.b an address (at C0E).
- `write_hardware_constant` (loader.cod (raw code file, load address unknown)): Writes the constant 1024 to a hardware register, tests a word at an offset from a6, and if the result is not negative, loads the constant 255 into d1. Kind: hardware constant/register check. Pass path: beq.b an address.
- `toggle_ciab_bit` (loader.cod (raw code file, load address unknown)): Writes a byte from d1 to a hardware register, modifies d1 based on a value from a stack offset, and writes the updated d1 back to the same hardware register. Kind: hardware status/disk presence check. Pass path: the branch at an address (likely the instruction following the bra.w in the caller).
- `modify_ciab_register` (loader.cod (raw code file, load address unknown)): Writes the value in d1 to a hardware register, clears bit 7 of d1, calls another routine, and branches to another location. Kind: disk hardware/register check. Pass path: the branch at an address (the instruction following the 'beq.b' which skips the 'bsr.w an address' call).
- `update_ciab_bits` (loader.cod (raw code file, load address unknown)): Calls another routine, clears bits 0 and 1 of d0, writes d0 to a hardware register, sets bit 0 of d0, writes it to the hardware register again, and branches to another location. Kind: disk hardware/CIAB_PRB check. Pass path: sub_0FF6 (loop continues via dbra) or sub_0FA8 (loop continues via bne).
- `write_byte_to_address` (loader.cod (raw code file, load address unknown)): Calls another routine and then stores the resulting byte in d0 into a longword at a specific hardware address. Kind: checksum or integrity trap. Pass path: the branch following the call that continues normal execution (likely the instruction immediately following the call in the caller).
- `update_ciab_via_table` (loader.cod (raw code file, load address unknown)): Reads a byte from a hardware register, performs arithmetic and bitwise operations on it, uses the result to index into a table, and conditionally clears a bit in d2 before returning. Kind: disk hardware/CIAB_PRB check. Pass path: the branch at an address (bmi.b an address) which skips the bclr.b instruction.

## Functions


### Alfred Chicken.adf:TRAINER

- `decompress_data_block`: Identifies a data structure by a magic constant, then performs a complex decompression of a block of data using a series of bit-manipulation and byte-copying loops.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### Alfred Chicken.adf:alfred

- `initialize_system_and_memory`: Calls two routines to allocate memory and open a library, stores the resulting pointers in global variables, and then performs a series of comparisons on values retrieved from a structure to determine a memory size limit before calling another routine and triggering a software interrupt.
- `set_interrupt_and_dma_status`: Sets the stack pointer to a fixed address, disables interrupts by writing to the status register, and writes specific values to three hardware registers.
- `copy_data_and_jump`: Copies three values from a table to specific memory locations, performs a loop to copy 5605 bytes from one location to another, and then jumps to an address stored at a memory location.
- `execute_dos_calls`: Calls a routine via a pointer in a table, and if the result is non-zero, calls two more routines via offsets in that same table before incrementing a hardware register.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_5`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Alfred Chicken.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset`: Performs a relative branch to an offset of 40 decimal.
- `initialize_loop_counter`: Sets d0 to 5 and enters a loop that writes a byte to an address in a0 and decrements d0, repeating until d0 is zero, then performs an indirect jump.
- `update_jump_table`: Decrements a byte in a global variable and, if it is not zero, writes a byte to an offset in a table; if a global variable is one, it writes several specific values to a table and jumps to a location in that table.
- `dispatch_command_byte`: Saves d0 and a0 to a global location, reads a byte from a memory address, and performs a series of comparisons to write specific words or bytes to various memory offsets based on that byte before performing an indirect jump.
- `initialize_system_state`: Performs extensive initialization including setting hardware registers, calling multiple routines to update tables, and executing loops to populate memory with calculated values.
- `update_table_and_call`: Increments a global variable, checks a condition, and if met, copies four words from a table to specific memory locations, calls another routine, and calls a routine via a pointer.
- `copy_glyph_to_two_rows`: Searches a table of up to 73 bytes for the current byte, then uses the match position to compute a record address.
- `calculate_and_program_blitter`: Swaps two pointers, programs blitter registers, and enters a loop that calculates new values using multiplication and bit shifts, stores them in a table, and calls two other routines.
- `reset_blitter_masks_and_modulo`: Waits until the blitter is idle, then writes full first and last mask words, a control word and two modulo values for the blitter.
- `fill_table_with_increments`: Fills a table with values starting at zero and incrementing by 40 for each entry through a loop of 255 iterations.
- `draw_line_with_blitter`: Orders two endpoints, computes the step counts and octant of a line, toggles one pixel bit at the start position, and programs the blitter with the modulo, control and size values to draw the line.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_and_input_state`: Sets up audio volume registers and initializes a table of pointers and values.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `update_audio_status`: Increments a byte at a global address, compares a byte from a table to another value, and then either calls another routine and branches or calls another routine and branches.
- `update_audio_channels`: Updates the four Amiga audio channel registers by calling a routine to process command data for each channel, then calculates a scaled value from a table and writes it to a global variable before calling the same routine again to refresh the channel states.
- `process_audio_command_data`: Processes a command structure by extracting a nibble-based index to look up sound parameters in a table, updates structure offsets for playback, and manipulates Amiga audio registers and DMA control.
- `process_scaled_value_command`: Reads a command byte from a structure in a6 and branches to specific handlers that either perform table lookups to scale a 16-bit value, subtract a masked byte from the value with a floor limit, or add a masked byte to the value with a ceiling limit.
- `scale_structure_value`: Calculates a table offset by multiplying a value from a structure in a6 by 74 decimal, enters a loop to compare values from the table, and updates a field in the structure in a6 based on the comparison results.
- `update_scaled_value`: Updates a value in a structure by adding or subtracting a byte, then performs a table lookup using multiplication and bitwise operations to store a result in a memory location.
- `update_scaled_coordinate`: Calculates a scaled value by looking up a byte in a table or performing bitwise shifts and multiplications based on flags in a structure, then adds this result to a value at offset 16 of the structure and writes it to offset 6 of another structure.
- `dispatch_input_command_data`: Reads a command byte from a structure and branches to specific handlers based on its lower and upper nibbles.
- `extract_and_store_nibble`: Extracts the lower nibble from a byte at offset 3 of a structure and stores the result into offset 18 of the same structure before returning.
- `update_pointer_based_on_flag`: The routine checks a high bit in a structure field; if set, it clears the field, adjusts a pointer based on a table lookup and offset, and negates the byte at the new pointer location.

### Alfred Chicken.adf:loader.cod (raw code file, load address unknown)

- `execute_system_boot_sequence`: Sets a hardware register, calls several routines to move data between memory locations, and performs jumps to various addresses based on memory comparisons.
- `dispatch_load_routine`: The routine calls another routine to obtain a value and, depending on a flag in a register, selects different code paths that call other routines and assemble a 64-bit result from two 32-bit parts.
- `copy_data_blocks`: Performs a multi-level nested copy of data from a source table to a destination buffer.
- `decrement_and_call_routine`: Checks a counter and a flag in a structure; if the counter is zero and the flag is set, it calls another routine and returns its result, otherwise it decrements the counter and returns zero.
- `process_complex_data_structure`: Calls several routines to perform conditional logic, updates various offsets in a structure in a6, and enters a loop that copies bytes from a source to a destination while decrementing a counter.
- `store_word_at_offset`: Calls another routine, and if the zero flag is not set, stores a word from d0 into a longword at an offset determined by d3 in a structure pointed to by a6.
- `update_bit_flags`: Calls several routines, enters a loop to process a list of values starting at an offset in a1, and calls another routine to check a value from a structure in a6.
- `set_bit_in_table`: The routine computes an index from the high and low parts of an input value, selects a 32-bit word from a table pointed to by A6, and sets a single bit within that word corresponding to the low bits of the index.
- `compare_and_call_routine`: Compares a value in d6 to 1792, and if they are equal, it calls another routine; otherwise, it calls another routine and then updates d6 with the value in d0 before returning.
- `test_and_compare_d6`: Calls a routine once, then enters a loop that repeatedly calls another routine, checking if a short register value equals a specific constant until the condition is met.
- `test_table_bit`: Calculates an index based on the value in d0, loads a longword from a table at a6 offset into d1, and tests a bit in d1 using the high word of the calculated index.
- `clear_table_bit`: The routine clears a single bit in a 32-bit word table indexed from a 32-bit value in D0.
- `execute_conditional_calls`: Loads a word from a constant offset, calls another routine, and if the zero flag is not set, calls a second routine before returning.
- `copy_bytes_with_counter`: Copies bytes from a source pointed to by a4 to a destination pointed to by a0 plus an offset of 465, decrementing a counter in d0 until it reaches 30 or the byte is zero.
- `load_table_entries`: Iterates over a table of pointers and copies a fixed-size block from a global table into a buffer via a pointer.
- `clear_memory_block`: Clears a block of memory starting at an offset of 13312 from a6 for 1279 total bytes.
- `initialize_data_buffers`: The routine initializes a memory block by writing specific constant values to offsets, loops until a call returns a specific status code, and stores results from subsequent calls into memory locations.
- `process_data_table`: Repeatedly calls another routine and checks a pointer for null, returning the high word of the call result or fixed return codes.
- `validate_and_store_byte`: It reads characters from another routine to detect a drive letter pattern, specifically checking for 'D' and 'F' followed by a digit and a colon.
- `tiny_move`: Small routine of 1 instruction that copies (a0)+ to d0.
- `mask_lowercase_character`: Checks if d0 is between 97 and 122 decimal, and if so, clears the fifth bit of d0.
- `process_string_data`: It scans a null-terminated string from a base pointer, advancing until it reaches a null terminator or a slash character.
- `update_table_entry`: Calls another routine and stores its result, then tests a flag and two fields of a pointer passed in a register.
- `call_conditional_routine`: Calls another routine and, if the zero flag is set, branches to another location.
- `set_constant_on_call`: Calls another routine and, if the zero flag is set, loads the constant 309 into d0 before returning.
- `sum_and_negate_list`: Iterates 127 times, adding values from a list pointed to by a0 into d0, and then negates the resulting sum in d0.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_logical_or`: Performs a logical OR operation on d0 with zero.
- `clears_data_register_zero`: Performs a logical OR operation on data register zero with zero.
- `perform_multiple_logical_ors`: Performs eight consecutive logical OR operations on d0 with zero.
- `store_result_in_offset`: Clears a longword at offset 20 of the structure in a0, calls another routine, and then stores the resulting value in d0 into the same offset 20 of the structure in a0.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d2.
- `call_buffer_offset_calculator`: Pushes d3 onto the stack, calls another routine, and returns with the result in d0 after restoring d3.
- `load_constant_d2`: Loads the constant 1 into register d2.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 0 into d3.
- `calculate_buffer_offsets`: Saves several registers into a structure in a6, performs arithmetic involving division by eleven and multiplication by eleven on values derived from the stack and registers, and calls several other routines to update or verify memory offsets and pointers within the structure before returning.
- `initialize_hardware_buffers`: Clears several words in a structure in a6, calls several routines to initialize memory buffers with specific patterns, and enters a loop that performs hardware checks and buffer updates.
- `configure_hardware_and_loop`: The routine writes a constant to memory derived from a register and conditionally calls another routine.
- `process_data_block`: The routine writes a constant to a memory location derived from a pointer, then repeatedly calls other routines to update a flag, toggle a flag, and compute a checksum using a decrementing counter.
- `process_conditional_data`: Repeatedly calls helper functions to check a flag and decrement a counter, returning early if the flag is clear or the counter expires.
- `update_system_state`: Calculates a memory offset using a value from a stack offset in a6, enters a loop that calls several routines while decrementing a value at a stack offset, and returns either zero or negative one.
- `calculate_scaled_value`: Subtracts a 16-bit value from a stack offset in a6 from d1, multiplies the result by 512 using d0, and returns.
- `set_stack_bit`: Sets bit 3 of a 16-bit value retrieved from a stack offset in a6, stores the result back to that stack offset, increments a value at a stack offset by 1, and returns.
- `fill_memory_with_zero`: Sets a 10-iteration loop that writes 0 to a memory location at offset 1088 from the address in a5.
- `extract_masked_word_components`: It calls another routine to obtain a 32-bit value and reassembles it so the low 16 bits come from the result and the high 8 bits come from the original high byte.
- `combine_masked_words`: Reads two 32-bit words from a pointer in a0, masks both with a constant, adds the first masked result to itself, and performs a bitwise OR with the second masked result before returning.
- `load_and_compare_data`: Calls another routine, pushes the result in d0 onto the stack, loads an address from offset 48 of a5 into a0, calls another routine, and then compares the result in d0 with the value popped from the stack.
- `load_address_and_constant`: Loads the address at offset 8 of a5 into a0 and loads the constant 40 into d1.
- `process_masked_data_loop`: Pushes d2 onto the stack, calculates a loop count based on d1, and enters a loop that performs a bitwise XOR between a value from a0 and d0, then masks d0 with a constant after the loop finishes and returns.
- `fill_memory_with_pattern`: The routine clears a block of memory by writing a repeated 32-bit pattern to each word.
- `loop_masked_bit_manipulation`: It interleaves the low and high bits of two 32-bit word streams pointed to by two input pointers.
- `loop_transform_data`: Receives two arguments and loops a number of times derived from the first argument.
- `call_routine_twice`: Pushes d0 onto the stack, shifts d0 right by one bit, calls another routine, restores d0 from the stack, and calls another routine again.
- `modify_byte_bit_7`: Reads a byte from a pointer in a0 and checks bit 0 of the byte at offset -1 from that pointer; if bit 0 is not set, it checks bit 6 of the byte in d0 and either sets or clears bit 7 of d0 before writing the byte back to the address in a0 and returning.
- `transform_and_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `configure_dma_registers`: The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable.
- `check_hardware_status`: Polls a byte in a structure for a specific bit flag.
- `write_hardware_constant`: Writes the constant 1024 to a hardware register, tests a word at an offset from a6, and if the result is not negative, loads the constant 255 into d1.
- `toggle_ciab_bit`: Writes a byte from d1 to a hardware register, modifies d1 based on a value from a stack offset, and writes the updated d1 back to the same hardware register.
- `modify_ciab_register`: Writes the value in d1 to a hardware register, clears bit 7 of d1, calls another routine, and branches to another location.
- `modify_table_value`: Reads a word from a global table using an index, conditionally calls another routine if the value is negative, computes the absolute difference against a register, and writes the result back to the table.
- `clear_hardware_bit`: Checks bit 4 of a hardware register; if set, it enters a loop that runs 85 times calling another routine, otherwise it calculates an offset from a value at offset 36 of a6 and clears a byte at that offset in a table.
- `update_ciab_bits`: Calls another routine, clears bits 0 and 1 of d0, writes d0 to a hardware register, sets bit 0 of d0, writes it to the hardware register again, and branches to another location.
- `write_byte_to_address`: Calls another routine and then stores the resulting byte in d0 into a longword at a specific hardware address.
- `update_ciab_via_table`: It builds a mask from a global variable and a byte taken from a structure, clears one bit in that mask based on a byte from the structure, and optionally clears another bit if a flag in a table indexed by the structure is set.
- `wait_for_bit`: Calls another routine, checks bit 0 of a hardware register, and loops by decrementing d0 until it is zero.
- `check_hardware_bit`: Checks bit 0 of a hardware register and branches to another routine or decrements d0 based on the result.
- `write_hardware_constants`: Writes three specific byte values to three different hardware registers and returns.
- `generate_complex_table`: Obtains a count from another routine and then repeatedly copies blocks of bytes from a source pointer to a destination pointer using counts returned by other routines.
- `process_data_list`: Scans a table of words for a matching condition, then calculates a bit mask based on byte values and register contents.
- `perform_arithmetic_and_shift`: Performs a bitwise AND and a subtraction, then calls another routine and performs a logical shift left if the subtraction result is not positive.
- `process_byte_with_table`: Adds d1 back into d7, shifts d6 right by d7, swaps d6, advances a3 by four and loads two bytes backwards into d6 with a byte swap, then resets d7 to sixteen minus d1.
- `load_bytes_in_loop`: Loads a byte from the address in a0 into d0, shifts d0 left by 8 bits, and repeats this process up to three times using a dbra loop.
- `generate_data_table`: Iterates over a buffer to match bytes against a running value, writing masks, shifted values, and node properties into a destination array to build a tree structure.

### Alfred Chicken.adf:AT80000.COD#unpacked (raw code file, load address unknown)

- `initialize_game_state_and_assets_2`: Clears numerous global variables and status flags, then performs a series of memory copies and data transformations to load asset tables and configuration parameters.
- `manage_sprite_animation_states`: Checks two global status flags and, if a specific byte value is found at a calculated address, initializes or decrements animation counters and pointers.
- `decrement_and_branch_table_pointer_3`: Decrements a 16-bit value in a global variable and updates a pointer based on its value.
- `process_and_copy_asset_data`: Clears a memory block and then performs conditional data transformations and copies based on the value of a global variable.
- `transform_and_copy_data_3`: Iterates through a source buffer and writes transformed bytes to a destination buffer using a lookup table.
- `copy_scaled_memory_block_2`: Copies a block of longwords from a source pointer to a destination pointer, where the number of iterations is determined by multiplying two bytes from a parameter list and dividing the result by four, then subtracting one.
- `copy_transformed_data`: Calculates a count based on values at specific offsets and then copies a block of memory from a source to a destination.
- `initialize_and_process_data_buffer_3`: Clears a memory buffer using two loops, then iterates through the buffer to perform conditional data transformations and copies.
- `process_and_copy_data`: Performs bitwise operations on a value in d3, then copies bytes from a source buffer to a destination buffer, occasionally calling another routine or performing complex data transformations involving a global variable and a table.
- `search_and_clear_memory`: Iterates through a table of 59 words, and if a word matches a specific value, it clears 128 words starting at a specific offset.
- `process_data_block_3`: Sets two global pointers, checks a byte from a global variable, and if it equals 4, performs a series of data transformations including copying bytes from a source to a destination, calling another routine, and executing multiple conditional branches that either copy blocks of data using a loop or call another routine multiple times with different pointers before jumping to a function pointer.
- `expand_memory_block_3`: Copies a single word from a source pointer to multiple consecutive locations in a destination buffer, repeating this process for 63 iterations.
- `tiny_moveq_move`: Small routine of 2 instructions that loads 0 into d2, then copies d3 to d2.
- `calculate_product_sum_3`: Calculates a value by multiplying a byte from a global variable with a byte from a register and adding it to the lower byte of another register, then adds this to 2048.
- `copy_scaled_data`: Reads two bytes from a buffer, stores the first in a global variable, and if the first byte's lowest bit is clear, copies a block of bytes from a source pointer to a destination pointer with a count derived from a multiplication of the second byte and a global value.
- `copy_and_process_data_2`: Uses a byte from a register to index into a table, calls another routine with a pointer from that table, and then copies 120 bytes from the resulting pointer to a global buffer.
- `process_table_values_3`: Iterates through a table of thirty-two words, performing bitwise manipulations on specific nibbles and updating each entry in place, then sets a global variable to 39.
- `clear_memory_block_3`: Repeatedly writes zero to specific offsets within a series of sixteen-byte blocks in memory, then copies a byte from a source to a global variable and sets another global variable to zero.
- `update_string_table_3`: Compares a string in a table with another and, if different, updates the table and copies 128 words of data from a source address to a destination address pointed to by a table entry, then calls itself recursively.
- `copy_memory_block_4`: Copies a block of bytes from a source address in a0 to a destination address in a1, decrementing a count in d2 until it reaches zero.
- `toggle_and_update_table_entry`: Checks a byte in a global variable to either clear it or set it to one based on a comparison with nineteen, then performs a 32-bit subtraction using two values from a table and a global variable to update an entry in a table at offset four of a structure.
- `calculate_and_copy_values`: Performs division and bit manipulation to calculate values, then conditionally copies bytes from a source to a destination while performing bitwise shifts and additions on the data.
- `update_attribute_flags_2`: Toggles a bit in a global variable and uses the value to index into a table, copying four bytes from that table to specific locations in memory.
- `update_attribute_flags`: Iterates through a nested structure of data pointed to by a global variable, performing a bitwise OR on words that match a specific value in a register.
- `increment_cyclic_value_3`: Increments a global variable within a range of zero to four, wrapping it back to zero when it reaches five, after calling another routine.
- `update_calculated_values`: Writes a byte to a destination, calls another routine, and then uses a value from a table to calculate and store three bytes into global variables.
- `handle_conditional_logic`: Checks a byte from a pointer against specific values, conditionally calls other routines, modifies global variables, and writes bytes to a memory location based on a calculated offset.
- `calculate_offset_address_3`: Performs bitwise shifts and arithmetic on registers and a global variable to calculate and return an address starting from 2048.
- `update_bcd_value`: Checks a global variable and, if it meets specific conditions, increments it by ten and performs BCD adjustments using values from a table.
- `decrement_bcd_global_variable`: Disables interrupts, subtracts a byte from a global variable using binary-coded decimal arithmetic, and returns.
- `increment_bcd_counters`: Disables interrupts, adds a word value to three consecutive bytes in a global variable, performs BCD adjustment on each byte using values from a table, and returns.
- `update_vhpors_with_offset`: Updates a global variable by adding its own value, a constant offset of 39, and the current value of the video hold pointer register, then returns.
- `iterate_null_terminated_list_2`: Reads bytes from a table until a terminator is encountered, calling another routine for each non-terminating byte and incrementing a counter.
- `copy_structured_data_blocks_2`: Calculates a destination address using multiple inputs and performs a loop to copy non-contiguous bytes from a source table into a destination buffer.
- `copy_data_indirect_3`: Calculates a destination address using a base pointer, a lookup table, and two offsets derived from a register, then copies sixteen 4-byte blocks from a source table to specific byte offsets within that destination structure via a loop.
- `update_calculated_memory_3`: Calculates a memory address by multiplying a value derived from an input register and a global variable, then stores a bit-manipulated byte from a register at that location with the lower four bits set to one.
- `store_masked_byte_3`: Calculates a destination address by multiplying a value derived from a global variable and an input by a factor from another global variable, then stores a byte from a pointer with its lower four bits set to one and shifted left by four into that address.
- `update_state_from_indexed_table`: Repeatedly reads an index from a global variable to fetch a byte from a table, then uses that byte to update a destination address derived from another global variable.
- `copy_and_update_data_structure`: Copies three bytes from a source location to a destination location, then checks a byte at the destination to determine if it should clear a 16-bit value and call another routine; if the byte is non-zero, it performs a series of sixteen byte-by-byte copies from the source to the destination using decremented pointers.
- `transfer_and_call_routine`: Moves two 16-bit values from global memory to offsets of 1 and 3 from a pointer in a6, then calls a function pointer from a table.
- `modify_and_store_global`: Performs arithmetic on a value retrieved from a register and stores the result into a global variable.
- `call_indirect_table_2`: Checks if a global variable is non-zero and then calls a function pointer from a table using that variable as an index.
- `call_indirect_via_table_3`: Calculates an offset into a table using a global variable and executes a function pointer found at that location.
- `validate_and_store_coordinate_offsets`: Checks two global bytes for non-zero values, calls another routine with a pointer and a constant, and then performs bounds checking on two calculated coordinate offsets before storing the results into global variables.
- `dispatch_indexed_function_pointer`: Reads ten bytes from a pointer in a0 into a series of global variables, calculates an offset by adding a byte from a register to a base address, and performs an indirect jump to a function pointer retrieved from a table.
- `calculate_dynamic_offset`: Calculates a value based on bytes from two global memory locations and a global variable, then updates the global variable using the result.
- `dispatch_event_by_byte`: Compares a byte from a pointer to two specific values, calling different routines if they match, and then writes several bytes to global variables.
- `compare_and_branch`: Copies a byte from a pointer in a0 to d3, compares it to 200, and if it is less than 200, compares it to 205, otherwise it calls another routine and branches.
- `call_indexed_function`: Increments a byte in a global variable, uses the new value to index into a table, and calls the resulting function pointer.

### Alfred Chicken.adf:FRONTEND.COD#unpacked (raw code file, load address unknown)

- `initialize_system_state_2`: Sets hardware registers, clears several global 16-bit variables, populates three memory blocks using loops, calls several routines, and conditionally executes a routine via a pointer based on a global variable.
- `execute_subroutines_sequentially`: Calls a routine via a pointer, sets a global 16-bit value to 1, and if it is non-zero, calls another routine; it also contains a loop that decrements a global 16-bit value and calls another routine until it reaches zero, eventually calling another routine and branching.
- `assign_memory_addresses`: Assigns a sequence of memory addresses to a series of consecutive global variables.
- `assign_memory_addresses_alt`: Assigns a sequence of memory addresses to a different set of consecutive global variables.
- `process_state_structure`: Decrements values within a structure pointed to by a0, manages state transitions for two different counters, and updates two 16-bit values at offsets 2 and 4 of a structure pointed to by a5 based on the results.
- `recursively_check_global`: Checks if a 16-bit global variable is non-zero and, if so, sets another global variable to one; otherwise, it calls itself recursively.
- `fill_table_with_32bit_increments`: Writes a series of five 32-bit values into a table in increments of eight bytes, where each value is incremented by forty in decimal.
- `populate_pointer_table`: Iterates through a source array to calculate memory addresses by indexing into a base table and adding an offset.
- `decrement_and_update_state_structure`: Decrements a 16-bit global variable and, if it reaches zero, resets it to 8 and performs a bitwise inversion on a different global variable.
- `populate_sprite_attribute_list`: Iterates through a data table to populate a list of sprite attributes.
- `populate_data_table`: Calculates an index from a global variable and uses it to fetch values from a table, storing the results in a series of word pairs at a specific memory location.
- `update_sequence_state_and_counters`: Manages a state machine by decrementing a counter and updating a pointer and a value from a data table when the counter reaches zero.
- `update_state_variable`: Performs conditional checks on a global variable and updates it or other memory locations based on specific constant values and offsets.
- `check_recursion_condition`: Checks if a global variable equals three and if a value at a specific memory offset is negative, then calls another routine if both conditions are met.
- `update_state_counters`: Decrements and resets several global counters and values based on conditional checks of a global variable.
- `decrement_and_check_counters`: Decrements a global counter and, upon reaching zero, resets it to a specific value and modifies several other memory locations.
- `adjust_global_pointer`: Increments or decrements a global variable and occasionally adds values retrieved from a table based on the contents of a memory offset.
- `update_sprite_attribute_table`: Calculates and writes sprite position, attribute, and flag data into a memory table based on values from a structure.
- `clear_memory_block_4`: Fills a block of memory with zeros using a loop.
- `verify_and_reallocate_buffer`: Checks if a memory block starts with a specific four-byte signature, then performs a series of memory shifts and reallocations by copying bytes between buffers based on length values read from the data structure.
- `concatenate_bytes`: Concatenates three consecutive bytes from a memory pointer into a single 32-bit value.
- `check_bit_status`: Checks if the lower seven bits of a byte are non-zero.
- `decrement_and_process_list`: Performs a loop that decrements d0 and updates d1 using a byte from a pointer in a5, and if d1 is not negative, decrements it further and conditionally calls another routine.
- `call_subroutine_and_test_d1`: Loads negative one into d0, calls another routine, decrements d1, and returns with d0 set to zero if d1 is negative; otherwise it returns.
- `traverse_and_process_linked_list`: Traverses a list of structures by reading a length field from a table at a specific offset, then calls another routine to process each entry.
- `update_global_counter`: Checks a 16-bit global variable and, if it is zero, decrements a 32-bit value at a global address, calls another routine if the result is negative, and updates the global variable based on its new value.
- `process_input_bits`: Reads joystick and mouse data from hardware registers and performs bitwise manipulations to update a global variable.
- `interpolate_table_values`: Iterates through two tables and adjusts the nibbles of words in the destination table to move them closer to the values in the source table.
- `zero_memory_array`: Sets a sequence of words to zero at a specific memory location using a loop.
- `clear_memory_block_5`: Fills a block of memory with zeros using a loop that runs for a fixed number of iterations.
- `update_global_value`: Performs arithmetic operations on a global variable and stores the resulting byte back into it.

### Alfred Chicken.adf:ATC00000.COD#unpacked (raw code file, load address unknown)

- `initialize_level_assets_and_state`: Clears multiple global status bytes and counters, copies several blocks of data from memory to various destinations using loops, and updates a video hold pointer register.
- `manage_state_transitions`: Checks global status flags and pointer values to conditionally update counters and reset state variables.
- `decrement_and_branch_table_pointer`: Decrements a 16-bit value in a global variable and updates a pointer based on its value.
- `dispatch_transformed_data_blocks`: Clears a memory block of 28 bytes and then performs a series of conditional data transformations and copies based on the value of a global variable.
- `transform_and_copy_data`: Iterates through a source buffer and writes transformed bytes to a destination buffer using a lookup table.
- `copy_memory_block`: Copies a block of memory from a source pointer to a destination pointer for a specified number of iterations.
- `copy_transformed_data_2`: Calculates a count by multiplying two bytes from a source buffer and dividing by four, then copies a block of longwords from a source to a destination using a loop.
- `initialize_and_process_data_buffer`: Clears a memory buffer with zeros using two loops, then iterates through a data stream to perform conditional byte transformations, pointer updates, and calls to process_and_copy_data and calculate_and_copy_values.
- `transform_and_copy_buffer`: Performs bitwise operations on d3, and if the result is non-zero, iterates through a buffer to copy and transform bytes, including a loop that clears a memory area and several conditional blocks that perform complex data movement and call another routine.
- `clear_memory_blocks`: Iterates through a table of pointers, and if a byte at a specific offset in the table matches a constant, it fills a 128-byte block starting at a destination address with zeros.
- `process_data_and_copy_blocks`: Updates several global pointers, checks a byte from a table against a global variable, and calls another routine.
- `expand_memory_block`: Copies a single word from a source pointer to multiple consecutive locations in a destination buffer for sixty-three iterations.
- `tiny_moveq_move_2`: Small routine of 2 instructions that loads 0 into d2, then copies d3 to d2.
- `calculate_product_sum`: Calculates a value by multiplying a byte from a global variable with a byte from a register and adding it to the lower byte of another register.
- `copy_data_block_2`: Copies a block of bytes from a source address to a destination address using a count derived from a calculation involving a global variable.
- `copy_structure_data`: Calls another routine with a pointer, then copies 120 bytes from the resulting pointer to a global destination.
- `process_table_values`: Iterates through a table of sixteen words, performing bitwise manipulations on specific nibbles and updating each entry in place.
- `clear_memory_block_2`: Repeatedly writes zero to specific offsets within a series of sixteen-byte blocks in memory.
- `update_string_table`: Compares a string in a table with another and, if different, updates the table and copies data from a source address.
- `copy_memory_block_2`: Copies a block of bytes from a source address to a destination address using a count in a register.
- `reset_global_variable`: Sets a global variable to zero unless it matches a specific value, in which case it enters an infinite loop.
- `calculate_and_copy_data`: Performs division and arithmetic to calculate values, then conditionally copies data from a table to a destination.
- `toggle_global_bit`: Performs a bitwise XOR on a global variable with one.
- `update_table_entries`: Iterates through a nested loop structure to check if the upper bits of words in a table match a value, and if so, sets the lower four bits of those words.
- `increment_cyclic_value`: Increments a global variable within a range of zero to four, wrapping it back to zero when it reaches five.
- `update_calculated_memory_3_2`: Calls a routine, then checks a byte at an offset from a pointer; if non-zero, it calls another routine and stores three bytes into a table at specific offsets.
- `process_conditional_data_2`: Checks a byte from a pointer against specific values, performs BCD arithmetic on a global variable, and calls several routines including an indirect function call via a table.
- `calculate_offset_address`: Performs bitwise shifts and arithmetic on registers and a global variable to calculate and return a memory address.
- `update_bcd_values`: Checks if a global variable is zero and not equal to 1433, then increments it by 10 and performs several BCD arithmetic operations using a table and a global variable, calling another routine via a function pointer if certain conditions are met.
- `decrement_bcd_variable`: Subtracts a value from a global variable and performs a binary-coded decimal adjustment.
- `increment_bcd_values`: Updates three global variables by adding values from a word and a constant, followed by BCD adjustment.
- `update_global_offset`: Updates a global variable by adding its own value to itself and adding a constant offset.
- `iterate_null_terminated_list`: Traverses a list of pointers in memory until a null terminator is encountered, calling another routine for each element.
- `copy_structured_data_blocks`: Calculates a destination address using two input values and a base pointer, then performs a loop to copy sixteen non-contiguous bytes from a source table into the destination buffer at specific offsets.
- `copy_data_indirect`: Copies sixteen 4-byte blocks from a source table to a destination address derived from a global variable and a lookup table.
- `update_calculated_memory`: Calculates a memory address using a global variable and an input value, then stores a bit-manipulated byte at that location.
- `store_masked_byte`: Calculates a destination address using a global variable and an input value, then stores a byte from a pointer with specific bits set.
- `process_input_event_sequence`: Reads a byte from a global variable to index into a table, then uses the retrieved value to determine a destination address and a source byte.
- `copy_and_process_data`: Copies three bytes from one location to another, then conditionally performs several operations including calling a routine, updating a table, and potentially executing a jump to a pointer in a table.
- `copy_structured_data_2`: Copies two 16-bit words from a source to offsets of one and three from a destination pointer, then calls another routine.
- `modify_and_write_data`: Performs arithmetic on a value retrieved from a register and stores the result into a global variable.
- `call_indirect_table`: Checks if a global variable is non-zero and then calls a function pointer from a table using that variable as an index.
- `call_indirect_via_table`: Calculates an offset into a table using a global variable and executes a function pointer found at that location, then performs several data transformations and calls a routine.
- `update_status_flags`: Checks a byte from a global variable and, if non-zero, calls another routine and performs arithmetic on values from several global variables to update two bytes at offsets 42 and 43 of a structure.
- `dispatch_event_table`: Copies several bytes from a pointer in a0 to global variables, uses a byte from a0 to index into a table and jumps to the resulting address, and then copies bytes from the table back to global variables.
- `calculate_and_update_offset`: Reads bytes from two memory locations using an offset from a global variable, performs arithmetic involving two's complement logic and bit shifts, and updates the global variable based on the results.
- `handle_attribute_events`: Compares a byte from a pointer to two constants; if a match is found, it calls a routine and then writes four bytes to specific global memory locations.
- `dispatch_event_byte`: Compares a byte from a pointer to two specific values and calls another routine if the byte matches one of them; otherwise it branches to another location.
- `call_indirect_pointer`: Increments a byte in a global variable and calls a function pointer from a table using the incremented value as an index.

### Alfred Chicken.adf:AT200000.COD#unpacked (raw code file, load address unknown)

- `initialize_game_state_and_assets`: Clears numerous global variables and status bytes, performs multiple memory block copies and clears using loops, and updates several tables based on the value of a global variable.
- `manage_state_transitions_2`: Checks two global status flags and, if they are zero, calculates a new address using arithmetic on values from a table and two bytes from memory, then calls another routine to update state with a pointer and a counter.
- `decrement_and_branch_table_pointer_2`: Decrements a global byte and updates a pointer based on its value.
- `process_sprite_data_and_update_buffers`: Clears a block of memory, then checks a global variable to determine a sequence of calls to transform and copy data routines using various source and destination buffers.
- `transform_and_copy_data_2`: Iterates through a source buffer and writes transformed bytes to a destination buffer using a lookup table.
- `copy_scaled_memory_block`: Copies a block of longwords from a source pointer to a destination pointer, where the number of iterations is determined by multiplying two bytes from a data structure and dividing the result by four, then subtracting one.
- `copy_scaled_longword_block`: Calculates a loop count by multiplying two bytes found at offsets 1 and 5 of a source buffer, dividing the result by 4, and subtracting 1, then copies longwords from a source buffer to a destination buffer for that many iterations.
- `initialize_and_process_data_buffer_2`: Clears a memory area with zero bytes using two loops, then iterates through a buffer to perform conditional byte transformations and structure updates.
- `decompress_and_transform_data_buffer`: Checks a bitmask in a register and, if valid, performs a complex decompression and transformation of a source buffer into a destination buffer using bitwise shifts, subtractions from a global value, and recursive calls to itself.
- `clear_memory_with_condition`: Iterates through a series of memory locations, checking if a byte at a specific offset matches a constant, and if so, fills a block of memory with zeros.
- `process_data_block_2`: Updates several global pointers and bytes, then conditionally copies data from a table to a buffer using a loop, or performs a series of calls to another routine with specific table addresses.
- `expand_memory_block_2`: Copies a single byte from a source pointer to multiple consecutive locations in a destination buffer, repeating this process for a fixed number of iterations.
- `set_register_byte`: Sets the lower byte of register d2 to the value of the byte in register d3.
- `calculate_product_sum_2`: Calculates a value by multiplying a byte from a global variable with a byte from a register and adding it to the lower byte of another register.
- `copy_variable_length_block`: Reads a byte and a second byte to determine a count, then copies a block of bytes from a source to a destination using a loop.
- `copy_and_call_routine`: Calculates an address from a byte in d3, calls a routine using a pointer from that address, and then copies a block of 120 bytes from a source to a destination.
- `process_table_values_2`: Iterates through a table of thirty-one words, extracting and modifying specific nibbles by subtracting sixty-four from the second nibble and four-hundred from the third nibble, then writes the resulting word to a destination table and sets a global variable to thirty-nine.
- `clear_memory_blocks_and_update_globals`: Repeatedly writes zero to specific offsets within a series of sixteen-byte blocks in memory, setting the ninth byte of each block to one, then copies a byte from the address in a0 to a global variable and clears another global variable before returning.
- `update_string_table_2`: Compares a string in a table with another and, if different, updates the table and copies 128 words of data from a source address to a destination address pointed to by a table entry, then calls itself.
- `copy_memory_block_3`: Copies a block of bytes from the address in a0 to the address in a1, decrementing a count in d2 for each byte transferred until it reaches zero.
- `update_state_and_table_entry`: Checks a global variable and a flag; if the flag is zero and the variable is not 25, it clears the flag, otherwise if the variable is 25 it sets the flag.
- `calculate_scaled_value_2`: Performs arithmetic operations involving a division by a byte from a global variable and bit shifts on the result.
- `calculate_scaled_coordinate_offsets`: Subtracts a constant from a long value in a0, divides the result by a byte read from a global variable, and performs bit shifts and subtractions on the quotient and its high word to store the results in d1 and d2 before returning.
- `update_table_entry_2`: Decrements a byte in a global variable and, if non-zero, updates a table entry with a 16-bit word from a calculated address.
- `toggle_attribute_and_update_data`: Toggles a bit in a global variable and, depending on the resulting value, copies bytes from a table into a structure and performs a block memory copy loop.
- `update_table_bits`: Iterates through nested loops to check if the upper bits of words in a table match a value, and if so, sets the lower nibble of those words.
- `increment_cyclic_value_2`: Increments a global variable by one, wrapping it to zero if it reaches five, or decrements it by one, wrapping it to four if it is currently zero.
- `copy_and_update_data_structure_2`: Writes a byte to a location, calls another routine, and then updates several global variables based on a byte read from a calculated address.
- `increment_bcd_counters_2`: Checks a byte at a pointer, performs arithmetic, and if certain conditions are met, calls other routines and updates several global variables and memory locations.
- `calculate_offset_address_2`: Performs bit shifts and arithmetic to calculate an address based on a table and register values.
- `update_bcd_values_2`: Checks if a byte at a global variable is zero and if a word at another global variable is not 1401; if these conditions are met and a carry flag is set, it increments a byte at a third global variable and performs several BCD arithmetic operations using data from a table and a global variable.
- `decrement_bcd_variable_2`: Disables interrupts, subtracts a byte from a global variable using binary-coded decimal arithmetic, and re-enables interrupts before returning.
- `increment_bcd_values_2`: Disables interrupts and adds a word value to three consecutive bytes in a global variable, performing a BCD adjustment on each byte using values from a second global variable.
- `update_global_offset_2`: Updates a global variable by adding its own value to itself and adding a constant offset.
- `iterate_null_terminated_list_3`: Traverses a list of pointers in memory until a null terminator is encountered, calling copy_structured_data_blocks_2 for each element.
- `copy_data_block`: Copies data from a source table to a destination memory area using a loop that performs non-contiguous word transfers.
- `increment_index_and_update_state`: Reads a byte from a location pointed to by a global variable and, if non-zero, toggles a bit in another global variable and increments the pointer or updates a word at a third global variable based on the value read, occasionally calling another routine.
- `copy_data_indirect_2`: Calculates a destination address using a base pointer, a lookup table, and two offsets derived from the input register, then copies sixteen 4-byte blocks from a source table to that destination using a stride of 152 bytes between each block.
- `update_calculated_memory_2`: Calculates a memory address using a global variable and an input value, then stores a bit-manipulated byte at that location.
- `store_masked_byte_2`: Calculates an offset by dividing a value minus 800 by a byte from a global variable, multiplies the result by a doubled value from another global variable, and stores a byte from a pointer with its lower nibble set to 15 at the resulting address.
- `update_input_state_buffer`: Reads an index from a global variable to access a table, then retrieves a byte from that table and stores it back into the global variable.
- `increment_index_and_update_state_2`: Copies a block of 17 words from a source to a destination, potentially clearing a global variable if it is non-zero.
- `copy_structured_data`: Copies multi-byte data from a source to a destination table using offsets and performs BCD adjustments on a global variable.
- `call_indexed_function_pointer`: Subtracts 128 from a word loaded from a global variable, and if the result is not negative, writes 1 to one global variable, 120 to another, and 40 to a third before returning.
- `update_table_values`: Performs arithmetic on a byte from a global variable, updates a table at offset 1800 with new values, and conditionally calls itself recursively.
- `execute_indirect_call`: Calls a function pointer retrieved from a table using a global variable as an index.
- `call_indirect_via_table_2`: Calculates an offset into a table using a global variable and executes a function pointer found at that location, while also performing various memory copies and bit manipulations.
- `update_status_flags_and_mask`: Performs the same arithmetic and updates the same two bytes at offsets 42 and 43 of a structure as R1, but also performs a series of bitwise OR operations on a buffer pointed to by a0.
- `dispatch_event_data_and_jump`: Copies ten consecutive bytes from a pointer in a0 to a series of global variables, calculates an offset based on a byte from a register, and performs an indirect jump to a function pointer from a table.
- `calculate_dynamic_address`: Reads two bytes from global memory using an offset from a global variable, performs signed arithmetic and bit shifts to calculate a new value, and updates the global variable.
- `update_attribute_flags_2_2`: Compares a byte from a pointer to specific values, calls other routines if they match, and then writes several bytes to global variables.
- `dispatch_event_byte_2`: Compares a byte from a pointer to two constants, calls another routine if a match is found, otherwise performs a series of bitwise OR operations on the stack and jumps to an address retrieved from a table.
- `build_address_table_2`: Increments a global variable by four and then performs an indirect call using the original value as an offset into a table.
- `build_address_table`: Processes a memory buffer by extracting byte components and storing them into a destination table.
