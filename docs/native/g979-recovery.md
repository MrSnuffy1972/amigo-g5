# Game 979 (GalaxyForce2 v1.2 0485): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00979). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2052041 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 404 raw-word lines |
| Functions named and described | pass, 561 of 561 named (0 author symbols), 561 of 561 described (22 by rule, 403 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 6557 of 6557 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11512 of 11657 instructions identical in start and length, 5 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21990 |
| code | 5244 |
| data | 1131365 |
| padding | 2 |
| unused | 893440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 111 | 24 |
| Disk.1:bootblock | 1 | 11 | 1 |
| logged disk load from Disk.1 boot-block load | 1 | 686 | 45 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 9619 | 446 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1225 | 40 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |

## Cases handled

- GalaxyForce2_v1.2_0485/GalaxyForce2_v1.2_0485.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree GalaxyForce2_v1.2_0485_hdf, which is recovered file by file; no block outside the filesystem holds data
- GalaxyForce2_v1.2_0485/GalaxyForce2_v1.2_0485_hdf/GalaxyForce2/Disk.1: the boot block reads 4608 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 10 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 58 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- GalaxyForce2_v1.2_0485/GalaxyForce2_v1.2_0485_hdf/GalaxyForce2/Disk.1: 10 logged loads cover 0 bytes of the image
- GalaxyForce2_v1.2_0485/GalaxyForce2_v1.2_0485_hdf/GalaxyForce2/Disk.1: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.21)
- 2 executed PCs start inside another executed instruction (overlapping-instruction idiom): decoded and counted as instruction starts

## Cracktros (kept, not protection)

- none

## Copy protection found

- `run_disk_dma_with_retry` (Disk.1 boot-block load +an address to an address (4608 bytes)): Calls a helper, then when a flag bit is set programs the disk DMA pointer and length, and enables disk DMA through the control registers. Kind: retry-based track/sector verification. Pass path: The loop continues as long as sVar2 (retry counter) > 1 and the condition (bVar3) (successful read) is false, meaning a successful read prevents the loop termination or certain failure branches..
- `set_adkcon_bit_from_index` (Disk.1 boot-block load +an address to an address (4608 bytes)): Writes a fixed control value to the audio-disk control register, then writes a value with one bit set or cleared according to a bit index taken from a frame word plus three. Kind: bit-set/clear trap (sabotage path). Pass path: the routine modifies a bit in a way that or-ing or or-ing with a mask (effectively setting/clearing a specific bit) or simply overwriting a bit, which likely targets a checksum or a specific hardware-dependent bit in the ADKCON register. If.
- `set_control_byte_call_two` (Disk.1 boot-block load +an address to an address (4608 bytes)): Writes 255 to a control byte, then calls two helper routines. Kind: loader-only protection (sabotage/check path). Pass path: execution continues to next instruction (no branch or crash).
- `update_port_b_status_bits` (Disk.1 boot-block load +an address to an address (4608 bytes)): Calls a helper, then clears bit one of a status byte when the helper's second output is non-negative, sets bit zero, stores the result and calls another routine. Kind: checksum trap / hardware-dependent bit check. Pass path: DAT_00bfd100 or bVar1 having bit 1 cleared/set based on extraout_D1b.
- `update_port_b_from_state` (Disk.1 boot-block load +an address to an address (4608 bytes)): Builds a mask from a frame index plus three and a state byte, clears one bit of the second CIA port value, and clears bit two when a table word is odd. Kind: bit-set/reset check (port state or checksum trap). Pass path: branch taken when bit is not set/cleared.
- `run_disk_dma_retry_b` (Disk.1 +an address loaded at an address (225280 bytes)): Calls a helper, then when a flag bit is set programs the disk DMA pointer and length and enables disk DMA through the control registers. Kind: track/disk check (retry-based). Pass path: The loop continues if sVar2 > 0 and the condition `(DAT_00bfe001 & 8) == 0` (or equivalent status bit check) remains valid, eventually returning a successful status or a non-error code (uVar1 or sVar2/bVar3 state)..
- `set_disk_control_words` (Disk.1 +an address loaded at an address (225280 bytes)): Writes two control words to a global variable in sequence, leaving the second value in place. Kind: not judged: timed out. Pass path: not determined.
- `store_result_to_global` (Disk.1 +an address loaded at an address (225280 bytes)): Calls a routine and stores its returned value in a global variable. Kind: track/disk check. Pass path: the branch after the call that avoids the failure/sabotage path.
- `compute_port_bit_mask` (Disk.1 +an address loaded at an address (225280 bytes)): Derives a byte from a global variable by clearing one bit chosen by a table index, with the bit position offset by three. Kind: disk/track check (bit-masking/bit-dependent check). Pass path: The branch where the mask or bit-check does not trigger a change or mismatch, specifically avoiding the bit-clearing/masking logic that would invalidate a valid bit-stream or checksum..

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `patch_jump_vectors_and_call`: Calls a routine through a function pointer held in a stored base structure, then writes an absolute jump instruction into two addresses and calls the code at a fixed address.
- `call_trap_after_base_pointer`: The decompiler reported unimplemented instructions here, so only part of the body is recovered.
- `write_register_value`: stores a word into a global variable offset and waits for a specific bit in another global variable to clear.
- `write_register_value_2`: stores a different word into a global variable offset and waits for a specific bit in another global variable to clear.
- `set_wait_flag_and_poll`: writes a word to a memory location and loops until a specific bit in a byte at a different memory location is cleared.
- `set_flag_and_wait_for_bit`: writes a value to a memory location and loops until a specific bit in a byte at a different memory location is cleared.
- `set_and_wait_flag`: writes a word to a memory location and polls a bit in a byte until a specific flag is cleared.
- `set_wait_condition`: writes a word to a memory location and polls a bit in a byte until a specific flag or condition is cleared.
- `set_and_wait_flag_2`: Writes a word to a memory location and polls a bit in a byte at a different offset until a specific bit is cleared.
- `set_wait_flag_and_poll_2`: writes a word to a memory location and then polls a bit in a byte until a specific bit is cleared.
- `set_wait_flag_and_poll_3`: writes a value to a memory location and loops until a specific bit in a nearby byte is cleared.
- `set_sync_flag_and_wait`: Writes a value to a memory location and polls a bit in a byte at a different offset until a specific bit is cleared.
- `replace_matching_table_entries`: Scans a table of 6,609 word entries and, wherever an entry matches a given word followed by a fixed marker word, overwrites that entry with a stored longword value.
- `write_word_then_wait_flag`: Writes a word to a register offset from a base pointer, then waits while bit six of a status byte is set.
- `write_word_and_wait`: Writes a word to a memory location and waits for a specific bit in a byte at a certain offset to clear.
- `store_byte_and_wait_for_flag`: stores a byte from a register into a memory location and loops until a specific bit in a nearby byte is cleared.
- `update_register_and_wait`: writes a word to a memory location and loops until a specific bit in a byte at a certain offset is cleared.
- `update_register_and_wait_2`: stores a word from a register into a memory location and loops until a specific bit in a byte at a certain offset is cleared.
- `write_and_wait_for_sync`: stores a word to a memory location and loops until a specific bit in a byte at a certain offset is cleared.
- `call_base_table_entry`: Calls a routine through a function pointer at a fixed offset from a stored base, then returns the input register unchanged.
- `wait_port_bits_when_flagged`: When a stored flag is nonzero, waits on the fire-button port.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### Disk.1:bootblock

- `boot_block_entry_do_io`: Boot block entry, reached through the system strap.

### logged disk load from Disk.1 boot-block load

