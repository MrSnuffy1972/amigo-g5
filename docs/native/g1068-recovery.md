# Game 1068 (mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01068). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (935088 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 31 raw-word lines |
| Functions named and described | pass, 220 of 220 named (0 author symbols), 220 of 220 described (10 by rule, 186 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 766 of 766 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 5120 bytes, 46 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4132 of 4209 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 35992 |
| code | 13994 |
| data | 814150 |
| padding | 808 |
| unused | 70144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf:bootblock | 1 | 49 | 2 |
| logged disk load from Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf boot-block load | 1 | 116 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3219 | 170 |
| Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf bytes 1024-6144 (code never loaded by a reference run) | 1 | 825 | 46 |

## Cases handled

- mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]/Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf: custom-format (non-DOS) disk, 1621 non-empty blocks: matched against track-loaded segments
- mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]/Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf: the boot block reads 18944 bytes from disk offset 817152 to an address (found statically); recovered as a unit
- runtime segment an address: 766 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]/Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf: bytes 1024-6144 look like code no run loaded: recovered position-independently (load address unknown)
- mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]/Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf: 2 logged loads cover 24064 bytes of the image
- mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]/Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf: blocks 22-1595 (bytes 11264-817152) hold a RNC1: Rob Northen RNC1 Compressor stream (805888 -> 32000 bytes)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `touch_hardware_register` (runtime segment an address): The routine accesses a hardware register. Kind: disk/track check. Pass path: an address.
- `reset_disk_controller` (Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf bytes 1024-6144 (code never loaded by a reference run)): The routine clears a disk controller register and writes a constant to a second disk controller register, then calls a small routine to load a constant into a data register and another routine that waits for a memory flag to change, and finally calls a routine that polls status and copies blocks. Kind: disk hardware/sync check. Pass path: the branch where bVar1 is false (unaff_D4 != 0).
- `nop_routine_2` (Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf bytes 1024-6144 (code never loaded by a reference run)): The routine is empty and performs no operations. Kind: sabotage path. Pass path: sub_0CEE.

## Functions


### Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf:bootblock

- `read_boot_payload_and_run`: Allocates memory through a library call and, when that succeeds, issues disk read requests into it until the request reports no error.
- `read_payload_into_fixed_memory`: Runs only when an initial library call succeeds.

### logged disk load from Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf boot-block load

