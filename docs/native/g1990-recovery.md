# Game 1990 (midwinter (1990)(microprose)(de)[cr tt][o]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01990). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (925648 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 72 of 72 named (0 author symbols), 72 of 72 described (4 by rule, 68 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 889 of 889 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 576 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2147 of 2151 instructions identical in start and length, 2 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 7780 |
| data | 903012 |
| padding | 8 |
| unused | 14848 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Midwinter (1990)(MicroProse)(De)[cr TT][o].adf:bootblock | 1 | 152 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 101 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 289 | 16 |
| logged disk load from Midwinter (1990)(MicroProse)(De)[cr TT][o].adf | 1 | 1428 | 34 |
| logged disk load from Midwinter (1990)(MicroProse)(De)[cr TT][o].adf | 1 | 163 | 9 |
| Midwinter (1990)(MicroProse)(De)[cr TT][o].adf bytes 11264-11776 (code never loaded by a reference run) | 1 | 0 | 0 |
| Midwinter (1990)(MicroProse)(De)[cr TT][o].adf bytes 22528-22592 beside a logged load (code never loaded by a reference run, load address unknown) | 1 | 18 | 1 |

## Cases handled

- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: custom-format (non-DOS) disk, 1773 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 492 of the 1001 PCs the canonical run logged (509 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- runtime segment an address (play-lgpwide): 98 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-lgpwide): 270 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: 288 executed PCs whose RAM was overwritten before any snapshot match its bytes 11296-16432: recovered as a unit run at an address
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: 113 executed PCs whose RAM was overwritten before any snapshot match its bytes 22592-23248: recovered as a unit run at an address
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: bytes 11264-11776 look like code no run loaded: recovered position-independently (load address unknown)
- Midwinter (1990)(MicroProse)(De)[cr TT][o].adf bytes 22528-22592 beside a logged load (code never loaded by a reference run, load address unknown): never-loaded code-like bytes recovered position-independently (load address unknown, 62 of 64 bytes decode)
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: 4 logged loads cover 5824 bytes of the image
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: blocks 2-20 (bytes 1024-10752) outside the filesystem hold data, not code (opcode density 0.040, entropy 6.97)
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: blocks 33-43 (bytes 16896-22528) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.66)
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: blocks 46-135 (bytes 23552-69632) outside the filesystem hold data, not code (opcode density 0.006, entropy 6.45)
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: blocks 137-1084 (bytes 70144-555520) outside the filesystem hold data, not code (opcode density 0.017, entropy 6.62)
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: blocks 1087-1279 (bytes 556544-655360) outside the filesystem hold data, not code (opcode density 0.041, entropy 6.26)
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: blocks 1284-1304 (bytes 657408-668160) outside the filesystem hold data, not code (opcode density 0.023, entropy 6.07)
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: blocks 1306-1457 (bytes 668672-746496) outside the filesystem hold data, not code (opcode density 0.026, entropy 6.01)
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: blocks 1459-1658 (bytes 747008-849408) outside the filesystem hold data, not code (opcode density 0.022, entropy 6.63)
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: blocks 1660-1672 (bytes 849920-856576) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.92)
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: blocks 1682-1694 (bytes 861184-867840) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.92)
- midwinter (1990)(microprose)(de)[cr tt][o]/Midwinter (1990)(MicroProse)(De)[cr TT][o].adf: blocks 1704-1803 (bytes 872448-923648) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.67)
- 1 never-loaded region(s), 512 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `xor_buffer_with_data_2` (Midwinter (1990)(MicroProse)(De)[cr TT][o].adf +an address run at an address (5136 bytes, matched to executed PCs)): Calculates a value by shifting a value from a global variable, performs a loop that XORs a buffer with a value, and then performs several word writes. Kind: disk hardware/track check. Pass path: the branch where sVar3 != 0 (the 'else' block containing u3__sub_1314).
- `init_hardware_and_call_routines` (Midwinter (1990)(MicroProse)(De)[cr TT][o].adf +an address run at an address (656 bytes, matched to executed PCs)): Saves the original word at offset 16 of the custom register base, stores the constant a fixed address at a global word, copies the incoming bytes from registers d2 and d3 into global words at offsets 5182 and 5184, clears a global word, then calls a routine and stores its result. Kind: disk hardware/track check. Pass path: unaff_D4w != 0.

