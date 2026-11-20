# Game 457 (James Pond Iii - Operation Starfish): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00457). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703360 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 129 of 129 named (0 author symbols), 129 of 129 described (76 by rule, 51 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1492 of 1492 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3877 |
| code | 4924 |
| data | 2124391 |
| padding | 824 |
| unused | 569344 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 1 of 3).adf:bootblock | 1 | 50 | 2 |
| logged disk load from James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 1 of 3).adf boot-block load | 1 | 1154 | 120 |
| logged disk load from James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 1 of 3).adf boot-block load | 1 | 288 | 7 |

## Cases handled

- james pond iii - operation starfish (1994)(millennium)(aga)[cr dlm](disk 1 of 3)/James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 1 of 3).adf: custom-format (non-DOS) disk, 1493 non-empty blocks: matched against track-loaded segments
- james pond iii - operation starfish (1994)(millennium)(aga)[cr dlm](disk 2 of 3)/James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 2 of 3).adf: custom-format (non-DOS) disk, 1412 non-empty blocks: matched against track-loaded segments
- james pond iii - operation starfish (1994)(millennium)(aga)[cr dlm](disk 3 of 3)/James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 3 of 3).adf: custom-format (non-DOS) disk, 1257 non-empty blocks: matched against track-loaded segments
- james pond iii - operation starfish (1994)(millennium)(aga)[cr dlm](disk 1 of 3)/James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 1 of 3).adf: the boot block reads 4608 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- james pond iii - operation starfish (1994)(millennium)(aga)[cr dlm](disk 1 of 3)/James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 1 of 3).adf: the boot block reads 4608 bytes from disk offset 6144 to an address (found statically); recovered as a unit
- james pond iii - operation starfish (1994)(millennium)(aga)[cr dlm](disk 1 of 3)/James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 1 of 3).adf: 2 logged loads cover 9216 bytes of the image
- james pond iii - operation starfish (1994)(millennium)(aga)[cr dlm](disk 1 of 3)/James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 1 of 3).adf: blocks 11-11 (bytes 5632-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.04)
- james pond iii - operation starfish (1994)(millennium)(aga)[cr dlm](disk 2 of 3)/James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 2 of 3).adf: blocks 2-16 (bytes 1024-8704) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.95)
- james pond iii - operation starfish (1994)(millennium)(aga)[cr dlm](disk 3 of 3)/James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 3 of 3).adf: blocks 2-15 (bytes 1024-8192) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.02)
- 3 packed track region(s), 2106368 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 1 of 3).adf:bootblock

- `allocate_dma_buffer`: Allocates a 5120-byte memory block with type 3, sets up an I/O request structure with specific command and address values, and performs I/O operations using the allocated buffer.
- `allocate_memory_block`: Allocates a memory block of 5120 bytes with type 3, stores the result in a global variable, and if allocation succeeds, sets up an I/O request structure with command 2, then executes the I/O operation, followed by command 9, and finally calls the allocated memory block as a function before freeing it.

### logged disk load from James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 1 of 3).adf boot-block load