- `halve_word_when_flag_set`: Returns half of one input word when a flag word equals one, and otherwise returns the first input word unchanged.
- `mask_word_with_table_entry`: Scales an index word to a byte offset into a fixed table of words, reads the entry at that offset and returns it ANDed with a second input word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_to_global`: Stores a word from a register into a global variable.
- `evaluate_input_condition`: Returns a value based on a comparison involving a global variable.
- `update_input_flags`: Reads joystick and mouse data from hardware ports and updates a global variable based on bitwise operations.
- `calculate_bit_flags`: Performs bitwise operations on input registers to derive a new value based on specific bit patterns.
- `combine_returned_values`: Combines two values returned from other routines using a bitwise OR operation.
- `modify_memory_block`: Writes a zero to a memory location, modifies certain bits, and performs bitwise logic or or-operations with a table or global variable.
- `clear_memory_at_offset`: Stores a zero into a memory location offset from a pointer provided in a register.
- `execute_infinite_loop`: This routine enters an infinite loop that prevents further execution of the calling code.
- `loop_until_zero`: If a global variable matches a specific value, it sets that variable to zero and enters a loop that continues as long as a bitwise AND of the variable remains non-zero.
- `execute_infinite_loop_2`: Enters an infinite loop that prevents further execution of the calling code.
- `subtract_two_from_global`: Subtracts two from a global variable.
- `duplicate_global_word_lower`: duplicates the lower word of a global variable into its upper word.
- `decrement_global_variable`: decrements a global variable by two.
- `calculate_scaled_value`: Multiplies a value by a register input, performs a bit shift, and returns the result plus one.
- `calculate_scaled_fixed_point`: Multiplies a value by one thousand and performs a bit shift to return a scaled result.
- `swap_memory_range`: Swaps a range of memory words between a global variable and a fixed address before entering an infinite loop.
- `update_structure_contents`: Modifies a bit-masked global variable and writes a bit-shifted value and several constants into a memory structure.
- `update_structure_and_loop`: Calculates a bit-shifted value and stores it into a structure, then enters an infinite loop.
- `extract_table_data`: Copies several words from a table into a global variable and other memory locations, including an absolute value and a bit-shifted value.
- `compute_complex_offsets`: Calculates several derived values using arithmetic and bit shifts, then stores them into a structure at various offsets.
- `return_from_subroutine`: The routine performs a single stack operation to return control to a calling function.
- `wait_for_bit`: Polls a specific bit in a global variable until it is cleared.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_global_bit`: Sets a specific bit in a global variable to a non-zero value.
- `mask_global_variable_bits`: Clears a specific bit in a global variable using a bitwise AND operation and then sets the variable to zero.
- `multiply_and_check_sign`: Multiplies a global variable by a value from a register and enters a loop that checks if the result is zero or negative.
- `check_global_status`: Checks if a global variable is negative and performs bitwise operations to return specific constant values.
- `repeat_routine_until_zero`: Repeatedly calls a routine and checks its return value, continuing the loop as long as the return value is non-zero.
- `modify_and_check_global`: Multiplies a global variable by a constant and loops while checking certain bit conditions or return values.
- `initialize_and_loop_on_flag`: Clears a memory block, copies a block of memory, and then enters a loop that continues until a specific flag or zero flag is cleared.
- `clear_memory_block`: Fills a block of memory starting at a pointer with zeroed words until a certain count is reached.
- `copy_memory_block`: Performs a nested loop to copy data from a source pointer to a destination pointer.
- `clear_global_variable`: Clears a global variable and then sets a specific bit within it.
- `copy_data_loop`: Repeatedly calls another routine to store bytes into memory locations based on a loop condition and a bit test in a global variable.
- `execute_empty_routine`: The routine is empty and performs no operations before returning.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `set_conditional_bit`: Checks if a specific bit in a global variable is clear, then sets a different bit in that same variable.
- `update_global_conditional`: If a specific bit in a global variable is not set, it sets another bit in that variable and updates it with a value from a register.
- `process_global_state_and_table`: Checks or modifies bits in a global variable and performs conditional pointer-based table lookups or updates to a series of memory locations.
- `configure_bitplane_hardware`: Copies a block of memory, modifies several hardware registers, and writes structured data to memory locations including bitplane and DMA related parameters.
- `fill_memory_with_register`: Fills a block of memory locations with a value held in a register and waits for a flag to clear.
- `process_input_and_audio_dma`: Updates display window and DMA settings, manipulates flags in a global variable and a byte pointed to by a pointer, and conditionally calls another routine while updating audio-related custom registers.
- `mask_multiple_bits`: Clears two specific bits in a global variable using bitwise AND operations.
- `initialize_resource_table_2`: Copies a source table to a global destination and checks a magic signature to perform pointer relocation and offset adjustments.
- `set_dma_enable_flag`: Performs several routine calls and then sets a global variable to a constant value.
- `modify_global_bitmask_2`: Clears a certain bit in a global variable and, if a condition is met, stores a bit-shifted value into a global or sets a global to zero.
- `mask_and_shift_global`: Clears specific bits in a global variable and, if a condition is met, stores a bit-shifted value into a global variable.
- `clear_global_variable_2`: Waits for a global variable to become zero and then sets another global variable to zero.
- `set_global_flag_bit`: Calls another routine and then sets a specific bit in a global variable.
- `set_global_flags_and_process_audio`: The routine stores two input words into a global variable, conditionally calls another routine and writes a byte derived from the global variable to memory, then sets bits in the global variable.
- `poll_four_slots_return_pair_2`: Iterates through a global table and calls a routine for each entry that is not a sentinel value, returning a pair of values.
- `return_registers`: Returns the current contents of two registers as a single concatenated value.
- `initialize_resource_table`: Copies a source table to a global destination, then checks a magic signature to perform pointer relocation and offset adjustments on a secondary table.
- `update_sprite_position_and_dma_2`: Iterates through a table of sprite or object pointers to calculate new horizontal or vertical coordinate offsets and updates a global pointer or table with the new position or DMA-related values.
- `initialize_dma_state_2`: Clears a memory location, sets a global variable to a specific value, and writes a value to a DMA controller register.
- `reset_dma_config_2`: Clears a memory location, sets a global variable to a different value, and writes a value to a DMA controller register.
- `reset_dma_and_globals_2`: Clears a memory location, sets a global variable to a third value, and writes a value to a DMA controller register.
- `reset_audio_state_parameters_2`: Clears a global variable and sets a global variable to a constant value and another global variable to a constant value.
- `update_sprite_attribute_and_position`: Calculates new attribute or position values for a sprite or object, updating a table with new coordinates and flags, and modifies DMA or audio hardware registers based on the calculated offset.
- `set_global_and_or_byte`: Stores a constant into a global variable and returns the input byte ORed with a constant.
- `update_sprite_position_and_dma`: Calculates new coordinate offsets and updates a table of pointers or or-masked values, while modifying DMA or audio registers.
- `initialize_dma_state`: Clears a memory location, sets a global variable to a specific value, and interacts with the DMA controller register.
- `reset_dma_config`: Clears a memory location, sets a global variable to a specific value, and writes to the DMA controller register.
- `reset_dma_and_globals`: Clears a global variable, sets a global variable to a new value, and modifies the DMA controller register.
- `reset_audio_state_parameters`: Clears a global variable and sets two other global variables to specific constant values.
- `copy_table_and_set_flags`: Copies 64 bytes from a global variable to a table, then sets a global variable based on several sentinel checks and finally stores a constant to a global variable.
- `return_input_register`: Returns the value currently held in a register.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 158 into d4.
- `poll_four_slots_return_pair`: Iterates through four consecutive entries in a global table, calling a routine for each entry that is not a sentinel value, and returns a register pair with a high-bit tag.
- `store_register_values`: Checks a global variable and, if not zero, calls another routine and stores a series of registers or return values into a sequence of memory locations.
- `update_sprite_pointer_table`: Iterates through a table of structures to update sprite or object pointer addresses and visibility flags.
- `update_sprite_dimensions_and_position`: Iterates through a table of sprite or object structures to modify or clamp their width, height, or horizontal position or visibility flags.
- `update_sprite_attribute_bounds`: Updates or clamps sprite or object position and size or visibility flags based on a type-specific instruction or or value in a table.
- `iterate_and_update_pointer_table`: Iterates through a table of structures, decrementing counters and updating pointer or address-related values based on conditional checks or table-driven offsets.
- `update_pointer_tables`: Updates several pointers or memory locations with values derived from a table, and conditionally updates other memory locations based on a flag.
- `update_audio_dma_registers`: Copies words from a global variable to the audio DMA registers and updates several minimum byte values in RAM.
- `empty_routine`: the routine is empty and performs no operations.
- `reset_global_variables`: Negates a value through a pointer, sets a global variable to a constant, and clears several other global variables.
- `conditional_ram_update`: Checks a RAM flag and, if set, clears several RAM locations and stores an input word into one of them before returning.
- `process_data_stream`: Checks status flags in a global variable and incrementally copies data from a table into a destination buffer.
- `copy_memory_block_unrolled`: Copies a block of memory from a source address to a destination address using an unrolled loop that processes multiple words or longwords at a time.
- `expand_bitmask_table`: Uses a bitmask to selectively copy words from a source table into a destination table across multiple iterations.
- `expand_bitmask_table_to_destination`: Iterates through a table of pointers, using a bitmask to selectively copy words from a source table into a destination buffer.
- `fill_sixteen_words`: fills a block of sixteen words starting at a specific memory location with a value provided in a register.
- `fill_sixteen_words_2`: Fills a block of sixteen words starting at a specific memory location with a value provided in a register.
- `wait_for_flag_clear`: Calls another routine and loops until a certain flag or zero flag is cleared.
- `empty_routine_2`: the routine is empty and performs no operations before returning.
- `wait_and_set_global_flag`: Waits for a bit in a global variable to be cleared, then stores three values to a memory location and sets that bit in the global variable.
- `no_op_on_global`: Loads a global variable back into itself.
- `calculate_and_loop`: Waits for a bit in a global variable to be cleared, performs a multiplication using a register and the global variable, and enters a loop that decrements a value until it reaches a specific threshold.
- `interpolate_color_values`: Decrements a global counter and performs a complex bitwise interpolation between two sets of values to update a destination table.
- `check_bit_and_halt_2`: Checks if a specific bit in a global variable is set and enters an infinite loop if it is not.
- `check_bit_and_halt`: Checks if a specific bit is set in a global variable; if not, it returns, otherwise it enters an infinite loop.
- `copy_word_to_memory`: Copies a word from a source pointer to a destination pointer.
- `search_for_value`: Iterates through a table of integers to check if a value matches a certain entry.
- `tiny_addq_moveq_rts`: Small routine of 3 instructions that adds 4 to a7, then loads 255 into d0, then returns.
- `write_interleaved_data_3`: Iterates through a loop to write a sequence of incrementing or modified values to a memory location.
- `write_interleaved_data_2`: Writes a word and a zeroed word to a memory location pointed to by a register.
- `write_interleaved_data`: Writes a series of words to a memory location by interleaving values from two registers and incrementing a base value.
- `compare_and_align_memory`: Performs a memory block copy and alignment operation while checking for a specific four-byte sequence.
- `return_immediately_2`: This routine is an empty stub that immediately returns to the caller without performing any operations.
- `return_immediately_stub`: The routine consists of a single return instruction and performs no work.
- `modify_global_bitmask`: Performs bitwise masking on a global variable and enters a loop that executes until a certain condition or register value is met.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `fetch_table_pointer`: Modifies a value based on specific matches and uses it to fetch a pointer from a table.
- `map_input_values`: Modifies a register value based on specific matches and then uses that value to fetch a pointer from a table.
- `update_counter_and_pointer`: Checks if a register matches a specific address, then stores a value from another register into a memory location pointed to by a register and clears the following word.
- `recursively_increment_counter`: Recursively calls itself and increments a value at a memory offset until a certain condition or register value is met.
- `call_self`: The routine calls itself recursively.
- `reset_disk_controller_3`: Clears a disk controller register, writes a constant to a second disk controller register, and loads a constant.
- `reset_disk_controller_2`: Clears a disk controller register and writes a constant to a second disk controller register, then calls a small routine to load a constant into a data register and another routine that waits for a memory flag to change, and finally calls a routine that polls status and copies blocks.
- `tiny_moveq`: Small routine of 1 instruction that loads 135 into d1.
- `write_constant_to_memory_2`: Calls another routine, then writes a constant byte to a memory location, writes a byte from a register to the same location, and writes the constant again.
- `touch_hardware_register`: The routine accesses a hardware register.
- `toggle_bits_and_wait`: Clears a bit in a memory location based on a value from another routine, performs a delay, and then enters a loop that waits for a specific bit in a memory location to change.
- `toggle_bits_with_delay`: Clears certain bits in a memory location, sets another bit, and then performs a fixed-count delay loop.
- `toggle_bits_and_delay`: Toggles two bits in a memory location, waits for a condition to be met, toggles a bit, and then performs a fixed-count delay loop.
- `wait_for_memory_change_3`: The routine loops indefinitely while a byte at an offset from a base pointer remains equal to itself.
- `poll_input_and_adjust_value_2`: Conditionally calls another routine and waits while a byte at an offset from a pointer has a certain bit set, then stores a code value, shifts and masks a byte, and enters a loop that adjusts two byte values until they match.
- `process_interleaved_data`: Iterates through several tables to perform bitwise operations and XORing, storing the results in a destination table while updating a global value.
- `tiny_move`: Small routine of 1 instruction that copies (a5)+ to d1.
- `compute_masked_sum_and_double`: masks an input register and a pointer-dereferenced value with a global mask, then returns the sum of the masked input and twice the masked pointer value.
- `xor_sum_table_elements`: The routine iterates through a table to compute the cumulative XOR sum of its elements and returns a concatenated result.
- `copy_interleaved_data`: Iterates through several tables to perform bitwise operations and XORing, storing the results in a destination table while updating a global.
- `tiny_ori_3`: Small routine of 1 instruction that merges 2 into d3.
- `process_bit_interleaved_data`: The routine iterates through three tables to perform bitwise operations and XORing, storing the results in a destination table while updating a global variable.
- `copy_and_align_memory_block`: This routine performs a memory block copy and alignment operation, ensuring destination addresses are word-aligned before transferring data.
- `generate_bit_mask_from_table`: Iterates through a table of words until a condition involving a global value and a table entry is met, then calculates a bit mask by or-ing a bit into a value based on a byte-offset or a calculated bit position.
- `mask_and_conditional_call`: Masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `return_empty_routine`: The routine is empty; it contains no instructions that modify memory or registers and immediately returns.
- `concatenate_memory_bytes`: The routine reads three consecutive bytes from a pointer and concatenates them into a single value.
- `generate_bitmask_table`: Iterates through a list to calculate and store bitmask values and related metadata into a table.
- `execute_conditional_logic_loop`: Performs a sequence of assignments and conditional checks on a global variable, including a loop that executes subroutines based on the state of that variable.
- `toggle_global_state`: Saves a global word, sets it to zero, executes several subroutines with alternating values of one and zero, and restores the original word.
- `decrement_conditional_global`: Decrements a global variable if its bit 5 is not set.
- `swap_bytes_if_positive`: Loads a value from a table using a scaled index, then checks if that value is positive to perform a byte swap or otherwise assigns a constant to a global variable.
- `update_status_byte`: Evaluates a value against constants and writes a modified byte with the high bit set to a specific memory location.
- `update_global_state`: Performs arithmetic on a value, modifies a global variable using bitwise OR, and resets a global variable to a constant value.
- `reset_global_variable`: Sets a global variable to zero, calls another routine, and then sets the same global variable to one.
- `initialize_complex_system_state`: Performs a large sequence of memory swaps, pointer initializations, and conditional loops that modify or clear various global variables and memory blocks.
- `set_global_zero_and_execute`: Sets a global variable to zero and executes a sequence of conditional subroutines if that variable is zero.
- `set_global_zero`: sets a global variable to zero after a sequence of bitwise or assignments and a recursive call to itself.
- `execute_conditional_subroutines`: Calls several subroutines and, if a global variable is zero, executes a sequence of additional subroutines.
- `update_global_word`: A seven-instruction routine that updates a global variable based on a shifted global byte and a constant or clears it.
- `reset_global_to_zero`: Calls another routine, then sets a global variable to itself and subsequently resets it to zero.
- `evaluate_conditional_logic`: Evaluates a global variable to return a constant, a fixed value, or the negation of another routine, or returns a value from another routine or a constant based on a global variable.
- `process_conditional_global_value`: Checks a global variable to decide whether to return a constant, return the negation of another routine, or return a fixed value.
- `get_conditional_value`: Checks a global variable and returns either a value from another routine or a constant.
- `return_immediately_3`: Performs a single return instruction to exit the current function.
- `return_from_subroutine_3`: This routine performs a single return instruction to exit the current function.
- `increment_decrement_value_towards_target`: Increments or decrements a 16-bit global variable by one or clamps it to a target value if it does not already equal that target.
- `clamp_value_towards_target`: Adjusts a 16-bit value at an offset from a pointer by one unit towards a second 16-bit value at a different offset.
- `fetch_table_value`: Fetches a word from a table using an index derived from a returned value and stores it in a global variable.
- `return_clamped_byte`: Returns a global byte value, or returns two if the value is unable to be clamped otherwise.
- `load_value_from_table`: Uses a returned index to fetch a longword from a table and stores it in a global variable.
- `zero_multiple_globals`: Sets a global variable to zero multiple times.
- `clear_pointer_list`: Increments a global variable and then iterates through a list of pointers, writing zero to the memory locations they point to until a null pointer is encountered.
- `fill_array_with_zeros`: Writes zero to six consecutive 32-bit words starting at a pointer.
- `clear_memory_via_null_terminator`: Iterates through a list of pointers and writes zero to the memory locations they point to until a null pointer isencountered.
- `evaluate_nested_conditional_assignments`: Performs a series of recursive conditional checks and assigns a constant or a decremented constant to a global variable based on the return status of those checks.
- `set_conditional_global`: Performs conditional checks and assigns a constant value to a global variable.
- `execute_recursive_conditional_assignment`: Calls another routine and, if the return value is true and a specific sequence of global variable assignments evaluates to zero, sets a global variable to a constant value.
- `initialize_complex_data_structures`: Performs a loop to initialize several memory locations and linked list elements with specific constant values.
- `process_conditional_list_elements`: Iterates through a list of pointers and calls two different routines if a specific bit is set in a value at an offset from the pointed-to address.
- `empty_stub_2`: This routine is empty and performs no operations other than returning.
- `verify_magic_constant`: checks if a sequence of bytes in memory matches a specific constant value.
- `copy_and_align_memory_block_2`: Performs a memory block copy and alignment operation, or executes a byte-by-byte copy if certain conditions are met.
- `reset_global_variable_2`: Sets a global variable to zero.
- `unnamed_routine_8460`: An oridinary routine consisting of eighteen instructions that calls another routine.

### Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf bytes 1024-6144 (code never loaded by a reference run)

- `relocate_code_block_and_start`: Calls routines and checks a result, sets a fixed global value and copies a block of longs from the start of the region to a fixed destination.
- `setup_display_and_clear_memory`: Clears a block of memory beyond a fixed address, then programs the display window, bitplane, fetch and copper registers and enables DMA, calling routines along the way.
- `prompt_for_disk_insert`: The decompiled body is only a return, while the listed facts give 27 instructions, two calls and a text reference to inserting disk 1 into the first drive.
- `combine_status_bits_to_byte`: Returns a byte built from bits of a global status byte: one bit is inverted into the result, and when another bit is clear the value two is ORed in.
- `poll_status_then_set_constant`: Sets a global to a fixed value, then calls a routine repeatedly until its returned value has its low two bits clear.
- `clear_thirty_two_words`: Writes zero to 32 consecutive words in a global block.
- `request_memory_block`: Requests a memory block through the memory allocation library stub and returns that stub's result unchanged.
- `mask_first_result_high_bits`: Calls one routine, then a second routine, and returns the first result masked with a fixed bit mask that clears the low address bits.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `decode_run_length_into_plane`: Decodes run-length encoded data from a source buffer held in a global into a fixed destination region.
- `increment_counter_set_constant`: Increments a global counter and sets another global variable to a fixed constant value.
- `spin_forever`: Loops forever without doing any work, so the processor stalls.
- `build_copper_list_entries`: Calls one routine, then fills a copper list in memory with pairs of display register writes built from fixed constants, writing six entries per row for a set of rows.
- `copy_counted_word_block`: Reads a count word from a source pointer and copies that many 16-bit words, starting from the words that follow it, to the destination pointer.
- `process_zero_terminated_string`: Walks a zero-terminated byte string, calling a routine for each character and advancing to the pointer that routine returns, until the terminating zero is reached.
- `copy_structured_data`: If the input word is non-negative, the routine computes a source address from the input and a constant base, then copies six 40-byte blocks from that source to a destination pointer, advancing both pointers by 40 each iteration.
- `rotate_nibbles_and_emit_five`: Shifts an input value eight places up, then five times rotates it by four bits and calls a routine with each rotated value.
- `call_then_return_global_pointer`: Calls one routine and then returns the value of a global code pointer to the caller.
- `map_input_to_pointer`: The routine modifies an input word based on specific constant values, uses it to index into a table to retrieve a pointer, and then loops until that pointer is non-zero.
- `return_immediately`: This routine performs no operations and returns immediately to its caller.
- `store_word_if_changed_global`: Compares an input word with a global variable and, when they differ, stores the input word through a pointer and calls a routine.
- `process_loop_with_counter`: The routine executes a loop that increments a value at a memory offset and calls another routine once a specific count is reached.
- `execute_empty_call`: This routine performs a tail call to another routine and then returns.
- `call_reset_disk_controller`: The routine calls another routine to reset the disk controller.
- `reset_disk_controller`: The routine clears a disk controller register and writes a constant to a second disk controller register, then calls a small routine to load a constant into a data register and another routine that waits for a memory flag to change, and finally calls a routine that polls status and copies blocks.
- `write_byte_to_pointer`: The routine writes a byte to a location pointed to by a register.
- `write_constant_to_memory`: Calls another routine, then writes a constant byte to a memory location pointed to by a register, writes a byte from a register to the same location, and writes the constant again.
- `nop_routine_2`: The routine is empty and performs no operations.
- `wait_for_memory_flag_change`: The routine calls another routine, writes a masked byte to a memory location pointed to by a register and writes a constant byte to the same location, then checks a byte at an offset from that pointer.
- `toggle_memory_bits`: Clears two specific bits in a byte at a pointer, calls another routine, and then sets a bit in that same byte before entering a loop.
- `wait_for_memory_change_2`: The routine loops indefinitely while a byte at an offset from a base pointer remains equal to itself, effectively waiting for an external change.
- `wait_for_memory_change`: The routine loops indefinitely while a byte at an offset from a base pointer remains equal to itself, effectively waiting for an external change or acting as an infinite loop if the value is constant.
- `poll_input_and_adjust_value`: The routine conditionally calls another routine and waits while a byte at an offset from a pointer has a bit set, then stores a code value and reads a byte from another routine.
- `process_input_and_xor_data`: The routine polls a hardware status byte in a loop and, when cleared, processes input data by calculating masks and XORing memory blocks, updating global variables and pointers in a loop.
- `calculate_input_mask_sum`: The routine computes a value by masking an input register and a pointer value with a global mask, doubling the pointer result, and adding them together.
- `calculate_input_mask_sum_2`: Computes a value by masking an input register and a pointer value with a global mask, doubling the pointer result, and adding them together.
- `xor_memory_block_and_mask`: The routine iterates through a block of memory, XORing each word into an accumulator, then combines the final accumulator with a global mask and an input register to form a 64-bit return value.
- `copy_audio_buffer_blocks`: Iterates through a table of pointers to copy blocks of data from source addresses to a destination buffer, resetting offsets after each block transfer.
- `tiny_ori_4`: Small routine of 1 instruction that merges 0 into d7.
- `subtract_two_from_input`: Returns the input value with two subtracted from it.
- `unpack_rnc_compressed_block_2`: Checks the four-byte header for the RNC compression tag and, if present, allocates a buffer, rebuilds the decoding table from the packed data and decompresses the block for the caller.
- `parse_bit_table`: Scans a table of words for a matching condition, then calculates a bit mask based on byte values and register contents.
- `mask_and_conditional_call_2`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `return_empty_routine_4`: The routine is empty and contains no instructions that modify memory or registers before returning.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `build_huffman_tree_nodes`: The routine iterates over a buffer to match bytes against a running value, writing masks, shifted values, and node properties like matched bytes and lengths into a destination array to build a tree structure.
