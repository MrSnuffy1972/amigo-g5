# Game 1907 (international karate + (1988)(system 3)[cr oracle][m invisible crime]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01907). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 350 of 350 named (0 author symbols), 350 of 350 described (2 by rule, 272 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 9514 of 9514 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11698 of 11804 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 40 |
| code | 864 |
| data | 327856 |
| padding | 1992 |
| unused | 570368 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:type | 2 | 117 | 2 |
| International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:ik+++ | 1 | 98 | 9 |
| International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:intro | 2 | 83 | 3 |
| International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:bootblock | 1 | 10 | 1 |
| logged disk load from International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:ik+++#bytekiller+124 unpacked at an address (362496 bytes) | 1 | 9905 | 290 |
| logged disk load from International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:intro#bytekiller+E8 unpacked at an address (131072 bytes) | 1 | 1505 | 42 |
| logged disk load from International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:ik+++ hunk 0 +32..+132 copied to an address | 1 | 86 | 3 |

## Cases handled

- international karate + (1988)(system 3)[cr oracle][m invisible crime]/International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:ik+++ hunk 0 +124: ByteKiller stream (234936 packed bytes, checksum verified) unpacked to 362496 bytes at an address by the program's own depacker (the hunk's start-up code copies its depacker (+an address..+an address) to an address and runs it; the depacker unpacks the stream whose header is at +an address backwards to an address..an address (checksum residue 0) and jumps to an address; reference runs execute the unpacked bytes); recovered as a unit from the unpacked bytes, 8026 executed PCs map to it
- international karate + (1988)(system 3)[cr oracle][m invisible crime]/International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:intro hunk 0 +E8: ByteKiller stream (66244 packed bytes, checksum verified) unpacked to 131072 bytes at an address by the program's own depacker (the cracktro's first routine unpacks the stream whose header is at +an address to an address..an address (checksum residue 0) and jumps to an address; reference runs execute the unpacked bytes); recovered as a unit from the unpacked bytes, 1310 executed PCs map to it
- international karate + (1988)(system 3)[cr oracle][m invisible crime]/International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:ik+++ hunk 0 +32..+132: copied verbatim to an address and run (the hunk start-up copies 64 longwords from offset 32 hex to address 500 hex with a longword copy loop and jumps to the copy; executed PCs from 500 hex match these bytes); recovered as its own unit (the same bytes are also classified in the hunk), 80 executed PCs map to it
- international karate + (1988)(system 3)[cr oracle][m invisible crime]/International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:ik+++#bytekiller+124 unpacked at an address (362496 bytes): 7 more executed PCs map by their opcode word only (the bytes after it differ at run time: an address, an address, an address, an address, an address, an address, an address)
- international karate + (1988)(system 3)[cr oracle][m invisible crime]/International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:intro#bytekiller+E8 unpacked at an address (131072 bytes): 1 more executed PCs map by their opcode word only (the bytes after it differ at run time: an address)
- international karate + (1988)(system 3)[cr oracle][m invisible crime]/International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:ik+++ +124..396E8: data by evidence (ByteKiller packed stream read by the depacker); 0 instructions decoded there were removed
- international karate + (1988)(system 3)[cr oracle][m invisible crime]/International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:intro +E8..103B8: data by evidence (ByteKiller packed stream read by the depacker); 0 instructions decoded there were removed
- international karate + (1988)(system 3)[cr oracle][m invisible crime]/International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:ik+++: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- international karate + (1988)(system 3)[cr oracle][m invisible crime]/International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:intro
- International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:intro#bytekiller+E8 unpacked at an address (131072 bytes)

## Copy protection found

- none found

## Functions


### International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:type

- `allocate_and_copy_table`: The routine walks a linked list of tables to find the largest size, then allocates a block of memory and fills it with a repeating pattern.
- `copy_block_and_call`: Copies a fixed number of words from a global table to a stack pointer if the pointer is valid, then calls another routine.

### International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:ik+++

- `ik_start`: Program entry of ik+++: points the privilege-violation vector at the next instruction so that the following write to the status register also works from user mode, then falls into the take-over.
- `ik_take_over_and_copy_depacker`: Masks all interrupts, switches every DMA channel and interrupt off, copies the 256-byte depacker that follows to address 500 hex and jumps to the copy.
- `bytekiller_depack_game`: ByteKiller depacker, run from its copy at 500 hex: moves the rest of the hunk up to 600 hex, then decodes the packed stream backwards into 600 hex onwards, writing the output pointer to COLOR00 as it goes, verifies the running checksum and jumps to the unpacked game.
- `bk_long_literal_run`: Depacker branch for a long literal run: sets an 8-bit count with a bias of 8 and continues with the literal copy loop.
- `bk_decode_match`: Depacker branch taken on a one bit: reads a 2-bit type that selects a 3 or 4 byte match with a 9 or 10-bit offset, a long literal run, or a match with an 8-bit length and a 12-bit offset.
- `bk_short_match`: Depacker branch for a short match: offset width 9 or 10 bits and a length of 3 or 4 bytes from the 2-bit type, then the backwards byte copy.
- `bk_checksum_error`: Depacker failure exit: the checksum did not cancel to zero, so it writes to COLOR01 sixty-five thousand times as a visible error and returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:intro

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `intro_bk_refill_bit_buffer`: Cracktro depacker refill: takes the previous packed longword, folds it into the checksum and shifts its first bit out with the marker bit set.
- `intro_bk_read_bits`: Cracktro depacker bit reader: gathers D1 bits into D2 from the bit buffer, refilling it from the packed stream when it empties.

### International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:ik+++#bytekiller+124 unpacked at an address (362496 bytes)

