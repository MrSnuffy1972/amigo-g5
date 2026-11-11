# Game 635 (Rick Dangerous): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00635). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (924288 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 121 raw-word lines |
| Functions named and described | pass, 190 of 190 named (0 author symbols), 190 of 190 described (13 by rule, 177 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2531 of 2532 (99.96%) |
| Recovered without load address (counted in proof 1, flagged) | 11264 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4542 of 4555 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20248 |
| code | 28410 |
| data | 630162 |
| padding | 732 |
| unused | 244736 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rick Dangerous.adf:bootblock | 1 | 56 | 1 |
| logged disk load from Rick Dangerous.adf boot-block load | 1 | 205 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 294 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 195 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3768 | 156 |
| Rick Dangerous.adf bytes 212992-214016 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rick Dangerous.adf bytes 221696-222720 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rick Dangerous.adf bytes 223744-224768 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rick Dangerous.adf bytes 226304-226816 (code never loaded by a reference run) | 1 | 0 | 0 |
| Rick Dangerous.adf bytes 228352-228864 (code never loaded by a reference run) | 1 | 24 | 1 |
| Rick Dangerous.adf blocks 2-15 (bytes 1024-8192) (code never loaded by a reference run, load address unknown) | 1 | 0 | 0 |

## Cases handled

- Rick Dangerous/Rick Dangerous.adf: custom-format (non-DOS) disk, 1280 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run play-long-joy slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 98304 of its 121382 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-keys slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 98304 of its 121382 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-long-mixed slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 98304 of its 121382 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- Rick Dangerous/Rick Dangerous.adf: the boot block reads 14848 bytes from disk offset 522240 to an address (found statically); recovered as a unit
- runtime segment an address (play-rs-m1500): 251 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 128 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2152 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Rick Dangerous/Rick Dangerous.adf: bytes 212992-214016 look like code no run loaded: recovered position-independently (load address unknown)
- Rick Dangerous/Rick Dangerous.adf: bytes 221696-222720 look like code no run loaded: recovered position-independently (load address unknown)
- Rick Dangerous/Rick Dangerous.adf: bytes 223744-224768 look like code no run loaded: recovered position-independently (load address unknown)
- Rick Dangerous/Rick Dangerous.adf: bytes 226304-226816 look like code no run loaded: recovered position-independently (load address unknown)
- Rick Dangerous/Rick Dangerous.adf: bytes 228352-228864 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying code at an address (48 bytes change between runtime segment an address (play-rs-m1500) and runtime segment an address (play-long-joy)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (38 bytes change between runtime segment an address (play-long-joy) and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- Rick Dangerous/Rick Dangerous.adf: 6 logged loads cover 18944 bytes of the image
- Rick Dangerous/Rick Dangerous.adf: 24 track-loaded blocks found in recovered runtime segments and classified through them
- Rick Dangerous.adf blocks 2-15 (bytes 1024-8192) (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 0 of 7168 bytes decode)
- Rick Dangerous/Rick Dangerous.adf: blocks 17-17 (bytes 8704-9216) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.50)
- Rick Dangerous/Rick Dangerous.adf: blocks 20-186 (bytes 10240-95744) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.29)
- Rick Dangerous/Rick Dangerous.adf: blocks 188-327 (bytes 96256-167936) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.60)
- Rick Dangerous/Rick Dangerous.adf: blocks 357-364 (bytes 182784-186880) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.77)
- Rick Dangerous/Rick Dangerous.adf: blocks 367-374 (bytes 187904-192000) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.65)
- Rick Dangerous/Rick Dangerous.adf: blocks 377-384 (bytes 193024-197120) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.86)
- Rick Dangerous/Rick Dangerous.adf: blocks 386-414 (bytes 197632-212480) outside the filesystem hold data, not code (opcode density 0.020, entropy 5.35)
- Rick Dangerous/Rick Dangerous.adf: blocks 423-423 (bytes 216576-217088) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.23)
- Rick Dangerous/Rick Dangerous.adf: blocks 426-428 (bytes 218112-219648) outside the filesystem hold data, not code (opcode density 0.034, entropy 0.76)
- Rick Dangerous/Rick Dangerous.adf: blocks 431-431 (bytes 220672-221184) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.32)
- Rick Dangerous/Rick Dangerous.adf: blocks 653-653 (bytes 334336-334848) outside the filesystem hold data, not code (opcode density 0.008, entropy 1.53)
- Rick Dangerous/Rick Dangerous.adf: blocks 843-907 (bytes 431616-464896) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.54)
- Rick Dangerous/Rick Dangerous.adf: blocks 915-915 (bytes 468480-468992) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 923-923 (bytes 472576-473088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 931-931 (bytes 476672-477184) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 939-939 (bytes 480768-481280) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 947-947 (bytes 484864-485376) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 955-955 (bytes 488960-489472) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 963-963 (bytes 493056-493568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 971-971 (bytes 497152-497664) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 979-979 (bytes 501248-501760) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 987-987 (bytes 505344-505856) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 990-1019 (bytes 506880-522240) hold 1 packed streams (RNC2: Rob Northen RNC2 Compressor) found by scanning, 93% of the bytes
- Rick Dangerous/Rick Dangerous.adf: blocks 1051-1051 (bytes 538112-538624) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1059-1059 (bytes 542208-542720) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1067-1067 (bytes 546304-546816) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1075-1075 (bytes 550400-550912) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1083-1083 (bytes 554496-555008) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1091-1091 (bytes 558592-559104) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1099-1099 (bytes 562688-563200) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1107-1107 (bytes 566784-567296) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1115-1115 (bytes 570880-571392) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1123-1123 (bytes 574976-575488) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1131-1131 (bytes 579072-579584) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1139-1139 (bytes 583168-583680) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1147-1147 (bytes 587264-587776) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1155-1155 (bytes 591360-591872) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1163-1163 (bytes 595456-595968) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1171-1171 (bytes 599552-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1179-1179 (bytes 603648-604160) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1187-1187 (bytes 607744-608256) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1195-1195 (bytes 611840-612352) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1203-1203 (bytes 615936-616448) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1211-1211 (bytes 620032-620544) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1219-1219 (bytes 624128-624640) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1227-1227 (bytes 628224-628736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1235-1235 (bytes 632320-632832) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- Rick Dangerous/Rick Dangerous.adf: blocks 1243-1759 (bytes 636416-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.87)
- 5 never-loaded region(s), 10752 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 98304 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Rick Dangerous.adf:bootblock

- `configure_io_and_loop_disk_read`: Sets parameters in a structure pointed to by a1, including a length of 65536, a source address of an address, and a destination address of 4096, then calls a library routine and enters a loop that repeats this process until a byte at offset 31 of the structure is non-zero.

### logged disk load from Rick Dangerous.adf boot-block load

- `copy_block_with_alignment_fixups`: Copies a block of data between buffers, adjusting the source and destination pointers to even addresses before copying runs of longwords.
- `initialize_data_buffers`: Loads a byte from an offset of 1 of a3 into d6, performs bitwise rotations and moves, calls three different routines with a2 as a pointer to various offsets, and then enters a loop that decrements d0 and branches back to a previous instruction.
- `decode_and_fill_table`: Reads a count from a routine, fills a stack buffer from another routine, then iterates over a stack buffer and a table, writing bit masks and derived values into table entries based on matching bytes.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `tiny_moveq_moveq`: Small routine of 2 instructions that loads 255 into d0, then loads 16 into d1.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `perform_arithmetic_on_registers`: Performs a series of arithmetic and bit manipulation operations on d1, d6, d7, and a3 before returning.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `decode_and_fill_table_2`: Calls another routine to retrieve a count, then fills a stack-based buffer with bytes from that routine.
- `build_and_execute_jump_table`: Copies ten words from a table to the stack and then jumps to the address stored at the top of the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_data_blocks`: Copies a block of data from a source pointer to two different destination memory areas using a loop.
- `reset_hardware_and_globals`: Sets several hardware registers and global variables to zero or specific constant values.
- `evaluate_table_and_branch`: Checks a value in a table via an index and either calls another routine or updates a global variable and a status flag.
- `reset_and_evaluate_table`: Calls a routine to reset hardware and globals, then performs a series of conditional calls based on the values of global variables and a table.
- `set_global_pointer`: Clears d2 and stores a long value into a global variable, then returns.
- `process_bitfield_flags`: Iterates through bits of a value to check specific flags and updates a global structure with values from a table based on those flags.
- `initialize_memory_tables`: Clears several memory ranges, populates a table with specific values derived from an index, and fills a sequence of bytes with a repeating pattern.
- `update_byte_from_table`: If a byte at a location relative to the program counter is non-zero, it loads three bytes from a table, combines them using bitwise OR, and stores the result at that location, otherwise it returns.
- `initialize_and_call_subroutines`: Writes a byte to the location pointed to by a0, then performs three consecutive sequences of loading a6 and a4 with relative addresses and calling another routine.
- `decode_compressed_data_stream`: Iterates through a data stream to decompress bytes into a destination buffer using a variable-length encoding scheme.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_values_to_structure_2`: Stores a byte from a register into a global variable at offset 32, stores a word from a pointer into a global variable at offset 21, and stores the incremented pointer into a global variable at offset 10.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fetch_stream_byte`: Stores a byte from d0 into offset 32 of a structure, fetches a byte from a pointer in a0 and stores it in offset 21 of a structure, updates a pointer at offset 10 of the structure with the incremented a0, and branches to another routine.
- `store_byte_and_pointer`: Stores a byte from a pointer in a0 into a structure at offset 21 of a4, then stores the incremented pointer in a0 into a structure at offset 10 of a4.
- `parse_command_stream`: Reads a byte from a buffer in a0 and performs conditional updates to a structure pointed to by a4 based on the byte value.
- `update_stream_state_and_fetch_byte`: Updates a structure containing bit-packed stream state and byte offsets, then fetches a byte from a source pointer to store in a destination structure while updating a pointer and a counter.
- `execute_sequential_calls`: Loads three different relative addresses into a6 and calls another routine three times.
- `update_system_state`: Disables interrupts and DMA, updates several values in a structure at a4, modifies hardware registers at a fixed address and a fixed address, and calls a routine via a pointer in a structure at a6.
- `step_and_clamp_counter`: Adds a byte from a source to a byte at an offset in a structure; if the result is negative or exceeds a value at offset 4 of the source, it clamps the counter and stores a handler address.
- `update_structure_pointer`: Adds a byte from a source to an offset in a structure, and if the result is negative, performs a comparison and potentially stores a new address at offset 20 of the structure.
- `store_address_to_global`: If a word at offset 12 of a structure in a6 is non-zero, it loads the address of a location relative to the program counter into a0 and stores it at offset 20 of the structure in a6, then returns.
- `update_memory_and_register`: Loads a byte from an offset of a5 into d0, adds d0 to a value at an offset of a6, and if the result is positive, performs several operations including clearing a byte at an offset of a6, moving a pointer to a4, writing 128 to a location at a4, and writing a value to a location at a4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_iterative_subroutine_calls`: Calls a routine twice, then enters a loop that calls the same routine seven times while incrementing a value in a register and adjusting a pointer in a0 by an offset of thirty.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d0.
- `zero_register_d0`: Performs a logical OR on register d0 with zero.
- `update_string_table`: Iterates through a two-dimensional data structure to perform character-based replacements and pattern matching.
- `redraw_cursor_and_name_buffer`: Converts the grid cursor position d2/d3 and the name-entry index d7 into screen coordinates and calls the draw routine three times to redraw the cursor, the name buffer and the current character.
- `test_bit_in_d0`: Tests a bit in register d0 using the value in register d2.
- `adjust_bcd_and_write`: Performs a binary-coded decimal adjustment on d2 using d0, stores a longword into a global variable, and then stores the byte eighty-nine into another global variable, then returns.
- `handle_serial_interrupt`: Saves d0, reads a byte from a global variable, tests bit 3, and if set, performs bitwise operations on a byte from another global variable, stores the result in a third global variable, writes specific bytes to three global variables, restores d0, writes 8 to a hardware register, and returns from interrupt.
- `return_from_interrupt`: If bit 0 of d0 is set, it writes the value 8 to a global variable, then writes 8 to a global variable and a hardware register before returning from an interrupt.
- `process_input_state`: Clears a word in a global variable, reads a byte from a global variable, and evaluates bits within hardware input registers to calculate and store new values into a routine address and a global byte.
- `store_long_to_global`: Performs a logical OR on d7 with zero and stores a long value into a global variable.
- `increment_and_restore_interrupt`: Increments a byte in a global variable, calls two other routines, writes the value 32 to a hardware register, and performs an interrupt return.
- `clear_global_if_not_one`: Compares a byte from a global variable with 1; if it is not 1 and not less than 1, it clears the global variable and returns.
- `set_bits_in_globals`: If a global variable is not less than 1, it sets bits 7 through 0 in five consecutive global variables.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `transform_color_table`: Zeros out a table and then performs a multi-pass transformation on sixteen words by applying conditional increments based on bitmasks and shifting values.
- `transform_and_copy_table`: Copies a block of memory to itself, then performs bitwise reductions on a sixteen-word table before shifting the results left and storing them in a global variable.
- `calculate_offset_and_call`: Calculates a value based on shifts and additions using registers d0, d1, and d2, then calls another routine.
- `traverse_linked_list_structure`: Iterates through a list of bytes, where each non-negative byte is used to calculate offsets for two consecutive calls to another routine, and returns when a byte is negative one.
- `copy_structured_data_loop`: Reads a byte from a buffer, checks if it is the end marker, and if not, calculates an offset by shifting the value left by five, adds it to a base address, and calls another routine to copy data to the resulting destination before incrementing the source pointer and looping.
- `copy_structured_data`: Copies a series of bytes from a source buffer to a destination buffer using a non-contiguous pattern of offsets.
- `initialize_global_values`: Stores two specific longword values into two consecutive global variables.
- `update_global_values`: Swaps two 32-bit values from global variables, performs a bitwise rotation and subtraction on one, and stores the results back into the original global variables.
- `tiny_ori_3`: Small routine of 1 instruction that merges 36 into d1.
- `tiny_ori_4`: Small routine of 1 instruction that merges 36 into d1.
- `tiny_ori_5`: Small routine of 1 instruction that merges 144 into d4.
- `tiny_ori_6`: Small routine of 1 instruction that merges 240 into d4.
- `tiny_ori_7`: Small routine of 1 instruction that merges 180 into d4.
- `clear_high_bit`: Clears the high bit of bytes in a table using a loop.
- `execute_conditional_calls`: Checks specific bits in a byte and conditionally calls another routine multiple times or once based on those bits.
- `validate_sprite_data`: Scans memory ranges to validate sprite data structures and updates bitfield data based on specific conditions.
- `initialize_sprite_object`: Populates a data structure with attributes and coordinate information loaded from a lookup table based on an input index.
- `do_nothing`: Performs a bitwise OR operation on register d0 with zero.
- `execute_complex_loop`: Executes a series of calls to another routine within a loop that performs conditional arithmetic and bitwise checks on a return value to update a global variable.
- `transfer_word_and_execute_subroutines`: Copies a 16-bit word from one memory location to another and calls three different subroutines before returning.
- `call_subroutines`: Calls two different routines and then returns.
- `assign_global_pointers`: Sets several global variables to specific addresses and values, then calls two other routines.
- `process_data_table`: Iterates through a table using a pointer to perform conditional updates and assignments to global variables.
- `call_two_routines`: Calls another routine twice and then returns.
- `execute_sequence_of_calls`: Performs a sequence of calls to another routine using two different base addresses derived from a global pointer and bit-changed values.
- `execute_bytecode_loop`: Decrements a value, then enters a loop that executes a routine multiple times while checking a global variable and performing blitter operations.
- `execute_bytecode_loop_alt`: Increments a value, then enters a loop that executes a routine multiple times while checking a global variable and performing blitter operations.
- `execute_partial_sequence_of_calls`: Calls several routines, updates a structure using a global pointer, clears two bytes at offsets 6576 and 6578 of a structure, and calls several more routines before returning.
- `execute_nested_loop_calls`: Executes a routine within a nested loop structure, continuing to iterate as long as the return values of the called routines are non-zero.
- `configure_blt_hardware`: Updates hardware registers for bitplane transfers and waits for the DMA controller to complete operations.
- `load_lookup_table`: Copies large blocks of data from a lookup table into a destination memory area using nested loops.
- `copy_structured_data_2`: Copies a series of bytes from a source table to a destination table using specific non-contiguous offsets.
- `wait_for_bit_clear`: Polls a global variable until bit 6 is cleared, then repeatedly writes values to hardware registers and global variables.
- `do_nothing_2`: Performs a bitwise OR operation on register d0 with zero.
- `tiny_ori_8`: Small routine of 1 instruction that merges 126 into d4.
- `set_zero_values`: Sets register d4 to a constant and performs several bitwise OR operations on register d0 with zero and a constant.
- `clear_status_bits`: Clears the lowest bit of two bytes at offsets from a pointer in a register.
- `clear_data_structure_list`: Checks if a word at a global variable is not negative sixty-five thousand five hundred thirty-five; if it is not, it clears a word at that location, two specific offsets of 22 and 28 from that location, and then jumps to a routine that sets the address in a0 to a new location.
- `update_flag_byte`: Sets bit 2 of a byte at offset 17 or 28 of a structure based on the state of bit 15 of a global variable.
- `check_and_branch_on_pointer`: Loads a pointer into a0, compares the word at that pointer to 65535, and branches if they are not equal.
- `write_zero_offsets`: Writes zero to two word offsets relative to a pointer in a0 and then branches to another routine.
- `process_structure_list`: Iterates through a list of structures, performing bitwise operations on flags and updating hardware registers and memory with calculated values based on structure offsets.
- `update_object_properties_and_bitmask`: Checks a global variable and either calls a function from a table or updates several offsets in a structure by adding a value from a local word.
- `clears_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `reset_game_state_variables`: Clears several global variables and sets specific memory locations to zero, six, or two hundred fifty-five.
- `clear_and_call_routine`: Calls another routine and then sets a global pointer to zero if a byte at a global address is non-zero.
- `process_bcd_value`: Performs BCD adjustments and arithmetic operations on a value passed in a register and stores the results in multiple global variables.
- `clear_and_call_routine_2`: Checks if a function pointer is non-zero, calls it, and then sets that pointer to zero.
- `execute_and_clear_function_pointer`: Checks if a global function pointer is non-zero, executes the routine it points to, and then clears the pointer to zero.
- `clear_and_execute_callback`: Checks if a global pointer is non-null, executes the routine at that address, and then clears the pointer to null.
- `fill_memory_pattern`: Fills a memory range starting at a pointer with a repeating pattern of bytes.
- `load_table_entry_to_globals`: Calculates an offset into a table using a multiplier and loads a series of words and a longword into global variables.
- `copy_and_initialize_data`: Copies a block of memory from one location to another using a loop and then initializes several global variables and pointers with values from a data structure.
- `execute_double_routine`: Calls another routine twice using a pointer from a global variable.
- `initialize_global_bytes`: Calls a routine, writes the value 255 to four consecutive bytes starting at a global address, calls four other routines, and returns.
- `clear_memory_blocks`: Clears four blocks of memory starting at offsets 0, 8192, 16384, and 24576 from the address in a0 using a loop that runs 1000 times.
- `update_table_index`: Updates a global index and pointer based on values read from a table, including conditional logic for resetting offsets and updating a secondary pointer.
- `store_byte_indexed`: Stores a byte from register d0 into a memory location at offset 16 plus the value in register d0 relative to the address in a4.
- `clear_word_global`: Clears a word in a global variable and returns.
- `decrement_and_update_timer`: Decrements a global variable and, if it reaches zero, calculates a value using subtraction and stores it in a global word.
- `clear_structure_offset`: Clears a long value at offset 34 of a structure in a0, loads a pointer into a2, and calls another routine before returning.
- `initialize_specific_values`: Sets a global variable to two hundred fifty-five, assigns a constant to another global, and stores a specific value into a third global.
- `clear_structure_offset_alt`: Clears a long value at offset 34 of a structure in a0, loads a different pointer into a2, and calls another routine before returning.
- `execute_conditional_callback`: Checks if a word at a global address is zero, then performs a range check on a value from a structure, calls a routine, executes a function pointer from a register, and conditionally calls another routine if a bit is set in a structure member.
- `set_byte_indexed`: Performs a bitwise OR operation on a byte at offset d0 relative to the address in a2 with ninety-five.
- `clear_all_globals`: Resets a large number of global variables, pointers, and memory locations to zero or specific constant values.
- `set_word_global`: Sets a word in a global variable to one and returns.
- `copy_multiple_values`: Copies six values from various global variables into a contiguous block of memory starting at a global variable.
- `assign_function_pointers`: Calls another routine and then assigns several values and addresses to various global variables and pointers.
- `update_scrolling_window_parameters`: Updates a set of coordinates and offsets within a data structure, performing bounds checking and bitwise manipulations on several global variables.
- `reset_state_and_check_flag`: Clears several global variables and sets a specific memory location to a constant value before performing a conditional check on a global byte.
- `update_offset_variable`: Adjusts a global variable by adding or subtracting eight based on the value of a word at a global address, performs a range check, and conditionally calls a routine and updates a value from a register into a global variable.
- `manage_recursive_table`: Checks a global flag to decide whether to traverse a table of pointers recursively, updates several global variables, and performs bounds checking on table indices.
- `return_concatenated_registers`: Loads three words from a global location into d0, d1, and d2, calls another routine, and returns.
- `update_global_flags`: Calls another routine and, if the carry flag is set, clears a word in a global variable and sets bit 2 of two different global variables.
- `test_register_and_clear_carry`: Tests a word in d0 and clears the carry bit if the value is zero.
- `validate_range_bounds`: Checks if two values derived from a structure in a0 fall within two specified ranges defined by other values in a1 and a2, otherwise it sets the least significant bit of the condition code register and returns.
- `clear_ccr_bit`: Clears the least significant bit of the condition code register and returns.
- `validate_range_bounds_2`: Checks if a value derived from a global variable and a pointer falls within two specified ranges defined by other global variables, otherwise it sets the least significant bit of the condition code register and returns.
- `check_bit_and_bounds`: Verifies if a specific bit is set in a memory location calculated from two registers and checks if other registers fall within specific global bounds.
- `update_tile_attribute_data`: Calculates a memory offset using bitwise operations on registers and updates a series of bytes within a data table.
- `store_address_and_call`: Stores a longword at offset 34 of a structure in a6 and calls another routine.
- `store_pointer_and_call_logic`: Stores a longword constant into a memory location at offset thirty-four from a pointer in a0, loads a longword into a2, and calls execute_conditional_logic.
- `write_constant_values`: Stores the value six into a global variable and the value two hundred and fifty-five into another global variable, then returns.
- `write_global_bytes`: Writes the value 6 to a global variable and the value 255 to another global variable.
- `execute_conditional_logic`: Tests a word at offset 58 of a structure and, if zero, calls three different routines sequentially with carry flag checks.
- `execute_conditional_callback_2`: Calls a routine and then executes a series of other routines if a specific carry flag is set.
- `fetch_and_verify_pointer`: Loads a pointer from a table using an index derived from a structure, increments a counter in that structure, and if the pointer is not all ones, stores it at offset 34 of the structure and calls another routine.
- `call_cleanup_routine`: Calls a routine and returns.
- `call_decrement_routine`: Sets d0 to zero and calls another routine before returning.
- `decrement_and_check_byte`: Checks a byte at an offset of 44 from a pointer in a0; if it is non-zero, it subtracts one from that byte and loops back to the check, otherwise it calls a routine and returns.
- `modify_structure_values`: Writes a longword to an offset of 34 from a pointer in a0 and subtracts 2 from a 16-bit word at offset 6 of that same pointer.
- `calculate_and_update_data`: Performs a calculation using a register value and a constant, stores the result in a structure, and calls other routines based on a boolean result.
- `update_entity_state_and_position`: Updates an entity's internal index and pointer based on a lookup table, while incrementing and checking coordinate bounds.
- `reset_structure_value_on_bit`: Sets a long value at offset 34 of a structure in a0 to zero, then checks bit 7 of a byte at offset 70 of the structure; if that bit is set, it writes the constant 35616 to the long value at offset 34 and calls another routine before returning.
- `decrement_word_conditionally`: Tests a byte at offset 73 of a structure in a0; if it is not zero, tests a word at offset 2 of the structure, and if that word is not zero, decrements a word at offset 4 of the structure and branches to another location.
- `update_structure_values_2`: Increments a word at offset 6 of a structure, performs a bitwise manipulation involving a word at offset 10 and a word at offset 8 to update offsets 6 and 10, increments a word at offset 42, and branches to another routine.
- `call_and_load_constant`: Calls another routine, and if the carry flag is not set, loads the constant 80 into d0, calls another routine, and branches to another location.
- `execute_conditional_calls_2`: Calls another routine, and if the carry flag is not set, calls another routine and then branches to another location.
- `modify_byte_and_branch`: Tests a byte at offset 74 of a structure in a0; if it is not zero, it decrements that byte, calls another routine, and if the carry flag is not set, sets that byte to 25, clears a long value at a global location, tests a byte at offset 72 of the structure, and branches based on the results of testing offset 74 and offset 72.
- `adjust_structure_value_and_increment`: Loads two 16-bit values from offsets 4 and 6 of a structure in a0, sets a value at offset 8 to 256, and updates a global byte to 127.
- `increment_and_store_value`: Adds 2 to register d7, calls another routine, and if the carry flag is set, stores d7 at offset 6 of a structure in a0 and increments a value at offset 42 of the same structure.
- `update_structure_and_decrement_counter`: Loads a word from offset 6 of a structure in a0 into d7, compares d6 to a global longword, and if the comparison results in equality or d6 is greater than or equal to that global, it branches; otherwise, it sets a word at offset 2 of the structure to 255, subtracts 2 from d6, calls another routine, and if the carry flag is set, branches.
- `update_structure_field_and_increment`: Clears a 16-bit word at offset 2 of a structure, calls another routine, and if the carry flag is not set, stores a value from a register at offset 4 of the structure, increments a value at offset 42 of the structure, checks bit 1 of a global variable, and either clears a byte at offset 48 of the structure and sets a value at offset 42 to 2 before clearing a word at offset 0 or otherwise returns.
- `update_object_coordinate_and_timer`: Combines two 16-bit values from offsets 6 and 10 of a structure into a single 32-bit value, then splits it to update offsets 6 and 10 of the same structure.
- `update_object_state_on_condition`: Checks specific conditions against a global variable and a table; if met, it modifies a structure in a0 by setting a word at offset 4, updating a byte at offset 7, clearing a 16-bit value at offset 10, and setting bytes at offsets 72 and 42 before branching to another routine.
- `update_object_state_and_accumulator`: Sets bits 0 and 1 of a byte at offset 7 of the structure in a0, clears a 16-bit value at offset 10, and sets a 16-bit value at offset 8 to 256.
- `test_byte_and_branch`: Tests a byte at offset 74 of the structure in a0; if the byte is zero, it branches to another routine, otherwise it branches to a third routine.
- `decrement_register_on_condition`: Tests a word at offset 2 of a structure in a0 and, if it is zero, branches; otherwise, it subtracts 2 from register d6 and branches.
- `update_structure_timer_and_state`: Increments a word at offset 42 of a structure in a0 and updates words at offsets 4 and 6 using values from d6 and d7.
- `compare_and_branch`: Compares a word at a global address with a value in d6, then tests a word at offset 2 of a structure and branches accordingly.
- `branch_on_word`: Tests a word at offset 2 of a structure in a0 and branches to another routine or a later instruction based on the result.
- `decrement_and_branch`: Subtracts 2 from a word at offset 44 of a structure in a0, then branches to another routine or a later instruction based on the result.
- `reset_structure_fields`: Moves a word from offset 48 to offset 44 of a structure, clears a word at offset 42, and if a word at offset 2 is not zero, clears it before branching to another routine.
- `update_structure_values`: Sets a word at offset 2 of a structure to 255 and, depending on various conditions and other routine calls, adds values from a table to an offset in the structure.
- `add_value_from_table`: Clears bit 0 of d1, doubles d1, and if a byte at offset 73 of a structure in a0 is non-zero, adds a longword from a table to the structure at offset 34.
- `add_accumulated_value`: If a byte at offset 72 of a structure in a0 is non-zero, it loads a longword from a pointer at offset 424e of a constant address into d1, adds it to a longword at offset 34 of the structure in a0, and returns.
- `increment_accumulated_value`: Adds a value from a global variable to a longword in a structure, optionally adding an offset if a specific byte in that structure is non-zero.
- `initialize_structure_data`: Sets specific bytes and words within a structure and performs BCD arithmetic via multiple routine calls.
- `check_list_bounds`: Iterates through a list of structures, checking if values at offsets 4 and 6 fall within specific ranges defined by values in the list elements.
- `check_bounds_in_list`: Iterates through a list of structures, checking if specific values at offsets 4 and 6 fall within ranges defined by values at offsets 2, 3, 4, and 6 of each structure.
- `update_pointer_and_branch`: Loads the address at offset 76 of a structure in a1 into a1 and branches to another routine.
- `set_carry_flag`: Restores registers d0 through d1 and a1 from the stack and sets the carry flag.
- `clear_carry_flag`: Restores registers d0 through d1 and a1 from the stack and clears the carry flag.
- `check_coordinate_bounds`: Compares two 16-bit values in d0 and d1 against four 16-bit values in a structure and sets or clears the carry flag based on the result.
- `verify_multiple_ranges`: Performs several range checks by comparing values derived from global variables and pointers against bounds stored in a structure.
- `set_status_bit`: Sets bit 7 of a byte located at offset 2 of a pointer retrieved from a structure.
- `update_masked_attribute_bitmask`: Calculates a bitmask by reading and ORing multiple bytes from a table using offsets derived from two input registers, then updates a global variable by ANDing it with this mask.
- `combine_table_bytes`: Reads bytes from a table to populate two registers, performs bitwise masking and clearing on those registers, and then ors them together before branching to another routine.
- `conditionally_update_mask`: Checks bit 2 of a register and either calls another routine or performs a series of table lookups and bitwise operations to update a register before branching to another routine.
- `update_masked_global`: Ors bytes from a table into a register using offsets from a pointer, performs range checks against three global variables, and updates a global variable with a masked value before potentially calling itself.
- `execute_complex_sequence`: Calls several subroutines and library functions, performs conditional checks on global variables and table values, and contains a loop that executes a routine multiple times based on a counter.
- `clear_memory_block`: Clears a block of memory by writing zero to each longword at the pointer in a0 for a total of 29191 iterations.
- `copy_memory_block`: Copies a block of sixteen words from one location to another and, if a global variable is zero, toggles a byte in that global variable via another routine.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `modify_registers_and_data`: Performs bitwise OR operations on d0, subtracts 555 from d4, tests bits in a table using d1 and d0, and performs bitwise AND operations on the value pointed to by a2.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `execute_nested_loops`: Calls several routines and enters a loop that repeats a sequence of calls and checks a bit in a global variable, then repeats this process in a second loop before returning.
- `copy_multiple_word_blocks`: Copies several blocks of words from a source to four destination areas starting at offsets 0, 8192, 16384, and 24576 from a base address.

### Rick Dangerous.adf bytes 228352-228864 (code never loaded by a reference run)

- `update_status_flags`: Compares a value from a global variable with a constant and checks a byte at a global address to set bit 6 of either a global variable or another global variable.
