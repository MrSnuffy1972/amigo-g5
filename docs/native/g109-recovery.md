# Game 109 (Wizball v1.2 0176): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00109). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (375680 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 153 of 153 named (0 author symbols), 153 of 153 described (9 by rule, 143 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2481 of 2481 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 294912 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4418 of 4418 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 52028 |
| code | 19996 |
| data | 303651 |
| padding | 5 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Wizball.Slave | 1 | 182 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4230 | 131 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |
| wizpics (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2320 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Wizball_v1.2_0176/Wizball/data/wizpics: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 294912 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Wizball.Slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `setup_copper_list_and_dma`: The routine configures the Amiga custom chip registers to enable interrupts and DMA, copies a copper list from ROM to RAM, and initializes a table of display list entries.
- `wait_while_global_bit_set`: Busy-waits while bit six of a global flag word stays set, then returns.
- `clear_global_flag`: Sets a global variable to zero after calling another routine.
- `set_global_max_after_wait`: Waits for a bit in a global flag to be cleared and then sets a global variable to all bits set.
- `assign_returned_value`: Waits for a bit in a global flag to be cleared and then stores the result of another routine into a global variable.
- `set_blit_depth_and_wait`: Sets the Blitter Depth register and waits for a global flag bit to clear.
- `copy_sprite_data_to_copper`: Checks if a pointer is below a specific address threshold, then copies four words of data from that pointer to a fixed location in the custom chip registers area (likely copper list or sprite data).
- `set_interrupt_request_value`: Writes the value 32 to a global variable, which corresponds to setting a specific bit in the INTREQ register to acknowledge or trigger an interrupt request.
- `set_interrupt_request_value_high`: Writes the value 1920 to a global variable, which corresponds to setting specific bits in the INTREQ register, likely acknowledging multiple interrupt sources or configuring interrupt behavior.
- `check_flag_and_dispatch`: The routine checks if a global variable is non-negative and odd, then calls another routine and executes a function pointer from a table before returning.
- `decode_and_call_function`: The routine calls a function pointer, conditionally executes another function pointer, and then XORs a block of 148 bytes with a decrementing counter starting from 147.
- `decode_high_score_table`: XORs a sequence of 148 bytes starting at a fixed memory address with a decrementing counter from 147 down to 0, effectively decoding a block of data, possibly a high score table or text string, and returns 148.
- `set_interrupt_handler`: The routine writes a pointer to another routine into a global variable.
- `check_input_and_set_state`: The routine checks a global variable and the CIAA_PRA hardware register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state`: The routine sets up initial global variables and performs a series of calls to other routines, likely for initialization or setup purposes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `populate_color_and_data_tables`: Initializes several memory buffers and data tables by performing repetitive arithmetic operations and copying values from a source location.
- `populate_data_tables`: Initializes several memory buffers and data tables by copying values from a source location and performing repetitive arithmetic operations on global variables.
- `increment_value_by_nibbles`: Compares two words and increments the first word by specific nibble or byte values based on the differences found between the two inputs.
- `copy_memory_block`: Copies a block of memory from a source pointer to a destination pointer for a fixed number of words.
- `initialize_color_and_data_tables`: Sets a color register and performs a series of memory operations including clearing buffers, filling memory ranges with specific values, and populating data tables through nested loops.
- `initialize_sprite_attribute_tables`: Copies data from memory buffers to initialize a global variable and populates a table with calculated offsets.
- `initialize_and_process_data`: Fills memory ranges with specific values, interacts with a color register, and executes a loop that processes data structures based on values returned from other routines.
- `update_color_palette_and_tables`: Updates a global variable through a series of conditional checks and table lookups, modifies a table of values via a loop, and writes to a color register.
- `update_state_via_recursion`: Evaluates specific bits within a global variable to perform recursive calls and conditional arithmetic updates.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `execute_complex_loop`: Performs a series of assignments to a global variable and executes a loop that decrements a value based on results from other routines.
- `decrement_and_adjust_blitter`: Decrements a global variable and adjusts its value by a constant offset based on whether it falls below a specific threshold.
- `update_state_variable`: Performs complex conditional arithmetic and decrements on a global variable to determine a new value and state.
- `set_global_one`: Sets a global variable to one.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_color_and_seed`: Modifies a global variable, performs conditional memory copies, and updates a seed value while interacting with a color register.
- `process_input_table`: Iterates through a table based on a global variable, performing bitwise operations and conditional updates to a global variable and table entries.
- `process_blt_data`: Iterates through a data structure to perform calculations and writes values to a table while interacting with various hardware registers.
- `call_unresolved_target_then_halt`: The routine makes a single call to an unresolved target and then stops at a point where the decompiler reports bad instruction data, so no further behaviour is recoverable from this evidence.
- `configure_blt_hardware`: A seventeen-instruction routine that modifies several hardware registers related to bitplane attributes, modes, and sizes.
- `update_color_palette_and_registers`: Updates a series of color palette entries by performing arithmetic on values stored in a table and writing them to memory.
- `update_entity_state_table`: Iterates through a table of structures to update entity properties, including decrementing counters, clamping values, and performing conditional logic based on entity types.
- `clear_memory_structure`: If a global variable is non-zero, it iterates through a memory block to clear specific offsets and the start of each segment.
- `copy_memory_block_2`: Copies a block of data from one memory location to another using a loop that transfers multiple words per iteration.
- `copy_inverted_memory`: Copies a block of data from a source address that is bitwise inverted with a constant to a destination address.
- `toggle_pointer_sign_bit`: fetches a pointer from a global variable and loads it into a register, then flips the most significant bit of that pointer before storing the result in another register.
- `copy_memory_blocks`: Copies data from one memory location to another using a loop that performs structured block transfers.
- `execute_repeatedly_2`: Executes another routine in a loop seven times.
- `execute_repeatedly`: Calls another routine eight times within a loop.
- `shift_left_two`: Returns the input value shifted left by two bits.
- `copy_conditional_memory`: Performs a conditional check on a value and then copies a block of data from a source address that may be bitwise inverted.
- `search_and_modify_table_entry`: Iterates through a table of short integers until a negative value is encountered.
- `update_table_entry`: Iterates through a table to find an entry meeting a specific condition and then updates several fields within that table entry.
- `check_table_bounds`: Compares a byte from a table against a limit and conditionally updates a global variable.
- `modify_and_copy_data`: Increments a byte and modifies a value in a table before copying a block of data to a destination address.
- `decrement_table_values`: Searches a small table for non-zero values and decrements them twice while also decrementing a global variable.
- `copy_and_xor_data`: Increments a value in a global variable and copies blocks of data, potentially applying a bitwise XOR to the destination address.
- `clear_large_table`: Fills a large memory area with zeros using nested loops.
- `initialize_memory_loop`: Sets a global variable to a specific address and enters a loop that increments that address multiple times.
- `load_blitter_data`: Loads a series of values from a table into memory locations, using offsets derived from table entries.
- `clear_memory_range`: Sets a specific range of memory to zero through a loop.
- `zero_global_buffer`: Fills a memory area starting at a global variable with zeros.
- `update_color_palette_table`: Calculates a new index based on a global counter and uses it to fetch values from a table to update several global variables.
- `update_state_logic_and_table`: Modifies a global variable through a series of conditional branches and arithmetic operations based on its current value.
- `toggle_global_state_and_reset`: Sets a global variable to one, calls another routine, and then resets that variable to zero before setting it to several specific constant values.
- `populate_table_data`: Checks if three consecutive values in a table are negative, then toggles a global variable and populates a table with calculated values.
- `clamp_and_set_global`: Clamps a global variable within a specific range and then overwrites it with a constant value.
- `execute_repeated_routine_calls`: Saves a global variable, sets it to a specific value, and then enters a loop that calls another routine while decrementing the global variable until it reaches zero.
- `execute_loop_with_counter`: Saves a global variable, sets it to a constant, and then executes a loop that calls another routine while decrementing the value.
- `apply_bitmask_to_data`: Uses a loop to find a value, applies a bitwise XOR operation using a bit-shifted value to a memory location, and conditionally XORs another value.
- `decrement_and_recurse_2`: Decrements a global variable and calls itself recursively if the value is greater than one.
- `call_routine_multiple_times`: Calls another routine five times in succession.
- `transform_and_scatter_data`: Performs bitwise rotations on word data and writes the results into specific patterns within a destination buffer.
- `perform_bit_shift_or`: Performs a bitwise rotation and OR operation on blocks of memory using nested loops and bit-shifting logic.
- `set_memory_to_all_ones`: Stores a value consisting of all ones into a memory location pointed to by a register.
- `evaluate_bitwise_conditions`: Performs complex bitwise logic and arithmetic on a global variable and its associated data to determine a return value or update the global variable.
- `recursively_call_routine`: Performs conditional recursive calls to another routine based on the value of a global variable.
- `update_blitter_state_recursively`: Checks a global variable and, if it meets specific criteria, performs a series of arithmetic operations and recursive calls.
- `load_blitter_parameters`: Loads values from a table into specific memory locations and updates registers through multiple calls.
- `tiny_clr`: Small routine of 1 instruction that clears d0.
- `update_blt_parameters`: Calculates values based on a table lookup and updates several Blitter registers and a global variable.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `mask_high_bits`: Returns the input value after applying a bitwise AND mask to clear the lower bits.
- `execute_repeated_block`: Calls another routine and then executes a loop that runs four times.
- `initialize_and_loop_global`: Sets a global variable to zero and executes another routine three times within a decrementing loop.
- `process_memory_blocks`: Performs complex nested loops to manipulate data between memory locations using bitwise operations and block transfers.
- `update_object_positions_and_collision`: Iterates through a table of objects to update their coordinate values based on velocity and checks for proximity against a list of other entities.
- `update_state_logic`: Evaluates a value in a register to perform conditional branching, table lookups, and updates to a global variable.
- `validate_and_update_tables`: Iterates through a table to perform bitwise comparisons and updates a global pointer or sets values to maximum if conditions are not met, then processes a second table using a recursive routine.
- `scan_table_and_update`: Iterates through a table of integers and updates a global variable based on values found in another table.
- `initialize_memory_pointers`: Stores several specific memory addresses into a set of global variables.
- `copy_word_sequence`: Copies a sequence of sixteen words from a source memory location to a destination memory location.
- `copy_memory_words`: Copies a sequence of words from a source to two identical destinations.
- `duplicate_memory_block`: Copies a sequence of sixteen words from a source location to two separate destination locations.
- `duplicate_memory_block_2`: Copies a sequence of sixteen words from a source location to two separate destination locations simultaneously.
- `duplicate_memory_block_3`: Copies a sequence of words from a source location to two separate destination locations.
- `copy_and_start_copper`: Copies a sequence of words to a global variable and starts a copper list.
- `copy_memory_range`: Copies a sequence of words from one memory location to another.
- `perform_redundant_loads`: The routine performs a sequence of three identical operations that load a global variable into itself without modifying its value.
- `populate_nested_data_table`: Performs a series of nested loops to copy data from a source to a global variable.
- `execute_nested_loops`: Performs a series of nested loops that repeatedly copy data from a source to a global variable.
- `nop_placeholder`: This routine is empty and performs no operations.
- `do_nothing_2`: This routine is empty and performs no operations before returning.
- `execute_complex_state_loop`: Performs a series of initializations and enters a loop that updates a global variable using a table lookup and conditional logic.
- `update_state_loop`: Enters a loop that increments a global variable and periodically fetches a value from a table to update another global variable.
- `advance_multiplicative_seed`: multiplies the low 16 bits of a global variable by 58925 and stores the result back into that variable.
- `extract_low_nibble`: returns the low four bits of a 16-bit register value.
- `update_display_registers`: writes several constant values into hardware registers associated with display control and positioning.
- `clear_memory_buffer`: Sets a global pointer to a specific address and fills a large block of memory with zeros.
- `swap_memory_blocks`: toggles a bit in a global variable and performs two separate block swaps between memory locations.
- `store_memory_pointers`: stores a global variable and three subsequent offsets of that variable into a table of pointers.
- `wait_for_flag`: Repeatedly checks a global variable and returns when a specific bit is set.
- `loop_until_nonzero`: Repeatedly calls another routine until it returns a non-zero value.
- `halt_in_infinite_loop`: enters an unconditional infinite loop so that execution never returns.
- `wait_and_reset_flag`: waits until a global variable reaches two or more, then clears it to zero and returns.
- `configure_hardware_registers_2`: Sets several values and configures hardware registers including DMA control and interrupt enable bits.
- `configure_hardware_registers`: Configures several hardware registers including bitplane pointers, colors, and DMA control while calling routines to configure interrupts, toggle bits, and traverse a linked list tree.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `disable_dma`: writes a value to a hardware register to disable DMA channels.
- `configure_interrupt_dma`: when a global variable is non-zero, it performs bitwise operations to calculate a mask and writes it to hardware registers before clearing the global variable.
- `mask_interrupt_flags`: Writes a constant with the audio channel bits set, derives a four-bit field from it and clears matching bits in a shadow word, writing the field back when none were set.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `update_input_state`: Reads input from hardware ports and updates a global variable based on bitwise comparisons.
- `empty_routine`: The routine contains no executable instructions and performs no operations.
- `toggle_bit_eight`: Toggles the eighth bit of a global variable.
- `tiny_move`: Small routine of 1 instruction that copies 1 to d0.
- `process_string_data`: Iterates through a memory buffer, modifying bytes based on a comparison and decrementing a pointer.
- `swap_memory_blocks_2`: Performs two separate block swaps between a global variable and two different memory locations.
- `swap_multiple_memory_blocks`: Toggles a bit in a global variable and performs two separate block swaps between different memory locations.
- `decrement_global_recursively`: Decrements a global variable through recursion and performs conditional assignments.
- `decrement_and_clamp`: Decrements a global variable and clamps its value to a specific minimum.
- `decrement_and_recurse`: Reduces a global variable by increments of ten until it is less than or equal to nine, then adds the resulting value to a character constant and calls itself four times recursively.
- `clear_global_variable`: Sets a global variable to zero after executing four subroutine calls.
- `set_global_constants`: Updates a global variable with a specific offset and several constant values.
- `increment_global_value`: Increments a global variable and then executes two subroutine calls.
- `call_subroutines_sequentially`: Executes a sequence of four subroutine calls.
- `increment_global_offset`: Increments a global variable by a constant and then performs a routine call.
- `traverse_null_terminated_string`: Iterates through a null-terminated string using a pointer and sets a global variable to a constant upon completion.
- `calculate_blt_address`: Calculates a value based on an input and stores it in a global variable, then increments that variable by eight.
- `traverse_string_and_return`: Iterates through a null-terminated string using a pointer and returns the value held in a register.
- `update_blt_parameters_2`: Performs conditional logic on an input byte to calculate and store a value in a global variable.
- `execute_trap`: Executes a software trap and stores the result in a global variable.
- `initialize_and_call_routine`: Sets a global variable to several consecutive constant values and register contents before calling another routine.
- `traverse_linked_list_tree`: Traverses a complex data structure using a stack-based approach to manage nested nodes and branches.
- `set_hardware_registers`: Writes specific constant values to hardware registers.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning.
- `return_from_subroutine`: The routine consists of a single instruction that returns control to the calling routine.
- `configure_dma_and_registers`: Calculates a bitmask based on a value from a pointer, updates a global variable, and writes several values to hardware registers at an offset determined by the pointer's contents.
- `set_keyboard_serial_state`: Rewrites a global variable to itself and then sets it to a constant value.
- `process_masked_value`: Performs bitwise operations and conditional logic on a global variable and a value from a register, potentially fetching a byte from a table based on the result.
- `nop_padding_sequence`: This routine consists of a sequence of no-operation instructions used as padding within the executable segment.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `do_nothing`: The routine is empty and returns immediately without performing any operations.
- `return_immediately`: The routine is empty and returns immediately without performing any operations or modifying data.
- `do_nothing_stub`: The routine contains no instructions and returns immediately.
- `return_immediately_2`: The routine contains no instructions and returns immediately to the caller.
- `return_immediately_3`: The routine contains no instructions and returns immediately to the caller.
