# Game 18 (Baal): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00018). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905840 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 68 of 68 named (0 author symbols), 68 of 68 described (16 by rule, 21 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 260 of 260 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1282 of 1282 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 670 |
| code | 3822 |
| data | 2276 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Baal.adf:bootblock | 1 | 17 | 1 |
| logged disk load from Baal.adf boot-block load | 1 | 124 | 4 |
| logged disk load from Baal.adf boot-block load | 1 | 18 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1104 | 58 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 3 |

## Cases handled

- Baal/Baal.adf: the boot block reads 1024 bytes from disk offset 2048 to an address (found statically); recovered as a unit
- Baal/Baal.adf: the boot block reads 1024 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 250 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 5 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 5 immediate
- Baal/Baal.adf: 2 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Baal.adf:bootblock

- `init_structure_and_jump`: Initializes a structure pointed to by a1 with several fixed constants, calls two library functions, then reinitializes the same structure with different constants and jumps to a fixed address.

### logged disk load from Baal.adf boot-block load

- `tiny_sub`: Small routine of 1 instruction that subtracts d2 from d1.
- `store_four_word_entry`: Stores two input words from registers d0 and d1 into consecutive 16-bit slots of a buffer pointed to by a2, interleaving two fixed 16-bit constants, then returns the first input word increased by 256.
- `store_interleaved_words`: Stores two fixed words and two register values into consecutive 16-bit slots of a buffer pointed to by a2, then returns a register incremented by 512.
- `call_library_pair_twice`: Calls two library functions at fixed offsets from a base, alternating between them twice while passing the same register values to each call, then adds a constant to a register and returns.

### logged disk load from Baal.adf boot-block load

