# Game 145 (Alianator v1.0 1905): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00145). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (917476 top-level bytes in 13 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 171 raw-word lines |
| Functions named and described | pass, 302 of 302 named (0 author symbols), 302 of 302 described (7 by rule, 292 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 9477 of 9477 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 360085 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 17333 of 17335 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 41681 |
| code | 51540 |
| data | 824246 |
| padding | 9 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 247 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3192 | 66 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 108 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13678 | 196 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 107 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| mainpart (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Alianator_v1.0_1905/Alianator_v1.0_1905.hdf: hard-disk image (AmigaDOS OFS, 896 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree Alianator_v1.0_1905_hdf, which is recovered file by file; no block outside the filesystem holds data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 2627 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 93 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6481 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 92 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Alianator_v1.0_1905/Alianator_v1.0_1905_hdf/Alianator/data/mainpart: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 360085 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_indirect_copy`: Executes a routine via a pointer and subsequently copies a block of instructions to a new location before triggering a trap.
- `execute_indirect_routine`: This routine executes a routine through a pointer retrieved from a memory location.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `call_indirect_subroutine`: Executes a routine through a pointer retrieved from a memory location.
- `copy_and_trap`: Copies a block of instructions from a source address to a destination address and then executes a trap instruction.
- `set_global_and_loop`: Sets a global variable to a fixed value and executes a loop that decrements a register until it reaches a certain value.
- `copy_memory_block_4`: Calculates a destination address based on a global variable and copies a block of memory using a loop.
- `copy_memory_block_3`: Performs a complex memory copy using a loop that checks or decrements a counter and moves bytes between a source and destination area.
- `copy_memory_block_2`: copies a certain number of bytes from a source memory area to a destination memory area using a loop that decrements a counter or checks for a terminator byte.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `update_global_pointers`: Sets a series of global variables and pointers to a specific address before calling a routine through a pointer.
- `update_table_values`: Iterates through a table of integers and replaces a value if it matches a certain amount.
- `set_and_wait_blt_status`: Writes a value to a global variable and polls a specific bit until it is cleared.
- `set_and_wait_blt_status_2`: Writes a different value to a global variable and waits for a specific bit to clear.
- `set_and_wait_blt_status_3`: Writes a value to a global variable and loops while a certain bit remains set.
- `set_and_wait_blt_status_4`: Writes a value to a global variable and polls a bit for a certain state.
- `set_and_wait_blt_status_5`: Writes a value to a global variable and loops until a bit is cleared.
- `set_and_wait_blt_status_6`: Updates a global variable and waits for a bit-clear condition.
- `set_and_wait_blt_status_7`: Sets a global variable and waits for a bit to change state.
- `store_and_wait_blt_status`: Stores a word from a source memory location to a destination and then polls a bit in a global variable.
- `store_and_wait_blt_status_2`: Stores a word from an offset source to a destination and then waits for a bit in a global variable to clear.
- `store_and_poll_bit_2`: stores a word from a source pointer to a destination pointer and then waits for a bit in a global variable to clear.
- `wait_for_bit_clear_and_store`: Stores a word to a memory location relative to a register and then polls a bit in a global variable until that bit is cleared.
- `store_and_poll_bit`: stores a word to a memory location calculated via an offset from a pointer and then waits for a specific bit in a global variable to clear.
- `store_and_wait_bit`: stores a byte to a memory location relative to a pointer and waits for a specific bit in a global variable to clear.
- `wait_for_bit_clear_and_store_2`: Stores a word to a memory location relative to a register and then polls a bit in a global variable or hardware register until that bit is cleared.
- `set_global_from_checks`: Checks several global variables and, if any are non-zero, sets a global variable to a constant value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `calculate_interpolated_values`: Calculates two new values using a scaling factor derived from a register and stores them into a memory table.
- `empty_routine_2`: The routine is empty and performs no operations before returning.
- `calculate_blt_dimensions`: Calculates the width and height for a bitblt operation using a fixed-point or integer-based scaling or interpolation method.
- `store_register_block`: Iteratively stores a sequence of registers and a value from a pointer into a block of memory relative to a global variable, and calls a routine to calculate bitblt dimensions.
- `populate_attribute_buffer`: Iteratively writes a sequence of 32-bit or 16-bit values into a memory buffer, where the source or destination or certain values are determined by a decrementing counter or a calculated offset.
- `populate_scaled_buffer`: Iteratively writes or copies values from a source pointer into a destination buffer, using a decrementing or incrementing step or offset.
- `populate_scaled_attribute_table`: Iterates through a calculated range to store or update a sequence of values in a memory-mapped table or structure.
- `populate_attribute_table`: Iterates through a memory range to store or update a sequence of values into a table or structure.
- `populate_attribute_table_2`: Iterates through a memory range to write a sequence of values into a table or buffer.
- `populate_attribute_table_3`: Iteratively writes a sequence of values, derived from a source pointer and a decrementing or incrementing counter, into a block of memory or a table.
- `update_sprite_attribute_table_3`: Iterates through a range of memory or a list of pointers to update a table of attribute-like structures.
- `populate_scaled_buffer_2`: Iteratively writes or copies values into a memory buffer using a decrementing or incrementing pointer or index.
- `populate_attribute_table_4`: Iteratively writes or updates values in a memory-mapped table or structure, likely used for storing sprite or object attributes.
- `populate_memory_block`: Iteratively writes a sequence of 32-bit or 16-bit values (derived from a source pointer or register) into a destination memory range.
- `populate_buffer_with_scaled_data`: Iteratively writes or copies data or register-stored values into a destination buffer using a decrementing counter and a step-based or scaled offset.
- `populate_sprite_attribute_table`: Iterates through a source buffer using a fixed-point or integer-based step, writing or copying attribute or pixel data into a destination table or buffer.
- `populate_scaled_attribute_buffer`: Iteratively writes or updates a sequence of values into a memory buffer, using a decrementing counter and a fixed-point or integer-based step or offset.
- `populate_sprite_attribute_table_2`: Iterates through a loop to write a sequence of values, including previously stored register contents and a source pointer, into a table in memory.
- `populate_attribute_table_5`: Iteratively writes a sequence of values or register-derived data into a table or memory block.
- `populate_bitblt_parameter_table`: Iteratively calculates and stores a sequence of bitblt-related parameters, such as source or destination pointers and register-like values, into a table in memory.
- `update_sprite_attribute_table_4`: Iterates through a range of memory or a table, calculating a new pointer or offset and storing a sequence of registers or previously read values into a destination buffer.
- `populate_scaled_buffer_3`: Iteratively writes or copies values from a source pointer into a destination buffer or table, using a decrementing counter or a fixed-point-style step to determine the next source or destination address.
- `populate_attribute_table_6`: Iteratively writes or updates values in a memory-mapped table or buffer using a pointer and a decrementing counter.
- `update_audio_dma`: Updates audio volume and length registers and modifies the DMA controller enable bit.
- `update_audio_parameters`: Updates certain audio hardware registers, including period and volume, by calculating or retrieving values from a table based on a bit-manipulation loop.
- `set_audio_registers`: Sets audio channel and length registers using a value from a table.
- `check_value_range`: Checks if a register contains a value within a specific range and returns if not.
- `calculate_scaled_offset`: Calculates a value using a multiplication and bit-shift, then subtracts it from a value in a table.
- `execute_recursive_calls`: The routine performs a recursive call to itself, with the execution path determined by a conditional check of a global variable.
- `initialize_memory_block`: Clears a series of memory locations and calls another routine.
- `recalculate_global_state_bounds`: Updates a global variable through a series of conditional checks and recursive calls, applying bitwise shifts and arithmetic offsets to force the variable into specific ranges or constant values.
- `scatter_table_bytes`: reads bytes from a source table and writes them to a destination buffer using a non-contiguous stride.
- `populate_buffer_from_table`: Calls a routine that writes bytes from a source table into a destination buffer using a fixed stride.
- `scatter_table_bytes_2`: Executes a routine that copies bytes from a source table to a destination buffer using a fixed stride.
- `scatter_table_entries_to_buffer`: Reads individual bytes from a source table using an offset and writes them to a destination buffer at a fixed, non-contiguous stride.
- `iterate_table_and_update_global`: Iterates through a table of values and updates a global variable if a certain value is not present.
- `apply_bitmask_to_table`: Applies a bitmask to a set of values from a source table and stores them in a destination table.
- `calculate_address_and_setup_blt`: Calculates a memory address and sets up a register with a constant before calling another routine.
- `calculate_address_and_setup_blt_2`: Calculates a memory address using a table and sets up a register before calling another routine.
- `update_input_state`: Checks the status of certain input buttons and updates a bitmask in a global variable.
- `execute_indirect_function`: Clears a global variable and executes a function pointer stored in a table.
- `copy_and_set_globals`: Calls a routine to copy memory blocks and then performs a series of writes to a global variable.
- `set_globals_conditionally`: Checks if a global variable is zero and, if so, calls a routine to copy memory blocks before writing several values to a global variable.
- `copy_memory_blocks`: copies a series of memory blocks or structures from a source table to a destination buffer.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `clamp_dma_parameters`: Checks if certain input values fall outside a specific range and clamps them to a predefined minimum or maximum.
- `update_dma_parameters`: calculates and sets a value in a global variable and a register-based calculation, then triggers a trap.
- `interpolate_value_forward`: performs a linear interpolation or lookup-based calculation to determine a new value and stores it in a global variable.
- `interpolate_value_backward`: calculates a value using a table-based or division-based approach and checks if it meets a certain threshold.
- `interpolate_value_alternate`: performs a calculation involving a table lookup and a conditional check against a global variable.
- `calculate_linear_interpolation`: performs a fixed-point linear interpolation or a lookup-based scaling calculation using a table or a reciprocal-based approximation.
- `configure_blitter_parameters`: Sets several blitter control and attribute registers to specific values and then waits for the blitter to complete its current operation.
- `initialize_data_table`: iteratively writes zeroed or constant values into a specific range of memory.
- `initialize_data_table_offset`: iteratively writes zeroed or constant values into a different range of memory.
- `configure_dma_and_blitter`: Sets a global variable to specific values and configures the blitter and DMA controller, then calls a routine that decrements a counter.
- `decrement_counter`: decrements a local register-based counter or variable until it reaches zero.
- `clear_table_and_reset_global`: Iterates through a memory range to clear a table with zeros and resets a global variable to zero.
- `reset_globals_to_constant`: Writes zero to four global variables and writes a constant value to a fifth global variable.
- `reset_global_state`: Increments a global variable and, if a certain condition is met, resets a global variable to zero, performs a loop that decrements a register, and updates several memory locations from a table.
- `update_audio_sample_parameters`: Updates audio register or DMA settings by reading a control structure and a lookup table.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_timer_and_position`: Updates a pointer-based timer or position value by adding or subtracting a certain amount, while checking for certain boundary or zero-limit conditions.
- `increment_and_recurse_global`: Increments a global variable and recursively calls itself if the variable does not reach zero.
- `read_mouse_input`: Reads a byte from the first or second joystick port and performs a series of arithmetic operations on a global variable that results in no net change to its value.
- `decrement_and_mask_global`: Decrements a global variable until it reaches zero and then applies a bitwise mask to it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_global_data`: Repeatedly overwrites a global variable with a sequence of hardcoded constants and addresses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_relocation_offsets`: calculates and stores relative offsets for a jump table and populates a secondary table with an inverse-proportional scaling factor.
- `initialize_relocation_tables`: Calculates and stores relative offsets for a series of function pointers into a table, then populates a secondary table with an inverse-proportional-style calculation, and finally clears several memory-mapped or global data buffers with a fixed pattern of zero and maximum-value words.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_dma_and_hardware_state`: Initializes a series of global variables and tables, configures DMA controller settings, and sets up interrupt enable bits.
- `increment_and_reset_global_counter`: Increments a global variable and resets it to zero if it reaches a certain threshold.
- `compute_object_bounding_extents`: Iterates through a list of objects to calculate the total perimeter or bounding box dimension, storing the calculated size and the object's pointer into a destination table.
- `update_table_parameters`: Calculates an offset based on a clamped global variable and copies a set of parameters from a source table into a destination buffer.
- `copy_table_parameters`: Calculates offsets and copies bytes from a source table into a destination buffer, while also clearing a specific offset in that buffer and modifying the DMA control register.
- `calculate_bounding_box_extents`: Calculates the maximum absolute distance or range between several memory-mapped or global pointers.
- `update_sprite_transform_table`: Iterates through a list of control parameters to calculate and store new coordinate or size values into a destination table.
- `apply_affine_transform_to_sprites`: Iterates through a list of control parameters to calculate and store new sprite or object coordinates and sizes into a destination table using a matrix-based transformation.
- `compute_complex_matrix_transform`: Performs a series of signed multiplications and bit-shifts using values loaded from a table to update a global variable.
- `calculate_sprite_transform`: Iterates through a list of input values to calculate new coordinate or size parameters using a combination of table lookups, bit-shifting, and fixed-point multiplication.
- `interpolate_texture_coordinates`: Iterates through a list of control points to calculate or interpolate texture-mapped or transformed vertex coordinates.
- `populate_lookup_table`: fills a table with calculated byte values using a nested loop and a scaling factor.
- `initialize_complex_table`: performs a series of nested loops to write specific bitwise-manipulated values into a global table.
- `fill_address_tables`: populates three separate memory areas with an incrementing sequence of integers.
- `reset_global_variable`: sets a global variable to zero multiple times.
- `set_global_and_call`: sets a global variable to a specific value and calls another routine twice.
- `copy_memory_block`: calculates a source address based on a global variable and copies a block of words to a destination offset from a global variable.
- `update_conditional_values`: decrements a global variable and writes a constant byte to a specific memory address or a global offset depending on the previous value.
- `initialize_large_table`: fills a memory range with a constant, then fills a subsequent range with values derived from a division-based formula.
- `wait_for_register_condition`: decrements a counter while checking if a specific bitmask in a global variable matches a certain value.
- `wait_for_bit_clear`: loops until a specific bit in a global variable is cleared.
- `validate_sprite_bounds`: Checks if a set of sprite or object coordinates or dimensions fall within a certain range.
- `configure_dma_and_interpolate`: Enables DMA-driven or certain DMA-enabled hardware transfers while performing conditional linear interpolation or boundary-checked value scaling.
- `increment_pointer_value`: Increments a value or a pointer stored in a memory location by a fixed amount.
- `update_pointer_offset`: Increments a pointer or a value stored in a memory location by a fixed amount.
- `update_dma_attribute_list`: Updates a linked list of attribute nodes by calculating interpolated attribute values and enabling DMA channels via the DMA controller.
- `update_dma_control_register`: Sets or clears a bitmask in a DMA controller register to enable or disable specific DMA channels.
- `update_interpolated_table_entry_2`: Calculates a new value for a secondary table entry using fixed-point interpolation or scaling based on a primary table.
- `swap_boundary_bounds_and_update_table`: Swaps two pairs of boundary values stored in a structure and then updates a secondary table or boundary value using a fixed-point interpolation lookup.
- `interpolate_table_bounds`: Calculates an interpolated value using a fixed-point scaling factor and a lookup table, then updates a secondary table or swaps boundary values in a structure.
- `update_interpolation_bounds`: Calculates new boundary values using a fixed-point interpolation or scaling method and updates a secondary table.
- `update_interpolation_table_bounds`: Updates a secondary table with new interpolated or scaled values by searching a primary table for a minimum value and calculating new bounds or scaled entries based on the distance between table entries.
- `interpolate_table_values`: Finds the minimum value in a table and calculates new scaled or interpolated values for a secondary table based on the distance between table entries.
- `recalculate_table_interpolation_bounds`: Searches a table for a minimum value and updates a secondary table with new interpolated or scaled values.
- `update_interpolated_table_entry`: Updates a secondary table entry by calculating a new value using a fixed-point interpolation or scaling factor derived from a primary table.
- `recalculate_interpolated_table_bounds_3`: Calculates new interpolated or scaled values for a secondary table based on the distance between entries in a primary table.
- `update_table_interpolation_bounds`: Identifies the minimum value within a table and updates adjacent table entries using fixed-point interpolation or scaling.
- `recalculate_interpolated_table_bounds`: Identifies the minimum value within a table and calculates new interpolated or scaled values for adjacent table entries using fixed-point multiplication.
- `update_interpolated_table_bounds`: Searches a table for a minimum value and updates a secondary table with new interpolated or scaled values.
- `recalculate_interpolated_table_bounds_2`: Identifies the minimum value in a table and updates a secondary table with new interpolated or scaled values.
- `interpolate_table_values_2`: Finds the minimum value in a table and calculates new interpolated or scaled values for a secondary table using a fixed-point multiplication lookup.
- `calculate_scaled_table_values`: Iterates through a table to find a minimum value and then calculates or updates a secondary table with scaled or interpolated values based on the difference between table entries and a target value.
- `enable_dma_channels_8`: Sets a bitmask or a specific value in a DMA controller register to enable or disable certain DMA channels.
- `enable_dma_channels_9`: Enables a DMA channel by writing a value to a DMA controller register.
- `enable_dma_channels_10`: Sets a bitmask or a specific value in a DMA controller register to enable certain DMA channels.
- `enable_dma_channels_11`: Enables a specific DMA channel or a set of DMA channels by writing a value to a DMA controller register.
- `enable_dma_channel`: Configures and enables a DMA channel by writing a bitmask or a specific value to a DMA controller register.
- `enable_dma_channel_three_3`: Configures and enables a DMA channel by writing to a DMA controller register.
- `enable_dma_channels_7_2`: Configures and enables a specific set of DMA channels by writing a bitmask to the DMA controller enable registers.
- `enable_dma_channels_7`: Configures and enables specific DMA channels by writing to the DMA controller enable registers.
- `enable_dma_channels_6`: Configures the DMA controller by setting specific bits in the DMA enable register.
- `enable_dma_channels_5`: Activates multiple DMA channels by setting specific bits in the DMA controller enable register.
- `enable_dma_channels_4`: Sets a bit in a DMA controller register to activate a specific DMA channel.
- `enable_dma_channel_3`: Sets a specific bit in the DMA controller enable register to activate a third DMA channel.
- `enable_dma_channel_three_2`: Configures and enables a DMA channel by modifying a DMA controller register.
- `setup_dma_channel_three`: Configures a DMA channel and enables a DMA channel by modifying a DMA controller register.
- `configure_dma_channel_three`: Sets a bit in the DMA controller register to activate a specific DMA channel and updates a memory location with a register value.
- `enable_dma_channel_three`: Sets a bit in the DMA controller register to activate a specific DMA channel.
- `set_register_value`: stores a value from a register into a memory location.
- `enable_dma_channel_bit`: Sets a specific bit in the DMA controller enable register to activate a DMA channel.
- `activate_dma_channels`: Sets a bit in the DMA controller enable register to activate a specific DMA channel.
- `update_function_pointer`: Updates a global pointer or a value in a table with the address of the routine itself.
- `enable_dma_channels_3`: Writes a value to the DMA controller register with the enable bit set to activate a DMA channel.
- `calculate_dma_parameters`: Calculates or updates a global variable or table with new timing or offset values based on a lookup or calculation involving a pointer-based list.
- `configure_dma_priority`: Resets a priority table index and configures or enables specific DMA channels by writing values to DMA controller registers.
- `reset_priority_index`: Resets the priority table index or pointer to its initial state.
- `reset_priority_table_index_2`: resets the index or pointer used to track the minimum value within a priority table to its initial state.
- `reset_priority_table_index`: Resets the index or pointer used to track the minimum value within a priority table to its initial state.
- `initialize_priority_table`: Loads the base address of a priority table and resets the pointer or index used to track the minimum value within that table.
- `get_priority_index_address`: loads the address of a priority index or pointer into a register.
- `reset_priority_table_pointer`: Resets the pointer or index used to track the minimum value within the priority table.
- `initialize_priority_table_pointer`: Loads the base address of a priority table into a register and updates a pointer or index to track the location of the minimum value within that table.
- `load_priority_table_base`: Loads the base address of a priority table into a register.
- `get_priority_table_address`: loads the base address of a priority table into a register.
- `find_minimum_priority_index_4`: Scans a table of priority values to identify and update the index containing the minimum value.
- `update_priority_index_with_value_2`: updates a priority value in a table and subsequently triggers a scan to find or update the index containing the minimum priority value.
- `update_priority_index_5`: Updates a pointer or index to track the location of the minimum value within a priority table.
- `update_priority_index_4`: Updates a pointer or index to track the location of the minimum value within a priority table.
- `update_priority_index_wrapper`: This routine acts as a wrapper that calls a secondary routine to scan a table of priority values and update the index containing the minimum value.
- `update_priority_index_2`: Scans a table of priority values to identify or update the index containing the minimum value.
- `get_unnamed_routine_purpose`: The provided Ghidra pseudo-code contains only unreachable block warnings and lacks any executable instructions or logic-bearing operations.
- `update_priority_index_with_value`: Updates a priority value in a table and subsequently triggers a scan to find or update the index containing the minimum priority value.
- `update_priority_index_3`: scans a table of priority values to identify or update the index containing the minimum value.
- `update_pointer_value`: Updates a memory location with a new address or value.
- `update_priority_index`: Scans a table of priority values to identify or update the index containing the minimum value.
- `find_minimum_priority_index_3`: Scans a table of priority values to identify the index containing the minimum value.
- `find_minimum_priority_index_2`: Scans a table of priority values to identify the index containing the minimum value.
- `find_minimum_priority_index`: Scans a table of priority values to identify the index containing the minimum value.
- `update_dma_priority_table`: Scans a table of priority values to find the minimum entry and updates a global variable or table with a new priority or weight based on the difference.
- `find_minimum_dma_priority`: scans a table of values to find the minimum entry and calculates a priority or weight based on the difference between that minimum and a target value.
- `enable_dma_channels_2`: Sets a bit in the DMA controller or any hardware register to enable DMA channels.
- `do_nothing`: The routine is empty and performs no operations.
- `test_bit_status`: checks a specific bit within a byte located at a memory address and returns a status or branch-dependent result.
- `check_bit_status`: Tests a specific bit within a byte located at a memory address.
- `configure_dma_controller`: Sets the DMA controller enable bit via the DMACON register.
- `verify_entity_coordinate_bounds`: Retrieves multiple coordinate or property values from a table using offsets or indices, then performs a series of range checks to determine if these values fall within or outside specific minimum and maximum thresholds.
- `validate_entity_bounds`: Checks if a set of entity or object coordinates, retrieved from a table using an index, fall within a specific valid range or satisfy certain boundary conditions.
- `validate_and_update_bounds`: Checks if certain pointer or index values fall within a valid range and, if so, updates a global table with new boundary or offset values.
- `validate_address_range`: Checks if two provided pointer or index values fall within a valid or non-negative range.
- `enable_dma_channels`: Sets a bit in the DMA controller or any register that enables DMA or any DMA channel.
- `interpolate_attribute_value_2`: Calculates a new attribute value using linear interpolation or a lookup table and updates a node or attribute within a linked list or tree structure.
- `interpolate_attribute_value`: Calculates a new value using linear interpolation or a lookup table and updates a node or attribute within a linked list or tree structure.
- `update_attribute_node_list`: Updates a linked list or a tree-like structure by inserting or replacing a node, or modifying a pointer, based on the values of a provided register and existing memory contents.
- `interpolate_value`: Performs a conditional linear interpolation or calculation using a table or a division-based fallback, then checks if a calculated value exceeds a certain threshold.
- `interpolate_value_alt`: Calculates a value based on a table or a division-based fallback using different register inputs, then checks if a calculated value exceeds a certain threshold.
- `interpolate_value_variant`: Performs a conditional calculation using a table or a division-based fallback, then checks if a calculated value exceeds a certain threshold.
- `calculate_linear_interpolation_2`: Performs a fixed-point or scaled-integer linear interpolation or a lookup-based slope calculation.
- `clear_specific_bits`: Clears a specific bit in a byte at a pointer and in three other fixed offsets within the same memory block.
- `empty_routine_3`: the routine is empty and performs no operations.
- `branch_if_bounds_valid`: Checks if two input registers fall within a certain range and, if so, executes a routine or a branch-to-address calculated from a register.
- `clear_bits_with_mask`: Clears a specific bit in several memory locations using a bitwise AND with an inverted mask.
- `set_and_clear_bits`: Sets a bit in two memory locations and clears a bit in several other memory locations using a bitwise mask.
- `modify_bitmask_attributes`: Sets specific bits in certain memory locations while clearing others using a bitwise NOT of a register value.
- `set_and_clear_bits_mixed`: Sets a bit in several memory locations and clears a bit in two other memory locations using a bitwise mask.
- `apply_bitmask_to_table_2`: Performs a bitwise OR operation between a register and several non-contiguous memory locations pointed to by a pointer.
- `update_bitmask_set`: updates specific bits in a set of memory-mapped or buffered bitmasks using bitwise AND or OR operations based on a lookup table.
- `branch_if_bounds_valid_2`: Verifies that two input registers are within a specific range before performing an indirect call to a routine located at an offset from a global variable.
- `set_bits_in_memory`: Performs a bitwise OR on a memory location and clears certain bits in other memory locations using a bitwise AND with a negated register value.
- `apply_bit_mask_and_dispatch`: Performs a bitwise OR and a bitwise AND with a bit-inverted register value on several memory locations via a pointer, then executes an indirect jump to a table if a certain register is non-zero.
- `set_bits_in_memory_2`: Performs a bitwise OR on a memory location and clears certain 32-bit or 16-bit memory locations using a bitwise AND with a negated register value.
- `set_bits_in_memory_3`: Performs a bitwise OR on a memory location and clears certain bits in other memory locations using a bitwise AND with a negated register-derived value.
- `check_bit_mask_bounds`: Performs bitwise logic on a word from a table and checks if certain bits are clear.
- `apply_bitwise_mask_to_struct`: Performs bitwise OR and AND operations using a bitwise NOT of a register value on several members of a structure pointed to by a register.
- `set_bits_in_memory_4`: Performs a bitwise OR on certain memory locations and clears certain other memory locations using a bitwise AND with a negated register value.
- `update_sprite_plane_bits`: Sets the bits given in D2 in the word at A0 and in the word at offset 42 bytes-words further (index 21), and clears those bits in the words at indices 42 and 63, treating the four words as parallel bitplane or flag masks.
- `apply_bitmask_to_memory`: Performs bitwise OR and bitwise AND (with a bit-inverted mask) on several memory locations pointed to by a register.
- `mask_and_branch_on_bounds`: Performs bitwise masking on several memory locations using the bitwise NOT of a register, then executes an indirect call to a jump table if a certain register is non-zero.
- `update_bitmask_and_dispatch`: Performs bitwise OR or AND operations on several memory locations pointed to by a register, then executes an indirect jump to a jump table if a certain register value is non-zero.
- `modify_and_branch_on_bounds`: Performs bitwise masking on several memory locations using a bitwise-not value from a register, then executes an indirect call if a certain register is non-zero.
- `test_bit_pattern_in_memory`: Checks if certain bits are absent in two memory locations by masking a register with a bitwise-inverted value.
- `test_bit_mask_pattern`: checks if certain bits are absent in two memory locations using a bitwise mask.
- `check_long_bit_mask_bounds`: Performs bitwise logic on a longword from a table and checks if certain bits are clear.
- `modify_bitmask_and_branch`: Sets specific bits in two memory locations using a register and clears certain bits in two other memory locations using a bitwise NOT, then performs an indirect call if a register is non-zero.
- `update_bitmask_and_branch`: Sets specific bits in a memory-resident bitmask or flagset using a register, clears certain bits using a bitwise NOT, and performs an indirect call if a certain value is non-zero.
- `update_bitmask_and_dispatch_2`: Performs bitwise OR and AND operations on specific memory locations relative to a pointer, then executes a conditional indirect jump to a jump table if a register value is non-zero.
- `update_bitmask_via_pointer`: Iterates through a memory range, performing bitwise OR and AND operations using a bitmask or flag.
- `update_bitmask_via_offset_loop`: Iterates through a sequence of memory offsets to apply a bitwise mask to a target or a bitwise-not-inverted mask to a target.
- `update_pointer_data`: Performs bitwise OR or AND operations on a longword at a pointer offset and a certain offset, then branches if a register is non-zero.
- `modify_pointer_values`: Performs bitwise OR or AND operations on certain halfword offsets of a pointer, then branches if a register is non-zero.
- `modify_bitmask_and_dispatch_jump`: Performs bitwise OR and AND operations on specific memory locations pointed to by a register using a register-held mask, then executes an indirect jump if a register-held value is non-zero.
- `apply_bitwise_mask`: Performs bitwise OR or AND operations on a longword at a pointer offset and a certain offset, then branches if a register is non or-zero.
- `update_bitmask_via_pointer_2`: performs a bitwise or operation using a register and a value from a memory location, then applies a bitwise and using a bit-inverted register value.
- `update_sprite_bitmask_flags`: Applies bitwise set and clear operations using a mask in D2 to the words of the structure at A0, combining a second value in D3, then conditionally dispatches through an indirect jump table when D7 is nonzero.
- `update_and_verify_mask`: Performs bitwise operations and logical checks using a register and a word from a table.
- `update_sprite_attribute_table`: Calculates a new position and attributeset for a sprite or object based on a table lookup and a pointer-based offset, then writes these values to a memory-mapped or temporary buffer before triggering a trap-based system call.
- `calculate_sprite_position_and_width`: Calculates a sprite's horizontal or vertical position and width based on a table lookup, then updates a memory-mapped or otherwise designated attribute table.
- `update_sprite_attribute_table_2`: Calculates a new attribute or position for a sprite or object and writes it to a table in memory.
- `set_global_constants`: Writes a set of constant values to several global memory locations based on a conditional check of a global variable.
- `set_global_constants_fixed`: Writes a set of identical constant values to several global memory locations based on a conditional check of a global variable.
- `set_global_constants_alt`: Writes a different set of constant values to several global memory locations based on a conditional check of a global variable.
- `update_offset_bounds`: Calculates an offset or bound based on a register and writes it, along with other calculated values, to several global memory locations.
- `update_entity_bounds`: calculates and stores new boundary or offset values in a global table based on a masked bit-field from a register, then recursively calls another routine while resetting certain global pointers.
- `update_conditional_table_bounds`: Checks a global variable and, depending on whether it is negative, non-zero, or zero, calculates a value based on a bit-shifted register and updates a set of global memory locations and a routine pointer before calling another routine and restoring certain global values.
- `update_bounds_alt_register`: Calculates a value from a register and writes it, along with other calculated or constant values, to several global memory locations.
- `update_bounds_with_difference`: Calculates a value based on the difference between two register-derived values and writes it, along with other calculated values, to several global memory locations.
- `update_bounds_with_sum`: Calculates a value based on the sum of two register-derived values and writes it, along with other calculated values, to several global memory locations.
- `update_global_state`: Calculates a value based on a register, writes it to several global memory locations, and calls another routine.
- `update_sprite_attribute_table_5`: Calculates an offset into a table to load or update a series of global memory-mapped attribute values, then returns a packed or masked value or a pointer-based result.
- `update_sprite_attribute_table_5_2`: Updates a series of global memory-mapped attribute values by calculating an offset into a table and applying bitwise masks or offsets.
- `execute_indirect_branch`: Performs a conditional check on the difference between two registers and, if certain conditions are met, executes a routine via a pointer stored in a register or a global variable.
- `execute_indirect_branch_2`: Evaluates a conditional branch or comparison and executes a routine via a pointer stored in a register or memory location.
- `dispatch_indirect_call`: Evaluates a conditional branch based on the difference between two registers and then executes a routine located at a pointer stored in a register.
- `update_and_mask_values`: Subtracts values from registers, checks if the result is negative, and applies a bitwise mask.
- `indirect_jump_stub`: Jump stub: passes control to an address held in or computed from register a5, so the target is only known at run time.
- `call_unnamed_routine`: Executes an indirect call using a pointer held in a register.
- `dispatch_indirect_jump`: Loads a function pointer from a pointer-to-pointer and executes it.
- `dispatch_indirect_jump_table`: Updates a memory location with a pointer and a constant value before performing an indirect jump to a destination retrieved from a table.
- `execute_absolute_jump`: Performs an indirect call to a routine address.
- `perform_indirect_jump`: Executes an indirect call to an unrecovered jump table address.
- `store_split_pointer_and_call`: Stores a pointer and its lower half into a memory location relative to a pointer before calling an indirect routine.
- `update_function_pointer_2`: extracts a pointer from a memory location and writes both the full pointer and its lower halfword to a nearby memory offset before branching to the new address.
- `execute_indirect_jump`: performs an indirect jump to an address stored in a memory location or register.
- `write_pointer_to_memory`: Writes a pointer and its lower half to a memory location relative to a pointer and then calls an indirect routine.
- `apply_mask_with_bounds_check`: Loads a halfword from a pointer, performs bitwise operations on several halfword offsets, and executes an indirect call based on range checks.
- `apply_mask_from_table`: Loads a longword from a table, performs bitwise operations on several halfword offsets, and executes an indirect call based on range checks.
- `update_mask_from_index`: Calculates an index using a masked or shifted value from a table and applies bitwise OR and AND operations to several global variables.
- `update_mask_from_two_indices`: Constructs a 32-bit value from two 16-bit values read from a table and applies bitwise OR and AND operations to several global variables.
- `update_mask_from_two_sources`: Reads two separate values from a table and applies bitwise OR and AND operations to a larger set of global variables.
- `clear_and_update_mask`: Reads two values from a table, applies bitwise OR and AND operations to global variables, and explicitly clears several other global variables to zero or certain values.
- `update_bitmask_and_clear_state`: Updates specific bitmasks in a structure using bitwise OR and AND operations with inverted values, while clearing several other memory locations to zero.
- `update_bitmask_flags`: Updates a set of bitmasks by performing bitwise OR and AND-NOT operations using values fetched from a table.
- `update_mask_set`: Loads a word from a table and performs a bitwise OR or AND-NOT on several memory locations.
- `update_mask_set_long`: Loads a longword from a table and performs a bitwise OR or AND-NOT on several memory locations.
- `update_mask_multi`: Loads two different values from a table and performs bitwise OR or AND-NOT on several memory locations.
- `update_mask_set_subset`: Loads a word from a table and performs bitwise OR or AND-NOT on a subset of the memory locations used in other routines.
- `update_mask_clear`: Loads a word from a table and performs bitwise AND-NOT or OR on several memory locations.
- `update_mask_set_alt`: Loads a word from a table and performs bitwise OR or AND-NOT on a different subset of memory locations.
- `update_mask_set_long_alt`: Loads a longword from a table and performs bitwise OR or AND-NOT on several memory locations.
- `update_mask_complex`: Loads two different values from a table and performs bitwise OR or AND-NOT on a large number of memory locations.
- `reset_and_update_mask`: Loads two values from a table, clears certain memory locations, and performs bitwise OR or AND-NOT on several others.
- `update_bitmask_attributes_2`: Updates specific bitmasks or flags within a structure or array by performing bitwise OR and AND operations with values loaded from a table.
- `update_bitmask_set_2`: Loads a word from a table using a calculated offset and applies it to a global variable using a bitwise OR, while also clearing bits in another global variable using a bitwise AND with a bitwise NOT.
- `update_bitmask_set_long`: Constructs a longword from two words read from a table and applies it to a global variable using a bitwise OR, while also clearing bits in another global variable using a bit or bitwise NOT.
- `update_multiple_bitmasks`: Loads multiple words and a longword from a table and applies them to various global variables using bitwise OR and bitwise AND with a bitwise NOT.
- `update_bitmask_attributes_3`: fetches a word from a table using an offset and applies it to several memory locations via bitwise OR and AND-NOT operations.
- `update_bitmask_from_table`: Reads a 32-bit value from a table using two offsets and applies it to several memory locations using bitwise OR and bitwise AND-NOT operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_4`: This routine is empty and contains only bad instruction data.
- `branch_on_range_bounds`: Checks if a value or a range falls within certain bounds and executes a routine or a function pointer from a table.
- `do_nothing_2`: the routine is empty and performs no operations before returning.
- `update_bitmask_attributes`: Loads a word from a table and performs bitwise OR and AND operations on several offsets relative to a pointer.
- `update_and_branch_logic`: Updates bitmask attributes using a table and then executes a conditional indirect call or jump based on register comparisons.
- `clear_blt_buffer_area`: Clears a large block of memory or a buffer or a table by writing zeros to a range of memory.
- `unresolved_routine`: This routine contains thirteen instructions and is called by two unnamed routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_routine`: This routine is empty and performs no operations.
