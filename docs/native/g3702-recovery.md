# Game 3702 (formula 1 3d (1991)(simulmondo)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03702). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (910128 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 8 raw-word lines |
| Functions named and described | pass, 86 of 86 named (0 author symbols), 86 of 86 described (36 by rule, 47 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 165 of 165 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 5120 bytes, 34 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1844 of 1844 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 11449 |
| code | 7410 |
| data | 887173 |
| unused | 4096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Formula 1 3D (1991)(Simulmondo).adf:bootblock | 1 | 287 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 481 | 38 |
| Formula 1 3D (1991)(Simulmondo).adf bytes 1024-6144 (code never loaded by a reference run) | 1 | 1076 | 34 |

## Cases handled

- formula 1 3d (1991)(simulmondo)/Formula 1 3D (1991)(Simulmondo).adf: custom-format (non-DOS) disk, 1750 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 165 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- formula 1 3d (1991)(simulmondo)/Formula 1 3D (1991)(Simulmondo).adf: bytes 1024-6144 look like code no run loaded: recovered position-independently (load address unknown)
- formula 1 3d (1991)(simulmondo)/Formula 1 3D (1991)(Simulmondo).adf: 1 logged loads cover 5120 bytes of the image
- formula 1 3d (1991)(simulmondo)/Formula 1 3D (1991)(Simulmondo).adf: 16 track-loaded blocks found in recovered runtime segments and classified through them
- formula 1 3d (1991)(simulmondo)/Formula 1 3D (1991)(Simulmondo).adf: blocks 13-276 (bytes 6656-141824) outside the filesystem hold data, not code (opcode density 0.025, entropy 7.10)
- formula 1 3d (1991)(simulmondo)/Formula 1 3D (1991)(Simulmondo).adf: blocks 285-285 (bytes 145920-146432) outside the filesystem hold data, not code (opcode density 0.047, entropy 7.06)
- formula 1 3d (1991)(simulmondo)/Formula 1 3D (1991)(Simulmondo).adf: blocks 294-294 (bytes 150528-151040) outside the filesystem hold data, not code (opcode density 0.039, entropy 4.47)
- formula 1 3d (1991)(simulmondo)/Formula 1 3D (1991)(Simulmondo).adf: blocks 296-308 (bytes 151552-158208) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.49)
- formula 1 3d (1991)(simulmondo)/Formula 1 3D (1991)(Simulmondo).adf: blocks 311-319 (bytes 159232-163840) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.15)
- 1 packed track region(s), 735232 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_dma_controller` (bootblock): Configures DMA controller registers and manages hardware bit states for video timing and display synchronization. Kind: disk hardware check. Pass path: The routine manipulates CIAB_PRB to control disk drive signals and then waits for specific status bits (likely involving motor control or sector sync detection). If the check passes, it proceeds to set up DMA and loads data. The key indicat.
- `configure_io_ports` (bootblock): Sets up I/O port configurations by modifying bits in hardware registers and writes a value to a specific memory location. Kind: disk hardware check. Pass path: CIAB_PRB is configured to enable disk drive motor control and read/write operations, with specific bit manipulations suggesting a track or sector validation check.
- `toggle_dma_bits` (bootblock): Toggles specific bits in a hardware register based on input values and performs conditional updates to a global variable, followed by a loop that repeatedly modifies the same register. Kind: disk hardware check. Pass path: beq.b an address.
- `process_display_loop` (runtime segment an address): Executes a display processing loop that handles input state management, processes text rendering, and manages hardware status flags while continuing until a specific condition is met. Kind: track/checksum. Pass path: the loop continues until sVar3 == 0, and if (bVar2 & 4) is set, func_0x000009b0() is called — suggesting a track or sector validation check that must pass for the program to proceed.
- `initialize_joystick` (Formula 1 3D (1991)(Simulmondo).adf bytes 1024-6144 (code never loaded by a reference run)): Configures joystick control registers by setting and clearing specific bits, then checks a global flag before initializing joystick input and updating a counter. Kind: disk hardware check. Pass path: The routine initializes joystick hardware by manipulating CIAB_PRB bits and performs a loop that checks for specific bit patterns in CIAB_PRB. If the check passes (i.e., the expected bit pattern is detected), it sets up the joystick interfa.
- `configure_io_ports_2` (Formula 1 3D (1991)(Simulmondo).adf bytes 1024-6144 (code never loaded by a reference run)): Configures I/O port settings by setting and clearing specific bits in hardware registers, followed by writing a value to a global variable. Kind: disk track check. Pass path: the routine modifies CIAB_PRB in a sequence that suggests checking for specific hardware or disk state, and the call from sub_06A2 implies this is part of a validation routine; if the check passes, execution continues normally (the 'rts' af.
- `update_io_status` (Formula 1 3D (1991)(Simulmondo).adf bytes 1024-6144 (code never loaded by a reference run)): Updates hardware I/O status by setting or clearing specific bits in control registers based on input values and current state, then performs delay operations. Kind: disk hardware check. Pass path: The check passes when the condition `sVar1 == sVar2` is true, meaning the routine returns early without modifying CIAB_PRB bits or calling `u2__sub_096A()`. If the check fails (i.e., `sVar1 != sVar2`), it proceeds to modify CIAB_PRB and cal.

## Functions


### Formula 1 3D (1991)(Simulmondo).adf:bootblock

- `initialize_system_registers`: Initializes system registers including DMA and interrupt control, calls a validation routine, stores values in global variables, and then executes a trap instruction.
- `copy_memory_block`: Copies 256 words from one memory location to another using a loop that increments both source and destination pointers while decrementing a counter.
- `initialize_data_structure_2_2`: Sets up a data structure by calculating a quotient and remainder from a word value, storing the results at specific offsets in the structure.
- `validate_memory_limits`: Checks memory limits from a structure and returns specific values based on comparison results, clearing a global pointer before the checks.
- `initialize_data_structure_2`: Sets up a data structure by calculating a quotient and remainder from a word value, storing the results at specific offsets in the structure, and copying pointer values to other locations within the same structure.
- `validate_data_structure`: Validates a data structure by checking specific magic numbers and performing checksum calculations to verify integrity.
- `compute_checksum_and_compare`: Processes data through multiple iterations to compute a checksum value, then compares it with another value and returns a boolean result.
- `frag_and_add_or_rts`: Short code fragment that masks d0 with d7, then masks d1 with d7, then adds d0 to d0, then merges d1 into d0, then returns.
- `copy_word_data`: Copies word values from one memory location to another while decrementing a counter, updating pointers and returning a boolean based on completion status.
- `initialize_hardware_registers`: Waits for a specific bit in a hardware register to clear, then writes multiple values to various hardware registers including joystick/mouse status.
- `setup_dma_controller`: Configures DMA controller registers and manages hardware bit states for video timing and display synchronization.
- `configure_io_ports`: Sets up I/O port configurations by modifying bits in hardware registers and writes a value to a specific memory location.
- `toggle_dma_bits`: Toggles specific bits in a hardware register based on input values and performs conditional updates to a global variable, followed by a loop that repeatedly modifies the same register.
- `write_dma_registers`: Writes byte values to DMA control registers, sets up timing values, and waits for a specific bit to be set before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_display_loop`: Executes a display processing loop that handles input state management, processes text rendering, and manages hardware status flags while continuing until a specific condition is met.
- `frag_movea_move_dbra_rts`: Short code fragment that copies the memory at an offset from a4 to a0, then copies the memory at an offset from a4 to a1, then copies 10239 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `process_input_buffer`: Processes input bytes from a buffer, updates cursor position, handles special control characters, and manages text input state.
- `frag_movem_asl_mulu_bsr_rts`: Short code fragment that saves registers on the stack, then shifts d0 left by 3, then multiplies d1 by 12 (unsigned), then calls another routine, then restores saved registers from the stack, then returns.
- `frag_move_bsr_mulu_rts`: Short code fragment that copies d1 to the stack, then calls another routine, then multiplies d1 by 12 (unsigned), then calls another routine, then copies the top of the stack to d1, then returns.
- `frag_lea_clr_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then clears the memory at an offset from a0 (3 times in a row), then clears the memory at an offset from a4, then copies 1 to the memory at an offset from a4, then returns.
- `calculate_rendering_parameters`: Calculates rendering parameters based on input values and stores them into a structure, including position and setting data.
- `frag_moveq_bra`: Short code fragment that loads 13 into d5, then jumps to another point in the code.
- `frag_cmp_bgt_exg_moveq_bra`: Short code fragment that compares d3 with d2, then branches if the comparison was greater, then exchanges d2 and d3, then loads 25 into d5, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 5 into d5, then jumps to another point in the code.
- `frag_sub_bgt_neg_cmp_exg_moveq_bra`: Short code fragment that subtracts d0 from d2, then branches if the comparison was greater, then negates d2, then compares d3 with d2, then branches if the comparison was greater, then exchanges d2 and d3, then loads 21 into d5, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 9 into d5, then jumps to another point in the code.
- `frag_cmp_bgt_exg_moveq_bra_2`: Short code fragment that compares d3 with d2, then branches if the comparison was greater, then exchanges d2 and d3, then loads 17 into d5, then jumps to another point in the code.
- `update_rendering_data`: Updates rendering parameters for graphical objects by calculating positions and settings based on input values, then stores these values into a structure referenced through a6.
- `copy_and_process_data`: Processes input data by copying and transforming it into multiple output buffers, then updates global variables with computed values based on input and register contents.
- `frag_movem_lea_tst_bpl_move_neg_bra`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a0, then tests d3, then branches if the last result was not negative, then copies 45 to the memory at a0 (advancing a0), then negates d3, then jumps to another point in the code.
- `frag_move_lea_moveq_sub_bmi_addq_bra`: Short code fragment that copies 43 to the memory at a0 (advancing a0), then loads the address of a location in the code into a1, then loads 4 into d4, then loads 0 into d5, then copies the memory at a1 (advancing a1) to d5, then loads 48 into d6, then subtracts d5 from d3, then branches if the last result was negative, then adds 1 to d6, then jumps to another point in the code.
- `frag_add_move_dbra_lea_bsr_movem_rts`: Short code fragment that adds d5 to d3, then copies d6 to the memory at a0 (advancing a0), then loops back while counting down d4, then loads the address of a location in the code into a0, then calls another routine, then restores saved registers from the stack, then returns.
- `process_input_byte`: Reads a byte from a global variable, clears its high bit, compares it with another global value, and returns zero if they match.
- `check_joystick_fire`: Checks the joystick fire button bit in CIAA_PRA register, and based on its state, sets bits in a return value while performing several comparisons with joystick data.
- `frag_cmp_bne_moveq_or_rts`: Short code fragment that compares d1 with 2, then branches if the last result was not zero, then loads 10 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_2`: Short code fragment that compares d1 with 1, then branches if the last result was not zero, then loads 8 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_3`: Short code fragment that compares d1 with 769, then branches if the last result was not zero, then loads 9 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_4`: Short code fragment that compares d1 with 768, then branches if the last result was not zero, then loads 1 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_5`: Short code fragment that compares d1 with 512, then branches if the last result was not zero, then loads 5 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_6`: Short code fragment that compares d1 with 256, then branches if the last result was not zero, then loads 4 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_7`: Short code fragment that compares d1 with 259, then branches if the last result was not zero, then loads 6 into d1, then merges d1 into d0, then returns.
- `check_mouse_fire`: Checks the mouse fire button bit in CIAA_PRA register, and based on its state, sets bits in a return value while performing several comparisons with mouse data.
- `frag_cmp_bne_moveq_or_rts_8`: Short code fragment that compares d1 with 2, then branches if the last result was not zero, then loads 10 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_9`: Short code fragment that compares d1 with 1, then branches if the last result was not zero, then loads 8 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_10`: Short code fragment that compares d1 with 769, then branches if the last result was not zero, then loads 9 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_11`: Short code fragment that compares d1 with 768, then branches if the last result was not zero, then loads 1 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_12`: Short code fragment that compares d1 with 512, then branches if the last result was not zero, then loads 5 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_13`: Short code fragment that compares d1 with 256, then branches if the last result was not zero, then loads 4 into d1, then merges d1 into d0, then returns.
- `frag_cmp_bne_moveq_or_rts_14`: Short code fragment that compares d1 with 259, then branches if the last result was not zero, then loads 6 into d1, then merges d1 into d0, then returns.
- `adjust_character_position`: Processes character position adjustments by iterating through a lookup table, modifying a counter based on table values, and storing results in memory.
- `frag_moveq_movea_tst_beq_addq_bra_asl_n`: Short code fragment that loads 0 into d0, then copies a0 to a1, then tests the memory at a1 (advancing a1), then branches if the last result was zero, then adds 1 to d0, then jumps to another point in the code, then shifts d0 left by 2, then negates d0, then adds 161 to d0, then returns.
- `calculate_cross_product`: Performs vector cross product calculations using values from a data structure, returning the result of a mathematical operation involving multiple coordinates.

