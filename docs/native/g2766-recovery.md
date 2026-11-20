# Game 2766 (atr - all terrain racing cd32 (1995)(team 17)(aga)[b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02766). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 143 of 143 named (0 author symbols), 143 of 143 described (70 by rule, 53 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 83 of 83 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2069 of 2069 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 682 |
| code | 5456 |
| data | 714468 |
| padding | 1314 |
| unused | 179200 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| ATR - All Terrain Racing CD32 (1995)(Team 17)(AGA)[b].adf:atr | 2 | 267 | 8 |
| powerpacker.library | 2 | 1784 | 134 |
| ATR - All Terrain Racing CD32 (1995)(Team 17)(AGA)[b].adf:bootblock | 1 | 18 | 1 |

## Cases handled

- atr - all terrain racing cd32 (1995)(team 17)(aga)[b]/ATR - All Terrain Racing CD32 (1995)(Team 17)(AGA)[b].adf:atr: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### ATR - All Terrain Racing CD32 (1995)(Team 17)(AGA)[b].adf:atr

- `process_hunk_headers`: Reads and processes hunk headers from a data stream, allocating memory blocks for each hunk and copying data into them based on header information.
- `decrement_counter`: Decrements a counter in a register until it reaches zero, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `shift_and_copy_data`: Processes data by shifting values in registers, performing bitwise operations, and copying bytes between memory locations based on counter values.
- `frag_lsr_beq_rts_move_roxr`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns, then copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `rotate_shift_loop`: Performs bitwise operations on data in registers and memory, rotating and shifting values based on counter and register contents, then returns.
- `load_and_or`: Loads a word from memory at offset 3085 of a2 into d4, then performs a bitwise OR operation with the value 24 on the byte at address in a3, and finally executes a trap instruction.

