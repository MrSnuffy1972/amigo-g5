# Game 1431 (benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)[t +3 flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01431). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905822 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 55 of 55 named (0 author symbols), 55 of 55 described (7 by rule, 44 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1069 of 1071 (99.81%) |
| Recovered without load address (counted in proof 1, flagged) | 1536 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1869 of 1893 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 8582 |
| code | 5888 |
| data | 885709 |
| padding | 11 |
| unused | 5632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf:bootblock | 1 | 152 | 9 |
| logged disk load from Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf boot-block load | 1 | 389 | 13 |
| logged disk load from Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf boot-block load | 1 | 103 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 139 | 2 |
| logged disk load from Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 389 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 394 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 117 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 88 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 80 | 2 |
| Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf bytes 1024-2560 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)[t +3 flt]/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf: custom-format (non-DOS) disk, 1747 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 2 64 KB pages each with >= 29491 distinct PCs executed, 21789 of its 21915 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x35k slid through memory (an address: 2 64 KB pages each with >= 29491 distinct PCs executed, 21789 of its 21915 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)[t +3 flt]/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf: the boot block reads 10752 bytes from disk offset 854528 to an address (found statically); recovered as a unit
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)[t +3 flt]/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf: the boot block reads 5632 bytes from disk offset 854528 to an address (found statically); recovered as a unit
- runtime segment an address: 113 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)[t +3 flt]/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf: 4 executed PCs whose RAM was overwritten before any snapshot match its bytes 160160-160256: recovered as a unit run at an address
- 954 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 6 first-run image unit(s) in 1 version group(s); 1222 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)[t +3 flt]/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf: bytes 1024-2560 look like code no run loaded: recovered position-independently (load address unknown)
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)[t +3 flt]/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf: 4 logged loads cover 12384 bytes of the image
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)[t +3 flt]/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf: blocks 12-12 (bytes 6144-6656) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.04)
- benefactor (1994)(psygnosis)[cr cls](disk 1 of 3)[t +3 flt]/Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf: blocks 1690-1759 (bytes 865280-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.21)
- 1 never-loaded region(s), 1536 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 4 packed track region(s), 845312 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf:bootblock

- `init_amiga_boot`: This routine initializes the Amiga's custom chip registers during the boot block startup sequence.
- `store_value_and_call`: Stores a value received in a register into a global variable and then calls another routine.
- `patch_and_jump_routine`: Copies a fixed block of code into a global table located near the top of memory, then copies another block into a second fixed location, and finally jumps to the newly written routine.
- `handle_keypress_state`: Reads a global variable, clears it, and uses a value from d0 to branch.
- `increment_word_check_key`: Increments a short stored at the address in a0 and, when the value in d0 equals 95, reads a global variable and either clears it or increments it.
- `clear_word_and_check`: Clears a word at the address in a0 and, when the word in d0 equals 65, loads a value from a global variable and writes 255 into it.
- `patch_table_and_jump`: Writes a rts instruction and a return address into two fixed slots in a table, then overwrites a second slot with a rts and a computed return address, and finally jumps through the table.
- `short_routine_calls_two`: A short seven-instruction routine that calls two unnamed routines and is invoked by one unnamed routine.
- `short_routine_calls_one`: A short seven-instruction routine that calls one unnamed routine and is invoked by two unnamed routines.

### logged disk load from Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf boot-block load

- `copy_and_decompress_buffer`: Copies a block of bytes from a global buffer into a screen buffer, then decodes a compressed data stream and writes the resulting bytes to a video memory pointer.
- `read_shift_and_store`: Reads a word from a location pointed to by a register, writes a concatenated pair of bytes to a global pointer, and returns a shifted version of the original word with a high bit set.
- `shift_and_reload_loop`: Loops a number of times taken from a register, shifting a value right each iteration and reloading it from another routine whenever it becomes zero.
- `initialize_and_run_display_loop`: Copies a block of words from a fixed location into a buffer returned by another routine, then sets two bytes in that buffer when a stored flag equals a specific character.
- `store_two_words_global`: Initializes a counter to two and stores a value returned by another routine into two consecutive words at a fixed address, advancing the destination pointer after each store.
- `adjust_nibbles_combine`: Compares the low nibbles of two values and adjusts each nibble toward the target by incrementing or decrementing, then combines the results.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `clear_memory_array`: Clears a fixed-size array in memory by writing zero to each of its entries in a loop.
- `call_open_libraries`: Reads a global pointer and calls the routine stored at two offsets from it, storing the returned values.
- `advance_string_to_newline`: Walks through a string until it reaches a newline or a null terminator, advancing a pointer as it goes.
- `count_string_dispatch`: Counts characters in a string until a newline or terminator, then calls several routines when the count is non-zero.
- `setup_and_call_routines`: Stores the address of a global variable into several slots and then calls the routines stored at two offsets from a library pointer.
- `call_two_routines`: Reads a global pointer and calls the routine stored at two offsets from it in sequence.

### logged disk load from Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf boot-block load

