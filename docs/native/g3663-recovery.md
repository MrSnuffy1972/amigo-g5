# Game 3663 (flaschbier (1988)(level four)[cr sky devils]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03663). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (915680 top-level bytes in 18 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 96 of 96 named (0 author symbols), 96 of 96 described (23 by rule, 66 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1764 of 1764 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 381312 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3635 of 3635 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2868 |
| code | 15531 |
| data | 443384 |
| padding | 3337 |
| unused | 450560 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Flaschbier (1988)(Level Four)[cr Sky Devils].adf:run | 2 | 117 | 2 |
| run | 2 | 479 | 13 |
| Type | 2 | 117 | 2 |
| cls | 3 | 19 | 1 |
| Flaschbier (1988)(Level Four)[cr Sky Devils].adf:Werner | 2 | 83 | 3 |
| Flaschbier (1988)(Level Four)[cr Sky Devils].adf:e | 2 | 117 | 2 |
| Flaschbier (1988)(Level Four)[cr Sky Devils].adf:starter | 2 | 20 | 1 |
| Flaschbier (1988)(Level Four)[cr Sky Devils].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2375 | 62 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 124 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 99 | 3 |
| Flaschbier (1988)(Level Four)[cr Sky Devils].adf:Werner#unpacked | 1 | 49 | 1 |
| Flaschbier (1988)(Level Four)[cr Sky Devils].adf:HoeHoe (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address (play-rs-m1500): 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1468 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 109 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 96 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- flaschbier (1988)(level four)[cr sky devils]/Flaschbier (1988)(Level Four)[cr Sky Devils].adf:Werner: crunched executable unpacked with ancient scan (ByteKiller 1.3 (xfd), 3648 -> 16384 bytes), 3 of 3 windows confirmed in reference RAM, loaded at an address: recovered as u12
- flaschbier (1988)(level four)[cr sky devils]/Flaschbier (1988)(Level Four)[cr Sky Devils].adf:HoeHoe: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate+address, 1 jump target
- flaschbier (1988)(level four)[cr sky devils]/Flaschbier (1988)(Level Four)[cr Sky Devils].adf:CL: raw file found loaded verbatim in runtime segment an address (play-rs-m1500) at an address; classified through the recovered segment
- flaschbier (1988)(level four)[cr sky devils]/Flaschbier (1988)(Level Four)[cr Sky Devils].adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- flaschbier (1988)(level four)[cr sky devils]/Flaschbier (1988)(Level Four)[cr Sky Devils].adf: blocks 2-3 (bytes 1024-2048) outside the filesystem hold data, not code (opcode density 0.021, entropy 5.11)
- flaschbier (1988)(level four)[cr sky devils]/Flaschbier (1988)(Level Four)[cr Sky Devils].adf: blocks 6-11 (bytes 3072-6144) outside the filesystem hold data, not code (opcode density 0.012, entropy 4.31)
- flaschbier (1988)(level four)[cr sky devils]/Flaschbier (1988)(Level Four)[cr Sky Devils].adf: blocks 24-57 (bytes 12288-29696) outside the filesystem hold data, not code (opcode density 0.003, entropy 2.59)
- flaschbier (1988)(level four)[cr sky devils]/Flaschbier (1988)(Level Four)[cr Sky Devils].adf: blocks 60-76 (bytes 30720-39424) outside the filesystem hold data, not code (opcode density 0.003, entropy 0.99)
- 2 never-loaded region(s), 381312 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_input_handling` (runtime segment an address (play-rs-m1500)): Sets up input device registers and performs initialization checks before proceeding with input processing, returning early if a condition is met. Kind: disk hardware check. Pass path: The routine initializes input handling by manipulating CIAB_PRB and CIAA_PRA, checking for specific hardware states (e.g., bit 3 of CIAA_PRA), and may involve a disk drive or hardware-based verification. If the check passes (i.e., the expec.
- `handle_input_and_permit` (runtime segment an address (play-rs-m1500)): Saves registers, calls a system function, manipulates hardware registers related to input devices, and performs timed input checks using a loop. Kind: disk hardware check. Pass path: The routine reads from CIA registers and performs bit manipulations on CIAB_PRB and CIAA_PRA, suggesting a check against hardware state (e.g., disk drive status or motor control). If the check passes (i.e., expected bits are set), it procee.
- `initialize_screen_buffer` (runtime segment an address (play-rs-m1500)): Initializes a screen buffer by clearing specific bits in a global variable, then calls graphics routines to set up the display and updates a buffer pointer with the result. Kind: disk hardware check. Pass path: The routine initializes screen buffer and performs a loop with CIA register reads (CIAA_SDR), suggesting a disk drive activity or hardware state check. If the check passes (e.g., no timeout or expected hardware state), it continues to initi.
- `clear_input_flags` (runtime segment an address (play-rs-m1500)): Clears specific bits in a memory location and calls system functions to manage input device access, then performs timed input checks using a hardware register. Kind: disk hardware check. Pass path: The routine clears input flags via CIAB_PRB, then reads CIAA_SDR in a loop, suggesting a sync or track validation check. If the check passes (no error), it continues execution and returns to the caller..

## Functions


### Flaschbier (1988)(Level Four)[cr Sky Devils].adf:run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### run

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_2`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `initialize_entity_data`: Writes multiple values to a structure at a1, including results from calls to another routine with different parameters, and performs bit manipulation and table lookups based on values from the structure.
- `update_entity_timer_2`: Reads a byte from a table using a value from a structure as index, stores it in the structure, then updates timer-related fields in the structure based on calculated offsets and conditional logic.
- `update_entity_timer`: Updates a timer value in a data structure and performs memory transfers based on calculated offsets.
- `update_entity_data`: Writes a byte value to a memory location calculated from register values, then performs multiple memory transfers and clears specific data fields based on conditional checks.
- `update_entity_state`: Updates entity state variables including counters, flags, and array indices based on conditional checks and arithmetic operations, then performs a final branch based on a comparison result.
- `frag_moveq_add_bra`: Short code fragment that loads 1 into d1, then adds the memory at an offset from a1 to d1, then jumps to another point in the code.
- `frag_tst_bne_bra`: Short code fragment that tests the memory at an offset from a1, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_move_and_tst_beq_ori_moveq_cmp_blt`: Short code fragment that copies 128 to d1, then masks d1 with the memory at an offset from a1, then tests d1, then branches if the last result was zero, then merges an address into the memory at an offset from a1, then loads 133 into d1, then compares d1 with the memory at an offset from a1, then branches if the comparison was less, then jumps to another point in the code.
- `copy_and_initialize_fields`: Copies a value from one structure field to another, initializes multiple fields in a data structure through direct memory writes, and performs conditional operations based on flags before returning.
- `initialize_structure_fields`: Initializes multiple fields in a data structure through direct memory writes and conditional operations based on flags, then performs cleanup and setup for subsequent processing.
- `initialize_and_process`: Initializes multiple data fields in a structure through several library calls, then performs conditional operations based on flags and pointer values before jumping to another routine.

### Type

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value in a field, writes a header with that value, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_3`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### cls

- `initialize_library_resources`: Compares a byte at a0 to 63, then if not equal, uses a library base pointer to call several functions with specific arguments and offsets.

### Flaschbier (1988)(Level Four)[cr Sky Devils].adf:Werner

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Flaschbier (1988)(Level Four)[cr Sky Devils].adf:e

- `find_max_and_prepare_buffer_4`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_4`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### Flaschbier (1988)(Level Four)[cr Sky Devils].adf:starter

- `open_close_library`: Opens a library using a global variable as the name and closes it immediately afterward, with no other operations performed.

### Flaschbier (1988)(Level Four)[cr Sky Devils].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_verify_executable`: Opens and reads an executable file into memory, calculates a checksum of a specific memory range, and compares it with a known value to verify integrity.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics_buffer_2`: Initializes a graphics buffer by clearing it with zeros, then calls several rendering routines multiple times while incrementing counters and setting up display parameters.
- `update_sprite_data`: Processes sprite-related data by copying byte sequences between memory regions, updating global variables with values from specific memory locations, and performing conditional operations based on a counter decrement.
- `initialize_graphics_buffer`: Initializes a graphics buffer by clearing it with zeros, then calls several rendering routines multiple times while incrementing counters and advancing memory pointers.
- `swap_word_pairs`: Exchanges words between two memory regions, iterating 107 times with incrementing pointers.
- `render_frame_loop`: Calls a rendering routine twice while incrementing counters and advancing memory pointers.
- `clear_memory_region`: Clears a memory region by writing zeros to consecutive byte locations, first writing to an address and then to the previous address in a loop that repeats 128 times.
- `initialize_input_handling`: Sets up input device registers and performs initialization checks before proceeding with input processing, returning early if a condition is met.
- `handle_input_and_permit`: Saves registers, calls a system function, manipulates hardware registers related to input devices, and performs timed input checks using a loop.
- `frag_movea_lea_move_dbra_rts`: Short code fragment that copies a fixed memory address to a0, then loads the address of a fixed memory address into a1, then copies 279 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then copies a0 to a fixed memory address, then returns.
- `initialize_screen_buffer`: Initializes a screen buffer by clearing specific bits in a global variable, then calls graphics routines to set up the display and updates a buffer pointer with the result.
- `poll_keyboard_input`: Reads keyboard input from the CIAA_SDR register and performs a busy-wait loop with nested counters before returning.
- `clear_input_flags`: Clears specific bits in a memory location and calls system functions to manage input device access, then performs timed input checks using a hardware register.
- `frag_movea_move_cmpi_beq_addi_rts`: Short code fragment that copies an address to a0, then copies the memory at a0 to d0, then compares d0 with 57, then branches if the last result was zero, then adds 1 to a fixed memory address, then returns.
- `frag_move_cmpi_beq_addi_rts`: Short code fragment that copies 48 to a fixed memory address, then compares a fixed memory address with 57, then branches if the last result was zero, then adds 1 to a fixed memory address, then returns.
- `update_counter_and_return`: Updates a global counter by incrementing it and stores a specific byte value in another global variable before returning immediately.
- `clear_and_process_input`: Clears a memory block, reads from a hardware register, processes input values through conditional checks, and updates memory based on input data.
- `process_input_sequence_2`: Processes input characters through conditional branches, updating a global variable based on character comparisons and calling other routines.
- `process_input_sequence_a`: Processes input characters with conditional branches, setting a global variable to specific values based on character comparisons.
- `process_input_sequence_b`: Processes input characters with conditional branches, setting a global variable to specific values based on character comparisons and returns an incremented counter.
- `handle_input_sequence`: Processes input bytes from a global variable, updating a counter and calling other routines based on the input value.
- `process_input_sequence_d`: Processes input characters with conditional branches, setting a global variable to specific values based on character comparisons and returns a decremented counter.
- `input_state_machine`: Runs a state machine that increments counters until reaching specific limits, resetting the first counter when the second reaches its limit.
- `read_joystick_input`: Reads joystick and mouse input from hardware registers to construct a combined input value in a register.
- `update_input_state`: Updates input state based on joystick and mouse data, modifies global variables according to button presses, and sets up audio channel parameters.
- `calculate_and_store_division`: Performs unsigned division of a value by a divisor, stores the quotient in a global variable, and calculates a remainder through multiplication and subtraction before calling another routine with the quotient as parameter.
- `setup_blitter_operation`: Configures blitter registers with fixed values and calculates memory addresses based on input parameters for a graphics operation.
- `frag_movem_subi_move_andi_jsr_addi_dbra`: Short code fragment that saves registers on the stack, then subtracts 1 from d3, then copies the memory at a0 (advancing a0) to d2, then masks d2 with 255, then calls another routine, then adds 1 to d0, then loops back while counting down d3, then restores saved registers from the stack, then returns.
- `configure_blitter_with_conditions`: Configures blitter registers with conditional calculations based on input parameters, setting memory addresses and performing arithmetic operations before initiating a graphics operation.
- `copy_rendering_data`: Copies data from one memory area to another using a loop that processes 70 words of data, with each iteration moving a single word from source to destination.
- `frag_movea_move_dbra_rts`: Short code fragment that copies an address to a0, then copies an address to a1, then copies 70 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `initialize_rendering_parameters`: Initializes rendering parameters by calling functions to set up display data and performs arithmetic operations on values read from memory locations before returning.
- `frag_movea_move_dbra_rts_2`: Short code fragment that copies an address to a0, then copies an address to a1, then copies 70 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `frag_movea_move_dbra_rts_3`: Short code fragment that copies an address to a0, then copies an address to a1, then copies 70 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `process_input_sequence`: Processes a sequence of input bytes, updating counters and triggering actions based on specific byte values, while managing a global variable and calling other routines for further processing.
- `frag_movem_movea_adda_mulu_clr_move_rts`: Short code fragment that saves registers on the stack, then copies an address to a0, then adds d0 to a0, then multiplies d1 by 20 (unsigned), then adds d1 to a0, then clears d7, then copies the memory at a0 to d7, then restores saved registers from the stack, then returns.
- `frag_movem_andi_movea_mulu_add_adda_mov`: Short code fragment that saves registers on the stack, then masks d7 with 255, then copies an address to a0, then multiplies d1 by 20 (unsigned), then adds d0 to d1, then adds d1 to a0, then copies d7 to the memory at a0, then restores saved registers from the stack, then returns.
- `frag_movem_andi_movea_mulu_add_adda_mov_2`: Short code fragment that saves registers on the stack, then masks d7 with 255, then copies an address to a0, then multiplies d1 by 20 (unsigned), then adds d0 to d1, then adds d1 to a0, then copies d7 to the memory at a0, then restores saved registers from the stack, then returns.
- `frag_movem_andi_movea_mulu_add_adda_mov_3`: Short code fragment that saves registers on the stack, then masks d7 with 255, then copies an address to a0, then multiplies d1 by 20 (unsigned), then adds d0 to d1, then adds d1 to a0, then copies d7 to the memory at a0, then restores saved registers from the stack, then returns.
- `frag_movem_movea_adda_mulu_clr_move_rts_2`: Short code fragment that saves registers on the stack, then copies an address to a0, then adds d0 to a0, then multiplies d1 by 20 (unsigned), then adds d1 to a0, then clears d7, then copies the memory at a0 to d7, then restores saved registers from the stack, then returns.
- `frag_movem_movea_adda_mulu_clr_move_rts_3`: Short code fragment that saves registers on the stack, then copies an address to a0, then adds d0 to a0, then multiplies d1 by 20 (unsigned), then adds d1 to a0, then clears d7, then copies the memory at a0 to d7, then restores saved registers from the stack, then returns.
- `frag_movem_andi_movea_mulu_add_adda_mov_4`: Short code fragment that saves registers on the stack, then masks d7 with 255, then copies an address to a0, then multiplies d1 by 20 (unsigned), then adds d0 to d1, then adds d1 to a0, then copies d7 to the memory at a0, then restores saved registers from the stack, then returns.
- `process_character_table`: Iterates through a character table at a fixed address, comparing each byte against specific values and calling different routines based on matches while incrementing counters for rows and columns.
- `process_input_sequence_3`: Iterates through a sequence of input bytes starting at a global address, processing each byte based on its value and calling another routine with a parameter derived from the byte.
- `setup_blitter_dma`: Configures blitter registers for memory-to-memory transfer with specific source and destination addresses, sets up the blitter control words and moduli, and returns the input parameters unchanged.
- `setup_blitter_dma_2`: Configures blitter registers for a different memory-to-memory transfer with specific source and destination addresses, sets up the blitter control words and moduli, and returns the input parameters unchanged.
- `setup_blitter_dma_3`: Configures blitter registers for a memory-to-memory transfer with specific source and destination addresses, sets up the blitter control words and moduli, and performs cleanup operations including library calls.
- `configure_audio_channel_0`: Sets up audio channel 0 with specific sound parameters including length, period, volume, and enables the audio DMA channel.
- `configure_audio_channels_0_and_2`: Sets up audio channels 0 and 2 with specific sound parameters including length, period, volume, and enables the audio DMA channel.
- `configure_audio_channel_0_2`: Sets up audio channel 0 with specific sound parameters including length, period, volume, and enables the audio DMA channel.
- `configure_audio_channel_1`: Sets up audio channel 1 with specific sound parameters including length, period, volume, and enables the audio DMA channel.
- `configure_audio_channel_1_2`: Sets up audio channel 1 with specific sound parameters including length, period, volume, and enables the audio DMA channel.
- `init_audio_channel`: Configures audio channel 1 registers with specific values and enables DMA for audio output.
- `configure_audio_channel_3`: Sets up audio channel 3 with specific sound parameters including length, period, volume, and enables the audio DMA channel.
- `initialize_graphics_data`: Writes word values to specific memory addresses in a pattern, copies data between memory regions using a loop that processes 16 words, and stores several long values into a global variable table.
- `update_animation_state`: Processes data from two memory locations by adding corresponding bytes, updates a counter when certain conditions are met, and modifies values in a global variable table based on byte comparisons.
- `initialize_audio_volumes`: Initializes audio volume registers to zero for four sound channels and clears multiple global variables while processing a data buffer with looped comparisons and calculations.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `frag_bsr_jmp`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_audio_channels`: Manages audio channel configuration by incrementing a counter, checking conditions for each of four audio channels, and updating hardware registers based on those checks.
- `update_audio_configuration`: Calculates audio hardware register values from a table and updates them, modifies a value in memory based on conditional checks, and adjusts the CIAA_PRA register to control fire buttons and disk status.
- `compare_a6_against_global_variable`: Compares the address in a6 against three different global variable addresses in succession and returns immediately upon finding a match, otherwise it returns after all comparisons without changing any data.
- `update_dma_and_render_object`: Loads object data from memory tables, performs arithmetic operations to update rendering parameters, and writes updated values to DMA control registers while managing conditional rendering states through multiple branches.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics`: Initializes graphics hardware by opening a library, setting up memory pointers, and configuring display registers.
- `copy_string_data`: Copies byte pairs from a table to another location while clearing two bytes after each pair, repeating this process a specific number of times.
- `adjust_hunk_value`: Adjusts a global value based on comparisons with constants, either incrementing or decrementing depending on the comparison result.
- `update_joystick_input`: Reads joystick data from a hardware register, updates a global variable based on comparison with a table value, and manages counters for input processing.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_bit_shifts_2`: Processes bit shifting operations in a loop, extracting bits from a source location and storing them into a destination, while handling conditional branching based on comparison results with specific values.
- `rotate_and_store`: Extracts a byte from memory, stores it in a word-sized location, sets the processor status register, and performs a right rotate operation on a long integer value before returning it.
- `process_bit_shifts`: Processes bit shifting operations in a loop, calls another routine, and handles conditional branching based on comparison results with specific values.

### Flaschbier (1988)(Level Four)[cr Sky Devils].adf:Werner#unpacked

- `setup_display_and_input`: Opens a library, configures display memory and DMA settings, reads joystick input status, waits for a button release, and prepares system state for further operations.
