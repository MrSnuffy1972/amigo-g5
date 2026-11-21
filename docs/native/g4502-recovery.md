# Game 4502 (navy seals (1990)(ocean)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04502). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (917744 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 8 of 8 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 79 of 79 named (0 author symbols), 79 of 79 described (41 by rule, 34 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 614 of 614 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2048 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1544 of 1547 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 17683 |
| code | 10014 |
| data | 884399 |
| padding | 16 |
| unused | 5632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Navy Seals (1990)(Ocean)[cr QTX].adf:bootblock | 1 | 40 | 1 |
| logged disk load from Navy Seals (1990)(Ocean)[cr QTX].adf boot-block load | 1 | 94 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 769 | 41 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 551 | 34 |
| Navy Seals (1990)(Ocean)[cr QTX].adf bytes 709120-709632 (code never loaded by a reference run) | 1 | 0 | 0 |
| Navy Seals (1990)(Ocean)[cr QTX].adf bytes 712192-712704 (code never loaded by a reference run) | 1 | 93 | 1 |
| Navy Seals (1990)(Ocean)[cr QTX].adf bytes 719872-720384 (code never loaded by a reference run) | 1 | 0 | 0 |
| Navy Seals (1990)(Ocean)[cr QTX].adf bytes 721408-721920 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: custom-format (non-DOS) disk, 1740 non-empty blocks: matched against track-loaded segments
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 274 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 340 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: bytes 709120-709632 look like code no run loaded: recovered position-independently (load address unknown)
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: bytes 712192-712704 look like code no run loaded: recovered position-independently (load address unknown)
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: bytes 719872-720384 look like code no run loaded: recovered position-independently (load address unknown)
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: bytes 721408-721920 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: 5 logged loads cover 7680 bytes of the image
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: 22 track-loaded blocks found in recovered runtime segments and classified through them
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: blocks 22-551 (bytes 11264-282624) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.78)
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: blocks 555-682 (bytes 284160-349696) outside the filesystem hold data, not code (opcode density 0.007, entropy 7.03)
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: blocks 684-1383 (bytes 350208-708608) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.36)
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: blocks 1395-1395 (bytes 714240-714752) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.51)
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: blocks 1400-1401 (bytes 716800-717824) outside the filesystem hold data, not code (opcode density 0.014, entropy 3.05)
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: blocks 1403-1404 (bytes 718336-719360) outside the filesystem hold data, not code (opcode density 0.059, entropy 5.15)
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: blocks 1411-1451 (bytes 722432-743424) outside the filesystem hold data, not code (opcode density 0.009, entropy 4.12)
- navy seals (1990)(ocean)[cr qtx]/Navy Seals (1990)(Ocean)[cr QTX].adf: blocks 1463-1759 (bytes 749056-901120) outside the filesystem hold data, not code (opcode density 0.012, entropy 7.19)
- 3 never-loaded region(s), 1536 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 4608 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Navy Seals (1990)(Ocean)[cr QTX].adf:bootblock

- `initialize_display`: Sets up display hardware registers and allocates memory for graphics operations, then performs a series of I/O operations to initialize the video subsystem before entering a loop that calls a dynamically loaded function.

