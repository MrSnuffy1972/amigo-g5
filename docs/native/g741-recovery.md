# Game 741 (playhouse strip poker (19xx)(r. brongers)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00741). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903052 top-level bytes in 33 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 87 of 87 named (0 author symbols), 87 of 87 described (32 by rule, 50 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 543 of 543 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 12800 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1695 of 1875 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 761448 |
| code | 7664 |
| data | 93832 |
| padding | 11436 |
| unused | 28672 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| 12 | 1 | 115 | 2 |
| 15 | 1 | 2 | 1 |
| 8 | 1 | 115 | 2 |
| Playhouse Strip Poker (19xx)(R. Brongers).adf:stripboot | 2 | 32 | 7 |
| diskfont.library | 13 | 830 | 53 |
| Playhouse Strip Poker (19xx)(R. Brongers).adf:bootblock | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 243 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 4 |
| Playhouse Strip Poker (19xx)(R. Brongers).adf:strippoker (raw code file, load address unknown) | 1 | 494 | 2 |

## Cases handled

- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4EF9) A7=an address (A7)=an address, slid to exec.library -an address): 65270 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 171 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- playhouse strip poker (19xx)(r. brongers)/Playhouse Strip Poker (19xx)(R. Brongers).adf:strippoker: raw code file no run loaded verbatim: recovered position-independently (load address unknown)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### 12

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `load_and_check_value`: Loads a 16-bit value from a global variable at offset 30 and stores it at offset 34 of a structure in a global variable, then compares that stored word against several decimal constants to select one of several branches.

### 15

- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.

### 8

- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `set_bits_and_store`: Performs a series of byte-wise logical-OR operations on registers, then stores four bytes from a register into consecutive offsets of a structure, and finally loops until a bit in a global variable becomes clear.

### Playhouse Strip Poker (19xx)(R. Brongers).adf:stripboot

- `boot_and_enter_program`: Opens a library, stores several pointers and constants in global variables, jumps to an entry point stored in a global variable, and then loops until a flag bit clears.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `copy_byte_via_pointers`: Reads a byte from the memory location pointed to by a0 and stores that byte into the memory location pointed to by a1.
- `clear_byte_via_pointer`: It writes a byte with the value zero through the pointer in a0, clearing the byte at that address, then returns.
- `store_byte_via_pointer`: Writes the low byte of the data register d0 to the memory location pointed to by address register a0, storing a single byte through the pointer.
- `initialize_d0_register`: Initializes the d0 register by OR-ing its byte with the value zero, which leaves the register unchanged.
- `check_then_update_state`: Calls a routine to obtain a value and returns immediately if it is zero, otherwise calls two more routines, clears the first value, sets a byte in a hardware register, and returns.

### diskfont.library

