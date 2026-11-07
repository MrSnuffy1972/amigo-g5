# Game 1870 (flashback (1992)(u. s. gold)[cr interpol][a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01870). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3609152 top-level bytes in 178 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 83 of 83 named (0 author symbols), 83 of 83 described (8 by rule, 54 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1058 of 1058 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 42104 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2370 of 2374 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2960048 |
| code | 7806 |
| data | 449566 |
| padding | 46324 |
| unused | 145408 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:ExpLoad | 1 | 115 | 8 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:INTERPOL | 2 | 267 | 8 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:flashback | 7 | 63 | 1 |
| explode.library | 1 | 475 | 33 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:Failat | 2 | 117 | 2 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:endrun | 1 | 117 | 4 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:bootblock | 1 | 10 | 1 |
| Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 2 of 4).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1200 | 25 |
| memo.cmp (raw code file, load address unknown) | 1 | 0 | 0 |
| memo.cmp (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 812 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- flashback (1992)(u. s. gold)[cr interpol][a2](disk 2 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 2 of 4).adf:cine/memo.cmp: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- flashback (1992)(u. s. gold)[cr interpol][a2](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 4 of 4).adf:cine/memo.cmp: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- flashback (1992)(u. s. gold)[cr interpol][a2](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 3 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr interpol][a2](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 4 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr interpol][a2](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:flashback: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 42104 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:ExpLoad

- `initialize_expansion_loader`: The routine sets up a small stack frame and calls a loader function from a table pointed to by a global variable.
- `initialize_memory_values`: Writes a constant value to a memory location and sets another memory location to a constant, then calls another routine.
- `execute_indirect_call`: Stores a value from a register into a memory location and then calls a routine or pointer located at an offset from a global variable.
- `parse_string_table`: Reads a zero-terminated string from a pointer table and returns the next offset.
- `call_open_library`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.
- `call_indirect_via_table`: The routine calls another routine through an indirect pointer obtained from a global variable and returns a 64-bit value formed by concatenating the two input registers.
- `store_byte_and_advance_pointer`: Stores a byte from a register into the memory location pointed to by a global pointer.
- `parse_string_table_entry`: The routine reads a zero-terminated string from a pointer table, advances the pointer past the string and returns the byte offset of the next entry.

### Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:INTERPOL

- `parse_amigadot_hunks`: Parses a memory block for specific AmigaDOS hunk type codes to perform relocation and memory allocation.
- `decrement_and_branch`: Decrements a register and loops until the register reaches zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `perform_memory_block_copy`: Executes a complex loop that copies or moves bytes between memory locations based on a calculated count and pointer offset.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_shifts`: Performs complex bitwise rotations and shifts on registers while iterating through a loop counter.
- `modify_string_and_flags`: Negates a character in a string, or-sets a byte in a flag, and updates a pointer from a table.

### Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:flashback

- `initialize_game_state`: The routine checks a global variable and a flag, then calls another routine from a table and handles an error path.

### explode.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `update_conditional_pointer_and_value_2`: Checks a value at a memory location and, if certain conditions or characters match, updates a global function pointer and a value.
- `update_conditional_pointer_and_value`: This library routine checks a value at a pointer, and if a specific condition or character matches, updates a global function pointer and a data field with a new address or an incremented value.
- `evaluate_conditional_callback`: Checks a character at a pointer and, if it matches a specific byte, performs conditional arithmetic or updates a function pointer before executing a routine called through a pointer-based structure.
- `execute_conditional_library_callback`: As library code, it checks a value at a pointer, performs conditional arithmetic, and executes a routine called through a pointer-based structure.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `update_library_structure`: Library code that decrements a value in a structure, checks a magic value, and updates a function pointer or related data if certain conditions or integrity checks pass.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_update_library`: Library code that checks magic values and function pointers within a structure to verify its integrity.
- `set_library_flag`: Library code that or-sets a specific bit in a byte at an offset in a structure.
- `verify_library_name`: As library code, it checks if a pointer-based structure contains a specific sequence of characters representing a library name or a string or sequence of words.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `verify_library_string_content`: compares a string or a sequence of words at an offset from a base pointer against a specific library name.
- `check_library_name`: The routine receives a pointer in a0 and checks whether the four words pointed to by that pointer spell a library name string.
- `decrement_timer_and_check_threshold`: Decrements a short integer in a structure and checks if a value in a global variable exceeds a certain threshold, calling a function pointer from a global table if it does.
- `calculate_dynamic_shift_weights`: This routine calculates a set of bit-shift weights or offsets by reading a source buffer and applying bitwise logic.
- `copy_and_expand_buffer_block_2`: A memory copy routine that uses 8-longword-wide transfers for bulk movement and a byte-by-byte fallback for any remaining data.
- `check_pointer_and_decrunch`: Library code that compares two pointers; if they are equal, it calls a routine to decrunch a block of data.
- `copy_and_expand_buffer_block`: This library routine performs a block memory copy or expansion by transferring data in 8-longword increments, followed by a single-element copy or a byte-by-byte copy if any remaining or needed bytes exist.
- `copy_and_expand_buffer_2`: This library routine performs a block memory copy or expansion by transferring data in 8-longword increments, followed by a single-element copy or a byte-by-byte cleanup loop to handle any remaining bytes.
- `copy_and_expand_buffer`: This routine implements a block memory copy or expansion routine that copies data in 8-longword increments, followed by a single-element copy or a loop that increments data or executes a callback-based loop based on a counter or a global variable.
- `decompress_bitstream_to_buffer`: This routine reads a variable-length bitstream to determine a count, then performs a series of memory copies or string-shuffling operations.
- `decode_bitstream_variable_length_integer`: This routine implements a variable-length bitstream decoding mechanism that reads a sequence of bits to reconstruct a multi-bit integer or offset.
- `parse_bitstream_encoded_string`: This routine parses a bit-stream or variable-length-encoded string to extract or calculate an offset or length.
- `extract_bitstream_length`: This routine parses a variable-length bit-encoded or bit-streamed string to calculate a length or offset.
- `parse_bitstream_encoded_table_offset`: This library routine decodes a bit-stream or variable-length encoded string to calculate a pointer offset into a table.
- `parse_bitstream_encoded_structure`: This routine decodes a bit-stream or variable-length-encoded structure or string, calculating offsets or lengths and using the resulting values to calculate a source pointer and range for a subsequent memory copy or processing loop.
- `decode_variable_length_string`: This routine implements a bit-stream or variable-length-encoded string decoding mechanism.
- `decode_bitstream_string`: Decodes a compressed bit-stream or variable-length-encoded string, expanding it into a destination buffer.
- `update_string_buffer`: This library routine implements a bit-stream or compressed-string decoding routine that shifts or copies bytes into a destination buffer.
- `update_string_buffer_with_bitstream`: Decodes a variable-length bitstream to determine a value used for a memory copy or string-shuffling operation.
- `execute_callback_loop`: The routine reads a counter from a structure, increments it, and repeatedly calls another routine and a callback taken from a table indexed by a global variable until the counter condition is met, then calls the routine once more and clears a word in memory.
- `decrement_loop_counter`: The routine receives an unsigned value in an input register, converts it to signed, subtracts one, and if the result is positive, repeatedly decrements it until it reaches -1.

### Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:Failat

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_memory_block`: copies a fixed number of longwords from a memory location offset from a register to the stack, then calls another routine.

### Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:endrun

- `configure_dma_setup_and_string_parse`: Initializes memory buffers and DMA-related parameters, then parses a string or label, updating a global pointer and certain memory-mapped or buffer-relative offsets.
- `return_empty_subroutine`: The routine is empty and performs no operations before returning.
- `enter_infinite_loop`: An empty routine that executes an infinite loop without performing any operations or modifying any registers.
- `configure_dma_setup`: Sets certain memory locations to zero or certain maximum values based on a provided offset, and calls another routine.

### Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Flashback (1992)(U. S. Gold)[cr Interpol][a2](Disk 2 of 4).adf:bootblock

- `check_dos_status`: Calls a function via a pointer and returns a status or a default value based on whether the call returns zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_blitter_and_wait`: The routine programs the Amiga blitter registers BLTCON0, BLTCON1, BLTBMOD, BLTBPTH, BLTDMOD, BLTDPTH and BLTSIZE and then repeatedly polls CIAA_PRA while performing a small blitter copy loop that moves bytes from a table into a set of memory locations.
- `update_blitter_setup`: Checks a value in a global variable and either decrements it or initializes a new value and updates several related memory locations.
- `update_global_values`: If a certain global value equals four, it resets that value to zero and updates two separate longwords by subtracting two and storing the result; otherwise, it increments that global value by four.
- `parse_string_to_table`: Iterates through a source buffer, skipping newline characters, and if an escape character is found, it uses a calculated index to copy a sequence of bytes from a table into a destination buffer.
- `build_copper_list_and_start_dma`: The routine builds a copper list in chip memory by writing word pairs that form copper move and wait instructions, using three pointers obtained from another routine and offsets added to two of them.
- `initialize_dma_and_coprocessor`: Opens the graphics library and allocates memory.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_button_release`: Polls a bit in a hardware register until it is cleared.
- `wait_for_zero`: Polls a global variable until it becomes zero.
- `populate_table_from_buffer`: Iterates through a buffer to find a specific value, then uses that or an offset to write a sequence of calculated longwords into a table.
- `disable_dma_channels`: The routine writes to a global variable five times, the last write storing a value that sets bit 15.
- `update_blitter_control_parameter`: updates a global or hardware-related parameter by performing a table-based lookup or a conditional increment/decrement.
- `update_parameter_with_table_lookup`: Updates a hardware-related control or audio parameter by performing a table-based lookup or a conditional increment/decrement or clamped subtraction/addition based on a masked or masked-and-shifted input value.
- `update_audio_parameter_and_table`: Updates a control structure or pointer based on a masked nibble or a calculated offset.
- `initialize_interrupt_table`: This routine writes a value from a register to a global variable, stores a constant to a global variable, and stores a pointer to itself in a global variable.
- `reset_audio_dma_pointers`: The routine copies eight words from a table in memory to a block of global variables and then writes four values to a global variable.
- `update_control_parameter_2`: Updates a control structure or pointer based on a masked nibble or a calculated offset.
- `copy_control_parameter_offset`: copies a word from a source pointer plus an offset to a destination pointer plus an offset.
- `search_table_and_update_offset`: scans a table for a value or a certain condition, then updates a structure member with a found value and sets a flag based on a comparison.
- `update_scaled_offset_2`: Modifies a value in a structure by adding or subtracting a byte-sized amount basedon a flag.
- `update_scaled_offset`: Calculates a new scaled or offset-based value and stores it in a memory location using a pointer, while also updating a byte in a structure.
- `update_control_parameter`: Updates a control structure or pointer based on a masked nibble or a calculated offset.
- `store_masked_byte`: Extracts the lower nibble of a byte from a structure and stores it in another location within the same structure or a related structure.
- `negate_and_advance_string`: The routine updates a signed byte field using a nibble from a byte at offset 1F and a table of values, then clears the field if its high bit is set.
- `other_routine`: The routine contains an infinite loop with no operations.