### logged disk load from Navy Seals (1990)(Ocean)[cr QTX].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_rts`: Short code fragment that copies d3 to the memory at an offset from a3, then returns.
- `lookup_table_access`: Performs indexed table lookups using bit-shifted indices and multipliers to access data in a large memory region, then reads two bytes from the calculated addresses and returns them in separate registers.
- `frag_move_andi_add_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with 255, then adds a fixed memory address to d0, then copies d0 to a fixed memory address, then returns.
- `process_input_state_2`: Reads a global variable, conditionally calls another routine, and updates global variables based on input state.
- `conditional_jump_dispatch`: Reads a word value from memory, compares it with eight, branches if greater, checks another condition, and if both conditions are met, calls another routine and performs a jump through an indirect table based on a calculated address.
- `initialize_object_entries`: Initializes object entries in a table with coordinates and flags, then processes a data stream to populate entry fields based on control bytes.
- `conditional_arithmetic`: Checks a global variable for zero and branches based on the result, then performs arithmetic operations on coordinates and table indices to calculate values used in a lookup operation.
- `handle_input_condition`: Reads a value from another routine, checks conditions against global variables, and returns the original value after updating several global variables with specific constants.
- `memory_copy_loop`: Calculates memory addresses using bit shifts and multipliers, stores a calculated pointer in a global variable, then copies data from one memory region to another using a loop that processes four bytes per iteration.
- `frag_btst_tst_bne_move_add_cmpi_ble_rts`: Short code fragment that tests bit d0 of d1, then tests a fixed memory address, then branches if the last result was not zero, then copies a fixed memory address to d4, then adds d3 to d4, then compares d4 with 204, then branches if the comparison was less or equal, then copies 204 to a fixed memory address, then returns.
- `frag_move_rts_2`: Short code fragment that copies d4 to a fixed memory address, then returns.
- `frag_move_sub_cmpi_bge_rts`: Short code fragment that copies a fixed memory address to d4, then subtracts d3 from d4, then compares d4 with 76, then branches if the comparison was greater or equal, then copies 76 to a fixed memory address, then returns.
- `frag_move_rts_3`: Short code fragment that copies d4 to a fixed memory address, then returns.
- `update_object_positions`: Updates object position coordinates based on comparison results and boundary checks, modifying values in global variables and setting flags accordingly.
- `input_processing`: Processes input values through multiple conditional branches based on comparison results, updates several global variables with calculated values, and sets up parameters for subsequent operations based on input conditions.
- `decode_joystick_input`: Reads joystick movement data from a hardware register, processes bitwise flags to determine directional input, combines this with fire button status from another hardware register, and stores the combined result in a global variable.
- `keyboard_input_handler`: Reads keyboard serial data from a hardware register, processes the input through bitwise operations, writes results to a global variable array, and manages interrupt flags during the input handling process.
- `update_animation_state`: Updates an animation counter and manages state transitions based on the counter's value, including resetting and setting flags.
- `update_global_counter`: Reads a word value from memory address 2313 relative to a1, clears a global word variable at address 44258, compares the read value with the cleared variable, and branches to the end if the read value is greater than or equal to zero.
- `update_object_position`: Reads object coordinates and a global counter, updates the counter by XORing with 4, and jumps to a function pointer in a table using the updated counter as an offset.
- `frag_bls`: Short code fragment that branches if the comparison was lower or same (unsigned).
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `frag_addi_bra`: Short code fragment that adds 9 to d3, then jumps to another point in the code.
- `frag_subq_bcc_moveq_bra`: Short code fragment that subtracts 2 from d3, then branches if the carry flag is clear, then loads 0 into d3, then jumps to another point in the code.
- `frag_addi_bra_2`: Short code fragment that adds 18 to d2, then jumps to another point in the code.
- `validate_object_range`: Validates object position ranges against boundary values, updates counters, and advances through a data structure using conditional jumps and comparisons.
- `validate_object_fields`: Checks object boundaries against threshold values and advances through a data structure, returning when a counter reaches zero or conditions are met.
- `initialize_object_fields`: Processes a data structure by incrementing a counter, setting various fields including a pointer, adding byte values to accumulators, shifting results left by four bits, storing the results in specific offsets, and updating a global counter before returning.
- `frag_movea_move_clr_lea_dbra_rts`: Short code fragment that copies a fixed memory address to a0, then copies a fixed memory address to d7, then clears the memory at an offset from a0, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d7, then returns.
- `validate_object_bounds`: Validates object boundaries by performing bit shifts and arithmetic operations on coordinates, then calls another routine for further processing.
- `frag_bsr_bra`: Short code fragment that calls another routine (2 times in a row), then jumps to another point in the code.
- `frag_move_andi_cmpi_bne_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with 65280, then compares d0 with 64256, then branches if the last result was not zero, then returns.
- `frag_move_movem_jsr_addq_rte`: Short code fragment that copies 32 to a fixed memory address, then saves registers on the stack, then calls another routine, then restores saved registers from the stack, then adds 1 to a fixed memory address, then returns from the exception.
- `handle_input_events`: Processes input events from a buffer, updates state variables based on character comparisons, and manages keyboard input handling with conditional branches.
- `update_timer_values`: Updates timer-related global variables by decrementing a counter, setting new values based on arithmetic operations, and calling a handler routine.
- `frag_cmpi_bge_bne_move_bra`: Short code fragment that compares d0 with 65, then branches if the comparison was greater or equal, then compares d0 with 64, then branches if the last result was not zero, then copies 32 to d0, then jumps to another point in the code.
- `process_character_data`: Processes character input by reading from a global table, updating pointers and buffer contents based on comparison results and conditional flags.
- `process_input_state`: Updates input-related global variables based on character data processing results and conditional checks, then sets up state for subsequent operations.
- `frag_cmpi_bne_tst_beq_subq_rts`: Short code fragment that compares a fixed memory address with 2, then branches if the last result was not zero, then tests a fixed memory address, then branches if the last result was zero, then subtracts 1 from a fixed memory address, then returns.
- `update_global_and_check_table`: Updates a global variable and checks values in a table based on the updated value, calling another routine if the check fails or if a table entry is non-zero.
- `update_global_counters`: Updates global counter values by adding operands to registers and storing the results back to memory locations in sequence.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `reset_audio_channel_state`: Saves registers to stack, clears two shared state words, calls a routine four times sequentially, clears a shared flag word, then restores registers from stack and returns.
- `reset_channel_record_fields`: Clears one word of a record, sets a second word to one and a third to zero, then clears a word at a second address.
- `frag_movem_subq_bmi_cmp_bcs_bsr_bra`: Short code fragment that saves registers on the stack, then subtracts 1 from d0, then branches if the last result was negative, then compares d0 with 25, then branches if the carry flag is set, then calls another routine, then jumps to another point in the code.
- `load_audio_volumes`: Loads audio volume values from a table into audio hardware registers, iterating four times while incrementing a pointer.
- `frag_bsr_dbra_movem_rts`: Short code fragment that calls another routine, then loops back while counting down d7, then restores saved registers from the stack, then returns.
- `frag_tst_beq_cmp_bcs_movem_move_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then compares d0 with a fixed memory address, then branches if the carry flag is set, then saves registers on the stack, then copies 16384 to d1, then copies d0 to a fixed memory address, then jumps to another point in the code.
- `clear_audio_register`: Clears a hardware register word and sets a flag in another global variable, then processes input data to initialize audio channel records.
- `update_input_state`: Reads a byte from a buffer, processes it based on flags in a global variable, updates several game state variables including positions and counters, and performs conditional calculations using table lookups.
- `process_input_buffer`: Reads bytes from a buffer, processes them based on flags, updates game state variables including positions and counters, and handles input state transitions.
- `frag_move_bra`: Short code fragment that copies a3 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bset_move_bra`: Short code fragment that sets bit 5 of d7, then copies the memory at a3 (advancing a3) to the memory at an offset from a4, then jumps to another point in the code.
- `frag_bclr_bra`: Short code fragment that clears bit 5 of d7, then jumps to another point in the code.
- `frag_addi_bra_3`: Short code fragment that adds 256 to the memory at an offset from a4, then jumps to another point in the code.
- `frag_bclr_bset_clr_bra`: Short code fragment that clears bit 4 of d7, then sets bit 7 of d7, then clears the memory at an offset from a4, then jumps to another point in the code.
- `frag_bset_move_bra_2`: Short code fragment that sets bit 3 of d7, then copies the memory at a3 (advancing a3) to the memory at an offset from a4 (2 times in a row), then jumps to another point in the code.
- `frag_andi_bset_move_bra`: Short code fragment that masks d7 with 65528, then sets bit 0 of d7, then copies the memory at a3 (advancing a3) to the memory at an offset from a4 (2 times in a row), then jumps to another point in the code.
- `frag_bclr_bra_2`: Short code fragment that clears bit 0 of d7, then jumps to another point in the code.
- `prepare_input_buffer`: Reads a byte from a buffer through a pointer, uses it as an offset to calculate a memory address, then stores several bytes and a long word from that calculated address into a structure at a fixed offset from another pointer.
- `frag_bclr_bra_3`: Short code fragment that clears bit 1 of d7, then jumps to another point in the code.
- `frag_andi_bset_move_bra_2`: Short code fragment that masks d7 with 65528, then sets bit 2 of d7, then copies the memory at a3 (advancing a3) to the memory at an offset from a4 (3 times in a row), then jumps to another point in the code.
- `frag_bclr_bra_4`: Short code fragment that clears bit 2 of d7, then jumps to another point in the code.
- `frag_lea_moveq_move_add_adda_bra`: Short code fragment that loads the address of a fixed memory address into a0, then loads 0 into d0, then copies the memory at a3 (advancing a3) to d0, then adds d0 to d0, then adds d0 to a0, then adds the memory at a0 to a0, then copies a0 to the memory at an offset from a4, then jumps to another point in the code.
- `update_game_state_2`: Calculates a memory address using a byte from a buffer, shifts it left by four bits, then reads several words and long words from that address and stores them into specific offsets of a structure pointed to by another register.
- `frag_bset_bra`: Short code fragment that sets bit 1 of a fixed memory address, then jumps to another point in the code.
- `update_game_state`: Updates various game state variables including counters, accumulators, and memory locations using arithmetic operations, bit tests, and conditional branches based on joystick input.
- `start_audio_channels_from_flags`: Sets up audio channel records from a shared channel-flag value: for each set flag bit it resets that channel's volume and pointer fields.

### Navy Seals (1990)(Ocean)[cr QTX].adf bytes 712192-712704 (code never loaded by a reference run)

- `process_data_table`: Calculates memory addresses using multiple arithmetic operations on global variables, loads data from a calculated source into a destination table, processes three bytes per iteration, and repeats until all entries are processed.
