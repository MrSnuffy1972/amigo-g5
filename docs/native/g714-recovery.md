# Game 714 (BobsBadDay v1.2 0637): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00714). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2271955 top-level bytes in 137 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 21 of 21 units identical (bytes and relocations), 214 raw-word lines |
| Functions named and described | pass, 340 of 340 named (0 author symbols), 340 of 340 described (18 by rule, 303 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 7226 of 7229 (99.96%) |
| Recovered without load address (counted in proof 1, flagged) | 1869886 bytes, 11 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12256 of 12883 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 119167 |
| code | 47436 |
| data | 2105342 |
| padding | 10 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 75 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1928 | 39 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2192 | 42 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1035 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1392 | 46 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 68 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1573 | 39 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2609 | 82 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 687 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 92 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 647 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |
| BOBMAIN2.RNC#unpacked (raw code file, load address unknown) | 1 | 4 | 1 |
| INTRO.RNC#unpacked (raw code file, load address unknown) | 1 | 562 | 10 |
| LUMP0.RNC#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| LUMP1.RNC#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| LUMP2.RNC#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| LUMP3.RNC#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| LUMP4.RNC#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| ONEMEG.RNC#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637.hdf: hard-disk image (AmigaDOS OFS, 2496 blocks): its filesystem parses and all 114 files in it are byte-identical to the extracted tree BobsBadDay_v1.2_0637_hdf, which is recovered file by file; no block outside the filesystem holds data
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/BOBMAIN2.RNC: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (136692 -> 259836 bytes)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/INTLUMP2.RNC: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (30702 -> 161408 bytes)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/INTRO.RNC: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (161055 -> 394288 bytes)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/LUMP0.RNC: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (54720 -> 166760 bytes)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/LUMP1.RNC: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (61652 -> 124112 bytes)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/LUMP2.RNC: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (61273 -> 122374 bytes)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/LUMP3.RNC: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (48816 -> 129688 bytes)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/LUMP4.RNC: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (52035 -> 151046 bytes)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/ONEMEG.RNC: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (210298 -> 521782 bytes)
- runtime segment an address (play-long-joy): 1540 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1019 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 511 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 195 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 65 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1303 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1272 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 514 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 86 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 647 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/BOBMAIN2.RNC#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/INTRO.RNC#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/LUMP0.RNC#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/LUMP1.RNC#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/LUMP2.RNC#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/LUMP3.RNC#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/LUMP4.RNC#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/ONEMEG.RNC#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- BobsBadDay_v1.2_0637/BobsBadDay_v1.2_0637_hdf/BobsBadDay/data/CIA.MAP: raw file found loaded verbatim in runtime segment an address (play-long-joy) at an address; classified through the recovered segment
- 8 never-loaded region(s), 1869886 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- INTRO.RNC#unpacked (raw code file, load address unknown)

## Copy protection found

- `set_global_and_return` (runtime segment an address (play-long-joy)): It sets a global variable to a constant value and returns zero. Kind: disk hardware/track validation. Pass path: the branch taken when d0 == an address (moveq #an address, d0).

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `reference_intro_rnc_file`: No decompiled body is available; the facts show it refers to the text intro.rnc and calls one other routine.
- `update_string_data_3`: Calculates an index from a global variable and a table, then copies a string from a source pointer to a destination buffer while updating a length field.
- `update_string_data`: Calculates an index from a global variable and a table, then copies a string from a source pointer to a destination buffer while updating a length byte.
- `write_and_wait_for_flag`: Writes a value to a memory location and enters a loop until a specific bit in a global variable is cleared.
- `store_word_to_offset`: Stores a word into a memory location at a specific offset from a pointer.
- `store_multiple_values`: Stores a word and a longword into memory locations at two different offsets from a pointer.
- `store_constant_word`: Stores a specific constant word into a memory location at a specific offset from a pointer.
- `store_constant_value`: Stores a specific constant value into a global variable.
- `store_constant_value_2`: Writes a fixed longword value to a global variable.
- `stub_returns_input_value`: Returns its first register argument unchanged with no other work, so it is a pass-through stub.
- `call_table_slot_eight`: Calls the routine whose address is stored at a fixed offset of eight bytes in a table referenced by a global, then returns.
- `call_function_pointer`: Calls a function through a pointer stored in a table and returns the result.
- `call_table_slot_twenty_four`: Calls the routine whose address is stored at a fixed offset of 24 bytes in a table referenced by a global, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_string_data`: Calculates an index into a table to retrieve a pointer, then copies a null-terminated string to a new destination and stores the resulting length.
- `update_joystick_graphics`: Adjusts joystick input values through memory manipulation and performs bitwise operations to update graphical data using the Blitter.
- `adjust_joystick_values`: Iterates through a block of memory to increment or decrement specific nibbles within each word to shift values toward target constants.
- `adjust_joystick_values_2`: Processes a sequence of words in memory by adjusting specific bit patterns to move them closer to predefined values.
- `configure_blt_hardware`: Calculates a value based on a global variable and updates several hardware registers associated with the Blitter and DMA controller.
- `process_bcd_data`: Splits a 32-bit value into bytes and performs binary-coded decimal adjustments on each component.
- `update_sprite_attribute_tables`: Extracts nibbles from a global variable to use as indices into a data table, then copies sixteen blocks of structured data from the selected table entries to a destination buffer.
- `update_sprite_attribute_tables_2`: Updates specific memory blocks by copying data from a source table to a destination table based on a global variable index.
- `initialize_blt_parameters`: Calculates a value for a global variable and writes several constant values to a structure pointed to by a register.
- `copy_data_blocks`: Copies multiple blocks of data from a source table to a destination address based on the value of a global variable.
- `bitwise_or_graphics_data`: Performs a bitwise OR operation between a source table and a destination buffer using a loop.
- `merge_graphics_tables`: Merges two large data tables by performing a bitwise OR operation on corresponding words in a loop.
- `update_sprite_attribute_table`: Checks a global variable against a range and, if valid, performs a loop to copy multiple attribute values from a source table into a destination structure.
- `setup_blt_hardware`: Updates a global variable and configures hardware registers for a bitblock transfer operation.
- `initialize_memory_tables`: Fills two consecutive memory areas with incrementing and decrementing values in steps of two hundred fifty-six.
- `generate_string_data`: Processes a bitmask to calculate character values and stores them into two global string buffers.
- `validate_encoded_values`: Compares several derived values against global variables and specific memory locations to return a status.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `render_text_to_graphics`: Parses a string of characters and performs bitwise operations to draw corresponding patterns into a graphics buffer.
- `draw_string_at_offset`: Uses provided coordinates and a string pointer to render character patterns into a destination memory area.
- `update_joystick_sprite_graphics`: Reads joystick movement and button states to update sprite data via blitter operations.
- `wait_for_hardware`: Polls a global variable until a specific bit is cleared and then writes values to hardware registers.
- `update_blt_registers`: Updates several Blitter hardware registers including mode, address, and size.
- `copy_data_blocks_2`: Copies multiple words from a source table to a destination table using a loop.
- `process_bit_table`: Iterates through a table of data and calls another routine if specific bits are set within each byte.
- `calculate_scaled_offset`: Performs trigonometric scaling and uses a pointer from a table to calculate and store a value into a global variable.
- `update_blt_parameters`: Calculates a value based on a table pointer and input registers, then stores it into a global variable and sets several memory locations.
- `tiny_move_move`: Small routine of 2 instructions that copies 6 to d0, then copies 6 to d1.
- `scan_string_for_characters`: Iterates through a null-terminated string and calls another routine when specific character values are encountered.
- `apply_bitmask_to_buffer`: Performs bitwise OR or AND operations on specific offsets within a buffer based on a global flag.
- `copy_and_format_data`: Copies a block of memory from a global variable to a fixed location and performs multiple bitwise shifts and masks on a global variable.
- `compare_and_copy_data`: Compares a series of values in a table against a counter and copies a block of memory if the counter does not reach zero.
- `write_global_values`: Stores several constant values into a global array.
- `execute_recursive_calls`: The routine performs a series of six consecutive calls to another routine before returning.
- `execute_recursive_calls_2`: The routine performs a series of consecutive calls to another routine before returning.
- `copy_memory_block`: Copies a block of bytes from a global variable to a fixed memory location.
- `execute_single_instruction_3`: This routine executes exactly one instruction.
- `count_flagged_bytes`: Iterates through a table to count occurrences of a specific byte value and stores the result in a BCD format.
- `set_table_values`: Iterates through three different lists of coordinate pairs to write specific byte values into a target table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_and_search_data_table`: Clears a memory block and performs a cyclic shift on a data table to locate a specific signature.
- `rotate_data_table`: Searches for a specific four-byte signature in a table and then performs a cyclic shift of the data elements until the signature is reached.
- `process_input_commands`: Calculates a memory offset from two input values and writes a character to it, then modifies several global variables and a table based on whether that character is 'S', 'R', or 'T'.
- `initialize_and_search_table`: Fills a memory block with zeros, performs a search through a table using a global pointer, and executes indirect calls based on found values.
- `execute_indirect_call`: Sets a global variable to zero and then performs an indirect call to an address calculated from register values.
- `set_global_constant`: Stores the value twelve hundred eighty into a global variable twice.
- `set_high_constant`: Stores the value thirty-four thousand five hundred sixty into a global variable twice.
- `return_from_subroutine_3`: The routine contains a single instruction and performs a return to its caller.
- `initialize_constant_value`: Stores the value twelve hundred eighty into a global variable twice via a subroutine call.
- `tiny_ori`: Small routine of 1 instruction that merges 198 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 24 into d3.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `return_immediately_stub_3`: The routine consists of a single return instruction and performs no work, no register or hardware access, and no library calls.
- `wait_for_register_bit`: Repeatedly clears a global variable in a loop until a specific bit from a hardware register is no longer set.
- `fill_memory_pattern_2`: Performs a nested loop to fill a large block of memory with a bitwise pattern and then clears it using a bitwise inversion.
- `fill_memory_pattern`: Fills a block of memory with an alternating bitwise-inverted pattern of a constant value and zeros using a loop.
- `clear_blitter_area`: Configures the blitter registers to perform a fill operation and then clears a large block of memory by writing constant values to a series of longword locations.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `initialize_global_values`: Stores a specific constant value into four different global variables.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
- `assign_routine_address`: Repeatedly stores the address of the current routine into a global variable.
- `update_sprite_pointers`: Writes values to eight consecutive hardware sprite pointer registers.
- `clear_global_variables`: Sets ten different global variables to zero.
- `copy_instruction_block`: Checks a global variable and, if zero, copies a sequence of instructions from a table into a destination memory area.
- `copy_code_block`: Checks a byte from memory, stores it in a global variable, and then copies a sequence of instructions from one location to another.
- `detect_collision_in_table`: Checks if a coordinate pair falls within a bounding box defined in a table and iterates through a list of collision shapes to verify intersection.
- `validate_range_and_recurse`: Performs a series of bounds checks on a global variable against multiple threshold values stored in a table.
- `calculate_scaling_vectors`: Updates multiple global variables and performs complex arithmetic involving trigonometric scaling and bitwise masking.
- `setup_scaling_parameters`: Loads several values into a table and performs trigonometric scaling calculations to prepare parameters for another routine.
- `configure_blt_parameters`: Calculates offsets and values for a Blitter operation, then writes several parameters to hardware registers and memory locations.
- `update_sprite_scaling_vectors`: Iterates through a data table to calculate scaled 2D coordinates using trigonometric values.
- `initialize_lookup_tables`: Fills, strides, and transforms multiple memory tables using incrementing values and clamping logic.
- `fill_increment_table`: Fills a table with longwords by starting with an initial value and incrementing it by a fixed amount for each entry.
- `update_sprite_scaling_table`: Updates a table of sixty-four longwords by incrementing values from a global variable and applies trigonometric scaling to 2D vectors.
- `fill_table_with_increments`: Fills a table with sixty-four longwords, starting from a global variable and incrementing by a fixed amount each step.
- `populate_stride_table`: Initializes a memory table by writing a starting value and incrementing it by a fixed offset for each subsequent entry.
- `initialize_two_tables`: Fills two separate tables with shortword values that increment by a fixed amount.
- `transform_table_values`: Iterates through a table to clamp specific minimum values and then stores the negated versions of those values in a second pass.
- `copy_and_negate_table`: Copies a table of words to a new location and then performs two passes of clamping and negating the values.
- `apply_trigonometric_scaling`: Calculates a 2D vector product using a table of values to scale a pair of short words.
- `calculate_vector_product_2`: Computes a 2D vector product using values from a table and stores the two resulting short words into a memory location.
- `calculate_vector_product`: Performs a 2D vector multiplication and stores the resulting two short words into a memory location pointed to by a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_state`: Modifies a global variable through a series of conditional checks, arithmetic shifts, and range clamping.
- `refresh_global_state`: This routine performs a redundant assignment of a global variable to itself before returning zero.
- `shift_stack_values_2`: A routine consisting of 18 instructions that is called by another routine.
- `shift_stack_values`: Clears several global variables and then stores two values from the stack into global variables after shifting them left by four bits.
- `validate_and_assign_constants`: Checks a global variable and assigns various constant values to it based on its state.
- `return_zero`: Calls another routine and returns zero.
- `set_constant_values`: Assigns specific constant values to a global variable through a conditional check.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 254 into d0, then returns.
- `process_global_value`: Performs bitwise shifts and conditional arithmetic on a global variable based on its value.
- `shift_global_variable`: Performs multiple left shifts on a global variable.
- `process_event_dispatch_loop`: Increments a global counter and uses it to index into a function pointer table for indirect execution.
- `update_status_flag`: Checks a status byte and, if non-zero, sets a global variable to a specific bit-shifted value; otherwise, it executes several other routines.
- `update_state_and_dispatch_event`: Increments a global counter and uses its value to index into a function pointer table for indirect execution.
- `return_stack_value`: Returns the value currently stored on the stack.
- `calculate_and_call_indirect_function`: Extracts a bitfield from a global variable to calculate an offset, then executes a function pointer retrieved from a table using that offset.
- `return_immediately_stub_2`: The routine consists of a single return instruction and performs no work, so it only returns control to its caller.
- `check_bitfield_status`: Calculates an index and bit position to check a bit in a table, then conditionally shifts a global variable.
- `update_input_state_flag`: Updates a global variable based on the result of a joystick state check.
- `update_joystick_state_2`: Checks a bitfield status to determine if a specific input bit is set, then updates a global variable based on the result.
- `decrement_global_counter`: Checks a global variable against specific threshold values and performs a bitwise-influenced subtraction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_or_clear_tables`: Copies data from a source table to four destination tables based on an input value, or clears those four tables with zeros if the input is three.
- `load_or_clear_dual_tables`: Copies data from a source table to two destination tables based on an input value, or clears those two tables with zeros if the input is three.
- `copy_table_segments`: Copies specific words from a source table into two different destination tables based on the value of a global variable.
- `copy_source_to_tables`: Copies specific words from a source table into two separate destination tables using a loop.
- `process_and_copy_large_tables`: Uses a global variable to select a source table and performs a series of word copies into two destination tables.
- `double_global_variable`: Multiplies a global variable by two.
- `load_multiple_tables`: Copies data from three different source tables into six destination tables using three separate loops.
- `clear_global_variable`: Sets a global variable to zero.
- `clear_global_variables_2`: Sets several global variables to zero after performing a conditional check.
- `update_sprite_parameters`: Performs conditional checks and updates global variables, potentially calling a routine to update a scaling table.
- `execute_recursive_logic`: Performs a recursive call based on the value of a global variable after performing several conditional checks and assignments.
- `update_sprite_scaling_parameters`: Calculates and updates sprite scaling and positioning data by applying trigonometric scaling and bitwise transformations to a global variable.
- `update_blt_scaling_parameters`: Calculates and updates scaling and positioning parameters for a Blitter operation.
- `iterate_data_table`: Iterates through a table of data using a pointer, calling another routine for each entry and updating the table values.
- `update_sprite_position_and_blit`: Calculates new coordinate offsets using trigonometric scaling and updates global position variables.
- `iterate_integer_array`: Iterates through an array of integers, checking for negative values and calling another routine.
- `check_coordinate_bounds`: Compares a global variable against two calculated values to determine if it falls within a specific range before updating a set of global words.
- `return_from_subroutine_2`: The routine performs a single return instruction and contains no functional logic.
- `copy_data_blocks_3`: copies multiple blocks of data from a source table to several destination locations using a loop.
- `test_word_and_branch`: Tests a word at a memory location and branches to another routine if the result is zero.
- `reset_global_on_range`: Tests a global variable against two range conditions, then either sets it to a specific value and calls other routines or performs arithmetic and calls two subroutines before setting the variable to a specific value.
- `compare_and_branch`: Compares a word in a register to a constant and branches to another routine if the result is not positive.
- `validate_and_reset_global`: Checks if a global variable falls within a specific range derived from its own value.
- `validate_and_negate_global`: Checks if a global variable is non-zero; if so, it sets the variable to a specific constant, runs a validation routine, and recurses.
- `validate_and_negate`: Negates a global variable twice and calls a validation routine before setting the variable to a specific constant.
- `negate_and_validate`: negates a global variable twice, calls a validation routine, and then sets a global variable to a specific value.
- `check_range_bounds`: performs arithmetic checks on a global variable to determine if it falls within specific ranges.
- `reset_global_values`: sets a global variable to three consecutive specific values.
- `set_global_to_max`: Calls another routine and then sets a global variable to a specific maximum value.
- `reset_and_update_sprite_data`: Clears specific global flags if a condition is met, then iterates through a source table to copy structured word offsets into a destination buffer.
- `update_sprite_attribute_table_2`: Iterates through a source table to copy specific word offsets into a destination buffer.
- `copy_and_scale_data_2`: Copies specific words from a source table to a destination using a loop and then applies trigonometric scaling to a global variable.
- `copy_and_scale_data`: Copies blocks of data from a source to a destination and calculates new values using trigonometric scaling and bitwise operations.
- `clear_global_flags`: sets two global variables and another global variable to zero.
- `validate_state_and_call`: checks a global variable against specific arithmetic conditions and calls several routines including a validation routine.
- `validate_global_range_3`: Checks if a global variable falls within a specific numerical range and performs redundant assignments to that variable if the condition is met.
- `validate_global_range_2`: Checks if a global variable falls within a specific numerical range.
- `validate_global_range`: Checks if a global variable falls within a specific numerical range and performs several redundant assignments to that variable if the condition is met.
- `swap_global_word`: Performs a series of moves that effectively swaps the values of a global variable with itself or temporary storage.
- `swap_global_values`: performs a series of operations that effectively swaps or reassigns values involving a global variable.
- `set_zero_sequence`: sets a global variable to one and then subsequently sets it and several other values to zero.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `process_scaled_table`: Copies data from a calculated offset in a table to several destinations and applies trigonometric scaling to a global variable.
- `clear_global_word_values`: Sets three specific global word-sized variables to zero.
- `calculate_sprite_scaling_parameters`: Calculates sprite positioning and scaling values by performing trigonometric lookups and bitwise transformations on global data.
- `update_blt_graphics_parameters`: Calculates sprite or background positioning and scaling values using trigonometric scaling and table lookups.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_blocks_4`: Copies multiple blocks of data from a source table to a destination global variable using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `validate_data_signature`: Checks a value at an offset from a pointer against specific constants and updates a global variable and a pointer based on the result.
- `configure_audio_parameters`: Loads values from a pointer into several registers and sets a global variable to a specific bitmask.
- `set_audio_channel`: Sets the period, frequency, and volume registers for a specific audio channel.
- `initialize_audio_channels`: Performs a delay loop and then initializes the period, frequency, and volume registers for four audio channels.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_offset_table`: Copies a block of data to a new location and then adds a calculated value to each word in the destination table through multiple passes.
- `validate_global_value`: Performs a series of bitwise checks and loops on a global variable to validate its value against specific constants.
- `update_sprite_data`: Processes a global variable to calculate offsets and updates a table of values and bitmask data.
- `update_bitmask_data`: Calculates values from a global variable to modify specific bits within a data table.
- `process_table_updates`: Uses a global variable to iterate through a table and perform bitwise operations on memory locations.
- `update_blt_sprite_data`: Calculates new coordinate and attribute values for a set of objects by performing bounds checking and table lookups.
- `update_blt_sprite_coordinates`: Calculates new position offsets for a set of graphical elements by iterating through a data table and applying boundary clamping.
- `render_blt_sprite_data`: Iterates through a data structure to calculate offsets and fetch pixel data for Blitter operations.
- `return_immediately_5`: The routine is empty and returns immediately.
- `initialize_global_values_2`: It stores the address of the stack into a global variable and sets another global variable to a constant value.
- `return_immediately_stub_4`: The routine consists of a single return instruction and performs no work, no register or hardware access, and no library calls.
- `set_dma_channel_mask`: Masks the lower six bits of an input byte and, if the result is less than four, stores a bit mask in a global variable.
- `loop_decrement_and_call`: Executes a loop from fifteen down to zero, calling another routine in each iteration before setting a global variable to zero.
- `update_structure_fields`: Calculates a bitmask from a register, updates several fields in a structure, and conditionally assigns a routine pointer and values based on arithmetic results.
- `enter_infinite_loop`: The routine contains no executable instructions and is effectively empty, immediately entering an infinite loop.
- `copy_target_from_source`: It reads a pointer from a global variable and, if a flag in the input byte is set and a source field is non-zero, copies a value and a 16-bit field from the source through an indirection pointer to a target location, then returns the input byte shifted right by one.
- `copy_source_to_target`: Checks a flag and a non-zero source field to copy a 32-bit value and a 16-bit value from a structure to a destination pointer.
- `execute_unspecified_logic`: This routine consists of seventeen instructions and is called by four other routines.
- `configure_dma_channel_mask`: Modifies the DMACON register to enable DMA channels and updates a global variable based on a masked input byte.
- `modify_memory_bits`: Performs bitwise OR and AND operations on various memory locations using values from registers and pointers.
- `process_data_list_2`: Iterates through a list of pointers, updating values in a structure and performing arithmetic on a counter.
- `update_clamped_values`: Modifies bytes in a structure using arithmetic and clamping logic.
- `update_structure_values`: Calculates a clamped value, updates a structure, and iterates through a list of pointers to modify data.
- `process_data_stream`: Iterates through a memory range, performing arithmetic on a counter and copying bytes from a source to a destination structure.
- `tiny_ori_4`: Small routine of 1 instruction that merges 2 into d0.
- `return_no_op`: The routine consists of a single return instruction and performs no work.
- `store_global_and_return_offset`: It stores the address of a global variable into a slot indexed by an uninitialized register and returns the sum of the value pointed to by the input pointer and the input pointer plus six.
- `copy_data_table`: It copies data from a source table into a global buffer, or clears the buffer if the source is invalid.
- `check_and_return_status`: Calls another routine and returns either a zero or a specific bitmask based on the result.
- `check_global_flag`: Checks if a global variable is non-zero and calls another routine if it is.
- `extract_structure_data`: Reads values from a memory structure pointed to by a register and updates a global variable based on a bitmask from the CIAA register.
- `clear_and_call`: Sets a global variable to zero and then calls another routine.
- `set_global_and_return`: It sets a global variable to a constant value and returns zero.
- `reset_global_flags`: This routine calls another routine that clears three global variables to zero before returning.
- `read_and_clear_global_flag`: Reads the current value of a global flag byte, then clears that flag to zero when it was nonzero, returning the value that was read.
- `perform_abs_move`: This routine consists of a single instruction that is accessed via an absolute move.
- `add_to_pointer_if_nonzero`: It reads the value pointed to by the pointer and, if that value is non-zero, adds the input value to it and stores the result back through the pointer.
- `adjust_pointer_offsets`: Runs a helper routine in a loop, then iterates through an array of pointers to add a base address to entries where a specific field is negative.
- `execute_single_instruction`: This routine consists of a single instruction and is entered from outside the static flow.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_blt_sequence`: Loads a sequence of values from a table and configures several hardware registers and memory locations for a block transfer operation.
- `setup_blt_parameters_2`: Retrieves a pointer from a table and populates multiple memory locations and registers with values derived from that pointer.
- `update_blt_sprite_animation`: Calculates an index for a sprite animation table based on a frame counter and updates Blitter registers to perform hardware bitblock transfers.
- `update_blt_sprite_parameters`: Calculates a pointer and offset based on an incrementing index and a data table to configure Blitter hardware registers.
- `update_blitter_sprite_data`: Calculates a source address and dimensions from a table based on a state machine and uses the Blitter to write data to memory.
- `update_blitter_sprite_sequence`: Updates Blitter hardware registers and memory locations by decoding a sequence of data from a table.
- `return_immediately_stub_6`: The routine consists of a single return instruction and performs no work, so it only transfers control back to its caller.
- `parse_encoded_sequence_data`: Decodes a stream of data by checking specific bit patterns and offsets against a set of constants.
- `initialize_blitter_loop`: Sets up several global variables and memory locations before executing a loop that performs repeated calculations and calls another routine.
- `configure_blt_registers`: Sets various Blitter control and parameter registers.
- `perform_abs_manipulation`: This routine consists of a single instruction and is accessed via absolute add, clear, or move instructions.
- `perform_abs_arithmetic`: This routine consists of a single instruction and is accessed via absolute add, compare, or move instructions.
- `execute_single_instruction_2`: This routine contains a single instruction and is invoked by three different routines via absolute addressing modes.
- `process_encoded_data`: Performs complex data manipulation including memory filling, register arithmetic, and bitwise masking.
- `tiny_ori_5`: Small routine of 1 instruction that merges 64 into d0.
- `tiny_ori_6`: Small routine of 1 instruction that merges 128 into d0.
- `tiny_ori_7`: Small routine of 1 instruction that merges 0 into -(a4).
- `tiny_ori_8`: Small routine of 1 instruction that merges 28 into d0.
- `update_blt_graphics_parameters_2`: Updates a series of Blitter registers and memory values by iterating through a table of data.
- `fill_memory_with_sequence`: Fills a block of memory with a sequence of values, starting from a specific value and incrementing each subsequent value by a fixed amount.
- `fill_memory_with_sequence_2`: Fills a block of memory with a sequence of 32-bit values that increment by a fixed amount each step.
- `return_bitwise_or_value`: Returns the value in a register bitwise ORed with a constant.
- `or_byte_with_constant`: Performs a bitwise OR operation between a byte in a register and a constant value.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `add_to_register`: Adds a constant value to a register.
- `apply_bitmask_to_register`: Performs a bitwise OR operation between a byte from a register and a constant value, returning the result.
- `move_address_pointer`: Moves a value into an address register.
- `move_byte_to_register`: Copies a byte into a data register.
- `decrement_and_compare`: Decrements a value and compares it against a constant.
- `test_and_modify`: Tests a value and performs arithmetic on a register.
- `store_byte_value`: Moves a byte value into a register.
- `transfer_byte_data`: Copies a byte from one location to another.
- `decrement_and_move`: Decrements a value and moves the result into a register.
- `apply_bit_mask`: Performs a bitwise OR operation on a byte using a constant value.
- `apply_bit_mask_and_test`: Performs a bitwise OR operation on a byte and updates status flags based on a comparison.
- `apply_bit_mask_simple`: Returns a byte modified by a bitwise OR operation.
- `return_masked_byte`: Returns a byte after applying a bitwise OR mask.
- `return_masked_value`: Returns a byte with a specific bit pattern applied via a bitwise OR.
- `modify_byte_with_mask`: Applies a bitwise OR to a byte and modifies a register using a decrementing instruction.
- `clear_and_mask_byte`: Clears a register and performs a bitwise OR on a byte.
- `test_and_mask_byte`: Tests a byte and returns it with a bitwise OR applied.
- `compare_and_mask_byte`: Compares a byte against a value and returns it with a bitwise OR applied.
- `tiny_ori_9`: Small routine of 1 instruction that merges 77 into d0.
- `return_input_byte`: Returns a byte value without modification.
- `return_uninitialized_value`: Returns the contents of a register without performing any operations.
- `update_global_counter`: Increments a global variable and performs conditional logic based on its value, potentially calling itself recursively.
- `clear_memory_block`: Fills a block of memory with zeros.
- `execute_nested_loops`: Executes a loop that repeatedly calls another routine and performs two nested decrementing loops.
- `decrement_global_with_loops`: Runs a loop containing multiple routine calls and two nested decrementing loops, while decrementing a global variable and clamping it to zero.
- `initialize_large_buffer`: Clears a large memory area, copies data from a source, and then performs a nested loop to fill a buffer with specific patterns and sentinel values.
- `clear_global_memory`: Sets sixteen consecutive double-word entries in a global table to zero.
- `loop_until_value`: Repeatedly calls another routine until it returns a specific non-zero value.
- `increment_nibble_differences`: Compares four-bit, eight-bit, and twelve-bit segments of two memory locations and increments or decrements a target value to match the difference.
- `return_immediately_stub`: The routine consists of a single return instruction and performs no work, so it does nothing when called.
- `set_dma_channel_mask_2`: Masks the lower six bits of an input byte.
- `loop_clear_global_flag`: Executes a countdown loop from fifteen to zero, calling another routine in each iteration, then sets a global variable to zero.
- `update_object_properties`: Updates a structure with values retrieved from a table and a pointer, while also modifying a global mask and setting various fields in a destination structure.
- `enter_infinite_loop_3`: The routine enters an infinite loop without executing any instructions.
- `copy_target_from_source_2`: Reads a pointer from a register and, if a bit in the input byte is set and a specific field is non-zero, copies a value and a 16-bit field from the source through an indirection pointer to a target location, then returns the input byte shifted right by one.
- `copy_attribute_data`: Checks a global flag and, if set, performs multiple subroutine calls and copies specific fields from one structure to another based on a pointer.
- `parse_sprite_definition_data`: Iterates through a data stream to decode sprite or object attributes, expanding compressed or encoded byte sequences into a structured memory format.
- `expand_sprite_definition_data`: Iterates through a compressed or encoded data stream to decode sprite attributes into a structured memory format.
- `modify_bitfields`: Performs bitwise OR and AND operations on specific bytes and words within various memory structures.
- `process_structured_data`: Iterates through a list of pointers to update values in a structure based on arithmetic derived from other memory locations.
- `update_entity_attribute_and_dispatch`: Modifies a byte value within a structure based on a nibble-based conditional logic and updates a secondary field.
- `update_clamped_value`: Calculates a value from two inputs, clamps it to a minimum threshold, and updates a structure's fields while iterating through a list.
- `copy_structured_data`: Copies data between structures, performs arithmetic on specific byte fields, and executes an indirect call based on a calculated address.
- `perform_self_assignment`: Stores a byte from a memory location back into the same location.
- `tiny_ori_10`: Small routine of 1 instruction that merges 2 into d1.
- `return_immediately_stub_5`: The routine consists of a single return instruction and performs no work, so it has no effect on registers, memory, or hardware.
- `store_global_and_return_sum_offset`: Stores the address of a global variable into a table at an offset determined by an uninitialized register.
- `update_global_buffer`: Checks a pointer within a structure for validity and non-zero length; if invalid, it clears a global buffer, otherwise it copies data from a source table into that buffer using a specified stride.
- `execute_conditional_logic`: Calls another routine and returns a status value based on the result of that call.
- `execute_conditional_subroutine`: Checks a global variable and calls another routine if the value is non-zero.
- `retrieve_structure_member`: Accesses a memory structure via a pointer to update global variables and return a specific word value.
- `call_conditional_routine`: Checks if a global variable is non-zero and, if so, calls another routine and returns its result.
- `set_global_to_zero`: Sets a global variable to zero and returns zero.
- `return_without_action`: The routine consists of a single return instruction and performs no work.
- `read_and_reset_flag`: Reads the value of a global byte and resets it to zero if it was non-zero.
- `add_to_pointer_via_address`: Adds a value to a location pointed to by a register if that location contains a non-zero value.
- `increment_pointed_value_if_nonzero`: Checks if the value at the address held in a register is non-zero, and if so, adds the value from another register to it and stores the result back at that address.
- `adjust_pointer_offsets_2`: Runs a helper routine in a loop until it reports completion, then walks an array of pointers adding a base address to each entry when a specific field is negative.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_single_word`: Stores a single word into a memory location relative to a register.
- `update_data_structure`: Calculates a value based on a table lookup and a global variable, then stores it along with several constants into a data structure.
- `update_data_structure_offset`: Calculates a value using an offset table and stores it into a data structure, including a conditional write of a global variable.
- `update_data_structure_with_index`: Determines a pointer based on a global variable and a bitmask, then writes it and a calculated value into a data structure.
- `update_data_structure_conditional`: Selects a value based on a register comparison, calculates a new value using a table, and writes the results to a data structure.
- `update_data_structure_alternate`: Performs a table lookup using a value derived from a register and a global variable, then populates a data structure.
- `calculate_sprite_coordinates`: Calculates a coordinate using a table lookup and bit-shifting, then writes several fields into a structure pointed to by a register.
- `update_blitter_source_offset_2`: Calculates a new source address offset by combining a bit-shifted register value, a table entry, and a global variable.
- `configure_blitter_hardware`: Sets multiple Blitter hardware registers and updates several global variables.
- `update_blitter_source_offset`: Calculates a new value by adding a bit-shifted register to a table entry and a global variable, then stores the result back to that global variable.
- `configure_blt_parameters_2`: Sets multiple Blitter hardware registers and updates several global variables.
- `update_data_structure_offset_72`: Calculates a value from a table and a global variable, then stores it into a data structure with specific constant values.
- `update_data_structure_offset_803`: Calculates a value using a table lookup and stores it into a data structure along with a global variable and specific constants.
- `update_data_structure_offset_603`: Calculates a value by adding an offset to a table lookup result and writes it into a data structure.
- `update_data_structure_offset_50`: Calculates a value by adding a larger offset to a table lookup result and populates a data structure.
- `update_data_structure_2`: Calculates a value using a combination of a register, a table, and a global variable, then stores the result and several constant values into a structure pointed to by a register.
- `update_data_structure_offset_2`: Calculates a value using a table lookup and bit-shifting, then writes several fields into a structure pointed to by a register using modified constants.
- `update_data_structure_simple`: Calculates a value from a register and a table, then writes that value and several other fields derived from a global variable into a structure.
- `apply_data_transformations`: Iterates through multiple data lists to perform sign inversions combined with bitwise masking, constant addition, or constant subtraction on specific elements.
- `process_data_list`: Iterates through a list of data blocks, performing sign inversions and bitwise masking on specific elements before writing them to a destination list.
- `process_data_list_with_offset`: Iterates through a list of data blocks, performing sign inversions and adding a constant offset to specific elements before writing them to a destination list.
- `process_data_list_with_subtraction`: Iterates through a list of data blocks, performing sign inversions and subtracting a constant from specific elements before writing them to a destination list.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_sequence_setup_into_record`: Writes fixed values into a record, including a maximum-value field, a mode word and a flag, and calls a helper.
- `initialize_four_record_blocks`: Calls a helper, then writes the same fixed pointer, a count of three and a value of five hundred eight into four fixed-offset blocks of one record.
- `return_register_pair_after_call`: Calls a helper and then returns the two register values it was given as one eight-byte result, passing them through unchanged.
- `enter_infinite_loop_2`: Performs no work and spins forever, so control never returns to the caller.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `set_global_to_sixteen`: Stores the value sixteen into a global variable and returns.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_helper_with_control_words`: Stores a computed offset and the address of a global, writes a fixed control word to that global, and calls a helper.
- `execute_dma_transfer`: A fourteen-instruction routine that interacts with the DMA controller and references a specific data file.
- `nop_instruction_placeholder`: This routine consists of a single instruction that performs no operation.
- `execute_control_word_helper`: A single instruction routine called by a helper that processes control words.
- `set_fixed_base_constant`: Stores a fixed constant equal to twelve mebibytes into a global longword variable and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `resolve_indirect_table_index`: Calculates a series of nested offsets by performing bitwise masking and conditional arithmetic on a global variable.
- `update_global_index`: Modifies a global variable based on its current value and then uses that value to calculate and store a new address into a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_2`: This routine is empty and returns to its caller without performing any operations.
- `return_immediately_4`: The routine is empty and performs no operations.
- `return_immediately_3`: This routine is empty and performs no operations before returning to its caller.
- `perform_empty_operation`: The routine performs no operations and returns immediately to the caller.
- `nop_instruction`: This routine is empty and performs no operations.

### BOBMAIN2.RNC#unpacked (raw code file, load address unknown)

- `execute_four_instructions`: A short routine consisting of four instructions at the start of a raw code region.

### INTRO.RNC#unpacked (raw code file, load address unknown)

- `execute_initial_subroutines`: This routine performs a sequence of small instruction blocks and color updates while interacting with custom hardware registers.
- `execute_eight_instructions`: A routine consisting of eight instructions that is called by another routine.
- `manage_hardware_interrupts_and_dma`: This routine coordinates bitplane and copper register updates, handles keyboard input via the CIAA, and manages DMA and interrupt enable bits.
- `update_blt_colors`: A routine consisting of twenty-six instructions that modifies bitplane DMA registers and color registers.
- `modify_dma_control`: A routine consisting of nine instructions that modifies the DMA controller register.
- `execute_six_instructions`: A routine consisting of six instructions that is called by another routine.
- `call_another_routine`: A short routine consisting of three instructions that calls another routine.
- `handle_cia_keyboard`: A routine consisting of thirty-seven instructions that interacts with the CIAA control, interrupt, and serial data registers.
- `execute_five_instructions`: A routine consisting of five instructions that is called by another routine.
- `perform_blt_dma`: A large routine consisting of one hundred ninety-eight instructions that manipulates bitplane DMA registers, copper registers, and interrupt enable registers.
