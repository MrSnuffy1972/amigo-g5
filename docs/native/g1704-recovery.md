# Game 1704 (new zealand story, the (1989)(ocean)[cr acc][t +3 band]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01704). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (933520 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 117 of 117 named (0 author symbols), 117 of 117 described (12 by rule, 102 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5663 of 5673 (99.82%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 6348 of 6348 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11057 |
| code | 27216 |
| data | 877826 |
| padding | 13 |
| unused | 17408 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf:bootblock | 1 | 6 | 1 |
| logged disk load from New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf boot-block load | 1 | 106 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1570 | 91 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 363 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1336 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2952 | 1 |

## Cases handled

- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: custom-format (non-DOS) disk, 1724 non-empty blocks: matched against track-loaded segments
- 10 first-execution byte records contradict the logged opcode word (first an address): bad captures, the opcode word is used
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: the boot block reads 6656 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 1118 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 157 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1321 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 2950 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 8 unexecuted gap bytes taken from the RAM snapshot; 10 not decodable
- self-modifying code at an address (3 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: 1 logged loads cover 6656 bytes of the image
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: blocks 15-117 (bytes 7680-60416) outside the filesystem hold data, not code (opcode density 0.007, entropy 5.46)
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: blocks 121-294 (bytes 61952-151040) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.95)
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: blocks 297-690 (bytes 152064-353792) outside the filesystem hold data, not code (opcode density 0.021, entropy 5.99)
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: blocks 693-993 (bytes 354816-508928) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.83)
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: blocks 1001-1027 (bytes 512512-526336) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.76)
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: blocks 1034-1634 (bytes 529408-837120) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.15)
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: blocks 1639-1689 (bytes 839168-865280) outside the filesystem hold data, not code (opcode density 0.023, entropy 5.90)
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: blocks 1693-1711 (bytes 866816-876544) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.20)
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: blocks 1715-1733 (bytes 878080-887808) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.84)
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: blocks 1737-1755 (bytes 889344-899072) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.25)
- new zealand story, the (1989)(ocean)[cr acc][t +3 band]/New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf: blocks 1759-1759 (bytes 900608-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.96)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `wait_and_retry_dma_setup` (runtime segment an address): Reads the interrupt request register and returns unless bit 1 is set, then configures DMA and disk controller registers and waits for a magic word in RAM. When the word matches it increments a counter in a structure and loops while the counter is below ten, otherwise it increments a different counter and loops while that stays below five. Kind: Disk hardware check (DSKLEN) with retry loop. Pass path: Returns 0 (success) via u2__sub_089C after the DMA setup loop completes (the branch where unaff_D6w reaches 5 without hitting the -1/-3 error paths).

## Functions


### New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf:bootblock

- `boot_decode_bitstream_into_memory`: Boot block entry that sets up a control block, calls one library routine, and then decodes a bit-coded stream into memory with the same kind of bit-reading loop as the other decoders.

### logged disk load from New Zealand Story, The (1989)(Ocean)[cr ACC][t +3 Band].adf boot-block load

