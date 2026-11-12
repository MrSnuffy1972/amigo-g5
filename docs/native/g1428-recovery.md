# Game 1428 (playhouse strip poker (19xx)(r. brongers)[cr random access]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01428). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903162 top-level bytes in 35 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 103 of 103 named (0 author symbols), 103 of 103 described (32 by rule, 55 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 629 of 629 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 12800 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1841 of 1972 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 761448 |
| code | 5238 |
| data | 124311 |
| padding | 11141 |
| unused | 1024 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf:strippoker | 2 | 87 | 3 |
| 12 | 1 | 2 | 1 |
| 15 | 1 | 2 | 1 |
| 8 | 1 | 2 | 1 |
| Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf:RANDOM_ACCESS | 2 | 139 | 6 |
| diskfont.library | 13 | 830 | 53 |
| Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 253 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 6 |
| logged disk load from Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf | 1 | 67 | 1 |
| logged disk load from Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf | 1 | 35 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 23 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf:strippoker#unpacked | 1 | 494 | 2 |

## Cases handled

- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 4 64 KB pages each with >= 29491 distinct PCs executed, 64714 of its 66024 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 4 64 KB pages each with >= 29491 distinct PCs executed, 64714 of its 66024 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 4 64 KB pages each with >= 29491 distinct PCs executed, 64714 of its 66024 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 4 64 KB pages each with >= 29491 distinct PCs executed, 64714 of its 66024 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 99 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- playhouse strip poker (19xx)(r. brongers)[cr random access]/Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf: 51 executed PCs whose RAM was overwritten before any snapshot match its bytes 484448-484848: recovered as a unit run at an address
- playhouse strip poker (19xx)(r. brongers)[cr random access]/Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf: 24 executed PCs whose RAM was overwritten before any snapshot match its bytes 484896-485120: recovered as a unit run at an address
- RAM snapshot an address: 21 executed PCs whose RAM was overwritten before any snapshot match its bytes 10800-10976: recovered as a unit run at an address
- 2 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 0 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- playhouse strip poker (19xx)(r. brongers)[cr random access]/Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf:strippoker: crunched executable unpacked with ancient scan (ByteKiller 1.3 (xfd), 7632 -> 12800 bytes), 7 of 7 windows confirmed in reference RAM, loaded at an address: recovered as u14
- playhouse strip poker (19xx)(r. brongers)[cr random access]/Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf: 2 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `init_and_obfuscate_block` (runtime segment an address): Calls a library routine to fetch a pointer and returns immediately if it is zero, otherwise calls two more library routines, clears the pointer, and sets a byte in a hardware register. Kind: obfuscated loader/integrity check. Pass path: The routine initializes a block of code/data in a runtime segment, sets up parameters (likely pointers to data and length/checksums) in a structure at an address, and calls the obfuscation routine. A successful check allows execution to continu.

## Functions


### Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf:strippoker

- `unpack_bitstream_to_memory`: Reads a compressed bitstream using a bit reader to reconstruct data into a destination buffer starting at a specific memory address.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### 12

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### 15

- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.

### Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf:RANDOM_ACCESS

- `unpack_initial_data`: Copies a block of data from one memory location to another using a loop, calls a routine at a specific address, and sets a bit in a hardware register.
- `initialize_and_unpack_data`: Clears a block of memory using a loop, calls several library routines to allocate and manage memory, copies two blocks of data using loops, and performs a loop that increments a global variable until it exceeds a threshold.
- `iterate_and_refresh_graphics`: Decrements a byte in a global variable and calls a routine with specific parameters; if the byte reaches zero, it resets the byte to five, calls two other routines, and increments a pointer in a global variable until it reaches a specific limit, at which point it resets the pointer to a new value and returns.
- `decrement_loop_constant_alt`: Decrements a value initialized to 4688 in a loop until it reaches zero, then returns.
- `configure_audio_channels_2`: Sets the channel, length, volume, and period for all four hardware audio channels using a common data source, then enables DMA channels 0 through 3.
- `write_constant_to_offset`: Writes the word value fifteen to an offset of ninety-four of a structure pointed to by a6 and returns.

### diskfont.library

