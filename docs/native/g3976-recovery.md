# Game 3976 (hyper viper (1996)(l. carminati)[h ale]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03976). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901200 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 14 raw-word lines |
| Functions named and described | pass, 126 of 126 named (0 author symbols), 126 of 126 described (35 by rule, 86 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 324 of 324 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2838 of 2838 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 164633 |
| code | 10904 |
| data | 92982 |
| padding | 3945 |
| unused | 628736 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Hyper Viper (1996)(L. Carminati)[h ALE].adf:type | 6 | 230 | 17 |
| Hyper Viper (1996)(L. Carminati)[h ALE].adf:HyperViper | 12 | 2581 | 107 |
| Hyper Viper (1996)(L. Carminati)[h ALE].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 1 |

## Cases handled

- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- hyper viper (1996)(l. carminati)[h ale]/Hyper Viper (1996)(L. Carminati)[h ALE].adf:type: Imploder (xfd) stream found by ancient scan (2812 -> 19504 bytes) but only 0 of 2 sampled windows are in reference RAM: not used

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_hardware` (HyperViper): Sets up hardware registers including DMA and interrupt control, reads current register values, modifies them with specific bit patterns, and stores the results back to global variables for later restoration. Kind: disk hardware check. Pass path: The routine initializes hardware registers including ADKCON and checks DMACON status, suggesting a disk drive or track validation check. If the check passes, execution continues normally; if it fails, a sabotage path or trap may be taken..

## Functions


### Hyper Viper (1996)(L. Carminati)[h ALE].adf:type

- `execute_init_and_free`: Calls an initial routine and a loop that executes a function pointer from a global variable, then clears a value at an offset from a pointer and returns a concatenated register value.
- `frag_move_lea_movea_adda_dbra_addq_rts`: Short code fragment that copies d6 to d7, then loads the address of a location in the code into a1, then copies the memory at a1 to a1, then adds a1 to a1 (2 times in a row), then loops back while counting down d7, then adds 4 to a1, then returns.
- `execute_data_processing_loop`: Executes a data processing loop that performs arithmetic operations, calls other routines, and manages counters while processing data through conditional branches.
- `process_data_loop`: Processes data through conditional branches and loops, calling other routines and performing arithmetic operations on memory locations.
- `frag_beq_movea_bsr_bra`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_pattern_data_2`: Processes compressed pattern data by decoding bits and copying bytes to a destination buffer, updating pointers and counters during decompression.
- `frag_moveq_add_bne_move_addx_dbra_addq`: Short code fragment that loads 2 into d7, then adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d0 and the extend flag to d0, then loops back while counting down d7, then adds 6 to d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_pattern_bits`: Processes bit patterns from a buffer, performing bitwise operations and conditional jumps based on carry flags, while updating counters and pointers.
- `frag_add_bne_move_addx_bra`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_pattern_bits_2`: Processes bitstream data from a buffer, decoding pattern information and updating internal counters and pointers.
- `decode_pattern_data`: Processes compressed pattern data by decoding bits and copying bytes to a destination buffer, updating pointers and counters during decompression.
- `frag_move_movem_bra`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Hyper Viper (1996)(L. Carminati)[h ALE].adf:HyperViper

- `initialize_system`: Initializes system resources by opening required libraries, setting up hardware registers including copper lists and blitter control, and configuring task priorities.
- `initialize_screen`: Sets up screen resources by calling intuition library functions and graphics library functions, then performs several operations on a global variable before returning.
- `setup_display`: Configures display parameters by setting values in a global structure, then calls graphics library functions to perform screen operations and finally frees allocated resources.
- `initialize_input_device`: Initializes an input device by setting up a global variable, performing I/O operations, and configuring device parameters before returning a status code.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `cleanup_device`: Checks if a cleanup flag is set, and if not, initializes device parameters and performs I/O operations before marking the cleanup as complete.
- `allocate_signal`: Allocates a system signal using exec library functions, sets up task information in a structure, and returns the allocated signal number or -1 on failure.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `initialize_hardware`: Sets up hardware registers including DMA and interrupt control, reads current register values, modifies them with specific bit patterns, and stores the results back to global variables for later restoration.
- `frag_movem_movea_move_cmp_bcs_jsr_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to a6, then copies the memory at an offset from a6 to d0, then compares d0 with 37, then branches if the carry flag is set, then calls another routine, then restores saved registers from the stack, then returns.
- `initialize_game_loop`: Initializes global variables and sets up a loop that manages game state through conditional checks and updates, configuring hardware registers including DMA control.
- `manage_game_state`: Updates a global variable based on conditional checks against a global value and results from another routine, with different behavior for various states.
- `update_input_state`: Updates a global variable based on input flags from memory, setting different values depending on which bits are set in a control byte.
- `update_enemy_direction`: Updates enemy movement direction based on a global state variable and performs conditional operations including counter modifications, comparisons with a register value, and memory writes to control flow.
- `update_state_machine`: Updates a global variable based on conditional checks against a global value and results from another routine, with different behavior for values 1 through 4.
- `update_enemy_position`: Updates enemy position based on a global state variable and performs conditional operations including counter modifications, comparisons with memory locations, and calls to other routines.
- `update_enemy_counter`: Updates a counter in a global variable and performs conditional operations based on its value, including calling other routines and modifying memory locations.
- `check_and_update_counter`: Checks conditions based on global variables and updates a counter, then calls another routine if certain conditions are met.
- `handle_direction_change`: Processes directional changes by modifying a counter value, calling another routine, and updating a global state variable based on comparisons with memory values.
- `frag_cmpa_beq_movea_moveq_bsr_rts`: Short code fragment that compares a0 with a location in the code, then branches if the last result was zero, then copies a0 to a6, then loads 0 into d7, then calls another routine, then returns.
- `manage_enemy_state`: Manages the state of an enemy by checking conditions and updating counters, then sets a flag and clears a word-sized counter when a condition is met.
- `manage_position`: Manages position updates by modifying a counter, performing comparisons with memory locations, and updating a global variable while potentially calling other routines.
- `process_text_data`: Processes text data by iterating through a string, checking for specific characters, and updating state variables based on the results.
- `increment_counter_cycle`: Increments a counter and performs conditional operations including comparison with a fixed value, clears another counter when conditions are met, and calls another routine for audio processing.
- `frag_move_add_cmp_bcs_addq_addi_rts`: Short code fragment that copies d0 to the stack, then adds d7 to a fixed memory address, then copies a location in the code to d0, then compares d0 with a location in the code, then branches if the carry flag is set, then adds 1 to a fixed memory address, then adds 5000 to a fixed memory address, then copies the top of the stack to d0, then returns.
- `frag_lea_move_moveq_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a6, then copies a location in the code to d7, then loads 40 into d6, then loads 5 into d5, then calls another routine, then returns.
- `process_command_data`: Processes command data by iterating through multiple memory locations, performing calculations on values, storing results in memory locations using indexed addressing, and updates a counter when a condition is not met.
- `process_command_stream`: Processes a command stream by iterating through data, performing calculations on values, and storing results in memory locations using indexed addressing.
- `copy_sprite_data`: Copies sprite data from a table to a destination structure using indexed addressing, copying 24 bytes for each entry based on a provided index.
- `check_and_process`: Checks a global byte variable for a specific value and conditionally executes two blocks of code that set up parameters and call another routine with those parameters.
- `process_input_stream`: Reads input bytes from a buffer, processes character values to update global variables, and stores computed results at specific memory locations.
- `frag_movea_moveq_bsr_addq_dbra_rts`: Short code fragment that copies a location in the code to a6, then loads 21 into d6, then loads 13 into d0, then loads 17 into d1, then calls another routine, then adds 1 to d6, then loops back while counting down d1, then adds 2 to d6, then loops back while counting down d0, then returns.
- `configure_blitter_operation`: Configures blitter DMA operations by setting up source and destination addresses, calculating parameters, and performing bit manipulation before returning.
- `frag_moveq_move_subq_divu_lsl_lea_adda`: Short code fragment that loads 0 into d0, then copies a location in the code to d0, then subtracts 1 from d0, then divides d0 by 10 (unsigned), then shifts d0 left by 2, then loads the address of a location in the code into a0, then adds d0 to a0, then copies the memory at a0 (advancing a0) to a fixed memory address, then copies the memory at a0 to a fixed memory address, then returns.
- `frag_move_moveq_lea_movea_rts`: Short code fragment that copies a0 to the stack, then loads 0 into d7, then copies the memory at a6 to d7, then loads the address of a location in the code into a0, then copies an indexed entry from a0 to d7, then copies the top of the stack to a0, then returns.
- `update_audio_volume`: Updates audio volume settings by incrementing a counter and writing the value to a hardware register until reaching a maximum threshold.
- `frag_moveq_bsr_subq_move_tst_bne_rts`: Short code fragment that loads 64 into d0, then calls another routine, then subtracts 1 from d0, then copies d0 to a fixed memory address, then tests d0, then branches if the last result was not zero, then copies 1 to a fixed memory address, then returns.
- `setup_audio_parameters`: Sets up audio parameters by calling multiple routines for audio processing, configures blitter registers with specific values, and performs multiple blitter operations with source and destination pointers.
- `configure_blitter`: Configures blitter registers with specific values and performs multiple blitter operations by setting source and destination pointers, then waits for completion.
- `initialize_memory_block`: Initializes a memory block by clearing memory locations, processes command data through multiple routines, and performs conditional operations based on character comparisons to control execution flow.
- `update_score_table`: Compares a value against entries in a table and inserts it if it's smaller than the largest entry.
- `clear_global_memory`: Clears a 5120-word block of global memory starting at offset 0, then clears a 3840-word block starting at offset 10240, and finally calls another routine twice.
- `load_score_file`: Opens a file named Viper-TopScores for reading, reads 220 bytes from it, then closes the file.
- `save_score_data`: Opens a file for writing using a global variable as the filename, writes 220 bytes of data from the same global variable, then closes the file.
- `clear_and_process`: Clears a block of memory starting at a global variable location, then calls routines to process data and performs conditional checks on a counter value before returning.
- `load_level_data`: Opens a file using a global variable as the filename, reads 16000 bytes into the same global variable, then closes the file.
- `clear_global_table`: Clears a table of 511 32-bit words starting at a global variable address and then calls several routines before returning.
- `manage_file_operations`: Manages file operations by opening a file, writing data to it, closing the file, and handling error conditions during the process.
- `frag_bsr_rts`: Short code fragment that calls another routine (2 times in a row), then returns.
- `clear_global_table_2`: Clears a table of 3072 words in a global variable by setting each word to zero, then calls multiple routines and returns.
- `clear_memory_block`: Clears a block of memory by writing zero values to a specific number of locations, then calls three other routines sequentially.
- `repeat_clear_memory`: Repeatedly clears memory blocks in four separate sequences, each with the same pattern of clearing and calling other routines.
- `initialize_graphics`: Sets up multiple graphics data structures by calling another routine with different parameters, then returns.
- `frag_subq_move_swap_addq_add_dbra_rts`: Short code fragment that subtracts 1 from d5, then copies d7 to the memory at an offset from a6, then swaps the halves of d7, then copies d7 to the memory at an offset from a6, then swaps the halves of d7, then adds 8 to a6, then adds d6 to d7, then loops back while counting down d5, then returns.
- `allocate_and_process_memory`: Allocates memory using a system call, processes pixel coordinates by iterating through data and storing results, then frees allocated memory and updates global state variables.
- `process_pixel_coordinates`: Processes pixel coordinates by iterating through data, performing calculations on values, and storing results in memory locations.
- `process_pixel_data`: Processes pixel coordinate data through a loop that shifts and combines values, then returns.
- `validate_coordinates`: Checks if a value at a specific memory location equals -2, or validates bit flags and range conditions before returning.
- `free_and_cleanup_resources`: Frees allocated memory using a system call, processes pixel coordinates by copying values from one location to another, and updates global state variables accordingly.
- `frag_moveq_move_swap_lea_dbra_rts`: Short code fragment that loads 3 into d0, then copies 432 to d1, then copies d1 to the memory at an offset from a6 (2 times in a row), then swaps the halves of d1, then copies d1 to the memory at an offset from a6 (2 times in a row), then loads the address of the memory at an offset from a6 into a6, then loops back while counting down d0, then returns.
- `setup_pixel_pattern`: Sets up pixel pattern data by copying values to specific memory locations and performing bit manipulation operations.
- `write_pixel_pattern`: Writes pixel data patterns to memory locations based on input coordinates and bit operations, then stores calculated values in a structure.
- `frag_movem_move_beq_bsr_addq_bra_rts`: Short code fragment that saves registers on the stack, then copies the memory at a5 (advancing a5) to d0, then branches if the last result was zero, then calls another routine, then adds 1 to a6, then jumps to another point in the code, then restores saved registers from the stack, then returns.
- `copy_sprite_data_2`: Copies sprite data from a source table to multiple destination locations with specific offsets, then returns.
- `parse_command_data`: Parses command data by iterating through a table, performing calculations on values, and storing results in memory locations using indexed addressing with specific offsets.
- `parse_command_stream`: Processes a command stream by reading bytes and executing different actions based on their values, including copying data, updating counters, and handling conditional logic.
- `iterate_structure_data`: Iterates through structured data elements, performing calculations and data movements while decrementing counters to control the loop execution.
- `parse_structure_data`: Parses structured data by comparing against a fixed identifier and handling conditional data movement based on bit flags.
- `copy_byte_sequence`: Copies byte sequences from input to output locations, handling both positive and negative count values with different copy strategies.
- `frag_movem_move_bsr_rts`: Short code fragment that saves registers on the stack, then copies d7 to the memory at a6, then copies 0 to a fixed memory address, then calls another routine, then restores saved registers from the stack, then returns.
- `configure_blitter_2`: Configures multiple blitter registers for graphics operations, then performs a loop of blitter operations using calculated memory addresses.
- `frag_move_rts`: Short code fragment that copies an address to a fixed memory address (2 times in a row), then copies 12960 to a fixed memory address, then returns.
- `frag_move_movea_addq_rts`: Short code fragment that copies a0 to the stack, then copies d7 to the memory at a6, then copies a location in the code to a0, then copies a6 to the memory at a0 (advancing a0), then copies d7 to the memory at a0, then adds 2 to a0, then copies an address to the memory at a0, then copies a0 to a fixed memory address, then copies the top of the stack to a0, then returns.
- `configure_blitter_registers`: Configures blitter registers with specific values and performs blitter operations using calculated memory addresses in a loop until reaching a termination condition.
- `setup_blitter_operation`: Sets up blitter registers with specific values and initiates a single blitter operation using provided memory addresses.
- `configure_blitter_dma`: Sets up blitter registers for a DMA operation using calculated source and destination addresses, then configures the blitter control registers with specific values.
- `frag_move_rts_2`: Short code fragment that copies an address to a fixed memory address (2 times in a row), then returns.
- `validate_and_update_position`: Validates position coordinates against boundary limits and updates position values when conditions are met, performing arithmetic operations and memory updates.
- `update_position_bounds`: Updates a position value in memory based on boundary checks, adjusting it when a calculated offset exceeds certain limits.
- `calculate_and_store_coordinates`: Calculates coordinate values and stores them in memory locations, with boundary checking that may update a stored value if conditions are met.
- `perform_multiple_blitter_operations`: Executes multiple blitter operations in a loop, updating source and destination addresses while configuring various blitter control registers for each iteration.
- `execute_blitter_dma_sequence`: Processes a sequence of blitter DMA operations by iterating through memory addresses, calculating values, and setting up blitter registers for each operation.
- `frag_bchg_bne_move_rts`: Short code fragment that flips bit 0 of a fixed memory address, then branches if the last result was not zero, then copies 0 to a fixed memory address, then copies 13868 to a fixed memory address, then returns, then copies 51200 to a fixed memory address, then copies 14384 to a fixed memory address, then returns.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a2, then copies 12799 to d0, then copies the memory at a0 to the memory at a1 (advancing a1), then copies the memory at a0 (advancing a0) to the memory at a2 (advancing a2), then loops back while counting down d0, then returns.
- `wait_for_vpos_bit_low_then_high`: Polls bit zero of the custom chip register at offset five until it reads low, then polls again until it reads high, then returns.
- `frag_subq_bsr_dbra_rts`: Short code fragment that subtracts 1 from d7, then calls another routine, then loops back while counting down d7, then returns.
- `compare_and_update_value`: Compares a value with another repeatedly using a helper routine, updating control flow based on comparison results until certain conditions are met.
- `frag_move_lsr_andi_rts`: Short code fragment that copies a fixed memory address to d7, then shifts d7 right by 8, then masks d7 with 511, then returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `setup_audio_channel_0`: Configures audio channel 0 by setting up DMA control registers, writing audio parameters to hardware registers, and managing bit flags in a global variable.
- `setup_audio_channel_1`: Configures audio channel 1 by setting up DMA control registers, writing audio parameters to hardware registers, and managing bit flags in a global variable.
- `setup_audio_channel_2`: Configures audio channel 2 by setting up DMA control registers, writing audio parameters to hardware registers, and managing bit flags in a global variable.
- `setup_audio_channel_3`: Configures audio channel 3 by setting up DMA control registers, writing audio parameters to hardware registers, and managing bit flags in a global variable.
- `update_audio_dma`: Configures audio DMA lengths for four channels based on a global variable and hardware status flags, then returns.
- `mute_all_audio_channels`: The routine sets the volume registers for all four audio channels to zero, effectively silencing the sound hardware.
- `process_text_data_stream`: Processes a text data stream by reading characters and executing different actions based on character values, including writing characters to memory and updating counters.
- `frag_btst_bne_moveq_move_bset_lea_rts`: Short code fragment that tests bit 7 of a location in the code, then branches if the last result was not zero, then loads 0 into d0, then copies a location in the code to d0, then sets bit 7 of a fixed memory address, then loads the address of a location in the code into a0, then copies an indexed entry from a0 to d0, then returns, then loads 0 into d0, then returns.
- `parse_decimal_string`: Processes a string of decimal digits by converting them to a numerical value through bit shifting and addition operations.
- `read_joystick_input`: Reads joystick movement data from hardware registers, updates global variables with directional states, and checks fire button status.
- `frag_btst_beq_bsr_bne_rts`: Short code fragment that tests bit 7 of a fixed memory address, then branches if the last result was zero, then calls another routine, then tests bit 7 of a fixed memory address, then branches if the last result was not zero, then returns.
- `initialize_input_handlers`: Reads mouse movement and potentiometer data, updates global variables with input differences, and sets flags based on button states.
- `initialize_graphics_2`: Sets up graphics registers including DMA control, sprite positions, and bitplane configurations.
- `handle_input_interrupt`: Processes input from a hardware register by testing specific bits, modifying data through bit operations, and updating registers while handling interrupt flags.
- `setup_system_interrupts`: Initializes interrupt and DMA control registers, calls several system routines for task management, and returns.
- `setup_system_task`: Adjusts task priority, disables system interrupts, and performs several system calls including disabling the processor and setting up memory management.
- `generate_random_seed`: Loads time-of-day data into a buffer, performs bit rotation operations on the values, and stores results back to memory.
- `update_rng_state`: Updates a pseudo-random number generator state by performing arithmetic operations on stored values and returns.

### Hyper Viper (1996)(L. Carminati)[h ALE].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_movem_movea_jsr_moveq_rts`: Short code fragment that saves registers on the stack, then copies a1 to a2, then copies an address to a0, then copies the memory at an offset from a2 to a1, then copies the memory at an offset from a2 to a2, then calls the routine whose address is in a2, then restores saved registers from the stack, then loads 0 into d0, then returns.
