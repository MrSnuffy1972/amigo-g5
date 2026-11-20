# Game 3133 (Chuckie Egg II): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03133). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908176 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 67 of 67 named (0 author symbols), 67 of 67 described (33 by rule, 32 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 195 of 195 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1149 of 1149 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2522 |
| code | 4274 |
| data | 208830 |
| padding | 1350 |
| unused | 691200 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Chuckie Egg II.adf:Chuckie2 | 2 | 187 | 16 |
| Chuckie Egg II.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 62 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 400 | 32 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 454 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 36 | 1 |

## Cases handled

- runtime segment an address (play-rs-m1500): 50 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 60 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 30 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 36 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Chuckie Egg II/Chuckie Egg II.adf:Chuckie2: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Chuckie Egg II.adf:Chuckie2

- `initialize_system`: Initializes system resources by opening and closing the DOS library, setting up interrupt vectors, and configuring video display registers.
- `copy_and_loop`: Copies bytes from one memory location to another in a loop until a counter reaches zero, then jumps to a fixed address and performs conditional checks on a memory location before branching.
- `frag_cmpi_beq`: Short code fragment that compares a fixed memory address with 89, then branches if the last result was zero, then compares a fixed memory address with 121, then branches if the last result was zero.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `frag_move_jmp`: Short code fragment that copies an address to a fixed memory address (2 times in a row), then jumps to another point in the code.
- `frag_move_jmp_2`: Short code fragment that copies 24576 to a fixed memory address, then jumps to another point in the code.
- `update_video_dma`: Reads a video position register, masks its upper four bits, writes the result to a custom hardware register, then performs bit rotation operations on data pointers based on conditional checks and counter values.
- `rotate_and_copy_bits_2`: Rotates bits through multiple registers while managing data movement between memory locations, using counters to control the process and update a destination pointer.
- `rotate_and_copy_bits`: Rotates bits through multiple registers while managing data movement between memory locations, using counters to control the process and updating a global variable with rotated values.
- `frag_moveq_bsr_addq_move_bra`: Short code fragment that loads 2 into d1, then calls another routine, then adds 7 to d2, then copies d2 to d3, then jumps to another point in the code.
- `frag_moveq_bsr_move_bra`: Short code fragment that loads 8 into d1, then calls another routine, then copies d2 to d3, then jumps to another point in the code.
- `frag_moveq_lsr_bne_bsr_bcs_bra`: Short code fragment that loads 8 into d1, then shifts d0 right by 1, then branches if the last result was not zero, then calls another routine, then branches if the carry flag is set, then loads 14 into d1, then jumps to another point in the code.
- `shift_and_copy`: Shifts a 32-bit value right by one bit and manages bit manipulation for data copying, using a counter and pointer arithmetic to move bytes between memory locations.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `frag_subq_clr_lsr_beq_roxl_dbra_rts`: Short code fragment that subtracts 1 from d1, then clears d2, then shifts d0 right by 1, then branches if the last result was zero, then rotates d2 left through the extend flag by 1, then loops back while counting down d1, then returns.
- `refill_bits_backward_until_count`: Repeatedly refills a bit register from the longword before the source pointer and shifts bits out, returning when a count runs out.

