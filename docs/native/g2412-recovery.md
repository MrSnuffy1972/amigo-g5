# Game 2412 (f-16 combat pilot (1989)(digital integration)[cr vf]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02412). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (913744 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 86 of 86 named (0 author symbols), 86 of 86 described (3 by rule, 82 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 490 of 490 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 107520 bytes, 22 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1939 of 2109 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 15516 |
| code | 7218 |
| data | 862805 |
| padding | 45 |
| unused | 28160 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf:bootblock | 1 | 176 | 4 |
| logged disk load from F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf boot-block load | 1 | 0 | 0 |
| logged disk load from F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf boot-block load | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1087 | 58 |
| F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf bytes 23552-87552 (code never loaded by a reference run) | 1 | 0 | 0 |
| F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf bytes 116224-125952 (code never loaded by a reference run) | 1 | 826 | 22 |
| F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf bytes 314368-348160 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: custom-format (non-DOS) disk, 1699 non-empty blocks: matched against track-loaded segments
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: the boot block reads 8192 bytes from disk offset 15360 to an address (found statically); recovered as a unit
- runtime segment an address: 490 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: bytes 23552-87552 look like code no run loaded: recovered position-independently (load address unknown)
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: bytes 116224-125952 look like code no run loaded: recovered position-independently (load address unknown)
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: bytes 314368-348160 look like code no run loaded: recovered position-independently (load address unknown)
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: 5 logged loads cover 121344 bytes of the image
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: blocks 2-6 (bytes 1024-3584) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.30)
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: blocks 174-174 (bytes 89088-89600) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: blocks 176-225 (bytes 90112-115712) outside the filesystem hold data, not code (opcode density 0.054, entropy 5.63)
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: blocks 250-501 (bytes 128000-257024) outside the filesystem hold data, not code (opcode density 0.030, entropy 5.83)
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: blocks 504-524 (bytes 258048-268800) outside the filesystem hold data, not code (opcode density 0.034, entropy 5.30)
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: blocks 526-609 (bytes 269312-312320) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.03)
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: blocks 705-789 (bytes 360960-404480) outside the filesystem hold data, not code (opcode density 0.010, entropy 5.38)
- f-16 combat pilot (1989)(digital integration)[cr vf]/F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf: blocks 807-1759 (bytes 413184-901120) outside the filesystem hold data, not code (opcode density 0.014, entropy 6.40)
- 2 never-loaded region(s), 97792 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 5632 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf:bootblock

- `load_and_decompress_boot_data`: Configures disk I/O parameters to read data from a storage device and then performs bitstream decompression by iteratively shifting values until a zero terminator is encountered.
- `decompress_bitstream_data`: Iteratively extracts bits from a memory stream by shifting values until a zero terminator is reached.
- `extract_bitstream_data`: Iteratively shifts a value until it reaches zero, at which point it decrements a pointer to load a new word from memory into a global variable and refills the register with the new value's bits.
- `process_bit_stream`: Iteratively shifts bits and loads values from memory into a global variable while interacting with a hardware register.

### logged disk load from F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf boot-block load

