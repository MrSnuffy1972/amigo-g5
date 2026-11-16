# Game 672 (Toki): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00672). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (967840 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 391 raw-word lines |
| Functions named and described | pass, 432 of 432 named (0 author symbols), 432 of 432 described (91 by rule, 332 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5245 of 5245 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 130388 bytes, 20 functions |
| Independent decoder cross-check (Ghidra against capstone) | 12780 of 12784 instructions identical in start and length, 2 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 17980 |
| code | 48036 |
| data | 842880 |
| padding | 64 |
| unused | 58880 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Toki.adf:bootblock | 1 | 18 | 1 |
| logged disk load from Toki.adf boot-block load | 1 | 163 | 2 |
| logged disk load from Toki.adf boot-block load | 1 | 73 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 171 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 345 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5092 | 178 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4147 | 161 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2098 | 51 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 1 |
| Toki.adf:blocks 22-133 (bytes 11264-68608)#unpacked (raw code file, load address unknown) | 1 | 654 | 20 |

## Cases handled

- Toki/Toki.adf: custom-format (non-DOS) disk, 1643 non-empty blocks: matched against track-loaded segments
- Toki/Toki.adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- Toki/Toki.adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 139 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 330 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2145 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2098 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 513 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 5 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate, 1 displacement, 1 operand, 1 branch displacement
- Toki/Toki.adf: 2 logged loads cover 6656 bytes of the image
- Toki/Toki.adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- Toki/Toki.adf: blocks 4-10 (bytes 2048-5632) outside the filesystem hold data, not code (opcode density 0.017, entropy 5.83)
- Toki/Toki.adf: blocks 22-133 (bytes 11264-68608) hold a Ice: Pack-Ice v2.0 - v2.20 stream (57344 -> 130388 bytes)
- Toki/Toki.adf:blocks 22-133 (bytes 11264-68608)#unpacked: code-like unpacked track stream no run loaded verbatim: recovered position-independently (load address unknown)
- Toki/Toki.adf: blocks 136-179 (bytes 69632-92160) outside the filesystem hold data, not code (opcode density 0.017, entropy 4.42)
- Toki/Toki.adf: blocks 1742-1759 (bytes 891904-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.87)
- 1 never-loaded region(s), 130388 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 740352 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Toki.adf:bootblock

- `load_segments_and_patch_code`: Reads two disk segments into memory with device I/O and copies a block of data to a second buffer.

### logged disk load from Toki.adf boot-block load

- `emit_word_and_shift_bitstream`: Reads a longword from the stream behind the input pointer, stores a 16-bit word combining its low byte with the high byte of a register, and returns the longword shifted right with its top bit set.
- `shift_bit_buffer_by_count`: Shifts a bit buffer right one bit per loop iteration for the count held in a register.

### logged disk load from Toki.adf boot-block load

- `frag_moveq_move_dbra_jmp`: Short code fragment that loads 64 into d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then jumps to another point in the code.
- `frag_jsr_move_jmp`: Short code fragment that calls another routine, then copies an address to a fixed memory address, then copies 14336 to a fixed memory address, then jumps to another point in the code.
- `frag_move_cmpi_beq_lea_bra`: Short code fragment that copies 17733 to a fixed memory address, then compares a fixed memory address with 17733, then branches if the last result was zero, then loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_move_cmpi_bne_lea_bra`: Short code fragment that copies 17733 to a fixed memory address, then compares a fixed memory address with 17733, then branches if the last result was not zero, then loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_move_cmpi_bne_lea_bra_2`: Short code fragment that copies 17733 to a fixed memory address, then compares a fixed memory address with 17733, then branches if the last result was not zero, then loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_move_rts`: Short code fragment that copies 24580 to a fixed memory address, then copies a0 to a fixed memory address, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_hardware_status`: Decrements two counters in a loop and checks bits 6 and 7 of a hardware register, potentially writing to a global variable and branching based on a comparison with a global word.
- `update_timer_logic`: Calls another routine and then modifies a global variable and a 16-bit counter based on conditional checks and comparisons.
- `populate_address_table`: Calculates four 32-bit values by adding a constant to a value retrieved from a global variable and stores each 32-bit value into a global variable as two separate 16-bit words.
- `update_circular_buffer`: Decrements a 16-bit counter in a global variable and, if it reaches zero, performs a series of word transfers between global variables and a table, then updates a pointer with a mask.
- `check_two_counters`: Tests a word at a global address and calls another routine if it is zero, then tests a second word at a different global address and calls another routine if it is zero.
- `copy_string_table`: Decrements a global variable and, if it reaches zero, copies fifteen words from a source pointed to by a register to a destination at a fixed address, then calls another routine.
- `decrement_and_set_register`: Decrements a 16-bit global variable and, if it reaches zero, sets register d5 to one.
- `process_bitmask_table`: Iterates through a table of 32 words using nested loops to perform conditional subtractions or bitwise masking based on values in d5, then increments a global 16-bit counter and resets it along with two other global variables when the counter reaches 16.
- `initialize_data_structures`: Sets several global status bytes and updates structure members via pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `advance_row_load_slot_timers`: When a flag byte holds a space character, it advances a row index modulo 24, loads three slot pointers and timer values from a 14-byte table entry and sets their active flags.
- `decode_control_record_fields`: When the first byte of a record is nonzero it stores values derived from a lookup table into the state block.
- `interpret_sequence_command_stream`: Reads opcodes from a byte stream: one opcode loads fields from a lookup table and sets flags, another sets a value from its parameter, and a terminating opcode ends processing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_process_data`: Copies a block of bytes from a source at a4 to a destination, then enters a loop that calls another routine to process values from a table, swaps the high and low words of a register, and copies bytes from a5 to a6.
- `rotate_and_loop`: Performs a loop four times, rotating a byte from a stack-based pointer and returning.
- `check_conditional_bits`: Returns immediately if the 31st bit of a 32-bit value in d7 is set or if the 0th bit of a 16-bit value in a5 is clear; otherwise it decrements a pointer in a5, loads a 32-bit value, adds it to d7, sets bit 0 of d7, and returns.
- `rotate_and_copy_bytes`: Performs a loop that shifts a 32-bit value in d7 left by one bit each iteration, potentially fetching a new byte from a pointer in a5 based on bit 0, and calls another routine multiple times to copy bytes from a table into the stack.
- `configure_blt_and_dma_parameters`: Configures Blitter and DMA controller registers, clears a series of structures in a loop, and populates a table with specific hardware configuration values including bitplane and DMA channel settings.
- `initialize_game_state_and_input`: Initializes several global structures and tables by clearing specific fields and copying data, then processes joystick input by checking hardware registers and updating a status byte.
- `dispatch_via_table_2`: Calls four routines, then performs a series of comparisons on a global byte to update a global word and several bytes, and finally enters a loop that clears bytes in a structure and calls a routine.
- `update_structure_values_3`: Tests a byte at a global address and, if zero, performs several comparisons between values in a structure pointed to by a6 and a0, then updates multiple global bytes and words and modifies a value at an offset of a5 before calling another routine.
- `frag_btst_beq_subi_move_clr_rts`: Short code fragment that tests bit 3 of d0, then branches if the last result was zero, then subtracts 24 from the memory at an offset from a5, then copies 65530 to a fixed memory address (2 times in a row), then copies 1 to a fixed memory address, then clears a fixed memory address, then returns.
- `combine_masked_values`: Loads two 32-bit values from a pointer in a0, masks both with a value in d6, doubles the second masked value, and returns the bitwise OR of the first masked value and the doubled second value.
- `update_structure_values_2`: Calculates new values based on offsets from a structure in a0 and several global words, calls another routine, and updates two fields in a structure using the results.
- `resolve_indirect_jump`: Loads a base address into a6, modifies a value from a global variable, uses it to index into a table to update a6, loads a word from that table into d4, and performs a jump to an offset determined by d4.
- `branch_to_offset_3`: Performs a relative branch to an offset.
- `validate_and_call_structure_member_2`: Performs several range checks on values within a structure and updates a global variable, then calls another routine using an offset from the structure.
- `jump_to_modify_and_call_indirect`: Jump stub: passes control straight on to modify_and_call_indirect.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `branch_to_offset_f5c`: Performs a relative branch to a specific offset.
- `branch_to_offset_f34`: Performs a short branch to a specific instruction located at the offset f34 within the current code segment.
- `validate_and_execute_callback_2`: Compares four values from a structure in a6 against registers, and if all conditions are met, modifies two global bytes and calls another routine, otherwise it performs bounds checks and potentially calls a routine via a pointer in the structure.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `jump_to_store_and_clear_globals`: Jump stub: passes control straight on to store_and_clear_globals.
- `execute_indirect_jump_3`: Performs multiple range checks using values from a stack-based structure, calls load_and_call_routines, and then performs an indirect jump using an offset from a structure in a6.
- `update_pointer_and_return`: Adds a longword from a pointer in a0 to d0 and returns.
- `modify_and_call_indirect`: Checks four values from a structure in a6 against registers and, if conditions are met, clears a global byte, sets a bit in another global byte, updates two global words, and calls a routine using an offset from a6.
- `check_bounds_and_call`: Compares four values from a pointer in a6 against registers, and if all conditions are met, modifies two global bytes and calls another routine via an offset from a6.
- `check_bounds_and_jump`: Compares values from a stack-based structure against several arguments, and if all conditions are met, clears a global byte, sets bit 2 of a global byte, increments a value in a global word, and calls another routine via an indirect jump.
- `check_bounds_and_jump_alt`: Compares values from a stack-based structure against several arguments, and if all conditions are met, clears a global byte, sets bit 0 of a global byte, increments a value in a global word, and calls another routine via an indirect jump.
- `validate_and_call_structure_member`: Checks if a 16-bit value from a table falls within a specific range defined by several parameters, updates a global variable and a structure with calculated offsets and flags based on that range, and then calls a routine using an offset stored in the table.
- `store_and_clear_globals`: Compares four values from a structure in a6 against registers and, if conditions are met, stores a long value to a global address and clears a global byte.
- `calculate_offset_and_jump`: Performs multiple range checks using values from a stack-based structure, calculates a new value using arithmetic on d4, a1, d5, and constants, and if specific conditions are met, updates several global bytes and words before calling another routine via an indirect jump.
- `load_and_call_routines`: Saves registers to the stack, loads a longword from a global address into d2, loads a longword from another global address into d0, and calls two different routines before branching to another location.
- `calculate_table_pointers`: Calculates several pointer values by adding offsets and register values to pointers retrieved from a structure in a0, clears a global byte, and calls another routine.
- `tiny_eor_rts_rts`: Small routine of 3 instructions that exclusive-ors d1 into d0, then returns, then returns.
- `tiny_move_rts`: Small routine of 2 instructions that copies (a0)+ to d1, then returns.
- `execute_indirect_jump_2`: Calculates an offset using a value from a global variable, loads a pointer from a table at that offset into a6, and performs an indirect jump to an address in that table.
- `branch_to_offset`: Performs a short branch to a specific instruction within another routine.
- `test_collision_and_dispatch_handler`: Performs a 2D bounding box intersection test using values from a table pointed to by a6 against registers a2, a3, and a4, sets a global variable to 1 if an intersection is detected, and calls another routine using an offset stored in the table at a6 plus 16 bytes if the value at that offset is non-negative.
- `jump_to_validate_bounds_and_jump`: Jump stub: passes control straight on to validate_bounds_and_jump.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `jump_to_shared_tail`: This routine consists of a single unconditional word-displacement branch (bra.w) that transfers control to a point inside another routine, so it performs no work of its own.
- `branch_to_collision_handler`: Performs a short branch to another routine that follows a collision detection test.
- `validate_and_execute_callback`: Compares four 16-bit values from a pointer in a6 against registers a1, a2, a3, and a4 to ensure they fall within specific bounds, then sets a byte in a global variable if the conditions are met.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `update_bounds_and_jump_2`: Checks several range conditions against a structure in a6, conditionally sets a global byte to 1, and performs an indirect jump using an offset from a6.
- `validate_bounds_and_jump`: Restores d4-d7 from a pointer in a6, compares these values against a1, a2, a3, and a4, sets a global byte and a bit in d3, and then performs an indirect jump based on a value from the stack.
- `check_bounds_and_set_byte`: Compares four values from a pointer in a6 against registers, and if all conditions are met, sets a byte in a global variable and calls another routine via an offset from a6.
- `set_status_and_jump`: Compares values from a stack-based structure against several arguments, and if all conditions are met, sets a global byte to 3 and sets bit 2 of d3 before calling another routine via an indirect jump.
- `store_status_and_jump`: Compares values from a stack-based structure against several arguments, and if all conditions are met, stores the value from a global address into another global address and sets bit 2 of d3 before calling another routine via an indirect jump.
- `check_collision_and_call_handler`: Performs a 2D bounding box intersection test using values from a table pointed to by a6 and compares them against registers a2, a3, and a4; if an intersection is detected, it updates a global variable and modifies a byte at a specific memory location before calling another routine via an indirect jump based on an offset in the table.
- `jump_if_offset_valid`: Checks if a word at an offset of 8 from a pointer in a6 is not negative, and if so, calls another routine via an indirect jump.
- `update_bounds_and_jump`: Performs several range checks using modified values from a stack-based structure, and if all conditions are met, sets a global byte to 1, updates d2 with a calculated value, clears d1, and sets bits 0 and 1 of d3 before calling another routine via an indirect jump.
- `dispatch_via_table`: Loads four 16-bit values from a global structure, performs arithmetic on them, and then jumps to an address calculated from a global variable and a table offset.
- `branch_to_middle_of_routine_2`: Performs a short branch to an offset within another routine.
- `execute_sequential_function_pointers_2`: Checks two global bytes and, if both are zero, executes a long sequence of functions by loading their addresses from various memory locations into a0 and calling them via a pointer in a1; it also performs a temporary save and restore of a byte at a specific memory location during the first function call.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `execute_address_list`: Branches to another routine, then if two specific global bytes are zero, it performs a series of calls to a routine pointed to by a1 using various addresses loaded into a0.
- `execute_sequential_function_pointers`: Checks a global byte and, if zero, executes a long sequence of functions via pointers loaded from various memory locations, performs a conditional check on a bit and a byte value, and finally calls another routine if a byte at a specific location is at least 25.
- `execute_sequential_function_pointers_3`: Checks a global byte and, if zero, executes a long sequence of function calls using a pointer in a1 and various data addresses loaded into a0.
- `execute_sequential_function_calls`: Checks a global byte; if zero, it performs a long sequence of calls to a function pointer in a1 using various data structures loaded into a0, then returns.
- `copy_data_block`: Compares a 16-bit value at a memory address to a constant; if they are not equal, it copies 64 words from a source table to the destination address using a loop.
- `frag_move_pea_jmp`: Short code fragment that copies 127 to the memory at a4, then pushes the address of a location in the code onto the stack, then jumps to the address in a5.
- `execute_sequential_function_calls_2`: Loads several addresses into a0 and calls a function pointer in a1 repeatedly for each address.
- `frag_bset_jsr_bclr_movea_rts`: Short code fragment that sets bit 7 of the memory at a4, then sets bit 3 of the memory at a4, then calls the routine whose address is in a5, then clears bit 3 of the memory at a4, then calls the routine whose address is in a5, then sets bit 3 of the memory at a4, then copies the top of the stack to a6, then returns.
- `call_validation_routines`: Calls a function through a pointer thirteen times, then loads a value into a0 and calls it once, followed by five calls to validate_range_parameters with different addresses in a0.
- `validate_address_list`: Calls a routine five times using a pointer in a1, then checks several global variables before calling another routine four times with different addresses in a0.
- `validate_multiple_addresses`: Calls a routine multiple times with different addresses in a0, then calls validate_and_update_bounds with a pointer in a1.
- `validate_and_update_bounds`: Checks if a structure's status and flags meet specific criteria, then compares its coordinate values against a table of bounds.
- `sum_masked_values`: Calls another routine, checks if the returned value matches a specific 16-bit constant and a value at a memory offset, then loops 1574 times to sum masked 32-bit values from a table and calls another routine to compare the result.
- `update_system_parameters`: Compares a 16-bit value at a memory address to 1; if not equal, it updates several memory locations with specific constants, modifies bits at offsets 24 and 28 of a structure in a0, and decrements a byte at offset 29 of that structure.
- `validate_bounds_structure`: Checks three specific bit and value conditions in a structure in a0, then performs several range comparisons using values from a0, d0, d1, d2, and d3 before storing a value in a global variable.
- `frag_tst_beq`: Short code fragment that tests the memory at an offset from a0, then branches if the last result was zero.
- `check_bounds_and_store`: Checks if a byte at offset 9 of a structure in a0 is non-zero, then performs multiple range comparisons using values from a table at a fixed address and a0, and stores a value in a global variable if all conditions are met.
- `check_global_and_bounds`: Checks if a byte at a memory address is zero and if bit 0 of a byte at offset 28 of a structure in a0 is clear; if so, it compares values in a0 against a table at a memory address to potentially store a value in a global variable.
- `validate_range_parameters`: Checks bits and bytes at offsets 24, 25, and 28 of a structure in a0, performs several comparisons against calculated ranges derived from a table, and stores a value in a global variable if all conditions are met.
- `validate_offset_ranges`: Checks if a byte at offset 8 of a structure in a0 is at least 3, then performs multiple range comparisons involving values at offsets 16 and 18 of the structure against d0, d1, d2, and d3 before storing a value in a global variable.
- `execute_jump_table`: Loads a 16-bit value from a memory address, doubles it, and performs an indirect jump to an address calculated by adding this value to a base address.
- `branch_to_offset_7520`: Performs a short branch to a location 7520 bytes ahead in the code.
- `execute_multiple_subroutines_2`: Stores a long value in a global variable, performs a sequence of calls to two different routines using addresses in a2 and a0, and finally inverts a byte in a global variable.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `jump_to_toggle_global_byte`: Jump stub: passes control straight on to toggle_global_byte.
- `frag_bra_11`: Short code fragment that jumps to another point in the code.
- `frag_bra_12`: Short code fragment that jumps to another point in the code.
- `sequence_collision_updates_2`: Sets a global variable to 255, calls check_collision_and_update_structure multiple times with various offsets, and then calls check_structure_bits and another routine multiple times.
- `execute_multiple_subroutines`: Stores a byte in a global variable, performs a long sequence of calls to another routine using various addresses in a0, and finally inverts a byte in a global variable before returning.
- `process_collision_sequence`: Stores a value in a global variable, calls check_collision_and_update_structure and check_structure_bits multiple times with various addresses in a0, and finally performs a bitwise NOT on a global variable.
- `initialize_asset_pointers`: Clears specific bits and sets bytes in four global structures, then copies a sequence of data from a buffer into a table while calculating offset pointers.
- `manage_dma_scripted_updates`: Parses a scripted data stream to update four separate DMA structures by loading pointers and byte values from a table, then manages the execution of these DMA channels by updating their control bits and calling another routine to trigger the transfers.
- `frag_beq_subq_movea_move_sub_bra`: Short code fragment that branches if the last result was zero, then subtracts 1 from the memory at a6 (advancing a6), then copies the memory at a6 to a1, then copies the memory at a1 (advancing a1) to d0, then copies a1 to the memory at a6 (advancing a6), then subtracts d0 from the memory at a6, then jumps to another point in the code.
- `update_dma_and_vpos`: Adds a value from a6 to a global variable, waits for the vertical position register to increment, and then writes modified values to the DMA controller and a global variable based on bitwise operations.
- `process_dma_command`: Loads a pointer from a structure in a6, updates several fields in that structure using data from the pointed-to buffer, and then modifies a 16-bit value at offset 56 of the structure based on a masked byte from the buffer.
- `parse_scripted_data_stream`: Parses a data stream using an index and a pointer to a table, performing conditional updates to a structure in a6 based on specific byte markers.
- `toggle_global_byte`: Sets a global variable to 255, then if the variable is not zero, it calls another routine multiple times before bitwise inverting the global variable and returning; otherwise, it calls another routine and returns.
- `frag_bsr_btst_bne_sf_rts`: Short code fragment that calls another routine, then tests bit 4 of the memory at an offset from a4, then branches if the last result was not zero, then clears the memory at an offset from a5, then returns.
- `execute_collision_loop`: Stores a value in a global variable, calls check_collision_and_update_structure many times with different addresses in a0, and conditionally calls it again after checking a bit in a global variable.
- `reset_and_collide`: Sets a global variable to zero, calls check_collision_and_update_structure five times, then sets the global variable to 255 and calls the routine eleven more times before inverting the global variable.
- `sequence_collision_updates`: Sets a global variable to 255, calls check_collision_and_update_structure five times, sets it to zero, calls the routine once, sets it to 255, calls the routine twice, and inverts the global variable.
- `check_structure_bits`: Offsets a0 by 30, checks bit 5 of a byte at offset 19 of the structure in a0, and checks bit 7 of a byte at offset 28 of the structure in a0, branching to another location if either bit is not set.
- `check_collision_and_update_structure`: Iterates through a table of four collision rectangles to check if a structure's bounding box overlaps with any active rectangle.
- `initialize_structure_loop`: Iterates through a list of structures, and if a byte at offset 24 is zero, initializes that structure with specific values and returns.
- `update_entity_state_and_animation`: Checks several global flags to determine an entity's state, calls various subroutines to update properties, and selects an animation frame from a table based on an index stored at offset 18 of the structure in a0.
- `rotate_and_dispatch`: Performs a left rotation on a 32-bit value and uses a byte from a structure to jump to a calculated offset.
- `update_entity_velocity_and_index`: Calculates new velocity values for two 16-bit fields in global variables based on a bitmask and a byte value, then iterates through a table to update an entity's attribute byte and index using data from a lookup table and calls update_entity_position_and_attributes or another routine.
- `frag_bra_13`: Short code fragment that jumps to another point in the code.
- `frag_bra_14`: Short code fragment that jumps to another point in the code.
- `frag_bra_15`: Short code fragment that jumps to another point in the code.
- `frag_bra_16`: Short code fragment that jumps to another point in the code.
- `frag_bra_17`: Short code fragment that jumps to another point in the code.
- `frag_bra_18`: Short code fragment that jumps to another point in the code.
- `frag_move_addi_addq_subq_st_rts`: Short code fragment that copies 57 to the memory at an offset from a0, then copies a fixed memory address to the memory at an offset from a0, then adds 19 to the memory at an offset from a0, then adds 1 to the memory at an offset from a0, then subtracts 1 from the memory at an offset from a0, then sets a fixed memory address to all ones, then returns.
- `frag_move_addq_subq_st_rts`: Short code fragment that copies 58 to the memory at an offset from a0, then copies a fixed memory address to the memory at an offset from a0, then adds 1 to the memory at an offset from a0 (2 times in a row), then subtracts 1 from the memory at an offset from a0, then sets a fixed memory address to all ones, then returns.
- `frag_move_clr_tst_beq_st_rts`: Short code fragment that copies a fixed memory address to the memory at an offset from a0, then clears a fixed memory address, then clears the memory at an offset from a0, then copies 1 to the memory at an offset from a0, then tests a fixed memory address, then branches if the last result was zero, then copies 7 to the memory at an offset from a0, then sets a fixed memory address to all ones, then returns.
- `frag_move_addq_subq_rts`: Short code fragment that copies 58 to the memory at an offset from a0, then copies a fixed memory address to the memory at an offset from a0, then adds 1 to the memory at an offset from a0, then subtracts 1 from the memory at an offset from a0, then returns.
- `frag_move_addi_addq_subq_rts`: Short code fragment that copies 57 to the memory at an offset from a0, then copies a fixed memory address to the memory at an offset from a0, then adds 19 to the memory at an offset from a0, then adds 1 to the memory at an offset from a0, then subtracts 1 from the memory at an offset from a0, then returns.
- `update_entity_state`: Clears a global variable, modifies a 16-bit value at offset 18 of a structure in a0, clears a byte at offset 12, and sets a byte at offset 8 to 49.
- `decrement_and_update_pointer_table`: Iterates through a table of structures, decrementing a 16-bit value at offset 18 of each entry and checking if a byte at offset 9 is zero.
- `update_pointer_table`: Sets a byte at offset 9 of a structure, copies a pointer from a structure to another, and modifies two 16-bit values at offsets 10 and 12 of a structure based on a global variable.
- `update_entity_position_and_attributes`: Iterates through a table of structures, updating various byte and word fields based on a global state value.
- `update_structure_attributes`: Clears three global values, loads a byte from a global address into d0, and copies a 16-bit word from offset 18 of a structure in a0 to a global location.
- `update_status_byte`: Pops two longwords from the stack, loads a longword from an offset of a0, stores a byte at an offset of a5, stores a byte from d0 at an offset of a5, and branches to another routine based on a byte at a specific address.
- `update_structure_animation`: Sets a global byte to three and updates the status register, then performs a series of conditional checks on a global byte and a structure in a0 to update its offset 8, offset 12, or offset 16, and calls several routines.
- `frag_lea_move_sf_bra`: Short code fragment that loads the address of a location in the code into a1, then copies 2 to a fixed memory address, then clears a fixed memory address, then jumps to another point in the code.
- `frag_lea_move_st_bra`: Short code fragment that loads the address of a location in the code into a1, then copies 7 to a fixed memory address, then sets a fixed memory address to all ones, then jumps to another point in the code.
- `frag_lea_move_st_bra_2`: Short code fragment that loads the address of a location in the code into a1, then copies 4 to a fixed memory address, then sets a fixed memory address to all ones, then jumps to another point in the code.
- `update_entity_state_2`: Updates various fields in a structure in a0 based on values in global variables and calls several routines including process_structure_update, update_structure_state, reset_status_and_copy, set_status_and_copy, set_status_via_sf, set_status_via_st, and update_entity_position_and_attributes_2.
- `increment_buffer_index`: Checks a 16-bit value at offset 12 of a structure, retrieves a byte from a table using that value, and updates a byte at offset 8 and the index at offset 12.
- `update_entity_position_and_attributes_2`: Iterates through a list of structures, updating their coordinate offsets by reading values from a table and adding them to existing values.
- `initialize_color_and_jump`: If a global byte is zero, it fills the color register with a descending loop; otherwise, it modifies several hardware registers and calls another routine via a calculated jump.
- `branch_to_animation_update_4`: Performs a short branch to the routine that updates an animation frame from a table.
- `frag_bra_19`: Short code fragment that jumps to another point in the code.
- `frag_bra_20`: Short code fragment that jumps to another point in the code.
- `frag_bra_21`: Short code fragment that jumps to another point in the code.
- `frag_bra_22`: Short code fragment that jumps to another point in the code.
- `frag_bra_23`: Short code fragment that jumps to another point in the code.
- `frag_bra_24`: Short code fragment that jumps to another point in the code.
- `frag_bra_25`: Short code fragment that jumps to another point in the code.
- `frag_bra_26`: Short code fragment that jumps to another point in the code.
- `frag_bra_27`: Short code fragment that jumps to another point in the code.
- `update_animation_frame_from_table_2`: Sets two global bytes, then performs multiple conditional checks on a global byte to select a table address in a1 and update various global bytes and a structure field via a call to another routine.
- `tiny_move_clr_rts`: Small routine of 3 instructions that copies 249 to d2, then clears d3, then returns.
- `tiny_move_clr_rts_2`: Small routine of 3 instructions that copies 249 to d3, then clears d2, then returns.
- `tiny_move_clr_rts_3`: Small routine of 3 instructions that copies 249 to d3, then clears d2, then returns.
- `tiny_move_clr_rts_4`: Small routine of 3 instructions that copies 7 to d3, then clears d2, then returns.
- `tiny_move_clr_rts_5`: Small routine of 3 instructions that copies 7 to d3, then clears d2, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 7 to d2, then copies 249 to d3, then returns.
- `tiny_move_move_rts_2`: Small routine of 3 instructions that copies 249 to d2, then copies 249 to d3, then returns.
- `tiny_move_move_rts_3`: Small routine of 3 instructions that copies 7 to d2, then copies 7 to d3, then returns.
- `update_animation_state_from_table`: Calls a routine to update an animation frame from a table and then calls another routine.
- `process_structure_update`: Checks a byte at a global address and compares a word at offset 18 of a0 with a global word; if conditions are met, it sets a global word to negative one, copies a word from a global address to offset 16 of a0, and calls another routine; otherwise, it clears bytes at offsets 12 and 13 of a0 and sets a byte at offset 14 to two.
- `update_structure_state`: Checks a byte at a global address and compares a word at offset 18 of a0 with a global word; if they match, it sets a global word to two, copies a word from a global address to offset 16 of a0, and calls another routine; otherwise, it sets a global byte to one, clears bytes at offsets 12 and 14 of a0, sets a byte at offset 13 to two, and calls another routine.
- `reset_status_and_copy`: Clears a byte at a global address, loads a pointer into a1, sets a global word to two, copies a word from a global address to offset 12 of a0, and calls another routine.
- `set_status_and_copy`: Sets a byte at a global address to one, loads a pointer into a1, sets a global word to negative one, copies a word from a global address to offset 12 of a0, and calls another routine.
- `set_status_via_sf`: Sets a byte at a global address based on the status register, loads a pointer into a1, sets a global word to two, copies a word from a global address to offset 12 of a0, and calls another routine.
- `set_status_via_st`: Sets a byte at a global address based on the status register, loads a pointer into a1, sets a global word to negative one, copies a word from a global address to offset 12 of a0, and calls another routine.
- `move_multiple_words`: Moves 16-bit words from various memory locations and registers into locations specified by pointers in a0, a1, a2, and a3.
- `move_multiple_words_offset`: Moves 16-bit words from memory locations using offsets and registers into a0, a1, a2, and a3.
- `set_status_and_pointer`: Sets a global byte to 1 and, depending on several conditions and a value at offset 0 of a structure, updates a byte at offset 4 and shifts values between two global 16-bit variables.
- `set_status_and_pointer_alt`: Sets a global byte to 0 and, depending on several conditions and a value at offset 0 of a structure, updates a byte at offset 4 and shifts values between two global 16-bit variables.
- `select_status_byte`: Checks a byte in d3 and a 16-bit global variable, then writes one of several possible byte values to offset 8 of a structure based on multiple global byte checks.
- `reset_global_values`: If a 16-bit global variable is 1 and a byte in d2 is 0, it stores a global pointer, sets two 16-bit values to 65535 and -10, updates two 16-bit globals to -10, and sets a byte at offset 8 of a structure.
- `set_global_flags`: Sets a byte in a global variable to 1, sets register d2 to 1, and clears a byte at offset 14 of the structure in a0 before returning.
- `update_animation_frame_from_table`: Selects a data table based on a global byte and bit tests in a register, then uses a byte at offset 14 of a structure in a0 as an index to fetch a value from the selected table into offset 8 of the structure.
- `update_status_and_index`: Calculates a difference between two longwords, sets a global byte to various values based on bit tests and a global byte check, and then either clears a byte at offset 14 of a structure and a global byte, or copies a byte from a table into offset 8 of a structure and increments offset 14.
- `frag_bset_subq_bclr_move_dbra_rts`: Short code fragment that sets bit 1 of the memory at a4, then subtracts 1 from the memory at an offset from a5, then clears bit 0 of the memory at a4, then sets bit 0 of the memory at a4, then copies d7 to the stack, then copies 6000 to d7, then loops back while counting down d7, then copies the top of the stack to d7, then returns.
- `loop_structure_updates`: Sets two global words, then enters a loop that iterates up to four times to update a structure at a global address by copying values from a0, reading bytes from a table using a global index, and performing arithmetic on structure offsets.
- `loop_structure_updates_extended`: Sets two global words, then enters a loop that iterates up to four times to update a structure at a global address by reading four bytes from a table using a global index, adding them to structure offsets, and performing conditional arithmetic.
- `branch_on_status_byte`: Loads a PC-relative address into a2, compares a global byte to 2, and if not equal, branches to another routine; otherwise it loads a different PC-relative address into a2.
- `loop_structure_updates_limited`: Sets two global words, then enters a loop that iterates up to four times to update a structure at a global address by reading four bytes from a table using a global index, adding them to structure offsets, and performing conditional arithmetic without the extra table access found in R9.
- `update_structure_fields`: Copies d2 and d3 to offsets 10 and 11 of a structure, synchronizes several 16-bit and 32-bit values within that structure, and if a global variable is 1, sets a byte at offset 8 to 3.
- `increment_pointer_table`: Increments a byte from a table pointed to by a global variable and, if the value exceeds a specific limit, updates that global variable to a new address.
- `update_object_coordinates_and_states`: Iterates through a table of structures, updating 16-bit coordinate values at offsets 16 and 18 using signed 8-bit offsets, and performs XOR operations on bytes at offsets 8 and 62.
- `process_structure_list`: Iterates through a list of structures in a loop, performing conditional updates to offsets 4, 6, 10, 12, and 18, and calls two other routines.
- `update_structure_elements_3`: Updates a structure by calling two routines, modifying offsets 9, 16, 18, and 24, and performing arithmetic on values retrieved from offsets 10 and 11.
- `execute_jump_table_2`: Calculates an offset by doubling a value from a global variable and performs an indirect jump to an address in a jump table.
- `update_game_object_structures`: Initializes asset pointers by clearing bits and copying data into four global structures, then iterates through a structure list to update specific offsets and calls two subroutines to modify elements within each structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_object_state_and_sub_objects`: Checks a flag at offset 9 of a structure and, if not set, performs a range check against a table to initialize a sequence of five sub-objects with specific values.
- `advance_sequence_index`: Checks a bit in a structure, and depending on the result, either increments an index and retrieves a byte from a table or clears specific bytes in the structure.
- `process_complex_structure`: Checks flags in a structure, updates a global word, performs multiple word-based arithmetic and memory copies between structures using several calls, and performs multiple calls to another routine with various offsets.
- `check_collision_bounds`: Tests a bit in a structure and, if not set, iterates through a list of bounds to check if a coordinate range overlaps with any entry, potentially setting a bit and clearing bytes if a match is found.
- `verify_coordinate_overlap`: Checks a bit in a structure and, if not set, performs a loop to compare a coordinate range against a list of entries, updating the structure if an overlap is detected.
- `update_counter_values`: Compares a byte at offset 14 of a structure to a constant; if it is not less than 24, it negates a byte at offset 12, increments it, and updates offset 14 and 13 accordingly.
- `fetch_table_byte`: Retrieves a byte from a table using an index from a structure, updates the structure's index and value, and sets two global variables based on the retrieved byte.
- `update_object_state`: Checks a byte at offset 15 of a structure; if it is not zero, it increments that byte and copies a value from a table into offset 8 of the structure.
- `process_object_data`: Increments a byte at offset 15 of a structure and copies a value from a table into offset 8.
- `manipulate_and_stack_data`: Pushes three bytes from a source pointer into a destination stack, performs a series of redundant byte moves to a register, applies a bitwise OR with 19 to a byte, and then moves several bytes between various pointers and registers including a stack operation.
- `subtract_and_increment`: Subtracts a word from a global variable into d3, moves 90 into d4, subtracts the same global variable from d3 again, and increments a byte at offset 0 of a structure.
- `update_structure_values`: Checks a byte at offset 24 of a structure and, depending on bit 0 of offset 28, either copies two words from a table into offset 16 of the structure or updates offset 12 and 16 using values from a table and offset 10; otherwise, it uses a byte from offset 10 to index a table, adds the resulting value to offset 18, increments offset 10, and sets offset 8 to either 5 or 6.
- `increment_table_index`: Sets bit 0 of a byte at offset 28 in a0, reads a byte from a table using an index at offset 10 of a0, and if the table byte is non-zero, adds it to a 16-bit value at offset 18, increments the index, and sets a byte at offset 8 to 6 or 5.
- `update_entity_state_and_counters_2`: Updates various counters and state flags within a structure based on its current values, including incrementing an index into a table, modifying a timer, and setting a status byte at offset 8 based on specific values at offset 11.
- `update_status_flags_2`: Checks a byte at offset 24 of a structure in a0; if the byte is negative and bit 7 of the byte at offset 28 is set, it clears bit 7 of the byte at offset 24 and returns, otherwise it returns.
- `update_position_and_direction`: Checks bit 0 of a byte at offset 28 of a structure, and if certain conditions are met, modifies offsets 14 and 15 and calls another routine.
- `update_animation_frame_index`: Checks several status bits and a byte at offset 11 of a structure in a0; if conditions are met, it either sets bit 3 of offset 25 and clears offset 10, or it increments a value at offset 14, performs bitwise rotations and arithmetic on values at offsets 16 and 18, and calls another routine.
- `update_fixed_point_coordinates`: Checks two flags at offsets 11 and 28 of a structure in a0; if both are zero, it performs fixed-point arithmetic by updating a 16-bit value at offset 12 using a table lookup and bitwise rotations, then updates values at offsets 12 and 14 based on the result.
- `branch_to_animation_update_3`: Loads a byte from offset 9 of a structure in a0, multiplies it by 4, and performs an indirect jump to an offset calculated from that value.
- `branch_to_animation_update`: Branches to an offset within another routine.
- `jump_to_frame_update`: The routine is a single unconditional word-displacement branch that transfers control to a shared block elsewhere in the segment, so it performs no work of its own.
- `branch_to_animation_update_2`: Performs a short branch to a location within the update_animation_frame routine.
- `branch_to_another_routine`: Performs a short relative branch to an instruction located at a different address.
- `update_state_from_table_2`: Loads a byte from offset 10 of a structure, uses it to index into a table, and if the result is non-zero, updates offset 8 and increments offset 10, otherwise it returns.
- `branch_to_middle_of_routine_3`: Performs a short branch to a location inside another routine.
- `jump_to_shared_tail_2`: The routine consists solely of a single unconditional word-displacement branch (bra.w) into the middle of another routine, so it does no work of its own and simply transfers control there.
- `branch_to_internal_offset`: Performs a short branch to a specific location within another routine.
- `branch_to_offset_2`: Performs a short relative branch to a specific location within another routine.
- `branch_to_sequence_update`: Performs a short branch into another routine to continue the execution of a sequence and state update.
- `update_animation_frame`: Checks a flag at offset 28 of the structure; if set, it clears a byte at offset 24 and otherwise increments an index at offset 10 and fetches a new byte from a table to store at offset 8 of the structure.
- `update_state_from_table`: Loads a byte from offset 10 of a structure, uses it to index into a table, and if the result is non-zero, updates offset 8 and increments offset 10.
- `clear_structure_byte`: Loads a byte from offset 10 of a structure in a0, uses it as an index into a table via a1, and if the resulting byte is non-zero, clears a byte at offset 24 of the structure in a0 before returning.
- `update_object_state_2`: Updates a byte at offset 8 and increments a byte at offset 10 of a structure, or performs complex conditional updates to offsets 18, 24, 25, and 26 based on values in offset 12 and a table, and calls another routine.
- `update_sequence_and_state`: Iterates through a sequence of bytes from a table using an index at offset 10 of a structure, updating offset 8 with the retrieved value and incrementing the index.
- `increment_counter_conditionally`: Compares a byte at offset 10 of a structure with 20; if it is less than 20, it clears offset 18, otherwise it increments offset 12 by 6 and offset 10 by 1.
- `modify_registers_and_bits`: Subtracts 4 from d4, performs bit tests on d2 and d5, performs an OR operation on d4, subtracts 5 from d5, and performs a bit test on d2 and d0.
- `modify_registers_and_bits_2`: Adds 6 to d6, performs bit tests on d3 and d7, performs an OR operation on d1, and performs an AND operation on d2.
- `fetch_table_index`: Sets bit 7 and bit 6 of a byte at offset 29 of a structure, then uses a byte at offset 11 as an index into a table to copy a value into offset 9 of the structure and increments the index.
- `fetch_table_value`: Loads a word from a structure in a0, masks the upper bits, and if the result is non-zero, uses a byte from offset 10 of a0 as an index into a table to load a byte into d0; otherwise, it clears a byte at offset 24 of a0 and returns.
- `store_and_increment`: Stores the byte in d0 into offset 8 of a structure and increments the byte at offset 10 of the same structure.
- `validate_and_fetch_data`: Checks if a byte at offset 17 is non-zero and performs a distance calculation between a global variable and a word at offset 16 of a structure.
- `fetch_table_value_alt`: Loads a byte from offset 10 of a0, uses it as an index into a table to load a byte into d0, and if the result is zero, clears a byte at offset 24 of a0 and returns.
- `increment_structure_field`: Stores a byte from register d0 into a structure at offset 8 of a0, then increments the byte at offset 10 of the same structure before returning.
- `frag_btst_addi_ori_andi`: Short code fragment that tests bit d2 of d5 (2 times in a row), then adds 6 to d6, then merges 1 into d1, then tests bit d0 of d2, then masks d2 with 0.
- `update_index_from_table`: Sets a bit in a structure in a0 based on a value in d0, and if a byte at offset 1a is non-zero, reads a value from a table using an index at offset 9, increments that index, and clears a global variable.
- `frag_move_addq_clr_rts`: Short code fragment that copies d0 to the memory at an offset from a0, then adds 1 to the memory at an offset from a0, then clears a fixed memory address, then returns.
- `handle_coordinate_update`: Checks a global variable and a byte at offset 9 of a0; if certain conditions involving a global variable and a range comparison are met, it updates several fields in the structure in a0 and a5, modifies global variables, and calls another routine.
- `process_range_event`: Calls another routine, checks a global variable and a byte at offset 9 of a0, and if a range comparison succeeds, updates a5, modifies global variables, and stores a signed byte from offset 25 of a0 into a global variable.
- `handle_range_event`: Checks a byte at offset 24 of a structure and a global variable, then calls two different routines.
- `update_offset_by_index`: Increments offset 11 of a structure, then based on its value, either adds the negative of offset 27 to offset 18, or adds a byte from offset 12 to offset 18 and increments offset 12, or clears offset 24.
- `populate_object_list`: If a bit is set in a byte at offset 28 of a structure, it copies a value from a table into offset 8 and increments the index at offset 10.
- `manage_structure_state`: Compares values in a structure against a global variable and a table to set offset 8 and 9, or if offset 9 is non-zero, it uses offset 10 to index a table to update offset 18 and offset 10, or modifies several global variables and offsets 8 and 12.
- `update_entity_state_and_counters`: Checks a byte at offset 9 of a structure to determine a state transition; if conditions involving range checks against global values and structure offsets are met, it updates bytes at offsets 8 and 9 and clears a byte at offset 10 of a second structure.
- `update_complex_state`: Clears bytes at offsets 25 and 26 of a0, modifies bytes at offsets 8 and 25 of a0 and a2 based on bit tests, and if a range comparison succeeds, updates several global variables and offsets in a0 and a5.
- `process_object_state_flags`: Checks bits 0, 1, and 2 of a status byte at offset 38 of a structure in a0 to determine which logic branch to execute.
- `update_status_and_flags`: If a byte at a global address is zero, it subtracts a word at offset 6 of a structure in a0 from a word at offset 18 of a0, stores the result at offset 18 of a structure in a5, sets a byte at offset 9 of a0 to 1, clears two global variables, and if a byte at offset 29 of a0 is negative, sets bit 2 of a byte at offset 26 of a0 before returning.
- `update_structure_field`: Adds a byte from offset 25 of a structure in a0 to offset 18 of the same structure, conditionally increments offset 25, and performs several writes to global variables and a structure in a0.
- `frag_bchg_beq_subq_rts`: Short code fragment that flips bit 0 of the memory at an offset from a0, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a0, then returns.
- `initialize_structure_data`: Calculates a value by subtracting 8 from a word at offset 12 of a structure in a0, stores it at offset 12 of a structure in a1, sets offset 10 of a1 to the value at offset 10 of a0 plus 8, and then performs multiple word and byte writes to various offsets in the structure in a1.
- `clear_structure_fields_2`: Clears offset 18 of a structure in a0, moves a longword from offset 12 of a0 to offset 16 of a0, and sets several byte values at offsets 1d, 19, 18, and 9 of a structure in a0.
- `update_object_loop`: Decrements a byte at offset 14 of a structure in a1, then enters a loop that masks a byte at offset 8 of a structure in a0, increments it, updates coordinates by adding signed bytes from offsets 10 and 11 to offsets 16 and 18 of a0, and calls another routine.
- `update_object_logic`: If a byte at offset 12 of a structure in a1 is at least 20, it sets bit 1 of offset 26 in a0 and enters a loop that updates several fields in a structure in a1 using values from a table.
- `update_object_with_logic`: Enters a loop that sets a byte at offset 24 of a structure in a0 to 11, sets a longword at offset 20 to a constant, updates coordinates by adding signed bytes from offsets 10 and 12 to offsets 16 and 18 of a0, and conditionally increments a byte at offset 12 of a0 after flipping a bit at offset 13 before calling another routine.
- `check_range_bounds`: If a byte at a global address is zero, it checks if a word at offset 16 of a structure in a0 is within a range defined by a word at offset 4 of a structure in a0 and a word at a global address, returning 1 if the condition is met and 0 otherwise.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_range_overlap`: Compares a value from a global variable against a range derived from a0 to return 1 if the value falls within the calculated bounds, otherwise it returns 0.
- `verify_coordinate_bounds`: Compares values from a0 and a global variable against a range defined by a5 to return 1 if the conditions are met, otherwise it returns 0.
- `process_bit_manipulation`: Uses a value from offset 14 of a0 and a table at a6 to perform bitwise operations and updates offsets 12, 16, 18, 25, and 26 of a0.
- `update_object_coordinates`: Checks a flag at offset 24 of a structure; if set, it calculates new coordinate values by indexing into a table using a value at offset 8, performs arithmetic with two global variables, and updates offsets 4, 5, 7, 16, and 18 of the structure.
- `subtract_offset_value`: Subtracts a 16-bit word from offset 6 of a0 from the 16-bit word at offset 2 of a0.
- `configure_blt_graphics_parameters`: Validates a structure in a0 and, if valid, calculates Blitter parameters including area dimensions and offsets to update a hardware-related structure and Blitter registers.
- `branch_to_update_routine`: Performs a short branch to another routine.
- `branch_to_update_table_entries`: Performs a short branch to the update_table_entries routine.
- `branch_to_update_structure_entries`: Performs a short branch into another routine.
- `branch_to_update_structure_entries_2`: Branches to another routine that updates a structure pointed to by a5 by writing a pointer, an offset, and two other values at offsets 72, 88, 64, and 48.
- `branch_to_update_structure_entries_3`: Performs a short branch to the middle of another routine.
- `store_structured_pointers_2`: Branches to another routine, otherwise it performs multiple writes of pointers and word values to a structure at a5, including bitwise OR operations on d2 and d3, and clears bit 6 of a byte at offset 24 of a0 before returning.
- `frag_bra_28`: Short code fragment that jumps to another point in the code.
- `jump_to_update_table_entries`: Jump stub: passes control straight on to update_table_entries.
- `frag_bra_29`: Short code fragment that jumps to another point in the code.
- `frag_bra_30`: Short code fragment that jumps to another point in the code.
- `frag_bra_31`: Short code fragment that jumps to another point in the code.
- `update_structure_entries_batch`: Performs multiple writes to a structure in a5, setting various 32-bit and 16-bit values at offsets 48, 4c, 50, 54, and 58 using values derived from a1, a2, a3, and d2, before clearing bit 6 of a byte at offset 24 of a0.
- `update_table_entries`: Performs multiple updates to a table at a5 by applying bitwise OR operations to values in d2 and d3 and storing them at offset 64, while also updating various pointer offsets in a5.
- `update_structure_entries`: Repeatedly updates a structure pointed to by a5 by writing a pointer, an offset, and two other values at offsets 72, 88, 64, and 48, using various combinations of arithmetic on a1, a2, a3, and d0.
- `update_structure_entries_complex`: A large routine that performs multiple sequences of updating a structure in a5 by adding offsets to a2 and writing to offsets 40 and 64 using bitwise OR operations on d2 or d3, while also moving registers and updating a1.
- `store_structured_pointers`: Stores several pointers from a1, a3, and a4 into a structure at a5 and performs bitwise OR operations on d2 and d3 to update offset 64.
- `update_structure_entries_with_pointers`: Updates a structure in a5 by writing 32-bit values to offsets 48, 4c, 50, and 54 using a1, a2, and a3, and writing 16-bit values to offset 40 using bitwise OR operations on d2 or d3.
- `update_structure_entries_subset`: Updates a structure in a5 by writing 32-bit values to offsets 48, 4c, 50, and 54 using a1, a2, and a3, and writing a 16-bit value to offset 64 using a bitwise OR operation on d2.
- `update_structure_entries_mixed`: Updates a structure in a5 by writing 32-bit values to offsets 48, 4c, 50, and 54 using a1, a2, and a3, and writing 16-bit values to offset 40 using bitwise OR operations on d2 or d3 with different constants.
- `configure_blitter_parameters_2`: Modifies values in a structure using data from a table, updates several global words, and calls another routine multiple times based on a byte value retrieved from a table.
- `configure_blitter_parameters`: Uses a global byte to index into a table, calculates an offset, and writes several bytes to a hardware-related structure at offset 23c80.
- `update_global_table_offsets`: Swaps two global pointers, checks a global word and a hardware register, then updates a series of 16-bit words at various offsets of a table at a1 using an incremented value.
- `handle_interrupt_request`: Checks bit 5 of the interrupt request register, increments a global word, calls a routine at a pointer, calls another routine with a pointer in a1, and returns from interrupt.
- `update_cia_interrupts`: Sets bit 6 of the CIAA control register, clears a byte at a specific offset from a pointer, and uses a loop to perform bitwise operations on a value before writing it to a global variable and returning via an interrupt return.
- `search_table_for_bounds`: Clears three words in a global variable, searches a table for an entry where a value is within a range defined by a structure in a6, and if found, updates several words and a longword in the global variable before calling another routine.
- `branch_to_next_instruction`: Performs a short relative branch to the instruction immediately following this routine.
- `branch_to_middle_of_routine`: Performs a short branch into the middle of another routine.
- `update_status_bits_via_comparison`: Calculates the difference between a 16-bit value at offset 16 of a structure in a6 and a value at offset 160 of a global variable, then sets bit 1, 2, or 3 of that global variable based on comparisons between the difference, a value in a0, and a value at offset 12 of a table in a0.
- `frag_bra_32`: Short code fragment that jumps to another point in the code.
- `frag_bra_33`: Short code fragment that jumps to another point in the code.
- `jump_to_update_status_flags`: Jump stub: passes control straight on to update_status_flags.
- `frag_bra_34`: Short code fragment that jumps to another point in the code.
- `frag_bra_35`: Short code fragment that jumps to another point in the code.
- `set_status_flags`: Compares values from a0 and a6 against ranges derived from global pointers to set specific bits in a global variable at offset 0.
- `update_status_flags`: Compares values from a structure in a6 and a table in a0 against global variables to set bits 1, 2, 4, or 8 of a global variable.
- `check_status_conditions`: Sets bits 2, 4, or 8 of a global variable based on comparisons between values in a6, a0, and global variables, then returns a calculated 16-bit value in d0.
- `frag_bsr_move_cmp_bge_ori_rts`: Short code fragment that calls another routine, then copies a fixed memory address to d3, then copies the memory at an offset from a0 to d2, then compares d3 with d2, then branches if the comparison was greater or equal, then merges 2 into a fixed memory address, then returns.
- `frag_bsr_move_cmp_ble_ori_rts`: Short code fragment that calls another routine, then copies a fixed memory address to d3, then copies the memory at an offset from a0 to d2, then compares d3 with d2, then branches if the comparison was less or equal, then merges 8 into a fixed memory address, then returns.
- `compare_and_modify_data`: Loads two 16-bit values from a structure in a6, performs arithmetic on the second value, and conditionally modifies a global variable based on comparisons with a value in a0.
- `frag_move_addi_sub_bmi_cmp_beq_ori_rts`: Short code fragment that copies the memory at an offset from a6 to d1, then copies a fixed memory address to d3, then adds 16 to d1, then subtracts d3 from d1, then branches if the last result was negative, then compares d3 with the memory at an offset from a0, then branches if the last result was zero, then merges 4 into a fixed memory address, then returns.
- `frag_move_subi_sub_cmpi_bpl_ori_bra`: Short code fragment that copies the memory at an offset from a6 to d1, then copies a fixed memory address to d3, then subtracts 176 from d3, then subtracts d3 from d1, then compares d1 with 64, then branches if the last result was not negative, then merges 1 into a fixed memory address, then jumps to another point in the code.
- `check_threshold_conditions`: Subtracts a value from a structure in a6 from a global variable and checks if the result is negative, then performs a second comparison with a value in a0 to conditionally set a bit in a global variable.
- `evaluate_multiple_bounds_2`: Loads words from a structure in a6 and two global variables, performs arithmetic, and conditionally sets bit 0 of a global variable.
- `evaluate_multiple_bounds`: Performs several comparisons using values from a structure in a6 and a pointer in a0 to conditionally set bits in a global variable.
- `update_status_flags_3`: Sets a value at offset 68 of the custom chip registers to negative one, then performs several conditional checks on a global word and a bitmask to call various routines and update offsets 8, 12, 16, or 18 of a structure in a0.
- `process_structure_list_2`: Compares a 16-bit value from a global variable against another, then enters a loop that updates a pointer in a global variable and calls two different routines with specific parameters based on the pointer's value.
- `decrement_structure_pointers`: Compares a 16-bit value from a global variable against another, then enters a loop that decrements values in two global variables and calls two different routines with specific parameters.
- `perform_blt_transfers_2`: Compares a 16-bit value plus 176 against another, then enters a loop that conditionally updates a pointer in a global variable and calls two different routines to perform memory transfers.
- `process_global_tables`: Compares values from two global variables and branches if they are equal, otherwise it increments a global variable, updates a pointer in a table, and performs several conditional blocks involving calls to other routines and arithmetic on global variables.
- `update_object_attribute_table`: Checks a global byte and, depending on its value, performs a series of multi-word memory writes to a structure pointed to by a6.
- `update_structure_elements`: Calculates a base address using several global pointers and an offset in d0, then enters a loop that performs 12 iterations to write three sets of values to a structure in a5 based on a byte retrieved from a table.
- `update_structure_and_copy`: Performs 15 iterations of writing three sets of values to a structure in a5 using a table-derived offset, then writes a zero to an offset in a5 and performs two additional sets of writes to a structure using values derived from global pointers and d1.
- `update_structure_multiple_times`: Enters a loop that runs 12 times, calculating a table-based offset to write five different sets of values to a structure in a5, then increments a global variable and returns.
- `update_structure_extended_loop`: Enters a loop that runs 15 times, calculating a table-based offset to write four different sets of values to a structure in a5, then increments a global variable and returns.
- `perform_blt_transfers`: Sets up the Amiga Blitter registers via a5, then enters a loop that runs 5 times to perform area transfers between two memory locations derived from a global pointer and a constant offset.
- `perform_blt_transfers_swapped`: Sets up the Amiga Blitter registers via a5, then enters a loop that runs 5 times to perform area transfers where the source and destination addresses are swapped compared to the previous routine.
- `perform_blt_transfers_short`: Sets up the Amiga Blitter registers via a5, then enters a loop that runs 3 times to perform area transfers between a memory location derived from a global pointer and another derived from a constant offset.
- `perform_blt_area_fills`: Configures the Blitter to perform multiple area fills by setting the color mode, mask, and destination parameters, then executes a loop that updates the source and destination pointers for each operation.
- `update_structure_elements_2`: Sets a value at offset 104 of a structure pointed to by a5, then performs two loops of 14 iterations that call another routine to update structure elements using values from global pointers and tables.
- `execute_indirect_jump`: Checks a byte at a global address; if it is non-zero, it calculates a jump target using a value from another global variable and performs an indirect jump.
- `branch_to_next_routine`: Performs a relative branch to another location.
- `update_data_tables`: Copies words from one of two source tables to a destination table and a structure in a1 based on a value in a2, and if certain conditions are met, decrements a global variable and calls another routine.
- `frag_bra_36`: Short code fragment that jumps to another point in the code.
- `frag_bra_37`: Short code fragment that jumps to another point in the code.
- `frag_bra_38`: Short code fragment that jumps to another point in the code.
- `frag_bra_39`: Short code fragment that jumps to another point in the code.
- `toggle_and_swap_data_2`: Copies eight 16-bit words from one structure to another, conditionally toggles a global bit, and calls update_structure_and_copy after performing arithmetic on a global variable.
- `copy_table_with_condition_2`: Checks a value at offset 22 of a global variable to determine a source table, then enters a loop that runs 32 times to copy words from a source table to a destination table and a third location.
- `copy_table_with_condition`: Enters a loop that runs 32 times, copying a word from a source table to a destination table and also to a location pointed to by a3, but only if a specific global variable is non-zero.
- `frag_tst_bne_move_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies 13 to a fixed memory address, then tests a fixed memory address, then branches if the last result was not zero, then copies 15 to a fixed memory address, then tests a fixed memory address, then branches if the last result was not zero, then returns.
- `update_indexed_table`: Calculates an index based on a global variable, performs multiplication, and copies three 16-bit values from a table into a structure at a specific offset.
- `copy_data_block_2`: Copies eight 16-bit words from a source table into a destination structure at specific offsets.
- `rotate_and_update_buffers`: Increments two counters at specific memory locations and, if they wrap, rotates four 16-bit words within a table and copies them to a destination structure.
- `update_audio_and_table_data`: Updates a sequence of 16-bit values by rotating them through a global buffer, or if a counter reaches its limit, initializes the two audio channels with specific period, length, and volume values and performs a loop of 100 iterations.
- `shift_and_copy_large_block`: Rotates three 16-bit words within a table and, if a global variable is non-zero, copies sixteen 16-bit words from a second table into a destination structure.
- `copy_short_table`: Copies eight 16-bit words from a source table into a destination structure at fixed offsets.
- `toggle_and_swap_data`: Toggles a bit in a global variable and, if the result is zero, swaps three 16-bit words within a table and copies them to a destination structure.
- `process_bcd_and_bitmask`: Calls another routine, performs BCD adjustments on values from two global variables, manipulates a longword using bitwise AND, XOR, LSR, and ASL, and stores the results into a table starting at a specific address before calling another routine.
- `merge_table_entries`: Loops through a source buffer to read pairs of bytes, uses them as offsets into two different tables to fetch bytes, performs a bitwise OR on the results, and stores the result into a structure pointed to by a6 at specific intervals.
- `merge_table_entries_2`: Updates a global variable if it is greater than or equal to another global variable, then performs bitwise operations on it to store two derived 32-bit values into a table at offset 27e84.
- `process_table_and_call`: Checks three global bytes, and if they meet specific conditions, modifies a structure at offset 2616c, updates a table at offset 27e84, and calls another routine.
- `update_pointer_tables`: Reads a value from a global variable to determine an offset, then uses that offset to fetch five bytes from one table and five bytes from another table, scaling the values by powers of two before storing them into a destination table at offset 504 or 517.
- `check_and_call_routines`: Checks two global words and calls two different routines if the words are non-zero.
- `decrement_and_copy_table`: Decrements a global variable and, if it reaches zero, copies 32 words from a source table pointed to by a1 to a destination table at offset 238a6.
- `decrement_global_counter`: Decrements a word in a global variable and, if the result is not zero, branches back to the start of the routine; otherwise, it sets a global word to two and sets d5 to one.
- `process_bitmask_array`: Iterates through a table of 32 words, performing bitwise operations to either subtract a value or mask bits based on a comparison with a value in d5, then increments a global counter and resets it along with two other globals when it reaches 16.
- `update_structure_list`: Loops three times to check a byte at offset 9 of a structure pointed to by a register; if it is zero, it updates several fields in that structure and a global variable, then returns.
- `check_object_status`: Checks if a byte at offset 24 of a structure pointed to by a0 is non-zero and if a byte at offset 9 is non-zero, then checks bit 0 of a byte at offset 28 of the same structure; if bit 0 is set, it calls another routine and returns.
- `update_object_properties`: Decrements a byte at offset 12 of a structure, and if it reaches zero, clears bytes at offset 9 and 18.
- `copy_memory_block`: Calculates a value based on three consecutive memory locations and, if the result matches a constant, copies 44 words from one memory location to another using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_structure_fields`: Clears specific 16-bit words at offsets 8, 28, 56, 84, and 112 within a structure pointed to by a6.
- `check_collision_and_update`: Checks a global byte and performs a series of boundary comparisons using values from a structure in a0 and a table in a1; if the conditions are met, it clears bytes in the structure, increments several global counters, and calls another routine.
- `iterate_object_list`: Loops through a list of structures starting at a global address, calls two routines and another routine if a byte at offset 9 of the current structure is non-zero, and increments the pointer by 30 bytes each iteration.
- `initialize_table_entry`: Searches through a table for an entry where a byte at offset 9 is zero; if found, it initializes several bytes and words at specific offsets within that entry and copies a 32-bit word from a structure in a0 to the entry.
- `update_object_kinematics_2`: Updates an object structure by reading values from a table, calling another routine, and modifying several fields including offsets 8, 10, 12, 13, 14, 16, 18, and 24 based on the results.
- `update_object_kinematics`: Checks if an object's position is within a specific range defined by a table, and if so, resets several bytes in a structure and clears a series of bytes at fixed intervals in a global array.
- `iterate_and_update_entity_state`: Iterates through a list of structures in a table, checking if a specific byte is non-zero and if a coordinate value falls within a defined range.
- `branch_to_intermediate_offset`: Performs a short branch to an instruction located within another routine.
- `branch_to_offset_142a`: Performs a short relative branch to a location within the same segment.
- `update_entity_state_counter_2`: Increments a byte at offset 12 of a structure in a0 and masks it with 3; if the result is 0, it modifies a byte at offset 11 by adding 5 and updates bytes at offsets 8 and 24 with the new value from offset 11 and the constant 3 respectively, then performs a branch.
- `branch_to_offset_147a`: Performs a short branch to a specific instruction offset.
- `branch_to_mid_routine`: Performs a short relative branch into the middle of another routine.
- `increment_state_and_dispatch_subroutine`: Increments a byte at offset 12 of a structure in a0 and masks it with 3; if the result is 0, it updates offset 11 by adding 13, copies the value from offset 11 to offset 8, and sets offset 24 to 3.
- `update_state_byte`: Increments a byte at offset 12 of a structure in a0 and masks it with 3; if the result is 0, it updates offset 11 by adding 13, otherwise it performs further bitwise masking and updates offset 11 before jumping to another location.
- `update_entity_state_counter`: Increments a byte at offset 12 of a structure in a0 and masks it with 3; if the result is 0, it modifies a byte at offset 11 by adding 5, copies that value to offset 8, and writes 3 to offset 24.
- `increment_and_update_state_structure`: Increments a byte at offset 12 of a structure in a0 and masks it with 3; if the result is 0, it updates offset 11 by adding 13, copies the new value from offset 11 to offset 8, and sets a byte at offset 24 to 3.
- `update_global_counters`: If a byte in a global variable is non-negative, it calls another routine, then checks bit 4 of another global variable; if that bit is set, it clears a global word, sets two other global words, and performs several stores and a final word move before returning.
- `increment_and_reset_counter`: Increments a 16-bit value at a global address and masks it with 31.
- `reset_memory_conditional`: Checks a bit in a global variable and, if set, writes several values to memory, clears a word in a global variable, and calls search_dispatch_table_and_reset_memory if a comparison with 45 is not less than zero.
- `frag_lea_movea_adda_moveq_bsr_move_dbra`: Short code fragment that loads the address of a fixed memory address into a0, then copies a fixed memory address to a1, then adds 2527 to a1, then loads 12 into d0, then calls another routine, then copies 100 to d0, then copies 32000 to d1, then loops back while counting down d1, then loops back while counting down d0, then returns.
- `apply_bitmask_to_table`: Uses a byte from a table to perform bitwise AND and OR operations on various offsets of a structure in a1, repeating this process through nested loops.
- `execute_jump_table_3`: Calculates a jump target by subtracting 1 from a word at a global address, multiplying the result by 4, and performs an indirect jump to that location.
- `branch_to_offset_1866`: Performs a short branch to a location within another routine.
- `process_pointer_tables`: Loads a pointer from a global variable into a1 and performs twenty-two calls to another routine, passing various addresses in a0 and counts in d0.
- `frag_bra_40`: Short code fragment that jumps to another point in the code.
- `frag_bra_41`: Short code fragment that jumps to another point in the code.
- `frag_bra_42`: Short code fragment that jumps to another point in the code.
- `frag_bra_43`: Short code fragment that jumps to another point in the code.
- `update_multiple_pointer_tables_2`: Performs a branch to another routine and then calls another routine multiple times with varying addresses in a0 and counts in d0 to update several pointer tables.
- `update_pointer_tables_2`: Loads a pointer from a global variable into a1 and performs several calls to another routine, passing various addresses in a0 and counts in d0.
- `update_multiple_pointer_tables`: Calls update_pointer_table_2 multiple times with different addresses in a0 and different counts in d0.
- `update_pointer_tables_subset`: Calls update_pointer_table_2 seventeen times using a specific set of addresses in a0 and counts in d0.
- `update_pointer_tables_alternate`: Calls update_pointer_table_2 seventeen times using a different set of addresses in a0 and counts in d0.
- `update_pointer_tables_short`: Calls update_pointer_table_2 nine times with various addresses in a0 and counts in d0.
- `update_pointer_tables_extended`: Calls update_pointer_table_2 thirteen times with various addresses in a0 and counts in d0.
- `update_pointer_table_2`: Calculates a difference between a value from a table and a value from a pointer in a0, adds this difference to the value at the pointer in a0, and repeats this in a loop for the count in d0.
- `search_dispatch_table_and_reset_memory`: Searches a table for a matching entry based on range checks, then either executes an indirect jump to a function from a jump table and clears a block of memory, or performs a large memory clear and copies a block of data from one location to another before returning.
- `clear_memory_and_dispatch_jump_table`: Clears a block of memory starting at a specific address up to a second address using a loop, then performs an indirect jump to a location determined by a value loaded from a global variable.
- `branch_to_offset_4`: Performs a short branch to another location.
- `set_status_bytes`: Sets specific bytes at offsets 24, 26, and 36 of three different memory locations to 255 and their preceding bytes to 0.
- `initialize_global_tables`: Sets a large number of global 16-bit and 32-bit values to specific constants, calls another routine, and then sets more global values.
- `initialize_memory_blocks`: Sets various memory locations to either 255 or 0, sets specific bits in some locations, and calls a routine at address 3192a.
- `initialize_memory_blocks_alternate`: Sets various memory locations to either 255 or 0, sets specific bits in some locations, and calls a routine at address 3192a using a different sequence of addresses.
- `initialize_data_structures_2`: Sets multiple 16-bit values in various memory locations by storing 255 at offset 28 and 0 at offset 24, sets bits 5 and 6 of a byte at offset 25 of two different structures, calls another routine, and stores 255 at offset 28 and 0 at offset 15 of several other structures before returning.
- `reset_memory_tables`: Sets 16-bit values to 255 at offset 28 and 0 at offset 24 for eight different memory locations before returning.
- `initialize_system_parameters`: Sets specific bytes and words at various global addresses, populates several structures with constant values and longwords, and calls a routine via a pointer before branching to another routine.
- `initialize_game_data_structures`: Populates a series of global variables with constant values and configuration parameters, then executes a loop that copies data from a source table into a destination structure using a series of offsets and increments.
- `initialize_structure_and_copy_data`: Writes several values to a structure at offset 30 of a0, calls two subroutines, and then enters a loop that copies data from a table in a1 to a destination at offset 23d98 using a counter in d0.
- `initialize_object_attribute_tables_2`: Populates several data structures with constant values and pointers, then uses nested loops to fill a table with repeated attribute data.
- `initialize_attribute_tables`: Performs a division, populates register d2 with a series of bitwise OR operations, and then writes various constant values to several global variables and memory locations via pointers.
- `initialize_object_attribute_tables`: Populates a series of data structures with constant values using nested loops, then copies a sequence of data from a source table into a destination table using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_joystick_input`: Checks the status of a hardware bit, modifies a byte in a table based on joystick and button input, and updates a global variable.

### Toki.adf:blocks 22-133 (bytes 11264-68608)#unpacked (raw code file, load address unknown)

- `initialize_graphics_and_process_tables`: Configures custom hardware registers, clears a memory range via a loop, and initializes several global variables and structures.
- `initialize_data_block`: Writes a sequence of 16-bit and 8-bit constants to a specific memory range.
- `initialize_blitter_and_counters`: Loads two addresses into a1, calls another routine twice, then enters a loop that runs 15 times to call another routine if the value at a specific hardware register offset is not 60.
- `setup_blitter_transfer`: Configures blitter registers and calls another routine in a loop to perform multiple area fills or transfers.
- `update_coordinate_table`: Calculates and stores a series of 16-bit values into a table by performing arithmetic on a value in d0 and a constant.
- `copy_words_to_global_buffer`: Copies seven 16-bit words from a structure pointed to by a0 to a global buffer pointed to by a1, then returns.
- `execute_blitter_loop`: Calculates blitter parameters including source, destination, and size based on a table, then executes a loop of blitter transfers.
- `merge_memory_tables`: Performs a bitwise OR operation on four different words from a source table and stores the result in a destination table in a loop.
- `dispatch_blitter_tasks`: Calculates blitter parameters and source/destination addresses based on a structure in a0, then executes a loop of blitter transfers and updates the structure in a0.
- `setup_blitter_loop`: Configures several blitter registers via a pointer to a hardware base address, then enters a loop that calls another routine five times.
- `render_blitter_structure_elements`: Calculates dimensions and coordinates for a blitter operation based on values in a structure, then calls another routine in a loop to perform multiple blitter transfers.
- `wait_for_vblank`: Loops until the value at the VHPOSR register is 60, then branches to another location.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `set_global_counters`: Clears a 16-bit global variable, sets another 16-bit global variable to 255, and calls another routine.
- `reset_and_check_counters`: Clears a 16-bit global variable, sets another 16-bit global variable to 255, and calls check_global_counters.
- `check_global_counters`: Checks two 16-bit global variables and calls another routine if either is zero, otherwise it returns.
- `copy_word_table`: Decrements a 16-bit global variable and, if it reaches zero, copies 32 words from a source table pointed to by a global variable to a destination table at a fixed address, then calls another routine.
- `decrement_global_counter_2`: Decrements a 16-bit global variable at offset 7259c; if the result is not zero, it branches to another location, otherwise it sets the global variable to 1 and sets d5 to 1.
- `process_bitmask_table_2`: Iterates through a table of words using nested loops to perform conditional bitwise masking or subtraction, then increments a global variable and potentially resets it and two other global variables if it reaches ten.
- `write_dma_control`: Writes the value 271 to the DMA control register and returns.
