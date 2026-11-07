# Game 606 (Klax): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00606). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (919824 top-level bytes in 5 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 54 raw-word lines |
| Functions named and described | pass, 80 of 80 named (0 author symbols), 80 of 80 described (6 by rule, 72 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1981 of 1981 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2904 of 2919 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 9860 |
| code | 10986 |
| data | 4512 |
| padding | 2 |
| unused | 894464 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Klax.adf:bootblock | 1 | 40 | 1 |
| logged disk load from Klax.adf boot-block load | 1 | 0 | 0 |
| logged disk load from Klax.adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1997 | 40 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 880 | 37 |

## Cases handled

- Klax/Klax.adf: the boot block reads 11264 bytes from disk offset 0 to an address (found statically); recovered as a unit
- Klax/Klax.adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1339 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-keys): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 640 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Klax/Klax.adf: 2 logged loads cover 4608 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_dma_parameters` (runtime segment an address): Updates several values in a table and modifies hardware registers to configure DMA and custom chip settings. Kind: DMA/Hardware parameter reconfiguration for custom disk access. Pass path: The routine executes the DMA setup (writing to custom registers/pointers) and returns the original D0/D1 values to the caller, allowing the subsequent disk read/DMA transfer to proceed..

## Functions


### Klax.adf:bootblock

- `initialize_trackdisk_device`: The routine opens the trackdisk.device, adds a message port, finds the current task, and writes to DMACON and custom registers to enable DMA. It then performs I/O via DoIO on the opened device.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_conditional_flags`: Checks two memory locations based on a calculated offset and calls another routine if a specific condition is met.
- `update_entity_kinematics`: Calculates and updates an entity's position, velocity, and frame counters using values retrieved from lookup tables.
- `process_entity_collision_and_state`: Evaluates conditions based on entity properties and table values to update state, modify coordinate offsets, and adjust internal counters.
- `update_object_table`: Updates a value in a table using an offset derived from a pointer and increments a byte at a calculated memory location.
- `update_structure_data`: Writes a constant to a structure, performs a complex calculation involving bit-shifting and multiplication to update another field, and stores a function pointer.
- `update_conditional_state`: Checks various flags and values within two different data structures to conditionally increment counters, modify bitmasks, or update a function pointer.
- `reset_entity_state`: Checks a bit in a structure to decide whether to return, then clears several fields in a structure and modifies a global variable based on a short value.
- `update_entity_physics_and_position`: Calculates velocity and position updates for an entity by applying bit-shifted modifiers to its movement attributes.
- `modify_entity_state`: Updates various fields in a structure, modifies counters, and writes constants to memory based on conditional checks of entity properties.
- `apply_bitwise_or_sequence`: Performs a sequence of bitwise OR operations on a data register using five immediate byte values.
- `validate_and_update_tile_matches`: Iterates through a table of data structures to compare specific byte fields and bitmasks, updating status flags and incrementing counters when matches are found.
- `update_status_and_branch`: Iterates through a memory table to compare values against a field in a structure, then either modifies a value in another structure or updates a field and performs a bitwise OR on a value in a structure.
- `subtract_and_branch`: Adds two words, subtracts a value from a memory location, and branches based on the result.
- `swap_and_subtract`: Swaps the contents of a register, masks it, subtracts the result from a memory location, and branches based on the result.
- `set_global_flag`: Sets a global variable to one and then calls itself recursively.
- `update_status_bits`: Sets a byte in a global table and performs a bitwise OR on a word at an offset from a pointer before calling another routine.
- `update_graphics_buffers`: This routine performs bulk memory transfers to populate graphics buffers from source tables using nested loops.
- `store_and_initialize_table`: Stores a longword into a memory location and then fills a sequence of words with a constant value using a loop.
- `decode_bitfield_to_value`: Processes an input bitfield by checking individual bits and applying arithmetic offsets to a running total.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `populate_byte_array_with_constants`: Decrements a register through a loop and then repeatedly calls another routine to store a constant byte into several memory locations via a pointer.
- `perform_integer_division`: Repeatedly subtracts one value from another until the subtrahend is no longer greater than or equal to the minuend.
- `execute_conditional_callback`: Checks a value in a structure and either executes a function pointer from a table or sets a global variable and calls another routine.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `copy_table_data`: Decrements a counter in a structure, potentially resets it, and uses nested loops to copy data from a table into a destination memory area.
- `update_structure_and_clear_memory`: Decrements a value in a structure, potentially loading data from a table into another structure, and then clears a series of memory blocks.
- `copy_table_to_memory`: Decrements a counter in a structure and, if zero, updates a function pointer and a global variable; otherwise, it performs a nested loop to copy data from a table into a destination memory area and then clears that area.
- `decrement_and_reset_counter`: Decrements a value in a structure and, if it reaches zero, loads new data from a table into that structure and resets the counter.
- `clear_memory_blocks`: Clears multiple large blocks of memory by writing zeros to them in a nested loop structure.
- `update_table_values`: Iterates through two tables to check specific bit flags and perform arithmetic updates on table entries based on conditional logic.
- `copy_memory_blocks_2`: Modifies a byte in a structure and performs multiple memory copy operations using calculated offsets.
- `copy_memory_blocks`: Copies data from one set of memory locations to three other destination sets using nested loops and calculated offsets.
- `execute_main_loop`: Initializes several variables and enters a complex loop that calls various routines and modifies a global variable until a specific condition is met.
- `execute_infinite_loop`: The routine enters an infinite loop that performs no operations.
- `update_string_and_reset_counters`: Updates a character in a structure and decrements counters, potentially writing a sequence of zeroed bytes and characters to a global buffer.
- `configure_system_state`: Sets values in a structure, modifies a global variable, and conditionally calls another routine based on a bitmask check.
- `initialize_function_table`: Clears several fields in a structure and then populates a table with function pointers based on a comparison with a value in a structure.
- `reset_string_buffer`: Checks a value in a structure against a global variable and, if the condition is met, writes a sequence of zeroed bytes and space characters into a memory buffer.
- `check_entity_conditions`: Iterates through a table to count matching entries and updates several fields in a structure if specific criteria are met.
- `reset_structure_fields`: Sets several fields within a structure to zero or specific constant values and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decompress_bitstream`: Reads bits from a stream to determine how many bytes to copy from a table into a destination buffer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `find_next_set_bit`: Shifts a value rightward repeatedly, reloading the value from a table when it reaches zero, to locate a bit position.
- `null_operation`: This routine performs a logical OR operation on a data register with zero.
- `tiny_or`: Small routine of 1 instruction that merges d0 into d0.
- `update_joystick_state`: Checks a global variable and updates several other global variables based on bitwise manipulations of that value and the joystick port.
- `update_status_flags`: Compares a global variable against a specific value and bitmask, then updates several global variables using bitwise shifts, XOR operations, and logical comparisons based on the result.
- `check_input_and_branch`: Reads a bit from a hardware register and, if certain conditions are met, pushes values onto the stack and returns.
- `process_interrupt_data`: Modifies memory at offsets from a pointer based on a bit test and performs a loop to decrement a value.
- `increment_global_counter`: Performs bitwise masking on a global variable, increments a global counter, and calls other routines.
- `mask_interrupt_bits`: Clears specific bits in a global variable and modifies hardware interrupt registers.
- `execute_conditional_traps`: Checks a value, clears it, and then executes a series of system traps or calls another routine based on the result.
- `clear_data_zero`: Performs an operation that clears the contents of a data register with zero.
- `clear_data_register`: Performs a logical OR operation on a data register with zero to ensure it contains no set bits.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `handle_system_trap`: Checks a global variable and, if non-zero, clears it and executes a series of system traps and other routines.
- `copy_memory_block`: Copies a block of memory from a source pointer to a destination pointer using longword and byte increments.
- `copy_data_from_table`: Iterates through a loop to copy values from a source into a global array.
- `verify_magic_numbers`: Compares two longwords at a pointer location against specific constant values and returns zero or a maximum integer.
- `search_and_divide`: Searches a table for a specific two-word signature and, if found, divides a constant by a value at a specific offset to store the result in a global variable.
- `search_and_shift`: Searches a table for a specific two-word signature and stores a bit-shifted version of a value from the table into a global variable.
- `update_dma_parameters`: Updates several values in a table and modifies hardware registers to configure DMA and custom chip settings.
- `manage_audio_state`: Updates global variables and manages audio channel parameters based on bitwise flags.
- `clear_audio_and_reset_state`: Clears multiple audio volume registers and resets a series of memory locations using a pointer-based loop.
- `configure_hardware_registers`: Sets several values to zero or specific constants and interacts with hardware registers including audio volumes and DMA and interrupt controls.
- `reset_global_variable`: Sets a global variable to zero.
- `initialize_and_recurse_global`: Checks a global variable, sets it to a specific constant value, and performs a recursive call to itself.
- `update_object_attributes`: Updates object data by indexing into a table and modifies multiple audio channel length and location registers.
- `process_object_attribute_table`: Reads a value from a data structure to index into a table, then updates various fields in the structure based on the retrieved data.
- `modify_global_flags`: Clears specific bits in a global variable and then sets a high bit before calling another routine.
- `configure_audio_registers`: Updates multiple audio channel length and location registers and writes a value to a custom hardware register.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_object_position_and_state`: Updates a 12-bit positional value within a data structure based on a command byte, handling movement increments, decrements, and boundary clamping.
- `call_and_load_register`: Calls another routine and then loads a word from a memory location into a register.
- `decrement_global_value`: Decrements a global variable by a fixed amount and clears two memory locations using a pointer.
- `update_input_status`: Stores a value into a global variable and modifies two other memory locations, including a bitwise operation on one.
- `scale_and_store_value`: Multiplies the lower byte of a register by a global variable and stores the result shifted right by eight bits into a memory location.
