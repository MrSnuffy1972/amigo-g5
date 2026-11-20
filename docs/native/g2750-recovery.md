# Game 2750 (Astatin v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02750). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2022272 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 84 of 84 named (0 author symbols), 84 of 84 described (31 by rule, 36 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1486 of 1486 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 308440 |
| code | 5104 |
| data | 1239476 |
| padding | 2308 |
| unused | 466944 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk.1:Type | 2 | 117 | 2 |
| Disk.1:game | 5 | 250 | 17 |
| Disk.1:BORDER | 3 | 225 | 6 |
| Disk.1:astatin-intro | 1 | 825 | 50 |
| game.slave | 1 | 59 | 8 |
| Disk.1:bootblock | 1 | 10 | 1 |

## Cases handled

- Astatin_v1.0/Astatin_v1.0.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree Astatin_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- Astatin_v1.0/Astatin_v1.0_hdf/Astatin/data/Disk.1:game: Imploder (xfd) stream found by ancient scan (74464 -> 224356 bytes) but only 0 of 56 sampled windows are in reference RAM: not used
- Astatin_v1.0/Astatin_v1.0_hdf/Astatin/data/Disk.1:BORDER: TNM Cruncher 1.1 (xfd) stream found by ancient scan (5576 -> 6752 bytes) but only 0 of 3 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target

## Cracktros (kept, not protection)

- Disk.1:astatin-intro

## Copy protection found

- none found

## Functions


### Disk.1:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### Disk.1:game

- `init_and_check_pattern`: Calls several routines and runs a loop three times, invoking a library function through a global base.
- `frag_move_lea_movea_adda_dbra_addq_rts`: Short code fragment that copies d6 to d7, then loads the address of a location in the code into a1, then copies the memory at a1 to a1, then adds a1 to a1 (2 times in a row), then loops back while counting down d7, then adds 4 to a1, then returns.
- `clear_cache_and_loop`: Increments a counter and performs a loop of operations, clearing a cache when a condition is met.
- `process_loop_iterations`: Processes a loop of iterations by loading a value from a global variable, checking if it's zero, and if not, calling another routine and performing arithmetic operations on data pointed to by a3.
- `frag_beq_movea_bsr_bra`: Short code fragment that branches if the last result was zero, then copies a3 to a1, then calls another routine, then jumps to another point in the code.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_beq_move_subq_bne_rts`: Short code fragment that compares a2 with a6, then branches if the last result was zero, then copies the memory just below a2 (moving a2 down) to the memory at a1 (advancing a1) (4 times in a row), then subtracts 2 from d0, then branches if the last result was not zero, then copies the memory just below a1 (moving a1 down) to d0, then returns.
- `decode_bit_pattern_3`: Reads bits from a buffer using shift operations and stores decoded bytes into another buffer while updating counters and status flags.
- `frag_moveq_add_bne_move_addx_dbra_addq`: Short code fragment that loads 2 into d7, then adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d0 and the extend flag to d0, then loops back while counting down d7, then adds 6 to d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 5 into d0, then loads 3 into d7, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 4 into d0, then loads 2 into d7, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 3 into d0, then loads 1 into d7, then jumps to another point in the code.
- `decode_bit_pattern_2`: Reads bits from a buffer using shift operations and stores decoded bytes into another buffer while updating counters.
- `frag_add_bne_move_addx_bra`: Short code fragment that adds d1 to d1, then branches if the last result was not zero, then copies the memory just below a0 (moving a0 down) to d1, then adds d1 and the extend flag to d1, then adds d2 and the extend flag to d2, then jumps to another point in the code.
- `decode_bit_pattern`: Reads bits from a buffer using shift operations and stores decoded bytes into another buffer while updating counters.
- `decode_pattern`: Decodes a pattern from a buffer, processes bits using shift operations, and copies decoded bytes to another buffer while updating counters and pointers.
- `frag_move_movem_bra`: Short code fragment that copies d0 to the memory just below a4 (moving a4 down), then copies d1 to the memory just below a4 (moving a4 down), then copies d2 to the memory just below a4 (moving a4 down), then restores saved registers from the stack, then jumps to another point in the code.

### Disk.1:BORDER

- `frag_bsr_jmp`: Short code fragment that calls another routine, then jumps to another point in the code.
- `prepare_border_buffer`: Initializes border data structures by allocating memory blocks based on table entries, setting up pointers and counters, and processing relocation information for graphical elements.
- `update_border_data`: Updates border data by incrementing a counter and setting a flag, then processes a loop that modifies memory addresses based on table lookups and offsets.
- `unpack_stream_with_pointer_shift`: Decompresses a packed stream like its neighbour, reading bits through a shift routine that takes a 32-bit value from the four bytes before the source pointer, shifts it right and sets its top bit.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bit_buffer_refill`: Shifts a 32-bit bit buffer right by a count taken from the second data register, reloading it from the previous longword when it empties and setting the top bit on each refill.