- `copy_boot_bytes_to_high_memory`: Sets a7, copies bytes from a PC-relative source until a0 reaches the limit in a1, then jumps to the destination start.
- `frag_move_tst_beq_nop_jmp`: Short code fragment that copies 29588 to d0, then copies d0 to a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then copies 20081 to a fixed memory address, then pauses for one no-operation step (4 times in a row), then jumps to the address in a7.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `restore_and_combine_words`: Reads a 16-bit word from the address in a0, restores registers d1 and a0 from the stack, and returns the combined 32-bit value of that word and the leftover stack word.
- `decrement_timer_counters`: Checks a global flag and then, based on the value of a 16-bit counter at a global variable, decrements a byte at offset 5 of a structure and updates a set of ASCII digits at nearby offsets, rolling over each field when it reaches its limit.
- `frag_move_lea_movea_rts`: Short code fragment that copies a0 to the stack, then copies 1920 to a fixed memory address, then loads the address of a location in the code into a0, then copies an address to the memory at a0, then copies 32783 to a fixed memory address, then copies the top of the stack to a0, then returns.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `setup_audio_channel`: Initializes an audio channel by clearing its volume and length registers, setting up its pointer and period values, then stores a flag in a table at an offset calculated from the input register.
- `frag_movem_moveq_lsr_lea_move_rts`: Short code fragment that saves registers on the stack, then loads 0 into d0, then shifts d1 right by 4, then loads the address of a location in the code into a0, then copies an indexed entry from a0 to d0, then restores saved registers from the stack, then returns.
- `frag_movem_moveq_bra`: Short code fragment that saves registers on the stack, then loads 255 into d3, then jumps to another point in the code.
- `configure_audio_dma`: Sets up DMA audio playback by calculating memory addresses, checking if a sound sample exists, storing channel data, configuring DMA control registers, and updating audio hardware registers with sample parameters.
- `initialize_audio_channels`: Clears audio volume registers for four channels and initializes several data structures with calculated values based on input parameters, then returns a zero value.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `frag_lea_clr_move_rts`: Short code fragment that loads the address of a location in the code into a0, then clears the memory at a0, then clears a fixed memory address (4 times in a row), then copies 15 to a fixed memory address, then returns.
- `cycle_audio_channels`: Increments a 16-bit counter at a global variable and resets it to 0 when it reaches 6.
- `process_cia_command`: Reads a 4-bit command from offset 2 of the structure in a6 and a nibble from offset 3, then adjusts a 16-bit value at offset 0 of a4 by adding or subtracting it with clamping, copies a byte to offset 8, or selects a table entry by a counter at offset 16 of a6.
- `toggle_cia_bit_and_store`: Toggles bit 1 of the CIA parallel register at address a fixed address, then reads a 4-bit value from a byte at offset 3 of the structure in a6, stores it as a 16-bit word at the address in a0, and returns.
- `setup_audio_and_dispatch`: Compares a register against several table addresses and returns early when they match, otherwise sets up audio registers, calls a handler four times, and manages counters before returning.
- `process_and_store_entry`: Copies a four-word entry from a table into a structure, updates several fields based on a selector, calls a routine, and stores results into another structure.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (andi.w) and then falls through into the code that follows, with no return of its own.
- `fill_address_table_loop`: Writes a base address and five derived addresses into a table, computes a scaled value, and enters an infinite loop when the running address exceeds 32768.
- `compute_color_values`: Calculates and stores color values in a table by performing arithmetic operations on input data, then compares the result to a threshold and enters an infinite loop if the condition is met.
- `set_color_and_loop`: Writes a fixed 12-bit value to a color register, then enters an infinite loop that never returns.
- `fill_tables_and_init`: Fills two tables with four derived addresses using a decrementing loop, then writes a register and initializes a structure, falling through to another routine.
- `init_copper_structure`: Initializes a structure by clearing several fields, writing fixed words, pointing a copper list register at a table, and returning the copper read value.
- `frag_move_lea_tst_bne_movea_rts`: Short code fragment that copies a0 to the stack, then loads the address of a location in the code into a0, then tests the memory at a0, then branches if the last result was not zero, then copies the top of the stack to a0, then returns.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `frag_bsr_lea_move_rts`: Short code fragment that calls another routine (2 times in a row), then loads the address of a location in the code into a0, then copies 1 to the memory at a0, then returns.
- `frag_bsr_lea_move_bra`: Short code fragment that calls another routine (2 times in a row), then loads the address of a location in the code into a0, then copies 1 to the memory at a0, then jumps to another point in the code.
- `frag_lea_move_rts`: Short code fragment that loads the address of a location in the code into a0, then copies a0 to a fixed memory address, then copies 49168 to a fixed memory address, then returns.
- `update_copper_and_call`: Clears an interrupt mask register, updates a structure based on a flag, writes a copper list register, decrements a counter, and calls a stored routine before returning.
- `tiny_move_rte`: Small routine of 2 instructions that copies (a7)+ to d0, then returns from the exception.
- `write_masked_value_twice`: Masks a 12-bit value and writes it to two locations selected by a shifted index, then restores registers and returns.
- `frag_move_lea_lsl_movea_rts`: Short code fragment that copies a0 to the stack, then loads the address of a location in the code into a0, then shifts d0 left by 1, then copies an indexed entry from a0 to d0, then copies the top of the stack to a0, then returns.
- `copy_color_table`: Copies 32 color values from one memory location to another, writing both the source value and a copy of it to separate locations in the color register area.
- `frag_movem_movea_bra`: Short code fragment that saves registers on the stack, then copies a location in the code to a0, then jumps to another point in the code.
- `frag_movem_movea_bra_2`: Short code fragment that saves registers on the stack, then copies a location in the code to a0, then jumps to another point in the code.
- `frag_movem_movea_bra_3`: Short code fragment that saves registers on the stack, then copies a location in the code to a0, then jumps to another point in the code.
- `frag_move_mulu_subq_clr_dbra_movem_rts`: Short code fragment that copies a location in the code to d0, then multiplies d0 by 2000 (unsigned), then subtracts 1 from d0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `decrease_color_components`: Decrements color component values in a table by one unit for each component that differs from corresponding values in another table, then updates both tables and hardware registers with the modified values.
- `increase_color_components`: Increments color component values in a table by one unit for each component that differs from corresponding values in another table, then updates both tables and hardware registers with the modified values.
- `frag_movea_move_mulu_subq_dbra_rts`: Short code fragment that copies a location in the code to a0, then copies a location in the code to a1, then copies a location in the code to d0, then multiplies d0 by 2000 (unsigned), then subtracts 1 from d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d0, then returns.
- `frag_movea_bra`: Short code fragment that copies a location in the code to a0, then copies a location in the code to a1, then jumps to another point in the code.
- `process_bitmask_data`: Processes bitmask data by reading values from an input stream, performing bitwise operations on them, and storing results to output locations while handling negative values and loop iterations.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `calculate_and_process`: Calculates parameters based on input values, performs bit testing, and conditionally calls another routine while processing data in a loop with shifting operations.
- `frag_bsr_move_clr_rts`: Short code fragment that calls another routine, then copies a0 to the memory at an offset from a1, then copies d0 to the memory at an offset from a1, then clears the memory at an offset from a1 (2 times in a row), then copies 256 to the memory at an offset from a1, then clears the memory at an offset from a1, then copies d1 to the memory at an offset from a1, then returns.
- `frag_move_andi_bset_or_bsr_rts`: Short code fragment that copies d1 to the stack, then copies the memory at a0 to d1, then masks the memory at a0 with 65504, then masks d0 with 31, then sets bit 7 of d0, then merges d0 into the memory at a0, then calls another routine, then copies d1 to the memory at a0, then copies the top of the stack to d1, then returns.
- `validate_and_process_coordinates`: Validates coordinate values against boundary conditions, performs multiplication and addition operations to compute pointer offsets, and calls other routines based on validation results.
- `compute_pointer_offsets`: Calculates pointer offsets using multiplication and addition operations, performs conditional comparisons, and branches to other routines based on the results of these comparisons.
- `calculate_shifted_value`: Performs bit shifting operations on input values to compute a shifted result, then stores this value in a register and branches to another location.
- `validate_and_adjust_coordinates`: Compares coordinate values against threshold limits, performs arithmetic operations on input data, and branches to other routines based on comparison results.
- `process_bit_flags`: Processes bit flags from a word value by shifting and testing each bit, calling another routine when a bit is set, and continues until all bits are processed.
- `store_structure_data`: Saves register contents to a structure at specific offsets, then reads configuration values from memory and applies them to the structure based on bit tests of input data.
- `frag_move_ori`: Short code fragment that copies the memory at a2 (advancing a2) to the memory at a6, then merges 154 into d6 (2 times in a row), then merges 154 into d7.
- `tiny_ori`: Small routine of 1 instruction that merges 1 into d0.
- `combine_input_bits`: Combines bits from a hardware register, a joystick movement register, and a parallel port register into a single value, stores it to a table, and returns.
- `read_cia_fire_bit`: Reads a byte from a global variable, masks bit 7 of it, and returns the masked value in d0.
- `setup_keyboard_interrupts`: Configures CIA registers for keyboard handling by setting interrupt control register values, managing serial data register states, and controlling interrupt enable bits.
- `handle_keyboard_input`: Handles keyboard input by reading CIA registers, setting up a serial communication protocol, performing bit manipulation on input data, and updating control registers with specific timing delays.
- `swap_and_clear_flag`: Reads a 16-bit value from a global variable, clears that variable, and returns it with bit 7 cleared if it was set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_and_dispatch_loop`: Reads a byte from another routine, calls a handler while that byte equals the letter Y, and repeats until the byte equals the at sign.
- `frag_moveq_bra`: Short code fragment that loads 0 into d0, then jumps to another point in the code.
- `frag_lea_move_tst_rts`: Short code fragment that loads the address of a location in the code into a0, then copies d0 to the memory at a0, then tests d0, then returns.