### powerpacker.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `set_library_base`: As library code, it stores an argument in a global variable, writes a library base into a table entry, calls several routines via a table to store returned values in global variables, and calls another routine twice.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `frag_moveq_subq_bne_btst_rts`: Short code fragment that loads 0 into d0, then subtracts 1 from the memory at an offset from a6, then branches if the last result was not zero, then tests bit 3 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `frag_movem_movea_tst_beq_bset_moveq_bra`: Short code fragment that saves registers on the stack, then copies a6 to a5, then copies a fixed memory address to a6, then tests the memory at an offset from a5, then branches if the last result was zero, then sets bit 3 of the memory at an offset from a5, then loads 0 into d0, then jumps to another point in the code.
- `frag_jsr_lea_cmpa_beq_moveq_bra`: Short code fragment that calls another routine, then loads the address of a location in the code into a0, then compares a0 with the memory at an offset from a6, then branches if the last result was zero, then calls another routine, then loads 0 into d0, then jumps to another point in the code.
- `initialize_library_state`: Calls several library functions through a table of offsets from a6, reads a longword from a global variable at offset 36 of the structure in a5, calls one more library function, and returns that longword in d0.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `call_decompression_handler`: This library routine calls another routine and then returns.
- `decode_bitstream_3`: Processes bit-level data from a buffer using shift operations and bitmasking to decode values, updating internal state and writing results to an output buffer.
- `frag_move_roxr_bra`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_2`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_3`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_4`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_5`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_6`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `decode_bits`: Processes bit-level data from a buffer using shift operations and bitmasking to decode values, updating internal state and writing results to memory.
- `unpack_data_bits`: Extracts and unpacks bit-stream data using shift operations and bitwise manipulations, updating internal state variables and writing decoded values to a destination buffer.
- `frag_move_roxr_bra_7`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_8`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_9`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `decode_bitstream_2`: Extracts and decodes bit-stream data using rotate operations and bitwise manipulations, updating internal state variables and writing decoded values to a buffer.
- `frag_move_roxr_bra_10`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_11`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_12`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `decode_bits_2_2`: Processes bitstream data by rotating bits through the carry flag and decoding values into a buffer, with conditional copying and loop control.
- `frag_move_roxr_bra_13`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_14`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_15`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_moveq_sub_move_add_jmp`: Short code fragment that loads 13 into d0, then subtracts an indexed entry from a2 from d0, then copies d0 to d1, then adds d0 to d0, then adds d1 to d0, then adds d0 to d0, then jumps to another point in the code.
- `decode_bits_2`: Processes bitstream data by rotating bits through the carry flag and decoding values into a buffer, with conditional copying and loop control based on decoded bit patterns.
- `frag_move_roxr_bra_16`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_17`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_18`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_19`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_move_roxr_bra_20`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by 1, then jumps to another point in the code.
- `frag_ori_bra`: Short code fragment that merges an address into d0, then jumps to another point in the code.
- `compute_rotating_word_checksum`: Iterates over a zero-terminated byte string at a0, rotating a 16-bit accumulator in d0 by the low bits of a running value and adding the current byte to it, then returns the accumulator as a 16-bit value in d0.
- `compute_checksum`: As part of the PowerPacker shared library, computes a checksum over a zero-terminated byte stream pointed to by a0.
- `xor_block_with_key`: Computes a longword count from the length in d0, then XORs each longword at a0 with the key in d1 and advances a0 by one longword per pass until the count reaches zero, then returns.
- `frag_moveq_bra`: Short code fragment that loads 1 into d5, then jumps to another point in the code.
- `process_hunk_relocation_3`: Processes hunk relocation entries from input data, updates memory addresses and counters based on hunk type codes, and handles relocation operations for executable loading.
- `compute_and_copy`: Saves the stack pointer to a0, calls a function at offset 1580 of the exec library base, adjusts the stack pointer, copies d0 to d1, loads a pointer from a4 into a0, doubles the value in a0 and advances it by four bytes, then calls another function at offset 1596 of the exec library base, and finally restores multiple registers from the stack.
- `process_hunk_relocation_4`: Processes hunk type codes from input data, allocates memory for each hunk, and updates relocation information in a global structure based on the hunk type codes.
- `unpack_hunk_data`: Processes hunk type codes from input data, allocates memory for each hunk, and updates relocation information in a global structure based on the hunk contents.
- `process_hunk_relocation`: Processes hunk relocation entries from input data, updates memory addresses and counters based on hunk type codes, and handles relocation operations for loaded executable segments.
- `process_hunk_entries`: Reads hunk type codes from a table and processes them, updating a counter and a memory address based on the values read.
- `frag_movea_subq_bcs_adda_bra`: Short code fragment that copies a5 to a1, then subtracts 4 from a1, then subtracts 1 from d1, then branches if the carry flag is set, then copies the memory at a1 to a1, then adds a1 to a1 (2 times in a row), then jumps to another point in the code.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 4 to a1, then returns.
- `process_hunk_header`: Processes a hunk header value from a register, updates counters and pointers based on comparisons with specific hunk type codes, and handles relocation operations for executable loading.
- `process_hunk_relocation_2`: Processes hunk relocation entries from input data, updates memory addresses and counters based on hunk type codes, and handles relocation operations.
- `load_multiple_words`: Loads words from memory locations with offsets 2313, 2570, 2827, 3084, and 3085 of a1 and a2 respectively into d4 each time.
- `frag_movem_move_lea_movea_tst_beq_bra`: Short code fragment that saves registers on the stack, then copies d0 to d6, then loads the address of the memory at an offset from a7 into a7, then copies a7 to a0, then tests d2, then branches if the last result was zero, then copies an address to the memory at a0 (advancing a0), then copies d3 to the memory at a0 (advancing a0), then jumps to another point in the code.
- `write_header_and_verify_result`: Stores the constant a fixed address and a value looked up from a table by the index in d1 into the two longwords at a0, then calls a function through a global variable.
- `frag_movem_pea_bra`: Short code fragment that saves registers on the stack, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_movem_pea_move_bra`: Short code fragment that saves registers on the stack, then pushes the address of a fixed memory address onto the stack, then copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `frag_movem_pea_bra_2`: Short code fragment that saves registers on the stack, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `open_and_call_library`: Saves registers d2/a3/a6 to the stack, prepares arguments for a library call, loads the exec library base, calls a function at offset -408 of the exec library base, checks if the result is zero and branches to another routine if so, otherwise loads a library pointer from d0, adds it to a value on the stack, restores registers, adjusts the stack pointer, prepares more arguments for another call, and returns.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_library_function_then_return`: Loads a pointer from the exec base, calls a function at a fixed negative offset from the library base in a6 with the constants 0 and 19 in D0 and D1, restores saved registers, clears D0 and returns.
- `lookup_inverted_clamped_table_entry`: Bitwise-inverts D0, clamps the result to a maximum of 7, and returns the 32-bit longword at that index in a table of longwords.
- `check_powerpacker_header`: Compares a longword at a1-4 with a constant, then compares a1 with a value at a1-4, and if both match, it increments d0 and pushes a value onto the stack before returning.
- `open_and_read_file`: Opens a file using a global function pointer, reads its header, validates the format, and allocates memory for the file content based on the header information.
- `copy_and_process_data`: Copies data from a file into a memory buffer and processes the buffer based on file format markers, setting flags and updating pointers before returning.
- `read_and_validate_header`: Reads and validates a file header using a global function pointer, then performs a second read operation and stores the result in a global variable before branching to another routine.
- `validate_file_header`: Reads and validates a file header from a stream, calculates buffer size, allocates memory for the buffer, and updates global variables with the buffer address and size.
- `process_x20_file_header`: Reads two bytes from a file stream and calls a function pointer provided in a register; if the read is successful, it calls another function pointer from a table and then calls a third function pointer passed via the stack before returning.
- `read_file_header_2`: Reads a file header from a stream, processes the data to calculate buffer size, allocates memory for the buffer, and updates global variable.
- `read_file_header`: Reads a file header from a stream, processes the data to calculate buffer size, allocates memory for the buffer, and updates global variables with the allocated buffer and its size.
- `frag_cmpa_bne_moveq_move_movea_jsr_bra`: Short code fragment that compares a3 with 65535, then branches if the last result was not zero, then loads 12 into d5, then copies d7 to d1, then copies the memory at an offset from a4 to a6, then loads 4 into d2, then loads 0 into d3, then calls another routine, then jumps to another point in the code.
- `validate_and_load_buffer`: Validates file header data, allocates memory for a buffer, reads data from a file into the allocated buffer, and updates global variables with the buffer pointer and size.
- `allocate_and_read_file`: Allocates memory using a global function pointer, reads data from a file into the allocated buffer, and updates global variables with the buffer address and size when the read operation succeeds.
- `read_and_process_file`: Sets up parameters for file operations, calls dos.Seek with specific values, reads data from a file using dos.Read, compares the result with expected length, and if they match, closes the file, processes the data, and sets up return values.
- `close_file_and_free_memory`: The routine calls a global function pointer and a function pointer taken from a table, then returns a value from the stack.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (movea.l) and then falls through into the code that follows, with no return of its own.
- `validate_and_count`: Checks if a parameter is zero, loads values from the stack into registers, and if not zero, calls another routine to compute a checksum, compares it with a stored value, increments a counter, and returns based on comparison results.
- `frag_move_addq_tst_bne_movea_jsr_lea_br`: Short code fragment that copies the memory at an offset from a5 to d0, then adds 1 to the memory at an offset from a5, then tests d0, then branches if the last result was not zero, then copies the memory at a5 to a0, then calls another routine, then copies d0 to the memory at an offset from a5, then loads the address of a location in the code into a0, then copies a0 to d0, then jumps to another point in the code.
- `frag_movea_jsr_cmp_beq_clr_lea_move_bra`: Short code fragment that copies the memory at a5 to a0, then calls another routine, then compares d0 with the memory at an offset from a5, then branches if the last result was zero, then clears the memory at an offset from a5, then loads the address of a location in the code into a0, then copies a0 to d0, then jumps to another point in the code.
- `frag_moveq_move_movem_rts`: Short code fragment that loads 1 into d0, then copies d0 to the memory at an offset from a5, then loads 0 into d0, then restores saved registers from the stack, then returns.
- `trigger_powerpacker_decompress`: The routine receives a value in a0 and, if it is non-zero, calls a library routine to free the PowerPacker library three times in succession.
- `initialize_header_fields_3`: Initializes various fields in a data structure based on input values, performs bit shifts and operations on word-sized constants, and sets up multiple parameters for processing.
- `initialize_header_fields_2`: Sets up various fields in a data structure based on input values, performs bit shifts and operations on word-sized constants, and initializes multiple array elements with specific calculated values.
- `frag_moveq_bra_2`: Short code fragment that loads 9 into d5, then loads 9 into d4, then loads 9 into d6, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 10 into d5, then loads 10 into d4, then jumps to another point in the code.
- `frag_moveq_bra_4`: Short code fragment that loads 12 into d5, then loads 12 into d4, then jumps to another point in the code.
- `initialize_header_fields`: Initializes various fields in a data structure with specific values including word-sized constants, performs bit shifts and operations on registers, sets up memory locations with computed values, and calls other routines to compute additional parameters.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `set_ride_stats`: The routine stores a value from a register into a field of a structure pointed to by a pointer, stores another value from a register into the next field, stores the sum of the two register values into a following field, clears a field in the structure, then calls another routine.
- `set_ride_stats_2`: The routine stores a value from a register into a field of a structure pointed to by a pointer, stores another register value into a different field of the same structure, stores the sum of those two register values into a third field of the structure, stores a value from an unused register into another field of the structure, then calls another routine and returns.
- `clear_tile_tables`: Clears two memory regions pointed to by parameters, each sized according to values in the structure, and updates internal pointers based on input data.
- `initialize_tile_processing`: Initializes tile-related parameters including offset calculations and table entries based on input data, then conditionally processes further based on a threshold value.
- `frag_bsr_movem_bra`: Short code fragment that calls another routine, then restores saved registers from the stack, then jumps to another point in the code.
- `update_offset_table`: Updates a table of offsets based on input data and performs conditional processing when a threshold is exceeded.
- `search_longest_match`: Scans a data stream to find the longest matching byte sequence, storing the resulting match length and offset into fields at offsets 34 and 36 of a structure pointed to by a4.
- `frag_move_movem_bra`: Short code fragment that copies d5 to the memory at an offset from a4, then copies d7 to the memory at an offset from a4, then restores saved registers from the stack, then jumps to another point in the code.
- `find_match_length`: Scans input data to find matching byte sequences and calculates their lengths, storing the results in structure fields at offsets 34 and 36.
- `frag_movem_bra`: Short code fragment that restores saved registers from the stack, then jumps to another point in the code.
- `decode_literal_copy`: Processes literal byte sequences from an input buffer, compares them with reference data, and updates output buffer pointers based on matching results.
- `clear_output_entries`: Clears word-sized entries in two output buffers based on a counter, updates a pointer table entry, and manages loop iterations until the counter reaches zero.
- `frag_bsr_movem_bra_2`: Short code fragment that calls another routine, then restores saved registers from the stack, then jumps to another point in the code.
- `process_bitstream_data`: Processes bitstream data by reading bytes from a buffer, performing bitwise operations including shifts and rotates, and writing results to a destination while maintaining loop control.
- `initialize_offset_tables`: Initializes two tables of word-sized entries by clearing them, then sets up offset mappings based on data from a structure pointed to by a4.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `initialize_offset_table_2`: Clears word-sized entries in two pointers while decrementing a counter, then initializes a table of offsets by clearing entries and setting up loop counters based on values from a structure.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `initialize_offset_table_3`: Clears a table of offsets by setting each entry to zero, then processes data using a loop that reads bytes and computes offsets from a pointer table.
- `initialize_offset_table`: Initializes a table of offsets by clearing entries and setting up loop counters based on values from a structure, then processes data using a bitstream routine.
- `initialize_bitstream`: Initializes bitstream processing by setting up loop counters and performing bitwise operations on data from a buffer, then calls another routine to handle decompression.
- `decode_bitstream`: Processes bitstream data from a buffer, performs bitwise operations including shifts and rotates, and writes the results to a destination.
- `frag_move_bsr_moveq_bra`: Short code fragment that copies an indexed entry from a4 to d0, then calls another routine, then loads 1 into d1, then loads 1 into d0, then jumps to another point in the code.
- `process_bitstream`: Reads bitstream data from a buffer, processes it through bitwise operations including shifts and rotates, and writes the results to a destination buffer.
- `frag_move_movea_jsr_bra`: Short code fragment that copies a6 to the stack, then copies a fixed memory address to a6, then calls another routine, then copies the top of the stack to a6, then jumps to another point in the code.
- `frag_cmpa_bcs_movea_moveq_bra`: Short code fragment that compares a6 with a5, then branches if the carry flag is set, then copies the memory at an offset from a4 to a7, then loads 255 into d0, then jumps to another point in the code.
- `frag_move_lsl_or_tst_bne_sub_bra`: Short code fragment that copies the top of the stack to d1, then shifts d1 left by 8, then merges d7 into d1, then copies d1 to the memory at a6 (advancing a6), then copies a6 to d0, then tests the memory at an offset from a4, then branches if the last result was not zero, then subtracts the memory at an offset from a4 from d0, then jumps to another point in the code.
- `frag_sub_movem_rts`: Short code fragment that subtracts the memory at an offset from a4 from d0, then restores saved registers from the stack, then returns.
- `build_tile_mapping_entries`: Loops over a range of byte-sized indices, looking up and updating a pointer table entry for each, and writes derived 16-bit offset values into two output tables at positions computed from the offset.
- `update_offset_tables_by_index`: Loops over a byte range, reading each byte as an index into a longword pointer table, computes an offset from the stored pointer, writes it into two 16-bit output tables when valid, and stores the current byte pointer back into the table.
- `update_single_offset_table`: Loops over a byte range, reading each byte as an index into a longword pointer table, computes an offset from the stored pointer, writes it into one 16-bit output table when valid, and stores the current byte pointer back into the table.
- `divide_and_process`: Compares the value in d7 with 4, branches if less than 4, otherwise divides d7 by 3 after subtracting 4, swaps the result, copies it to d1, calls another routine, swaps back, loads 3 into d1, and calls another routine again.
- `frag_subq_bsr_moveq_rts`: Short code fragment that subtracts 1 from d1, then calls another routine, then loads 0 into d1, then calls another routine, then loads 0 into d7, then returns.
- `frag_lsr_roxl_bcs`: Short code fragment that shifts d1 right by 1, then rotates d5 left through the extend flag by 1, then branches if the carry flag is set.
- `frag_lsr_roxl_bcs_2`: Short code fragment that shifts d1 right by 1, then rotates d5 left through the extend flag by 1, then branches if the carry flag is set.
- `frag_lsr_roxl_bcs_rts`: Short code fragment that shifts d1 right by 1, then rotates d5 left through the extend flag by 1, then branches if the carry flag is set, then returns.
- `shift_and_loop`: Decrements d0, shifts d1 right by one bit, rotates d5 left through the carry flag by one bit, checks if the carry flag is set, and if so, branches back to an earlier instruction, otherwise continues execution.
- `frag_dbra_rts`: Short code fragment that loops back while counting down d0, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a location in the code onto the stack, then jumps to another point in the code.
- `frag_pea_bra_2`: Short code fragment that pushes the address of a location in the code onto the stack, then jumps to another point in the code.
- `frag_pea_move_moveq_cmpa_bcc_rts`: Short code fragment that pushes the address of a location in the code onto the stack, then copies d5 to the memory at a6 (advancing a6), then loads 1 into d5, then compares a6 with a5, then branches if the carry flag is clear, then returns.
- `frag_tst_beq_movea_moveq_bra`: Short code fragment that tests the memory at an offset from a4, then branches if the last result was zero, then copies the memory at an offset from a4 to a7, then loads 255 into d0, then jumps to another point in the code.
- `copy_and_update_pointer`: Saves registers d0-d1/a0-a2 to the stack, loads a pointer from memory at offset 84 of a4 into a2 and a6, loads another pointer from memory at offset 92 of a4 into a0, copies data using exec.CopyMemQuick, updates a pointer in memory at offset 84 of a4, compares pointers, and branches based on the comparison result.
- `frag_lea_movem_rts`: Short code fragment that loads the address of the memory at an offset from a4 into a6, then restores saved registers from the stack, then returns.
- `frag_move_addq_movea_jsr_tst_bne_bra`: Short code fragment that copies a6 to the stack, then adds 4 to d0, then copies d0 to the stack, then copies a fixed memory address to a6, then calls another routine, then tests d0, then branches if the last result was not zero, then adds 4 to a7, then jumps to another point in the code.
- `store_stack_longword_advance_pointer`: Writes the longword taken from the top of the stack to the address in D0, increments that pointer by four bytes and returns it in D0.
- `frag_move_beq_movea_jsr_rts`: Short code fragment that copies a1 to d0, then branches if the last result was zero, then copies a6 to the stack, then copies the memory just below a1 (moving a1 down) to d0, then copies a fixed memory address to a6, then calls another routine, then copies the top of the stack to a6, then returns.

### ATR - All Terrain Racing CD32 (1995)(Team 17)(AGA)[b].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
