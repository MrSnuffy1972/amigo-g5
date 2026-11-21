# Game 2855 (beat it (1988)(softgang)[cr gsc]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02855). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901360 top-level bytes in 27 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 130 of 130 named (0 author symbols), 130 of 130 described (41 by rule, 70 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1403 of 1404 (99.93%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3461 of 3462 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 395079 |
| code | 12788 |
| data | 146988 |
| padding | 5001 |
| unused | 341504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Beat It (1988)(Softgang)[cr GSC].adf:install | 3 | 406 | 7 |
| install | 3 | 406 | 7 |
| Beat It (1988)(Softgang)[cr GSC].adf:Type | 2 | 117 | 2 |
| Port-Handler | 1 | 306 | 1 |
| Disk-Validator | 1 | 386 | 1 |
| Beat It (1988)(Softgang)[cr GSC].adf:GSCINTRO | 2 | 142 | 8 |
| Beat It (1988)(Softgang)[cr GSC].adf:game | 10 | 1567 | 98 |
| Beat It (1988)(Softgang)[cr GSC].adf:bootblock | 1 | 118 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 1 executed PCs start inside another executed instruction (overlapping-instruction idiom): decoded and counted as instruction starts

## Cracktros (kept, not protection)

- Beat It (1988)(Softgang)[cr GSC].adf:GSCINTRO

## Copy protection found

- none found

## Functions


### Beat It (1988)(Softgang)[cr GSC].adf:install

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `frag_lea_jsr_tst_beq_movea_moveq_rts`: Short code fragment that loads the address of a location in the code into a1, then calls another routine, then tests d0, then branches if the last result was zero, then copies d0 to a0, then copies the memory at an offset from a0 to a0, then loads 0 into d0, then returns.
- `frag_moveq_bra`: Short code fragment that loads 255 into d0, then jumps to another point in the code.
- `initialize_structure`: Initializes multiple fields in a data structure starting at address in a1, sets up pointers and counters, and performs a series of conditional operations based on computed values and table lookups.
- `update_and_execute_2`: Updates a counter in a structure and executes a series of function calls through a table until the counter reaches a limit, then performs additional memory operations.
- `update_and_execute`: Updates a counter at offset 528 in a structure and executes a series of function calls through a table until the counter reaches a limit, then writes a negated value to a memory location and continues executing more function calls.

### install

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_2`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `frag_lea_jsr_tst_beq_movea_moveq_rts_2`: Short code fragment that loads the address of a location in the code into a1, then calls another routine, then tests d0, then branches if the last result was zero, then copies d0 to a0, then copies the memory at an offset from a0 to a0, then loads 0 into d0, then returns.
- `frag_moveq_bra_2`: Short code fragment that loads 255 into d0, then jumps to another point in the code.
- `initialize_structure_2`: Initializes multiple fields in a data structure starting at address in a1, including counters, pointers, and status flags.
- `update_and_execute_3`: Updates a counter in a structure and executes a series of function calls through a table until the counter reaches a limit, then performs additional memory operations.
- `update_and_call`: Updates a counter at offset 528 relative to a1, then performs a series of function calls using a5 as the function pointer.

### Beat It (1988)(Softgang)[cr GSC].adf:Type

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value in a field, writes a header with that value, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_3`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### Port-Handler

- `validate_hunk_headers`: Compares loaded hunk type codes with expected values and initializes various data structure offsets based on the hunk type.

### Disk-Validator

- `initialize_game_state`: Sets up initial game parameters including screen buffer pointers, counter values, and memory mappings.

### Beat It (1988)(Softgang)[cr GSC].adf:GSCINTRO

