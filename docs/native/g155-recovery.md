# Game 155 (AlienWorld v1.0 1398): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00155). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2476153 top-level bytes in 40 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 221 of 221 named (0 author symbols), 221 of 221 described (14 by rule, 186 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2974 of 2974 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 355790 bytes, 6 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5104 of 5132 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 716466 |
| code | 19730 |
| data | 1739830 |
| padding | 127 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 121 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 738 | 25 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3571 | 146 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 108 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 156 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 126 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 82 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| alien.prg (raw code file, load address unknown) | 1 | 1 | 1 |
| intro.prg (raw code file, load address unknown) | 1 | 218 | 5 |
| lev2.prg (raw code file, load address unknown) | 1 | 0 | 0 |
| lev3.prg (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- AlienWorld_v1.0_1398/AlienWorld_v1.0_1398.hdf: hard-disk image (AmigaDOS OFS, 2688 blocks): its filesystem parses and all 30 files in it are byte-identical to the extracted tree AlienWorld_v1.0_1398_hdf, which is recovered file by file; no block outside the filesystem holds data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 475 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1954 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 88 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- AlienWorld_v1.0_1398/AlienWorld_v1.0_1398.hdf: 132 executed PCs whose RAM was overwritten before any snapshot match its bytes 1040400-1041024: recovered as a unit run at an address
- AlienWorld_v1.0_1398/AlienWorld_v1.0_1398.hdf: 120 executed PCs whose RAM was overwritten before any snapshot match its bytes 848400-848928: recovered as a unit run at an address
- AlienWorld_v1.0_1398/AlienWorld_v1.0_1398.hdf: 82 executed PCs whose RAM was overwritten before any snapshot match its bytes 1040912-1041296: recovered as a unit run at an address
- 4 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- AlienWorld_v1.0_1398/AlienWorld_v1.0_1398_hdf/AlienWorld/data/alien.prg: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AlienWorld_v1.0_1398/AlienWorld_v1.0_1398_hdf/AlienWorld/data/intro.prg: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AlienWorld_v1.0_1398/AlienWorld_v1.0_1398_hdf/AlienWorld/data/lev2.prg: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AlienWorld_v1.0_1398/AlienWorld_v1.0_1398_hdf/AlienWorld/data/lev3.prg: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- AlienWorld_v1.0_1398/AlienWorld_v1.0_1398_hdf/AlienWorld/data/lev1.prg: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- 4 never-loaded region(s), 355790 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- intro.prg (raw code file, load address unknown)

## Copy protection found

- `test_and_copy_word` (runtime segment an address): Loads a word from offset 18 of a structure, masks it, and if the result is zero, clears bit 2 of a hardware register, calls another routine, and stores the original word from offset 18 into offset 16 of the structure. Kind: checksum/status trap. Pass path: the branch where DAT_00bfd100 is modified by / 4 (setting a bit flag).
- `set_bit_and_copy` (runtime segment an address): Sets bit 2 of a hardware register, calls another routine, and then copies a 16-bit word from offset 18 of a structure in a5 to offset 16 of the same structure. Kind: checksum or integrity trap. Pass path: the instruction following the call (the copy operation).
- `set_hardware_bit_and_branch` (runtime segment an address): Sets bit 1 of a hardware register, checks bit 4 of another hardware register, and if that bit is clear, calls a subroutine before branching to another location. Kind: disk hardware/track check. Pass path: beq.w an address.
- `set_bit_and_decrement` (runtime segment an address): Sets bit 1 of a hardware register, calls another routine, and decrements a word at offset 16 of a structure by 2. Kind: checksum or integrity trap. Pass path: the branch following the bit manipulation (likely the code path that continues normal execution if the bit is set correctly).
- `clear_and_set_bit` (runtime segment an address): Clears bit 0 of a hardware register, sets bit 0 of the same register, and calls another routine. Kind: bit-manipulation based status check/trap. Pass path: sub_2752 (the branch that increments the counter/status).
- `clear_set_bit_and_increment` (runtime segment an address): Clears bit 0 of a hardware register, calls another routine, increments a 16-bit value at offset 16 of a structure in a5, and returns. Kind: bitmask check and counter increment. Pass path: bra.w an address.
- `configure_hardware_and_wait` (runtime segment an address): Writes values to hardware registers, calls another routine, copies a 16-bit word from offset 18 of a5 to offset 126 of a hardware register structure, writes a value to offset 36 of a structure in a6, and enters a loop that checks bit 1 of a value at offset 30 of a5 before either returning or entering an infinite loop. Kind: disk hardware/sync check. Pass path: bmi.w an address.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `set_copper_list_pointers`: Writes the copper list location registers for the first and second copper lists.
- `clear_and_xor_decode_stream`: Initialises the interrupt and DMA control registers, clears a block of sixteen thousand long words and XOR-decodes a stream of records into it, using escape codes to repeat or skip bytes.
- `run_frame_update_loop`: Runs a loop that calls routines for each active object and advances per-object frame counters, branching on their values, and calls a pointer when a longword matches the text MON colon.
- `call_table_routine_keep_registers`: Calls a routine through a function pointer held in a table at a fixed offset and returns the data registers unchanged.
- `call_external_function_2`: calls a function through a pointer stored in a global table and returns a constant value.
- `patch_instructions_to_jsr`: Scans memory up to an end address and replaces three specific instruction patterns with calls to three fixed low-memory addresses, patching the code in place.
- `patch_custom_register_references`: Scans a range of words and sets the top four bits of each word whose value falls inside the window around the custom chip register block.
- `start_blit_and_wait`: Writes a blitter size value to the blitter size register, then waits for the blitter to finish.
- `store_word_and_branch`: Stores a word from register d3 into a global variable at offset 88 of the structure in a6, then branches to another routine.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `check_byte_range`: Rotates register d0, negates it, subtracts 1, and compares it to a byte at offset 31 of the program counter; if the result is 0 or if d0 equals 95, it returns, otherwise it returns.
- `halt_in_infinite_loop`: Loops forever with no exit.
- `call_external_function`: Calls a function through a pointer stored in a global table and returns the combined results of two registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `increment_and_copy_table_entry`: Increments a word at offset 54 of a structure and, if it is between 64 and 111 and its lower three bits are zero, calls another routine to copy data from a source defined by offsets 2 and 4 of the structure.
- `increment_and_copy_table_entry_alt`: Increments a word at offset 54 of a structure and, if it is between 96 and 127 and its lower three bits are zero, calls another routine to copy data from a source defined by offsets 2 and 4 of the structure.
- `store_byte_and_return`: Stores the byte value 255 into the address pointed to by a0 and returns.
- `update_structure_fields`: Checks if a value at offset 2 of a pointer in a0 is within a specific range relative to a global variable; if so, it updates several fields in the structure at a0, including setting a pointer to the global variable and potentially calling another routine, otherwise it sets a byte at offset 23 to 4.
- `process_nested_data_table`: Checks a short at offset 294 and a byte at offset 8 of a structure, then enters a loop that iterates up to five times to call another routine using data from a table and a global variable.
- `increment_counters_and_check_bounds`: Increments a word at offset 34 of a structure in a0 and checks if words at offsets 2 and 4 are within specific bounds.
- `update_and_copy_offset_data`: Increments a word at offset 36 of a structure and calls another routine to copy data using values from offsets 2 and 0 of a3 and offsets 4 and 10 of a3 to calculate a destination.
- `update_entity_position_and_status`: Decrements a counter in a structure and updates a status byte in a structure, then calculates new coordinate offsets by adding values from a table to existing coordinates.
- `clear_structure_data`: Checks bit 5 of a byte at offset 15 of a structure in a0; if bit 5 is clear and a word at offset 2 of a0 is non-negative, it copies three words from a0 to offsets 514, 516, and 518 of a structure in a5, clears several fields in the structure in a0, and sets a word at offset 22 to 64 and a pointer at offset 5 to a global variable.
- `reset_status_on_threshold`: Checks if a word at offset 286 of a structure in a5 is positive; if it is and a word at offset 2 of a0 is greater than 126, it sets a word at offset 290 of a structure in a5 to 2 and clears the word at offset 286 of a structure in a5.
- `set_negative_status`: Checks if a word at offset 286 of a structure in a5 is negative; if so and a word at offset 2 of a0 is less than 128, it sets a word at offset 290 of a structure in a5 to 65534 and clears the word at offset 286 of a structure in a5.
- `update_status_flags`: Clears a longword at offset 22 of a structure in a5, sets a byte at offset 27, and calls two different routines.
- `calculate_status_from_table`: Calls a routine and then performs two conditional checks on a short at offset 288 of a structure, using values from a table and offset 248 of the structure to update offset 288.
- `bitwise_or_table_elements`: Calls another routine, then performs two loops that bitwise OR bytes from two different source locations into bytes at offsets 31 and 20 of a structure in a5, using counts from offsets 286 and 288 of that structure.
- `update_status_and_counters`: Performs two loops that bitwise OR bytes from offsets 172 and 164 of a structure in a5 into offsets 19 and 18 of that structure, then conditionally updates a 16-bit value at offset 286 of the structure based on calculations involving a global structure and a value from a0.
- `copy_indexed_bytes`: Clears two longwords at the address in a1, calculates offsets using values from a0, a3, and a5, and then enters a loop that copies bytes from a table into the destination at a1 using indices derived from two different source locations.
- `copy_bytes_with_offset`: Calculates source and destination addresses using values from a0, a3, and a5, then enters a loop that copies bytes from a table into a destination at a1 with specific offsets of 16 and 24 relative to the destination pointer.
- `reset_structure_fields_on_bit_test`: Calls another routine, tests bit 5 of a byte at offset 15 of a structure in a0, and if set, writes a longword to offset 20, clears offsets 16, 16, and 18, sets offset 14 to 32, and calls another routine.
- `process_collision_and_trigger_event`: Iterates through a table of objects to check for spatial overlap with a coordinate-based bounding box derived from a structure in a0.
- `copy_data_to_table`: Iterates through a table starting at a specific address, then copies several words and longwords from a source in a0 to a destination table, and finally calls another routine and potentially calls it again based on a value at an offset of the returned pointer.
- `load_pointer_from_table`: Checks if a word at the address in a0 is non-negative; if so, it shifts the word left by 2 and uses it as an offset to load a longword from a table into offset 6 of the structure in a0.
- `populate_table_from_source`: Iterates through a table using a pointer in a1, extracting a source address and two 16-bit offsets to call copy_data_to_table, then either stores the resulting pointer into a structure at offset 356 of a5 or continues the loop based on a signed 16-bit value in a1.
- `update_structure_values`: Checks bit 5 of a byte at offset 15 of a structure in a0; if set, it stores a byte at offset 0 of a0 and, provided a 16-bit value at offset 50 of a5 is not 6, increments both offsets 50 and 52 of a5 by 2 before calling another routine.
- `clamp_structure_values`: Compares a 16-bit value at offset 4 of a0 plus 48 against 256, and if that condition is not met, compares a 16-bit value at offset 2 of a0 plus 48 against 448; if that condition is also not met, it writes 65534 to offset 0 of a0 and 32 to offset 14 of a0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_subroutines`: Checks a byte at a hardware register, then writes several longwords to a table at a PC-relative address, sets multiple hardware registers to specific values, and calls three different routines before returning.
- `store_word_split`: Stores the lower 16 bits of d0 at offset 6 and the upper 16 bits of d0 at offset 2 of a structure located via the program counter, then writes 215 to a global variable.
- `store_vhpposr_split`: Reads a 16-bit value from the VHPOSR hardware register, stores it at offset 0 of a5, and stores the bitwise NOT of that value added to the original value at offset 2 of a5.
- `process_bit_manipulation`: Performs arithmetic and bitwise operations on values from a5 and a custom hardware register, stores the results in a5, and returns a 32-bit value derived from the modified data.
- `update_joystick_state`: Reads the joystick port from a custom hardware register, performs bitwise logic to determine direction, updates bits in a structure at a5, and handles a byte from the CIAA serial register based on a flag in a5.
- `handle_serial_interrupt`: Checks bit 3 of a global variable; if set, it performs a sequence of writes to CIAA registers and calls another routine, then stores a byte from the serial register into offset 9 of a5 before returning.
- `copy_and_execute_data`: Increments a global variable, adjusts it based on a comparison, copies data from a source table to a destination structure in a5 using two different loops, and finally calls a function pointer stored at offset 368 of a5.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `update_audio_hardware`: Checks a byte at offset 29 of a structure in a5 and compares a value to a hardware register.
- `write_address_table`: Runs a loop four times to store 32-bit values into a table at offset 6 and offset 2 of a structure, incrementing the values by 36864 each iteration, and finally writes 4896 and 3 to offsets 14 and 10 of the structure.
- `store_word_split_swap`: Stores the lower 16 bits of d0 at offset 6 and the upper 16 bits of d0 at offset 2 of a structure located via the program counter, using swap instructions to rearrange the bits.
- `initialize_data_table`: Runs a loop four times to store a 32-bit value, split into two 16-bit parts, into a table at 8-byte intervals, incrementing the value by 9728 each iteration.
- `initialize_data_table_offset`: Runs a loop four times to store a 32-bit value, split into two 16-bit parts, into a table at 8-byte intervals, incrementing the value by 1568 each iteration.
- `clear_memory_blocks`: Runs a nested loop to clear memory by writing zero to three blocks of 2048 words each, with the starting address of each block increasing by 36864.
- `clear_two_memory_regions`: Clears a memory region of 8192 words with zero, then clears a second memory region at offset 31600 with 2048 words of zero.
- `update_sprite_attribute_tables`: Calculates a destination address for a table based on values in a structure, then performs nested loops to copy data from a source table into multiple destination buffers.
- `fill_incrementing_values`: Runs a loop 159 times to store an incrementing 32-bit value, increasing by 48 each time, into a table at 4-byte intervals.
- `setup_blitter_transfer`: Configures several hardware registers for a blitter operation, including setting the source address, destination address, and control parameters, then calls a routine at offset 204.
- `clear_word_table`: Runs a loop 1879 times to write zero into a table at offset 4600, moving forward by 4 bytes each iteration.
- `update_blitter_sprite_coordinates`: Checks for equality between two 16-bit values at offsets 246 and 250 of a structure and performs a swap if they differ.
- `swap_structure_pointers`: Copies three 32-bit values from one set of offsets in a structure to another, then conditionally swaps the order of two of those values based on a byte at offset 13.
- `execute_blitter_loops_2`: Sets several hardware registers for the blitter, then enters a loop that runs four times to call another routine with updated destination addresses derived from the input pointers.
- `execute_blitter_loops`: Calculates a destination address based on several values in a structure, then runs a nested loop to perform three blitter operations via a routine at offset 198, updating the destination and source pointers.
- `copy_structure_field`: Copies a 32-bit value from offset 246 of a structure to offset 250 of the same structure.
- `execute_jump_table_entry`: Loads a 16-bit word from an offset of 284 in a structure pointed to by a5, uses it as an index into a jump table located via a PC-relative address, and jumps to the resulting address.
- `reorder_and_call_structure_fields`: Copies three longwords from a structure into new locations within the same structure, inverts a byte at offset 13, and calls another routine using a value from offset 48 if the byte is not zero; otherwise it returns.
- `copy_structure_fields`: Copies a 32-bit value from offset 4c of a structure to offset 48 and another 32-bit value from offset 50 to offset 70, then calls another routine.
- `update_address_table`: Writes three 32-bit values to a structure, modifies those values by adding an offset derived from a field at offset 246, updates a global word and a byte in the structure, and then calls another routine either once or twice based on a bit test.
- `update_entity_linked_lists`: Traverses a linked list of data structures to update counters and state, performing bitwise checks and arithmetic to manage object properties.
- `update_linked_list_counters`: Increments counters within a structure and updates their values by traversing linked lists of data blocks.
- `update_sprite_attributes_and_render`: Updates a structure with new coordinate and attribute data derived from a counter and a base offset, then calls a subroutine four times to render the object at different memory offsets.
- `calculate_offset_and_call_subroutine`: Performs conditional arithmetic on d0 based on a byte at offset 13 and a word at offset 284 of a structure in a5, stores the result in a structure in a0, and then calls a subroutine four times using values from a2 and a6.
- `update_sprite_animation_state`: Updates counters in a structure based on a bitmask check, then calculates sprite frame offsets and calls a subroutine four times to update sprite positions at various memory offsets.
- `update_blit_registers_and_loop`: Configures hardware registers at a fixed address, iterates through a list of structures using a dbra loop to perform four calls to a subroutine, and conditionally updates several long words in a structure pointed to by a5.
- `jump_via_indirect_table`: Loads a pointer from a structure at offset 152 of a5 into a6, uses a word from a pointer at a2 to index into a table, and jumps to the address found there.
- `process_nested_loops`: Calculates a value based on two fields in a structure, then enters a triple-nested loop that calls another routine and stores results into a destination buffer, finally performing an indirect jump via a table.
- `merge_and_store_nibbles`: Reads four 16-bit words from a source at a0 with an offset in d4, stores them into a destination at a1, and returns a bitwise OR of those four words.
- `store_pointers_to_structure`: Stores the value of a6 into a structure at offset 152 of a5 and stores the value of a1 into a structure at offset 148 of a5.
- `store_and_branch`: Stores a word from a pointer at a2 into a structure at offset 262 of a5 and then branches to another routine.
- `store_and_branch_offset_264`: Stores a word from a pointer at a2 into a structure at offset 264 of a5 and then branches to another routine.
- `store_and_branch_offset_270`: Stores a word from a pointer at a2 into a structure at offset 270 of a5 and then branches to another routine.
- `update_pointers_and_branch`: Stores the value of a6 into a structure at offset 132 and the value of a1 into a structure at offset 136, then branches to another routine.
- `update_timer_and_execute_blitter_loop`: Compares a longword from a hardware register with a value at offset 340 of a structure in a5, updates offsets 340 and 342, performs a bitwise NOT on a byte at offset 230, executes a loop that calls a routine via a pointer to perform blitter operations, and calls three other routines.
- `sync_byte_pair`: Compares a byte at offset 62 of a structure with a byte at offset 66, and if they differ, calls another routine with a pointer to a constant and the value 2 in d7; it then compares a byte at offset 63 with a byte at offset 67, calls another routine if they differ, and finally stores a word from offset 62 into offset 66 of the structure.
- `calculate_and_apply_pointer_offsets`: Shifts and masks two values, then calculates two separate pointers by adding the masked values to a base address from a global variable and calls another routine using the first calculated pointer before preparing the second pointer.
- `copy_bytes_to_sparse_offsets`: Copies seven bytes from a source at a2 to a destination at a0 using specific offsets of 0, 40, 80, 120, 160, 208, and 240.
- `sync_byte_pair_offset`: Compares a byte at offset 54 of a structure with a byte at offset 56, and if they differ, calls another routine with a pointer to a constant and the value 2 in d7; it then compares a byte at offset 55 with a byte at offset 57, calls another routine if they differ, and finally stores a word from offset 54 into offset 56 of the structure.
- `call_process_single_byte`: Loads a byte from offset 59 of a structure into d0, loads a constant address into a0, sets d7 to 1, and calls another routine.
- `copy_nibble_tables`: Copies 16 words from a table pointed to by offset 128 of a structure to a destination, updates the pointer in offset 128, copies 16 words from a second table pointed to by offset 136 to another destination, updates the pointer in offset 136, and calls another routine.
- `decrement_nibbles_in_tables`: Checks a constant at a specific memory location, calls a routine if it matches, and then enters a loop that calls another routine to decrement nibbles in four different tables before potentially repeating the entire process.
- `decrement_nibbles_in_array`: Enters a loop that runs seven times, calling another routine to decrement nibbles in two consecutive words at a time, and then calls that routine one final time on a word at a different offset.
- `subtract_nibbles_from_word`: Checks the low, middle, and high nibbles of a 16-bit word in d0 and subtracts one from each non-zero nibble before returning the combined result.
- `store_and_update_globals`: Sets a value at offset 128 of a hardware register, stores the contents of a0 and a1 into offsets 392 and 396 of a structure, calls another routine, and then stores those same values from the structure into offsets 128 and 136 of a structure.
- `process_nibble_data_sequences`: Checks a magic constant in memory and calls a function via a pointer if it matches, otherwise it performs three distinct loops to process data from three different source buffers into destination arrays using a sub-routine.
- `merge_nibbles_from_two_words`: Compares the nibbles of d1 and d5 and performs a series of bitwise OR operations to merge the nibbles of d5 into d1 based on the comparison results.
- `increment_word_by_nibble_mismatch`: Compares the nibbles of a 16-bit word from a structure in a0 with the nibbles of d0 and increments the word by 1, 16, or 256 depending on which nibbles do not match.
- `process_word_nibbles`: Loops seven times, calling another routine to increment a 16-bit word from a source list using nibble-based logic and storing the result back into a destination structure.
- `initialize_data_tables`: Sets two global word values, calls two routines, and then performs a nested loop to copy four longwords at a time from one memory region to another destination table.
- `manage_state_and_tables`: Clears a byte at offset 30 of a structure, performs several setup routines including clearing a byte at offset 8, calls another routine to process a table, manages a loop that executes a function pointer from a structure, and finally performs two nested loops that call another routine to modify table values.
- `branch_with_constant`: Loads the constant 7936 into register d2 and branches to another location.
- `copy_nested_blocks`: Performs a nested loop to copy word-sized blocks of data from one memory location to another, using a third loop to manage offsets and increments.
- `initialize_memory_and_copy`: Clears 799 bytes of memory starting at offset 22528 in a loop, sets a value at offset 364 of a structure, calls another routine, and then copies 15 words from a table to a destination starting at offset 2 of a pointer.
- `extract_and_store_parameters`: Calls another routine, then extracts four values from a list in a6 and stores a pointer to a6 plus 12 into a global variable at offset 364 of a5, followed by a call to another routine.
- `clear_memory_block`: Fills a memory area starting at a fixed address with zeros for a total of 2880 words.
- `interleave_and_store_data`: Calls another routine, then performs a loop 10 times where it combines four values from a source table into a single longword and stores it at an incrementing destination, then conditionally calls another routine based on a byte at offset 30 of a structure.
- `copy_structured_data`: Copies ten words from a source pointer and, for each word, copies three additional longwords from specific offsets within the source to the destination.
- `update_nibble_table`: Calls another routine, then performs a loop 16 times where it reads a word from a table, calls another routine to modify it, and writes the result back to the table, finally checking a byte at offset 30 of a structure.
- `parse_and_copy_data`: Calls another routine, then parses a byte stream to calculate an offset based on character values, and performs nested loops to copy blocks of data from a source to a destination table.
- `interleave_structured_data`: Calls another routine, then performs a loop 72 times where it combines four words from a source table into a single word and stores it at an incrementing destination, then conditionally calls another routine based on a byte at offset 30 of a structure.
- `copy_interleaved_data`: Copies seventy-one words from a source to a destination, while also copying three specific longwords from fixed offsets within the source to corresponding offsets in the destination.
- `clear_and_interleave_data`: Calls another routine, then performs a loop 72 times where it combines four words from a source table into a single word, stores it at a destination, and clears three specific words at fixed offsets from that destination, then conditionally calls another routine.
- `clear_structured_data`: Clears a series of words at specific offsets within a structure, repeating this for seventy-one iterations with an incrementing pointer.
- `update_status_byte`: Calls two routines and, if a byte at offset 8 of a5 is non-zero, stores 255 into a byte at offset 30 of a5.
- `initialize_and_process_data_table`: Clears a table, updates hardware registers, and executes a countdown loop based on a value in a structure.
- `decrement_table_values`: Calls another routine, fills a table with 16 words of 4095, then performs a nested loop 16 times where it compares words from a table and a source, decrementing the value in the table by 1, 16, or 256 depending on how many nibbles differ.
- `check_and_call_handler`: If a value at offset 4 of a table is 18767, it calls a routine via a pointer, then if a word at offset 34 of a structure is greater than 1, it clears that word and calls two other routines.
- `execute_loop_sequence`: Calls another routine and then enters a loop that executes that routine 16 times using a decrementing counter.
- `initialize_and_process_tables`: Clears 16 words at a table pointed to by a0, fills a table at offset 88 of a structure with 7999 instances of zero, calls another routine, clears a word at offset 504 of a structure, calls another routine, sets a word at offset 504 of a structure to zero, calls another routine, clears offset 70 of a structure, clears 7 words at a table, and calls another routine.
- `apply_xor_commands`: Reads bytes from a source list and performs XOR operations on a destination table, handling specific command bytes to either perform single XORs, multi-byte XORs, or jump to another routine.
- `process_conditional_logic`: If a byte at offset 9 of a structure is 183, it clears that byte and, if a byte at offset 29 is zero, clears offset 70, calls another routine, sets offset 29 to 255, and calls another routine three more times; otherwise, it calls another routine, clears two global bytes, sets offset 29 to zero, calls another routine, sets two global bytes, and calls another routine twice.
- `store_registers_to_memory`: Stores the values of registers d0, d1, and d2 into three consecutive memory locations at a fixed offset from the program counter.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `process_bit_and_search_table`: Toggles a bit in a memory location using a value from a4, stores several values into a structure in a5, calls four routines, and if a condition is met, performs bitwise operations on values from a structure and calls another routine.
- `store_negative_constant`: Loads the constant negative 25 into register d0, stores it as a longword at offset 52 of a5, and branches to another location.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `interleave_and_copy_data_tables`: Sets a word at offset 18 of a structure in a5, calls two routines, copies a block of data from one pointer to another, performs bitwise interleaving on bytes from a source to a destination table, and uses a table to perform sixteen lookups.
- `branch_with_constant_2`: Loads the constant negative 6 into register d0 and branches to another location.
- `find_matching_entry_2`: Calls another routine, then searches through a table starting at offset 152 of a pointer found at offset 4 of a structure, comparing longwords and words at each entry against a constant table, and returns the pointer to the matching entry or zero if no match is found after 52 iterations.
- `find_matching_entry`: Increments the pointer in a0 by 16, then enters a loop controlled by d7 that branches to another location; otherwise it sets d0 to zero and returns.
- `copy_string_limited`: Copies characters from a source to a destination until a null terminator or a newline character is encountered, up to a maximum of ten characters.
- `configure_dma_and_interrupts`: Writes a value to a memory location, disables interrupts, and performs multiple writes to hardware registers to configure DMA and custom hardware settings.
- `set_dma_register`: Adds three to a value from a global variable, stores the result in d0, and writes the constant value 33296 to a hardware register.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `get_modified_value`: Reads a value from a global variable, adds three to it, and returns the result in d0.
- `compare_and_branch_on_values`: Compares a word at offset 18 of a structure in a5 with a word at offset 16, and if they are not equal, performs bitwise shifts on both values to compare them, then calls another routine if a carry flag is set.
- `branch_to_subroutine`: Calls a subroutine and then branches to another location.
- `test_and_copy_word`: Loads a word from offset 18 of a structure, masks it, and if the result is zero, clears bit 2 of a hardware register, calls another routine, and stores the original word from offset 18 into offset 16 of the structure.
- `set_bit_and_copy`: Sets bit 2 of a hardware register, calls another routine, and then copies a 16-bit word from offset 18 of a structure in a5 to offset 16 of the same structure.
- `set_hardware_bit_and_branch`: Sets bit 1 of a hardware register, checks bit 4 of another hardware register, and if that bit is clear, calls a subroutine before branching to another location.
- `clear_word_at_offset`: Clears a 16-bit word at offset 16 of the structure pointed to by a5 and returns.
- `set_bit_and_decrement`: Sets bit 1 of a hardware register, calls another routine, and decrements a word at offset 16 of a structure by 2.
- `clear_and_set_bit`: Clears bit 0 of a hardware register, sets bit 0 of the same register, and calls another routine.
- `clear_set_bit_and_increment`: Clears bit 0 of a hardware register, calls another routine, increments a 16-bit value at offset 16 of a structure in a5, and returns.
- `check_global_variable_equality`: Compares a global variable against its own value after an offset is applied.
- `process_interleaved_data`: Compares two values in a structure and, if they differ, performs bitwise interleaving on a data stream and calculates a sum of values from a table.
- `configure_hardware_and_wait`: Writes values to hardware registers, calls another routine, copies a 16-bit word from offset 18 of a5 to offset 126 of a hardware register structure, writes a value to offset 36 of a structure in a6, and enters a loop that checks bit 1 of a value at offset 30 of a5 before either returning or entering an infinite loop.
- `write_word_to_offset`: Writes the value 16384 to offset 36 of the structure pointed to by a6 and returns.
- `calculate_division_and_copy_block`: Divides a word at offset 48 of a structure in a5 by 6, stores the quotient and remainder in offsets 18 and 50, calls two routines, and then either copies a block of bytes or calls a function pointer based on a bit test and a comparison.
- `copy_block_and_increment`: Copies a block of data from a source to a destination and increments two pointers by a fixed amount.
- `call_function_and_update`: Stores a byte from one location, calls a function pointer, and updates a memory location and a pointer.
- `copy_table_and_call_routine`: Copies 49 words from a table to a destination starting at a global variable using a loop, then calls another routine four times.
- `update_bitmask_and_data`: Checks a byte at offset 29 of a5, performs a series of checks on values in a table to set a bitmask at offset 346 of a5, copies a block of data from a table to a destination in a0, calls another routine, and adds the result to a 16-bit value at offset 2 of a0.
- `update_audio_and_data`: If a byte at offset 29 of a structure is not zero, it performs a series of calculations involving a table to update a global variable, writes specific values to audio registers, and executes a loop that modifies memory based on table data.
- `reset_audio_volumes`: Sets the four audio volume registers to zero and writes the value 15 to offset 150 of the structure pointed to by a5.
- `update_entity_audio_and_status`: Checks a global variable and, if non-zero, iterates through a structure to update audio hardware registers and entity properties based on a switch-like logic.
- `copy_byte_from_memory`: Compares a byte at a global memory location with d4, and if it is greater than 64, copies that byte from memory into offset 5 of a structure in a2 before branching to another location.
- `store_word_and_branch_2`: Stores a word from d6 into offset 2 of a structure in a6 and then branches to another location.
- `lookup_and_store_byte`: Stores a word from d6 into offset 2 of a structure in a6, loads a byte from a table using a value from a global memory location as an index, and stores that byte into offset 7 of the structure in a6 before branching to another location.
- `transfer_byte_and_branch`: Stores a byte from register d4 into offset 1 of the structure in a6 and branches to another routine.
- `update_entity_status`: Compares a value in a register with a 16-bit integer at offset 36 of a structure in a2, then updates a byte at offset 50 and a word at offset 26 of that structure based on the comparison result.
- `update_entity_values`: Subtracts a byte from offset 3 of a structure in a0 from a word at offset 24 of the same structure, clamps the result to a minimum of 113, and stores the result in offset 6 of a structure in a4.
- `update_entity_properties`: If a byte at offset 3 of a structure in a0 is non-zero, it is copied to offset 29.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `process_state_machine_and_event_loop`: Updates a global structure by setting various fields and calling multiple routines in a sequence, then enters a nested loop that performs conditional checks on a global variable and updates a series of registers and memory locations based on the results of other routines.
- `call_indexed_function`: Calls a function from a table using a word from offset 38 of a structure as an index, then conditionally updates a global variable and a byte at offset 9 of the structure based on a comparison with 25.
- `initialize_audio_and_memory`: Calls several routines to set up data, writes a value to offset 152 of a structure, copies a pointer to a global variable, and clears two bytes at offsets 327A and 327B of a pointer in d0.
- `setup_data_structure`: Calls a routine three times, writes two longwords to offsets 128 and 136 of a structure, and initializes several fields in the structure including offsets 38, 48, 17, 50, 52, 58, and 62.
- `update_entity_state_and_render_buffer`: Clears a memory block, updates a structure with data from a table including several 32-bit and 16-bit values, and performs complex calculations to update multiple offsets in a structure.
- `execute_sequence_of_subroutines`: Calls seven different routines in sequence and then returns.
- `process_buffer_parameters`: Calls two routines using values from offsets 242 and 244 of a structure and a pointer in a0.
- `load_long_from_offset`: Loads a 32-bit word from offset -4 relative to a0 into d7.
- `copy_scaled_block`: Copies a block of 32-bit words from a source to a destination, where the number of iterations is determined by shifting d7 right by two and subtracting one.
- `handle_serial_input`: If a byte at offset 12 of a structure is zero, it calls a routine, compares a byte from a table to a byte at offset 9 of the structure, and if they match, increments offset 36 and sets bit 7 of offset 9.
- `fill_nested_tables`: Performs a nested loop to fill a memory area starting at a specific address with 16-bit words, alternating between 65535 and 0.
- `clear_and_reset_table`: Clears bit 7 of a byte at an offset calculated from a value in a global variable, then enters nested loops to reset a series of words in a global structure to zero and set a word at the start of that structure to negative one.
- `subtract_values_from_table`: Subtracts two 16-bit values from two consecutive 16-bit words in a table at offset 2 and offset 4, then repeats this process for 46 iterations.
- `update_index_and_copy_data`: Increments a 16-bit value at offset 300 of a structure, and if it exceeds 3, resets it and increments a value at offset 302, then uses the value at offset 302 multiplied by 6 to copy a 32-bit double-word from a table into a destination pointed to by a6.
- `increment_and_rotate_data`: Increments a 16-bit value at offset 300 of a structure, and if it exceeds 1, resets it and performs a 32-bit rotation by moving the lower word of the data pointed to by a1 into the upper word position.
- `update_clamped_value`: Calculates a value by adding a word from a table to a value in a structure, clamps the result between 0 and 152, updates a counter in a structure, and writes a character derived from the clamped value to a global variable and a structure offset.
- `rotate_data_with_counter`: Increments a 16-bit value at offset 300 of a structure, and if it exceeds 1, resets it and rotates a 32-bit value by swapping its upper and lower 16-bit words.
- `clear_and_process_buffer`: Clears bit 7 of 4000 bytes starting at a fixed address, then calculates an offset into a buffer to process 9 rows of data, where each row contains 24 elements that are modified and passed to another routine.
- `process_conditional_buffer_segments`: Compares values in a structure to determine which segment of a buffer to process, then iterates through elements in the selected segment to set bit 7 and call another routine.
- `process_bcd_and_call_subroutine`: Performs BCD arithmetic on values at offsets 64 and 66 of a structure in a5, clears a word at offset 66, and calls another routine if a byte at offset 62 meets specific criteria.
- `process_bcd_values`: Adds a byte from offset 3d of a structure in a5 to the byte at offset 3b of the same structure, performs a BCD adjustment, adds the byte at offset 3c to the byte at offset 3a, performs another BCD adjustment, clears a word at offset 3c, and calls another routine.
- `process_table_entries`: Decrements a word at offset 338 of a structure in a5 if the byte at offset 339 is non-zero, and if bit 0 of that byte is set, sets the word at offset 338 to 255.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `start_copper_and_interrupts`: The decompiler timed out, so only facts are known: it touches the copper list pointer, the copper jump strobe, the DMA control register, the interrupt enable register and a parallel port status register.
- `build_pointer_table_from_index`: Builds six pointer halves that step by the input value, writes a decimal digit derived from the input into a text location, copies sixteen words from a table selected by the input, and sets fixed words in the state record.
- `store_word_then_call_routine`: Stores a fixed word into a global location and calls an unnamed routine, then writes a word with the top bit clear and all other bits set to a register at a fixed offset from the base pointer.
- `wait_for_global_marker_byte`: Spins until a global byte equals a period character, then returns.
- `set_flag_or_spin_forever`: If a bit in a global is already set, it spins forever; otherwise it sets that bit and returns.
- `store_three_registers_2`: Stores the contents of three 32-bit registers into three consecutive global longwords.
- `stub_unrecovered_single_instruction_2`: The decompiler timed out, so only facts are known: a single instruction, called from a tiny routine and one unnamed routine.
- `stub_immediate_return_3`: Returns immediately with no body and no calls.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `return_with_jump`: Returns immediately or performs a short branch to a location 192 bytes ahead.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_runtime_memory_tables`: Clears a block of memory and then uses a pointer table to perform a sequence of XOR-based memory modifications.
- `initialize_pointer_table`: populates a table in memory with a sequence of address-length pairs, where each address points to a new location and a fixed length of six is assigned to each.
- `update_global_counters`: This routine checks a bit in a hardware register, increments two global variables, and potentially increments a pointer before calling another routine.
- `reset_memory_value`: This routine loops through a memory location until a certain condition is met, then sets a value at a pointer-based address to zero.
- `configure_dma_and_audio`: This routine toggles a boolean, checks a global, and uses a loop to write instruction or data or values to hardware registers or custom addresses based on bit-masked values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_memory_and_interrupts`: Sets a global variable and certain memory or register-related values, then clears a block of memory using a loop, and performs an XOR-based bit manipulation on a memory range.
- `empty_routine`: the routine performs no operations and contains no valid instructions or executable logic.
- `execute_unreached_routine`: This routine consists of 5 instructions and is unreachable as a gap in the code.
- `execute_unknown_routine_3`: This routine consists of 27 instructions and is called by 1 unnamed routine.
- `modify_interrupt_request`: This routine consists of 12 instructions and touches the interrupt request register or a similar hardware register.
- `execute_unknown_routine_4`: This routine contains three instructions and is called by another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_audio_registers`: This routine modifies certain audio hardware registers and a custom register.
- `update_audio_parameters`: This routine modifies several audio hardware registers including volume and period registers.
- `execute_subroutine_sequence`: Executes a routine that contains thirty instructions and is called by another routine.
- `execute_called_routine`: This routine contains thirty instructions and is called by another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_and_call_value`: Sets a 16-bit value at offset 150 of a structure to a value derived from bitwise ORing 32768 with 0, then calls a routine at a long address.

### alien.prg (raw code file, load address unknown)

- `stub_raw_region_entry`: The decompiler timed out, so only facts are known: one instruction at the start of a raw code region in the alien data file.

### intro.prg (raw code file, load address unknown)

- `configure_copper_and_interrupts`: The decompiler timed out, so only facts are known: a 73-instruction routine at the start of a raw intro code region that touches the copper list pointer, DMA control, interrupt enable and interrupt request registers.
- `test_interrupt_request_register`: The decompiler timed out, so only facts are known: a 45-instruction routine that touches the interrupt request register and its read copy and makes one unnamed call.
- `configure_audio_channel_zero`: The decompiler timed out, so only facts are known: a 53-instruction routine that touches audio channel zero's pointer, length, period and volume registers, the audio-disk control register, the DMA control register and the interrupt request register.
- `stub_unrecovered_small_routine`: The decompiler timed out, so only facts are known: a 17-instruction routine with one unnamed call and a tiny-routine caller.
- `stub_unrecovered_routine_entry`: The decompiler timed out, so only facts are known: a 30-instruction routine called by two unnamed routines, with no recorded calls or hardware access.