- `branch_to_offset`: As library code, it performs a short branch to an offset of fifty.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `manage_font_resources`: As library code, it calls library functions to open or close resources, manages a linked list by inserting a new node if certain conditions are met, or calls other library functions to clean up if they are not.
- `update_font_list_2`: This library routine traverses a linked list from a pointer in a2, removes nodes that meet a specific condition by updating pointers, and calls a library function to decrement a counter in a structure in a6.
- `update_font_list`: As library code, it checks two 16-bit values at offsets 38 and 40 of a structure in a6; if they meet specific criteria, it calls two routines using a pointer from a global variable, calculates an offset, calls a third routine, and inserts a node into a linked list.
- `set_bit_and_clear_d0`: As library code, it sets bit seven of a byte at an offset of fourteen from a6 and clears d0.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `test_bits_and_jump`: As library code, it decrements a word at an offset of thirty-eight from a6, checks if it is zero, tests bit seven of a byte at an offset of fourteen from a6, and jumps to an offset of negative eighteen from a6.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_font_offset`: Calculates a signed offset value based on the difference between two 16-bit values and a bitwise comparison of two byte arrays.
- `process_font_character_table`: Iterates through a table of font character structures to find the entry with the highest metric value.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `increment_pointer_loop`: This library routine checks if the byte at the address in a0 is zero; if not, it increments a0 and branches back to the start.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `calculate_string_length`: Iterates through a null-terminated string to count the number of characters before the terminator.
- `compare_bytes_in_list`: This library routine compares a byte from a pointer in a0 with bytes from a pointer in a1 using a loop, and returns zero if the end of the a0 string is reached without a match.
- `compare_byte_and_branch`: Loads a byte from a pointer in a0 into d0, decrements a1 and uses the result as a new pointer in a0, then compares the byte at the new pointer with d0 and branches or sets d0 to a constant value.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_byte_at_pointer`: Loads a pointer from a stack offset and a byte from another stack offset, then compares the byte at the pointer to the byte from the stack.
- `increment_and_branch`: This library routine increments the value in a0 and branches to another location.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward`: This library routine calculates the length of a string and then scans it backwards from the end to locate the first occurrence of a specified character.
- `validate_font_handle`: This library routine checks the validity of a font handle by attempting two specific operations with different constants.
- `parse_font_metadata_and_cache_2`: Iterates through a linked list of font metadata structures, calculates the length of string data, and copies that data into a destination buffer while updating a list of character descriptors with pointers and metadata.
- `update_font_buffer_pointer`: Decrements a pointer stored at a negative offset from a6 by a value in d4 and updates the same location with the new address.
- `copy_font_data_to_buffer`: Copies a sequence of bytes from a source pointer in a1 to a destination buffer pointed to by a value at a negative offset from a6, using a loop controlled by a value in d4 and a counter in d3.
- `parse_font_metadata_and_cache`: Iterates through a font data structure to locate specific character metadata, calculates string lengths, and copies character data into a cache while updating a global counter and a list of font entries.
- `decrement_buffer_pointer_and_counter`: Subtracts a 16-bit value from a pointer stored at offset negative 4 of a structure and also subtracts that same value from a counter in register d2.
- `copy_font_data_loop`: Copies a sequence of bytes from a source buffer to a destination buffer pointed to by a value in a structure, incrementing a counter for each byte until the counter reaches a specified limit.
- `read_and_cache_font_data`: Calls a routine to read data from a source, then if the read length matches a specific constant, it populates a structure with a type, a pointer, and several bytes of data while incrementing a counter.
- `execute_library_calls_2`: This library routine calls three different library functions, then checks if a register is negative to return either the negated value or zero.
- `return_zero_constant`: The library routine immediately returns a zero value without performing any other operations or accessing memory.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `read_mouse_and_init`: Reads the mouse data register, splits its low and high bytes into two global variables, stores fixed values into several other global variables, calls three other routines, and waits for a bit in the CIAA PRA register to be cleared before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_and_wait_cia`: Calls a library routine, stores two fixed values into global variables, and calls the library routine again.
- `init_mouse_subsystem`: Initializes the mouse subsystem by calling the library setup routine twice, stores the mouse handler address plus a 50 offset into a structure, and writes a 5000 value to a register at offset 33068.
- `decrement_and_loop`: Loads a constant into a register, decrements it in a loop until it reaches zero, and returns when the decrement branch is not taken.
- `compare_and_store_flag`: Copies two 16-bit values from one global structure to another, then compares the copied values against several constants, storing either a negative constant or zero depending on which signed comparisons succeed, and finally stores one when none match.
- `check_range_and_set_word`: Compares a word in a global variable at offset 30132 to 82; if the value is positive, it compares it to 75, and if that value is negative, it branches to another routine, otherwise it writes 65535 to a global variable at offset 32eb2.
- `compare_and_branch_word`: Compares a word in a global variable at offset 30132 to 58; if the value is positive, it branches to another routine, otherwise it writes 65535 to a global variable at offset 32eb2.
- `check_threshold_and_set_word_2`: Compares a word in a global variable at offset 30130 to 214; if the value is negative, it branches to another routine, otherwise it writes 65535 to a global variable at offset 32eb2.
- `check_threshold_and_set_word`: Compares a word in a global variable to eighty-six; if it is negative, it writes the word value sixty-five thousand five hundred thirty-five to a global variable and returns, otherwise it returns.
- `check_threshold_and_set_word_alt_2`: Compares a word in a global variable at offset 30130 to 142; if the value is negative, it branches to another routine, otherwise it writes 65535 to a global variable at offset 32eb2.
- `check_threshold_and_set_word_alt`: Compares a word in a global variable to seventy-two; if it is negative, it writes the word value sixty-five thousand five hundred thirty-five to a global variable and returns, otherwise it returns.
- `set_global_zero`: Writes the word value zero to a global variable and returns.
- `set_global_one`: Writes the word value one to a global variable and returns.
- `process_mouse_and_jump`: Saves eight data and six address registers, then calls a routine.
- `read_and_clamp_mouse`: Reads the two bytes of the mouse port data register and adjusts each against its previous value, clamping the results to fixed minimums and maximums before halving them twice.
- `adjust_global_values`: Adds a value to a global variable at offset 30126, then updates a second global variable at offset 30124 by subtracting its current value from the high byte of d2, and finally writes a calculated value back to the second global variable.
- `pack_flags_from_globals_2`: Adjusts a value at a global variable based on its sign, adds it to another global variable, clamps both variables within specific ranges, calculates half of each value, and calls another routine before returning.
- `pack_flags_from_globals`: Reads two adjacent words from a global variable, derives a bit-packed flag value from them, and stores both the derived value and a modified copy of the source word into two consecutive words of a structure at a fixed address, then returns.
- `initialize_obfuscated_blocks`: Sets up several pointers and addresses in global variables and calls a library routine or another routine multiple times to process them.
- `init_and_obfuscate_block`: Calls a library routine to fetch a pointer and returns immediately if it is zero, otherwise calls two more library routines, clears the pointer, and sets a byte in a hardware register.
- `obfuscate_memory_block`: Loops over four consecutive 32-bit words, applying an xor, a bitwise complement, and another xor to each one while decrementing a counter, then returns when the counter reaches zero.
- `open_libraries_and_resources`: Opens two libraries, stores their bases, then opens a third library and a custom resource, and returns without doing anything when a flag is set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### logged disk load from Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf

