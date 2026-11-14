# Game 628 (Warhead): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00628). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (923360 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 215 of 215 named (0 author symbols), 215 of 215 described (4 by rule, 189 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1815 of 1815 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2923 of 2923 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1440 |
| code | 12368 |
| data | 16107 |
| padding | 5 |
| unused | 893440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Warhead.adf:bootblock | 1 | 23 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2900 | 214 |

## Cases handled

- 907 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- runtime segment an address: 1815 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 branch displacement
- Warhead/Warhead.adf: blocks 11-21 (bytes 5632-11264) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.77)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `reset_all_audio_channels` (runtime segment an address): Clears volume settings for all four audio channels, disables DMA channels, clears interrupt requests, resets audio period values, and initializes multiple global variables with a fixed routine address. Kind: disk hardware check. Pass path: The routine resets audio channels and ADKCON, then calls u1__sub_45F2(). If this is part of a disk check, the pass path would be the branch that continues execution after verifying disk integrity or hardware state, typically leading to data.
- `process_disk_dma_transfer` (runtime segment an address): Clears a local word variable, calls another routine, tests a bit in a hardware register, and based on the result sets up disk transfer parameters and performs additional operations. Kind: disk track check. Pass path: The routine proceeds to load data if the disk check passes, indicated by the loop continuing and eventually breaking only on failure or timeout (sVar3 == -1). On success, it would fall through to the final return after u1__sub_566E()..
- `configure_disk_dma_transfer` (runtime segment an address): Writes disk and audio control registers, including the sync word and DMA enable bits, sets the pointer register to the value in a0, clears the disk interrupt request, and then writes a length derived from d0 with the high bit set. Kind: disk sync check. Pass path: branch taken when DSKSYNC value matches expected pattern, likely followed by a jump to load data or continue execution.
- `frag_move_tst_rts` (runtime segment an address): Short code fragment that copies 2 to a fixed memory address, then copies 16384 to a fixed memory address, then tests d0, then returns. Kind: disk hardware check. Pass path: rts instruction is executed, followed by moveq #an address, d2 and clr.w -an address(a6).
- `check_and_set_flag` (runtime segment an address): Sets an audio control register, tests a condition based on a value in memory, and branches to different code paths depending on the result. Kind: disk hardware check. Pass path: beq.b an address.
- `set_bit_in_global_flag` (runtime segment an address): Computes a bit index from a stored 16-bit value plus three, then writes the input byte with that bit set to the shared control word. Kind: disk hardware check. Pass path: CIAB_PRB is modified twice with bit manipulation, suggesting a read-modify-write operation on hardware register that likely checks for specific hardware state or disk drive behavior.
- `drive_select_pulse_and_delay` (runtime segment an address): Sets d1 to 255 and writes it to CIAB port B, clears bit 7, calls the pulse routine, then loads 200 into d0 and calls a delay routine. Kind: disk hardware check. Pass path: The routine writes to CIAB_PRB and calls other subroutines, suggesting a hardware-based verification check that must pass for normal operation to continue.
- `update_port_status_byte` (runtime segment an address): Reads a byte from a routine, clears bit one when a second routine output is non-negative, forces bit zero on, stores the result in a global and calls another routine. Kind: disk hardware check. Pass path: The check passes if the value from `u1__sub_563A()` results in `extraout_D1b` being negative (i.e., condition `-1 < extraout_D1b` is false), which would skip clearing bit 2 of `CIAB_PRB`. If the check passes, the routine proceeds to call `u.
- `write_port_byte` (runtime segment an address): Calls another routine to compute a byte value, then writes that byte to a hardware port register. Kind: disk hardware check. Pass path: CIAB_PRB is written with value from u1__sub_563A(), indicating a hardware-based verification check.
- `build_drive_select_byte` (runtime segment an address): Reads the current CIAB port B value, sets bits 0-6, clears the bit indexed by the drive number plus three, optionally clears bit 2 from a table flag, and returns the byte in d0. Kind: disk hardware check. Pass path: The routine modifies CIAB_PRB based on drive selection and possibly a track/sector check; if the check passes (e.g., bit 2 is cleared), it returns a modified byte value in the lower 8 bits of the return register. The successful path involve.

## Functions


### Warhead.adf:bootblock

- `allocate_and_run_io_block`: Queries available memory and allocates a block through the system library, storing an aligned address in the request structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_dma_blit`: Configures multiple Amiga hardware registers for DMA blitting operations using a global variable as source address and sets specific control values for bit block transfer.
- `setup_dma_blit_alt`: Initializes DMA blitting parameters with different register values compared to the first routine, setting up transfer dimensions and control modes for graphics operations.
- `initialize_dma_address`: Loads a memory address from a global location, copies it to another register, adds a constant value to that register, stores the result back to a different global variable, and then performs a conditional branch based on a bit test of a register's value.
- `setup_dma_transfer`: Initializes a DMA transfer by calling a routine, updating an address pointer, testing a bit in a register, and conditionally branching based on the result.
- `initialize_dma_copy`: Calls a DMA readiness routine, updates an address pointer, tests a bit in a register, and conditionally branches to another routine based on the test result.
- `dma_copy_with_condition`: Calls a routine to wait for DMA readiness, updates an address pointer, tests a bit in a register, and conditionally branches to another routine based on the test result.
- `poll_dma_then_copy`: Calls a routine that waits for DMA readiness and performs a graphics copy operation, then returns.
- `wait_dma_ready_then_blit`: Waits for a specific DMA channel to become ready by polling a status register, then configures blitting parameters including source address and control registers for graphics transfer.
- `wait_dma_ready_and_copy`: Polls the DMA status register until a channel is ready, then performs a graphics copy operation by setting up blitting parameters with different control values than other similar routines.
- `process_color_data_2`: Processes color data from a source table by shifting and masking bytes, combines them into words, and stores the results in a color register area, then performs multiple audio channel setup operations.
- `copy_bytes_loop`: Copies bytes from one memory location to another in a loop, decrementing a counter until it reaches zero, then branches to the end.
- `copy_and_loop`: Copies data from one memory location to another while decrementing a counter, continues looping until the counter reaches zero, then returns.
- `copy_memory_blocks`: Copies data from one memory location to four separate destination locations in a pattern that duplicates and spreads the source data across multiple memory regions.
- `decode_compressed_data`: Decodes compressed data streams using a global variable as a counter and multiple conditional branches to handle different operation codes, writing decoded bytes to a destination buffer.
- `initialize_game`: Sets up multiple game data sections by loading word values from tables, calling decompression routines, and configuring audio parameters.
- `initialize_coordinates`: Extends two word values in registers to long words, loads a constant into register d0, sets a word value at a specific memory address, then calls another routine with a word parameter before testing the result and returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_and_copy_blocks`: Sets up memory addresses for copying data blocks and calls two different routines to perform memory copy operations.
- `decode_and_copy_data`: Updates pointers to memory locations, calls a decompression routine, then copies data between memory locations using another routine.
- `handle_input_and_set_flag`: Calls a routine that returns a value, checks if it's zero and branches to an exit if so, otherwise calls another routine to compare a byte value with a constant, then sets a global variable to -1.
- `reset_and_process_input`: Clears specific global variables, calls another routine to set up audio parameters, saves and restores a register's value from the stack, processes input through another routine, and then branches based on a condition test.
- `setup_audio_parameters`: Sets up several audio-related parameters in registers, tests a condition based on a global variable's value, and calls another routine if the condition is met, otherwise it stores a specific value to a global location and returns.
- `copy_audio_data_blocks`: Copies data between memory locations in a structured pattern using multiple loop counters, performs several memory transfers with specific offsets, and calls another routine multiple times based on a condition test.
- `update_global_variables`: Stores a word value in a global variable, decrements another register value, stores that result in a different global variable, then calls another routine.
- `process_and_update_bytes`: Processes bytes in a memory area by extracting and modifying their nibble values, updates the memory locations with new values, and repeats until no changes occur.
- `compare_and_modify_bytes`: Compares byte values between two memory areas, modifies the first area based on comparison results, and repeats until all bytes match.
- `copy_word_pairs`: Decrements a word value in a register by one and performs an arithmetic right shift of one bit, then copies word values from one memory location to another while decrementing the counter until it reaches negative one.
- `copy_four_word_sets`: Loads a constant value into a register, calculates four memory addresses based on that constant and an input address, then copies word values from four different memory locations to a destination location in groups of four words while decrementing the counter.
- `copy_data_blocks`: Copies word-sized data from one memory location to four different destination locations in a specific pattern, repeating this operation for a fixed number of iterations while updating pointers.
- `initialize_audio_channel_0_c`: Initializes audio channel 0 by loading a memory address, writing a constant value to a hardware register, calling an initialization routine, and then writing two more values to hardware registers before returning.
- `setup_audio_channel_0_3`: Initializes audio channel 0 by setting specific register values and writing a constant to a global variable at offset 32, then returns.
- `initialize_audio_channel_0_b`: Initializes audio channel 0 by setting two memory locations to a constant value, loading a memory address, writing a constant value to a hardware register, calling an initialization routine, and then writing two more values to hardware registers before returning.
- `initialize_audio_channel_1_b`: Configures audio channel 1 by loading a memory address, writing a constant value to a hardware register, calling an initialization routine, and then writing two more values to hardware registers before returning.
- `initialize_audio_channel_2_b`: Initializes audio channel 2 by loading a memory address, writing a constant value to a hardware register, calling an initialization routine, checking a condition and potentially calling another routine, then writing two more values to hardware registers before returning.
- `initialize_audio_channel_0_2`: Sets a global variable to 64, loads a memory address into a register, writes a constant value to a data register, calls another routine, then sets additional audio channel registers and a DMA control word.
- `initialize_audio_channel_0`: Initializes audio channel 0 by setting two memory locations to a constant value, loading a memory address, writing a constant value to a hardware register, calling an initialization routine, and then writing two more values to hardware registers before returning.
- `initialize_audio_channel_1`: Configures audio channel 1 by loading a memory address, writing a constant value to a hardware register, calling an initialization routine, and then writing two more values to hardware registers before returning.
- `initialize_audio_channel_2`: Initializes audio channel 2 by loading a memory address, writing a constant value to a hardware register, calling an initialization routine, and then writing two more values to hardware registers before returning.
- `initialize_audio_channel_3`: Sets up audio channel 3 by loading a memory address, writing a constant value to a hardware register, calling an initialization routine, and then writing two more values to hardware registers before returning.
- `setup_audio_channels_0_1_c`: Writes identical smaller constant values to two memory locations, calls routines to initialize two audio channels with different parameters, conditionally calls another routine based on a global variable's value, then sets a specific value in a hardware register and returns.
- `initialize_audio_2`: Writes a constant value to a global variable, then writes another constant value to a different global variable before returning.
- `setup_audio_channels_0_1_a`: Writes identical constant values to two memory locations, calls routines to initialize two audio channels with different parameters, then sets a specific value in a hardware register and returns.
- `configure_audio_registers_1`: Sets a specific value in a hardware register, restores saved registers from the stack, and writes another constant value to a different hardware register before returning.
- `initialize_audio_channels_0_1_b`: Writes identical smaller constant values to four memory locations, calls routines to initialize two audio channels with different parameters, then sets a global variable to a specific value and stores another routine's address.
- `initialize_audio_channels_2_3_b`: Loads a memory address into a register and calls routines with specific parameters twice, writes a constant value to a hardware register, then restores saved registers and returns.
- `configure_audio_registers_2`: Sets a specific value in a hardware register, restores saved registers from the stack, and writes another constant value to a different hardware register before returning.
- `initialize_audio_channels_0_1`: Writes identical constant values to four memory locations, calls routines to initialize two audio channels with different parameters, then sets a global variable to a specific value and stores another routine's address.
- `initialize_audio_channels_2_3`: Sets up two audio channels by loading a memory address into a register and calling routines with specific parameters, then writes a constant value to a hardware register and returns.
- `setup_audio_registers`: Initializes specific values in four global variables and configures audio channels by calling routines with different parameters, then updates a global variable with a pointer to another routine.
- `initialize_audio_registers`: Sets specific values in four global variables and configures audio channels by calling routines with different parameters, then updates a global variable with a function pointer and returns.
- `initialize_audio_system_2`: Configures audio channels by setting specific values in global variables and calling routines to set up audio registers and DMA control, then stores a constant value in a global variable and returns.
- `initialize_audio_channels_9`: Configures two audio channels by setting specific values in global variables and calling routines to set up audio registers and DMA control, then restores saved registers and returns.
- `setup_audio_0`: Configures audio channel 0 by setting a specific word value at a memory address, restoring registers from the stack, and writing another word value to a memory location before returning.
- `initialize_audio_channels_8`: Sets initial volume values for four audio channels in a global table, initializes two audio channels by loading data from a fixed memory location, and configures channel parameters with specific word values before returning.
- `setup_audio_channels_2_3`: Initializes two audio channels by loading data from a fixed memory location and calling routines that configure channel parameters with specific word values, then restores registers and returns.
- `setup_audio_channels_0_1`: Initializes two audio channels by loading data from a fixed memory location and calling routines that configure channel parameters with specific word values, then restores registers and returns.
- `setup_audio_5`: Configures audio channel 5 by setting a specific word value at a memory address, restoring registers from the stack, and writing another word value to a memory location before returning.
- `setup_audio_channels_5`: Configures audio channel registers with specific values and calls routines to initialize two audio channels using data from a fixed memory location.
- `initialize_audio_system`: Sets up audio channel registers with specific values and calls routines to initialize two audio channels using data from a fixed memory location.
- `initialize_audio_channels_1_0`: Sets specific word values to memory locations for two audio channels, initializes audio channels with data from memory locations, configures.
- `initialize_audio_channels_3_2_2`: Sets specific word values to memory locations for two audio channels, initializes audio channels with data from memory locations, configures.
- `initialize_audio_channels_3_2`: Sets specific word values to memory locations for two audio channels, initializes audio channels with data from memory locations, configures audio registers and DMA control for channel 0 and channel 1 with different parameter values than routines R1 and R4.
- `initialize_audio_channels_2_2`: Sets specific word values to memory locations for two audio channels, initializes audio channels with data from memory locations, configures audio registers and DMA control for channel 0 and channel 1 with different parameter values than routine R1.
- `initialize_audio_channels_7`: Sets specific word values to memory locations for two audio channels, initializes audio channels with data from memory locations, configures audio registers and DMA control for channel 0 and channel 1.
- `setup_audio_2_channels`: Sets specific word values to memory locations for two audio channels, initializes audio channels with data from memory locations, configures DMA control registers, and sets up global variables for audio output.
- `configure_audio_channels`: Sets specific word values to multiple memory locations, loads function addresses into global variables, initializes audio channels with different parameters, and configures DMA control registers.
- `initialize_audio_3_channels`: Initializes three audio channels by setting up specific register values and copying data from memory locations to audio registers, then configures DMA control for each channel.
- `setup_audio_channels_3`: Initializes three audio channels by loading data from specific memory locations, calling initialization routines with different parameters, writing a constant value to a hardware register, and setting up an interrupt handler.
- `setup_audio_channels_1`: Initializes two audio channels by loading data from specific memory locations, calling initialization routines with different parameters, writing a constant value to a hardware register, and setting up an interrupt handler.
- `initialize_audio_channels_5`: Sets up multiple audio channels by loading data from specific memory locations, calling initialization routines with different parameters, writing a constant value to a hardware register, and setting up an interrupt handler.
- `initialize_audio_channels_3`: Initializes audio channel parameters by loading data from a fixed memory location, calling initialization routines with specific parameters, writing values to audio control registers, and then restoring register contents from the stack.
- `initialize_audio_channels_1`: Sets up audio channel parameters by loading data from fixed memory locations, calling initialization routines with specific parameters, writing values to audio control registers, and then restoring register contents from the stack.
- `initialize_audio_channels`: Sets up audio channel parameters by loading data from memory, calling initialization routines, writing specific values to audio registers, and restoring registers before returning.
- `setup_audio_channels`: Initializes multiple audio channels by setting up specific register values, copying data to audio registers, configuring DMA control, and updating global interrupt vectors.
- `initialize_audio_sequence`: Sets up multiple audio channels with specific parameters and values, then calls routines to configure audio channels 0, 1, and 2 in sequence.
- `set_dma_control_word`: Stores a specific long value at a fixed memory location and returns immediately.
- `setup_three_audio_channels_2`: Initializes three audio channels by copying data from specific memory locations to audio registers, setting up DMA control, and configuring.
- `setup_three_audio_channels`: Initializes three audio channels by copying data from specific memory locations to audio registers, setting up DMA control, and configuring global audio settings with a specific address value, using different memory addresses than routine R2.
- `setup_four_audio_channels_2`: Initializes four audio channels by copying data from specific memory locations to audio registers, setting up DMA control, and configuring global audio settings with a specific address value, using different memory addresses than routine R2.
- `setup_four_audio_channels`: Initializes four audio channels by copying data from specific memory locations to audio registers, setting up DMA control, and configuring global audio settings with a specific address value.
- `initialize_audio_4_channels`: Initializes four audio channels by copying data from specific memory locations to audio registers, setting up DMA control, and configuring global variables for audio output with different parameters for each channel.
- `initialize_audio_3_channels_2`: Initializes three audio channels by copying data from specific memory locations to audio registers, setting up DMA control, and configuring global variables for audio output.
- `setup_audio_6_channels`: Configures six audio channels by setting initial register values, copying data from memory locations to audio registers, and updating global configuration variables for multiple channels.
- `initialize_audio_channels_6`: Sets initial values for two audio channel configuration registers, configures audio channel 2 and 3 with specific parameters, and updates global audio control registers.
- `setup_audio_channels_2`: Initializes two global variables with the same constant value, loads data from a memory location, calls initialization routines with different parameters, writes a constant value to a hardware register, and sets up an interrupt handler.
- `setup_audio_channels_4`: Initializes two global variables with the same constant value, loads data from a memory location, calls initialization routines with different parameters, writes a constant value to a hardware register, and sets up an interrupt handler.
- `initialize_audio_channels_4`: Initializes audio channel parameters by setting specific word values in global variables, loading data from a fixed memory location, calling initialization routines with parameters, writing values to audio control registers, and then restoring register contents from the stack.
- `initialize_audio_channels_2`: Initializes audio channel parameters by setting specific word values in global variables, loading data from a fixed memory location, calling initialization routines with parameters, writing values to audio control registers, and then restoring register contents from the stack.
- `configure_audio_globals_2`: Updates global audio variables with specific word values, calls channel initialization routines, writes register values, and restores registers before returning.
- `configure_audio_globals`: Sets specific word values into global variables, configures audio channel 2 and 3 through dedicated routines, and updates hardware interrupt flags for audio processing.
- `configure_audio_and_globals`: Stores specific word values into global variables, calls routines to set up audio channels 2 and 3, configures a global variable with another routine address, and returns.
- `set_global_flag`: Stores a word value of -1 into a specific memory address and jumps to another memory address without returning.
- `initialize_audio`: Sends specific values to a hardware register at offset 4130 and then writes another value to a memory location at offset 4130.
- `setup_hardware_registers`: Writes fixed values to multiple hardware registers including display control, color registers, coprocessor addresses, and interrupt enable bits to configure the Amiga's graphics and interrupt system.
- `set_hardware_register`: Writes a fixed value to a hardware register at a specific memory address.
- `update_joystick_input`: Reads joystick and mouse input data from hardware registers, processes directional and fire button states into global variables, and manages game loop timing based on input polling results.
- `initialize_flag`: Sets a global word variable to -1, then enters a loop that reads the same variable and branches back if it's not zero.
- `process_color_data`: Processes 16 words of data by masking and shifting bits, then stores the results in a table at a fixed address.
- `copy_color_registers`: Copies 16 color values from a source table to both a hardware color register area and another memory location.
- `shift_color_components`: Processes 32 color component values from a table, shifting and combining high and low nibbles before storing the results back to the same table.
- `update_color_components`: Processes color data from two sources, modifies component values based on comparisons, and repeats until no changes occur.
- `set_global_flag_2_2`: Writes the value 2 to a global word variable at a fixed memory address, then writes the value 1 to the same variable.
- `set_global_flag_3`: Writes the value 3 to a global word variable at a fixed memory address, then writes the value 1 to the same variable.
- `set_global_flag_2`: Writes the value 1 to a global word variable at a fixed memory address.
- `configure_cia_interrupts`: Writes a byte value to two different CIA interrupt control registers, then sets a memory location to a specific value.
- `handle_keyboard_input`: Configures a hardware interrupt request register, reads from a keyboard serial register, performs bit operations on the read byte, and updates memory locations based on the results of conditional checks.
- `update_write_pointer`: Writes a byte from a register to a memory location, advances a pointer, and resets it to a base address when reaching an end marker.
- `calculate_offset`: Subtracts two global word variables, adds 10 if the result is negative, and returns the final value.
- `read_and_update_pointer`: Reads a byte from a memory location, advances a pointer, and resets it to a base address when reaching an end marker, then returns the read byte.
- `wait_for_flag_clear`: Sets a global word variable to -1, reads it back, and enters an infinite loop if it's not zero.
- `store_word_pair`: Stores two word values from registers to fixed memory locations.
- `load_operand_data`: Loads word values from tables at specific offsets based on a global variable index, stores them in memory locations, and processes a byte stream until reaching a zero terminator.
- `update_memory_bits`: Performs bitwise operations on word values and applies them to memory locations at various offsets, advancing pointers through a loop of five iterations.
- `apply_bitwise_and`: Performs bitwise AND operations between a register value and memory locations at specific offsets, then advances a pointer through a loop of four iterations.
- `apply_bitwise_and_or`: Performs bitwise AND operations between a register value and memory locations at specific offsets, followed by a bitwise OR operation with another register value on the final memory location.
- `apply_bitwise_and_or_2`: Performs bitwise AND operations between a register value and memory locations at specific offsets, followed by a bitwise OR operation with another register value on one of the memory locations.
- `apply_bitwise_and_or_3`: Performs bitwise AND operations between a register value and memory locations at specific offsets, followed by bitwise OR operations with another register value on two of the memory locations.
- `modify_memory_bits`: Performs bitwise AND operations on a data register with memory locations at specific offsets, then applies OR operations to another memory location while advancing a pointer through a loop.
- `update_bitwise_operations`: Executes bitwise AND and OR operations on data registers with memory values at various offsets, updating memory locations and advancing a pointer through a loop structure.
- `combine_memory_modifications`: Applies bitwise AND and OR operations to data registers with memory values at specific offsets, then performs an OR operation on another memory location while advancing a pointer through a loop.
- `execute_bitwise_sequence`: Carries out bitwise AND and OR operations on data registers with memory locations at various offsets, followed by an OR operation on a memory location while advancing a pointer through a loop.
- `alter_memory_with_and_operations`: Applies OR operations to memory locations at specific offsets using data register values, then performs AND operations with memory locations and advances a pointer through a loop structure.
- `process_memory_bits`: Executes bitwise OR and AND operations on data registers with memory values at various offsets, updating memory locations while advancing a pointer through a loop.
- `adjust_memory_values`: Performs OR operations on memory locations using data register values, then applies AND operations to another memory location and advances a pointer through a loop structure.
- `execute_dual_operations`: Carries out bitwise OR operations on memory locations with data register values at specific offsets, followed by an OR operation on another memory location while advancing a pointer through a loop.
- `modify_memory_with_or_operations`: Applies OR operations to memory locations using data register values at various offsets, then performs AND operations with memory locations and advances a pointer through a loop structure.
- `update_memory_regions`: Performs bitwise OR and AND operations on word values with memory locations at specific offsets from a register, advances the register pointer, and loops back to an earlier instruction within this routine.
- `perform_dual_bitwise_sequence`: Executes multiple bitwise AND operations on data registers with memory values at specific offsets, then applies additional AND operations using another data register before advancing a pointer through a loop.
- `update_bits_and_store`: Performs bitwise AND operations on data in registers with memory locations at specific offsets, then stores results back to memory while advancing a pointer through a loop of five iterations.
- `update_bits_with_or`: Executes bitwise AND and OR operations on register values with memory locations at various offsets, updating memory contents and advancing a pointer through a loop of five iterations.
- `update_bits_and_or_store`: Carries out bitwise AND and OR operations between registers and memory addresses at specific offsets, storing results while advancing a pointer through a loop of five iterations.
- `update_bits_with_mixed_operations`: Applies bitwise AND and OR operations on register values with memory locations at different offsets, updating memory contents and advancing a pointer through a loop of five iterations.
- `update_bits_and_or_mixed`: Executes bitwise AND and OR operations between registers and memory addresses at specific offsets, storing results while advancing a pointer through a loop of five iterations.
- `update_bits_with_three_operations`: Performs bitwise AND and OR operations on register values with memory locations at various offsets, updating memory contents and advancing a pointer through a loop of five iterations.
- `update_memory_bits_2`: Performs bitwise OR operations on data in registers with values at specific offsets in memory, and advances a pointer through a loop structure.
- `update_bits_with_first_or`: Executes bitwise OR operations on register values with memory locations at specific offsets, then performs AND operations with other memory addresses, updating memory contents and advancing a pointer through a loop of five iterations.
- `update_bits_with_second_or`: Carries out bitwise OR and AND operations between registers and memory addresses at specific offsets, storing results while advancing a pointer through a loop of five iterations.
- `update_bits_with_alternating_operations`: Performs alternating bitwise OR and AND operations on register values with memory locations at various offsets, updating memory contents and advancing a pointer through a loop of five iterations.
- `update_memory_bits_3`: Performs bitwise OR and AND operations on word values from registers and applies the results to specific memory locations at various offsets, then advances a pointer through a loop of five iterations.
- `update_memory_bits_4`: Performs bitwise OR operations between word values and memory locations at specific offsets, and applies bitwise AND operations to other memory locations.
- `modify_memory_values`: Executes a sequence of bitwise OR and AND operations on word values with memory locations at various offsets from a register, advances the register pointer, and loops back to an earlier instruction within this routine.
- `swap_and_process`: Swaps the bytes of a register, calls another routine twice, then returns without modifying any data.
- `process_input_word`: Saves two registers to the stack, processes each byte of a word value through another routine using different shift operations, restores the registers from the stack, and returns with the processed values.
- `process_input_nibble`: Processes a byte value by shifting it right, masking its lower nibble, comparing it against a threshold, branching if it exceeds the threshold, adding a constant to the shifted value, and then branching to an exit point.
- `process_byte_input`: Processes byte input by adding a constant to a register, calling another routine, comparing the result with a threshold, and branching based on the comparison outcome.
- `process_byte_value`: Adds a constant value to a register, copies the result to another register, calls another routine with the updated value, then restores registers and returns.
- `loop_and_adjust`: Saves two registers to the stack, loads a constant value into one register, subtracts a constant from another register, branches to a label if the result is negative, adds a constant to the second register, and then branches back to an earlier instruction.
- `adjust_and_process`: Adds a constant value to a register, combines the result with another register, stores the combined value back to the first register, then calls another routine and returns.
- `initialize_interrupt_handler`: Writes a specific 32-bit value to a global variable and returns immediately.
- `clear_interrupt_flag`: Writes zero to a global word variable and sets a hardware interrupt request register to 64 before returning.
- `wait_for_flag_clear_2`: Sets a global variable to 255 and waits in a loop until that variable becomes zero, then returns the value that was originally in the data register.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `jump_through_global_table`: Saves all general-purpose and address registers on the stack, loads an address from a global variable, and jumps to that address as a function pointer.
- `set_audio_interrupt_flag`: Writes a specific value to a hardware register at offset 112.
- `handle_audio_interrupt`: Processes audio interrupt requests for four sound channels, updating register values based on channel state and triggering sound playback.
- `setup_audio_and_interrupts`: Configures audio registers for sound playback, calls another routine, sets up interrupt enable flags, and returns from the interrupt handler.
- `countdown_loop`: Initializes a global variable with 2000, then decrements it in a loop until it reaches zero.
- `setup_audio_channel_0_2`: Compares a word value with zero and branches if equal, otherwise writes a value to a memory location and copies data from a memory address to audio registers.
- `setup_audio_channel_0`: Initializes audio channel 0 with specific register values and copies data from a memory location to audio registers, then sets up DMA control and interrupt enable bits.
- `setup_audio_channel_1_2`: Compares a word value with zero and branches if not equal, otherwise writes a value to a memory location and copies data from a memory address to audio registers.
- `setup_audio_channel_1`: Initializes audio channel 1 with specific register values and copies data from a memory location to audio registers, then sets up DMA control and interrupt enable bits.
- `setup_audio_channel_2_2`: Compares a word value with zero and branches if not equal, otherwise writes a value to a memory location and copies data from a memory address to audio registers.
- `setup_audio_channel_2`: Initializes audio channel 2 with specific register values and copies data from a memory location to audio registers, then sets up DMA control and interrupt enable bits.
- `initialize_audio_channel`: Sets up audio channel 3 by loading register values from a data table and storing them into audio registers, then stores a word value from the data table into a global variable.
- `setup_audio_channel_3`: Initializes audio channel 3 with specific register values and copies data from a memory location to audio registers, then sets up DMA control and interrupt enable bits.
- `reset_all_audio_channels`: Clears volume settings for all four audio channels, disables DMA channels, clears interrupt requests, resets audio period values, and initializes multiple global variables with a fixed routine address.
- `set_byte_constant`: Loads an immediate byte value into a register and then branches unconditionally to a specific address, effectively skipping the next instruction in the sequence.
- `negate_and_process_3`: Negates a signed byte value at an address calculated from two registers, then calls another routine multiple times while updating a global variable with incrementing values.
- `process_data_with_swap`: Saves all general-purpose registers to the stack, loads a pointer from a fixed location, calls a routine, restores registers, and performs arithmetic operations on data before branching.
- `negate_and_process_2`: Negates a signed byte value pointed to by a3 with index in d3, then processes the result through a series of routine calls while updating a global variable.
- `process_and_update`: Saves all general-purpose registers to the stack, loads a specific address into a register, calls another routine, restores registers from the stack, copies a value from a3 to d0, processes the value, updates a memory location with a constant, and then branches to another location.
- `negate_and_process`: Negates a signed byte value at an address calculated from a4 and d3, then calls another routine twice, updates a global variable with 20, and performs similar operations multiple times with different register values.
- `execute_and_update_registers`: Saves all general-purpose and address registers to the stack, calls another routine with a specific address, restores the registers, updates a global variable with an increment, sets another global variable to a specific value, and then branches to another location.
- `process_and_swap_bytes`: Negates a byte value at an offset in memory, saves all general-purpose and address registers to the stack, calls another routine with a specific address, restores the registers, updates a global variable with an increment, and then branches to another location.
- `save_and_execute_2`: Saves all general-purpose and address registers to the stack, calls another routine with a specific address, restores the registers, updates memory values, and branches to another location in the code flow.
- `negate_and_execute`: Negates a byte value at a calculated memory address, saves all general-purpose and address registers to the stack, calls another routine with a specific address, restores the registers, and updates memory values before branching.
- `save_and_process`: Saves all general-purpose and address registers to the stack, calls another routine with a specific address, restores the registers, and updates memory values before branching to another location.
- `negate_and_save`: Negates a byte value at a specific memory location and saves all general-purpose and address registers to the stack before calling another routine.
- `decrement_and_wait`: Decrements a word value at a memory address, saves all registers to the stack, calls another routine with a memory address, restores all registers from the stack, adjusts the stack pointer, processes a word value through another routine, and then calls another routine.
- `process_tile_index`: Splits an input word by eleven into a quotient and remainder that serve as row and column, consumes a bounded step count through repeated routine calls, and packs the result into a return value.
- `manage_dma_operations`: Manages DMA operations by initializing counters, clearing structure entries, waiting for blitter completion, and performing multiple conditional operations based on game state.
- `process_disk_dma_transfer`: Clears a local word variable, calls another routine, tests a bit in a hardware register, and based on the result sets up disk transfer parameters and performs additional operations.
- `manage_dma_retry_loop`: Initializes a counter and loops while calling routines, checking conditions and updating variables before returning a specific value or continuing the loop.
- `manage_object_state`: Processes object state by performing multiple operations including bit testing, comparisons, memory access, and conditional branching based on various game conditions.
- `frag_move_sub_mulu_rts`: Short code fragment that copies d3 to d1, then subtracts the memory at an offset from a6 from d1, then copies 512 to d0, then multiplies d1 by d0 (unsigned), then returns.
- `frag_move_bset_addq_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then sets bit d3 of d0, then copies d0 to the memory at an offset from a6, then adds 1 to the memory at an offset from a6, then returns.
- `clear_structure_entries`: Clears 10 entries in a data structure by writing zero values to specific offsets within the structure.
- `extract_high_byte_pair`: Calls a helper for its 32-bit result and returns a value with the low word in the low half and the high byte of that result in the high half.
- `interleave_word_bits`: Reads two consecutive words through a pointer and merges their alternating bits, taking the odd-position bits from the second word and the even-position bits from the first word shifted up by one.
- `chain_call_result`: Calls an unnamed routine and passes its 32-bit result as the argument to a second unnamed routine.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `xor_reduce_and_mask`: Performs an exclusive OR operation on a series of longwords, then applies a bitwise AND with a constant mask.
- `fill_1024_bytes_with_aa_pattern`: Loads a0 from the longword at offset minus 24 of a6, sets a1 to a0 plus 1024, fills that region backwards with the value aa repeated via movem, and returns when a1 reaches a0.
- `initialize_object_state`: Calls a helper and derives three integers from its result and two output values, storing them at fixed offsets in the structure.
- `set_object_transforms`: Stores helper results into several fields of a structure, sets constant words and derives a short field from a call result.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `reset_blitter_masks_and_modulo`: Waits for the blitter, then sets its control and mask registers to fixed values and clears the modulo registers.
- `shift_and_call_twice`: Stores a longword on the stack, shifts it right by one bit, calls another routine twice with different parameters.
- `update_sign_bit_from_neighbour`: Adjusts bit seven of a byte using bit zero of the byte before it: when that bit is clear, it sets bit seven unless bit six is already set, and when it is set, it clears bit seven.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `configure_disk_dma_transfer`: Writes disk and audio control registers, including the sync word and DMA enable bits, sets the pointer register to the value in a0, clears the disk interrupt request, and then writes a length derived from d0 with the high bit set.
- `process_dma_and_flags`: Sets up a memory address, performs a subroutine call, tests a bit in memory, and branches based on the result while executing additional subroutine calls.
- `frag_move_tst_rts`: Short code fragment that copies 2 to a fixed memory address, then copies 16384 to a fixed memory address, then tests d0, then returns.
- `check_and_set_flag`: Sets an audio control register, tests a condition based on a value in memory, and branches to different code paths depending on the result.
- `set_bit_in_global_flag`: Computes a bit index from a stored 16-bit value plus three, then writes the input byte with that bit set to the shared control word.
- `drive_select_pulse_and_delay`: Sets d1 to 255 and writes it to CIAB port B, clears bit 7, calls the pulse routine, then loads 200 into d0 and calls a delay routine.
- `update_table_entry`: Reads a 16-bit entry from a table indexed by a structure field and calls a helper when the entry is negative.
- `joystick_input_loop`: Tests a bit in a hardware register, and if it's clear, clears a word at an offset from a pointer and returns zero.
- `update_port_status_byte`: Reads a byte from a routine, clears bit one when a second routine output is non-negative, forces bit zero on, stores the result in a global and calls another routine.
- `write_port_byte`: Calls another routine to compute a byte value, then writes that byte to a hardware port register.
- `build_drive_select_byte`: Reads the current CIAB port B value, sets bits 0-6, clears the bit indexed by the drive number plus three, optionally clears bit 2 from a table flag, and returns the byte in d0.
- `wait_for_routine_success`: Calls a routine repeatedly until it returns one, spinning between calls while the low bit of a global is set.
- `check_and_decrement`: Tests a bit in a memory location and branches to an exit if set, otherwise decrements a register value and branches to the same exit point.
- `frag_move_rts`: Short code fragment that copies 8 to a fixed memory address, then copies 204 to a fixed memory address, then copies 2 to a fixed memory address, then returns.