- `initialize_audio_system`: Initializes audio hardware registers with specific values for sound channels, sets up DMA control, and configures interrupt handling for audio playback.
- `clear_and_copy_blocks`: Clears a 14336 word block at address a fixed address, then copies data from two different source addresses to destinations at a fixed address and a fixed address with lengths 2280 and 500 words respectively.
- `frag_lea_move`: Short code fragment that loads the address of a fixed memory address into a0, then copies a0 to a fixed memory address, then loads the address of a fixed memory address into a0, then copies a0 to a fixed memory address, then copies 646 to a fixed memory address, then copies 32784 to a fixed memory address.
- `setup_audio_dma`: Checks joystick and mouse input conditions, then configures audio volume registers to zero, sets DMA control register to 3, and stores values from memory locations to specific addresses.
- `frag_move_jsr_dbra_movea_jmp`: Short code fragment that copies d0 to the stack, then copies a fixed memory address to d0, then calls another routine, then loops back while counting down d0, then copies the top of the stack to d0, then copies a fixed memory address to a1, then jumps to the address in a1.
- `update_animation_pointers_2`: Increments three word pointers and reads word values from a table at an offset, updating corresponding global variables when the read value meets specific conditions.
- `update_animation_pointers`: Increments three word pointers and reads word values from a table at an offset, updating corresponding global variables when the read value is zero, otherwise resetting the pointer to zero.
- `rotate_and_shift_data`: Performs bit rotation operations on data in two memory regions, shifting bits left through a series of words and updating registers with rotated values based on loop counters.

### Beat It (1988)(Softgang)[cr GSC].adf:game

