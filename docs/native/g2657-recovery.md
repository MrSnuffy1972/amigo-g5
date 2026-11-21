# Game 2657 (alien target (1994)(infinite dreams)(pl)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02657). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703360 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 64 of 64 named (0 author symbols), 64 of 64 described (27 by rule, 34 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 12800 bytes, 53 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2011 of 2011 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 7506 |
| data | 2584232 |
| padding | 6 |
| unused | 111616 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf:bootblock | 1 | 87 | 10 |
| Alien Target (1994)(Infinite Dreams)(Pl)(Disk 2 of 3).adf:bootblock | 1 | 18 | 1 |
| Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf bytes 1024-13824 (code never loaded by a reference run) | 1 | 1906 | 53 |

## Cases handled

- alien target (1994)(infinite dreams)(pl)(disk 1 of 3)/Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf: custom-format (non-DOS) disk, 1651 non-empty blocks: matched against track-loaded segments
- alien target (1994)(infinite dreams)(pl)(disk 2 of 3)/Alien Target (1994)(Infinite Dreams)(Pl)(Disk 2 of 3).adf: custom-format (non-DOS) disk, 1740 non-empty blocks: matched against track-loaded segments
- alien target (1994)(infinite dreams)(pl)(disk 3 of 3)/Alien Target (1994)(Infinite Dreams)(Pl)(Disk 3 of 3).adf: custom-format (non-DOS) disk, 1665 non-empty blocks: matched against track-loaded segments
- alien target (1994)(infinite dreams)(pl)(disk 1 of 3)/Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf: bytes 1024-13824 look like code no run loaded: recovered position-independently (load address unknown)
- alien target (1994)(infinite dreams)(pl)(disk 1 of 3)/Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf: 1 logged loads cover 12800 bytes of the image
- alien target (1994)(infinite dreams)(pl)(disk 1 of 3)/Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf: blocks 47-55 (bytes 24064-28672) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.80)
- alien target (1994)(infinite dreams)(pl)(disk 1 of 3)/Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf: blocks 1694-1694 (bytes 867328-867840) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.82)
- 6 packed track region(s), 2570752 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_video_dma` (Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf bytes 1024-13824 (code never loaded by a reference run)): Sets up video hardware registers including interrupt and DMA control, configures sprite bits, and manages a loop that updates specific memory locations with word values based on a counter. Kind: disk track check. Pass path: the routine proceeds through all iterations (sVar2 from 3 to 6) without breaking early, and the final branch after sVar2 == 7 returns normally.
- `process_screen_coordinates` (Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf bytes 1024-13824 (code never loaded by a reference run)): Calculates screen position values from input parameters, initializes video hardware registers, and manages a complex loop that updates memory locations and calls multiple routines based on conditional checks. Kind: disk track check. Pass path: the branch where tst.l d0; bne.w/bne.b is NOT taken (i.e., d0 == 0), allowing execution to proceed past the bne instructions and into the main logic of process_screen_coordinates.
- `handle_input_and_render` (Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf bytes 1024-13824 (code never loaded by a reference run)): Processes joystick input by testing specific bits in hardware registers, manages rendering operations through multiple routine calls, and updates memory locations with calculated values while handling loop iterations. Kind: disk track check. Pass path: The routine proceeds to load data and initialize game state if the disk check passes (i.e., when `tst.l d0` after `bsr.w an address` is zero, indicating successful validation)..

## Functions


### Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf:bootblock

- `setup_boot_interrupts`: Initializes CIA timer registers and control bits, sets up interrupt vectors, copies initialization data from one memory location to another using a loop with decrementing counter, then jumps to the copied code.
- `setup_interrupts_and_dma`: Initializes interrupt and DMA settings by enabling specific bits in control registers, sets up an interrupt vector, and executes a trap instruction.
- `frag_movea_adda_suba_move_bsr_jmp`: Short code fragment that copies a location in the code to a0, then adds an address to a0, then copies a0 to a7, then subtracts 1024 from a0, then copies a0 to the user stack pointer, then copies 0 to the status register, then calls another routine, then copies a location in the code to a fixed memory address, then jumps to another point in the code.
- `frag_move_addq_bra`: Short code fragment that copies 0 to d0, then copies d0 to a fixed memory address, then adds 1 to d0, then jumps to another point in the code.
- `frag_movea_move_jsr_cmpi_bcs_rts`: Short code fragment that copies a fixed memory address to a6, then copies 65536 to d0, then copies 65538 to d1, then calls another routine, then compares d0 with 28672, then branches if the carry flag is set, then returns.
- `frag_movea_lea_move_cmpi_bne_rts`: Short code fragment that copies a fixed memory address to a6, then loads the address of a location in the code into a0, then copies the memory at an offset from a6 to d0, then compares d0 with an address, then branches if the last result was not zero, then copies the memory at an offset from a6 to d0, then branches if the last result was not zero, then copies 0 to the memory at a0, then returns.
- `frag_subi_move_rts`: Short code fragment that subtracts an address from d0, then copies d0 to the memory at a0, then returns.
- `wait_for_vpos_bit_low_then_high`: Polls bit zero of the custom chip register at offset five until it reads low, then polls again until it reads high, then returns.
- `frag_lea_move_dbra_rts`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then copies 7167 to d7, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d7, then returns.
- `initialize_io_request`: Sets up an I/O request structure with command 2, buffer address, and transfer parameters, then executes the I/O operation.

### Alien Target (1994)(Infinite Dreams)(Pl)(Disk 2 of 3).adf:bootblock

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Alien Target (1994)(Infinite Dreams)(Pl)(Disk 1 of 3).adf bytes 1024-13824 (code never loaded by a reference run)

- `initialize_game_hardware`: Sets up video and DMA hardware registers, configures interrupt controllers, and initializes global variables based on input from the joystick or mouse fire button.
- `keyboard_poll_loop`: Executes a loop that waits for specific vertical blanking intervals, checks for keyboard input from CIAA_ICR, reads from CIAA_SDR, and manipulates CIAA_CRA to handle keyboard serial communication.
- `joystick_input_handler`: Processes different input values by loading addresses from program memory, calling another routine with those addresses, and checking a bit in a hardware register.
- `frag_tst_beq_cmpi_lea_bsr_moveq_btst_bn`: Short code fragment that tests d0, then branches if the last result was zero, then compares d0 with 2, then branches if the last result was zero, then loads the address of a location in the code into a0, then calls another routine, then loads 1 into d0, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then returns.
- `initialize_hardware`: Sets up video hardware registers by writing specific values to COP1LCH and other control registers, then calls multiple subroutines to process input and screen coordinates before returning.
- `execute_input_sequence`: Executes a series of input processing operations by calling other routines multiple times with different parameters, continuing until each routine returns a non-zero value.
- `setup_dma_interrupts`: Configures DMA and interrupt controller registers with specific bit patterns, executes several input processing routines, and manages game state transitions through multiple conditional branches.
- `manage_game_state`: Updates global variables with input values, performs conditional checks against memory locations, and manages a loop that waits for specific hardware conditions before setting a coprocessor register.
- `process_display_strings`: Processes multiple string data segments through various routines, waits for vertical synchronization signals, and checks input status before returning.
- `integer_to_string_conversion`: Converts a number into a string representation by dividing it by 10 repeatedly, converting remainders to characters, and storing the result in reverse order in memory.
- `string_processor`: Processes characters in a string by checking for null terminators or specific character values, modifying the string content, and advancing through the string until termination conditions are met.
- `frag_cmpi_bne_move_rts`: Short code fragment that compares a fixed memory address with 255, then branches if the last result was not zero, then copies 2526 to a fixed memory address, then copies 3664 to a fixed memory address, then copies 0 to a fixed memory address, then returns.
- `initialize_dma_interrupts`: Configures DMA and interrupt controller registers with specific bit patterns, enables vertical blanking interrupts, and manages a loop that waits for specific input conditions before proceeding.
- `frag_lea_bsr_btst_bne_rts`: Short code fragment that loads the address of a location in the code into a0, then calls another routine, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_moveq_move_rts`: Short code fragment that loads 0 into d0, then copies a fixed memory address to d0, then returns.
- `update_display_and_input`: Processes input data to update display registers, handles character encoding and lookup operations, and manages conditional execution paths based on comparison results between memory values.
- `frag_lea_moveq_move_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads 0 into d0, then loads 23 into d7, then copies d0 to the memory at a0 (advancing a0), then loops back while counting down d7, then returns.
- `frag_bsr_move_rts`: Short code fragment that calls another routine, then copies 32 to the memory at an offset from a5, then returns.
- `frag_bsr_move_rts_2`: Short code fragment that calls another routine, then copies 32800 to the memory at an offset from a5, then returns.
- `frag_lea_btst_beq_cmpi_bne_rts`: Short code fragment that loads the address of a fixed memory address into a5, then tests bit 0 of the memory at an offset from a5, then branches if the last result was zero, then compares the memory at an offset from a5 with 54, then branches if the last result was not zero, then returns.
- `decode_compressed_data`: Processes compressed data by shifting bits and accumulating carry flags, then stores decoded bytes into a target buffer while updating pointers and handling conditional logic based on bit operations.
- `shift_and_accumulate`: Shifts a byte value left by one bit position and accumulates the carry flag into the result, repeating this process a specified number of times determined by a counter.
- `process_bit_data`: Processes bit-level data from a buffer, performing shifts, comparisons, and conditional operations while updating internal state variables.
- `process_data_bits`: Handles bit operations similar to routine R9, performing shifts, comparisons, and conditional processing of data from memory locations before returning.
- `frag_lsr_lea_move_dbra_rts`: Short code fragment that shifts d0 right by 4, then loads the address of the memory at an offset from a6 into a6, then copies the memory at a5 (advancing a5) to the memory at a6 (advancing a6) (4 times in a row), then loops back while counting down d0, then returns.
- `frag_move_lea_bsr_movea_rts`: Short code fragment that copies a0 to the stack, then loads the address of a fixed memory address into a5, then calls another routine, then copies the top of the stack to a0, then calls another routine (2 times in a row), then copies 7002 to the memory at an offset from a5, then returns.
- `frag_movea_move_moveq_dbra_rts`: Short code fragment that copies 12838 to a0, then copies 749 to d7, then loads 0 into d0, then copies d0 to the memory at a0 (advancing a0) (4 times in a row), then loops back while counting down d7, then returns.
- `parse_byte_sequence`: Reads bytes from a memory buffer and processes them based on their values, calling other routines conditionally and advancing the buffer pointer until a zero byte is encountered.
- `coprocessor_setup_loop`: Configures a coprocessor by setting up a loop that waits for vertical blanking intervals, updates a global variable with incrementing values, and continues until reaching a specific target value.
- `memory_copy_with_offset`: Copies data from one memory location to another using offset calculations based on input parameters, performing multiple word transfers in a loop while updating pointers and counters.
- `byte_copy_with_offset`: Copies byte data between memory locations with offset calculations, performing multiple byte transfers in a loop while updating pointers and counters based on input parameters.
- `initialize_video_dma`: Sets up video hardware registers including interrupt and DMA control, configures sprite bits, and manages a loop that updates specific memory locations with word values based on a counter.
- `process_screen_coordinates`: Calculates screen position values from input parameters, initializes video hardware registers, and manages a complex loop that updates memory locations and calls multiple routines based on conditional checks.
- `handle_input_and_render`: Processes joystick input by testing specific bits in hardware registers, manages rendering operations through multiple routine calls, and updates memory locations with calculated values while handling loop iterations.
- `data_encoding_loop`: Executes a complex encoding routine that initializes data structures, performs bit manipulation operations, and processes large amounts of data through multiple loops with various arithmetic and logical operations.
- `frag_move_lsr_bsr`: Short code fragment that copies d0 to d3, then shifts d0 right by 1, then calls another routine, then copies d3 to d0, then calls another routine.
- `frag_move_btst_bne_bset_bra_bclr_rts`: Short code fragment that copies the memory at a1 to d0, then tests bit 0 of the memory at an offset from a1, then branches if the last result was not zero, then tests bit 6 of d0, then branches if the last result was not zero, then sets bit 7 of d0, then jumps to another point in the code, then clears bit 7 of d0, then copies d0 to the memory at a1, then returns.
- `encode_clock_bits_longword`: Merges the data bits of one longword with clock bits computed from its neighbouring bits using an alternating-bit mask, clearing the top bit when a flag in the preceding byte is set.
- `xor_accumulate_and_mask`: Loops through a list of words, XORing them into an accumulator a specific number of times based on a register, and returns the result masked with a bit pattern.
- `data_decoding_and_validation`: Processes encoded data by validating specific patterns, performing bitwise operations on data chunks, and checking for zero results after XOR operations before returning a final value.
- `copy_screen_data`: Copies screen data from a source location to a destination buffer, handling multiple rows and blocks with specific addressing offsets.
- `fill_memory_and_set_flags`: Fills memory with a specific value in a loop, updates several global variables with fixed values, waits for a condition in a global variable before setting another flag and returning.
- `initialize_video_parameters`: Sets up video-related registers with specific values, checks a condition based on a global variable, and updates a status flag if the condition is met.
- `frag_btst_bne_rts`: Short code fragment that tests bit 5 of the memory at a4, then branches if the last result was not zero, then returns.
- `manage_sprite_bits`: Sets and clears specific bits in memory locations related to sprite management, then calls another routine and returns based on a bit test result.
- `frag_move_moveq_bset_bclr_rts`: Short code fragment that copies d0 to the stack, then loads 0 into d0, then copies the memory at an offset from a3 to d0, then sets bit d0 of the memory at a5, then sets bit 7 of the memory at a5, then clears bit d0 of the memory at a5, then sets bit d0 of the memory at a5, then copies the top of the stack to d0, then returns.
- `frag_bset_btst_beq_bsr_move_bra_clr_rts`: Short code fragment that sets bit 1 of the memory at a5, then tests bit 4 of the memory at a4, then branches if the last result was zero, then calls another routine, then copies 48 to d7, then calls another routine (2 times in a row), then jumps to another point in the code, then clears the memory at an offset from a3, then returns.
- `update_position_and_call`: Updates position values in memory, calls other routines multiple times, and modifies a value based on comparison results before returning.
- `frag_movem_move_cmp_beq_dbra_rts`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a6 to d6, then compares d6 with the memory at an offset from a6, then branches if the last result was zero, then loops back while counting down d7, then restores saved registers from the stack, then returns.
- `frag_bclr_move_bsr_bset_rts`: Short code fragment that clears bit 0 of the memory at a5, then copies 5 to d7, then calls another routine, then sets bit 0 of the memory at a5, then returns.
- `frag_ori`: Short code fragment that merges 0 into d0, then merges 0 into d5, then merges 0 into d0 (6 times in a row).
- `wait_for_sync_and_lookup`: Waits for vertical synchronization signals from hardware registers, then performs a lookup operation using data from memory and returns a byte value.
- `frag_moveq_move_bset_clr_bclr_rts`: Short code fragment that loads 0 into d0, then copies a fixed memory address to d0, then sets bit 6 of a fixed memory address, then clears a fixed memory address, then copies 255 to a fixed memory address, then clears bit 6 of a fixed memory address, then returns.