- `game_entry`: Game entry at 600 hex, reached from the depacker: clears the long frame counter at 64C hex and jumps to the machine and game initialisation at 17A0 hex.
- `attract_then_main_loop`: Runs one pass of the title and attract handler and then re-enters the main loop.
- `fault_restart`: Target of the processor fault vectors: if the title phase was running it rebuilds the title screen, then resets the phase and the frame counters, selects the default player mode, reinitialises the game and the music and re-enters the main loop.
- `install_fault_vectors`: Points the fourteen processor exception vectors (bus error up to the trap range) at the fault restart routine so a crash returns to the title.
- `main_loop_phase_dispatch`: Two return stubs followed by the main loop: clears two display-buffer flags and calls the handler for the game phase byte at 678 hex (1 fight, 3 between bouts, 4 match over, 5 bonus stage, 6 and 7 further phases, 8 title and attract sequence), then loops forever.
- `load_global_offset_value`: Reads a word from a table using an offset derived from a global variable.
- `set_bpl_path_offsets`: Adds specific byte offsets to a global variable corresponding to the Bitplane Path registers.
- `configure_blitter_copper`: Writes specific values to a series of hardware registers to configure the Blitter and Copper channels for graphics processing.
- `pause_idle_loop_and_frame_interrupt`: An idle loop that the pause logic parks the main program in, followed by the vertical-blank work run from the interrupt: bumps the frame counters, runs the music tick, input and keyboard handling and the display updates, and implements pause by swapping the interrupted return address with the idle loop.
- `copy_audio_palette_state`: The routine copies six 32-bit values from a RAM table to hardware audio and color registers, then calls another routine.
- `wait_and_disable_interrupts`: Performs a short delay loop if a specific bit in a global variable is set, then clears that variable and disables CPU interrupts and DMA channels.
- `music_tick_divider`: Music pacing: when the speed-compensation flag is set the music tick is skipped on every sixth frame, otherwise the music player is called each frame while music is enabled.
- `reset_audio_dma_interrupts`: Clears the period and volume registers for all four audio channels to silence sound.
- `set_global_flags_2`: The routine writes a constant value to a global variable, then overwrites it with a different constant value, and returns.
- `configure_audio_dma_interrupts`: Sets specific values to a global variable and calls another routine.
- `set_byte_from_table`: The routine copies a byte from a table entry into another byte at a related offset, and if the two bytes differ it loads a value from a third offset into a global variable, updates the global variable with the copied byte, and calls another routine.
- `lookup_byte_from_table`: The routine searches a table of bytes for a match with a global variable and then returns a byte selected from a second table based on the found index and a few flag checks.
- `set_state_flags`: The routine masks a word input to five bits and, if the result is less than a threshold and a global variable is non-zero, it writes a constant and the masked value into two different pairs of global variables depending on whether the global variable is zero or not.
- `restore_interrupt_dma_state`: Restores the interrupt enable and DMA control registers from saved values, clearing the saved state variables if they were non-zero.
- `set_audio_channel_3`: The routine configures audio channel 3 by writing to AUD3PER, AUD3VOL, AUD3LEN and AUD3LCH, then enables the corresponding DMA channel via DMACON and enables interrupts via INTENA. It also clears two global variables when a condition on a global variable is met, otherwise it updates a global variable through a table indexed by a masked value from a global variable.
- `initialize_input_state`: The routine clears and copies a set of global variables and then drives another routine with a series of control values to set up the input state.
- `install_interrupt_vectors`: Disables and clears all interrupts, installs the game's level 3 handler and the audio handler, and points the other interrupt levels at the acknowledge-and-return handler.
- `check_interrupt_status`: Reads the interrupt request register to determine if a specific hardware interrupt has occurred.
- `ack_interrupt_return`: Handler for interrupt levels the game does not use: writes the pending requests back to clear them and returns from the exception.
- `setup_hardware_and_data`: Disables interrupts and DMA before clearing global variables and copying a table from low memory to a high-memory buffer.
- `clear_interrupt_and_dma_flags`: The routine writes to DMACON and INTENA and INTREQ to clear DMA and interrupt request flags and then re-enables interrupts and DMA channels.
- `self_3`: The routine checks a global variable and, if a flag is set, computes a transformed byte value and updates the global variable and a byte table entry.
- `read_joystick_and_ciaa`: The routine calls another routine twice and stores the results in a global variable.
- `transform_input_flags`: Performs bitwise operations on input flags from registers D0 and D7, shifting and XORing specific bits before combining them with a constant mask.
- `prepare_graphics_once`: One-time set-up guarded by its own flag: runs the graphics table preparation on three image blocks the first time it is called.
- `copy_blocks_to_global_and_back`: The routine copies 20 word blocks from a source pointer to four fixed tables and a global variable, then copies 80 words back from the global variable to the source area, repeating the process 200 times.
- `copy_word_block_to_memory`: The routine copies 16,000 words from a global variable area to another global variable area and to a fixed memory region.
- `self_11`: The routine first resolves a table of pointers stored in a global variable by dereferencing each entry and writing the resolved address back into the table.
- `align_word_to_block`: The routine takes a 32-bit value from a register and a 16-bit value from a register, aligns the 32-bit value down to a multiple of 160 by clearing its low 8 bits, then builds a 32-bit result from the aligned value and the remainder offset plus eight, combined with the quotient and the 16-bit input.
- `rotate_word_right`: Rotates a 16-bit word right by one bit position for a fixed number of iterations.
- `align_word_index_and_pack`: The routine aligns the low word of a value down to a multiple of a constant, extracts the low word remainder, shifts it, and combines the parts with an input word to form a packed 32-bit result.
- `check_and_set_state`: Calls an external routine and checks the zero flag to determine execution flow.
- `bout_frame_body`: Per-frame body of the bout and between-bout phases: prepares the frame, updates the three fighters, runs the end-of-bout logic in phases 3 and 4, draws the fighters and the scripted extras in an order chosen by the bout timer and finishes the frame.
- `parse_byte_script_stream`: The routine reads a byte stream from a global pointer, interpreting specific values as control codes or commands.
- `clear_memory_and_loop`: The routine clears several memory ranges to zero, copies a few words from one set of RAM locations to another, zeros a block of bytes in a table, then enters a loop that repeatedly clears a block of words, waits for a flag, and repeatedly clears bytes in a small table while calling another routine.
- `loop_call_subroutine`: The routine initializes a counter and repeatedly calls another routine while decrementing the counter until it reaches a specific limit.
- `select_and_copy_block_table`: Picks one of sixteen entries of a pointer table from a rotating counter and copies the block table it points to.
- `set_global_flags`: Writes sequential integer values to a global variable and conditionally resets or updates it based on specific bit flags.
- `build_cell_table`: Fills a 40 by 10 table: for every cell it calls the two value generators and stores their two results.
- `fill_tile_data_table`: Iterates through a table of word-sized entries, calling another routine to generate values and storing them sequentially.
- `compute_collision_response`: The routine takes two input values and performs a series of shifts and bit tests on the first value to derive several intermediate words, then combines those with a scaled version of the second value to form a 32-bit result.
- `interleave_bit_streams`: Combines two 16-bit values into a single 32-bit integer by interleaving their bits.
- `copy_block_table`: The routine copies a table of entries from a source pointer to a destination pointer, repeating for a count derived from a register.
- `copy_sprite_data_block`: Clears four global variables to zero.
- `set_palette_and_dispatch`: The routine copies a block of palette words from a table to hardware registers, optionally overwriting part of the block with another table and a global value, then conditionally calls a function from a jump table using an index derived from a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `set_global_flag_sequence`: Sets a global variable to a specific constant value, calls another routine, and then toggles the global variable between zero and one.
- `self_12`: The routine updates a set of RAM words and long values from a table and then conditionally advances a global counter, copying a short value into two long slots when the counter is even or beyond a threshold.
- `configure_copper_jumper`: Sets up the copper list jump address by writing to the COPJMP1 hardware register.
- `initialize_sprite_tables`: The routine computes a base offset from a global variable and writes the resulting addresses into ten global table pointers.
- `random_value`: Returns a fresh pseudo-random value by calling the random-number generator.
- `read_from_table_to_pointer`: It repeatedly loads two words from a table using indices derived from a global variable and a decrementing counter, and copies them into a destination area.
- `process_sprite_data`: Calls another routine to check a condition and either copies word pairs from a source table or processes signed values with sign-bit masking.
- `update_sprite_palette_entries`: The routine iterates over a table of six-word entries in a global variable.
- `clamp_global_variable`: Calls another routine and checks if a global variable is greater than four.
- `attract_step_fade_and_dots`: Attract-sequence step: fades the working palette one step toward its colour list every fourth frame and moves the field of bouncing dots.
- `attract_step_fade_slow_and_dots`: Attract-sequence step: fades the working palette toward its colour list every fifth frame and moves the bouncing dots.
- `clear_zeroed_table`: The routine clears a block of memory by writing zero to a sequence of 40 consecutive 32-bit words and then to a single word, repeating the pattern 200 times while moving the pointer backwards each iteration.
- `set_global_flag`: Calls another routine and sets a global variable to one.
- `attract_step_draw_logo_block`: Attract-sequence step: draws a masked logo block into the screen work area and marks the step as finished.
- `set_global_flag_2`: Calls another routine and sets a global variable to 1.
- `attract_step_fade_fast_a`: Attract-sequence step: fades the working palette one step every frame toward the first of three colour lists used by these fast fade steps.
- `attract_step_fade_fast_b`: Attract-sequence step: fades the working palette one step every frame toward the second of the three fast-fade colour lists.
- `attract_step_fade_fast_c`: Attract-sequence step: fades the working palette one step every frame toward the third fast-fade colour list.
- `call_sub_and_set_flag`: Calls another routine and then sets a global variable to one if it is not already zero.
- `reset_global_threshold`: The routine checks whether a global variable is greater than 299 and, if so, writes the value 1 to that global variable.
- `attract_step_fade_medium_a`: Attract-sequence step: fades the working palette every third frame toward the first of two closing colour lists.
- `attract_step_fade_medium_b`: Attract-sequence step: fades the working palette every third frame toward the second closing colour list.
- `copy_sprite_data_block_2`: Copies a block of eight consecutive words from a source memory location to a destination buffer.
- `reset_global_flag`: The routine checks whether a global variable is greater than a constant and, if so, writes a constant value to that global variable.
- `attract_step_fade_medium_c`: Attract-sequence step: fades the working palette every third frame toward the third fast-fade colour list.
- `reset_global_flags`: The routine checks a global variable and, if it is greater than a threshold, writes zero to the same global variable four times in a row.
- `fill_pattern_table`: Copies a sequence of five global variables into a memory table at regular intervals.
- `clamp_global_variable_2`: The routine checks a global variable and, if its value is greater than a constant, writes a constant value to the same global variable.
- `load_sprite_data_block`: Clears several global variables and calls an external routine.
- `copy_player_state_entries`: The routine copies a fixed number of entries from one table to another by moving several fields per entry, then updates a global counter with wrap-around and increments another global counter, setting a flag if the counter overflows.
- `compute_scaled_remainder`: Calculates the remainder of a 16-bit value divided by 10 and scales it by 2 for the lower word.
- `self`: The routine calls another routine, then selects a short value and a table base from a global flag and uses the current index to fetch a function pointer from a table and call it.
- `shift_bitplane_data_right`: Iterates through a block of memory, shifting each word right by one bit and propagating the least significant bit to the most significant position of the subsequent word.
- `shift_rows_left_with_carry`: The routine processes a block of short words arranged as a table.
- `shift_tile_palette_data`: Shifts a series of 16-bit words forward in a memory buffer using a stride calculated from a global counter.
- `rotate_table_entries`: The routine rotates the contents of a table of structures by moving each entry’s four 16-bit fields to the next entry, using a counter derived from an input value.
- `clamp_global_variable_2_2`: Calls another routine and checks if a global variable exceeds a specific threshold.
- `swap_and_mask_global`: The routine reads six short values from a global variable and four memory locations, builds a masked 32-bit value from the input register and a global variable, and if the global variable is a multiple of four it rotates the six short values among the global variable and the four memory locations before returning the masked value.
- `initialize_global_counters`: Calls another routine and then sets three separate global variables to specific constant values.
- `attract_step_colour_cycle_effects`: Attract-sequence step: rotates the colour cycle every fourth frame, runs two drawing effects (the second on odd frames only) and counts down the step timer.
- `shift_sprite_data_array`: Iterates through a table of 10-word structures and performs a 32-bit arithmetic left or right shift on each entry.
- `shuffle_score_tables_and_clear_flags`: The routine shuffles two score related tables by moving groups of five longwords in opposite directions and then clears five longwords in two tables.
- `set_global_flag_one`: Sets a specific global variable to the value 1.
- `compare_and_update_table`: The routine increments a global counter modulo 16 and, when a condition on an input value is met, compares two 16-bit tables nibble by nibble.
- `rotate_coplist_pointer`: Decrement a global counter and reload a set of four pointers from a memory table when the counter reaches zero.
- `copy_struct_table_entries`: The routine copies selected words from a global structure table to a fixed memory table.
- `merge_tables_with_mask`: The routine merges four 32-bit words from a source table into a destination table using a bitwise mask derived from the source words.
- `merge_table_rows`: The routine merges four 32-bit words from a source block into a destination block using a mask derived from the source words.
- `copy_and_clear_sprite_data`: Copies specific fields from a source table to a destination table in a nested loop, then zeroes out those same fields in both tables.
- `self_2`: The routine iterates over a global table and ORs together four words from each entry, writing the result back into the entry.
- `copy_global_structures_to_buffer`: Copies specific words from a global array to a fixed memory location in a nested loop.
- `swap_and_clear_tables`: The routine swaps bytes between a table and a global workspace, clears several tables, and then repeatedly processes a set of entries by rotating a byte value and writing a byte from a global array into a table slot.
- `swap_and_clear_state`: The routine repeatedly invokes another routine and, when a flag is set, swaps two bytes in a table indexed by a global counter.
- `clear_global_flag`: The routine checks a global variable for a null value and a specific set of values.
- `empty_routine_stub`: The routine contains 14 instructions and performs no observable work, returning immediately.
- `check_global_range`: The routine reads a global variable and tests whether it is zero or between one and twenty, or greater than five, returning zero when the condition holds and one otherwise.
- `clear_state_and_loop`: The routine iterates through a range of offsets, clearing specific memory locations by writing zero bytes.
- `decode_and_clear_table`: The routine builds a byte value from a bitstream stored in memory and then uses it to index a table, clearing a set of bytes in a global structure and writing a byte from the table into the structure.
- `toggle_global_flag`: Calls another routine repeatedly while modifying a global variable.
- `update_three_fighters`: Runs the fighter update for all three fighters: the computer fighter first, then the two players in an order that alternates with a frame bit.
- `toggle_global_state_sequence`: The routine repeatedly calls another routine while modifying a global variable by adding and subtracting specific constant values.
- `bout_frame_sequence`: One frame of a bout without the judge: prepares the frame, updates the three fighters, draws them and finishes the display.
- `call_subroutines_conditionally`: Calls three other routines unconditionally.
- `loop_with_conditional_calls`: The routine repeatedly checks a global variable and calls another routine multiple times per iteration.
- `load_configuration_table`: Increments a global counter until a condition in a lookup table is met, then loads a series of 32-bit values from a fixed memory region into global variables.
- `handle_input_byte`: The routine reads a byte from a pointer and uses it together with a global variable to update the global variable and call another routine.
- `check_fire_button_state`: Reads the CIAA_PRA register to check the status of the joystick fire button.
- `store_long_from_pointer`: The routine reads four consecutive bytes from the memory pointed to by a1 and combines them into a 32-bit value, then writes that value to a global variable.
- `call_three_routines`: Calls three other routines sequentially in succession.
- `set_global_and_call_routine`: The routine stores a constant value in a global variable and then invokes another routine four times in succession before returning.
- `update_timer_state`: Checks a global variable against a specific value and updates it using a lookup table or sets it to a default.
- `set_global_and_recurse`: The routine writes the constant 6 to a global variable, then conditionally overwrites that variable with a byte from a small table indexed by bits of the variable, and finally calls another routine twice before returning.
- `call_sequence_routines`: Invokes another routine four times in succession.
- `draw_four_bobs`: Draws four objects in a row by calling the masked bob blitter routine four times with the prepared registers.
- `decrement_flag_table_bytes`: Iterates through a range of memory locations, decrementing bytes in a specific table if corresponding entries in another table are non-zero.
- `draw_flash_pair_a`: Draws a two-part object with two bob blits using the first image pair; called on alternate phases of a countdown so the object flashes.
- `draw_flash_pair_b`: Draws the same two-part object with the second image pair, the other half of the flashing effect.
- `decode_byte_stream`: The routine repeatedly reads a byte from a global variable and a table indexed by an accumulator, combines bytes from the accumulator and the global variable, and writes the resulting byte to a memory location pointed to by an offset.
- `load_karate_character_data`: Calculates an index from a global variable and uses it to fetch character attributes from a table.
- `select_table_index`: The routine reads a global variable and indexes a table with it to obtain an index, then uses that index to pick a byte from the table and compares two bytes at a fixed offset from the chosen entries.
- `evaluate_game_state_flags`: Reads specific bytes from a global structure to determine game state conditions.
- `self_4`: The routine repeatedly calls another routine and compares the low byte of a register with the low five bits of the returned value.
- `compute_state_flags`: Reads bytes from two memory structures and combines them with conditional offsets based on a zero-check.
- `clear_byte_table_and_search`: The routine clears a byte table and then performs a nested search through a table of pointers.
- `infinite_loop_when_flag_set`: The routine checks a global variable.
- `wait_for_vertical_position`: The routine reads a global variable, extracts bits to form a value and repeatedly waits until that value is at least a first input and at most a second input.
- `set_up_three_fighters_alt`: Alternative entry to the three-fighter set-up: sets the fighter count to three and a mode value of one, then joins the set-up loop.
- `set_up_three_fighters`: Prepares a new bout: runs the common set-up, then for each of the three fighters runs the fighter initialisation and its first update.
- `reset_flag_and_call_helpers`: The routine clears a global variable, calls another routine twice, sets the global variable to one, then calls a further routine before returning.
- `clear_memory_and_initialize_globals`: The routine clears a block of memory by writing zero bytes to consecutive locations, then writes constant values to several global variables and to a set of four global pointers.
- `call_and_return_result`: The routine calls another routine and returns its result unchanged.
- `compute_obfuscation_checksum`: The routine increments a global variable by several constants, extracts its high byte, and returns a value derived from the variable combined with bytes read from another routine using the variable as an index.
- `decode_sprite_attribute`: The routine reads a byte from a table indexed by a global variable, updates the global variable and calls another routine, then uses the returned value to index further tables and perform arithmetic on bytes.
- `write_sprite_mask_tile`: The routine writes masked bytes from a global variable table into two parallel framebuffer areas for a 7-row tile.
- `update_global_and_call_routine`: The routine reads two bytes from a table indexed by a global variable, updates a counter byte in that table and clamps it, then calls another routine six times while toggling the global variable between the two table bytes based on the counter.
- `update_bcd_counters_and_recurse`: The routine checks a global variable and, if it is zero, reads a pointer from it, then updates two BCD-encoded bytes through indirect pointers derived from that pointer and from a byte at an offset from it, applying BCD adjustment to each.
- `save_and_restore_global_variable`: The routine saves the current value of a global variable, clears it, calls another routine, then restores the original value from the saved copies.
- `decode_global_byte_table`: The routine checks a global variable and, if it is non-zero, it clears it and calls another routine.
- `draw_masked_pair`: Draws a two-part object with two calls of the masked plotting routine.
- `draw_indicator_bob`: Draws one indicator bob at a position taken from the current player's table entry.
- `initialize_player_state_tables`: The routine clears a block of bytes in a table and a small byte array, then computes per-player values from two input tables and stores derived bytes into three output tables.
- `normalize_global_byte`: The routine reads a global variable and applies a small conditional transformation to it.
- `restore_global_variable_and_call`: The routine saves three bytes from a global variable, modifies the variable with a character constant and a conditional table lookup, clears it, computes a new value, calls another routine, restores the saved bytes in reverse order, and calls another routine again.
- `decrement_flag_loop`: The routine sets a global variable to seven and then repeatedly decrements it while calling another routine.
- `draw_panel_into_both_buffers`: Runs the drawing routine that follows into both display buffers in turn and restores the current-buffer pointer.
- `decode_script_bytes`: The routine walks a byte stream using an index stored in a global variable, reads bytes from a table pointed to by another global, and dispatches on the byte value.
- `self_13`: The routine builds an index from the low byte of an input register and a global variable, then repeatedly calls another routine, using its returned value to decide whether to continue looping, and finally increments the global variable and returns a combined value from the index and another input register.
- `self_5`: The routine checks a global variable and, if it is in a valid range, updates the variable and repeatedly calls another routine while adjusting the variable in a loop until it reaches a sentinel value.
- `return_global_variable`: The routine loads a global variable and returns its value.
- `apply_byte_mask_to_table`: The routine iterates over a block of memory using a pointer and a counter.
- `draw_panel_current_buffer`: Calls the drawing routine that follows once, for the current buffer only.
- `copy_sprite_attribute_blocks`: The routine copies a fixed number of 40-word attribute blocks from a source table to a destination buffer built from a global variable base.
- `increment_counter_and_recurse`: The routine reads a short from a global variable and tests a byte at an offset from that variable.
- `draw_fighter_extra_pair`: Draws a two-part extra image for the current fighter when that fighter's flag is set.
- `draw_fighter_extra_pair_late`: Draws a two-part extra image for the current fighter when its flag is set and the bout counter is in a narrow window near forty.
- `dispatch_via_indexed_table`: The routine masks a global variable to a small index, calls a function taken from a table indexed by that value, then recombines the high byte of the variable with a byte read from a fixed offset relative to the variable and stores the result back into the variable.
- `draw_fighter_frame`: Draws the current fighter's animation frame into the drawing buffer: picks the frame from one of two frame tables (selected by a fighter flag), positions it from the fighter's coordinates and copies its runs of image words into the bitplanes.
- `decode_sprite_pattern`: The routine walks a table indexed by a global variable and decodes a packed sprite pattern into a destination buffer using a rotation and mask operation, then calls another routine.
- `xor_rotate_sprite_block`: Iteratively reads a sprite descriptor from a global table, computes a destination pointer and rotates the source words by a shift derived from the global index, then XORs the rotated words into a hardware-like buffer and writes a transformed copy back to the sprite structure.
- `four_steps_per_frame`: Calls the following step routine four times in a row, giving four steps in one frame.
- `clear_memory_words`: The routine clears a block of memory by repeatedly zeroing ten consecutive 32-bit words per iteration.
- `copy_strip_if_enabled`: When its enable word is set, copies the saved strip into the second screen buffer with the twenty-word row copier.
- `duplicate_word_table_entries`: The routine copies a 16-bit word from a source table to a destination table and writes the same value to three additional offsets within each destination entry.
- `reset_animation_state`: The routine reads a byte from an object structure and, if it is zero, increments a counter in a global variable and conditionally calls another routine.
- `initialize_sprite_structure`: The routine writes a byte from a register into two byte fields of a structure pointed to by a register, copies a byte from a global table indexed by that register into two byte fields of the same structure, and then clears several byte fields of the structure and sets one byte field to one.
- `process_player_state_flags`: The routine reads and updates a set of bytes in a global structure indexed by a global variable, manipulates several flag bytes with bitwise operations, copies values between fields of the structure, and repeatedly calls another routine while comparing bytes from a table with values stored in the structure.
- `update_byte_with_table_offset`: The routine reads a flag byte at an offset from a base pointer and, depending on its value, updates a byte at another offset using a global variable table indexed by two bytes from the same base pointer.
- `update_byte_table_entry`: The routine updates a byte in a per-object block using an index derived from a global table and a flag, then clamps the stored value and writes a constant to a global variable.
- `match_over_set_up`: Phase 4 entry: unless already done, sets the two flash counters, clears the fifty-byte name table, picks the result row for one or two players and prepares the end-of-match results and high-score check.
- `initialize_game_state`: The routine clears several bytes in a table for two indices, calls another routine repeatedly, and then writes a fixed sequence of values to a global variable.
- `reset_global_flag_and_call`: The routine sets a global variable to a constant, calls another routine twice, then sets the global variable to two different constants, masks it with a constant, calls another routine once, and returns.
- `reset_low_nibble_flag`: The routine writes a constant to a global variable, then clears it, masks the variable with the low nibble, and calls another routine.
- `self_calling_state_toggle`: The routine calls another routine repeatedly and toggles a global variable between 1 and 0 before conditionally setting it to 2 and performing further calls.
- `advance_scrolling_object`: Moves the scrolling object one step when it is active: erases it at the old position, advances the fine position by two in a 64-step cycle (the coarse position drops by one on each wrap, modulo 8) and draws it again.
- `self_14`: The routine repeatedly calls another routine four times with different repetition counts.
- `increment_global_counter`: The routine calls another routine through an address in a register and then increments a global variable by one.
- `decode_screen_tiles`: The routine decodes a block of screen tiles using a small lookup table and a mask derived from input registers.
- `apply_sprite_tile_pattern`: The routine writes a 4×4 block of bytes derived from a small pattern table and a control byte into a tiled region of a global buffer, using a computed base offset and a stride of 160 bytes per row.
- `initialize_display_state`: The routine clears several global variables, repeatedly calls another routine while a global byte stays below a threshold, and updates a few global bytes based on conditions derived from those globals.
- `mask_and_xor_global_byte`: The routine reads a byte from memory addressed by a global variable offset, masks the low nibble and stores the result back into the global variable.
- `call_loop_with_decrement`: The routine calls another routine once, then enters a loop that calls the same routine repeatedly while decrementing a short counter from seven down to negative one, and finally returns the concatenation of the two input registers.
- `reset_and_run_eight_items`: Clears the item counter, runs the set-up routine and then the per-item routine eight times.
- `choose_background_colours`: Chooses the two background colour values for the frame: fixed colours in phase 6, otherwise per-player colours from a table when that player is active, with defaults when not.
- `compare_d7_to_table`: The routine loads a short zero into a loop counter and repeatedly compares an input short from D7 with successive entries of a global short table, advancing the counter by two each iteration.
- `check_at_signs_in_table`: The routine clears a global variable, writes two constants to it, then iterates over a table starting at index zero up to a limit.
- `initialize_bcd_counter`: The routine clears a global variable, calls another routine, then sets up a BCD-adjusted loop that repeatedly increments the variable using a flag derived from a countdown byte.
- `print_bcd_byte`: Prints a two-digit packed decimal byte through the character routine twice, suppressing leading zeros.
- `self_modifying_loop`: The routine repeatedly writes a byte to a global variable and then enters a nested loop that builds a 16-bit value from a word counter and the global byte, calls another routine twice conditionally based on carry and sign flags, and updates the counter from the routine’s return value.
- `compare_and_select_byte`: The routine reads a byte from an offset in a structure pointed to by a register, compares it with the same offset in another structure pointed to by a second register, and if the two bytes are equal it returns the byte from a second offset in the first structure; otherwise it returns the first byte.
- `swap_bytes_in_table`: The routine swaps a byte at a fixed offset within a pair of structures repeatedly.
- `reset_game_state_3`: The routine clears four global variables to zero and then returns.
- `draw_into_both_buffers`: Runs the screen drawing routine once for each of the two display buffers by pointing the current-buffer variable at each in turn, then restores it.
- `copy_sprite_attribute_blocks_2`: The routine copies a block of 40 longwords from a source table to a destination buffer derived from a global variable, repeating the copy for 83 iterations.
- `erase_bobs_and_reset_list`: Erases the objects drawn in the current buffer last time by restoring the saved background under each one, then resets that buffer's drawing list.
- `restore_background_under_bobs`: Walks the current buffer's list of drawn objects (screen address and size, up to sixty entries) and copies the clean background back over each rectangle.
- `zero_global_table_entries`: The routine clears a contiguous block of entries in a global variable table by writing zero to each element from the first entry up to a higher offset, effectively resetting the whole table.
- `empty_recursive_routine`: The routine contains no instructions and immediately returns.
- `clear_zeroed_table_2`: The routine clears a block of memory by writing zeros to a series of long words.
- `clear_sprite_attribute_table`: The routine clears a block of memory referenced by a pointer in a0.
- `rotate_word_table`: The routine rotates a 5-word block from a table indexed by an input word and a flag, using a rotation amount taken from a register, and ORs the rotated words into a destination buffer at a fixed offset from a register, repeating for eight iterations.
- `shift_lfsr_state`: The routine updates a set of 16-bit words located in a global variable by repeatedly shifting each word left by one bit and feeding back the bit that falls out of the previous word, forming a chain of linear-feedback shift registers.
- `reset_game_state`: The routine clears a small table of four words at a fixed address and then repeatedly writes a series of constant bytes to a global variable and to memory locations indexed by that variable.
- `set_global_palette_entries`: The routine writes a sequence of constant byte values to a single global variable, then invokes another routine twice.
- `self_modifying_loop_2`: The routine repeatedly calls another routine five times, optionally calls it a sixth time depending on a global variable, then writes a constant to that global variable and calls the routine again, looping until the global variable becomes non-zero.
- `check_and_call_flagged_routine`: The routine tests a global variable for non-zero and non-zero again, then calls another routine and checks if the same global variable equals one.
- `clear_sprite_flags`: This routine clears a block of one-byte flags in memory.
- `set_global_from_table_byte`: The routine reads a byte from a table indexed by a global variable and compares it with '.' and '*'. If the byte matches one of those values, the routine writes constant values to a global variable.
- `process_state_table`: The routine checks a global flag and, if set, calls another routine and then iterates over a table of state structures by index, updating bytes in each structure based on flags and values read from the structures.
- `decrement_or_reset_counter`: The routine reads a global variable, uses it as a base for a structure, and decrements or increments a byte field in that structure, resetting it when it underflows or overflows.
- `update_player_stats`: The routine updates several byte fields within a structure pointed to by a global variable, performing addition with carry propagation between related bytes.
- `self_15`: The routine reads a byte from a structure pointed to by a register, inverts it, writes the inverted value back plus one, and updates another byte in the same structure with an inverted value plus a conditional increment.
- `self_6`: The routine reads a byte from an offset in a structure pointed to by a register, inverts it, writes the inverted value back plus one, and updates another byte in the same structure with an inverted value plus a conditional increment based on the previous byte.
- `shift_byte_pair_2`: The routine reads a byte from an object pointed to by a register, shifts it right by one bit and writes the result back to the same location.
- `shift_byte_pair`: The routine reads a byte from an object pointed to by a register, halves it by shifting right, and writes the result back to the same location.
- `decode_sprite_state_table`: The routine reads and updates bytes in a small table of sprite-like records.
- `decrement_global_until_negative`: The routine sets a global variable to three and then repeatedly calls another routine while decrementing the variable each iteration.
- `restore_object_background`: For the current bonus object, if it has a saved screen position, copies the matching background words back over the eight rows of the object in every bitplane.
- `rotate_xor_table_update`: The routine reads a byte from a global variable offset and uses it to select a source table entry, then conditionally updates a 32-bit table entry by rotating a 16-bit word from the global variable and XOR-combining it with a rotated 16-bit word from the source table.
- `fill_table_from_callee`: The routine calls another routine three times in a loop and stores the value returned in a register into consecutive entries of a global table.
- `clear_and_update_flags`: The routine clears two flag bytes for each of four entries in a table and then, for each entry whose table value is non-zero, it calls another routine and, depending on a global variable and a register value returned by a call, sets the two flag bytes based on the high and low parts of a value returned by another routine.
- `decode_byte_from_table`: The routine decodes a byte from a table pointed to by a global variable and returns a value derived from it.
- `reset_input_state_and_wait_for_key`: The routine clears a small block of memory at two fixed tables, then repeatedly calls another routine while polling a global variable that acts as an input index.
- `initialize_palette_and_clear_buffer`: The routine writes a fixed sequence of byte values to a global variable, then calls another routine eight times in a row.
- `mask_byte_loop`: The routine repeatedly calls another routine and, while a global variable is non-zero, masks the low nibble of a byte located at an offset from the variable’s value.
- `check_player_state`: The routine reads a global variable as an index, then checks two bytes in a table referenced by that index.
- `update_bonus_objects`: In phase 7, when the bonus objects are active, calls the type handler of each of the ten live objects through the handler table, counting them, then runs the follow-up routine unless its flag is set.
- `initialize_sprite_state`: The routine initializes a set of byte fields in two structures located at offsets from two small indices, clears several long fields in those structures, writes a byte derived from a table lookup into a byte field of a third structure, and updates a global counter and a global variable.
- `check_global_and_call`: The routine tests a global variable for non-zero and equal to five twice, and if the condition holds it calls another routine each time.
- `reset_stateful_routine_flags`: The routine saves a set of global variables and a hardware register, then checks a byte offset from a base pointer and, if in range, increments a flag byte and indexes a table to obtain a value.
- `clear_flag_and_call`: The routine checks a byte in a table indexed by a global variable.
- `save_object_screen_position`: Stores the current screen pointer in the bonus object's saved-position slot and draws the object with the masked plotting routine.
- `copy_sprite_attributes`: The routine copies sprite attribute bytes from a source structure referenced by a global variable into a destination structure, adjusting a mask byte and writing several fixed attribute bytes.
- `self_16`: The routine reads a byte from a table indexed by a value derived from a global variable and an input register, and if the byte is non-zero it calls another routine twice with arguments built from bytes in the same table entry and then stores a result from the second call into a table indexed by the input value.
- `clear_opponent_state`: The routine clears a per-opponent state entry and, when a flag is set, invokes another routine with parameters derived from bytes in a per-entity structure and a global table.
- `music_frame_all_channels`: Music player frame: runs the channel sequencer for each of the four audio channels.
- `music_channel_sequencer`: Music channel sequencer: counts down the current note, then reads the channel's command stream (note, sample, loop and pointer commands through a jump table) and programs the channel's period, volume and sample with DMACON.
- `decrement_word_and_update_pointers`: The routine writes a word from a pointer to an object field, decrements that word, advances a pointer field, copies a long from one object offset to another, copies a word from one object offset to another, clears a word at an offset from an unused register, and stores the address of a global variable into an object pointer field.
- `execute_external_routine`: The routine is twelve instructions long and is entered from outside the static flow.
- `execute_two_routines`: The routine contains six instructions and calls two other routines.
- `music_cmd_set_channel_pointer`: Music sequence command: copies the next longword of the command stream into a pointer field of the channel record and continues with the next command.
- `music_cmd_set_two_fields`: Music sequence command: copies the next two longwords of the command stream into two fields of the channel record and continues with the next command.
- `invoke_subroutine`: The routine consists of seven instructions and calls another routine, passing control to it.
- `music_cmd_clear_flag_and_loop`: Music sequence commands: clears a channel flag and continues; the following entry restarts the channel's sequence from its saved start pointer.
- `reset_game_state_2`: The routine clears a number of global variables and a small block of memory by writing zeros through a pointer, then calls another routine.
- `title_logo_frame`: One frame of the animated title sequence: updates the logo objects, the second effect on even frames only, and the remaining title elements in a fixed order.
- `clear_table_entries`: The routine clears a block of word entries in a global table and then writes a constant to two global variables.
- `decrypt_global_bytes`: The routine increments a global variable and then processes twenty bytes of a global buffer in a loop.
- `initialize_sprite_tables_2`: The routine fills a series of entries in a table with zeros and with values read from a global variable and from offsets relative to an index taken from the global variable, then it clears specific bits in four words of a global variable.
- `write_masked_bytes_to_table`: The routine reads a byte from a global table using a running index and XOR-masks a byte in a working buffer, then copies the same byte to three fixed offsets in that buffer and masks another offset.
- `self_7`: The routine computes several byte values by xoring pairs of bytes from RAM and writes each result to four mirrored locations derived from a pair of global short variables, using both the variable and its negation as bases.
- `initialize_global_table`: The routine checks a global variable and, if it is zero, increments it, shifts it right, and then performs a loop that updates a series of bytes in the same global memory area by adding a byte taken from a position derived from the global variable and the loop index, masking the result.
- `rotate_and_copy_sprite_words`: The routine updates a global counter and dispatches on its low bits.
- `decode_input_byte`: The routine reads a byte from a global variable and, depending on its value, clears two control flags and sets a state variable, then returns a value obtained from another routine.
- `increment_global_index`: The routine masks a global variable with three, and if the result is zero it updates the global variable by incrementing or decrementing it depending on a byte from a table indexed by the variable, then compares the variable with a byte from another table and possibly clears a global dword and a global variable before calling another routine.
- `copy_sprite_blocks`: The routine copies a fixed number of 40-byte blocks from a source area derived from a global variable to two destination areas derived from the same global variable.
- `mask_sprite_planes`: The routine masks sprite planes using a global variable as an index.
- `copy_opponent_sprite_data`: A routine that updates a global variable with small arithmetic and bit operations and then copies or masks word fields from a source table into two destination tables using a rotating mask derived from a table entry.
- `reset_one_shot_flag`: The routine checks a global variable for a specific value and a non-zero flag stored in another global variable.
- `title_object_grow_step`: Title sequence object step: every fourth frame advances a sub-counter and, every twelve steps, grows the object size by three until it reaches its table limit, then draws it with the bob routine.
- `process_global_state`: The routine reads and writes a global variable and uses another routine to obtain values.
- `reset_flag_and_recurse`: The routine checks a global variable and a flag stored in a global variable.
- `copy_struct_array`: The routine copies a fixed set of fields from one table entry to another, repeatedly for a number of entries.
- `reset_animation_state_2`: The routine resets a set of global variables when a flag is set, then updates an index and a counter using values read from a table indexed by the result of another routine.
- `reset_and_poll_input_state`: The routine resets a set of globals when a flag is set and then conditionally calls another routine.
- `reset_and_update_state`: The routine resets a set of global variables when a flag is set and then updates several counters and reads bytes from a table indexed by those counters.
- `conditionally_recurse_and_clear_flag`: The routine checks a global variable for a specific value and a flag variable for non-zero.
- `compute_sprite_tile_offset`: The routine computes a word offset from two input values and a table base pointer, then uses the result to index a table and conditionally clears a word.
- `self_8`: The routine computes an index from the low twelve bits of an input value, combines it with a masked input and an external pointer, and writes the result to a global variable.
- `apply_masked_bitplane_blend`: The routine blends four source bitplanes into four destination bitplanes using a mask, producing a masked copy of the source into the destination.
- `process_sprite_masks`: The routine iterates over a set of mask words, builds four rotated masks from four source words per block, combines them with a control word and writes the resulting mask to an optional output table.
- `check_input_range_and_call`: The routine checks a masked value from a register against a threshold and, if in range, derives an index from another register, tests it against fixed ranges and conditionally calls another routine.
- `check_input_range_and_dispatch`: The routine checks a masked value from a register against a threshold and, if the condition holds, masks another register value and tests it against several ranges.
- `self_17`: The routine processes four 16-bit words from a source table, combines them with a rotating mask derived from a global variable and a rotating counter, and applies the mask to four 32-bit words in a destination buffer using bitwise AND and OR operations.
- `check_collision_mask`: The routine builds a 32-bit mask from four words of a source structure and compares it against two tables indexed by input values.
- `mask_tile_words_with_rotated_key`: The routine masks four 32-bit words per iteration using a rotated key derived from a global variable and the input word.
- `apply_sprite_mask_to_tiles`: The routine iterates over a set of tile entries and applies a per-pixel mask derived from a source word and a rotation amount.
- `apply_rotated_word_mask_to_table`: The routine conditionally iterates a fixed number of times, building a 32-bit mask from a word in a table by rotating it and mixing it with an accumulator value, then applying the mask with a bitwise AND to four 32-bit words spaced 40 bytes apart in a source table and OR-ing the result into the corresponding words of a destination table.
- `apply_sprite_mask_to_screen`: The routine masks a block of screen words with a per-pixel mask derived from a source word and writes the result through a pointer computed from input values, using a global table as a mask source.
- `self_18`: The routine clears a large block of short words by subtracting a constant, zeroes a small code area, resets several global variables and then copies a set of words from one memory area to another.

