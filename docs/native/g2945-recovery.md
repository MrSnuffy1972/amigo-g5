# Game 2945 (bomber 2000 (1993)(h. mueller)(de)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02945). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 83 of 83 named (0 author symbols), 83 of 83 described (11 by rule, 68 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2185 of 2185 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6160 of 6160 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 394889 |
| code | 19868 |
| data | 45832 |
| padding | 1299 |
| unused | 439296 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bomber 2000 (1993)(H. Mueller)(De).adf:Bomber-2000 | 2 | 6149 | 81 |
| Bomber 2000 (1993)(H. Mueller)(De).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 8 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- bomber 2000 (1993)(h. mueller)(de)/Bomber 2000 (1993)(H. Mueller)(De).adf: blocks 212-212 (bytes 108544-109056) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.81)
- bomber 2000 (1993)(h. mueller)(de)/Bomber 2000 (1993)(H. Mueller)(De).adf: blocks 215-215 (bytes 110080-110592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- bomber 2000 (1993)(h. mueller)(de)/Bomber 2000 (1993)(H. Mueller)(De).adf: blocks 217-217 (bytes 111104-111616) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.54)
- bomber 2000 (1993)(h. mueller)(de)/Bomber 2000 (1993)(H. Mueller)(De).adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.64)
- bomber 2000 (1993)(h. mueller)(de)/Bomber 2000 (1993)(H. Mueller)(De).adf: blocks 1154-1154 (bytes 590848-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- bomber 2000 (1993)(h. mueller)(de)/Bomber 2000 (1993)(H. Mueller)(De).adf: blocks 1157-1157 (bytes 592384-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Bomber 2000 (1993)(H. Mueller)(De).adf:Bomber-2000

- `initialize_system`: Initializes system structures and clears multiple global variables, sets up interrupt handlers, and configures hardware registers including DMA and interrupt control.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `handle_input_event`: Reads a byte from a global variable, processes it through another routine three times, then writes it back to a table.
- `check_condition_and_return`: Compares a byte value with itself and returns immediately if they are equal, otherwise continues execution.
- `frag_movem_bsr_moveq_rts`: Short code fragment that saves registers on the stack, then calls another routine (2 times in a row), then restores saved registers from the stack, then loads 0 into d0, then returns.
- `update_object_data`: Writes register values to a structure at an offset and updates a counter, then checks if the counter has reached a limit and resets it if so.
- `process_audio_channels`: Processes audio channel data by decrementing counters, setting up DMA registers when counters reach zero, and managing audio playback state transitions.
- `frag_move_beq_neg_bra_rts`: Short code fragment that copies d2 to d6, then branches if the last result was zero, then negates d6, then jumps to another point in the code, then returns.
- `frag_movem_move_addq_bsr_lea_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d2, then adds 1 to d2, then calls another routine, then loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then calls another routine, then restores saved registers from the stack, then returns.
- `render_text_string`: Copies character data from a lookup table to multiple destinations based on an input string, advancing both source and destination pointers, then calls another routine to convert numbers to strings.
- `frag_movem_move_bsr_lea_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d2, then calls another routine, then loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then calls another routine, then restores saved registers from the stack, then returns.
- `save_registers`: Saves all general-purpose and address registers to the stack, then restores them before returning.
- `copy_character_data`: Copies character data from a lookup table to multiple destinations based on an input string, advancing both source and destination pointers.
- `convert_number_to_string`: Converts a number to a string representation in a buffer, handling negative numbers by setting a flag and using a loop to process digits.
- `wait_for_vblank`: Waits for vertical blank by polling a global variable until it equals -1, then calls another routine and writes -1 to two locations in a data structure, repeating this process a specific number of times based on a counter that is decremented and reloaded.
- `update_position_components`: Updates position components based on comparison results between two values, modifying the first value and setting a flag when differences are detected.
- `initialize_object_properties`: Initializes object properties by clearing flags and setting default values, then copies data from a source pointer to an object structure.
- `update_joystick_input`: Reads joystick input from a hardware register and updates multiple global variables based on button states and movement directions.
- `frag_bsr_rts`: Short code fragment that calls another routine, then returns.
- `calculate_line_parameters`: Calculates line drawing parameters based on coordinate differences, determining step directions and initializing counters for Bresenham's algorithm.
- `update_position_data`: Updates position coordinates based on movement parameters and handles boundary conditions for object positioning.
- `update_object_flags`: Processes object state flags and coordinates, sets a global counter to one when certain conditions are met, and updates object data pointers in a loop.
- `update_object_states`: Updates state variables of a structured data array, including counters and flags, while managing conditional logic based on current values and updating a global variable with a processed value.
- `manage_object_counters`: Manages object counters and state variables, increments a counter until it reaches a limit, then resets it and updates another counter, while clearing object flags when limits are exceeded.
- `validate_and_update_object`: Validates object coordinates against boundaries, updates object state counters, and handles conditional state changes based on object type.
- `update_position_values`: Updates position values in a data structure based on conditional checks and boundary limits, resetting them to a default value when limits are exceeded.
- `adjust_movement_parameters`: Adjusts movement parameters based on global flags and conditions, updates a counter with wraparound logic, and toggles a bit in a global variable.
- `process_coordinate_clipping`: Processes coordinate clipping by adjusting values within bounds, calculates memory addresses for rendering operations, and updates object properties.
- `handle_object_movement`: Processes object movement by updating coordinates with boundary checks and handles wraparound conditions for position values.
- `update_linked_list_pointers`: Updates a linked list of pointers by shifting values in a circular fashion and writes structured data to a buffer using a loop.
- `decrement_counter`: Decrements a 16-bit value in a global variable by 36 each time the routine is called, effectively reducing it by 72 every other call.
- `decrement_counter_2`: Decrements a 16-bit signed integer in a global variable by thirty-six each time, effectively reducing it by 72 in total across all iterations.
- `process_object_animation`: Updates object animation states by decrementing counters and managing frame transitions, while handling conditional execution based on object properties and calling associated animation routines.
- `update_object_position`: Updates object coordinates and state flags based on conditional logic and boundary checks, modifying values at multiple offsets within the object structure.
- `update_object_position_2`: Updates object coordinates with boundary wrapping and manages animation frame counters, then calls rendering functions when specific conditions are met.
- `set_object_state`: Sets object state flags based on coordinate comparisons and manages a counter that triggers rendering operations when it reaches zero.
- `manage_animation_frame`: Decrements an animation frame counter and resets it to 8 when it reaches zero, while also managing a state index that cycles through values 0-2.
- `handle_object_movement_2`: Updates object position coordinates with boundary wrapping, manages a counter that triggers rendering operations, and handles special movement patterns when reaching certain thresholds.
- `execute_rendering_sequence`: Manages a counter that triggers rendering operations, calls multiple rendering functions with calculated parameters, and updates object state flags based on coordinate conditions.
- `adjust_object_direction`: Updates object coordinates with boundary wrapping, manages animation frame counters, and adjusts horizontal movement based on comparison with a global value when a flag is clear.
- `process_object_cycle`: Updates object coordinates with boundary wrapping, manages animation frame counters, and triggers rendering operations when specific conditions are met including coordinate validation.
- `update_bomb_state`: Updates bomb state information including counters and coordinates, processes object rendering and clipping, and manages object lifecycle based on state flags and boundary checks.
- `update_movement_counter_2`: Updates a movement counter and handles boundary conditions for object positioning, including resetting counters when they reach zero or exceed defined limits.
- `update_movement_counter`: Updates a movement counter and handles boundary conditions for object positioning, including resetting counters when they reach zero or exceed maximum values.
- `adjust_position_bounds_2`: Adjusts position coordinates based on boundary checks and updates related counters, managing object movement within defined limits using conditional logic and counter decrements.
- `adjust_position_bounds`: Adjusts position coordinates based on boundary checks and updates related counters, managing object movement within defined limits using conditional branching.
- `process_object_animation_2`: Manages object animation states by updating counters, handling frame transitions, and writing data to a global structure when specific conditions are met.
- `calculate_directional_change`: Calculates directional changes for objects based on position differences and lookup tables, adjusting movement counters and updating object state information.
- `update_global_coordinates`: Updates global coordinate values across multiple data structures with boundary checking and wraparound handling for position tracking.
- `initialize_structure`: Initializes a data structure by setting specific fields to predefined values and advances through array elements until finding a match with 21845, then writes values to the matching element.
- `update_object_data_2`: Updates object data fields with calculated values based on input parameters, writing to memory locations with specific offsets and setting additional fields to fixed values after finding a match with 21845.
- `create_rendering_object`: Creates new rendering objects by initializing their coordinates, state flags, and linking them to a global object list.
- `process_object_updates`: Processes object updates by validating coordinates, calling rendering functions, and updating object state information.
- `check_and_update_flags`: Iterates through a table of 6 entries checking for a specific value at offset 0, and if found, updates a word at offset 26 with a register value before calling another routine.
- `clear_and_process_data`: Clears 16 longwords in a table and then processes 29 entries, performing calculations on coordinates and setting bits in a global variable based on the results.
- `check_bomb_placement`: Validates bomb placement coordinates against boundaries and existing objects, updates object state flags, and manages rendering object creation when conditions are met.
- `reset_data_blocks`: Resets multiple data blocks to the value 21845 by iterating through different memory regions with varying step sizes and loop counts, then sets a final global variable to 21845.
- `frag_moveq_move_mulu_addq_eor_and_rts`: Short code fragment that loads 0 into d0, then copies a location in the code to d0, then multiplies d0 by 221 (unsigned), then adds 1 to d0, then copies a fixed memory address to d1, then exclusive-ors d1 into d0, then copies d0 to a fixed memory address, then masks d0 with 31, then returns.
- `frag_move_moveq_mulu_addq_eor_rts`: Short code fragment that copies d1 to the stack, then loads 0 into d0, then copies a location in the code to d0, then multiplies d0 by 221 (unsigned), then adds 1 to d0, then copies a fixed memory address to d1, then exclusive-ors d1 into d0, then copies d0 to a fixed memory address, then copies the top of the stack to d1, then returns.
- `copy_and_check_entries`: Checks if a specific value exists at offset 0 of a table entry, and if so, copies 6 longwords from an input pointer to the table entry before returning with a success flag.
- `process_and_update_entries`: Checks for a specific value at offset 0 of a table entry, and if found, copies 6 longwords from an input pointer to the table entry, then performs additional calculations on coordinates and updates values in the table.
- `initialize_object_data`: Initializes a table of object data entries with specific bitmasked values from another routine's output, setting coordinates, flags, and parameters for multiple object instances.
- `setup_audio_dma`: Configures audio DMA settings by initializing data structures, processing input data to calculate offsets, setting up DMA channels, and clearing various hardware registers related to sound output.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address (4 times in a row), then copies 15 to a fixed memory address, then clears a fixed memory address, then returns.
- `update_audio_channels`: Updates audio channel registers based on global variables and table lookups, then writes values to hardware registers.
- `update_audio_registers`: Updates audio register values for four sound channels based on table lookups and conditional calculations, then writes the results to hardware registers.
- `update_object_state`: Processes input data to update object position and state variables, including handling movement calculations and boundary checks, then stores results to global structure offsets.
- `update_position_index_2`: Updates a position index value by iterating through a table based on input parameters, clamps the result within bounds, and stores the final value.
- `update_position_index_3`: Updates a position index value by iterating through a table based on input parameters, clamps the result within bounds, and stores the final value at a specific offset.
- `update_position_index`: Updates a position index value by iterating through a table based on input parameters, clamps the result within bounds, and stores the final value to a global variable.
- `update_position_range`: Updates a position value based on input parameters and a lookup table, clamping the result within bounds and storing it to a global variable.
- `calculate_and_update_position`: Calculates position adjustments based on input values and a lookup table, updating a global variable with calculated results before returning.
- `process_input_byte`: Processes a byte from input data, extracts nibbles for indexing into lookup tables, updates global variables with calculated results, and modifies hardware registers based on input values.
- `store_input_mask`: Extracts the lower nibble of a byte parameter and stores it to a specific offset in the routine's data structure.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `frag_movem_lea_move_addq_cmp_bne_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then copies the memory at a0 to d0, then adds 4 to d0, then copies the memory at a0 to d1, then compares d1 with d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `update_position_forward`: Updates position and rendering data for an object moving forward by calculating coordinates based on input values, storing calculated values in memory locations with specific offsets, and updating multiple fields in a structure.
- `update_position_backward`: Updates position and rendering data for an object moving backward by calculating coordinates based on input values, storing calculated values in memory locations with specific offsets, and updating multiple fields in a structure.
- `process_multiple_entries_forward`: Processes multiple entries in two different tables by calling routines with different parameters, incrementing a counter after each call.
- `update_position_reverse`: Updates position and rendering data for an object moving in reverse direction by calculating coordinates based on input values, storing calculated values in memory locations with specific offsets, and updating multiple fields in a structure.
- `update_position_decrease`: Updates position and rendering data for an object decreasing in value by calculating coordinates based on input values, storing calculated values in memory locations with specific offsets, and updating multiple fields in a structure.

### Bomber 2000 (1993)(H. Mueller)(De).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
