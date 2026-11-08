# Game 377 (R-Type II): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00377). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (914560 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 102 of 102 named (0 author symbols), 102 of 102 described (6 by rule, 86 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1461 of 1463 (99.86%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2430 of 2430 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 218147 |
| code | 8124 |
| data | 679585 |
| unused | 8704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| R-Type II.adf:bootblock | 1 | 61 | 4 |
| logged disk load from R-Type II.adf boot-block load | 1 | 92 | 2 |
| logged disk load from R-Type II.adf boot-block load | 1 | 0 | 0 |
| logged disk load from R-Type II.adf boot-block load | 1 | 116 | 2 |
| logged disk load from R-Type II.adf boot-block load | 1 | 99 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 106 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 544 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 873 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 535 | 5 |

## Cases handled

- R-Type II/R-Type II.adf: custom-format (non-DOS) disk, 1626 non-empty blocks: matched against track-loaded segments
- R-Type II/R-Type II.adf: the boot block reads 18944 bytes from disk offset 822272 to an address (found statically); recovered as a unit
- R-Type II/R-Type II.adf: the boot block reads 56320 bytes from disk offset 844800 to an address (found statically); recovered as a unit
- R-Type II/R-Type II.adf: the boot block reads 147456 bytes from disk offset 1536 to an address (found statically); recovered as a unit
- R-Type II/R-Type II.adf: the boot block reads 5120 bytes from disk offset 895488 to an address (found statically); recovered as a unit
- runtime segment an address: 84 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 403 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 586 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 384 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- R-Type II/R-Type II.adf: 4 logged loads cover 222720 bytes of the image
- R-Type II/R-Type II.adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.004, entropy 1.99)
- R-Type II/R-Type II.adf: blocks 755-762 (bytes 386560-390656) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.73)
- R-Type II/R-Type II.adf: blocks 764-1436 (bytes 391168-735744) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.84)
- R-Type II/R-Type II.adf: blocks 1439-1439 (bytes 736768-737280) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.12)
- R-Type II/R-Type II.adf: blocks 1444-1445 (bytes 739328-740352) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.37)
- R-Type II/R-Type II.adf: blocks 1448-1605 (bytes 741376-822272) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.71)
- 1 packed track region(s), 237056 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### R-Type II.adf:bootblock

- `read_disk_blocks_via_doio`: Issues a sequence of disk requests through exec DoIO into several memory destinations, including a request with command nine and zero length between reads, then calls a fixed routine and writes the colour and interrupt registers; the decompiler reports bad instruction data at the end, so the tail is unreliable.
- `wait_until_flag_clear`: Busy-waits while bit six of a global is set, then returns; the listed facts also name the DMA control read register as touched.
- `read_payload_then_relocate_code`: Sends a read request through DoIO for five kilobytes from a disk offset into a destination, then copies a 33-byte block of its own code into low memory.
- `execute_trap_zero`: Invokes trap zero and returns the combined 64-bit value of the D0 and D1 registers.

### logged disk load from R-Type II.adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from R-Type II.adf boot-block load

- `select_half_or_input`: Returns the input word unchanged unless the D7 word equals one, in which case it returns the D6 word shifted right by one bit, so half its value.
- `lookup_masked_table_word`: Reads a word from a table indexed by twice the D1 word, ANDs it with the D6 word, and returns the result.

### logged disk load from R-Type II.adf boot-block load