### logged disk load from International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:intro#bytekiller+E8 unpacked at an address (131072 bytes)

- `set_byte_flag`: The routine receives a pointer in A0 and sets a flag bit in the byte immediately before the pointed-to address.
- `set_global_variable_table`: The routine writes a constant to a global variable, inverts it, and then conditionally copies four words from one of two tables into the same global variable.
- `self_9`: The routine walks a zero-terminated string at a fixed address comparing each byte with the previous one and decrements a counter.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `init_audio_channels`: The routine increments a global counter and, on the sixth call, resets state, invokes another routine four times, and sets up a loop that writes to four audio channel base registers.
- `handle_vampire_sound`: The routine decodes a nibble from a control byte and updates a short value in a structure, clamping it, then copies it to a caller’s field.
- `decode_byte_operations`: The routine reads a byte from a pointed structure and uses its high and low nibbles to index tables.
- `self_modifying_stub`: The routine clears a global variable, calls another routine, complements the variable, calls the other routine again, then increments the variable.
- `set_global_variable`: The routine writes a constant value to a global variable and then returns.
- `intro_wait_first_frame_enable_dma`: Enables the cracktro's frame interrupt, waits until the interrupt has run once, then switches on bitplane, copper, blitter and sprite DMA.
- `initialize_graphics_library`: The routine calls another routine through a global table and stores the result in a global variable.
- `clear_interrupt_flags`: The routine writes to a global variable and then writes a constant to it, effectively clearing a flag.
- `clear_global_variable`: The routine writes a constant to a global variable, reads the same global variable back, writes zero to the global variable, and returns.
- `intro_allocate_buffers`: Cracktro set-up: allocates one block of chip memory through exec and splits it into the screen and work buffers it needs, giving up if the allocation fails.
- `initialize_display_registers`: The routine writes a series of constant values to a set of global variables.
- `intro_build_copper_lists`: Cracktro set-up: allocates two chip-memory blocks through exec for its copper lists and writes their instructions (colour bars, bitplane pointers and the scroller split).
- `initialize_graphics_tables`: The routine writes a series of word constants into a global buffer and builds two small tables by copying words from fixed memory areas into the buffer with incremental offsets.
- `unknown_routine`: The routine reads a global variable, calls a routine through a pointer derived from that variable, then calls another routine through a pointer derived from a different global variable.
- `disable_interrupts_and_clear_custom`: The routine writes to INTENA with bit 15 set and then writes to a custom hardware register, effectively disabling interrupts and clearing a custom register.
- `invoke_five_callbacks`: The routine calls a function pointer taken from a global variable five times in a row and then returns.
- `clear_global_variable_2`: The routine reads a byte from a global variable and then writes zero to that same global variable.
- `set_global_variable_3`: The routine stores a constant value into a global variable and then returns.
- `or_data`: The routine combines four word tables with a bitwise OR into a fifth word table, iterating a fixed number of times.
- `xor_decode_table`: The routine decodes a 16-entry table by XORing each word from a fixed table with a constant and ORing the result with a constant, writing the results to a global variable area, then clears a global variable.
- `configure_blitter_and_update_table`: The routine programs the Amiga custom chip blitter registers and DMA control, then iterates over a table of 16-bit entries, increments each entry modulo 128 and copies a value from a second table indexed by the original entry.
- `set_global_flags_2`: The routine writes two values to a global variable and then returns.
- `reset_global_flag_2`: The routine clears a global variable and then checks a byte in a table indexed by another global variable.
- `clear_global_flag_2`: The routine checks a global variable and a table entry indexed by another global variable.
- `self_10`: The routine checks a byte from a memory-mapped I/O area and, depending on its value, reads a word or long from the following bytes and adds a small constant to it, storing the result in a global variable.
- `set_player_state_for_two_frames`: The routine writes fixed values to several words in a structure pointed to by A5, then loops twice.
- `initialize_player_state`: The routine clears several words in a structure pointed to by A5, sets a few fixed word values in that structure, then loops twice to build two 32-bit pointers from a global table base and an index, writes those pointers and a constant word into the structure, and finally returns.
- `set_palette_and_tile_pointers`: The routine increments a global counter modulo 128 and returns early if the counter is already non-zero.
- `set_palette_and_sprite_pointers`: The routine computes a palette index from a table entry and a flag, then writes four word values to a pair of global tables depending on a global flag.
- `poll_global_variable`: The routine repeatedly increments a global variable by two and reads a short from a fixed offset relative to it, breaking when the read value is not -1.
- `fill_screen_tiles`: The routine writes a repeating pattern of 22-word blocks into a short buffer pointed to by a1.
- `rotate_word_table_2`: The routine decrements a global counter and, when it reaches zero, resets the counter and processes a block of words from a table.
- `initialize_memory_table`: The routine checks a global variable by calling another routine through an indirect pointer and loops forever if the result is zero.
- `decode_table_bytes`: The routine decodes a table of 16-bit words by swapping each word’s bytes and applying a small rotating subtraction to the low byte.
- `copy_palette_entries`: The routine copies a fixed number of 16-bit entries from a table indexed by a global variable to four separate tables.
- `fill_and_clear_word_table`: The routine computes a step value from a global variable and uses it to fill a 28-word table starting at that variable with a running sum, then writes a decreasing sequence into the following 28 words.
- `copy_word_pairs_to_tables`: The routine copies 40 word pairs from a source table in chip RAM to four destination tables in chip RAM, writing the first word of each pair to one destination and the second word of each pair to another destination.
- `fill_palette_table`: The routine writes a series of constant word values into a table pointed to by a register, initializing twelve consecutive entries.

