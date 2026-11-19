# Game 716 (soccer kid (1994)(krisalis software)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00716). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3613532 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 31 of 31 units identical (bytes and relocations), 52 raw-word lines |
| Functions named and described | pass, 108 of 108 named (0 author symbols), 108 of 108 described (36 by rule, 48 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1348 of 1356 (99.41%) |
| Recovered without load address (counted in proof 1, flagged) | 14320 bytes, 5 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2467 of 2620 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3283 |
| code | 9412 |
| data | 3600263 |
| padding | 62 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf:bootblock | 1 | 176 | 13 |
| logged disk load from Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf boot-block load | 1 | 116 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 377 | 5 |
| logged disk load from Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf | 1 | 77 | 3 |
| logged disk load from Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf | 1 | 32 | 3 |
| logged disk load from Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf | 1 | 51 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 69 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 384 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 62 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 65 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 416 | 26 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 196 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 92 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 137 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 16 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 149 | 4 |
| Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 505856-505952 beside a logged load (code never loaded by a reference run, load address unknown) | 1 | 0 | 0 |
| Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 506096-506368 beside a logged load (code never loaded by a reference run, load address unknown) | 1 | 56 | 2 |
| Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 506368-506432 beside a logged load (code never loaded by a reference run, load address unknown) | 1 | 12 | 1 |
| Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 506672-506880 beside a logged load (code never loaded by a reference run, load address unknown) | 1 | 27 | 1 |
| Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 640512-640880 beside a logged load (code never loaded by a reference run, load address unknown) | 1 | 0 | 0 |
| Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf blocks 990-1015 (bytes 506880-520192) (code never loaded by a reference run, load address unknown) | 1 | 6 | 1 |

## Cases handled

- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- soccer kid (1994)(krisalis software)[cr flt](disk 2 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 2 of 4).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- soccer kid (1994)(krisalis software)[cr flt](disk 3 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 3 of 4).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- soccer kid (1994)(krisalis software)[cr flt](disk 4 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 4 of 4).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: the boot block reads 3584 bytes from disk offset 618496 to an address (found statically); recovered as a unit
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: 71 executed PCs whose RAM was overwritten before any snapshot match its bytes 506432-506672: recovered as a unit run at an address
- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: 26 executed PCs whose RAM was overwritten before any snapshot match its bytes 505952-506096: recovered as a unit run at an address
- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: 17 executed PCs whose RAM was overwritten before any snapshot match its bytes 640880-641072: recovered as a unit run at an address
- RAM snapshot an address: 7 executed PCs whose RAM was overwritten before any snapshot match its bytes 15296-15568: recovered as a unit run at an address
- RAM snapshot an address: 6 executed PCs whose RAM was overwritten before any snapshot match its bytes 493840-493920: recovered as a unit run at an address
- RAM snapshot an address: 4 executed PCs whose RAM was overwritten before any snapshot match its bytes 8720-8800: recovered as a unit run at an address
- 1217 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 15 first-run image unit(s) in 1 version group(s); 2614 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 505856-505952 beside a logged load (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 0 of 96 bytes decode)
- Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 506096-506368 beside a logged load (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 148 of 272 bytes decode)
- Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 506368-506432 beside a logged load (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 52 of 64 bytes decode)
- Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 506672-506880 beside a logged load (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 116 of 208 bytes decode)
- Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 640512-640880 beside a logged load (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 0 of 368 bytes decode)
- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: 9 logged loads cover 4160 bytes of the image
- Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf blocks 990-1015 (bytes 506880-520192) (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 20 of 13312 bytes decode)
- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: blocks 1017-1207 (bytes 520704-618496) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.95)
- soccer kid (1994)(krisalis software)[cr flt](disk 1 of 4)/Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf: blocks 1215-1250 (bytes 622080-640512) outside the filesystem hold data, not code (opcode density 0.049, entropy 6.79)
- 3 never-loaded region(s), 13776 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 5 packed track region(s), 3464704 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf:bootblock

