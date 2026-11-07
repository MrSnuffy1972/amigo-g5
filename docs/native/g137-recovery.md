# Game 137 (5th Gear): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00137). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (917760 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 32 raw-word lines |
| Functions named and described | pass, 135 of 135 named (0 author symbols), 135 of 135 described (5 by rule, 127 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2349 of 2351 (99.91%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3903 of 3909 instructions identical in start and length, 4 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2259 |
| code | 28252 |
| data | 827345 |
| unused | 59904 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| 5th Gear.adf:bootblock | 1 | 16 | 1 |
| logged disk load from 5th Gear.adf boot-block load | 1 | 157 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2642 | 111 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 548 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 546 | 12 |

## Cases handled

- 5th Gear/5th Gear.adf: custom-format (non-DOS) disk, 1641 non-empty blocks: matched against track-loaded segments
- 5th Gear/5th Gear.adf: the boot block reads 2048 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1502 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 427 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 422 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 5th Gear/5th Gear.adf: 1 logged loads cover 2048 bytes of the image
- 5th Gear/5th Gear.adf: 31 track-loaded blocks found in recovered runtime segments and classified through them
- 5th Gear/5th Gear.adf: blocks 6-22 (bytes 3072-11776) outside the filesystem hold data, not code (opcode density 0.011, entropy 7.11)
- 5th Gear/5th Gear.adf: blocks 26-35 (bytes 13312-18432) outside the filesystem hold data, not code (opcode density 0.010, entropy 6.17)
- 5th Gear/5th Gear.adf: blocks 37-69 (bytes 18944-35840) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.05)
- 5th Gear/5th Gear.adf: blocks 135-160 (bytes 69120-82432) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.72)
- 5th Gear/5th Gear.adf: blocks 162-198 (bytes 82944-101888) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.80)
- 5th Gear/5th Gear.adf: blocks 312-404 (bytes 159744-207360) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.52)
- 5th Gear/5th Gear.adf: blocks 495-511 (bytes 253440-262144) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.94)
- 5th Gear/5th Gear.adf: blocks 539-1062 (bytes 275968-544256) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.95)
- 5th Gear/5th Gear.adf: blocks 1065-1083 (bytes 545280-555008) outside the filesystem hold data, not code (opcode density 0.009, entropy 7.15)
- 5th Gear/5th Gear.adf: blocks 1086-1103 (bytes 556032-565248) outside the filesystem hold data, not code (opcode density 0.008, entropy 7.13)
- 5th Gear/5th Gear.adf: blocks 1106-1124 (bytes 566272-576000) outside the filesystem hold data, not code (opcode density 0.010, entropy 7.16)
- 5th Gear/5th Gear.adf: blocks 1300-1700 (bytes 665600-870912) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.95)
- 5th Gear/5th Gear.adf: blocks 1718-1718 (bytes 879616-880128) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- 5th Gear/5th Gear.adf: blocks 1749-1749 (bytes 895488-896000) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.49)
- 13 packed track region(s), 199680 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `wait_for_flag_and_set_registers` (runtime segment an address): Busy-waits for a routine to return while checking a bit in a global variable, and updates several hardware registers and a global variable. Kind: track/sector check (length/validity check). Pass path: the loop continues if the condition (sVar2 != -1) or (bVar3 == false) holds, specifically checking if the disk length/parameter or a specific flag or certain register value allows the routine to proceed or exit via the break condition..
- `set_audio_dma_control` (runtime segment an address): The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable. Kind: disk/track check (sync/length/path validation). Pass path: bsr.w an address (sub_2F5A or similar) or bne.b (branch if non-zero/success).
- `set_global_flags` (runtime segment an address): It writes two constants to the same global variable in sequence, leaving the second value in place. Kind: disk hardware/length check. Pass path: rts.
- `set_bit_in_global_flag` (runtime segment an address): It reads a 16-bit value from a table, adds a constant, and sets a bit in a global variable using a mask derived from that value. Kind: checksum/bit-masking trap. Pass path: the branch where the bit-set/clear logic results in a valid state, likely avoiding a failed comparison or a jump to a sabotage routine.
- `set_bit_in_global_flag_2` (runtime segment an address): It reads a 16-bit value from a table pointed to by a register, adds a constant, and sets a bit in a global variable using a mask derived from the masked result. Kind: checksum trap. Pass path: the bitset flag or subsequent logic dependent on the global flag being set.
- `set_flag_and_call_twice` (runtime segment an address): Stores a constant byte into a global variable, then calls another routine twice in sequence. Kind: track/disk check. Pass path: u2__sub_3106.
- `read_port_into_status_byte` (runtime segment an address): Calls another routine to obtain a byte, clears a bit if a secondary value is non-negative, and then forces bit zero high before storing the result into a global variable. Kind: disk/track check. Pass path: the branch where the condition (extraout_D1b >= 0) is met, allowing the bitmasking and subsequent assignment.
- `update_ciab_prb_mask` (runtime segment an address): Updates a bit mask in a hardware register by calling a routine that derives a mask from a global variable and a table. Kind: disk hardware check (CIAB_PRB). Pass path: the branch or return path where the hardware-specific mask/check validates the disk/track.
- `compute_ciab_mask_from_table` (runtime segment an address): It derives a bit mask from a global variable by clearing a bit chosen by a table-relative index and potentially clearing a second bit based on a condition checked in a table. Kind: manual/code-wheel/colour-card lookup. Pass path: the branch where the bitmask check does not trigger a secondary bit clear (the default bVar4 calculation or the branch where the condition fails or results in no change to the mask bit).

