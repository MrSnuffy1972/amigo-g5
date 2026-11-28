# Game 4090 (john barnes european football (1992)(krisalis software)(m5)[cr csl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04090). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903904 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 84 of 84 named (0 author symbols), 84 of 84 described (46 by rule, 27 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 206 of 206 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1024 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 947 of 947 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 5088 |
| data | 897088 |
| padding | 192 |
| unused | 1536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| John Barnes European Football (1992)(Krisalis Software)(M5)[cr CSL].adf:bootblock | 1 | 176 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 771 | 71 |
| John Barnes European Football (1992)(Krisalis Software)(M5)[cr CSL].adf bytes 1024-2048 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- john barnes european football (1992)(krisalis software)(m5)[cr csl]/John Barnes European Football (1992)(Krisalis Software)(M5)[cr CSL].adf: custom-format (non-DOS) disk, 1755 non-empty blocks: matched against track-loaded segments
- reference run failed (Karl 4 Nov 2026): run play-lgpc01 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 18143 of its 18357 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-lgpc02 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 18143 of its 18357 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-lgpc03 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 18143 of its 18357 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-lgpwide slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 18143 of its 18357 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m1500 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 18143 of its 18357 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-m3000 slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 18143 of its 18357 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-menu slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 18143 of its 18357 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-mixed slid through memory (an address: 1 64 KB pages each with >= 29491 distinct PCs executed, 18143 of its 18357 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address: 206 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- john barnes european football (1992)(krisalis software)(m5)[cr csl]/John Barnes European Football (1992)(Krisalis Software)(M5)[cr CSL].adf: bytes 1024-2048 look like code no run loaded: recovered position-independently (load address unknown)
- john barnes european football (1992)(krisalis software)(m5)[cr csl]/John Barnes European Football (1992)(Krisalis Software)(M5)[cr CSL].adf: 1 logged loads cover 1024 bytes of the image
- john barnes european football (1992)(krisalis software)(m5)[cr csl]/John Barnes European Football (1992)(Krisalis Software)(M5)[cr CSL].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- john barnes european football (1992)(krisalis software)(m5)[cr csl]/John Barnes European Football (1992)(Krisalis Software)(M5)[cr CSL].adf: blocks 8-18 (bytes 4096-9728) outside the filesystem hold data, not code (opcode density 0.040, entropy 6.92)
- john barnes european football (1992)(krisalis software)(m5)[cr csl]/John Barnes European Football (1992)(Krisalis Software)(M5)[cr CSL].adf: blocks 22-1759 (bytes 11264-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.90)
- 1 never-loaded region(s), 1024 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### John Barnes European Football (1992)(Krisalis Software)(M5)[cr CSL].adf:bootblock

- `initialize_dma_transfer`: Sets up and executes a DMA I/O request for memory block transfer, configures hardware registers for DMA and interrupt control, then jumps to a setup routine.
- `store_slots_and_copy_block`: Writes three successive addresses into one fixed memory slot, calling an unnamed routine after each write.
- `frag_movea_lea_move_bsr_jmp`: Short code fragment that copies a5 to a2, then loads the address of a fixed memory address into a0, then copies 383 to d0, then calls another routine, then loads the address of a fixed memory address into a0, then copies 2047 to d0, then calls another routine, then jumps to another point in the code.
- `frag_move_dbra_rts_bra`: Short code fragment that copies the memory at a2 (advancing a2) to the memory at a0 (advancing a0), then loops back while counting down d0, then returns, then copies 3840 to a fixed memory address, then copies 0 to a fixed memory address, then jumps to another point in the code.
- `write_fat_marker_by_flag`: Writes one of two four-character text markers, FAT0 or FAT1, into a memory location, choosing FAT1 when a flag bit of a global value is set.
- `detect_refresh_rate_hz`: Resets the CIA-A and CIA-B port and timer registers, then reads a counter from two timer bytes and returns 50 or 60 depending on whether it is below a threshold.
- `wait_until_status_byte_clear`: Loops reading a status byte at a fixed offset from a base register until its top bit is clear, then returns minus one.
- `initialize_memory_block`: Clears a block of memory with zeros, writes fixed signature values at specific offsets, and performs a series of validation checks on memory regions while storing results at designated addresses.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (movea.l, movea.l) and then falls through into the code that follows, with no return of its own.
- `check_pointer_condition`: Tests a pointer for zero value and reads a word from a fixed offset, then performs a bit test on that word using a register value as the bit index.
- `load_and_branch`: Loads a byte from a table using a register value as an index, then conditionally branches based on that byte's value.
- `frag_movea_move_swap_rts_moveq`: Short code fragment that copies a1 to a7, then copies the top of the stack to the memory at a0, then swaps the halves of d0, then copies 17232 to d0, then swaps the halves of d0, then copies d0 to the memory at an offset from a5, then returns, then loads 0 into d1.
- `setup_io_request`: Sets up an I/O request structure with fixed values for buffer address, transfer length, and command, then calls the Amiga OS library to execute the I/O operation and subsequently calls into loaded code at a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_interrupt_loop`: Waits for a specific interrupt signal, then processes a series of operations through a jump table based on calculated values from a data stream.
- `frag_move_add_rts`: Short code fragment that copies the memory at an offset from a5 to d0, then adds d0 to d0 (2 times in a row), then copies an indexed entry from a5 to d0, then returns.
- `process_data_with_shift`: Calls another routine to get a value, stores it in memory, performs bit manipulation operations on that value, calculates an offset using a table, and calls another routine with the calculated value.
- `frag_moveq_bsr_tst_beq_move_add_addi_rt`: Short code fragment that loads 0 into d1, then calls another routine, then tests d0, then branches if the last result was zero, then copies the memory at an offset from a5 to d1, then adds d1 to d1 (2 times in a row), then adds 32 to d0, then copies d0 to an indexed entry from a5, then returns.
- `frag_move_bra`: Short code fragment that copies 3840 to a fixed memory address, then jumps to another point in the code.
- `increment_on_nonzero_flag`: It checks a byte at a fixed offset in a structure and, when that byte is nonzero, clears it to zero and increments the adjacent short value.
- `frag_lea_move_clr_bsr_rts`: Short code fragment that loads the address of a location in the code into a0, then copies a0 to the memory at an offset from a5, then copies d2 to the memory at an offset from a5, then clears the memory at an offset from a5, then calls another routine, then returns.
- `increment_pointer_and_decrement_counter`: Checks a 16-bit value at offset 270 of a structure; if it is between one and three, it reads a word from a pointer, increments that pointer twice, and decrements the structure value by two; otherwise, it reads a longword from a pointer, increments that pointer, and decrements the structure value by four.
- `read_word_with_decrement`: Checks a counter value, if it's not zero or less than four, reads a word from a pointer, increments that pointer, then calls another routine.
- `increment_pointer_by_one`: Checks a 16-bit value at offset 270 of a structure; if it is zero, it calls a subroutine, otherwise it reads a byte from a pointer at offset 4 of the structure, increments that pointer by one, and decrements the structure value by one.
- `frag_movem_bsr_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `process_data_with_loop_2`: Processes data through a loop that iterates a calculated number of times, performing bit operations and conditional branches based on comparison results.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_moveq_bra`: Short code fragment that loads 7 into d1, then loads 8 into d4, then jumps to another point in the code.
- `rotate_right_extend_long`: Returns a 32-bit word read from memory, shifted right one bit, with the extend-flag bit placed at the top and the top bit forced set.
- `frag_clr_lsr_beq_addx_dbra_rts`: Short code fragment that clears d2, then shifts d0 right by 1, then branches if the last result was zero, then adds d2 and the extend flag to d2, then loops back while counting down d1, then returns.
- `frag_move_eor_roxr_bset_bra`: Short code fragment that copies the memory just below a0 (moving a0 down) to d0, then exclusive-ors d0 into d5, then rotates d0 right through the extend flag by 1, then sets bit 31 of d0, then jumps to another point in the code.
- `process_data_branch`: Processes conditional branches based on the value in a data register, calling different routines depending on whether the value is less than two, less than three, or otherwise.
- `process_data_with_loop`: Processes data through a loop that iterates a calculated number of times, performing bit operations and conditional branches based on comparison results, then returns a value from a global variable.
- `frag_move_addi_bsr_rts`: Short code fragment that copies 5632 to the memory at an offset from a5, then copies the memory at an offset from a5 to d0, then copies d0 to the memory at an offset from a5, then copies 5632 to d1, then copies the memory at an offset from a5 to d2, then adds 11 to the memory at an offset from a5, then calls another routine, then returns.
- `sum_tagged_ram_records`: Scans 64 entries of a fixed low-memory table and, for each entry whose tag matches the text RAM followed by M, adds the 32-bit value stored eight bytes after the tag to a running total, which it returns.
- `frag_lea_moveq_cmp_bne_move_movea_tst_r`: Short code fragment that loads the address of a fixed memory address into a0, then loads 63 into d7, then compares d0 with the memory at a0, then branches if the last result was not zero, then copies the memory at a0 (advancing a0) to d1, then copies the memory at a0 (advancing a0) to a1, then copies the memory at a0 (advancing a0) to d0, then tests d1, then returns.
- `decrement_and_branch`: Decrements a1 by two, performs a loop based on d7, sets d0 to zero, and returns.
- `clear_and_validate`: Clears a table of words at a fixed address, then performs multiple validation checks on memory signatures and addresses, updating flags based on comparison results.
- `tiny_move_clr_rts`: Small routine of 3 instructions that copies d0 to (a0), then clears (a1), then returns.
- `validate_memory_signature`: Compares memory values at specific offsets with fixed signatures, performs arithmetic operations on register values, and validates addresses through multiple comparisons before returning.
- `frag_move_clr_movea_rts`: Short code fragment that copies d0 to the memory at an offset from a2, then clears the memory at an offset from a2, then copies the top of the stack to a2, then returns.
- `frag_addq_tst_beq_lea_bra`: Short code fragment that adds 4 to a0, then tests the memory at a0, then branches if the last result was zero, then loads the address of the memory at an offset from a0 into a0, then jumps to another point in the code.
- `write_memory_block`: Writes three specific longwords to memory locations starting at a register pointer, with the third value being a modified version of a data register.
- `frag_movem_bsr_rts_2`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `check_signature_and_call`: Compares a register value with a fixed signature, stores a register value on the stack, tests a memory byte, and performs conditional branches based on the results of these operations.
- `frag_move_bsr_bne_bra`: Short code fragment that copies d0 to the stack, then calls another routine, then branches if the last result was not zero, then copies the top of the stack to d0, then copies an address to d1, then jumps to another point in the code.
- `frag_movem_bsr_rts_3`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_lea_st_bsr_sf_rts`: Short code fragment that loads the address of a location in the code into a5, then sets the memory at an offset from a5 to all ones, then calls another routine, then clears the memory at an offset from a5, then returns.
- `check_control_byte`: Compares a register value with a fixed signature, then stores a register value on the stack, tests a memory byte, and performs conditional branches based on that test result.
- `tiny_addq_tst_rts`: Small routine of 3 instructions that adds 4 to a7, then tests d0, then returns.
- `frag_movem_bsr_rts_4`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `manage_memory_allocation`: Performs bit operations on a register value, allocates memory by adjusting pointers, and manages a memory pool size through comparisons and arithmetic operations before calling another routine.
- `frag_bsr_add_move_movea_clr_rts`: Short code fragment that calls another routine, then adds a1 to d0, then copies d0 to the memory at a1 (advancing a1), then copies d0 to a0, then clears the memory at a0, then copies the top of the stack to d0, then calls another routine, then copies a1 to d0, then returns.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `frag_tst_bne_movem_lsr_clr_subq_rts`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was not zero, then saves registers on the stack, then shifts d0 right by 2, then clears the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.
- `frag_tst_beq_movea_bra`: Short code fragment that tests the memory at a1, then branches if the last result was zero, then copies the memory at a1 to a1, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_movem_bsr_rts_5`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `validate_and_return`: Validates three signatures in sequence by calling another routine with different values, and returns the result of the last validation check.
- `return_pair_after_hardware_call`: Calls one unnamed routine and returns the two data registers unchanged as a 64-bit result.
- `return_pair_after_hardware_call_2`: Saves all registers to the stack, calls an unnamed routine, then restores registers and returns the two data registers unchanged as a 64-bit result.
- `step_counter_modulo_eleven`: Stores two data registers into a structure, divides a 16-bit count by eleven and stores the quotient and remainder in fields.
- `modify_control_register`: Reads a byte from a fixed offset, modifies it through bit operations including setting and clearing specific bits, then calls another routine with a predefined word value before returning.
- `frag_bsr_btst_beq_bra`: Short code fragment that calls another routine (2 times in a row), then tests bit 4 of the memory at a2, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_counter_and_wait`: Compares two values in memory, updates a counter based on that comparison, performs bit tests on the counter, and calls routines conditionally based on those tests while waiting for a specific bit to be set.
- `frag_bset_rts`: Short code fragment that sets bit 1 of the memory at an offset from a1, then returns.
- `frag_bclr_rts`: Short code fragment that clears bit 1 of the memory at an offset from a1, then returns.
- `poll_status_bit_with_limit`: Clears the top bit of a control byte and then a counter-selected bit, then polls a status byte through an unnamed routine up to fifteen times, returning as soon as one bit is clear.
- `frag_move_bsr_lsr_btst_beq_rts`: Short code fragment that copies d0 to the stack, then calls another routine, then copies the top of the stack to d0, then copies d0 to the memory at an offset from a1, then shifts d0 right by 8, then copies d0 to the memory at an offset from a1, then tests bit 0 of the memory at an offset from a1, then branches if the last result was zero, then returns.
- `frag_move_andi_addq_rts`: Short code fragment that copies the memory at an offset from a1 to d0, then masks d0 with 192, then adds 8 to d0, then copies d0 to the memory at an offset from a1, then copies 127 to the memory at an offset from a1, then returns.
- `frag_bsr_move_bra`: Short code fragment that calls another routine, then copies 12888 to d0, then jumps to another point in the code.
- `frag_bclr_moveq_bsr_bset_move_bra`: Short code fragment that clears bit 0 of the memory at an offset from a1, then loads 80 into d0, then calls another routine, then sets bit 0 of the memory at an offset from a1, then copies 2148 to d0, then jumps to another point in the code.
- `frag_move_ori_moveq_bsr_rts`: Short code fragment that copies d0 to the stack, then merges 120 into the memory at an offset from a1, then loads 80 into d0, then calls another routine, then copies the top of the stack to d0, then returns.
- `start_sync_word_transfer`: Writes a fixed sync word into a buffer and into several hardware registers, then polls a status bit through an unnamed routine up to fifteen times.
- `process_data_with_validation`: Processes data by comparing values against fixed signatures, performs bit rotation operations, and handles conditional branches based on comparison results while updating memory locations with processed data.
- `merge_masked_words_doubled`: Combines two memory words under a mask: bits from the first word are kept where the mask is set, and bits from the second word are doubled before being merged in.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `parse_color_map`: Parses a color map from a data stream, copying palette entries into a global table while advancing a pointer through the input data.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 252 into d4, then returns.
- `frag_moveq_rts`: Short code fragment that loads 251 into d4, then returns, then loads 250 into d4, then returns.
- `frag_andi_lsr_moveq_move_mulu_subq_rts`: Short code fragment that masks d1 with 65520, then shifts d1 right by 3, then loads 0 into d4, then copies d2 to d4, then multiplies d4 by d1 (unsigned), then subtracts 1 from a fixed memory address, then copies a fixed memory address to d0, then subtracts 1 from d1, then subtracts 1 from d2, then returns.
- `frag_bsr_movem`: Short code fragment that calls another routine, then saves registers on the stack (2 times in a row).
