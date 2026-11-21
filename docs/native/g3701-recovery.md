# Game 3701 (forgotten worlds (1988)(u. s. gold)[cr telstar]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03701). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1806112 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 44 of 44 named (0 author symbols), 44 of 44 described (9 by rule, 32 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1012 of 1012 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1199 of 1199 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 23821 |
| code | 4146 |
| data | 1088481 |
| unused | 689664 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Forgotten Worlds (1988)(U. S. Gold)[cr Telstar](Disk 1 of 2).adf:bootblock | 1 | 12 | 1 |
| logged disk load from Forgotten Worlds (1988)(U. S. Gold)[cr Telstar](Disk 1 of 2).adf boot-block load | 1 | 220 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 220 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 451 | 15 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 281 | 14 |

## Cases handled

- forgotten worlds (1988)(u. s. gold)[cr telstar](disk 1 of 2)/Forgotten Worlds (1988)(U. S. Gold)[cr Telstar](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1261 non-empty blocks: matched against track-loaded segments
- forgotten worlds (1988)(u. s. gold)[cr telstar](disk 2 of 2)/Forgotten Worlds (1988)(U. S. Gold)[cr Telstar](Disk 2 of 2).adf: custom-format (non-DOS) disk, 908 non-empty blocks: matched against track-loaded segments
- forgotten worlds (1988)(u. s. gold)[cr telstar](disk 1 of 2)/Forgotten Worlds (1988)(U. S. Gold)[cr Telstar](Disk 1 of 2).adf: the boot block reads 24576 bytes from disk offset 626688 to an address (found statically); recovered as a unit
- runtime segment an address: 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 108 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 420 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 246 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- forgotten worlds (1988)(u. s. gold)[cr telstar](disk 1 of 2)/Forgotten Worlds (1988)(U. S. Gold)[cr Telstar](Disk 1 of 2).adf: 1 logged loads cover 24576 bytes of the image
- 2 packed track region(s), 1085952 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `joystick_input_processing` (runtime segment an address): Processes joystick input by polling hardware registers, performing calculations based on input differences, and updating control bits in a hardware register. Kind: disk hardware sync check. Pass path: The routine initializes ADKCON and DSKSYNC, then performs a loop that checks for specific bit patterns in the joystick input. If the check passes (i.e., the expected timing or bit sequence is detected), the routine continues normally and re.
- `initialize_disk_operations` (runtime segment an address): Sets up disk controller registers with specific values including a command word, address pointer, and length parameters, then waits for interrupt status bit 1 to clear before proceeding. Kind: disk sync/checksum verification. Pass path: The routine returns when sVar4 reaches -1, indicating successful validation of track data..
- `hardware_register_control` (runtime segment an address): Controls hardware registers by performing bit tests and sets or clears specific bits in a register, then executes delay loops based on comparisons. Kind: disk hardware register control. Pass path: The routine modifies CIAB_PRB to control disk hardware signals, likely for a track or sector validation check. When the check passes, the branch that sets or clears specific bits in CIAB_PRB is taken, and the delay loop (with busy-waiting) .

## Functions


### Forgotten Worlds (1988)(U. S. Gold)[cr Telstar](Disk 1 of 2).adf:bootblock

- `initialize_boot`: Sets up initial system state by configuring memory pointers and interrupt settings, then performs a series of data manipulations and copies before jumping to a fixed address.

### logged disk load from Forgotten Worlds (1988)(U. S. Gold)[cr Telstar](Disk 1 of 2).adf boot-block load

- `setup_and_process_bits`: Sets up interrupt enable bits in a hardware register, then performs a loop that modifies data in a table using exclusive OR operations with a counter value, and finally disables interrupts.
- `copy_and_process_bits`: Copies data from a source table to a destination location while processing bits through rotate operations, updating memory with shifted values.
- `memory_shift_and_update`: Shifts bits in a value using rotate operations, updates memory locations with processed data, and performs conditional operations based on comparison results and loop counters.
- `recursive_bit_processing_loop`: Processes bits recursively by shifting values right and calling another routine when reaching zero, decrementing a counter until completion while maintaining bit state information.
- `bit_shift_and_recursive_processing`: Processes data by shifting bits right and performing recursive operations when the result becomes zero, updating counters and handling memory locations during the process.
- `bit_shift_and_recursive_call`: Shifts bits in a value right by one position, recursively calls another routine when the result is zero, and decrements a counter until it reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware`: Sets up interrupt enable, memory mapping, and copies data from a table to memory locations using a loop that processes entries until it encounters a termination marker.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_lea`: Short code fragment that copies 32767 to d0, then copies d0 to the memory at an offset from a0 (3 times in a row), then loads the address of a location in the code into a1, then copies a1 to a fixed memory address.
- `hardware_initialization_sequence`: Initializes hardware registers including interrupt enable, joystick test, and various display control registers, then performs a loop that waits for specific vertical position conditions before continuing.
- `interrupt_handling`: Checks interrupt status bits, clears pending interrupts, and calls another routine to update color values if specific interrupt conditions are met.
- `update_color_values`: Processes color data by incrementing or decrementing values based on bit comparisons between current and previous color components, updating both a display buffer and a color table.
- `joystick_input_processing`: Processes joystick input by polling hardware registers, performing calculations based on input differences, and updating control bits in a hardware register.
- `delay_loop`: Executes a simple delay loop that decrements a counter from 8192 until it reaches zero.
- `wait_for_joystick_input`: Waits for joystick input by polling a hardware register, then calls another routine and processes button states to update game data.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `system_setup_and_interrupt_handling`: Sets up system libraries and graphics hardware by opening and closing libraries, configuring DMA and interrupt registers, then enters a loop waiting for joystick input before proceeding.
- `perform_movement_sequence`: Executes multiple movement operations by calling another routine with different parameters, performing addition and subtraction operations on coordinates based on loop counters.
- `frag_move_add_adda_rts`: Short code fragment that copies d7 to the memory at an offset from a0 (2 times in a row), then adds d5 to the memory at an offset from a0, then copies d7 to the memory at an offset from a0, then adds d5 to the memory at an offset from a0 (2 times in a row), then adds 240 to a0, then returns.
- `frag_lea_move_adda_dbra_rts`: Short code fragment that loads the address of a location in the code into a0, then copies 23 to d1, then loads the address of a location in the code into a1, then copies 7 to d0, then copies the memory at a1 (advancing a1) to the memory at an offset from a0, then adds 120 to a0, then loops back while counting down d0, then loops back while counting down d1, then returns.
- `text_data_parser`: Parses text data by reading characters from memory, performing lookups in tables, and advancing pointers through the data structure until reaching a specific marker.
- `blitter_setup_loop`: Configures blitter registers for graphics operations by setting up various parameters, then enters a loop that continues until a counter reaches zero.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address, then copies 63984 to a fixed memory address, then clears a fixed memory address (2 times in a row), then copies an address to a fixed memory address, then copies an address to a fixed memory address, then copies 65535 to a fixed memory address (2 times in a row), then copies 474 to a fixed memory address, then returns.
- `setup_blitter_parameters`: Configures blitter registers for graphics operations by calculating memory addresses from division results and setting up various blitter control and address values.
- `update_joystick_positions`: Reads joystick data, updates position values based on input states, and writes updated coordinates to a buffer for further processing.
- `configure_blitter_for_rendering`: Sets up blitter registers for rendering operations with specific control values, then iterates through a loop to configure memory addresses and execute blitter commands.
- `process_lookup_table_data`: Processes data from lookup tables by performing multiplication and arithmetic operations on values, storing results in a destination buffer while updating an index counter.
- `blitter_coordinate_calculation`: Sets up blitter registers with fixed values, then performs coordinate calculations and bit tests on memory data to configure blitter operations for graphics rendering.
- `calculate_blitter_coordinates`: Calculates blitter coordinate parameters based on input differences, determines direction flags, and sets up various blitter registers for graphics rendering operations.
- `frag_move_lea_swap_addq_dbra_rts`: Short code fragment that copies an address to d0, then loads the address of a location in the code into a2, then copies 7 to d6, then copies d0 to the memory at an offset from a2, then swaps the halves of d0, then copies d0 to the memory at an offset from a2, then swaps the halves of d0, then adds 8 to a2, then loops back while counting down d6, then returns.
- `initialize_graphics_data`: Initializes a graphics data structure by writing multiple 16-bit values to memory locations, then performs a loop that updates these values based on comparisons and increments.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `data_buffer_management`: Manages data buffers through memory operations including pointer adjustments, bit manipulation, and conditional processing based on buffer states and data validation checks.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `initialize_disk_operations`: Sets up disk controller registers with specific values including a command word, address pointer, and length parameters, then waits for interrupt status bit 1 to clear before proceeding.
- `validate_and_merge_data_blocks`: Validates data blocks by comparing checksums and checking specific conditions, then merges alternating bits from two longwords using bitwise operations and stores the results.
- `compare_memory_blocks`: Compares two memory blocks by calculating checksums and returning zero if they match, otherwise returning 255.
- `bit_manipulation_sequence`: Performs a sequence of bit shifting and manipulation operations on data values, storing intermediate results through memory pointers while checking specific bit conditions.
- `frag_bclr_move_rts`: Short code fragment that clears bit 7 of d0, then copies d0 to the memory at a0, then returns.
- `bit_manipulation_and_store`: Performs bitwise operations on a value, checks a bit in memory, clears or sets specific bits based on the result, and stores the final value to memory.
- `frag_clr_move_eor_dbra_andi_rts`: Short code fragment that clears d0, then copies the memory at a0 (advancing a0) to d2, then exclusive-ors d2 into d0, then loops back while counting down d1, then masks d0 with an address, then returns.
- `merge_alternating_bit_longwords`: Merges two longwords by keeping the alternating-bit positions of each, shifting the first one left by one bit and OR-ing it with the second.
- `frag_addq_bsr_subq_rts`: Short code fragment that adds 2 to a fixed memory address, then calls another routine, then subtracts 2 from a fixed memory address, then calls another routine, then returns.
- `hardware_register_control`: Controls hardware registers by performing bit tests and sets or clears specific bits in a register, then executes delay loops based on comparisons.
