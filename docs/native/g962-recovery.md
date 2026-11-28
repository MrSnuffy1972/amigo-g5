# Game 962 (killing game show, the (1990)(psygnosis)(disk 1 of 2)[intro only]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00962). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901200 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 25 raw-word lines |
| Functions named and described | pass, 92 of 92 named (0 author symbols), 92 of 92 described (11 by rule, 48 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 13 of 13 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 366698 bytes, 69 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1246 of 1246 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 546 |
| data | 883868 |
| padding | 914 |
| unused | 15872 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro | 1 | 181 | 21 |
| Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro#unpacked | 1 | 1411 | 39 |
| Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:intr#unpacked (raw code file, load address unknown) | 1 | 1162 | 30 |

## Cases handled

- killing game show, the (1990)(psygnosis)(disk 1 of 2)[intro only]/Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:prt1: (CRND) Data Cruncher (xfd)-packed, unpacked with ancient (331824 -> 413428 bytes)
- killing game show, the (1990)(psygnosis)(disk 1 of 2)[intro only]/Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:prt2: (CRND) Data Cruncher (xfd)-packed, unpacked with ancient (216180 -> 252912 bytes)
- killing game show, the (1990)(psygnosis)(disk 1 of 2)[intro only]/Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:intr: (CRND) Data Cruncher (xfd)-packed, unpacked with ancient (265772 -> 347640 bytes)
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- killing game show, the (1990)(psygnosis)(disk 1 of 2)[intro only]/Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro: crunched executable unpacked with ancient scan (Unknown Cruncher 1 (xfd), 13436 -> 19058 bytes), 9 of 9 windows confirmed in reference RAM, loaded at an address: recovered as u3
- killing game show, the (1990)(psygnosis)(disk 1 of 2)[intro only]/Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:intr#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- killing game show, the (1990)(psygnosis)(disk 1 of 2)[intro only]/Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 1 never-loaded region(s), 347640 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro
- Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro#unpacked

## Copy protection found

- none found

## Functions


### Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro

- `copy_table_to_global`: Copies the 16-byte table located at offset 16 from the program counter into the 16 bytes of a global variable at offset 128, then returns.
- `init_and_decompress_stream`: Enables the CPU and the CIA, resets the blitter and both copper lists, then decompresses a bit-packed stream from the source buffer into the destination buffer, copying the decompressed data until a terminator is reached.
- `compare_then_rotate`: Compares d1 with the constant 6 and branches away when the carry is set, branches away when d1 is zero, otherwise rotates d3 left through the carry after loading a value from the stack and jumps through a jumptable in a4.
- `decrement_then_dispatch`: Decrements d1 and branches away when the carry is clear, otherwise loads a library base into a6 and jumps through a jumptable in a4.
- `load_base_then_branch`: Loads a library base address into register a5 and then branches to the routine at offset 152, falling through into it.
- `dispatch_via_jumptable`: Loads the base address of a library into a6, then jumps through a jumptable in a4.
- `rotate_store_byte_jump`: Adds a constant to a register and rotates a bit through another while incrementing a counter in a loop, then stores a byte onto the stack in a second loop and compares pointers before jumping to another routine.
- `rotate_store_byte_loop`: Pops a 32-bit value from the stack into a register and rotates a bit through a register while incrementing a counter in a loop, then stores a byte onto the stack in a second loop counting down another register.
- `load_base_then_fall`: Loads a library base into a5, then falls through into a following routine.
- `index_jumptable_via_d2`: Sets d0 to 1 and loads the address of a table into a6, then jumps through that table using an index in d2, which is set to 3 when d1 equals 2 and otherwise left as passed, returning via the jumptable.
- `select_and_rotate`: Sets d0 and d2 to small constants, then branches away when d1 is zero, sets d2 to 3 when d1 equals 2, and calls a jumptable when d1 equals 3, otherwise rotating d3 left through the carry after loading a value from the stack.
- `dispatch_via_jumptable_2`: Loads a library base into a6, then jumps through a jumptable in a4.
- `compare_then_adjust`: Compares d1 with the constant 2 and branches away when they are not equal, otherwise adds 4 to d1 and falls through into a following routine.
- `compare_then_dispatch`: Compares d1 with the constant 3 and branches away when they are equal, otherwise loads a library base into a6 and jumps through a jumptable in a4.
- `add_branch_into_routine`: Adds a constant to a register and then unconditionally branches into the middle of another routine.
- `jump_selected_routine`: Loads a constant into a register, points a register at another routine, and jumps to a routine selected by a value in a register.
- `select_rotation_sequence`: Adds a constant to a register and copies it to another, then selects one of several rotation sequences based on a flag bit, decrementing a counter and restoring the status register between each branch.
- `shift_bytes_left_loop`: Shifts a block of bytes left by one position in a loop counting down a register, then compares the source and destination pointers and clears a word in the structure in a3 when the source is not behind the destination.
- `wait_cia_display_loop`: Waits by clearing bit 6 of the CIA display register in a loop that counts down a 16-bit counter from 999, then stores the value 32767 at offset -228 of the structure in a3.
- `rotate_bit_return_zero`: Clears a register and rotates a bit through it, returning to an earlier routine when the register is zero, otherwise looping a fixed number of times before jumping to another routine.
- `pop_stack_rotate_loop`: Pops a 32-bit value from the stack into a register and stores it as a word in the structure in a3, then rotates a bit through a register while incrementing a counter in a loop that counts down another register.

### Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_words_repeat_loop`: Copies 16-bit words from the source table at offset 16 to the destination table at offset 432 in a loop, then jumps back to the start of the table copy to repeat it forever.

### Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:kgs_intro#unpacked

- `initialize_system_state`: Copies a block of data from one memory location to another using a loop, then performs several writes to hardware registers and global variables before jumping to a new address.
- `execute_data_stream`: Iterates through a list of function pointers stored in a structure, calling each one and performing conditional logic based on the return value until a specific condition is met.
- `update_hardware_configuration`: Updates several global variables and hardware registers, including the COP1LCH register, and performs conditional branches based on the contents of a structure.
- `process_buffer_data`: Manipulates a destination buffer by copying data from a source, performing arithmetic on word values, and updating several global variables and the COP2LCH register.
- `write_data_structure`: Stores a pointer to a memory location in a global variable, then loops to write a series of words and bytes to a destination buffer based on values read from a source table.
- `decompress_data_stream`: Iterates through a source buffer to copy or expand data into a destination buffer using length and offset values provided within the stream.
- `update_data_structure`: Rotates and modifies a word from a global variable, stores it in another global, calculates a long value by multiplying a third global by 36, stores several bit-manipulated words into a table, and clears two other global words.
- `toggle_address_table`: Toggles a bit in a global variable and updates a table of four long values with arithmetic variations of a base address.
- `perform_blt_operations`: Sets up Blitter registers, copies a block of data, and performs bitwise logical operations on multiple memory blocks using DMA.
- `clear_memory_block`: Fills a block of memory with zeros using a loop controlled by a constant count.
- `apply_color_transformations`: Loops through a count from a global variable to call a transformation routine on two different sets of data buffers.
- `process_packed_colors`: Reads a magic identifier, extracts bit-packed nibbles from a source, scales them by a multiplier, and writes the results to a destination table.
- `setup_hardware_and_clear_buffers`: Processes a data stream to copy bytes, modifies hardware registers including CIA and interrupts, and calls a routine to clear two memory areas.
- `initialize_interrupts`: Sets up interrupt handling by reading hardware status registers, checking specific bits, and configuring various system control registers including CIA and DMA settings before returning.
- `frag_st_moveq_nop_dbra_move_bclr_movem`: Short code fragment that sets an indexed entry from a0 to all ones, then loads 40 into d0, then pauses for one no-operation step, then loops back while counting down d0, then copies 255 to a fixed memory address, then clears bit 6 of a fixed memory address, then restores saved registers from the stack, then returns from the exception.
- `manage_interrupt_service`: Processes interrupt requests by checking hardware status bits in multiple registers and conditionally jumping to different handlers based on those bits.
- `handle_interrupt_service`: Manages interrupt service routines by checking various hardware status bits, performing conditional jumps based on input states, and handling different interrupt types through multiple branches.
- `setup_graphics_mode`: Configures graphics display parameters by setting up bitplane pointers, color registers, and display window coordinates while managing a counter that tracks the current mode.
- `handle_interrupt_processing`: Handles interrupt processing by checking hardware status registers, updating display attributes through bit manipulation operations, and calling position update routines for multiple graphics objects before returning.
- `update_position_coordinates`: Updates position coordinates by adding values, performing bit shifts, and storing results in memory locations while handling boundary conditions and clearing specific memory areas when certain conditions are met.
- `initialize_graphics_data`: Initializes graphics-related data by clearing memory locations, performing arithmetic operations on coordinates, and managing data structures through multiple subroutine calls and conditional branches.
- `process_character_data`: Processes character data by validating range, adjusting case, and manipulating memory bits using bit manipulation instructions, ultimately clearing or setting specific flags in hardware registers.
- `frag_move_st_addq_bclr_ror_bset_rts`: Short code fragment that copies d0 to the stack, then sets the memory at a5 to all ones, then copies the memory at an offset from a6 to d0, then adds 3 to d0, then clears bit d0 of the memory at a5, then rotates d7 right by 8, then sets bit d0 of the memory at a5, then copies the top of the stack to d0, then returns.
- `process_position_data`: Processes position-related data by performing division, comparisons, and conditional updates of a counter variable, then calls multiple routines based on the results of these operations.
- `frag_move_st_bsr_rts`: Short code fragment that copies d0 to the stack, then sets the memory at an offset from a6 to all ones, then calls another routine (2 times in a row), then copies the top of the stack to d0, then returns.
- `frag_tst_bpl_bset_btst_beq_bsr_bra_clr`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then sets bit 1 of the memory at a5, then tests bit 4 of the memory at an offset from a5, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then clears the memory at an offset from a6, then returns.
- `update_position_flags`: Processes position data by shifting a value right, clearing or setting specific bits in memory based on comparison results, and updating a counter variable before returning.
- `frag_move_bclr_ror_bset_dbra_st_rts`: Short code fragment that copies d7 to the stack, then clears bit 0 of the memory at a5, then rotates d7 right by 8, then sets bit 0 of the memory at a5, then copies the memory at an offset from a6 to d7, then loops back while counting down d7, then sets the memory at an offset from a6 to all ones, then copies the top of the stack to d7, then returns.
- `frag_tst_beq_move_dbra_sf_rts`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was zero, then copies d7 to the stack, then copies the memory at an offset from a6 to d7, then loops back while counting down d7, then clears the memory at an offset from a6, then copies the top of the stack to d7, then returns.
- `setup_display_attributes`: Sets up display attributes for graphics objects by loading parameters from memory locations, performing bit manipulation operations, and storing results in memory.
- `configure_display_attributes`: Sets up display attributes for a graphics object by loading various parameters from memory locations and performing bit manipulation operations on a counter value.
- `frag_move_btst_bne_subq_moveq_rts`: Short code fragment that copies 2 to the memory at an offset from a4, then copies an address to d0, then tests bit 1 of the memory at an offset from a4, then branches if the last result was not zero, then subtracts 1 from d0, then branches if the last result was not zero, then loads 253 into d0, then returns, then loads 0 into d0, then returns.
- `frag_movea_moveq_clr_lea_dbra_rts`: Short code fragment that copies the memory at an offset from a6 to a0, then loads 10 into d0, then clears the memory at an offset from a0, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d0, then returns.
- `update_graphics_position`: Updates graphics object positions by reading counter values, performing conditional checks, and storing updated coordinates in memory locations.
- `validate_data_checksum`: Validates data checksums by performing bitwise XOR operations on memory values, comparing results with expected values, and returning a status code.
- `process_data_masking`: Processes data masking operations by performing bitwise AND operations on memory values, combining results through addition and OR operations, then returns.
- `process_bitstream_data_2`: Processes bitstream data from memory locations, performs arithmetic operations on counters, and calls another routine to handle the processed data.
- `update_graphics_parameters`: Updates graphics parameters by processing data from memory locations, calling another routine to process bitstream data, and performing arithmetic operations on register values before returning.
- `process_bitstream_data`: Processes bitstream data through multiple conditional branches that perform bit testing, arithmetic operations, and data movement based on loop counters and bit positions.

### Killing Game Show, The (1990)(Psygnosis)(Disk 1 of 2)[intro only].adf:intr#unpacked (raw code file, load address unknown)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `initialize_audio_tables_and_mute`: Scans bytes to derive a table offset, builds 31 pointers while clearing their targets, sets a CIA port bit, clears four audio volumes and resets state fields.
- `initialize_audio_tables`: Initializes audio table entries by copying data from memory locations, performing bitwise operations, and calling other routines to set up audio parameters.
- `update_audio_parameters`: Updates audio parameters by reading values from memory locations, performing bitwise operations, and writing results to hardware registers.
- `manage_audio_state`: Manages audio state by processing input values, performing arithmetic operations on counters, handling conditional branches based on input states, and updating audio parameters in memory.
- `repeat_palette_increment_passes`: Sets d6 to 15 and d7 to six, then repeats two calls while passing a fixed base in a0 and a target word in d1.
- `repeat_palette_decrement_passes`: Sets d6 to 15 and d7 to six, then repeatedly calls a timing function and a decrementing update using a fixed a0 base.
- `repeat_thirty_two_word_updates`: Runs 16 outer passes, each calling a timing function then making 32 updates from successive words at a1 to locations spaced four bytes through a0.
- `prepare_first_copper_sequence`: Loads three source pointers, calls preparation, writes the copper list pointer, performs an update pass, then makes three timed calls and tail branches with d7 set.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `prepare_second_copper_sequence`: Loads an alternate source and two shared pointers, calls preparation, writes the copper list pointer, performs an update pass, then makes three timed calls and tail branches.
- `mark_prepare_run_copper_updates`: Stores all ones in a state word, prepares data from three pointers, writes the copper list pointer and tail branches to the repeated word update code.
- `show_first_intro_buffer_sequence`: Calls setup, loads buffer and text pointers, renders text, performs incrementing updates, delays, performs decrementing updates, then tail branches for another delay.
- `show_second_intro_buffer_sequence`: Calls setup, loads the same buffer with an alternate text pointer, renders text, performs incrementing and decrementing updates separated by delays, then tail branches.
- `show_two_text_buffers_sequence`: Calls setup, renders two strings into separate buffers, performs incrementing updates, delays, performs decrementing updates, then tail branches with a second delay value.
- `decode_cmap_and_body_planes`: Searches for color-map and body chunks, converts 32 packed color entries into words, establishes five plane destinations, repeatedly calls the body decoder and returns when its length ends.
- `decode_rle_row_to_forty_bytes`: Reads signed run controls from a0, copies repeated or literal bytes into a6 while decrementing d7, and continues until 40 output bytes have been written.
- `scale_delay_wait_for_counter`: Halves d7, optionally multiplies it by five and divides by four, then waits until adding the result changes a shared counter before returning.
- `fractional_delay_wait_for_counter`: Replaces d7 with combined shifted fractions, optionally scales it by five quarters, then waits until adding it changes a shared counter and returns.
- `center_render_terminated_text`: Counts bytes through the string terminator, restores a1, centers a0 using four times the length, then calls the character renderer for each nonzero byte.
- `draw_thirteen_row_glyph_mask`: For non-space d0, selects glyph data, applies four glyph bytes to four destination bytes across 13 rows, then advances a0 by four and returns.
- `draw_glyph_mask`: Draws glyph masks by reading data from memory locations, performing bit rotation operations, and writing results to hardware registers for display.
- `configure_graphics_v2`: Configures graphics display parameters by processing data through conditional checks, arithmetic operations, and calling another routine to finalize setup.
- `update_graphics_parameters_v2`: Updates graphics parameters similar to another routine but processes data differently through a series of conditional checks, arithmetic operations, and calls to bitstream processing routines.
- `process_bitstream_data_v2`: Processes bitstream data with similar operations to another routine but uses different register values and conditional paths for bit testing and arithmetic operations.
- `wait_for_input_or_state_change`: Returns when either state byte is nonzero or CIA input bit six clears; otherwise it repeats while CIA input bit seven remains set.
- `clear_ten_thousand_longwords`: Sets a0 to the display buffer, writes zero to 10000 consecutive longwords using d7, then returns.
- `repeat_word_decrement_passes`: Runs 16 outer passes, each calling a timing function then making 32 decrementing updates at locations spaced four bytes through a0, and returns.
- `increment_three_color_nibbles`: Compares three packed nibbles in the word at a0 against d1 and increments each differing nibble independently while preserving the other bits, then returns.
- `decrement_nonzero_color_nibbles`: Tests three packed nibbles in the word at a0 and decrements each nonzero nibble independently while preserving the other bits, then returns.