- `initialize_system`: Writes the value ten to a memory location at offset 128 and then triggers a software interrupt exception.
- `setup_interrupt_handlers`: Saves current interrupt state, sets up two interrupt vector pointers in memory, restores interrupt state, and then calls multiple routines in sequence to configure display, DMA, and graphics settings.
- `audio_delay_and_status_check`: Calls another routine, performs a delay loop by counting down a global variable from an address to zero, then calls another routine and returns.
- `frag_move_bra_4`: Short code fragment that copies 512 to a fixed memory address (2 times in a row), then copies a fixed memory address to d1, then copies a fixed memory address to d3, then jumps to another point in the code.
- `delay_loop_countdown`: Counts down a global variable from an address to zero in a loop, then calls another routine, followed by counting down from an address to zero in another loop before returning.
- `handle_display_cycle`: Manages display color transitions by incrementing a counter and updating color register values when the counter reaches specific thresholds, while also handling interrupt flags and calling other routines.
- `configure_interrupts_dma`: Reads interrupt request status, enables DMA and interrupt channels based on specific bit conditions, and returns with updated register values for interrupt and DMA control.
- `setup_bitplane_registers`: Initializes multiple graphics hardware registers including bitplane control, display window settings, and copper list pointers with predefined values to configure the Amiga's display hardware.
- `frag_movem_moveq_lea_move_dbra_rts`: Short code fragment that saves registers on the stack, then loads 15 into d0, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a2, then copies the memory at a2 (advancing a2) to the memory at a1 (advancing a1), then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `frag_movem_moveq_lea_move_dbra_rts_2`: Short code fragment that saves registers on the stack, then loads 15 into d0, then loads the address of a fixed memory address into a1, then copies 0 to the memory at a1 (advancing a1), then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `frag_movem_lea_lsl_move_swap_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then shifts d2 left by 3, then copies d3 to an indexed entry from a0, then swaps the halves of d3, then copies d3 to an indexed entry from a0, then restores saved registers from the stack, then returns.
- `update_timer_values`: Stores a value in two memory locations, calls another routine four times with incrementing counters, and updates a third location with accumulated results from those calls.
- `frag_bsr_move_rts`: Short code fragment that calls another routine, then copies d3 to the stack, then copies 0 to d3, then calls another routine, then copies the top of the stack to d3, then returns.
- `frag_movem_lea_lsl_move_swap_rts_2`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then shifts d2 left by 3, then copies d3 to an indexed entry from a0, then swaps the halves of d3, then copies d3 to an indexed entry from a0, then restores saved registers from the stack, then returns.
- `initialize_dma_channels`: Calls multiple routines in sequence, sets a specific hardware register to enable DMA channels, and then performs several operations including setting and clearing values in memory locations.
- `check_audio_status`: Reads a hardware register, tests specific bits, and branches based on the results to determine whether to call another routine, otherwise it returns.
- `setup_audio_channel`: Configures audio channel parameters including length, period, and volume using data from registers and memory locations, then enables DMA and interrupts for audio playback.
- `frag_andi_lsl_move_and_beq_rts`: Short code fragment that masks d0 with 15, then shifts d0 left by 7, then copies a fixed memory address to d5, then masks d5 with d0, then branches if the last result was zero, then copies d0 to a fixed memory address, then returns.
- `configure_audio_parameters`: Calculates audio parameters based on input values, stores calculated values in audio channel registers, enables DMA and interrupts, and returns with updated register settings.
- `setup_audio_channel_2`: Configures audio channel registers with calculated values from input parameters and enables DMA and interrupt handling for the audio subsystem.
- `frag_move_rts`: Short code fragment that copies a fixed memory address to d7, then copies a fixed memory address to a fixed memory address (3 times in a row), then copies d7 to a fixed memory address, then returns.
- `process_data_table`: Copies and processes data from one memory location to another through a loop that iterates a fixed number of times while updating pointers.
- `convert_bit_pattern`: Processes a 32-bit input value by examining each bit position and constructing a new 16-bit value with bits set according to specific shifting and branching logic.
- `initialize_colors`: Writes fixed color values to specific hardware registers for color palette initialization and sets up several global variables with predefined byte values.
- `frag_moveq_move_bra`: Short code fragment that loads 0 into d2, then copies 2796 to d3, then jumps to another point in the code.
- `process_joystick_input`: Reads joystick data from hardware registers, applies conditional modifications based on button states, updates scroll position values, and clamps results to specific ranges.
- `frag_move_andi_beq_rts`: Short code fragment that copies a fixed memory address to d1, then masks d1 with 62464, then branches if the last result was zero, then returns.
- `frag_moveq_sub_beq_bmi_bra_move_rts`: Short code fragment that loads 0 into d0, then subtracts d3 from a fixed memory address, then branches if the last result was zero, then branches if the last result was negative, then loads 255 into d0, then jumps to another point in the code, then loads 1 into d0, then copies d3 to a fixed memory address, then returns.
- `frag_moveq_sub_beq_bmi_bra_move_rts_2`: Short code fragment that loads 0 into d0, then subtracts d3 from a fixed memory address, then branches if the last result was zero, then branches if the last result was negative, then loads 255 into d0, then jumps to another point in the code, then loads 1 into d0, then copies d3 to a fixed memory address, then returns.
- `frag_move_dbra_rts`: Short code fragment that copies d0 to the stack, then copies 6000 to d0, then loops back while counting down d0, then copies the top of the stack to d0, then returns.
- `handle_joystick_input`: Reads joystick button states from hardware registers, compares them with previous values, and calls other routines based on the comparison results to handle input changes.
- `frag_move_dbra_rts_2`: Short code fragment that copies d0 to the stack, then copies 1000 to d0, then loops back while counting down d0, then copies the top of the stack to d0, then returns.
- `initialize_colors_and_wait`: Sets specific color values in global variables, waits for button input with a countdown loop, and manages color state transitions based on input conditions.
- `copy_sprite_data`: Copies sprite data from one memory location to another, processing 22 iterations of word-sized transfers while applying a bitwise AND operation on every other word.
- `process_game_state`: Processes game state by checking various conditions, calling other routines for different actions, managing sprite coordinates, and handling input processing through multiple conditional branches and loops.
- `check_range_conditions`: Compares a value against multiple range limits and sets a flag to either 1 or 0 depending on whether the value falls within the specified bounds.
- `frag_bsr_bra`: Short code fragment that calls another routine (3 times in a row), then jumps to another point in the code.
- `frag_ori_move_subi_bpl_btst_beq_andi_rt`: Short code fragment that merges 1792 into the status register, then copies an address to a fixed memory address, then subtracts 1 from a fixed memory address, then branches if the last result was not negative, then tests bit 7 of a fixed memory address, then branches if the last result was zero, then masks the status register with 8959, then returns.
- `copy_and_update_coordinates`: Copies word-sized data from one memory location to another, adds 9 to both copied values, then calls another routine, updates a byte at an offset calculated from a register value, and stores the register value in a global variable.
- `normalize_coordinates`: Takes input word values, extracts high and low bytes, adds three to each, and stores the results in specific memory locations for coordinate normalization.
- `frag_movem_bsr_lea_move_rts`: Short code fragment that saves registers on the stack, then calls another routine, then loads the address of a fixed memory address into a1, then copies 1 to an indexed entry from a1, then restores saved registers from the stack, then returns.
- `load_and_mark_entry`: Loads two words from a table using a scaled index, stores them into another table, marks a byte in a global structure with value 1, then returns.
- `load_and_clear`: Loads two 16-bit values from a table using a register value as an index, stores them in global variables, calls another routine, and clears a byte in a global array.
- `wait_for_button_state_change`: Monitors joystick button states in a loop until a change is detected in either the fire button or mouse button status, then returns a specific value based on the final state.
- `frag_move_dbra_rts_3`: Short code fragment that copies d0 to the stack, then copies 1000 to d0, then loops back while counting down d0, then copies the top of the stack to d0, then returns.
- `copy_memory_block_1`: Copies 400 32-bit words from one memory location to another using a loop with pre-increment addressing and a decrementing counter.
- `copy_memory_block_2`: Copies 400 32-bit words from one memory location to another using a loop with pre-increment addressing and a decrementing counter, but in the reverse direction.
- `copy_small_block`: Copies 9 32-bit words and 1 byte from one memory location to another using sequential move instructions with pre-increment addressing.
- `store_byte_sequence`: Writes a 32-bit value from a global variable into a byte sequence at a fixed memory location, then sets the final byte to 255.
- `frag_move_rts_2`: Short code fragment that copies 1 to a fixed memory address, then copies 0 to a fixed memory address (2 times in a row), then returns.
- `clear_three_bytes`: Sets three specific bytes in global memory to zero and then returns.
- `frag_clr_move_lsr_bsr`: Short code fragment that clears a fixed memory address, then copies d6 to a fixed memory address, then shifts d6 right by 8, then copies d6 to a fixed memory address, then calls another routine.
- `frag_lea_move_bra`: Short code fragment that loads the address of a fixed memory address into a4, then copies 56 to a fixed memory address, then copies 256 to a fixed memory address, then jumps to another point in the code.
- `frag_lea_move_bra_2`: Short code fragment that loads the address of a fixed memory address into a4, then copies 104 to a fixed memory address, then copies 256 to a fixed memory address, then jumps to another point in the code.
- `fall_through_entry`: Entry point part-way through straight-line code: runs three instructions (lea.l, lea.l, andi.w) and then falls through into the code that follows, with no return of its own.
- `bcd_subtract_bytes`: Performs binary-coded decimal subtraction on two byte pairs, adjusting results and handling carry flags for each byte operation.
- `copy_and_process_data`: Copies word-sized data from one memory location to another in a loop, then performs bit manipulation and memory operations based on calculated offsets and values, updating global variables with processed coordinates and dimensions.
- `setup_blitter_parameters`: Initializes multiple hardware blitter registers with various values including sizes, modes, and memory offsets, then performs a loop of blitter operations.
- `frag_lea_btst_bne_rts`: Short code fragment that loads the address of a fixed memory address into a6, then tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `process_structure_entries`: Iterates through a data structure, writing specific values to memory locations and calling another routine for each entry until a negative value is encountered.
- `frag_lea_bsr_move_addq_add_bra`: Short code fragment that loads the address of a fixed memory address into a0, then calls another routine, then copies 0 to d1, then copies d1 to d3, then adds 2 to d3, then adds a fixed memory address to d3, then jumps to another point in the code.
- `frag_movem_move_beq_bsr_addq_bra_rts`: Short code fragment that saves registers on the stack, then copies the memory at a0 (advancing a0) to d3, then branches if the last result was zero, then calls another routine, then adds 1 to d0, then jumps to another point in the code, then restores saved registers from the stack, then returns.
- `update_bitmask_data`: Processes bit manipulation operations on a data structure using multiple lookup tables and performs bitwise set/reset operations based on input values.
- `frag_move`: Short code fragment that copies 280 to a fixed memory address, then copies 2 to d6, then copies the memory at a0 (advancing a0) to d5, then copies the memory at a0 (advancing a0) to d7.
- `frag_movem_bsr_move_tst_beq_bra`: Short code fragment that saves registers on the stack, then calls another routine, then copies d7 to d1, then calls another routine, then tests a fixed memory address, then branches if the last result was zero, then copies d5 to d3, then jumps to another point in the code.
- `manage_library_resources`: Manages library opening and closing operations, performs file I/O operations with specific buffer sizes, and returns concatenated register values.
- `parse_header_data`: Parses structured data from a memory buffer, validates its format, and extracts configuration parameters including size and flags.
- `frag_movem_bsr_rts`: Short code fragment that saves registers on the stack, then calls another routine (4 times in a row), then restores saved registers from the stack, then returns.
- `frag_move_lea_moveq_addi_dbra_rts`: Short code fragment that copies an address to d0, then loads the address of a fixed memory address into a1, then loads 3 into d1, then copies d0 to the memory at a1 (advancing a1), then adds 8000 to d0, then loops back while counting down d1, then returns.
- `convert_nibble_pairs`: Converts packed nibble data into word values by combining high and low nibbles from consecutive bytes in a source buffer.
- `update_compressed_data`: Processes run-length encoded data by reading control bytes and either copying or repeating sequences of bytes, updating a table with decompressed values.
- `decode_compressed_data`: Decompresses run-length encoded data by reading control bytes and either copying or repeating sequences of bytes.
- `load_and_process_score_data`: Initializes various game components including DMA channels, loads score file data, processes input handling, manages library resources, and configures audio parameters before enabling DMA channels for display.
- `load_score_file`: Opens a score file, reads data into memory with specific buffer sizes, and manages library resources for file operations.
- `call_old_open_library`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `frag_move_bra_5`: Short code fragment that copies 1006 to d2, then jumps to another point in the code.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `initialize_palette_and_audio`: Calls several initialization routines including color setup and audio configuration, checks a condition and sets a global variable if the condition is met, then calls multiple routines in sequence before looping back to execute some routines multiple times.
- `frag_lea_moveq_swap_move_bra`: Short code fragment that loads the address of a fixed memory address into a0, then loads 3 into d1, then loads 6 into d0, then swaps the halves of d0, then copies 10 to d0, then jumps to another point in the code.
- `frag_lea_moveq_swap_move_bra_2`: Short code fragment that loads the address of a fixed memory address into a0, then loads 3 into d1, then loads 6 into d0, then swaps the halves of d0, then copies 10 to d0, then jumps to another point in the code.
- `check_threshold_condition`: Compares a value against entries in a lookup table and sets processor flags based on whether the comparison result meets specific criteria.
- `store_and_process_data`: Writes a value to a memory location, then processes character data by applying bit manipulation and conditional character replacement.
- `frag_move_andi_addi_rts`: Short code fragment that copies d0 to d1, then masks d1 with 15, then adds 48 to d1, then copies d1 to the memory just below a0 (moving a0 down), then returns.
- `copy_memory_blocks`: Copies memory blocks between different locations using word-sized transfers and handles specific index-based copying operations.
- `copy_memory_block`: Copies a block of memory (10 words, from the count loaded into d3) from the address in a2 to the address in a3 one word at a time, using a dbra loop counter, and returns.
- `frag_moveq_move_lsl_rts`: Short code fragment that loads 0 into d0, then copies the memory at a0 (advancing a0) to d0, then shifts d0 left by 8, then copies the memory at a0 (advancing a0) to d0, then shifts d0 left by 8, then copies the memory at a0 (advancing a0) to d0, then copies d0 to a fixed memory address, then returns.
- `copy_and_increment`: Copies data from a table using pointers and increments values, then performs a loop of fixed iterations before returning.
- `update_string_buffer`: Initializes a string buffer with specific character values, processes character input through conditional checks, and updates memory locations based on input validation, including handling special characters and updating a counter.
- `decrement_and_store`: Decrements a value in a global variable and stores a constant byte at an offset calculated from the decremented value in another global variable.
- `validate_character_range_2`: Validates if character values fall within specific ranges, sets processor flags accordingly, and handles conditional execution based on validation results.
- `frag_move_cmp_bcs_bgt_andi_rts`: Short code fragment that copies the memory at a1 to d2, then compares d0 with d2, then branches if the carry flag is set, then copies the memory at an offset from a1 to d2, then compares d0 with d2, then branches if the comparison was greater, then masks the status register with 65534, then returns (2 times in a row).
- `validate_character_range`: Validates if a character value falls within specific ranges and sets processor flags accordingly to indicate valid or invalid characters.
- `wait_joystick_input`: Waits for specific joystick button states by polling hardware registers until both button 6 and button 7 are pressed, then returns.
- `copy_and_update_pixels`: Copies pixel data from source locations to destination addresses, updating counters and performing multiple iterations of data transfer operations.

### Beat It (1988)(Softgang)[cr GSC].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sets_global_pointer_returns_minus_one`: Stores a constant into a global variable, removes 4 bytes from the stack, restores registers from memory, and returns the value -1 in d0.
- `frag_move_ori_eori`: Short code fragment that copies d0 to d2, then merges 0 into d0, then exclusive-ors 0 into d0, then merges 0 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_lea_btst_bne_rts_2`: Short code fragment that loads the address of a fixed memory address into a6, then tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