- `setup_interrupts_and_dma`: Configures interrupt and DMA settings by enabling specific bits in hardware control registers, sets up an I/O request structure with command code 2, and executes a library call to perform the I/O operation.
- `install_vectors_copy_block`: Writes three routine addresses into a fixed low memory vector slot, clears a shared word, and copies a block of twenty-seven words from a routine to a fixed memory location.
- `copy_word_block`: Calls a routine that copies words from a source pointer to a destination pointer, advancing both by one word per step until a counter in the data register runs out.
- `copy_word_block_forward`: Copies words from a source pointer to a destination pointer, advancing both by one word per step until a counter in the data register runs out.
- `write_text_marker_by_flag`: Writes the four-character text marker FAT0 to a destination pointer, or FAT1 when a bit of a shared state word is set.
- `detect_refresh_rate_hz`: Resets the CIA port and timer registers, then reads a counter from two bytes and compares it with a fixed threshold.
- `wait_until_status_byte_clear`: Repeatedly checks a status byte at a fixed offset from a base register and returns minus one once its top bit is clear.
- `clear_buffer_and_fill_tags`: Clears a fixed-size buffer, writes two four-character text tags into it, then calls two other routines.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (movea.l, movea.l) and then falls through into the code that follows, with no return of its own.
- `walk_list_from_given_node`: Walks the same kind of linked list starting from the pointer already held in a register, stopping at the first node with the selected bit set or at the end.
- `store_control_word_or_halt`: If a shared flag is clear, it stores a computed control word into a structure and returns.
- `copy_longword_and_tag`: Copies one longword from a source pointer to a destination pointer, then stores a tagged word that combines the data register D0 with a fixed two-letter tag into a structure field.
- `setup_io_request`: Configures an I/O request structure with command code 2, sets up several memory fields, and then executes a library call to perform the I/O operation.

### logged disk load from Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf boot-block load

- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `lookup_and_mask`: Performs a lookup in a table using a calculated index and applies a bitwise AND operation with a value from another register before returning the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_ori_bne_movem_rts`: Short code fragment that merges 128 into d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `frag_tst_beq_movea_bra`: Short code fragment that tests the memory at a1, then branches if the last result was zero, then copies the memory at a1 to a1, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_ori`: Short code fragment that merges 0 into d0 (10 times in a row).
- `update_structure_and_set_flags`: Adds two values from a structure and a byte to a third value within that same structure, sets bit 3 of a byte at offset 3584 of a structure in a1, and writes the values 129, 0, and 2 to offsets 1280, 1284, and 2048 of that same structure.
- `modify_structure_fields`: Sets bit 3 of a byte at offset 3584 from a pointer in a1, writes the value 129 to offset 3328, writes 0 to offset 1024, and writes 2 to offset 1280 from the same pointer.
- `update_structure_and_set_flags_2`: Adds a byte and two values from memory locations to a third memory location, sets bit 3 of a byte at offset 3584 of a structure in a1, and writes the values 129, 0, and 2 to offsets 1280, 1024, and 1280 of that same structure.
- `modify_structure_fields_2`: Sets bit 3 of a byte at offset 3584 of a structure pointed to by a1, writes the value 129 to offset 3328, writes 0 to offset 1024, and writes 2 to offset 1280 of the same structure.

### logged disk load from Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf

- `extract_bit_stream_2`: Performs bitwise operations on longwords fetched from memory to extract bits, using a loop to write processed values into a destination table pointed to by a2.
- `rotate_right_extend_long`: Returns a 32-bit word read from memory, shifted right one bit, with the extend-flag bit placed at the top and the top bit forced set.
- `extract_bit_stream`: Reads a longword from a memory location pointed to by a0, performs bitwise operations to extract a single bit, and updates a value in d5 through a loop controlled by d1.

### logged disk load from Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf

- `frag_bne_jmp`: Short code fragment that branches if the last result was not zero, then jumps to another point in the code.
- `frag_movem_rte`: Short code fragment that restores saved registers from the stack, then returns from the exception.
- `reorganize_nibbles_in_table`: Iterates through a table and rearranges the nibbles within each word based on a bit-shifting pattern.

### logged disk load from Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf

