# Game 2680 (Amiga Poker): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02680). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901216 top-level bytes in 17 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 114 of 114 named (0 author symbols), 114 of 114 described (38 by rule, 75 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 751 of 751 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 359792 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4809 of 4809 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 246345 |
| code | 27420 |
| data | 425544 |
| padding | 3763 |
| unused | 198144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Amiga Poker.adf:creator | 2 | 581 | 13 |
| Amiga Poker.adf:willi | 2 | 30 | 1 |
| Amiga Poker.adf:go | 2 | 4179 | 97 |
| Amiga Poker.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9 | 2 |
| Amiga Poker.adf:total (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 4 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Amiga Poker/Amiga Poker.adf:total: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- Amiga Poker/Amiga Poker.adf: blocks 922-928 (bytes 472064-475648) outside the filesystem hold data, not code (opcode density 0.033, entropy 3.07)
- 1 never-loaded region(s), 359792 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_graphics_mode` (go): Initializes graphics hardware registers including copper lists, display window settings, and bitplane configurations by loading specific values into Amiga custom chip registers. Kind: disk hardware check. Pass path: execution continues with no error, DSKBYTR is set to 0 and other custom registers are initialized normally.
- `initialize_graphics_system` (go): Initializes graphics by opening a library, copying data to a buffer, setting up display registers, and configuring bitplane DMA parameters. Kind: disk hardware check. Pass path: CUSTOM_DSKBYTR is set to 0 and routine continues normally.

## Functions


### Amiga Poker.adf:creator

- `initialize_graphics`: Initializes graphics data by parsing a bitmap header, extracting dimensions and color information, then setting up DMA transfer configurations for pixel data.
- `decode_data_blocks`: Processes data blocks by copying bytes from a source buffer to a destination, handling both regular and negated byte sequences based on a control flag, and updates a counter for each block.
- `copy_data_blocks`: Copies data from one memory location to multiple destination buffers based on a configuration value, using a loop that processes rows and columns.
- `initialize_graphics_data`: Initializes multiple graphics-related data tables and sets up DMA transfer configurations, including loading pixel data from files and configuring audio registers.
- `frag_movem_btst_bne_bra`: Short code fragment that saves registers on the stack, then tests bit 5 of a fixed memory address, then branches if the last result was not zero, then jumps to another point in the code.
- `initialize_blitter`: Initializes blitter registers with specific values and performs a sequence of operations involving hardware register modifications and memory updates, including setting up blitter control words and waiting for blitter completion before proceeding.
- `frag_move_bra`: Short code fragment that copies an address to a fixed memory address, then jumps to another point in the code.
- `update_game_state`: Increments a counter and performs conditional operations based on the current state, updating pointers and counters in memory.
- `update_game_counters`: Increments a global counter at offset 340 and decrements another at offset 348, then performs arithmetic operations using a value from a memory location to calculate a new value that gets stored in a register before calling a library routine.
- `frag_move_bra_2`: Short code fragment that copies an address to a fixed memory address, then jumps to another point in the code.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `frag_movem_jmp`: Short code fragment that restores saved registers from the stack, then jumps to another point in the code.

### Amiga Poker.adf:willi

- `initialize_system`: Sets up system variables by storing the current stack pointer and processor state, allocates memory using system calls, and handles message processing from a task's message port.

### Amiga Poker.adf:go

- `initialize_graphics_and_audio`: Initializes graphics mode by setting up copper lists, display windows, and bitplane configurations, while also configuring audio registers for sound output.
- `poll_joystick_and_write`: Reads the joystick/mouse fire button status from a hardware register, writes data to a file descriptor using a system call, and loops until the button is released.
- `write_audio_data`: Writes multiple values to a global variable through a file handle using the dos.Write function, then waits for a button press by polling a hardware register until it indicates the button is released.
- `delay_loop`: Executes a countdown loop that decrements a register value until it reaches zero, effectively creating a delay mechanism.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (addq.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `decode_multiple_blocks`: Decodes data from three different memory locations using a compression algorithm and updates the source pointers after each decode operation.
- `decode_data_block`: Processes data from one memory location to another using a compression algorithm that handles both literal bytes and repeated byte sequences based on control values.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `setup_graphics_mode`: Initializes graphics hardware registers including copper lists, display window settings, and bitplane configurations by loading specific values into Amiga custom chip registers.
- `setup_graphics_system`: Initializes graphics registers and display parameters, performs multiple conditional checks and updates to various memory locations, then calls several subroutines to configure audio and update game state.
- `frag_move_cmp_bne_rts_bsr_bra`: Short code fragment that copies a fixed memory address to d1, then compares d1 with d0, then branches if the last result was not zero, then returns, then calls another routine (2 times in a row), then jumps to another point in the code.
- `update_color_index`: Increments a global word variable and writes specific color register values to a hardware address based on the variable's value, resetting the variable to zero if it reaches a certain threshold.
- `configure_dma_parameters_2`: Updates display register values through a series of conditional checks and memory writes, calling another routine after each update to configure DMA parameters.
- `update_game_state_2`: Updates display register values in a loop based on a counter, then performs audio and DMA configuration before returning.
- `configure_audio_channels`: Sets up audio channel parameters including sample addresses, lengths, periods, and volumes for two audio channels, then enables DMA for audio output by writing to a hardware register.
- `configure_audio_system`: Sets up audio channel parameters including sample addresses, lengths, periods, and volumes for two audio channels, then enables DMA for audio output.
- `setup_audio_parameters`: Configures audio channel settings including sample addresses, lengths, periods, and volumes for two audio channels, then enables DMA for audio output by writing to a hardware register.
- `setup_audio_dma`: Configures audio DMA parameters by writing predefined values to multiple display register locations and enables DMA channels.
- `update_dma_parameters`: Increments a global counter and configures multiple display register values while updating a memory address based on the counter's value, then calls another routine and resets the counter when it exceeds a threshold.
- `configure_display_registers`: Increments a global counter and sets fixed display register values before calling a configuration routine, then resets the counter to zero when it exceeds another threshold.
- `process_input_events`: Reads input data from memory locations, checks against specific byte values using comparison operations, and updates a global variable with different values based on the results of these comparisons.
- `configure_audio_with_parameter`: Sets up audio channel parameters including sample addresses, lengths, periods, and volumes for two audio channels using a parameter value from a register, then enables DMA for audio output by writing to a hardware register.
- `setup_audio_channels`: Configures two audio channels with specific waveform parameters and enables DMA for audio output.
- `initialize_audio_sequence`: Configures audio period registers based on global state values, updates counters through subroutine calls, and manages audio DMA setup before returning.
- `render_digits_loop`: Sets fixed display register values and processes a sequence of bytes from a memory buffer by mapping each digit to an address and calling a configuration routine for each, updating a memory address with specific values between calls.
- `render_digits_with_offset`: Sets fixed display register values and processes a sequence of bytes from a memory buffer by mapping each digit to an address and calling a configuration routine for each, updating a memory address with specific values between calls, starting from an offset.
- `initialize_audio_channels`: Initializes audio channel registers with fixed values and enables DMA channels, then waits for a specified number of iterations before updating a memory location with a value from another location.
- `initialize_audio_dma`: Sets up audio channel parameters for a sound effect and enables DMA for audio output.
- `frag_move_rts`: Short code fragment that copies 3 to a fixed memory address, then returns.
- `update_game_state_3`: Checks conditions against global variables and updates game state based on comparisons, then calls display and audio update routines while modifying memory values.
- `process_game_logic`: Processes game state by checking conditions against memory values and updating counters, then calls multiple subroutines to handle display and audio updates while modifying register values.
- `update_display_counter`: Updates a display counter by checking conditions against memory values and incrementing a counter, then calls audio and display routines to render updated values when specific conditions are met.
- `render_digits_with_double_offset`: Sets fixed display register values and processes a sequence of bytes from a memory buffer by mapping each digit to an address and calling a configuration routine for each, updating a memory address with specific values between calls, starting from a double offset.
- `render_digits_with_triple_offset`: Sets fixed display register values and processes a sequence of bytes from a memory buffer by mapping each digit to an address and calling a configuration routine for each, updating a memory address with specific values between calls, starting from a triple offset.
- `handle_timer_events`: Handles different timer-based events by comparing a value with thresholds and performing decrement operations, then calling display update routines with specific parameters based on the matched condition.
- `frag_move_bsr_rts`: Short code fragment that copies an address to a fixed memory address, then copies 661 to a fixed memory address, then copies 358 to a fixed memory address, then copies 40 to a fixed memory address, then copies 32769 to a fixed memory address, then calls another routine, then copies 1920 to a fixed memory address, then calls another routine, then returns.
- `frag_move_bsr_rts_2`: Short code fragment that copies an address to a fixed memory address, then copies 661 to a fixed memory address, then copies 40 to a fixed memory address, then copies 32769 to a fixed memory address, then calls another routine, then copies 1920 to a fixed memory address, then calls another routine, then returns.
- `frag_move_bsr_rts_3`: Short code fragment that copies an address to a fixed memory address, then copies 661 to a fixed memory address, then copies 40 to a fixed memory address, then copies 32770 to a fixed memory address, then calls another routine, then returns.
- `frag_move_bsr_rts_4`: Short code fragment that copies an address to a fixed memory address, then copies 661 to a fixed memory address, then copies 20 to a fixed memory address, then copies an address to a fixed memory address, then copies 661 to a fixed memory address, then copies 20 to a fixed memory address, then copies 32780 to a fixed memory address, then calls another routine, then returns.
- `frag_movea_move_btst_beq_bsr_rts`: Short code fragment that copies a fixed memory address to a4, then copies the memory at a4 to a fixed memory address, then tests bit 7 of a fixed memory address, then branches if the last result was zero, then copies 1 to a fixed memory address, then calls another routine, then returns.
- `frag_movea_move_btst_beq_rts`: Short code fragment that copies a fixed memory address to a4, then copies the memory at a4 to a fixed memory address, then tests bit 7 of a fixed memory address, then branches if the last result was zero, then copies 1 to a fixed memory address, then returns.
- `update_display_registers`: Updates multiple display register values by loading digit characters from a table and processing them through a subroutine, while incrementing a memory address for each register update.
- `initialize_display_registers`: Sets specific values in global variables controlling display parameters and processes a sequence of bytes from a memory buffer by calling a routine for each byte while updating a memory address with fixed values.
- `map_digit_to_address`: Maps a digit character to a memory address and stores that address in a global variable based on the input character.
- `initialize_audio_registers`: Initializes several audio-related registers with zero values, sets up a memory address pointer, performs division operations on a register, and calls another routine multiple times.
- `format_digit_and_setup`: Adds a constant to a register, stores the result as a character in a buffer, clears and swaps the register, then sets up audio DMA parameters and registers.
- `frag_clr_add_cmpi_beq_rts`: Short code fragment that clears d0, then adds a fixed memory address to d0 (6 times in a row), then compares d0 with 6, then branches if the last result was zero, then returns.
- `frag_clr_add_cmpi_beq_rts_2`: Short code fragment that clears d0, then adds a fixed memory address to d0 (6 times in a row), then compares d0 with 6, then branches if the last result was zero, then returns.
- `configure_audio_dma`: Sets up audio DMA parameters with specific frequency and volume settings, then enables DMA channels for audio playback.
- `configure_audio_and_dma`: Sets up audio DMA parameters based on a global variable's value, configures audio registers for two channels, enables DMA, and performs a loop of register updates before disabling DMA.
- `handle_audio_dma_setup`: Configures audio DMA parameters and display registers, then waits for specific raster lines before enabling DMA channels.
- `configure_audio_dma_2`: Sets up audio DMA parameters by writing specific values to audio register addresses, configures display DMA control registers, and waits for vertical blanking intervals using a loop that checks joystick status before proceeding.
- `configure_display_dma`: Configures DMA display parameters by writing specific values to memory addresses representing display registers, then calls two other routines multiple times while updating the register values in a loop.
- `setup_video_dma`: Sets up video DMA parameters by writing predefined values to display register memory locations, repeatedly calls two routines with updated register values, and concludes with an additional routine call before returning.
- `compare_registers`: Compares values from multiple memory locations and branches to a label when values match, otherwise continues execution.
- `compare_register_values`: Compares a word value from a memory location with multiple other word values from different memory locations.
- `compare_register_values_2`: Compares a word value from a memory location against multiple other word values from different memory locations, returning immediately if any comparison matches, otherwise it returns without performing additional operations.
- `compare_word_values`: Compares a word value from a fixed memory location against multiple other word values from different memory locations, returning immediately if any match is found.
- `compare_word_values_loop`: Compares a word value from a fixed memory location against multiple other word values from different memory locations, returning immediately if any match is found, with a loop structure.
- `update_audio_parameters`: Writes fixed values to multiple memory locations, calls other routines for audio parameter updates, compares word values from memory locations, and updates flags based on comparison results.
- `check_conditions_loop`: Executes a series of conditional checks on global variables, calling different routines based on comparison results in a loop that decrements a counter.
- `configure_dma_parameters_3`: Reads a bit from the CIAA_PRA register and waits for it to become set, then configures DMA parameters based on the value in d4 and updates multiple global variables before calling another routine.
- `initialize_display_params`: Checks a global variable for specific values and sets display register parameters accordingly, then calls another routine twice with different memory addresses.
- `frag_andi_beq_movea_move_bne`: Short code fragment that masks a fixed memory address with 64, then branches if the last result was zero, then copies a fixed memory address to a4, then copies the memory at a4 to a fixed memory address, then masks a fixed memory address with 64, then branches if the last result was not zero.
- `validate_character_range`: Validates if two input bytes fall within specific ranges and returns a corresponding word value, or -1 if they don't match any range.
- `configure_dma_parameters_4`: Sets up DMA configuration parameters by writing specific values to display register addresses and calling another routine multiple times with different register values.
- `initialize_card_values`: Initializes card value counters and processes card data from a table, setting specific counters based on comparisons with card values.
- `configure_dma_parameters_5`: Sets up DMA parameters by writing values to memory locations and calling configuration routines, with conditional branches based on register comparisons.
- `conditional_dma_setup`: Conditionally configures DMA parameters based on comparisons between a global variable and values from memory, then calls another routine with updated register values.
- `conditional_dma_setup_v2`: Similar to the second routine but uses different memory offsets for comparison checks before configuring DMA parameters and calling another routine.
- `configure_dma_parameters`: Calls a DMA configuration routine multiple times while incrementing two memory addresses, updating display register values between calls.
- `configure_bitplane_dma`: Waits for a bitplane DMA to complete, copies data from memory locations to display registers, and configures bitplane DMA parameters for screen rendering.
- `frag_move_bra_3`: Short code fragment that copies 1 to a fixed memory address, then jumps to another point in the code.
- `evaluate_poker_hand`: Analyzes poker hand values stored in global variables and assigns a score based on hand type.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `initialize_display_regs`: Initializes multiple display register values to a fixed constant, clears several global variables, and performs a series of operations involving comparisons and assignments based on card values and display states.
- `frag_bsr_move_rts`: Short code fragment that calls another routine, then copies an address to a fixed memory address, then calls another routine, then returns.
- `frag_bsr_move_rts_2`: Short code fragment that calls another routine, then copies an address to a fixed memory address, then calls another routine, then returns.
- `frag_bsr_move_rts_3`: Short code fragment that calls another routine, then copies an address to a fixed memory address, then calls another routine, then returns.
- `frag_bsr_move_rts_4`: Short code fragment that calls another routine, then copies an address to a fixed memory address, then calls another routine, then returns.
- `frag_bsr_move_rts_5`: Short code fragment that calls another routine, then copies an address to a fixed memory address, then calls another routine, then returns.
- `setup_display_registers_2`: Initializes display register values and repeatedly calls a configuration routine while incrementing memory addresses until reaching a specific limit, then resets the address pointers.
- `initialize_display_data`: Initializes display-related data based on a input value, updating global variables with calculated addresses and values.
- `frag_move_bra_4`: Short code fragment that copies 4095 to a fixed memory address, then jumps to another point in the code.
- `setup_display_registers`: Sets specific values in display registers and then repeatedly calls a DMA configuration routine while incrementing memory addresses.
- `frag_clr_move_divs_swap_addi_rts`: Short code fragment that clears d0, then clears d1, then copies a fixed memory address to d1, then divides d1 by 208 (signed), then swaps the halves of d1, then copies d1 to d0, then divides d0 by 4 (signed), then adds 1 to d0, then returns.
- `frag_clr_move_divs_swap_addi_rts_2`: Short code fragment that clears d0, then clears d1, then copies a fixed memory address to d1, then divides d1 by 156 (signed), then swaps the halves of d1, then copies d1 to d0, then divides d0 by 3 (signed), then adds 1 to d0, then returns.
- `frag_clr_move_divs_swap_addi_rts_3`: Short code fragment that clears d0, then clears d1, then copies a fixed memory address to d1, then divides d1 by 104 (signed), then swaps the halves of d1, then copies d1 to d0, then divides d0 by 2 (signed), then adds 1 to d0, then returns.
- `frag_clr_move_divs_swap_addi_rts_4`: Short code fragment that clears d0, then copies a fixed memory address to d0, then divides d0 by 52 (signed), then swaps the halves of d0, then adds 1 to d0, then returns.
- `frag_clr_move_divs_swap_addi_rts_5`: Short code fragment that clears d0, then copies a fixed memory address to d0, then divides d0 by 26 (signed), then swaps the halves of d0, then adds 1 to d0, then returns.
- `open_and_read_file`: Opens a file using a global variable as the filename, reads data into a buffer, and then closes the file handle.
- `call_old_open_library`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `copy_color_table`: Copies 19 color values from a global table to hardware color registers, advancing both source and destination pointers with each iteration.
- `initialize_graphics_system`: Initializes graphics by opening a library, copying data to a buffer, setting up display registers, and configuring bitplane DMA parameters.
- `call_close_library_2`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.

### Amiga Poker.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sets_global_pointer_returns_minus_one`: Stores a constant into a global variable, removes 4 bytes from the stack, restores registers from memory, and returns the value -1 in d0.
- `frag_move_ori_eori`: Short code fragment that copies d0 to d2, then merges 0 into d0, then exclusive-ors 0 into d0, then merges 0 into d0.
