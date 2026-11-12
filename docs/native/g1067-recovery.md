# Game 1067 (MortalKombat2 v1.0 1MB 1105): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01067). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (6648772 top-level bytes in 50 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 34 of 34 units identical (bytes and relocations), 41 raw-word lines |
| Functions named and described | pass, 370 of 370 named (0 author symbols), 370 of 370 described (41 by rule, 329 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1603 of 1604 (99.94%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5833 of 5997 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 359482 |
| code | 20272 |
| data | 6223896 |
| padding | 66 |
| unused | 45056 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 292 | 14 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.2 | 1 | 0 | 0 |
| logged disk load from Disk.3 | 1 | 0 | 0 |
| logged disk load from Disk.3 | 1 | 0 | 0 |
| logged disk load from Disk.3 | 1 | 0 | 0 |
| logged disk load from Disk.3 | 1 | 0 | 0 |
| logged disk load from Disk.3 | 1 | 0 | 0 |
| logged disk load from Disk.3 | 1 | 0 | 0 |
| logged disk load from Disk.3 | 1 | 0 | 0 |
| logged disk load from Disk.3 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4984 | 287 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 380 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 55 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 257 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.1: raw disk image (970752 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.2: raw disk image (970752 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.3: raw disk image (970752 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105.hdf: hard-disk image (AmigaDOS OFS, 7104 blocks): its filesystem parses and all 11 files in it are byte-identical to the extracted tree MortalKombat2_v1.0_1MB_1105_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 25 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 1091 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 218 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 5 first-run image unit(s) in 1 version group(s); 402 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.1: 16 logged loads cover 151040 bytes of the image
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.1: chunks 446-1057 (bytes 114176-270848) hold a RNC1: Rob Northen RNC1 Compressor stream (156672 -> 32000 bytes)
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.1: chunks 1202-3697 (bytes 307712-946688) hold a RNC1: Rob Northen RNC1 Compressor stream (638976 -> 7506 bytes)
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.1: chunks 3720-3721 (bytes 952320-952832) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.1: chunks 3744-3745 (bytes 958464-958976) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.1: chunks 3768-3769 (bytes 964608-965120) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.2: 1 logged loads cover 512 bytes of the image
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.2: chunks 2-3707 (bytes 512-949248) hold 15 packed streams (C: Compact, RNC1: Rob Northen RNC1 Compressor) found by scanning, 99% of the bytes
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.2: chunks 3720-3721 (bytes 952320-952832) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.2: chunks 3744-3745 (bytes 958464-958976) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.2: chunks 3768-3769 (bytes 964608-965120) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.3: 8 logged loads cover 171008 bytes of the image
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.3: chunks 2-1233 (bytes 512-315904) hold a RNC1: Rob Northen RNC1 Compressor stream (315392 -> 32000 bytes)
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.3: chunks 1886-3189 (bytes 482816-816640) hold 10 packed streams (RNC1: Rob Northen RNC1 Compressor) found by scanning, 98% of the bytes
- MortalKombat2_v1.0_1MB_1105/MortalKombat2_v1.0_1MB_1105_hdf/MortalKombat21MB/Disk.3: chunks 3204-3781 (bytes 820224-968192) hold a RNC1: Rob Northen RNC1 Compressor stream (147968 -> 11520 bytes)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `execute_chained_pointers`: Stores a pointer from a register into a table, calls another routine using a pointer from that table, updates two global variables, and if a comparison matches, performs a sequence of three calls through a table before jumping to an address in a structure.
- `execute_chained_calls`: Calls a routine using a pointer from a table, then performs a series of calculations involving d0 and d1, calls another routine, and finally calls a third routine using a pointer from a5 before jumping to an offset of a5.
- `process_complex_loop`: Performs a loop that updates a value in a destination pointer by adding a source value and a boolean flag, writes a byte to a specific offset in a structure, and updates a 16-bit value in another structure.
- `increment_and_wrap_global`: Increments a 16-bit global variable and resets it to zero if it reaches three.
- `calculate_and_call`: If d1 is not zero, it exchanges d0 and d1, exchanges d1 and d2, performs arithmetic on d0 and d1, increments d2, and calls a routine using a pointer from a2.
- `set_bit_conditionally`: Compares d0 to a byte at a relative offset; if they are equal, it branches to another location, otherwise it sets bit 6 of a byte at an offset from a0 and returns.
- `decode_compressed_chunk`: Validates a data header signature and processes a compressed data stream using a stack-based algorithm.
- `parse_bit_table`: Scans a table of words for a matching condition, then calculates a bit mask based on byte values and register contents.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `perform_arithmetic_on_registers`: Performs a series of arithmetic and bitwise operations involving registers d1, d6, and d7, and accesses two bytes from a table using a pointer in a3.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `build_huffman_tree_nodes`: Iterates over a buffer to match bytes against a running value, writing masks, shifted values, and node properties into a destination array to build a tree structure.
- `branch_to_subroutine`: Pushes the current stack pointer onto the stack, performs a relative branch to a different location, and then restores the stack pointer by adding four to it before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_subsystems`: Disables interrupts and sets hardware registers, initializes a stack frame, and populates a table of function pointers.
- `copy_bytes_to_memory`: Copies twelve bytes from a source location to a destination starting at a fixed memory address using a loop.
- `clear_memory_range`: Clears a range of 32-bit words in memory starting at the address in a0 until the address reaches the value in d0.
- `update_hardware_and_status`: Manipulates a global variable using bits from a hardware port, performs bitwise operations on input data, and calls several routines including one that takes a pointer to a structure and another that performs a calculation and stores the result in a global variable.
- `evaluate_input_condition`: Calls another routine, then checks two global variables to return 2, 1, or 0 based on their values.
- `update_input_flags`: Reads joystick and mouse data from hardware ports and updates a global variable based on bitwise operations.
- `calculate_bit_flags`: Performs bitwise operations on input registers to derive a new value based on specific bit patterns.
- `combine_returned_values`: Calls three different routines and combines the results of the second and third calls using a bitwise OR operation.
- `update_structure_flags`: Copies two words from a structure in a1 to offsets 4 and 6 of the same structure, clears the word at a1, and then performs bitwise operations to update the structure based on values in a0, a2, and d3.
- `zero_memory_offset`: Writes zero to a 16-bit word at an offset of four from the address held in a1.
- `modify_registers_and_branch`: Clears register d1, loads a word from a table into d3, performs bit tests and arithmetic on d0, and branches to another routine.
- `execute_infinite_loop`: Compares a byte from a table with itself and, if they are equal, enters an infinite loop.
- `loop_until_zero`: If a global variable matches a specific value, it sets that variable to zero, calls another routine, and enters a loop that continues as long as a bitwise AND of three global variables remains non-zero.
- `loop_and_call_routine`: Calls another routine within a loop that decrements d0, returning when d0 reaches zero.
- `subtract_two_from_global`: Subtracts two from a global variable.
- `duplicate_global_word_lower`: Duplicates the lower word of a global variable into its upper word by swapping the bits of a value from a global variable and storing the result at a table location.
- `call_and_test_bit`: Calls another routine and then tests bit 0 of d0 before returning.
- `calculate_scaled_value`: Multiplies a value by a register input, performs a bit shift, and returns the result plus one.
- `calculate_scaled_fixed_point`: Multiplies a value by one thousand and performs a bit shift to return a scaled result.
- `assign_constant_values`: Stores two specific constant values into a global variable sequentially.
- `swap_memory_blocks`: Swaps eleven words between two memory locations using a loop, then writes a value to the custom chip registers and enters an infinite loop.
- `prepare_branch_target`: Loads a long address into a0, pushes a long address onto the stack, and branches to an earlier instruction.
- `initialize_and_branch`: Clears bit 4 of a global variable, calls another routine, updates several values in a structure, performs multiple calls to other routines, and branches to another routine.
- `extract_table_data`: Calls another routine, copies several words from a table into a global variable and other memory locations, including an absolute value and a bit-shifted value.
- `compute_complex_offsets`: Calculates several derived values using arithmetic and bit shifts, then stores them into a structure at various offsets.
- `modify_registers_conditionally`: Sets d2 to a constant and d3 to a constant, then adds 18 to d3 if bit 7 of a global variable is not set.
- `load_addresses_and_call`: Loads two long addresses into a0 and calls another routine twice before branching to another routine.
- `clear_buffer_with_multiplication`: Calculates a value using three global variables and then enters a loop that clears a buffer pointed to by a0 until a counter reaches zero.
- `wait_for_bit`: Polls bit 14 of a register at a fixed offset from a base address until it is cleared, then returns.
- `perform_nested_copy_2`: Loads two pointers from global variables and performs a triple-nested loop to copy words from the source pointer to the destination pointer.
- `perform_nested_copy`: Executes a triple-nested loop to copy words from a source pointer to a destination pointer based on three values loaded from global variables.
- `swap_and_modify_pointers`: Swaps the values of two global pointers and then sets bit 3 of a global word.
- `copy_multi_dimensional_data`: Calculates offsets using multiplication and performs a nested loop to copy words from a source to a destination, inserting a zero at the end of each inner loop sequence.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_data_block`: Copies eight 32-bit words from a source table to a destination address pointed to by a register.
- `copy_data_blocks`: Copies two consecutive 32-byte blocks of data from a source table to a destination structure pointed to by a register.
- `increment_pointer_and_byte`: Increments a pointer in a0 and a byte at a location relative to the program counter, then loops back to the start of the routine.
- `compare_and_store_byte`: Compares a byte at a pointer in a0 to a value from a global word; if they are equal and the byte is not all ones, it stores that value into a location pointed to by a2 and returns.
- `toggle_and_call_routine`: Toggles the second bit of a byte at an offset of 1 from a pointer in a6, calls another routine, and returns with negative one.
- `clear_memory_block_2`: Clears a sequence of ten bytes by writing zero to each byte starting from the routine's own location.
- `set_status_bits`: Clears a byte at a global variable and sets bit 5 of another global variable.
- `mask_global_variable_bits`: Clears bit 5 of a global variable, sets a byte at offset 9a9 to zero, and then performs a multiplication of a value from a global variable, stores the result in another global variable, calls two other routines, and returns either zero or one based on the sign and value of the result.
- `check_status_and_return`: Stores a word in a global variable, calls two other routines, and checks various flags and bitmasks to decide whether to return zero or one.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `multiply_and_check_bounds`: Multiplies a word from a global variable by d0, stores the result in a global variable, calls execute_infinite_loop and check_global_status, and returns 0 or 1 based on the value of d0 and a bitwise check of a byte from a global variable.
- `multiply_and_check_sign`: Multiplies a global variable by a value from a global variable, stores the result in another global variable, calls two other routines, and returns zero if the result is negative or if a branch is taken, otherwise it returns zero.
- `check_global_status`: Checks if a global variable is negative and performs bitwise operations to return specific constant values.
- `multiply_and_branch`: Calls another routine, then calculates a value by multiplying a constant by a value from a global variable before branching.
- `branch_to_offset_3`: Calls another routine, sets d0 to 3, and then branches to another location.
- `prepare_buffer_operations_2`: Saves a pointer to the stack, calls two subroutines, loads a pointer from a global variable into a1, calls another subroutine, restores the pointer to a0, sets d0 to negative one, and branches to another routine.
- `prepare_buffer_operations`: Loads a pointer from a global variable into a0, calls two subroutines, loads another pointer from a global variable into a0, and branches to another routine.
- `copy_nested_blocks`: Performs a triple nested loop to copy data from one memory location to another using several counters.
- `clear_and_set_bits`: Clears a byte at a global variable and sets bit 1 of another global variable.
- `clear_status_bit`: Clears bit 1 of a global variable and returns.
- `fill_data_buffers`: Fills a destination buffer by calling another routine in a loop, then checks a bit in a global variable to decide whether to fill a second buffer using a different routine in another loop.
- `call_and_rotate`: Calls another routine and then rotates the bits of d1 by eight positions.
- `copy_data_block_word`: Performs bitwise operations and shifts on d1 and d2 using values from d4, d5, d6, and d7.
- `branch_with_address_load`: Pushes a0 onto the stack, loads a relative address into a0, and branches to another routine.
- `update_global_pointer`: Moves the address of a global variable into a0 and branches to another routine.
- `update_global_conditional`: If bit 7 of a global variable is not set, it sets bit 6 of that variable and updates the variable with the value from a register.
- `process_state_machine`: Checks bit 7 of a global variable and, if not set, performs complex updates to a structure and a global variable based on values from a table.
- `write_hardware_registers`: Clears bit 6 of a global variable and writes two words from a table to the hardware registers at offsets 142 and 144 of the custom chip base address.
- `configure_hardware_and_data`: Copies 16 words from one table to another, updates several hardware registers, calculates two 16-bit values based on d3 and stores them in global variables, writes a series of words to a memory location derived from a global variable, and clears 256 bytes starting at that global variable.
- `setup_hardware_registers`: Calls two other routines, loads a series of values from a table into hardware registers and other global variables, calls another routine, and then jumps to another routine with a pointer to a table and a value of negative one in d0.
- `update_list_parameters`: Loads a word from a global variable into d0, clears d0, and sets up a0 and a1 with relative addresses before calling another routine with d1 set to 13.
- `add_and_store_list`: Adds a long value in d0 to each long value in a source list pointed to by a0 and stores the results in a destination list pointed to by a1, looping for the count in d1.
- `check_hardware_bits`: Saves registers to the stack, loads the address of a hardware register into a0, loads a memory address into a1, reads a byte from an offset of 768 from the address in a0, and branches based on the status of bit 0 and bit 3 of that byte.
- `test_joystick_fire_bits`: Reads the status of the first and fourth bits from the CIAA Port A Data Register and branches to different locations based on their values; otherwise it returns.
- `invert_and_store_bit_rotated_byte`: Reads a byte from an offset of 1536 in a structure, inverts its bits, performs a single-bit rotate right, stores the result at an offset of 1 from a pointer in a1, calls another routine, sets a byte at offset 1280 of the structure to 1, and branches to another location.
- `clear_register_bit_and_copy_byte`: Clears bit 6 in a memory location relative to a pointer in a0 and copies a byte from an offset of 1 of a pointer in a1 to the location pointed to by a1.
- `process_input_and_audio_dma`: Checks a bit in the interrupt status register to update a global counter and flags, then manipulates DMA control and copper registers based on input status.
- `set_interrupt_mask_seven`: Writes 7 to the interrupt mask register and returns from an interrupt.
- `set_interrupt_mask_high`: Writes 1920 to the interrupt mask register and returns from an interrupt.
- `set_interrupt_mask_very_high`: Writes 6144 to the interrupt mask register and returns from an interrupt.
- `set_interrupt_mask_eight_thousand`: Writes 8192 to the interrupt mask register.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `set_bit_and_branch`: Loads two long addresses into a0 and a2, calls another routine, and sets bit 5 of a global variable.
- `mask_multiple_bits`: Clears bit 2 of a global variable, clears bit 6 of another global variable, and tests a longword in a global variable before returning.
- `branch_on_mask`: Calls a subroutine and otherwise returns or branches to another location.
- `set_dma_control_register`: Calls five subroutines and then writes the value fifteen to the DMACON register.
- `modify_global_word`: Calls a subroutine, and if a condition is not met, loads a word from a global variable, shifts it left, performs a bitwise OR with three, and writes the result to a global variable.
- `update_global_word`: Calls another routine, and if it does not branch, loads a word from a global variable, shifts it left by 8 bits, performs a bitwise OR with 2, and stores the result in a longword global variable.
- `clear_global_variable`: Calls another routine, and if a word at a global variable is not zero, it clears a word in another global variable and returns.
- `set_global_flag`: Calls a subroutine and sets bit 1 of a global variable.
- `set_global_flags_and_process_audio`: The routine stores two input words into a global variable, conditionally calls another routine and writes a byte derived from the global variable to memory, then sets bits in the global variable.
- `modify_byte_via_pointer`: Performs bitwise OR operations on d0 and d3, then stores a byte derived from d0 and a byte from a memory location pointed to by a0 into a global table.
- `branch_to_address`: Performs a relative branch to another address.
- `branch_to_sprite_attribute_updater`: Performs a short branch to the middle of the process_sprite_attribute_table routine.
- `branch_twice`: Performs two consecutive relative branches to different locations.
- `jump_to_update_sprite_and_audio_parame`: Jump stub: passes control straight on to update_sprite_and_audio_parameters_2.
- `branch_to_offset_2`: Performs a relative branch to another location.
- `branch_to_offset_4`: Branches to a specific offset.
- `branch_to_offset_198c`: Performs a short branch to the instruction at offset 198c.
- `branch_to_offset_19ae`: Performs a short branch to the instruction at offset 19ae.
- `branch_to_offset_19d0`: Performs a short branch to the instruction at offset 19d0.
- `branch_to_offset_19f2`: Performs a short branch to the instruction at offset 19f2.
- `initialize_data_structures_2`: Sets up several pointers in a table by loading addresses from the program memory, calls another routine, and sets bit 1 of a hardware register.
- `jump_to_process_sprite_animation_and_p`: Jump stub: passes control straight on to process_sprite_animation_and_physics_2.
- `initialize_data_table`: Branches to another routine, saves registers, populates a table with several longwords from various memory locations, calls a routine to clear a buffer, and sets bit 1 of a hardware register.
- `store_byte_to_table`: If d1 is 3 and a value at a table address is zero, it stores the byte from d0 into a table at an offset determined by d1.
- `update_sprite_and_audio_parameters`: Updates a structure with new coordinate and velocity data, modifies hardware registers for audio and DMA, and calculates new positions for a set of objects using a table-based lookup.
- `update_sprite_and_audio_parameters_2`: Iterates through a table of structures to update sprite coordinates, animation frames, and hardware audio registers, while performing calculations for sprite scaling and bitplane visibility.
- `process_sprite_animation_and_physics_2`: Updates a hardware register using a value from d0 and d1, copies 64 words from a table to a new location, and then iterates through a structure to update various offsets, clear several memory blocks, and calculate new coordinates using a table.
- `initialize_complex_structure`: Calls copy_table_and_set_global, copies 64 words from a table to a new location, clears several memory areas using loops, and populates a structure with various pointers and values.
- `copy_table_and_set_global`: Sets a hardware register to a constant, copies a table of 64 words from a global variable to a destination pointed to by a pointer in a table, and then sets a hardware register to one of several values based on sentinel checks in a table before returning.
- `save_register_state`: Tests a global word, calls a subroutine, saves registers to a table, calls another subroutine, and then saves registers to a different table before returning.
- `update_sprite_attribute_table`: Iterates through a table of four entries to update sprite attributes, including modifying sprite positions and bitwise manipulation of sprite data based on specific flag values.
- `process_sprite_attribute_table`: Decrements a counter in a structure and returns, otherwise it iterates through a table of four entries to update sprite attributes, including pointer manipulation for linked lists, updating hardware-related data via a table, and modifying a status byte in a structure.
- `update_sprite_data_and_audio`: Checks a global variable, if non-zero it clears a field in a structure, updates hardware registers, and enters a loop that processes a table to update pointers and values in a structure, eventually copying data from several source tables into a destination structure based on sentinel values.
- `copy_byte_and_branch`: Copies a byte from offset 2 of the structure in a5 to offset 15 of the same structure and branches to another routine.
- `resolve_sprite_frame_data`: Calculates a pointer to a sprite frame by scaling a byte from a structure, then loads multiple word values from a table into a structure at offsets 14, 100, and 144, and finally updates a pointer at offset 12 using an offset from the table.
- `update_sprite_attribute_nibbles`: Reads a byte from offset 2 of a structure in a5 and splits its low and high nibbles to merge them with the low and high nibbles of a byte at offset 22 of the same structure, then writes the result back to offset 22.
- `merge_sprite_attribute_nibbles`: Reads a byte from offset 2 of a structure, splits it into two 4-bit nibbles, and merges them with the existing nibbles stored in a byte at offset 24 of the same structure.
- `dispatch_input_command`: Reads a byte from an offset in a structure, uses the high nibble to index a jump table for a tail branch, and otherwise modifies various fields in the structure at offsets 12, 14, 16, 18, 20, 26, and 42 based on the lower 4 bits of the input byte, including updating a hardware register via a bitwise OR operation.
- `jump_to_copy_table_and_set_global`: Jump stub: passes control straight on to copy_table_and_set_global.
- `update_attribute_byte`: Checks a byte at an offset of 2 from a5; if it is 1, it increments the value, stores it at offsets 9, 11, and 12 of a structure in a3, and branches to another routine.
- `update_sprite_attribute_table_data`: Calculates new sprite position and attribute values by performing bitwise operations and table lookups based on data in a structure, then updates the sprite attribute table via tail branches to process_sprite_attribute_table or update_sprite_attribute_table.
- `update_audio_dma_registers`: Checks a value in a global variable and updates several minimum byte values in RAM, then copies 64 bytes from a global variable to a table.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `write_word_and_bytes_to_memory`: Writes a 16-bit word from a register to a memory location and then writes four consecutive bytes of value 255 to the following memory addresses.
- `wait_for_non_zero_word`: Polls a 16-bit global variable in a loop until its value is zero, then returns.
- `configure_hardware_registers`: Initializes a structure in memory with several 16-bit values, sets a byte in another memory location, and writes values to the AUD3VOL, DMACON, and INTENA hardware registers.
- `conditional_ram_update`: Checks a word at an offset of 2 from a pointer in a1; if it is zero, it returns, otherwise it clears several locations in that table and stores a word from d0 into the location at offset 4.
- `process_data_stream`: Checks status flags in a global variable and incrementally copies data from a table into a destination buffer.
- `clear_buffer_and_update_pointer`: Sets a hardware register to a constant, clears 64 words in a buffer pointed to by a table, sets a hardware register to a different constant, and if a global variable does not match a specific routine address, updates that global variable and another memory location.
- `write_hardware_value`: Stores a long value from a PC-relative address into a global variable and writes the value 780 to a hardware register, then returns.
- `update_audio_buffer_pointer`: Checks a bit in a value read from a hardware register and, if set, updates the audio DMA destination address and length using a table of parameters, then performs an unrolled block copy of data from a source to the new audio buffer location.
- `configure_audio_hardware`: Sets several hardware registers for audio, including a destination address, length, and volume, then branches to another routine.
- `copy_memory_block_unrolled`: Copies a block of data from a source pointer in a2 to a destination pointer in a1 using an unrolled sequence of longword moves, where the number of copied words is determined by a value in d0.
- `restore_stack_pointer`: Returns immediately after restoring the stack pointer from a memory location.
- `bulk_copy_data_stream`: Copies blocks of data from a source pointer in a0 to a destination table based on a length value retrieved from a global table.
- `setup_copy_loop`: Calls another routine, copies the pointer in a0 to a1, sets d0 to 255, and branches to an earlier instruction.
- `restore_and_copy`: Pushes d0 to the stack, moves d1 to d0, calls another routine, copies a0 to a1, restores d0 from the stack, and branches to an earlier instruction.
- `fill_memory_with_word`: Fills sixteen consecutive words starting at a fixed address with the value provided in d0 using a loop.
- `prepare_fill_parameters`: Loads a value into d0, sets d1 to 0, pushes d0 onto the stack, and calls another routine.
- `check_status_flags`: Calls two routines and returns if the second routine does not set the zero flag.
- `test_bit_and_return`: Tests bit 0 of a word at a fixed address and returns.
- `wait_and_set_global_flag`: Waits for a bit in a global variable to be cleared, then stores three values to a memory location and sets that bit in the global variable.
- `copy_byte_within_structure`: Copies a byte from an offset of minus 1 relative to a pointer in a4 to an offset of minus 2 relative to the same pointer.
- `execute_complex_sequence`: Waits for a bit in a global variable to be cleared, stores values to a structure, and then enters a loop that performs a multiplication and calls another routine multiple times.
- `update_color_palette`: Decrements a byte in a global variable and, if non-negative, performs a complex nested loop to update color data in a structure and write it to the hardware color registers.
- `check_dma_controller`: Calls a routine, reads the DMA controller status register, and returns if bit 5 of the value in d0 is zero.
- `check_bit_and_halt`: Checks if a specific bit is set in a global variable; if not, it returns, otherwise it enters an infinite loop.
- `initialize_interleaved_tables`: Calls three other routines, clears a global variable, and then enters a loop that writes interleaved words to a buffer pointed to by a global variable, using values from another table to increment the data written.
- `update_and_interleave_data`: Calls two routines to update a pointer and write interleaved data, then branches to another routine.
- `initialize_data_tables`: Initializes eight entries in a table starting at a location pointed to by a global variable, setting specific values and copying data from a source buffer.
- `store_structure_to_table`: If a global variable is less than 8, it calculates an offset in a table, stores a pointer to that offset, and copies several registers and a pointer into the table at that location before incrementing the global variable.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `interleave_and_copy_data`: Calls a routine, performs a nested loop to copy words from a source to a destination based on a count from a table, and then performs a second loop to copy eight longwords from one location to another.
- `process_pointer_list`: Calls another routine, then iterates through a list of pointers from a global variable, comparing a word at offset 4 of each pointer with a word at offset 6 of a pointer from a table, and calls another routine if a byte at offset 8 of the table pointer is not negative.
- `update_and_shift_buffer`: Calculates a value based on a global variable and other registers, writes several bytes to a destination buffer, and then copies a block of data from a source to the destination buffer using a loop.
- `copy_and_terminate_buffer`: Copies a block of data from one location to another using a loop, then writes a longword to a location near the destination, clears a byte, and writes a word to the end of the destination.
- `loop_compare_pointers`: Checks bit 0 of d1 and, if set, enters a loop that compares a word at offset 4 of a pointer in a4 with d2, increments a pointer in a3, and repeats using a decrementing counter in d1.
- `process_buffer_and_branch`: Calls a routine, calculates a word value by subtracting a global variable from a value in a4, writes it to an offset of 2 in a structure, copies a longword from a6 to a4, sets bit 7 of the byte at offset 3 of that longword, and calls another routine before branching to a previous instruction.
- `write_data_to_offset_table`: Calculates an offset by multiplying d0 by 14, writes d1 and d2 to that offset in a table, and optionally writes a longword from a0 if d3 is not zero.
- `update_sprite_buffer_data`: Calculates an offset into a table using a multiplier, then writes a four-byte header consisting of two bytes of data, a sum of two values, and a status byte to a destination.
- `process_and_sync_buffer_data`: Writes a header of bytes to a destination, copies a block of data from a source, updates a global variable if it has changed, and optionally copies another block of data to a location derived from a global variable and a table offset.
- `sort_pointer_list`: Decrements a value from a global variable and, if non-negative, enters a loop that compares words at offset 6 of consecutive pointers in a list and performs a swap of the pointers if the second is smaller than the first.
- `decrement_and_return`: Sets a loop counter in d0 using a constant value and then returns.
- `store_pointer_to_global`: Stores the value of a register into a global variable and returns.
- `calculate_buffer_size`: Calculates a value in d7 by iterating through a list of data pointed to by a0, where each entry's size is derived from bits 8 to 9 of a word and a subsequent word, then calls a routine to clear a block of memory.
- `clear_and_index_table`: Clears a block of memory starting at a pointer in a1 with a size determined by d7, then reads two words from a0 to call another routine and calculates a destination address using bitwise shifts and additions.
- `shift_register_right`: Returns the value of register d7 shifted right by 2 bits.
- `iterate_nested_tables`: Extracts bitfields from d2 to control nested loops, calculates an offset into a table using d1 and a global pointer, calls another routine, and iterates through the resulting data structures.
- `dispatch_indirect_jump`: Sets a bit in a global variable based on bit 5 of d0, loads four bytes from a table at offset 0, 8, 16, and 24 from a6 into d2, d3, d4, and d5, and performs an indirect jump to an address in a5 determined by bits 3 and 4 of d0.
- `copy_bytes_from_table`: Copies four bytes from a table at a memory location indexed by registers d2, d3, d4, and d5 into those same registers.
- `execute_indirect_call`: Stores four bytes from registers d2, d3, d4, and d5 into a memory location at a base address in a4 using offsets 0, 2, d7, and d7 plus 2, increments a4 by 4, adds d6 to a6, and loops using d1 before returning.
- `copy_interleaved_data`: Copies data from a source buffer to a destination buffer using nested loops, performing specific word-sized moves and pointer increments based on values in d0, d1, d2, and d3.
- `duplicate_pointers_to_memory`: Copies the contents of a0, a1, and a2 twice each to a destination starting at a fixed address and returns.
- `update_pointer_from_search`: Calls another routine and then copies the value at the pointer in a1 to the word at offset 4 of that same pointer.
- `search_table_for_value`: Searches a table for a match with the value in a0, returning 1 if found, 0 if the loop completes, or negative two if a specific condition is met.
- `write_interleaved_data_2`: Checks a bit in a global variable and then either writes interleaved words to a buffer in a loop or calls another routine twice.
- `write_interleaved_data`: Writes a series of words and swapped words to a buffer in a loop, incrementing the word value and adding d3 to d1 each iteration.
- `write_interleaved_data_3`: Calls a routine, then enters a loop that writes a sequence of words to a buffer pointed to by a0, where each word is composed of a counter and a value from a table, incrementing the counter and adding a value from the table at each step until the counter reaches 352.
- `initialize_data_structure`: Calls another routine, then loads a pointer from a table, writes the value 258 to the location pointed to by that pointer, clears the following byte, and branches to another routine.
- `store_scaled_product`: Negates d0, calls another routine, and then stores the resulting word, longword, and word into a structure at offsets 0, 2, and 6.
- `calculate_scaled_product`: Calculates a value by multiplying two bytes from a table at an offset derived from d0, then multiplying the result by 32 and 47.
- `branch_to_sub_535c_and_jump`: Calls a subroutine at offset 535c and then performs an unconditional jump to offset 53ac.
- `clamp_and_loop_value`: Saves registers to the stack, compares d4 to a constant, and if d4 is not less than that constant, enters a loop that subtracts a value from d4 and increments d5 until d4 is no longer greater than or equal to the subtracted value.
- `process_and_restore_registers`: Adds a longword from a pointer in a0 to d4, shifts d5 left by 4 bits, loops using d6, performs a bitwise OR between d4 and d5, moves the result to d4, and restores registers from the stack before returning.
- `modify_buffer_byte_conditionally`: Loads three bytes from a buffer into registers, checks bit 3 of a register, and if it is clear, subtracts a byte from the second byte of the buffer before branching to another routine.
- `store_three_bytes`: Stores the bytes from registers d1, d2, and d3 into three consecutive memory locations starting at a fixed address.
- `pad_string_with_spaces`: Calls a routine to format a hexadecimal string into a buffer, then shifts the string to the right within that buffer to align it to a specific length of 8 or 10 characters, filling the vacated leading positions with spaces.
- `format_hex_string`: Converts a value in register d4 into a string of characters, potentially including commas, and writes the result to a buffer pointed to by a0.
- `load_three_bytes`: Loads three consecutive bytes from a buffer pointed to by a0 into registers d1, d2, and d3.
- `parse_encoded_data_stream`: Reads a data structure from a pointer in a1 to extract a multiplier and a count, then iterates through a byte stream in a0 to process control characters and calls process_encoded_data_stream and calculate_scaled_product.
- `process_encoded_data_stream`: Reads a data structure from a pointer in a1 to extract a multiplier and a count, then iterates through a byte stream in a0.
- `update_memory_offset_and_check_status`: Multiplies a value from a global variable by 8 and adds it to another global variable, increments a byte in a register, and compares it to a byte at offset 7 of a structure in a1; if they match, it checks a bit in a global variable to return either 0 or -1, otherwise it returns 0.
- `manage_dynamic_memory_pool`: Iterates through a nested structure to clear memory blocks and manage a pointer-based allocation pool.
- `increment_pointer_and_branch`: Increments a long value in a register by one, adds a word from another register to that same register, and performs a short branch to another routine.
- `branch_on_byte_value`: Compares a byte in register d0 to 255 and branches to another routine if equal, compares it to 254 and branches to another routine if equal, otherwise it branches to a previous routine.
- `increment_and_check_counter`: Increments a global variable by a value derived from a word at a specific memory location multiplied by eight, then compares a byte at offset seven of a structure in a1 against a counter in d3.
- `calculate_offset_and_pointer`: Calculates a memory offset by multiplying a byte from a buffer by 8 and adding a value from a global variable, stores the result in a global variable, and then calculates a second offset using a byte from the buffer and a multiplier from a global variable to return a pointer in a3.
- `calculate_table_index_and_call_subroutine`: Calculates a value by dividing a modified pointer offset by eight and another value by three hundred eighty-four, then uses the result to fetch a longword from a table and calls a subroutine with two saved pointers from memory.
- `find_string_length`: Iterates through a string in a buffer to find the position of a null terminator and returns a calculated value based on its offset.
- `check_status_and_call_subroutine`: Checks bit 1 of a global word and compares a byte at a negative offset from a2 to 32; if the byte is greater than 32, it saves registers, calls another routine with d0 set to 5, restores registers, and updates bit 1 of the global word based on the sign of the returned value.
- `clear_bit_and_branch`: Clears bit 0 of a word at a specific memory location and then branches to another routine.
- `update_bitmask_table`: Calculates a destination address using several global pointers and performs bitwise operations to update a table with bytes from a source buffer through a loop.
- `execute_jump_table_entry`: Calculates an offset from d0, loads a pointer from a table at that offset into a0, and jumps to the address contained in a0 if d1 is non-negative.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `process_combined_value`: Calls another routine, clears bit 7 of a byte from a table at a1 plus d0, and then either calls another routine or modifies a byte at a1 plus d0 or a1 plus a new d0 based on bit 0 of d2 and the sign of d0.
- `modify_table_entry`: Checks bit 1 of d2, clears bit 7 of a byte at a1 plus d0, increments d0, compares d0 to a byte in a0, and if d0 is less than that byte, sets bit 7 of a byte at a1 plus d0 and calls another routine; otherwise, it clears bit 7 of a byte at a1 plus d0 and calls another routine.
- `mask_and_test_byte`: Loads a byte from a table at a1 plus d0 into d1, masks d1 with 127, checks bit 2 of d2, and then either decrements d1 or clears it based on whether d1 is positive or negative before branching.
- `update_status_byte_3`: Checks bit 3 of d2, increments d1, compares d1 to a byte at an offset of a2, and if the condition is met, sets bit 7 of a modified d1 and stores it in a table at a1 plus d0, then calls two other routines.
- `clear_global_byte`: Saves d0 through d2 to the stack, clears a byte at a global variable address, and restores the registers before returning.
- `execute_sequence_of_callbacks`: Loads a byte from a pointer in a0, calls another routine, and then enters a loop that checks a byte from a table in a1, and if negative, calls another routine using a pointer and an offset derived from a byte in a0.
- `jump_to_next_routine`: Performs a jump to a location relative to the program counter.
- `jump_to_address`: Performs a jump to an address stored in a memory location relative to the program counter.
- `convert_integer_to_ascii_buffer`: Calculates the quotient and remainder of a value divided by ten to convert a number into decimal characters, then writes these characters into a buffer pointed to by a register and calls format_numeric_string_and_update_buffer.
- `parse_scaled_data`: Tests bit 4 of a byte at a global address, and if clear, loads two bytes from a pointer in a2 into d2 and d3, decrements d2, and calls another routine with a pointer in a0.
- `parse_buffer_bytes`: Increments a pointer in a0, loads two bytes from the incremented pointer into d2 and d3, calls another routine, and then moves the value from a2 into a0 before returning.
- `parse_encoded_data_stream_2`: Loads a byte from the stack, loads two bytes into d2 and d3, calls another routine, and then enters a loop that decrements a value in d4 and calls another routine until d4 is negative or a zero byte is encountered.
- `process_byte_stream`: Calls another routine, loads a pointer and a byte from the stack, decrements the byte, and then enters a loop that checks bytes at the pointer until a zero byte is found or the byte counter reaches zero.
- `format_numeric_string_and_update_buffer`: Increments a pointer in a0, loads two bytes into d2 and d3, calls another routine, sets a0 to a new address, performs division on a value in d4, and if the result is not zero, writes two ASCII characters to a buffer before calling another routine.
- `process_complex_data_structure`: Clears a bit in a global variable, writes a value to a global variable, and if a condition is met, calls another routine and enters a loop that calls a subroutine while decrementing a global variable.
- `branch_to_offset`: Performs a short branch to a fixed offset.
- `process_input_and_state`: Calls several routines to evaluate input, updates a global byte by masking bits, and performs conditional calls based on the results of those routines.
- `compare_and_clear`: Clears a word at a memory location, calls a routine, and returns if the value in d1 is not equal to a value at a global variable; otherwise it returns with d2 set to zero.
- `process_input_and_update_status`: Calls a routine to retrieve a value, then uses it as an index into a table to load a pointer into a0 and a value into a1.
- `loop_and_call_function`: Calls a routine, compares d1 to a value at a global variable, and if not equal, enters a loop that calls a routine at a pointer in a table and decrements d2, returning with d2 set to zero if the loop completes or -1 otherwise.
- `initialize_and_call_subroutine`: Clears a byte at a memory location pointed to by a0, loads a word from a global variable into d0, and calls another routine.
- `write_color_values_to_register`: Enters a loop that calls another routine to write specific color values derived from d1 and d2 to a hardware register pointed to by a6.
- `loop_with_memory_writes`: Writes a value from d5 to the color register, then enters a loop that calls a routine, writes zero to a memory location pointed to by a6, and calls the routine again, finally calling the routine one last time before returning.
- `load_and_call_subroutine`: Loads a long value from a memory location into a1, clears d4, calls a subroutine, tests d0, and returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_constant`: Returns the constant value a fixed address.
- `write_constant_sequence`: Writes a sequence of constant values to several global word and byte locations.
- `toggle_global_state`: Saves two words from a global variable, sets the global variable to zero, executes several subroutines with alternating values of zero and one, and then restores the original words to the global variable.
- `decrement_conditional_global`: Decrements a global variable if its bit 5 is not set.
- `update_status_byte_2`: Calls a subroutine, performs arithmetic on a value, writes a word from a table to a global variable, and conditionally calls another subroutine twice while performing bitwise rotations on a global variable.
- `update_status_byte`: Calls three other routines to evaluate a word in d0 and writes a modified byte with the high bit set to a specific memory location.
- `update_global_state_2`: Calls a subroutine, performs arithmetic on a value, modifies a global variable using bitwise OR, and resets a global variable to a constant value.
- `reset_global_variable`: Clears a 16-bit global variable, calls two subroutines, sets a global variable to one, and moves a byte from one global variable to another.
- `execute_complex_logic`: Performs a sequence of conditional calls, updates multiple global variables, manipulates a global word with bitwise operations, and eventually performs an absolute jump.
- `branch_to_routine`: Pushes the address of a local offset onto the stack, calls two subroutines, and branches to another routine.
- `set_global_zero`: Calls another routine, sets bit 0 of a global variable, sets bit 1 of another global variable, stores a byte from a global variable, and clears a third global variable.
- `execute_conditional_subroutines`: Calls three routines and then calls a third routine; if a global variable is zero, it executes a sequence of four additional subroutines.
- `update_global_word_2`: Calls two subroutines, jumps to another routine, compares a global word to one, and conditionally calls a third subroutine before returning.
- `update_and_jump`: Calls a subroutine, copies a word from a global variable to d0, updates three global word locations based on d0, and performs an absolute jump.
- `test_and_branch`: Clears register d0, calls two routines, and returns after testing d0.
- `test_global_and_branch`: Loads a word from a global variable into d1, tests a byte in another global variable, and branches to a different location if the byte is negative, otherwise it branches back to the start of the routine.
- `test_and_negate`: Loads a word from a global variable into d1, tests a byte at a global variable, calls a routine, and if the zero flag is set, negates d0 before returning.
- `test_and_branch_negative`: Tests a word in d0 and branches to a different location if the result is negative.
- `return_if_bit_set`: Checks bit 7 of d1 and returns if it is clear, otherwise it moves d2 into d0 and returns.
- `load_address_and_call`: Loads a memory address into a0, calls another routine, and then loads a different memory address into a0.
- `clamp_value_towards_target`: Adjusts a 16-bit value at an offset from a pointer by one unit towards a second 16-bit value at a different offset.
- `fetch_table_value`: Clears three global variables, calls another routine to obtain a value, uses that value as an index to fetch a word from a table and stores it in a global variable, then calls another routine before returning.
- `return_clamped_byte`: Returns a global byte value, or returns two if the value is unable to be clamped otherwise.
- `update_clamped_status_value`: Checks a global variable against a constant and a second global variable, then calls check_status_flags_2.
- `clamp_and_update_value`: Decrements a global word, calls a clamping routine, and then uses the result to index into a table to fetch a value which is stored in another global variable if it is larger than the current value.
- `fetch_table_value_2`: Calls another routine, adds d0 to itself, and uses the result to fetch a word from a table at a0.
- `clamp_global_variable_range`: Adds a 16-bit value from a register to a global variable, then clamps the resulting value between 0 and 9 before storing it back to the global variable.
- `load_value_from_table`: Calls another routine, uses the returned value to calculate an offset into a table, and stores the resulting longword from that table into a global variable.
- `check_status_flags_2`: Checks two word-sized global variables and returns 0, 1, or 2 based on whether they are non-zero.
- `increment_and_clamp_value`: Increments a word in a global variable and resets it to 0 if the result is 9 or greater.
- `zero_multiple_globals`: Sets a sequence of global word locations to zero.
- `increment_and_clear_pointers`: Increments two global variables, and if the second exceeds four, resets it to one and calls another routine eight times with different addresses, then calls two other routines using pointers from a list.
- `clear_memory_block`: Clears six consecutive longwords starting from the address in a0 using a loop.
- `clear_memory_via_null_terminator`: Iterates through a list of pointers and writes zero to the memory locations they point to until a null pointer is encountered.
- `evaluate_conditional_sequence`: Loads two bytes from a table, masks them, and then performs a series of calls with different values in d2, branching based on the results to either store a word in a global variable or update a global variable with a specific value.
- `conditional_store_word`: Calls a routine and, if the result is not zero, stores the word 8 into a global variable before branching to another location.
- `execute_recursive_conditional_assignment`: Calls another routine and, if the result is not zero, checks a byte at a global variable and, if it is not zero, sets a global variable to six after decrementing a counter in d7.
- `initialize_data_structures`: Iterates through a table of pointers, writing a constant value to a structure pointed to by the table, and setting specific fields in that structure to zero or a maximum value.
- `call_routines_conditionally`: Iterates through a list of pointers and, if a bit is set in a byte at an offset from the pointed-to address, calls two different routines.
- `clear_table_bits`: Iterates through a table using a counter, checking if a bit is set at an offset of a structure and if a word at another offset equals 20, then clears that bit if both conditions are met.
- `iterate_data_table`: Pushes the value of a0 to the stack, loads a count and several pointers from a table, and enters a loop that tests bit 0 of an offset 16 from a pointer in a3, decrementing the count until the loop finishes.
- `pop_pointer_and_return`: Calls another routine, pops a pointer from the stack into a0, clears d0, and returns.
- `update_structure_and_traverse_list`: Updates a pointer in a structure, clears an offset 52 word, and then traverses a linked list of words to clear specific offsets in each node.
- `compare_and_return`: Compares d0 to d2 and d1 to d2, then returns.
- `load_addresses_and_call_2`: Loads two word addresses into a0 and then calls another routine.
- `update_byte_with_flag`: Checks if a byte at a pointer in a0 is negative; if so, it fetches a value from a table using a pointer and an index, sets bit 7, and writes it to a0, or alternatively sets bit 7 of a decremented value from a global variable into a0.
- `dispatch_indirect_call`: Checks if a global variable is equal to one; if not, it calculates an offset from a word in a global variable, loads a word from a table, and performs an indirect jump to an address loaded from a table.
- `process_data_tables`: Calls a subroutine three times with different values in d0 and a0, iterates through a loop seven times using a counter in d1, and if a global variable is non-zero, performs a second loop that writes values from a table into a destination pointed to by a3.
- `test_bits_and_push_pointers`: Tests various bits in registers d0 and d1 against offsets in registers a0 through a6, then pushes several longwords onto the stack pointed to by a1.
- `jump_to_address_2`: Performs an absolute long jump to a specific address.
- `copy_data_with_check`: Tests d1 and a word in a global variable, and if both are non-zero, calls a subroutine, swaps a0 and a1, and if the result in d0 is zero, copies bytes from a0 to a1 in a loop until a counter in d2 reaches zero.
- `reset_global_variable_2`: Calls another routine twice with different values in d0 and a0, then sets a global variable to zero.
- `load_and_verify_asset_data`: Performs four sequential data transfers using specific lengths and source addresses, then checks two bytes from a global variable against values in a table to determine if a subsequent data swap or copy operation should be executed.
- `copy_and_process_data`: Copies data from one location to another using various loop lengths and calls several routines, including one that performs a jump to a fixed address.
- `execute_conditional_calls`: Calls two different routines and, if the first does not result in a zero flag, calls a third routine before comparing d0 and d1 and returning.
- `setup_search_parameters`: Loads a pointer into a4 and moves d0 or d1 into d4 before branching to another location.
- `search_table_for_value_2`: Iterates through a table of words, comparing each entry to d0 or d1, and returns if a match is found.
- `load_pointers_and_bytes`: Loads a pointer into a0, a pointer into a1, a byte into d1, calls another routine, then loads a different pointer into a0, a different pointer into a1, and a different byte into d1.
- `update_pointer_table`: Stores a pointer in a location, calculates an index from a register, and if a value at a table offset is non-zero, stores the pointer into a second table location and calls another routine.
- `update_global_flags`: Checks a global variable against multiple conditions and performs bitwise operations or calls another routine based on the result.
- `populate_table_from_global`: If a global variable is not zero, it copies its value and the result of another routine into a table using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_and_call_table`: Calls a routine, then enters a loop that decrements d2 from 2 to 0, calling another routine with the counter and values loaded from a table, stores a byte in a global variable, and checks a byte at a global variable before returning.
- `iterate_and_call_routine`: Decrements a counter from 2 to 0 in a loop, calling another routine with the counter and a value from a table, and returns if a global variable is non-zero.
- `execute_sequence_and_jump`: Calls two different routines, loads the value 260 into d0, loads the address of a global variable into a0, and jumps to another routine.
- `execute_jump_sequence`: Calls two routines, loads a long value into a pointer in a0, sets d0 to 267, and jumps to another routine.
- `initialize_and_process_loop`: Clears a longword in a global variable, calls several routines, initializes several global words and a pointer, and enters a loop that increments two global words and calls a routine until a global byte is zero or a comparison with a value from another routine fails.
- `execute_jump_sequence_alt`: Calls two routines, stores an address in a global variable, sets d0 to 267, and jumps to another routine.
- `execute_callback_from_structure`: Loads a pointer from a global variable into a6, calls another routine if the word at that pointer is not zero, and if a comparison with a global word succeeds, it increments a6, fetches a value and a pointer from the structure, and calls the pointer via a jump.
- `update_stack_pointer`: Increments a6 by 2, loads a new value into a6 from a pointer, clears a 16-bit word in a global variable, and stores the new a6 into another global variable.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_with_address`: Loads a long value into a0 and jumps to another routine.
- `decrement_global_word`: Loads two words into d0, calls another routine, loads a third word into d0, and if d0 is not zero, decrements it and stores the result back into a global word.
- `modify_global_word_via_stack`: Pushes a logical right-shifted version of d0 onto the stack, calls another routine, stores the shifted d0 into a global byte, restores d0 from the stack, and if d0 is not 2048, adds 256 to it and stores the result into a global word.
- `update_byte_from_global`: Loads a byte from a global variable, and if it is not negative, extends it to a word, calls another routine, and stores the original byte back into the global variable.
- `execute_sequence_and_store_constant`: Calls another routine with 1032 in d0, calls another routine with 1033 in d0, calls a third routine, stores 9 in d0 into a global word, and branches to another routine.
- `execute_conditional_task`: Reads a byte from a global variable and, if the value is not negative, extends the sign of the byte and calls another routine before storing the original byte into the global variable; otherwise, it returns.
- `write_four_words`: Writes the constant values 40, 8000, 4044, and 3066 to four consecutive words in a global memory area and returns.
- `write_different_words`: Writes the constant values 40, 8960, 1484, and 256 to four consecutive words in a global memory area and returns.
- `compare_and_return_2`: Calls another routine, compares the result in d0 to a global word, and returns if the result is less than or equal to that word; otherwise, it adjusts the stack pointer and returns.
- `compare_and_return_alt`: Calls another routine, compares the result in d0 to a global word, and returns if the result is greater than or equal to that word; otherwise, it adjusts the stack pointer and returns.
- `calculate_scaled_value_2`: Tests bit 7 of a global variable at offset 500; if set, it branches to another routine, otherwise it multiplies d0 by 5, divides d0 by 6, and returns.
- `calculate_and_update_graphics`: If d0 is not zero, it decrements d0, calls several routines including one that calculates an offset from d1 and d2 added to a base address, and then performs multiple calls to other routines before returning the original d0 value.
- `load_table_data`: Multiplies d0 by 14, uses the result to calculate an offset into a table, loads a long word from that table into a0, loads five consecutive words into d3, d4, d1, d2, and a global variable, and returns.
- `update_blt_parameters_and_draw`: Calculates Blitter parameters including source, destination, and size based on input values and global variables, then performs multiple Blitter transfers using the hardware registers.
- `update_structure_offset_data`: Calculates several offset values based on differences between input words and a global variable, then updates a structure pointed to by two registers using a loop that runs four times to write these calculated values to various offsets.
- `calculate_offset_differences`: Subtracts two words from a constant, and if the result is positive, it adds the result to a global word, negates it, and stores the doubled negative value in another global word.
- `update_structure_data`: Performs several calculations using values from a global variable and a table, then enters a loop that runs four times to update various fields in a structure pointed to by a6, including long words, integers, and words, before returning.
- `copy_table_and_update_globals`: Calls two routines, then loops from 23 down to 0 to copy bytes from a table into a destination area, updates several global variables with specific values, and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_2`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_3`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_4`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_5`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_6`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_7`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_8`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_9`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_10`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_11`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_12`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_13`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_14`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `return_and_branch`: Returns immediately, though it is followed by a branch instruction.
- `nop_padding_15`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_16`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_17`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_18`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_19`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_20`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_21`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_22`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_23`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_24`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `reset_interrupt_requests`: Writes a value of 32767 to the interrupt request register and returns from an interrupt.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_trap`: Sets a global variable to 32767, calls another routine, and then performs a trap if certain conditions are met.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_color_registers`: The routine iterates through a range of memory addresses associated with the custom chip's color registers, writing zero to each location to clear the palette.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_decode`: The routine clears color registers, decodes run-length encoded data into a destination, calls another routine up to two times in a loop, checks specific bits in a memory location, and manages DMA and interrupt enable registers.
- `call_routine_iteratively_and_store_word`: Calls another routine up to two times in a loop, decrementing a counter each time if the return value is non-zero, and then stores a 16-bit value from a global variable into two consecutive byte locations.
- `check_ciaa_bits`: Checks bit 7 and bit 6 of a memory location via a pointer in a5, setting d0 to 0 if either bit is set.
- `clear_color_registers_2`: Iterates through thirty-two consecutive words starting at the hardware color registers, writing zero to each to clear the palette, then returns.
- `decode_rle_data_block`: The routine iterates through a source buffer pointed to by a global variable and decodes run-length encoded data into a fixed destination area, repeating or copying bytes based on the sign of control values.
- `increment_and_set_interrupt`: The routine increments a global counter variable and sets another global variable to a specific constant value, then returns from an interrupt.
- `compare_and_exit`: The routine compares a word from a memory location with itself and returns if they are not equal; otherwise it returns.
- `generate_data_table`: The routine calls another routine to copy data, then enters nested loops to calculate and write a series of bytes and words to a memory range starting at a fixed address, finally copying data from a table to a destination.
- `copy_word_array`: The routine copies a sequence of 16-bit words from a source pointer to a destination pointer, decrementing a counter until the operation is complete.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_jump`: The routine clears register d0 and jumps to a long address.
