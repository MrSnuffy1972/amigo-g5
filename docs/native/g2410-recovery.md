# Game 2410 (dragon's breath (1990)(palace)[cr trilogy][b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02410). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 119 of 119 named (0 author symbols), 119 of 119 described (57 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 27 of 27 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1920 of 1920 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2057 |
| code | 6828 |
| data | 1040122 |
| padding | 4177 |
| unused | 749056 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| run | 2 | 759 | 12 |
| db | 2 | 210 | 12 |
| joy_acc-handler | 3 | 276 | 23 |
| 9 | 1 | 2 | 1 |
| 9 | 1 | 2 | 1 |
| diskfont.library | 13 | 830 | 67 |
| Dragon's Breath (1990)(Palace)[cr Trilogy][b](Disk 1 of 2).adf:bootblock | 1 | 16 | 1 |
| logged disk load from Dragon's Breath (1990)(Palace)[cr Trilogy][b](Disk 1 of 2).adf boot-block load | 1 | 126 | 2 |

## Cases handled

- dragon's breath (1990)(palace)[cr trilogy][b](disk 2 of 2)/Dragon's Breath (1990)(Palace)[cr Trilogy][b](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- dragon's breath (1990)(palace)[cr trilogy][b](disk 1 of 2)/Dragon's Breath (1990)(Palace)[cr Trilogy][b](Disk 1 of 2).adf:ddata: high-entropy data file (7.99 bits/byte), classified as packed data
- dragon's breath (1990)(palace)[cr trilogy][b](disk 1 of 2)/Dragon's Breath (1990)(Palace)[cr Trilogy][b](Disk 1 of 2).adf: the boot block reads 5632 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- dragon's breath (1990)(palace)[cr trilogy][b](disk 1 of 2)/Dragon's Breath (1990)(Palace)[cr Trilogy][b](Disk 1 of 2).adf: 1 logged loads cover 0 bytes of the image
- 1 packed track region(s), 900096 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_entity_data`: Reads byte values from tables using indices, stores them into structure fields, updates counters and offsets, and performs conditional data copying between memory regions.
- `update_entity_data_2`: Reads a byte from a table using an index, stores it in a structure, updates counters and offsets, then performs conditional data copying and updates based on the read value.
- `copy_entity_data`: Copies entity data from one location to another based on calculated offsets, updates counters, and performs conditional operations on entity structure fields.
- `update_entity_data`: Updates entity data structures by copying values between array elements and modifying counters based on conditions.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `frag_tst_bne_bra`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_move_and_tst_beq_ori_moveq_cmp_blt`: Short code fragment that copies 128 to d1, then masks d1 with the memory at an offset from a1, then tests d1, then branches if the last result was zero, then merges an address into the memory at an offset from a1, then loads 133 into d1, then compares d1 with the memory at an offset from a1, then branches if the comparison was less, then jumps to another point in the code.
- `copy_and_initialize`: Copies a value from one location in a data structure to another, then initializes multiple fields in the same structure based on values from other structures, including setting a counter to zero and copying specific fields.
- `initialize_structure_2`: Initializes multiple fields in a data structure pointed to by a1, including setting a counter to zero and copying values from the structure.
- `initialize_structure`: Initializes multiple fields in a data structure pointed to by a1, including setting a counter to zero and copying values from the structure to memory locations.

### db

- `initialize_program_environment`: This routine checks a global value, copies a block of data from one location to another, and then calls a decryption routine.
- `decrypt_and_jump`: This routine checks a memory condition, modifies a stack-based structure, and then executes code at a pointer before entering an infinite loop.
- `decrypt_and_execute`: Decrypts data using XOR operation with a constant and a counter, then executes a function pointer loaded from a global table.
- `frag_lea_adda_rts`: Short code fragment that loads the address of a location in the code into a4, then adds 32924 to a4, then returns.
- `frag_jmp_move_movea`: Short code fragment that jumps to another point in the code, then copies the memory at an offset from a7 to d1, then copies the memory at an offset from a4 to a6, then jumps to another point in the code.
- `frag_jmp_movem_movea`: Short code fragment that jumps to another point in the code, then copies several registers to or from memory, then copies the memory at an offset from a4 to a6, then jumps to another point in the code.
- `frag_jmp_movem_movea_2`: Short code fragment that jumps to another point in the code, then copies several registers to or from memory, then copies the memory at an offset from a4 to a6, then jumps to another point in the code.
- `frag_jmp_movem_movea_3`: Short code fragment that jumps to another point in the code, then copies several registers to or from memory, then copies the memory at an offset from a4 to a6, then jumps to another point in the code.
- `frag_jmp_movea`: Short code fragment that jumps to another point in the code, then copies the memory at an offset from a7 to a1, then copies the memory at an offset from a4 to a6, then jumps to another point in the code.
- `frag_jmp_movem_movea_4`: Short code fragment that jumps to another point in the code, then copies several registers to or from memory, then copies the memory at an offset from a4 to a6, then jumps to another point in the code.
- `frag_jmp_movea_move`: Short code fragment that jumps to another point in the code, then copies the memory at an offset from a7 to a1, then copies the memory at an offset from a7 to d0, then copies the memory at an offset from a4 to a6, then jumps to another point in the code.
- `frag_jmp_movea_move_2`: Short code fragment that jumps to another point in the code, then copies the memory at an offset from a4 to a6, then copies the memory at an offset from a7 to a1, then copies the memory at an offset from a7 to d0, then jumps to another point in the code.

### joy_acc-handler

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `execute_recursive_call`: The routine performs a direct recursive call to itself before returning.
- `process_joystick_input_and_score`: Updates a score counter with overflow handling and modifies movement vectors based on joystick register bits.
- `update_joystick_state`: This routine reads a bit from a hardware register and updates several fields in a data structure based on the result.
- `initialize_handler_data`: This routine populates a memory block with various values and performs several indirect calls to other routines.
- `initialize_data_structures`: Stores several values into a memory block and performs multiple indirect calls to library functions and other routines.
- `process_indirect_list`: This routine stores a value from a pointer and then iterates through a list of pointers, calling another routine for each until it returns zero.
- `frag_movea_movem_move_jmp`: Short code fragment that copies the top of the stack to a0, then saves registers on the stack, then copies a0 to the stack, then jumps to another point in the code.
- `frag_movea_movem_jmp`: Short code fragment that copies the top of the stack to a0, then restores saved registers from the stack, then jumps to the address in a0.
- `clear_and_setup_handler`: Clears a block of memory by writing zeros to it, sets up a handler routine in a global variable, and calls the supervisor function if a specific bit is set.
- `frag_lea_jsr_move_bne_bra`: Short code fragment that loads the address of a location in the code into a1, then calls another routine, then copies d0 to the memory at an offset from a4, then branches if the last result was not zero, then copies an address to d7, then calls another routine, then jumps to another point in the code.
- `frag_jsr_addq_rts`: Short code fragment that calls another routine, then adds 8 to a7, then returns.
- `frag_lea_rts`: Short code fragment that loads the address of a fixed memory address into a4, then returns.
- `perform_signed_division`: The routine reads two signed inputs, flips the sign flag based on their signs, calls another routine and optionally negates its result, then combines that result with the second input into a 32-bit value.
- `prepare_signed_division`: It saves registers on the stack and clears a local counter.
- `call_and_return_concatenated_2`: Pushes the original contents of register d1 onto the stack, calls a routine that performs a 32-bit division, then restores d1 and returns a 64-bit value formed by concatenating the original d1 contents with the quotient left in d0.
- `frag_move_bsr_tst_rts`: Short code fragment that copies d1 to the stack, then calls another routine, then copies the top of the stack to d1, then tests d0, then returns.
- `perform_long_division`: Calculates a quotient and remainder using a loop of shifts and subtractions or a division algorithm based on the input values.
- `swap_and_rotate_words`: The routine swaps the high and low words of a value in a register, then repeatedly doubles a pair of words while propagating carry, subtracting a constant if the intermediate value exceeds a threshold, and decrements a counter fifteen times.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `frag_jmp_movea_2`: Short code fragment that jumps to another point in the code, then copies the memory at an offset from a7 to a0, then copies the memory at an offset from a4 to a6, then jumps to another point in the code.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### 9

- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.

### diskfont.library

- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `release_library_resources_2`: This library routine attempts to close two libraries and, if successful, modifies a structure to manage memory or resource tracking.
- `release_library_resources`: As library code, it calls functions to close a library and free memory.
- `unlink_and_cleanup_font_resources`: Iterates through a linked list of font resource structures to unlink and free those marked for removal.
- `manage_library_state`: As library code, it either updates a linked list and frees memory or sets a high bit in a byte within a structure.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (bset.b, moveq) and then falls through into the code that follows, with no return of its own.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `frag_subq_beq_btst_jmp`: Short code fragment that subtracts 1 from the memory at an offset from a6, then branches if the last result was zero, then tests bit 7 of the memory at an offset from a6, then branches if the last result was zero, then jumps to another point in the code.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `unpack_font_data_list`: Iterates through a linked list of font string structures to extract character data and metadata into a destination buffer.
- `unpack_font_string_list`: Iterates through a linked list of string structures to copy character data into a destination buffer.
- `copy_string_to_buffer`: Copies a sequence of bytes from a source pointer to a destination buffer.
- `parse_font_file_data`: Iterates through a font data file to extract character dimensions and metadata.
- `populate_font_metadata_table`: Iterates through structured font data to extract character dimensions and memory pointers.
- `parse_font_header_data`: Parses structured font data from a disk source by reading specific byte sequences and populating a buffer with character metadata.
- `read_and_parse_font_data`: Reads data from a disk source and parses it into a structured buffer.
- `validate_and_get_absolute_value`: This routine invokes several system functions to manage file locks, directory paths, and memory, then returns the absolute value of a register.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, removes the stack frame with unlk, then returns to the caller.
- `calculate_bitmask_difference`: This library code calculates a value based on the difference between two word values and adjusts it using a loop that performs bitwise comparisons against a global table.
- `subtract_bitmask_values`: This library routine iterates through a loop seven times to perform bitwise comparisons between two sets of data.
- `apply_bitmask_subtractions`: Iterates through a bitmask up to seven times, subtracting values from a register based on bitwise comparisons against a table and specific byte flags.
- `process_bitmask_table`: This library routine iterates through a loop up to seven times to subtract values from a register.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `update_font_index_metadata`: Iterates through a table of font entries to find the entry with the highest attribute value.
- `load_and_parse_font_data`: Opens a font file from disk, allocates memory, and reads its contents into a buffer.
- `frag_clr_moveq_movea_bra`: Short code fragment that clears the memory at an offset from a6, then loads 255 into d3, then clears d7, then copies d7 to a2, then jumps to another point in the code.
- `update_font_selection_table`: Iterates through a table of font entries to find the entry with the highest attribute value and updates a tracking variable.
- `find_best_font_index`: Iterates through a table of font entries to find the index of the entry with the highest attribute value.
- `search_font_index`: Iterates through a table of font entries to find the entry with the maximum value for a specific attribute.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
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
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `find_string_end`: This library code iterates through a null-terminated string until it reaches the terminator.
- `copy_string_stub`: This library routine acts as a wrapper that calls another routine to copy a null-terminated string from a source to a destination pointer.
- `count_string_length`: This library routine calculates the number of bytes in a null-terminated string.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `compare_strings`: This library routine compares two strings and returns a status value based on their lexicographical order.
- `compare_bytes`: This library routine compares a byte from a pointer with a byte at an offset from another pointer and returns a status value.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `find_byte_in_string`: This library code searches for a specific byte within a null-terminated string and returns a pointer to its location or zero if not found.
- `frag_addq_bra`: Short code fragment that adds 1 to a0, then jumps to another point in the code.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `search_string_backward_2`: This library code calculates the length of a string and then searches backwards from the end for a specific byte.
- `frag_subq_bra`: Short code fragment that subtracts 1 from a0, then jumps to another point in the code.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `search_for_delimiters`: This library routine searches for specific byte values within a string using a backward search.
- `function_epilogue_4`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### Dragon's Breath (1990)(Palace)[cr Trilogy][b](Disk 1 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from Dragon's Breath (1990)(Palace)[cr Trilogy][b](Disk 1 of 2).adf boot-block load

- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_right_bits_with_early_exit`: The routine shifts a 32-bit value right by one bit repeatedly, loading a new 32-bit word from a table pointed to by a pointer each time the shifted value becomes zero, and stops early if a counter reaches -1.