- `decompress_bitstream_to_memory`: Decompresses a bitstream into a destination buffer by fetching shifted words and processing bits through a bit buffer, then proceeds to copy data and set interrupt vectors.
- `fetch_shifted_word_and_store`: Reads the longword just before the address pointer, writes its low byte into a location given by another pointer, and returns that longword shifted right by one bit with the top bit set.
- `shift_bit_buffer_counted`: Shifts a bit buffer right one step per count, reloading it through an unnamed routine whenever it empties, and stops when the count runs out.
- `copy_word_table`: Copies 31 words from a source table at a specific address to a destination table starting at offset 2 of a second address, incrementing the destination pointer by 4 each iteration.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `install_decompress_and_set_vectors`: Jumps to the routine at offset 1024 to install and decompress code, then stores the address of a table of five interrupt vectors into a global variable.
- `install_and_decompress_code_2_2`: Disables interrupts, copies 1024 words from a ROM location to RAM at offset 1024, decompresses a bitstream into RAM using a bit buffer, patches a 109 byte sequence in the copied block, and jumps to the start of the copied block.
- `decompress_and_expand_stream`: Decompresses a bit-packed stream into a buffer by reading bits from a shifted word and copying or repeating byte values, then expands a run-length-encoded section into a fixed 1792-byte region and copies that region into another buffer.
- `copy_and_decompress_memory`: Copies a run of bytes from a source buffer to a destination buffer, shifting bits and fetching more data from a bit buffer as needed, then decompresses a RLE-style stream from a source table into a destination table and copies a fixed block of memory.
- `call_decompress_then_setup_disk`: Calls the decompression routine and returns if the result is non-negative, otherwise loads a word from offset 12 of the structure in a6 into a data register and jumps to the disk setup routine.
- `decode_palette_rle`: Reads a compressed byte stream from the palette data at the palette pointer, decoding run-length markers (a byte of 128 ends a run, a negative byte counts as a repeat), and calls the store routine for each decoded byte.
- `copy_bytes_loop_store`: It masks d0 to seven bits, copies successive bytes from the source in a0 into d1, calls the store routine for each byte, and decrements the loop counter until it reaches zero.
- `store_byte_and_update_counters`: Stores the byte in d1 through the pointer in a1, then checks bit 7 of d5 and increments it until it reaches 40, at which point it clears d5.
- `copy_and_expand_stream`: Copies a reversed block from one memory region to another, then processes a stream by copying bytes and repeating a value a number of times until a counter reaches zero.
- `copy_pair_bytes_loop`: It clears d1, copies two consecutive bytes from the source in a1 into d0 and d1, stores the first byte to the destination in a0, decrements d3, and repeats until d3 is exhausted before restoring registers and returning.
- `set_joystick_bitmask_and_wait`: Sets bits 3 to 7 and bit 2 of the joystick port register, clears bit derived from a structure value, then waits until bit 5 of the keyboard register is clear.
- `toggle_joystick_bit`: Sets bits 3 to 7 and 7 of the joystick port register, then toggles a bit derived from a structure value by clearing and setting it.
- `setup_disk_controller_with_retries`: Saves several registers into a structure, then repeatedly configures the disk controller and DMA registers and waits for the keyboard and disk status bits to settle, retrying a few times before giving up.
- `save_registers_call_routines`: It saves d7 and a structure word on the stack, calls the first routine, restores the structure word, calls the second routine, restores d7, and jumps back to the calling routine.
- `wait_and_retry_dma_setup`: Reads the interrupt request register and returns unless bit 1 is set, then configures DMA and disk controller registers and waits for a magic word in RAM. When the word matches it increments a counter in a structure and loops while the counter is below ten, otherwise it increments a different counter and loops while that stays below five.
- `check_bytes_call_helper`: Checks whether two consecutive bytes equal 68 and 137, advances the pointer when they do, loads an address into a4, sets d7 to 10, and calls a helper routine, then increments a structure counter and branches based on its value.
- `decrement_counter_loop`: Decrements a 16-bit counter at offset 8 of the structure in a6 and increments the one at offset 12, adds 5632 to the value at offset 4, clears the word at offset 0, and loops the first decrement until it reaches zero before calling a routine and clearing d0.
- `swap_stack_pointer`: Saves d0 on the stack, calls a routine, restores d0 from the stack, then replaces the stack pointer with a base address loaded from a global variable before returning.
- `run_helpers_check_byte`: Runs a helper routine four times with the constant 4, the constant 16, and the constant 4 twice in d1, then reads a byte from a structure in a4 and returns negative one unless that byte equals 255, in which case it loads a byte from offset 2, scales it by 512, adds a longword from offset 4 of a structure in a6, and calls a helper routine.
- `compute_address_loop`: Builds an address from a byte at offset 2 of the structure in a4 scaled by 512 plus the value at offset 4 of the structure in a6, calls a routine with that address, then calls two more routines and loops comparing a value at offset 4 until it matches 17529.
- `tiny_move_moveq_rts`: Small routine of 3 instructions that copies d0 to d1, then loads 254 into d0, then returns.
- `scan_for_marker_word`: Reads words from a pointer, advancing it, until a word equals 17529, then clears d0 and returns.
- `sum_array_words`: Clears d0 and d2, then loops d1 times adding each word from a pointer to the running total in d0.
- `combine_bit_patterns`: Loops d1 times combining bits from two source pointers with the mask 85 and writing the result to a destination pointer, then restores a2 to a0.
- `clear_and_set_structure_words`: Clears the word at offset 12 of the structure in a6 and stores 82 at offset 14, then returns.
- `process_input_timing_loop`: Loops while bit 4 of the keyboard register is set, adjusting a structure value by a direction flag with delay loops, then clears structure fields and selects a delay constant based on its value.
- `set_audio_and_blitter_registers`: Clears bit 6 of the keyboard register, then writes a series of constant values to the audio and blitter registers before returning.
- `clear_registers_memory`: Clears the CPU interrupts, writes constant values to several video and blitter registers, clears a 8191-word block starting at address an address, clears eight word registers, clears a 31-word block of sprite registers, and re-enables interrupts before returning.
- `setup_interrupt_vectors`: Opens a library and, if successful, sets up five interrupt vectors with a trap-and-return sequence, installs a video interrupt handler, and stores a set of computed source addresses in a table, then returns.
- `install_handlers_and_decompress_data`: Writes the trap-and-rts sequence to five global vectors, sets a video interrupt handler and installs a copy of itself at the reset vector, then decodes a bit-packed table and decompresses it into a memory block.
- `copy_source_blocks`: Computes two source addresses from a value in d0, then loops 17 times to copy a 6-byte chunk from each source into a destination table at a fixed address.
- `copy_source_blocks_alt`: Computes two source addresses from a value in d0, then loops 17 times to copy a 6-byte chunk from each source into a destination table at a different fixed address.
- `copy_from_a2_blocks`: Computes two destination addresses from a value in d2, then loops 17 times to copy a 6-byte chunk from the address in a2 into each destination.
- `compute_indexed_offsets`: Reads a 16-bit index from a global variable and multiplies it by 40 to form an offset.
- `run_blitter_transfers`: Sets up source and destination addresses and calls a blitter transfer routine twice, then adjusts the destination by two different offsets and calls a second blitter transfer routine twice before returning.
- `setup_blitter_transfer`: Calls another routine, then writes values to the blitter registers, storing the source address in one register and the destination address in another before returning.
- `setup_blitter_transfer_alt`: Calls another routine, then writes values to the blitter registers, storing the source address in one register and a different destination address in another before returning.
- `fill_memory_with_byte`: Computes a destination address and loops 1703 times to store the byte value 255 at each location.
- `write_boundary_bytes`: Computes two destination addresses from a value in d1, then writes a 4-byte value and a single byte to each location.
- `update_byte_table`: Adds a byte from one memory location to another, and if the result is zero or matches a specific constant, negates a second byte and updates both locations before calling another routine.
- `update_circular_buffer`: Loops 15 times to write values to a table at a fixed address, updating two global variables based on conditional checks of their values and the loop counter.
- `set_three_values`: Writes the word value 10 to a location, the word value 15 to a location 24 bytes later, and the word value 10 to a location 24 bytes after that.
- `store_and_increment_bytes`: Stores the byte in d0 into a global variable at offset 180, then enters a loop that writes the same byte to a table at 12-byte intervals, incrementing the byte by one each iteration for 15 iterations.
- `decrement_and_process_buffer`: Decrements a global longword if it is non-zero, otherwise loads two addresses into registers, reads a word from a pointer, calls a helper routine, and then checks a byte at a pointer, replacing it with a fixed address when it is zero and setting the global longword to 100 when it equals one.
- `call_setup_blitter_area`: Calls setup_blitter_area_offset using the same address for both source and destination pointers in a0 and a1.
- `setup_blitter_area_offset`: Calls the blitter wait routine, then sets the blitter destination address and mask registers and configures the area width, word count, mode, and height registers using the value in d1 plus 2544.
- `setup_blitter_area_constants`: Calls the blitter wait routine, then sets the blitter destination address and mask registers and configures the area width, word count, mode, and height registers using the constant 2544.
- `configure_blitter_transfer`: Calls the blitter wait routine, then sets the blitter destination address and mask registers and configures the area width, word count, mode, and height registers with a width of 0 and height of 38.
- `setup_blitter_area_calculated`: Computes a destination address by subtracting 32 from d0, multiplying by 32, and adding it to a base address, then calls the blitter wait routine and sets the destination address, mask, and area registers with a width of 0 and height of 40.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `populate_data_table`: Reads a global variable, computes a value from it, then enters a loop that writes a word to a table and another word at a 6-byte offset, advancing the pointer by 8 bytes and decrementing a counter for 43 iterations.
- `fill_table_with_structs`: Enters a loop that writes a long value and three words to a table, copying a word from another location, writing a fixed long value, and incrementing a value by a fixed address for 43 iterations.
- `clear_table_entries`: Clears a byte at a memory location, then enters a loop that advances the pointer by 36 bytes and clears another byte for 8 iterations.
- `read_mouse_and_set_value`: Reads the mouse port movement register, shifts it right by eight, and stores 1, 2, or 3 into a global longword depending on which of the thresholds 40, 80, or 120 the value exceeds, then calls a helper routine to restore the saved register and return.
- `compare_byte_load_four`: Compares the byte in d0 with 160 and branches when it is greater, then loads the constant 4 into d0 and jumps to a shared routine.
- `restore_and_store_offsets`: Restores the return address from the stack and returns.
- `compare_and_branch`: Compares a byte in d0 to 200, and if it is higher, branches to another routine; otherwise, it sets d0 to 5 and branches to another routine.
- `update_global_values`: Stores a value from the stack into a global variable, performs arithmetic on it, and writes the results to two different global byte locations; it then checks a byte in a table using an offset derived from the global variable and calls check_input_status if a condition is met.
- `check_input_status`: Writes 0 to a global variable, then checks a joystick button bit and a hardware register bit, writing 1, leaving the value, or writing all ones depending on the results before returning.
- `set_boundary_values`: Compares a byte at an offset from a global variable to 255; if they differ, it writes 255 to two memory locations, writes 0 and the offset value to two more locations, updates the compared byte, then restores registers and returns.
- `perform_multiple_library_calls`: Saves all registers, loads a global variable, then calls a series of routines with different values in d0 and a computed address before restoring the registers and returning.
- `clear_boundary_values`: Reads a byte index from a global variable and compares it to zero; if it is not zero it sets a byte at offset 3618 of a structure to 255, clears a byte at offset 3616 and a word at offset 3620, stores the index in the word at offset 3622, and clears the byte at the computed offset, otherwise it returns.
- `copy_interpolated_blocks`: Computes a memory range using a value from a global variable, calls a routine to clear that range, and then enters a loop to copy 12-byte blocks from two tables into two destination areas using a calculated offset.
- `clear_memory_blocks`: Computes a count from a word in a global variable, scales it, and adds it to two base addresses to obtain two destinations, then loops 17 times clearing four-byte blocks at both locations.
- `convert_number_to_ascii`: Reads a byte from a global variable, divides it by ten, and stores the quotient and remainder as two ASCII digit characters into a table at offset 61ea6, then returns.
- `execute_nested_loops`: Loops over a count stored in a global variable, calling a series of library stubs and other routines, then runs a second loop that calls the same set of routines a fixed number of times.
- `loop_and_call_subroutines`: Decrements a value from a global longword and enters a loop that performs division on a value in d0, checks a byte in a table, and calls various subroutines with specific register parameters based on the table contents.
- `iterate_and_call_library`: Enters a loop that updates a global longword with a starting value of 64, calls another routine, and increments the global longword by 20 each iteration until d0 matches a value from a global longword, then calls another routine.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `execute_several_library_calls`: Loads values from a global variable into registers along with constants, then performs four calls to library routines with different parameter combinations.
- `conditional_library_calls`: Tests a byte at an offset from a global variable and, when it is non-zero, runs a sequence of eight library calls with specific register parameters and destination addresses.
- `invoke_library_helpers`: Loads a base address from a global variable, increments it by one and two, and calls three different library helper routines with distinct register values, then jumps back.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_functions`: Stores four incoming registers into global variables, then repeatedly loads a library base from a global variable and calls three library functions, adjusting the values in the registers between calls.
- `process_bitstream_data`: Loops over a byte table, doubling a 16-bit value at a destination address on each inner step and adding three when a bit is set, then copies the value forward and advances to the next block.
- `configure_blitter_transfer_2`: Reads a byte from a table to compute a blitter destination address, calls the blitter wait routine, and if the incremented value equals eight it performs additional setup calls and writes to a table.
- `call_library_vector_60_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_240_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_342_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `fill_memory_loop`: Writes an incrementing byte value to a destination starting at a global variable, calling three routines on each iteration, until the counter reaches negative one hundred and twenty-eight.
- `execute_nested_delays`: Runs two nested loops that decrement registers d0 and d1 to create a delay, then restores the saved registers and returns.
- `populate_data_table_2`: Fills a memory area with a repeating pattern of bytes and 32-bit words retrieved from a table using an index, adjusting the pattern and switching to a second byte after the first inner loop completes.
- `add_table_values`: Loops 131 times, adding a byte from one table to a byte in another table and storing the result, advancing the destination by eight bytes and the source by one byte each iteration.
- `disable_interrupts_swap_vector`: Reads the interrupt enable register and stores it in memory, then disables interrupts by writing to the enable register.
- `toggle_interrupts`: Subtracts 32768 from a global variable and writes it to the INTENA register to change interrupt status, then calls two library functions via pointers stored in the exec library base.
- `decode_and_reencode_bitstream`: Decodes a bit-packed stream from a global buffer into a byte table, copying run-length groups forward or expanding single bytes backward, then re-encodes the result into a second buffer while translating the byte 109 into a multi-byte sequence.
- `copy_word_sequence`: Loops 31 times copying a word from a source table to a destination table at four-byte intervals, then restores the saved registers and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_vector_and_disable_ints`: Writes the address of the next routine into the vector table entry at offset 32 of the system library and sets the interrupt mask bits in the system register, then returns.
- `initialize_and_decompress_data`: Initializes Amiga hardware state by writing to the color palette and blitter, then decompresses a data block by decoding a bit-packed table and storing the resulting bytes into a destination buffer.
- `copy_and_decode_bitstream`: Copies a run of bytes from one buffer to another while decoding a bit stream, then copies a block of 256 words from one routine's code into a fixed 1024-byte region of RAM and jumps back into that routine.
- `call_and_store_byte`: Calls another routine, then stores the byte from d1 through the pointer in a1 and returns.
- `copy_compressed_bytes`: Reads bytes from a source table, handling negative and marker values by calling a helper routine, and stores each resulting byte through a pointer into a destination buffer.
- `store_byte_at_pointer`: Stores the byte in d1 through the pointer in a1, then returns.
- `shift_and_copy_memory_blocks`: Shifts a block of bytes left within a structure by the count in d1, then copies a trailing run of bytes from a second source into the structure, filling gaps left by the shift.
- `modify_global_bitmask`: Sets bits 3 to 7 and bit 2 of the global variable at a fixed address, clears the bit selected by a value in the structure in a6, then spins until bit 5 of that variable is set.
- `set_specific_bit`: Sets bits 3 to 7 of the global variable at a fixed address, then sets the bit selected by a value in the structure in a6, leaving that bit in the set state.
- `manage_dma_and_disk`: Configures DMA channels and disk controller registers, then monitors hardware status bits and updates a structure using timing loops.
- `process_array_with_helper`: Calls a helper routine four times, returns negative one if a byte at a pointer is negative one, otherwise computes an address and calls two more routines before looping over a word array.
- `return_constant_value`: Copies d0 into d1, loads d0 with the constant 254, and returns.
- `sum_array_elements`: Adds each word in the array pointed to by a1 to a running total in d0, decrementing the loop counter in d1, then returns the sum in d0.
- `interleave_bit_patterns`: Loops through two source buffers, writing to the destination at a1 a byte formed by combining the low nibble bits of the matching source bytes, then restores a0 from a2 and returns.
- `clear_and_load_word_pair`: Clears the word at offset 12 in the structure in a6, then loads the constant 82 into the word at offset 14.
- `process_input_timing`: Loops while bit 4 of the global variable at a fixed address is set, updating a counter in the structure in a6, applying delays, and toggling bits of that variable, then clears two counters and loads a constant into the audio volume register.
- `set_hardware_registers`: Points a0 at the custom chip base, clears bit 6 of the global variable at a fixed address, then writes a sequence of constant values to the audio channel registers and a global variable before returning.
- `initialize_hardware_state`: Writes multiple constant values to Amiga hardware registers, clears a large block of memory, zeroes a range of sprite registers, and calls a helper routine before restoring interrupts.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `describe_data_table`: It contains no working instructions, only zero words that do nothing, so this range is most likely a data table that other code refers to by address rather than executable code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_data_table`: This routine consists of a data table containing values that are interpreted as instructions but perform no functional operations.