- `shift_and_store_decompressed`: Copies a block of bytes from a global variable into a buffer, then repeatedly reads a word from memory, decompresses a bit-packed stream, and stores the resulting bytes through a global pointer.
- `read_shift_and_store_2`: Reads a word from a location pointed to by a register, writes a concatenated pair of bytes to a global pointer, and returns a shifted version of that word.
- `shift_and_reload_loop_2`: Loops a count of times, shifting a value right by one bit each iteration and reloading it from another routine whenever it becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_expand_rle_buffer`: Copies a block of bytes from one buffer to another using a pre-decrement loop, then decodes a bit-packed run-length sequence from a source table, expanding each run into repeated bytes written to a destination buffer.
- `expand_rle_buffer`: Decodes a run-length encoded stream from a source buffer into a destination buffer using a series of conditional checks and table lookups to determine expansion counts and data values.

### logged disk load from Benefactor (1994)(Psygnosis)[cr CLS](Disk 1 of 3)[t +3 FLT].adf

- `process_bcd_data`: Performs arithmetic on several registers, executes a BCD adjustment, and then adds a value to a location pointed to by a0 before moving a byte from a5 to a3.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_structure_parameters`: Increments a global byte, checks a value from a table, and then populates various offsets of a structure in a6 with calculated values and pointers.
- `search_and_patch_buffer`: Searches for a specific byte within a buffer by comparing it against a global variable, and if found, copies the subsequent byte to that location.
- `compare_nibble_pairs`: Compares a byte with a value formed by combining nibbles from two other bytes, decrementing the source values in a loop before setting four bytes from a table.
- `combine_nibbles_into_register`: Rotates four registers, masks specific bits from each, and combines the results into register d7 before returning.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (moveq, cmp.l) and then falls through into the code that follows, with no return of its own.
- `update_structure_with_rle`: Updates a global byte and a structure in a6 with values derived from a table, potentially performing two separate updates based on calculated results.
- `apply_bitmask_to_buffer`: Iterates through a list of offsets, reads bytes from a table, and performs bitwise OR and bitwise AND operations on specific offsets of a buffer pointed to by a1.
- `initialize_and_apply_buffer`: Sets several fields in a structure in a6 and then enters a loop that applies bitwise OR operations to a buffer using data from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_64`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `update_audio_hardware_and_table`: Writes a byte to a custom chip control register, sets a value in a custom chip interrupt control register, updates the DMA control register with a value from a global variable, and adds 48 to a longword at offset 120 of a table pointed to by a global variable before returning via an exception return.
- `update_audio_registers`: Sets four pairs of audio channel and length registers using values from a table, updates the interrupt request register, and writes a 32-bit value to offset 120 of a structure in a6.
- `update_entity_state_from_table`: Updates an entity structure by copying values from a data table based on an index calculated from a register, then performs bitwise operations on a field and updates several offsets within the structure using values retrieved from the table.
- `update_entity_properties`: Copies a value from a register to a structure, performs bitwise operations to update a field, and calculates a new pointer based on a table lookup and an index.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_word_if_conditions_met`: Clears a 16-bit word at a specific offset in a structure if a byte at a different offset does not equal one and another value shifted right is non-zero.
- `set_attribute_mask`: Checks a byte at an offset from a pointer; if it is not one, it clears a word in a structure and potentially stores a masked byte from another structure into a specific offset.
- `frag_move_cmpi_bls_rts`: Short code fragment that copies the memory at an offset from a4 to d0, then compares d0 with 64, then branches if the comparison was lower or same (unsigned), then copies d0 to the memory at an offset from a4, then returns.
- `update_structure_counters`: The routine stores a byte from a register through a pointer into a word field of a structure and increments two short counters in the same structure.
- `copy_state_entries`: Copies four word-sized values from four source globals to four destination offsets in a structure pointed to by a6, invoking another routine and a table-driven callback before each copy, then stores a pointer to another routine into a global table and sets a global variable to 25.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `apply_bitwise_or`: Loads a word from a structure and, if it is non-zero, performs a series of bitwise OR operations on that value before returning.
- `frag_move_beq_ori_rts`: Short code fragment that copies the memory at an offset from a4 to d1, then branches if the last result was zero, then merges 0 into d0 (7 times in a row), then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_scaled_parameters`: Updates a 16-bit value at offset 54 of a structure in a6 based on a nibble extracted from offset 5, then calls another routine and performs arithmetic involving a table-driven multiplication to update a 16-bit value at offset 6 of a structure in a3 and a byte at offset 19 of the structure in a4.
- `calculate_scaled_offset`: Extracts nibbles from a byte in a structure, performs a multiplication using a value from a table, and updates a word in another structure and a byte in the original structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_data_table`: Inverts a word in a global variable and, if non-zero, iterates through a list of pointers to populate a structure in a6 with values and calculated addresses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_data_block_list`: Loops through a list of values, moving data from a table into registers and calling another routine for each entry until a null pointer is encountered.
- `transform_word_table`: Iterates through two tables of 16-bit words, performs arithmetic on the nibbles of the second table based on the first, and stores the result back in the second table.
