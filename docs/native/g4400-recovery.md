# Game 4400 (MightyBombJack v1.3 0536): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04400). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1647622 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 13 of 13 units identical (bytes and relocations), 23 raw-word lines |
| Functions named and described | pass, 521 of 521 named (0 author symbols), 521 of 521 described (269 by rule, 251 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1012 of 1012 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10704 of 10704 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 410797 |
| code | 39102 |
| data | 1059992 |
| padding | 4867 |
| unused | 132864 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 180 | 19 |
| logged disk load from Disk.1 | 1 | 306 | 21 |
| logged disk load from Disk.1 | 1 | 9879 | 439 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 257 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 61 | 34 |

## Cases handled

- MightyBombJack_v1.3_0536/MightyBombJack_v1.3_0536_hdf/MightyBombJack/Disk.1: raw disk image (718848 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- MightyBombJack_v1.3_0536/MightyBombJack_v1.3_0536.hdf: hard-disk image (AmigaDOS OFS, 1760 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree MightyBombJack_v1.3_0536_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 9 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- MightyBombJack_v1.3_0536/MightyBombJack_v1.3_0536_hdf/MightyBombJack/Disk.1: 9 logged loads cover 442368 bytes of the image
- MightyBombJack_v1.3_0536/MightyBombJack_v1.3_0536_hdf/MightyBombJack/Disk.1: chunks 0-3 (bytes 0-1024) outside the filesystem hold data, not code (opcode density 0.049, entropy 4.38)
- MightyBombJack_v1.3_0536/MightyBombJack_v1.3_0536_hdf/MightyBombJack/Disk.1: chunks 1656-1656 (bytes 423936-424192) hold ASCII text
- MightyBombJack_v1.3_0536/MightyBombJack_v1.3_0536_hdf/MightyBombJack/Disk.1: chunks 1680-1759 (bytes 430080-450560) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.27)
- MightyBombJack_v1.3_0536/MightyBombJack_v1.3_0536_hdf/MightyBombJack/Disk.1: chunks 1776-2012 (bytes 454656-515328) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.00)
- MightyBombJack_v1.3_0536/MightyBombJack_v1.3_0536_hdf/MightyBombJack/Disk.1: chunks 2304-2367 (bytes 589824-606208) outside the filesystem hold data, not code (opcode density 0.011, entropy 5.33)
- 1 packed track region(s), 44800 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `frag_clr_lea_move_rts` (Disk.1 +an address loaded at an address (6144 bytes)): Short code fragment that clears a fixed memory address, then loads the address of a fixed memory address into a5, then copies 38144 to the memory at an offset from a5, then copies 25088 to the memory at an offset from a5, then copies 37154 to the memory at an offset from a5, then returns. Kind: disk hardware sync check. Pass path: The routine sets up ADKCON and DSKSYNC registers, likely checking for a specific disk sync pattern. If the check passes (i.e., the expected sync value is detected), execution continues normally; if not, it may jump to a different path or tr.
- `configure_io_ports` (Disk.1 +an address loaded at an address (6144 bytes)): Sets up CIA port configurations, manipulates bit patterns in memory, and controls joystick/mouse input handling with a delay loop. Kind: disk hardware check. Pass path: CIAB_PRB is manipulated to check for specific hardware state (e.g., CIA port bits), and the routine waits for a condition via sub_0226() — likely checking for a particular disk drive or hardware response. The check passes if sub_0226() retu.
- `frag_move_ori_tst_beq_bclr_rts` (Disk.1 +an address loaded at an address (6144 bytes)): Short code fragment that copies a fixed memory address to d2, then merges 4 into d2, then tests d0, then branches if the last result was zero, then clears bit 2 of d2, then copies d2 to a fixed memory address, then returns. Kind: disk hardware check. Pass path: branch taken when in_D0w == 0 (the bclr operation is executed).
- `control_port_bits` (Disk.1 +an address loaded at an address (6144 bytes)): Checks a hardware status bit, modifies port values through bit set/clear operations, and executes a delay loop based on a counter. Kind: disk hardware check. Pass path: The loop continues until bit 4 of CIAA_PRA is clear, then _DAT_00001020 is set to 0 and the function returns. A successful check would proceed past the initial bit check and execute the return..
- `update_game_state_2` (Disk.1 +an address loaded at an address (6144 bytes)): Updates a global variable based on input and hardware register values, performs bitwise operations on memory locations, and handles conditional execution paths for positive and negative differences. Kind: disk hardware check. Pass path: The routine updates game state based on CIAB_PRB changes and performs bit manipulation on hardware registers. A successful check would proceed through the 'addi.w #an address, an address.w' and 'subi.w #an address, an address.w' instructions after the bsr.w an address cal.
- `shift_and_loop` (Disk.1 +an address loaded at an address (6144 bytes)): Performs bitwise arithmetic operations on a register value, manipulates memory bits in a loop, and decrements a counter until zero. Kind: disk hardware check. Pass path: The routine completes successfully without triggering any abort conditions, and the final branch after the loop (when sVar1 == -1) allows execution to continue normally..
- `modify_port_bits` (Disk.1 +an address loaded at an address (6144 bytes)): Modifies bit patterns in a hardware port register, performs bit rotation calculations, and conditionally clears specific bits based on input. Kind: disk hardware check. Pass path: The routine modifies CIAB_PRB to set certain bits and then checks conditions based on input registers. A successful check would proceed with the normal execution path (e.g., the branch after the call where CIAB_PRB is written back without t.
- `initialize_disk_dma` (Disk.1 +an address loaded at an address (6144 bytes)): Sets up disk DMA registers with specific values, clears interrupt request flags, and waits for a particular interrupt status before proceeding. Kind: disk track sync/check. Pass path: The routine continues to the next iteration after successful sync check, with _DAT_00001020 = uVar1 and then calls u1__sub_0330()..
- `frag_bsr_clr_lea_move_rts` (Disk.1 +an address loaded at an address (61440 bytes)): Short code fragment that calls another routine (2 times in a row), then clears a fixed memory address, then loads the address of a fixed memory address into a5, then copies 38144 to the memory at an offset from a5, then copies 25088 to the memory at an offset from a5, then copies 37154 to the memory at an offset from a5, then calls another routine, then returns. Kind: disk hardware sync/check. Pass path: The routine sets up ADKCON and DSKSYNC registers, then calls sub_3DEC which likely performs a disk sync or track validation check. If the check passes (no protection violation), execution continues normally to sub_3EF8 and returns. The bsr..
- `configure_io_ports_2` (Disk.1 +an address loaded at an address (61440 bytes)): Configures CIA port registers for joystick input handling, performs bit manipulation on input data, and calls another routine with a delay. Kind: disk hardware check. Pass path: CIAB_PRB is set to bVar2 (which has been modified by the bitwise operations), and u2__sub_40EE() is called, indicating a successful validation path..
- `frag_move_ori_tst_beq_bclr_rts_2` (Disk.1 +an address loaded at an address (61440 bytes)): Short code fragment that copies a fixed memory address to d2, then merges 4 into d2, then tests d0, then branches if the last result was zero, then clears bit 2 of d2, then copies d2 to a fixed memory address, then returns. Kind: disk hardware check. Pass path: branch taken when in_D0w == 0 (the bclr operation clears bit 2 of CIAB_PRB).
- `update_joystick_input` (Disk.1 +an address loaded at an address (61440 bytes)): Reads the joystick fire button status from CIAA_PRA and updates the game's input state in a global variable, then calls another routine while the button is pressed. Kind: disk hardware check. Pass path: The routine clears an address.w and sets up a CIA register access sequence; successful check likely returns via RTS after setting _DAT_0000040a = 0.
- `process_movement_delta` (Disk.1 +an address loaded at an address (61440 bytes)): Calculates movement differences between current and previous positions, updates a global variable based on direction, and calls another routine multiple times while adjusting the global variable value. Kind: disk hardware check. Pass path: The routine modifies CIAB_PRB to simulate movement and checks for changes in the hardware state, indicating a disk or track integrity check..
- `modify_output_bits` (Disk.1 +an address loaded at an address (61440 bytes)): Reads a byte from memory, modifies its bits using bitwise operations with a rotated value, and writes the result back to memory while conditionally clearing one bit. Kind: disk hardware check. Pass path: CIAB_PRB is modified and returned as in_D0w + 3U.
- `handle_disk_operations` (Disk.1 +an address loaded at an address (61440 bytes)): Handles disk operations including reading file headers, validating format signatures, and processing data blocks while managing hardware registers. Kind: disk track/checksum verification. Pass path: if (uVar1 == _DAT_0000040a) break; - continues to next iteration, successful check leads to incrementing _DAT_0000040e.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `initialize_game_state`: Initializes global variables and hardware registers including DMA control, interrupt enable, and bitplane configuration.
- `frag_movem_move_movea_bsr_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d7, then copies a fixed memory address to d6, then copies a fixed memory address to a5, then calls another routine, then restores saved registers from the stack, then returns.
- `initialize_game_state_2`: Initializes global variables and hardware registers, sets up display and color palettes, and configures interrupt settings.
- `frag_eori_bra`: Short code fragment that exclusive-ors 25642 into a fixed memory address (2 times in a row), then exclusive-ors an address into a fixed memory address, then exclusive-ors an address into a fixed memory address, then jumps to another point in the code.
- `frag_move_tst_bne_lea_bsr_movea_jsr_mov`: Short code fragment that copies a location in the code to d0, then tests d0, then branches if the last result was not zero, then loads the address of a location in the code into a0, then calls another routine, then copies a location in the code to a2, then calls another routine (2 times in a row), then restores saved registers from the stack, then returns.
- `frag_lea_moveq_not_move_eor_subq_bne_su`: Short code fragment that loads the address of a fixed memory address into a1, then loads 45 into d0, then inverts d0, then copies d0 to the stack, then exclusive-ors d0 into the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then copies the top of the stack to d0, then subtracts d0 from a1, then returns.
- `initialize_and_call`: Initializes multiple registers and calls a function through a pointer in a global variable, then restores all registers and returns.
- `frag_bsr_jmp`: Short code fragment that calls another routine, then jumps to another point in the code.
- `patch_jump_vectors`: Iterates through a table of pointers, writes jump instructions to memory locations, and sets function pointers at specific offsets.
- `frag_tst_btst_bne_move_rts`: Short code fragment that tests a fixed memory address, then tests bit 6 of the memory at an offset from a0, then branches if the last result was not zero, then copies 1025 to the memory at an offset from a0, then returns.
- `frag_tst_btst_bne_move_rts_2`: Short code fragment that tests a fixed memory address, then tests bit 6 of the memory at an offset from a0, then branches if the last result was not zero, then copies 1026 to the memory at an offset from a0, then returns.
- `frag_move_tst_btst_bne_rts`: Short code fragment that copies a2 to the memory at an offset from a0, then tests a fixed memory address, then tests bit 6 of the memory at an offset from a0, then branches if the last result was not zero, then copies d5 to the memory at an offset from a0, then returns.
- `calculate_coordinates`: Performs arithmetic operations on register values to calculate coordinate positions and calls another routine with the results.
- `frag_moveq_move_cmp_beq_dbra_rts`: Short code fragment that loads 2 into d1, then copies a fixed memory address to d2, then compares d2 with a fixed memory address, then branches if the last result was zero, then loops back while counting down d1, then returns.
- `frag_cmp_beq_cmpi_bne_bsr_andi_ext_rts`: Short code fragment that compares d0 with a location in the code, then branches if the last result was zero, then compares d0 with 95, then branches if the last result was not zero, then calls another routine, then masks d0 with 127, then sign-extends d0, then returns.
- `frag_suba_movea_jmp`: Short code fragment that subtracts a1 from a1, then copies a location in the code to a2, then jumps to another point in the code.
- `frag_moveq_movea_jmp`: Short code fragment that loads 1 into d2, then copies a location in the code to a2, then jumps to another point in the code.
- `frag_pea_bra_move_addq_rts`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code, then pushes the address of a fixed memory address onto the stack, then copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.

### logged disk load from Disk.1

- `initialize_hardware_2`: Sets up custom hardware registers including DMA control, interrupt enable, and video display settings with specific bit patterns and values.
- `initialize_hardware`: Initializes custom hardware registers including DMA control, interrupt enable, and video display settings.
- `handle_input_processing`: Saves all registers to the stack, reads values from hardware registers at fixed offsets, calls several routines with different parameters, updates counter variables in memory, and restores registers before returning.
- `update_input_state_2`: Reads input port values, updates counters in global variables, and calls routines to process input data before returning.
- `frag_movem_move_bsr_addi_subi_bne_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then calls another routine (2 times in a row), then adds 1 to a fixed memory address, then subtracts 1 from a fixed memory address, then copies a fixed memory address to d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `update_video_registers`: Tests hardware status flags, updates video register values, and manages timing counters for display synchronization.
- `frag_move_bset_movem_rte`: Short code fragment that copies a fixed memory address to a fixed memory address, then sets bit 0 of a fixed memory address, then copies 32 to a fixed memory address, then restores saved registers from the stack, then returns from the exception.
- `frag_bclr_beq_rts`: Short code fragment that clears bit 0 of a fixed memory address (2 times in a row), then branches if the last result was zero, then returns.
- `frag_clr_lea_move_rts`: Short code fragment that clears a fixed memory address, then loads the address of a fixed memory address into a5, then copies 38144 to the memory at an offset from a5, then copies 25088 to the memory at an offset from a5, then copies 37154 to the memory at an offset from a5, then returns.
- `frag_move_bsr_rts`: Short code fragment that copies 0 to d0, then copies 0 to d1, then calls another routine, then returns.
- `frag_move_bsr_rts_2`: Short code fragment that copies 0 to d0, then copies 0 to d1, then calls another routine, then returns.
- `configure_io_ports`: Sets up CIA port configurations, manipulates bit patterns in memory, and controls joystick/mouse input handling with a delay loop.
- `frag_move_ori_tst_beq_bclr_rts`: Short code fragment that copies a fixed memory address to d2, then merges 4 into d2, then tests d0, then branches if the last result was zero, then clears bit 2 of d2, then copies d2 to a fixed memory address, then returns.
- `control_port_bits`: Checks a hardware status bit, modifies port values through bit set/clear operations, and executes a delay loop based on a counter.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address, then returns.
- `update_game_state_2`: Updates a global variable based on input and hardware register values, performs bitwise operations on memory locations, and handles conditional execution paths for positive and negative differences.
- `shift_and_loop`: Performs bitwise arithmetic operations on a register value, manipulates memory bits in a loop, and decrements a counter until zero.
- `modify_port_bits`: Modifies bit patterns in a hardware port register, performs bit rotation calculations, and conditionally clears specific bits based on input.
- `initialize_disk_dma`: Sets up disk DMA registers with specific values, clears interrupt request flags, and waits for a particular interrupt status before proceeding.
- `update_game_checksum`: Processes data through multiple loops, performs bitwise operations on values, and updates global variables based on comparison results.
- `frag_addi_rts`: Short code fragment that adds 6144 to a fixed memory address, then returns.

### logged disk load from Disk.1

- `enable_dma_and_interrupts`: Sets bits 15 of the DMA control and interrupt enable registers, then loads a pointer into a global variable and triggers a system call before branching to an address.
- `initialize_hardware_2_2`: Initializes custom hardware registers including bitplane and display settings, clears multiple memory regions, and sets up color table entries.
- `update_bomb_jack_state_2`: Updates internal state variables based on input conditions and joystick status, including handling button presses and updating counters.
- `initialize_animation_data`: Sets up animation parameters by loading values from a structure, calling subroutines to process them, and performing conditional checks on specific byte offsets before returning.
- `initialize_game_state_3`: Sets multiple global variables to specific values including zero, negative one, and seven, then calls another routine before entering a loop that waits for vertical blanking and checks for a specific hardware register value.
- `reset_game_variables`: Initializes several global variables to predefined values including ten, zero, and negative one, then calls another routine before entering a loop that waits for vertical blanking and checks for a specific hardware register value.
- `copy_rendering_parameters`: Calculates rendering coordinates based on input values, copies data between memory locations using a loop structure, and updates global variables with calculated results.
- `transfer_pixel_data`: Transfers pixel data from one memory location to another through a nested loop structure, performing bitwise operations on data during the transfer process.
- `process_bitwise_operations`: Processes bitwise operations on data from multiple memory locations, performs rotation operations based on calculated shift values, and updates memory with combined results.
- `setup_rendering_coordinates`: Calculates rendering coordinates using input parameters, loads data from a table, and copies data between memory locations in a loop structure.
- `transfer_pixel_data_with_condition`: Conditionally transfers pixel data between memory locations when a specific value is not equal to -1, using nested loops to process multiple data elements.
- `process_bitwise_operations_with_shift`: Processes bitwise operations with shift calculations on data from multiple memory locations, performs rotation operations based on calculated values, and updates memory with combined results.
- `update_frame_counter`: Updates a frame counter in a global variable, calculates an address using table lookup, and initializes several memory locations with predefined values.
- `update_input_buffer`: Maintains a buffer of input bytes by reading from a global structure, updating counters, and storing processed input data.
- `process_animation_loop`: Processes animation data through multiple iterations using loop counters and address calculations, updating memory locations based on conditional checks and incrementing values.
- `clear_memory_blocks`: Clears multiple memory blocks by setting word values to zero, using loop counters to iterate through address ranges.
- `frag_movea_cmpi_bne_adda_move_rts`: Short code fragment that copies a5 to a4, then compares the memory at a4 (advancing a4) with 253, then branches if the last result was not zero, then adds 2 to a4, then copies 32 to the memory at a4 (advancing a4) (2 times in a row), then copies 32 to the memory at a4, then returns.
- `frag_movea_cmpi_bne_adda_move_rts_2`: Short code fragment that copies a5 to a4, then compares the memory at a4 (advancing a4) with 253, then branches if the last result was not zero, then adds 2 to a4, then copies 32 to the memory at a4 (advancing a4) (3 times in a row), then copies 32 to the memory at a4, then returns.
- `frag_movea_cmpi_bne_adda_rts`: Short code fragment that copies a5 to a4, then compares the memory at a4 (advancing a4) with 253, then branches if the last result was not zero, then adds 2 to a4, then returns.
- `frag_movem_movea_move_dbra_rts`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a6 to a1, then copies 18816 to d0, then copies 65535 to the memory at a1 (advancing a1), then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `wait_for_vblank`: Waits for a specific vertical blanking interval by polling a global variable until it matches a known value, then clears a counter and returns.
- `wait_for_vblank_2`: Waits for a specific vertical blanking interval by polling a global variable until it reaches zero, while continuously checking a hardware register for a particular value.
- `manage_color_fade`: Manages color fading effects by reading color data from memory, performing arithmetic operations on color components, and writing results to hardware color registers.
- `wait_for_vblank_3`: Waits for a specific vertical blanking interval by polling a hardware register until a condition is met, then calls another routine and loops until a counter reaches zero.
- `wait_for_vblank_4`: Waits for a specific vertical blanking interval by polling a hardware register until a condition is met, then calls another routine and loops until a counter is zero.
- `update_color_palette`: Updates color palette entries based on a counter value, writing modified color data to two hardware color registers, and adjusts the counter based on its current state.
- `count_iterations`: Counts iterations through nested loops using a 16-bit counter and a 32-bit counter, storing the final count in a global variable.
- `frag_move_clr_addi_cmp_bne_rts`: Short code fragment that copies d0 to the stack, then clears d0, then adds 1 to d0, then compares d0 with an address, then branches if the last result was not zero, then copies the top of the stack to d0, then returns.
- `initialize_display_buffers`: Clears specific memory regions by writing zero words to addresses starting at a fixed location through a loop, then calls multiple routines and updates global variables based on conditional checks.
- `update_score_display`: Writes zero words to a series of memory locations in a loop, clears a global variable, and if a condition is met, performs division and stores character values in memory before calling several routines.
- `update_game_state_3`: Processes game state based on a counter value read from memory, updating global variables and calling other routines depending on the counter's value.
- `convert_number_to_string`: Converts a 32-bit number into a string representation with decimal digits stored at specific memory locations.
- `frag_move_tst_bne_lea_bra`: Short code fragment that copies d0 to the stack, then copies a0 to the stack, then tests the memory at an offset from a6, then branches if the last result was not zero, then loads the address of a location in the code into a0, then jumps to another point in the code.
- `shift_and_store_values`: Shifts values in a sequence of words, storing the result in multiple memory locations and updating a global variable.
- `update_animation_data`: Processes animation-related data by updating memory addresses and handling conditional jumps based on byte values read from a buffer, then performs a jump through a table based on a calculated offset.
- `frag_lea_adda_movea_jmp`: Short code fragment that loads the address of a fixed memory address into a3, then adds d7 to a3, then copies a2 to a1, then loads the address of a location in the code into a4, then adds d4 to a4, then adds the memory at a4 to a4, then jumps to the address in a4.
- `process_animation_data`: Processes animation data by incrementing counters, updating memory addresses, and handling conditional jumps based on data values.
- `copy_byte_sequences`: Copies byte sequences from two source locations to a destination, processing 8 bytes per iteration with specific offsets.
- `compare_byte_sequences`: Compares byte sequences from two memory locations, returning a comparison result based on the first differing byte.
- `initialize_bomb_jack_data`: Initializes various data structures and clears multiple memory locations, including display buffers and status flags.
- `frag_lea_move_cmp_bne_dbra_rts_clr`: Short code fragment that loads the address of a fixed memory address into a1, then copies 8 to d1, then copies the memory at a2 (advancing a2) to d0, then compares d0 with the memory at a1 (advancing a1), then branches if the last result was not zero, then loops back while counting down d1, then copies 1 to d7, then returns, then clears d7, then returns.
- `initialize_counters`: Initializes a 16-bit counter to -1 and counts down through a loop until it reaches -1, then stores the value -1 into a memory location at offset 32 of a register.
- `copy_and_divide_bytes`: Copies bytes from one location to another with offset calculation, then divides a value by 10 and stores the digits.
- `reverse_copy_bytes`: Reverses byte sequences from two memory locations, copying data with specific offsets and handling conditional logic.
- `compare_and_update_counter`: Compares byte sequences between two memory locations, updating a counter based on comparison results and returning the final value.
- `handle_display_buffer`: Handles display buffer updates by checking a hardware register, toggling a flag, and copying data from specific memory locations.
- `frag_bsr_rts`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_move_clr_bsr_rts`: Short code fragment that copies 0 to d0, then clears d1, then calls another routine, then returns.
- `frag_move_clr_bsr_rts_2`: Short code fragment that copies 0 to d0, then clears d1, then calls another routine, then returns.
- `frag_move_bsr_rts_3`: Short code fragment that copies d7 to d0, then calls another routine, then copies a5 to a fixed memory address, then calls another routine, then returns.
- `frag_move_bsr_addi_subi_bne_rts`: Short code fragment that copies d7 to d0, then calls another routine, then copies a5 to a fixed memory address, then copies d6 to a fixed memory address, then calls another routine, then adds 1 to a fixed memory address, then subtracts 1 from a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_bsr_clr_lea_move_rts`: Short code fragment that calls another routine (2 times in a row), then clears a fixed memory address, then loads the address of a fixed memory address into a5, then copies 38144 to the memory at an offset from a5, then copies 25088 to the memory at an offset from a5, then copies 37154 to the memory at an offset from a5, then calls another routine, then returns.
- `frag_move_bsr_rts_4`: Short code fragment that copies 0 to d0, then copies 0 to d1, then calls another routine, then returns.
- `frag_move_bsr_rts_5`: Short code fragment that copies 0 to d0, then copies 0 to d1, then calls another routine, then returns.
- `configure_io_ports_2`: Configures CIA port registers for joystick input handling, performs bit manipulation on input data, and calls another routine with a delay.
- `frag_move_ori_tst_beq_bclr_rts_2`: Short code fragment that copies a fixed memory address to d2, then merges 4 into d2, then tests d0, then branches if the last result was zero, then clears bit 2 of d2, then copies d2 to a fixed memory address, then returns.
- `update_joystick_input`: Reads the joystick fire button status from CIAA_PRA and updates the game's input state in a global variable, then calls another routine while the button is pressed.
- `process_movement_delta`: Calculates movement differences between current and previous positions, updates a global variable based on direction, and calls another routine multiple times while adjusting the global variable value.
- `modify_output_bits`: Reads a byte from memory, modifies its bits using bitwise operations with a rotated value, and writes the result back to memory while conditionally clearing one bit.
- `handle_disk_operations`: Handles disk operations including reading file headers, validating format signatures, and processing data blocks while managing hardware registers.
- `frag_subq_bsr_dbra_rts`: Short code fragment that subtracts 1 from d0, then calls another routine, then loops back while counting down d0, then returns.
- `frag_bset_move_rte`: Short code fragment that sets bit 0 of a fixed memory address, then copies 32 to a fixed memory address, then returns from the exception.
- `frag_bclr_beq_rts_2`: Short code fragment that clears bit 0 of a fixed memory address (2 times in a row), then branches if the last result was zero, then returns.
- `parse_audio_data`: Parses audio data by validating file headers, extracting color information from data blocks, and copying processed color values to memory locations.
- `copy_byte_sequence`: Copies bytes between memory locations in a loop, handling both positive and negative counts by either copying or duplicating bytes based on the count value.
- `duplicate_memory_blocks`: Copies data from one memory location to four different destination locations with specific offsets, performing multiple iterations of copying 19 lines of 19 bytes each.
- `copy_to_five_locations`: Copies data from one memory location to five different destination locations with specific offsets, performing multiple iterations of copying 19 lines of 19 bytes each.
- `parse_chunk_header`: Reads and validates a chunk header from memory, advancing the pointer past the header, and returns a value based on the chunk type.
- `xor_memory_operations`: Processes a sequence of data by reading values and performing XOR operations on memory locations at different offsets based on a selector value, updating multiple memory locations in a loop.
- `wait_for_interrupt`: Waits for specific interrupt conditions by polling a hardware register, clears the interrupt flag, and then writes a value to another hardware register before returning.
- `update_bomb_jack_state`: Processes input flags and updates position coordinates for Bomb Jack character, handling movement direction and animation state changes based on joystick input and game logic conditions.
- `update_bomb_jack_state_3`: Processes input flags and updates position coordinates for Bomb Jack character, handling movement direction and animation state changes based on game logic conditions.
- `initialize_bomb_state`: Initializes bomb-related state variables and sets up pointers to bomb data structures.
- `copy_bomb_states`: Copies 16-bit values from one global variable structure to another, updating bomb-related state information.
- `update_bomb_state_14`: Updates bomb-related state variables based on conditions and performs bit manipulation on a global counter, then returns.
- `frag_move_addi_bra`: Short code fragment that copies the memory at a0 to d0, then adds 1 to d0, then copies d0 to the memory at a0, then jumps to another point in the code.
- `frag_move_addi_bra_2`: Short code fragment that copies the memory at an offset from a0 to d0, then adds 1 to d0, then copies d0 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_move_subi_bra`: Short code fragment that copies the memory at a0 to d0, then subtracts 1 from d0, then copies d0 to the memory at a0, then copies 15 to the memory at an offset from a0, then jumps to another point in the code.
- `decrement_bomb_counter`: Decrements a word value at offset 156 of the structure in a6, clears a word value at offset 162 of the same structure, and updates two color registers.
- `frag_cmp_bne_cmpi_subi_clr_move_bra`: Short code fragment that compares d0 with 2, then branches if the last result was not zero, then compares the memory at an offset from a6 with 20, then branches if the last result was not zero, then subtracts 1 from the memory at an offset from a6, then clears the memory at an offset from a6, then copies 14 to a fixed memory address, then copies 10 to a fixed memory address, then jumps to another point in the code.
- `update_bomb_status_7`: Decrements a counter at offset 156 of the structure in a6, clears a status word at offset 162, and updates color registers.
- `frag_tst_beq_subq_move_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a6, then copies 121 to the memory at an offset from a1, then jumps to another point in the code.
- `update_input_state`: Reads input state from memory offsets, updates a flag based on bit 15 of a value, and manages input buffer pointers and counters.
- `frag_move_btst_bne_clr_bra`: Short code fragment that copies the memory at an offset from a6 to d7, then tests bit 15 of d7, then branches if the last result was not zero, then clears the memory at an offset from a6, then jumps to another point in the code.
- `set_bomb_state_flag`: Sets a global variable to one and checks several 16-bit counters, branching to different offsets based on their values.
- `update_bomb_state_4`: Updates bomb state and color settings based on internal counters and conditions, then modifies global variables and hardware color registers accordingly.
- `frag_cmp_bne_move_bra`: Short code fragment that compares d0 with 1, then branches if the last result was not zero, then copies 14 to a fixed memory address, then copies 10 to a fixed memory address, then jumps to another point in the code.
- `frag_cmp_bne_move_bra_2`: Short code fragment that compares d0 with 2, then branches if the last result was not zero, then copies 3984 to a fixed memory address, then copies 3184 to a fixed memory address, then jumps to another point in the code.
- `update_bomb_colors`: Sets specific color values in hardware registers and updates a global variable when the input condition equals three.
- `update_bomb_horizontal_position`: Updates the horizontal position of a bomb by incrementing or decrementing its value based on specific conditions, then stores the result back to memory.
- `update_frame_counter_2`: Updates a frame counter located at a memory address pointed to by a1, modifying its value based on conditions involving global variables.
- `frag_move_bra`: Short code fragment that copies 12 to the memory at an offset from a1, then jumps to another point in the code.
- `update_bomb_position`: Updates the horizontal position of a bomb by decrementing it and storing the result back to memory.
- `update_animation_state`: Updates an animation state value based on conditions involving global variables and comparisons, then stores the result back to memory.
- `frag_move_bra_2`: Short code fragment that copies 5 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies 13 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_tst_beq_move_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was zero, then tests the memory at an offset from a6, then branches if the last result was zero, then copies 14 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_tst_beq_move_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was zero, then copies 14 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_cmp_bgt_bra`: Short code fragment that copies 19 to the memory at an offset from a1, then copies the memory at an offset from a6 to d0, then compares d0 with 5, then branches if the comparison was greater, then copies 1 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_cmp_bgt_move_bra`: Short code fragment that compares d0 with 10, then branches if the comparison was greater, then copies 2 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_cmp_bgt_move_bra_2`: Short code fragment that compares d0 with 20, then branches if the comparison was greater, then copies 3 to the memory at an offset from a6, then jumps to another point in the code.
- `update_bomb_state_13`: Updates the state of a bomb based on its position and flags, then increments a counter and modifies related status variables.
- `frag_tst_bne_move_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not zero, then tests the memory at an offset from a6, then branches if the last result was not zero, then tests the memory at an offset from a6, then branches if the last result was not zero, then copies the memory at an offset from a6 to d7, then jumps to another point in the code.
- `frag_move_btst_beq_tst_bne_clr_bra`: Short code fragment that copies the memory at an offset from a6 to d7, then tests bit 14 of d7, then branches if the last result was zero, then tests the memory at an offset from a6, then branches if the last result was not zero, then clears the memory at an offset from a6 (3 times in a row), then jumps to another point in the code.
- `frag_btst_beq_addi_tst_bne_move_bra`: Short code fragment that tests bit 14 of d7, then branches if the last result was zero, then adds 1 to the memory at an offset from a6, then tests the memory at an offset from a6, then branches if the last result was not zero, then tests the memory at an offset from a6, then branches if the last result was not zero, then copies the memory at an offset from a6 to d0, then jumps to another point in the code.
- `update_bomb_status_9`: Updates bomb status and timer based on specific flags and conditions, then modifies related game state variables.
- `update_bomb_status_8`: Updates bomb timer and status flags based on a condition, then clears specific flags and returns.
- `update_bomb_timer`: Updates a bomb's timer and status flags based on its current state, then clears specific flags and returns.
- `frag_move_cmp_bgt_bra_2`: Short code fragment that copies the memory at an offset from a6 to d0, then compares d0 with 5, then branches if the comparison was greater, then copies 65535 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_cmp_bgt_move_bra_3`: Short code fragment that compares d0 with 10, then branches if the comparison was greater, then copies 65534 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_cmp_bgt_move_bra_4`: Short code fragment that compares d0 with 20, then branches if the comparison was greater, then copies 65533 to the memory at an offset from a6, then jumps to another point in the code.
- `update_bomb_counters_4`: Decrements a word value at offset 5186 of the structure in a6, increments another word value at offset 5176 of the same structure, and compares the incremented value with six.
- `update_bomb_position_3`: Updates the horizontal position of a bomb based on its velocity and boundary checks, then clears several status flags and updates related counters.
- `frag_move_bra_4`: Short code fragment that copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_bomb_velocity`: Updates a bomb's horizontal velocity by reading from a structure, adding a global value, performing boundary checks, and storing the result back to the structure.
- `setup_bitmap_blits`: Configures bitmap blitting hardware registers with fixed values and calculates destination addresses for multiple blits using a loop, then updates global variables with bit pattern data.
- `copy_and_or_tile_data`: Copies and combines tile data rows using OR operations between source addresses calculated from offsets, while managing counters and address pointers through multiple iterations.
- `process_tile_data`: Processes tile data by reading offsets from memory, calculating addresses using those offsets, performing OR operations on 32-bit values from four source pointers, and storing the results to a destination pointer with specific addressing increments.
- `process_tile_data_v2_2`: Processes tile data by reading offsets from memory, calculating addresses using those offsets, performing OR operations on 32-bit values from multiple sources and writing results to a destination buffer with specific stride calculations.
- `process_tile_data_v2`: Processes tile data by reading offsets from memory, calculating addresses using those offsets, performing OR operations on 32-bit values from four source pointers, and storing the results to a destination pointer with specific addressing increments, but without adding a fixed offset to the first source pointer.
- `setup_blit_parameters`: Configures bitmap blitting hardware registers with fixed values and calculates destination addresses for multiple blits using a loop, then updates a global variable with data from a table.
- `initialize_blit_parameters`: Sets up bitmap blitting parameters by calculating addresses and storing them in a structure, then configures hardware registers for bit block transfer operations.
- `initialize_bitmap_blit`: Sets up bitmap blitting parameters using values from a data structure, configures hardware registers for bit block transfer operations, and calculates offset values for fragment rendering based on coordinate differences.
- `update_fragment_data`: Calculates fragment offset values based on input coordinates and stores results in structure fields.
- `calculate_fragment_offset`: Calculates a fragment offset based on input values and stores the result in multiple global structure fields, then updates related pointers and offsets.
- `frag_move_bra_5`: Short code fragment that copies 102 to d0, then jumps to another point in the code.
- `frag_move_bra_6`: Short code fragment that copies 93 to d0, then jumps to another point in the code.
- `update_fragment_pointers`: Updates fragment-related pointers and offsets in a data structure based on table lookups and arithmetic operations, then modifies a global counter and returns.
- `update_bomb_count`: Updates a 16-bit counter in a global structure, increments another 16-bit counter, and handles carry propagation for decimal digit counters.
- `update_bomb_counters_3`: Increments a 16-bit counter at offset 3450 of a global structure, and decrements another 16-bit counter at offset 186 of the same structure if it's non-zero.
- `update_bomb_positions_2`: Processes bomb data entries, clears specific bomb states when certain conditions are met, and updates global bomb counters based on position.
- `frag_move_andi_cmp_bne_addi_bra`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with 12, then compares d0 with 4, then branches if the last result was not zero, then adds 152 to d7, then adds 152 to d6, then jumps to another point in the code.
- `update_bomb_positions`: Processes bomb data entries, clears specific bomb states when certain conditions are met, and updates global bomb counters based on position checks.
- `clear_bomb_data`: Clears four longword values starting at a memory location pointed to by a1.
- `process_bomb_data`: Reads bomb-related data from memory, performs boundary checks, and updates game state based on conditions.
- `frag_adda_bra`: Short code fragment that adds 4 to a2, then jumps to another point in the code.
- `update_bomb_state_12`: Updates bomb-related game state based on conditional checks and modifies counters, flags, and status bytes.
- `update_bomb_state_11`: Updates bomb-related game state based on conditional checks and modifies counters, flags, and status bytes.
- `frag_cmpi_beq_move_cmp_blt_bgt_bra`: Short code fragment that compares the memory at an offset from a6 with 1, then branches if the last result was zero, then copies the memory at a1 to d0, then compares d0 with d7, then branches if the comparison was less, then compares d0 with d6, then branches if the comparison was greater, then jumps to another point in the code.
- `update_bomb_state_10`: Processes bomb object state changes based on position and type comparisons, updating coordinates and flags while handling special conditions for different bomb states.
- `frag_cmp_beq_bne_move_andi_bra`: Short code fragment that compares d1 with 4, then branches if the last result was zero, then compares d1 with 2, then branches if the last result was not zero, then copies the memory at an offset from a1 to d2, then masks d2 with 1, then branches if the last result was zero, then masks the memory at an offset from a1 with 254, then copies 26 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_cmp_bne_move_andi_ori_bra`: Short code fragment that compares d1 with 8, then branches if the last result was not zero, then copies the memory at an offset from a1 to d2, then masks d2 with 1, then branches if the last result was not zero, then merges 1 into the memory at an offset from a1, then copies 20 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_cmp_beq_tst_addi_bne_bra`: Short code fragment that copies the memory at an offset from a1 to d2, then compares d2 with 65535, then branches if the last result was zero, then tests the memory at an offset from a6, then branches if the last result was zero, then adds 1 to d2, then compares d2 with 2, then branches if the last result was not zero, then jumps to another point in the code.
- `update_bomb_state_5`: Updates bomb-related state values based on comparisons with specific constants, writing new values to memory locations offset from a1.
- `frag_move_andi_cmp_bgt_bra`: Short code fragment that copies the memory at an offset from a1 to d2, then masks d2 with 15, then compares d2 with 2, then branches if the comparison was greater, then copies 65535 to the memory at an offset from a1, then copies 32 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_eori_andi_bne_bra`: Short code fragment that copies d2 to the memory at an offset from a1, then copies the memory at an offset from a1 to d2, then exclusive-ors 1 into d2, then copies d2 to the memory at an offset from a1, then masks d2 with 1, then branches if the last result was not zero, then copies 26 to the memory at an offset from a1, then jumps to another point in the code.
- `update_timer_and_state`: Updates a timer value and state flags based on comparison results, then branches conditionally to different code paths.
- `frag_move_bra_7`: Short code fragment that copies 64 to the memory at an offset from a1, then copies 26 to the memory at an offset from a1, then jumps to another point in the code.
- `advance_tile_counter`: Increments or decrements a counter in a data structure based on a condition, then updates related state values and performs conditional modifications to tile coordinates and flags.
- `decrement_tile_counter`: Decrements a counter in a data structure and updates related state values based on conditional checks.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `update_bomb_state_9`: Reads position and state data from a structure, performs bitwise operations and comparisons on the data, updates the state and direction flags based on comparison results, and writes the modified values back to the structure.
- `frag_cmp_bgt_move_andi_beq_ori_bra`: Short code fragment that compares d4 with 65516, then branches if the comparison was greater, then copies the memory at an offset from a1 to d4, then masks d4 with 61440, then compares d4 with 4096, then branches if the last result was zero, then compares d4 with 16384, then branches if the last result was zero, then merges 32 into d0, then jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_move_andi_cmp_bne_ori_bra`: Short code fragment that copies d1 to d2, then masks d2 with 4095, then masks d1 with 61440, then compares d0 with 16, then branches if the last result was not zero, then compares d1 with 4096, then branches if the last result was not zero, then merges 16384 into d2, then jumps to another point in the code.
- `frag_ori_move_bra`: Short code fragment that merges 12288 into d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra`: Short code fragment that compares d0 with 128, then branches if the last result was not zero, then compares d1 with 4096, then branches if the last result was not zero, then merges 8192 into d2, then jumps to another point in the code.
- `frag_ori_move_bra_2`: Short code fragment that merges 12288 into d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_2`: Short code fragment that compares d0 with 32, then branches if the last result was not zero, then compares d1 with 8192, then branches if the last result was not zero, then merges 4096 into d2, then jumps to another point in the code.
- `frag_ori_move_bra_3`: Short code fragment that merges 16384 into d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `validate_and_modify_flags`: Compares a word value in a register against specific constants and branches to different locations based on equality or inequality conditions, then modifies another register by setting specific bits before branching to a final location.
- `frag_cmp_bne_ori_bra_3`: Short code fragment that compares d0 with 8192, then branches if the last result was not zero, then merges 16384 into d2, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_4`: Short code fragment that compares d0 with 12288, then branches if the last result was not zero, then merges 4096 into d2, then jumps to another point in the code.
- `frag_ori_move_bra_4`: Short code fragment that merges 8192 into d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_5`: Short code fragment that compares d0 with 64, then branches if the last result was not zero, then compares d1 with 16384, then branches if the last result was not zero, then merges 4096 into d2, then jumps to another point in the code.
- `frag_ori_move_bra_5`: Short code fragment that merges 8192 into d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_clr_move_bra`: Short code fragment that clears d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `update_position_bounds`: Reads word values from memory locations offset by 8 and c0a from a1 and a6 respectively, performs arithmetic operations on these values, and conditionally updates a word at offset 6 of a1 based on comparison results.
- `frag_andi_ori_bra`: Short code fragment that masks the memory at an offset from a1 with 4095, then merges 16384 into the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_sub_bmi_andi_ori_bra`: Short code fragment that copies the memory at an offset from a6 to d1, then subtracts the memory at an offset from a1 from d1, then branches if the last result was negative, then masks the memory at an offset from a1 with 4095, then merges 12288 into the memory at an offset from a1, then jumps to another point in the code.
- `update_character_state_2`: Updates character state values based on bitwise operations and conditional checks, modifying data at memory locations pointed to by a1.
- `frag_move_addi_subi_bra`: Short code fragment that copies the memory at a1 to d0, then adds 1 to d0, then copies d0 to the memory at a1, then copies the memory at an offset from a1 to d0, then subtracts 1 from d0, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_addi_bra_3`: Short code fragment that copies the memory at a1 to d0, then adds 1 to d0, then copies d0 to the memory at a1, then copies the memory at an offset from a1 to d0, then adds 1 to d0, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_subi_addi_bra`: Short code fragment that copies the memory at a1 to d0, then subtracts 1 from d0, then copies d0 to the memory at a1, then copies the memory at an offset from a1 to d0, then adds 1 to d0, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `update_character_state`: Updates a character's state by incrementing a value at offset 4 from a pointer, then stores the result back to that location.
- `adjust_character_code`: Adjusts a character code in memory based on conditions, updating the value to either 63 or 64 under specific criteria, then continues processing.
- `frag_move_bra_8`: Short code fragment that copies 63 to the memory at an offset from a1, then jumps to another point in the code.
- `update_bomb_position_2`: Updates bomb position and state based on direction flags, clamps values between specific ranges, and manages a counter at offset 10 of a structure.
- `frag_cmp_bne_bra`: Short code fragment that compares d1 with 96, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_6`: Short code fragment that compares d1 with 192, then branches if the last result was not zero, then merges 12288 into d2, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_7`: Short code fragment that compares d1 with 144, then branches if the last result was not zero, then merges 8192 into d2, then jumps to another point in the code.
- `frag_cmp_bne_move_andi_beq_ori_bra`: Short code fragment that compares d1 with 64, then branches if the last result was not zero, then copies the memory at an offset from a1 to d3, then masks d3 with 61440, then compares d3 with 4096, then branches if the last result was zero, then merges 12288 into d2, then jumps to another point in the code.
- `frag_cmp_bne_move_andi_tst_ori_bra`: Short code fragment that compares d1 with 16, then branches if the last result was not zero, then copies the memory at an offset from a1 to d3, then masks d3 with 61440, then tests d3, then branches if the last result was not zero, then merges 4096 into d2, then jumps to another point in the code.
- `frag_ori_bra`: Short code fragment that merges 8192 into d2, then jumps to another point in the code.
- `frag_cmp_bne_move_andi_tst_ori_bra_2`: Short code fragment that compares d1 with 32, then branches if the last result was not zero, then copies the memory at an offset from a1 to d3, then masks d3 with 61440, then tests d3, then branches if the last result was not zero, then merges 12288 into d2, then jumps to another point in the code.
- `frag_ori_bra_2`: Short code fragment that merges 8192 into d2, then jumps to another point in the code.
- `update_position_and_counter`: Updates position values based on direction flags and manages a counter at offset 10 of a structure, then performs conditional updates to other counters and a global variable.
- `frag_move_addi_bra_4`: Short code fragment that copies the memory at a1 to d0, then adds 1 to d0, then copies d0 to the memory at a1, then copies the memory at an offset from a1 to d0, then adds 1 to d0, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_subi_addi_bra_2`: Short code fragment that copies the memory at a1 to d0, then subtracts 1 from d0, then copies d0 to the memory at a1, then copies the memory at an offset from a1 to d0, then adds 1 to d0, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_subi_bra_2`: Short code fragment that copies the memory at a1 to d0, then subtracts 1 from d0, then copies d0 to the memory at a1, then copies the memory at an offset from a1 to d0, then subtracts 1 from d0, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `reset_counter_if_max`: Checks if a 16-bit value at offset 12 of a structure equals six, and if so, clears that value.
- `update_bomb_state_7`: Updates bomb state and position based on input flags and clamps values between specific ranges.
- `process_input_flags`: Compares a register value with two constants and performs conditional operations including bitwise masking, subroutine calls, and memory reads.
- `frag_ori_move_cmp_bne_bra`: Short code fragment that merges 12288 into d1, then copies d1 to the memory at an offset from a1, then copies 35 to the memory at an offset from a1, then compares d0 with 4096, then branches if the last result was not zero, then copies 8193 to the memory at an offset from a1, then jumps to another point in the code.
- `update_bomb_state_2`: Updates bomb state and position based on input flags and clamps values between specific ranges.
- `check_collision_bounds`: Subtracts values from memory locations offset by c0c and c0a from a6 and a1 respectively, compares the results, and sets specific bits in d3 based on the comparison outcome.
- `frag_ori_bra_3`: Short code fragment that merges 4096 into d3, then jumps to another point in the code.
- `update_character_state_3`: Updates character state based on input flags and clamps values between specific ranges.
- `frag_addi_bra`: Short code fragment that adds 1 to the memory at a1, then jumps to another point in the code.
- `frag_addi_bra_2`: Short code fragment that adds 1 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_subi_bra`: Short code fragment that subtracts 1 from the memory at a1, then jumps to another point in the code.
- `clamp_character_code`: Clamps a character code between 47 and 53, increments it if a global flag is set, and updates the code in memory.
- `adjust_character_code_2`: Adjusts a character code in memory based on comparison with boundary values, then stores the updated code back to memory.
- `clamp_character_code_2`: Adjusts a character code in memory to ensure it stays within the range 35 to 40, updating it to 35 if it goes out of bounds, and increments the value if a global flag is set.
- `process_collision_data`: Reads collision-related data from memory, performs bitwise operations and comparisons on position and type values, updates the data based on comparison results, and writes back the modified values.
- `frag_cmp_bgt_move_andi_beq_ori_bra_2`: Short code fragment that compares d4 with 65516, then branches if the comparison was greater, then copies the memory at an offset from a1 to d4, then masks d4 with 61440, then compares d4 with 4096, then branches if the last result was zero, then compares d4 with 8192, then branches if the last result was zero, then merges 64 into d0, then jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_move_andi_cmp_bne_ori_bra_2`: Short code fragment that copies d1 to d2, then masks d2 with 4095, then masks d1 with 61440, then compares d0 with 16, then branches if the last result was not zero, then compares d1 with 4096, then branches if the last result was not zero, then merges 16384 into d2, then jumps to another point in the code.
- `frag_ori_move_bra_6`: Short code fragment that merges 12288 into d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_8`: Short code fragment that compares d0 with 128, then branches if the last result was not zero, then compares d1 with 4096, then branches if the last result was not zero, then merges 8192 into d2, then jumps to another point in the code.
- `frag_ori_move_bra_7`: Short code fragment that merges 12288 into d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_9`: Short code fragment that compares d0 with 32, then branches if the last result was not zero, then compares d1 with 8192, then branches if the last result was not zero, then merges 4096 into d2, then jumps to another point in the code.
- `frag_ori_move_bra_8`: Short code fragment that merges 16384 into d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `check_byte_values`: Compares a word value in register d0 against four specific constants and branches to the same location if any match, otherwise continues execution.
- `frag_cmp_bne_ori_bra_10`: Short code fragment that compares d0 with 8192, then branches if the last result was not zero, then merges 16384 into d2, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_11`: Short code fragment that compares d0 with 12288, then branches if the last result was not zero, then merges 4096 into d2, then jumps to another point in the code.
- `frag_ori_move_bra_9`: Short code fragment that merges 8192 into d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_12`: Short code fragment that compares d0 with 64, then branches if the last result was not zero, then compares d1 with 16384, then branches if the last result was not zero, then merges 4096 into d2, then jumps to another point in the code.
- `frag_ori_move_bra_10`: Short code fragment that merges 8192 into d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_clr_move_bra_2`: Short code fragment that clears d2, then copies d2 to the memory at an offset from a1, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `clamp_position_value_2`: Reads a word value from a global variable at offset 3082 of a6, adds 8 to it, subtracts a word value from memory location pointed by a1, and conditionally updates a word at offset 6 of the structure in a1 based on the result.
- `frag_andi_ori_bra_2`: Short code fragment that masks the memory at an offset from a1 with 4095, then merges 16384 into the memory at an offset from a1, then jumps to another point in the code.
- `clamp_position_value`: Reads a word value from memory location offset by c0c from a6, subtracts a value from a1, and conditionally updates a word at offset 6 of a1 with masked and merged bits based on the subtraction result.
- `update_character_state_4`: Updates character state values based on bit pattern conditions, modifies counters and position data, then performs boundary checks and updates a global counter.
- `frag_move_addi_subi_bra_2`: Short code fragment that copies the memory at a1 to d0, then adds 1 to d0, then copies d0 to the memory at a1, then copies the memory at an offset from a1 to d0, then subtracts 1 from d0, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_addi_bra_5`: Short code fragment that copies the memory at a1 to d0, then adds 1 to d0, then copies d0 to the memory at a1, then copies the memory at an offset from a1 to d0, then adds 1 to d0, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_subi_addi_bra_3`: Short code fragment that copies the memory at a1 to d0, then subtracts 1 from d0, then copies d0 to the memory at a1, then copies the memory at an offset from a1 to d0, then adds 1 to d0, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `adjust_character_code_3`: Checks a word value from memory at offset 4 of a1 against range limits, increments it when within range, then stores a specific value back to that memory location.
- `decrement_character_code`: Validates a word value from memory at offset 4 of a1 against range limits, decrements it when within range, then stores another specific value back to that memory location.
- `frag_move_bra_9`: Short code fragment that copies 80 to the memory at an offset from a1, then jumps to another point in the code.
- `update_memory_value`: Compares a word value in d1 with six, and if equal, reads from two memory locations, performs arithmetic operations on them, and conditionally modifies memory at a1.
- `frag_addi_move_sub_beq_bpl_subi_bra`: Short code fragment that adds 1 to the memory at a1, then copies the memory at an offset from a6 to d0, then subtracts the memory at an offset from a1 from d0, then branches if the last result was zero, then branches if the last result was not negative, then subtracts 1 from the memory at an offset from a1, then jumps to another point in the code.
- `update_animation_frame`: Updates an animation frame counter and manages frame index changes in a data structure, with conditional logic based on global state and frame values.
- `frag_cmp_bne_tst_move_bra`: Short code fragment that compares d1 with 7, then branches if the last result was not zero, then tests the memory at an offset from a1, then branches if the last result was not zero, then copies 1 to the memory at an offset from a1, then jumps to another point in the code.
- `adjust_velocity_sign`: Extracts lower 4 bits from a word at offset 6 of a1, branches if zero, otherwise subtracts values from memory locations offset by c0a and a1, performs negation on the result, and calls another routine with the negated value.
- `frag_bsr_move_sub_tst_bpl_neg_bra`: Short code fragment that calls another routine, then copies d0 to the memory at an offset from a1, then copies the memory at an offset from a6 to d0, then subtracts the memory at an offset from a1 from d0, then tests d0, then branches if the last result was not negative, then negates d0, then calls another routine, then negates d0, then jumps to another point in the code.
- `update_direction_flag`: Updates a direction flag based on input conditions and performs bit toggling operations on a byte value, then returns.
- `toggle_and_update`: Toggles bits in two bytes based on conditionals derived from a word value, then updates velocity values and performs boundary checks on position data.
- `frag_andi_beq_move_bpl_not_bra`: Short code fragment that masks d1 with 128, then branches if the last result was zero, then copies the memory at an offset from a1 to d0, then branches if the last result was not negative, then inverts the memory at an offset from a1, then jumps to another point in the code.
- `update_velocity`: Updates velocity values based on input and global conditions, clamping results within specific ranges before storing them back.
- `update_character_code`: Reads a 16-bit value from a global variable, compares it with 65, and branches if less than.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `update_bomb_state_5_2`: Updates bomb object state by decrementing a counter, incrementing another, and modifying position and status flags based on conditions involving timer values and global flags.
- `update_bomb_flag`: Reads a byte from offset 6 of a structure, clears its least significant bit, and writes it back.
- `check_bomb_timer`: Compares a 16-bit value at offset 8 of the structure in a1 with 8, and branches to an address if they are not equal.
- `update_bomb_state_6`: Loads a 16-bit value from offset 10 of the structure in a1 into d2, compares it with -1, and branches to an address if they are equal.
- `frag_move_andi_cmp_bgt_bra_2`: Short code fragment that copies the memory at an offset from a1 to d2, then masks d2 with 15, then compares d2 with 2, then branches if the comparison was greater, then copies 65535 to the memory at an offset from a1, then copies 32 to the memory at an offset from a1, then jumps to another point in the code.
- `toggle_bomb_flag`: Stores a 16-bit value from d2 into offset 10 of the structure in a1, reads a byte from offset 6 of that structure into d2, toggles bit 0 of d2, and stores the result back to offset 6.
- `reset_bomb_timer`: Sets a 16-bit value at offset 12 of the structure in a1 to 20, then reads a byte from offset 6 of that structure and branches to an address if the value is not zero.
- `frag_move_bra_10`: Short code fragment that copies 64 to the memory at an offset from a1, then copies 26 to the memory at an offset from a1, then jumps to another point in the code.
- `advance_bomb_state`: Advances the state of a bomb object by incrementing counters and modifying position data based on conditional checks, then updates related object properties.
- `update_bomb_state`: Updates bomb object state by decrementing a counter, incrementing another, and modifying position and status flags based on conditions involving global variables and bit manipulation.
- `update_flag_bits`: Updates flag bits in a data structure based on conditions involving a global variable and bit manipulation, then stores the result back to the structure.
- `frag_move_cmp_bne_subi_bra`: Short code fragment that copies the memory at an offset from a1 to d2, then compares d2 with 6001, then branches if the last result was not zero, then subtracts 1 from the memory at an offset from a1, then branches if the last result was not zero, then copies 32766 to the memory at a1, then jumps to another point in the code.
- `frag_subi_bra_2`: Short code fragment that subtracts 2 from the memory at an offset from a1, then jumps to another point in the code.
- `update_counter`: Increments a 16-bit value at offset 8 of a structure, then compares it with 3 and branches if not equal.
- `frag_move_bra_11`: Short code fragment that copies 15 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_subi_move_bra`: Short code fragment that subtracts 2 from the memory at an offset from a1, then copies 93 to the memory at an offset from a1, then jumps to another point in the code.
- `update_entity_counter`: Updates a counter in a data structure and modifies related fields based on conditions, including checking a global variable and performing bit operations on word values.
- `update_tile_position`: Updates a tile's position and state by modifying its coordinates and flags based on calculated values, then clears a specific field in the structure.
- `update_bomb_counter_2`: Decrements a counter at offset 12 of a structure and updates a value at offset 4, then checks if the counter reaches zero to trigger further processing.
- `update_object_counter`: Reads a 16-bit value from offset 12 of the structure in a1, increments it by one, compares it with 512, and branches to an address if they are not equal.
- `update_direction_flags`: Updates direction flags and position coordinates based on conditional logic involving bit operations and comparisons, then adjusts the position values and updates a global counter.
- `frag_cmp_bne_bra_2`: Short code fragment that compares d1 with 96, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_13`: Short code fragment that compares d1 with 192, then branches if the last result was not zero, then merges 12288 into d2, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_14`: Short code fragment that compares d1 with 144, then branches if the last result was not zero, then merges 8192 into d2, then jumps to another point in the code.
- `frag_cmp_bne_move_andi_beq_ori_bra_2`: Short code fragment that compares d1 with 64, then branches if the last result was not zero, then copies the memory at an offset from a1 to d3, then masks d3 with 61440, then compares d3 with 4096, then branches if the last result was zero, then merges 12288 into d2, then jumps to another point in the code.
- `frag_cmp_bne_move_andi_tst_ori_bra_3`: Short code fragment that compares d1 with 16, then branches if the last result was not zero, then copies the memory at an offset from a1 to d3, then masks d3 with 61440, then tests d3, then branches if the last result was not zero, then merges 4096 into d2, then jumps to another point in the code.
- `frag_ori_bra_4`: Short code fragment that merges 8192 into d2, then jumps to another point in the code.
- `frag_cmp_bne_move_andi_tst_ori_bra_4`: Short code fragment that compares d1 with 32, then branches if the last result was not zero, then copies the memory at an offset from a1 to d3, then masks d3 with 61440, then tests d3, then branches if the last result was not zero, then merges 12288 into d2, then jumps to another point in the code.
- `frag_ori_bra_5`: Short code fragment that merges 8192 into d2, then jumps to another point in the code.
- `update_position_and_timer`: Updates position coordinates and a timer field based on conditional logic, then adjusts the position values and updates a global counter.
- `frag_move_addi_bra_6`: Short code fragment that copies the memory at a1 to d0, then adds 2 to d0, then copies d0 to the memory at a1, then copies the memory at an offset from a1 to d0, then adds 2 to d0, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_move_subi_addi_bra_4`: Short code fragment that copies the memory at a1 to d0, then subtracts 2 from d0, then copies d0 to the memory at a1, then copies the memory at an offset from a1 to d0, then adds 2 to d0, then copies d0 to the memory at an offset from a1, then jumps to another point in the code.
- `decrement_position`: Decrements two 16-bit values at offsets 0 and 2 from a pointer, then conditionally updates a third value at offset 4 based on a global variable's state.
- `update_object_state`: Updates the state of an object based on its type and properties, modifying memory locations related to object positioning and status.
- `frag_cmp_bcc_subi_adda_bra`: Short code fragment that compares d0 with 10, then branches if the carry flag is clear, then subtracts 1 from the memory at an offset from a6, then adds 16 to a1, then jumps to another point in the code.
- `process_bomb_state`: Updates bomb-related game state based on conditional checks and modifies counters, flags, and status bytes.
- `advance_bomb_state_2`: Updates bomb-related game state based on a conditional value, modifies counters and status flags, and writes specific bytes to a buffer.
- `update_bomb_state_8`: Updates bomb-related game state based on a conditional value, modifies counters and status flags, and writes specific bytes to a buffer.
- `update_bomb_counters`: Updates bomb-related counters and status flags based on conditional checks, increments values at specific offsets, and writes status bytes to a buffer.
- `update_bomb_state_3`: Updates bomb-related game state including counters, flags, and status bytes based on conditional checks and increments.
- `update_bomb_counter`: Increments a bomb counter stored at offset 79 in a global structure and adds 1000 to a word value at offset 168.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `check_bomb_condition`: Checks if a specific value in a register matches ten, and if so, writes a constant value to a global variable and updates another global variable with an incremented value.
- `initialize_bomb`: Sets specific values in global variables and updates a status byte when a condition is met, then branches to another routine.
- `handle_bomb_activation`: Processes bomb activation states based on conditional checks against global variables, updates counters and status bytes, and manages bomb-related game state including incrementing a global counter, setting a status byte, and modifying memory locations related to bomb behavior.
- `frag_move_movea_bra`: Short code fragment that copies a5 to the stack, then copies the memory at an offset from a6 to a5, then copies 10 to the memory at a5 (advancing a5), then copies a5 to the memory at an offset from a6, then copies the top of the stack to a5, then jumps to another point in the code.
- `update_bomb_status_2`: Increments a global counter at offset 5210 of the structure in a6, updates a byte at offset 78 of the structure in a6, adds 200 to a word at offset 168 of the structure in a6, and calls another routine.
- `update_bomb_status`: Updates bomb-related game state including counters, flags, and status bytes based on conditional checks and increments.
- `process_object_collision`: Processes object collision data by reading properties from a structure, performing bit operations on coordinates, and updating collision flags based on comparison results.
- `frag_cmp_bpl_move_cmpi_bne_bra`: Short code fragment that compares d0 with 16, then branches if the last result was not negative, then copies 1 to the memory at a4, then copies 1 to the memory at an offset from a4, then compares the memory at an offset from a4 with 14, then branches if the last result was not zero, then copies 14 to the memory at a4, then jumps to another point in the code.
- `frag_cmpi_bne_move_beq_cmp_bge_bra`: Short code fragment that compares the memory at an offset from a4 with 14, then branches if the last result was not zero, then copies 14 to the memory at an offset from a4, then compares the memory at an offset from a6 with 1, then branches if the last result was zero, then compares d0 with the memory at an offset from a6, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `frag_cmp_blt_move_cmpi_bne_bra`: Short code fragment that compares d0 with 320, then branches if the comparison was less, then copies 1 to the memory at an offset from a4 (2 times in a row), then compares the memory at a4 with 14, then branches if the last result was not zero, then copies 14 to the memory at an offset from a4, then jumps to another point in the code.
- `check_collision_flag`: Tests a global variable's value at offset 4 of the structure in a4, branches if not equal to 14, otherwise sets another global variable at offset 6 of the structure in a4 to 14.
- `setup_collision_flags`: Initializes collision-related flags in a global structure and performs conditional checks on object properties to determine further collision handling.
- `update_collision_flags`: Updates collision state flags based on object properties and performs conditional resets of certain flag values.
- `frag_cmpi_beq_bne_move_bra`: Short code fragment that compares the memory at a4 with an address, then branches if the last result was zero, then compares the memory at a4 with 14, then branches if the last result was zero, then compares the memory at a4 with an address, then branches if the last result was not zero, then copies 1 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_cmp_bne_bra`: Short code fragment that copies the memory at an offset from a4 to d0, then compares d0 with an address, then branches if the last result was not zero, then copies 2 to the memory at an offset from a6, then jumps to another point in the code.
- `check_and_set_flag`: Tests values at specific offsets in memory and updates a global variable based on comparison results.
- `check_flag_conditions`: Tests values at specific offsets in memory and updates a global variable based on comparison results, with conditional branches that may skip the update.
- `clear_flag_entries`: Clears specific flag entries in memory when they equal a certain value, then updates global variables based on game state conditions.
- `frag_move_movea_bra_2`: Short code fragment that copies a5 to the stack, then copies the memory at an offset from a6 to a5, then copies 10 to the memory at a5 (advancing a5), then copies a5 to the memory at an offset from a6, then copies the top of the stack to a5, then jumps to another point in the code.
- `frag_move_bra_12`: Short code fragment that copies 1 to the memory at an offset from a6, then copies 2 to the memory at an offset from a6, then jumps to another point in the code.
- `check_collision_flags`: Reads collision flags from a structure, updates global variables based on flag values, and performs conditional operations including clearing flags and calling another routine.
- `frag_addi_adda_bra`: Short code fragment that adds 1 to d7, then adds 8 to a1, then jumps to another point in the code.
- `process_input_collision`: Checks input conditions and updates collision-related data based on coordinate values, then calls routines to handle collision states and object positioning.
- `update_object_collision`: Processes object collision data by checking specific flags, updating tile values based on coordinates, incrementing a timer, and calling additional routines to handle collision states and events.
- `update_tile_collision_2`: Updates tile collision properties based on object coordinates and flags, modifies tile values in tables, increments a timer, and handles conditional updates depending on object states.
- `update_tile_properties`: Reads tile coordinates and properties from memory, updates tile values in two tables based on conditions, increments a timer, and calls other routines to handle tile state changes.
- `process_tile_data_2`: Reads tile information from a table using coordinates, updates tile values and properties in memory, and handles specific conditions based on tile data and position checks.
- `update_tile_data_2`: Reads tile data from a table using coordinates, stores the tile value and its properties at specific memory locations, and updates related counters based on the tile information.
- `update_tile_data`: Updates tile data at specific memory locations based on input flags and position coordinates, then clears corresponding flags in the input structure.
- `process_input_flags_3`: Processes input flags from a register, checks conditions based on global variables, updates tile data at specific memory locations, and performs bit manipulations on a table entry before returning.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `update_tile_position_2`: Reads tile coordinates from a table and updates their positions in memory based on calculated offsets.
- `update_tile_collision`: Processes tile collision data by reading coordinates from memory, performing boundary checks, and updating tile attributes based on input flags.
- `process_input_flags_2`: Processes input flags from a register, performs boundary checks on tile data, updates position coordinates, and modifies tile attributes based on comparison results.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `clear_tile_flags`: Clears specific flags in tile attributes based on bit comparisons and updates tile positions accordingly.
- `frag_bgt_moveq_bra`: Short code fragment that branches if the comparison was greater, then loads 1 into d4, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 5 into d4, then jumps to another point in the code.
- `frag_tst_bne_bgt_moveq_bra`: Short code fragment that tests d7, then branches if the last result was not zero, then tests d6, then branches if the comparison was greater, then loads 7 into d4, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 3 into d4, then jumps to another point in the code.
- `frag_tst_blt_bgt_moveq_bra`: Short code fragment that tests d6, then branches if the comparison was less, then tests d7, then branches if the comparison was greater, then loads 2 into d4, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 4 into d4, then jumps to another point in the code.
- `frag_tst_bgt_move_bra`: Short code fragment that tests d7, then branches if the comparison was greater, then copies 8 to d4, then jumps to another point in the code.
- `align_tile_position`: Aligns a tile's vertical position to a grid boundary and updates tile attributes based on comparison results.
- `frag_ori_bra_6`: Short code fragment that merges 16 into the memory at an offset from a0, then jumps to another point in the code.
- `adjust_tile_position`: Adjusts tile X coordinate to grid boundary and updates tile attributes based on comparison results.
- `set_tile_attributes`: Sets specific bits in memory and performs conditional operations based on test results, then calls another routine to process tile data.
- `frag_cmp_bne_bge_tst_bra`: Short code fragment that compares d4 with 3, then branches if the last result was not zero, then compares d7 with 4, then branches if the comparison was greater or equal, then tests the memory at an offset from a4, then branches if the last result was not zero, then jumps to another point in the code.
- `validate_tile_data`: Checks for zero values at specific memory locations and updates tile attributes based on these conditions before calling a processing routine.
- `align_object_coordinates`: Aligns object coordinates to tile grid boundaries based on comparison results and updates flags in a structure.
- `modify_tile_flags`: Sets bits in memory and performs conditional checks on data, then calls another routine to handle tile attribute updates.
- `align_coordinates_3`: Aligns object coordinates to tile grid boundaries based on comparison results and updates flags in a structure.
- `frag_ori_bra_7`: Short code fragment that merges 64 into the memory at an offset from a0, then jumps to another point in the code.
- `align_coordinates_2`: Adjusts object coordinates to tile grid boundaries based on comparison results and updates flags in a structure.
- `update_tile_status`: Modifies memory flags and performs conditional tests on data, then calls a processing routine with specific bit operations based on test results.
- `frag_cmp_bne_bge_tst_bra_2`: Short code fragment that compares d4 with 7, then branches if the last result was not zero, then compares d7 with 4, then branches if the comparison was greater or equal, then tests the memory at a4, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_tst_bne_beq_move_ori_bra`: Short code fragment that tests the memory at a4, then branches if the last result was not zero, then tests the memory at an offset from a4, then branches if the last result was zero, then copies the memory at an offset from a3 to the memory at an offset from a0, then tests d5, then branches if the last result was zero, then merges 128 into the memory at an offset from a0, then jumps to another point in the code.
- `align_tile_coordinates_5`: Aligns tile X coordinates to a 16-pixel grid and adjusts the coordinate based on comparison results.
- `configure_tile_display`: Sets bits in memory and performs conditional checks on data, then calls another routine to process tile display attributes.
- `align_tile_coordinates_4`: Aligns tile X and Y coordinates to grid boundaries and updates tile attributes based on comparison results.
- `frag_ori_bra_8`: Short code fragment that merges 16 into the memory at an offset from a0, then jumps to another point in the code.
- `adjust_tile_coordinates`: Adjusts tile X and Y coordinates to grid boundaries based on comparison results, then updates tile attributes and processes a sequence of tile entries until a termination marker.
- `frag_ori_bra_9`: Short code fragment that merges 16 into the memory at an offset from a0, then jumps to another point in the code.
- `frag_sub_tst_beq_ori_bra`: Short code fragment that subtracts d6 from the memory at an offset from a0, then tests d5, then branches if the last result was zero, then merges 32 into the memory at an offset from a0, then jumps to another point in the code.
- `align_tile_coordinates`: Adjusts tile X and Y coordinates to grid boundaries based on comparison results, then updates tile attributes and processes a sequence of tile entries until a termination marker is encountered.
- `frag_ori_bra_10`: Short code fragment that merges 16 into the memory at an offset from a0, then jumps to another point in the code.
- `frag_add_tst_beq_ori_bra`: Short code fragment that adds d6 to the memory at an offset from a0, then tests d5, then branches if the last result was zero, then merges 128 into the memory at an offset from a0, then jumps to another point in the code.
- `align_coordinates`: Adjusts object coordinates to tile grid boundaries based on comparison results and updates flags in a structure.
- `frag_ori_bra_11`: Short code fragment that merges 64 into the memory at an offset from a0, then jumps to another point in the code.
- `align_tile_coordinates_3`: Adjusts tile X and Y coordinates to grid boundaries based on comparison results, updates flags in a structure, and processes a sequence of tile entries until a termination marker.
- `frag_ori_bra_12`: Short code fragment that merges 64 into the memory at an offset from a0, then jumps to another point in the code.
- `frag_sub_tst_beq_ori_bra_2`: Short code fragment that subtracts d6 from the memory at an offset from a0, then tests d5, then branches if the last result was zero, then merges 32 into the memory at an offset from a0, then jumps to another point in the code.
- `align_tile_coordinates_2`: Adjusts tile X and Y coordinates to grid boundaries based on comparison results, updates flags in a structure, and processes a sequence of tile entries until a termination marker is encountered.
- `frag_ori_bra_13`: Short code fragment that merges 64 into the memory at an offset from a0, then jumps to another point in the code.
- `update_tile_attributes`: Updates tile attributes based on comparison results and bit operations, then processes a sequence of tile entries until a termination marker is found.
- `frag_cmp_blt_ori_adda_addq_bra`: Short code fragment that compares d1 with 8, then branches if the comparison was less, then merges 1024 into the memory at an offset from a0, then adds 16 to a0, then adds 16 to a3, then adds 1 to d5, then jumps to another point in the code.
- `frag_clr_tst_beq_bra`: Short code fragment that clears the memory at an offset from a6, then tests the memory at an offset from a6, then branches if the last result was zero, then clears the memory at an offset from a6 (2 times in a row), then jumps to another point in the code.
- `update_digit_counters`: Updates decimal digit counters for a value stored at offset 168 in a structure, handling carry propagation between digits and updating the tens digit based on a multiplier from offset 147.
- `update_score_digits`: Updates decimal digit representations of a score value stored at offset 168 in a structure, handling carry propagation between digits and updating corresponding memory locations.
- `frag_move_clr_addi_subi_bcs_bra`: Short code fragment that copies d1 to the memory at a2 (advancing a2), then clears d1, then adds 10000 to d0, then subtracts 1000 from d0, then branches if the carry flag is set, then adds 1 to d1, then jumps to another point in the code.
- `frag_move_clr_addi_subi_bcs_bra_2`: Short code fragment that copies d1 to the memory at a2 (advancing a2), then clears d1, then adds 1000 to d0, then subtracts 100 from d0, then branches if the carry flag is set, then adds 1 to d1, then jumps to another point in the code.
- `frag_move_clr_addi_subi_bcs_bra_3`: Short code fragment that copies d1 to the memory at a2 (advancing a2), then clears d1, then adds 100 to d0, then subtracts 10 from d0, then branches if the carry flag is set, then adds 1 to d1, then jumps to another point in the code.
- `update_bomb_counters_2`: Updates two bomb-related counters by adding values from a structure, adjusting for overflow, and storing the results back.
- `update_bomb_display`: Updates bomb display properties based on a global counter and structure offsets, then configures related game object attributes.
- `update_bomb_status_6`: Adjusts a byte value at offset 78 of a structure, then configures and invokes another routine with specific parameters.
- `update_bomb_status_5`: Updates bomb-related status bytes based on input conditions and performs calculations involving global counters, then stores specific values into memory locations determined by offset calculations.
- `frag_addi_adda_move_bra`: Short code fragment that adds 2 to d0, then adds d0 to a2, then copies 131 to the memory at a2, then jumps to another point in the code.
- `frag_movea_move_adda_tst_bne_bra`: Short code fragment that copies the memory at an offset from a6 to a2, then copies the memory at an offset from a1 to d0, then adds d0 to a2, then tests the memory at an offset from a1, then branches if the last result was not zero, then copies 128 to the memory at a2, then copies 1 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_subi_cmp_bne_clr_bra`: Short code fragment that copies 130 to the memory at a2, then subtracts 1 from d1, then compares d1 with 128, then branches if the last result was not zero, then clears d1, then copies 1 to the memory at an offset from a6 (2 times in a row), then copies d1 to the memory at an offset from a1, then jumps to another point in the code.
- `frag_movea_move_cmp_beq_bne_clr_bra`: Short code fragment that copies the memory at an offset from a6 to a5, then copies the memory at a5 (advancing a5) to d6, then compares d6 with 65535, then branches if the last result was zero, then compares d6 with d7, then branches if the last result was not zero, then clears the memory at an offset from a1, then copies 2 to the memory at an offset from a6, then jumps to another point in the code.
- `update_entity_state`: Updates entity state based on input conditions and writes values to memory addresses calculated from global variables.
- `frag_move_bra_13`: Short code fragment that copies 86 to the memory at a2, then jumps to another point in the code.
- `frag_addi_adda_move_bra_2`: Short code fragment that adds 2 to d0, then adds d0 to a2, then copies 55 to the memory at a2, then jumps to another point in the code.
- `frag_movea_move_adda_tst_bne_cmpi_beq_b`: Short code fragment that copies the memory at an offset from a6 to a2, then copies the memory at an offset from a1 to d0, then adds d0 to a2, then tests the memory at an offset from a1, then branches if the last result was not zero, then compares the memory at an offset from a6 with 3, then branches if the last result was zero, then copies 62 to the memory at a2, then jumps to another point in the code.
- `frag_move_bra_14`: Short code fragment that copies 85 to the memory at a2, then jumps to another point in the code.
- `frag_move_subi_cmp_bne_addi_adda_bra`: Short code fragment that copies 54 to the memory at a2, then subtracts 1 from d1, then compares d1 with 64, then branches if the last result was not zero, then copies 1 to d1, then copies 2 to the memory at an offset from a6, then copies d1 to the memory at an offset from a1, then adds 1 to d7, then adds 8 to a1, then jumps to another point in the code.
- `update_bomb_status_4`: Updates bomb-related animation and status variables based on input conditions, then performs a series of calculations involving global count.
- `update_bomb_status_3`: Updates bomb-related animation and status variables based on input conditions, then performs a series of calculations involving global counters and table lookups before returning.
- `update_animation_frame_2`: Updates animation frame data by processing a table of values, updating global variables, and setting bits in a status byte based on calculated offsets.
- `load_and_clear`: Loads a word value from a global variable, clears two other global variables, then performs a table lookup using the loaded value as an index.
- `frag_clr_tst_beq_lea_bra`: Short code fragment that clears d0, then clears a fixed memory address, then clears the memory at an offset from a6 (2 times in a row), then tests the memory at an offset from a6, then branches if the last result was zero, then loads the address of the memory at an offset from a6 into a1, then loads the address of the memory at an offset from a6 into a2, then jumps to another point in the code.
- `update_sprite_data`: Updates sprite position and rendering data based on calculated offsets and copies tile information from two source tables into destination buffers.
- `frag_move_rte`: Short code fragment that copies 32 to a fixed memory address, then returns from the exception.
- `check_and_update_state`: Tests a word value at a specific offset in a structure, and if it is non-zero, copies a byte from another offset to a register and compares it with a constant.
- `frag_cmp_bne_move_bra_3`: Short code fragment that compares d0 with 80, then branches if the last result was not zero, then copies 1 to the memory at an offset from a6, then jumps to another point in the code.
- `update_game_state`: Checks if a specific condition is met and updates game state variables accordingly, then writes to a hardware interrupt register.
- `frag_cmp_bgt_clr_bra`: Short code fragment that compares d0 with 6, then branches if the comparison was greater, then clears d0, then jumps to another point in the code.
- `frag_cmp_bgt_move_bra_5`: Short code fragment that compares d0 with 30, then branches if the comparison was greater, then copies 1 to d0, then jumps to another point in the code.
- `frag_cmp_bgt_move_bra_6`: Short code fragment that compares d0 with 60, then branches if the comparison was greater, then copies 2 to d0, then jumps to another point in the code.
- `frag_cmp_bgt_move_bra_7`: Short code fragment that compares d0 with 110, then branches if the comparison was greater, then copies 2 to d0, then jumps to another point in the code.
- `frag_cmp_bgt_move_bra_8`: Short code fragment that compares d0 with 180, then branches if the comparison was greater, then copies 3 to d0, then jumps to another point in the code.
- `tiny_move_rts`: Small routine of 2 instructions that copies 4 to d0, then returns.
- `update_position_data`: Loads position coordinates from memory, performs boundary checks on a value, and updates memory locations based on comparison results.
- `calculate_position_data`: Clears a memory block, calculates position coordinates based on input values and flags, then stores the results in a data structure.
- `frag_cmp_bne_move_bra_4`: Short code fragment that compares d7 with 129, then branches if the last result was not zero, then copies 133 to the memory at an offset from a5, then copies 11 to the memory at an offset from a5, then copies 6 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_cmp_bne_move_addi_adda_clr_movem_r`: Short code fragment that compares d7 with 130, then branches if the last result was not zero, then copies 85 to the memory at an offset from a5, then copies 15 to the memory at an offset from a5, then adds 1 to the memory at an offset from a6, then adds 2 to a1, then copies the memory at a1 to d0, then clears d1, then restores saved registers from the stack, then returns.
- `process_tile_collision`: Reads tile data from memory, compares it against specific values, and updates counters or memory locations based on the comparison results.
- `frag_cmp_bne_addi_move_movea_bra`: Short code fragment that compares d1 with 7, then branches if the last result was not zero, then adds 1 to the memory at an offset from a6, then copies a5 to the stack, then copies the memory at an offset from a6 to a5, then copies 3 to the memory at a5 (advancing a5), then copies a5 to the memory at an offset from a6, then copies the top of the stack to a5, then jumps to another point in the code.
- `frag_cmp_bne_addi_move_movea_bra_2`: Short code fragment that compares d1 with 9, then branches if the last result was not zero, then adds 1 to the memory at an offset from a6, then copies a5 to the stack, then copies the memory at an offset from a6 to a5, then copies 3 to the memory at a5 (advancing a5), then copies a5 to the memory at an offset from a6, then copies the top of the stack to a5, then jumps to another point in the code.
- `validate_tile_coordinates`: Compares a word value with 13, loads data from a table, and performs conditional operations based on coordinate comparisons.
- `write_tile_data`: Loads a word value from memory, calculates an address offset, writes a specific byte value to that location, then manages stack and memory pointers.
- `frag_cmp_bne_move_movea_bra`: Short code fragment that compares d1 with 12, then branches if the last result was not zero, then copies a5 to the stack, then copies the memory at an offset from a6 to a5, then copies 3 to the memory at a5 (advancing a5), then copies a5 to the memory at an offset from a6, then copies the top of the stack to a5, then jumps to another point in the code.
- `frag_cmp_beq_bne_move_movea_rts`: Short code fragment that compares d1 with 5, then branches if the last result was zero, then compares d1 with 8, then branches if the last result was not zero, then copies a5 to the stack, then copies the memory at an offset from a6 to a5, then copies 2 to the memory at a5 (advancing a5), then copies a5 to the memory at an offset from a6, then copies the top of the stack to a5, then returns.
- `update_collision_state`: Updates collision detection state for game objects by checking position coordinates against a table, modifying object flags and counters based on matches, and setting a specific bit in a global structure.
- `frag_cmp_bne_move_bra_5`: Short code fragment that compares d1 with 24, then branches if the last result was not zero, then copies 65535 to d1, then jumps to another point in the code.
- `update_object_position`: Loads coordinates from a table using a shifted index, adds a constant to the x-coordinate, subtracts four from the y-coordinate, and stores the results in a structure pointed to by a0.
- `manage_animation_sequence`: Updates animation counters, manages stack pointers, and performs conditional operations based on input values and memory states.
- `manage_timer_events`: Writes a byte value to a memory location, decrements a timer value, and if the timer reaches zero, writes another byte value to the same location.
- `set_position_coordinates`: Calculates X and Y coordinate values based on input parameters and flags, stores them in memory, then sets fixed values in other memory locations.
- `process_game_coordinates`: Loads position data from memory, applies bitwise operations to coordinates, and updates memory locations with calculated values.
- `frag_cmp_bne_addi_adda_cmpi_beq_bra`: Short code fragment that compares d4 with 6, then branches if the last result was not zero, then adds 32 to d1, then adds 16 to a0, then compares the memory at a0 with 32766, then branches if the last result was zero, then compares the memory at a0 with 32767, then branches if the last result was not zero, then adds 16 to a0, then jumps to another point in the code.
- `update_render_data`: Checks a condition and if met, updates position data, writes specific values to memory, manages a stack pointer, and sets timer values.
- `handle_collision_detection`: Performs collision checks on memory locations, calculates offsets, copies data from one location to another, updates memory with fixed values, and manages stack pointers.
- `calculate_score_value`: Processes multiple byte values from memory through arithmetic operations, performs multiplication and addition, then stores the final result in a memory location.
- `calculate_score`: Reads multiple byte values from offsets 109, 112, 110, 115, 114, 90-92 relative to a6, performs arithmetic operations including shifts, additions, and multiplications, and stores the final result at offset 36 of the structure in a6.
- `calculate_score_2`: Computes a score value by combining several byte values from offsets 139, 142, 140, 145, and 144 relative to a6, performs arithmetic operations including shifts and additions, clamps the result to a maximum of 5, adds this clamped value to a base calculation, and stores the final result at offset 36 of a6.
- `frag_btst_beq`: Short code fragment that tests bit 0 of a fixed memory address, then branches if the last result was zero.
- `frag_move_lea_clr_movea_rts`: Short code fragment that copies a0 to the stack, then loads the address of a fixed memory address into a0, then clears the memory at a0 (advancing a0) (2 times in a row), then loads the address of a fixed memory address into a0, then clears the memory at a0 (advancing a0) (2 times in a row), then copies the top of the stack to a0, then returns.
- `read_joystick_input`: Reads joystick input from a hardware register, performs bitwise operations to extract specific bits, compares the result with a value at offset 170 of a6, and sets a flag in register d7 based on the comparison result.
- `frag_cmp_bne_ori_bra_15`: Short code fragment that compares d0 with the memory at an offset from a6, then branches if the last result was not zero, then merges 4 into d7, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_16`: Short code fragment that compares d0 with the memory at an offset from a6, then branches if the last result was not zero, then merges 2 into d7, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_17`: Short code fragment that compares d0 with the memory at an offset from a6, then branches if the last result was not zero, then merges 1 into d7, then jumps to another point in the code.
- `frag_cmp_bne_ori_bra_18`: Short code fragment that compares d0 with the memory at an offset from a6, then branches if the last result was not zero, then merges 8192 into d7, then jumps to another point in the code.
- `frag_cmp_bne_ori_move_tst_or_bra`: Short code fragment that compares d0 with the memory at an offset from a6, then branches if the last result was not zero, then merges 49152 into d7, then copies the memory at an offset from a6 to d6, then tests d1, then branches if the last result was not zero, then merges d7 into d6, then jumps to another point in the code.
- `frag_not_and_move_movem_rts`: Short code fragment that inverts d7, then masks d6 with d7, then copies d6 to the memory at an offset from a6, then restores saved registers from the stack, then returns.
- `update_position_flags`: Reads two 16-bit values from offsets 362 and 364 relative to a6, performs bitwise operations including masking and shifting, updates a value at offset 364 of a6 based on a condition, then reads another value from offset 362 and applies a mask before storing it back to offset 360 of a6.
- `handle_joystick_state`: Reads joystick state from hardware registers, performs bitwise operations to determine button states, updates counters at offsets 356 and 358 relative to a6 based on conditions, and stores the final combined state in a word at offset 354 of a6.
- `update_animation_frame_3`: Increments an animation counter, compares values from memory, and updates frame data in a table structure.
- `update_animation_counter`: Increments an animation counter at offset 15c0 relative to a6, writes values from registers d1 and d2 to memory locations pointed to by a1, and manages a loop that continues until the counter reaches a specific value.
- `frag_move_rts`: Short code fragment that copies a1 to the memory at an offset from a6, then returns.
- `handle_input_buffer`: Processes input buffer data, updates counters, and manages memory locations based on conditional checks and data comparisons.
- `process_input_buffer`: Checks if a word at the address in a1 equals -1, and if so, copies a value from a1 to offset 354 of a6, decrements a counter at offset 5574 of a6, and updates several memory locations based on conditions.
- `read_joystick_data`: Reads joystick input from hardware registers, performs bitwise operations to extract button states, and stores the results in memory.
- `render_sprite_pixels`: Calculates pixel data for sprite rendering by reading from a table, performs bit manipulation operations including shifts and masks, writes pixel values to memory locations pointed to by a1, and updates multiple memory locations with specific byte values.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_graphics`: Sets up display and graphics registers including bitplane control, color tables, and copper list pointers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `frag_clr_addi_cmp_bne_movem_rts`: Short code fragment that clears d0, then adds 1 to d0, then compares d0 with an address, then branches if the last result was not zero, then adds 1 to d1, then compares d1 with a fixed memory address, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `frag_move_clr_addi_cmp_bne_rts_2`: Short code fragment that copies d0 to the stack, then clears d0, then adds 1 to d0, then compares d0 with an address, then branches if the last result was not zero, then copies the top of the stack to d0, then returns.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_2`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_3`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_4`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_5`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_6`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_7`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_8`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_9`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_10`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_11`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_12`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_13`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_14`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_15`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_16`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_17`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_18`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_19`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_20`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_21`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_22`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_23`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_24`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_25`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_26`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_27`: Padding: one no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
