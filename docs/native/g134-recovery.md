# Game 134 (3D World Boxing): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00134). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (914192 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 181 of 181 named (0 author symbols), 181 of 181 described (42 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 434 of 434 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3158 of 3158 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10050 |
| code | 19942 |
| data | 883688 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| 3D World Boxing.adf:bootblock | 1 | 6 | 1 |
| logged disk load from 3D World Boxing.adf boot-block load | 1 | 97 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3055 | 178 |

## Cases handled

- 3D World Boxing/3D World Boxing.adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- 434 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- 3D World Boxing/3D World Boxing.adf: the boot block reads 5632 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- runtime segment an address: 434 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3D World Boxing/3D World Boxing.adf: 1 logged loads cover 5632 bytes of the image
- 3D World Boxing/3D World Boxing.adf: 24 track-loaded blocks found in recovered runtime segments and classified through them
- 3D World Boxing/3D World Boxing.adf: blocks 2-3 (bytes 1024-2048) outside the filesystem hold data, not code (opcode density 0.027, entropy 3.13)
- 3D World Boxing/3D World Boxing.adf: blocks 28-40 (bytes 14336-20992) outside the filesystem hold data, not code (opcode density 0.012, entropy 4.57)
- 3D World Boxing/3D World Boxing.adf: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.24)
- 1 packed track region(s), 868352 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `handle_input_validation_2` (runtime segment an address): Validates input data by performing bit operations on memory values and calling other routines to process the results. Kind: disk hardware check. Pass path: the branch taken when CIAB_PRB reading indicates valid disk presence/condition.
- `process_collision_data_2` (runtime segment an address): Processes collision-related data by calculating distances and coordinates, then updates memory locations with computed values. Kind: disk hardware check. Pass path: CIAB_PRB read value matches expected hardware signature.
- `configure_port_b_pins` (runtime segment an address): Modifies port B pins through memory-mapped I/O by setting and clearing specific bits in a sequence of operations. Kind: disk hardware check. Pass path: The routine modifies CIAB_PRB (Port B) to configure pins, likely checking for specific hardware states or responses. The pattern of setting and clearing bits suggests a track or disk integrity check where the correct pin states indicate suc.

## Functions


### 3D World Boxing.adf:bootblock

- `frag_move_jsr_jmp`: Short code fragment that copies 2 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies 5632 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then calls another routine, then jumps to another point in the code.

### logged disk load from 3D World Boxing.adf boot-block load

