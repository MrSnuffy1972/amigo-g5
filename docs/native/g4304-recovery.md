# Game 4304 (luxor (1988)(software horizons)[v blizzard v1.0]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04304). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901248 top-level bytes in 32 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 123 of 123 named (0 author symbols), 123 of 123 described (77 by rule, 45 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2361 of 2361 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 77538 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3338 of 3338 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 265803 |
| code | 14544 |
| data | 131350 |
| padding | 7247 |
| unused | 482304 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Luxor (1988)(Software Horizons)[v Blizzard v1.0].adf:type | 2 | 117 | 2 |
| Luxor (1988)(Software Horizons)[v Blizzard v1.0].adf:lx | 12 | 3180 | 118 |
| Luxor (1988)(Software Horizons)[v Blizzard v1.0].adf:bootblock | 1 | 39 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Luxor (1988)(Software Horizons)[v Blizzard v1.0].adf:ENNEW.SPR (raw code file, load address unknown) | 1 | 0 | 0 |
| Luxor (1988)(Software Horizons)[v Blizzard v1.0].adf:LUX.SPR (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- luxor (1988)(software horizons)[v blizzard v1.0]/Luxor (1988)(Software Horizons)[v Blizzard v1.0].adf:ENNEW.SPR: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- luxor (1988)(software horizons)[v blizzard v1.0]/Luxor (1988)(Software Horizons)[v Blizzard v1.0].adf:LUX.SPR: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 77538 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Luxor (1988)(Software Horizons)[v Blizzard v1.0].adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### Luxor (1988)(Software Horizons)[v Blizzard v1.0].adf:lx

- `initialize_game_state`: Initializes global variables and memory pointers, sets up video registers, and configures display parameters.
- `initialize_game_state_2`: Initializes multiple global variables to zero, sets up audio parameters, and processes game object states based on conditional logic.
- `frag_cmpi_beq_subi_jsr_bclr_bra`: Short code fragment that compares a fixed memory address with 0, then branches if the last result was zero, then compares a fixed memory address with 1, then branches if the last result was zero, then subtracts 1 from a fixed memory address, then calls another routine, then clears bit 8 of d0, then clears bit 0 of d0, then jumps to another point in the code, then calls another routine.
- `update_game_state_2`: Processes input flags to update game state variables including position, animation frame, and mode settings, then calculates screen coordinates for rendering.
- `update_game_objects`: Processes game object states and updates their properties based on conditional logic, including incrementing counters and modifying values in a global data structure.
- `initialize_audio_channel`: Initializes an audio channel by clearing a global counter, setting a status flag, and writing waveform parameters to audio register addresses.
- `validate_and_update_position`: Validates position coordinates against boundary limits and updates them if valid.
- `initialize_variables`: Initializes a series of word-sized values in a table starting at a specific address, then conditionally sets a global pointer to different addresses based on the value of another global variable.
- `clear_memory_blocks`: Clears memory locations in six different address pointers by writing zero values to them, iterating through a fixed count of iterations.
- `frag_move_rts`: Short code fragment that copies 160 to a fixed memory address, then copies 120 to a fixed memory address, then copies 0 to a fixed memory address, then copies 1 to a fixed memory address, then returns.
- `frag_move_jmp`: Short code fragment that copies 0 to a fixed memory address (7 times in a row), then copies 1 to a fixed memory address, then copies 63 to a fixed memory address, then jumps to another point in the code.
- `frag_move_movea_addi_cmpi_bne_rts`: Short code fragment that copies 0 to d0, then copies 9260 to a0, then copies 0 to the memory at a0 (advancing a0), then adds 1 to d0, then compares d0 with 50, then branches if the last result was not zero, then returns.
- `reset_frame_counters`: Clears three global variables and returns immediately without performing any other operations.
- `frag_jsr_btst_beq_bne_rts`: Short code fragment that calls another routine, then tests bit 15 of d0, then branches if the last result was zero, then calls another routine, then tests bit 15 of d0, then branches if the last result was not zero, then returns.
- `process_pattern_data_2`: Processes pattern data by reading source values, applying boundary checks, calculating offsets, and calling a routine to handle pattern data operations.
- `process_pattern_data_3`: Processes pattern data by reading 16-bit values from a source pointer, applying bitwise operations including shifts, ORs, and ANDs with inverted values, and storing results into multiple memory locations.
- `process_pattern_data_4`: Processes pattern data by reading 16-bit values from a source pointer, applying bitwise operations including shifts, ORs, and ANDs with inverted masks, and writing results to memory locations.
- `update_bitmask_data_2`: Updates bitmask data by reading 16-bit values from a buffer and applying bitwise operations to four memory locations, clearing bits based on a shift amount.
- `process_bitmask_data`: Processes bitmask data by reading 16-bit values from a buffer and applying bitwise operations to four memory locations, clearing bits based on the input values.
- `copy_memory_regions`: Copies data between memory regions using multiple loop iterations, with varying offsets and counts, performing word-sized copies at specific intervals and handling conditional copy operations based on counter values.
- `update_pattern_state`: Updates pattern state by comparing word-sized values at specific memory locations and conditionally modifying other memory values, then calls several routines before returning.
- `update_graphics_mask`: Updates graphical masks in memory by performing bitwise OR operations with generated bitmasks based on input parameters, iterating through a loop of 16 iterations.
- `update_collision_data`: Processes collision detection data by comparing coordinates from two tables, updating a global variable with collision results, and incrementing a counter in another global variable.
- `update_game_state_3`: Updates game state variables based on input parameters and performs conditional operations including audio configuration, counter adjustments, and status flag modifications.
- `update_collision_state`: Processes collision detection logic by reading position values from a table, performing arithmetic comparisons, and updating a global flag based on distance thresholds.
- `update_game_state`: Updates internal game state variables based on input conditions and manages a data structure with multiple entries.
- `update_game_entities`: Iterates through game entity data, updates position and state variables based on conditional logic, and manages entity lifecycle by modifying counters and pointers in a global table.
- `frag_move_nop_dbra_rts`: Short code fragment that copies 7500 to d0, then pauses for one no-operation step (2 times in a row), then loops back while counting down d0, then returns.
- `frag_nop_dbra_rts`: Short code fragment that pauses for one no-operation step (2 times in a row), then loops back while counting down d0, then returns.
- `frag_move_rts_2`: Short code fragment that copies 0 to a fixed memory address, then returns.
- `frag_move_bra_2`: Short code fragment that copies 35000 to d0, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies 13000 to d0, then jumps to another point in the code.
- `perform_bcd_addition`: Performs multiple BCD (Binary Coded Decimal) addition operations on values from memory locations pointed to by a0 and a2, updating the destination values through successive addition and adjustment steps.
- `decode_compressed_data`: Decompresses run-length encoded data from a source pointer to a destination, handling both literal and repeated byte sequences.
- `apply_shifted_mask`: Processes data from a source table through a shift amount to clear and set bits in a destination table across multiple memory regions.
- `update_multiple_bitplanes`: Updates multiple memory regions with shifted bit patterns derived from source data, clearing and setting bits in a complex memory layout.
- `process_pattern_data`: Processes pattern data by reading 16-bit values from a source pointer, applying bitwise operations including shifts, ORs, and ANDs with inverted masks, and storing results into four memory locations offset from a destination pointer.
- `update_bitmask_data`: Processes a series of 16-bit values from a buffer and applies bitwise operations to four memory locations, clearing bits based on a shift amount and setting bits based on the buffer data.
- `frag_move_lea_rts`: Short code fragment that copies 15 to a fixed memory address, then loads the address of a fixed memory address into a0, then copies an address to the memory at a0 (advancing a0), then copies an address to the memory at a0, then copies 1920 to a fixed memory address, then copies 58 to a fixed memory address, then returns.
- `handle_interrupt_flags`: Reads interrupt request flags from a hardware register, processes individual bits to update counters in memory locations, and writes modified values back to DMA control and interrupt enable registers.
- `configure_audio_parameters`: Calculates audio register addresses based on input parameters, writes waveform data to audio registers, updates DMA control settings, and configures interrupt enable flags for specific audio channels.
- `frag_lea_moveq_tst_ble_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 3 into d0, then tests the memory at a0 (advancing a0), then branches if the comparison was less or equal, then loops back while counting down d0, then returns.
- `initialize_video_registers`: Initializes various video hardware registers with specific values, sets up display parameters, and configures memory addresses for graphics data.
- `frag_lea_jmp`: Short code fragment that loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `update_color_table`: Updates a color table entry in hardware register COLOR00 using values from a data table, with conditional execution based on global variables and loop control.
- `frag_move_andi_lsr_eor_btst_bra`: Short code fragment that copies a fixed memory address to d0, then masks d0 with 771, then copies d0 to the stack, then shifts d0 right by 1, then masks d0 with 257, then exclusive-ors d0 into the top of the stack, then copies a fixed memory address to d0, then tests bit 6 of d0, then jumps to another point in the code.
- `read_joystick_input`: Reads joystick input from hardware registers, processes the data through bitwise operations, and sets specific bits in a global variable based on button states and joystick movement.
- `frag_lea_moveq_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a1, then loads 15 into d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `frag_move_rts_3`: Short code fragment that copies 0 to a fixed memory address, then returns.
- `frag_move_rts_4`: Short code fragment that copies 2 to a fixed memory address, then returns.
- `frag_move_movea_dbra_rts`: Short code fragment that copies 7999 to d0, then copies a fixed memory address to a2, then copies 0 to the memory at a2 (advancing a2), then loops back while counting down d0, then returns.
- `frag_move_lea_andi_eori_rts`: Short code fragment that copies a fixed memory address to d0, then copies a fixed memory address to a fixed memory address, then copies d0 to a fixed memory address, then loads the address of a fixed memory address into a1, then masks the memory at a1 with 1, then exclusive-ors 1 into the memory at a1, then returns.
- `configure_interrupts_and_copy`: Configures interrupt enable registers with specific values, copies data from memory locations to a buffer, and updates several bytes in a global structure with values from another memory location.
- `frag_move_dbra_rts`: Short code fragment that copies 1999 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then copies the memory at an offset from a0 to the memory at an offset from a1 (3 times in a row), then loops back while counting down d0, then returns.
- `frag_jsr_clr_move_rts`: Short code fragment that calls another routine, then clears a fixed memory address, then copies 15 to a fixed memory address, then returns.
- `decrement_and_dispatch`: Decrements a global variable and dispatches to a function pointer table based on the value, handling different audio channel configurations and initializing audio registers.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_move_bra_4`: Short code fragment that copies a1 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bsr_bra`: Short code fragment that copies 750 to a fixed memory address, then copies 16 to a fixed memory address, then calls another routine, then copies d1 to a fixed memory address, then copies 33281 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bsr_bra_2`: Short code fragment that copies 801 to a fixed memory address, then calls another routine, then copies d1 to a fixed memory address, then copies 33281 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bra_5`: Short code fragment that copies the memory at a1 (advancing a1) to d1, then jumps to another point in the code.
- `frag_bsr_move_bra`: Short code fragment that calls another routine, then copies d1 to a fixed memory address, then copies 33281 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bra_6`: Short code fragment that copies the memory at a1 (advancing a1) to d1, then jumps to another point in the code.
- `initialize_audio_system`: Configures audio channel registers for four sound channels by writing specific values to DMA control, interrupt enable, and audio register a.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `frag_bra_11`: Short code fragment that jumps to another point in the code.
- `frag_bra_12`: Short code fragment that jumps to another point in the code.
- `frag_bra_13`: Short code fragment that jumps to another point in the code.
- `frag_bra_14`: Short code fragment that jumps to another point in the code.
- `frag_bra_15`: Short code fragment that jumps to another point in the code.
- `frag_bra_16`: Short code fragment that jumps to another point in the code.
- `frag_bra_17`: Short code fragment that jumps to another point in the code.
- `frag_bra_18`: Short code fragment that jumps to another point in the code.
- `frag_move_bra_7`: Short code fragment that copies a1 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bsr_bra_3`: Short code fragment that copies 750 to a fixed memory address, then copies 16 to a fixed memory address, then calls another routine, then copies d1 to a fixed memory address, then copies 33282 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bsr_bra_4`: Short code fragment that copies 801 to a fixed memory address, then calls another routine, then copies d1 to a fixed memory address, then copies 33282 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bra_8`: Short code fragment that copies the memory at a1 (advancing a1) to d1, then jumps to another point in the code.
- `frag_bsr_move_bra_2`: Short code fragment that calls another routine, then copies d1 to a fixed memory address, then copies 33282 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bra_9`: Short code fragment that copies the memory at a1 (advancing a1) to d1, then jumps to another point in the code.
- `initialize_audio`: Configures audio channel registers for four sound channels by writing specific values to DMA control, interrupt enable, and audio register a, then returns.
- `frag_bra_19`: Short code fragment that jumps to another point in the code.
- `frag_bra_20`: Short code fragment that jumps to another point in the code.
- `frag_bra_21`: Short code fragment that jumps to another point in the code.
- `frag_bra_22`: Short code fragment that jumps to another point in the code.
- `frag_bra_23`: Short code fragment that jumps to another point in the code.
- `frag_bra_24`: Short code fragment that jumps to another point in the code.
- `frag_bra_25`: Short code fragment that jumps to another point in the code.
- `frag_bra_26`: Short code fragment that jumps to another point in the code.
- `frag_bra_27`: Short code fragment that jumps to another point in the code.
- `frag_move_bra_10`: Short code fragment that copies a1 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bsr_bra_5`: Short code fragment that copies 750 to a fixed memory address, then copies 16 to a fixed memory address, then calls another routine, then copies d1 to a fixed memory address, then copies 33284 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bsr_bra_6`: Short code fragment that copies 801 to a fixed memory address, then calls another routine, then copies d1 to a fixed memory address, then copies 33284 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bra_11`: Short code fragment that copies the memory at a1 (advancing a1) to d1, then jumps to another point in the code.
- `frag_bsr_move_bra_3`: Short code fragment that calls another routine, then copies d1 to a fixed memory address, then copies 33284 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bra_12`: Short code fragment that copies the memory at a1 (advancing a1) to d1, then jumps to another point in the code.
- `frag_move_bra_rts`: Short code fragment that copies 4 to a fixed memory address, then jumps to another point in the code, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (btst.l) and then falls through into the code that follows, with no return of its own.
- `frag_moveq_move_add_lea_rts`: Short code fragment that loads 0 into d1, then copies the memory at a1 (advancing a1) to d1, then adds a fixed memory address to d1, then adds d1 to d1, then loads the address of a fixed memory address into a2, then copies an indexed entry from a2 to d1, then returns.
- `initialize_audio_channels`: Configures audio channel registers for four sound channels by writing specific values to DMA control, interrupt enable, and audio register addresses, then sets up volume and length parameters for each channel.
- `initialize_game_resources`: Initializes game resource management by opening a library, setting up global variables, and performing multiple data copy operations based on file references.
- `initialize_level_resources`: Initializes level-specific resources by opening and reading data files based on a global variable's value, then copies decompressed data to a destination buffer.
- `load_level_data`: Opens the level3.pi1 file, reads its contents into a buffer, and processes the data through multiple decompression and memory copy operations before closing the file.
- `frag_bchg_dbra_rts_jmp`: Short code fragment that flips bit 7 of the memory at a0 (advancing a0), then loops back while counting down d0, then returns, then jumps to another point in the code.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address, then copies a fixed memory address to a fixed memory address, then copies 28 to a fixed memory address, then returns.
- `process_keyboard_input`: Reads from a hardware interrupt control register, sets a specific bit in the register, performs bitwise operations on data from keyboard registers, and updates global variables based on the processed values.
- `frag_move_clr_bclr_bne_rts`: Short code fragment that copies a fixed memory address to d0, then clears a fixed memory address, then clears bit 7 of d0, then branches if the last result was not zero, then clears d0, then returns.
- `clear_memory_and_update_pointers`: Clears a block of memory using a loop and updates a global variable with a pointer.
- `frag_move_bra_13`: Short code fragment that copies a fixed memory address to a fixed memory address, then jumps to another point in the code.
- `execute_and_copy_data`: Executes a series of function pointers from a table and then copies a block of data to a global variable.
- `populate_callback_table`: Iterates through a table of pointers and writes a value from a register into the memory location pointed to by the current entry.

### Luxor (1988)(Software Horizons)[v Blizzard v1.0].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
