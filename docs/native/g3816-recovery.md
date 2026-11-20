# Game 3816 (gluecksrad (1991)(gametek)(de)[v cli-manager]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03816). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 109 of 109 named (0 author symbols), 109 of 109 described (43 by rule, 66 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 826 of 826 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 31200 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3681 of 3681 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 188957 |
| code | 12334 |
| data | 96231 |
| padding | 4046 |
| unused | 599552 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 386 | 1 |
| Gluecksrad (1991)(GameTek)(De)[v CLI-Manager].adf:WheelMain | 1 | 3243 | 106 |
| Gluecksrad (1991)(GameTek)(De)[v CLI-Manager].adf:bootblock | 1 | 52 | 2 |
| Gluecksrad (1991)(GameTek)(De)[v CLI-Manager].adf:Puzzle.BM (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- gluecksrad (1991)(gametek)(de)[v cli-manager]/Gluecksrad (1991)(GameTek)(De)[v CLI-Manager].adf:Words.BM: high-entropy data file (7.80 bits/byte), classified as packed data
- gluecksrad (1991)(gametek)(de)[v cli-manager]/Gluecksrad (1991)(GameTek)(De)[v CLI-Manager].adf:Puzzle.BM: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 31200 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Disk-Validator

- `initialize_disk_validator`: Sets up disk validation parameters including counters, flags, and memory pointers based on global state, then performs a series of operations to prepare for disk checking routines.

### Gluecksrad (1991)(GameTek)(De)[v CLI-Manager].adf:WheelMain

- `initialize_wheel_main_2`: Allocates memory for a data structure, initializes its elements, and sets up library handles while managing memory allocation failures by freeing previously allocated blocks.
- `update_keyboard_state`: Reads the keyboard serial data register and updates a global structure with shifted bit patterns when a specific interrupt flag is set, then sets a timer value in another global structure.
- `process_wheel_input`: Increments a word value at offset 94 of a structure, then conditionally calls multiple routines based on bit states of a byte at offset 82 of the same structure, and finally sets a word value at offset 156 of a structure to 32.
- `frag_move_beq_subq_rts`: Short code fragment that copies the memory at an offset from a4 to d0, then branches if the last result was zero, then subtracts 1 from d0, then copies d0 to the memory at an offset from a4, then returns.
- `frag_movea_adda_move_clr_dbra_bclr_rts`: Short code fragment that copies the memory at an offset from a5 to a0, then adds 40000 to a0, then copies 499 to d0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then clears bit 3 of the memory at an offset from a4, then returns.
- `increment_and_swap`: Increments a byte value at offset 83 of a structure and swaps two 16-bit words in another structure when the counter reaches character '#'.
- `update_joystick_position`: Reads joystick movement values from hardware registers, calculates position changes with subtraction, clamps values within specified ranges, and updates memory locations.
- `initialize_wheel_main`: Initializes wheel-related data including position coordinates, memory addresses, and graphics settings, then sets up interrupt vectors for input handling.
- `accumulate_vector_components`: Processes three consecutive memory locations by adding their 32-bit values to corresponding accumulators, then clears the source values and sets their 16-bit components to zero.
- `frag_btst_bne_moveq_bsr_rts`: Short code fragment that tests bit 6 of a fixed memory address, then branches if the last result was not zero, then loads 102 into d0, then calls another routine, then returns.
- `frag_btst_bne_moveq_bsr_beq_rts`: Short code fragment that tests bit 6 of a fixed memory address, then branches if the last result was not zero, then loads 102 into d0, then calls another routine, then loads 100 into d0, then calls another routine, then tests bit 6 of a fixed memory address, then branches if the last result was zero, then returns.
- `process_input_character`: Processes input characters from a source buffer, writes transformed values to a destination buffer, and updates a global flag when specific conditions are met.
- `frag_move_lea_cmp_bne_movea_rts`: Short code fragment that copies a0 to the stack, then loads the address of a fixed memory address into a0, then compares d0 with the memory at an offset from a0, then branches if the last result was not zero, then copies the top of the stack to a0, then returns.
- `frag_movem_lea_move_btst_bne_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then copies the memory at an offset from a0 to d0, then tests bit 14 of d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `frag_movem_movea_lea_bra`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies the memory at an offset from a5 to a0, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `frag_movem_movea_lea_bra_2`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a5 to a0, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `frag_movem_bra`: Short code fragment that saves registers on the stack, then jumps to another point in the code.
- `frag_movem_bra_2`: Short code fragment that saves registers on the stack, then jumps to another point in the code.
- `setup_bitblt_parameters`: Calculates and sets up bitblt parameters for graphics operations using coordinates and offsets from global variables, then writes these parameters to custom hardware registers.
- `scan_and_update`: Scans through a data structure using nested loops, compares bytes, updates a counter, and modifies memory locations based on comparison results.
- `update_wheel_position`: Updates wheel position coordinates based on boundary checks and flag states, then performs iterative operations including display updates and conditional branching based on counter values.
- `frag_moveq_bsr_rts`: Short code fragment that loads 104 into d0, then calls another routine, then returns.
- `frag_movem_moveq_bra`: Short code fragment that saves registers on the stack, then loads 0 into d2, then jumps to another point in the code.
- `update_position_and_check_bounds`: Calculates position values using multiplication and addition, checks a condition, updates coordinates with increment/decrement operations, performs boundary checking, and manages a counter with conditional resets.
- `toggle_flag_and_display`: Toggles a flag byte at offset 153 of a structure, then calls routines for display operations multiple times based on a counter value.
- `configure_blit_operation`: Sets up bitblt operation parameters including buffer sizes, addresses, and control registers based on input values and global data pointers.
- `update_wheel_state`: Updates a wheel state counter based on input values and performs bit manipulation on a pattern buffer, then resets the counter after a specific condition is met.
- `update_position_flags`: Clears a memory location, processes two 16-bit values by shifting and adding constants, then sets specific bits in the cleared location based on comparison results.
- `adjust_collision_values`: Performs bitwise operations on pairs of 16-bit values from two memory locations, comparing them and adjusting one value based on the comparison result in a nested loop structure.
- `frag_move_beq_movem_bsr_addq_bra_rts`: Short code fragment that copies the memory at a0 (advancing a0) to d2, then branches if the last result was zero, then saves registers on the stack, then calls another routine, then restores saved registers from the stack, then adds 8 to d0, then jumps to another point in the code, then returns.
- `initialize_graphics_operation`: Validates coordinate inputs against bounds, calculates memory addresses using division and multiplication operations, then sets up graphics parameters for bitblt operations.
- `frag_move_cmp_beq`: Short code fragment that copies the memory at an offset from a4 to d0, then compares d0 with the memory at an offset from a4, then branches if the last result was zero.
- `compare_character_sequences`: Compares a character from a global table with a value in a global structure, advancing through the table until a match is found or end of table is reached.
- `frag_lea_movea_bra`: Short code fragment that loads the address of a location in the code into a1, then copies the memory at an offset from a5 to a0, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `frag_lea_bra`: Short code fragment that loads the address of a location in the code into a1, then jumps to another point in the code.
- `frag_lea_bclr_movea_bsr_bset_rts`: Short code fragment that loads the address of a location in the code into a1, then clears bit 0 of the memory at an offset from a4, then copies the memory at an offset from a5 to a0, then loads the address of the memory at an offset from a0 into a0, then calls another routine, then sets bit 0 of the memory at an offset from a4, then returns.
- `frag_lea_bra_2`: Short code fragment that loads the address of a location in the code into a1, then jumps to another point in the code.
- `frag_movea_lea_move_clr_dbra_rts`: Short code fragment that copies the memory at an offset from a5 to a0, then loads the address of the memory at an offset from a0 into a0, then copies 5499 to d0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `process_player_input_and_setup`: Displays text prompts, reads input values, validates player count input, stores the count in memory, formats player names, copies name data to memory locations, and manages display operations for multiple players.
- `process_input_buffer`: Processes input characters from a buffer, clears the current position, and handles backspace and character insertion operations while managing a counter and pointer.
- `process_table_entries`: Loads values from a lookup table based on an index, then processes these values through a subroutine until reaching a null terminator in the table.
- `setup_graphics_and_display`: Sets up graphics display parameters by loading addresses into registers, performs multiple operations including memory copying and register updates, calculates display values, and configures hardware registers for graphics operations.
- `frag_cmp_bne_moveq_bra`: Short code fragment that compares d2 with 48, then branches if the last result was not zero, then loads 32 into d2, then jumps to another point in the code.
- `format_number_string`: Converts a number into a string representation by repeatedly dividing by ten and storing digits in reverse order, then null-terminates the result.
- `update_wheel_pattern`: Processes a wheel pattern based on a input value, shifts bits through multiple memory locations, and updates bitmasks in a structured data layout.
- `update_wheel_value`: Calculates a modified value from multiple hardware register readings and memory locations, stores the result back to a global variable, then returns the remainder of dividing that result by another value.
- `calculate_step_position`: Computes an index based on memory differences, performs arithmetic operations using that index, updates a memory location with a new calculated value, and returns a word from a table at an offset.
- `frag_movem_bsr_addq_move_cmp_bcs_rts`: Short code fragment that saves registers on the stack, then calls another routine, then adds 1 to the memory at an offset from a4, then copies the memory at an offset from a4 to d0, then compares d0 with the memory at an offset from a4, then branches if the carry flag is set, then copies the memory at an offset from a4 to the memory at an offset from a4, then restores saved registers from the stack, then returns.
- `frag_movea_moveq_move_dbra_clr_bra`: Short code fragment that copies the memory at an offset from a4 to a2, then loads 8 into d0, then copies the memory at an offset from a5 to a0, then copies a0 to a1, then copies the memory at a2 (advancing a2) to the memory at a1 (advancing a1), then loops back while counting down d0, then clears the memory at a1, then loads 16 into d0, then loads 60 into d1, then jumps to another point in the code.
- `check_and_decrement`: Checks a byte value at a specific offset in a structure, and if it's non-zero, decrements that byte and returns a negative value.
- `update_wheel_main`: Updates wheel position coordinates based on boundary checks and flag states, then performs iterative operations including display updates and input processing.
- `count_vowels_consonants`: Compares characters from two memory locations, counts vowels and consonants in one string when the other string has a specific marker, and returns both counts as a 32-bit value.
- `check_button_state`: Checks joystick button states and performs bit manipulation on a memory value to determine if a specific bit should be cleared, returning a value based on that operation.
- `process_input_and_update_bitfield`: Loops while checking hardware button states and memory values, performs division on a memory location, updates a bitfield in memory, and returns a byte from a lookup table when a specific bit is cleared.
- `update_memory_with_bitmask`: Iterates through a string until finding a null terminator or matching character, calculates an offset based on the position, performs bit manipulation to create a mask, and applies that mask to memory locations.
- `find_character_position`: Searches through a memory location for a specific marker byte, counts matching characters, and returns a 32-bit value containing quotient and remainder from dividing the position by twelve.
- `validate_input_sequence`: Compares input characters against a reference sequence, advancing through both sequences and updating a counter, then handles input validation and display updates based on character matches or mismatches.
- `render_digit_pair`: Calculates digit values from a number, converts them to ASCII characters, and writes them to display memory using two separate graphics operations.
- `frag_mulu_add_bra`: Short code fragment that multiplies d0 by 10 (unsigned), then multiplies d1 by 10 (unsigned), then adds 100 to d0, then adds 26 to d1, then jumps to another point in the code.
- `frag_move_bsr_mulu_add_rts`: Short code fragment that copies 503 to d0, then calls another routine, then multiplies d0 by 49 (unsigned), then adds the memory at an offset from a5 to d0, then returns.
- `copy_memory_blocks`: Copies data between two memory locations in chunks, performs multiple iterations of copying four longwords at a time, and updates pointers during each iteration.
- `transfer_and_copy_data_blocks`: Transfers a value from one memory location to another, copies data between two memory regions in chunks, and performs multiple iterations of copying four longwords at a time.
- `process_and_transfer_data_blocks`: Checks if a memory location is non-zero, transfers its value to another location and clears it, then copies data between two memory regions in chunks, performing multiple iterations of copying four longwords at a time.
- `frag_movem_moveq_bsr_move_bset_rts`: Short code fragment that saves registers on the stack, then loads 0 into d0, then loads 102 into d1, then calls another routine, then copies 200 to the memory at an offset from a4, then sets bit 3 of the memory at an offset from a4, then restores saved registers from the stack, then returns.
- `frag_movem_bclr_moveq_bsr_rts`: Short code fragment that saves registers on the stack, then clears bit 3 of the memory at an offset from a4, then loads 0 into d0, then loads 102 into d1, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_move_lea_bsr_movea_rts`: Short code fragment that copies a0 to the stack, then loads the address of a location in the code into a0, then calls another routine, then copies the top of the stack to a0, then returns.
- `format_and_display_score`: Processes a value by shifting and dividing with constants, converts the result to ASCII, and calls another routine twice with formatted data, then performs additional operations including memory copying and graphics setup.
- `initialize_wheel_data`: Initializes wheel-related data including position coordinates, memory addresses, and graphics settings.
- `frag_bsr_subq_cmp_bne_rts`: Short code fragment that calls another routine, then subtracts 1 from d0, then compares d0 with 100, then branches if the last result was not zero, then returns.
- `encrypt_memory_block`: Processes bytes in a memory block using rotation, addition, subtraction, and XOR operations with constants, updating each byte based on the previous byte's value and loop counters.
- `frag_lea_clr_bra`: Short code fragment that loads the address of a location in the code into a2, then clears the memory at an offset from a4, then jumps to another point in the code.
- `frag_lea_move_bra`: Short code fragment that loads the address of a location in the code into a2, then copies 1 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_lea_move_bra_2`: Short code fragment that loads the address of a location in the code into a2, then copies 1 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_lea_move_bsr_bset_rts`: Short code fragment that loads the address of a location in the code into a2, then copies 1 to the memory at an offset from a4, then calls another routine, then loads the address of a fixed memory address into a0, then copies 32783 to the memory at an offset from a0, then sets bit 1 of the memory at an offset from a4, then returns.
- `frag_bclr_bsr_lea_move_rts`: Short code fragment that clears bit 1 of the memory at an offset from a4, then calls another routine, then loads the address of a fixed memory address into a0, then copies 15 to the memory at an offset from a0, then returns.
- `initialize_audio_channels`: Sets up audio channel registers with specific values from a data buffer, then calls another routine to process each channel.
- `frag_clr_move_movea_rts`: Short code fragment that clears the memory at an offset from a0, then copies 1 to the memory at an offset from a0, then copies the memory at a0 to a1, then clears the memory at an offset from a1, then copies the memory at an offset from a0 to a1, then copies the memory at a1 to the memory at an offset from a0, then copies the memory at an offset from a1 to the memory at an offset from a0, then returns.
- `frag_movea_bsr`: Short code fragment that copies the memory at an offset from a5 to a0, then calls another routine, then copies the memory at an offset from a5 to a0, then calls another routine, then copies the memory at an offset from a5 to a0, then calls another routine, then copies the memory at an offset from a5 to a0.
- `frag_movea_clr_rts`: Short code fragment that copies the memory at a0 to a1, then clears the memory at an offset from a1, then returns.
- `update_wheel_data_4`: Checks a byte value at offset 156 of a structure, and if non-zero, calls another routine four times with addresses loaded from offsets 100, 104, 108 and 112 of another structure.
- `update_wheel_state_3`: Updates wheel-related data by decrementing a counter at offset 16 of a structure, and based on its value, modifies flags and performs memory operations.
- `update_wheel_state_2`: Updates wheel-related data by decrementing a counter at offset 16 of a structure, and based on its value, modifies flags and performs memory operations.
- `frag_movea_move_bra`: Short code fragment that copies the memory at an offset from a0 to a1, then copies a1 to the memory at an offset from a0, then jumps to another point in the code.
- `process_wheel_data`: Updates wheel-related data by shifting a value from a pointer, storing it in multiple locations, and performing arithmetic operations on it.
- `update_wheel_data`: Updates wheel-related data by shifting a value from a pointer, storing it in multiple locations, and performing arithmetic operations on values from another pointer before branching to a loop.
- `handle_dma_control`: Checks a value against 129 and branches if not equal, writes a word value to a memory location, then sets a bit in another memory location.
- `frag_bclr_addq_bra`: Short code fragment that clears bit 0 of the memory at an offset from a0, then adds 4 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_cmp_bne_tst_beq_bset_move_ori_bra`: Short code fragment that compares d0 with 130, then branches if the last result was not zero, then tests the memory at an offset from a1, then branches if the last result was zero, then sets bit 1 of the memory at an offset from a0, then copies the memory at an offset from a0 to d0, then merges 32768 into d0, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `frag_bclr_addq_bra_2`: Short code fragment that clears bit 1 of the memory at an offset from a0, then adds 4 to the memory at an offset from a0, then jumps to another point in the code.
- `update_wheel_data_1`: Updates wheel-related data structure based on calculations involving offsets and counters, configures DMA interrupts and updates related values.
- `update_wheel_data_3`: Updates a wheel-related data structure based on calculations involving offsets and counters, then configures DMA interrupts and updates related values.
- `update_wheel_data_2`: Updates a wheel-related data structure based on calculations involving offsets and counters, then configures DMA interrupts and updates related global variables.
- `check_condition_and_update`: Tests a bit in memory and branches if clear, then compares word values from memory locations with a register, updates a counter based on comparison results, and stores the final value back to memory.
- `frag_sub_cmp_bcs_move_bra`: Short code fragment that subtracts the memory at an offset from a0 from d0, then compares d0 with the memory at an offset from a0, then branches if the carry flag is set, then copies d0 to the memory at an offset from a0, then jumps to another point in the code.
- `configure_dma_and_interrupts`: Copies a word value from a structure to another location, processes a counter with a loop, and enables DMA channels with specific bits set.
- `calculate_coordinate_deltas`: Compares two coordinate values with stored values, calculates direction and magnitude of movement, performs division operations to determine step sizes, and updates coordinates while setting a flag when complete.
- `calculate_wheel_statistics`: Processes character data from memory, performs arithmetic operations including multiplication and division, compares results with thresholds, and calls multiple routines for audio playback, graphics updates, and memory management based on the calculations.
- `manage_wheel_game_state`: Loads a structure address from offset 120 of a structure, checks a byte value at offset 20 of that structure, and if non-zero decrements the byte value before calling another routine, otherwise returns zero.
- `format_and_store_string`: Multiplies a register value by 1280, loads memory addresses into a register, checks a condition, performs multiple operations including string formatting and character storage, and updates memory locations with calculated values.
- `update_sprite_coordinates`: Calculates sprite positions using multiplication and addition operations, stores calculated values in memory locations, and sets bit 0 of a byte based on a condition.
- `frag_move_bra`: Short code fragment that copies 20000 to d7, then jumps to another point in the code.
- `copy_data_with_loop`: Copies data from one buffer to another with conditional branching, processes bytes using loop counters, and handles negative values by adjusting pointers.
- `perform_bit_operations`: Shifts bits in a sequence of words through multiple iterations, stores results in memory locations, and performs data shuffling operations on arrays.
- `check_joystick_status`: Tests a bit in a hardware register, compares byte values from memory locations, and updates memory with new values based on comparison results.
- `convert_number_to_string`: Prepares for string conversion by saving registers, divides a value by 10, performs division operations to extract digits, stores characters in memory, and handles decimal point placement during number formatting.
- `play_audio_sample`: Checks joystick status and a flag, loads audio parameters from memory, waits for vertical blank interrupt, and sets up audio registers for playback.

### Gluecksrad (1991)(GameTek)(De)[v CLI-Manager].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `copy_memory_block`: Copies data between memory locations in chunks of four bytes, clears memory areas, and jumps to a specific address after completion.