- `rotate_and_store`: Reads a long value from memory, extracts a byte, stores it in another location, rotates bits right, and returns the rotated value.
- `process_bits_and_copy`: Processes bits through a loop, performs bit rotation operations, and handles data copying with conditional branches based on comparison results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_clr_lea_move_bclr_jsr`: Short code fragment that clears the memory at an offset from a4, then loads the address of a location in the code into a0, then copies a0 to the memory at an offset from a4, then copies a0 to the memory at an offset from a6, then copies d0 to the memory at an offset from a6, then clears bit 7 of the memory at an offset from a4, then calls another routine.
- `execute_loop_with_trap_2`: Saves all general-purpose registers to the stack, performs a multiplication operation on a register, executes a loop based on the result, then restores registers and returns.
- `load_address_and_trap`: Loads an address from a program counter relative location, stores it in a global variable, and executes a system trap.
- `initialize_and_jump`: Loads a word value from a specific offset in a structure, sets bits in status bytes, and jumps to an address loaded from another location.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a2, then copies a1 to the memory at an offset from a4, then copies a2 to the memory at an offset from a4, then copies a1 to the memory at an offset from a2, then copies a2 to the memory at an offset from a1, then returns.
- `initialize_boxing_game`: Initializes boxing game state by clearing counters, setting up pointers, calling update routines, and manipulating hardware registers for input handling.
- `frag_andi_rts`: Short code fragment that masks the memory at an offset from a0 with 191, then returns.
- `initialize_rendering_data_4`: Sets up rendering parameters including dimensions, offsets, and derived values for a structure pointed to by a4.
- `frag_lea_move_lsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies d0 to the memory at an offset from a0, then shifts d0 right by 8, then copies d0 to the memory at an offset from a0, then copies 129 to the memory at an offset from a0, then copies 17 to the memory at an offset from a0, then returns.
- `frag_move_rts`: Short code fragment that copies 1 to a fixed memory address, then returns.
- `read_cia_timer_values`: Checks if a word register is non-zero, and if so, reads bytes from specific CIA timer registers and does nothing else.
- `frag_lea_move_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then copies 2 to the memory at an offset from a0, then copies 1 to the memory at an offset from a0, then returns.
- `process_game_flags`: Tests bits in memory locations, performs conditional jumps based on those tests, and calls a trap routine when certain conditions are met.
- `check_condition_and_trap`: Compares a word value from a global structure with a register, branches if the register value is less than or equal, and calls another routine before returning.
- `set_bit_and_trap`: Sets a specific bit in a global structure and executes a system trap, then returns from the stack.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `frag_move_lea_rts`: Short code fragment that copies a0 to the memory at an offset from a4, then loads the address of the memory at an offset from a7 into a0, then copies a0 to the memory at an offset from a4, then returns.
- `initialize_rendering_data`: Sets up various data pointers and registers for rendering operations, initializes memory values, and performs calculations involving multiple variables before returning.
- `fill_with_negative_ones`: Fills two memory areas with negative one values using a loop that decrements a counter until it reaches zero.
- `update_boxing_state`: Updates boxing game state by clearing bit 3 of a control byte, decrementing a counter, reading joystick input from CIAA_PRA, and updating score-related data when certain conditions are met.
- `frag_moveq_lea_jsr_bsr_rts`: Short code fragment that loads 11 into d0, then loads the address of a fixed memory address into a0, then calls another routine (3 times in a row), then loads the address of a fixed memory address into a0, then calls another routine, then returns.
- `update_boxing_state_2`: Updates boxing game state by modifying global variables and handling input flags, then performs rendering setup and calls game logic routines.
- `frag_lea_jsr_dbra_bra`: Short code fragment that loads the address of a fixed memory address into a0, then calls another routine, then loads the address of a fixed memory address into a0, then calls another routine, then loops back while counting down d5, then jumps to another point in the code.
- `initialize_boxing_data_2`: Sets multiple memory locations with specific values including long words, words, and bytes, then calls another routine and branches to a subroutine before returning.
- `save_context_and_switch`: Sets up a context save area in memory by copying register values from the current execution context to a structure pointed to by a global variable.
- `restore_context_and_switch`: Restores execution context from a saved structure, including registers and stack pointer, and clears specific bits in global variables before returning.
- `handle_input_validation`: Checks a bit in memory, sets another bit if the condition is not met, calls a trap routine, and stores register contents to memory locations before returning.
- `execute_callback_and_update_state`: Calls a routine pointed to by a global variable if it is not null, clears that variable, sets a byte flag to one, and updates another global variable with a specific value.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `clear_and_set_flags`: Clears a long word in memory, sets a bit in another location, clears a different bit, and calls a trap routine when a condition is met.
- `process_joystick_input`: Disables interrupts, checks joystick status bits, reads from hardware registers, processes input data, and calls another routine when specific conditions are met.
- `update_input_state`: Reads a byte from the keyboard serial register, processes it with bitwise operations, stores the result in a global variable, and updates several hardware registers including CIA timer and interrupt control registers.
- `update_game_timer`: Checks joystick button states, calls a timer handling routine, clears specific bits in memory, updates pointers, and sets a hardware register before returning.
- `handle_timer_countdown`: Checks if a specific bit is set in a global variable, decrements a timer value, and performs conditional operations based on the timer's state.
- `update_rendering_data_4`: Updates rendering data by reading values from memory locations, performing bit tests on a control byte, and calling another routine to process the data.
- `execute_loop_with_trap`: Saves all general-purpose registers to the stack, performs a multiplication operation on a register, executes a loop based on the result, and calls another routine with conditional bit testing before returning.
- `set_bit_and_execute`: Sets a bit in memory, calls a trap routine, restores registers from stack, calls another routine, decrements a counter, and returns.
- `process_coordinate_components`: Saves all registers to the stack, reads two 16-bit values from memory locations pointed to by a0 and a1, performs bitwise AND operations on components of these values, compares and adjusts values based on component differences, and writes the results back to the memory location pointed to by a1.
- `frag_addi_cmp_beq_blt_subi_bra`: Short code fragment that adds 256 to d3, then compares d4 with d1, then branches if the last result was zero, then branches if the comparison was less, then subtracts 16 from d4, then jumps to another point in the code.
- `frag_addi_cmp_beq_blt_subi_bra_2`: Short code fragment that adds 16 to d4, then compares d5 with d2, then branches if the last result was zero, then branches if the comparison was less, then subtracts 1 from d5, then jumps to another point in the code.
- `frag_addi_or_move_adda_dbra_movem_rts`: Short code fragment that adds 1 to d5, then merges d3 into d5, then merges d4 into d5, then copies d5 to the memory at a1, then adds a2 to a0, then adds a3 to a1, then loops back while counting down d7, then restores saved registers from the stack, then returns.
- `initialize_boxing_data`: Sets up boxing-related data including coordinates and flags based on input parameters, then stores results in global variables.
- `frag_neg_sub_ble_add_blt_move_bra`: Short code fragment that negates d0, then subtracts d0 from d6, then branches if the comparison was less or equal, then adds d0 to d4, then subtracts the memory at an offset from a4 from d2, then branches if the comparison was less, then subtracts d2 from d6, then branches if the comparison was less or equal, then copies an address to d2, then jumps to another point in the code.
- `frag_add_tst_blt_move_bra`: Short code fragment that adds d3 to d3, then tests d0, then branches if the comparison was less, then adds d0 to d5, then copies d7 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_neg_subq_sub_ble_add_move_bra`: Short code fragment that negates d0, then subtracts 1 from d0, then subtracts d0 from d6, then branches if the comparison was less or equal, then adds d0 to d4, then subtracts 1 from d5, then copies a location in the code to the memory at an offset from a4, then jumps to another point in the code.
- `frag_sub_bge_addq_clr_bra`: Short code fragment that subtracts the memory at an offset from a4 from d2, then branches if the comparison was greater or equal, then adds 1 to d6, then clears the memory at an offset from a4, then jumps to another point in the code.
- `frag_sub_ble_move_bra`: Short code fragment that subtracts d2 from d6, then branches if the comparison was less or equal, then copies a location in the code to the memory at an offset from a4, then jumps to another point in the code.
- `calculate_vertex_positions`: Performs bitwise operations on input values, calculates vertex coordinates using multiplication and addition, handles conditional logic based on input signs, and updates memory locations.
- `update_vertex_coordinates`: Calculates vertex position coordinates based on input parameters and stores the results in a structure at offset 80 of a6, then returns.
- `adjust_boxing_position_3`: Processes boxing position adjustments based on input values and global statistics, updating internal state parameters.
- `adjust_boxing_position_2`: Updates boxing state parameters including position adjustments and bitfield operations based on input values and global statistics.
- `adjust_boxing_position`: Updates boxing state parameters including position adjustments and bitfield operations based on input values and global statistics.
- `update_boxing_state_3`: Calculates and stores boxing-related parameters including position adjustments and bitfield operations based on input values and global state, then performs conditional updates to movement data and status flags.
- `frag_move_btst_beq_mulu_bra`: Short code fragment that copies d1 to the memory at an offset from a4, then tests bit 0 of the memory at an offset from a5, then branches if the last result was zero, then multiplies d0 by the memory at an offset from a5 (unsigned), then jumps to another point in the code.
- `update_boxing_parameters`: Calculates and stores boxing-related parameters including position adjustments and bitfield operations based on input values and global state, then performs conditional updates to rendering data structures.
- `update_rendering_parameters`: Processes input data through multiple arithmetic operations, updates memory locations with calculated values, performs bit testing and shifting operations, and manages rendering state variables.
- `frag_move_lsr_bcc_adda_bra`: Short code fragment that copies d7 to the memory at an offset from a6, then copies 0 to the memory at an offset from a6, then shifts d0 right by 1, then branches if the carry flag is clear, then copies 65535 to the memory at an offset from a6, then copies d3 to the memory at an offset from a6, then adds d1 to a3, then jumps to another point in the code.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `clear_screen_buffer`: Clears a screen buffer area by writing -1 values to a table of words using loop counters and buffer offsets.
- `initialize_rendering_data_3`: Processes rendering parameters from a data structure, calculates derived values including dimensions and offsets, stores them in a structure at offset 64 of a6, and writes a series of words to a buffer pointed to by a0.
- `initialize_rendering_data_2`: Initializes rendering parameters from a data structure, stores them in a structure at offset 64 of a6, then returns.
- `swap_and_update_matrix_elements`: Swaps matrix elements in a data structure and updates values in another structure using loop counters and arithmetic operations.
- `setup_audio_buffer`: Initializes audio buffer pointers, clears memory areas, sets up volume registers, and returns after processing audio data.
- `frag_move_bsr_rts`: Short code fragment that copies d0 to the stack, then calls another routine, then copies the top of the stack to d0, then copies d0 to a fixed memory address, then returns.
- `frag_move_rts_2`: Short code fragment that copies d0 to a fixed memory address, then returns.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `frag_movem_addq_move_cmp_bcs_clr_tst_be`: Short code fragment that saves registers on the stack, then adds 1 to a fixed memory address, then copies a location in the code to d0, then compares d0 with a location in the code, then branches if the carry flag is set, then clears a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_audio_registers`: Updates audio period registers based on input values and performs bitwise operations on global variables, then returns a modified value.
- `configure_audio_channels`: Configures multiple audio DMA channels by setting up channel addresses and lengths, then updates various counters and flags while managing audio state information through a series of memory operations.
- `update_boxing_state_5`: Processes boxing game state data based on input commands and updates position information, then configures DMA settings and handles further processing.
- `update_boxing_position`: Updates boxing game position data based on input command and performs DMA configuration, then calls another routine to handle further processing.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_boxing_state_4`: Updates boxing game state based on input command and performs DMA configuration, then calls another routine to handle further processing.
- `configure_audio_dma`: Configures DMA audio channels by setting up channel addresses and lengths, then updates various counters and flags.
- `process_input_value`: Reads a value from a structure, performs bitwise operations and comparisons, then stores a modified word at another offset in the structure.
- `frag_move_rts_3`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `divide_and_compare`: Loads a byte from a specific memory location, divides it by three using signed division, swaps the result bytes, then compares the quotient with zero and two to determine branch targets.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `lookup_and_compare_values`: Looks up a value in a table using an index, compares it with another value, and stores the result in a structure at offset 6 of a5.
- `frag_move_rts_4`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `adjust_and_clamp_value`: Subtracts a masked value from a word at offset 16 of a6, then clamps the result between zero and 113 using bitwise operations on the lower 12 bits.
- `increment_and_limit_value`: Adds a masked value to a word at offset 16 of a6, then limits the result to 856 using bitwise operations on the lower 12 bits.
- `update_animation_state`: Processes a value from a structure, performs bitwise operations and comparisons, then stores a modified word at another offset in the structure.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_beq_clr`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies d0 to the memory at an offset from a6, then clears the memory at an offset from a6.
- `adjust_position_and_update`: Reads a word from a structure, performs comparisons and arithmetic operations on another word in the structure, then stores the result back to the structure.
- `process_collision_data`: Processes collision data by comparing values, updating counters, and storing results in a structure at offset 6 of a5.
- `combine_and_process_bits`: Combines nibbles from two memory locations, performs arithmetic operations with a lookup table value, and stores the result in a structure at offset 6 of a5.
- `calculate_movement_offset`: Calculates a movement offset based on input parameters, performs arithmetic operations, and stores the result in a structure at offset 6 of a5.
- `frag_tst_bpl_move_sub_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `frag_move_andi_mulu_lsr_tst_bmi_add_bra`: Short code fragment that copies an indexed entry from a4 to d2, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then multiplies d2 by d0 (unsigned), then shifts d2 right by 7, then copies the memory at an offset from a6 to d0, then tests the memory at an offset from a6, then branches if the last result was negative, then adds d2 to d0, then jumps to another point in the code.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_bsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_5`: Short code fragment that calls another routine, then jumps to another point in the code.
- `process_and_clamp_input`: Processes input bits through multiple operations including bit shifting, table lookups, and arithmetic calculations, then clamps the result between zero and sixty-four before storing it at offset 8 of a5.
- `frag_tst_bpl_move_sub_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `clamp_and_accumulate_value`: Loads a byte from memory, multiplies it by a value, shifts the result, accumulates it with another value, and returns.
- `clamp_and_store_value`: Clamps the signed difference between d2 and d0 to a range between zero and sixty-four, stores the result as a word at offset 8 of a structure in a5, and updates a byte at offset 29 of a structure in a6 by adding the high nibble of the byte at offset 28 shifted right by two.
- `update_velocity_limit`: Reads a byte from offset 3 of a structure, copies it to a data register, and performs conditional arithmetic operations on that value before storing the result at offset 20 of the same structure.
- `frag_move_rts_5`: Short code fragment that copies 1 to the memory at an offset from a6, then returns.
- `adjust_speed_value`: Extracts the high nibble of a byte from offset 3 of a structure, adds it to a value at offset 19, and clamps that result to a maximum of 64 before storing it back.
- `decrement_and_clamp_byte`: Subtracts the low nibble of the byte at offset 3 of the structure in a6 from the byte at offset 19, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `frag_move_subq_clr_st_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then subtracts 1 from d0, then copies d0 to a fixed memory address, then clears a fixed memory address, then sets a fixed memory address to all ones, then returns.
- `clamp_byte_and_store`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5.
- `calculate_index_value`: Processes a byte from offset 3 of a structure by shifting its high nibble, multiplying by 10, adding the low nibble, and stores the result in a global memory location.
- `frag_move_beq_clr_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then clears a fixed memory address, then copies d0 to a fixed memory address, then returns.
- `process_input_command`: Reads a byte from offset 2 of a structure, extracts its lower nibble, and uses that value as an index to dispatch to different handling routines based on the input command.
- `decode_input_command`: Reads a byte from offset 3 of a structure, extracts its upper nibble, and uses it as an index to dispatch to different handling routines.
- `update_global_from_ciaa_pra`: Masks bit zero of the byte at offset 3 of the structure in a6, shifts it left by one, and ORs the result into a global variable after clearing bit one of that variable.
- `mask_and_merge_byte`: Masks the low nibble of a byte at offset 31 from a pointer in a6 and then merges the low nibble of a byte at offset 3 from the same pointer into it.
- `frag_move_andi_or_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with 15, then masks the memory at an offset from a6 with 240, then merges d0 into the memory at an offset from a6, then returns.
- `store_masked_byte`: The routine reads a byte from an offset in a structure pointed to by a register, masks it with a constant, and stores the result into a different offset in the same structure.
- `validate_and_update_counter`: Validates a byte value against a global variable, updates a counter at offset 34 of a6, and sets flags based on the comparison results.
- `frag_move_bra_4`: Short code fragment that copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_lsr_rts`: Short code fragment that copies a location in the code to d0, then shifts d0 right by 4, then copies d0 to the memory at an offset from a6, then returns.
- `mask_and_merge_nibble`: Masks the low nibble of a byte at offset 30 from a pointer in a6 and merges the high nibble from a byte at offset 3 of the same pointer, storing the result back at offset 30.
- `configure_dma_transfer`: Reads data from a structure to configure DMA channels, performs a delay loop, then updates DMA control registers with specific values before returning.
- `frag_tst_bne_moveq_move_andi_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_andi_bra_2`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_moveq_move_andi_cmp_bne_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then clears the memory at an offset from a6, then copies 0 to the memory at an offset from a5, then returns.
- `frag_moveq_move_andi_cmp_bne_beq_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a fixed memory address, then branches if the last result was not zero, then copies the memory at a6 to d0, then branches if the last result was zero, then copies d1 to the stack, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_andi_addq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then tests a fixed memory address, then branches if the last result was not zero, then adds 1 to d0, then copies d0 to a fixed memory address, then returns.
- `frag_tst_bne_move_andi_lsl_or_beq`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 4, then masks the memory at an offset from a6 with 15, then merges d0 into the memory at an offset from a6, then tests d0, then branches if the last result was zero.
- `update_position_limit`: Checks a byte at offset 31 of a structure, increments a value at offset 35 if a bit is set, and compares a calculated limit against a value to potentially update pointers in a structure and a global variable.
- `initialize_timer_registers`: Writes values to timer registers through a memory-mapped I/O address, performs bit operations on timer control registers, and waits for a specific condition before returning.
- `frag_tst_bne_moveq_rts`: Short code fragment that tests the memory at an offset from a4, then branches if the last result was not zero, then loads 255 into d0, then returns.
- `handle_input_validation_2`: Validates input data by performing bit operations on memory values and calling other routines to process the results.
- `process_collision_data_2`: Processes collision-related data by calculating distances and coordinates, then updates memory locations with computed values.
- `configure_port_b_pins`: Modifies port B pins through memory-mapped I/O by setting and clearing specific bits in a sequence of operations.
- `frag_move_clr_rts`: Short code fragment that copies a fixed memory address to d0, then copies d0 to the memory at an offset from a4, then clears the memory at an offset from a4 (3 times in a row), then copies 255 to the memory at an offset from a4, then returns.
- `process_timer_input`: Processes timer input values by calling a routine multiple times with decrementing loop counter, then checks a global variable and calls another routine if it's not zero.
- `compare_byte_sequence`: Compares an input byte against a sequence of bytes in memory, returning the index of the matching byte or -1 if no match is found within four comparisons.
- `update_boxing_flags`: Updates boxing-related flags and port B pin states based on timer and input conditions, modifying bit patterns in memory and hardware registers.
- `manage_bit_flags_and_memory`: Sets and clears bit flags in memory locations based on input values, performs conditional jumps based on register states, and copies data from a calculated memory address to another location.
- `update_bit_flags_and_copy_data`: Sets and clears bit flags in memory based on input values, modifies port B pins with specific bit operations, and copies a word value from one memory location to another.
- `process_flag_operations`: Processes bit flag operations based on a word value in d2, calling other routines conditionally and returning different values depending on branch conditions.
- `initialize_timer_data`: Sets up timer-related data in a structure using division and bit manipulation operations, then repeatedly calls other routines based on conditional checks until a counter reaches zero.
- `configure_timer_registers_2`: Sets up timer control registers based on input values, performs bit operations to configure timer behavior, and calls a timer initialization routine.
- `verify_data_structure`: Validates data in a buffer by comparing values with expected constants, computes checksums using lookup tables, and returns success or failure indicators.
- `validate_checksum`: Processes data from a buffer through a lookup table, computes an XOR checksum, and compares it with a reference value to determine success or failure.
- `frag_and_add_or_rts`: Short code fragment that masks d0 with d7, then masks d1 with d7, then adds d0 to d0, then merges d1 into d0, then returns.
- `copy_data_block`: Copies word-sized data from one memory location to another while managing counters and handling boundary conditions for data transfer.
- `execute_bit_operations`: Saves registers to stack, performs bit flag operations on input values, and calls routines based on conditional jumps that may repeat the operation.
- `frag_move_bsr_moveq_rts`: Short code fragment that copies d2 to d0, then calls another routine, then loads 255 into d0, then returns.
- `update_timer_data`: Updates timer-related data in a structure using division and bit manipulation operations, then performs conditional checks and updates multiple memory locations based on the results.
- `check_timer_conditions`: Performs conditional checks on timer data by calling routines and branching based on their results, with a loop that continues until specific conditions are met.
- `frag_bsr_move_beq_bra`: Short code fragment that calls another routine, then copies d0 to d2, then copies d3 to d0, then calls another routine, then copies d3 to d0, then calls another routine, then branches if the last result was zero, then jumps to another point in the code.
- `frag_addq_tst_bne_moveq_movem_rts`: Short code fragment that adds 1 to d3, then tests d2, then branches if the last result was not zero, then loads 0 into d0, then restores saved registers from the stack, then returns.
- `frag_move_moveq_bra`: Short code fragment that copies 240 to the memory at an offset from a6, then loads 255 into d0, then jumps to another point in the code.
- `configure_timer_registers`: Sets up timer control registers based on input values, performs bit operations to configure timer behavior, and calls a timer initialization routine multiple times.
- `initialize_rendering_data_3_2`: Processes rendering parameters from a data structure, calculates derived values including dimensions and offsets, then stores them in a structure.
- `frag_move_moveq_rts`: Short code fragment that copies 16384 to the memory at an offset from a6, then loads 255 into d0, then returns.
- `initialize_data_blocks`: Writes a fixed value to a series of memory locations pointed to by a global variable, then processes data blocks through a loop with specific offset calculations and bitwise operations before writing another fixed value at the end.
- `transform_and_store_data`: Transforms data through bitwise operations including shifts, ANDs, ORs, and XORs, then stores the processed results in multiple memory locations.
- `modify_word_bits`: Modifies bits in a sequence of words using bitwise operations including NOT, AND, OR, and shift operations to transform the data in place.
- `transfer_data_block`: Transfers word-sized data between memory locations while managing counters and handling boundary conditions for data transfer, similar to routine R2 but with reversed source and destination.
- `frag_moveq_move_tst_rts`: Short code fragment that loads 0 into d0, then copies the top of the stack to d2, then tests d0, then returns.
- `decode_bitstream`: Decodes a bitstream from a buffer, extracting bits according to specific rules and storing decoded values into a destination buffer.
- `initialize_boxing_data_3`: Initializes boxing-related rendering parameters when a4 is not zero, setting various dimensions and offsets in a data structure, then calls another routine before returning.
- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `update_movement_parameters`: Updates movement parameters by copying register values to local storage, performing arithmetic operations on counters, and calling other routines to process input validation before returning updated values.
- `adjust_boxing_position_4`: Adjusts boxing position parameters based on input values and global statistics, performing arithmetic operations on coordinates and updating internal state parameters through a loop of conditional checks.
- `frag_movea_movem_rts`: Short code fragment that copies a2 to a0, then restores saved registers from the stack, then returns.
- `frag_moveq_tst_bne_rts`: Short code fragment that loads 0 into d0, then tests the memory at a0, then branches if the last result was not zero, then returns.
- `frag_moveq_move_beq_movea_add_adda_bra`: Short code fragment that loads 0 into d1, then copies the memory at a0 (advancing a0) to d1, then branches if the last result was zero, then copies the memory at an offset from a4 to a1, then adds d1 to d1, then adds an indexed entry from a1 to a1, then adds the memory at an offset from a1 to d0, then adds the memory at an offset from a4 to d0, then jumps to another point in the code.
- `frag_sub_rts`: Short code fragment that subtracts the memory at an offset from a4 from d0, then returns.
- `read_joystick_input`: Reads joystick input from a hardware register and performs bitwise operations on the data to determine button states, storing the result in a global variable.
- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `frag_jmp_2`: Short code fragment that jumps to another point in the code.
- `frag_jmp_3`: Short code fragment that jumps to another point in the code.
- `frag_jmp_4`: Short code fragment that jumps to another point in the code.
- `frag_jmp_5`: Short code fragment that jumps to another point in the code.
- `frag_jmp_6`: Short code fragment that jumps to another point in the code.
