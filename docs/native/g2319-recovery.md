# Game 2319 (smash tv (1991)(ocean)[cr flt][t +4 legend]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02319). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 165 of 165 named (0 author symbols), 165 of 165 described (61 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 96 of 96 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2289 of 2289 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 326649 |
| code | 6868 |
| data | 519976 |
| padding | 11 |
| unused | 47616 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf:bootblock | 1 | 72 | 4 |
| logged disk load from Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf boot-block load | 1 | 92 | 2 |
| logged disk load from Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf boot-block load | 1 | 252 | 10 |
| logged disk load from Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf boot-block load | 1 | 1873 | 149 |

## Cases handled

- smash tv (1991)(ocean)[cr flt][t +4 legend]/Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf: custom-format (non-DOS) disk, 1664 non-empty blocks: matched against track-loaded segments
- 96 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- smash tv (1991)(ocean)[cr flt][t +4 legend]/Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf: the boot block reads 3584 bytes from disk offset 897024 to an address (found statically); recovered as a unit
- smash tv (1991)(ocean)[cr flt][t +4 legend]/Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf: the boot block reads 22528 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- smash tv (1991)(ocean)[cr flt][t +4 legend]/Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf: the boot block reads 335872 bytes from disk offset 1536 to an address (found statically); recovered as a unit
- smash tv (1991)(ocean)[cr flt][t +4 legend]/Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf: 3 logged loads cover 358400 bytes of the image
- smash tv (1991)(ocean)[cr flt][t +4 legend]/Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.44)
- smash tv (1991)(ocean)[cr flt][t +4 legend]/Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf: blocks 659-795 (bytes 337408-407552) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.16)
- 1 packed track region(s), 423424 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf:bootblock

- `initialize_system_and_copy_data_2`: Disables interrupts, sets the stack pointer to a value in a0, calls another routine, configures DMA and bitplane registers, and then copies data.
- `initialize_system_and_copy_data`: Disables interrupts, sets the stack pointer to a value in a0, calls another routine, configures DMA and bitplane registers, and then copies 32 words from a source table to a destination table using a loop before jumping to the destination.
- `frag_lea_move_subq_bgt_jmp`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 83839 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then subtracts 1 from d0, then branches if the comparison was greater, then jumps to another point in the code.
- `wait_for_interrupt_flag`: Polls a global variable until a specific bit is set, then clears that bit.

### logged disk load from Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf boot-block load

- `perform_bit_shifts_2`: Copies a block of memory, then enters a loop that performs logical shifts on a longword in d0 and calls another routine multiple times while updating a value in d2 based on the results of the shifts and carry flags.
- `shift_and_store_word`: Moves a longword from a0 to d0, stores a byte from d0 into a memory location pointed to by a5, and performs a logical shift right on d0.
- `perform_bit_shifts`: Contains a loop that performs logical shifts on a register and calls another routine when the register reaches zero.
- `decode_bitstream`: Processes a bitstream by reading bits from a buffer, refilling the buffer when needed, and performing operations based on the read values.
- `frag_moveq_bra`: Short code fragment that loads 8 into d1, then loads 8 into d4, then jumps to another point in the code.
- `load_and_compare_bits`: Loads a value into a register, calls another routine to process bits, compares the result with specific constants, and branches to different locations based on those comparisons.
- `update_color_register`: Reads a value from a memory location, performs a bitwise AND operation with a constant, and writes the result to the COLOR00 hardware register.
- `frag_move_and_dbra_moveq_rts`: Short code fragment that copies 65535 to d0, then masks a fixed memory address with d0, then loops back while counting down d0, then loads 255 into d0, then returns.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Smash TV (1991)(Ocean)[cr FLT][t +4 Legend].adf boot-block load

