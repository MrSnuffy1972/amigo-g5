# Game 528 (Lion King, the): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00528). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3613216 top-level bytes in 29 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 19 of 19 units identical (bytes and relocations), 72 raw-word lines |
| Functions named and described | pass, 331 of 331 named (0 author symbols), 331 of 331 described (26 by rule, 301 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1099 of 1100 (99.91%) |
| Recovered without load address (counted in proof 1, flagged) | 3301234 bytes, 162 functions |
| Independent decoder cross-check (Ghidra against capstone) | 11566 of 15639 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 756006 |
| code | 60968 |
| data | 2107879 |
| padding | 4843 |
| unused | 683520 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Type | 1 | 426 | 2 |
| 1 | 2 | 3 | 1 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:LKATRO.EXE | 8 | 45 | 2 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:lionking | 4 | 1693 | 124 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:bootblock | 1 | 10 | 1 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 2 of 4).adf:bootblock | 1 | 54 | 1 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 3 of 4).adf:bootblock | 1 | 54 | 1 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 4 of 4).adf:bootblock | 1 | 54 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 548 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 462 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:LION (raw code file, load address unknown) | 1 | 12285 | 162 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:LEVEL1#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 2 of 4).adf:LEVEL3#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 2 of 4).adf:LEVEL4#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 2 of 4).adf:LEVEL5#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 4 of 4).adf:LEVEL8#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |
| Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 4 of 4).adf:LEVELA#unpacked (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- lion king, the (1994)(virgin)(aga)[cr cls](disk 1 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:LEVEL1: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (448788 -> 818052 bytes)
- lion king, the (1994)(virgin)(aga)[cr cls](disk 2 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 2 of 4).adf:LEVEL3: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (293182 -> 528216 bytes)
- lion king, the (1994)(virgin)(aga)[cr cls](disk 2 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 2 of 4).adf:LEVEL4: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (126344 -> 218380 bytes)
- lion king, the (1994)(virgin)(aga)[cr cls](disk 2 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 2 of 4).adf:LEVEL5: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (138661 -> 260506 bytes)
- lion king, the (1994)(virgin)(aga)[cr cls](disk 4 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 4 of 4).adf:LEVEL8: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (298871 -> 555238 bytes)
- lion king, the (1994)(virgin)(aga)[cr cls](disk 4 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 4 of 4).adf:LEVELA: RNC1: Rob Northen RNC1 Compressor-packed, unpacked with ancient (337825 -> 620386 bytes)
- runtime segment an address: 495 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 345 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- lion king, the (1994)(virgin)(aga)[cr cls](disk 1 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:LION: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lion king, the (1994)(virgin)(aga)[cr cls](disk 1 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:LEVEL1#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lion king, the (1994)(virgin)(aga)[cr cls](disk 2 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 2 of 4).adf:LEVEL3#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lion king, the (1994)(virgin)(aga)[cr cls](disk 2 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 2 of 4).adf:LEVEL4#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lion king, the (1994)(virgin)(aga)[cr cls](disk 2 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 2 of 4).adf:LEVEL5#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lion king, the (1994)(virgin)(aga)[cr cls](disk 4 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 4 of 4).adf:LEVEL8#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- lion king, the (1994)(virgin)(aga)[cr cls](disk 4 of 4)/Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 4 of 4).adf:LEVELA#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 6 never-loaded region(s), 3000778 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `clear_cia_port_bit` (lionking): Derives a bit index from a register and applies it to a global variable, clearing the associated CIA port bit. Kind: disk hardware/timing check. Pass path: the code continues to the dbra loop (sub_3_0A6A) which likely processes the loaded data or proceeds to the next track.
- `configure_disk_dma_transfer` (lionking): Initializes DMA control registers and disk path/length parameters, then loops to poll a status flag while managing a counter. Kind: track/disk validation loop. Pass path: the loop continues (sVar2 decrementing) if the check returns false, or breaks if the check returns true/fails.
- `configure_dma_registers` (lionking): It writes specific values to hardware registers associated with DMA control, including ADKCON, DMACON, DSKLEN, DSKPTH, and DSKSYNC. It enables DMA channels by setting bit 15 in the DMACON register. Kind: disk hardware/track check. Pass path: bne.b an address.
- `write_adkcon_and_test` (lionking): Writes the value 1024 to the ADKCON hardware register and then tests a 16-bit word at offset -30 of the structure in a6, branching if the result is positive. Kind: disk hardware/track validation. Pass path: beq.b an address.
- `set_bit_in_global_flag` (lionking): It reads a 16-bit value from a table pointed to by a register, adds a constant, and uses the result to set and then clear a bit in a global variable via the CIAB_PRB register. Kind: disk hardware/track check. Pass path: bra.w an address.
- `set_cia_and_wait` (lionking): Writes a constant to a hardware register, clears bit 7 of d1, calls another routine, and then loads a constant into d0 before branching. Kind: disk hardware/track check. Pass path: the loop continues (iVar1 != 1).
- `modify_flag_and_wait` (lionking): Saves d0 to the stack, calls another routine, and if d1 is not negative, clears bits 0 and 1 of d0, writes d0 to a hardware register, sets bit 0 of d0, writes it to the register again, restores d0, and branches. Kind: disk hardware/sync check. Pass path: iVar1 == 1.
- `store_masked_byte` (lionking): Calls another routine and stores the resulting byte into a global variable at offset 0 of the structure at address bf d100. Kind: disk hardware/checksum trap. Pass path: the routine continues execution normally (the call is a side-effect check).
- `mask_input_byte` (lionking): It builds a mask from a global variable and a byte taken from a structure, clears one bit in that mask based on a byte from the structure, and optionally clears another bit if a flag in a table indexed by the structure is set. Kind: bitmask/checksum trap. Pass path: the branch where the bitmask is applied normally without triggering a failure state.

## Functions


### Type

- `parse_type_command_arguments`: Parses command-line arguments to identify hex or number flags, validates that they are not used simultaneously, and handles error reporting for invalid options or file access failures.
- `format_and_print_string`: Iterates through a loop to format a string and numeric values into a local buffer using a hexadecimal character table, then writes the resulting string to a destination via a library call.

### 1

- `wait_for_ciaa_bit`: Polls bit 6 of the CIAA Port A register until it is cleared, then returns.

### Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:LKATRO.EXE

- `run_hdinstall_entry`: Runs a fixed sequence of helper calls and a loop that calls FreeMem three times, clears a word in a shared record, and swaps one table entry with a RAM variable when it matches a fixed constant.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:lionking

- `run_game_entry`: Calls apply_relocation_hunks and push_stack_and_trap, then returns zero.
- `apply_relocation_hunks`: Parses a data structure containing hunk type codes to perform memory relocations and data patching.
- `decrement_and_loop`: Decrements a value in register d1 and loops until it reaches zero, while also performing pointer arithmetic and calling a routine.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_bit_shifts_2`: Loads a pointer into a5, sets several registers, and moves a long value from a0 into d1; if the byte in d1 is non-zero, it calls another routine, performs arithmetic and shifts on d1, and then enters a loop that calls another routine to perform arithmetic and moves bytes from a table into a destination pointed to by a3.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bit_shifts`: Performs a series of bitwise shifts and rotations on registers d4 and d5 within a loop, and moves bytes from a table into a destination pointed to by a3.
- `execute_trap_sequence`: Moves a word from an offset of a2 into d4, performs a bitwise OR on d1, calls a trap, and then performs a bitwise OR on d1 with d0.
- `push_stack_and_trap`: Pushes the address of the next instruction onto the stack, pops a 32-bit value from the stack into a hardware register, and executes a software interrupt.
- `copy_data_with_interrupt_control`: Sets the interrupt enable, color, and DMA control registers, then enters a loop that copies bytes from a source to a destination using a counter in d0, and finally jumps to the destination.
- `setup_dma_and_call`: Sets the color and DMA control registers, loads several addresses into registers, calls another routine, and then jumps to a location in a1.
- `save_registers_and_branch`: Saves registers d0-d7 and a0-a6 to the stack, moves d0 and d1 to d6 and d7, moves a0-a2 to a4-a6, calls another routine, and then either calls another routine or moves d1 to an offset of 4 from the stack pointer before jumping.
- `branch_on_byte_match_2`: Compares the byte in d6 with 1, and if equal, calls another routine, otherwise it performs a jump to another location.
- `branch_on_byte_match`: Compares the byte in d6 with 2 and, if equal, calls another routine and otherwise performs a jump to another location.
- `compare_and_copy_pointer`: Compares the byte in d6 with 3 and, if equal, calls another routine and stores the value of d1 into a global word at offset 4 of the stack.
- `check_byte_and_call`: Compares the byte in d6 with 4 and, if equal, calls another routine and branches to another location.
- `compare_byte_and_branch_2`: Compares the byte in d6 with 5 and, if equal, calls another routine, saves d0 and d1 to the stack, and branches to another location.
- `compare_byte_and_branch`: Compares a byte in d6 with 6 and, if not equal, branches to another location; otherwise, it calls another routine and branches to a third location.
- `compare_byte_and_save_registers`: Compares a byte in register d6 with seven; if not equal, it calls another routine, saves registers d0 and d1 to the stack, and branches to another location, otherwise it continues to another routine.
- `compare_and_store_result`: Compares a byte in register d6 with eight; if not equal, it calls another routine, stores the result from d0 onto the stack, and branches elsewhere; otherwise, it returns.
- `init_disk_transfer`: Saves d0 to the stack, clears d1 and d2, loads a word into d3, calls another routine, restores the registers, and returns d0.
- `check_pointer_validity`: Loads a value into d5, calculates an address relative to the program counter, and performs a conditional check to either return or continue based on the value at that address.
- `initialize_structure_from_pointer`: Calls another routine and, if the zero flag is not set, populates a structure at a3 with a pointer from a5, a zeroed word, a word containing negative one, a zeroed long, and a long from offset 324 of a structure in a0.
- `clear_a4_offset`: Calls another routine and then clears the long value at offset 10 of the structure pointed to by a4.
- `store_min_value_to_offset`: Calls another routine, compares the long value at offset 14 of the structure in a4 with d1, and stores the minimum of these two values at offset 6 of the structure in a4.
- `update_minimum_delta_and_counters`: Calculates the difference between two values in a structure, updates a global minimum value if the difference is smaller, and performs conditional increments and resets on structure fields based on comparisons with a scaled value.
- `update_structure_counters`: Increments a counter within a structure and performs conditional checks against other fields, calculates a value based on register inputs, updates memory locations via pointers, and handles alignment or masking operations before returning a combined result.
- `calculate_scaled_value`: Performs a series of arithmetic operations including multiplication and division on a value in d0, and returns the result in d2 masked to 16 bits.
- `compute_price_index_from_a4`: Calculates a 16-bit value by multiplying a value derived from a4 by 18, checks a global variable at that index, and returns the index or zero.
- `clear_and_load_registers`: Calls another routine and, if the zero flag is not set, loads a long from offset 144 of a structure in a0 into d7, clears a long at offset 1fc of a structure in a6, and loads the address of offset 138 of that same structure into a2.
- `compare_and_increment_pointer`: Calls another routine and, if the zero flag is not set, copies d1 to d2 and d4, modifies a pointer in a5, and either branches to another routine or compares a value in d7 against 488 before incrementing d2.
- `process_iterative_data_structure`: Calls another routine and, if the zero flag is not set, increments a global variable and compares it to a value in d1; it then enters a loop that performs arithmetic on a value from a structure, calls another routine, and updates a value on the stack.
- `manipulate_stack_and_registers`: Pushes two words onto the stack, performs bitwise AND operations on them, and rotates register d0 before branching.
- `copy_memory_block_2`: Copies a block of memory from the address in a0 to the address in a5 one byte at a time using a dbra loop counter, and returns.
- `copy_memory_block`: Copies a block of memory from the address in a0 to the address in a5 one byte at a time, using a dbra loop counter, and returns.
- `compare_structure_fields`: Compares two fields of a structure pointed to by a register and, if they match and a flag field is non-zero, obtains an identifier from another routine and returns it.
- `build_entity_data_buffer`: Initializes a structure by setting several fields, including a constant value and a pointer, and then enters a loop that populates a buffer with metadata and byte data.
- `update_entity_state_and_buffer`: Updates a structure's values by calling another routine, clears a longword at offset 20 of a structure in a0, and performs a loop that decrements a value from d6 while repeatedly calling another routine and copying data into a buffer.
- `update_structure_values_3`: Calls another routine and, if the result is non-zero, stores a word from d0 into a structure at an offset determined by d3.
- `return_constant`: Returns the constant 221 in d0 to the caller without any other effect.
- `update_structure_and_loop`: Calls several routines and, if they return non-zero, performs a loop that decrements a value from a structure and calls another routine; it also updates a field in a structure pointed to by a6.
- `set_flag_in_bitfield`: The routine computes an index from the high and low parts of an input value, selects a 32-bit word from a table pointed to by A6, and sets a single bit within that word corresponding to the low bits of the index.
- `decrement_and_loop_until_match`: Decrements a value from register d6 and repeatedly calls another routine until the value equals 1791.
- `wait_for_specific_value`: Sets d1 to zero, calls another routine, and if the result is non-zero, enters a loop that repeatedly calls another routine and checks if a word value in d0 equals 1790; if it does, it returns the original value of d6.
- `combine_registers_into_return_value`: The routine receives two 32-bit values in registers and returns a 64-bit value formed by concatenating the first value as the high part and the second value as the low part.
- `clear_bit_from_table`: The routine clears a single bit in a 32-bit word table indexed from a 32-bit value in D0.
- `check_sum_and_load`: Loads a word from a PC-relative address into d1, sets a0 to an address relative to a6, calls another routine, and if the zero flag is not set, calls another routine and, if the zero flag is still not set, returns.
- `copy_name_bytes`: The routine copies a null-terminated byte string from a source pointer to a destination buffer located at an offset from a base pointer, advancing both pointers each iteration.
- `copy_bytes_from_table`: Iterates over a table of pointers and copies a fixed-size block from a global table into a buffer via a pointer.
- `clear_zero_table`: The routine clears a block of memory by writing zeros to consecutive 32-bit words.
- `initialize_disk_transfer_block`: Initializes a memory block by writing specific constant values to offsets, loops until a call returns a specific status code, and stores results from subsequent calls into memory locations.
- `tiny_movea`: Small routine of 1 instruction that copies a6 to a0.
- `validate_and_copy_string_data`: Calls another routine to process a pointer in a4, then performs a series of checks involving a structure in a0 and a byte in a4, potentially copying data from a structure to a location relative to the program counter.
- `set_conditional_d0`: Sets d0 to 524, compares d6 to 3, and if d6 is not 3, checks a byte at the address in a4; if that byte is non-zero, it sets d0 to 205, otherwise it sets d0 to 525.
- `copy_value_to_pc_relative_address`: Checks a byte at a4, and if non-zero, copies a word from an offset of a0 to a location relative to the program counter.
- `parse_drive_identifier`: The routine checks a global condition and reads characters via another routine to verify a 'DF' sequence.
- `tiny_move`: Small routine of 1 instruction that copies (a0)+ to d0.
- `empty_routine`: The routine performs a bitwise AND on a byte in D0 and then returns.
- `copy_path_segment_to_buffer`: Scans a string from a4 for a slash or null terminator, copies the preceding characters to a buffer, and returns a value calculated from the character count.
- `check_extraout_flag`: Calls another routine and stores its result, then tests a flag and two fields of a pointer passed in a register.
- `call_and_test_d0`: Saves d2 and d3 to the stack, calls another routine, restores d2 and d3, and branches if d0 is non-zero.
- `check_sum_result`: Calls another routine and, if the zero flag is not set, loads the value 373 into d0 before returning.
- `sum_negate_array_words`: It adds 128 consecutive 32-bit words pointed to by a pointer in A0, accumulating the sum in a register.
- `fill_zero_data`: This routine performs nine consecutive bitwise OR operations on D0 with zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_zero_or`: This routine performs a single bitwise OR operation on D0 with zero.
- `set_zero_register`: Sets the low byte of register d0 to zero.
- `set_multiple_zero_registers`: Sets the low byte of register d0 to zero eight times consecutively.
- `update_offset_value`: Clears a longword at offset 20 of the structure in a0, calls another routine, and then stores the resulting d0 value into that same offset.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d2.
- `recursive_call_stub`: Saves d3 to the stack, sets d3 to 1, calls itself, restores d3, and returns.
- `load_constant_two`: Loads the value two into register d2.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 0 into d3.
- `combine_divide_values`: Saves a1 to the stack, copies a6 to a1, loads a word from a global variable into d0, calls another routine, and restores a1 before returning.
- `branch_if_d3_not_three`: Saves registers and the stack pointer, links a stack frame, compares d3 to 3, and if not equal it branches to a different location; otherwise it calls another routine and then branches to a third location.
- `process_data_buffer`: Stores several values from registers and pointers into a structure, performs division and arithmetic to calculate new values, and uses a loop to update a data buffer based on the results of other routines.
- `concatenate_d0_and_a6_word`: Pushes d0 to the stack, calls another routine, restores d0, and if d0 is non-zero, performs arithmetic using a word from offset -18 of a6, a word from offset -20 of a6, and words from offsets -16 and -6 of a6, then stores the result at offset 40 of the stack and returns.
- `loop_toggle_cia_bit_2`: Loops a fixed number of times, calling a routine that toggles a CIA port bit and accumulating the results in a register.
- `loop_toggle_cia_bit`: It loops thirty times, each pass clearing a CIA port bit and recording the result.
- `clear_cia_port_bit`: Derives a bit index from a register and applies it to a global variable, clearing the associated CIA port bit.
- `process_data_buffer_loop`: Executes a loop that iterates up to two times, performing conditional checks on a structure in a6 and updating a data buffer at an offset calculated from a word in the structure.
- `configure_disk_dma_transfer`: Initializes DMA control registers and disk path/length parameters, then loops to poll a status flag while managing a counter.
- `loop_write_memory_block`: Loops a fixed number of times, writing a constant word to a memory address and calling other routines.
- `configure_dma_registers_2`: Loads a constant into d0, calls another routine, and then performs a series of conditional calls and a loop based on a value in d2.
- `update_structure_values_2`: Calls another routine, and if the zero flag is set and a word at offset -18 of a6 matches d1, and if d2 and d3 meet specific bounds, it decrements d3, stores it at offset -12 of a6, and performs arithmetic with the value 11 at offset -10 of a6.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 24 into d0, then returns.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 27 into d0, then returns.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 25 into d0, then returns.
- `process_data_sequence_2`: Calculates a memory offset using a word from a structure, calls several routines, and performs multiple conditional checks on bits and values in a structure before executing a final sequence of calls and branches.
- `process_data_sequence`: Performs a series of calls to compare values, check a bit in a structure, and manipulate a counter in a structure until a condition is met or a counter reaches 11.
- `tiny_moveq_rts_4`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `calculate_scaled_difference`: Subtracts a 16-bit value at a negative offset from a pointer in a6 from register d1, multiplies the result by 512, and returns.
- `set_flag_and_increment_counter`: It sets a bit in a 16-bit word located a few bytes before a pointer in a6, using a bit index derived from d3.
- `clear_table_entries`: It clears a table by writing zero to successive entries.
- `extract_high_byte_pair`: It calls another routine to obtain a 32-bit value and reassembles it so the low 16 bits come from the result and the high 8 bits come from the original high byte.
- `interleave_word_bits`: It reads two consecutive words from a pointer and returns a value formed from their low bits.
- `apply_interleave_xor_mask`: It calls a routine to interleave word bits and obtains a 32-bit value.
- `setup_pointer_and_count`: Loads a pointer into register a0 from an offset of 8 from a5 and loads the value 40 into register d1.
- `compute_xor_checksum`: It iterates through a sequence of words pointed to by an input pointer, XORing each into an accumulator.
- `fill_memory_pattern`: Clears a block of memory by writing a repeated 32-bit pattern to each word.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `interleave_word_bits_2`: It interleaves the low and high bits of two 32-bit word streams pointed to by two input pointers.
- `loop_clear_flag_bitwise_mix`: Receives two arguments and loops a number of times derived from the first argument.
- `bitwise_mix_store_word_2`: Pushes d0 to the stack, shifts it right by one bit, calls another routine, restores d0, and calls another routine.
- `modify_byte_bit`: Loads a byte from the address in a0 into d0, checks bit 0 of the byte at offset -1 from the pointer in a0, and then conditionally sets bit 7 of d0 based on bit 6 of d0.
- `clear_high_bit`: Clears bit 7 of d0 and stores the resulting byte into the memory location pointed to by a0.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `configure_dma_registers`: It writes specific values to hardware registers associated with DMA control, including ADKCON, DMACON, DSKLEN, DSKPTH, and DSKSYNC. It enables DMA channels by setting bit 15 in the DMACON register.
- `check_ciab_and_write_constant`: Loads a constant into d0, calls another routine, checks bit 1 of a hardware register, and then either calls another routine or sets d0 to a constant value before branching.
- `write_constants_return`: It writes a constant to a global variable, then writes another constant to the same global variable, and returns.
- `write_adkcon_and_test`: Writes the value 1024 to the ADKCON hardware register and then tests a 16-bit word at offset -30 of the structure in a6, branching if the result is positive.
- `tiny_moveq_3`: Small routine of 1 instruction that loads 255 into d1.
- `set_bit_in_global_flag`: It reads a 16-bit value from a table pointed to by a register, adds a constant, and uses the result to set and then clear a bit in a global variable via the CIAB_PRB register.
- `set_cia_and_wait`: Writes a constant to a hardware register, clears bit 7 of d1, calls another routine, and then loads a constant into d0 before branching.
- `update_table_entry`: Reads a word from a global table using an index, conditionally calls another routine if the value is negative, computes the absolute difference against a register, and writes the result back to the table.
- `modify_flag_and_wait_2`: Saves registers to the stack, tests bit 4 of a hardware register, and then enters a loop that calls another routine and performs conditional branches based on d0 and d1.
- `clear_table_entry`: It computes an index from a short value stored at an offset from a register, uses that index to address a global variable, and stores zero into the selected 16-bit slot.
- `modify_flag_and_wait`: Saves d0 to the stack, calls another routine, and if d1 is not negative, clears bits 0 and 1 of d0, writes d0 to a hardware register, sets bit 0 of d0, writes it to the register again, restores d0, and branches.
- `store_masked_byte`: Calls another routine and stores the resulting byte into a global variable at offset 0 of the structure at address bf d100.
- `mask_input_byte`: It builds a mask from a global variable and a byte taken from a structure, clears one bit in that mask based on a byte from the structure, and optionally clears another bit if a flag in a table indexed by the structure is set.
- `wait_for_flag_and_result`: It repeatedly sets a CIA timer until the routine returns one.
- `check_ciab_bit`: Tests bit 0 of the CIAB_CRA register and, if not set, decrements a 32-bit value in d0 and branches if d0 is zero.
- `set_ciab_timer`: It writes three constant values to a global variable in sequence.

### Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 2 of 4).adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 3 of 4).adf:bootblock

- `boot_entry_3`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 4 of 4).adf:bootblock

- `boot_entry_4`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_and_tables`: Sets the interrupt enable and DMA control registers to enable interrupts and DMA, clears a range of memory using a loop, populates several table entries with specific 16-bit values, and calls another routine before updating several global pointers.
- `store_and_branch`: Stores the contents of register d0 into a longword at a specific memory location and then performs a short branch.
- `manage_blitter_state_and_interrupts`: Updates several hardware registers including Blitter and DMA control, performs conditional jumps based on a structure in a5, and calls several routines to manage data streams and interrupt flags.
- `dispatch_indirect_jump`: Sets the user stack pointer from a value on the stack, clears the interrupt mask bit 6, and performs an indirect jump using a pointer popped from the stack.
- `clear_d0_byte`: Performs a logical OR on register d0 with zero.
- `tiny_ori_2`: Small routine of 1 instruction that merges 202 into d0.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d7.
- `update_state_and_blitter_2`: Tests bit 5 of a hardware register and, if set, saves all registers, calls seven different routines, restores the registers, writes a value to a hardware register, and returns from an interrupt.
- `tiny_ori_4`: Small routine of 1 instruction that merges 64 into d0.
- `wait_for_blt_interrupt`: Loops while bit 6 of a hardware register is set, then writes several values to Blitter and DMA registers before returning if bit 6 is cleared.
- `parse_data_stream`: Reads bytes from a source, advancing a destination pointer by 640 bytes if the byte is 1, or copying 8 blocks of 96 bytes from a table based on the byte value minus 32.
- `update_state_and_blitter`: Manages a counter and index to fetch a 16-bit value from a table, or updates Blitter registers and a pointer based on a sequence of bytes in a data stream.
- `rotate_and_store_words`: Rotates four 32-bit values in a sequence and then stores three 16-bit values into a table using a loop that runs three times.
- `update_blitter_tables`: Checks bits 0, 1, and 2 of a global variable to conditionally call another routine with different offsets from a base address, then clears the global variable, copies several 16-bit values between tables, and writes a series of constant 16-bit values to two memory areas.
- `calculate_blitter_parameters`: Loops while bit 7 of a hardware register is set, then calculates a destination address and several Blitter parameters from a structure in a0 and writes them to hardware registers.
- `calculate_scaled_values`: Loops a specified number of times, calling another routine to get a value, then performs 32-bit shifts and divisions to calculate and store two 16-bit results into a table at offsets 0 and 2 from a pointer.
- `perform_matrix_math`: Performs a series of 16-bit multiplications and subtractions using values from two tables and two input registers to calculate a result stored in d1 and d2.
- `check_rectangle_intersections`: Loops through a list of pointers, fetches coordinate pairs from a table, and writes 1 or 65535 to a destination buffer based on a cross-product calculation.
- `wrap_pointers_in_range`: Adds three different values to three global pointers and wraps each result within a specific 4096-byte range relative to a base address.
- `process_data_structures`: Iterates through a list of structures using a loop, performing different actions based on the value at an offset in the structure, including calling another routine and updating a global variable.
- `update_min_max_values`: Compares and updates four pairs of 16-bit values stored at offsets 0, 2, 4, and 6 from a pointer in a0 using values from d2, d3, d0, and d1.
- `configure_blitter_hardware`: Calls another routine, performs signed arithmetic to find the maximum of two absolute differences, and then writes several 16-bit and 32-bit values to hardware registers starting at a fixed address based on the results and a byte from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset`: Performs a relative branch to an offset.
- `decompress_and_configure_audio`: Decompresses a data stream by applying bitwise NOT operations and multi-byte expansions into a destination buffer, then configures audio registers, DMA, and interrupt settings.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `reset_hardware_state`: Clears four 16-bit words at offsets 168, 184, 200, and 216 from a6, modifies a bit in the CIAA_PRA register, and stores a 16-bit value and a pointer from memory into a6-relative offsets.
- `process_data_stream_structure`: Iterates through a structure in a loop to decompress data by reading bytes and applying bitwise NOT operations or multi-byte expansions.
- `process_compressed_data_stream`: Iterates through a structure in a loop to decompress data by reading bytes and applying bitwise NOT operations or multi-byte expansions based on specific marker values.
- `process_data_byte`: Reads a byte from offset 2 of a structure in a5 and, if it is not zero, stores it at offset 21 of the structure; otherwise, it performs a series of bitwise and arithmetic operations using values from the structure and a table in a2 to update offsets 8 and 18 of the structure before branching to another routine.
- `transfer_byte_and_branch`: Moves a byte from offset 13 of a structure in a5 to offset 13 of the same structure and then branches to another routine.
- `expand_compressed_data_structure`: Reads a byte from a source pointer to determine a control value, then uses it to fetch four 16-bit words from a table and stores them at specific offsets within a structure.
- `configure_decompression_parameters`: Reads a byte from a structure to determine expansion or scaling factors, then updates a control structure by writing specific values to offsets 7, 9, and 10, or performs a division to calculate and store two related values at offsets 26 and 28.
- `configure_dma_transfer`: Sets a source address, a destination address, and a transfer count or control value, affecting hardware DMA registers.
- `load_audio_registers`: Copies four sets of 32-bit data from memory into the audio registers starting at a fixed address, sets a 16-bit value at an offset from a1, and writes two bytes to hardware registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `invoke_two_init_stages`: Calls two subroutines in sequence, the first at the start of the segment and the second at a separate location, then returns zero in the data register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Lion King, The (1994)(Virgin)(AGA)[cr CLS](Disk 1 of 4).adf:LION (raw code file, load address unknown)

- `initialize_system_and_populate_tables`: Initializes hardware registers, clears memory regions, and calls initialize_hardware_and_system.
- `render_blt_sprite_data`: Iterates through a data structure to calculate screen coordinates and dimensions for Blitter operations, then updates Blitter registers and DMA control to draw multiple sprite segments or tiles.
- `call_and_store_byte`: Calls another routine and stores the resulting byte from register d0 into a global variable.
- `configure_hardware_interrupts`: Sets several values in CIAA registers, updates a global pointer with a constant, and modifies the hardware interrupt enable and mask registers before returning.
- `update_pointer_table`: Reads a value from a global variable, adds various offsets to it, and stores the results into four consecutive global variables.
- `copy_data_conditionally`: If a global variable is not equal to 5, it performs two loops to copy data from one location to another and calls another routine; otherwise it returns.
- `clear_and_call_function`: Stores 100 in a global variable, clears seven different global variables, calls another routine, calls a function at a specific address, and finally stores 25 in a global variable before returning.
- `check_and_update_state`: Compares a 16-bit value at offset 50 of a structure in a1 against three constants, and if the value is 33, 34, or 35 and a global variable is zero, it calls another routine, stores a constant in a global variable, calls another routine, and stores 16 in a global variable before returning.
- `update_global_state`: Stores a constant in a global variable, calls another routine, and then conditionally updates several global variables based on the values of two other global variables.
- `compare_and_return`: Compares a word from a global variable with itself and returns if they are equal; otherwise it returns.
- `reset_and_update_counters`: Clears several 16-bit global variables, then checks multiple global status flags and counters to decide whether to increment a specific counter, call another routine, or reset a series of values and trigger a data copy routine via a call to another routine.
- `execute_system_call`: Calls a routine at a specific address using d1 as a parameter, then performs multiple conditional checks on various global variables to decide whether to update a pointer, clear a global word, or call another routine.
- `update_coordinates`: Masks bits of two global variables, adds 32 and 64 to values from a source structure, stores them in a new structure, and sets a global variable to 1.
- `load_and_call_routine`: Loads a value from a global variable into a6, calls another routine, and then stores the constant 47 in d6.
- `decrement_and_process_counters`: Decrements three consecutive bytes in a table within a loop, then performs three specific routine calls and updates a global variable if a counter reaches a specific value.
- `select_d0_value`: Compares a global variable to 6 and sets d0 to 5 if it is greater than 6, otherwise it sets d0 to 4 and calls another routine.
- `prepare_structure_update_parameters`: Loads a pointer into a5, a pointer into a4, and two word values into d1 and d2 before calling update_structure_data.
- `update_object_attribute_table`: Calls another routine and, if the zero flag is set, iterates through a table of structures, checking specific word values at offsets 34 and 0, and offset 20.
- `copy_list_elements`: Calculates an offset using d1, accesses a table via a1, and enters a loop that copies four 16-bit words from the table to a destination structure in a0 for thirteen iterations.
- `adjust_offset_value`: Subtracts a global variable from a 16-bit word at offset 30 of a structure in a0, then conditionally updates several global variables and calls another routine if the result is 384.
- `update_entity_pointer`: Decrements a 16-bit value at offset 2 of a structure in a5, and if it reaches zero, modifies a pointer at offset 10 of that structure by adding 36096.
- `update_state_counter_and_trigger_events`: Checks multiple global flags to determine if the routine should proceed, then increments a 16-bit counter in a global variable.
- `store_constant_values`: Calls another routine, stores four 32-bit constants into a global variable at offsets 0, 4, 6, and 10, calls another routine again, and then stores four 32-bit constants into the same global variable at offsets 0, 4, 6, and 10 in a different order.
- `initialize_memory_blocks`: Loads a constant into a memory location relative to a pointer and clears three consecutive 16-bit global variables.
- `clear_structure_offsets`: Clears three 16-bit values at specific offsets within a structure pointed to by register a5.
- `initialize_and_execute_sprite_batch`: Sets a global pointer to one of two values, then calls three separate routines in sequence before setting the global pointer to a different value and calling three more routines.
- `initialize_hardware_and_system`: Configures various hardware registers including DMA and interrupt control, clears several memory regions, and performs a large sequence of system initializations including copying tables and calling multiple subroutines.
- `initialize_and_copy_table`: Clears ten 32-bit words in a buffer, copies a series of words and longwords from a source table to a destination table and other global variables, and then sets four specific global variables based on a condition.
- `process_list_elements`: Checks if a global variable is 255, then enters a loop that subtracts a value from elements in a list starting at a specific address and updates a global variable.
- `process_command_stream_data`: Iterates through a list of data structures, performing arithmetic operations and conditional updates to global variables and structure members based on values at specific offsets.
- `initialize_structure_and_call`: Initializes a structure at a fixed address with several constant values and two values from registers, then calls two subroutines.
- `initialize_system_parameters_2`: Sets the coprocessor register, clears multiple global words and longwords, fills a memory block with zeros using a loop, and performs conditional updates to various global parameters based on the results of other routines.
- `check_and_adjust_pointer`: Checks a word at a global address and, if not equal to a specific value, subtracts a long from a pointer in a5, increments that pointer by 14, and checks if the resulting word is negative.
- `decrement_counters_and_call`: Calculates a value based on a global variable and a value at a pointer in a1, then conditionally updates several global variables and calls another routine.
- `update_structure_and_call`: Increments a word at offset 8 of a structure in a5 and compares it to 209, performs pointer arithmetic and updates a structure in a5, calls two different routines via absolute addresses, and updates a word at offset 10 of the structure in a5.
- `process_complex_state`: Performs a series of checks on global variables and pointers, updates various counters and flags, calls multiple subroutines, and modifies memory locations based on conditional logic.
- `process_dynamic_counter_thresholds`: Compares values from two different global structures against specific thresholds and calls various subroutines multiple times based on the resulting differences.
- `update_entity_animation_state_2`: Checks several global status flags and counters to determine if an animation frame should increment, then updates a frame counter and calls various subroutines to manage entity state, sprite data, and counter values.
- `calculate_frame_delta`: Subtracts a 16-bit value from a global variable from a 16-bit value pointed to by a0, stores the result back at the pointer in a0, and returns.
- `increment_counters`: Increments three different 16-bit global variables by 1 and returns.
- `update_pointer_and_flag`: Adds a long from a global address to a pointer derived from a global address, then checks if the word at that pointer is negative; if not, it sets a word at a global address to 2.
- `manage_pointer_and_state`: Updates a pointer in a global variable using a value from a0, then performs complex conditional logic to update various global counters, status words, and calls another routine.
- `update_counters`: Checks a 16-bit value in a global variable; if it is zero, it adds 8 to two different 16-bit locations, otherwise it subtracts 8 from those same two locations and returns.
- `search_parameter_table`: Iterates through a table starting at offset 54 of a structure in a0 using a loop that decrements a counter, checking if specific values exist at offset 34 and offset 14 of each entry.
- `tiny_clr`: Small routine of 1 instruction that clears d0.
- `set_pointer_and_call`: If a global variable is 0, it sets several global variables to specific values and then calls another routine, otherwise it sets a global variable to the value in d0 and calls another routine.
- `process_entity_state_and_collision`: Checks several global status flags and, if conditions are met, updates a global counter, modifies a pointer in a global variable, and calls other routines to handle specific state changes.
- `process_callback_list_2`: Checks several global variables and a pointer table; it updates a global word, increments two global counters, calls another routine, and if a counter reaches 5, clears it and calls a routine via a pointer from a global variable before jumping to a location.
- `process_state_and_update_counters`: Checks a 16-bit value in a global variable and, if non-zero, clears a 16-bit value in another global variable, sets a 16-bit flag, and performs several subroutine calls.
- `call_two_subroutines`: Calls two subroutines at fixed addresses.
- `call_routine_with_addresses`: Loads four different addresses into a0, sets d0 to either 23 or 13, and calls another routine for each pair.
- `clear_memory_block_via_loop`: Clears a block of memory by writing zero to four 32-bit words at a time using a loop controlled by a counter in a register, then returns.
- `clear_global_word`: Clears a 16-bit word in a global variable and then branches to another location.
- `reset_entity_status`: If a 16-bit value at a global pointer is non-zero, it sets a 16-bit global variable to 1, clears five other global 16-bit or 32-bit values, and calls another routine.
- `parse_sprite_data_table`: Reads a value from a global variable to index into a table, then enters a loop that iterates three times to extract data from a pointer-based structure.
- `process_callback_list`: Iterates through a list of pointers starting at a fixed address, calls a subroutine for each non-zero entry, and conditionally calls another subroutine via a pointer from a global variable if a comparison matches.
- `copy_data_blocks`: Uses a count from a stack pointer to copy multiple bytes from a source list into a destination starting at a fixed address, incrementing pointers by three bytes per iteration.
- `clear_memory_regions`: Clears three distinct memory regions of varying sizes by looping through them and writing zero to each word or longword.
- `clear_memory_block`: Clears a block of memory (9152 longwords, from the count loaded into d7) starting at the address in a0 one longword at a time, using a dbra loop counter, and returns.
- `update_table_with_offset_data`: Iterates through a table, compares bytes from a source and a target, and writes an incremented byte to the target if they do not match.
- `update_entity_states`: Iterates through a list of structures, performing various data movements and arithmetic on fields including offsets 1, 2, 10, 16, 22, 26, 30, 34, 36, 44, and 48, while using values from two additional source lists.
- `load_pointers_and_subtract`: Loads a pointer from a global variable and two 16-bit values from other global variables, then subtracts 16 from one of those values.
- `setup_blitter_transfers`: Configures several Blitter registers at the hardware base address and iterates through a list of structures to set up transfer parameters including source, destination, and size.
- `manage_multiple_counters`: Decrements four different 16-bit global counters in a loop, calls various subroutines based on the values of these counters, clears several global words, and performs conditional updates to several other global variables.
- `increment_table_values`: Runs a loop 41 times, adding the value in d0 to a 16-bit word at an offset of 4 from a base address, then increments that base address by 90.
- `update_global_counters`: Performs complex arithmetic on two global words, conditionally calls several subroutines, and updates various global variables and structure members based on bit tests and comparisons.
- `modify_global_counters`: Performs arithmetic on two global words based on bit tests, then enters a loop that calls two subroutines and conditionally increments or decrements the global words based on a bit test of a returned value.
- `manage_complex_counters`: Performs extensive arithmetic on two global variables, calls several routines including increment_counters and fetch_indexed_byte, and updates multiple global variables and pointers based on various conditions.
- `calculate_byte_difference`: Calls two routines to obtain values, then compares a byte in d0 to a value in d3 to perform subtraction and return a result.
- `lookup_indexed_byte`: Processes a 16-bit input to calculate an index, uses that index to fetch a byte from a table via a global pointer, performs further arithmetic and indexing into another table, and returns a byte.
- `calculate_indexed_difference`: Calls two routines and performs arithmetic on d1 and d3 using values from a global variable and a structure pointed to by a6, then updates d1 or d3 based on comparison results.
- `fetch_indexed_data`: Checks a 16-bit value in a6; if it is 65535 or 65534, it performs a series of table lookups and arithmetic to return a value, otherwise it performs a complex sequence of conditional checks, updates multiple global variables and pointers, and calls several other routines.
- `fetch_indexed_byte`: Calculates an index from a 16-bit input, uses it to read a byte from a table pointed to by a global variable, and then uses that byte to index into a second table to return a byte.
- `evaluate_animation_conditions`: Checks bits in d1 and a global word at offset 4f1a, then compares a value derived from adding a word at 447a to a word at d144 with constants to determine whether to clear a global word at fbda and call another routine.
- `toggle_animation_state_and_reset_counters`: Checks several global status flags and counters to determine if an animation frame should increment, then updates a frame counter and calls update_animation_frame_and_counters.
- `update_animation_frame_and_counters`: Checks several global status flags and counters to determine if an animation frame should increment, then updates a frame counter and calls a routine to handle the resulting state change or animation sequence.
- `process_entity_counters`: Checks a global word, and if zero, performs conditional logic involving two loops that call other routines to decrement counters and update global values.
- `select_parameter_and_branch`: Tests bits in d1 to perform various branches and calls, eventually writing a value to a structure at a6 offset, a global word, and a global long before branching to another routine.
- `decrement_and_call_if_zero`: Decrements a 16-bit word in a global variable and calls another routine via an indirect call if the value reaches zero.
- `update_animation_state`: Checks a pointer in a global variable and, if non-zero, copies a word from a global variable to a location pointed to by that pointer.
- `setup_blt_parameters`: Iterates up to four times to read bytes from a source, calculates offsets and sizes, and writes several values to the hardware registers and a structure starting at offset 256 of a6.
- `update_counter_values`: Copies two words from global memory into registers, calls another routine, and then either adds or subtracts a value from a global word based on the result.
- `check_status_flags`: Tests various bits in d1 and compares a value from a global variable with a constant, eventually returning 0 or 1 based on several conditional checks.
- `select_parameter_and_branch_2`: Adjusts a6, calls another routine, and then performs multiple bit tests on d1 to conditionally call another routine, clear a global word, or return specific values in d1.
- `toggle_entity_state_and_reset_counters`: Checks a global flag and, if not already set, calls another routine and verifies several other global status flags; if these conditions are met, it updates a pointer with a constant value, calls another routine, and toggles a bit in a global flag before updating a state variable and clearing several counters.
- `update_entity_animation_state`: Checks global flags and entity status to determine if an animation sequence should advance or reset.
- `pack_color_data`: Loops 15 times to extract nibbles from three bytes in a0, performs bitwise shifts and OR operations to pack them into a 16-bit value, and stores this value into a structure at offsets 0, 354, 106, and 1536.
- `increment_counters_2`: If a global variable is zero and another global variable is less than 512, it increments four different global variables by one; otherwise, it performs other conditional updates to global variables.
- `increment_counters_with_conditional_logic`: Compares a 16-bit word from a pointer in a2 with a global variable and, if certain conditions are met, decrements three global variables and increments four others.
- `execute_blitter_sequence`: Manipulates a structure in a5 by updating pointers and values, configures Blitter registers via a pointer to a0, and performs a loop that calls several routines to update global counters and structure fields.
- `decrement_counters`: If a global variable is zero and another global variable is less than 48, it decrements four different global variables by one; otherwise, it performs other conditional updates to global variables.
- `decrement_counters_with_conditional_logic`: Compares a 16-bit word from a pointer in a2 with a global variable and, if certain conditions are met, increments three global variables and decrements one.
- `update_structure_and_blitter`: Checks a global word and a value at offset 4 of a structure in a5, increments a global variable, and if a specific bitmask condition is met, modifies several pointers in a5, configures hardware registers at a fixed address, and calls three different routines.
- `execute_multiple_subroutines`: Pushes a global pointer onto the stack, then performs a series of checks on four different global words; based on these values, it calls one of four different routines multiple times, and finally restores the global pointer from the stack before returning.
- `update_config_values`: Compares a long value at a global variable and, depending on the result, writes a specific set of eight long values and one word to various memory locations and a hardware register.
- `toggle_global_value`: Compares a global variable to a constant and, if they are equal, writes a different constant to that global variable; otherwise, it writes the original constant and sets a word in memory to one.
- `toggle_global_word`: Tests a word at a global address and, if it is zero, performs an exclusive OR on a different global word and tests that result.
- `process_offset_table`: Iterates through a table to find a specific marker, performs arithmetic on a value, and updates a pointer in a global variable.
- `calculate_offset_with_addition`: Performs arithmetic on d1 and d2 using values from two global variables, multiplies d2 by a value from another global variable, and adds the result to a pointer in a6.
- `calculate_offset_simple`: Performs arithmetic on d1 and d2 using bit shifts and a multiplication by a value from a global variable, then adds the result to a pointer in a6.
- `update_structure_and_copy_data`: Increments a value in a structure and, upon reaching a threshold, updates a pointer and calls two other routines to copy data blocks.
- `reset_and_call_routines`: If a long at a global address is non-zero, it returns; otherwise, it clears two global words, loads a long from a global address into a structure, and calls five subroutines.
- `decrement_structure_and_copy_data`: Decrements a value in a structure and, upon reaching zero, updates a pointer and calls two other routines to copy data blocks.
- `update_blitter_structure`: Increments several pointers within a structure, configures Blitter hardware registers using values from a structure and a global pointer, and calls three subroutines to perform transfers.
- `perform_blitter_transfers_2`: Configures Blitter hardware registers, decrements pointers within a structure, and executes multiple data transfers using a loop that processes bit-packed data and stack-based values.
- `extract_flag_bit_from_table`: Scans a table of word pairs until a condition on a global mask is met, then reads a byte from a table entry and uses it as an index into a global mask to extract a bit, calling another routine if a borrow condition is detected.
- `mask_and_conditional_call`: The routine masks a word from a register with a word from a register and, if a byte from a register is less than a byte from a register, it calls another routine and uses its returned word as the result; otherwise it returns the masked word.
- `manipulate_registers_from_table`: Reads two bytes from a table pointed to by a3, performs bitwise shifts and swaps on them, and modifies d1, d6, and d7.
- `accumulate_32bit_from_bytes`: The routine reads four consecutive bytes from a pointer in a register and builds a 32-bit value in a register by concatenating each byte into the result.
- `decode_and_fill_table`: Reads a count, fills a buffer from another routine, then iterates over a stack buffer and a table, writing bit masks and derived values into table entries based on matching bytes.
- `multiply_and_copy_data_2`: Reads several words from a source, performs arithmetic to calculate a new value, and then uses multiple loops to copy and fill data into a destination buffer at a fixed address.
- `multiply_and_copy_data`: Multiplies two 16-bit words from a source list, stores the result in d6, and then copies words from the source to a destination at a fixed address until two consecutive bytes of 255 are encountered.
- `update_counters_conditionally`: Copies two words from a structure to a global table, modifies the values in the structure based on comparisons with a global variable and a constant, and then enters a loop that calls another routine 47 times, incrementing a value in the structure if certain conditions regarding d1 and d3 are met.
- `decrement_counters_iteratively`: Copies two words from a structure to a global table, subtracts 32 from one structure value and 24 from another, and then enters a loop that calls another routine 47 times, decrementing a value in the structure if certain conditions regarding d1 and d3 are met.
- `process_entity_event_list`: Iterates through a list of pointers, extracts an ID from each entry, and uses that ID to select a data structure and a loop count.
- `update_entity_coordinate_offsets`: Checks a value at offset 52 of a structure against a list of excluded constants; if not excluded, it calculates a difference between values at offsets 50 and 52 and applies that difference to offsets 4 and 52 of the structure, or performs specific additions, subtractions, or clears based on the calculated result.
- `process_entity_action_list`: Iterates through a list of entity identifiers, looking up action data in a table based on the identifier.
- `modify_list_and_counters`: Clears a global word, performs arithmetic on values from a structure, and enters several loops that either increment or decrement values at specific offsets within that structure based on bit tests and comparisons.
- `update_structure_list_2`: Iterates through a list of structures, checking specific field values to clear bytes and update pointers within those structures.
- `loop_and_call_handler`: Loads a pointer from a global variable, decrements it by 14, and enters a loop that calls another routine 28 times while incrementing the pointer by 2 each iteration.
- `call_subroutine`: Calls a routine at a fixed address.
- `initialize_system_parameters`: Calls four subroutines via a jump table, updates several global word values, clears a global word, and either calls a subroutine at an address from a global pointer or calls a routine at an address from a different global pointer.
- `modify_structure_offset`: Calls another routine, calculates a value for d2 by adding or subtracting a constant from a global word based on a test of another global word, and then calls a subroutine before storing a value in a structure at offset 14 of a6.
- `call_indirect_routine`: Loads a long value from a global variable into a5 and calls the routine at the address stored in a5.
- `update_structure_data`: Decrements a word at offset 56 of a structure in a1, then uses a value from offset 34 of that structure to perform an indirect jump via a table.
- `check_status_value`: Compares a word at offset 34 of a structure in a1 with the values 5, 23, or 6, and calls another routine if the value is 5 or 6.
- `update_structure_list`: If a 16-bit value at offset 52 of a structure in a1 is 5 or 6, it performs a loop three times to copy words from a list pointed to by offset 72 of the structure to a new destination, clears two values at offset -6 and -2 of the current list element, and updates offset 68 of the structure with the new list pointer.
- `initialize_data_structure`: Populates a structure in a6 with various values, including pointers from a4 and a5, and performs arithmetic using d1, d2, and values from several global variables to store results in offsets 50 and 52 of the structure.
- `modify_global_and_process_list`: Performs arithmetic on a global word, updates it, and then either clears two global words and calls a routine, or iterates through a linked list at offset 30 of a structure in a1 to update entries.
- `process_linked_list`: Loops through a list of pointers at offset 30 of a structure in a1, updating the pointer to the next element if the current element is negative, then uses a value from a table to update several fields in the structure in a1.
- `dispatch_command_stream_opcode`: Checks a structure for a specific marker and decrements a counter at offset 8, then reads a word from the structure and performs a large switch-case dispatch based on its value to call various subroutines or update global pointers and counters.
- `process_data_command_stream`: Iterates through a list of data structures, performing various arithmetic operations, pointer updates, and conditional list manipulations based on command values.
- `render_sprite_data_blocks`: Iterates through a data structure to perform multiple Blitter transfers, using values from a table to calculate source and destination addresses for drawing pixel blocks.
- `update_structure_values`: Reads two bytes from a source table via a pointer in a global variable, performs arithmetic based on a value in a global variable and a table entry, and writes several calculated 16-bit values to a destination structure at offset 0 and offset 8, with additional writes to offsets 1632, 1634, 1636, and 1638 depending on a comparison.
- `update_scaled_coordinate_table`: Calculates and stores a series of scaled 16-bit coordinates into a structure by performing additions and bitwise swaps on values retrieved from a table.
- `write_scaled_table_data`: Copies a block of data from one location to another using a loop, and performs a series of calculations involving bitwise operations and additions to update multiple entries in a structure.
- `write_scaled_values_to_table`: Calculates a value based on a word at offset 10 of a structure in a6 and a pointer in a5, then enters a loop that writes this value and a swapped version of it into a table at a2, incrementing the table pointer by 8 each iteration.
- `initialize_system_parameters_3`: Clears d1, calls another routine, decrements a word in a global variable, loads two pointers into a0 and a1, and calls a third routine before returning.
- `modify_structure_offset_2`: Stores a word into a global variable, loads a word from a global variable into d1, and either branches or increments d1 based on a comparison.
- `update_structure_offsets_2`: Stores a 16-bit word from d1 into a global variable, then accesses a table via a1 to conditionally add or subtract values from offsets 2 and 50 of a structure based on bit flags and comparisons.
- `update_structure_offsets`: Checks specific bit flags and values within a structure, then either adds or subtracts a value from offsets 2 and 50 of that structure based on a global variable.
- `update_collision_bounds`: Calculates a bounding range based on a structure's properties and a table, then performs collision detection by checking if a coordinate falls within specific intervals.
- `branch_with_offset`: Sets a1 to point to offset 91 of the structure in a1, branches to another routine, and otherwise clears d0 and returns.
- `evaluate_bit_pattern_one`: Calls another routine and returns 0, 1, or 2 based on a specific combination of bits 1, 4, 6, and 7 in the returned d1 register.
- `evaluate_input_bitmask`: Calls another routine and performs a series of bit tests on the returned value in register d1 to return 0, 1, or 2 based on specific bit combinations.
- `process_data_list`: Iterates through a list of pointers using a count from a0, calls a subroutine, and if a global word is not zero, calls another routine to clear a byte at offset 1 and a longword at offset 36 of a structure in a1.
- `update_object_property_table`: Iterates through a list of entries to update a structure, calculating specific offsets and values based on a table lookup and a counter.
- `process_table_entries`: Iterates through a list of entries, copying pointers from a table to a structure at offset 80, and performs conditional updates to the structure at offset 88 and offset 56 based on a value from a global variable.
- `copy_data_blocks_2`: Copies 20 words of data from a table to a destination buffer, using an offset determined by a value in a global variable, and repeats this in a loop.
- `set_structure_parameters`: Writes the values 2, 2, a pointer from a2, a pointer from a3, and 115 to offsets 100, 102, 80, 84, and 88 of a structure in a6, then returns if a bit in offset 2 is not set.
- `perform_blitter_transfers`: Configures Blitter registers and performs multiple data transfers by iterating through tables to fetch source and destination addresses, then calls another routine.
- `update_state_and_dispatch_event`: Calculates a value from a global word, adds 94, and stores the resulting byte and a constant word into global variables.
- `initialize_subsystems_and_loop_routine`: Sets a global word to one, calls five different routines, and then executes a loop that calls another routine ninety times before calling a final routine and returning.
- `update_config_values_3`: Writes the word value 528 to a global variable, then checks a long value at a global variable; if it is non-zero, it checks another long value and, if it is non-zero, it writes the word 224 to an offset of 554 in a6 and calls another routine.
- `update_config_values_2`: Performs arithmetic on a value from a global variable and writes several calculated 16-bit and 32-bit values to various offsets in a structure pointed to by a global variable.
- `initialize_table_pointer`: Sets a global word at f3da, loads an address into a0 from f294, calls a routine at 6928, and branches to another location.
- `initialize_and_process_data_table`: Sets a global word to one, calls a routine with a pointer, and iterates through a table to find a non-negative value.
- `execute_sequence_of_calls`: Calls several subroutines and a routine at a fixed address, using values in d2, d1, and d0.
- `jump_to_address`: Clears d0, loads a value into a0 from a global variable, loads a constant into a1, and jumps to an address in a1.
- `execute_loop_call`: Calls a routine and then enters a loop that repeats the call until d1 is zero.
- `update_config_values_4`: Stores d2 into a global variable, calls two subroutines, calls another subroutine, calls a third subroutine, increments d2, calls a fourth subroutine, and branches to another routine after loading a pointer from a global table.
- `clear_memory_blocks`: Clears three specific memory areas by iterating through them with a loop, and also performs a series of bitwise OR operations on register d1.