- `frag_bra_bsr_pea_jmp`: Short code fragment that jumps to another point in the code, then calls another routine (2 times in a row), then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `initialize_system_components`: Initializes system components by loading addresses into registers, calling initialization routines, and performing conditional jumps to specific memory locations.
- `frag_lea_bra`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a location in the code into a1, then jumps to another point in the code.
- `frag_cmp_beq_move_bpl_lea_bra`: Short code fragment that compares d0 with the memory at an offset from a0, then branches if the last result was zero, then copies the memory at a1 (advancing a1) to d0, then branches if the last result was not negative, then loads the address of a location in the code into a1, then jumps to another point in the code.
- `process_character_data`: Compares data from two memory locations, copies words from one location to another in a loop, and when certain conditions are met, clears a large memory block, sets up custom hardware registers for coprocessor operations, and processes character data by converting characters to indices and copying corresponding values to memory locations.
- `initialize_hardware_registers`: Initializes several hardware registers including CIAA_PRA, COLOR00, DMACON, INTENA, and custom+0 with specific values, enables DMA channels, sets up a jump vector for a coprocessor, and configures interrupt enable bits.
- `frag_lea_move_jmp`: Short code fragment that loads the address of a fixed memory address into a7, then copies 9984 to the status register, then loads the address of a fixed memory address into a0, then copies 20083 to the memory at a0, then copies a0 to a fixed memory address, then jumps to the address in a5.
- `frag_lea_bsr_rts`: Short code fragment that loads the address of a location in the code into a0, then calls another routine (4 times in a row), then returns.
- `setup_task_parameters`: Checks supervisor mode flags, sets up task parameters in memory, and calls initialization routines based on flag states.
- `check_supervisor_mode`: Checks if the supervisor flag is set in the system status register, clears a memory location if the flag is not set, and otherwise calls a supervisor routine to handle the operation.
- `handle_fpu_state`: Writes a specific value to memory, saves the FPU state, reads a byte from the stack, compares it with 24, and if they are equal, writes another value to memory; otherwise, it writes a different value, then restores the FPU state and returns.
- `modify_trap_code`: Finds the current task using a system call, modifies the trap code field of that task to point to another routine, restores the original trap code value, and stores -1 in a specific memory location.
- `frag_moveq_move_cmp_beq_addq_rte`: Short code fragment that loads 0 into d0, then copies the top of the stack to d1, then compares d1 with 11, then branches if the last result was zero, then copies 68851 to d0, then adds 4 to the memory at an offset from a7, then returns from the exception.
- `read_video_registers`: Reads from a video position register, extracts specific bits, stores the result in memory, disables interrupts, reads another hardware register, processes its value, and stores the processed result in memory before re-enabling interrupts.
- `frag_movea_move_clr_rts`: Short code fragment that copies a fixed memory address to a6, then copies the memory at an offset from a6 to d0, then clears d0, then copies d0 to a1, then copies the memory at an offset from a1 to the memory at an offset from a0 (2 times in a row), then copies the memory at an offset from a6 to the memory at an offset from a0 (2 times in a row), then returns.
- `calculate_memory_ranges`: Clears several memory locations, calculates memory ranges based on system values, and performs a loop to traverse a memory list, checking conditions on each node and updating memory range values accordingly.
- `frag_lea_bra_2`: Short code fragment that loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then jumps to another point in the code.
- `initialize_coprocessor`: Clears a large memory block with zeros, sets up coprocessor registers with specific values including jump addresses and DMA control settings, and processes character data from a table to copy bytes to memory locations.
- `frag_cmp_bne_subi_bra`: Short code fragment that compares d0 with 32, then branches if the last result was not zero, then subtracts 32 from d0, then jumps to another point in the code.
- `frag_cmp_blt_bgt_subi_bra`: Short code fragment that compares d0 with 48, then branches if the comparison was less, then compares d0 with 57, then branches if the comparison was greater, then subtracts 47 from d0, then jumps to another point in the code.
- `convert_and_copy_characters`: Compares character values with specific bounds, adjusts character indices, calculates an address based on the index, and copies multiple bytes from a calculated location to another location in memory while handling string termination.
- `frag_lea_adda_movea_moveq_move_tst`: Short code fragment that loads the address of the memory at an offset from a0 into a4, then adds the memory at a0 to a0, then copies a1 to a3, then loads 3 into d6, then loads 1 into d4, then loads 7 into d7, then loads 1 into d5, then copies a3 to a2, then copies the memory just below a0 (moving a0 down) to d1, then tests d1.
- `copy_and_shift_data`: Performs bit shifting operations on data values, calculates memory addresses based on shift results, copies data from tables while performing conditional operations, and stores shifted values to a destination buffer.
- `frag_lsr_beq_rts`: Short code fragment that shifts d5 right by d4, then branches if the last result was zero, then returns.
- `frag_move_roxr_rts`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (sub.w) and then falls through into the code that follows, with no return of its own.
- `frag_moveq_lsr_beq_roxl_dbra_rts`: Short code fragment that loads 0 into d1, then shifts d5 right by d4, then branches if the last result was zero, then rotates d1 left through the extend flag by d4, then loops back while counting down d0, then returns.
- `frag_move_roxr_bra`: Short code fragment that copies the memory just below a0 (moving a0 down) to d5, then rotates d5 right through the extend flag by d4, then jumps to another point in the code.
- `copy_and_shift`: Copies data from a table while shifting values and performs conditional operations based on comparisons with global variables, then stores the results in a destination buffer.
- `frag_bsr_move_add_dbra_cmpa_bcs_rts`: Short code fragment that calls another routine, then copies d1 to d3, then adds d4 to d2, then copies an indexed entry from a3 to the memory just below a3 (moving a3 down), then loops back while counting down d2, then compares a2 with a3, then branches if the carry flag is set, then returns.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_movem_bsr_lea_rts`: Short code fragment that saves registers on the stack, then calls another routine (4 times in a row), then restores saved registers from the stack, then loads the address of a location in the code into a0, then returns.
- `copy_reverse_block`: Copies data in reverse order from one memory location to another using a loop that decrements a counter, and returns after completing the copy.
- `frag_subq_move_dbra_rts`: Short code fragment that subtracts 1 from d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `copy_and_shift_data_2`: Performs bit shifting operations on data values, calculates memory addresses based on shift results, copies data from tables while performing shifts, and updates control bytes.
- `frag_subq_bmi_move_dbra_rts`: Short code fragment that subtracts 1 from d2, then branches if the last result was negative, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d2, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_game_state`: Initializes game state variables including a counter and status flags, updates values through multiple subroutine calls, and manages conditional logic based on counter limits before returning.
- `update_animation_state`: Updates an animation state byte at offset 19 from a5, increments it up to 159, and calls another routine before returning.
- `decrement_counters`: Decrements a counter at offset 8 of a data structure, and if the counter becomes zero or negative, resets it to 50 and decrements another counter at offset 21 of the same structure.
- `configure_dma_channel`: Clears a flag byte, reads a system configuration value, sets specific bits in a hardware register based on the configuration, calls a system configuration routine, and performs bit clearing operations to control DMA channel settings.
- `setup_dma_interrupts`: Clears a flag byte, clears a specific bit in a system register, sets bits in a hardware register based on a configuration value, calls a system configuration routine multiple times, and performs bit setting operations to configure interrupt handling.
- `frag_bsr_move_rts`: Short code fragment that calls another routine, then copies 5 to the memory at an offset from a5, then returns.
- `frag_moveq_move_bset_bsr_btst_bne_bra`: Short code fragment that loads 0 into d7, then copies the memory at an offset from a5 to d7, then sets bit d7 of the memory at an offset from a4, then calls another routine, then tests bit 4 of the memory at an offset from a5, then branches if the last result was not zero, then sets bit 7 of the memory at an offset from a4, then jumps to another point in the code.
- `frag_bclr_bsr_rts`: Short code fragment that clears bit 7 of the memory at an offset from a4, then calls another routine, then clears bit d7 of the memory at an offset from a4, then returns.
- `frag_moveq_move_bset_bsr_rts`: Short code fragment that loads 0 into d7, then copies the memory at an offset from a5 to d7, then sets bit d7 of the memory at an offset from a4, then calls another routine, then returns.
- `frag_btst_beq_bsr_bra`: Short code fragment that tests bit 4 of the memory at an offset from a4, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a5, then returns.
- `enable_dma_channel_1`: Sets bit 1 in a hardware register, clears bit 0, sets bit 0, and calls a system configuration routine to complete the DMA channel setup process.
- `disable_dma_channels`: Clears bit 1 and bit 0 in a hardware register, sets bit 0, and calls a system configuration routine to complete the DMA channel disable process.
- `update_and_process_values`: Updates byte values in memory by shifting and subtracting, performs conditional operations based on results, and manages a loop with decrementing counter.
- `frag_ext_subq_bsr_dbra_andi_beq_bclr_rt`: Short code fragment that sign-extends d7, then subtracts 1 from d7, then calls another routine, then loops back while counting down d7, then masks d5 with 1, then branches if the last result was zero, then clears bit 2 of the memory at an offset from a4, then calls another routine, then returns.
- `frag_bset_bsr_rts`: Short code fragment that sets bit 2 of the memory at an offset from a4, then calls another routine, then returns.
- `initialize_table_entries`: Fills a table with zero values using a loop that decrements a counter, then stores specific byte values at fixed offsets from the table start, and finally copies two longword values from registers to the table.
- `copy_string_with_limit`: Copies bytes from one memory location to another until a null terminator is encountered or a counter reaches zero, returning the value from a register.
- `frag_movem_movea_move_moveq_dbra_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a0, then copies 1407 to d0, then loads 0 into d1, then copies d1 to the memory at a0 (advancing a0), then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `setup_hardware_registers`: Writes specific values to several hardware registers including CIA port directions and custom chip control registers to configure interrupt and DMA settings.
- `frag_move_lea_moveq_bra`: Short code fragment that copies 255 to the memory at an offset from a4, then loads the address of a location in the code into a5, then loads 3 into d0, then jumps to another point in the code.
- `configure_dma_channel_2`: Clears a flag byte, reads a system configuration value, sets specific bits in a hardware register based on the configuration, and calls a system routine.
- `frag_bset_bsr_lea_dbra_move_rts`: Short code fragment that sets bit 7 of the memory at an offset from a5, then calls another routine (3 times in a row), then loads the address of the memory at an offset from a5 into a5, then loops back while counting down d0, then copies 255 to the memory at an offset from a4, then returns.
- `frag_movea_lea_moveq_move_bra`: Short code fragment that copies a location in the code to a1, then loads the address of the memory at an offset from a1 into a1, then loads 0 into d1, then copies 350 to d7, then jumps to another point in the code.
- `process_data_loop`: Processes a sequence of bytes from memory, calculates a sum, and stores a result based on comparison with a fixed value.
- `frag_move_bra`: Short code fragment that copies 12 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies 13 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_movea_lea_moveq_move_bra_2`: Short code fragment that copies a location in the code to a1, then loads the address of the memory at an offset from a1 into a1, then loads 0 into d0, then copies 350 to d7, then jumps to another point in the code.
- `decode_and_accumulate`: Reads a sequence of bytes from a buffer, decodes a value using bit manipulation, accumulates it into a running total, and repeats this process while decrementing a counter until it reaches zero.
- `frag_bsr_move_movea_rts`: Short code fragment that calls another routine, then copies 1 to the memory at an offset from a5, then calls another routine, then copies a location in the code to a0, then calls another routine (3 times in a row), then returns.
- `setup_hardware_registers_2`: Writes specific values to several hardware registers including CIA port directions and custom chip control registers to configure interrupt settings.
- `frag_cmpi_bne_move_lea_bra`: Short code fragment that compares the memory at a0 with 1, then branches if the last result was not zero, then copies a1 to the stack, then loads the address of the memory at an offset from a0 into a1, then jumps to another point in the code.
- `check_and_loop`: Reads a byte from a global variable at offset 20 of a structure in a5, branches to exit if zero, otherwise reads a longword from a pointer in a1 and continues looping while the value is non-zero.
- `check_flags_and_loop`: Checks specific bit flags in memory, performs a loop operation while decrementing a counter, and sets a status byte upon completion.
- `copy_string_with_counter`: Copies bytes from one memory location to another until a null terminator is encountered or a counter reaches zero, returning when the counter reaches zero.
- `compare_and_loop`: Compares bytes from two memory locations and continues looping while they match, decrementing a counter each iteration until it reaches zero or a mismatch occurs.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `configure_system_flags_2`: Sets specific bits in system registers to configure hardware settings, waits for a condition to be met using a polling loop, then restores the previous state.
- `configure_hardware_settings`: Configures hardware registers with specific values, performs bit manipulation on control bytes, and manages state transitions through branching.
- `process_dma_status`: Checks DMA status flags and updates control bytes in a global structure based on conditional branches, then clears a counter and modifies bits in a status byte before looping back to process the next entry.
- `frag_bsr_clr_andi_ori_move_bra`: Short code fragment that calls another routine, then clears the memory at a5, then masks the memory at an offset from a5 with 223, then merges 64 into the memory at an offset from a5, then copies 4 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_clr_andi_ori_lea_dbra_movem_rts`: Short code fragment that clears the memory at a5, then masks the memory at an offset from a5 with 191, then merges 32 into the memory at an offset from a5, then loads the address of the memory at an offset from a5 into a5, then loops back while counting down d0, then restores saved registers from the stack, then returns.
- `reset_dma_flag`: Clears a bit in a global variable, reads a status byte, and if a condition is met, sets another bit in the same global variable and writes a value to a different global variable.
- `configure_system_flags`: Sets specific bits in system registers to configure hardware settings, waits for a condition to be met using a polling loop, then restores the processor status register.
- `frag_movea_moveq_move_add_bcc_addq_dbra`: Short code fragment that copies a0 to a1, then loads 0 into d0, then copies 1407 to d1, then adds the memory at a1 (advancing a1) to d0, then branches if the carry flag is clear, then adds 1 to d0, then loops back while counting down d1, then returns.
- `frag_bsr_rts`: Short code fragment that calls another routine (4 times in a row), then returns.
- `frag_movea_move_dbra_lea_rts`: Short code fragment that copies a location in the code to a3, then copies an address to d6, then copies 3199 to d7, then copies d6 to the memory at a3 (advancing a3), then loops back while counting down d7, then copies a location in the code to a3, then loads the address of the memory at an offset from a3 into a3, then copies an address to the memory at a3 (advancing a3), then returns.
- `frag_moveq_move_bsr_rts`: Short code fragment that loads 0 into d0, then copies 256 to d0, then copies the memory at an offset from a5 to d0, then calls another routine, then returns.
- `frag_bsr_rts_2`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_move_bsr_dbra_rts`: Short code fragment that copies 1407 to d4, then copies the memory at a0 (advancing a0) to d0, then calls another routine, then loops back while counting down d4, then calls another routine, then returns.
- `frag_move_moveq_lsr_bsr`: Short code fragment that copies an address to d7, then loads 31 into d6, then loads 0 into d5, then copies d0 to d3, then shifts d0 right by 1, then calls another routine, then copies d3 to d0.
- `bit_manipulate_and_store`: Performs bitwise operations on data values including AND, OR, EOR and bit set/clear operations, tests a bit in memory, and stores the result to a memory location.
- `frag_move_btst_bne_bset_bra`: Short code fragment that copies the memory at a3 to d0, then tests bit 0 of the memory at an offset from a3, then branches if the last result was not zero, then tests bit 6 of d0, then branches if the last result was not zero, then sets bit 7 of d0, then jumps to another point in the code.
- `frag_bclr_move_rts`: Short code fragment that clears bit 7 of d0, then copies d0 to the memory at a3, then returns.
- `system_configuration_check`: Checks system flags and configures hardware settings based on flag states, using a loop to wait for conditions before setting status bytes.
- `frag_move_bra_3`: Short code fragment that copies 1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bsr_dbra_tst_bmi_rts`: Short code fragment that copies 2 to the memory at an offset from a5, then calls another routine, then loops back while counting down d7, then copies 16384 to the memory at an offset from a6, then copies 4098 to the memory at an offset from a6, then tests d7, then branches if the last result was negative, then copies 0 to the memory at an offset from a5, then returns.
- `frag_moveq_bsr_tst_bne_movea_rts`: Short code fragment that loads 3 into d3, then calls another routine, then tests the memory at an offset from a5, then branches if the last result was not zero, then calls another routine, then tests the memory at an offset from a5, then branches if the last result was not zero, then copies a1 to a0, then loads 0 into d0, then returns.
- `frag_movem_bsr_dbra_moveq_rts`: Short code fragment that saves registers on the stack, then calls another routine (2 times in a row), then restores saved registers from the stack, then loops back while counting down d3, then loads 255 into d0, then returns.
- `copy_memory_block_with_loop`: Executes a loop that calls another routine, checks a condition, increments a counter, and when the counter reaches a limit, copies a block of memory from one location to another.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `validate_data_sequence`: Compares data values from memory locations, performs bitwise operations, and sets status bytes based on comparison results.
- `frag_move_bra_4`: Short code fragment that copies 14 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_5`: Short code fragment that copies 8 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_bra_6`: Short code fragment that copies 6 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_moveq_tst_bmi_rts`: Short code fragment that copies 7 to the memory at an offset from a5, then loads 255 into d0, then tests d0, then branches if the last result was negative, then copies 0 to the memory at an offset from a5, then returns.
- `frag_move_movem_and_add_or_rts`: Short code fragment that copies an address to d7, then copies several registers to or from memory, then masks d5 with d7, then masks d6 with d7, then adds d5 to d5, then merges d5 into d6, then returns.
- `configure_dma_and_wait`: Configures DMA settings for a hardware channel, waits for specific interrupt conditions, and updates a status byte in a structure.
- `initialize_dma_and_wait`: Initializes DMA channel configuration and sets up interrupt handling, then waits for a condition before clearing a flag byte and returning.
- `frag_movem_moveq_bsr_rts`: Short code fragment that saves registers on the stack, then loads 40 into d1, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_move_bsr_rts`: Short code fragment that saves registers on the stack, then copies 1024 to d1, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_moveq_lsr_subq_move_eor_dbra_andi`: Short code fragment that loads 0 into d0, then shifts d1 right by 2, then subtracts 1 from d1, then copies the memory at a2 (advancing a2) to d2, then exclusive-ors d2 into d0, then loops back while counting down d1, then masks d0 with an address, then returns.
- `process_data_pairs`: Processes pairs of longword values by performing bitwise AND operations with a constant, doubling one value, combining them with OR operation, and storing the result in memory.

### logged disk load from James Pond III - Operation Starfish (1994)(Millennium)(AGA)[cr DLM](Disk 1 of 3).adf boot-block load

- `process_relocation_data`: Reads relocation information from a data structure and applies address adjustments to referenced locations.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `frag_movem_addq_movea_bsr_rts`: Short code fragment that saves registers on the stack, then adds 6 to a0, then copies several registers to or from memory, then copies a0 to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `decode_bitfields`: Processes bitfield data from a buffer, performs shifts and rotations on word values, and copies decoded bytes to an output buffer while handling various conditional branches based on comparison results.
- `select_half_or_input_word`: Returns half of a second word when a flag equals one, otherwise returns the input word unchanged.
- `extract_masked_bitfield_word`: Copies d6 to d0, shifts d6 right by d1, subtracts d1 from d7, rotates d6 by d7 and refills from -(a2), then masks d0 with a table word indexed by doubled d1.
- `enable_cache`: Enables cache operations by setting specific bits in the status register, writes cache control values to cache control registers, and returns from the routine.