### Chuckie Egg II.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware`: Initializes Amiga hardware registers including display and interrupt control, sets up a stack pointer, calls a routine to enter supervisor mode, and configures various custom chip registers for graphics rendering.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_clr_move_bsr_bne_btst_beq_rts`: Short code fragment that clears a fixed memory address, then copies 1 to d0, then calls another routine (2 times in a row), then branches if the last result was not zero, then tests bit 6 of a fixed memory address, then branches if the last result was zero, then tests bit 7 of a fixed memory address, then branches if the last result was not zero, then returns.
- `frag_tst_bne_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then jumps to another point in the code.
- `process_joystick_input`: Reads joystick status from a hardware register, performs bitwise operations on the data, and stores the result in a global variable after combining it with existing data.
- `process_color_data`: Processes color data from a source table by multiplying components with a scaling factor, shifts bits to combine values, and stores the results in a destination buffer while iterating through multiple passes.
- `scale_color_data_up`: Processes color data from a source table by multiplying components with an increasing scaling factor, shifts bits to combine values, and stores the results in a destination buffer while incrementing the scale factor through multiple passes.
- `scale_color_data_down`: Processes color data from a source table by multiplying components with a decreasing scaling factor, shifts bits to combine values, and stores the results in a destination buffer while decrementing the scale factor through multiple passes.
- `frag_move_cmp_beq_bsr_bra`: Short code fragment that copies the memory at a0 (advancing a0) to d0, then compares d0 with 255, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_cmp_beq_bsr_bra_2`: Short code fragment that copies the memory at a0 (advancing a0) to d0, then compares d0 with 255, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `frag_move_cmp_beq_bsr_bra_3`: Short code fragment that copies the memory at a0 (advancing a0) to d0, then compares d0 with 255, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `calculate_table_offset`: Processes a value in a data register to calculate an address offset, then uses that offset to access memory locations in a table and performs multiplication and addition operations on the accessed data.
- `update_pattern_table`: Updates a pattern table by rotating bits in a global variable and performing bit operations on memory locations based on control values, then advances pointers and loops through a fixed number of iterations.
- `rotate_and_copy`: Rotates data registers and performs bit operations on memory locations based on control values, then advances pointers and loops through a fixed number of iterations.
- `frag_ror_bcs_bra`: Short code fragment that rotates d3 right by 1, then branches if the carry flag is set, then jumps to another point in the code.
- `frag_ror_bcs_bra_2`: Short code fragment that rotates d2 right by 1, then branches if the carry flag is set, then jumps to another point in the code.
- `frag_movea_adda_dbra_addq_rts`: Short code fragment that copies a3 to a2, then adds 8000 to a4, then copies a4 to a1, then loops back while counting down d5, then adds 1 to a fixed memory address, then returns.
- `frag_move_or_adda_dbra_bra`: Short code fragment that copies the memory at a2 (advancing a2) to d0, then merges d0 into the memory at a1, then adds 40 to a1, then loops back while counting down d4, then jumps to another point in the code.
- `frag_move_not_and_adda_dbra_bra`: Short code fragment that copies the memory at a2 (advancing a2) to d0, then inverts d0, then masks the memory at a1 with d0, then adds 40 to a1, then loops back while counting down d4, then jumps to another point in the code.
- `frag_move_adda_dbra_bra`: Short code fragment that copies 255 to the memory at a1, then adds 40 to a1, then loops back while counting down d4, then jumps to another point in the code.
- `frag_move_not_adda_dbra_bra`: Short code fragment that copies the memory at a2 (advancing a2) to d0, then inverts d0, then copies d0 to the memory at a1, then adds 40 to a1, then loops back while counting down d4, then jumps to another point in the code.
- `frag_move_adda_dbra_bra_2`: Short code fragment that copies the memory at a2 (advancing a2) to the memory at a1, then adds 40 to a1, then loops back while counting down d4, then jumps to another point in the code.
- `frag_move_adda_dbra_bra_3`: Short code fragment that copies 0 to the memory at a1, then adds 40 to a1, then loops back while counting down d4, then jumps to another point in the code.
- `frag_cmp_bne_move_rts`: Short code fragment that compares d0 with 1, then branches if the last result was not zero, then copies the memory at a0 (advancing a0) to a fixed memory address (2 times in a row), then returns.
- `frag_cmp_bne_move_rts_2`: Short code fragment that compares d0 with 2, then branches if the last result was not zero, then copies the memory at a0 (advancing a0) to a fixed memory address, then returns.
- `frag_cmp_bne_move_rts_3`: Short code fragment that compares d0 with 3, then branches if the last result was not zero, then copies the memory at a0 (advancing a0) to a fixed memory address, then returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_game_state`: Updates game state variables including counters and flags, performs data shuffling operations on tables, and handles conditional logic based on global state values.
- `process_keyboard_input`: Reads a byte from the keyboard serial register, rotates it right by one bit, and stores the result in a table at an offset determined by the rotated value.
- `update_hardware_status`: Sets a hardware register to 8, reads a byte from a global variable, uses that byte as an index to read another byte from a table, stores the table byte in a global variable, clears the global variable, sets the hardware register to 32776, and returns the table byte.
- `rotate_and_accumulate`: Performs a bitwise rotation operation on data from two tables, accumulating the results into a register through multiple iterations.
- `system_reset`: Executes a system reset instruction followed by an unconditional jump to a fixed memory address.
- `frag_move_tst_bne_rts`: Short code fragment that copies d0 to a fixed memory address, then tests a fixed memory address, then branches if the last result was not zero, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra`: Short code fragment that jumps to another point in the code.
- `configure_audio_channels`: Configures audio channel registers based on interrupt status flags and enables DMA channels, then returns with updated register values.
- `frag_tst_beq_lea_move_movea_bra`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was zero, then loads the address of a location in the code into a0, then copies a0 to the memory at an offset from a5, then copies the memory at an offset from a0 to a1, then jumps to another point in the code.
- `update_and_modify_state`: Updates various state values in a structure based on conditional checks and arithmetic operations, then performs bit manipulation and memory copying.
- `accumulate_and_copy_data`: Adds a byte from memory to another byte within a structure, then copies a multi-byte block to a new destination.
- `update_counter_and_value`: Updates a counter and performs arithmetic operations on a value based on the state of several flags.
- `frag_move_cmp_bne_cmpi_beq_clr_bclr_rts`: Short code fragment that copies the memory at an offset from a0 to d0, then compares d0 with the memory at an offset from a0, then branches if the last result was not zero, then compares the memory at a1 with 135, then branches if the last result was zero, then copies 1 to the memory at an offset from a0, then clears the memory at an offset from a0, then clears bit 1 of the memory at an offset from a0, then returns.
- `frag_btst_bne_rts`: Short code fragment that tests bit 2 of the memory at an offset from a0, then branches if the last result was not zero, then returns.
- `decrement_and_modify`: Checks a bit in memory, decrements a counter if the bit is set, and performs arithmetic operations on a value based on the sign of another value.
- `update_byte_sequence`: Updates a byte value in a sequence based on a counter and bit manipulation, then returns the adjusted counter value.
- `fill_byte_sequence`: Fills a sequence of bytes with 127 based on a counter value from a structure, then writes 128 and returns the adjusted counter value.
- `update_timer_and_reset`: Updates a timer counter and performs conditional operations based on its value.
- `update_position_and_status`: Updates a position value in a structure and conditionally modifies a status byte based on a bit test result.
- `initialize_game_state`: Initializes various game state variables and performs a series of operations including clearing multiple bytes, setting timer values, and calling another routine four times with different parameters.
- `initialize_pointers`: Loads word values from a table, uses them as offsets to calculate pointers, and stores these pointers in a structure.
- `validate_and_initialize`: Validates input data against thresholds, updates state variables, and initializes game objects with specific values before branching to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_display`: Sets up display registers including bitplane control, double data fetch start/stop, and window start/stop values.
