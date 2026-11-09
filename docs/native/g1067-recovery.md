# Game 1067 (MortalKombat2 v1.0 1MB 1105): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01067). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (6648772 top-level bytes in 50 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 34 of 34 units identical (bytes and relocations), 36 raw-word lines |
| Functions named and described | pass, 224 of 224 named (0 author symbols), 224 of 224 described (12 by rule, 212 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1603 of 1604 (99.94%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4189 of 4288 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 362696 |
| code | 15046 |
| data | 6225910 |
| padding | 64 |
| unused | 45056 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 277 | 13 |
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
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3668 | 190 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 52 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |
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
- `increment_and_wrap_global`: Increments a 16-bit global variable and resets it to zero if it reaches three.
- `calculate_and_call`: If d1 is not zero, it exchanges d0 and d1, exchanges d1 and d2, performs arithmetic on d0 and d1, increments d2, and calls a routine using a pointer from a2.
- `set_bit_conditionally`: Compares d0 to a byte at a relative offset; if they are equal, it branches to another location, otherwise it sets bit 6 of a byte at an offset from a0 and returns.
- `decode_compressed_chunk`: Validates a data header signature and processes a compressed data stream using a stack-based algorithm.
- `parse_bit_table`: Scans a table of words for a matching condition, then calculates a bit mask based on byte values and register contents.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `perform_arithmetic_on_registers`: Performs a series of arithmetic and bitwise operations involving registers d1, d6, and d7, and accesses two bytes from a table using a pointer in a3.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `build_huffman_tree_nodes`: Iterates over a buffer to match bytes against a running value, writing masks, shifted values, and node properties into a destination array to build a tree structure.
- `prepare_stack_pointer`: Clears d0 and pushes a constant value onto the stack before branching to another location.

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
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_status_bits`: Clears a byte at a global variable and sets bit 5 of another global variable.
- `mask_global_variable_bits`: Clears bit 5 of a global variable, sets a byte at offset 9a9 to zero, and then performs a multiplication of a value from a global variable, stores the result in another global variable, calls two other routines, and returns either zero or one based on the sign and value of the result.
- `multiply_and_check_sign`: Multiplies a global variable by a value from a global variable, stores the result in another global variable, calls two other routines, and returns zero if the result is negative or if a branch is taken, otherwise it returns zero.
- `check_global_status`: Checks if a global variable is negative and performs bitwise operations to return specific constant values.
- `multiply_and_branch`: Calls another routine, then calculates a value by multiplying a constant by a value from a global variable before branching.
- `branch_to_offset_3`: Calls another routine, sets d0 to 3, and then branches to another location.
- `prepare_buffer_operations_2`: Saves a pointer to the stack, calls two subroutines, loads a pointer from a global variable into a1, calls another subroutine, restores the pointer to a0, sets d0 to negative one, and branches to another routine.
- `prepare_buffer_operations`: Loads a pointer from a global variable into a0, calls two subroutines, loads another pointer from a global variable into a0, and branches to another routine.
- `copy_nested_blocks`: Performs a triple nested loop to copy data from one memory location to another using several counters.
- `clear_and_set_bits`: Clears a byte at a global variable and sets bit 1 of another global variable.
- `fill_data_buffers`: Fills a destination buffer by calling another routine in a loop, then checks a bit in a global variable to decide whether to fill a second buffer using a different routine in another loop.
- `call_and_rotate`: Calls another routine and then rotates the bits of d1 by eight positions.
- `copy_data_block_word`: Performs bitwise operations and shifts on d1 and d2 using values from d4, d5, d6, and d7.
- `branch_with_address_load`: Pushes a0 onto the stack, loads a relative address into a0, and branches to another routine.
- `update_global_conditional`: If bit 7 of a global variable is not set, it sets bit 6 of that variable and updates the variable with the value from a register.
- `process_state_machine`: Checks bit 7 of a global variable and, if not set, performs complex updates to a structure and a global variable based on values from a table.
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
- `branch_to_address`: Performs a relative branch to another address.
- `branch_to_sprite_attribute_updater`: Performs a short branch to the middle of the process_sprite_attribute_table routine.
- `branch_twice`: Performs two consecutive relative branches to different locations.
- `process_sprite_animation_and_physics`: Iterates through a table of structures to update sprite coordinates, animation frames, and hardware audio registers, while performing calculations for movement and frame transitions.
- `branch_to_offset_2`: Performs a relative branch to another location.
- `branch_to_offset_4`: Branches to a specific offset.
- `branch_to_offset_198c`: Performs a short branch to the instruction at offset 198c.
- `branch_to_offset_19ae`: Performs a short branch to the instruction at offset 19ae.
- `branch_to_offset_19d0`: Performs a short branch to the instruction at offset 19d0.
- `branch_to_offset_19f2`: Performs a short branch to the instruction at offset 19f2.
- `initialize_data_structures_2`: Sets up several pointers in a table by loading addresses from the program memory, calls another routine, and sets bit 1 of a hardware register.
- `store_byte_to_table`: If d1 is 3 and a value at a table address is zero, it stores the byte from d0 into a table at an offset determined by d1.
- `update_sprite_and_audio_parameters`: Updates a structure with new coordinate and velocity data, modifies hardware registers for audio and DMA, and calculates new positions for a set of objects using a table-based lookup.
- `set_global_and_or_byte`: Stores 6 into a global variable and returns the input byte ORed with 254.
- `update_sprite_and_audio_parameters_2`: Iterates through a table of structures to update sprite coordinates, animation frames, and hardware audio registers, while performing calculations for sprite scaling and bitplane visibility.
- `store_zero_byte_sequence`: Stores a sequence of thirteen zero bytes into a data table.
- `store_zero_bytes_to_data_register`: Repeatedly performs a bitwise OR operation with zero on a data register, effectively acting as a sequence of zeroed bytes within a data table.
- `reset_global_values`: Sets a global variable to 0, another global variable to 1, and a third global variable to 255 before returning.
- `process_sprite_animation_and_physics_2`: Updates a hardware register using a value from d0 and d1, copies 64 words from a table to a new location, and then iterates through a structure to update various offsets, clear several memory blocks, and calculate new coordinates using a table.
- `copy_table_and_set_global`: Sets a hardware register to a constant, copies a table of 64 words from a global variable to a destination pointed to by a pointer in a table, and then sets a hardware register to one of several values based on sentinel checks in a table before returning.
- `clear_data_register_d0`: Repeatedly performs a bitwise OR operation with zero on the data register d0.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 158 into d4.
- `poll_four_slots_return_pair`: Iterates through four consecutive entries in a global table, calling a routine for each entry that is not a sentinel value, and returns a register pair with a high-bit tag.
- `save_register_state`: Tests a global word, calls a subroutine, saves registers to a table, calls another subroutine, and then saves registers to a different table before returning.
- `update_sprite_attribute_table`: Iterates through a table of four entries to update sprite attributes, including modifying sprite positions and bitwise manipulation of sprite data based on specific flag values.
- `update_attribute_value`: Loads a byte from an offset of two from a5 into d0, subtracts d0 from a word at offset twelve of a5, compares the result to seventy-one, and if the result is greater than seventy-one, writes the value to offset six of a4 and branches to another routine.
- `move_and_branch`: Moves a word from an offset of a5 to an offset of a4 and branches to another routine.
- `process_sprite_attribute_table`: Decrements a counter in a structure and returns, otherwise it iterates through a table of four entries to update sprite attributes, including pointer manipulation for linked lists, updating hardware-related data via a table, and modifying a status byte in a structure.
- `update_sprite_data_and_audio`: Checks a global variable, if non-zero it clears a field in a structure, updates hardware registers, and enters a loop that processes a table to update pointers and values in a structure, eventually copying data from several source tables into a destination structure based on sentinel values.
- `update_audio_dma_registers`: Checks a value in a global variable and updates several minimum byte values in RAM, then copies 64 bytes from a global variable to a table.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `modify_pointer_and_hardware`: Negates the word at the pointer in a0, moves a byte from a2 to a global variable, writes several values to a table and to hardware registers, and returns the value in d0 ORed with 3.
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
- `initialize_data_tables`: Initializes eight entries in a table starting at a location pointed to by a global variable, setting specific values and copying data from a source buffer.
- `duplicate_pointers_to_memory`: Copies the contents of a0, a1, and a2 twice each to a destination starting at a fixed address and returns.
- `update_pointer_from_search`: Calls another routine and then copies the value at the pointer in a1 to the word at offset 4 of that same pointer.
- `search_table_for_value`: Searches a table for a match with the value in a0, returning 1 if found, 0 if the loop completes, or negative two if a specific condition is met.
- `write_interleaved_data_2`: Checks a bit in a global variable and then either writes interleaved words to a buffer in a loop or calls another routine twice.
- `write_interleaved_data`: Writes a series of words and swapped words to a buffer in a loop, incrementing the word value and adding d3 to d1 each iteration.
- `write_interleaved_data_3`: Calls a routine, then enters a loop that writes a sequence of words to a buffer pointed to by a0, where each word is composed of a counter and a value from a table, incrementing the counter and adding a value from the table at each step until the counter reaches 352.
- `store_scaled_product`: Negates d0, calls another routine, and then stores the resulting word, longword, and word into a structure at offsets 0, 2, and 6.
- `calculate_scaled_product`: Calculates a value by multiplying two bytes from a table at an offset derived from d0, then multiplying the result by 32 and 47.
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
- `load_value_from_table`: Calls another routine, uses the returned value to calculate an offset into a table, and stores the resulting longword from that table into a global variable.
- `zero_multiple_globals`: Sets a sequence of global word locations to zero.
- `increment_and_clear_pointers`: Increments two global variables, and if the second exceeds four, resets it to one and calls another routine eight times with different addresses, then calls two other routines using pointers from a list.
- `clear_memory_block`: Clears six consecutive longwords starting from the address in a0 using a loop.
- `clear_memory_via_null_terminator`: Iterates through a list of pointers and writes zero to the memory locations they point to until a null pointer is encountered.
- `evaluate_conditional_sequence`: Loads two bytes from a table, masks them, and then performs a series of calls with different values in d2, branching based on the results to either store a word in a global variable or update a global variable with a specific value.
- `conditional_store_word`: Calls a routine and, if the result is not zero, stores the word 8 into a global variable before branching to another location.
- `execute_recursive_conditional_assignment`: Calls another routine and, if the result is not zero, checks a byte at a global variable and, if it is not zero, sets a global variable to six after decrementing a counter in d7.
- `initialize_data_structures`: Iterates through a table of pointers, writing a constant value to a structure pointed to by the table, and setting specific fields in that structure to zero or a maximum value.
- `call_routines_conditionally`: Iterates through a list of pointers and, if a bit is set in a byte at an offset from the pointed-to address, calls two different routines.
- `compare_and_return`: Compares d0 to d2 and d1 to d2, then returns.
- `process_data_tables`: Calls a subroutine three times with different values in d0 and a0, iterates through a loop seven times using a counter in d1, and if a global variable is non-zero, performs a second loop that writes values from a table into a destination pointed to by a3.
- `modify_and_push_registers`: Sets a bit in a byte at the address in a2, performs several bit tests on various registers, and pushes several longwords onto the stack.
- `jump_to_address_2`: Performs an absolute long jump to a specific address.
- `copy_data_with_check`: Tests d1 and a word in a global variable, and if both are non-zero, calls a subroutine, swaps a0 and a1, and if the result in d0 is zero, copies bytes from a0 to a1 in a loop until a counter in d2 reaches zero.
- `reset_global_variable_2`: Calls another routine twice with different values in d0 and a0, then sets a global variable to zero.
- `load_and_verify_asset_data`: Performs four sequential data transfers using specific lengths and source addresses, then checks two bytes from a global variable against values in a table to determine if a subsequent data swap or copy operation should be executed.
- `copy_and_process_data`: Copies data from one location to another using various loop lengths and calls several routines, including one that performs a jump to a fixed address.
- `execute_conditional_calls`: Calls two different routines and, if the first does not result in a zero flag, calls a third routine before comparing d0 and d1 and returning.
- `setup_search_parameters`: Loads a pointer into a4 and moves d0 or d1 into d4 before branching to another location.
- `search_table_for_value_2`: Iterates through a table of words, comparing each entry to d0 or d1, and returns if a match is found.
- `update_global_flags`: Checks a global variable against multiple conditions and performs bitwise operations or calls another routine based on the result.
- `populate_table_from_global`: If a global variable is not zero, it copies its value and the result of another routine into a table using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_and_call_table`: Calls a routine, then enters a loop that decrements d2 from 2 to 0, calling another routine with the counter and values loaded from a table, stores a byte in a global variable, and checks a byte at a global variable before returning.
- `iterate_and_call_routine`: Decrements a counter from 2 to 0 in a loop, calling another routine with the counter and a value from a table, and returns if a global variable is non-zero.
- `execute_sequence_and_jump`: Calls two different routines, loads the value 260 into d0, loads the address of a global variable into a0, and jumps to another routine.
- `copy_table_and_update_globals`: Calls two routines, then loops from 23 down to 0 to copy bytes from a table into a destination area, updates several global variables with specific values, and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `return_and_branch`: Returns immediately, though it is followed by a branch instruction.

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
