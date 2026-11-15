# Game 1402 (bug bomber (1991)(kingsoft)[cr ip][t +2 ip]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01402). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (925328 top-level bytes in 20 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 64 raw-word lines |
| Functions named and described | pass, 279 of 279 named (0 author symbols), 279 of 279 described (79 by rule, 196 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1294 of 1294 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 346014 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4797 of 4797 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 16631 |
| code | 17672 |
| data | 392221 |
| padding | 3700 |
| unused | 495104 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:BUGBOMBER | 1 | 116 | 5 |
| disk-validator | 1 | 386 | 1 |
| Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:IP.txt | 2 | 24 | 4 |
| Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:IP! | 2 | 116 | 3 |
| Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:iconx | 12 | 806 | 37 |
| Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 881 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1117 | 56 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 128 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1183 | 131 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:mainpart (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 4194 of its 7155 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 4194 of its 7988 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 4194 of its 7631 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 4194 of its 8213 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference crashes at an address (HLE illegal instruction an address at an address: through vector 0, as before (vector 4 is 0 again)): 2658 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- 128 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- runtime segment an address: 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 119 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 308 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 32 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 636 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- bug bomber (1991)(kingsoft)[cr ip][t +2 ip]/Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:IP!: Time Cruncher 1.7 (xfd) stream found by ancient scan (1732 -> 2816 bytes) but only 2 of 2 sampled windows are in reference RAM: not used
- bug bomber (1991)(kingsoft)[cr ip][t +2 ip]/Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:mainpart: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 branch displacement, 1 immediate
- 1 never-loaded region(s), 346014 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `validate_password` (runtime segment an address (play-rs-m1500)): Processes input characters into a global buffer, performs bitwise transformations on a global variable based on buffer contents, and sets specific global values if transformation results match zero. Kind: copy protection (password validation with obfuscated checks). Pass path: The routine returns early with success if all the computed checksums match (bVar5 == 0 && bVar6 == 0 && bVar7 == 0 && bVar8 == 0 && bVar9 == 0 && bVar10 == 0). This indicates a password-based validation check where the input is processed th.
- `process_game_state` (runtime segment an address (play-rs-m1500)): Processes game state based on a global variable value, performs bit manipulation to determine character values, and stores these characters in memory locations starting at a specific address. Kind: copy protection (manual/code-wheel lookup). Pass path: The routine modifies RAM locations cRam0000123f through cRam00001244 with values derived from bitwise operations on game state variables. A successful check would proceed to write these computed values into the password buffer, likely used .
- `memory_xor_operations` (runtime segment an address (play-rs-m1500)): Performs XOR operations on sixteen consecutive 32-bit memory locations using a calculated offset, then updates global variables and performs additional memory operations based on hardware register values. Kind: copy protection (memory XOR operations). Pass path: The routine performs XOR operations on specific memory locations within the runtime segment. If the check passes, the program continues execution normally; if not, it likely jumps to a sabotage path or terminates..
- `update_status_and_memory` (runtime segment an address (play-rs-m1500)): Updates several global variables from a data table, performs a loop to clear a block of memory, and modifies a global variable via a bitwise OR. Kind: disk hardware/status check. Pass path: The routine updates internal state variables (uRam/DAT) based on hardware status; a successful check allows the routine to proceed to the memory/status update logic without triggering a crash or error state..

## Functions


### Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:BUGBOMBER

- `initialize_program_environment`: Opens files and libraries, performs a delay loop, and copies a block of code to a new memory location.
- `update_color_palette_indices`: Copies a block of data and performs conditional byte manipulations based on signed values.
- `copy_and_modify_data`: Copies a block of memory and performs complex conditional operations on bytes within a loop.
- `copy_bytes_with_conditional_jump`: Negates a byte, decrements a counter, and loops to copy bytes from one location to another.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### disk-validator

- `update_display_buffer`: Calculates character offsets and populates a memory buffer with numeric string representations of values.

### Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:IP.txt

- `call_library_entry`: Invokes another routine and checks the zero flag.
- `open_dos_library`: Calls the exec library function OldOpenLibrary to load dos.library.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.

### Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:IP!

- `generate_bitstream_table`: Constructs a bitstream in memory by iteratively shifting and masking values from a source table.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_right_bits_with_early_exit`: The routine shifts a 32-bit value right by one bit repeatedly, loading a new 32-bit word from a table pointed to by a pointer each time the shifted value becomes zero, and stops early if a counter reaches -1.

### Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:iconx

- `initialize_application_environment`: Sets up the initial execution state by retrieving system pointers, parsing command-line arguments, and handling string conversions.
- `restore_patch_state`: Retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `dispatch_cli_or_workbench_start`: Chooses between a command-line start and a Workbench start.
- `build_wait_and_endcli_command`: Builds a command string from a template and the given name, including a wait command with a long timeout and an endcli redirected to NIL, after checking whether the name starts with a dot.
- `install_library_functions`: Creates and installs a library from a descriptor: it makes the library structure, sets up its function table, and returns the result.
- `scan_ram_entries_for_names`: Allocates a buffer and walks a linked list of entries, copying the entries whose name bytes are above the control range into it and passing them to helper routines.
- `concatenate_strings_and_call_helper`: Copies a source string into a destination buffer, appends a second string to the end of the first, and then calls another routine using the resulting buffer and the total combined length as arguments.
- `count_string_length`: The routine receives a pointer to a zero-terminated byte string and returns the number of bytes before the terminating zero.
- `copy_null_terminated_string`: As part of library code, this routine copies bytes from a source pointer to a destination pointer until a null terminator is encountered.
- `append_string_end_and_call`: Advances a pointer to the terminating zero of a string, then passes that position and a second argument to a helper routine.
- `parse_signed_decimal`: Parses a signed decimal integer from a null-terminated character string, handling an optional leading sign.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_schedule`: Library stub: loads its arguments and the exec library base, calls Schedule and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_exception`: Library stub: loads its arguments and the exec library base, calls Exception and returns its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_alloc_entry`: Library stub: loads its arguments and the exec library base, calls AllocEntry and returns its result to the caller.
- `recurse_with_stack_slot`: Calls another routine passing two parameters and a pointer to a specific location on the stack.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `invoke_table_entry`: The library code invokes a function pointer retrieved from an offset within a global variable.
- `store_byte_at_address`: Stores the contents of a data register into a memory location pointed to by an address register.

### Bug Bomber (1991)(Kingsoft)[cr IP][t +2 IP].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_jmp`: Short code fragment that copies 204 to a fixed memory address, then jumps to another point in the code.
- `frag_move_jmp_2`: Short code fragment that copies 20217 to a fixed memory address, then copies 0 to a fixed memory address, then copies 22462 to a fixed memory address, then jumps to another point in the code.
- `frag_move_jmp_3`: Short code fragment that copies 20216 to a fixed memory address, then copies 280 to a fixed memory address, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game_state_2`: Sets up global variables and hardware registers including DMA control, interrupt settings, and CIA port directions.
- `initialize_game_state`: Initializes global variables and hardware registers including DMA control, interrupt settings, and CIA port directions.
- `frag_tst_bne_not_move_jsr_moveq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then inverts a fixed memory address, then copies an address to d0, then copies an address to d1, then calls another routine, then loads 0 into d0, then calls another routine, then returns.
- `setup_audio_channels`: Configures audio volume levels for four channels and enables DMA channels when a global variable is non-zero, while also performing state reset and initialization operations.
- `update_collision_flags_2`: Processes indexed data from two memory locations by reading bytes and words, performing multiplication and addition operations, then calling another routine to update collision flags based on the computed values.
- `frag_movem_move_bsr_rts`: Short code fragment that saves registers on the stack, then copies d2 to d0, then copies d3 to d1, then copies d4 to d2, then calls another routine, then restores saved registers from the stack, then returns.
- `update_collision_flags`: Updates collision-related flags in a data structure based on indexed offsets and a global variable, then performs a series of conditional calls to another routine depending on the value of the global variable.
- `display_text_elements`: Displays various text elements on screen by loading string addresses, processing input values through lookup tables, and calling rendering routines with different parameters based on game state conditions.
- `load_input_table_entry`: Loads a memory address from a table based on the input value in register D0 and returns it in A0.
- `validate_password`: Processes input characters into a global buffer, performs bitwise transformations on a global variable based on buffer contents, and sets specific global values if transformation results match zero.
- `display_text_strings`: Displays three text strings at different positions on the screen by calling a subroutine with specific coordinates for each string.
- `frag_lea_move_bsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 88 to d6, then copies 96 to d7, then calls another routine, then returns.
- `process_game_state`: Processes game state based on a global variable value, performs bit manipulation to determine character values, and stores these characters in memory locations starting at a specific address.
- `frag_lea_move_bsr_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then copies 40 to d6, then copies 80 to d7, then calls another routine, then loads the address of a fixed memory address into a0, then copies 8 to d6, then copies 112 to d7, then calls another routine, then returns.
- `process_input_string`: Processes a null-terminated string from address A0 by reading each byte, using it as an index into a lookup table at 5220, and calling another routine with the indexed value.
- `process_input_string_2`: Processes a null-terminated string from address A0 by reading each byte, using it as an index into a lookup table at 5220, and calling another routine with the indexed value.
- `frag_move_lea_clr_lsl_bsr_dbra_rts`: Short code fragment that copies 12 to d5, then loads the address of a fixed memory address into a0, then clears d6, then copies d5 to d7, then shifts d7 left by 4, then calls another routine, then loops back while counting down d5, then returns.
- `frag_move`: Short code fragment that copies the memory just below a0 (moving a0 down) to d0, then copies d0 to d0 (5 times in a row).
- `handle_keyboard_input`: Processes keyboard input by reading a value from memory location 258, clearing it, setting a timer, and updating various game state variables based on the input value through multiple conditional checks.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 15 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `check_bit_condition`: Checks if a specific bit in a memory location is set, using an offset calculated from a value in memory.
- `validate_and_return_value`: Validates a condition based on multiple memory values and returns either a modified or unchanged value from register D0 depending on the validation results.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 48 into d3, then returns.
- `input_polling_loop`: Repeatedly calls input processing routines and checks a result register, looping until the register contains zero or non-zero values in alternating phases.
- `copy_and_clear_data_blocks`: Copies data blocks from one memory location to another, then clears a subsequent block of memory with zeros.
- `initialize_game_objects`: Initializes multiple game object structures with specific parameters based on a global difficulty level, clearing several memory locations and setting up object properties such as type, size, and behavior flags.
- `update_memory_with_condition`: Updates memory locations based on conditions, iterating through a range of values and modifying specific memory addresses when certain conditions are met.
- `configure_blitter_sequence`: Configures blitter parameters based on a global variable's value, updating hardware registers with specific settings for different sequence stages, and then calls another routine before returning.
- `process_blitter_flags`: Reads a 16-bit value from a global variable, clears it, and processes individual bits using conditional branches.
- `configure_blitter_from_flags`: Checks bits 6 through 11 of a register and for each set bit calculates destination parameters using data from a structure and a lookup table, then calls a subroutine to update blitter settings.
- `blitter_parameter_calculation`: Calculates blitter parameters based on input values through a series of conditional comparisons and arithmetic operations, then calls another routine with calculated coordinates.
- `update_blitter_parameters`: Calculates blitter parameters using table lookups and register inputs, then updates multiple hardware registers including bitplane and area parameters for graphics rendering.
- `configure_blitter_double`: Configures blitter parameters twice with different memory addresses and values, updating hardware registers for graphics operations.
- `configure_blitter`: Sets up blitter registers with specific values including source and destination addresses, then waits for DMA to complete before configuring the blitter again with different parameters.
- `setup_blitter_dma_parameters`: Sets up blitter DMA parameters by calculating memory addresses from table lookups, updates hardware registers including address pointers and control words, and configures blitter operations for graphics rendering.
- `setup_blitter_parameters`: Writes values to a pointer and updates several blitter-related hardware registers and memory locations including address pointers, control words, and size parameters.
- `vertical_position_check_loop`: Checks vertical position against a value read from hardware register, performs conditional operations based on comparison results, and manages a counter that decrements until zero before branching to another routine.
- `frag_cmpi_beq_lea_bra`: Short code fragment that compares d0 with 65535, then branches if the last result was zero, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `frag_lea_bra`: Short code fragment that loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_bmi_bsr_subq_bne_bra`: Short code fragment that copies the memory at a0 to d0, then branches if the last result was negative, then calls another routine, then subtracts 1 from a fixed memory address, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_cmpi_beq_lea_bra_2`: Short code fragment that compares d0 with 65535, then branches if the last result was zero, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `frag_lea_bra_2`: Short code fragment that loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `wait_for_raster_line_2`: Checks the vertical position of the display beam against specific raster line values and continues looping until a match is found or a counter expires.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `update_blitter_dma`: Updates blitter parameters and DMA control registers with specific values, performing conditional operations based on a test result before writing to hardware registers.
- `frag_btst_bne_move_rts`: Short code fragment that tests bit 6 of the memory at an offset from a6, then branches if the last result was not zero, then copies d0 to the memory at an offset from a6, then copies d1 to the memory at an offset from a6, then copies 2544 to the memory at an offset from a6, then copies an address to the memory at an offset from a6, then copies 5121 to the memory at an offset from a6, then returns.
- `copy_and_clear_memory`: Copies data from one memory location to another while simultaneously clearing multiple memory locations, repeating this process for a fixed number of iterations.
- `configure_blitter_loop`: Initializes pointers to two memory locations, clears two word registers, copies the value of one register to another, shifts the values left by four bits, loads a byte from one location and branches if it's non-zero, otherwise clears a register and branches to another routine.
- `frag_cmpi_bne_bsr_bra`: Short code fragment that compares d5 with 1, then branches if the last result was not zero, then calls another routine, then jumps to another point in the code.
- `blitter_configuration_loop`: Configures blitter parameters in a loop structure, incrementing counters and performing conditional branches based on counter values, with calls to configure blitter functions at specific intervals.
- `toggle_coprocessor_dma`: Configures coprocessor jump and latch registers before enabling DMA channels, performing a recursive sequence of calls that modifies global state and triggers delays.
- `wait_for_condition_and_set_flag`: Enters a loop that decrements a counter while checking a bitmask against a global variable, setting specific values to global variables depending on whether the condition is met before the loop expires.
- `frag_move_dbra_rts`: Short code fragment that copies 50 to d0, then loops back while counting down d0, then returns.
- `process_keyboard_input`: Reads keyboard serial data from hardware registers, processes it through a loop that decrements a counter, and calls another routine if a global variable is non-zero.
- `frag_move_rte`: Short code fragment that copies the top of the stack to d0, then copies 32767 to a fixed memory address (2 times in a row), then copies an address to the memory at an offset from a7, then copies 4 to a fixed memory address, then returns from the exception.
- `process_input_and_update_state`: Reads input data from a table and updates several global variables based on specific bitwise conditions and button states.
- `process_audio_bitmask`: Iterates through bits in a word, performs actions based on specific bit settings, and decrements several global counters.
- `frag_moveq_tst_bne_lea_rts`: Short code fragment that loads 0 into d5, then tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d4, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a3, then returns.
- `frag_tst_bne_moveq_lea_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 1 into d4, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a3, then returns.
- `frag_tst_bne_moveq_lea_rts_2`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 2 into d4, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a3, then returns.
- `frag_moveq_lea_rts`: Short code fragment that loads 3 into d4, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a3, then returns.
- `set_audio_registers`: Checks specific bits of a global variable and sets audio hardware registers for channels 0 through 3 based on those checks.
- `calculate_vertical_position`: Performs a series of bit operations and arithmetic calculations using hardware register values to determine a vertical position value.
- `memory_xor_operations`: Performs XOR operations on sixteen consecutive 32-bit memory locations using a calculated offset, then updates global variables and performs additional memory operations based on hardware register values.
- `xor_memory_blocks`: Calculates an offset based on a global variable and performs a bitwise XOR operation on sixteen consecutive 32-bit memory locations.
- `clear_and_init_struct`: Clears a block of memory starting at a fixed address, initializes several fields in a data structure based on table lookups and conditional logic, and sets up pointers for further processing.
- `update_game_state`: Updates game state variables based on table lookups and conditional checks, then performs iterative operations on global data structures before returning.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `process_collision_data_2`: Processes collision information from a table, performs multiplication and bit operations on the data, and stores the results in memory locations.
- `process_collision_data`: Reads collision information from a memory location, processes it using multiplication and bit operations, and stores the results in a destination buffer.
- `load_and_process_data`: Loads a word value from a memory location and uses it as an index to read a byte, which is then stored in another register.
- `frag_move_dbra_lea_cmpi_bne_clr_rts`: Short code fragment that copies 2 to an indexed entry from a1, then loops back while counting down d2, then loads the address of a fixed memory address into a1, then copies 259 to d0, then compares an indexed entry from a1 with 3, then branches if the last result was not zero, then clears an indexed entry from a1, then loops back while counting down d0, then returns.
- `frag_move_rts`: Short code fragment that copies a fixed memory address to d4, then returns.
- `frag_move_swap_eor_rts`: Short code fragment that copies d1 to d0, then swaps the halves of d0, then copies a fixed memory address to d0, then copies a fixed memory address to d1, then exclusive-ors d1 into d0, then swaps the halves of d0, then copies d0 to d1, then swaps the halves of d0, then returns.
- `frag_move_subq_bne_rts`: Short code fragment that copies an address to d0, then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `initialize_data_tables`: Populates two separate memory tables with calculated integer values using loops.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `concatenate_registers`: Combines the contents of two 32-bit registers into a single 64-bit value.
- `process_string_and_graphics`: Calculates string lengths and uses library calls to manipulate hardware registers and memory.
- `branch_to_routine`: Performs a relative branch to another routine.
- `increment_pointer_and_store`: Checks if a value at a specific memory location is zero, and if not, stores two words from that location into another memory area before incrementing the original pointer.
- `store_constant_value`: Stores a constant value into a global variable.
- `set_global_pointer`: stores a constant value into a global variable.
- `initialize_global_constants`: Stores several constant word and byte values into specific memory locations and performs a bit test on a memory address.
- `branch_to_offset`: Performs a short relative branch to an address stored in a pointer register.
- `process_string_table_and_dispatch`: Calculates the length of strings within a table and stores a derived value into a memory offset before calling a function pointer for each character.
- `close_library_and_set_value`: Closes a library and stores a constant value into a global variable.
- `write_constant_values`: Writes three specific constant values into three consecutive memory locations.
- `write_memory_and_jump`: Writes two 16-bit values to a table and then jumps to a specific address.
- `copy_memory_blocks`: Copies a block of memory of a fixed size to a global variable and then copies a second block from a specific memory address to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_routine_2`: Performs a short relative branch to another routine.
- `reset_state_and_initialize_globals`: Clears several global variables and then performs a series of assignments to specific memory locations, including setting a variable to a fixed value and others to specific constants.
- `branch_to_address_2`: Performs a short relative branch to a specific memory address.
- `update_object_state_via_command`: Parses a 32-bit command value to update various fields within an object structure in a table.
- `branch_to_address`: Performs a relative branch to another address.
- `reset_hardware_and_globals`: Clears several global variables and resets audio period registers and DMA control.
- `jump_to_reset_system_state`: Jump stub: passes control straight on to reset_system_state.
- `jump_to_update_sprite_properties`: Jump stub: passes control straight on to update_sprite_properties.
- `jump_to_reset_system_state_2`: Jump stub: passes control straight on to reset_system_state.
- `jump_to_compare_and_set_flags`: Jump stub: passes control straight on to compare_and_set_flags.
- `branch_to_offset_15`: Performs a short relative branch to a location offset by 57824 bytes from the current instruction.
- `jump_to_reset_system_state_3`: Jump stub: passes control straight on to reset_system_state.
- `update_table_index_3_2`: Sets a global byte to one, stores a 16-bit value from a register into a global variable, and calculates a new pointer by performing a table lookup using a bit-shifted version of that register and storing the result in another global variable.
- `update_state_structure_2`: Sets a byte in a global variable to one, stores a word from d0 and a long from a1 into a structure pointed to by a5, and clears a long at a specific offset within that structure before returning via a relative branch.
- `branch_to_check_table`: Performs a short branch to another routine.
- `jump_to_set_global_and_call`: Jump stub: passes control straight on to set_global_and_call.
- `reset_audio_and_dma`: Calls a routine to reset system state, clears four audio volume registers, sets the DMA controller to disable all channels, and writes several values to a table before returning.
- `decrement_and_loop`: Decrements a value in a data structure and enters a series of nested loops if the value becomes negative.
- `tiny_addq`: Small routine of 1 instruction that adds 4 to a0.
- `update_entity_animation_state`: Updates an entity's current animation frame, pointer, and state attributes by indexing into a frame data table.
- `branch_to_offset_3`: Performs a short relative branch to a specific offset within another routine.
- `branch_to_offset_4`: Performs a short relative branch to a fixed offset within another routine.
- `process_animation_frame_control_codes`: Processes control bytes within an animation frame data sequence to update entity properties, frame indices, and sprite pointers.
- `branch_to_offset_2c0`: Performs a short branch to a specific offset.
- `branch_to_animation_logic`: Performs a short relative branch to a location within another routine.
- `branch_to_offset_6`: Performs a short relative branch to a specific instruction offset within another routine.
- `branch_to_animation_sequence`: performs a short relative branch to a location within another routine.
- `branch_to_animation_sequence_2`: Performs a short relative branch to an offset within another routine.
- `process_entity_state_logic`: Updates entity attributes and pointers by indexing into a table based on a byte value.
- `process_entity_state_machine`: Iterates through a list of data structures using a loop, performing different operations based on a command byte extracted from a table.
- `process_entity_attribute_table`: Checks a global flag to initialize an entity structure by copying bytes from offsets 13 and 15 to offsets 29, 30, and 31, then enters a loop that increments an index and performs a switch-based update of entity properties, such as attributes, pointers, and state, based on a value fetched from a table.
- `update_entity_sprite_and_pointer_2`: Updates an entity's sprite data and pointer by calculating an index and offset from a structure in a6, then enters a loop that processes a sequence of commands from a table.
- `branch_to_routine_3`: Performs a relative branch to a specific offset.
- `process_entity_action_switch`: Increments a counter in a structure, loads a value from a table into a structure using that counter, and performs a series of operations based on the lower nibble of a byte at offset 13.
- `reset_entity_state`: Clears a 16-bit word at offset 26 of the structure in a6, sets a byte at offset 72 of the structure in a0 to 255, and returns.
- `process_animation_frame_sequence`: Updates an entity's current animation frame and pointer by indexing into a frame data table.
- `update_sprite_properties_2`: Clears a word at offset 26 of a structure in a6, stores a byte at offset 72 of a0, and then returns.
- `update_pointer_data`: Moves a longword from one location to another, performs arithmetic on a byte, and uses that byte to index into a table to update a pointer.
- `copy_and_branch`: Copies a longword from offset 136 of a structure in a5 to offset 40 of the same structure, copies a word from offset 168 to offset 104, and branches to another routine.
- `update_entity_direction_and_state`: Checks a global flag and, if not set, initializes a structure by copying bytes from offsets 13 and 15 to offsets 29, 30, and 31, then sets a byte at offset 11 to 1.
- `clear_and_branch_2`: Moves a byte from offset 29 of a structure in a6 to offset 28, clears a byte at offset 11 of the structure in a6, clears a word at offset 0 of a structure in a1, and branches to another routine.
- `update_table_entry`: Loads a pointer into a1, masks a byte from a structure in a6 with 3, doubles that value, and moves a word from offset 14 of the structure in a6 to an offset in a table determined by the modified value, then branches to another routine.
- `update_entity_sprite_and_pointer`: Calculates an index and offset based on bytes at offsets 13 and 14 of a structure in a6, then writes bytes from a6 to offsets 72 and 73 of a structure in a5.
- `update_entity_attribute_list`: Reads a sequence of eight values from a memory table and updates a corresponding list of pointers and status bytes.
- `reset_memory_byte`: Stores a zero byte into a memory location calculated by adding an offset to a pointer held in a register.
- `update_entity_attribute_list_2`: Decrements a 16-bit counter in a structure pointed to by a6, increments an index in a5, and then fetches a sequence of eight 16-bit values from a table to update multiple attribute offsets within the structure in a5.
- `unpack_entity_data_block`: Increments an index in a structure and uses it to fetch a data block from a table, then iterates through eight 16-bit values within that block to update corresponding pointer and status fields in a separate structure.
- `update_sequence_and_load_data_table`: Increments a 16-bit counter at offset 4 of a structure in a5 and, if a global flag is zero, updates direction and byte values in a6 based on data in a0.
- `update_entity_attribute_list_sub_routine`: Increments a 16-bit counter at offset 4 of the structure in a5 and performs a series of lookups from a table to update multiple attribute fields in a5.
- `clear_memory_byte`: Stores a zero byte into a memory location using an offset from a register.
- `decrement_word_counter_if_positive`: Tests a 16-bit word at offset 34 of the structure in a6; if the value is zero or negative, it branches to another routine, otherwise it decrements the word at offset 34 of the structure in a6 and branches to another routine.
- `increment_and_branch`: Stores a 16-bit value of negative one into a structure at offset 34 of a6, increments a 16-bit value at offset 4 of a5, and branches to another routine.
- `decrement_and_branch`: Decrements a 16-bit value from a pointer in a0, stores the result at offset 34 of a6, stores the original value from a0 at offset 4 of a5, and branches to another routine.
- `copy_and_increment`: Copies a 16-bit value from a pointer in a0 to offset 6 of a5 and offset 16 of a6, increments a 16-bit value at offset 4 of a5, and branches to another routine.
- `increment_counter_and_update_direction`: Increments a 16-bit value at offset 4 of a5 and updates a direction byte at offset 11 of a6 based on a comparison between bytes at offsets 28 and 29 of a6.
- `move_and_clear`: Moves a byte from offset 29 of a6 to offset 28 of a6, stores zero in offset 11 of a6, clears a 16-bit value at offset 0 of a1, and branches to another routine.
- `load_and_call_sequence`: Loads four different addresses into a register and calls another routine after each load.
- `update_entity_state_and_interpolate_2`: Updates an entity's internal timers, position, and movement vectors through a series of decrements and arithmetic interpolations.
- `branch_to_offset_2`: Performs a short relative branch to a specific instruction location.
- `jump_to_offset`: Executes a short relative branch to another instruction location.
- `transfer_to_offset`: Redirects execution via a short relative branch to a different instruction location.
- `branch_to_offset_7`: Performs a relative branch to another location.
- `branch_to_offset_8`: Performs a short relative branch to a specific location within the current segment.
- `update_index_and_call`: Modifies a byte and a word within a structure based on specific conditions and performs an indirect call using a table index.
- `jump_to_store_calculated_pointer`: Jump stub: passes control straight on to store_calculated_pointer.
- `branch_to_offset_13`: Executes a short relative branch to a different location.
- `branch_to_offset_9`: Performs a short relative branch to a specific location within the same code segment.
- `branch_to_offset_10`: Performs a short relative branch to a specific location within the code segment.
- `reset_entity_and_dispatch`: Clears several fields in a data structure, increments a value at an offset, and performs an indirect call using a value retrieved from a table.
- `update_entity_state_and_dispatch_event`: Updates an entity's state by copying bytes from a source structure in a6 to a destination structure in a5, increments a counter at offset 16, and calls a function from a table using a value retrieved from a pointer at offset 12 of the source structure.
- `branch_to_offset_11`: Performs a short relative branch to a specific offset within the code segment.
- `branch_to_offset_14`: Performs a short relative branch to another location.
- `calculate_and_call`: Performs arithmetic calculations involving a table and a multiplier, stores the result in a structure, and performs an indirect call.
- `branch_to_offset_12`: Performs a short relative branch to a specific instruction offset within the current segment.
- `update_structure_and_call_2`: Modifies values within a structure based on conditional checks and then performs an indirect call using a value retrieved from a table.
- `jump_to_update_entity_data`: Jump stub: passes control straight on to update_entity_data.
- `update_position_and_index_2`: Adds a 16-bit value from offset 14 of a6 to a 16-bit value at offset 52 of a5, stores the result in offset 52 of a5 and offset 4 of a4, increments a 16-bit counter at offset 16 of a5, fetches a 32-bit value from a table using the counter and a pointer in a5, and calls another routine using a value from that table.
- `jump_to_update_entity_state_and_interp`: Jump stub: passes control straight on to update_entity_state_and_interpolate.
- `branch_to_offset_5`: Performs a short relative branch to a specific offset.
- `update_structure_and_call`: Updates a data structure with values from another source and performs an indirect call using a byte retrieved from a table.
- `update_entity_interpolation_state_2`: Copies a 32-bit value from offset 56 of a structure in a5 to offset 12 of the same structure, copies a 16-bit value from offset 64 of the structure in a5 to offset 16 of the same structure, increments a 16-bit counter at offset 16 of the structure in a5, fetches a 32-bit value from a table using the counter and a pointer in a5, and calls another routine using a value from that table.
- `jump_to_update_conditional_index`: Jump stub: passes control straight on to update_conditional_index.
- `branch_to_stack_offset`: Performs a short branch to an address located at an offset from the current stack pointer.
- `branch_to_indirect_address`: Performs a short branch to an address stored in a memory location pointed to by a register.
- `update_entity_kinematics_and_timers`: Updates an entity's internal timers, direction flags, and position using fixed-point arithmetic.
- `jump_to_decrement_byte_conditionally`: Jump stub: passes control straight on to decrement_byte_conditionally.
- `jump_to_compare_and_branch`: Jump stub: passes control straight on to compare_and_branch.
- `compare_and_branch_2_2`: Compares a byte from offset 13 of a6 with a byte at offset 24 of a5; if the byte in a6 is smaller, it copies a 16-bit value from offset 14 of a6 to offset 16 of a5, otherwise it increments a 16-bit counter at offset 16 of a5, then fetches a 32-bit value from a table using the counter and a pointer in a5 and calls another routine using a value from that table.
- `jump_to_decrement_byte_conditionally_2`: Jump stub: passes control straight on to decrement_byte_conditionally.
- `update_interpolated_entity_state`: Calculates an interpolated value using a table lookup and scaling, then updates several fields in a structure including counters, timers, and state flags.
- `jump_to_process_indirect_call`: Jump stub: passes control straight on to process_indirect_call.
- `jump_to_offset_2`: Performs a relative branch to a specific offset.
- `jump_to_update_entity_state_and_interp_2`: Jump stub: passes control straight on to update_entity_state_and_interpolate.
- `jump_to_update_entity_state_and_interp_3`: Jump stub: passes control straight on to update_entity_state_and_interpolate.
- `jump_to_update_entity_state_and_interp_4`: Jump stub: passes control straight on to update_entity_state_and_interpolate.
- `jump_to_update_entity_state_and_interp_5`: Jump stub: passes control straight on to update_entity_state_and_interpolate.
- `update_entity_state_and_interpolate`: Updates various counters, timers, and state flags within a data structure, including decrementing countdowns and toggling direction.
- `update_entity_data`: Copies bytes and words between structures, increments a value, and performs an indirect call based on a table index.
- `update_position_and_index`: Adds a 16-bit value from offset 14 of a6 to a 16-bit value at offset 52 of a5, stores the result in offset 52 of a5 and offset 4 of a4, increments a 16-bit value at offset 16 of a5, and branches to another routine.
- `update_entity_physics_and_state`: Updates an entity's internal timers, direction flags, and position using fixed-point arithmetic.
- `decrement_and_check_counter`: Decrements a value stored in a table and checks if it has reached zero, potentially resetting a byte or updating a global variable.
- `compare_and_branch`: Compares a byte from a table to a value, conditionally branches, and moves a word to a destination if the condition is met.
- `compare_and_branch_2`: Compares a byte from offset 13 of a6 with a byte at offset 24 of a5, and if the comparison does not carry, stores a 16-bit value from offset 14 of a6 into offset 16 of a5 before branching to another routine.
- `decrement_byte_conditionally`: Checks a byte in a structure and decrements it if it is not zero or not equal to a specific value before branching.
- `store_and_increment`: Stores a byte into a structure, increments a word in that structure, and branches to another location.
- `decrement_and_store`: Decrements a byte from a table, stores the result into a structure, moves a word to that structure, and branches to another location.
- `interpolate_and_call`: Updates a structure's index and byte values based on multiple conditional checks before performing an indirect call.
- `calculate_interpolated_value`: Calculates a sum from multiple values in a table and stores the result in a data structure.
- `clear_and_load_registers_2`: Compares a byte to a value, moves bytes into registers, clears a word in a structure, and loads an address into a register.
- `update_table_index_2`: Increments a counter in a data structure and performs an indirect call using a value retrieved from a table.
- `update_sprite_attribute_bits`: Reads a byte from an offset of 5 in a structure pointed to by a5 and stores it at offset 12 of a structure in a6, then reads a byte from offset 9 of the a5 structure, shifts it left by 4 bits, and performs a bitwise OR with the byte at offset 14 of the a6 structure.
- `calculate_interpolated_entity_value`: Calculates a value by indexing into a table using a byte from a structure in a5 and a byte from a6, scales this value by a factor derived from a 16-bit integer in a5, and stores the result in a 16-bit field at offset 40 of the structure in a5 before calling update_entity_state_and_interpolate.
- `update_conditional_index`: Updates a counter and conditionally stores a value from a table before performing an indirect call via a table index.
- `update_entity_state_and_interpolate_3`: Updates an entity's state by copying bytes from a source structure in a6 to a destination structure in a5, increments a counter at offset 16 of the destination structure, and performs an indirect call via a table lookup using a value retrieved from a pointer at offset 12 of the source structure.
- `reset_entity_and_call_indirect`: Resets several fields in a data structure, increments a counter, and performs an indirect call using a value retrieved from a table.
- `store_calculated_pointer`: Stores values into a data structure and performs an indirect call using a byte retrieved from a table.
- `update_entity_interpolation_state`: Updates an entity's pointer and frame counter, then calculates a new pointer and frame value based on an offset and a bit-shifted value from a secondary structure, before calling update_entity_state_and_interpolate or update_entity_state_and_interpolate_2.
- `process_indirect_call`: Updates a value in a table, increments a counter, and performs an indirect call using a value retrieved from a pointer.
- `update_sprite_properties`: Updates a sprite or object structure in a table based on a decoded command or value in a register.
- `compare_and_set_flags`: Compares a byte value against a stored value and updates several global variables based on the comparison results.
- `clear_and_copy_bytes`: Copies a byte from one memory location to another, clears a byte at a specific offset, and sets a value through a pointer to zero.
- `load_address_to_a0`: Loads a long value into a0 and returns immediately.
- `update_table_index_3`: Sets a byte in a structure to one, calculates a new pointer using a table lookup based on a bit-shifted value from d0, and stores this pointer in a structure, then clears a long value and sets a byte in a structure before returning.
- `update_state_structure`: Sets a byte to one in a global variable, stores a word from d0 and a long from a1 into a structure pointed to by a5, clears a long at offset 132 of that structure, and stores a byte from d0 at offset 102 of that structure before returning.
- `check_table_value_2`: Saves registers to the stack, loads two pointers from a table, moves a word from d0 to d2, and checks if a longword at offset 464 of the first pointer is zero; if it is not zero, it branches to another routine, otherwise it calculates a new pointer by adding the second pointer to an offset of the first pointer and branches to another routine.
- `check_table_value`: Loads a pointer from a global variable into a6, shifts d2 left by 3, compares a byte at the offset of a5 plus d2 to 251, and if not equal, loads a word from a5 plus d2 into d0, calls another routine, and branches to a final offset.
- `update_object_attribute_table`: Checks a condition based on a value in a table and a sign bit, then updates a structure's pointer, byte, and word values using data from a source list.
- `clear_memory_fields`: Sets a byte to zero and then sets a longword at a specific offset to zero.
- `reset_system_state`: Clears several global variables and hardware registers including audio volumes, DMA control, and interrupt settings.
- `set_global_and_call`: Sets a global variable to the result of a bitwise OR between an input register and a constant, then calls another routine.
- `update_status_and_memory`: Updates several global variables from a data table, performs a loop to clear a block of memory, and modifies a global variable via a bitwise OR.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system`: Initializes system resources by calling a library function twice with different parameters, then writes specific values to hardware registers and memory locations before jumping to a fixed address.
- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
