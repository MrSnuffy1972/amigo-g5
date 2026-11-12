# Game 1068 (mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01068). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (935088 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 42 raw-word lines |
| Functions named and described | pass, 361 of 361 named (0 author symbols), 361 of 361 described (14 by rule, 323 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 766 of 766 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 5120 bytes, 46 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5987 of 6123 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 32513 |
| code | 19936 |
| data | 811685 |
| padding | 810 |
| unused | 70144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf:bootblock | 1 | 49 | 2 |
| logged disk load from Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf boot-block load | 1 | 116 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5133 | 311 |
| Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf bytes 1024-6144 (code never loaded by a reference run) | 1 | 825 | 46 |

## Cases handled

- mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]/Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf: custom-format (non-DOS) disk, 1621 non-empty blocks: matched against track-loaded segments
- mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]/Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf: the boot block reads 18944 bytes from disk offset 817152 to an address (found statically); recovered as a unit
- runtime segment an address: 766 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]/Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf: bytes 1024-6144 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate
- mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]/Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf: 2 logged loads cover 24064 bytes of the image
- mortal kombat ii (1994)(acclaim)[cr psg](disk 1 of 4)[a2]/Mortal Kombat II (1994)(Acclaim)[cr PSG](Disk 1 of 4)[a2].adf: blocks 22-1595 (bytes 11264-817152) hold a RNC1: Rob Northen RNC1 Compressor stream (805888 -> 32000 bytes)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `touch_hardware_register` (runtime segment an address): The routine accesses a hardware register. Kind: disk/track check. Pass path: an address.
- `clear_cia_register_bit` (runtime segment an address): Calculates an address based on a value from a6 and a word from a global variable, clears the longword at that address, and writes a byte to a CIA register. Kind: disk hardware/register manipulation. Pass path: the routine sets a specific bit in the CIA register (likely to clear an interrupt or status flag) based on a value returned from a hardware check function `u2__sub_5F8A`..
- `compute_cia_drive_control_mask` (runtime segment an address): Builds a control byte from the current global value by clearing one bit selected by an index, additionally clearing a second bit when a per-index table flag is set, and returns it. Kind: disk hardware/track check. Pass path: the routine returns a bitmask used to control CIA (Complex Interface Adapter) chips, likely to mask/unmask specific drive control lines or interrupt bits based on the physical properties of the disk (e.g., track length or sync patterns). A .
- `pulse_drive_control_bits` (runtime segment an address): Repeats a count-controlled sequence: each step reads a control byte from a helper, optionally clears a bit based on a sign flag, forces the low bit set in the drive control port, and calls a helper. Kind: disk hardware/timing control. Pass path: the loop continues to the next iteration (in_D0b != 0).
- `reset_and_toggle_port_bit` (runtime segment an address): Sets a value in the custom hardware register, then checks a signed word at a negative offset from a6; if negative, it clears a bit in a port register and immediately sets the same bit back using a bit index derived from a value at a negative offset from a6. Kind: disk hardware timing/sync check. Pass path: the branch where the if-condition is false (skipping the CIAB_PRB bit manipulation).
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
- `modify_d1_and_branch`: Clears bit 0 of d1, moves a word from a memory location pointed to by a0 into d3, tests bit 0 of d2, performs arithmetic on d0, and branches to another routine.
- `execute_infinite_loop`: This routine enters an infinite loop that prevents further execution of the calling code.
- `loop_until_zero`: If a global variable matches a specific value, it sets that variable to zero and enters a loop that continues as long as a bitwise AND of the variable remains non-zero.
- `execute_infinite_loop_2`: Enters an infinite loop that prevents further execution of the calling code.
- `subtract_two_from_global`: Subtracts two from a global variable.
- `duplicate_global_word_lower`: duplicates the lower word of a global variable into its upper word.
- `decrement_global_variable`: decrements a global variable by two.
- `calculate_scaled_value`: Multiplies a value by a register input, performs a bit shift, and returns the result plus one.
- `calculate_scaled_fixed_point`: Multiplies a value by one thousand and performs a bit shift to return a scaled result.
- `store_pointers_to_globals`: Loads two long pointers into a0 and a1, then stores these pointers into four different global word locations.
- `initialize_data_tables`: Populates two separate memory areas with specific long words and zeroed words using a pointer initialized from a global address.
- `swap_memory_range`: Swaps a range of memory words between a global variable and a fixed address before entering an infinite loop.
- `update_structure_contents`: Modifies a bit-masked global variable and writes a bit-shifted value and several constants into a memory structure.
- `update_structure_and_loop`: Calculates a bit-shifted value and stores it into a structure, then enters an infinite loop.
- `extract_table_data`: Copies several words from a table into a global variable and other memory locations, including an absolute value and a bit-shifted value.
- `compute_complex_offsets`: Calculates several derived values using arithmetic and bit shifts, then stores them into a structure at various offsets.
- `return_from_subroutine`: The routine performs a single stack operation to return control to a calling function.
- `wait_for_bit`: Polls a specific bit in a global variable until it is cleared.
- `perform_nested_copy_2`: Performs a triple-nested loop to copy words from a source pointed to by a0 to a destination pointed to by a1, using counts derived from three values.
- `perform_nested_copy`: Performs a triple-nested loop to copy words from a source pointed to by a0 to a destination pointed to by a1, using counts derived from three global words.
- `swap_and_modify_pointers`: Swaps the values of two global pointers and then sets bit 3 of a global word.
- `copy_structured_data_2`: Calculates offsets using multiplication to perform a nested loop that copies words from one memory area to another and clears a word at a calculated destination.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_data_block`: Copies eight 32-bit words from a source table to a destination address pointed to by a register.
- `copy_data_blocks`: Copies two consecutive 32-byte blocks of data from a source table to a destination structure pointed to by a register.
- `increment_pointers`: Increments a0 and a pointer retrieved from a table if the value at that pointer is not zero, then branches back to the start of the routine.
- `compare_and_copy_byte`: Compares a byte at a pointer in a0 to a constant and to a value in d0; if they match, it copies the value from d0 to a pointer in a2 and returns.
- `toggle_and_clear_byte`: Toggles bit 0 of a byte at an offset of 1 from a pointer in a1, calls another routine, and returns.
- `clear_byte_array`: Clears ten consecutive bytes in memory starting from a location relative to the program counter.
- `set_global_bit`: Sets a specific bit in a global variable to a non-zero value.
- `mask_global_variable_bits`: Clears a specific bit in a global variable using a bitwise AND operation and then sets the variable to zero.
- `check_status_and_return`: Stores d0 in a global word, calls two other routines, tests a global byte and a bitmask in a global word, and returns either 0 or 1 based on these conditions.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `multiply_and_check_bounds`: Multiplies a word from a global variable by d0, stores the result in a global variable, calls execute_infinite_loop and check_global_status, and returns 0 or 1 based on the value of d0 and a bitwise check of a byte from a global variable.
- `multiply_and_check_sign`: Multiplies a global variable by a value from a register and enters a loop that checks if the result is zero or negative.
- `check_global_status`: Checks if a global variable is negative and performs bitwise operations to return specific constant values.
- `repeat_routine_until_zero`: Repeatedly calls a routine and checks its return value, continuing the loop as long as the return value is non-zero.
- `modify_and_check_global`: Multiplies a global variable by a constant and loops while checking certain bit conditions or return values.
- `initialize_and_loop_on_flag`: Clears a memory block, copies a block of memory, and then enters a loop that continues until a specific flag or zero flag is cleared.
- `clear_memory_block`: Fills a block of memory starting at a pointer with zeroed words until a certain count is reached.
- `copy_memory_block`: Performs a nested loop to copy data from a source pointer to a destination pointer.
- `clear_global_variable`: Clears a global variable and then sets a specific bit within it.
- `clear_global_bit`: Clears bit 1 of a global variable and returns.
- `copy_data_loop`: Repeatedly calls another routine to store bytes into memory locations based on a loop condition and a bit test in a global variable.
- `execute_empty_routine`: The routine is empty and performs no operations before returning.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning.
- `set_conditional_bit`: Checks if a specific bit in a global variable is clear, then sets a different bit in that same variable.
- `update_global_conditional_2`: Loads a pointer into a0, then calls another routine via a branch.
- `update_global_conditional`: If a specific bit in a global variable is not set, it sets another bit in that variable and updates it with a value from a register.
- `process_global_state_and_table`: Checks or modifies bits in a global variable and performs conditional pointer-based table lookups or updates to a series of memory locations.
- `write_hardware_registers`: Clears bit 6 of a global variable and writes two words from a table to the hardware registers at offsets 142 and 144 of the custom chip base address.
- `configure_bitplane_hardware`: Copies a block of memory, modifies several hardware registers, and writes structured data to memory locations including bitplane and DMA related parameters.
- `fill_memory_with_register`: Fills a block of memory locations with a value held in a register and waits for a flag to clear.
- `initialize_array_offsets`: Clears a word, loads two pointers, and calls another routine twice with different counts and pointer pairs.
- `add_offset_to_array`: Adds a longword value from a register to each longword in a source list and stores the results in a destination list using a loop controlled by a counter in a register.
- `process_input_and_audio_dma`: Updates display window and DMA settings, manipulates flags in a global variable and a byte pointed to by a pointer, and conditionally calls another routine while updating audio-related custom registers.
- `set_interrupt_request_seven`: Writes 7 to the interrupt request register and returns from interrupt.
- `set_interrupt_request_high`: Writes 1952 to the interrupt request register and returns from interrupt.
- `set_interrupt_request_mid`: Writes 6144 to the interrupt request register and returns from interrupt.
- `set_interrupt_request_low`: Writes 8192 to the interrupt request register and returns from interrupt.
- `mask_multiple_bits`: Clears two specific bits in a global variable using bitwise AND operations.
- `initialize_resource_table_2`: Copies a source table to a global destination and checks a magic signature to perform pointer relocation and offset adjustments.
- `set_dma_enable_flag`: Performs several routine calls and then sets a global variable to a constant value.
- `modify_global_bitmask_2`: Clears a certain bit in a global variable and, if a condition is met, stores a bit-shifted value into a global or sets a global to zero.
- `mask_and_shift_global`: Clears specific bits in a global variable and, if a condition is met, stores a bit-shifted value into a global variable.
- `clear_global_variable_2`: Waits for a global variable to become zero and then sets another global variable to zero.
- `set_global_flag_bit`: Calls another routine and then sets a specific bit in a global variable.
- `set_global_flags_and_process_audio`: The routine stores two input words into a global variable, conditionally calls another routine and writes a byte derived from the global variable to memory, then sets bits in the global variable.
- `modify_byte_via_pointer`: Reads a byte from the address pointed to by a0 and performs bitwise OR operations on various registers before branching to another routine.
- `poll_four_slots_return_pair_2`: Iterates through a global table and calls a routine for each entry that is not a sentinel value, returning a pair of values.
- `return_registers`: Returns the current contents of two registers as a single concatenated value.
- `initialize_resource_table`: Copies a source table to a global destination, then checks a magic signature to perform pointer relocation and offset adjustments on a secondary table.
- `jump_to_update_sprite_position_and_dma`: Jump stub: passes control straight on to update_sprite_position_and_dma.
- `branch_to_handler`: Performs a short branch to another location.
- `initialize_dma_state_2`: Clears a memory location, sets a global variable to a specific value, and writes a value to a DMA controller register.
- `reset_dma_config_2`: Clears a memory location, sets a global variable to a different value, and writes a value to a DMA controller register.
- `reset_dma_and_globals_2`: Clears a memory location, sets a global variable to a third value, and writes a value to a DMA controller register.
- `reset_audio_state_parameters_2`: Clears a global variable and sets a global variable to a constant value and another global variable to a constant value.
- `branch_to_offset`: Performs a relative branch to a specific offset.
- `set_audio_volume_2`: Stores a word into an audio volume register at an address calculated by adding a shifted value from d1 to the register base address.
- `set_audio_period`: Stores a word into an audio period register at an address calculated by adding a shifted value from d1 to the register base address.
- `populate_pointer_table`: Copies a series of longwords from various memory locations into a destination table pointed to by a2, then calls another routine and sets bit 1 of the CIAA_PRA register before returning.
- `store_byte_to_table`: If d1 is 3 and a value at a table address is zero, it stores the byte from d0 into a table at an offset determined by d1.
- `update_sprite_attribute_and_position`: Calculates new attribute or position values for a sprite or object, updating a table with new coordinates and flags, and modifies DMA or audio hardware registers based on the calculated offset.
- `update_sprite_position_and_dma`: Calculates new coordinate offsets and updates a table of pointers or or-masked values, while modifying DMA or audio registers.
- `set_dma_one_and_globals`: Clears a 16-bit global variable, sets the DMA controller register to one, and writes two hundred fifty-five to another global variable.
- `reset_dma_config`: Clears a memory location, sets a global variable to a specific value, and writes to the DMA controller register.
- `reset_dma_and_globals`: Clears a global variable, sets a global variable to a new value, and modifies the DMA controller register.
- `reset_audio_state_parameters`: Clears a global variable and sets two other global variables to specific constant values.
- `set_audio_volume`: Calculates an offset by shifting the value in d1 left by 4 and adding it to the address of the audio volume register, then stores the word from d0 at that location.
- `initialize_audio_and_load_data`: Sets a value in the audio period register, calls copy_table_and_set_flags, copies a block of 64 words from a source to a destination, performs a series of memory clears on four tables, and populates a structure with offsets and values derived from a data block.
- `copy_table_and_set_flags`: Copies 64 bytes from a global variable to a table, then sets a global variable based on several sentinel checks and finally stores a constant to a global variable.
- `store_register_values`: Checks a global variable and, if not zero, calls another routine and stores a series of registers or return values into a sequence of memory locations.
- `update_sprite_pointer_table`: Iterates through a table of structures to update sprite or object pointer addresses and visibility flags.
- `iterate_and_update_pointer_table`: Iterates through a table of structures, decrementing counters and updating pointer or address-related values based on conditional checks or table-driven offsets.
- `update_pointer_tables`: Updates several pointers or memory locations with values derived from a table, and conditionally updates other memory locations based on a flag.
- `update_byte_and_branch`: Stores a byte from offset 2 of a pointer in a5 into offset 15 of the same pointer, then branches to another routine.
- `resolve_pointer_table_entry`: Calculates an offset from a byte in a structure, scales it, and adds it to a base address to populate several pointer fields within a structure.
- `merge_nibbles_in_structure`: Reads a byte from offset 2 of a structure, then reads a byte from offset 22.
- `merge_nibbles_in_structure_2`: Reads a byte from offset 2 of a structure, extracts its lower nibble, and merges it into the lower nibble of a byte at offset 24 of the same structure; if the lower nibble is zero, it returns; otherwise, it checks the upper nibble of the original byte and merges it into the upper nibble of the byte at offset 24 if that upper nibble is non-zero.
- `dispatch_input_command_table`: Reads a byte from a structure, masks it, and uses the result to jump to a specific address in a jump table.
- `jump_to_copy_table_and_set_flags`: Jump stub: passes control straight on to copy_table_and_set_flags.
- `update_status_byte_2`: Checks a byte from a structure at offset 2 of a5, and if it is not zero, increments it and stores the result into offsets 9, 11, and 12 of a3 before branching.
- `update_sprite_pointer_table_2`: Calculates sprite pointer addresses by performing bitwise operations and table lookups based on values in a structure, then updates the sprite pointer table and a secondary pointer table via tail branches.
- `update_audio_dma_registers`: Copies words from a global variable to the audio DMA registers and updates several minimum byte values in RAM.
- `empty_routine`: the routine is empty and performs no operations.
- `write_word_and_bytes_to_memory`: Writes a 16-bit word from a register to a memory location and then writes four consecutive bytes of value 255 to the following memory addresses.
- `wait_for_non_zero_word`: Polls a 16-bit global variable in a loop until its value is zero, then returns.
- `configure_hardware_registers`: Initializes a structure in memory with several 16-bit values, sets a byte in another memory location, and writes values to the AUD3VOL, DMACON, and INTENA hardware registers.
- `conditional_ram_update`: Checks a RAM flag and, if set, clears several RAM locations and stores an input word into one of them before returning.
- `process_data_stream`: Checks status flags in a global variable and incrementally copies data from a table into a destination buffer.
- `clear_audio_buffer`: Clears a memory range using a loop, updates a global pointer if it does not match a specific address, and writes a value to a hardware register.
- `update_intena_register`: Copies a 32-bit value from a memory location to a register and then writes a 16-bit value to the INTENA hardware register.
- `update_audio_buffer_and_dma`: Checks an interrupt request bit and, if set, manages an audio buffer by updating the DMA address and length registers.
- `configure_hardware_registers_2`: Loads a pointer from a memory location, sets several hardware registers including DMA and interrupt registers, and branches to another routine.
- `copy_memory_block_unrolled_2`: Copies a block of data from a source pointer in a2 to a destination pointer in a1 using an unrolled sequence of longword moves, where the number of copied words is determined by a value in d0.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `copy_memory_block_unrolled`: Copies a block of memory from a source address to a destination address using an unrolled loop that processes multiple words or longwords at a time.
- `expand_bitmask_table`: Uses a bitmask to selectively copy words from a source table into a destination table across multiple iterations.
- `expand_bitmask_table_to_destination`: Iterates through a table of pointers, using a bitmask to selectively copy words from a source table into a destination buffer.
- `expand_bitmask_and_fill`: Calls a routine to expand a bitmask, then calls another routine to fill sixteen words, and finally branches to another routine.
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
- `clear_memory_and_call`: Calls another routine and then clears a word at a specific memory location.
- `execute_interleaved_writes`: Loads two pointers from memory, calls two different routines, and then branches to another routine.
- `initialize_data_structures`: Initializes eight entries in a table by writing a constant longword, clearing two words, and setting a word to 29, then copies data from a source list into the table using a loop.
- `store_data_to_table`: If a global variable is less than 8, it calculates an offset into a table, stores a longword, clears a word, and writes several words from registers and a pointer to the table, then increments the global variable.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `copy_and_offset_tables`: Calls another routine, then copies words from one table to another using a count derived from a table at offset 3306, and finally copies longwords from a third table to a fourth table after applying a subtraction and addition offset to each value.
- `sort_pointer_table_2`: Calls another routine, then enters a loop that compares values in a table of pointers and calls another routine if a specific byte is negative.
- `update_buffer_with_logic`: Calculates a value based on a global variable and register contents, writes several bytes to a destination buffer, and performs a loop to copy data from a source pointer.
- `copy_and_terminate_buffer`: Copies a block of data from a source pointer to a destination pointer using a loop, then writes a word and clears a longword at the end of the destination.
- `compare_and_loop_buffer`: Checks bit 0 of d1 and branches; otherwise it enters a loop that compares a word at offset 4 of a structure pointed to by a0 with d2, and compares a word at offset 4 of a structure pointed to by a4 with d2, using a dbge loop on d1.
- `process_buffer_data`: Loads a pointer from a structure, calls another routine, calculates a value by subtracting a global variable from a register, and then calls a third routine before branching.
- `write_data_to_offset_table`: Calculates an offset by multiplying d0 by 14, writes d1 and d2 to that offset in a table, and optionally writes a longword from a0 if d3 is not zero.
- `update_complex_buffer_structure`: Calculates an offset into a table based on a register, then writes a four-byte header containing a value, a bit-shifted value, a sum, and a status byte to a destination buffer.
- `process_complex_buffer_update`: Updates a destination buffer with bytes derived from registers, copies a block of data, and if a bit in a global variable is set, writes to a table and copies another block of data.
- `sort_pointer_table`: Loads a count from a global variable and uses it to loop through a table of pointers, swapping pointers in the table if the word at offset 6 of the pointed-to structure is smaller than the previous one.
- `decrement_and_return`: Sets a loop counter using a value from a constant and then returns.
- `store_a6_to_address`: Stores the contents of register a6 into a memory location at offset 51e.
- `calculate_skip_offset`: Calculates a value based on bits 8 and 9 of a word at an offset of 2 from a0, uses this to update a counter in d7, and branches based on a sign check of a word at an offset of minus 2 from the incremented a2 pointer.
- `clear_and_calculate_offset`: Clears a memory range using a loop based on d7, then calls another routine and calculates a new address in a1 by adding scaled values from a0 to the original pointer.
- `shift_register_right`: Returns the value of register d7 shifted right by 2 bits.
- `process_nested_loops`: Calculates an offset into a table using d0 and d1, then executes two nested loops that call another routine and modify a pointer and a counter.
- `load_and_jump_bytes`: Checks bits 3, 4, and 5 of d0 to set a global variable and select a jump target, then loads four bytes from a buffer at offset 7 of a6 into d2, d3, d4, and d5 before jumping to the selected target.
- `load_bytes_from_table`: Loads four bytes from a table using offsets provided by registers d2, d3, d4, and d5 into those same registers.
- `write_multiple_bytes`: Writes four bytes from registers to a destination buffer using offsets, increments the destination pointer, and runs a loop.
- `copy_data_blocks_2`: Performs a nested loop to copy 16-bit words from a source buffer to a destination buffer with specific stride offsets calculated from registers d1 and d2.
- `duplicate_pointers_to_table`: Copies the 32-bit values from registers a0, a1, and a2 into a table at a specific location, with each value being written twice consecutively.
- `copy_word_to_memory`: Copies a word from a source pointer to a destination pointer.
- `search_for_value`: Iterates through a table of integers to check if a value matches a certain entry.
- `tiny_addq_moveq_rts`: Small routine of 3 instructions that adds 4 to a7, then loads 255 into d0, then returns.
- `write_interleaved_data_3`: Iterates through a loop to write a sequence of incrementing or modified values to a memory location.
- `write_interleaved_data_2`: Writes a word and a zeroed word to a memory location pointed to by a register.
- `write_interleaved_data`: Writes a series of words to a memory location by interleaving values from two registers and incrementing a base value.
- `write_interleaved_data_4`: Calls a routine to search for a value, then writes a series of 16-bit words to a destination buffer where the high and low bytes are interleaved using values from a table and a global variable.
- `initialize_buffer_values`: Calls a routine to search for a value, then writes a 16-bit constant and a zero byte to a destination buffer pointed to by a pointer in a structure.
- `store_scaled_product`: Negates d0, calls another routine, and then stores the resulting word, longword, and word into a structure at offsets 0, 2, and 6.
- `calculate_scaled_product`: Calculates a value by multiplying two bytes from a table at an offset derived from register d0, then scales the result by 32 and 47.
- `parse_and_execute_data_stream`: Iterates through a data stream, performing conditional arithmetic on a value and calling clamp_and_increment_value or decrement_byte_at_offset_one based on specific byte markers, then returns.
- `clamp_and_increment_value`: Compares d4 to a constant and clamps it if it is larger, then enters a loop that increments d5 and subtracts a value from d4 until d4 is no longer negative.
- `process_buffer_loop`: Adds a longword from a buffer to d4, performs a logical shift left on d5, runs a loop, performs a bitwise OR between d4 and d5, stores the result in d4, and restores registers before returning.
- `decrement_byte_at_offset_one`: Checks bit 3 of a register and, if set, subtracts a byte value from the byte located at offset 1 of a pointer in a0, then performs a bitwise AND on the register and branches.
- `store_three_bytes`: Stores the bytes from registers d1, d2, and d3 into three consecutive memory locations starting at a fixed address.
- `copy_and_pad_string`: Copies a sequence of bytes from a source buffer to a destination buffer using a length in d7, then pads the remaining space in the destination with spaces if specific bits are set in d0.
- `format_hex_string`: Converts a 32-bit value into a decimal-like string representation by iterating through nibbles and writing characters to a destination buffer.
- `load_three_bytes`: Loads three consecutive bytes from a source buffer into registers d1, d2, and d3.
- `decode_scaled_data_stream_2`: Parses a data stream by checking for specific control bytes, performing scaled arithmetic using values from tables, and updating a global variable with the calculated results.
- `decode_scaled_data_stream`: Reads a byte from a data stream and uses it as an index into a table to calculate a value, which is then multiplied by a scale factor derived from the data stream and added to a running total.
- `update_offset_and_check_status`: Adds a scaled value derived from a 16-bit word in a global variable to another global variable, increments a byte in a register, and compares it against a byte at offset 7 of a structure in a1.
- `parse_nested_data_structure`: Iterates through a multi-level data structure, clearing bytes in a nested loop and updating a global pointer based on values read from the structure.
- `increment_pointer_and_branch`: Increments a longword in a register by one, adds a word from another register to that same register, and performs a short branch to another routine.
- `branch_on_byte_value`: Compares a byte in register d0 to 255 and 254, branching to different locations if either match, otherwise it branches to another routine.
- `increment_counter_and_update_global`: Increments a value in a global variable by a calculated amount based on a value from a pointer, then increments a byte in a structure at offset 7 and checks if it matches a target value; if the target value is reached and a byte at the address in a2 is 255, it resets the counter to zero, otherwise it decrements the counter.
- `calculate_offset_and_pointer`: Calculates a memory offset by multiplying a byte from a buffer by a value from a table and adding a constant, then stores the result in a global variable and returns a pointer to a new location in a3.
- `calculate_and_fetch_table_index`: Reads a byte from a data stream, performs arithmetic to calculate a scale factor based on the difference between two global values, and uses the result to fetch a longword from a table using a shifted index.
- `find_string_length`: Iterates through a string in a buffer to find the position of a null terminator and returns a calculated value based on its offset.
- `process_data_stream_with_conditional_flag`: Checks a bit in a global variable and compares a byte at a negative offset from a pointer to 32; if both conditions are met, it calls another routine and updates the global variable bit based on the result's sign.
- `clear_bit_and_branch`: Clears bit 0 of a word at a memory location and then performs a relative branch.
- `update_bitmask_buffer`: Calculates a target address in a buffer and performs a series of bitwise operations to update bytes based on four source bytes and a calculated mask.
- `dispatch_table_entry`: Calculates an index by negating and shifting a word from d0, uses it to fetch a pointer from a table in a0, and calls the address in a0 if a bit in d1 is set.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `process_returned_index`: Calls another routine, loads a byte from a table using the returned d0 into d1, clears bit 7 of d1, and if bit 0 of d2 is set, it clears bit 7 of a table entry and either decrements d0 or loads a new value from a0 and decrements it before returning.
- `update_table_entry`: If bit 1 of d2 is set, it clears bit 7 of a byte in a table, increments d0, and if d0 is less than or equal to a byte in a0, it clears d0 and sets bit 7 of a different entry in the table, otherwise it negates d0 and calls another routine.
- `modify_index_byte`: Loads a byte from a table using an index in d0 into d1, masks the lower 7 bits of d1, and if bit 2 of d2 is clear, decrements d1 and either clears d1 or sets bit 7 of d1 before returning.
- `update_status_byte_3`: Checks bit 3 of d2, increments d1, compares d1 to a byte at an offset of a2, and if the condition is met, sets bit 7 of a modified d1 and stores it in a table at a1 plus d0, then calls two other routines.
- `clear_global_byte`: Saves d0 through d2 to the stack, clears a byte at a global variable address, and restores the registers before returning.
- `execute_data_stream_commands`: Loads a byte from a pointer, calls parse_buffer_bytes, and then enters a loop that calls another routine using a table lookup based on a byte from the buffer, finally calling another routine if a bit in a global variable is clear.
- `jump_to_parse_buffer_bytes`: Performs a jump to another routine.
- `process_data_stream_segments_2`: Calls another routine to load data from a pointer, then uses a value from a register to either call a buffer loop or iterate through a series of null-terminated strings located at an offset from a pointer.
- `jump_to_format_numeric_string_and_update`: Performs a jump to another routine.
- `parse_scaled_data`: Tests bit 4 of a byte at a global address, and if clear, loads two bytes from a pointer in a2 into d2 and d3, decrements d2, and calls another routine with a pointer in a0.
- `parse_buffer_bytes`: Increments a pointer in a0, loads two bytes from the incremented pointer into d2 and d3, calls another routine, and then moves the value from a2 into a0 before returning.
- `process_data_stream_segments`: Loads bytes from a pointer, calls another routine, and then uses a value from d4 to either call loop_buffer_bytes or perform a loop that checks bytes at a pointer until a null byte is found.
- `loop_buffer_bytes`: Calls another routine, loads a pointer from the stack into a0, loads a byte from the stack into d0, decrements d0, and then enters a loop that checks if the byte at the pointer in a0 is non-zero, decrementing d0 on each iteration until it returns.
- `format_numeric_string_and_update_buffer`: Calculates a value by adding a byte from a buffer to a byte at an offset, converts this value into an ASCII decimal string, and calls another routine twice to update a bitmask buffer with the resulting characters.
- `compare_and_align_memory`: Performs a memory block copy and alignment operation while checking for a specific four-byte sequence.
- `validate_table_index`: Checks if a value in d0, masked with 127, matches a word at a memory location pointed to by a6 plus an offset, and if the value at that location is non-negative, it increments the pointer in a1 and branches back to the start.
- `prepare_color_loop`: Sets d0 to 30 and d1 to 240, then branches to another routine.
- `reallocate_table_data`: Manipulates a value from a pointer in a1 to update a global variable, then calls another routine and potentially performs a byte-by-byte comparison and memory copy if certain conditions are met.
- `verify_data_magic_and_reallocate`: Sets bit 3 of a global variable, checks a bit in a value popped from the stack, and if certain conditions are met, performs a byte-by-byte comparison of a three-byte sequence against a constant before calling copy_and_align_memory_block.
- `return_immediately_2`: This routine is an empty stub that immediately returns to the caller without performing any operations.
- `return_immediately_stub`: The routine consists of a single return instruction and performs no work.
- `modify_global_bitmask`: Performs bitwise masking on a global variable and enters a loop that executes until a certain condition or register value is met.
- `toggle_global_variable`: Executes a loop that calls an initialization routine, checks if a value in d1 matches a global variable, and then toggles that global variable and potentially clears it based on its value.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d2, then returns.
- `initialize_and_call_subroutine`: Clears a byte at a memory location pointed to by a0, loads a word from a global variable into d0, and calls another routine.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_color_registers`: Calculates a value from d0, then performs a loop that calls another routine four times to update the first color register with different bit-shifted components of a value in d2.
- `loop_color_and_memory_updates`: Sets the first color register to a value from a register, calls execute_infinite_loop_2, writes zero to a memory location pointed to by a6, calls execute_infinite_loop_2 again, and repeats this process based on a loop counter in d6 before calling execute_infinite_loop_2 one final time.
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
- `process_bit_interleaved_data`: The routine iterates through three tables to perform bitwise operations and XORing, storing the results in a destination table while updating a global variable.
- `process_scaled_table_entry`: Calculates an address from a value at an offset of negative 34 from a6 and a scaled index, then performs a bitwise right shift on the 16-bit word at that address.
- `pulse_drive_control`: Checks bit 4 of the CIAA_PRA register, and if set, calls another routine to perform a loop that runs d1 times, where d1 is initialized to 255 and decremented.
- `clear_cia_register_bit`: Calculates an address based on a value from a6 and a word from a global variable, clears the longword at that address, and writes a byte to a CIA register.
- `compute_cia_drive_control_mask`: Builds a control byte from the current global value by clearing one bit selected by an index, additionally clearing a second bit when a per-index table flag is set, and returns it.
- `pulse_drive_control_bits`: Repeats a count-controlled sequence: each step reads a control byte from a helper, optionally clears a bit based on a sign flag, forces the low bit set in the drive control port, and calls a helper.
- `reset_and_toggle_port_bit`: Sets a value in the custom hardware register, then checks a signed word at a negative offset from a6; if negative, it clears a bit in a port register and immediately sets the same bit back using a bit index derived from a value at a negative offset from a6.
- `wait_for_flag_and_result`: It repeatedly sets a CIA timer until the routine returns one.
- `set_ciab_timer`: It writes three constant values to a global variable in sequence.
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
- `update_clamped_value`: Checks a global word, calls another routine to check status, and if conditions are met, increments a global variable and uses a clamped value to index into a table.
- `clamp_and_update_value`: Decrements a global word, calls a clamping routine, and then uses the result to index into a table to fetch a value which is stored in another global variable if it is larger than the current value.
- `fetch_scaled_table_value`: Calls return_clamped_byte, doubles the result, and uses it as an offset to fetch a word from a table in a0.
- `clamp_global_variable_range`: Adds a 16-bit value from a register to a global variable, then clamps the resulting value between 0 and 9 before storing it back to the global variable.
- `load_value_from_table`: Uses a returned index to fetch a longword from a table and stores it in a global variable.
- `check_status_flags`: Checks two word-sized global variables and returns 0, 1, or 2 based on whether they are non-zero.
- `increment_and_clamp_value`: Increments a word in a global variable and resets it to 0 if the result is 9 or greater.
- `zero_multiple_globals`: Sets a global variable to zero multiple times.
- `clear_pointer_list`: Increments a global variable and then iterates through a list of pointers, writing zero to the memory locations they point to until a null pointer is encountered.
- `fill_array_with_zeros`: Writes zero to six consecutive 32-bit words starting at a pointer.
- `clear_memory_via_null_terminator`: Iterates through a list of pointers and writes zero to the memory locations they point to until a null pointer isencountered.
- `evaluate_nested_conditional_assignments`: Performs a series of recursive conditional checks and assigns a constant or a decremented constant to a global variable based on the return status of those checks.
- `set_conditional_global`: Performs conditional checks and assigns a constant value to a global variable.
- `execute_recursive_conditional_assignment`: Calls another routine and, if the return value is true and a specific sequence of global variable assignments evaluates to zero, sets a global variable to a constant value.
- `initialize_complex_data_structures`: Performs a loop to initialize several memory locations and linked list elements with specific constant values.
- `process_conditional_list_elements`: Iterates through a list of pointers and calls two different routines if a specific bit is set in a value at an offset from the pointed-to address.
- `clear_flag_on_condition`: Iterates through a table of pointers and, if bit 0 of an offset 16 byte is set and an offset 54 byte equals 20, clears bit 0 of that same offset 16 byte.
- `load_data_and_loop`: Pushes a0 to the stack, loads a word from a table into d0, loads three values into a2, a3, and a pointer, checks a bit in a value from a3, and enters a loop that decrements d0.
- `pop_and_return_zero`: Calls another routine, pops a longword from the stack into a0, clears d0, and returns.
- `update_structure_and_traverse_list`: Updates a pointer in a structure, clears an offset 52 word, and then traverses a linked list of words to clear specific offsets in each node.
- `empty_stub_2`: This routine is empty and performs no operations other than returning.
- `load_table_pointers`: Loads a word from a table into a0, calls another routine, and then loads a different word from the table into a0.
- `update_byte_with_flag`: Checks if a byte at a pointer in a0 is negative; if so, it fetches a value from a table using a pointer and an index, sets bit 7, and writes it to a0, or alternatively sets bit 7 of a decremented value from a global variable into a0.
- `dispatch_indirect_call`: Checks if a global variable is equal to one; if not, it calculates an offset from a word in a global variable, loads a word from a table, and performs an indirect jump to an address loaded from a table.
- `verify_data_integrity`: Performs several calls to another routine to verify magic constants and then enters a loop that stores values into a table based on results from another routine.
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