- `set_background_color_and_return`: This routine is a 15-instruction 68000 subroutine that is reached through a PC-relative address-taken reference and is called by one unnamed routine.
- `execute_five_instructions`: This routine consists of five instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_infinite_loop`: An infinite loop that prevents further execution.
- `process_input_and_update_counters`: Reads joystick and mouse data, updates several global variables, and performs arithmetic on values derived from hardware port inputs.
- `perform_no_operation`: The routine executes a sequence of instructions that results in no state change or functional output before returning.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `clear_global_and_loop`: Sets a global variable to zero and enters a loop based on a bitwise OR of a register value.
- `clear_global_variable`: sets a global variable to zero and returns.
- `clear_global_variable_2`: Sets a global variable to zero.
- `clear_global_variable_signed`: Sets a global variable to zero and performs a loop check based on a signed register value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_immediately`: An empty routine that returns immediately.
- `update_hardware_state`: Checks hardware register bits and updates several global variables and memory locations accordingly.
- `return_from_subroutine_3`: The routine is empty and performs no operations before returning.
- `loop_until_zero`: Repeatedly calls another routine as long as a global variable is non-zero.
- `clear_global_variable_3`: Sets a global variable to zero and repeatedly checks a byte in memory until it is non-zero before clearing it again.
- `clear_memory_and_global`: Stores a zero byte at an offset from a pointer in a register and sets a global variable to zero.
- `call_indirect_pointer`: Loads a pointer from a global variable and executes the routine at that address.
- `copy_block_of_data`: Copies a block of data from a source to a destination using a loop with a fixed count.
- `execute_recursive_calls`: This routine performs a series of self-referential calls to itself before returning.
- `update_global_from_trap`: Executes a trap and either updates a global variable with the resulting value or increments the existing global variable.
- `increment_global_variable`: Increments a global variable by one.
- `increment_global_on_negative`: Executes a trap and increments a global variable only if the resulting value is negative.
- `increment_global_variable_2`: Adds one to a global variable.
- `increment_global_on_negative_alt`: Executes a trap and increments a global variable if the returned value is less than zero.
- `increment_global_variable_3`: Performs an increment operation on a global variable.
- `loop_and_copy_data`: Enters an infinite loop after copying data from a source to a destination and storing the iteration count in a global variable.
- `copy_and_rotate_bitmaps`: Copies data blocks while counting iterations and performs bitwise circular rotations and inversions on bitmap data.
- `rotate_bitmaps`: Performs a bitwise circular rotation on blocks of data.
- `copy_and_count_blocks`: Copies multiple non-contiguous words from a source block to a destination block and repeats this for a specified number of blocks.
- `copy_data_with_limit`: Copies data from a source to a destination and stores the iteration count in a global variable up to a limit.
- `copy_memory_blocks`: Copies data from one memory location to another using nested loops and specific offsets.
- `rotate_and_mask_bitmaps`: Performs a bitwise circular rotation on blocks of data and calculates their bitwise inverse.
- `reconstruct_bitstream_buffer`: Reassembles a bitstream from a source buffer into a destination buffer using variable bit-shifts.
- `bit_stream_reconstruct_and_mask`: Reconstructs a bitstream from a source buffer using a variable shift amount, applies a bitwise inversion across a target block, and performs a bitwise AND/OR mask operation to update a destination buffer.
- `assign_global_values`: Sets a global variable to specific constant values based on a bit test of a register.
- `update_global_pointer`: Traverses a series of pointers stored in a global variable and performs arithmetic on a value before loading a new pointer from a table.
- `iterate_and_rotate_table`: Iterates through a table of pointers and calls another routine for each non-terminating entry.
- `initialize_data_copy_parameters`: Resets a global variable to zero and uses it as a pointer to store its own value into a memory location.
- `copy_structured_data_blocks_variant`: Copies data from a source to a destination while applying specific constant values to certain offsets within the destination structure.
- `transform_and_reformat_data_blocks`: Iterates through a source table to copy specific word fields into a destination table while applying constant masks and field resets.
- `copy_structured_data_blocks`: Performs a nested loop to copy specific word values from a source table to a destination table while modifying certain fields in the source.
- `copy_table_blocks`: Performs a nested loop to copy blocks of data from a source table to a destination table using word-sized increments.
- `check_pointer_and_loop`: Iterates through a memory location until a specific termination value is encountered or a counter reaches a limit.
- `nop_placeholder`: This routine is empty and performs no operations.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d2, then returns.
- `iterate_and_rotate_table_values`: Iterates through a table of 16-bit words until a terminator value is encountered, performing bitwise rotations and bitwise OR operations on a local variable based on comparisons of the table elements.
- `compare_and_branch_data`: Compares values from a table and performs conditional branching based on the results.
- `nop_padding_filler`: The routine is empty and performs no operations.
- `search_list_for_range_match`: Iterates through a list of four-word structures until it finds an entry where a global variable's doubled value is contained within the bounds defined by the second and fourth words of the structure, or until a terminator value is encountered.
- `update_global_value`: Compares a value in a register to a global variable and updates the global variable if they differ.
- `update_global_flags`: Modifies a global variable and another memory location based on the state of several individual bytes.
- `clamp_and_call_pointer`: Clamps a global variable between zero and two hundred fifty-five, then uses the value to call a function pointer.
- `update_clamped_values`: Updates two separate values by clamping them within specific ranges and recursively calls itself if the values change.
- `check_and_clear_flag`: Checks a global variable, clears it if it is non-zero, and then calls another routine.
- `synchronize_global_variable`: Reads a value from a global variable and writes it back to the same location.
- `execute_unnamed_sequence`: Performs a sequence of calls and register assignments without modifying global memory.
- `configure_dma_transfer`: Sets up memory addresses and control values in specific hardware registers and global variables to prepare for a DMA operation.
- `write_interrupt_registers`: Writes specific values to the interrupt enable and interrupt request hardware registers.
- `modify_global_state`: Performs bitwise operations on a global variable and updates another global variable based on a bit test.