### Formula 1 3D (1991)(Simulmondo).adf bytes 1024-6144 (code never loaded by a reference run)

- `setup_hardware_and_load_data`: Initializes hardware registers including DMA and interrupt control, sets up memory pointers, and calls multiple routines to load and configure game data.
- `frag_movem_lea_move_and_andi_rte`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a6, then copies the memory at an offset from a6 to d0, then masks d0 with the memory at an offset from a6, then masks d0 with 16383, then copies d0 to the memory at an offset from a6 (2 times in a row), then restores saved registers from the stack, then returns from the exception.
- `frag_subq_bmi_st_tst_bne_dbra_rts`: Short code fragment that subtracts 1 from d0, then branches if the last result was negative, then sets a fixed memory address to all ones, then tests a fixed memory address, then branches if the last result was not zero, then loops back while counting down d0, then returns.
- `initialize_data_structure`: Initializes a data structure by writing sequential values to memory locations and setting specific register values for later use in graphics or game logic.
- `update_graphics_buffer`: Updates a graphics buffer by copying data from source memory, performing bit manipulation on input values, and calculating memory addresses for the copy operation.
- `copy_color_data`: Copies word-sized color component values from one location to another in a nested loop structure, updating pointers and decrementing counters at each level.
- `process_color_components`: Processes color component values through multiple bitwise operations and conditional adjustments, modifying input values based on comparison results to produce output color values.
- `update_pixel_matrix`: Updates a matrix of pixel values by calling another routine for each element, decrementing counters in nested loops before returning.
- `adjust_pixel_value`: Adjusts a pixel value by performing bitwise operations and conditional subtractions, modifying the input value through multiple stages of bit manipulation.
- `configure_hardware_registers`: Initializes multiple hardware registers with predefined values, performs conditional operations based on bit tests, and manages data copying through a series of subroutine calls while handling loop control logic.
- `frag_btst_bne_move_clr_rts`: Short code fragment that tests bit 6 of the memory at an offset from a6, then branches if the last result was not zero, then copies 57344 to the memory at an offset from a6, then copies an address to the memory at an offset from a6, then clears the memory at an offset from a6, then copies 32788 to the memory at an offset from a6, then returns.
- `frag_moveq_movea_tst_beq_addq_bra_asl_n_2`: Short code fragment that loads 0 into d0, then copies a0 to a1, then tests the memory at a1 (advancing a1), then branches if the last result was zero, then adds 1 to d0, then jumps to another point in the code, then shifts d0 left by 2, then negates d0, then adds 161 to d0, then returns.
- `frag_movea_lea_move_bsr_rts`: Short code fragment that copies a0 to a2, then loads the address of the memory at an offset from a2 into a0, then copies d1 to the memory at a0 (advancing a0), then loads the address of a fixed memory address into a1, then calls another routine, then loads the address of the memory at an offset from a2 into a0, then copies a2 to a1, then calls another routine, then returns.
- `process_render_data`: Reads structured data from a buffer, processes bit shifts and counters, writes bytes to a destination pointer, and performs conditional operations based on counter values.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (moveq) and then falls through into the code that follows, with no return of its own.
- `decrement_and_shift`: Decrements a counter and performs bit shifting operations on another value while decrementing a byte register until it reaches zero, then returns.
- `setup_configuration_data`: Calculates quotient and remainder values from input parameters, stores these results in a configuration structure, copies pointer values to specific memory locations, and executes a series of initialization routines with conditional loop control.
- `validate_checksum_2`: Processes data through a series of bitwise operations and comparisons to validate a checksum, returning a boolean result based on the comparison.
- `validate_checksum`: Processes data through a series of bitwise operations and comparisons to validate a checksum, returning a boolean result based on the comparison between computed and expected values.
- `frag_and_add_or_rts_2`: Short code fragment that masks d0 with d7, then masks d1 with d7, then adds d0 to d0, then merges d1 into d0, then returns.
- `copy_data_block`: Copies word-sized data from one memory location to another while managing counters and handling conditional branch logic to control the copy operation.
- `initialize_hardware`: Initializes hardware registers by writing specific values to various address locations, including handling joystick input status before proceeding with register writes.
- `initialize_joystick`: Configures joystick control registers by setting and clearing specific bits, then checks a global flag before initializing joystick input and updating a counter.
- `configure_io_ports_2`: Configures I/O port settings by setting and clearing specific bits in hardware registers, followed by writing a value to a global variable.
- `update_io_status`: Updates hardware I/O status by setting or clearing specific bits in control registers based on input values and current state, then performs delay operations.
- `write_io_registers`: Writes data to hardware I/O registers using byte-sized operations, then waits for a specific bit status before returning from the routine.
- `render_text_glyphs`: Processes character data to render text glyphs by copying character patterns from a table into two output buffers, then configures display parameters for text rendering including position and color settings.
- `frag_move_bsr_st_rts`: Short code fragment that copies a0 to a fixed memory address, then calls another routine, then sets a fixed memory address to all ones, then returns.
- `frag_clr_bsr_rts`: Short code fragment that clears a fixed memory address, then calls another routine, then returns.
- `initialize_audio_channels`: Initializes audio channel data structures and clears volume registers while setting up hardware state for audio processing.
- `clear_audio_registers`: Clears audio volume registers for four channels, sets a DMA control register to a specific value, and clears a bit in the CIA port A register before returning.
- `update_input_state_2`: Processes input state based on control data and updates position values in a structure.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to twelve bits, stores the masked value into an offset of the same structure and clears a byte at another offset.
- `update_input_state`: Processes input state based on control data and updates position values in a structure.
