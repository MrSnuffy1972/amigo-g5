# Game 3304 (DaleyThompsonsOlympicChallenge v1.0 1Disk 0957): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03304). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1881982 top-level bytes in 54 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 167 of 167 named (0 author symbols), 167 of 167 described (58 by rule, 103 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 986880 bytes, 152 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5230 of 5230 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 104008 |
| code | 490 |
| data | 1777482 |
| padding | 2 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 7 | 3 |
| dtoc | 1 | 160 | 12 |
| event1d#unpacked (raw code file, load address unknown) | 1 | 2346 | 70 |
| event2a#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| event4a#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| event5e#unpacked (raw code file, load address unknown) | 1 | 2346 | 70 |
| event6a#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| event8a#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| event9a#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| ts#unpacked (raw code file, load address unknown) | 1 | 371 | 12 |

## Cases handled

- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957.hdf: hard-disk image (AmigaDOS OFS, 2080 blocks): its filesystem parses and all 29 files in it are byte-identical to the extracted tree DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf, which is recovered file by file; no block outside the filesystem holds data
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event1a: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (14786 -> 32052 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event1b: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (29465 -> 64820 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event1c: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12522 -> 32052 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event1d: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (8412 -> 22528 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event2a: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (115653 -> 223744 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event3a: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (6134 -> 32052 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event3b: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (25328 -> 64820 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event3c: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (4516 -> 32052 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event4a: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (177725 -> 256256 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event5a: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (11898 -> 32052 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event5b: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (14997 -> 32052 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event5c: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (29465 -> 64820 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event5d: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (12522 -> 32052 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event5e: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (8412 -> 22528 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event6a: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (26264 -> 90112 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event7a: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (10850 -> 32052 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event7b: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (25757 -> 64820 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event8a: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (24885 -> 102912 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event9a: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (33373 -> 113664 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/hiscores: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (238 -> 512 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/medal: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (14190 -> 32576 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/pic0: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (26575 -> 32576 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/ts: ProPack (RNC) Data (xfd)-packed, unpacked with ancient (72305 -> 155136 bytes)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/dtoc: RNC1: Rob Northen RNC1 Compressor (old) stream found by ancient scan (85200 -> 146432 bytes) but only 0 of 62 sampled windows are in reference RAM: not used
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event1d#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event2a#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event4a#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event5e#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event6a#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event8a#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/event9a#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/ts#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957/DaleyThompsonsOlympicChallenge_v1.0_1Disk_0957_hdf/DaleyThompsonsOC1Disk/data/dtoc: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 6 never-loaded region(s), 941824 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.

### dtoc

- `initialize_system`: Sets up the system by calling two routines through the exec library, configures the interrupt controller, clears a memory region, and jumps to another routine.
- `frag_move_lea_subq_bne_bsr_jmp`: Short code fragment that copies 85200 to d0, then loads the address of a fixed memory address into a1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then subtracts 4 from d0, then branches if the last result was not zero, then loads the address of a fixed memory address into a0, then calls another routine, then jumps to the address in a0.
- `copy_and_shift_bytes`: Copies bytes from one memory location to another while shifting the source pointer, and performs conditional byte manipulation based on loop counters and register values.
- `frag_move_sub_movea_bra`: Short code fragment that copies a2 to d0, then subtracts a3 from d0, then copies a3 to a0, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `frag_lsl_bne_move_roxl_rts`: Short code fragment that shifts d3 left by 1, then branches if the last result was not zero, then copies the memory just below a6 (moving a6 down) to d3, then rotates d3 left through the extend flag by 1, then returns.
- `decode_bit_coded_match_copy`: Builds a variable-length code one bit at a time, checks it against a mask and a lookup table, then copies a run of bytes backwards within the output buffer.
- `consume_bit_reader_values`: Calls the bit reader in a counting loop that stops on a flag, then keeps reading while the returned count is nonzero when a table entry at a computed index is set.
- `read_joystick_data`: Reads joystick input data using a lookup table and bit manipulation operations, accumulating results in a register.
- `read_joystick_fire_bits`: Reads the fire button bits into d7 by calling the helper with two different port masks, shifting each result into d7 and adding d1 before returning.
- `frag_moveq_lsl_move_dbra_rts`: Short code fragment that loads 3 into d1, then shifts d0 left by 8, then copies the memory at a0 (advancing a0) to d0, then loops back while counting down d1, then returns.
- `copy_bytes_and_zero_fill`: Copies d0 bytes from a0 to a1, then zero-fills the remaining bytes up to the original a0-a1 distance, restores registers from the stack and returns.

### event1d#unpacked (raw code file, load address unknown)

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `update_event_data_2`: Updates a global variable based on random selection and modifies event parameters according to specific conditions, then adjusts a counter and returns.
- `frag_cmp_bmi_subi_addq_bra_cmpi_beq`: Short code fragment that compares d0 with 6000, then branches if the last result was negative, then subtracts 6000 from d0, then adds 1 to the memory at a4, then jumps to another point in the code, then adds 1 to a4, then compares the memory at a4 with 58, then branches if the last result was zero.
- `increment_counter_with_bra`: Increments a byte value at a4 when d0 is greater than or equal to 1000, then branches back to itself with an incremented a4 pointer.
- `increment_and_add_value`: Increments a byte value at a4 when d0 is greater than or equal to 10, then adds the remaining d0 value to that same byte location and returns.
- `update_game_state_2`: Updates game state variables based on various conditions and performs calculations involving memory addresses and counters.
- `update_position_and_velocity`: Updates position and velocity values based on input from memory locations, performs bit shifts and additions, and calls a function at the end.
- `frag_move_movea_rts`: Short code fragment that copies a fixed memory address to d0, then copies an address to a0, then copies an indexed entry from a0 to a fixed memory address (3 times in a row), then copies an address to a0, then copies an indexed entry from a0 to a fixed memory address (2 times in a row), then returns.
- `handle_dma_sequence`: Processes DMA transfer sequences by checking memory addresses, updating counters, and performing calculations before returning.
- `handle_collision_and_update_graphics`: Handles collision detection by checking various conditions, updates graphics parameters, and calls a function when certain thresholds are met.
- `calculate_and_update_offsets`: Calculates memory offsets using bit shifts and additions, stores results in multiple locations, and calls another routine with specific values.
- `calculate_display_position`: Calculates a display position value by subtracting a word from a register, dividing by another word, adding one, and storing the result in a global variable.
- `compute_sprite_offset`: Computes a sprite offset value by subtracting a word from a register, dividing by eight, adding one, and storing the result in a global variable.
- `update_display_position`: Updates display position parameters based on input values, performs arithmetic operations, and handles conditional state changes.
- `frag_cmpi_bne_bpl_bra`: Short code fragment that compares a fixed memory address with 10, then branches if the last result was not zero, then compares a fixed memory address with 20, then branches if the last result was not negative, then jumps to another point in the code.
- `decrement_and_increment_registers`: Decrements two word values in a structure and increments two other word values in the same structure, based on conditional checks against global constants.
- `update_frame_counter`: Decrements a frame counter at offset 192 in the structure pointed to by a0, and updates it based on the value in register d0.
- `update_position_bounds`: Loads a value from a table at offset a fixed address using a word from the structure at a0, then compares it with the value at a1.
- `update_position_with_condition`: Checks a global variable and performs different position update operations based on its value.
- `update_game_state_3`: Reads a value from a global variable, performs conditional updates based on its range and bitwise properties, and modifies another global variable before returning.
- `update_multi_byte_counter`: Loads a global variable value into d1 and initializes d0 to 2 or 4 depending on the global variable.
- `configure_dma_and_blitter`: Configures DMA control registers to enable specific channels, waits for a blitter status bit to clear, sets another DMA register value, then returns.
- `validate_and_update_event_data`: Validates conditions based on memory comparisons and bit operations, then calls multiple routines to update event parameters and global variables accordingly.
- `update_event_graphics`: Loads a value from a table using an offset from a structure, then compares it with a fixed value.
- `update_event_graphics_2`: Updates graphical data for an event based on random selection and modifies event parameters according to specific conditions, then adjusts a counter accordingly.
- `update_event_data`: Updates a global variable based on random selection and modifies event parameters according to specific conditions, then adjusts a counter and stores the result.
- `initialize_event_data_2`: Initializes multiple global variables with specific values and performs a series of operations that modify these variables based on calculations involving shift and addition operations, then returns.
- `frag_clr_move_asl_bsr_andi_addq_add_dbr`: Short code fragment that clears d1, then copies 2 to d2, then shifts d1 left by 4, then calls another routine, then masks d0 with 3, then adds 2 to d0, then adds d0 to d1, then loops back while counting down d2, then copies d1 to a fixed memory address, then returns.
- `frag_btst_bne_beq_rts`: Short code fragment that tests bit 7 of a fixed memory address, then branches if the last result was not zero, then tests bit 7 of a fixed memory address, then branches if the last result was zero, then returns.
- `frag_btst_bne_rts`: Short code fragment that tests bit 7 of a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_movea_move_dbra_rts`: Short code fragment that copies an address to a0, then copies 63 to d0, then copies 65535 to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `frag_move_dbra_rts`: Short code fragment that copies 8191 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `copy_memory_block`: Copies memory from a0 to a1 in chunks of 2000 bytes, repeating this process three times.
- `frag_bsr`: Short code fragment that calls another routine (2 times in a row).
- `frag_move_bsr_subq_bne_rts`: Short code fragment that copies 63 to a fixed memory address, then copies a fixed memory address to d0, then calls another routine, then subtracts 1 from a fixed memory address, then branches if the last result was not zero, then returns.
- `swap_memory_blocks`: Swaps memory contents between two blocks of 192 words each, starting at addresses a fixed address and a fixed address.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `setup_blitter_parameters`: Configures blitter registers with specific values based on data from a structure at a1.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (clr.l) and then falls through into the code that follows, with no return of its own.
- `update_sprite_attributes`: Updates sprite display attributes including position and size based on table lookups, then configures bitplane DMA parameters for rendering.
- `frag_move_andi_asr_exg_adda_rts`: Short code fragment that copies a0 to d0, then masks d0 with 32767, then shifts d0 right by 2, then exchanges d0 and a0, then masks d0 with an address, then adds d0 to a0, then returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `frag_movem_moveq_move_beq_bsr_bra_rts`: Short code fragment that saves registers on the stack, then loads 0 into d0, then copies the memory at a4 (advancing a4) to d0, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then restores saved registers from the stack, then returns.
- `process_character_input`: Processes character input by checking its value against ASCII ranges.
- `load_input_sequence`: Reads a sequence of bytes from a pointer, stores each byte into a global variable, and processes each character until a null terminator is encountered.
- `process_input_char`: Processes a character input by mapping it to a value, adjusting offsets, and storing the result in a global variable, then adds a value from a table to another global variable before returning.
- `frag_movem_moveq_move_beq_bsr_bra_rts_2`: Short code fragment that saves registers on the stack, then loads 0 into d0, then copies the memory at a4 (advancing a4) to d0, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then restores saved registers from the stack, then returns.
- `process_input_character`: Processes character input similar to routine R8 but with different arithmetic operations.
- `setup_dma_transfer`: Sets up DMA transfer parameters by loading data from memory, calculating addresses, and writing values to DMA control registers.
- `frag_moveq_jsr_lea_bra`: Short code fragment that loads 0 into d0, then calls another routine, then loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_movea_bra`: Short code fragment that copies an address to a0, then jumps to another point in the code, then copies an address to a0, then jumps to another point in the code.
- `frag_lea_bra`: Short code fragment that loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_movea_bra_2`: Short code fragment that copies an address to a0, then jumps to another point in the code.
- `copy_color_data`: Copies word-sized data from two source locations to a third location using a loop that decrements a counter, then returns.
- `generate_random_number`: Updates a global variable with a multiplication result and shifts it right by eight bits, returning the lower 12 bits of the result.
- `lookup_table_access`: Reads a byte from a global location, processes it through bit rotation and inversion, then uses it as an index to fetch another byte from a table.
- `extract_low_nibble`: Returns the lower four bits of a word value while clearing the upper bits.
- `configure_display_dma`: Initializes various display and DMA control registers including bitplane modes, display window settings, and interrupt configurations, then enables DMA transfers by setting the appropriate control bits.
- `initialize_dma_transfer`: Sets up DMA transfer parameters and enters an infinite loop that never returns.
- `halt_in_infinite_loop`: Enters an unconditional infinite loop with no operations, so execution never returns from this point.
- `frag_cmpi_blt_clr_rts_bra`: Short code fragment that compares a fixed memory address with 2, then branches if the comparison was less, then clears a fixed memory address, then returns, then jumps to another point in the code.
- `setup_dma_and_interrupts`: Configures DMA and interrupt registers, then enters an infinite loop that never returns.
- `initialize_dma_and_interrupts`: Sets up DMA and interrupt control registers, clears a global variable, and configures hardware ports for input handling.
- `process_keyboard_input`: Checks several bits in hardware registers related to keyboard input and sets corresponding bits in a global variable based on the results.
- `clear_memory_block`: Fills a memory block with zeros using a loop that decrements a counter, then returns.
- `frag_move_bsr_dbra_rts`: Short code fragment that copies d0 to the stack, then calls another routine, then copies the top of the stack to d0, then loops back while counting down d0, then returns.
- `handle_input_event`: Processes input events by checking global flags and updating variables based on key values, with specific handling for space and 's' keys.
- `frag_jsr_move_rts`: Short code fragment that calls another routine, then copies 33792 to a fixed memory address, then returns.
- `frag_jsr_move_rts_2`: Short code fragment that calls another routine, then copies 33792 to a fixed memory address, then returns.
- `wait_for_dma_and_set_bits`: Waits for a specific DMA status bit to clear, then sets particular bits in a hardware control register.

### event5e#unpacked (raw code file, load address unknown)

- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `update_event_data_2_2`: Updates a global variable based on random selection and modifies event parameters according to specific conditions, then adjusts a counter accordingly.
- `frag_cmp_bmi_subi_addq_bra_cmpi_beq_2`: Short code fragment that compares d0 with 6000, then branches if the last result was negative, then subtracts 6000 from d0, then adds 1 to the memory at a4, then jumps to another point in the code, then adds 1 to a4, then compares the memory at a4 with 58, then branches if the last result was zero.
- `increment_and_compare`: Repeatedly subtracts specific values from a word and increments a byte pointer while comparing against thresholds, then returns.
- `update_counter_loop`: Processes a counter value by decrementing it in chunks of ten until it reaches zero, then increments a byte pointer and adds the remaining counter value to another byte location.
- `update_olympic_state`: Updates a global counter and status flags based on input conditions, then performs conditional calculations involving a lookup table and registers.
- `calculate_coordinates`: Performs arithmetic operations on values from memory locations, shifts results, and stores computed values to various memory addresses based on conditional checks.
- `frag_move_movea_rts_2`: Short code fragment that copies a fixed memory address to d0, then copies an address to a0, then copies an indexed entry from a0 to a fixed memory address (3 times in a row), then copies an address to a0, then copies an indexed entry from a0 to a fixed memory address (2 times in a row), then returns.
- `update_olympic_state_2`: Updates a global counter and handles state transitions based on comparison results, then performs calculations involving a table lookup and writes the result to a global variable before calling a DMA setup routine.
- `update_game_state`: Updates game state variables by performing subtraction, bit shifting, and conditional assignments while also modifying memory locations with specific values based on comparison results.
- `calculate_pointer_offset`: Calculates a memory address offset using bit shifts and additions, stores the result, and performs conditional operations based on a counter value before calling another routine twice.
- `calculate_position_adjustment`: Performs arithmetic operations on values from a structure to calculate a new position adjustment, storing the result in a global variable after applying division and increment operations.
- `compute_alternate_position`: Calculates an alternative position value using subtraction and division operations with fixed constants, then stores the computed result in a global variable for display positioning.
- `update_olympic_position`: Updates a global variable based on a condition check, then performs conditional operations on multiple global variables before returning.
- `frag_cmpi_bne_bpl_bra_2`: Short code fragment that compares a fixed memory address with 10, then branches if the last result was not zero, then compares a fixed memory address with 20, then branches if the last result was not negative, then jumps to another point in the code.
- `decrement_and_compare`: Decrements a word value at offset 192 of the structure pointed to by a0, then compares it with -246 and branches if less than.
- `update_animation_frame`: Manages animation frame updates by decrementing a counter, checking conditions, and updating memory locations with values from tables based on current frame state.
- `validate_position_range`: Checks if a position value is within a valid range by comparing differences with a reference value, then updates the position and a related memory location accordingly.
- `handle_collision_detection`: Performs collision detection by comparing position differences with thresholds, updating positions and memory locations based on whether conditions are met or not.
- `update_score_counter`: Loads a value from a global variable, performs conditional arithmetic operations on it, and stores the result back to the same global variable.
- `update_digit_counter`: Updates a multi-digit counter by adding values and handling overflow conditions, decrementing digits when they exceed specified limits while maintaining proper digit alignment.
- `wait_for_dma_and_setup`: Waits for a specific DMA status bit to clear, then configures DMA control registers to enable bitplane DMA operations.
- `process_event_conditions`: Evaluates multiple conditional checks against global variables and structure values, calling other routines based on the results of these comparisons to update event data and position parameters.
- `update_event_position`: Updates position coordinates based on event data and boundary checks, modifying values in a structure pointed to by a1 and handling specific conditions for different event states.
- `update_event_state_2`: Updates event-related data based on random selection and modifies event parameters according to specific conditions, then adjusts a counter accordingly.
- `update_event_state`: Updates a global variable based on random number generation and state checks, then modifies a table entry depending on conditional logic.
- `update_event_data_2_3`: Updates global variables with specific values based on event parameters and random selections, then adjusts counters and calls other routines to modify game state accordingly.
- `frag_clr_move_asl_bsr_andi_addq_add_dbr_2`: Short code fragment that clears d1, then copies 2 to d2, then shifts d1 left by 4, then calls another routine, then masks d0 with 3, then adds 2 to d0, then adds d0 to d1, then loops back while counting down d2, then copies d1 to a fixed memory address, then returns.
- `frag_btst_bne_beq_rts_2`: Short code fragment that tests bit 7 of a fixed memory address, then branches if the last result was not zero, then tests bit 7 of a fixed memory address, then branches if the last result was zero, then returns.
- `frag_btst_bne_rts_2`: Short code fragment that tests bit 7 of a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_movea_move_dbra_rts_2`: Short code fragment that copies an address to a0, then copies 63 to d0, then copies 65535 to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `frag_move_dbra_rts_2`: Short code fragment that copies 8191 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `copy_memory_block_2`: Copies memory blocks between two locations by moving 2000 words in a loop, then copies additional data from one location to another using a double loop structure.
- `frag_bsr_2`: Short code fragment that calls another routine (2 times in a row).
- `frag_move_bsr_subq_bne_rts_2`: Short code fragment that copies 63 to a fixed memory address, then copies a fixed memory address to d0, then calls another routine, then subtracts 1 from a fixed memory address, then branches if the last result was not zero, then returns.
- `swap_memory_blocks_2`: Swaps contents of two memory blocks by exchanging 192 longwords between locations, then performs additional operations including setting registers and calling another routine.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `setup_blitter_dma`: Configures blitter registers for memory copying operations using parameters from a table, then performs multiple blitter operations with incrementing source and destination addresses.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs one instruction (clr.l) and then falls through into the code that follows, with no return of its own.
- `setup_dma_transfer_3`: Sets up DMA transfer parameters by calculating addresses and values from input data, then configures multiple hardware registers including BLTCON0, BLTCON1, and BLTSIZE for a graphics operation.
- `frag_move_andi_asr_exg_adda_rts_2`: Short code fragment that copies a0 to d0, then masks d0 with 32767, then shifts d0 right by 2, then exchanges d0 and a0, then masks d0 with an address, then adds d0 to a0, then returns.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `frag_movem_moveq_move_beq_bsr_bra_rts_3`: Short code fragment that saves registers on the stack, then loads 0 into d0, then copies the memory at a4 (advancing a4) to d0, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then restores saved registers from the stack, then returns.
- `process_character_input_2`: Processes ASCII character input by converting letters to offsets, handling special control characters, and updating various global variables based on the input value.
- `initialize_event_data`: Sets specific values in global variables, copies bytes from a source pointer to a global variable, and processes a sequence of bytes until a null terminator is encountered.
- `convert_ascii_character`: Converts ASCII character input into a value by subtracting 65, then performs bit shifts and additions to calculate an address offset.
- `frag_movem_moveq_move_beq_bsr_bra_rts_4`: Short code fragment that saves registers on the stack, then loads 0 into d0, then copies the memory at a4 (advancing a4) to d0, then branches if the last result was zero, then calls another routine, then jumps to another point in the code, then restores saved registers from the stack, then returns.
- `handle_numeric_input`: Processes numeric and control character input by validating ranges, adjusting values, and updating global variables with calculated addresses and counters.
- `write_dma_config`: Writes configuration values to DMA control registers and enables DMA channels by setting specific bits in interrupt and DMA control registers.
- `frag_moveq_jsr_lea_bra_2`: Short code fragment that loads 0 into d0, then calls another routine, then loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_movea_bra_3`: Short code fragment that copies an address to a0, then jumps to another point in the code, then copies an address to a0, then jumps to another point in the code.
- `frag_lea_bra_2`: Short code fragment that loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_movea_bra_4`: Short code fragment that copies an address to a0, then jumps to another point in the code.
- `copy_color_data_2`: Copies color data between memory locations using a loop that decrements a counter, with different behavior based on the value of a global variable.
- `generate_random_number_2`: Generates a pseudo-random number using multiplication and bit manipulation operations, storing the result in a global variable and returning part of it.
- `read_keyboard_byte`: Reads a keyboard input byte, processes it through bit rotation and inversion, uses the result as an index into a lookup table, and returns the indexed value.
- `extract_nibble`: Extracts the lower four bits from a value in a register and returns the result as a word.
- `setup_display_dma_2`: Configures display and DMA control registers including screen dimensions, bitplane settings, and transfer modes, then enables DMA for video operations.
- `setup_display_dma`: Initializes display DMA settings by copying data from a structure, setting up hardware registers, and performing multiple memory writes in a loop with decrementing counter.
- `halt_in_infinite_loop_2`: Enters an unconditional infinite loop with no operations, so execution never returns from this point.
- `frag_cmpi_blt_clr_rts_bra_2`: Short code fragment that compares a fixed memory address with 2, then branches if the comparison was less, then clears a fixed memory address, then returns, then jumps to another point in the code.
- `initialize_dma_system`: Initializes DMA system by configuring various hardware registers, setting up memory addresses, and entering an infinite loop after completing the initialization sequence.
- `initialize_joystick_input`: Initializes joystick input handling by setting a global variable to 1 and configuring the DMA control register with value 32896, then returns immediately.
- `update_cia_flags`: Tests bits in a global CIA register and updates a working register based on the results, then writes to another global register before returning.
- `clear_memory_block_2`: Clears a large memory block by writing zeros to it using a loop with a decrementing counter.
- `frag_move_bsr_dbra_rts_2`: Short code fragment that copies d0 to the stack, then calls another routine, then copies the top of the stack to d0, then loops back while counting down d0, then returns.
- `process_keyboard_state`: Checks the state of keyboard input bits and processes them through conditional branches.
- `frag_jsr_move_rts_3`: Short code fragment that calls another routine, then copies 33792 to a fixed memory address, then returns.
- `frag_jsr_move_rts_4`: Short code fragment that calls another routine, then copies 33792 to a fixed memory address, then returns.
- `setup_dma_transfer_2`: Configures DMA control registers and waits for a specific DMA status bit before setting another register value, then returns.

### ts#unpacked (raw code file, load address unknown)

- `initialize_game_state`: Initializes multiple global variables and memory locations with hardcoded values, sets up pointers into tables based on a counter, and performs several memory clear operations before proceeding to other initialization routines.
- `frag_move_beq_addq_bra_rts`: Short code fragment that copies the memory at a1 (advancing a1) to d2, then branches if the last result was zero, then copies d2 to the memory at a0 (advancing a0), then adds 1 to d1, then jumps to another point in the code, then returns.
- `format_number_string`: Converts a number into a formatted string representation with padding, storing individual digits in memory.
- `frag_jsr_move_btst_bne_dbra_rts`: Short code fragment that calls another routine, then copies 24 to d1, then calls another routine, then tests bit 7 of d0, then branches if the last result was not zero, then calls another routine, then loops back while counting down d1, then returns.
- `process_input_buffer`: Reads bytes from an input buffer and processes them based on specific conditions, updating various global variables during the process.
- `setup_dma_parameters`: Prepares DMA transfer parameters by loading addresses and values, then jumps to a DMA setup routine without returning.
- `execute_dma_operation`: Executes a DMA operation by calling another routine, then performs bit masking on a global variable before using it as an index to jump to a function pointer in a table.
- `frag_move_bsr_movea_rts`: Short code fragment that copies a0 to the stack, then calls another routine, then copies the top of the stack to a0, then returns.
- `process_olympic_input`: Reads a byte from an input buffer and processes it to update game state variables, including handling boundary conditions and performing arithmetic operations on input values.
- `frag_move_jmp`: Short code fragment that copies 32 to d0, then jumps to another point in the code.
- `poll_input_event`: Reads input events in a loop until a specific bit is set, then reads input events again until that bit is cleared, and returns.
- `handle_input_event_2`: Handles input events by calling system routines, masking a value with 143, and storing it in memory.