- `set_jump_target_and_trap`: Stores a longword value into a memory location pointed to by the value in register 8 and then executes a software interrupt.
- `setup_display_and_start_system`: Loads the boot payload's entry, which sets the display window and data fetch registers, bitplane modulo and control registers, the copper pointer, and the interrupt and DMA enable bits, then calls eight helpers.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `write_intreq_value_seven`: Writes the value seven to the interrupt request register and returns.
- `write_intreq_value_eight`: Writes the value eight to the interrupt request register and returns.
- `write_intreq_value_sixteen`: Writes the value sixteen to the interrupt request register and returns.
- `write_intreq_value_1920`: Writes the value 1,920 to the interrupt request register and returns.
- `write_intreq_value_6144`: Writes the value 6,144 to the interrupt request register and returns.
- `write_intreq_value_8192`: Writes the value 8,192 to the interrupt request register and returns.
- `return_immediately_as_noop`: Does nothing and returns at once, so it works as a no-op callback.
- `compute_grid_position_and_loop`: Splits a stored position into a quotient and remainder by eleven, and when a second value is nonzero and the sum stays below 1,761, runs a loop.
- `write_disk_sync_header_retry`: Clears three frame words and retries up to three times.
- `run_disk_dma_with_retry`: Calls a helper, then when a flag bit is set programs the disk DMA pointer and length, and enables disk DMA through the control registers.
- `retry_checked_read_status`: Retries up to ten times, calling helpers and checking their flag results.
- `search_table_for_matching_entry`: Loops over a table with entries spaced 1,088 bytes apart, waiting on status bits and comparing stored values until it finds a matching entry.
- `return_without_any_work`: Returns at once without doing any work.
- `set_flag_bit_and_count`: Sets one bit in a 16-bit flag word, with the bit chosen by a register value, then increments a 16-bit counter stored beside it.
- `clear_ten_table_entries`: Zeroes ten longword entries in a table spaced 272 bytes apart, starting at a pointer register.
- `rearrange_helper_result_bytes`: Calls a helper and rearranges its 32-bit result.
- `combine_alternating_bit_halves`: Reads two adjacent 32-bit values from a pointer and returns them combined.
- `call_and_forward_result`: Calls one routine, then passes its returned value as the argument to a second routine.
- `compute_alternating_bit_xor_checksum`: XORs ten longwords starting eight bytes past a pointer into one accumulator, then returns the result masked to alternating bits.
- `compute_xor_checksum_block`: XORs a block of 32-bit words, with the count taken from a register divided by four, and returns the accumulated value masked to alternating bits.
- `fill_block_with_alternating_pattern`: Fills 256 longwords with an alternating-bit pattern, writing eight per pass, starting from a pointer kept in a frame word.
- `initialise_structure_fields`: Calls one routine and stores three derived values at fixed offsets in a structure, along with two constant words and a word derived from the call result.
- `initialise_blitter_structure_fields`: Waits for the blitter to finish, then writes several fields of a structure from helper results, including two constant words and a word derived from a call result.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `reset_blitter_masks_and_modulo`: Waits for the blitter to finish, then clears a flag and writes fixed mask, modulo and data constants to the blitter registers, and sets a DMA control value.
- `adjust_mfm_clock_bit_late`: Runs two helper routines, then adjusts the top bit of one encoded byte using the low bit of the preceding byte.
- `adjust_mfm_clock_bit_b`: Adjusts the top bit of one encoded byte using the low bit of the preceding byte.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `start_disk_read_with_sync`: Sets the disk control and DMA registers, writes the Amiga disk sync word, stores the buffer pointer in the disk pointer register, then sets the transfer length from a register value shifted right by one.
- `wait_for_disk_flag`: Polls bit one of a status byte, calling a helper between checks.
- `reset_disk_length_and_intreq`: Writes two values in turn to the disk length and interrupt request registers, leaving the last value in place.
- `set_adkcon_bit_from_index`: Writes a fixed control value to the audio-disk control register, then writes a value with one bit set or cleared according to a bit index taken from a frame word plus three.
- `set_port_b_bit_index`: Computes a bit index from a frame word plus three, then writes a value to the second CIA port register with that bit set and the other bits taken from a register.
- `set_control_byte_call_two`: Writes 255 to a control byte, then calls two helper routines.
- `delay_by_table_word_difference`: Reads a word from a table indexed by a frame value and compares it with a second word.
- `poll_port_bit_clear_entry`: Polls one bit of a status byte up to 86 times, calling a helper on each pass.
- `update_port_b_status_bits`: Calls a helper, then clears bit one of a status byte when the helper's second output is non-negative, sets bit zero, stores the result and calls another routine.
- `store_helper_result_in_state`: Calls one helper and stores its return value in a state byte.
- `update_port_b_from_state`: Builds a mask from a frame index plus three and a state byte, clears one bit of the second CIA port value, and clears bit two when a table word is odd.
- `wait_for_helper_result_one`: Calls a helper repeatedly, waiting on bit zero of a status byte between calls, until the helper returns one.
- `write_timer_control_if_idle`: When a status bit is clear and the input is not one, writes three control values in order, eight, 204 and two, to the CIA timer control register.
- `write_timer_control_sequence`: Writes three control values, eight, 204 and two, to the CIA timer control register in order.

### logged disk load from Disk.1