- `validate_index_range`: Checks if a signed integer falls within a specific range and verifies if a corresponding byte in a table is non-zero.
- `update_structure_fields`: Copies a 32-bit value from one offset to another within a structure in a6, modifies a byte at offset 100 of a structure in a1 using bitwise operations, and writes several 16-bit values to offsets of structures pointed to by a4 and a6, then calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_buffered_data_stream`: Iterates through a memory structure to find a specific marker, then performs data manipulation by calling calculate_masked_bitwise_result to extract values and stores them into a destination buffer.
- `calculate_masked_bitwise_result`: Masks two values from memory using a mask in d4, doubles the first masked value, and returns the bitwise OR of both results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_memory_and_pointers`: Performs bitwise operations on a byte at an offset of 256 from a pointer in a1, calls another routine, and updates several memory locations and pointers using values from a4 and a6.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bset_move_clr_movem_rte`: Short code fragment that sets bit 3 of the memory at an offset from a1, then copies 129 to the memory at an offset from a1, then clears the memory at an offset from a1, then copies 2 to the memory at an offset from a1, then restores saved registers from the stack, then returns from the exception.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_blitter_and_display_parameters`: Calculates a value based on the vertical position register and a table, clears a block of memory, populates a table with values incremented by 40, and configures several blitter registers including control, mask, and mode registers before calling update_blt_parameters_from_table.
- `update_blt_parameters_from_table`: Iterates through a table, calculating a new address and updating several bitplane and blitter registers or a global variable based on the table contents.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `frag_bra`: Short code fragment that jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movea_moveq_bra`: Short code fragment that copies a location in the code to a0, then loads 2 into d0, then jumps to another point in the code.
- `copy_table_data`: Performs a nested loop to copy 16 words from a source table to a destination table, while also clearing a word at a specific location in a global variable.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `set_blt_parameters_fixed`: Sets several hardware registers and a global variable to fixed or calculated values.
- `set_blt_parameters_swapped`: Sets several hardware registers and a global variable to fixed values, but swaps the source and destination addresses compared to another routine.
- `frag_sf_bra`: Short code fragment that clears a fixed memory address, then jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_address_tables`: Calls two routines to set address offsets and update an address table, then enters an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_sequence_of_functions`: Calls several routines in sequence and conditionally calls another routine based on a byte at an offset from a register.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_structure_state`: Performs a series of calls and memory operations involving a structure in a6, including updating bytes and bitwise flags based on values at specific offsets.
- `frag_move_swap_rts`: Short code fragment that copies d0 to the memory at an offset from a6, then copies d1 to the memory at an offset from a6, then copies d2 to the memory at an offset from a6, then swaps the halves of d2, then copies d2 to the memory at a6, then returns.
- `modify_structure_bits`: Sets bits 2, 3, and 6 of two consecutive bytes in a structure via a pointer in a register, writes a 16-bit value to an offset of 34, and calls another routine.
- `write_structure_values`: Writes the word values 16384 and 39425 to offset 36 of a structure in a6, writes the word value 2 to offset 156 of the same structure, and then writes 39425 to offset 36 again after calling another routine.
- `wait_for_interrupt_and_write`: Calls another routine, waits for a specific bit in a hardware register to be set while checking a global variable, then writes two words to offsets of a pointer in a6.
- `frag_bsr_ori`: Short code fragment that calls another routine, then merges 0 into d0 (5 times in a row).
- `frag_bclr_or_move_moveq_bsr_rts`: Short code fragment that clears bit d7 of d1, then merges d1 into d0, then copies d0 to the memory at an offset from a1, then loads 90 into d0, then calls another routine, then returns.
- `process_data_buffer`: Checks a byte from a structure in a6 against a value in d0, then iterates through a loop to copy and manipulate longwords from a buffer into a destination pointed to by a5.
- `combine_masked_longwords`: Fetches two longwords from memory using pointers in a0 and a3, applies a bitmask from d4 to both, doubles the masked value from a0, and returns the bitwise OR of the doubled value and the masked value from a3.
- `write_word_wait_for_flag`: Clears the wait flag, writes the low and high bytes of its argument to two device registers, then spins until another routine sets the flag.
- `update_control_byte_bits`: Keeps the top three bits of one byte at a fixed offset from a base pointer while setting bit three, and writes the constant 130 into another byte at a fixed offset.
- `frag_move_bsr_rts`: Short code fragment that copies d0 to the stack, then calls another routine, then copies the top of the stack to d0, then calls another routine, then returns.
- `frag_move_bsr_sf_lsr_rts`: Short code fragment that copies d0 to the stack, then calls another routine, then copies the top of the stack to d0, then clears a fixed memory address, then copies d0 to the memory at an offset from a1, then shifts d0 right by 8, then copies d0 to the memory at an offset from a1, then returns.
- `update_control_byte`: Calls another routine, loads a word from offset 46 of a structure in a6 into d6, calls another routine, and if the zero flag is not set, loads a word from offset 46 of a6 into d0, calls another routine, loads 15548 into d0, and calls another routine before returning.
- `frag_move_andi_ori_bclr_nop_rts`: Short code fragment that copies the memory at an offset from a1 to d0, then masks d0 with 253, then merges 0 into d0, then clears bit d7 of d0, then copies d0 to the memory at an offset from a1, then pauses for one no-operation step, then returns.
- `update_control_bits`: Checks a byte at an offset from a pointer, and if non-zero, calls two routines, sets that byte to 255, and clears a specific bit in a byte at a fixed offset from another pointer.
- `update_byte_flags`: Clears a byte at offset 23 of a structure in a6 using d7, then performs a series of bitwise operations on a byte at offset 256 of a1, including clearing bit 2, and finally sets and clears bit 7 of d0 before calling another routine.
- `frag_lea_moveq_bsr_addq_cmpi_ble_rts`: Short code fragment that loads the address of the memory at an offset from a6 into a3, then loads the address of the memory at an offset from a6 into a5, then loads 3 into d7, then calls another routine, then adds 1 to a3, then adds 4 to a5, then adds 1 to d7, then compares d7 with 6, then branches if the comparison was less or equal, then returns.
- `modify_byte_with_bit`: Reads a byte from offset 256 of a1, clears bit 7 of that byte, calls another routine, and then checks a bit in a byte from a2 to decide whether to set bit 7 of the value in a1 at offset 256.
- `frag_move_subq_add_rts`: Short code fragment that copies d7 to d0, then subtracts 3 from d0, then adds d0 to d0, then copies an indexed entry from a6 to d0, then returns.
- `frag_move_subq_add_rts_2`: Short code fragment that copies d1 to the stack, then copies d7 to d1, then subtracts 3 from d1, then adds d1 to d1, then copies d0 to an indexed entry from a6, then copies the top of the stack to d1, then returns.
- `frag_move_subq_add_addq_rts`: Short code fragment that copies d0 to the stack, then copies d7 to d0, then subtracts 3 from d0, then adds d0 to d0, then adds 1 to an indexed entry from a6, then copies the top of the stack to d0, then returns.
- `frag_move_subq_add_cmp_rts`: Short code fragment that copies d7 to d0, then subtracts 3 from d0, then adds d0 to d0, then compares d6 with an indexed entry from a6, then returns.
- `compare_and_branch_on_equality`: Calls combine_masked_longwords and frag_lsr_subq_moveq_move_eor_dbra_and_r, comparing their return values to determine if a subsequent pair of calls to those same routines should be executed.
- `frag_lsr_subq_moveq_move_eor_dbra_and_r`: Short code fragment that shifts d1 right by 2, then subtracts 1 from d1, then loads 0 into d0, then copies the memory at a0 (advancing a0) to d2, then exclusive-ors d2 into d0, then loops back while counting down d1, then masks d0 with d4, then returns.
- `modify_status_byte`: Checks a byte at offset 49 of a structure in a6; if the byte is zero, it calls another routine, calls another routine, and if the zero flag is set, writes 255 to offset 49 of the structure in a6 before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_ori_rts`: Short code fragment that copies d0 to the stack, then copies the memory at an offset from a1 to d0, then merges 0 into d0, then copies d0 to the memory at an offset from a1, then copies the top of the stack to d0, then returns.
- `frag_st_rts`: Short code fragment that sets a fixed memory address to all ones, then returns.
- `frag_sf_rts`: Short code fragment that clears a fixed memory address, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_longword_to_table`: Waits for a global variable to become zero, then writes a longword consisting of a fixed value and a register value into a table, and finally sets a global variable to a fixed value.
- `handle_interrupt_request`: Reads a byte from a global variable, modifies the interrupt mask, and if a specific bit is set in the CIAA interrupt status, performs bitwise operations on the CIAA control register and writes a value to a table based on the CIAA data register.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `read_joystick_state`: Checks various bits in a table at offset 32 of a0 and the CIAA control register, then processes bits from the joystick data register and a custom bitwise transformation of the joystick data to set bits in d0, finally storing d0 in a global variable.
- `handle_interrupt_request_2`: Reads a byte from a hardware register, sets a bit in the custom chip interrupt request register, and updates a global byte based on the input and a bitmask before returning via an interrupt return.
- `process_interrupt_requests`: Reads the interrupt request register and masks it to keep only bits 0 and 2.
- `setup_interrupt_table`: Masks the interrupt request register, writes four 16-bit values into a table starting at offset 3ef30, and clears a global variable before returning.
- `check_global_status`: Tests a byte at a global variable and returns immediately if it is zero, otherwise it performs several bitwise OR operations on d0 and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `decrement_and_clear`: Decrements a byte-sized global variable and clears a word-sized variable at another global address when the counter goes below zero.
- `clear_interrupts`: Reads interrupt status from a hardware register, masks specific bits, writes the result back to another register, and then executes a loop that decrements a counter while performing no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_memory_free_list`: Traverses a linked list of memory blocks to find a suitable free segment, splits the block if it is larger than requested, and updates the list pointers.
- `update_linked_list`: Performs a linked list insertion or update by comparing a pointer in a0 against a threshold, swapping nodes in a list starting at a pointer in a1, and potentially adding values from a structure at a0 to a structure at a1, then calls another routine.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `modify_global_value`: Multiplies a global variable by two and conditionally applies a bitwise XOR if the carry flag is not set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_recursive_calls`: Calls another routine twice and returns the concatenated result of the registers used in the calls.
- `frag_movea_bsr_rts`: Short code fragment that copies a fixed memory address to a0, then calls another routine, then copies a fixed memory address to a0, then calls another routine, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_interrupt_flags`: Reads a hardware register repeatedly while checking for a specific bit, clears interrupt flags in multiple global variables, and updates structured data with values from registers before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movem_clr_tst_beq_ori_rts`: Short code fragment that saves registers on the stack, then clears a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then tests a fixed memory address, then branches if the last result was zero, then merges 0 into d0, then restores saved registers from the stack, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block`: Copies a block of words from a source pointer to a destination pointer using a nested loop structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_address_table`: Updates a series of memory offsets with new address values while waiting for a global flag to clear.
- `set_address_offsets`: Writes a fixed value to a memory offset and then updates a pointer-based table with incremental address values.
- `set_address_offsets_alt`: Writes a fixed value to a memory offset and then updates a pointer-based table with incremental address values using a different source register.
- `mask_and_shift_value`: Applies a bitmask to an input value and shifts the result right by three bits.

### Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 506096-506368 beside a logged load (code never loaded by a reference run, load address unknown)

- `copy_word_block_2`: Copies a block of words from a source pointer in a0 to a destination pointer in a1 using a loop that runs 31 times.
- `calculate_nibble_offsets`: Calculates a new 16-bit value by extracting nibbles from two input words, performing arithmetic using a multiplier, and recombining them.

### Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 506368-506432 beside a logged load (code never loaded by a reference run, load address unknown)

- `modify_hardware_registers`: Modifies several bytes at offsets 400, 500, and 1e00 from a pointer in a1, and stores a bit-rotated value into a global variable.

### Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf bytes 506672-506880 beside a logged load (code never loaded by a reference run, load address unknown)

- `update_io_registers`: Decrements a byte at an offset from a pointer in a2, and if it reaches zero, performs several writes to hardware registers at offsets 2048, 2560, 3328, and 3840 from a pointer in a6.

### Soccer Kid (1994)(Krisalis Software)[cr FLT](Disk 1 of 4).adf blocks 990-1015 (bytes 506880-520192) (code never loaded by a reference run, load address unknown)

- `frag_move_movem_addq_andi_rts`: Short code fragment that copies the memory just below a2 (moving a2 down) to d0, then restores saved registers from the stack, then adds 1 to d2, then masks d2 with 3, then copies d2 to a fixed memory address, then returns.