- `initialize_graphics_and_copy_data`: Clears a memory block, allocates a large buffer, and opens a library to initialize graphics structures.

### logged disk load from Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf

- `call_offset_function`: Clears register d1 and calls a function at an offset from a pointer in a6, then returns.
- `process_logic_and_update_pointer`: Calls a function using a pointer from a global variable, decrements a byte in a global variable, and if that byte reaches zero, calls two other functions and increments a global pointer until it reaches a specific address.
- `decrement_loop_constant`: Decrements a value initialized to 4688 in a loop until it reaches zero, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_audio_channels`: Sets the channel, length, volume, and period for four audio channels using a pointer in a0, then enables DMA channels by writing to the DMA control register.
- `write_word_to_offset`: Writes the value 15 to a 16-bit location at offset 150 from a6, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_and_jump`: Restores registers from the stack and jumps to an address stored in a long word at a specific location.

### Playhouse Strip Poker (19xx)(R. Brongers)[cr Random Access].adf:strippoker#unpacked

- `read_mouse_and_init_2`: Reads the mouse data register, splits its low and high bytes into two global variables, stores several fixed values into multiple global variables, and calls three other routines before returning.
- `validate_and_branch_on_values`: Decrements a value in a loop, then performs multiple comparisons on two values loaded from global pointers, writing 65535 to a global variable or specific values to a global variable based on the results, and finally calls two functions and jumps to an address.