- `derive_joystick_state_bits`: Builds a state byte from the joystick port movement and fire button register bits.
- `wait_for_bit_clear`: Polls a global variable until a specific bit is cleared.
- `clamp_value_to_159`: Clamps a 16-bit value to zero when it exceeds 159 and stores it in a state longword.
- `copy_multiple_words`: Copies a block of eight words from a source pointer to a destination pointer.
- `reset_engine_state_tables`: Clears a block of memory or a sequence of pointers and resets certain global state variables.
- `write_to_self`: Writes a value from a register into the instruction pointer's own memory location.
- `clear_engine_state_tables`: Clears a block of memory or a sequence of pointers and resets certain global state variables.
- `update_global_flags`: Stores a word to a calculated memory offset and updates two global flags based on the state of a bit in a global variable.
- `tiny_nop`: Small routine of 1 instruction that does nothing for one instruction.
- `tiny_ori_2`: Small routine of 1 instruction that merges 1 into d0.
- `update_status_flags`: Checks the least significant bit of a global variable and updates two other global flags accordingly.
- `initialize_engine_state_tables`: Initializes a series of global tables and hardware-related pointers, including setting up copper state tables, channel tables, and row offset tables.
- `update_global_counter`: Updates a global variable by adding another global variable to it and conditionally negates a global variable based on its value.
- `extract_register_to_globals`: Extracts a byte from a register and stores it into separate global variables while performing bcd adjustment.
- `check_memory_bits`: Iterates through a range of memory, checking if certain bits in a byte are set and calling another routine if they are.
- `update_rotated_value`: Calculates a difference between two global variables, updates a third global variable with that difference, and then stores a bit-rotated version of that third global variable into another global variable.
- `update_structure_fields_3`: Checks certain global conditions and, if met, initializes a structure with a specific routine pointer, a constant, and several other values.
- `decrement_global_counter`: Decrements a global variable and writes zero to a memory location if the variable becomes negative.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_pointer_list_3`: Decrements a value through a pointer and replaces a null pointer in a list with a specific address.
- `update_pointer_list_2`: Sets a global variable and a memory location to a constant, then searches a list of pointers to replace a null pointer with a specific address.
- `modify_pointer_offsets`: Adds a constant to a value at an offset from a pointer and sets another offset from that pointer to a value derived from a global variable.
- `check_structure_bounds`: Iterates through a list of pointers to check if certain bitflags are set and if certain coordinate or size-related values fall within a specific range.
- `check_bounds_overlap`: Checks if two or more rectangular or range-based boundaries overlap using comparisons of offsets from a pointer and a global pointer.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_and_mask`: Compares a value at an offset from a pointer against a value at an offset from another pointer, returning a masked version of a register if the comparison fails.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d0, then returns.
- `calculate_scaled_offset`: Performs a complex check involving range-based bounds, then calculates a value using a bit-shift and division, and finally checks if a modified version of that value falls within a certain range.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `update_global_values`: Increments a global variable and sets other global variables to specific constant values.
- `update_structure_coordinates`: Calculates and stores new offset values into a structure based on a comparison, and decrements a global variable or searches a list to update a pointer.
- `insert_into_list_3`: Iterates through a list of pointers and replaces a null pointer with a specific address.
- `write_to_memory_loop`: Writes a sequence of constant values to a memory location using a loop.
- `update_list_with_bounds_check`: Iterates through a list of pointers, updating a value if it falls within a certain range, and searches for a null pointer to replace if no updates occur.
- `update_list_values`: Iterates through a list of pointers, updating a value at an offset and searching for a specific pointer to nullify.
- `do_nothing`: this routine is empty and performs no operations before returning.
- `decrement_and_insert_pointer`: Decrements a value through a pointer, checks a global variable, and searches a list to replace a null entry with a routine pointer.
- `insert_into_list_4`: Decrements a value through a pointer and then searches a list of pointers to replace a null pointer with a specific routine address.
- `store_register_to_global`: Stores a word from a register into a global variable.
- `update_pointer_and_set_offset`: Updates a global pointer using an offset calculated from another global variable, then conditionally sets a secondary offset or decrements a counter before loading a value from a table or memory into a global variable.
- `modify_memory_with_bit_shift`: Increments a byte at a memory location, then performs a bitwise XOR on a byte in a table using a bit-shifted value, and finally masks a return value into a global variable.
- `flip_bit_in_table`: Flips a specific bit in a table or array based on a calculated index and updates a global variable.
- `modify_structure_members`: Clears a bit in a structure member, clears a pointer, and sets several other structure members to constant values.
- `iterate_and_call_functions`: Iterates through a list of structures in memory, checking a flag and a function pointer before calling any non-null functions found.
- `search_and_update_list_flags`: Iterates through a list of pointers, checking certain bitflags and value ranges, and updates a bitflag or inserts a pointer into a list.
- `update_global_pointer_2`: Stores a routine address into a global variable.
- `update_table_index`: Increments a global variable by two masked by a bitmask and loads a new value into a global variable from a table using the updated variable as an offset.
- `modify_structure_and_dispatch`: Updates a structure with new values and routine pointers, and dispatches an indirect function pointer from a global table.
- `update_structure_and_counters_2`: Updates a structure with new values and routine pointers, and conditionally decrements or increments certain counters or structure members.
- `update_structure_and_counters`: Updates a field in a structure and increments several global counters or resets structure fields if certain conditions are met.
- `return_no_operation`: The routine consists of a single instruction that returns immediately, performing no work.
- `modify_structure_and_counters`: Decrements a value in a structure and, if certain conditions are met, resets structure fields and increments global counters.
- `reset_structure_fields`: Resets several fields in a structure to zero or a negative value and increments global counters.
- `return_immediately_stub_3`: The routine consists of a single instruction that returns to its caller, performing no work.
- `modify_structure_bits`: Clears certain bits in a byte at an offset from a pointer, sets a pointer to a specific address, and updates a value in a structure.
- `decrement_and_check_bounds`: Decrements a value in a structure and resets a structure or increments a counter if a certain threshold is reached.
- `increment_memory_value`: Adds a constant value to a word stored at an offset from a pointer.
- `update_structure_fields_4`: Modifies certain fields within a structure and updates a global counter or pointer.
- `reset_structure_fields_4`: Modifies a bit in a structure and a global variable, then conditionally resets several fields in a structure and updates a global pointer.
- `return_stub_placeholder`: The routine consists of a single return instruction and performs no work, so it is a no-op stub.
- `reset_structure_single_field`: Resets a single field in a structure and performs a reset or increment on other global variables.
- `calculate_address_value`: Calculates a value based on a global variable and stores it into a memory location at an offset from a pointer.
- `store_calculated_values`: Calculates several values using different arithmetic routines and stores the masked results into global variables.
- `calculate_difference_with_bounds_check`: Calculates the difference between two values, potentially negating one based on a flag, and returns zero if the result falls within a certain range.
- `set_bit_and_value`: Sets a bit in a byte at an offset from a pointer and then overwrites the pointer's target with a constant.
- `set_structure_parameters`: Checks if a value in a structure is below a certain threshold and, if so, sets a pointer and several other members to constant values.
- `clamp_and_accumulate_values`: Clamps a value in a structure to a minimum of zero and accumulates a result into a memory location.
- `decrement_and_reset_if_zero`: Decrements a value in a structure and resets a structure or increments global counters if the value reaches zero.
- `set_structure_parameters_2`: Writes certain values and pointers to a structure, and enters a loop that calculates new offsets and updates a structure or pointer.
- `configure_structure_and_flags`: Updates a global pointer, and if certain conditions are met, writes several constant values to a structure and sets certain flags.
- `initialize_structure_with_checks`: Checks a value from a pointer against several constants and, if certain conditions are met, writes several constant values and a function pointer to a structure.
- `initialize_structure_fields_2`: Sets several fields in a structure, including a pointer, a constant value, and several other fixed values.
- `setup_structure_and_call`: Writes several constant values to a structure, sets a function pointer, and then calls another routine.
- `update_global_pointers_2`: Copies several values from a pointer-based source to a set of global variables.
- `update_pointer_and_negated_value`: Stores an incremented pointer into a global variable and stores the value pointed to by that pointer, along with its negation, into two other global variables.
- `configure_structure_fields`: Writes several constant values and a function pointer to a structure, and sets a bit in a byte at an offset, then updates a field in another structure.
- `return_no_op_stub`: The routine consists of a single return instruction and performs no work, so it does nothing when called.
- `modify_structure_and_call`: Sets certain bits in a structure, writes a constant and a pointer to a structure, and performs an indirect call.
- `process_structure_list`: Iterates through a list of pointers, updating structure fields and calling a routine until a terminator or a specific value is found.
- `initialize_structure_with_values`: Writes a constant, a pointer, and several constant shorts to a structure, then returns a masked value.
- `initialize_structure_with_values_2`: Writes a constant, a pointer, and several constant shorts to a structure.
- `initialize_structure_with_values_3`: Writes a constant, a pointer, and several constant shorts or zeros to a structure, then performs an indirect call.
- `initialize_structure_with_values_4`: Writes a constant, a pointer, and several constant shorts or a different constant to a structure, then performs an indirect call.
- `initialize_struct_fields_2`: Initializes a block of memory or a structure using a pointer, setting a function pointer and several integer or constant values at specific offsets.
- `initialize_object_properties`: Sets a constant value, a function pointer, and several integer parameters or counters within a structure pointed to by a register.
- `initialize_object_properties_2`: Sets a series or a block of memory or a structure with a constant value, a function pointer, and several fixed-size or fixed-value parameters.
- `write_pointer_and_constants`: Writes a constant value, a function pointer, and several other constants to a structure relative to a pointer in a register.
- `initialize_object_properties_3`: Sets a constant value, a function pointer, and several integer constants into a block of memory relative to a pointer held in a register.
- `write_pointer_and_function_ptr`: Writes a constant value, a function pointer, and a function pointer or a constant to a structure relative to a pointer in a register.
- `update_structure_and_branch_2`: Updates a structure field and checks a range, then conditionally writes a function pointer and constants to a structure or updates a global variable.
- `increment_two_shorts`: Adds a constant value to two consecutive short integers at an offset from a pointer.
- `modify_two_shorts`: Subtracts a constant from one short integer and adds a constant to another short integer at an offset from a pointer.
- `set_pointer_and_offset`: Stores a routine pointer into a structure and calculates a new offset into a table.
- `reset_structure_and_branch_2`: Checks a value in a structure, and if it falls within a certain range, updates a structure field and conditionally resets several structure fields and increments global counters.
- `update_structure_field`: Checks a structure field against a range, and if certain conditions are met, modifies a structure field based on a comparison with a global variable.
- `update_object_position_and_reinitialize`: Modifies an object's coordinate or offset based on a range check, then conditionally reinitializes a structure with a new function pointer, constant, and copied coordinate data if certain bitflags and range conditions are met.
- `update_structure_and_branch`: Updates a structure field, checks a range, and conditionally updates a global variable and a structure with a function pointer and constants.
- `update_structure_fields`: Adds a value to a field in a structure and updates another field using a product result.
- `calculate_scaled_index_2`: Calculates a value based on a difference, checks a range, and writes a masked or modified value to a structure.
- `insert_into_list_5`: Iterates through a list of pointers and replaces a null pointer with a specific routine address.
- `search_and_clear_routine_pointer`: scans a sequence of up to thirty-two memory words for a pointer matching the current routine's address and clears the associated integer value if a match is found.
- `initialize_structure_with_mask`: Checks a masked global value and, if zero, initializes a structure with a routine pointer, a constant, and a specific short value.
- `insert_pointer_into_list_2`: Sets a global variable to a pointer and searches a list to find and replace a null entry with a routine pointer.
- `remove_pointer_from_list`: Searches a list of pointers and clears a pointer if it matches a specific routine address.
- `modify_structure_fields`: Updates several fields in a structure, including a short or integer offset, a bitmask, and a routine pointer, based on the result of a masked bitwise check.
- `insert_pointer_into_list`: Iterates through a list of pointers to find a null entry and stores a pointer or a value in that location.
- `search_and_clear_routine_pointer_2`: Iterates backward through a sequence of instructions or pointers, checking if a pointer matches the current routine's address, and clears the pointer to zero if a match is found.
- `update_structure_with_offset_2`: Checks a masked global value and, if zero, calculates an offset-based value and updates a structure with a routine pointer and a constant.
- `reset_structure_counters`: Modifies a pointer in a structure and clears several fields or increments global counters if certain conditions or values are met.
- `do_nothing_2`: this routine is empty and performs no operations.
- `insert_into_list_2`: Sets a global pointer to an incremented address, stores a value from a pointer, and iterates through a list of pointers to replace a null pointer with a routine address.
- `remove_from_list_2`: Iterates through a list of pointers and clears a pointer if it matches a specific routine address.
- `update_structure_with_offset`: Checks a masked value, and if zero, updates a structure with a calculated offset, a function pointer, and constants.
- `find_and_insert_pointer`: Searches a list for a null pointer and replaces it with a routine pointer while updating a global variable.
- `search_and_clear_routine_pointer_3`: Iterates backward through a sequence of routine pointers, checking if any entry matches the current routine's address, and clears the entry if a match is found.
- `initialize_structure_with_offset_2`: Checks a masked global value and, if zero, initializes a structure with a routine pointer, a constant, and an offset-calculated value.
- `modify_structure_and_branch`: Adds a constant to a structure field and, if a certain threshold is reached, updates a global variable and a structure with a function pointer and constants.
- `patch_routine_pointer_table`: Updates a sequence of routine pointers by replacing the first null pointer found in a fixed-length table with a new routine address, while simultaneously updating a global pointer and a secondary global variable with the next pointer in the sequence.
- `search_and_clear_pointer`: scans a sequence of longwords for a specific pointer value and clears that pointer to zero if a match is found.
- `update_object_properties_conditionally`: Checks a bitmask against a return value to decide whether to update a pointer-based structure with a new address, a constant, and a bitmask.
- `update_pointer_conditionally`: Writes a constant or an alternative constant to a 32-bit location via a pointer, and decrements a short integer at an offset from that pointer.
- `clear_pointer_list`: Sets a global variable to zero and then iterates through a list of pointers, replacing any null pointer found with a specific routine address.
- `reset_global_counter`: Calls another routine and then sets a global variable to zero.
- `remove_from_list_3`: Iterates through a list of pointers and sets a pointer to zero if it matches a specific routine address.
- `clear_matching_pointer_in_range`: Iterates backward through a sequence of pointers starting from the routine's own address and clears a pointer to zero if it matches the routine's address.
- `insert_into_list`: Iterates through a list of pointers and replaces a null pointer with a provided value.
- `remove_from_list`: Iterates through a list of pointers and clears a pointer if it matches a provided value.
- `clear_routine_table`: clears a sequence of memory locations by writing zero to each, starting from the routine's own address and continuing for a fixed number of entries.
- `execute_list_pointers`: Iterates through a list and calls any non-null routine pointer found.
- `execute_linked_list`: Iteratively traverses and executes a list of routine pointers while updating a global pointer and a control variable.
- `traverse_and_execute_chain`: Iteratively traverses a chain of pointers, updating a global pointer and executing an indirect call if certain conditions are met.
- `set_routine_pointers`: Sets a global variable to a pointer and another global variable to the next pointer in a sequence.
- `update_global_pointer`: Sets a global pointer to the value of another global pointer.
- `initialize_global_state`: Sets several global variables to specific constants and conditionally updates a pointer or a value based on a global flag and a comparison.
- `initialize_global_state_2`: Sets several global variables and memory locations to zero or constant values, then loads a value from a table-based offset.
- `reset_global_pointers`: Clears a range of memory, sets several global pointers to a common address, and copies certain values from a table-based offset.
- `initialize_global_pointer_table`: Clears a block of memory and initializes a series of global pointers to that memory block, followed by a sequence of constant values.
- `initialize_pointer_tables`: Initializes several global pointers to a common memory block and sets a series of global variables to specific constant values.
- `set_global_routine_address`: Stores the current routine address into a global variable.
- `do_nothing_stub`: The routine consists of a single return instruction and performs no work.
- `initialize_global_values_2`: Sets a global variable to a constant and another global variable to a function pointer.
- `load_table_values`: Calculates an offset into a table and loads three dwords from that table into global variables.
- `set_pointer_sequence`: Stores a pointer from a memory location into a global variable and sets another global variable to point to the next memory location.
- `update_bcd_value_2`: Sets a global variable to a constant and performs a BCD-adjusted increment or modification using a library routine.
- `initialize_structure_with_pointer`: Updates a global pointer, populates a structure with a value, a routine address, and a constant, and sets a flag bit.
- `initialize_structure_with_offset`: Updates a global pointer, populates a structure with a value, a routine address, and a constant, and sets certain flag bits.
- `initialize_structure_with_alternate_flag`: Updates a global pointer, populates a structure with a value, a routine address, and a different constant, and sets certain flag bits.
- `initialize_structure_with_extended_flags`: Updates a global pointer, populates a structure with a value, a routine address, and a constant, and sets additional flag bits and a routine pointer.
- `initialize_routine_metadata_structure`: Initializes a structure or object by setting a pointer, calculating a relative offset, and populating a table with a routine address, a constant, and a flag.
- `initialize_structure_and_set_flags`: Sets a global pointer, copies a value, and if a condition is met, populates a structure with a value, a routine address, and a constant, while setting several bitflags in a structure.
- `update_object_properties_and_copper_state`: Iteratively modifies structure offsets and pointer-based values, then updates a global pointer and a sequence of copper-related or state-table-related constants.
- `initialize_struct_fields`: Updates a global pointer, and if a condition is met, initializes several fields of a structure, including a pointer, a constant, and certain bitflags.
- `configure_memory_structure`: Updates a global pointer, modifies several memory offsets relative to a pointer, and performs bitwise OR operations on a byte in a structure.
- `initialize_data_structure`: Sets a global pointer, writes a constant value to a memory offset, and applies bitwise flags to a byte in a structure.
- `execute_conditional_callbacks`: Iterates through a list of pointers, checks a bitflag in a structure, and calls a function if the flag is set.
- `update_blitter_rectangle_list_2`: Updates a list of blitter rectangle parameters by calculating new source and destination pointers and setting a bitflag.
- `set_struct_parameters`: Writes a constant to a structure, sets a bitflag, and writes several constant values to other offsets in the same structure.
- `initialize_structure_members_2`: Checks a global flag, and if certain conditions are met, initializes several fields in a structure, including pointers, constants, and routine addresses, while searching a list.
- `update_structure_fields_2`: Updates several fields in a structure, including a conditional check on a value and a conditional assignment of a pointer or a constant.
- `calculate_scaled_index`: Calculates a scaled index or a clamped value based on the absolute value of an input and a divisor, then retrieves a value from a table.
- `reset_structure_fields_2`: Clears several fields in a structure and updates a global pointer and counters.
- `set_global_constant`: Sets a global variable to a constant value.
- `reset_structure_and_branch`: Resets several fields in a structure and performs an indirect call to a calculated address if certain conditions or range checks pass.
- `execute_indirect_routine`: Performs an indirect call to a routine pointer stored in a global variable.
- `modify_global_and_reset_structure`: Decrements a global variable and, if it becomes negative, clears certain bits in a structure and resets several fields or routine pointers.
- `modify_global_bcd`: Calls another routine and performs a BCD-adjusted modification to a global variable.
- `clear_structure_fields`: Clears certain bits in a structure field, clears a pointer-based value, and sets a routine pointer and a constant.
- `update_structure_recursively`: Modifies a structure field based on a calculated value and performs recursive calls to another routine while adjusting a structure field.
- `modify_structure_and_call_2`: Increments a value in a structure, performs an indirect call, and then modifies a structure field using a value from a register.
- `initialize_recursive_object_structures`: Decrements a global counter and, if certain conditions are met, recursively initializes two separate object structures with a set of predefined or calculated field values, including pointers to another routine and certain flags.
- `update_structure_with_bounds_check`: Updates certain structure fields using a product of a register and a value, and conditionally updates a structure field if a certain range check passes.
- `iterate_and_assign_structure_fields`: Iterates through a list of pointers, checking a flag in a structure, and assigns values to a structure if the flag is set.
- `initialize_global_values`: Sets several global variables to zero or specific constant values.
- `initialize_global_tables`: Clears a range of memory and populates a table with a sequence of pointers and incrementing values.
- `initialize_structure_members`: Clears several structure members to zero or a constant and sets a flag and a function pointer within a structure.
- `reset_structure_fields_3`: Clears several fields in a structure, updates a pointer, and sets a flag in a byte at an offset.
- `reset_structure_counters_2`: Clears fields in a structure pointed to by a global pointer and increments certain global counters.
- `decrement_structure_value`: Decrements a value in a structure by a certain amount, where the amount depends on a global variable, and potentially clears a global variable.
- `reset_structure_and_counters`: Clears several fields in a structure, increments a global counter if a certain value is present, and increments two other global counters.
- `apply_mask_on_counter`: Checks a global counter and returns either a masked or full 32-bit mask while decrementing certain global counters.
- `decrement_counters_and_return_mask`: Decrements two global counters and returns either a masked input or a full mask depending on a global counter's value.
- `sort_list_by_key`: Performs an insertion sort on a list of pointers, comparing a value at a specific offset within the pointed-to structures.
- `calculate_vector_interpolation`: Performs complex arithmetic using global variables and pointer-based offsets to calculate a new coordinate or vector component.
- `calculate_complex_value`: Performs a series of arithmetic operations involving a global variable and returns a concatenated 32-bit or 48-bit result.
- `calculate_scaled_vector_offset`: calculates a new 32-bit coordinate or vector component by applying a scaling factor and an offset, using a division-based interpolation or transformation if a certain threshold is met.
- `calculate_scaled_offset_2`: Calculates a new coordinate or vector component using scaling, offsets, and division-based interpolation.
- `return_immediately_stub`: The routine consists of a single return instruction and performs no work, so it acts as an empty placeholder.
- `setup_structure_parameters`: Checks a global counter, and if it is below a threshold, initializes a structure with several constant values and updates a global counter.
- `setup_linked_list_node`: Checks a global value, updates a global pointer, and if a condition is met, initializes a structure with a pointer, a routine, and several flags.
- `clear_memory_range_2`: Clears several global or memory-based variables to zero or a fixed value.
- `initialize_global_variables`: Sets a global variable and several other global memory locations to zero, while setting another global variable to a constant value.
- `reset_global_state`: Clears several global variables to zero and sets one global variable to a constant value.
- `set_function_pointer`: Sets a global variable to the address stored in a pointer and updates another global variable to point to the next memory location.
- `update_pointer_sequence`: Stores a pointer or an incremented pointer into two global variables.
- `set_register_and_global`: Sets a register to a constant value and then clears the least significant bit of that register before storing it back to a global variable.
- `set_constant_values`: Sets a global variable to a constant value and performs a bitwise masking operation on a constant value before storing it back to the same location.
- `set_bit_flag`: Sets a bit in a global variable using a bitwise or operation.
- `clear_bit_flag`: Clears a bit in a global variable using a bitwise and operation.
- `set_memory_values`: Writes a constant value to three consecutive offsets relative to a pointer passed in a register.
- `copy_and_initialize_table`: Copies a block of data from a global variable to a destination table while applying an offset to certain words, then initializes several memory locations with a routine pointer and constant values.
- `update_vector_interpolation_bounds`: Calculates a new interpolated or scaled vector component and updates a global structure or table with the clamped or scaled result.
- `adjust_pointer_offset`: Compares values at an offset from two pointers and updates a value at a pointer offset if a certain threshold is met.
- `update_pointer_offset`: If a register value is less than a certain threshold, adds a constant to a value at an offset relative to a pointer.
- `compute_interpolation_bounds_helper`: Called from update_vector_interpolation_bounds and executed during the reference run, this 7-instruction routine is not an empty stub; its body is not shown in the supplied pseudo-code, which reduces to a bare return, so the exact operation cannot be confirmed from the evidence given.
- `calculate_product_result`: Calculates a product involving a register and a global variable, returning a concatenated result.
- `tiny_move`: Small routine of 1 instruction that copies d0 to -(a0).
- `modify_global_with_bcd_adjustment`: calculates a value based on a masked register input and a global variable, then applies a binary-coded-decimal adjustment routine to the result before storing it back to a global variable.
- `update_global_with_offset`: Stores a value or a modified version of a register into a global variable and performs bcd adjustment on a temporary value.
- `update_global_with_offset_2`: Performs a write to a global variable using a value derived from a register and executes bcd adjustment.
- `tiny_ori_3`: Small routine of 1 instruction that merges 121 into d0.
- `modify_global_with_bcd_adjustment_2`: Takes a value from a register, or-ed with a constant, and performs a series of arithmetic operations and BCD adjustments before storing the result back into a global variable.
- `tiny_ori_4`: Small routine of 1 instruction that merges 2 into d0.
- `update_global_with_raw_value`: Stores a register value into a global variable and performs bcd adjustment on a temporary value.
- `transform_byte_to_bcd`: takes a byte from a global variable, doubles it, and applies a binary-coded decimal adjustment before storing the result back to a global variable.
- `modify_global_with_bcd`: Takes a value from a register, doubles it, and performs BCD adjustment before storing it back to a global variable.
- `update_global_with_bcd_logic`: Performs arithmetic and BCD adjustment on parts of a register value and stores the result in a global variable.
- `transform_byte_to_bcd_complex`: Extracts bytes from a register, performs arithmetic and BCD adjustments, and stores a concatenated result into a global variable.
- `calculate_difference_to_global`: Calculates a difference between register-derived values and stores the result in a global variable, potentially clearing it to zero.
- `update_bcd_value`: Checks a global variable and, if zero, updates a global variable with a value derived from a register and applies a binary-coded-decimal adjustment.
- `initialize_structure_and_list`: Updates a global variable, performs a bcd-adjusted calculation, and if certain conditions are met, initializes a structure and searches a list to insert a routine pointer.
- `process_register_to_globals`: Extracts bytes from a register and stores them into separate global variables while performing bcd adjustment.
- `reset_structure_and_increment_counter`: Increments a global counter if a certain value is present, clears several structure fields, and sets a routine pointer and a constant.
- `tiny_move_2`: Small routine of 1 instruction that copies 2 to d0.
- `update_bcd_value_3`: Doubles a value and performs a BCD adjustment before storing it back to a global variable.
- `increment_global_on_condition`: Multiplies a global variable by two and increments it if certain bit-level conditions are met.
- `setup_blt_parameters_2`: Populates several hardware registers or memory locations with constant values and executes a loop that increments a pointer and a counter while updating a list of transfer addresses.
- `check_pointer_bounds`: Checks if a value pointed to by an offset from a pointer falls within a specific range.
- `tiny_movea_rts`: Small routine of 2 instructions that copies (a7)+ to a0, then returns.
- `check_address_range`: Checks if a value stored at an offset from a pointer falls within a specific range and returns if the condition is met.
- `return_empty_routine`: the routine performs no operations and immediately returns to its caller.
- `modify_byte_and_scan_table`: Sets specific bits in a byte at a pointer and then iterates through a table, calling another routine if a byte matches certain character ranges.
- `process_twenty_text_characters`: Walks twenty characters from a string pointer and calls a helper for each character that falls in certain ranges and is not a space or a full stop.
- `copy_table_rows_from_index`: Copies fourteen rows of four words from a table entry selected by a register value times eight, spreading each row across four positions at fixed offsets and stepping to the next row.
- `populate_table_rows`: Checks a bit in a pointer, then conditionally copies rows from a table and updates a structure field with an offset or a constant.
- `expand_table_data`: Iteratively checks a pointer and a counter, copying table rows and incrementing a counter or updating a structure pointer based on certain conditions.
- `update_blitter_rectangle_list`: Calculates and writes a series of block transfer parameters, including source and destination pointers, to a list of rectangles in chip RAM. The routine updates or initializes rectangle-related registers or memory buffers based on a calculated offset or size.
- `copy_data_with_stride`: If a global variable does not equal a specific value, it copies certain words from a source pointer to a destination pointer using a stride-based loop.
- `copy_data_with_stride_2`: Copies words from a source pointer to a destination pointer using a stride-based loop if a global variable condition is met.
- `copy_strided_quads`: Copies a block of words from a source pointer to a destination pointer using a fixed stride and a loop that processes four words per iteration.
- `return_no_operation_stub`: The routine consists of a single return instruction and performs no work, so it does nothing when called.
- `execute_indirect_function_2`: Calculates an offset into a table of function pointers and executes the routine found at that location.
- `populate_structure_in_loop`: Iterates through a range, populates a structure with a value, a routine, and a constant, and executes a loop that decrements a global variable.
- `set_transfer_address`: Sets a global variable to a specific value and calls another routine multiple times.
- `copy_data_with_stride_3`: Copies words from a source pointer to a destination pointer using a stride-based loop.
- `execute_and_copy_data`: Executes a routine via a function pointer, calls a copy routine, and then copies words from a source pointer to a destination pointer using a stride-based loop.
- `update_buffer_and_sort`: Performs a complex sequence of memory updates, bitwise masking, and sorting a list of pointers by a key.
- `execute_sequence_of_routines`: Executes a sequence of several routine calls, including an indirect call via a global pointer.
- `clear_data_register`: Performs a logical OR operation on a data register with zero.
- `update_buffer_with_bitwise_logic`: Performs bitwise masking and oring operations to update a destination buffer using data from a source buffer.
- `setup_blt_parameters_3`: Calculates a value based on a pointer offset and a global variable, then writes several constant values to a range of memory locations.
- `empty_routine_4`: the routine contains no executable instructions and performs no operations before returning.
- `calculate_blt_bounds`: Performs several range checks and calculations involving a pointer, then writes a calculated value to a global variable and a memory location.
- `initialize_buffer_values`: Sets several specific memory locations to certain constant or calculated values while checking a global variable bit.
- `update_blt_params`: Updates a set of hardware registers and a global variable with a calculated value derived from a table and a register.
- `set_blt_parameters`: Writes specific values to hardware registers and calculates a new value for a global variable using a table lookup.
- `configure_blitter_transfer`: Sets certain global memory or register values to fixed or zeroed constants and calculates a new value for a destination or source pointer or size based on a table lookup and register-based arithmetic.
- `render_blt_strips`: Iteratively checks a range of values and calls another routine if certain conditions or bounds checks pass.
- `render_blt_strip`: Calculates a range of values, updates hardware registers, and loops to update a global variable and a secondary register with a calculated offset.
- `modify_structure_values`: Modifies a short value and an integer value at specific offsets within a structure using a combination of subtraction and addition.
- `set_global_constants`: Sets two global variables to a specific constant value.
- `copy_table_blocks`: Iterates through a table of words and calls another routine if certain bits in the word are set.
- `copy_table_block`: Copies a sequence of bytes from a source table or memory area to a destination area using an offset.
- `scatter_table_values`: Copies bytes from a source pointer to several non-contiguous, fixed offsets in a destination pointer.
- `scan_table_values`: Iterates through a table of words, calling another routine for each word, until a zero or a value below a certain threshold is encountered.
- `render_text_string_to_tiles`: Reads a zero-terminated string and maps each digit and letter to a tile index before drawing it into a 40-byte-stride area.
- `process_blt_chain_2`: Updates a global variable and writes several constant values to a range of memory, then enters a loop that updates a pointer and a global variable.
- `process_blt_chain`: Iteratively updates hardware registers and a global variable while traversing a list of addresses or pointers.
- `update_blt_line_parameters`: Calculates and writes a sequence of block transfer (Blit) parameters, including source/destination pointers and or-patterns, to a table in chip RAM. The routine modifies or-patterns and line-end or-patterns based on a calculated offset or a lookup table, while updating destination or source pointers or-patterns and line-end or-patterns.
- `update_blt_line_parameters_2`: Calculates and writes a seriesed list of block transfer (Blitter) parameters, including source/destination or even/odd line offsets and or-patterns, to a buffer in memory.
- `update_blt_rectangles`: Updates a series of block transfer (Blitter) parameters, including source/destination pointers, or a list of rectangles, by reading or calculating values from a table and writing them to a set of global registers.
- `update_blt_rectangles_2`: Calculates and writes a series of block transfer (Blitter) parameters, including source, destination, and size, to a table in memory.
- `update_blt_line_parameters_3`: Calculates and writes a series or a single line of Blitter (BLT) or Bitplane (BPL) or DMA-related parameters, such as source/destination coordinates and attribute or mode settings, based on a calculated offset and a loop that iterates through a list of memory-stored values.
- `execute_indirect_call_4`: Calculates a target address using a table lookup and a pointer offset, then performs an indirect call if the target is within a certain range.
- `execute_indirect_call`: Calculates an offset from a pointer-based value and executes a function call at a calculated address if certain range checks pass.
- `resolve_indirect_call_target`: Calculates a target address from a table using an offset derived from a value in a structure, then performs a conditional indirect call if the resulting index or bounds-check passes.
- `execute_indirect_call_2`: Performs a conditional indirect call to a function pointer located in a table, based on a value offset from a pointer.
- `execute_indirect_function`: Checks if a value from a pointer-based structure is within a certain range, calculates an index, and calls a function pointer from a table.
- `execute_indirect_call_3`: Uses a value from a pointer-based structure to index into a table and calls the resulting function pointer.
- `dispatch_indirect_function_pointer`: Calculates an offset or index from a structure member and uses it to look up a function pointer in a global table, then executes that function.
- `modify_structure_value`: Updates a value in a structure by adding or subtracting a bit-shifted version of a global variable.
- `empty_routine_5`: The routine contains no executable instructions and performs no operations.
- `update_pointer_list`: Updates a pointer in a structure, increments a global counter, and modifies a pointer-based list or queue.
- `reset_structure_and_counters_2`: Resets fields in a structure, increments a global counter if a condition is met, and updates a pointer or table entry.
- `adjust_pointer_offset_2`: Increments a value in a structure, assigns a value from a global table, and then modifies a short value in a structure by adding or subtracting a halved register value.
- `apply_offset_to_structure`: Adds a value to a short field in a structure and updates an integer field in a structure with an offset, then reverses the change.
- `apply_and_revert_offset`: adds a value to a halfword and a constant to a dword via a pointer, then subtracts those same values to restore the original state.
- `calculate_blt_parameters`: Calculates and writes scaled or offset values to a global variable and a structure, then iterates through a range to update further values.
- `setup_blt_parameters`: Calculates source and destination addresses and certain hardware parameters, then writes them to specific memory locations.
- `configure_blt_hardware`: Sets up hardware registers for a block transfer, including source, destination, and size, while checking certain range constraints.
- `validate_and_scale_value`: calculates a scaled value stored in a global variable by dividing a constant by a value from a pointer, then checks if a certain offset or a sum of offsets falls within a specific range.
- `calculate_scaled_value`: Calculates a value by dividing a constant by a value from a structure, then checks if a certain range or threshold is met.
- `update_blt_parameters_2`: Calculates a scaled value and updates several memory locations, including Blitter hardware registers, based on a certain range check.
- `initialize_structure_fields`: Writes several constant values and a value from a global variable into a structure located at an offset from a pointer.
- `set_multiple_structure_values`: Sequentially updates several fields in a structure with different values from a global variable or constants.
- `update_blt_registers`: Updates a sequence of hardware blt registers and a pointer-based list with incrementing addresses and values.
- `update_global_thresholds`: Checks if a global value is within a specific range and, if so, resets or updates other global variables to certain constant values.
- `set_global_thresholds`: Checks if a global variable meets certain criteria and, if so, sets a global variable to a specific constant or a certain value.
- `populate_blt_registers_2`: Updates a pointer and certain memory locations, then executes a loop that writes a sequence of incrementing addresses and values to memory.
- `populate_blt_registers`: Iteratively writes a sequence of addresses and certain constant values into hardware registers and memory locations.
- `check_list_flags`: Iterates through a list of pointers and executes another routine if certain bitflags are set in the structure.
- `increment_structure_offset`: Checks if certain values in a structure are within a range, then increments a value in that structure by a constant.
- `decrement_structure_counter`: Checks if a value in a structure is within a range, then decrements a value in that structure and checks if it becomes negative.
- `set_single_global_value`: Sets a global variable to a specific constant value and calls another routine.
- `setup_blt_transfer`: Calculates a transfer address and populates hardware registers with a sequence of values, including a loop that updates a pointer-based list.
- `initialize_memory_blocks_2`: Iteratively writes specific patterns to memory blocks and then initializes several fields in a structure with constant values or values from a pointer.
- `test_and_branch_on_address_bits`: Checks specific bit patterns within the routine's own memory address and executes a sequence of conditional or unconditional calls to itself or other routines based on those bit tests.
- `update_global_with_bcd`: Performs a write to a global variable using a register-derived value and executes a BCD adjustment.
- `update_structure_with_sum`: Calculates a sum of a register and a value, then writes it to a structure along with several other constant or register-based values.
- `set_structure_with_offset`: Writes several constant values to a structure and then updates a field in that structure with a value plus an offset.
- `set_table_parameters_2`: Checks bits of a global variable and calls other routines to update a table with new values or reset table entries to fixed constants.
- `update_register_values`: Adds a value from a memory location to a register and stores the result in a table, followed by clearing several table entries with fixed values.
- `set_table_parameters`: Stores a value into a table and resets several table entries to zero or fixed constants.
- `calculate_table_offset`: Calculates a new value by adding a constant to a value fetched from a table and then populates a table with fixed constants.
- `copy_disjoint_memory_blocks`: Copies or moves data between two memory buffers using a loop that handles non-contiguous or offset-based memory access.
- `copy_word_quads_with_stride`: Copies four-word groups from a source into a destination with a fixed stride between words, looping over a count and moving to the next source row on each pass.
- `clear_program_start_block`: Zeroes about 16,193 consecutive longwords starting from the program's start label.
- `return_without_work`: Returns immediately without doing any work.
- `return_immediately_stub_2`: The routine consists of a single RTS instruction and performs no work: it neither reads nor writes any registers, memory, or hardware, and makes no library calls.
- `return_from_gap_stub`: Returns immediately without doing any work.
- `return_as_callback`: Returns immediately without doing any work.
- `return_after_unrecovered_call`: Returns at once.
- `store_handler_pointer`: Stores the address of one routine into a handler slot in the shared data area and returns.
- `tick_counters_and_call_handler`: Increments two counters, calls a stored handler routine when one is set, then calls two helpers.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 117 into d0, then returns.
- `build_color_table_from_source`: Calls one helper, then fills sixteen entries in two tables.
- `copy_scaled_color_entries`: Fills sixteen entries in two tables from a source pointer, each entry twice the source word plus 273.
- `copy_table_elements_2`: Copies a fixed number of 32-bit or 4-byte words from a source table to a destination table.
- `update_pointer_and_table_entry`: Updates a global pointer to the next entry in a sequence and writes a masked value from a pointer-based source into a table indexed by a bit-shifted value from that same pointer.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `rotate_global_values_2`: Performs a circular shift or swap of several global or nearby memory-resident values.
- `empty_loop_routine`: The routine contains no reachable instructions and consists of an infinite loop that never executes any operation.
- `load_color_registers_from_table`: Writes sixteen colour register values from a source table, each twice the source word plus 273, and returns the input register unchanged.
- `point_copper_at_state_table`: Points the copper list address register at a state table, then clears a second custom register value.
- `build_color_table_point_copper`: Fills sixteen table entries with twice a source word plus 273, then points the copper list address register at the table and clears a second register.
- `fill_row_offsets_table`: Fills 200 word entries with a sequence that starts at minus 320 and rises by forty each step, a row-stride pattern for a forty-byte-wide area.
- `run_timed_display_loop`: Runs an endless loop of two counted phases, each up to 400 passes, calling helper routines on every pass.
- `execute_main_loop`: Runs a nested loop that calls several subroutines, updates a counter, and checks a bit in a global variable to decide whether to exit or continue.
- `draw_grid_rows_with_helpers`: Runs a nested loop over 200 rows and four columns of twenty steps each, calling helper routines for every cell and moving the target by a computed offset on each row.
- `process_memory_blocks`: Performs a nested loop that copies bytes from a fixed memory address while skipping certain markers and updating a pointer.
- `render_score_and_bonus_text`: Formats a numeric value into a character and renders a string of text or score-related labels to tiles.
- `draw_score_table_labels`: Positions a table, then references the four text labels NAME, RANK, SCORE and START. It runs a loop of twenty rows, calling helper routines for each row.
- `decrement_and_render_text`: Decrements a global counter and, if it reaches zero, resets it and calls a routine to render text strings if certain character conditions are met.
- `get_joystick_state`: Calls a routine to update a bitmask and returns the current value of a global variable.
- `update_font_or_string_buffer`: Checks bit flags in a global variable to increment or decrement a counter, then copies a byte from a table into a buffer using a pointer.
- `clear_data_register_twice`: Performs two consecutive logical OR operations on a data register with zero.
- `process_table_data`: Calculates a starting address based on a global value and iterates through a memory range to copy table rows if certain character criteria are met.
- `insert_into_sorted_table`: Calls two helpers, then walks backwards through a table of records, moving each entry that is not greater than the stored value one slot along and swapping the field blocks.
- `format_longword_as_digits`: Converts a 32-bit value into eight characters, one per nibble, using a sixteen-character digit table, then appends spaces.
- `update_clamped_values`: Updates certain global values using addition and bitwise masking, while checking if the new values fall within a specific range or if a bit is set.
- `copy_byte_skipping_run_marker`: Reads bytes from a source pointer, skipping every value of 128, then stores the next byte into a destination.
- `initialize_system_state`: Initializes several tables and hardware pointers, then writes a sequence of values from registers to a block of memory.
- `update_blt_source_table`: Calculates and writes a series of modified values into a memory table, then configures DMA or copper-related pointers and checks for specific joystick or flag-based exit conditions.
- `copy_struct_array`: copies a fixed number of multi-byte or word-sized values from a source table to a destination table using a pointer-based loop.
- `copy_backwards_array`: copies a fixed number of long-word-sized values from a source table to a destination table while decrementing the destination pointer.
- `setup_blt_transfer_2`: Calculates a source address based on a table offset and a constant, then calls another routine multiple times with that address.
- `repack_byte_to_long_table`: rearranges bytes from a source buffer into a destination table by packing or unpacking four bytes into a single longword.
- `execute_indirect_table_entry`: Calculates an offset into a table of function pointers and executes the resulting address.
- `execute_indirect_call_5`: Fetches a function pointer from a table using an offset and executes it.
- `permute_data_buffer_indices_2`: Performs a series of loops that copy and permute 16-bit words from a source pointer to a destination pointer using a fixed stride and a non-sequential index pattern.
- `permute_and_expand_data_blocks`: Performs a series of nested loops that copy and permute 16-bit words between memory buffers using fixed strides and hardcoded index remappings.
- `permute_data_blocks`: Performs multiple loops that copy and permute 16-bit words from a source pointer to a destination pointer using a fixed stride.
- `permute_sprite_attribute_buffer`: Performs a series of loops that copy and permute 16-bit words from a source pointer to a destination pointer using a fixed stride and a hardcoded index remapping pattern.
- `permute_data_buffer_blocks`: Performs a series of loops that copy and permute 16-bit words from a source pointer to a destination pointer using a fixed stride and a hardcoded index pattern.
- `permute_data_blocks_4`: Executes multiple loops that copy and reorder 16-bit words between source and destination pointers using fixed strides and hardcoded patterns.
- `permute_data_blocks_fixed`: Performs a single loop that copies and reorders 16-bit words between source and destination pointers using fixed strides.
- `permute_and_copy_data`: Performs loops that permute 16-bit words and a final loop that copies 16-bit words from a source pointer to a destination pointer.
- `permute_data_blocks_2`: Performs a loop that copies and permutes 16-bit words from a source pointer to a destination pointer using a fixed stride.
- `permute_data_block_stride`: Copies and reorders a sequence of 16-bit words from a source pointer to a destination pointer using a fixed stride.
- `permute_data_blocks_3`: Performs a series of loops that copy and reorder 16-bit words from a source pointer to a destination pointer using fixed strides and hardcoded index offsets.
- `reorder_data_blocks`: Performs a series of loops that copy and permute 16-bit words from a source pointer to a destination pointer using a fixed stride.
- `reorder_data_blocks_2`: Copies and permutes 16-bit words from a source pointer to a destination pointer using a fixed stride.
- `permute_data_buffer_indices`: Iterates through a buffer, reordering or copying words from a source pointer to a destination pointer using a fixed sequence of non-sequential index offsets.
- `copy_data_blocks`: Copies 16-bit words from a source pointer to a destination pointer using a fixed stride.
- `copy_data_words`: Copies 32-bit words from a source pointer to a destination pointer using a fixed stride.
- `interpolate_vector_data`: Initializes a block of memory to zero and then executes a loop that calculates and stores interpolated or scaled vector-like values into a destination buffer.
- `copy_data_block`: copies a block of 55 or 56 words from a source table to a destination buffer using an unrolled loop.
- `copy_data_blocks_3`: Performs a loop that copies 16-bit words from a source pointer to a destination pointer using a fixed stride.
- `copy_interleaved_tile_rows`: Copies a fixed grid of 16-bit words from one buffer into another in a nested pattern: four outer passes over 24 inner steps, each step writing four words scattered across three destination slots while reading strided source words, and advancing the destination by a fixed stride after each outer pass.
- `copy_data_blocks_2`: copies a series of data blocks from a source table to a destination buffer using a nested loop structure.
- `unroll_and_copy_table`: copies a block of data from a source table to a destination buffer using a nested loop structure with a fixed stride.
- `rearrange_data_block`: copies and redistributes a block of data from a source table to a destination buffer using a nested loop structure that modifies the source pointer offset and destination pointer or pointer-to-pointer logic.
- `scan_and_copy_nested`: Executes nested loops that call a routine to scan memory buffers for a sentinel byte and copy a following byte to a destination.
- `populate_string_buffer_nested`: Performs a triple-nested loop that repeatedly calls a string-copying routine to write bytes or strings into a memory buffer, incrementing the source pointer and decrementing a loop counter.
- `call_helper_with_endless_guard`: Calls one helper routine.
- `iterate_string_copy_buffer`: Performs a triple-nested loop that repeatedly calls a string scanning and copying routine.
- `scan_and_copy_string`: Performs nested loops that call a routine to scan a memory buffer for a sentinel byte and copy the following byte to a destination pointer.
- `find_string_sentinel_and_copy`: scans a memory buffer for a byte that does not have its high bit set, then copies the byte following that found byte to a destination pointer.
- `compute_grid_position_loop_b`: Splits a stored position into a quotient and remainder by eleven, and when a second value is nonzero and the sum stays below 1,761, runs a loop that subtracts a clamped count from a frame word, calling six helper routines on each pass.
- `write_disk_sync_retry_b`: Clears three frame words and retries up to three times.
- `run_disk_dma_retry_b`: Calls a helper, then when a flag bit is set programs the disk DMA pointer and length and enables disk DMA through the control registers.
- `retry_status_check_update_record`: Retries a status check up to ten times, calling a poll routine on each pass.
- `scan_bitmap_rows_for_match`: Walks up to eleven rows of a table, testing a status bit and a column bit mask for each entry in range.
- `empty_stub_routine`: It performs no observable work and simply returns.
- `set_bit_and_bump_counter`: Sets one bit in a 16-bit flag word inside a record, with the bit chosen by the low five bits of a register value.
- `clear_strided_long_words`: Writes zero to ten 32-bit values spaced 272 bytes apart, starting at the address held in an address register and walking forward.
- `extract_high_byte_pair`: Calls another routine and returns a value whose low word is the lower 16 bits of that result and whose high word is the top byte of that result.
- `merge_alternating_bit_masks`: Reads two consecutive 32-bit values, keeps only the alternating bit pattern of each, and merges them so the first is shifted left by one bit before being combined with the second.
- `call_and_forward_result_2`: Calls one routine and passes its return value directly into a second routine as its argument.
- `xor_fold_fixed_block`: XORs ten consecutive 32-bit values from a block addressed relative to a base register, then keeps only the alternating bits of the result.
- `xor_checksum_block_masked`: XORs a run of 32-bit values from a source pointer, with the count derived from a size argument, then keeps only the alternating bits of the result.
- `fill_block_alternating_pattern`: Writes an alternating bit pattern into a block of 256 long words whose start address comes from a global variable.
- `compute_record_bounds`: Calls a routine and uses its result together with two register outputs to compute three values.
- `set_record_transform_fields`: Fills in several fields of a record pointed to by an address register, using values returned from other routines.
- `wait_for_global_flag_clear`: Loops while bit six of a global variable is set and returns once that bit clears.
- `reset_blitter_registers`: Sets a global flag word, then writes masks, modulo values, a blitter data constant and zeros to the blitter registers.
- `mix_then_update_top_bit`: Calls two routines first, then reads a byte and the flag byte just before it.
- `update_top_bit_from_flag`: Reads a byte and the flag byte just before it.
- `bitwise_mix_store_word_2`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `start_disk_dma_read`: Writes a sync word and a buffer pointer to the disk registers, then starts a DMA transfer.
- `check_transfer_status_and_stop`: Checks a status bit in a device record.
- `set_disk_control_words`: Writes two control words to a global variable in sequence, leaving the second value in place.
- `set_global_all_ones`: Writes a constant to a global variable, then overwrites it with an all-ones byte.
- `set_bit_from_table_index`: Reads a 16-bit value from a table through a base register, adds three, and uses the low five bits as a bit index.
- `set_global_ones_call_pair`: Writes all ones to a global variable, then calls two routines in sequence.
- `delay_by_table_difference`: Reads a word from a global table selected by a frame variable and compares it with a value passed in a data register.
- `wait_status_then_clear_entry`: Polls bit four of a global status byte, calling a routine between checks.
- `store_port_status_byte`: Calls a routine to read a byte, clears bit one of it when a secondary output is non-negative, forces bit zero on, and stores the result in a global variable.
- `store_result_to_global`: Calls a routine and stores its returned value in a global variable.
- `compute_port_bit_mask`: Derives a byte from a global variable by clearing one bit chosen by a table index, with the bit position offset by three.
- `wait_until_routine_returns_one`: Calls a routine repeatedly and waits for the lowest bit of a global flag to clear between calls, stopping once the routine returns one.
- `write_fixed_values`: Checks a bit in a global variable and, if certain conditions are met, writes three fixed values to that global variable.
- `write_three_control_values`: Writes three fixed control values, in order, to a global variable, leaving the last one in place.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `handle_pending_request_state`: Checks a pending request value.
- `resume_pending_request_state`: A near duplicate of the adjacent pending-request handler.
- `handle_pending_request_entry`: Shares the same pending-request logic as its neighbouring handlers, including the same branches and command writes.
- `program_audio_channels_dma`: Programs audio channel pointers, lengths and periods, then enables DMA for the channels it configures.
- `load_long_from_offset_table`: Returns a 32-bit value read from a fixed address, offset by a word held in a data register.
- `call_two_routines_in_order`: Calls two routines one after the other and returns.
- `advance_channel_sequence_step`: Advances one channel through its sequence: it counts down the channel's timers, moves to the next data item, adjusts an output value, and writes it into the channel state.
- `start_channel_sequence_output`: Initialises a channel record from a source pointer, then computes and stores an output word from a sequence entry with an offset from the record.
- `empty_routine`: the routine contains no executable instructions and performs no operations.
- `call_initialize_channel_tables`: Call stub: calls initialize_channel_tables as a subroutine and returns to its own caller when it returns.
- `update_channel_sequence_pointer`: Updates a sequence pointer or index within a structure, potentially adjusting a pointer or a counter-based offset based on a value read from a table.
- `update_channel_sequence`: Checks a pointer in a structure and updates a sequence or pointer or a global variable based on a value in a table.
- `advance_sequence_pointer`: Updates a structure or pointer using a value from a table and updates a global variable.
- `load_sequence_data`: Copies data from a source pointer into a structure and updates a sequence or pointer or a global variable.
- `initialize_sequence_structure`: Sets certain fields in a structure and updates a sequence or pointer or a global variable.
- `set_sequence_index`: Sets a field in a structure to zero and updates a sequence or pointer or a global variable.
- `advance_channel_sequence_step_2`: Updates a pointer or index within a sequence or table, decrementing a counter and checking for a wrap-around or a specific end-of-sequence condition.
- `clear_request_state`: Sets a global variable and certain state flags to zero or certain values.
- `reset_channel_period_state`: Sets a global flag to one, writes a period value passed in a data register to a global variable, clears a per-channel field, and clears a state variable.
- `clear_state_and_set_period`: Sets a global flag to one, clears a shared state variable, writes a period value from a data register to a global variable, and clears a per-channel field.
- `load_channel_sequence_entry`: Sets a flag in a channel record, reads the first sequence entry from a source pointer, and stores its output word.
- `reset_audio_channel_defaults`: Writes fixed default values to four sets of channel fields, with period entries taken from a table, then clears the shared state variables.
- `initialize_channel_tables`: Clears a global block of 205 words, loads four pointers from a table selected by an index, points eight channel fields at the same block, and writes six fixed index values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointers_and_values`: Updates a word at a certain offset and a pointer to a new address, repeating this for several different addresses, while waiting for a flag bit to clear.
- `set_pointer_addresses`: Sets two different pointers to the same address and updates a word at a certain offset, repeating this for several different addresses, while waiting for a flag bit to clear.
- `initialize_pointer_values`: Sets a pointer to a new address and clears a word at a certain offset, repeating this for several different addresses, while waiting for a flag bit to clear.
- `set_word_to_constant`: Sets a word at a certain offset to a constant value and updates a pointer to a new address, repeating this for several different addresses, while waiting for a flag bit to clear.
- `update_structure_and_call_subroutine`: Checks bit 6 of a value pointed to by a5; if set, it updates two pointers and a 16-bit word in a structure at a6, rotates a 16-bit value in d3, and conditionally calls another routine and modifies a0 before entering a loop.
- `copy_table_elements`: Calculates a source pointer from a table and a register, then copies a large number of bytes from that source to a destination offset from a global variable.
- `scatter_table_elements`: Copies a series of bytes or words from a source pointer to various non-contiguous offsets in a destination buffer.
- `iterate_sequence_elements`: Iterates through a list of values, calling itself recursively if a value is within a certain range.
- `empty_routine_2`: The routine contains no executable instructions and performs no operations before returning.
- `update_blt_parameter_table`: Calculates and writes a sequence of source and destination coordinates and masking parameters to a global table.
- `configure_blt_transfer`: Configures certain memory or hardware registers using a calculated offset and a loop that calls a transfer routine.
- `update_blt_parameters`: Calculates and writes a sequence of source and destination coordinates and or/masking parameters to a global table.
- `update_global_pointers`: Repeatedly updates a global variable with a value from a register and sets a corresponding pointer in a global structure to a new address.
- `call_subroutine`: executes another routine and returns.
- `configure_blt_and_set_constants`: Sets memory constants and initializes or updates hardware or memory registers with pointer and value data before executing a drawing routine.
- `set_memory_constants`: Sets certain memory locations to constant values based on a conditional check.
- `update_blt_parameters_and_execute`: Updates a set of hardware or memory registers with new pointer and value data before calling a drawing routine in a loop.
- `configure_blt_and_draw`: Sets several hardware registers to specific values and then enters a loop that calls another routine to perform a drawing or copying operation with updated source and destination pointers.
- `iterate_and_update_table`: Iterates through a list of pointers and checks certain bits in a byte at an offset, calling another routine if certain bits are set.
- `initialize_blt_parameters`: Sets several memory locations to constant values and calculates a value based on a pointer and a table, then executes a loop that writes certain values to memory.
- `initialize_memory_blocks`: Iterates through a series of memory blocks, setting specific offsets to certain constant values.
- `call_external_routine`: Executes another routine and returns.
- `call_subroutine_2`: This routine executes another routine and then returns.
- `call_subroutine_3`: calls another routine and returns.
- `call_stub`: Call stub: calls a routine at a fixed address outside this code unit, then returns to its own caller.
- `forward_call_to_target`: The routine consists of a single unconditional call to one other function followed by return, with no arguments set up, no local state, and no hardware register access.
- `copy_data_with_strides`: Performs a nested loop that copies or redistributes words from a source buffer to a destination buffer using a fixed stride or offset.
- `copy_data_with_variable_stride`: Performs a nested loop that copies or redistributes words from a source buffer to a destination buffer where the destination offset or stride is determined by a register.
- `clear_memory_range`: Fills a range of memory starting from a specific label with zeros.
- `empty_routine_3`: the routine is empty and performs no operations.
- `rotate_global_values`: Swaps or rotates values between a set of global variables.
- `empty_loop_routine_2`: The routine contains no reachable instructions and consists of an infinite loop that never executes any operation.
- `update_color_table`: Calculates a new value by doubling a source value and adding a constant, then stores it into a table.
- `set_pointer_and_clear_register`: Stores the address of a global variable into itself and clears a custom hardware register.
- `initialize_state_table_and_wait_for_input`: Initializes a table of values derived from a global pointer and updates state-related global variables before entering a loop that monitors a joystick or input bit.
- `zero_memory_loop`: Iteratively writes zeros to memory starting at address zero.
- `copy_memory_block`: Copies a block of data from one memory location to another, followed by a final write of two specific values to the end of the destination range.
- `copy_memory_block_2`: Copies a fixed number of words from a source pointer to a destination pointer, then updates a global variable with a value from a memory location.
- `calculate_address_and_call`: Calculates a new address by adding an offset and a pointer value, then calls a routine multiple times with that address.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_empty_return`: Does nothing and returns at once.
- `stub_single_instruction_entry`: A single-instruction entry that was executed during the reference run.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately`: the routine performs no operations and returns immediately.
- `return_immediately_2`: The routine is empty and performs no operations before returning.
- `return_to_caller`: the routine is empty and performs no operations before returning.