- `fetch_bitstream_longword`: Reads a longword just before the current input pointer, stores its low byte into an output word, and returns the value shifted right one bit with the top bit set, the pattern of a bit-stream reader.
- `shift_bits_with_refill`: Shifts a bit buffer right once per count taken from the D1 register, refilling the buffer from another routine whenever it becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_fields`: Copies two 16-bit words from a source to offsets 12 and 16 of a structure in a5, clears the 16-bit words at offsets 14 and 18, and returns.
- `update_partial_structure`: Copies a 16-bit word from a source to offset 12 of a structure in a5 and clears the 16-bit word at offset 14 before returning.
- `copy_long_words`: Copies two 32-bit words from a source to offsets 20 and 24 of a structure in a5 and returns.
- `copy_single_long_word`: Copies a 32-bit word from a source to offset 20 of a structure in a5 and returns.
- `copy_long_words_offset_3c`: Copies two 32-bit words from a source to offsets 28 and 32 of a structure in a5 and returns.
- `copy_long_word_offset_3c`: Copies a 32-bit word from a source to offset 28 of a structure in a5 and returns.
- `copy_word_offset_4`: Copies a 16-bit word from a source to offset 4 of a structure in a5 and returns.
- `copy_word_offset_6`: Copies a 16-bit word from a source to offset 6 of a structure in a5 and returns.
- `calculate_pointer_offset`: Reads a 16-bit word from a source, adds it to the address in a0, and stores the resulting 32-bit address at offset 48 of a structure in a5.
- `calculate_pointer_and_set_bit`: Clears a 16-bit word at offset 78 of a structure in a5, calculates a 32-bit address by adding a 16-bit word from a source to the address in a0, stores that address at offset 74 of the structure, and sets bit 2 of d0.
- `copy_two_words`: Copies two consecutive words from the source pointed to by a0 to offsets 176 and 178 of the structure in a5 and returns.
- `copy_four_words`: Copies four consecutive words from the source pointed to by a0 to offsets 194, 196, 198, and 200 of the structure in a5 and returns.
- `tiny_or_rts`: Small routine of 2 instructions that merges (a0)+ into d0, then returns.
- `tiny_and_rts`: Small routine of 2 instructions that masks d0 with (a0)+, then returns.
- `call_indirect_via_offset`: Reads a word from a0, adds it to the address in a0 to calculate a target address, and calls the routine at that address.
- `store_word_and_pointer`: Stores a word from the source in a0 at offset 58 of the structure in a5, then stores the following long word from a0 at offset 60 of the structure in a5.
- `decrement_and_check_counter`: Decrements a 16-bit word at offset 58 of the structure in a5 and returns; if the value is not zero, it loads a long word from offset 60 of the structure in a5 into a0 and returns.
- `tiny_move_adda_rts`: Small routine of 3 instructions that copies (a0)+ to d1, then adds d1 to a0, then returns.
- `tiny_moveq_moveq_rts`: Small routine of 3 instructions that loads 0 into d0, then loads 1 into d7, then returns.
- `call_and_set_byte_thirteen`: Calls another routine and then stores the value 13 at offset 182 of the structure in a5.
- `call_and_set_byte_fifteen`: Calls another routine and then stores the value 15 at offset 182 of the structure in a5.
- `call_and_set_byte_twentythree`: Calls another routine and then stores the value 23 at offset 182 of the structure in a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_global_word_one`: Sets a 16-bit word at a global variable location to 1.
- `clear_global_word`: Clears a 16-bit word at a global variable location.
- `initialize_global_parameters`: Sets several global variables to specific values, including a 16-bit value, two 32-bit values, and a byte at a specific offset.
- `update_structure_values`: Checks a 16-bit value at offset 72 of a structure in a6; if it is 1, it increments a 16-bit value at offset 60 if bit 0 of the 16-bit value at offset 16 is set, otherwise it adds a bit-shifted version of the value at offset 72 to the value at offset 60.
- `execute_function_table_loop`: Loads a pointer from a structure in a6, then enters a loop that calls functions from a table using an index derived from the pointer, updating the pointer with the return value of each call.
- `store_two_words`: Copies two consecutive words from the location pointed to by a0 into offsets 48 and 4a of the stack frame in a6.
- `store_two_words_offset`: Copies two consecutive words from the location pointed to by a0 into offsets 4c and 4e of the stack frame in a6.
- `process_joystick_input`: Calculates a value based on a global variable and a byte at offset 0e of a structure, modifies bit 6 of that value based on a byte at offset 0d, and increments a pointer at offset 20 of the structure while checking a limit at offset 24.
- `copy_large_data_blocks`: Calls two other routines, then copies a block of 444 bytes from a source to a destination, followed by a loop that copies 8000 sets of four words from a source to four different destination memory areas.
- `populate_rotation_lookup_tables`: Calls a routine to generate rotated tables three times, then performs several loops to copy data from various source locations into multiple destination memory buffers of varying sizes.
- `update_status_flag`: Checks a word at offset 7e of a structure and sets a global variable to either zero or 3327 to a value determined by bit 3 of a word at offset 10 of the structure.
- `copy_parameters_to_global`: Loads a pointer from a global address, then copies a longword from offset 4 of the stack and two words from offsets 40 and 42 of the stack into that memory location.
- `copy_parameters_from_stack`: Loads a pointer from a global address, then copies a longword from the stack, a word from offset 10 of the stack, and a word from offset 42 of the stack into that memory location.
- `update_object_attributes`: Performs a bitwise rotation on a value in d0, then enters a loop that checks bits in the rotated value to copy eight bytes from a table into specific offsets of a structure pointed to by a0.
- `adjust_pointer_and_branch`: Decrements the address in a1 by 1 and then branches to another location.
- `process_byte_triplet`: Loads three consecutive bytes from a buffer in a0 into d0, d1, and d2, calls another routine with a pointer from a1 on the stack, and enters a loop that continues as long as the byte at the current a0 position is positive.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `process_byte_stream`: Calculates a 16-bit value based on d0, d1, and a1, then enters a loop that iterates through a buffer in a0.
- `execute_unnamed_function_repeatedly`: Calls a function pointer stored in a register eight times in succession.
- `update_structure_fields_variant_1`: Stores the byte from register d1 at the address in a1, then stores the byte from register d0 at offsets 40, 80, and 120 of that same address.
- `update_structure_fields_variant_2`: Stores the byte from register d0 at the address in a1, then stores the byte from register d1 at offset 40, and stores the byte from register d0 at offsets 80 and 120 of that same address.
- `update_structure_fields_variant_3`: Stores the byte from register d1 at the address in a1 and at offset 40, then stores the byte from register d0 at offsets 80 and 120 of that same address.
- `update_structure_fields_variant_4`: Stores the byte from register d1 at the address in a1 and at offsets 40 and 80, then stores the byte from register d0 at offset 120 of that same address.
- `update_structure_fields_variant_5`: Stores the byte from register d0 at the address in a1 and at offsets 40 and 80, then stores the byte from register d1 at offset 120 of that same address.
- `update_structure_fields_variant_6`: Stores the byte from register d1 at the address in a1 and at offset 120, then stores the byte from register d0 at offsets 40 and 80 of that same address.
- `bitwise_or_multiple_buffers`: Performs a loop 8000 times, bitwise ORing four 16-bit values from four different memory locations and storing the result in a fifth location.
- `generate_rotated_tables`: Calls another routine three times, then performs three loops.
- `process_and_rotate_bit_data`: Initializes a destination table with a constant, then performs a loop that concatenates 16-bit values from two source tables, performs a bitwise left rotation on them, and stores them in a third table at specific intervals.
- `initialize_sprite_data_tables`: Clears several memory blocks, performs a nested loop to expand a small data table into a larger structure, and then populates a long array with values from a series of global variables based on a count retrieved from a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_to_memory_with_offset`: Calculates a memory address based on a value in a structure, then enters a loop that copies 32-bit values from that address into a destination table at offset 66524 using 16-bit writes.
- `calculate_and_store_pointer_array`: Calculates a base address by adding a scaled value derived from a 4-bit field at offset 66 of a structure in a6 to the structure's base address, then stores the high and low 16-bit words of four subsequent offsets into global variables.
- `update_hardware_and_globals`: Writes values to hardware registers, modifies a global variable using bitwise operations, stores a pointer to a global variable into a structure at offset 128, and sets a global variable to 65280.
- `update_status_flags`: If a global variable is non-zero, it calls another routine.
- `decrement_nibble_packed_data`: Decrements a counter in a global variable and, if it reaches zero, enters a loop to decrement values within a table.
- `copy_long_words_2`: Copies 15 long words from a source pointed to by a0 to a destination pointed to by a global variable using a loop.
- `initialize_structure`: Writes several values to a structure at a location relative to the program counter, including a 16-bit value from d0, a byte value of 3, and a word value of 15.
- `initialize_structure_variant`: Writes a long word from a0, a 16-bit value from d0, and several other values to a structure at a location relative to the program counter.
- `check_bit_status`: Checks if bit 0 is set in a byte at offset 4 of a structure relative to the program counter.
- `compare_long_word`: Compares a long word at offset 8 of a structure in a6 with itself and returns if they are equal, otherwise it returns.
- `perform_bit_manipulation`: Performs a series of bitwise operations and shifts on the contents of d0 and returns a 32-bit result.
- `test_and_modify_hardware`: Pushes a0 and d0 onto the stack, writes 8 to a hardware register, and tests bit 3 of a value in d0.
- `update_hardware_registers`: Clears a byte at offset 3072 of a structure in a0, modifies a byte in a register via a bitwise NOT and ROR, and writes to the custom hardware register and the interrupt request register before returning from an interrupt.
- `branch_to_other_routine`: Performs a short branch to another routine.
- `iterate_sprite_data_list`: Sets bit 15 of a 32-bit value at offset minus 4 from a pointer in a0, then enters a loop that calls process_sprite_tile_data while incrementing a pointer in a2 by 8 and decrementing a 16-bit value read from offset 2 of the structure in a2 until that value reaches minus 1.
- `process_sprite_tile_data`: Calculates a pointer to a data table using a structure offset and a register, then iterates through a list of entries if a count is negative.
- `execute_indirect_jump`: Performs several conditional checks on a 32-bit value in d4 and a 16-bit value at offset 32 of a structure in a5, then calls a routine via an indirect jump.
- `setup_blt_parameters_a`: Sets several values in a table and calls another routine three times to write data to three different structures using values from registers and the stack.
- `setup_blt_parameters_b`: Sets several values in a table with a different constant than R8 and calls another routine three times to write data to three different structures.
- `setup_blt_parameters_c`: Sets several values in a table and calls another routine three times to write data to three different structures using a specific combination of register and stack values.
- `update_structure_data`: Sets several values in a global structure and calls another routine three times, storing the returned value in a 16-bit field at offset 58 of the structure each time.
- `populate_structure_fields`: Writes multiple values to a global structure and calls another routine three times, storing the returned value in a 16-bit field at offset 58 and writing 32-bit values to offsets 48, 4c, 50, and 54.
- `write_structure_parameters`: Writes several values to a global structure and calls another routine three times, storing the returned value in a 16-bit field at offset 58 and writing 32-bit values to offsets 4c, 50, and 54.
- `update_structure_offsets`: Sets several values in a global structure and calls another routine three times, storing the returned value in a 16-bit field at offset 58 and writing 32-bit values to offsets 50 and 54.
- `calculate_structure_indices`: Writes several values to a global structure and calculates two 32-bit values by multiplying registers by 3 and adding others, storing the results at offsets 4c, 50, and 54.
- `copy_data_blocks`: Performs bitwise operations and conditional checks on data pointed to by a5, calls a routine to check conditional bits, calls another routine that decrements a 16-bit register from 3 to -1, and copies bytes between a pointer derived from a register and a global variable in three separate loops.
- `process_bit_stream_conditional_logic`: Performs bitwise operations and conditional checks on data pointed to by a5, using check_conditional_bits to determine control flow.
- `decrement_counter_loop`: Sets a 16-bit register to 3 and enters a loop that decrements the value until it reaches -1 before returning.
- `check_conditional_bits`: Returns immediately if the 31st bit of a 32-bit value is set or if the 0th bit of a 16-bit value is clear; otherwise it returns.
- `process_bit_stream`: Decrements a 16-bit counter in d0 and performs bitwise shifts and logical operations on data pointed to by a5, calling check_conditional_bits and decrement_counter_loop during the process.
- `branch_to_offset`: Performs a short branch to a specific offset.
- `branch_to_offset_2`: Performs a short branch to a different specific offset.
- `call_func_00000504`: Calls another routine and returns.
- `call_subroutine`: Calls another routine and then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_bit_fields`: Performs three bit changes on a byte in register d0 using offsets relative to a0.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_state`: Sets several global variables to zero or all ones and modifies a global variable by performing a bitwise OR with 2.
- `update_sprite_dma_parameters`: Iterates through a loop to update sprite data structures by reading from a table and writing to memory, including setting up DMA controller registers for hardware sprite or bitplane transfers.
- `update_audio_and_hardware_state`: Copies twelve bytes from a table to a destination buffer using a loop, then enters a loop that clears specific offsets within a structure, sets a byte to one, sets another to two hundred and fifty-five, and copies a byte from offset one hundred and twelve to offset sixteen, before calling another routine.
- `update_audio_and_dma_state`: Iterates through a loop to update audio volume registers and DMA control settings based on values retrieved from a table.
- `update_index_from_table`: Increments a byte at offset 16 of a structure in a1, then enters a loop that uses that byte to index into a table.