- `forward_to_entry`: This library routine forwards control to a different entry point within diskfont.library.
- `check_and_set_font_handle`: This library routine calls a routine via a table entry using a library base.
- `initialize_font_library`: Initializes a diskfont library instance by storing its base address, opening two font-related libraries, and allocating a block of memory for font handling, returning the instance pointer on success.
- `unlink_list_entries`: Walks a doubly linked list removing entries whose flags allow it, decrementing a counter, and unlinks the base structure when the list empties.
- `unlink_list_entry`: Checks two flags in a library structure and returns zero if either is set, otherwise unlinks an entry from a doubly linked list, restores the library base, and returns the saved value.
- `set_flag_and_fall`: Sets bit 7 of a byte at offset 14 in a library structure and clears a register, then falls through into the following routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_flag_and_increment_counter`: Library code that clears the most significant bit of a byte at a fixed offset and increments a short integer at another fixed offset.
- `branch_on_counter_and_status`: Decrements a byte counter and branches away when it reaches zero, then checks bit 7 of a status byte and branches away if that bit is clear, otherwise jumps indirectly through a pointer in a6 at offset -18.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `compute_font_offset`: Calculates a signed offset from the difference between two 16-bit values, scaling it by a factor depending on their order, then subtracts entries from a global lookup table for each differing bit pair in two byte arrays.
- `process_font_character_table`: Copies the first eight bytes of a font file into a stack buffer, then searches a table of font character structures for the entry with the largest metric value, updating that entry's fields and flags.
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
- `copy_string_from_stack`: Pushes a pointer onto the stack, loads another pointer from the stack into a register, and copies a value from that stack location into a data register.
- `copy_string_stub`: Pushes two pointers onto the stack, calls a routine at the library base, removes the pushed pointers, restores a register, and returns.
- `calculate_string_length`: Walks through a null-terminated string, incrementing a counter for each byte until the terminator is reached, then returns the total character count in d0.
- `compare_byte_and_load_2`: Loads two pointers from the stack, reads a byte through the first pointer, and compares it against the byte at the second pointer before testing the next byte in a loop.
- `compare_byte_and_load`: Reads a byte from the pointer in a0, decrements a second pointer, and compares the next byte against the read value, loading a negative constant when the comparison is not below.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `advance_pointer_on_match`: Loads a byte from a global variable into a register and loads the byte at the address in a pointer, then compares them.
- `increment_pointer_and_branch`: Increments a pointer register by one and then branches back to an earlier instruction in the routine.
- `return_constant_6`: Returns the constant an address in d0 to the caller without any other effect.
- `find_char_backward`: Calculates the length of a null-terminated string, then walks backwards from its end decrementing a pointer until it finds a matching byte or runs past the start.
- `validate_font_handle`: Saves two registers, reads a handle from the stack, and calls a validation routine with two different constants in sequence.
- `copy_font_name_buffer`: Copies the characters of a font file's name into a buffer, advancing a destination pointer and incrementing a counter for each matching entry found in the directory.
- `decrement_font_length_counter`: Decrements a 32-bit counter at offset -4 of the structure in a6 by the 16-bit value in d4, then decrements a length field at d2 by that value plus 10 and returns if the result is negative.
- `copy_font_byte_loop`: Copies one byte from the source pointer in a1 to a destination buffer and increments a byte counter, repeating while the counter stays within the value in d4.
- `process_font_directory_entry`: Opens a font file from the directory in a3, scans its name for a matching extension, copies the matching characters into a buffer, and for each valid font chunk extracts its header fields into a table, then releases the directory and returns the number of characters processed.
- `copy_font_data_chunk_2`: Decrements two 16-bit counters in the structure at a6 and at d2 by the value in d4, then copies that many bytes from the buffer at a3+8 to the destination computed from the decremented counter, calling another routine to fetch the next chunk.
- `copy_font_data_chunk`: Copies one byte from the source pointer in a1 to the destination computed from the structure at a6 plus a running counter in d3, then increments the counter.
- `copy_file_data_to_buffer`: Reads a chunk of data from a file into a buffer and copies it into a destination block, repeating until the buffer is filled, then reads the remaining bytes and returns the number of bytes copied.
- `unlock_and_free_directory`: Loads a pointer from a global variable and calls two library stubs to release it and set the current directory, then pushes a constant and calls another stub to free the memory.
- `return_zero_constant`: Restores the caller's registers from the stack and the stack pointer, then immediately returns a zero value in d0 without performing any other operation.
- `call_unknown_handler`: The routine invokes another unnamed function and returns.
- `call_external_handler`: The routine calls another unnamed routine and returns.

### Playhouse Strip Poker (19xx)(R. Brongers).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_os_library_loop`: Loads the value 18736 into register d1, sets registers d2 and d3 to zero, points a6 at the base of an Amiga OS library, and calls a routine within that library, then enters an infinite loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_mouse_delta_and_wait`: Invokes a routine through a pointer, then performs a series of assignments to global variables and stores four bytes derived from a 32-bit value into a structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_and_wait_cia`: Calls a library routine, stores two fixed values into global variables, and calls the library routine again.
- `init_mouse_subsystem`: Initializes the mouse subsystem by calling the library setup routine twice, stores the mouse handler address plus a 50 offset into a structure, and writes a 5000 value to a register at offset 33068.
- `decrement_and_loop`: Loads a constant into a register, decrements it in a loop until it reaches zero, and returns when the decrement branch is not taken.
- `compare_and_store_flag`: Copies two 16-bit values from one global structure to another, then compares the copied values against several constants, storing either a negative constant or zero depending on which signed comparisons succeed, and finally stores one when none match.
- `process_mouse_and_jump`: Saves eight data and six address registers, then calls a routine.
- `read_and_clamp_mouse`: Reads the two bytes of the mouse port data register and adjusts each against its previous value, clamping the results to fixed minimums and maximums before halving them twice.
- `pack_flags_from_globals`: Reads two adjacent words from a global variable, derives a bit-packed flag value from them, and stores both the derived value and a modified copy of the source word into two consecutive words of a structure at a fixed address, then returns.
- `init_and_obfuscate_block`: Calls a library routine to fetch a pointer and returns immediately if it is zero, otherwise calls two more library routines, clears the pointer, and sets a byte in a hardware register.
- `obfuscate_memory_block`: Loops over four consecutive 32-bit words, applying an xor, a bitwise complement, and another xor to each one while decrementing a counter, then returns when the counter reaches zero.
- `open_libraries_and_resources`: Opens two libraries, stores their bases, then opens a third library and a custom resource, and returns without doing anything when a flag is set.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `zero_fill_data_area`: Contains a single instruction that leaves the data register unchanged, indicating a zero-filled data area referenced by address rather than executed code.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 16 into d3, then merges 0 into d0, then merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_to_routine`: Executes a single instruction that leaves the data register unchanged and then jumps to another routine, so it performs no work before transferring control.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Playhouse Strip Poker (19xx)(R. Brongers).adf:strippoker (raw code file, load address unknown)

- `read_mouse_and_init`: Reads the mouse data register, splits its low and high bytes into two global variables, stores a fixed value into several other global variables, and calls three other routines before returning.
- `wait_then_check_value`: Waits for a delay counter to reach zero by repeatedly subtracting one, then returns.
