# Game 399 (Mega-lo-mania): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00399). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1811424 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 73 of 73 named (0 author symbols), 73 of 73 described (2 by rule, 70 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 494 of 494 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1473 of 1473 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 75441 |
| code | 5590 |
| data | 1724761 |
| unused | 5632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf:bootblock | 1 | 37 | 2 |
| Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 2 of 2).adf:bootblock | 1 | 37 | 1 |
| logged disk load from Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf boot-block load | 1 | 353 | 21 |
| logged disk load from Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 402 | 19 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 465 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 14 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 165 | 8 |

## Cases handled

- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 1 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1748 non-empty blocks: matched against track-loaded segments
- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 2 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 1 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf: the boot block reads 2560 bytes from disk offset 898560 to an address (found statically); recovered as a unit
- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 1 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf: the boot block reads 73728 bytes from disk offset 712192 to an address (found statically); recovered as a unit
- runtime segment an address: 166 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 227 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 96 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 1 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf: 2 logged loads cover 76288 bytes of the image
- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 1 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf: blocks 2-5 (bytes 1024-3072) outside the filesystem hold data, not code (opcode density 0.024, entropy 3.75)
- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 1 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf: blocks 13-1390 (bytes 6656-712192) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.39)
- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 1 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf: blocks 1535-1684 (bytes 785920-862720) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.33)
- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 1 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf: blocks 1687-1687 (bytes 863744-864256) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 1 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf: blocks 1689-1754 (bytes 864768-898560) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.46)
- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 2 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 2 of 2).adf: blocks 2-4 (bytes 1024-2560) outside the filesystem hold data, not code (opcode density 0.030, entropy 3.79)
- mega-lo-mania (1991)(imageworks)(de)[cr subway](disk 2 of 2)/Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 2 of 2).adf: blocks 6-1759 (bytes 3072-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.71)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_dma_and_clear_buffer` (Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf boot-block load +an address to an address (2560 bytes)): Configures DMA parameters by writing values to custom hardware registers, clears a memory buffer using a loop, and then enters a nested loop structure that calls decrement_and_restore and another routine to process data into a destination pointer while managing several counters and hardware register states. Kind: disk hardware/track check. Pass path: The loop continues and eventually reaches the return statement after the `_DAT_00000254` counter hits zero, indicating successful data loading/verification..

## Functions


### Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf:bootblock

- `initialize_hardware_and_load`: Sets several hardware registers including DMA and color registers, performs a loop based on a value from a hardware register, and calls two different routines using a library function and a long address.
- `disable_interrupts_and_jump`: Disables interrupts by writing to the status register and jumps to a long address.

### Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 2 of 2).adf:bootblock

- `initialize_memory_and_copy_data`: Clears a block of memory from a starting address to a limit, then iterates through a table to copy bytes from a source to a destination based on values found in the table.

### logged disk load from Mega-lo-Mania (1991)(ImageWorks)(De)[cr Subway](Disk 1 of 2).adf boot-block load

- `initialize_hardware_and_unpack`: Clears a memory range from 50000 to 60000, calls a library routine via a pointer, configures various hardware registers including bitplanes and copper registers, calls another routine, and then performs a loop to copy 209 words from a source to address 70000 using a routine that processes a byte stream.
- `process_byte_stream`: Loads a byte from a table and calls another routine if the byte is neither zero nor negative one, then either increments a pointer by one or by 960 depending on the branch taken.
- `copy_data_with_stride`: Copies 8 bytes from a source table to a destination buffer, using a loop that increments the source pointer by 78 and the destination pointer by 80 each iteration.
- `copy_data_blocks`: Calls a routine to save registers, copies a block of data from a source to address 4000 until a limit is reached, then copies 96 words from a second source to address 100 and writes a value to a global variable before jumping to an address in a table.
- `save_registers_and_branch`: Saves registers d0 through d7 and a0 through a5 onto the stack, clears d6 and d4, and branches to another routine.
- `configure_dma_and_clear_buffer`: Configures DMA parameters by writing values to custom hardware registers, clears a memory buffer using a loop, and then enters a nested loop structure that calls decrement_and_restore and another routine to process data into a destination pointer while managing several counters and hardware register states.
- `decrement_and_restore`: Decrements a 16-bit value at a specific offset in a global variable and, if not zero, branches to another routine; otherwise, it restores registers from the stack and returns.
- `clear_memory_at_pointer`: Writes a 16-bit value to a location pointed to by a6, checks if the value at a0 is zero, and if not, clears the longword at a0 and increments the pointer before branching.
- `process_data_stream`: Initializes a structure in a global variable using a0 and a4, enters a loop that clears words at a pointer and checks for a specific 16-bit value, calls another routine multiple times within a loop, and if a comparison between a byte from a global variable and a swapped value in d2 matches, it calls another routine and decrements a global variable before looping or returning.
- `calculate_offset_address`: Performs a bitwise rotation on d2, compares it to d4, and if they match, calculates an address based on d2 and a constant, calls another routine, and returns with a pointer in a2.
- `decode_mfm_longword_pair`: Waits until a data word is nonzero, then takes it together with a second word, clears both source words and the two output longwords, and combines the masked bits of the two values, with the second shifted up one position, into one result.
- `call_decode_routine`: Sets a2 to point to an offset of 4 from a0, calls another routine, and then moves the value of a2 into a0 before returning.
- `unpack_compressed_data_block`: Validates a data block header, copies a small set of values from a table to the stack, and then performs a bitstream decompression by reading bits to determine how many bytes to copy from a source pointer to a destination pointer.
- `decode_variable_length_bitstream`: Reads a sequence of bits from a bitstream to determine a value, where each zero bit triggers a fetch of a new byte from a pointer in a3 to continue the calculation, and finally sets d4 and d0 to specific constants before branching.
- `parse_bitstream_header`: Reads a sequence of bits from a bitstream to calculate a value for a register and updates a counter in a register by adding 6.
- `set_d4_d0_and_branch`: Sets d4 to 5, d0 to 3, and branches to another routine.
- `set_d4_d0_and_branch_2`: Sets d4 to 4, d0 to 2, and branches to another routine.
- `set_d4_d0_and_branch_3`: Sets d4 to 3, d0 to 1, and branches to another routine.
- `decompress_bitstream_data_2_2`: Reads a sequence of bit-counts and bit-lengths from a bitstream to perform a dictionary-based decompression, expanding data into a destination buffer.
- `decompress_bitstream_data_2`: Reads a sequence of bit-counts and bit-lengths from a bitstream to perform a dictionary-based decompression, expanding data into a destination buffer using a combination of literal byte copies and previously decompressed data segments.
- `decompress_bitstream_data`: Reads a length and a bit-count from a table to perform bitstream decompression, using bit-shifting and carry-based logic to expand data into a destination buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_registers_from_stack`: Sets the low byte of a data register to sixteen and restores several data and address registers from the stack before returning.
- `calculate_scaled_values`: Performs three separate calculations where it extracts bytes from a source, multiplies them by a value in a register, divides by 100, and accumulates the results into a destination register with specific bit shifts.
- `store_registers_to_globals`: Stores the contents of registers a1 and a2, and two 16-bit words from a structure pointed to by a0, into four consecutive global variables.
- `copy_structured_data_loop`: Iterates through a source structure to copy 16-bit words into two destination buffers, using an offset and a count provided within the source structure.
- `write_constant_table`: Writes the constant value 522e into two different locations in memory, specifically at offsets 6 and 2 of two different structures.
- `process_input_and_timers`: Checks a hardware bit, increments a global counter, and conditionally calls several routines, performs divisions, and updates various global variables and hardware registers.
- `process_interrupt_status_and_call_handler`: Reads the interrupt status register, masks bits 7, 8, and 9, and if bit 7 is set, calls another routine with the status value and a pointer from a table.
- `branch_on_bit_test`: Tests bit 8 of d0; if set, it loads the address of the custom chip into a0, loads a value into a1, pushes d0 to the stack, calls another routine, restores d0 from the stack, and branches to another location; otherwise it branches to another location.
- `call_on_bit_test`: Tests bit 9 of a register; if set, it loads two addresses into registers and calls another routine, otherwise it branches to a different location.
- `call_subroutine_on_bit_test`: Tests bit 10 of a register; if set, it calls another routine using a pointer from a table and then returns.
- `process_joystick_input`: Reads joystick data, calculates differences to update two 16-bit counters, and adjusts a third counter based on a conditional check of a global variable.
- `update_input_state`: Checks CIAA and custom hardware bits to update several status bytes in a table, and increments a 16-bit counter in a global variable while conditionally setting bytes at offsets 38, 39, 40, and 42 based on the counter's value.
- `write_long_constant`: Writes the constant 26064 to a longword at offset 20 of a global variable and performs a division of register d7 by zero.
- `initialize_system_state`: Disables interrupts, calls two routines, fills two memory buffers with specific patterns using a loop, clears several global variables, updates hardware registers, and calls multiple routines before jumping to another routine.
- `process_conditional_calls`: Checks three 16-bit global variables to decide whether to call another routine or perform a calculation involving two pointers and two 16-bit values, then calls another routine with a modified d3 register.
- `get_word_from_table`: Copies a 16-bit word from a global variable into d0 and returns.
- `set_exception_vectors`: If a bit in a global variable is set, it writes two 32-bit values into specific memory locations and branches to another routine.
- `write_exception_vectors`: Writes two 32-bit values into specific memory locations and returns.
- `reset_input_buffers`: If a 32-bit global variable is zero, it copies three bytes from one set of global variables to another and clears the original three bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_word`: Clears a 16-bit global variable and returns.
- `decrement_global_counter`: If a 16-bit global variable is non-zero, it decrements a 16-bit global variable and returns.
- `update_audio_volumes`: Multiplies four different values by a value from a global variable, shifts the results, and stores them in the four audio volume hardware registers.
- `reset_audio_volumes`: Stores a pointer from a0 into a global variable, writes 64 to a global variable, calls another routine, sets a global variable to 15, clears four consecutive audio volume registers to 0, and returns.
- `update_pot_logic`: Calls another routine, checks bit 10 of the POTGOR register, and if it is clear, decrements a byte in a global variable at offset 58998; if that byte reaches zero, it sets it to 8 and decrements a byte in a global variable at offset 58984, which if it reaches zero, sets it to 2 before returning.
- `tiny_andi_rts`: Small routine of 2 instructions that masks d0 with 0, then returns.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `apply_zero_or_logic`: Performs a bitwise OR with zero on d0 using both a word and a byte instruction.
- `apply_zero_byte_logic`: Performs two consecutive bitwise OR operations with zero on d0 using byte instructions.
- `zero_data_padding`: This routine consists of two identical instructions that perform a logical OR operation with zero on a data register.
- `perform_logical_or`: Performs two bitwise logical OR operations on register d0 with zero.
- `clear_data_register_zero`: Performs a logical OR operation on the least significant byte of data register zero with zero twice.
- `clears_data_register_d0`: Performs two consecutive logical OR operations with zero on the data register d0.
- `zero_data_padding_2`: This routine performs two identical logical OR operations with zero on a data register.
- `clear_audio_and_tables`: Uses a loop to iterate through a table, clears memory at calculated offsets, and then clears four audio volume registers and several global variables.
- `reset_audio_hardware`: Clears four audio volume registers, four global variables, and sets a hardware register to the value 15.
- `update_audio_parameters`: Increments a byte in a global variable, and if it is not less than another global variable, it performs a series of calls to update audio registers using data from a structure and a table.
- `update_audio_entity_parameters`: Reads an entity structure from a pointer, extracts an index from combined nibbles of two bytes, and uses that index to fetch audio properties from a table into the structure.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_entity_property_structure`: Reads a 16-bit word from an offset of 2 in a structure pointed to by a6, masks the lower 12 bits, and branches based on the value of the lower 4 bits of that word.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `test_bit_and_return`: Tests bit 3 of register d3 using register d1 and then returns.
- `increment_pointer_table`: Checks two global words; if both are non-zero and the byte at the address pointed to by a global variable is positive, it copies that byte to another global variable, increments the pointer in the global variable, and returns; otherwise it returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_logic_loop`: Calls several routines including one that checks a bit in a POT register and another that performs table population, and if a specific condition is met, sets a global variable to 128 and 0 before returning.
- `execute_conditional_logic`: Decrements d0, compares a word from a global variable with itself, calls a routine at an address in a global variable, and then checks two bytes at specific global offsets before looping via dbra.
- `initialize_data_tables`: Runs a loop 15 times to store zero words into two tables, then updates several global pointers and longwords, calls another routine twice, and finally writes a value to a hardware register and a pointer to a hardware register.
- `write_word_sequence`: Runs a loop four times, storing the lower and upper words of register d0 into a table starting at a constant address, while adding the value in d7 to d0 in each iteration.
- `write_offset_word_sequence`: Runs a loop four times, storing the lower and upper words of the sum of d0 and d6 into a table starting at a constant address, while adding the value in d7 to that sum in each iteration.
- `populate_tables_loop`: Runs a loop 31 times to store values from d3 into two tables by calling another routine, then decrements a counter in d2 and loops if it remains positive.
- `populate_tables_until_limit`: Runs a loop 31 times to store values from d3 into two tables by calling another routine, then increments a counter in d2 and loops if it is less than or equal to 100.
- `update_hardware_registers`: Sets bits in register d0, clears a bit in a memory location at offset 386 of a6, moves a word from d2 to offset 390 of a6, and sets a bit in a memory location at offset 392 of a5.
