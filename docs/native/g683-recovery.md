# Game 683 (PhotonStorm v1.0 1427): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00683). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2058764 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 48 raw-word lines |
| Functions named and described | pass, 113 of 113 named (0 author symbols), 113 of 113 described (3 by rule, 110 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1054 of 1066 (98.87%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2259 of 2270 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 21928 |
| code | 9620 |
| data | 1133761 |
| padding | 15 |
| unused | 893440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 65 | 6 |
| Disk.1:bootblock | 1 | 22 | 1 |
| logged disk load from Disk.1 boot-block load | 1 | 14 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 481 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 89 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1598 | 72 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- PhotonStorm_v1.0_1427/PhotonStorm_v1.0_1427.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree PhotonStorm_v1.0_1427_hdf, which is recovered file by file; no block outside the filesystem holds data
- PhotonStorm_v1.0_1427/PhotonStorm_v1.0_1427_hdf/PhotonStorm/Disk.1: the boot block reads 2560 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 3 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 81 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 85 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 826 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- PhotonStorm_v1.0_1427/PhotonStorm_v1.0_1427_hdf/PhotonStorm/Disk.1: 3 logged loads cover 0 bytes of the image
- PhotonStorm_v1.0_1427/PhotonStorm_v1.0_1427_hdf/PhotonStorm/Disk.1: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `return_all_ones`: The routine returns the constant value all bits set, which on a 32-bit signed integer is -1.
- `initialize_task_state`: The routine stores a value from a register into a global variable, calls another routine, writes a constant and a pointer to another routine into two fields of a structure pointed to by a register, writes a constant and a pointer to another routine into two more fields of the same structure, then performs an indirect call through a function pointer stored in the structure.
- `initialize_task_state_2`: Calculates a running sum or checksum using a loop, copies a block of data from one location to another, and clears a block of memory with zeros.
- `call_other_routine_and_return_input`: The routine calls another routine and then returns the value that was in D1 on entry.
- `call_indirect_through_table`: The routine invokes another routine via an indirect call through a table entry and returns the combined value of two input registers.
- `return_concatenated_input_values`: The routine calls another routine via a pointer stored in a global variable and then returns the concatenation of the two input values.

### Disk.1:bootblock

- `initialize_boot_io`: The routine obtains a jump table entry from a global variable and, if present, repeatedly writes a pointer to that entry and constant values into a structure pointed to by a1, then invokes another routine from a global variable until a flag byte in the structure becomes zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state`: The routine resets a set of global variables, repeatedly calls other routines and itself, and then dispatches through an indirect jump table.
- `reset_game_state_flag`: The routine writes a constant to a global variable and clears a function pointer, then returns the low byte of D0 ORed with five.
- `clear_global_and_table_entry`: The routine stores the constant 3000 into a global variable and then clears the entry for another routine in a table by writing a null pointer.
- `set_palette_and_restore`: The routine selects two table base addresses depending on whether a global pointer is non-zero, calls another routine, saves and clears a global variable, builds a 32-bit value from two outputs of that call, then invokes a graphics routine three times with the constructed parameters and restores the saved global variable before returning the constructed value.
- `set_global_pointers_and_call`: The routine conditionally sets two global pointers to different values and then calls another routine.
- `set_globals_and_call_other`: The routine sets two global variables to constant values, optionally changes them based on whether another routine is non-zero, calls that routine, saves and clears a global variable, then calls a helper routine three times with arguments built from the helper's outputs and the saved globals, restores the saved global variable, and returns a combined value from the helper's outputs.
- `set_palette_and_restore_2`: The routine selects two palette-related table pointers based on a flag from another routine, calls that routine, reads a global variable and a result pair, clears the global variable, then invokes a graphics routine three times with the obtained values and a constant, and finally restores the saved global variable before returning the combined result.
- `set_global_flags`: The routine writes the values 2, 1 and 0 sequentially to a global variable and then reaches a bad instruction that truncates control flow.
- `randomize_three_globals`: The routine calls another routine three times to obtain a random word.
- `copy_table_and_clear_buffers`: The routine copies a small fixed-size block from one memory area to two other memory areas, then conditionally writes to a global variable, calls another routine, clears a large buffer pointed to by a register argument, calls another routine, and finally xors a word in a structure pointed to by A5.
- `empty_routine`: The routine contains no usable instructions and ends immediately.
- `store_byte_via_global_ptr`: The routine is not reachable with valid instructions, so it appears empty.
- `poll_input_until_timeout`: The routine repeatedly calls another routine until a counter expires, then writes three constant values to a global variable in sequence, clears a global variable, and returns the last value from the called routine plus two.
- `wait_for_trigger_byte`: The routine repeatedly calls another routine until it returns a specific byte value, then returns.
- `set_global_and_loop`: The routine writes a constant to a global variable, calls another routine, writes a different constant to the same global variable, then repeatedly calls the same routine eight times while decrementing a counter.
- `loop_call_other_routine`: The routine sets a short counter to eight and repeatedly calls another routine, decrementing the counter each iteration, until the counter reaches -1.
- `restore_global_and_call_thrice`: The routine saves a global variable, clears it twice, builds two 32-bit values from input registers and a table indexed by an input register, then calls another routine once with no arguments and three times with those values and several input registers, restores the saved global variable, and returns the combined 64-bit value formed from the two built values.
- `restore_global_and_call_helper`: The routine saves a global variable, clears it twice, calls another routine with a constant argument three times using the incoming registers, then restores the saved global value and returns the concatenation of two input registers.
- `scan_message_table`: The routine computes an index from a global variable and searches a zero-terminated table of ASCII strings for the entry "yeak". It advances a pointer through the table entry by entry until the index is exhausted, effectively selecting a table entry.
- `decode_digit_and_load_table`: The routine stores a global variable into a RAM location, normalises a digit value, splits a small signed value into quotient and remainder, and then uses the quotient as an index to load a series of 16-bit entries from a table into a global variable and two other globals.
- `decode_digit_and_load_table_2`: The routine stores a global variable into a RAM location, normalises a control byte, extracts a decimal digit from the RAM value and writes the quotient and remainder to temporaries.
- `shift_global_pointer`: The routine checks a global variable; if it is not a sentinel value, it dereferences it and either sets the global to a sentinel when the value is negative or left-shifts the dereferenced value, adds one, and stores the result back into the global variable.
- `reset_game_state`: The routine clears several global variables to zero and specific constant values, then calls a few other routines, sets up a few globals including a pointer to itself and a constant, and loops calling another routine until a global counter passes a threshold, after which it writes two byte values to memory and returns.
- `reset_game_state_2`: The routine clears a number of global variables to zero and to specific constant values, calls several other routines, then enters a loop that repeatedly calls another routine until a global variable reaches a threshold, and finally writes constant values to a global variable and two other memory locations before returning.
- `modify_register_and_mask`: Decrements a value through a pointer and performs a bitwise AND operation between a register and a value through a pointer.
- `adjust_global_and_copy_word`: The routine updates a global variable by adding a constant and then conditionally resets or sets it based on its low bits, calls another routine, and copies a word from one pointer to another before storing a stack value through an address register.
- `reset_game_state_3`: The routine clears a global variable and writes constant values to several global variables, then calls three other routines.
- `reset_game_state_4`: The routine clears a global variable and then writes a series of constant values to several global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_table_and_seed`: The routine clears a table at a fixed address and then repeatedly writes pseudo-random values into a caller-provided buffer using another routine, also writing constant words into the buffer.
- `wrap_short_table_values`: The routine iterates over a short table, subtracts a global variable from each entry, writes the result back, and if the result is negative adds a constant to it.
- `iterate_and_test_value`: The routine repeatedly calls another routine to obtain a short value and checks it against a range derived from a short returned via the D1 register and a short returned via the A0 register.
- `randomize_table_entries`: The routine clears a global variable, then repeatedly calls another routine to obtain a short value and writes transformed values into three separate short locations derived from two output pointers and offsets.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scale_value_by_divisor`: The routine returns the input value unchanged when an uninitialized word is zero; otherwise it adds to the input the integer quotient of the product of the input and that word divided by the difference between a global variable and that word, using 16-bit arithmetic.
- `apply_tile_mask_to_plane`: The routine computes an index from an input value and a global table, stores the index and a constant into a structure pointed to by a pointer, then uses two masks loaded from a table indexed by the input value to combine with four 16-bit words from a plane buffer pointed to by a pointer, writing the results back to the buffer at offsets corresponding to four screen planes.
- `apply_sprite_mask_layers`: The routine computes an offset from input values and a global table, stores the offset and constants into a four-word output structure, then blends four 4KB layers of a destination buffer using per-layer masks and a pair of 16-bit masks selected from tables, writing the blended results back through a pointer.
- `unknown_routine`: The routine calls another routine four times and returns a 32-bit value formed by concatenating the low 9 bits of an input word with a 16-bit input word.
- `blend_tile_pixels_2`: Performs bitwise masking and logical operations on pixels or or even bit-plane data while using a pointer to a source or destination buffer.
- `blend_tile_pixels`: The routine blends or masks a 4x4 tile of 16-bit words from a source buffer into a destination buffer using two mask tables, with a different combine mode when a global flag is set.
- `expand_and_copy_buffer`: Checks a value, updates a global pointer, and uses nested loops to copy and expand data from a source buffer to a destination buffer.
- `clear_zero_table`: The routine clears a block of sixteen 32-bit entries in a global table by writing zero to each entry in a loop.
- `initialize_photon_storm_object`: The routine calls several other routines, sets a global variable to a constant, clears several word fields in a structure pointed to by a register, writes a set of constant word and long values into that structure, calls another routine, and then writes the same constant word value three times to a global variable.
- `jump_to_empty_routine_6`: Jump stub: passes control straight on to empty_routine_6.
- `reset_custom_chip_registers`: The routine writes a constant to eight consecutive long words starting at a custom chip base address, then writes four constants to four global variables.
- `read_keyboard_serial_byte`: The routine reads a byte from the CIAA keyboard serial register when a global flag is set.
- `return_immediately`: The routine is empty.
- `fill_palette_table`: The routine writes a fixed header into a table at a fixed address and then iterates over a short array pointed to by a register, accumulating a running sum and building a 16-bit value from the array elements.
- `empty_routine_2`: The routine contains no instructions and performs no operations; it returns immediately.
- `empty_routine_3`: The routine contains no instructions and performs no operations; it is an empty function.
- `empty_routine_4`: The routine contains no instructions and performs no operations.
- `empty_routine_5`: The routine contains no instructions and performs no operations.
- `reset_interrupt_and_dispatch`: The routine writes to a custom hardware register and the interrupt request register, then calls another routine and copies a value from a table indexed by a global variable into memory pointed to by the stack.
- `update_input_state`: Performs conditional checks and updates a global variable based on logic involving bitmasks or input-related register or flag state.
- `unknown_routine_2`: The routine calls another routine and then enters an infinite loop.
- `empty_routine_6`: The routine contains no instructions and performs no operations.
- `clear_global_byte_flag`: The routine clears a byte flag in a global variable by masking the byte with an even mask after setting it to a constant.
- `modify_global_with_pointer`: Reads a value from a pointer, adds it to a global variable, and then masks a value with a constant.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_self_modifying_update`: Updates a global variable using a value retrieved from a pointer and a constant, then masks a value.
- `write_byte_and_mask`: The routine writes the low byte of a register into a global variable and then overwrites that global variable with a constant.
- `set_flag_and_copy_bytes`: The routine sets a byte pointed to by a pointer by OR-ing it with a constant, then copies three consecutive bytes from that memory location into a global variable, performing arithmetic on the copied values with clamping to zero and a decrement when a condition holds, and finally returns a 32-bit value formed from two input words.
- `copy_four_pointer_tables`: The routine clears a global variable if it is non-zero and then copies the contents of one table to three other tables by repeatedly storing each source word into three destination words, advancing all four pointers each iteration until the loop counter reaches -1.
- `copy_player_sprite_tiles`: The routine copies a small block of 16-bit tiles from a source table into four parallel destination buffers derived from a global variable.
- `unknown_routine_3`: Routine is empty.
- `self_modifying_loop`: The routine walks a global short table starting from a global variable, decrementing a counter from six while skipping zero entries.
- `normalize_and_reset_global_state`: The routine normalizes an input value in D1 to 499 when it is -500, then calls another routine.
- `invoke_state_reset`: Calls another routine without any further instructions.
- `copy_sprite_tiles`: The routine walks a table of entries stored in a global variable and for each entry copies a block of 16-bit words from a source area to a destination area, using offsets derived from the entry fields and a register value.
- `call_and_return_result`: The routine calls another routine and returns the value it receives.
- `transform_global_variable`: Updates a global variable using a multiply-and-add operation with a constant.
- `execute_indirect_call`: Uses a value from a pointer as an index into a table and calls the routine found at the resulting address.
- `dispatch_via_table`: The routine reads a short value from a pointer in a0, shifts it left by two, adds the result to a fixed base address to form a pointer, and calls the routine found there.
- `rotate_word_table_entries`: The routine adjusts a table pointer and range limits using a global variable and input values, then iterates over a set of four-word entries.
- `rotate_word_array`: The routine reads a length and a start offset from a structure pointed to by a register, clamps the start to a global limit and computes a word count.
- `rotate_word_table_entries_2`: The routine reads a short from the first argument and uses it as an index into a table of signed bytes to compute a rotation amount, then iterates over a table of words pointed to by the second argument.
- `rotate_and_mask_words`: The routine rotates four 32-bit words derived from a four-word block by a nibble-derived amount and masks them with a computed mask before writing them through a pointer.
- `rotate_and_mask_blit_words`: The routine rotates four 16-bit words from a source block by a nibble-derived amount, builds a mask from the rotated words, and writes the rotated words into a destination block using masked OR with the existing contents.
- `other_routine`: The routine iterates over a block of four 16-bit words pointed to by a pointer, rotates each word by a variable amount derived from the low nibble of a register, combines the rotated words with a mask, and writes the masked results into a destination area whose address is computed from an index table and an offset.
- `rotate_and_mask_words_2`: The routine rotates four 32-bit values built from registers and memory words, combines them with a mask derived from their OR, and writes the masked results back to a word buffer using a rotating index.
- `rotate_word_masks_and_apply`: The routine rotates four 32-bit values by a variable amount, builds a mask from their OR and applies the mask to four 16-bit words in a buffer, repeating the masked write for the high halves.
- `decode_tile_row`: The routine walks a table of 16-bit words pointed to by a pointer argument and, for each non-zero entry, writes a byte derived from a small lookup table into a destination buffer whose base is computed from another table and an offset argument.
- `rotate_word_array_2`: The routine walks a word array pointed to by a parameter and rotates each 32-bit word formed from consecutive words using a shift amount derived from a table and a stack value.
- `fill_table_with_incrementing_values`: The routine writes a sequence of incrementing 32-bit values into a global table.
- `clear_zero_table_2`: The routine clears a fixed number of 32-bit entries by writing zero through a pointer, decrementing a counter until it reaches -1.
- `decrement_and_update_color_table`: The routine decrements a global counter and updates a table of short values by adding the original counter value to each entry, then clears the counter if a condition is met.
- `empty_routine_7`: The routine contains no instructions and returns immediately.
- `accumulate_table_values`: Iterates through a null-terminated list of bytes to calculate a running sum, and then uses those values to call another routine with a modified offset.
- `accumulate_and_draw_stars`: The routine accumulates a sum from a table indexed by a byte string and a global variable, then iterates over the same string, calling another routine with a decreasing value derived from the sum while updating the value with table entries.
- `process_sprite_table`: The routine walks a byte table pointed to by a pointer, using each byte as an index into a short table to fetch a value, accumulates an offset with a short from the fetched entry, and calls another routine with a computed position and a value from a register.
- `empty_routine_8`: The routine contains no instructions and returns immediately.
- `execute_sequence_of_routines`: Calls a sequence of other routines and returns a concatenated value from two registers.
- `store_register_to_global`: Stores the contents of two registers into a single global variable.
- `reset_game_state_5`: The routine clears a global variable and a table entry, then calls another routine and loops until the variable is zero.
- `disable_dma_channels`: The routine writes to a global variable four times, with the first write setting a value that has bit 15 set, which on the Amiga corresponds to enabling DMA channels via the DMACON register.
- `set_objective_data`: The routine parses a buffer of bytes as colon-terminated fields, and when the first field matches an empty entry in a table, it writes a byte from the buffer into that table entry, initializes a block of 14 words at the corresponding table pointer to zero, stores a sentinel and a self-pointer, and continues parsing the buffer until the second byte of the first field is zero.
- `parse_and_store_string`: The routine walks a buffer of colon terminated strings, looks up each two byte index in a table of entries, writes a byte from the buffer into a per entry byte and records the pointer after the header in the entry.
- `parse_script_command`: The routine walks a null-terminated command string pointed to by a field in a structure passed in A1, interpreting characters as operators.
- `set_dma_control_mask`: The routine computes a mask from an input value and stores it in a global variable.
- `validate_hex_digit_string`: The routine checks a pointer in A0 byte by byte, decrementing a counter starting at three.
- `scan_input_bytes`: The routine walks a byte stream pointed to by a pointer, decrementing a counter each iteration and returning when the counter reaches -1.
- `set_audio_channel_pointer`: The routine reads a character from a pointer, converts it to a digit value with a fallback, then writes a computed address to a custom chip register area and sets a control word, and finally writes a bitmask with the high bit set to a global variable.
- `set_custom_chip_register`: The routine writes a 32-bit value and a 16-bit value to two consecutive custom chip registers using an offset derived from an uninitialized register, then sets a global variable to a bit mask derived from the same register.
- `initialize_table_entries`: The routine initializes a global variable to zero and sets a base pointer.
- `set_audio_channel_pointer_2`: The routine writes a computed value to the audio channel pointer register for channel 0.
- `set_audio_channel_volume`: The routine writes a value derived from a register to the AUD0LCH hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_2`: This routine is empty and performs no operations before returning.