### Disk.1:astatin-intro

- `initialize_graphics_and_dma`: Initializes graphics data by writing predefined 16-bit values to multiple memory locations with specific offset calculations, sets up display parameters, configures sprite DMA, and manages interrupt and DMA control registers.
- `initialize_graphics_data`: Writes predefined 16-bit values to multiple memory locations, setting up graphics-related data structures with specific offset calculations and register manipulations.
- `setup_display_parameters`: Initializes display parameters by writing 16-bit values to various memory addresses, performing arithmetic operations on a register before each write.
- `copy_and_transform_data`: Copies data between memory locations while transforming 16-bit values through arithmetic and bit manipulation operations, repeating this process sixteen times.
- `setup_sprite_dma`: Saves registers to the stack, writes a value to a hardware register, calls another routine, compares a global variable with a constant, and branches based on the result.
- `handle_state_transitions`: Checks a global variable's value and branches to different code paths based on whether it equals specific constants, calling other routines for each branch.
- `frag_move_cmpi_beq_addi_rts`: Short code fragment that copies a fixed memory address to d0, then compares d0 with 2, then branches if the last result was zero, then adds 1 to d0, then copies d0 to a fixed memory address, then returns.
- `align_coordinates_2`: Clears a word at a specific memory location, initializes counters, copies word values from two addresses, calls another routine, updates memory locations, and repeats until a counter reaches a limit.
- `align_coordinates`: Aligns coordinate values by masking low bits, comparing differences, and adding increments when values are equal, processing three bit groups sequentially.
- `frag_move_cmpi_beq_addi_rts_2`: Short code fragment that copies a fixed memory address to d0, then compares d0 with 2, then branches if the last result was zero, then adds 1 to d0, then copies d0 to a fixed memory address, then returns.
- `adjust_alignment_2`: Clears a word at a specific memory location, initializes counters, copies a word value from an address with an offset, calls another routine, updates memory locations, and repeats until a counter reaches a limit.
- `adjust_alignment`: Adjusts alignment values by masking low bits, checking for zero, subtracting increments, and adding adjusted values, processing three bit groups sequentially.
- `process_input_data`: Initializes counters, reads a byte from memory, subtracts a constant from it, initializes another counter, calls two routines, increments the first counter, and repeats until the counter reaches a limit.
- `frag_moveq_cmpi_bmi_addi_subi_bra_add_r`: Short code fragment that loads 0 into d2, then compares d1 with 20, then branches if the last result was negative, then adds 640 to d2, then subtracts 20 from d1, then jumps to another point in the code, then adds d1 to d1, then adds d1 to d2, then returns.
- `copy_and_transform_blocks`: Copies 16-bit values between memory locations in a loop, performing arithmetic operations on indices and repeating this process multiple times.
- `frag_cmpi_beq_rts`: Short code fragment that compares a fixed memory address with 1, then branches if the last result was zero, then returns.
- `process_data_with_multiplication`: Processes data by reading from a table, multiplying by constants, calculating differences, storing a byte value, and writing transformed values to memory locations.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address (2 times in a row), then returns.
- `init_music_sample_table`: Searches a byte table for the smallest value, then builds a pointer table by linking zeroed entries to sample start addresses indexed from a second table, sets bit 1 of a global variable, clears several global variables, and returns.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `frag_movem_lea_addq_move_cmp_blt_clr_br`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then adds 1 to a fixed memory address, then copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the comparison was less, then clears a fixed memory address, then jumps to another point in the code.
- `update_audio_dma`: Loads DMA control and channel addresses/lengths from lookup tables into audio hardware registers for four audio channels, then branches to a loop.
- `calculate_division_result`: Initializes a counter, reads a byte from memory, divides it by three, swaps the result, compares with zero, and branches to different locations based on the comparison results.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `lookup_and_compare_values`: Performs a lookup in a table using a register index, compares the result with another value, and advances the table pointer when the comparison fails.
- `frag_move_rts`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `update_audio_dma_params`: Processes audio configuration data from a structure, calculates DMA parameters, clears a hardware register, and updates audio channel addresses and lengths in the custom hardware registers.
- `update_dma_parameters_2`: Reads configuration data from a structure, processes it to update DMA settings, and writes the results to hardware registers.
- `update_dma_parameters`: Updates DMA configuration values in a structure and writes to hardware registers based on conditional checks.
- `configure_dma_parameters`: Copies values between memory locations, masks a value with 4095, writes a value to a hardware register, clears a byte at an offset, and performs several memory operations before branching.
- `update_audio_hardware`: Updates audio hardware registers by setting up DMA control and channel addresses/lengths from lookup tables, increments a global counter, and resets it when reaching 400 while updating other global variables.
- `frag_move_andi_clr_cmp_beq_bge_rts`: Short code fragment that copies the memory at a6 to d2, then masks d2 with 4095, then copies d2 to the memory at an offset from a6, then copies the memory at an offset from a6 to d0, then clears the memory at an offset from a6, then compares d2 with d0, then branches if the last result was zero, then branches if the comparison was greater or equal, then copies 1 to the memory at an offset from a6, then returns.
- `frag_clr_rts_2`: Short code fragment that clears the memory at an offset from a6, then returns.
- `manage_timing_counters`: Manages timing counters by updating and comparing values in a structure, then writes the final counter value to another location.
- `update_timing_values`: Subtracts a value from memory, compares two values, branches if the first is less than the second, updates memory locations with new values, and returns.
- `calculate_phase_value`: Reads a byte from offset 3 of a structure in a6, stores it at offset 26, then reads another byte from offset 27 and uses it to index into a table.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_move_rts_2`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `process_state_machine`: Reads a byte from offset 2 of a structure in a6, masks it with 15, and uses the result as a switch case to execute different operations on data in the same structure.
- `update_counter_clamped`: Loads a byte from offset 3 of a structure in a6, shifts it right by 4 bits, and tests if the result is zero.
- `decrement_counter_clamped_zero`: Subtracts the low nibble of a parameter byte from a 16-bit counter, clamps the counter at zero, and copies it to an output field.
- `subtract_and_clamp_word`: Subtracts a byte from a word, clamps the result to a minimum threshold, and stores the lower twelve bits of the result into a global variable.
- `add_and_clamp_word`: Adds a byte to a word, clamps the result to a maximum threshold, and stores the lower twelve bits of the result into a global variable.
- `dispatch_table_operation`: Reads a byte from offset 2 of a structure in a6, masks its lower nibble, and compares it against several constants.
- `update_global_from_ciaa_pra`: Masks bit zero of the byte at offset 3 of the structure in a6, shifts it left by one, and ORs the result into a global variable after clearing bit one of that variable.
- `frag_not_rts`: Short code fragment that inverts a fixed memory address, then returns.
- `invert_byte_from_pointer`: The routine reads a byte from an address computed from an uninitialized pointer plus a small offset, subtracts one from it, then stores the bitwise NOT of the result into a global variable.
- `clamp_parameter_to_maximum`: Clamps a parameter byte to a maximum of 64 and copies the result to an output field.
- `update_global_with_mask`: Checks if the lower five bits of a byte at offset 3 of a structure in a6 are non-zero; if so, it clears a global longword and updates another global longword with the masked value.

### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `compare_memory_blocks`: Calculates a pointer from a register value, reads a byte count from the pointed location, and compares memory blocks using a loop.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movem_movea_move_addi_cmpi_bne_rts`: Short code fragment that saves registers on the stack, then copies a0 to a1, then copies a1 to d0, then adds 1276 to d0, then copies d0 to a0, then compares the memory at a0 with an address, then branches if the last result was not zero, then copies an address to the memory at a0, then restores saved registers from the stack, then returns.
- `frag_cmpa_beq_jmp`: Short code fragment that compares a4 with a location in the code, then branches if the last result was zero, then jumps to another point in the code.
- `frag_lea_movem_move_tst_beq_bsr_jmp`: Short code fragment that loads the address of the memory at an offset from a5 into a6, then saves registers on the stack, then copies the memory at a1 to d0, then tests d1, then branches if the last result was zero, then calls another routine, then restores saved registers from the stack, then calls another routine, then jumps to the address in a6.
- `frag_move_addi_rts`: Short code fragment that copies a location in the code to the stack, then adds 32 to the top of the stack, then returns.
- `wait_for_vpos_bit_low_then_high`: Polls bit zero of the custom chip register at offset five until it reads low, then polls again until it reads high, then returns.

### Disk.1:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
