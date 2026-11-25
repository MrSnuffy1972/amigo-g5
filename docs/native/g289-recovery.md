# Game 289 (galaga '92 v1.4 (1992)(cool-g)(fw)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00289). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901376 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 174 of 174 named (0 author symbols), 174 of 174 described (4 by rule, 169 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3612 of 3612 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4706 of 4706 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 238889 |
| code | 17930 |
| data | 52588 |
| padding | 2145 |
| unused | 589824 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Galaga '92 v1.4 (1992)(Cool-G)(FW).adf:Galaga'92+ | 10 | 4641 | 170 |
| Galaga '92 v1.4 (1992)(Cool-G)(FW).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 54 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 11 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 40 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 12 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 9 jump target, 1 immediate, 1 call target, 1 branch displacement

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Galaga '92 v1.4 (1992)(Cool-G)(FW).adf:Galaga'92+

- `jump_and_manipulate_stack`: Jumps to an absolute address, then performs a series of stack and pointer manipulations involving conditional branches.
- `set_color_register`: Loads a value into a register, adds 126 to it, stores the byte at a memory location, and writes the word to the color register.
- `configure_interrupt_and_update_hardware`: Calls several routines to initialize data structures, modifies custom chip registers including the interrupt enable and DMA control registers, and updates interrupt vectors before returning via an interrupt return instruction.
- `configure_interrupt_and_update_table`: Sets the interrupt enable register to enable interrupts and modifies the custom chip register, then checks several global variables to conditionally decrement counters, call other routines, or execute a loop that populates a table with four entries by splitting a longword into two words and adding an offset.
- `dispatch_command_sequence`: Checks a word in a global variable and either jumps to a routine via a pointer or executes a sequence of subroutine calls based on a command ID. If a specific condition is met, it updates several global variables and performs a series of subroutine calls before returning via an exception return.
- `handle_joystick_input_and_state`: Reads joystick input from the CIAA, updates hardware registers, and performs a series of conditional checks on the input to update various global state words or call other routines.
- `set_active_handler_on`: Stores the value 28070 into a global variable at offset 22516, sets a 16-bit value to 1 in a global variable at offset 21884, and branches to another routine.
- `set_active_handler_off_a`: Stores the value 28117 into a global variable at offset 22516, clears a 16-bit value in a global variable at offset 21884, and branches to another routine.
- `set_active_handler_off_b`: Stores the value 28229 into a global variable at offset 22516, clears a 16-bit value in a global variable at offset 21884, and branches to another routine.
- `set_active_handler_off_c`: Stores the value 28340 into a global variable at offset 22516, clears a 16-bit value in a global variable at offset 21884, and branches to another routine.
- `set_active_handler_off_d`: Stores the value 28450 into a global variable at offset 22516, clears a 16-bit value in a global variable at offset 21884, and branches to another routine.
- `set_active_handler_off_e`: Stores the value 28553 into a global variable at offset 22516, clears a 16-bit value in a global variable at offset 21884, and branches to another routine.
- `set_active_handler_off_f`: Stores the value 28640 into a global variable at offset 22516, clears a 16-bit value in a global variable at offset 21884, and branches to another routine.
- `update_pointer_table`: Calls another routine, loads a pointer from a table using an offset from the program counter, moves a word from that pointer to a register, and if the word is not zero, stores it in a global variable and updates the table pointer before branching.
- `setup_system_state`: Calls two routines, writes 2 to a global variable, writes 1 to another global variable, clears a word at a specific offset, stores a pointer in a global variable, and if a global variable is zero, decrements another global variable and calls three more routines before storing a pointer in a global variable.
- `initialize_system_parameters`: Stores a pointer in a global variable, calls another routine, writes a word to a global variable, calls another routine, and if a word at a global location is non-zero, calls three more routines before setting several global pointers and clearing a word.
- `decrement_and_check_counter`: Decrements a word in a memory location, compares it to 390, and if not greater, calls another routine, tests the value, and if non-zero, writes a longword and a word to specific locations before branching to another routine.
- `initialize_game_state`: Clears a word, calls four routines, sets several global values, copies 32 words from a table to a global location using a loop, calls another routine, clears multiple words, writes 48 bytes to two memory locations, and calls another routine before branching.
- `setup_system_state_2`: Stores several long words into specific global locations, checks three different global words to see if they are greater than zero, and if so, updates a global word by adding 120 to it, calls three different routines, and then performs further conditional calls based on a word in d0.
- `compare_and_branch_handler`: Loads a constant into d0, compares it to a word at a global address, and if they are equal, calls another routine; otherwise it calls another routine and branches.
- `load_initial_data_sequence`: Calls four routines, sets a pointer in a1, calls another routine, stores a pointer in a global variable, and writes a sequence of five words and bytes to a memory location pointed to by a global variable.
- `decrement_and_loop_table`: Calls another routine, decrements a byte in a global variable, and if non-zero, enters a loop that calculates a new byte value, compares a pointer to a constant, and adds a word from a table to the pointer until a zero result is found.
- `load_modified_data_sequence`: Calls four routines, stores a pointer in a global variable, writes 110 to a global variable, and writes a sequence of three words and two bytes to a memory location pointed to by a global variable.
- `decrement_and_call_routine`: Tests a word in a global variable; if it is not zero, it subtracts 1 from the word and branches back to the start of the routine, otherwise it loads an address into a1, calls another routine, and stores the constant 2802 in a global variable.
- `modify_global_and_branch`: Subtracts one from a longword in a global variable, and if the result is non-zero, branches to another routine; otherwise, it writes four to a global word, adds one to another global word, loads a pointer from a location near the program counter into a1, compares a word at offset 4 of the structure in a1 to 122, and branches based on whether the value is less than, greater than, or equal to that constant.
- `decrement_structure_word`: Subtracts one from a word at offset 4 of a structure pointed to by a1 and branches to another location.
- `update_structure_counters`: Increments a word at offset 4 of a structure, then if a word at offset 6 is greater than zero, subtracts a constant from offsets 6 and 44, and if offset 6 remains positive, fetches a word from a table using its value as an index and stores it at offset 46 before calling another routine and checking two global values to potentially update a pointer.
- `modify_structure_and_branch`: Writes a long value, a word, and clears a word at specific offsets within a structure pointed to by a1, calls another routine, and then branches.
- `call_handler_and_branch`: Calls a subroutine and then performs an unconditional branch to another location.
- `process_entity_state_transition`: Decrements a counter in a global variable and, if zero, iterates through a list of pointers.
- `decrement_specific_counter`: Compares a word at offset 24 of a structure in a1 with 1; if it is not equal, it branches, otherwise it loads a memory address into a3, decrements a word at that address, and branches.
- `decrement_global_counter`: Compares a word at offset 24 of a structure in a1 with 4; if it is not equal, it branches, otherwise it loads a memory address into a3, decrements a word at that address, and branches.
- `update_structure_fields_3`: Compares a word at offset 24 of a structure in a1 with 15; if it is greater than 15, it branches, otherwise it writes several values to offsets 24, 38, 26, and 38 of the structure, then calls another routine and branches.
- `move_pointer_and_branch`: Moves a longword from a register to a memory location and then performs an unconditional branch.
- `store_constant_and_branch`: Stores the constant value 2984 in a longword at a memory location and then performs an unconditional branch.
- `initialize_and_branch`: Subtracts a word from a word loaded from a table and branches if the result is non-zero; otherwise, it loads a longword into d0, calls two different routines, writes 200 to a global variable, and branches to another location.
- `initialize_table_data`: Calls another routine three times with different addresses and values, writes 300 to a global variable, constructs a longword by shifting and combining four bytes from a table into a register, calls two other routines, and finally updates a pointer with the address of another routine.
- `clear_and_set_pointer`: Calls another routine, subtracts one from a word in a global variable, and if non-zero, writes a longword to a global location, calls another routine, and branches to another routine.
- `dispatch_object_command_table`: Reads a command ID from a table and executes a corresponding routine or state update based on the ID. Depending on the ID, it may clear memory blocks, instantiate new structures from a table, update global pointers, or perform a sequence of specific data initializations.
- `initialize_entity_from_table`: Iterates through a list of pointers until it finds an entry where a field at offset 24 is zero, then populates that structure with data from a table using an index derived from a register, sets a function pointer at offset 58, assigns specific values to offsets 4, 40, 44, and 46, and calls another routine before resetting the frame index at offset 38 and a counter at offset 36.
- `setup_memory_and_pointers`: Clears a word at a global variable, sets two longwords at global variables, runs a loop 31 times to clear words at offset 2 of a pointer in a0, and sets two longwords at global variables.
- `set_global_pointers`: Writes a longword to a global variable and sets two longwords at global variables using values from a table.
- `copy_data_in_loop`: Writes a word to a global variable, then enters a loop that copies words from one structure to another based on a count in d0, and finally calls another routine and branches.
- `interpolate_nibble_data_table`: Iterates through a table of 32 words, comparing and adjusting the high nibble, middle nibble, and low nibble of each word to match a source table.
- `initialize_structure_from_table`: Calls a routine, loads a pointer from a table, and if non-zero checks a word at offset 24; if zero, it copies data from a table into the structure, updates several fields including a pointer and a word, and performs arithmetic on values from another table.
- `reset_structure_pointer`: If a global variable is not zero, it clears that variable, sets a word at offset 24 of a structure pointed to by the value of that variable to 2, stores a routine pointer at offset 58 of that structure, and clears a word at offset 26 of that structure if two specific global variables are zero.
- `update_state_f0_f1`: If a value from a table is not zero, it checks a 16-bit field at offset 24 of a structure; if the field is 240, it decrements a global variable, and if the field is 241, it decrements another global variable, updates the structure with a new pointer and values, and sets its state to 242.
- `update_state_fe`: If a value from a table is not zero, it checks if a 16-bit field at offset 24 of a structure is 254; if so, it updates the structure with a new pointer and values, sets its state to 254, and clears a global variable before returning.
- `process_state_list`: Iterates through a list of pointers from a table; if a 16-bit field at offset 24 of a structure is 17 or 18, it updates the structure with a new pointer and values, sets its state to 19, and clears several global variables.
- `process_entity_data`: Saves registers, manipulates a word from a structure in a1 by swapping bytes and adding 12, loads three addresses into a2, a3, and a4, compares a word from the address in a3 to 43690, and if not equal, loads a word from a2, sets up registers, calls another routine, and branches to another routine.
- `configure_entity_structure`: Saves registers, loads a pointer from a table, and if non-zero checks a word at offset 24; if zero, it copies data from a table into the structure, clears several fields, performs bit shifts and table lookups, calls another routine, and calculates a final word value.
- `initialize_structure_from_source`: Checks a word at offset 24 of a structure, and if it is zero, copies a pointer from a table into the structure, sets several fields including a function pointer and a counter, calls another routine, and calculates two final word values using a table lookup and a byte from another location before returning.
- `update_structure_state`: Calls four different routines, then checks if a word at offset 36 of a structure pointed to by a0 is 255; if so, it updates the structure by setting a pointer at offset 32, a word at offset 36, a word at offset 38, a word at offset 42, a word at offset 46, and a word at offset 54 using values from a table and other calculations, then calls two more routines.
- `update_entity_complex`: Iterates through a list of pointers from a table; if a 16-bit field at offset 24 of a structure is zero, it updates the structure with a new pointer and state 34, copies a word from a second structure to offset 40, calculates new values for offsets 4, 44, and 46 using a table lookup, and sets the animation state to 3.
- `reset_structure_on_match`: Loads a pointer from a table, checks if the pointer is zero, and if not, compares a word at offset 24 of the pointed-to structure to 34; if they match, it clears that word and updates two other fields at offsets 58 and 26.
- `compare_memory_words`: Loads a pointer from a table into a3, restores d0 and d1 from the stack, and compares d1 to d0, branching to another routine if d0 is less than d1, otherwise it returns.
- `update_entity_fe_variant`: Iterates through a list of pointers from a table; if a 16-bit field at offset 24 of a structure is zero, it increments a global variable, updates the structure with a new pointer and state 254, calculates new values for offsets 4, 44, and 46 using a table lookup, and calls another routine.
- `initialize_entity_from_template`: Checks if a field at offset 36 of a structure is zero; if so, it increments a global counter, copies a template structure into the current structure, sets offset 58 to a specific routine, sets offset 36 to a constant, and updates several fields including offset 46 via a table lookup and offset 44 by adding 5 to a value from offset 6.
- `check_and_call_pointers`: Checks two global pointers; if a pointer is non-zero, it loads the address it points to into a register and calls another routine, otherwise it returns.
- `update_entity_fe_final`: Iterates through a list of pointers from a table; if a 16-bit field at offset 24 of a structure is zero, it increments a global variable, updates the structure with a new pointer and state 254, calculates new values for offsets 4 and 44, copies a word from a second structure to offset 46, and sets the animation state to 2.
- `initialize_entity_from_template_2`: Iterates through a list of pointers until a null pointer or a specific field value is found, then initializes a new entity structure by copying data from a template, setting a counter to 240, updating position and animation frame values from the input structure, and calling update_entity_state_and_animation.
- `spawn_entity_and_calculate_velocity`: Iterates through a list of pointers until a null or zero field is found, then initializes a new entity structure by copying data from a table, setting a function pointer, and calculating initial position and timer values.
- `frag_andi_ori_btst_subi`: Short code fragment that masks d3 with 5, then merges 4 into d2, then tests bit d2 of d0, then masks d4 with 0, then merges 3 into d5, then subtracts 2 from d5, then tests bit d1 of d4, then tests bit d2 of d0, then masks d4 with 0.
- `fetch_table_value`: Masks the lower 5 bits of a word in d0, uses the result as an index into a table to load a byte into d1, and then continues to the next routine.
- `initialize_object_from_table`: If a global variable is not zero, it fetches a pointer from a table, updates a structure at that pointer by adding 224 to a word at offset 24, sets a new pointer at offset 20, stores a routine pointer at offset 58, updates a word at offset 40, performs a swap on a word from a structure in a1, calculates and stores several values at offsets 4, 44, 46, and 36 of the structure, and calls another routine.
- `process_warp_data`: Reads data from a table and updates a global pointer based on the first four bytes, handling 'warp' and 'fini' command codes.
- `parse_and_instantiate_objects`: Checks a magic value in a pointer to determine a mode, then iterates through a data table to instantiate new object structures into a global linked list.
- `load_address_to_a0`: Loads a long word from a memory location relative to the program counter into register a0.
- `execute_indirect_jump`: Loads a long word from the address in a0 into d0, and if d0 is not zero, loads a pointer from d0 into a1, then loads a pointer from the address in a1 into a4, loads a pointer from an offset of 58 in a1 into a3, and jumps to the address in a3; otherwise it returns.
- `update_entity_state_and_pointer`: Compares a 16-bit value at offset 54 of a structure with a constant, and if they match, increments that value by a 16-bit offset from offset 56.
- `compare_and_update_pointer`: Compares a word from a constant to a word at offset 28 of the structure in a1, and if they are not equal, writes a longword to offset 58 of that structure.
- `update_entity_animation_and_position_4`: Updates an entity's frame index and sprite data based on a counter, calculates new coordinate offsets using a table lookup, and writes the resulting position and sprite attributes to a structure and a hardware-related memory area.
- `check_entity_condition`: Subtracts a constant from a word at offset 36 of the structure in a1, applies a bitwise AND with 31 to the result, and branches if the result is non-zero; otherwise, it compares a word at offset 28 of the structure in a1 to one and calls another routine if the value is not one.
- `update_entity_state_and_animation`: Updates an entity's state and animation frame by reading a byte from a pointer in the structure, then modifies the entity's position, sprite pointer, and animation frame index using values from lookup tables.
- `update_sprite_animation_state_4`: Checks an animation mask via a call to check_animation_mask, updates a sprite's frame index and coordinate data within a structure, and writes several calculated values to a hardware-related structure in a5.
- `check_animation_mask`: Loads a word from offset 36 of a structure in a1, subtracts a constant, applies a bitwise AND with 15, and if the result is non-zero, branches to another routine, otherwise it calls another routine.
- `update_entity_animation_and_position_3`: Checks if an entity's animation frame index has changed, updates the sprite pointer from a table, and modifies the entity's position and state fields within its structure.
- `update_entity_animation_and_position_2`: Compares two sets of coordinate values within a structure to determine a state, then updates a state field at offset 36 and a function pointer at offset 58.
- `compare_and_store_pointer`: Loads a word into d0 from a table, compares d0 to a word at offset 28 of the structure in a1, and if they are equal, stores the longword 4879 into offset 58 of the structure in a1, otherwise it branches to another routine.
- `update_sprite_animation_and_position_4`: Decrements frame counters in a structure and, if zero, resets them to values from a table and updates the sprite's animation frame and position.
- `process_timer_value`: Loads a word from offset 36 of a structure in a1, performs an arithmetic shift right, subtracts a constant, applies a bitwise AND with 63, and calls another routine if the result is non-zero.
- `update_sprite_animation_and_position_3`: Decrements a frame counter in a structure and, if zero, resets it to a value from a table and updates the sprite's animation frame and position.
- `queue_multiple_timed_events`: Calculates several future timestamps by adding increasing offsets to a base value and queues multiple events into a structure using a pointer in a register, then calls another routine via a pointer in a structure.
- `update_entity_event_queue`: Checks an animation frame index against a table to determine if a specific state has been reached, then updates a structure with a new function pointer and a status value.
- `queue_object_events`: Updates an object timer and pointer, then performs a series of writes to a structure in a register to queue multiple timed events with varying offsets, finally calling a function pointer from a table if a pointer is non-zero.
- `update_object_position_and_attributes`: Calculates a new position value by adding a constant to a 16-bit value from a structure, then updates several fields in that structure including a scaled version of the position, a modified attribute byte, and a bit-manipulated 32-bit value.
- `update_object_timer_and_pointer`: Compares a 16-bit value in a structure with a global constant, and if they match, increments the value in the structure by an offset and updates a pointer within the same structure by reading from a linked list.
- `decrement_and_update_handler`: Decrements a word at offset 28 of a structure in a1 and, if the result is non-zero, stores a new pointer into offset 58 of the same structure.
- `update_structure_counters_2`: Compares a word at offset 6 of a structure in a1 with a value at offset 32 of the same structure after subtracting 10 from the latter; if the comparison is less than, it increments the word at offset 6, increments a word at offset 44, adds 20 to a word at offset 46, and stores the result back to offset 6, then branches based on whether the value is less than 265.
- `update_entity_position_and_timer`: Increments a 16-bit value at offset 44 and adds 64 to a value at offset 46 of the structure in a1, then updates a 16-bit counter at offset 6.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_pointer_and_counter`: Decrements a word at offset 36 of the structure in a1; if not zero, it branches to another routine.
- `register_function_pointer`: Writes the address of this routine into a pointer located at offset 58 of a structure in a1, then branches to another routine.
- `decrement_timer_and_update_position`: Decrements a word at offset 6 and two other values at offsets 44 and 46 of a structure in a1, then updates a position value at offset 18 by adding a value from offset 16 and shifts the result right by 3 to store in offset 4 before calling another routine.
- `initialize_structure_and_decrement_global`: Clears a word at offset 24 of the structure in a1, stores the long word 7360 at offset 58, and stores the word 2 at offset 26.
- `decrement_and_branch`: Decrements a word at offset 28 of a structure in a1 and branches to another location if the result is not zero, otherwise it stores the constant 8718 at offset 58 of the structure.
- `check_and_update_status_flag`: Loads a pointer from offset 16 of a structure, reads a byte from the location pointed to by that pointer, and if the byte is non-zero, branches to another routine; otherwise, it writes a longword to offset 58 of the structure and branches to another routine.
- `update_sprite_frame_and_position_2`: Checks if the current animation frame index has changed, updates the sprite pointer from a table, and modifies the sprite's position by adding a value to the coordinate offsets.
- `check_structure_threshold`: Loads two words from offsets 4 and 6 of a structure in a1 and branches to different locations based on a comparison of the first word with 242.
- `compare_register_value`: Compares the value in d2 with 8 and branches to different locations based on the result.
- `store_word_and_branch`: Stores the word in d2 at offset 4 of the structure in a1, then branches to a different location if d3 is greater than or equal to 265, otherwise it branches to another location.
- `compare_and_branch`: Compares a word in register d3 to 6 and branches to a different location if it is greater than or equal, otherwise it branches to another location.
- `update_sprite_frame_and_position_3`: Checks if a byte at offset 30 of a structure in a1 has changed, updates a sprite pointer from a table based on that byte, and modifies several coordinate and animation fields within the structure using arithmetic operations and table lookups.
- `update_structure_and_jump`: Stores a pointer to itself at offset 58 of the structure in a1, clears a word at offset 24 of the structure in a1, writes two to offset 26 of the structure in a1, subtracts one from a longword in a global variable, and if the result is zero, stores a longword into a global variable before branching to a location near the program counter; otherwise, it performs an indirect jump using a pointer in a0.
- `decrement_and_reset_structure`: Loads a word from offset 6 of the structure in a1 into d0, subtracts 3 from it, and if the result is less than 3, clears a word at offset 24 of the structure, stores the long word 7360 at offset 58, and stores the word 2 at offset 26.
- `validate_and_reset_entity_state`: Adds three to a word at offset six of the structure in a1 and compares it to one hundred ninety-eight; if less than that value, it clears a word at offset twenty-four, stores two words at offsets fifty-eight and sixty-two, sets a word at offset twenty-six to two, decrements a word at a PC-relative address, and writes two long words to a table before branching to another routine.
- `increment_object_timer_and_update_state`: Increments a 16-bit value at offset 6 of a structure and updates a 16-bit value at offset 46 by adding 32.
- `increment_sprite_animation_frame`: Compares a value from a global variable with a 16-bit value at offset 54 of a structure, and if they match, increments the value at offset 56 by the value at offset 58.
- `update_sprite_animation_state_3`: Decrements a counter at offset 36 of a structure and, if it reaches zero, updates the structure with a new animation frame, resets a counter at offset 24, and calls another routine to update a pointer in a table before jumping to a new address.
- `decrement_and_update_sprite_chain`: Decrements a 16-bit counter at offset 36 of a structure and, if it reaches zero, updates the structure with a new pointer and performs a series of writes to a destination structure using values from a source table.
- `update_entity_timer`: Decrements a timer value at offset 36 of the structure, and when it reaches zero, resets the timer to 5 and increments a counter at offset 38.
- `cycle_sprite_animation_frame`: Decrements a counter at offset 36 of a structure and, if it reaches zero, resets it to 7 and increments a frame index at offset 38.
- `update_entity_state_and_position`: Decrements a counter in a structure, updates its state and function pointer, and performs a collision check against a table of entities.
- `increment_and_update_structure_state`: Increments a 16-bit word at offset 6 of a structure by 1, adds 32 to a 16-bit word at offset 46, and compares the value at offset 6 to 246.
- `update_sprite_animation_state`: Increments a counter and an offset within a structure, and if the counter reaches a specific value, resets it and updates a pointer to a new data structure.
- `compare_timer_values`: Loads a pointer from a table into a3, loads a word from offset 4 of the structure in a3 into d1, adds a word from offset 28 of the structure in a1 to d1, and compares d1 to a word at offset 4 of the structure in a1, branching to another routine if d1 is greater than or less than that value, otherwise branching to another routine.
- `update_entity_animation_and_position`: Decrements a 16-bit counter at offset 64 of a structure, toggles a bit at offset 62, and if a specific flag is set, updates a pointer to a table, increments a timer at offset 6, selects a value from a table based on that timer, and calculates new position and animation values before calling another routine.
- `update_sprite_animation_state_2`: Compares a value from a global variable with a 16-bit field in a structure, and if they match, increments that field by a value from another offset in the same structure.
- `update_sprite_data_table`: Calculates new sprite attributes and positions by reading from a source structure and writing to a destination structure, performing a series of four repetitive updates with increasing offsets.
- `update_entity_state`: Decrements a counter in a structure pointed to by a pointer from a global variable, updates several fields in that structure using values from another structure, and performs a series of conditional writes to a table in a5 based on a bit test of a byte at offset 2.
- `store_long_value`: Stores the constant value 410 in a longword at a global variable location and returns.
- `store_long_value_alt`: Stores the constant value 4502 in a longword at a global variable location and returns.
- `store_long_value_third`: Stores the constant value 8692 in a longword at a global variable location and returns.
- `store_long_value_fourth`: Stores the constant value 16416 in a longword at a global variable location and returns.
- `store_long_value_fifth`: Stores the constant value 19972 in a longword at a global variable location and returns.
- `call_multiple_stores`: Loads four different addresses into a1 and calls another routine four times, passing a different value in d1 for each call.
- `manage_object_list`: Compares the value at a pointer in a1 against several constants and, depending on the match, clears a structure, copies data from a pointer to a table in a5, or updates a bitmask in a5 and modifies the value in a1.
- `process_data_structure`: Loads a pointer from a global variable, iterates through a data block using a loop, and if a condition is met, clears several global variables and calls another routine.
- `write_constant_to_globals`: Stores the value two into four consecutive longword global variables and calls another routine.
- `update_audio_state`: Saves registers to the stack, increments a byte in a global variable, compares that byte to another value, and either clears the global variable and calls another routine or calls another routine twice depending on a test of a third global variable.
- `update_audio_channels_and_sprites`: Updates the four audio channel registers by calling a routine four times with different pointers and data structures, then performs a calculation involving a table lookup and bit shifts to update a value in a global variable and a specific memory location.
- `update_sprite_and_audio_attributes`: Updates a structure in a6 by fetching data from a table based on a nibble-derived index, then performs conditional updates to sprite attributes, hardware registers, and DMA settings.
- `dispatch_sprite_attribute_update_type`: Reads a byte from offset 2 of a structure and uses its lower nibble to branch to various subroutines that perform specific attribute updates, such as scaling coordinates or clearing sprite entries.
- `dispatch_attribute_updates`: Copies a word from offset 16 of the structure in a6 to offset 6 of the structure in a5 and returns.
- `lookup_and_update_scaled_value`: Searches a table using a byte from offset 18 of the structure to find a 16-bit value that satisfies a comparison against a masked 16-bit value from offset 6.
- `update_scaled_coordinate_2`: Adjusts a 16-bit value at offset 16 of a structure based on a byte at offset 23, performs a table lookup using a value from offset 18 and offset 31, and stores the result in a location pointed to by a5.
- `update_scaled_coordinate`: Calculates a value by looking up a byte in a table or performing bitwise scaling on a byte from offset 27 of a structure, then adds this result to a 16-bit value at offset 16 of the structure and stores it at offset 6 of another structure.
- `dispatch_sprite_attribute_updates`: Reads a byte from offset 2 of a structure and uses its lower nibble to branch to specific subroutines that update various fields in the structure, such as scaling coordinates, masking and storing nibbles, or modifying specific bytes at offsets 15, 18, 30, or 31 based on the value of the nibble.
- `mask_and_store_byte`: Extracts the lower nibble from a byte at offset 3 of a structure and stores it at offset 18 of the same structure.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `clear_memory_block`: Clears a block of memory by looping through a count of 277 iterations and writing zero to eight consecutive longwords in each iteration.
- `clear_memory_block_2`: Clears 299 blocks of 8 bytes each at a global variable location by writing zero to each block in a loop.
- `clear_four_memory_blocks`: Clears 599 blocks of 8 bytes each at four different global variable locations by writing zero to each block in a loop.
- `clear_two_memory_blocks`: Clears 299 blocks of 8 bytes each at two different global variable locations by writing zero to each block in a loop.
- `check_status_and_call`: Tests if a longword at a global variable is zero and if a longword at another global variable equals 2952, then calls another routine if both conditions are met before returning.
- `append_object_to_list`: Reads a longword, word, longword, and byte from a source list to populate a structure at a global address.
- `load_and_process_table`: Pops registers from the stack, reads a longword and a word from a pointer in a1, then enters a loop that processes a data table to update a structure in a5.
- `clear_two_words`: Clears two 16-bit words at two specific global memory locations and returns.
- `initialize_global_state`: Calls another routine, clears a 16-bit global variable, and stores a pointer from a1 and two other constants into global variables before returning.
- `update_pointer_and_value`: Calls two routines, then checks a bit in a word from a pointer in a0; if bit 4 is set, it writes the word to a global variable and updates the pointer, otherwise it calls another routine, checks a longword in a1, and if non-zero updates a global variable with it, or clears a word at a global variable and writes 208 to a global variable.
- `load_multiple_tables`: Writes three different longword values into a global variable and calls another routine three times.
- `prepare_table_call`: Saves registers d0 through d7 and a0 through a6 to the stack, loads a constant address into a1, and branches to another routine.
- `save_registers_and_branch`: Saves the contents of eight data registers and six address registers onto the stack, loads a constant value into a1, and branches to another routine.
- `fill_table_with_bytes`: Saves registers, writes the value 32 to three consecutive bytes at a memory location, then enters a loop that reads a byte from a table and, if it is between 5 and 254, writes 65 to a memory location and decrements a counter, otherwise it writes 3 or 65 depending on the byte value.
- `save_registers_and_branch_to_table_loader`: Saves eight data and six address registers to the stack, loads a pointer into a1 from an offset relative to the program counter, and branches into another routine.
- `prepare_table_processing_jump`: Saves all data and address registers to the stack, loads a pointer into a1 from a relative offset, and branches into another routine.
- `update_structure_parameters`: Compares a word from a global variable with another global variable, and if they differ, updates various fields in a structure pointed to by a5, including setting a long-word to a maximum value, clearing a word and a long-word, and updating several other offsets.
- `decrement_and_update_timer`: Decrements a word at a global variable location; if it reaches zero, it sets it to 10, decrements another word, and if that reaches zero, sets it to 10 and negates a third word, then adds a value to a fourth word and stores a pointer in a global variable.
- `update_entity_status`: Loads a pointer from a table, compares a word at offset 36 of that structure to 255, and if equal, performs several operations including copying a pointer, testing bits in a register, and conditionally calling other routines based on comparisons with specific word values.
- `update_structure_fields`: Stores a bit-shifted version of a word from a structure into offset 48, copies the original word to offset 40, calculates a new value by adding a byte from a pointer-referenced location to the word and stores it at offset 42, and finally stores a transformed word at offset 50.
- `toggle_structure_flags`: Decrements a global byte, uses it as an index to locate two structures, and toggles specific bits and long-word values within those structures based on their current contents.
- `decrement_and_call_subroutines`: Decrements two different values stored in memory using relative addressing and, if either reaches zero, sets them to specific non-zero values and calls two different subroutines.
- `increment_and_shift_data`: Increments bytes in a structure through a loop, and if certain conditions are met, shifts a block of data and updates two global long-words with modified values.
- `increment_and_shift_data_alt`: Increments bytes in a structure through a loop, and if certain conditions are met, shifts a block of data and updates two global long-words with different modified values.
- `process_entity_event_queue`: Iterates through a linked list of entity structures, performing bounds checks on various properties and updating entity states.
- `loop_and_modify_registers`: Clears d1 and d2, moves a byte from d0 to d2, and enters a loop that increments d1 and subtracts 10 from d0 as long as d2 is not less than 10.
- `convert_to_ascii`: Adds 48 to the values in d1 and d2 and stores the resulting bytes into two consecutive memory locations pointed to by a1.
- `modify_table_bytes`: Runs a loop six times to update bytes in a table by adding a value derived from a byte and a constant, then enters a loop that processes bytes from a table to perform various memory writes and register updates.
- `modify_indirect_structure`: Loads a long-word from a table into d0, and if it is not zero, uses it as a pointer in a1 to write a constant to offset 58 and checks offset 24; if offset 24 is non-zero, it clears it and writes 2 to offset 26.
- `initialize_and_process_entity_data`: Clears a memory range, copies sixteen words from a table into a global structure, and sets a long word in a global variable.

### Galaga '92 v1.4 (1992)(Cool-G)(FW).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `install_handler_and_trap`: Stores its own address in a global variable, raises trap zero, and returns zero.
- `decrypt_and_process_memory`: Performs XOR decryption on three separate memory blocks using a constant, then iterates through a memory range to perform XOR operations on bytes until a null byte is encountered, eventually calling a function via a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