- `frag_btst_bne_rte`: Short code fragment that tests bit 5 of a fixed memory address, then branches if the last result was not zero, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then returns from the exception.
- `wait_for_interrupt_flag_2`: Polls a global variable until a specific bit is set, then clears that bit.
- `configure_cia_timers`: Clears two bytes in a global variable, sets timer values and control bits for CIAA and CIAB, and sets the interrupt enable register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_move_bsr_bcc_bcs_rts`: Short code fragment that copies 236 to a fixed memory address, then calls another routine, then branches if the carry flag is clear, then calls another routine, then branches if the carry flag is set, then returns.
- `frag_tst_bne_dbra_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loops back while counting down d0, then returns.
- `frag_move_lsr_st_rts_tst_bne`: Short code fragment that copies 17 to a fixed memory address, then copies d0 to a fixed memory address, then shifts d0 right by 8, then copies d0 to a fixed memory address, then copies 73 to a fixed memory address, then sets a fixed memory address to all ones, then returns, then tests a fixed memory address, then branches if the last result was not zero, then returns.
- `handle_cia_interrupt`: Checks specific bits in the CIAB interrupt register; if bit 1 is set, it clears a byte in a global variable, and if bit 4 is set, it clears another byte in a global variable, then clears the interrupt request register and returns.
- `handle_interrupt_request`: Checks a hardware register for a specific bit, calls another routine if set, increments two global variables, and then returns from interrupt.
- `initialize_display`: Sets up bitplane and display registers including mod values, double data fetch start/stop, display window start/stop, and coprocessor pointers.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `execute_trap_and_stop`: Triggers a trap, sets the status register, checks a byte in a global variable, and either stops the processor or branches to another instruction.
- `frag_move_rts`: Short code fragment that copies 0 to the status register, then returns.
- `update_display_list`: Updates graphics display list entries using data from tables, manipulates register values, and manages a counter in a global variable before returning.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address, then copies 65280 to a fixed memory address, then returns.
- `frag_tst_bmi_move_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was negative, then tests a fixed memory address, then branches if the last result was negative, then copies 0 to the condition codes, then returns.
- `frag_move_rts_2`: Short code fragment that copies 1 to the condition codes, then returns.
- `frag_bsr_bcc`: Short code fragment that calls another routine (2 times in a row), then branches if the carry flag is clear.
- `frag_bsr_bcs_rts`: Short code fragment that calls another routine, then branches if the carry flag is set, then returns.
- `process_keyboard_input`: Reads a byte from the keyboard serial register, clears it, sets a control register, and then performs bitwise operations to store a value in a global variable or a table based on the input.
- `update_input_states`: Reads joystick and mouse data, performs bitwise operations via another routine, and updates several global variables based on the status of CIAA and CIAB registers.
- `manipulate_d0_bits`: Performs a series of bit tests and arithmetic operations on d0 and d1, including a bit change operation, and returns.
- `frag_movea_move_bpl_bclr_moveq_bra`: Short code fragment that copies a location in the code to a0, then copies d0 to d2, then branches if the last result was not negative, then copies a location in the code to a0, then clears bit 7 of d2, then copies 12820 to d0, then loads 4 into d7, then jumps to another point in the code.
- `execute_blt_loop`: Checks a global variable and, if non-zero, calls another routine to configure Blitter registers and enters a loop that updates Blitter parameters using values from a table and a counter.
- `populate_interleaved_data_tables`: Calculates a memory address based on a base pointer and an offset scaled by 200, then uses nested loops to fill five separate destination tables with a value provided by another routine.
- `fill_memory_with_byte`: Uses a loop to write the value from d0 into a table at intervals defined by d1.
- `distribute_words_to_tables`: Uses nested loops to copy words from a source buffer into five different destination tables at specific offsets.
- `frag_movea_move_bra`: Short code fragment that copies a location in the code to a0, then copies a location in the code to a1, then copies 200 to d1, then jumps to another point in the code.
- `frag_movea_move_bra_2`: Short code fragment that copies a location in the code to a1, then copies a location in the code to a0, then copies 200 to d1, then jumps to another point in the code.
- `configure_blt_parameters`: Calculates a value from a register, checks a global byte, calls another routine, and then writes several values to custom hardware registers and a global variable.
- `copy_memory_blocks`: Calculates a destination address based on several registers, calls another routine if a global byte is non-zero, and then uses a loop to copy memory from one location to another.
- `frag_move_dbra_adda_rts`: Short code fragment that copies d2 to d1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d1, then adds d0 to a1, then loops back while counting down d3, then returns.
- `xor_memory_blocks`: Calculates a destination address, calls another routine if a global byte is non-zero, and then uses a loop to exclusive-or memory from a source into a destination.
- `frag_move_eor_dbra_adda_rts`: Short code fragment that copies d2 to d1, then copies the memory at a0 (advancing a0) to d4, then exclusive-ors d4 into the memory at a1 (advancing a1), then loops back while counting down d1, then adds d0 to a1, then loops back while counting down d3, then returns.
- `copy_memory_with_padding`: Calculates a destination address, calls another routine if a global byte is non-zero, and then uses a loop to copy memory while writing specific bytes to various offsets in the destination.
- `frag_move_dbra_adda_rts_2`: Short code fragment that copies d2 to d1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then copies the memory at a0 (advancing a0) to the memory at an offset from a1 (3 times in a row), then copies d0 to the memory at an offset from a1, then loops back while counting down d1, then adds d4 to a1, then loops back while counting down d3, then returns.
- `copy_memory_extended`: Calculates a destination address, calls another routine if a global byte is non-zero, and then uses a loop to copy memory while writing specific bytes to various offsets in the destination using a larger stride.
- `frag_move_dbra_adda_rts_3`: Short code fragment that copies d2 to d1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then copies the memory at a0 (advancing a0) to the memory at an offset from a1 (4 times in a row), then loops back while counting down d1, then adds d4 to a1, then loops back while counting down d3, then returns.
- `merge_bitmask_data_2`: Calculates a destination address, calls another routine if a global byte is non-zero, and then uses a loop to merge bits from a source into a destination using a calculated mask.
- `merge_bitmask_data`: Uses nested loops to read bytes from a source, performs a bitwise AND with a mask derived from a destination table, and then ORs the result back into the destination table at various offsets.
- `execute_multiple_blts`: Calculates a value from a table, calls another routine if a global byte is non-zero, and then performs a sequence of writes to custom hardware registers to trigger multiple hardware operations.
- `execute_complex_blts`: Calculates a destination address and a value from a table, calls another routine if a global byte is non-zero, and then performs a sequence of writes to custom hardware registers to trigger multiple hardware operations with varying parameters.
- `frag_adda_moveq_sub_bsr_exg_move_bra`: Short code fragment that adds 80 to a0, then loads 254 into d1, then loads 0 into d2, then loads 6 into d4, then subtracts d0 from d4, then calls another routine, then exchanges d1 and d2, then copies d4 to d0, then jumps to another point in the code.
- `frag_move`: Short code fragment that copies d1 to the memory at a0 (advancing a0), then copies d2 to the memory at an offset from a0, then copies d1 to the memory at an offset from a0, then copies d2 to the memory at an offset from a0.
- `frag_dbra_rts`: Short code fragment that loops back while counting down d0, then returns.
- `copy_and_clear_buffer`: Copies a word from a pointer in a1 to a global variable, then copies up to 63 words from a1 to a global variable using a loop, and finally calculates an offset using a1 and a2 to call another routine.
- `clear_and_expand_buffer`: Clears a block of memory using a loop, then reads a word from a pointer in a1, calls another routine, and performs a second loop to clear a memory range based on a calculated length.
- `expand_bitstream_to_buffers`: Clears a memory buffer using a loop, then processes a bitstream by reading bits from a register to either skip or write bytes to both a destination buffer and a lookup table.
- `process_indexed_data`: Calls another routine, and if the carry flag is set, enters a loop to read bytes from a table and write them to both a destination buffer and another indexed location.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `frag_cmpa_bcc_moveq_move_rts`: Short code fragment that compares a1 with a4, then branches if the carry flag is clear, then loads 0 into d4, then copies the memory at a1 (advancing a1) to d4, then returns.
- `frag_move_rts_3`: Short code fragment that copies 1 to the condition codes, then returns.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `configure_blt_parameters_4`: Sets up bit block transfer parameters including control registers, source and destination addresses, and size based on input values and a lookup table, then performs multiple BLT operations with varying configurations.
- `execute_parameterized_blts`: Calculates several values including a bitmask and a destination address, calls another routine if a global byte is non-zero, and then performs a sequence of writes to custom hardware registers to trigger multiple hardware operations.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `configure_blt_hardware_parameters_2`: Calculates bitplane and address parameters from a data structure in a0, then writes these values to the Blitter control, address, and size registers.
- `configure_blt_parameters_2`: Calculates several values using bitwise operations and a table, then writes multiple words to a destination starting at a hardware register address, and performs this sequence multiple times based on a global byte check.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `configure_blt_transfer`: Calculates bitplane and dimension parameters from a data structure in a0, then writes values to the bitplane transfer hardware registers including size, mode, and source/destination pointers.
- `setup_blt_transfer`: Performs bitwise operations on registers, writes values to hardware registers including BLTCON0 and BLTSIZE, and sets the DMA mode and modification registers using values from a table.
- `frag_movea_neg_addi_mulu_adda_add_jmp`: Short code fragment that copies a location in the code to a0, then negates d1, then adds 200 to d1, then multiplies d1 by 200 (unsigned), then adds d1 to a0, then adds d2 to d2, then jumps to another point in the code.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `apply_bitmask_loop`: Calculates a bitmask based on the lower nibble of d0 and a pointer derived from d0, then loops eight times to apply this mask to a series of words at offsets 0, 20, 40, 60, and 80 from the pointer.
- `apply_bitmask_with_scaling`: Calculates a bitmask and a pointer from d0, then loops eight times to apply the mask to specific word offsets while scaling the mask and pointer values in each iteration.
- `apply_long_bitmask`: Calculates a bitmask from d0 and a pointer derived from d0, then applies the mask to a longword at the pointer and performs bitwise AND operations on four subsequent longwords at specific offsets.
- `execute_state_machine`: Decrements a global byte and, if it reaches zero, executes a sequence of up to four different routines stored in memory, using d0 to determine which to call and managing state via global bytes.
- `clear_structure_and_save_state`: Clears a global byte and four 16-bit words at offsets 6, 8, 10, and 12 from a pointer in a0, then conditionally saves two global bytes based on the sign of d0.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_rts_4`: Short code fragment that copies a0 to a fixed memory address, then copies 4 to a fixed memory address, then returns.
- `frag_move_tst_bpl_rts`: Short code fragment that copies 51142 to a fixed memory address, then tests a fixed memory address, then branches if the last result was not negative, then copies 4 to a fixed memory address, then returns.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies d0 to an indexed entry from a0, then returns.
- `frag_cmp_beq_blt_moveq_bge_bra`: Short code fragment that compares d0 with 28, then branches if the last result was zero, then branches if the comparison was less, then loads 15 into d0, then compares d1 with 144, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `frag_moveq_cmp_bge_bra`: Short code fragment that loads 14 into d0, then compares d1 with 144, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `frag_moveq_cmp_bge_bra_2`: Short code fragment that loads 16 into d0, then compares d1 with 144, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `frag_cmpa_bne_move_rts`: Short code fragment that compares a0 with 44804, then branches if the last result was not zero, then copies d0 to a fixed memory address, then returns.
- `frag_move_rts_5`: Short code fragment that copies d0 to a fixed memory address, then returns.
- `frag_move_rts_6`: Short code fragment that copies d0 to a fixed memory address, then returns.
- `frag_move_rts_7`: Short code fragment that copies d0 to a fixed memory address, then returns.
- `frag_cmpa_bne`: Short code fragment that compares a0 with 44804, then branches if the last result was not zero.
- `frag_clr_move_ext_rts`: Short code fragment that clears a fixed memory address, then copies d0 to a fixed memory address, then sign-extends d0, then copies a location in the code to a fixed memory address, then returns.
- `frag_clr_move_ext_rts_2`: Short code fragment that clears a fixed memory address, then copies d0 to a fixed memory address, then sign-extends d0, then copies a location in the code to a fixed memory address, then returns.
- `frag_movem_lea_bsr_rts`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a0, then loads the address of a location in the code into a1, then calls another routine, then restores saved registers from the stack, then returns.
- `multiply_and_store_offset`: Multiplies two 16-bit memory values, adds the second value, adjusts the result by its high and low halves, and stores the modified 16-bit result back into the first source location.
- `frag_move_bne_subq_rts`: Short code fragment that copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then branches if the last result was not zero, then subtracts 1 from a0, then returns.
- `frag_move_cmp_bne_tst_rts`: Short code fragment that copies the memory at a0 (advancing a0) to d0, then compares d0 with the memory at a1 (advancing a1), then branches if the last result was not zero, then tests d0, then branches if the last result was not zero, then returns.
- `frag_move_tst_bne_subq_suba_exg_rts`: Short code fragment that copies a0 to d0, then tests the memory at a0 (advancing a0), then branches if the last result was not zero, then subtracts 1 from a0, then subtracts d0 from a0, then exchanges d0 and a0, then returns.
- `perform_repeated_divisions`: Clears d0 and d1, then performs a sequence of four unsigned divisions by 10000, 1000, 100, and 10, calling another routine after each division before clearing a byte at a0.
- `frag_or_bne_swap_rts`: Short code fragment that merges d0 into d1, then branches if the last result was not zero, then swaps the halves of d0, then returns.
- `frag_addi_move_clr_swap_rts`: Short code fragment that adds 48 to d0, then copies d0 to the memory at a0 (advancing a0), then clears d0, then swaps the halves of d0, then returns.
- `calculate_offset_from_division`: Compares d0 to 20, and if greater, extends d0 to a longword, divides it by 10, swaps the high and low words, and then calculates a new address based on a table and the result.
- `frag_move_cmp_bcc`: Short code fragment that copies a location in the code to d0, then compares d0 with 127, then branches if the carry flag is clear, then copies 127 to a fixed memory address.
- `frag_move_cmp_bcc_ext_lea_beq_rts`: Short code fragment that copies a location in the code to d0, then compares d0 with 127, then branches if the carry flag is clear, then copies 127 to a fixed memory address, then sign-extends d0, then loads the address of a location in the code into a0, then copies an indexed entry from a0 to d0, then branches if the last result was zero, then returns.
- `frag_bsr_cmp_bcc_bne_rts`: Short code fragment that calls another routine, then compares d0 with 32, then branches if the carry flag is clear, then compares d0 with 13, then branches if the last result was not zero, then returns.
- `frag_tst_bmi_add`: Short code fragment that tests a fixed memory address, then branches if the last result was negative, then tests a fixed memory address, then branches if the last result was negative, then adds d0 to d0.
- `add_to_bcd_variable`: Adds a 32-bit value to a 4-byte global variable using byte-wise addition and BCD adjustment for each byte, unless the pointer in a0 equals a specific constant.
- `conditionally_add_to_bcd_variable`: Checks two global bytes for negative values, doubles d0, and then adds the result to a 4-byte global variable using byte-wise addition and BCD adjustment if a specific bit in d6 is set.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 2 to a7, then returns.
- `process_fragment_data`: Loads two tables into a5 and a6, pushes d0 to the stack, and if d0 is not zero, performs several pointer arithmetic operations using values from a5 and a table to call another routine.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_table_offset`: Loads a value from a global variable at offset 4 of a5, and if it is not zero, performs a division and bitwise check to either branch or add a value from a table to a0 and branch.
- `execute_blt_sequence`: Performs several calculations to derive addresses from a table, calls another routine multiple times to configure hardware or process data, and pushes values onto the stack before calling another routine.
- `frag_subq_bra`: Short code fragment that subtracts 8 from d0, then jumps to another point in the code.
- `configure_blt_parameters_3`: Pushes a calculated address onto the stack, pushes d0 and d1, calls another routine, and then restores d1, d0, and a0 from the stack before optionally calling another routine.
- `validate_blt_parameters`: Calculates dimensions and memory offsets based on a pointer in a0, performs bounds checking against provided width and height values, and otherwise returns immediately.
- `fill_memory_with_constant`: Loops 29 times to write the value -2 to a series of words starting at a specific address, then writes a constant to a global word and clears two global bytes.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (adda.w) and then falls through into the code that follows, with no return of its own.
- `frag_cmpa_bcc_tst_bpl_rts`: Short code fragment that compares a4 with an address, then branches if the carry flag is clear, then tests the memory at a4, then branches if the last result was not negative, then returns.
- `frag_ori_rts`: Short code fragment that merges 1 into the condition codes, then returns.
- `frag_suba_cmpa_bcs_tst_bpl_rts`: Short code fragment that subtracts 16 from a4, then compares a4 with an address, then branches if the carry flag is set, then tests the memory at a4, then branches if the last result was not negative, then returns.
- `update_entity_counters_and_indices`: Clears a table of 29 double-words, then iterates through an entity list to increment byte counters at specific offsets within a secondary table.
- `dispatch_table_entry`: Copies a word from the stack to a register, then uses it as an offset to a table to load a pointer and calls the routine at that address if the offset is non-negative.
- `frag_move_ext_bmi_bsr_bra`: Short code fragment that copies a0 to the top of the stack, then sign-extends d0, then branches if the last result was negative, then calls another routine, then jumps to another point in the code.
- `tiny_addq_rts_2`: Small routine of 2 instructions that adds 4 to a7, then returns.
- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `frag_btst_andi_move_addi_subq_jmp`: Short code fragment that tests bit d0 of d0 (2 times in a row), then masks the memory at a0 with 592, then tests bit d2 of d0 (2 times in a row), then copies d0 to d0, then adds 160 to the memory at a0 (advancing a0), then subtracts 2 from the memory at a0, then copies the memory at a0 to d0, then jumps to another point in the code.
- `update_global_value`: Subtracts a word from a global variable, calls another routine, and then updates a value at an offset of 4 from a pointer in a2 if it is zero.
- `process_callback_ranges`: Reads a word from a memory location, performs checks on a byte derived from it, and calls two different routines with specific addresses in a0 depending on the results.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `compare_and_add_values`: Compares a word at an offset of a6 with values from a table, adding values to d5 and d6 if certain comparison conditions are met.
- `calculate_scaled_value`: Performs arithmetic using values from a structure in a6 and a comparison against a constant, then calls another routine and branches to another routine.
- `execute_callback_list`: Checks a value at offset 18 of a0, then iterates through a table of 40 entries, calling a routine for each entry if its value is within a specific range.
- `execute_callback_loop`: Iterates through a list of pointers in a structure, calling a routine for each entry if the entry's value is within a specific range.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `frag_bra_11`: Short code fragment that jumps to another point in the code.
- `compare_table_values`: Calculates an address using a pointer in a5 and an index in d4, then compares values at that address against d5 and d6, updating d5 or d6 if certain conditions are met.
- `write_data_to_memory`: Loads values from a table using d4 as an index, calls another routine, and then writes a byte and a word to a memory location pointed to by a6.
- `frag_move_rts_8`: Short code fragment that copies 65532 to the memory at a6, then returns.
- `clamp_arithmetic_results`: Performs several subtractions and additions using values from a table and registers, updating values at a pointer in a5 if they fall within specific bounds.
- `update_clamped_values`: Adds values to two consecutive memory locations while ensuring the results do not fall below zero or exceed a specific limit.
- `validate_range_constraints`: Loops through a table to compare differences between values in a structure and a global table, calling another routine if certain bounds are exceeded.
- `frag_st_rts`: Short code fragment that sets the memory at an offset from a0 to all ones, then returns.
- `frag_lsl_movem_move_jsr_adda_rts`: Short code fragment that shifts d0 left by 2, then copies several registers to or from memory, then copies d3 to the memory at an offset from a4, then calls another routine, then adds 16 to a4, then returns.
- `update_structure_data`: Sets a bit in a byte at a0, then enters a loop that calls another routine and updates several fields in a structure pointed to by a4 based on the results.
- `update_structure_fields`: Calls another routine and then updates several fields in a structure pointed to by a4 using bitwise operations and arithmetic.
- `frag_move_bra`: Short code fragment that copies 120 to d2, then copies 177 to d5, then jumps to another point in the code.
- `frag_move_subq_moveq_bsr_bra`: Short code fragment that copies d5 to d3, then subtracts 3 from d3, then loads 2 into d7, then calls another routine, then jumps to another point in the code.
- `process_state_sequence`: Executes a loop that calls another routine multiple times, updating a structure at a4 with different values and bit-shifted results based on the return values.