## Functions


### Midwinter (1990)(MicroProse)(De)[cr TT][o].adf:bootblock

- `initialize_boot_environment_3`: Disables interrupts and DMA by writing to hardware registers, copies 512 bytes from a source table to a destination buffer, and then calls another routine.
- `store_pending_interrupt_mask`: If the master interrupt-enable bit of the interrupt enable register is set, stores the bits set in both the enable and request registers, limited to the low 14 bits, in a global variable.
- `initialize_boot_environment_2`: Sets a global variable, calculates a bitmask based on values at offsets 16 and 18 of a structure, writes this mask to offset 20 of the structure and to a hardware register, and then enters a loop that decrements a value until it reaches zero.
- `write_port_sequence_clearing_bits`: Writes six port values in turn to the same byte register at a fixed offset, each with a different single bit cleared, and ends with all bits set.
- `wait_port_bit_clear_word`: Repeatedly calls pulse_control_port_count while bit 4 of a byte at offset 4097 from the pointer in a4 remains set, then clears a 16-bit word at offset 22 of the structure pointed to by a3 before returning.
- `update_offset_and_branch`: Moves a word from d0 to d6, checks a word at offset 22 of a structure, calls a routine, modifies d6 based on the value at that offset, and then either adds d6 to the value at that offset or negates d0.
- `pulse_control_port_count`: For each count in its input, writes a pair of control-port values one bit apart so that each pair produces a single pulse, then waits a fixed delay loop before the next pair.
- `decrement_and_loop_structure_counter`: Stores two words into a structure at offsets 16 and 18, calls initialize_boot_environment_2, and then enters a loop that decrements a word at offset 24 of the structure.
- `initialize_boot_environment`: Disables interrupts and DMA, then copies 512 bytes from a source table to a destination buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_rotate_bits`: Copies data from a source table to a destination, rotating bits through multiple iterations based on extracted values.
- `bit_extract_and_store`: Extracts bits from a value, stores them in memory locations, and updates processor status flags while returning the rotated result.
- `rotate_and_accumulate`: Performs bitwise rotation operations on a value and accumulates results through multiple iterations, updating a global counter and status flags.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `close_graphics_library`: Opens the graphics library, stores its handle and position/size values in global variables, performs several operations with forbid/permit guards, then closes the library and returns zero.
- `input_state_machine`: Processes input states from hardware registers, updates counters and arrays based on comparisons, and handles conditional execution paths for game logic flow.
- `memory_clear_and_copy`: Clears specific memory locations and copies data from one area to another while updating counters and handling conditional logic based on global variables.
- `clear_memory_blocks`: Writes zero bytes to a series of memory locations spaced 40 bytes apart, repeating 7 times starting from a calculated address.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (bchg.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `process_character_code`: Reads a byte from a pointer, checks if it matches specific values, and writes a byte to global variables based on the comparison results.
- `fill_memory_with_zeros`: Writes zero values to a memory region of 16384 words, incrementing the pointer after each write operation.
- `coordinate_update_loop`: Updates coordinate values in multiple arrays by performing arithmetic operations, swapping bytes, and incrementing counters in a loop until completion.
- `swap_and_store_word`: Stores the low and high words of a register value at specific offsets in memory, using a swap operation to reorder the bytes.
- `update_multiple_arrays`: Decrements values in three arrays by different amounts, then shifts elements within each array and stores the first element at the end.
- `increment_and_compare_counter`: Increments a counter and performs comparisons with specific values, updating other variables when conditions are met.
- `update_counter_and_arrays`: Increments multiple counters and performs conditional updates on array elements based on comparison results with specific values.
- `adjust_vertical_position`: Reads a vertical position register, adds it to a global variable, and continues updating until the result exceeds a threshold.
- `iterate_and_calculate_values`: Iterates through memory locations until reaching a specific marker, then calculates and stores word values based on byte data.

### logged disk load from Midwinter (1990)(MicroProse)(De)[cr TT][o].adf

- `configure_hardware_and_initialize_tables`: Configures the Amiga custom chipset registers for display and DMA, initializes a series of hardware states through multiple subroutine calls, clears a block of memory, and populates a table by doubling values from a source structure.
- `init_interrupt_vectors`: Saves registers, clears interrupt request and enable bits, loads a vector table with six routine addresses in a loop, stores two more addresses, enables interrupts, restores registers, and returns.
- `modify_interrupt_mask`: Checks a bit in a value read from a hardware register, and if not set, performs bitwise AND operations on a value from another hardware register before writing the result to a specific hardware register.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `initialize_game_state`: Performs a sequence of hardware initializations, populates a structure with various values, calls several subroutines to process data, populates a table by doubling values from another table, and finally stores three longwords into a global table before jumping to a fixed address.
- `setup_and_process_pair`: Sends seven fixed values to a routine, clears a word at offset 86 of the structure in a6 and stores the word 3 at offset 88, then sets up values and calls a routine that returns a flag which decides whether to stop.
- `init_structure_and_check_flags`: Clears the word at offset 86 of the structure in a6 and sets the word at offset 88 to 3, then calls a routine repeatedly with different numeric arguments and stores the results, checking a flag byte at offset 9 of a0 and a returned value to decide when to stop.
- `process_until_negative`: Sends four fixed values to a routine, then enters a loop that repeatedly calls a routine and stops when the longword it returns is negative, otherwise it calls another routine and loops, and finally returns.
- `send_bytes_and_rotate_words`: Sends four byte values to another routine in sequence while clearing and setting structure fields, then rotates three 32-bit words in a structure twice before returning.
- `send_fixed_values`: Calls a routine four times with the values 3, 10, 2, and 1 passed in d0.
- `call_copy_struct_fields`: Loads two fixed addresses into registers and calls another routine to copy fields from one structure to another.
- `increment_structure_field`: Calls another routine and then increments a 16-bit word at offset 52 of a structure in a6.
- `rotate_queue_reset_counter`: Decrements a counter in a structure and, when it reaches zero, resets it to seven and rotates a four-word queue by writing shifted values to a hardware register, then clears two global words and returns via a hardware return.
- `clear_longword_block`: Clears 800 consecutive longwords starting at a global variable obtained from a pointer at offset 56 of the structure in a6, using a counter initialized to 1999 that decrements until it underflows.
- `swap_and_copy_longwords`: Swaps three longwords held at offsets 30, 34 and 38 of the structure in a6 among themselves, then copies the longword at offset 30 into the register at custom offset 200.
- `stub_infinite_loop`: Stores the value negative one into a global short at offset 4036, tests that same location and branches back to the test when it is nonzero, so the loop never exits.
- `copy_and_transform_bytes`: Copies bytes from a source structure into a table, negating values whose high bit is set and handling a special marker byte, then copies sixteen 16-bit values into a destination buffer and returns.
- `write_offset_table`: Reads a short count from the first word pointed to by a0, then writes that many longwords into a global table reached through a pointer at offset 6844.
- `scale_differences_by_root`: Computes the difference between input values and structure fields, then calls a square-root routine to scale those differences before storing a flag byte and adjusting the structure values.
- `initialize_memory_region`: Clears a memory region of 8000 words starting from an address derived from a pointer register.
- `index_table_and_update_flags`: Multiplies an input by ten to index a global table, stores two words from that table into a structure, then calls a routine and updates comparison flags based on the results.
- `compute_offset_pointer`: Computes an address by adding a multiple of the input register to a base at a global table, then returns that address in the output registers without touching memory.
- `compute_integer_square_root`: Computes the integer square root of its input by the bit-pair method, consuming two input bits per iteration and building a partial result.
- `calculate_tile_map_index`: Calculates a pointer to a tile or map data structure by performing coordinate transformations and bitwise operations on input values.
- `toggle_table_bits_from_structure`: Clears bit 0 of a 16-bit value read from offset 96 of a structure and then, for up to 15 iterations, toggles the corresponding bit in four 16-bit words spaced 16000 apart in a 64000-word table, using two source pointers, and returns.
- `copy_struct_fields_loop`: Copies ten 32-bit fields from the source pointer in a0 to the destination pointer in a1, then advances both pointers by 320 and repeats the copy until a counter in d0 reaches zero.
- `build_test_sequences`: Calls a routine to build a joystick test sequence, then repeats three similar loops that store a constant word and its swapped high word into a table before storing the table address and returning.
- `write_joystick_test_sequence`: Loads a 32-bit constant into a register, stores its high word eight times into a table, stores its low word eight times into the table, then clears bit 7 of the joystick test register at offset 54 of the hardware structure and returns.
- `copy_sixteen_longs_from_table`: Loads the address of a fixed constant table into a register, then copies sixteen consecutive 32-bit values from that table into the buffer that the other address register points to, and returns.
- `xor_buffer_with_data_2`: Calculates a value by shifting a value from a global variable, performs a loop that XORs a buffer with a value, and then performs several word writes.
- `copy_byte_buffer_loop`: Saves d1, a3, and a5, calls a routine, restores the registers, and if the carry flag is clear, copies a variable number of bytes from one buffer to another using a loop that decrements a counter.
- `store_pointer_after_call`: Saves d1, a3, and a5, calls a routine, restores the registers, and if the carry flag is clear, stores the resulting pointer into a global variable.
- `copy_byte_buffer_fixed`: Saves d1, a3, and a5, calls a routine, restores the registers, and if the carry flag is clear, copies a fixed number of bytes from one buffer to another using a loop that decrements a counter.
- `xor_buffer_with_data`: Calculates a value by shifting a value from a global variable, performs a loop that XORs a buffer with a value, and then performs several word writes from a table based on an offset.

### logged disk load from Midwinter (1990)(MicroProse)(De)[cr TT][o].adf

- `initialize_hardware_and_process`: Sets bits in a hardware register based on a global value, performs a loop to find a value, and then processes bitmapped data using a loop.
- `write_port_sequence_clearing_bits_2`: Writes six port values in turn to the same byte register at a fixed offset, each with a different single bit cleared, and ends with all bits set.
- `wait_and_clear_word`: Waits in a loop while a bit in a hardware register stays set while calling a routine, then clears a word in a global variable and returns.
- `adjust_global_word`: Moves a word from d0 to d6, checks a word at offset 36 of a global variable, calls a routine, modifies d6 based on the value at that offset, and then either adds d6 to the value at that offset or negates d0.
- `send_pulse_pairs_with_delay`: Reads a byte from a global variable and sets bit 7 or bits 7 and 1 in it, then writes the inverted values to the byte at offset 256 of the structure in a4 a number of times given by a 16-bit register, inserting a fixed 520-iteration delay loop after each write.
- `init_hardware_and_call_routines`: Saves the original word at offset 16 of the custom register base, stores the constant a fixed address at a global word, copies the incoming bytes from registers d2 and d3 into global words at offsets 5182 and 5184, clears a global word, then calls a routine and stores its result.
- `loop_with_subroutine_calls`: Writes a word to an offset of 6 in a structure, saves registers to the stack, calls two subroutines, restores registers, increments a value in d0, and enters a loop that repeats based on the value in d1.
- `decrement_and_branch`: Restores a pointer from the stack to a0, restores two words to d0 and d1, decrements a global word at offset 38, and then branches based on the result.
- `process_bitmapped_data`: Initializes a structure, searches a table for a specific word, calculates an offset based on bitwise operations, and then enters a loop that XORs data and writes it to a destination buffer.

### Midwinter (1990)(MicroProse)(De)[cr TT][o].adf bytes 22528-22592 beside a logged load (code never loaded by a reference run, load address unknown)

- `calculate_xor_sum`: Saves registers, calculates a value using a multiplication and a pointer from a global structure, and then enters a loop that XORs a value with a byte from a table and decrements a counter.