### logged disk load from International Karate + (1988)(System 3)[cr Oracle][m Invisible Crime].adf:ik+++ hunk 0 +32..+132 copied to an address

- `bytekiller_depack_game_copy`: The ByteKiller depacker as it runs from 500 hex (a verbatim copy of ik+++ hunk 0 from offset 32 hex): moves the packed data up, unpacks the game to 600 hex onwards, checks the checksum and jumps to the game entry.
- `bk_refill_bit_buffer_copy`: Bit buffer refill of the depacker copy at 500 hex: takes the previous packed longword, adds it to the checksum and shifts out the first bit with the end marker set.
- `bk_read_bits_copy`: Bit reader of the depacker copy at 500 hex: shifts D1 bits of the packed stream into D2, refilling the 32-bit buffer as needed.

## Notes from the b1-1907 agent (1 Nov 2026)

- Variant: the AmigaDOS release cracked by Oracle with the Invisible Crime intro (boot block, startup-sequence,
  the cracktro, the game and the AmigaDOS Type command). It runs end to end on the reference harness: cracktro,
  title, credits, key help, demo fights, one and two player games, the bonus stage, match over and the high-score
  table. The other IK+ project in Ami-Go is a WHDLoad install of the same game; this one carries the cracktro.
- The game and the cracktro are each one code hunk holding a ByteKiller depacker and a packed stream. The first
  automatic run saw only the depackers. This run unpacks both streams with a depacker written from the recovered
  one (checksum verified), recovers the unpacked images as units at the addresses the programs unpack them to, and
  recovers the depacker copy the game runs from low memory as its own unit.
