# Game 685 (MonsterBusiness v1.2): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00685). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (869921 top-level bytes in 34 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 419 raw-word lines |
| Functions named and described | pass, 543 of 543 named (0 author symbols), 543 of 543 described (23 by rule, 500 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4118 of 4118 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 135900 bytes, 229 functions |
| Independent decoder cross-check (Ghidra against capstone) | 14874 of 16456 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 179741 |
| code | 33390 |
| data | 652378 |
| padding | 4412 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 98 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7055 | 283 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 743 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Boot (raw code file, load address unknown) | 1 | 0 | 0 |
| MB1 (raw code file, load address unknown) | 1 | 484 | 24 |
| MBF (raw code file, load address unknown) | 1 | 816 | 9 |
| MB2#unpacked (raw code file, load address unknown) | 1 | 7259 | 196 |

## Cases handled

- MonsterBusiness_v1.2/MonsterBusiness_v1.2.hdf: hard-disk image (AmigaDOS OFS, 928 blocks): its filesystem parses and all 23 files in it are byte-identical to the extracted tree MonsterBusiness_v1.2_hdf, which is recovered file by file; no block outside the filesystem holds data
- MonsterBusiness_v1.2/MonsterBusiness_v1.2_hdf/MonsterBusiness/MB2: LOB: LOB's File Compressor-packed, unpacked with ancient (54932 -> 123612 bytes)
- MonsterBusiness_v1.2/MonsterBusiness_v1.2_hdf/MonsterBusiness/MBA: LOB: LOB's File Compressor-packed, unpacked with ancient (26246 -> 60044 bytes)
- MonsterBusiness_v1.2/MonsterBusiness_v1.2_hdf/MonsterBusiness/MBB: LOB: LOB's File Compressor-packed, unpacked with ancient (31716 -> 78918 bytes)
- MonsterBusiness_v1.2/MonsterBusiness_v1.2_hdf/MonsterBusiness/MBC: LOB: LOB's File Compressor-packed, unpacked with ancient (30692 -> 62802 bytes)
- MonsterBusiness_v1.2/MonsterBusiness_v1.2_hdf/MonsterBusiness/MBD: LOB: LOB's File Compressor-packed, unpacked with ancient (6708 -> 32000 bytes)
- MonsterBusiness_v1.2/MonsterBusiness_v1.2_hdf/MonsterBusiness/MBE: LOB: LOB's File Compressor-packed, unpacked with ancient (12712 -> 32000 bytes)
- runtime segment an address: 3407 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 637 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- MonsterBusiness_v1.2/MonsterBusiness_v1.2_hdf/MonsterBusiness/Boot: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- MonsterBusiness_v1.2/MonsterBusiness_v1.2_hdf/MonsterBusiness/MB1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- MonsterBusiness_v1.2/MonsterBusiness_v1.2_hdf/MonsterBusiness/MBF: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- MonsterBusiness_v1.2/MonsterBusiness_v1.2_hdf/MonsterBusiness/MB2#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- MonsterBusiness_v1.2/MonsterBusiness_v1.2_hdf/MonsterBusiness/Scores: raw file found loaded verbatim in runtime segment an address at an address; classified through the recovered segment
- 1 never-loaded region(s), 1024 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_hardware_loop` (runtime segment an address): Initializes hardware registers and enters a complex loop that clears a memory range, validates bit patterns, and updates global counters and pointers based on hardware status. Kind: disk track/sector verification loop. Pass path: the loop continues to increment _u1__d0_845A (likely a buffer pointer or sector offset) and _u1__d0_8458 (remaining sectors/tracks) as long as u1__sub_7E74 returns a non-negative value.
- `execute_conditional_loop` (runtime segment an address): Writes constant values to memory and enters a loop that performs conditional calls and decrements a counter based on a bitwise check of a global variable. Kind: disk track/sector retry loop with hardware status check. Pass path: the loop executes and eventually returns 0 if iVar1 (the read result) is -1 (indicating success/end of loop) or if the loop completes normally.
- `process_hardware_and_memory` (MB1 (raw code file, load address unknown)): Modifies interrupt and disk registers, performs calculations, and writes data to memory via pointers. Kind: disk hardware/signature check. Pass path: the branch following the bsr.w an address call (likely the instruction at the offset immediately following the subroutine return).
- `manipulate_hardware_state` (MB1 (raw code file, load address unknown)): Updates CIAA and CIAB registers and performs a series of data manipulations and memory writes. Kind: disk hardware/sync check. Pass path: the branch following the bsr.w an address call (likely the instruction at the offset immediately following the subroutine's return).
- `manipulate_hardware_registers` (MB1 (raw code file, load address unknown)): Modifies the interrupt control, disk length, and interrupt request registers. Kind: disk hardware/track check. Pass path: bmi.w an address (branch if negative) / bmi.b an address (branch if negative).
- `interact_with_hardware` (MB2#unpacked (raw code file, load address unknown)): Performs forty-two instructions while accessing the interrupt request, disk length, and serial port interrupt clear registers. Kind: disk hardware/track check. Pass path: bpl.b an address.
- `interact_with_cia_hardware` (MB2#unpacked (raw code file, load address unknown)): This forty-three instruction routine accesses CIAA, CIAB, and custom hardware registers while calling multiple small subroutines. Kind: disk hardware/track check. Pass path: the branch following the sub_B214 call (likely where d1 is checked or d0 is validated).
- `dispatch_hardware_logic` (MB2#unpacked (raw code file, load address unknown)): This forty-nine instruction routine accesses CIAA, CIAB, and custom hardware registers and executes several distinct subroutines. Kind: disk hardware/track check. Pass path: bmi.b an address.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `copy_block_then_call`: Copies 256 long words from the source into a fixed low-memory area, then calls a function through a pointer that was passed in.
- `adjust_buffers_backwards_and_terminate`: Walks backwards through two buffers, adjusting each word by a fixed offset and masking each byte, then writes a fixed terminating long word and calls a subroutine.
- `adjust_buffers_backwards_and_call`: Walks backwards through two buffers, adjusting each word by a fixed offset and masking each byte, then calls subroutines and stores return values.
- `return_immediately`: An empty stub that returns straight away without changing any memory or registers.
- `call_through_stored_pointer`: Calls a function through a pointer kept in a fixed slot and returns the caller's registers unchanged.
- `call_indirect_function`: Calls a function through a pointer stored in a memory location and returns the combined contents of two registers.
- `call_stored_hook_return_zero`: Calls a function through a stored pointer and returns zero; the code references the Scores text but does not use it directly.
- `call_service_with_request_code`: Calls a stored function with a fixed request code and zero arguments, then continues only when the result is nonzero; the Scores text is referenced nearby.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_call`: Stores a constant value through a pointer, calls another routine, and then stores a value derived from a global variable through a pointer before calling that same routine again.
- `modify_memory_bits`: Performs arithmetic on memory locations via pointers and applies a bitwise XOR operation to a byte using a bit-shifted value.
- `update_pointer_offset`: Stores a constant value through a pointer, calls another routine, and then stores a value derived from a global variable plus a different offset through a pointer.
- `store_constant_value`: Stores a constant value through a pointer and calls another routine.
- `decrement_global_loop`: Decrements a global variable in a loop while calling another routine three times per iteration.
- `increment_global_loop`: Increments a global variable in a loop while calling another routine three times per iteration until a threshold is reached.
- `modify_memory_values`: Adds a value to a memory location, performs a bitwise AND on another, and executes two function calls.
- `loop_with_decrement`: Repeatedly calls two functions while decrementing a global variable until it is no longer positive.
- `execute_repeated_subroutines`: Repeatedly calls a specific subroutine three times while incrementing a global variable by a fixed amount until a threshold is reached.
- `modify_and_store_data`: Performs arithmetic and bitwise masking on memory locations, updates a global variable, and then stores a value derived from a global variable through a pointer.
- `increment_and_mask_memory`: Increments a word at a memory location, applies a bitwise AND mask to a byte, updates a global variable, and calls another routine.
- `update_memory_and_call`: Increments a word in memory, applies a bitwise mask to a byte, updates a global variable, and calls another routine.
- `modify_memory_and_call_subroutine`: Increments a word at a memory location, applies a bitwise AND mask to a byte at a memory location, updates a global variable, and calls another routine twice.
- `copy_memory_block_2`: Copies data from a source table to a destination buffer using nested loops and specific offsets.
- `store_fixed_state_constants`: Stores several fixed constants and zeroes into state words.
- `replicate_memory_patterns`: Copies data from a source to a destination while duplicating values at specific large offsets within a loop.
- `process_blt_data`: Iterates through a table of values to calculate offsets and parameters, interacting with hardware registers for bitblock transfer operations.
- `execute_single_instruction_4`: A routine consisting of 117 instructions that calls another routine via an absolute move.
- `store_word_at_pointer_2`: Stores a word at the location pointed to by a register.
- `copy_memory_on_condition`: Copies a block of memory from a global variable to another location if a returned value is non-negative.
- `reorganize_and_populate_data_tables`: Iterates through a memory list to calculate offsets, clears a large memory block, and populates a table with specific byte values.
- `find_char_in_string`: Iterates through a null-terminated string to find a specific character and returns a value based on its position.
- `process_memory_list`: Iterates through a list of memory blocks, using a length byte to copy each block sequentially until a terminator is found.
- `process_data_lists`: Clears memory blocks, copies data between tables, and traverses multiple types of data and indirect lists.
- `traverse_data_list`: Iterates through a list of data using a pointer, calculating a value based on the second byte of each entry until a terminator is reached.
- `traverse_indirect_list`: Follows a pointer stored in a global variable to begin iterating through a list of data entries until a terminator is found.
- `populate_data_tables`: Clears memory, copies structured data from a source table into a destination table using nested loops, and populates specific memory locations with constants and routine pointers.
- `process_and_traverse_data_list`: Checks a global variable and updates a character value based on its state.
- `convert_string_digits_2`: Converts numeric characters from a source string into a destination string with specific character replacements.
- `convert_string_digits`: Iterates through a source string to convert numeric characters into a new string, applying specific character replacements based on a counter and a global variable.
- `initialize_and_copy_data_buffer`: Copies a structured data block and populates a series of memory locations with values from a table.
- `convert_to_ascii`: Adds the character '0' to values retrieved from memory to convert them into ASCII digits.
- `convert_values_to_ascii`: Retrieves bytes from memory and adds a character offset to store them as ASCII digits in a global variable.
- `parse_ascii_digits`: Loops through memory to convert bytes to ASCII digits and performs conditional logic based on the resulting values.
- `clear_memory_block`: Fills a specific range of memory with zeros using a loop.
- `copy_structured_data`: Uses nested loops to copy blocks of data from a source table into a destination table at specific offsets.
- `copy_memory_sequentially`: Iterates through a list, calculating offsets and calling a routine to copy memory blocks until a terminator is reached.
- `copy_memory_blocks`: Copies multiple words of data from a source address to a destination address using a loop that processes data in chunks.
- `update_callback_state`: Modifies a byte in memory, performs a memory copy, and updates two global pointers using a value derived from a bitwise XOR operation.
- `execute_callback_and_update_memory`: Decrements a global variable and executes a subroutine via a pointer before storing several calculated values into memory locations.
- `initialize_global_flag`: Checks if a routine address and a global variable are zero, then sets the global variable to a specific constant value.
- `patch_code_sequence`: Checks a global pointer, performs conditional calls, and writes two words into a memory location to modify code.
- `check_and_reset_pointer`: Checks if a global pointer is non-null and if a specific byte equals a constant before resetting the pointer and updating a global variable.
- `manage_global_state`: Modifies a global integer based on bitwise flags and executes a subroutine through a pointer while performing boundary checks.
- `check_bit_and_initialize_struct`: Checks if a specific bit is set in a register; if so, it assigns a value to a global variable, a pointer to a global variable, and a function pointer to a table or structure.
- `select_value_from_global_flag`: Returns two if a global variable is non-zero and four if it is zero.
- `set_status_values`: Sets two global bytes to a constant value and assigns a global pointer to a specific address.
- `modify_pointer_address`: Performs conditional checks on a global pointer and its associated memory to update the pointer with a new address.
- `update_bitmask_and_state`: Sets a bit in a byte at an offset from a pointer and updates several global variables and function pointers based on conditional checks of return values.
- `update_state_via_callbacks`: Modifies a byte in memory using a bitwise shift and executes a series of conditional function calls to update various global pointers and values.
- `manage_pointer_chain`: Manipulates a pointer and a counter to traverse or update a linked structure in memory.
- `process_dynamic_callback_chain`: Iterates through a linked list of function pointers while performing bitwise validation and conditional arithmetic on global state variables.
- `update_grid_position`: Calculates a relative offset between two coordinate sets and updates a global position variable if the difference falls within a specific range.
- `check_cell_rows_differ`: Returns minus one when two pairs of position values fall into different sixteen-unit blocks, and zero otherwise.
- `initialize_callback_data`: Stores a constant, a small integer, and a pointer to a global variable into three consecutive memory locations.
- `bitwise_or_table_entries`: Fetches two single bytes from a table using offsets in registers and returns their bitwise OR result.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `check_function_and_state`: Returns a byte value after checking if a function pointer is non-zero and a global variable does not equal a specific value.
- `execute_conditional_callback`: Checks if a global pointer is non-null and if a specific byte in a global variable does not equal a constant value before calling a subroutine.
- `execute_conditional_callback_2`: Checks if a global pointer is non-null and if a specific byte value at a memory location does not equal a constant, then executes a subroutine if these conditions are met.
- `validate_and_execute_callback`: Checks if a global pointer is non-null and if a specific global byte matches a required value before executing the routine pointed to by that global.
- `validate_and_execute_callback_2`: Checks if a global pointer is non-null and if a specific byte in a global variable matches a constant value before calling a subroutine.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `execute_conditional_callback_3`: Checks if a global pointer is non-null and if a specific global byte does not equal a constant value before calling a subroutine.
- `tiny_ori_2`: Small routine of 1 instruction that merges 255 into d0.
- `check_global_state_and_call`: Checks if a global variable is non-zero and if another global variable does not equal a specific constant, then calls a routine if these conditions are met.
- `validate_function_pointer`: Checks if a global variable is non-zero and if another global variable does not equal a specific constant value before calling a function through a pointer.
- `tiny_ori_ori_7`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `execute_conditional_calls`: Checks two different conditions involving a global pointer and a byte value before calling other routines.
- `execute_indirect_call`: Calls a routine through a pointer stored at a specific memory location.
- `execute_indirect_function`: Calls a function through a pointer stored in a global variable.
- `advance_entry_clear_when_low`: When a global flag is set, reads the current table entry and reloads the pointer when its marker is 1000.
- `call_indirect_table`: If a global variable is zero, it assigns a value from another global and executes a routine via a pointer from a table.
- `update_conditional_state_and_callback`: Checks specific bit patterns in registers and a global variable to update several global values and perform pointer arithmetic.
- `initialize_data_structure_2`: Sets several global values, clears a memory range with a constant, and populates a table with incrementing or decrementing values.
- `process_timer_table`: Updates a table of values by adding or subtracting a constant and checks for a threshold to set a status bit.
- `clear_status_and_bit`: Clears two global cells and resets the bit with value two of a status byte at a fixed offset.
- `scan_table_until_negative`: Scans forward through a table of 16-bit entries, stopping at the first negative entry or after a counted number of steps.
- `init_sequence_state_once`: When a state word is zero, sets it to nine and stores a fixed pointer and a value read from another pointer.
- `step_active_sequence_entry`: While a state word is nonzero, advances a 16-bit entry pointer by one word and clears the state on a marker of 1000.
- `clear_d0_register`: Performs a logical OR operation on a data register with zero.
- `process_callback_list`: Iterates through a list of pointers to clear memory and then executes a series of routines stored in a table.
- `compute_scaled_offset_from_reference`: Sets a global to a quarter of the distance by which a position lies outside a band from sixty to eighty.
- `process_entity_list`: Loops through a list of data structures to execute specific code pointers and conditional subroutines based on value comparisons.
- `iterate_entity_table`: Iterates through a table of structures to execute function pointers and perform conditional logic based on distance values.
- `process_collision_events`: Iterates through a table of collision bounds to check for intersections between a primary object and multiple secondary objects.
- `initialize_data_structure`: Calls another routine and populates a memory structure with specific offsets using values derived from a pointer, constants, and global variables.
- `execute_sequential_calls`: Executes three consecutive calls to another routine.
- `execute_multiple_calls_4`: Executes four consecutive calls to another routine.
- `execute_recursive_calls_3`: Performs three consecutive calls to itself before returning.
- `compare_and_assign_pointer`: Compares the sum of two structure members against a calculated value and, depending on the result, stores a routine address in a structure and executes several calls.
- `execute_two_calls`: Executes two consecutive calls to another routine.
- `update_entity_counters`: Performs arithmetic on structure members and toggles a byte flag if certain numerical conditions are met.
- `update_structure_pointer`: Copies values between structure members and updates a pointer within a structure based on a conditional check against a constant value.
- `clamp_structure_value`: Updates structure members and performs bounds checking to ensure a specific short value remains within a defined range.
- `update_structure_state`: Modifies structure offsets and pointers based on comparisons between table values and register-held offsets, potentially updating a routine pointer and a constant value.
- `align_structure_offsets`: Decrements an offset and conditionally updates structure members, including pointers and longwords, based on bitwise comparisons.
- `process_offset_alignment`: Increments a structure offset and conditionally updates routine pointers and data if specific bitwise conditions are met during table lookups.
- `decrement_structure_offsets`: Decrements two structure offsets by two and adjusts a pointer to align with a nibble boundary if a bitwise condition is met.
- `increment_structure_offsets`: Increments a structure offset by two and decrements another by two, performing a nibble-aligned pointer adjustment if a condition is met.
- `update_entity_state_and_score`: Decrements a counter within a structure and updates linked list pointers or entity attributes based on specific threshold values.
- `copy_memory_block_3`: Copies twenty-one words from one memory location to another.
- `search_list_for_zero`: Iterates through a list using a fixed stride until a zero word is encountered.
- `update_linked_list_state`: Advances a pointer through a linked structure, updates a counter, and clears a value if a threshold is met.
- `update_accumulated_value`: Adds a value from a table to a longword in a structure and checks if the high word falls within a specific range, otherwise clearing a pointer.
- `clear_word`: Sets a word at a specific memory location to zero.
- `update_accumulated_values`: Calculates a difference between a routine address and a table value, then updates several accumulated longword values in a structure.
- `populate_record_slot`: Iterates through a loop to find a free slot and populates it with a series of constants, values, and a routine pointer.
- `populate_record_slot_2`: Iterates through a memory range to find a slot and populates it with various constants, values, and a routine pointer.
- `increment_counters_and_check`: Increments two values in a structure by adding values from another structure and checks a specific word against a constant, clearing a pointer if the condition is met.
- `nullify_pointer_target`: Stores a zero word into the memory location pointed to by a value held in a register.
- `update_entity_state_and_pointers`: Evaluates bitmask flags from a fetched value to conditionally update a structure's pointer field.
- `update_entity_state_logic`: Updates an entity structure by modifying its internal counters, toggling a state flag, and performing arithmetic on specific fields based on bitwise checks of a return value.
- `update_entity_position_and_state`: Calculates new coordinate offsets and updates structure members based on conditional comparisons and jump table lookups.
- `adjust_structure_offsets`: Updates values within a structure by copying data between offsets and performing conditional arithmetic based on comparisons with other memory locations.
- `adjust_offset_value`: Modifies a short integer at an offset within a structure based on the value of a byte at a different offset.
- `decrement_and_traverse_list`: Decrements a counter in a structure and, if it reaches a negative value, updates the structure with data from a linked list.
- `traverse_and_copy_list_entry`: Follows a pointer within a structure, potentially traversing a linked list, and copies data from the target location into the structure.
- `assign_function_pointer`: Copies a longword from a pointer-based location into a structure and assigns a specific routine address to a function pointer field.
- `set_record_state_on_direction`: Increments a counter field, loads a value from a pointer field, copies a timer value, and stores a state address selected by a direction flag.
- `advance_pointer_with_loop_marker`: Advances a pointer field to the next entry and follows its link when the entry marker is 1000.
- `add_values_and_increment_pointer`: Adds two values from one structure to two others and increments a pointer in a structure if a specific word does not equal a constant.
- `clear_memory_word`: stores a zero word into the memory location pointed to by a register.
- `initialize_record_structure`: Updates a value in a structure by adding a field to another, then conditionally resets the structure or populates it with a new pointer and specific constant values.
- `nullify_pointer_target_2`: Writes a zero value to the memory location pointed to by a register.
- `initialize_record_with_data`: Sets fields in a structure and, if a free slot is found, populates it with specific values and a routine pointer.
- `update_conditional_structure_member`: Checks if a value in a structure matches a specific constant, then copies a longword, updates a function pointer, and sets a byte flag if it is zero.
- `process_conditional_state_updates`: Checks a value within a structure and, based on a byte flag, performs a sequence of conditional updates to multiple nested structure pointers.
- `initialize_nested_structures`: Checks a value within a structure and, based on a byte flag, performs a sequence of conditional updates to multiple nested structure pointers and fields.
- `update_pointer_on_condition`: If a value at a specific offset equals one thousand, it copies a longword from one offset to another and updates a pointer.
- `update_structure_pointers`: Assigns a routine pointer to a structure and conditionally updates a longword and a pointer based on a value equaling one thousand.
- `update_object_structure`: Updates a structure by assigning a pointer, clearing a field, and conditionally copying a longword based on a value check.
- `process_nested_structures`: Sequentially checks the return values of several routines to conditionally initialize multiple structures with specific constant values and routine pointers.
- `populate_object_structure`: Initializes a structure with constant values and a pointer from memory, then populates several fields using values from a table and an offset provided in a parameter.
- `load_offset_data`: Loads two short values from a table using an offset and stores them into a structure along with a constant.
- `find_free_record_slot`: Scans records spaced 45 words apart, up to a counted limit, and returns the pointer to the first record whose first word is zero.
- `update_dynamic_asset_table`: Calculates memory offsets and updates a global state variable based on a recursive call.
- `clear_global_and_call`: Checks if a global variable is non-zero, clears it, and then calls two other routines.
- `merge_data_tables`: Iterates through a source table to perform bitwise operations between source words and mask words, storing the results into a destination table.
- `merge_masked_tables`: Performs bitwise operations between source data and mask data from two tables and writes the results into two destination memory areas.
- `copy_masked_data`: Iterates through two source tables, performing bitwise AND operations between source words and a mask table, then writes the results to two separate destination tables.
- `search_byte_sequence`: Searches through a memory buffer for a specific three-byte sequence based on nibble comparisons and stores a byte from the buffer into a global variable.
- `calculate_address_from_table`: Uses a value from a table to calculate two longword addresses and loads several bytes from a table into global variables.
- `distribute_bytes_from_table`: Iterates through a table of bytes and uses the values to calculate offsets for storing bytes into two different memory regions.
- `stub_empty_return`: This routine is empty and performs no operations before returning.
- `fetch_jump_table_entry`: Returns a single byte from a table using an offset stored in a register.
- `update_pseudo_random_state`: Updates a global variable by adding another value to it and then applying a bitwise transformation involving a left shift and an XOR operation.
- `initialize_random_seed`: Sets a global variable by performing a bitwise OR with a constant on a value returned from another routine and XORs two other values into a global variable.
- `rotate_and_copy_buffer`: Rotates a set of four 32-bit values stored in global memory and performs a nested loop to copy data from a pointer to a memory location.
- `nested_loop_wait`: Executes nested loops that wait for a condition to be met while decrementing a counter and copying data from a pointer.
- `copy_data_with_loops`: Decrements a global counter and performs nested loops to copy data from a source pointer to a destination memory location.
- `decrement_and_copy_nested_loop`: Decrements a global counter while performing nested loops to copy values from a pointer to a memory location.
- `initialize_structure_and_wait`: Populates a structure with calculated values and constants, then enters a loop that waits until a specific bit in a byte is cleared before storing remaining values.
- `transpose_data_block`: Transposes a block of 32-bit words into a series of 16-bit values by interleaving high and low words into a new memory structure.
- `update_sprite_blt_attributes`: Validates sprite index and quantity bounds before loading animation or attribute data from a table into memory and hardware registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_blt_attribute_registers`: Checks input bounds and uses a table to load values into memory and hardware registers.
- `populate_sprite_attribute_list`: Iterates through a loop to populate a structured data block with color, coordinate, and offset values derived from a table.
- `populate_sprite_event_sequence`: Populates a structure with a sequence of coordinate, color, and timing data by iterating through a loop.
- `push_event_to_queue`: Calculates an offset and parameters based on input registers and stores them into a growing queue structure in memory.
- `push_event_to_queue_2`: Calculates an offset and index based on input registers to store data into a structured queue.
- `enqueue_sprite_event`: Calculates sprite animation and position parameters from input registers and stores them into a growing event queue structure.
- `enqueue_sprite_event_data`: Calculates timing, coordinate, and color parameters for a sprite animation sequence and pushes them into a circular buffer or linked list.
- `update_dynamic_event_queue`: Manages a linked list or queue of event structures by calculating offsets and updating pointers.
- `push_event_to_queue_3`: Calculates an offset and event parameters based on input registers and pushes them into a circular buffer or linked list structure.
- `push_event_to_queue_4`: Calculates an offset and event data based on input parameters and pushes them into a linked list or circular buffer.
- `manage_dynamic_object_list`: Updates a linked list or dynamic array by calculating offsets and inserting new data entries.
- `enqueue_sprite_attribute_update`: Calculates sprite attribute offsets and writes update data into a circular buffer.
- `nop_placeholder`: This routine is empty and performs no operations.
- `enqueue_sprite_update_event`: Checks bounds and validates input parameters before appending a new sprite attribute update entry to a circular buffer or linked list.
- `update_sprite_attribute_table`: Updates a table of sprite attributes by calculating new coordinate and color values based on provided offsets and indices.
- `populate_entity_attribute_list`: Populates a structure with a sequence of coordinate, color, and offset values by iterating through a loop.
- `populate_sprite_attribute_list_2`: Populates a structure with a sequence of sprite attributes including color, X/Y coordinates, and frame data.
- `populate_sprite_attribute_list_3`: Populates a structure with a sequence of coordinate, color, and attribute data by iterating through a loop.
- `populate_sprite_attribute_list_4`: Populates a structure with a series of sprite-related attributes including color, coordinates, and priority flags.
- `populate_object_attribute_sequence`: Populates a structure with a series of attribute entries including coordinates, colors, and offsets.
- `populate_sprite_attribute_list_5`: Populates a structure with a series of sprite or object attributes, including color, coordinate, and pointer data.
- `populate_sprite_attribute_table`: Populates a structure with color, coordinate, and attribute data using a loop.
- `set_d4_and_clear_memory`: Loads a specific byte value into a data register and stores a longword of zero at the address pointed to by an address register, incrementing that register.
- `clear_and_increment_attribute_pointer`: Sets the lower two bits of a data register to zero and writes a zeroed longword to the memory location pointed to by a register before incrementing that pointer.
- `set_d3_and_store_constant`: Loads a specific byte value into a data register and stores a specific longword constant at the address pointed to by an address register, incrementing that register.
- `set_attribute_flags_and_increment`: Sets the lower two bits of a byte in a register and writes a longword value to a memory location pointed to by an address register before incrementing that pointer.
- `fill_copper_list_entries`: Writes a fixed-stride sequence of entries into a table addressed through registers, pointing each entry at a source pointer and a running offset, decrementing a counter until it reaches minus one.
- `fill_blitter_sprite_rows`: Writes a counted series of per-row values into a block of 32-bit longword slots, stepping the source address by a fixed row pitch, and busy-waits on a status bit in the blitter register block before each row is written.
- `store_structure_fields`: performs a loop to store several integer and word values into specific offsets of a structure pointed to by a register.
- `initialize_complex_structure`: calculates several values from a table and then performs a loop to populate a structure with coordinates, colors, and other data.
- `update_pointer_structure`: Updates several global variables by performing arithmetic on a value and dereferencing a pointer modified by a bitwise operation.
- `update_dynamic_memory_structure`: Modifies a complex data structure by calculating new offsets and updating pointers based on the sign of an input value.
- `perform_recursive_subtraction_loop`: Executes a loop that conditionally modifies a global variable based on its sign and performs repeated subtractions using values derived from bitwise operations on that variable.
- `update_lookup_tables`: Checks if a value has changed and then updates several memory locations by performing indexed lookups into tables.
- `copy_block_of_words`: Copies a sequence of 16-bit words from a source pointer to three different destination memory locations.
- `unpack_and_distribute_table_data`: Performs bitwise reconstruction of packed data from a table and writes the resulting values into multiple sets of global variables.
- `expand_bit_packed_table_data`: Reconstructs multi-component values from a bit-packed source table using bitwise OR and AND operations.
- `load_table_offsets`: Compares a global value against another and, if different, uses a value to calculate offsets into tables to update several memory locations via a loop.
- `copy_memory_block_4`: Copies a sequence of 16-bit words from a source buffer to three different destination memory locations.
- `unpack_and_distribute_table_data_2`: Extracts bit-packed values from a source table using bitwise operations and stores the resulting components into multiple destination memory locations.
- `unpack_bitpacked_table_data`: Extracts multiple bit-packed values from a source table using bitwise masking and OR operations.
- `execute_indirect_calls`: checks if a global variable contains a specific address and then performs a series of indirect calls to that address.
- `execute_conditional_function_chain`: Checks if a global variable matches the current routine's address before updating that variable with the stored value and executing a sequence of subsequent routine calls.
- `execute_recursive_calls_2`: Updates a global variable with a new address and performs a sequence of nested and recursive calls to that address.
- `load_data_table`: calculates an offset based on a global value and copies multiple longwords from a table into several global variables.
- `load_table_data`: Calculates an offset from a global value and copies multiple longwords from a table into several global variables.
- `fetch_table_entry`: loads a longword from a table into a global variable using an index multiplied by four.
- `update_global_from_table`: Calculates an offset into a table using a global value multiplied by four and stores the resulting longword into another global variable.
- `execute_jump_table`: Compares two global values and, if the condition is met, enters a loop to fetch and execute a sequence of routines from a table.
- `tiny_ori_3`: Small routine of 1 instruction that merges 7 into d0.
- `tiny_ori_4`: Small routine of 1 instruction that merges 40 into d0.
- `copy_memory_block`: Copies a block of data from one memory location to another using a decrementing counter.
- `copy_word_block`: Copies a sequence of words from a source table to a destination table using a loop.
- `reset_global_state`: sets a global variable to zero and calls another routine.
- `copy_words_to_code`: Iterates through a sequence of instructions and writes two words to memory at the location of a null instruction.
- `clear_code_sequence`: Iterates through a sequence of instructions and sets non-zero values to zero, calling itself recursively.
- `copy_and_invert_data_block`: Copies a block of word-sized data from one memory location to another while performing a bitwise OR on the words and writing the bitwise NOT of that result into the destination buffer.
- `perform_data_transformation`: reads words from a source, writes them to a destination, and stores a bitwise inverted mask of the words into a table.
- `execute_nested_calls`: Performs a sequence of three consecutive calls to another routine.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `process_nested_data_loop`: Loads data from a pointer into several global variables and enters a loop that performs conditional calls and memory clearing.
- `process_hardware_loop`: Initializes hardware registers and enters a complex loop that clears a memory range, validates bit patterns, and updates global counters and pointers based on hardware status.
- `write_constant_bytes_2`: Writes a specific sequence of three bytes to a memory location stored in a register.
- `process_loop_sequence`: Clears a memory range and enters a loop that calls a routine repeatedly, updating several global counters and pointers based on the results.
- `return_immediately_2`: This routine is empty and performs no operations before returning to its caller.
- `validate_bit_pattern`: Performs a series of conditional checks and bitwise operations to validate specific data patterns and return status codes.
- `return_constant_2`: returns a constant value of all bits set.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 254 into d0, then returns.
- `find_bit_pattern`: Searches a memory range for a specific bit pattern and returns the number of shifts required to align it.
- `calculate_modulo_sixteen`: Subtracts a value from sixteen and returns the result, resetting to zero if the result equals sixteen.
- `shift_bits_left`: Performs a multi-pass left bitwise shift on a sequence of words, propagating the most significant bit as a carry through the array.
- `combine_nibbles`: Reads pairs of bytes from a source and combines them by masking and shifting values from a table into a destination byte.
- `calculate_crc_table`: Computes a series of 16-bit values using a bitwise shift and XOR operation, storing the results in a memory table.
- `write_constant_values`: Writes three specific constant values to offsets relative to a register.
- `update_masked_byte`: Writes a bit-masked value to a memory location based on whether a global variable is non-zero.
- `write_constant_bytes`: Writes a specific sequence of three bytes to a memory location pointed to by a register.
- `wait_for_bit`: Polls a specific bit in a memory location for a set number of iterations, returning zero if the bit is cleared or a negative value if the loop expires.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `check_bit_status`: Iterates through a memory range checking a specific bit in each byte and updates global variables if the bit is not set.
- `reset_global_values`: Sets a global byte to zero and another global variable to a specific constant value.
- `sync_global_counters`: Compares two global values and executes a loop a specific number of times based on the difference between them.
- `execute_loop_sequence`: Executes a sequence of routines within a loop until a specific condition is met.
- `modify_byte_bits`: Performs bitwise operations to set and clear specific bits in a byte pointed to by a register.
- `modify_byte_bits_alt`: Performs bitwise operations to clear and set specific bits in a byte pointed to by a register.
- `decrement_register`: Decrements a value in a register until it reaches negative one.
- `initialize_hardware_registers`: Writes several values to memory offsets and waits in a loop for a global variable to change.
- `initialize_hardware_state`: Writes several multi-byte values to specific offsets in memory and performs a loop that waits for a global variable to change.
- `process_bit_stream`: Processes a sequence of bits from a byte and stores the resulting value into a memory location.
- `modify_word_byte`: Clears the most significant bit of a byte within a word and then writes that word back to memory.
- `extract_bit_stream_sequence`: Invokes a routine to process a sequence of bits from a byte and stores the resulting value into a memory location.
- `store_word_at_pointer`: Stores a 16-bit value from a register into the memory location pointed to by another register.
- `execute_conditional_loop`: Writes constant values to memory and enters a loop that performs conditional calls and decrements a counter based on a bitwise check of a global variable.
- `loop_with_conditional_calls`: Checks a bit in a register and enters a loop that performs conditional calls and updates a global variable until a counter reaches zero.
- `decrement_global_counter_loop`: Initializes a global variable to a specific value and enters a loop that repeatedly calls another routine.
- `write_constant_bytes_3`: Writes a sequence of three bytes to a memory location pointed to by a register.
- `extract_bit_stream_data`: Processes a sequence of bits from a byte and stores the resulting values into memory using a loop.
- `decrement_and_call`: Decrements a value in a register and calls another routine in a loop until the value reaches zero.
- `execute_single_instruction_3`: A single instruction routine entered from outside the static flow.
- `call_register_storage_routine`: Calls a routine that stores two registers into global memory and returns a value from a third global location.
- `store_registers_to_globals`: Stores the contents of two registers into global memory locations and returns a value from a third global location.
- `decompress_data_loop`: Copies a block of memory while performing bitwise operations on data from a source table.
- `copy_and_decompress_data`: Copies a block of memory using a loop and performs bitwise operations to process data from a source table.
- `copy_table_data`: Iterates through a table to copy multiple longwords from a source to various global variables based on calculated offsets.
- `execute_sequential_subroutines_2`: Calls a specific routine multiple times in succession, followed by a single call to another routine, before returning.
- `clear_multiple_globals`: Sets several global variables and a specific memory location to zero.
- `reset_global_variable`: Loads a value from a table into a global variable and then clears that variable.
- `repeat_until_stable`: Repeatedly calls another routine in a loop until a global variable no longer changes its value.
- `decrement_table_value`: Loads a 16-bit value from a table using an index and subtracts one from it.
- `clear_memory_blocks`: Fills a large range of memory with zeros and then clears specific intervals within a second range.
- `load_indexed_table_values`: Calculates offsets using global variables to fetch two short words from a table and stores them into other global variables.
- `initialize_memory_pointers`: Stores several calculated memory addresses into global variables.
- `set_three_globals`: Assigns specific 16-bit values to three consecutive global memory locations.
- `load_configuration_parameters`: Calculates an offset into a table to load several short and long values into global variables and pointers.
- `initialize_global_parameters`: Sets multiple global variables to specific constant values and then calls another routine.
- `set_memory_addresses`: Stores two specific constant values into global variables.
- `tiny_ori_5`: Small routine of 1 instruction that merges 0 into d4.
- `clear_memory_block_2`: Clears a block of memory by writing zero to a sequence of bytes.
- `tiny_ori_6`: Small routine of 1 instruction that merges 224 into d4.
- `clear_memory_block_3`: Clears a block of memory by writing zero to each byte starting from a specific address.
- `clear_memory_block_4`: Fills a block of memory with zeros using a counter and a pointer.
- `clear_memory_block_5`: Iterates through a memory range to set each byte to zero.
- `parse_data_table`: Iterates through a table to extract and transform various bytes and pointers into a structured destination table.
- `execute_recursive_calls`: The routine performs two consecutive calls to itself before returning.
- `process_data_table`: Reads a table of pointers and values, uses a value to calculate an offset, and copies data to a destination while calling a function pointer.
- `transform_memory_block`: Performs nested loops to read data from a source and write bitwise-inverted values into a destination table.
- `populate_nested_data_buffer`: This routine performs a multi-level nested loop to populate a memory buffer by copying, bitwise inverting, and shifting values from a source table.
- `calculate_bit_sum`: Calculates a value by iteratively shifting and adding the individual bits of two input words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_return_stub_and_reset`: When a selector byte is minus one, zeroes a set of status globals, writes a return instruction into a jump slot and clears a flag bit, then calls a subroutine.
- `jump_to_decrement_two_counter_bytes`: Jump stub: passes control straight on to decrement_two_counter_bytes.
- `tiny_nop`: Small routine of 1 instruction that does nothing for one instruction.
- `store_global_byte_four_places`: Calls four subroutines and then writes the global byte value into four consecutive positions relative to the caller's pointer.
- `advance_channel_sequence_records`: When a channel's duration counter is zero, reads the next twelve-byte event record and stores its fields, jumping to a handler on end-of-sequence markers; otherwise it counts the duration down.
- `step_counter_with_wrap_flag`: Adjusts a byte counter by one step within a range, setting its high bit as a marker when it reaches the limit.
- `tiny_addq_subq_rts`: Small routine of 3 instructions that adds 1 to (a5), then subtracts 1 from (a6), then returns.
- `step_counter_with_stop_flag`: Decrements a byte counter, stopping at zero and setting its high bit as a marker when it is reached.
- `decrement_two_counter_bytes`: Decrements two byte counters through pointers.
- `set_flag_byte_to_ones`: Writes the all-ones value to a flag byte at a fixed offset from the input pointer.
- `clear_flag_byte`: Writes zero to a flag byte at a fixed offset from the input pointer.
- `copy_record_fields_from_table`: Copies one word from a source record into a field, and a long word looked up from a table by an offset from the same record.
- `copy_partial_data`: Copies a word from one location to another and a longword from a table to a destination.
- `copy_attribute_data`: Copies several words and longwords from a source table into a destination structure.
- `copy_memory_block_5`: Copies a sequence of words and longwords from one memory location to another.
- `copy_and_realign_data`: Copies four words from a source pointer into a destination structure and then performs four longword copies to shift data within that destination structure.
- `update_data_from_table`: Copies words from a source, uses a value to index into a table, and copies a sequence of longwords from that table to a destination.
- `update_table_index`: Copies several words from a source to a destination, then performs a division to fetch a word from a table and stores it in a specific offset.
- `load_step_data`: Copies a block of data from a table indexed by a value into a destination structure.
- `initialize_record_fields`: Copies fields from an input record into a state record, sets position words, and calls a subroutine to finish the initialization.
- `reset_record_fields_from_input`: Sets a status byte to all ones, copies a word from the input into two record fields, and shifts four long words within the record.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.

### MB1 (raw code file, load address unknown)

- `configure_display_hardware_2`: This sixteen instruction routine modifies color, bitplane, and CIA registers and performs memory writes via pointers.
- `configure_bitplane_display`: Sets up bitplane modes, copper pointers, display start positions, and DMA/interrupt enable bits.
- `set_bitplane_pointers`: Updates bitplane base addresses and modifies DMA and interrupt control registers.
- `execute_minimal_instructions`: Performs four instructions and returns.
- `process_hardware_and_memory`: Modifies interrupt and disk registers, performs calculations, and writes data to memory via pointers.
- `manipulate_hardware_state`: Updates CIAA and CIAB registers and performs a series of data manipulations and memory writes.
- `process_multiple_data_blocks`: Performs fifty-three instructions including calls to routines that manipulate data blocks, modify memory contents, and update state.
- `process_data_block`: Executes twenty-one instructions to manipulate data.
- `modify_memory_contents`: Performs nineteen instructions to process data.
- `update_routine_state`: Executes twenty-three instructions to modify data or state.
- `perform_short_calculation`: Executes sixteen instructions to process data.
- `perform_calculation_wrapper`: Performs five instructions and calls another routine to execute a short calculation.
- `handle_small_task`: Executes eight instructions to manipulate data.
- `execute_brief_sequence`: Performs six instructions to process data.
- `run_short_routine`: Executes nine instructions to manipulate data.
- `modify_memory_via_pointer_6`: Writes a register value to a memory location using a pointer held in another register.
- `write_memory_via_pointers`: Writes register values to memory locations using pointers.
- `modify_memory_via_pointer_5`: Performs six instructions including a sub-routine that writes a register value to a memory location pointed to by another register.
- `write_to_pointer`: Performs six instructions including a sub-routine that writes a value from a register into a memory location through a pointer.
- `modify_memory_via_pointer`: The routine performs two instructions to write a value from a register into a memory location pointed to by another register.
- `manipulate_hardware_registers`: Modifies the interrupt control, disk length, and interrupt request registers.
- `call_unnamed_routine_2`: Performs fourteen instructions and calls another routine consisting of forty-nine instructions.
- `call_unnamed_routine`: Performs fourteen instructions and calls a forty-nine instruction routine.
- `process_unnamed_routine_call`: This routine consists of forty-nine instructions and is invoked by a single unnamed caller.

### MBF (raw code file, load address unknown)

- `update_system_state`: This routine calls another routine that modifies data at memory locations via pointers and updates audio volume, CIAA, and DMA control registers.
- `update_memory_via_pointer`: The routine performs five instructions to modify data at a memory location.
- `store_byte_via_pointer`: The routine stores a single byte from a register into a memory location determined by a pointer.
- `access_via_lea_hardware`: An seventeen instruction routine accessed via a relative load effective address.
- `update_hardware_and_dma`: Modifies data via pointers, updates audio volume, CIAA, and DMA control registers, and calls another routine.
- `perform_indirect_branch`: The routine executes twenty-four instructions and is invoked via a PC-relative load effective address instruction.
- `execute_long_routine`: Performs one hundred and three instructions.
- `perform_indirect_jump`: The routine executes a sequence of fifty-two instructions that concludes by jumping to a location determined by a value stored in memory.
- `update_hardware_and_memory`: Modifies data at memory locations via pointers and updates audio volume, CIAA, and DMA control registers.

### MB2#unpacked (raw code file, load address unknown)

- `initialize_hardware_subsystems`: Modifies the first color register, Blitter registers, the DMA controller, and CIAA/CIAB ports.
- `configure_hardware_and_memory`: Modifies color, bitplane, and CIA registers while performing memory writes via pointers.
- `calculate_relative_offset_2`: Calculates a relative offset from the current program counter.
- `calculate_relative_offset`: This routine calculates a relative offset from the current program counter using a load effective address instruction.
- `configure_display_hardware`: Configures the display controller, bitplane modes, and DMA channels.
- `manage_hardware_registers`: Interacts with various custom hardware registers, DMA, and CIAA ports while calling several other routines.
- `undefined_routine`: The routine is empty and contains no instructions.
- `execute_short_routine`: A very short routine consisting of four instructions.
- `execute_medium_routine`: A routine consisting of nine instructions that is accessed via a relative address.
- `execute_longer_routine`: A routine consisting of fourteen instructions.
- `manage_hardware_inputs`: A large routine that interacts with CIAA ports for joystick and mouse data, DMA controllers, and various custom hardware registers and interrupt settings.
- `manage_hardware_and_interrupts`: Interacts with custom hardware registers, DMA, and CIAA ports while calling a mid-length routine.
- `execute_mid_length_routine`: A routine consisting of twenty-nine instructions.
- `provide_relative_address`: A routine consisting of twenty-two instructions that is accessed via a relative address.
- `write_data_and_call`: Writes bytes from registers to memory locations and executes several other subroutines.
- `write_byte_and_call`: Writes a byte from a register to a memory location and calls four subroutines.
- `execute_minimal_routine`: A very short routine consisting of six instructions.
- `execute_standard_routine`: A routine consisting of sixteen instructions.
- `execute_short_routine_alt`: A routine consisting of twelve instructions.
- `execute_subroutines`: Calls a short routine, a routine that provides a relative address, and a seven-instruction routine.
- `perform_complex_tasks`: Executes a large number of subroutines, including hardware register modifications and pointer-based memory storage.
- `execute_standard_routine_alt`: A routine consisting of seventeen instructions.
- `execute_fourteen_instructions`: Performs fourteen instructions and is called by one routine.
- `perform_data_operations_2`: Provides a relative address, loads a value into a register, and executes another routine.
- `perform_data_operations`: Provides a relative address, loads a value into a register, and executes a nineteen-instruction routine.
- `execute_six_instructions`: Performs six instructions and is called by three routines.
- `tiny_moveq`: Small routine of 1 instruction that loads 15 into d2.
- `execute_nineteen_instructions`: Performs nineteen instructions and is called by six routines.
- `execute_seventy_four_instructions`: Performs seventy-four instructions and is called by one routine.
- `execute_nested_routine`: This routine executes six instructions before calling another routine.
- `execute_twenty_nine_instructions`: Performs twenty-nine instructions and is called by one routine.
- `execute_small_routine_3`: Executes a sequence of ninety-four instructions including a ten-instruction routine.
- `call_instruction_sequences_short`: Calls a seven instruction routine and a large routine that modifies a custom hardware register.
- `execute_seven_instructions_4`: Performs seven instructions and is called by two other routines.
- `execute_seven_instructions`: Performs seven instructions and is called by three routines.
- `execute_short_sequence_2`: Executes a small number of instructions and calls another routine.
- `execute_twenty_instructions`: Performs twenty instructions and is called by one routine.
- `execute_subroutine_sequence_2`: Executes a sequence of subroutines including one accessed via a relative address calculation.
- `execute_subroutine_sequence_3`: Calls several subroutines, including one that uses a relative address calculation.
- `execute_subroutine_sequence_4`: Performs a series of subroutine calls including one accessed via a relative address.
- `execute_sequential_subroutines`: This routine executes a sequence of three distinct subroutines.
- `execute_lea_routine`: Executes a six-instruction routine accessed via a relative address calculation.
- `execute_six_instructions_via_lea`: Performs six instructions and is accessed via a relative address calculation.
- `execute_twenty_three_instructions`: Performs twenty-three instructions and is called by four routines.
- `execute_subroutine_sequence`: This routine consists of forty instructions and calls another routine.
- `execute_small_sequence`: This routine consists of three instructions and calls another routine.
- `execute_three_instructions`: Performs three instructions and is called by two routines.
- `configure_blitter_and_dma_hardware`: Modifies Blitter registers and the DMA controller register while invoking a large sequence of subroutines that manipulate memory via pointers and execute various instruction sequences.
- `execute_eight_instructions`: This routine consists of eight instructions and is entered via a relative program counter address.
- `call_relative_routines`: Invokes a medium-length routine and an eight-instruction routine using relative addressing.
- `invoke_relative_subroutines`: This routine executes two separate subroutines of varying lengths using relative addressing.
- `execute_offset_routines`: Executes a medium-length routine and an eight-instruction routine via relative program counter addresses.
- `invoke_relative_subroutines_3`: Executes a short routine and invokes two other subroutines using relative addressing.
- `call_short_subroutines`: Performs a four instruction routine and calls two separate subroutines via relative addressing.
- `invoke_relative_subroutines_2`: Invokes two separate subroutines using relative addressing.
- `execute_multiple_subroutines_2`: This routine invokes a short four-instruction routine followed by a sequence that calls both a nineteen-instruction routine and an eight-instruction routine.
- `call_multiple_routines_2`: Executes a four-instruction routine and two other routines of varying lengths.
- `call_multiple_routines`: Calls a nineteen-instruction routine and an eight-instruction routine.
- `execute_nested_subroutines_3`: Invokes three different subroutines using relative program counter addressing.
- `execute_nested_subroutines_4`: Invokes three different subroutines using relative program counter addressing and a very short four-instruction routine.
- `execute_nested_subroutines`: This routine invokes three different subroutines of varying lengths using relative program counter addressing.
- `invoke_subroutines`: This routine executes a short four-instruction sequence followed by a nineteen-instruction sequence that is called twice.
- `execute_repeated_routines_2`: Invokes a four-instruction routine and calls a nineteen-instruction routine two times.
- `execute_repeated_routines`: Calls a nineteen-instruction routine twice.
- `execute_twelve_instructions`: This routine consists of twelve instructions and is called by one other routine.
- `modify_memory_via_pointer_2`: The routine performs a sequence of operations involving a pointer to write data to a memory location.
- `execute_nine_instructions`: This routine consists of nine instructions and is called by one other routine.
- `configure_blt_hardware`: This routine consists of one hundred thirty-four instructions and modifies several Blitter registers and the DMA controller register.
- `modify_memory_and_execute`: Writes a value to a memory location via a pointer and then calls two other routines.
- `modify_memory_via_pointer_3`: The routine performs a sequence of operations to write a value to a memory location determined by a pointer.
- `execute_eleven_instructions`: This routine consists of eleven instructions and is called by one other routine.
- `execute_relative_routine`: Performs one hundred and one instructions and calls another routine via a relative address.
- `execute_internal_logic`: This one hundred and thirty-seven instruction routine is entered via a relative address and calls two other unnamed routines.
- `dispatch_hardware_logic_2`: This routine consists of nine instructions and accesses CIAA, CIAB, and custom hardware registers.
- `execute_nested_subroutines_2`: This routine calls four different subroutines of varying lengths.
- `execute_nested_logic`: Performs multiple nested subroutine calls and writes a value to memory via a pointer.
- `update_memory_value_2`: Writes a byte from a register into a memory location pointed to by another register and performs several subroutine calls.
- `update_memory_value`: The routine writes a byte from a register into a memory location pointed to by another register.
- `execute_multiple_subroutines_3`: This routine executes twenty instructions and invokes two different subroutines.
- `call_multiple_subroutines_3`: Executes a sequence of instructions including a relative address call and multiple subroutines of varying lengths.
- `call_multiple_subroutines_2`: Performs twenty instructions and invokes two separate subroutines.
- `execute_multiple_subroutines_4`: This routine consists of six instructions and calls two different subroutines.
- `perform_relative_addressing`: Executes a series of routines including one that provides a relative address and another that performs thirteen instructions.
- `execute_lea_routine_3`: Accesses a six-instruction routine via a relative address calculation and calls two other subroutines.
- `execute_lea_routine_2`: This routine consists of eleven instructions and accesses a six-instruction routine via a relative address calculation.
- `execute_sixteen_instructions`: This routine consists of sixteen instructions and is called by one other routine.
- `perform_relative_calls_2`: Performs multiple relative address calculations and executes a complex sequence of various subroutines.
- `perform_relative_calls`: This routine consists of thirty-six instructions and performs a relative address calculation, a six-instruction routine call, and calls two different subroutines.
- `execute_complex_sequence_2`: This routine consists of forty-two instructions and calls a four-instruction routine, a six-instruction routine via a relative address calculation, and two different subroutines.
- `modify_hardware_register_5`: Provides a relative address and modifies a custom hardware register.
- `execute_complex_subroutine_sequence`: This routine invokes several short subroutines, including one that executes thirteen instructions and another that calls multiple subroutines.
- `execute_six_instructions_2`: This routine consists of six instructions and is called by two other routines.
- `execute_thirteen_instructions`: Performs thirteen instructions and is called by four different routines.
- `modify_hardware_register`: This routine consists of thirty-five instructions and modifies a custom hardware register while calling a six-instruction routine, a single instruction, and two different subroutines.
- `execute_single_instruction`: Executes a single instruction and is accessed via a relative address.
- `call_multiple_subroutines`: Executes twenty instructions and calls two different subroutines.
- `execute_thirteen_instructions_2`: Consists of thirteen instructions and is called by one routine.
- `execute_twenty_five_instructions`: Performs twenty-five instructions and is accessed via a relative address.
- `execute_seven_instructions_2`: Executes seven instructions and is called by two routines.
- `execute_large_sequences_2`: This routine consists of ninety-four instructions and calls several sequences of varying lengths.
- `execute_large_sequences`: This thirty-seven instruction routine calls several large sequences of instructions that include multiple subroutines.
- `execute_large_sequence_3`: Calls a routine that executes a large sequence of instructions including several subroutines.
- `execute_five_instructions`: Performs five instructions and is called by one routine.
- `call_nine_instruction_routine`: Executes six instructions and calls another routine.
- `execute_large_sequence_2`: Executes a large sequence of instructions that includes several subroutines of different lengths.
- `execute_large_sequence`: This routine consists of one hundred and seventy-seven instructions and calls a twenty-six-instruction routine and a ten-instruction routine.
- `execute_twenty_six_instructions`: Consists of twenty-six instructions and is called by one routine.
- `execute_small_routine_4`: This routine consists of twenty-one instructions and calls a ten-instruction routine.
- `store_via_pointer`: Stores a value from a register into a memory location using a pointer and calls another routine.
- `modify_memory_via_pointer_4`: The routine performs a sequence of operations to store a value from a register into a memory location determined by a pointer.
- `execute_twenty_four_instructions`: Consists of twenty-four instructions and is called by one routine.
- `execute_sixteen_instructions_2`: Performs sixteen instructions and is called by one routine.
- `execute_complex_sequence`: Performs one hundred and sixty instructions and calls three different routines.
- `execute_nine_instructions_2`: A routine consisting of nine instructions that is called by three different callers.
- `execute_seven_instructions_3`: A routine consisting of seven instructions that is called by one caller.
- `execute_nine_instructions_alt`: A routine consisting of nine instructions that is called by one caller.
- `execute_thirty_instructions`: A routine consisting of thirty instructions that is called by one caller.
- `execute_forty_instructions`: A routine consisting of forty instructions that is called by one caller.
- `execute_twenty_two_instructions`: A routine consisting of twenty-two instructions that is called by one caller.
- `invoke_hardware_subroutine`: This routine calls another routine that modifies a custom hardware register and executes forty-five instructions.
- `execute_hardware_routine_2`: This routine consists of forty-five instructions and modifies a custom hardware register while calling another routine.
- `execute_hardware_routine`: This routine consists of one hundred and forty-five instructions and modifies a custom hardware register while calling a six-instruction routine and a twenty-nine-instruction routine.
- `call_seven_instruction_routine`: Executes twenty-nine instructions and calls another routine.
- `modify_hardware_register_4`: This routine consists of four instructions and calls a routine that modifies a custom hardware register.
- `execute_multiple_calls_3`: This routine consists of five instructions and performs several subroutine calls, including one that modifies a custom hardware register.
- `modify_hardware_registers`: Performs multiple hardware register modifications and various subroutine calls.
- `execute_multiple_subroutines_6`: This routine contains two hundred and seven instructions and performs a series of various subroutine calls.
- `execute_complex_subroutines`: This routine consists of eighty-seven instructions and calls several subroutines of varying lengths, including one that modifies a custom hardware register.
- `execute_various_subroutines_2`: Calls a large routine consisting of one hundred and sixty-two instructions.
- `execute_various_subroutines`: This routine consists of one hundred and sixty-two instructions and calls a seven-instruction routine and a ten-instruction routine.
- `execute_multiple_calls_2`: Executes a sequence of instructions that includes two different subroutine calls and a ten-instruction routine.
- `perform_hardware_and_subroutine_tasks`: This thirty-one instruction routine calls a routine that modifies a custom hardware register and executes a ten-instruction sequence.
- `execute_multiple_calls_sequence`: Performs a series of subroutine calls including a forty-two instruction routine and two ten-instruction routines.
- `modify_hardware_register_3`: This three instruction routine calls a routine that modifies a custom hardware register.
- `modify_hardware_register_2`: Modifies a custom hardware register and executes a large sequence of instructions alongside multiple subroutine calls.
- `execute_multiple_calls`: This routine consists of forty-two instructions and calls two different routines.
- `execute_ten_instructions`: A routine consisting of ten instructions that is called by one caller.
- `calculate_relative_offset_3`: This routine calculates a relative offset from the current program counter.
- `execute_seven_instructions_alt`: A routine consisting of seven instructions that is called by four different callers.
- `execute_fifteen_instructions`: A routine consisting of fifteen instructions that is called by one caller.
- `execute_multiple_subroutines`: Performs seventy-eight instructions and calls two different routines.
- `execute_twenty_eight_instructions`: A routine consisting of twenty-eight instructions that is called by one caller.
- `execute_short_routine_2`: Performs a sequence of seven instructions and is called by three different routines.
- `execute_multiple_subroutines_5`: Calls four different subroutines of varying instruction lengths.
- `call_medium_routine`: Executes eleven instructions and calls another routine.
- `execute_medium_routine_2`: Performs a sequence of nineteen instructions and is called by one routine.
- `call_longer_routine_twice`: Executes twenty-one instructions and calls the same routine twice.
- `execute_longer_routine_2`: Performs a sequence of twenty-three instructions and is called by two routines.
- `execute_extended_routine`: Performs a sequence of seventy-one instructions and is called by one routine.
- `execute_small_routine`: Performs a sequence of ten instructions and is called by one routine.
- `execute_small_routine_2`: Executes a sequence of ten instructions.
- `execute_large_routine_2`: Modifies a custom hardware register and executes a sequence of one hundred and four instructions.
- `execute_large_routine`: Performs a sequence of one hundred and four instructions and is called by one routine.
- `update_blitter_mask_registers`: Modifies the Blitter mask and write registers before calling another routine.
- `empty_routine`: This routine is empty and contains no instructions.
- `update_blta_registers`: Modifies Blitter mask and write registers and calls several other routines including a large one.
- `execute_complex_logic_2`: Performs a sequence of two instructions and calls a large routine consisting of one hundred and thirty-one instructions.
- `access_via_lea`: An eight instruction routine accessed via a relative load effective address.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 254 into d2, then merges 0 into (a4)+.
- `execute_two_instructions`: Contains two instructions and is accessed via a relative address.
- `manipulate_blt_registers`: Modifies BltA and BltL registers and custom hardware while performing various bitwise operations and relative address calculations.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 254 into d3, then merges 0 into (a2)+.
- `update_pointer_via_lea`: Calculates a relative address using a load effective address instruction.
- `load_relative_address`: Calculates a target address using a PC-relative load effective address instruction.
- `tiny_ori_ori_3`: Small routine of 2 instructions that merges 0 into d2, then merges 0 into (a4)+.
- `manipulate_blt_registers_2`: This routine modifies Blitter mask and final address registers and a custom hardware register.
- `tiny_ori_ori_4`: Small routine of 2 instructions that merges 254 into d4, then merges 0 into (a0)+.
- `load_address_pointer`: Performs a two-instruction operation to load a relative address into a register.
- `load_address_pointer_2`: Uses a two-instruction sequence to calculate and store a relative address.
- `load_address_pointer_3`: A two-instruction routine that provides a relative address via a pointer.
- `tiny_ori_ori_5`: Small routine of 2 instructions that merges 0 into d3, then merges 0 into (a2)+.
- `tiny_ori_ori_6`: Small routine of 2 instructions that merges 2 into d2, then merges 0 into (a4)+.
- `execute_complex_logic`: A large routine consisting of one hundred and thirty-one instructions that is called by six different locations.
- `dispatch_subroutine_tasks`: Executes a sequence of three distinct subroutines in succession.
- `process_data_block_2`: A twenty-one instruction routine called by another routine.
- `perform_subroutine_task`: A nineteen instruction routine that is executed when called by another routine.
- `execute_short_sequence`: A twenty-three instruction routine invoked by a single caller.
- `handle_routine_logic`: A sixteen instruction routine called by another routine.
- `handle_routine_logic_2`: Executes a sixteen instruction routine.
- `execute_small_block`: An eight instruction routine that is called by two different routines.
- `perform_minimal_task`: A six instruction routine called by two separate routines.
- `execute_nine_instructions_3`: Performs nine instructions and is called by two unnamed routines.
- `execute_instruction_sequence`: This routine invokes another routine that performs a sequence of two instructions.
- `execute_instruction_sequences`: Invokes several different routines that each perform a small number of instructions.
- `execute_two_instructions_3`: Performs a sequence of two instructions.
- `execute_dual_instructions`: This routine calls another routine that performs two instructions.
- `execute_two_instructions_2`: Performs two instructions and is called by five unnamed routines.
- `interact_with_hardware_2`: Accesses interrupt request, disk length, and serial port interrupt clear registers.
- `interact_with_hardware`: Performs forty-two instructions while accessing the interrupt request, disk length, and serial port interrupt clear registers.
- `execute_thirty_six_instructions`: Performs thirty-six instructions and is called by three unnamed routines.
- `execute_thirty_six_instructions_2`: Performs a sequence of thirty-six instructions.
- `interact_with_cia_hardware`: This forty-three instruction routine accesses CIAA, CIAB, and custom hardware registers while calling multiple small subroutines.
- `call_instruction_sequences`: Executes three different sequences of thirty-six instructions.
- `execute_thirty_six_instructions_3`: Executes a sequence of thirty-six instructions.
- `dispatch_hardware_logic`: This forty-nine instruction routine accesses CIAA, CIAB, and custom hardware registers and executes several distinct subroutines.
- `execute_fifteen_instructions_2`: Performs fifteen instructions and is accessed via a relative address.
