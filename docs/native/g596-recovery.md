# Game 596 (SpindizzyWorlds v1.1 1179): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00596). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2026906 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 50 raw-word lines |
| Functions named and described | pass, 403 of 403 named (0 author symbols), 403 of 403 described (2 by rule, 401 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1288 of 1289 (99.92%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7486 of 7503 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9533 |
| code | 1120 |
| data | 1117180 |
| padding | 1 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 148 | 15 |
| disk.1:bootblock | 1 | 86 | 5 |
| logged disk load from disk.1 | 1 | 86 | 5 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 7181 | 376 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| logged disk load from disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- SpindizzyWorlds_v1.1_1179/SpindizzyWorlds_v1.1_1179.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree SpindizzyWorlds_v1.1_1179_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 6 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- SpindizzyWorlds_v1.1_1179/SpindizzyWorlds_v1.1_1179_hdf/SpindizzyWorlds/disk.1: 6 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_ciab_prb_bit` (disk.1 +an address loaded at an address (215040 bytes)): Calculates a bitmask based on a value and a global variable, then modifies a bit in the CIAB_PRB register. Kind: CIAB_PRB bitmask check. Pass path: not determined.

## Functions


### game.slave

- `return_error_code`: The routine is the program entry point.
- `set_game_state`: Calls a world-transition routine and a world-camera routine indirectly.
- `update_world_camera`: The routine writes to a global variable and calls two other routines, then adjusts a short via a pointer and toggles bits in two memory bytes using shifts based on registers.
- `initialize_game_state`: The routine writes constants and jump-table entries into several global variables and RAM tables, installs pointers to wait-for-blitter and wait-for-blt-ready routines, and then enters an infinite loop that repeatedly writes a constant to a global variable and calls another routine.
- `set_world_constants`: The routine writes three constants to memory: two 32-bit values to two RAM locations and a 32-bit value to a global variable, then returns.
- `check_world_transition_flag`: The routine checks a short value in D1 against a constant and branches to another routine if it does not match.
- `handle_world_transition_2`: The routine checks a value in a register against a constant; if it does not match, it calls another routine via a table and returns the register value.
- `wait_for_blitter_ready`: The routine polls a global variable until a bit in it is cleared, then returns.
- `wait_for_blt_ready`: Polls a global variable until a specific bit is cleared.
- `wait_for_blitter_idle`: The routine writes a value to a global variable and then polls that variable until a bit is cleared, indicating the blitter is idle.
- `wait_for_blitter_ready_2`: The routine loads a 16-bit value from register D1 into a global variable and then spins until the bit in that variable that corresponds to the blitter ready flag is cleared.
- `wait_for_dma_idle`: The routine spins until a global variable no longer has a specific bit set, then returns.
- `invoke_deferred_callback`: The routine calls a function pointer stored in a global variable.
- `call_deferred_callback_2`: The routine calls a function pointer stored at an offset from a global variable.
- `call_deferred_callback_3`: The routine calls a function pointer stored in a global variable.

### disk.1:bootblock

- `initialize_boot_block`: The routine copies a block of code from another routine into a global variable and then performs two byte-wise XOR operations using values from registers.
- `init_display_hardware`: The routine sends an I/O request, clears a world state, copies a block of words from a source table to a destination table in the custom chip memory, then copies and doubles a small register table into the custom chip registers, resets the custom chipset, clears the world state again and stores a pointer to another routine in a global variable.
- `set_io_request_and_send`: The routine writes fields into an I/O request structure pointed to by a parameter, then invokes a function through a global variable and another function through a global variable, then writes further fields into a second structure and invokes the same function through the global variable again.
- `reset_custom_chipset`: The routine writes a set of values to custom chipset registers, including bitplane control, display window and DMA control registers, and clears a block of copper list registers.
- `clear_world_state`: The routine clears the first six 32-bit words of a global variable by storing zero into each element via a pointer in A1.

### logged disk load from disk.1

- `copy_memory_block`: Copies a fixed number of longwords from one memory location to another.
- `initialize_hardware_palette_and_custom`: The routine copies a 4-kiloword block from chip memory to a larger memory area and then doubles 16 words from a source area into custom chip registers.
- `set_io_request_fields_and_dispatch`: The routine writes several fields into an I/O request structure pointed to by a register, sets a global variable, calls another routine via a table, updates the request fields again, calls the same table routine, clears a field and sets a command field, then dispatches the request through a table routine.
- `clear_custom_chip_registers`: The routine writes a fixed set of values to several custom chip registers, including BPL1MOD, BPL2MOD, BPLCON0, BPLCON1, COLOR00, COP1LCH, COPJMP1, DDFSTOP, DDFSTRT, DIWSTOP, DIWSTRT and DMACON, and then clears a block of chip memory starting at a custom base address.
- `clear_five_longwords`: The routine receives a pointer in A1 and writes zero to the longword pointed to by A1 and the next five longwords, clearing six consecutive longwords in memory.

### logged disk load from disk.1

- `empty_stub_routine`: The routine contains no instructions and therefore performs no operation.
- `empty_routine`: The routine contains a few instructions but performs no observable work and returns a constant value.
- `set_global_and_loop`: The routine writes a constant to a global variable, calls another routine, then enters an infinite loop where it repeatedly writes a different constant to the same global variable and calls the same other routine.
- `decrement_counter_and_select_entry`: The routine decrements a counter byte in a structure and, if the counter reaches zero or a global flag is clear, increments a second byte in the same structure and uses it as an index into a table pointed to by a register to select a short value.
- `store_byte_from_d1_to_pointer`: The routine takes a byte from a register and stores it into a byte field located at a fixed offset from a pointer.
- `check_and_set_flag`: The routine reads a short from a global variable and compares it with a short from another global variable.
- `check_and_update_object_state`: The routine reads a short from a structure pointed to by a4 and compares it with a short from a global variable pointed to by a5.
- `initialize_object_state`: The routine writes constant values and copies from registers into fields of a structure pointed to by a register, then loops three times waiting for a flag in the structure to clear, updating several integer fields and a short field each iteration.
- `wait_and_copy_blitter`: The routine waits for a hardware flag to clear, then programs the Amiga blitter registers with values derived from its inputs and a global variable, and repeatedly updates blitter source and destination pointers while incrementing a counter and waiting for the flag each iteration.
- `wait_for_blitter_and_copy`: The routine waits for a condition on a global variable, programs the Amiga blitter registers for a copy operation, then repeatedly waits for the same condition and writes values from registers and a table into blitter-related memory locations before returning.
- `wait_and_initialize_object_state`: The routine waits for a flag in a structure to clear, then writes several constants and computed values into fields of that structure, including a scaled offset derived from a global variable and a register, and copies a pointer and a counter into other fields.
- `update_function_pointer`: Updates a global variable with a value selected from a table using a bitmask or offset.
- `copy_table_entries`: The routine copies a value from one location in a global table to another location in the same table, and then copies a value from the start of the table to another location in the table.
- `clear_global_word`: The routine stores zero into a global variable.
- `increment_offset_with_clamp`: The routine reads a 16-bit value from a global variable, adds an input offset, and if the sum exceeds the maximum 16-bit signed range it returns a negative sentinel.
- `store_word_to_table_and_return_offset`: The routine stores a 16-bit value from a register into a global variable and returns the sum of a 16-bit value from a register with a base pointer plus a constant offset.
- `copy_word_block`: The routine copies 8000 words from one memory area to another using pointers taken from a global variable.
- `copy_8000_words_from_pointer`: The routine copies 8000 words from a source pointer to a destination pointer.
- `fill_stack_frame_table`: The routine builds a table in memory by repeatedly writing a pointer to another routine followed by three global variables, using a counter that starts at 1999 and decrements to -1.
- `do_nothing`: The routine is empty and performs no operations before returning.
- `clamp_scaled_difference`: The routine reads a short from a global variable and subtracts a short from a table entry pointed to by a pointer in a register.
- `compute_scaled_difference`: The routine takes two short values from registers, adds them and checks if the sum is greater than -1.
- `clear_two_parallel_arrays`: The routine clears two parallel arrays by writing zero to each element of both arrays in lockstep.
- `mask_byte_with_counter`: The routine masks the input byte with a constant and updates a global counter.
- `decrement_counter_with_reset`: The routine checks a global flag and, if it is clear, uses another routine as a condition.
- `decrement_counter_with_reset_2`: The routine checks a global flag and, if set, decrements a global counter and wraps it to a fixed value while storing a value from another routine into a global variable.
- `handle_serial_keyboard_input`: The routine reads a byte from CIAA_SDR, clears CIAA_CRA and CIAA_ICR, and processes the serial keyboard value.
- `return_cached_flag`: The routine reads a global variable, clears it to zero if it is non-zero, and returns the original value.
- `spindizzy_worlds_v1_1_1179_sub_09c2`: The routine repeatedly calls another routine until the call returns a non-zero value, then returns.
- `wait_for_flag_clear`: The routine repeatedly calls another routine until it returns non-zero, then returns.
- `poll_input_loop`: The routine repeatedly calls another routine and checks its return value.
- `wait_on_global_variable`: The routine loads a global variable and tests it against zero in a loop that repeats until the variable is non-zero.
- `enter_infinite_loop`: The routine enters an infinite loop.
- `decrement_and_loop`: Decrements a register or variable in a loop until it reaches a certain value.
- `copy_palette_from_pointer`: The routine copies eight 32-bit words from a pointer in A0 into eight consecutive locations in custom chip memory starting at COLOR00.
- `double_palette_entries`: The routine copies sixteen 16-bit entries from a source table in memory to the Amiga custom chip color registers, writing each source value doubled to successive color registers starting at COLOR00.
- `double_short_table`: The routine doubles a block of 16 short values.
- `reset_global_variable`: The routine clears a global variable by storing zero into it.
- `indirect_jump_via_table`: The routine performs an indirect jump through a table pointer.
- `indirect_jump_through_table`: The routine performs an indirect jump through a table.
- `check_and_sign_extend_result`: The routine calls another routine and tests its return value.
- `propagate_error_flag`: The routine calls another routine and checks its return value.
- `process_input_and_reset_globals`: Performs conditional checks on global memory and updates memory or flags based on a loop or external input.
- `negate_flag_and_return`: The routine calls another routine three times and then returns a 16-bit value formed by negating the condition-code flag NF and concatenating it with a constant byte.
- `initialize_custom_chipset`: The routine writes values to custom chipset registers to set up display and DMA parameters, configures CIA port A, and stores a pointer to itself in a global variable before clearing a flag.
- `invoke_callback_and_clear`: The routine calls a function pointed to by a global variable if it is non-zero, then clears the variable by storing a constant value.
- `initialize_copper_list_pointers`: The routine sets four copper list pointer locations in custom chip registers to successive offsets from a base address, clears a counter if it is positive, and writes two control values to a global variable.
- `copy_palette_from_table`: The routine copies four 16-bit words from a global table into a structure pointed to by A5 and updates a size field based on a 16-bit value in the structure.
- `update_animation_timer_and_frame`: The routine updates a pair of timer counters stored in a global structure, clamping their values to a maximum, and clears a flag in the structure.
- `copy_word_table_with_checksum`: The routine copies a block of 16-bit words from a source pointer to a destination pointer while building a checksum.
- `copy_table_with_offset`: The routine copies a 32-bit value from a register to each entry of a table pointed to by a pointer, and writes a second 32-bit value from another register to an offset within each entry.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_world_state`: The routine copies a block of memory from one location to another and then writes constant values to several global variables and a table of pointers.
- `write_custom_co_copper_list`: The routine writes a series of short values derived from its own code into a set of custom copper registers.
- `scale_custom_values_to_hardware`: The routine calls another routine and then copies sixteen 16-bit values from a table pointed to by the return value of that call, scaling each by two, into consecutive locations in custom hardware registers starting at the base address of the custom chip registers.
- `apply_table_transform`: Modifies a range of a table by subtracting a bit-shifted value from another source, then writes doubled values to a hardware register range.
- `scale_palette_from_source`: The routine reads sixteen 16-bit values from a source pointer, applies a fixed-point scaling by subtracting a shifted and masked portion of each value, stores the results into a global table, then reads those sixteen values back and writes each doubled to a hardware register block.
- `extract_tile_flags`: The routine reads a global variable, masks the low two bits of a word from an offset within it, shifts the result left by five, and returns it as a short.
- `extract_low_bits_to_index`: The routine reads a 16-bit value from D0, masks it with three low bits, shifts the result left by five, and returns it as a short.
- `update_custom_chip_registers`: The routine updates a global variable and performs byte-wise XOR operations on memory pointed to by arguments, masks bytes, copies words from one area to another, and conditionally reads a byte from a global variable.
- `execute_empty_stub`: The routine performs a single branch to another routine without any intermediate instructions.
- `update_hardware_palette`: The routine reads a byte from a structure pointed to by a4, choosing between two offsets depending on a flag in the same structure and a call to another routine, and compares it with a byte stored in a global variable.
- `update_copper_list_from_source`: The routine copies a byte from a source structure into a 16-bit field of a global variable and then calls another routine.
- `clear_global_variable`: The routine stores zero into a global variable.
- `reset_state_flags`: The routine writes constant byte and word values into several offsets of a structure pointed to by A5, clears a global variable, and then writes a zero word into another offset of the same structure.
- `check_unaff_sum_threshold`: The routine calls another routine and then checks the sum of three short values held in registers.
- `empty_routine_2`: The routine contains no instructions and returns immediately without accessing memory or registers.
- `compare_short_values`: The routine compares a short value from an offset in a pointer with a short value from an offset in another pointer and returns early if the first is less than the second.
- `clear_four_longwords`: The routine writes the same 32-bit all-ones value to four consecutive longword slots in a structure pointed to by a register, then clears a global variable to zero.
- `store_state_fields`: The routine stores a pointer, a byte from a register, and a word from a register into three consecutive fields of a structure pointed to by a pointer in a register.
- `copy_and_clear_flag`: The routine reads a short value from a global variable via an address in A5, copies it to another short in the same structure and, if the value is non-negative, updates a pair of stored int and char fields from the structure when they differ, then writes a sentinel byte to the flag field and returns.
- `check_flag_and_dispatch`: The routine checks a short flag in a global variable and calls another routine if the flag is clear.
- `add_offset_to_global_variable`: The routine adds a constant to the value stored in a global variable and stores the result in another global variable.
- `set_flag_and_call_indirect`: The routine calls another routine and then, if a condition is false, calls an indirect routine via a pointer stored in a global variable, writes a byte to a global variable, and conditionally writes another byte and copies a value from a global variable with an offset to another global variable before returning.
- `clear_flag_and_set_timer`: The routine clears a byte in a global variable and, if a short flag in the same global structure is zero, it sets that flag and copies a value from another global variable into a timer field with an added constant.
- `add_offset_to_global_field`: The routine reads a 32-bit value from a global variable, adds a constant offset to it, and stores the result into another global variable.
- `clear_two_flags`: The routine checks two short fields in a structure pointed to by A5.
- `update_word_at_offset`: The routine reads a 16-bit value from a global variable, multiplies it by a constant, adds a constant to the low 16 bits, and writes the combined 32-bit result back to a global variable.
- `swap_adjacent_words`: The routine swaps the contents of a global variable with the value stored in the next word of the same table.
- `parse_input_string`: The routine repeatedly calls another routine to obtain a character value and processes each character, updating several global variables and invoking other routines under various conditions.
- `check_flag_and_return_value`: The routine tests a byte input combined with a constant; if the test is non-zero it clears a global variable and returns a global variable, otherwise it returns the word input ORed with a constant.
- `install_jump_table_entry_and_invoke`: The routine writes the address of another routine into a global variable and then calls that routine three times, the last call being via the stored pointer.
- `wait_for_vposr_and_flag`: The routine spins until a global variable reaches a specific range and then clears a global flag.
- `synchronize_global_pointer`: The routine checks whether a global variable matches the value stored through a pointer derived from a register.
- `loop_until_null_terminator`: The routine receives an uninitialized pointer in a register and repeatedly checks the byte at a fixed offset from that pointer until it finds a zero byte, then returns without modifying memory.
- `set_flag_and_sort_table`: The routine sets a flag byte in memory pointed to by a0, sets a bit in a word table indexed by d0, toggles a flag byte using d3, then calls another routine four times.
- `update_pointer_tables`: Iterates through a table to replace values with values from another table, then performs a reverse-order copy or merge of words.
- `set_custom_chip_registers`: The routine clears a byte in a global structure and resets two global counters to zero.
- `clear_copper_list_and_load_palette`: The routine clears a byte in a global structure, resets two global variables to zero, and then writes a series of 16 short values from a source table to consecutive locations in the custom chip register block, effectively loading a palette into the copper list.
- `initiate_platform_controller_state`: The routine clears a global byte and a byte in the controller structure, calls several other routines, and loops processing controller input.
- `return_immediately`: The routine contains no instructions and returns immediately.
- `copy_word_and_decrement_loop`: The routine copies a word from one offset of a global structure to another offset of the same structure, then runs a loop that decrements a short counter starting from a value derived from another word in the structure down to -1, calling another routine repeatedly on each iteration.
- `decrement_loop_with_flag_write`: The routine reads a short from a pointer, returns if it is less than one, then loops decrementing the value while recursively calling another routine.
- `restore_display_state`: The routine saves two words from a global structure, sets them to fixed values, and if an input value is small it indexes into a global table of strings and repeatedly calls another routine while advancing a pointer through the string.
- `restore_text_buffer`: The routine checks a small index and, if valid, uses it to select a 13-byte entry from a global table.
- `set_camera_target_and_check_flags`: The routine writes a computed value into a global variable and then calls another routine, returning its result unless a flag is set.
- `check_global_flag`: The routine reads a global variable and returns zero if the value is zero, otherwise it returns all bits set.
- `xor_rotate_decrypt_block`: The routine loads two 16-bit words from a fixed offset in the A5 address space and iterates over 2047 words of a global variable.
- `clear_global_and_call_handler`: The routine clears a global variable and writes a byte to memory via an address derived from an uninitialized pointer, then calls another routine through a function pointer stored in the global variable.
- `fill_output_table`: The routine repeatedly calls another routine four times per inner iteration and writes the value from a register to four consecutive output pointers, repeating the inner loop three times and the outer loop 128 times.
- `provide_constant_result`: The routine is empty and simply returns a constant value.
- `check_flag_and_return`: The routine reads a byte from a global variable and tests whether it equals one.
- `store_returned_value`: The routine calls another routine and stores the returned 32-bit value into a global variable.
- `set_pointer_from_flag`: The routine writes a constant to a word in a structure pointed to by a register, then conditionally overwrites that word if a byte flag in the same structure is set, and finally stores the address of a global variable into a pointer field in the structure.
- `set_animation_frame_and_pointer`: The routine writes constants to a structure pointed to by A5, repeatedly calls another routine, and then conditionally writes to two words in that structure based on the value of a global variable.
- `invert_word_blocks_loop`: The routine calls another routine and then iterates 128 times over a block of memory.
- `update_entity_table`: Searches a table for an empty entry and populates it with a pointer to a new object and several initialized values.
- `set_flag_and_call_helpers`: The routine stores a constant word into a global variable and then calls another routine four times in a row.
- `copy_word_from_table`: The routine checks a signed flag in a structure and calls another routine, then selects a pointer to a global variable or to a caller-provided pointer.
- `check_two_globals_and_return`: The routine compares a global variable with a value in D0 and compares two global variables, and returns the value in D0 if both comparisons match, otherwise returns all bits set.
- `find_entry_in_table`: The routine calls another routine and checks the condition code flag set by it.
- `clear_stack_slots`: The routine clears a block of memory by writing zeros to consecutive 32-bit slots.
- `reset_object_state`: The routine clears a block of fields in a structure pointed to by a register when a flag is zero, otherwise it recursively calls another routine based on three flag words in the structure.
- `copy_object_fields_from_table`: The routine computes an index from the incoming word, then copies a set of longword fields from a global table entry and from a fixed table into a structure pointed to by an address register.
- `configure_blitter_copy`: The routine waits for the blitter to be idle then programs the Amiga custom chip blitter registers to perform a series of four 16-byte block copies from a source table derived from an input index into a destination table derived from another input value, using a fixed blitter control word and size.
- `return_immediately_2`: the routine is empty and performs no operations before returning.
- `process_game_state_flags`: The routine calls another routine to obtain a status word and then branches based on bits in that word.
- `check_and_mark_table_entry`: The routine checks a table pointed to by a pointer in A6 and a value in A1.
- `clear_flag_in_table`: The routine calls another routine twice and then clears a single bit in a byte stored in a global variable by performing a bitwise AND with a mask that leaves all bits set except one.
- `decode_tile_map`: The routine reads a byte from a table pointed to by a global variable and a byte from a global buffer, combines them with a running value, and updates the running value with the result of another routine when the buffer byte is non-zero.
- `clear_two_tables`: The routine clears two tables pointed to by a global variable and an offset from the stack pointer.
- `clear_two_tables_2`: The routine clears two tables referenced via a global variable and a pointer derived from the A5 frame.
- `clear_and_reset_state`: The routine clears a word and a byte in a structure pointed to by a register, then reads a byte from that structure, masks the low two bits, shifts them left, and uses the result as an index into a global byte table to write the same table byte into two consecutive bytes of the structure.
- `clear_flag_and_dispatch_table`: The routine clears a word in a global variable and then calls another routine twice.
- `traverse_pointer_list`: Iterates through a null-terminated list of pointers and calls a routine at an offset or address derived from the pointer contents.
- `empty_routine_unreached`: The routine contains no instructions and is empty.
- `decrement_loop_guard`: A routine that reads a byte from a global variable, masks it to the two low bits, subtracts one and, if the result is non-negative, repeatedly decrements it until it reaches -1.
- `empty_routine_3`: The routine consists of nine instructions and ends with a return, performing no observable side effects on registers, memory or a global variable.
- `check_table_entry`: The routine loads a short from a table indexed by the top three bits of a word pointed to by a pointer, and returns early if the loaded value is negative.
- `restore_saved_state`: The routine checks a short in a global variable against a pointer value and, if the condition holds, saves two words from a global variable, updates several bytes and a word in the global variable with values from registers and an accumulator, masks the word, conditionally calls another routine once or twice, then restores the saved words.
- `restore_object_transform`: The routine saves a global variable and a word from a structure, adds several bytes from registers to four bytes in the structure, adds a word from a register to a word in the structure, masks that word, then conditionally calls another routine twice based on a shift test, and finally restores the saved global variable and word.
- `find_entry_in_table_2`: The routine walks a table stored in a global variable, advancing a pointer by the value stored in the current entry plus a fixed offset.
- `reset_object_state_2`: The routine conditionally calls another routine and updates several fields in a structure pointed to by a register, computing values from bytes in the structure and two caller-provided words, then clears flags and three long fields in the structure and copies a word from a caller-provided pointer into another field.
- `reset_object_state_3`: The routine clears a set of fields in a global structure and writes derived byte values into other fields of the same structure, using values from registers and from the structure itself.
- `update_object_slot`: The routine checks a global short and if it is not a sentinel value, it stores a pointer from a register into a table indexed by that short, writes three bytes derived from two global bytes and three register bytes into the same table, increments the global short by eight, and returns a byte from the pointed-to object.
- `decrement_loop_with_callback`: The routine checks a global short value and, if non-zero, repeatedly calls another routine with an incrementing integer argument while decrementing the global short by eight each iteration until it reaches zero.
- `call_other_and_return_byte`: The routine calls another routine and then returns the byte located one byte before the address in register A0.
- `read_byte_before_pointer`: The routine calls another routine and then returns the byte located one byte before the address in A0.
- `apply_collision_masks`: The routine iterates over a set of short lists, building four 32-bit masks by AND-ing entries from three tables indexed by the low byte of each list entry.
- `clear_and_write_tile_masks`: The routine clears a small block of words in a global variable and then repeatedly processes a table of tile indices, accumulating four 32-bit masks by AND-ing entries from a table indexed by the low byte of each tile.
- `compute_scaled_difference_2`: The routine returns the difference between two short values held in registers, shifted right by four bits.
- `adjust_object_offsets`: The routine reads three values from a structure pointed to by a pointer, calls another routine, then writes the first two values back to a different offset in the same structure with a constant subtracted, copies the third value unchanged, and stores a register value plus a constant at another offset.
- `clear_collision_flags`: The routine clears a block of sixteen 32-bit entries in a global table by writing a constant value to each entry, then writes a zero 16-bit value to a separate global location.
- `copy_table_entry_to_global`: The routine copies a 32-bit value from a table indexed by a short offset stored in a global variable into another global variable and then clears a 16-bit field in the same global structure.
- `increment_flag_and_return_index`: The routine reads a 16-bit index from a global variable, increments that index, then reads a 32-bit flag word from a global variable.
- `increment_counter_and_set_flag`: The routine increments a byte stored in a global variable and then sets a bit in a 32-bit word stored in a global variable.
- `draw_tile_rows`: The routine reads a signed offset from a global variable, clamps it to zero, then calls another routine once.
- `toggle_flag_and_iterate`: The routine toggles a flag byte in a structure and, if the flag was clear, increments a counter.
- `toggle_flag_and_iterate_2`: Toggles a bit in a global byte and conditionally decrements a counter or calls another routine.
- `set_custom_viewport_and_wait`: The routine computes a base screen position from a table entry and a global offset, optionally adjusts it by calling another routine when a threshold is exceeded, then waits for a hardware flag to clear.
- `wait_and_set_blitter_window`: The routine waits for a global flag to clear, then programs several custom chip registers to set up a blitter window and mask, writes a value derived from a table entry to a blitter register, adjusts two pointers by repeated increments based on a masked value, stores the adjusted pointers to custom registers, and writes a computed value to a global variable.
- `fill_display_list`: The routine writes four consecutive entries into a display list structure pointed to by A6, each entry setting two offsets and a constant word, then repeats this in a loop that increments the base pointers A0 and A1 while decrementing D5, followed by four final entries using D0.
- `copy_tile_words`: The routine copies 16-bit words from a source area to a destination area using a fixed stride.
- `copy_tile_attribute_block`: The routine copies a block of tile attribute words from a source table into a destination buffer using a base address and size derived from a global variable and a register parameter.
- `save_and_restore_word`: The routine saves a global variable, updates it by adding the sign of another global variable, calls another routine, then restores the original value of the saved variable.
- `clamp_two_axis_ranges`: The routine updates two pairs of range values stored via a base pointer.
- `store_word_and_byte_flags`: The routine stores a 16-bit value from a register into a word field of a structure pointed to by a pointer, stores the same value shifted right by three into another word field of the same structure, and writes a byte of all ones into a third byte field of the same structure.
- `clamp_and_store_value`: Calculates a value based on a register, clamps it within a range, and stores it or a modified version of it into a global variable or a hardware register.
- `update_object_position`: The routine checks a flag in a global variable and a distance condition involving two short values from a global structure and an input offset.
- `clear_and_process_flags`: The routine clears two short flags from a global structure and then conditionally calls another routine based on those flags and several other fields in the same structure.
- `copy_table_entries_4`: copies a block of longwords from a source table to a destination table while decrementing a global counter.
- `copy_table_entries_2`: The routine reads a byte from a global variable and uses it as an index to copy a fixed number of words from one table to another via a pointer derived from the stack frame.
- `copy_tile_rows`: The routine copies blocks of four 32-bit words from a source table to a destination table using an index derived from a word input, repeating the copy 168 times per iteration.
- `call_table_of_pointers`: The routine iterates over a table of function pointers stored in a global variable, calling each non-null entry once, then advances the pointer and repeats until a counter expires.
- `execute_routine_list`: iterates through a list of pointers and executes the routine located at each pointer until a null pointer or the end of the list is reached.
- `dispatch_callback_table`: The routine builds a 32-bit value from the input register and iterates over a fixed-size table of function pointers stored in a global variable.
- `update_table_values`: overwrites values in a destination table with values from a source table if the source value is smaller, or appends source values to the end of a list.
- `find_last_table_entry`: The routine walks a short table pointed to by a global variable, decrementing a counter and tracking the second short of each pair until it finds a sentinel value.
- `clear_level_entity_tables`: A routine that clears several per-entity fields for a range of entities and then writes a sequence of entity identifiers into a table.
- `clear_unaffiliated_pointer_fields`: The routine writes zero to two consecutive 32-bit words pointed to by an uninitialized pointer and writes a 16-bit all-ones value to a location six words later, then returns.
- `dispatch_table_entry`: The routine walks a fixed-size table of entries in memory and finds the first entry whose first word is zero.
- `copy_object_fields_and_increment_counter`: The routine copies four word fields from a source structure to a destination structure, increments a counter in the destination and optionally increments it again based on a flag in the source.
- `check_and_update_position`: The routine reads a short from a structure pointed to by a register, subtracts a short from a global variable, and checks the result against a range.
- `copy_object_data_and_check_bounds`: The routine copies four 16-bit words from a source structure in memory to a destination structure, then calls another routine.
- `reset_and_load_palette`: The routine clears a set of long-word entries in two tables derived from a base pointer, then copies 16-bit words from a set of source tables into two destination tables using an index derived from a global variable and a flag, writing the words to four interleaved locations within each destination table.
- `copy_palette_blocks`: The routine copies ten 4-word blocks from a table selected by a 2-bit index into a global variable area to two destination tables derived from a pointer argument.
- `copy_word_blocks`: The routine copies a sequence of word blocks from a source area pointed to by a global variable to a destination area pointed to by another global variable.
- `update_object_state`: When the stored short matches the input short, the routine sets a flag in a word field, calls another routine twice, and skips the update path.
- `decode_input_word`: The routine stores a 16-bit value from the first input register into a global variable and then conditionally transforms the second input register using flags in the first input register and values from global variables.
- `store_input_value`: stores a provided word into a global memory location, increments a global offset, and initializes a flag or status byte.
- `disable_custom_controller`: The routine checks a short global variable and, if non-zero, iterates over a table of shorts.
- `copy_short_from_pointer_to_struct`: The routine reads a short value from a pointer and stores it into a field of a structure pointed to by another pointer.
- `recursively_update_level_state`: Decrements a counter and recursively calls itself if certain object or boundary conditions are met, otherwise it clears large blocks of memory and copies a sequence of words from a table or a conditional source to two different memory buffers.
- `clear_and_load_tile_patterns`: The routine stores a word from the input into a global variable and, unless a flag is set, clears four blocks of long words in two tables.
- `copy_block_to_two_tables`: The routine copies five consecutive words from a source pointer into two destination pointers, writing the second through fifth words at offsets of 2000, 4000 and 6000 words from each destination base, then advances both destinations by ten words and repeats the process while a counter in D7 decrements to -1.
- `store_pair_three_times`: The routine calls another routine three times and after each call stores the two word values from registers D4 and D5 into consecutive word slots of a structure pointed to by A5.
- `store_pair_to_table`: The routine writes two 16-bit values from registers to consecutive word slots in a table pointed to by a pointer, repeating the store for three successive pairs of slots.
- `store_pair_three_times_2`: The routine calls another routine three times and after each call stores the two word values from registers into consecutive word slots of a structure pointed to by a pointer in A5.
- `store_word_pairs_to_table`: The routine writes two word values from registers to consecutive word slots in a table pointed to by a pointer, repeating the write three times with a fixed stride.
- `store_word_pairs_to_table_2`: The routine writes two 16-bit values from registers to consecutive word slots in a table pointed to by a pointer, repeating the operation three times with a fixed stride.
- `store_pair_to_global`: The routine calls another routine three times and after each call stores the two word values returned in D4 and D5 into consecutive word slots of a global variable referenced via A5.
- `void_routine`: This routine is empty and performs no operations before returning.
- `compute_transformed_vector`: The routine reads six short values from a global table via a pointer in a register and two short values from registers, performs a series of multiplications and subtractions, shifts the intermediate results, and combines the results into a 32-bit value that is returned.
- `wait_and_set_blitter_parameters`: The routine waits for a global flag to clear, then writes several values to custom chip registers including BLTAMOD, DMACONR and others, and sets up a blitter operation by storing pointers from its arguments into a small table.
- `wait_and_configure_blitter`: The routine spins until a global flag is clear, then writes values to several custom chip registers and the blitter control registers, loading source and destination pointers from arguments and a size from an input register, and updates a global variable with a computed value before returning.
- `apply_control_flags_to_accumulators`: The routine reads a global byte, stores a derived flag into a byte pointed to by an uninitialized pointer, and conditionally adds a fixed increment or decrement to two 32-bit global accumulators based on the four low bits of the byte.
- `fill_world_object_tables`: The routine writes a fixed set of values from several global variables and its own address into a table pointed to by a global pointer, repeating the writes for a number of entries with a stride of ten words.
- `increment_modulo_seven_value`: A short routine that calls another routine, then increments the returned word value by one and returns the result modulo eight.
- `reset_world_state`: The routine updates a global counter and copies a few global values into a table entry, then iterates over that table to clear bits in a mask and collect two values.
- `check_and_clear_flag_loop`: The routine checks a short field in a structure pointed to by A5.
- `clear_level_state`: The routine calls several other routines and then clears and updates a set of global variables.
- `clear_object_flags_and_state`: The routine clears a set of global variables and updates a mask derived from a table.
- `restore_text_buffer_2`: The routine saves and restores two words from a global structure and then iterates over a zero-terminated string using another routine, returning early if a flag at offset 14000 is clear.
- `decrement_counter_by_scaled_value`: The routine scales the low word of the input register to a 0-100 range, then repeatedly divides the result by ten, calling another routine and decrementing a short at a fixed offset from an uninitialized pointer on each iteration until the scaled value reaches zero.
- `scale_word_by_constant`: The routine takes a value in D0, masks the lower 16 bits, and returns the product of that word with a constant.
- `copy_word_table`: The routine checks a word in D0 and, if it is less than ten, calls another routine and then copies ninety-five words from a source pointer in A0 to a destination table located at an offset from A5.
- `copy_word_table_2`: The routine checks a word in D0 and, if it is less than ten, calls another routine and then copies ninety-five words from a global table to a destination pointed to by A0.
- `clear_flag_and_poll_until_set`: The routine clears a 16-bit global variable and then repeatedly calls another routine in a nested loop, polling the same global variable until it becomes non-zero.
- `run_countdown_loop`: Performs a loop that calls another routine while decrementing a register and passing a decreasing or increasing value or a pointer-related value to another routine.
- `decrement_and_return_counter`: The routine reads a byte from a global variable, clears it, and then calls another routine to obtain a 32-bit value.
- `empty_stub_2`: The routine is empty.
- `initialize_game_state_2`: The routine resets some globals and then, if certain conditions on a global pointer and a global value are met, returns early.
- `set_palette_table`: The routine writes a byte and a word to a global variable and then calls another routine.
- `build_string_from_input`: The routine prepares a four-word buffer and two 16-bit indices in A5 globals, then repeatedly fetches a byte from input, swaps it into the buffer via a temporary and updates the current index, handling backspace, newline, N/O commands and the letters F, A, E while also clearing the buffer on reset.
- `decrement_loop_call_other`: The routine initializes a short counter to a small positive value and then repeatedly increments an uninitialized word-sized variable and calls another routine with the current counter value and the incremented variable.
- `process_input_word`: The routine checks a flag in a global variable indexed by the low byte of the input word and conditionally calls another routine, then calls three other routines unconditionally.
- `recursive_call_twice`: The routine calls another routine twice and then returns.
- `or_table_to_memory`: The routine copies 32 words from a table pointed to by a global variable, using an index from a global variable, and ORs each word into four separate memory planes spaced apart in a buffer pointed to by a1, repeating the operation for eight iterations.
- `iterate_table_until_negative`: The routine walks through a table of 16-bit entries pointed to by a pointer in A2, calling another routine for each entry while the entry is greater than -1, advancing the pointer by two bytes each iteration.
- `traverse_table_and_check_value`: Compares a value from a global table against an index stored in a global variable, increments that index, and checks for a null terminator or certain bit patterns.
- `wait_for_key_input_loop`: The routine implements a wait loop that polls for input and reacts to specific keys.
- `copy_palette_block`: The routine copies a block of palette entries from a source table to a destination table when a flag in a global structure is set, using a loop that moves four words per iteration.
- `retry_loop_with_counter`: The routine calls another routine several times, then enters a loop that runs a fixed number of times, calling another routine each iteration and returning the result if it is non-zero, otherwise continuing until the counter expires and returning zero.
- `poll_input_with_timeout`: The routine calls another routine six times, then enters a loop that repeatedly calls another routine, calls another routine, and calls another routine whose return value is tested.
- `search_with_counter`: Executes a loop that calls another routine and checks a returned byte, decrementing a counter until a non-zero value is returned or the counter reaches zero.
- `retry_loop_with_counter_2`: The routine calls another routine five times, then enters a loop that calls another routine twice, calls another routine and returns its result if non-zero, otherwise decrements a short counter starting at 500 and repeats until the counter reaches -1, then returns zero.
- `write_custom_co_copper_list_2`: The routine writes sixteen 16-bit values from a table in the code segment to a hardware register block, doubling each value before storing it.
- `retry_loop_with_counter_3`: The routine calls another routine three times, then sets a short counter to 500 and calls another routine twice.
- `initialize_collision_table`: The routine processes a null-terminated string in memory and then initializes a 32-entry table from a global variable by copying 16-bit values using an index table.
- `copy_struct_fields_from_buffer`: The routine copies several fields from a byte buffer pointed to by a register into a structure pointed to by another register.
- `decrement_counter_and_reset`: This routine decrements a short counter in a structure and returns a failure code if the counter becomes negative.
- `copy_block_data_to_buffer`: The routine calls another routine and then, if a condition is set, copies fields from a source block to a destination buffer in a repeated loop, writing a short from the source to the destination and updating pointers.
- `copy_block_table`: The routine calls another routine and then, if a condition holds, copies a block of five words from a source pointer through a pointer to a destination pointer with fixed offsets, repeating a fixed number of times, and returns the value from the first call.
- `copy_struct_table`: The routine calls another routine and then, if a condition holds, copies a table of five-word entries from a source pointer to a destination pointer, writing the first word to the current destination, the second, third and fourth words to destinations offset by fixed strides, then advances both pointers.
- `copy_word_table_5x4`: A routine that reads a value via another routine and, if a condition based on the high and low bits of a byte is true, copies five words from a source table to a destination table in a 4-times stride pattern, using a counter in a register to repeat the operation.
- `fallback_to_global`: The routine calls another routine and returns its result if it is non-zero.
- `initialize_global_flags`: The routine writes constant values to several global variables and then returns.
- `loop_call_store`: The routine repeatedly calls another routine and stores a 16-bit value from a register through a pointer until the called routine returns zero.
- `check_loop_condition`: Evaluates a condition to determine loop continuation, returning a status to the caller.
- `copy_tilemap_to_buffers`: The routine copies 20-byte rows from a source table indexed by a byte map into two destination buffers, repeating the copy for 100 rows per layer and for three layers.
- `wait_for_counter_decrement`: The routine stores a value from a register into a table entry, clears a word in the same table, calls another routine, clears a global variable, then enters a loop that repeatedly calls another routine and checks a signed counter in the table.
- `set_up_and_poll_table`: The routine stores a value derived from a register into a table, clears a word in the same table, calls another routine, then repeatedly calls two other routines and checks a flag.
- `copy_masked_bytes_from_table`: The routine copies one to four bytes from a table pointed to by a global variable into a destination buffer using an offset from the same table, applying a mask and shift when a flag in a control word is set.
- `fetch_data_by_index`: The routine walks a linked list of short entries using a counter from a global variable to select an entry, updates two global pointers and counters, then reads a descriptor word from the selected entry.
- `reset_and_iterate_flags`: The routine clears two global words and sets a global word to eight, then repeatedly calls another routine while decrementing a counter loaded from a global word and incrementing a global word index.
- `decode_object_flags`: The routine reads a word from memory derived from a pointer in A0, masks and tests flag bits, and conditionally calls other routines before looping to a global variable.
- `decode_input_flags`: The routine reads a word from D1 and a pointer from the user stack pointer, then selects one of several constant values to store through that pointer and calls another routine repeatedly.
- `process_input_buffer`: The routine reads a byte from a global variable and clears it, returning early if set.
- `recursive_self_call`: The routine immediately calls another routine and then returns.
- `parse_input_value`: The routine stores two words from registers into a global variable, calls another routine and inspects the returned value.
- `no_name_in_the_executable`: The routine contains no code.
- `loop_until_negative_flag`: The routine calls another routine, then repeatedly calls two other routines until a flag derived from an unsigned register becomes negative, at which point the loop exits and the routine returns.
- `call_loop_check_d6`: Calls another routine, then repeatedly calls two other routines until a test on register D6 is false, looping while the test is true.
- `poll_input_until_yes`: The routine repeatedly calls another routine and loops while the returned short value is zero, incrementing a global variable each iteration and aborting if the variable wraps to -1.
- `increment_counter_and_process_string`: The routine increments a counter stored in a global variable, calls another routine, then iterates over a null-terminated string pointed to by a register, calling another routine for each character while preserving two words from a global variable and clearing a word in the global variable before returning.
- `write_inverted_bits_to_memory`: Iterates through a range of memory, calculates a bitwise inversion of certain bits from a source table, and stores the result in a destination table.
- `copy_tilemap_with_optional_mask`: The routine copies a block of bytes from a source area to a destination area using offsets derived from input values and global variables.
- `set_table_pointer_from_index`: The routine computes an offset from a short value in D0, adds it to a global variable base, and stores the resulting pointer into a slot of a table pointed to by A5.
- `check_short_value`: The routine loads a short from a global variable, adds a constant and tests the result against zero.
- `spindizzy_worlds_empty_routine`: The routine contains no instructions; it is empty.
- `tiny_add_subq_rts`: Small routine of 3 instructions that adds (a0)+ to d4, then subtracts 1 from d4, then returns.
- `empty_routine_4`: The routine contains three instructions and performs no observable work.
- `initialize_global_table_entries`: The routine writes three 16-bit values to consecutive locations in a global variable.
- `set_global_word_values`: The routine writes three 16-bit values into three consecutive words of a global variable.
- `set_flag_byte`: The routine writes a constant byte value to an offset within a global variable.
- `store_byte_from_pointer`: The routine copies a byte from a pointer in a register to a byte at an offset from a base address held in a register.
- `copy_word_to_offset`: Stores a word from a pointer location to a different offset using the same base pointer.
- `copy_word_from_a5_offset`: The routine copies a 32-bit word from an address derived from a5 plus a constant offset to another address derived from a5 plus a different constant offset.
- `return_concatenated_registers`: The routine calls another routine with a value derived from a global variable and an argument from the stack, then returns the concatenation of two registers.
- `empty_routine_5`: The routine contains four instructions and performs no observable work; it ends immediately without accessing memory or registers.
- `copy_plane_data_with_mask`: The routine copies 4-byte groups from a source pointer to a destination plane calculated from a global offset and index fields, writing to four 8 KiB-spaced planes.
- `initialize_world_state_2`: The routine writes several fields in a structure pointed to by A5, setting a word to a constant, a word to a constant, a word to zero, a pointer to a global variable, a long to zero, then calls another routine.
- `restore_two_word_variables`: The routine saves two word-sized values from a global variable area, repeatedly calls another routine while scanning a zero-terminated string via a pointer, then restores the two saved words and clears a word in the same global area.
- `no_name_in_the_executable_2`: The routine contains twelve instructions, calls another routine, and then returns.
- `save_two_word_pair`: The routine saves two 16-bit values from a global variable into local temporaries, calls another routine, then restores the two values back to the same global variable locations.
- `fill_tile_block`: The routine copies 32-bit values from two registers into a tile block addressed by a base pointer and two indices, writing the high and low bytes to the block header and then filling the block with the four 16-bit words from the registers.
- `execute_empty_routine`: executes another routine without performing any other operations or modifying any registers.
- `empty_routine_6`: The routine contains eight instructions and performs no observable work; it ends with a return without modifying registers, memory, or calling other code.
- `copy_tile_table_to_buffer`: The routine copies 16 entries from a global table selected by the low nibble of an input register, with an offset applied for values above 9, into a destination buffer whose base is derived from a register and a global pointer.
- `copy_tilemap_block`: The routine copies sixteen 32-byte blocks from a table indexed by the low four bits of an input word into a destination area pointed to by a register plus an offset from a global variable.
- `swap_nibble_byte`: The routine calls another routine three times and uses the return value from the second call.
- `swap_nibble_byte_2`: The routine calls another routine twice, keeps the first returned byte, and returns that byte with its high and low nibbles exchanged.
- `return_register_value_2`: The routine returns the value currently held in a register without modification.
- `test_bit_and_negate`: The routine checks a signed word in a register and returns the input value if it is non-negative.
- `convert_digit_recursive`: The routine takes a 16-bit value from D0, divides the low word by ten and recurses with the quotient and remainder packed into a new value until the quotient is zero, then calls another routine with no argument.
- `clear_memory_table`: The routine clears a block of memory by writing zeros to consecutive words, then sets several global variables to constant values and calls another routine multiple times.
- `empty_stub_routine_2`: The routine contains no instructions and performs no operations.
- `process_input_buffer_2`: The routine clears and polls a global input buffer, repeatedly invoking another routine to update its state and handling a negative flag, a zero flag and a special 'e' marker.
- `handle_input_command_state`: Evaluates a command or input value to reset or modify game state, clearing specific flags, resetting level or object pointers, and clearing certain table entries or counters.
- `decrement_on_condition`: The routine calls another routine and returns its result.
- `call_other_routine_loop`: The routine calls another routine seven times, then enters a loop where it calls another routine, calls another routine and stores its short result, breaks if the result is zero, otherwise calls another routine twice.
- `reset_game_state`: The routine clears several word and byte fields in a memory block referenced by an address register and then sets a global variable to a constant.
- `set_global_constant`: The routine stores the constant 10 into a global variable and then returns.
- `set_global_to_eight`: The routine stores the value 8 into a global variable and then returns.
- `set_masked_pattern_to_memory`: Iterates backwards from a certain value, calculating a bit-patterned byte based on the input value and storing it into a table at a calculated offset.
- `decode_tile_map_2`: The routine decodes a packed tile map by reading nibbles from a table referenced through a pointer and writes the resulting bytes into four interleaved planes of a destination buffer.
- `execute_unnamed_routine`: Calls another routine recursively or sequentially.
- `recursive_self_call_2`: The routine calls another routine five times in a row and then returns.
- `recursive_call_loop`: The routine calls another routine twice in succession and then returns.
- `decode_tilemap_block`: The routine decodes a 4×4 block of bytes using a table indexed by a value derived from the input word and a table pointer, writing the resulting bytes to four separate output buffers spaced 8000 bytes apart.
- `fill_tile_table`: The routine reads an integer from a global variable and uses it as a base address.
- `iterate_table_entries`: The routine walks a table starting at a global variable, advancing the pointer by two bytes each iteration.
- `check_global_value`: Compares two global variables and, if they are not equal, updates one with the value of the other and calls another routine multiple times.
- `set_memory_flags_and_call`: The routine writes constant bytes to two bytes of a global variable and sets a bit in a word of the same global variable, calls another routine, clears that bit, then calls the same routine five more times in succession.
- `copy_word_list_to_global_buffer`: The routine copies a word list from a source pointer into a global buffer and installs the buffer address in a global pointer.
- `copy_word_table_to_global`: The routine copies a word table pointed to by a register into a global variable area and updates related global pointers.
- `copy_word_list_to_buffer`: The routine copies a word list from a source pointer into a global buffer, stores the buffer address in a global pointer and a length flag, then repeatedly processes each word by calling another routine and updating a global counter and pointer until a zero word is encountered or the processing routine signals completion.
- `store_three_shifted_bytes`: The routine calls another routine and then stores three bytes derived from registers into offsets of a structure pointed to by a pointer.
- `copy_and_process_word_buffer`: The routine copies a word buffer backwards from one area to another, then copies a second word block into a global table.
- `loop_until_flag_cleared`: The routine repeatedly calls another routine until a global variable pointed to by a register is non-zero, then returns.
- `reset_and_save_pointer`: The routine clears a byte at an offset from a base pointer, clears a word at another offset, sets a byte at a third offset to all ones, calls another routine twice, saves a word pointer from a global variable, clears that global variable, calls another routine twice, restores the saved pointer to the global variable, and then calls another routine three times.
- `check_global_flag_and_call_helpers`: The routine calls another routine four times unconditionally, then checks a global variable and if it is positive calls the same routine twice more before returning.
- `restore_multiple_buffers`: Executes a series of routines that restore text buffers and restore two-word variables.
- `check_flag_and_dispatch_2`: The routine tests a global variable for non-zero and checks a bit in a register against a mask derived from that variable.
- `poll_input_flags_loop`: The routine clears a flag byte in a global structure and then repeatedly checks two short flag fields in that structure.
- `parse_input_command`: The routine reads a character from a pointer returned by another routine and updates several global variables based on the character value, applying masks to keep the values within ranges.
- `copy_three_bytes_from_table`: The routine copies three bytes from a source location in a global structure to a destination location in the same structure.
- `clear_two_word_flags`: The routine clears two word-sized fields in a structure pointed to by a register and then walks a table of signed short entries located in memory.
- `copy_2048_bytes_from_global`: The routine copies 2048 words from a global variable to a destination pointed to by a pointer obtained from the stack.
- `copy_2048_bytes_from_pointer`: The routine copies 2048 words from a source pointer obtained from a structure pointed to by A5 to a global variable table.
- `increment_and_loop`: The routine increments a short field in a structure pointed to by a pointer in A5, then repeatedly calls other routines while masking a register value and using the result of one call to control the loop.
- `decode_two_byte_table_entry`: The routine reads a byte from a global table using an index derived from a byte in D0 and a global base pointer, then combines it with a second table byte using a shift and addition, and finally combines the result with a third table byte to produce a 16-bit value.
- `decode_and_xor_table`: The routine reads bytes from another routine and interprets them as a small command stream that controls two indices.
- `compute_scaled_offset`: Calculates a value using a multiplication and a bitwise shift, then combines it with an addition.
- `store_state_from_register`: The routine stores a 32-bit value from a register into a global variable, copies a 32-bit value from one memory location to another, and writes a 16-bit value from a register and a zero word into two consecutive words of a structure.
- `copy_signed_word_block`: The routine copies a signed word block from a table pointed to by a register to a destination buffer, using a size derived from the input word and a flag.
- `copy_struct_from_offset`: The routine calls another routine and, if the negative flag is clear, copies a block of 32-bit words from a table pointed to by an offset stored in a global variable to a destination derived from that variable.
- `process_returned_value`: The routine calls another routine, extracts parts of its 64-bit result and conditionally stores a computed value into a global variable.
- `walk_string_table`: The routine calls another routine and then, if a condition is false, walks a zero-terminated table of short entries pointed to by an address in a register, advancing the pointer by the size of the entry plus the value stored in the entry, stopping when a zero entry is found.
- `evaluate_flag_and_mask_bit`: Checks a specific bit in a global variable and masks it out, while also performing conditional checks on a memory-based pattern to return a signed or masked integer value.
- `poll_input_flag`: The routine calls another routine repeatedly until it returns non-zero, then returns.
- `copy_from_table_and_store`: Retrieves a byte from a table using a pointer and a value, then stores it into a memory location offset from a pointer.
- `clear_memory_block`: The routine clears a block of memory by writing zeros to a series of words.
- `empty_routine_7`: The routine contains no instructions that operate on registers, memory, or call external code; it is empty.
- `process_keyboard_input`: The routine repeatedly clears a flag, calls several other routines, reads a value from another routine and processes a character code from it.
- `empty_routine_8`: The routine is empty.
- `return_register_value`: The routine loads a byte from a register and returns it unchanged.
- `copy_table_entries_3`: The routine copies a series of four-word entries from a table pointed to by a global variable into a destination buffer, advancing both pointers with each iteration.
- `process_tile_index`: The routine computes a tile index from an input value and a signed offset, using division and modulo by eleven and a flag derived from the input word.
- `initialize_input_state`: The routine clears several short fields in a stack frame and repeatedly calls other routines while checking a global flag and a short flag in the frame.
- `poll_input_and_update_globals`: The routine repeatedly calls another routine and updates a global variable based on a counter and the value held in a stack slot.
- `retry_input_validation_loop`: The routine repeatedly calls another routine, checking a flag and a counter, and returns early on certain conditions.
- `check_tile_collision_state`: The routine repeatedly reads state from a global structure and a global table and calls another routine.
- `empty_stub_routine_3`: The routine consists of five instructions that perform no observable work and then return.
- `set_flag_and_increment_counter`: The routine sets a bit in a 16-bit word located at an offset from a pointer in A6 and then increments a 16-bit counter located at another offset from the same pointer.
- `clear_table_entries`: The routine clears a series of table entries by repeatedly advancing a pointer and writing zero to the pointed location, looping a fixed number of times.
- `extract_high_byte_pair`: The routine calls another routine, takes its 32-bit result, and returns a 32-bit value formed by the low 16 bits of the result in the low half and the original high 8 bits of the result in the high half.
- `extract_and_combine_word_masks`: The routine reads two consecutive 32-bit words from a pointer in A0 and returns a 32-bit value formed by OR-ing the low and high words after masking each with a repeating 0101 pattern and shifting the first mask result left by one bit.
- `call_and_forward_result`: The routine calls another routine, stores the returned value in a local variable, then calls a second routine passing that value as an argument.
- `xor_fold_10_words_mask_low_bits`: The routine reads ten consecutive 32-bit words from a table pointed to by a global variable, XORs them together, and returns the result masked to keep only the low bits of each byte pair.
- `xor_checksum_words`: The routine computes an XOR checksum over a block of words pointed to by a pointer, using a word count derived from an input word.
- `fill_memory_with_pattern`: The routine writes a repeating 32-bit pattern to a block of memory obtained from a global variable.
- `initialize_object_state_2`: The routine calls another routine and uses its return value together with two output values to compute three integers that are stored at fixed offsets from a pointer in memory.
- `set_object_transforms`: The routine writes several fields in a structure pointed to by a2, using return values from other routines, and updates a short field derived from a call result.
- `wait_for_flag_clear_2`: The routine spins until a global variable has its bit 6 cleared, then returns.
- `reset_blitter_state`: The routine clears a global variable and writes constants to several custom chip registers, including the blitter control and mask registers and the DMA control register.
- `set_flag_byte_via_pointer`: The routine calls another routine twice, reads a byte from a pointer, conditionally modifies the byte based on the previous byte’s flag bit, and writes the byte back through the same pointer.
- `set_flag_byte_2`: The routine reads a byte from a pointer, inspects a flag in the preceding byte, and conditionally sets or clears a high bit in the pointed-to byte before writing it back.
- `mix_bits_into_word`: The routine mixes bits from an input word with a derived value and stores the result through a pointer.
- `set_disk_dma_parameters`: The routine writes values to a global variable, to a custom chip register and to another global variable, then stores an input pointer into a custom chip register and writes a constant to a global variable.
- `check_flag_loop_set_globals`: The routine calls another routine twice, each time using the returned pointer to test a flag byte and then sets two global variables to fixed values before returning.
- `set_global_flags`: The routine writes the value 2 to a global variable and then writes the value 16384 to the same global variable, overwriting the first write.
- `set_global_flag_from_table`: The routine writes a constant to a global variable and then, if a signed byte in a structure is negative, it reads a 16-bit value from that structure, adds a small constant, masks it, and uses it to build a byte mask that is stored back into the global variable.
- `set_bit_in_global_flag`: The routine reads a 16-bit value from a table pointed to by a register, adds a constant, masks the result to 5 bits, and sets the corresponding bit in a global variable.
- `reset_global_flag`: The routine writes a constant byte to a global variable and then calls another routine twice.
- `update_table_entry`: The routine reads a 16-bit value from a table via an index in a structure, optionally calls another routine when the value is negative, then spins a number of times derived from the difference between two 16-bit fields before writing a value from a register back into the table and calling another routine.
- `poll_joystick_fire_wait`: The routine polls a global variable and repeatedly calls another routine while decrementing a counter from 85 down to zero.
- `update_ciab_prb`: The routine reads a value from a peripheral port via CIAB_PRB and optionally masks a byte returned by another routine based on an extra output from that routine, then stores a masked and adjusted byte to a global variable and calls another routine.
- `call_other_routine_and_store`: The routine calls another routine and stores the returned value into a global variable.
- `update_ciab_prb_bit`: Calculates a bitmask based on a value and a global variable, then modifies a bit in the CIAB_PRB register.
- `wait_for_flag_and_result`: The routine repeatedly calls another routine until it returns 1, spinning on a global variable's least significant bit being clear between calls.
- `set_global_flags_2`: The routine checks a global variable and an input value and, if the conditions are met, writes three values to the same global variable.
- `set_global_values`: The routine writes three different byte values to a global variable and then returns.
- `initialize_audio_channels`: The routine copies a small block from a global variable into a table and clears and sets fields in four channel structures, then writes a few bytes to a global variable and returns the input registers.
- `process_audio_channels`: The routine iterates over four audio channels, updates per-channel counters and state bytes from a global table, and writes channel volume values to the four audio volume registers.
- `decode_tile_attribute`: The routine decodes a byte from a table indexed by a value taken from a structure and a base offset, updating several fields in the structure with the decoded values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_from_register`: The routine stores a byte from a register through a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `spindizzy_worlds_stub`: The routine contains no instructions and performs no operation, so it is an empty stub.