## Functions


### 5th Gear.adf:bootblock

- `load_blocks_and_jump`: Boot block code that issues two disk read requests through exec DoIO, the second with a larger destination, then transfers control into the loaded code; the listed facts also name the bitplane control register as touched.

### logged disk load from 5th Gear.adf boot-block load

- `build_copper_list_from_table`: Opens the graphics library, walks a table of entries writing copper list register/value pairs into a buffer ended by an end marker, and stores that buffer's address as the system copper list; the listed facts also name the DMA control register as touched.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_string_and_memory`: Searches for a byte in a string, updates a global variable with a scaled offset, and iterates backwards through a memory range to add table values.
- `update_register_with_calculated_offset`: Calculates the difference between a global variable and a register value, then if the difference falls within a certain range, updates a global pointer and several memory-mapped registers using a formula based on the quotient and remainder of that difference divided by ten.
- `find_byte_in_string`: Iterates through a null-terminated string looking for a specific byte value, returning one if found or zero if a null terminator is encountered.
- `test_bit_and_extract_bits`: Checks if a specific bit is set in a table entry using a bit-shift-based index, then returns a value constructed from the upper bits of a register and a portion of that index.
- `call_subroutine_and_return`: executes another routine and then returns.
- `do_nothing`: The routine is empty and performs no operations.
- `update_global_with_scaled_offset`: Calculates a value using a combination of a pointer-based offset, a global variable, and a scaling factor, then stores the result back into a global variable.
- `add_table_values_to_memory`: Iterates backwards through a memory range, adding bytes from a source table to a destination, and applies BCD adjustment.
- `execute_recursive_calls`: Performs two recursive calls to another routine without any intervening instructions or state modification.
- `set_constant_values`: Executes a loop a fixed number of times and then writes a sequence of constant values to a global variable.
- `adjust_and_store_bcd`: Performs BCD-adjusted subtraction on several global variables and then writes a fixed value to a global variable.
- `modify_global_and_decrement_memory`: Sets a global variable to a constant, checks a memory value, decrements that memory value, and applies BCD adjustment.
- `set_global_value`: Performs several redundant assignments to a global variable before setting it to a fixed value and calling another routine.
- `increment_and_adjust_global`: Increments a global variable and a memory-mapped register, applies a BCD adjustment, and calls another routine.
- `loop_string_conversion`: Decrements a register in a loop and calls a routine that converts a byte to an ASCII string.
- `subtract_and_adjust_bcd`: Subtracts a byte from a source memory location using a second memory pointer and applies a binary-coded decimal adjustment to the result.
- `subtract_table_values_with_borrow`: Iterates backwards through a memory range, subtracting bytes from a source table from a destination, and applies BCD adjustment.
- `add_bcd_table_with_carry`: adds a source byte or a value from a table to a destination byte or a value in a table, performing a binary-coded decimal adjustment and propagating a carry bit.
- `iterate_and_convert_byte`: executes a loop that calls another routine twice while decrementing a counter and converts a byte from a pointer into an ASCII string.
- `increment_global_and_memory_with_bcd`: Clears a global variable and a memory location, then enters a loop that increments these values while applying BCD adjustment.
- `convert_byte_to_ascii_string`: Takes a byte from a pointer and stores its high and low nibbles as ASCII characters into a destination buffer.
- `update_string_and_registers`: Checks a global variable and a string, then writes a fixed value to a memory-mapped register and a certain value to a global variable or a string terminator.
- `increment_global_with_bounds`: Increments a global variable or resets it to zero or a certain value if it reaches a specific threshold or certain value.
- `increment_global_value`: Increments a global variable or resets it to a specific value if it reaches a certain threshold.
- `update_pointer_from_table`: Loads a value from a table using a global variable as an index and stores it into a memory location, potentially updating a global variable.
- `update_ui_string_elements`: Updates global string or numeric display values by calculating scaled offsets and performing BCD-adjusted string processing.
- `check_and_branch_on_byte`: compares a global variable to a constant value and calls another routine if the condition is met.
- `process_string_and_bcd`: Performs BCD adjustment on a global variable and calls other routines that perform string or pointer-based operations.
- `decrement_global_and_call`: If a certain byte in memory is non-zero, it calls another routine and decrements a global variable by a constant amount.
- `modify_global_pointer`: Checks a null terminator in a string and, if present, modifies a global variable and calls other routines.
- `copy_and_decrement_list`: Iteratively copies several longwords or words from a source pointer to a destination pointer and decrements a global variable until it reaches zero.
- `accumulate_and_update_table`: Iterates through a table, adding values from a source table to a destination table, while updating certain global or memory-based flags and a counter.
- `transform_and_update_global`: Calculates a new value for a global variable by applying a multiplier or bit-shift based on the current value, then updates the global variable with the result.
- `update_global_with_conditional_check`: Assigns several values to a global variable and calls another routine.
- `increment_global_if_small`: Increments a global variable by a constant amount if it is below a certain threshold.
- `decrement_global_if_large`: Decrements a global variable by a constant amount if it is above a certain negative threshold.
- `setup_copper_table`: Fills a large range of memory with a pattern of alternating values and bitwise-inverted values from a table, and sets a global variable to a routine address.
- `setup_copper_registers`: Updates several hardware registers and a global variable, and copies a series of values from a table to a range of memory.
- `clear_custom_register_range`: Sets a global variable to a constant, then writes zero to a sequence of custom hardware registers.
- `set_interrupt_enable`: Sets a global variable to a routine address and writes a value to a hardware register to enable or disable interrupts.
- `configure_display_parameters`: Updates hardware registers and copies a series of values from a table to a range of memory, then calls other routines.
- `initialize_hardware_and_memory`: If a bit in a global variable is set, it initializes a range of custom hardware registers with a specific value and fills a memory range with certain values.
- `modify_global_bits`: Performs bitwise operations and conditional bit-setting on a global variable based on its own bit-pattern.
- `populate_memory_from_table`: Iteratively writes a sequence of incrementing or table-based values into a memory area, ending with a pointer assignment.
- `iterate_string_and_set_flag`: Checks a register value to set a global variable to a specific byte, then iterates through a null-terminated string, calling another routine for every character encountered.
- `expand_string_to_buffer`: Reads a character and, depending on its value, copies a sequence of bytes from a calculated source address into a destination buffer using a fixed-size stride.
- `traverse_and_recurse_string`: Iterates through a null-terminated string, recursively calling itself for every non-zero byte, and updates a global variable based on a specific register value and the current state of that global variable.
- `copy_and_invert_data`: Calculates a source address from a table and copies bytes to a destination, while performing a bitwise NOT or XOR operation on the written bytes.
- `copy_string_data`: Iterates through a null-terminated string and copies a fixed block of bytes from a lookup table to a destination pointer.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a0, then returns.
- `copy_unrolled_data_block`: Calculates a source address from a character-based lookup table and copies a fixed block of bytes to a destination pointer using an unrolled loop.
- `copy_data_to_offset`: Copies bytes from a calculated source address to a destination pointer using a fixed loop count.
- `configure_blt_line_parameters_2`: Iterates through a range of memory and configures blitter parameters if a certain byte is non-zero, while also updating a global variable.
- `update_blt_parameters_2`: Sets a global variable to a specific value and then iterates through a memory range, calling another routine if a certain byte is non-zero.
- `update_blt_clipping_rect_2`: Iterates through a memory range and updates a global variable and blitter clipping parameters if a certain byte is non-zero.
- `update_blt_clipping_rect_range`: Iterates through a specific memory range and calls another routine to update blitter clipping parameters if a certain byte is non-zero.
- `update_blt_clipping_rect_skip`: Iterates through a memory range and calls another routine to update blitter clipping parameters if a certain byte is non-zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_blt_clipping_rect_short`: Iterates through a short memory range and calls another routine to update blitter clipping parameters if a certain byte is non-zero.
- `update_memory_with_fixed_value_2`: Writes fixed values to several memory-mapped registers and then calls another routine multiple times to perform a memory-fill or block-transfer operation.
- `update_memory_with_fixed_value`: Writes a fixed value and a calculated address to a destination, then increments a source pointer and waits for a specific bit in a memory location to clear.
- `recursively_execute_routine`: recursively calls itself a fixed number of times before returning.
- `update_memory_with_register_value`: Writes a calculated address and a value from a register to a destination, then increments a source pointer and waits for a specific bit in a memory location to clear.
- `configure_blt_line_parameters`: Calculates and writes a series or a range of block transfer (Blitter) parameters, including source/destination addresses and line-based or block-based drawing parameters, to a set of registers and a memory-mapped table.
- `update_blt_parameters`: Calculates and writes a series of values, including a width-related or offset-related value, to a block of memory using a loop.
- `update_blt_clipping_rect`: Calculates a new clipping or drawing area and updates the bitplane, block, and size registers for the bitboard or blitter engine.
- `set_global_to_max_value`: Writes a large constant value to a memory location pointed to by a register after calling another routine.
- `populate_table_with_increments`: Writes a sequence of incrementing values into a table or array.
- `set_pointer_to_invalid`: Writes a specific constant value to the memory location pointed to by a register.
- `clear_memory_block`: Clears a large block of memory by setting its contents to zero using nested loops.
- `zero_global_array`: Iteratively sets a range of memory starting from a global variable to zero.
- `update_table_pointers`: Iterates through a table of pointers and increments the upper word of certain entries if a corresponding byte in a secondary table is non-zero.
- `search_memory_for_value`: Recursively modifies a global variable and searches a memory range for a specific byte value, then modifies a certain memory location if a condition is met.
- `set_blta_registers`: Sets certain hardware registers for the blt chip and recursively executes itself a fixed number of times.
- `fill_blt_parameter_table`: Calculates and writes a sequence of values, including a width-related or offset-related value, to a block of memory using a loop.
- `manipulate_global_bits`: Performs bitwise operations and shifts on a global variable and then sets it to a constant.
- `compute_grid_position_and_loop`: Divides a 16-bit value by eleven, storing the quotient and remainder in local fields, then loops subtracting a clamped count from a frame word while calling unnamed routines.
- `validate_and_clear_memory`: Checks a structure for certain bit flags and pointer-based values, then clears a block of memory or writes a specific pattern to a memory location if certain conditions are met.
- `wait_for_flag_and_set_registers`: Busy-waits for a routine to return while checking a bit in a global variable, and updates several hardware registers and a global variable.
- `repack_and_update_memory`: Iteratively calls routines and, if certain conditions involving a global variable and a pointer-based value are met, updates a memory location with a repacked value.
- `validate_structure_and_state`: Performs a complex series of checks on a structure, including verifying bit flags, pointer-based memory values, and range checks, before potentially returning a status or overwriting a global variable.
- `empty_stub_routine`: It performs no observable work and simply returns.
- `set_flag_and_increment_counter`: It sets a bit in a 16-bit word located a few bytes before a pointer in A6, using a bit index derived from D3.
- `clear_memory_block_2`: Clears a block of memory by writing zero to each longword, starting from a pointer and iterating a fixed number of times.
- `repack_result_word_and_byte`: Calls another routine and repacks its 32-bit result, placing the low word first and the top byte of the result in the second word.
- `interleave_word_bits`: Reads two consecutive words from a pointer and returns a value formed from their low bits.
- `chain_call_result`: Calls an unnamed routine and passes its 32-bit result as the argument to a second unnamed routine.
- `xor_checksum_mask`: Calculates a cumulative XOR checksum of nine consecutive longwords from a memory location and applies a bitwise mask to the result.
- `xor_accumulate_limited`: Xors a sequence of longwords a certain number of times and returns the result masked with a pattern of alternating bits.
- `fill_memory_with_pattern`: Fills a block whose pointer is stored in the frame with a repeating alternating-bit longword, writing eight longwords per loop pass.
- `initialize_object_state`: Calls another routine and uses its result to compute three integers stored at fixed offsets in a structure, then writes two constant words and a derived word into it.
- `set_object_transforms`: Writes several fields of a structure from the results of unnamed routines, storing constant words and copying values between the results, and calls a routine that waits for the blitter to finish.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `reset_blitter_state`: Waits for the blitter, then writes constant masks and a control value into the blitter and DMA registers and clears a global variable.
- `modify_byte_based_on_preceding`: Reads a byte and checks a bit in the preceding byte.
- `clear_flag_byte`: Reads a byte pointed to by a pointer and a flag from the preceding byte.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `set_audio_dma_control`: The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable.
- `update_global_flags`: Checks a bit in a global variable and, if certain conditions are met, overwrites a global variable with a sequence of constant values.
- `set_global_flags`: It writes two constants to the same global variable in sequence, leaving the second value in place.
- `set_bit_in_global_flag`: It reads a 16-bit value from a table, adds a constant, and sets a bit in a global variable using a mask derived from that value.
- `set_bit_in_global_flag_2`: It reads a 16-bit value from a table pointed to by a register, adds a constant, and sets a bit in a global variable using a mask derived from the masked result.
- `set_flag_and_call_twice`: Stores a constant byte into a global variable, then calls another routine twice in sequence.
- `update_table_entry`: Reads a 16-bit value from a table via an index in a structure, calls another routine when the value is negative, then spins a number of times derived from the difference between two 16-bit fields before writing a value from a register back into the table and calling another routine.
- `loop_and_clear_memory`: Loops a certain number of times while checking a bit in a global variable, and clears a memory location if a certain condition is met.
- `read_port_into_status_byte`: Calls another routine to obtain a byte, clears a bit if a secondary value is non-negative, and then forces bit zero high before storing the result into a global variable.
- `update_ciab_prb_mask`: Updates a bit mask in a hardware register by calling a routine that derives a mask from a global variable and a table.
- `compute_ciab_mask_from_table`: It derives a bit mask from a global variable by clearing a bit chosen by a table-relative index and potentially clearing a second bit based on a condition checked in a table.
- `wait_until_routine_signals_one`: Calls a routine repeatedly, and between calls busy-waits while the low bit of a global flag remains set, returning zero once the called routine returns one.
- `overwrite_global_variable_2`: Checks a global flag and, if certain conditions are met, overwrites a global variable with a sequence of constant values.
- `overwrite_global_variable`: It overwrites a global variable with a sequence of constant values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_concatenated_registers`: Returns a 64-bit value formed by concatenating the contents of two 32-bit registers.
- `call_and_return_registers`: Calls another routine and returns the concatenated result in registers.
- `update_table_entry_and_flag`: If a register contains a non-zero value, it calculates an offset into a table to load a word into two global variables; otherwise, it sets a global variable to a constant and sets another global variable to a fixed value.
- `update_address_table`: If a certain flag is not set, sets the flag and calculates a series of offsets and addresses into a table or array using a base address and an incrementing offset.
- `update_audio_and_dma_parameters`: Updates a series of global variables or table entries with new audio register values and pointer-based offsets, while decrementing counters to trigger conditional updates to DMA or graphics-related pointers.
- `update_bitmapped_sprite_attributes`: Updates a pointer-based data structure or table, calculating new offsets and copying byte-sequences from a source buffer to a destination buffer.
- `parse_graphics_command_stream`: Parses a custom bytecode stream to update graphics or DMA-related parameters, such as pointer offsets, or setting bitmasks in a global variable.
- `configure_dma_and_interrupts`: Configures the DMA controller and interrupt enable or disable settings via the custom chip registers.
- `empty_routine`: The routine contains no valid instructions and performs no operations.
- `empty_routine_2`: The routine contains no executable instructions and performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_unmodified_registers`: returns the contents of two registers without modification.
- `update_global_state`: Checks a 16-bit register and, if non-zero, overwrites a global variable with a sequence of constant values and a memory location, otherwise resets a global variable and a flag.
- `configure_audio_dma_channels`: Sets the period, length, and volume for an audio channel and enables DMA transfers.
- `initialize_graphics_table`: Performs a routine that calls another routine twice.
- `initialize_graphics_and_table_data`: Checks a flag to perform a one-time initialization of a table or data structure, calculating several pointer offsets and values based on previously stored or calculated data.
- `update_dma_control`: Calculates a value based on a memory read and stores a bitmask into a global variable.
- `initialize_audio_and_dma`: Initializes memory buffers, converts bytes to strings, and configures DMA channels and audio registers while checking certain global flags.
- `parse_display_command_stream`: Parses a custom bytecode stream to update display or DMA-related parameters, such as pointer addresses and DMA controller enable bits.
- `update_dma_config`: Updates a global variable by or-ing it with a value derived from a pointer, and writes a value to the DMA controller register.
- `initialize_memory_buffers`: Clears a range of memory and fills another range with a constant value, then executes a routine to build a copper list.
- `tiny_ori_ori_ori_ori`: Small routine of 4 instructions that merges 0 into d0, then merges 0 into d0, then merges 4 into d0, then merges 0 into d4.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