### F-16 Combat Pilot (1989)(Digital Integration)[cr VF].adf bytes 116224-125952 (code never loaded by a reference run)

- `sort_and_reorder_data_list`: Identifies the minimum value within a list of data pairs and performs a series of shifts and reordering operations to reorganize the list.
- `compare_absolute_differences`: Calculates the absolute difference between two sets of values and compares them against a third difference.
- `add_repeatedly_until_limit`: Repeatedly adds a value to another in a loop and stores the resulting value in a memory location.
- `increment_value_conditionally`: Increments a value and stores it in a memory location if a specific comparison condition is met.
- `subtract_repeatedly_until_limit`: Repeatedly subtracts a value from another in a loop and stores the resulting value in a memory location.
- `update_memory_value`: Decrements a value and stores the result in a memory location if a condition is met.
- `apply_memory_mask`: Applies a bitwise AND mask to a range of memory words.
- `mask_memory_elements`: Applies bitwise AND masks to specific word elements within a memory structure using values from a table.
- `clear_and_mask_memory_blocks`: Clears a range of memory by zeroing out specific word offsets within a loop, then applies a bitwise AND mask to the start and end of the memory range.
- `mask_memory_descending`: Iterates through a memory range in descending order, applying a bitwise AND mask to four word offsets per entry.
- `mask_memory_values`: Updates four memory words by performing a bitwise AND with a value from a register.
- `clear_and_mask_memory_blocks_2`: Clears a range of memory by setting words to zero and then applies a bitwise AND mask to specific word offsets within those memory blocks.
- `mask_memory_ascending`: Iterates through a memory range in ascending order, applying a bitwise AND mask to four word offsets per entry.
- `mask_memory_word_elements`: Performs a bitwise AND operation between a word from a register and four specific word-sized elements located at fixed offsets within a memory structure pointed to by a register.
- `mask_four_memory_words`: Performs a bitwise AND operation between a register value and four specific words located at fixed offsets from a pointer, storing the results back into those memory locations.
- `return_from_subroutine`: Performs a single return instruction to exit the current execution flow.
- `compare_and_branch_values`: Compares two values and performs conditional branches based on their relative differences.
- `apply_bitwise_mask`: Performs bitwise AND operations on structured data using inverted values retrieved from a source table.
- `return_to_caller`: The routine is empty and performs no operations before returning to the calling routine.
- `apply_complex_mask`: Uses a loop to apply bitwise masks to memory words based on values retrieved from a table.
- `return_from_subroutine_2`: The routine is empty and performs no operations before returning to its caller.
- `update_memory_bitmask_tables`: Updates multiple sets of global memory locations with one of two possible 16-bit values based on the bit state of a register.