- Code was found by region: every non-code gap in the code areas was decoded and read by hand. Real code found
  that way: the bonus stage's object handlers reached through an absolute pointer table, and four unreferenced
  routines. Buffers and tables that an earlier rule had decoded as code (a zero-filled buffer of about six
  kilobytes among them) were returned to data.
- Copy protection: none found. No disk hardware access, no trackdisk I/O, no manual, word or code-wheel lookup,
  no checksum over code and no trace-vector trick in the game or the cracktro. The original protection was in the
  publisher's loader, which this release replaced with AmigaDOS files. The keyboard code words are game cheats.
- Cracktro: the Invisible Crime intro (starfield, globe, copper bars, scroller, music; waits for the left mouse
  button) is recovered and labelled cracktro. There is no trainer in this release.
- Naming: names come from deterministic rules, two local models (sharded across Ryzen and IntelPC) and hand
  entries; the routines the models described wrongly in the review (for example calling a wrapper "recursive", or
  calling the pause idle loop with the frame interrupt behind it "empty") were rewritten by hand. A full
  read-through of every description happens in stage 3, where each routine is ported.
- Reassembly: every unit is identical in bytes and relocations; the cracktro's executable file additionally carries
  an empty relocation block after its code hunk, which vasm does not write, so that one file is hunk-identical
  rather than file-identical.
