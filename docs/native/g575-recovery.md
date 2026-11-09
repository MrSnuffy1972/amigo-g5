# Game 575 (Immortal, the): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00575). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1839728 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 324 of 324 named (0 author symbols), 324 of 324 described (10 by rule, 314 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4701 of 4929 (95.37%) |
| Recovered without load address (counted in proof 1, flagged) | 3072 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5850 of 5851 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 38366 |
| code | 34340 |
| data | 1744426 |
| padding | 68 |
| unused | 22528 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf:bootblock | 1 | 94 | 3 |
| logged disk load from Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf boot-block load | 1 | 187 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 665 | 39 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1412 | 158 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 86 | 5 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 45 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2801 | 98 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 47 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 514 | 6 |
| Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf bytes 757760-758784 (code never loaded by a reference run) | 1 | 0 | 0 |
| Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf bytes 784384-785920 (code never loaded by a reference run) | 1 | 0 | 0 |
| Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf bytes 788480-788992 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: the boot block reads 22528 bytes from disk offset 856064 to an address (found statically); recovered as a unit
- runtime segment an address: 343 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1188 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 85 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2574 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 47 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 357 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: bytes 757760-758784 look like code no run loaded: recovered position-independently (load address unknown)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: bytes 784384-785920 look like code no run loaded: recovered position-independently (load address unknown)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: bytes 788480-788992 look like code no run loaded: recovered position-independently (load address unknown)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: 4 logged loads cover 25600 bytes of the image
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: 47 track-loaded blocks found in recovered runtime segments and classified through them
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.77)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 4-879 (bytes 2048-450560) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.68)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 882-1259 (bytes 451584-645120) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.11)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1261-1262 (bytes 645632-646656) outside the filesystem hold data, not code (opcode density 0.021, entropy 4.39)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1268-1302 (bytes 649216-667136) outside the filesystem hold data, not code (opcode density 0.014, entropy 5.79)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1305-1305 (bytes 668160-668672) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.50)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1311-1311 (bytes 671232-671744) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.15)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1313-1314 (bytes 672256-673280) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.38)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1318-1320 (bytes 674816-676352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.33)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1322-1324 (bytes 676864-678400) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.98)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1326-1326 (bytes 678912-679424) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.99)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1328-1457 (bytes 679936-746496) outside the filesystem hold data, not code (opcode density 0.022, entropy 6.51)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1463-1464 (bytes 749056-750080) outside the filesystem hold data, not code (opcode density 0.021, entropy 1.76)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1466-1466 (bytes 750592-751104) outside the filesystem hold data, not code (opcode density 0.047, entropy 5.89)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1483-1483 (bytes 759296-759808) outside the filesystem hold data, not code (opcode density 0.020, entropy 3.66)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1488-1493 (bytes 761856-764928) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.06)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1542-1542 (bytes 789504-790016) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.67)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1544-1671 (bytes 790528-856064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.60)
- immortal, the (1990)(electronic arts)[cr pdx](disk 1 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf: blocks 1716-1759 (bytes 878592-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.57)
- immortal, the (1990)(electronic arts)[cr pdx](disk 2 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 2 of 2).adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- immortal, the (1990)(electronic arts)[cr pdx](disk 2 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 2 of 2).adf: blocks 4-879 (bytes 2048-450560) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.59)
- immortal, the (1990)(electronic arts)[cr pdx](disk 2 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 2 of 2).adf: blocks 882-1759 (bytes 451584-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.42)
- immortal, the (1990)(electronic arts)[cr pdx](disk 2 of 2)/Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- 3 never-loaded region(s), 3072 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `loop_cia_register_writes` (runtime segment an address): Clears a 32-bit value at a constant address, writes two bytes to a hardware register address, performs a series of bitwise operations on a byte from a hardware register and writes it back, then enters a loop that calls another routine 250 times before returning a 32-bit value from a constant address. Kind: disk hardware/timing check. Pass path: the loop completes normally (sVar1 reaches -1).

## Functions


### Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf:bootblock

- `initialize_system_memory_and_hardware`: Checks available memory via a library call and, if sufficient, configures a structure to perform a DMA transfer of a memory block to a specific destination.
- `copy_vector_table_block`: Disables interrupts, copies a table of 17 longwords from a fixed location relative to the program counter to a destination address held in a register, stores that destination address at an offset of 16384 from the start of the destination, and returns.
- `copy_memory_blocks`: Copies a block of memory to a destination in the 30000 range using a loop, then calls another routine.

### logged disk load from Immortal, The (1990)(Electronic Arts)[cr PDX](Disk 1 of 2).adf boot-block load

- `copy_and_initialize_color`: Copies a byte from a source pointed to by a2 to a destination pointed to by a1, which is offset from a constant, then enters a loop that calls another routine and decrements d0 until d0 reaches zero, finally writing a 16-bit value to a hardware register.
- `call_routine_return_argument`: Calls a routine at a fixed address and returns the argument unchanged.
- `extract_bitstream_data`: Reads a variable-length sequence of bits from a source pointer in a3 and writes the resulting bytes to a destination pointer in a4.
- `decode_variable_length_bitstream`: Reads a sequence of bits from a source pointer in a3 by checking for zero values and applying shifts to determine a bit count and a resulting value, then stores the final value in d4 and sets d0 to 3 before branching to another routine.
- `expand_bitstream_data_2`: Reads a sequence of bits from a source pointer, using a table to determine the number of bits to extract and the resulting value, and writes the expanded data into a destination buffer.
- `set_registers_and_branch`: Sets register d4 to 5 and register d0 to 3, then branches to another routine.
- `set_registers_and_branch_2`: Sets register d4 to 4 and register d0 to 2, then branches to another routine.
- `set_registers_and_branch_3`: Sets register d4 to 3 and register d0 to 1, then branches to another routine.
- `expand_bitstream_data`: Reads a bitstream from a source pointer, using a table to determine bit counts and lengths, and writes the resulting expanded data into a destination buffer.
- `decompress_bitstream_data_2_2`: Performs bitstream decompression by reading a count and length from a table, expanding bits into a destination buffer, and copying data from a source buffer using bit-driven logic and table lookups.
- `decompress_bitstream_data_2`: Reads a length and a bit-count from a table to perform bitstream decompression, expanding bits into a destination buffer using a series of shifts and additions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_hardware_initialization`: Stores a 16-bit word from d0 into a global variable, then checks a byte at a global variable; depending on whether that byte is zero or one, it calls a sequence of up to six different routines before returning a 32-bit value from a global variable in d0.
- `initialize_hardware_state`: Writes several values to global variables, calls three different routines, and performs a loop that decrements a global variable until it reaches zero or a specific condition is met.
- `execute_complex_state_update`: Saves registers to the stack, clears a 32-bit global variable, and stores several values into a structure and other global variables; it then calls several routines and enters a loop that decrements a 16-bit value at a global variable and increments a 32-bit value at a global variable until a counter reaches eleven, while also performing a nested loop that decrements another 16-bit value.
- `decrement_and_call_subroutines`: Decrements a 16-bit value at a global variable and, if not zero, branches to another routine; otherwise, it writes a 16-bit constant to a global variable, calls two different routines, and returns d0 after restoring registers.
- `decrement_and_branch`: Decrements a 32-bit value at a constant address, branches if the result is not zero, and otherwise performs an unconditional branch to another location.
- `decrement_global_counter_and_branch`: Decrements a 16-bit word in a global variable and branches to another routine if the result is non-zero, otherwise it performs an unconditional short branch to a third routine.
- `decrement_and_call_routine`: Decrements a 16-bit value at a constant address, calls another routine if the result is not zero, restores registers from the stack, loads a 32-bit value from a constant address into d0, and returns.
- `copy_data_to_destination`: Calculates a destination address based on a value in a global variable, copies 128 words from a source pointer to that destination, clears a global variable, and returns.
- `process_data_and_loop_counters`: Saves registers to the stack, clears a 32-bit global variable, and stores several values into a structure and other global variables; it then calls several routines and enters a loop that decrements a 16-bit value at a global variable and increments a 32-bit value at a global variable until a counter reaches eleven.
- `copy_data_from_source`: Calculates a source address based on a global variable, copies 128 words from that source to a destination pointer, clears a global variable, and returns.
- `xor_checksum_loop`: XORs a series of longwords from a pointer into a register and returns the result masked with a bit pattern.
- `call_three_routines_pass_input`: Calls three shared routines in sequence and returns the input register unchanged.
- `store_masked_bitplane_word`: Builds a word from the alternating bit pattern of the input and its shifted neighbours, clears the top bit when a flag byte before the target is set, and stores the result through the pointer.
- `modify_byte_based_on_bits`: Loads a byte from the address in a0 into d0, checks bit 0 of the byte at offset minus 1 from a0, and if bit 0 is set, checks bit 6 of d0; if bit 6 is set, it sets bit 7 of d0, otherwise it returns.
- `clear_high_bit`: Clears the most significant bit of a byte in d0 and stores the result at the pointer in a0.
- `initialize_data_structures`: Writes a constant longword to an offset of 4 in a structure, calls a routine in a loop four times, and calls three other routines with specific offsets and values in a0 and d1.
- `call_four_shared_routines`: Calls four shared routines in sequence and returns.
- `initialize_structure_variants`: Checks a bit in a structure, then performs one of several initialization sequences that set various 16-bit and 32-bit values in a structure based on data from another structure.
- `interleave_two_bitplane_words`: Takes two longwords from consecutive memory locations and combines them by interleaving their bits, returning the merged longvalue.
- `call_shared_routine_pass_input`: Calls one shared routine and returns the input register unchanged.
- `setup_structure_parameters`: Initializes a structure with specific 16-bit and 32-bit values derived from a source structure and a global pointer.
- `reset_status_bytes`: Clears a byte at one global variable location and sets another global variable to decimal 48.
- `set_global_values_one`: Writes the byte 1 to a global variable and the byte 40 to another global variable, then returns.
- `set_global_values_two`: Writes the byte 2 to a global variable and the byte 24 to another global variable, then returns.
- `loop_cia_register_writes`: Clears a 32-bit value at a constant address, writes two bytes to a hardware register address, performs a series of bitwise operations on a byte from a hardware register and writes it back, then enters a loop that calls another routine 250 times before returning a 32-bit value from a constant address.
- `write_cia_registers`: Writes the value 255 to the first byte and 135 to the second byte of a memory location pointed to by a constant address.
- `update_cia_control_registers`: Modifies a byte at a hardware register address by masking bits 7 and 6 and setting bit 3, then writes 127, 100, and 8 to three other hardware register addresses, and returns if bit 0 of the first register was not set.
- `process_hardware_and_counters`: Clears a 32-bit value at a constant address, compares a 16-bit value from a constant address to another 16-bit value at a constant address, and branches based on the comparison to either decrement or increment a 16-bit value at a constant address, or to call several routines and update a 16-bit value from a constant address.
- `conditionally_write_cia_register`: Writes a byte to a hardware register address by ORing 43 with a byte from a hardware register, and if a 16-bit value at a constant address is not zero, it ORs 47 with that byte and writes it to the hardware register, otherwise it ORs 47 with the byte and writes it to the hardware register before calling another routine twice.
- `write_multiple_cia_values`: Writes four different byte values, each ORed with a byte from a hardware register, to a hardware register address and then branches to another routine.
- `write_sequence_cia_values`: Writes three different byte values, each ORed with a byte from a hardware register, to a hardware register address and then branches to another routine.
- `clear_and_loop_cia`: Loads a 16-bit constant into a register, clears a 32-bit value at a constant address, calls another routine, tests bit 4 of a hardware register, and if set, enters a loop that decrements the register until it reaches zero before clearing a 16-bit value at a constant address.
- `process_division_and_swap`: Divides a 16-bit value from a global variable by 11, clears a global variable, and if the quotient's lowest bit is set, increments that global variable, shifts the quotient right, stores it in another global variable, swaps its high and low words, and updates two global variables with the result.
- `fill_memory_and_loop_routine`: Fills a memory range starting at a specific address with the value 170 using a loop, calculates a value based on two global variables and a swap operation, then enters a loop that calls another routine and modifies two pointers and a counter until the counter reaches zero.
- `check_cia_bit_and_set_mask`: Tests bit 5 of a hardware register and, if clear, branches to another routine; otherwise, it stores the constant negative eight in a global variable and returns d0 with that value.
- `check_cia_bit_and_set_constant`: Tests bit 3 of a hardware register and, if set, branches to another routine; otherwise, it stores the constant negative one in a global variable and returns d0 with that value.
- `configure_memory_structure`: Clears a 32-bit global variable, calls another routine, and populates several fields in a structure pointed to by a6 with specific constants and calculated values.
- `verify_and_initialize_hardware_state`: Checks a bit in a CIA register and returns a specific value if set; otherwise, it initializes several fields in a structure pointed to by a6, modifies bits in a CIA register, and enters a loop that repeatedly calls validate_and_process_data while checking a sequence of words in a table and comparing return values.
- `validate_and_process_data`: Writes values to global variables and a hardware register, calls several routines, checks a sequence of words in a table, and if valid, performs a series of checks and calls a routine to copy data to a destination calculated from a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_address`: Performs an unconditional branch to another location.
- `execute_indirect_jump_3`: Stores the current program counter into a global variable, performs several byte-swapping operations between registers, and jumps to an address loaded from a pointer at the location pointed to by a5.
- `jump_to_masked_address`: Masks the lower 8 bits of a value in d0 and jumps to the address stored in a5.
- `jump_via_table_entry`: Ors a 16-bit value from a table into d0, masks it to 8 bits, moves it to d4, loads a new address from the table into a0, and jumps to that address.
- `jump_after_bit_shift`: Adds d0 to itself, moves the result to d4, rotates d3 right through the carry, loads an address from a table into a0, and jumps to that address.
- `jump_with_bitwise_or`: Loads an address from a table into a0, performs a bitwise OR between d0 and the byte at that address, moves the result to d4, loads a second address from the table into a0, and jumps to that address.
- `jump_after_byte_doubling`: Loads an address from a table into a0, moves the byte at that address to d4, doubles that byte, rotates d3 right through the carry, writes the doubled byte back to the same address, loads a second address from the table into a0, and jumps to that address.
- `transfer_and_jump`: Copies a byte from offset 1 and a byte from offset 0 of a structure pointed to by a value from a table into a destination structure at a6, performs a bitwise OR between a byte at a0 plus an offset in d2 and d0, and then jumps to a second address retrieved from the table.
- `jump_via_table`: Clears d3 and jumps to an address retrieved from a table using a pointer from a5.
- `jump_with_offset`: Adds a 16-bit value from a table to d2, performs a bitwise OR between a byte at the resulting address and d0, and then jumps to a second address retrieved from the table.
- `mask_and_jump`: Performs a bitwise AND between a 16-bit value from a table and d0, then jumps to an address retrieved from a second 16-bit value in the table.
- `mask_byte_and_jump`: Performs a bitwise AND between a byte at an address from a table and d0, then jumps to a second address retrieved from the table.
- `rotate_byte_and_jump`: Loads a byte from an address in a table, rotates it left through the carry bit while rotating d3 right through the carry bit, writes the result back to the same address, and jumps to a second address from the table.
- `rotate_byte_with_offset_and_jump`: Adds a 16-bit value from a table to d1, rotates the byte at that address left through the carry bit while rotating d3 right through the carry bit, and jumps to a second address from the table.
- `jump_with_constant`: Sets d3 to 255 and jumps to an address retrieved from a table using a pointer from a5.
- `mask_and_jump_with_offset`: Adds a 16-bit value from a table to d2, performs a bitwise AND between a byte at the resulting address and d0, and then jumps to a second address retrieved from the table.
- `xor_and_jump`: Masks a 16-bit value from a table to 8 bits, performs a bitwise XOR between that value and d0, and then jumps to an address retrieved from the table.
- `shift_and_jump`: Shifts the byte in d0 right by one, rotates the bit into d3, and then jumps to an address loaded from a table pointed to by a5.
- `shift_byte_and_jump`: Loads an address from a table, shifts the byte at that address right by one, and then jumps to another address loaded from the table.
- `add_offset_and_jump`: Adds a word from a table to d1, uses the result as an address to shift a byte at that location right by one, and then jumps to another address from the table.
- `copy_bytes_and_jump`: Adds a word from a table to d1, copies two bytes from that location to offsets 2 and 3 of a structure in a6, and then jumps to an address from a table.
- `jump_if_positive`: Tests a byte in d3 and, if it is positive, increments a pointer in a5, loads a word from that pointer into d5, and jumps to the address in d5.
- `jump_if_negative`: Tests a byte in d3 and, if it is negative, increments a pointer in a5, loads a word from that pointer into d5, and jumps to the address in d5.
- `jump_if_nonzero`: Tests a byte in d4 and, if it is not zero, increments a pointer in a5, loads a word from that pointer into d5, and jumps to the address in d5.
- `jump_if_not_zero`: If the byte in d4 is not zero, it increments a pointer in a5 by two, loads a word from that location into d5, moves that word into a0, and jumps to the address in a0; otherwise it increments a5 by two and jumps to the address in a0.
- `jump_if_positive_2`: If the byte in d4 is positive, it increments a pointer in a5 by two, loads a word from that location into d5, moves that word into a0, and jumps to the address in a0; otherwise it increments a5 by two and jumps to the address in a0.
- `jump_if_not_negative`: If the byte in d4 is not negative, it increments a pointer in a5 by two, loads a word from that location into d5, moves that word into a0, and jumps to the address in a0; otherwise it increments a5 by two and jumps to the address in a0.
- `xor_byte_and_jump`: Loads an address from a table, performs an exclusive OR between the byte at that address and d0, and then jumps to another address from the table.
- `copy_and_xor_jump`: Loads an address from a table, copies two bytes from that location to offsets 2 and 3 of a structure in a6, performs an exclusive OR between a byte at an offset of a0 and d0, and then jumps to another address from a table.
- `modify_d3_and_jump`: Modifies d3 based on the sign of d4, pushes the result onto the stack, and then jumps to an address loaded from a table.
- `test_bits_and_jump`: Tests bits in a word from the stack to set d4, and then jumps to an address loaded from a table.
- `push_d0_and_jump`: Pushes the word in d0 onto the stack and then jumps to an address loaded from a table.
- `pop_d0_and_jump`: Pops a word from the stack into d0, then jumps to an address loaded from a table.
- `dispatch_via_stack_word`: Pushes a 16-bit word from the stack onto the stack, fetches a 16-bit word from a table pointed to by a5, and jumps to the address stored in that word.
- `dispatch_via_stack_word_and_d4`: Moves a 16-bit word from the stack into d1 and d4, fetches a 16-bit word from a table pointed to by a5, and jumps to the address stored in that word.
- `dispatch_via_d2_word`: Pushes a 16-bit word from d2 onto the stack, fetches a 16-bit word from a table pointed to by a5, and jumps to the address stored in that word.
- `dispatch_via_stack_word_and_d2`: Moves a 16-bit word from the stack into d2 and d4, fetches a 16-bit word from a table pointed to by a5, and jumps to the address stored in that word.
- `dispatch_with_pointer_push`: Fetches a 16-bit word from a table pointed to by a5, pushes the updated pointer to the stack, then fetches another 16-bit word from the new pointer location and jumps to that address.
- `dispatch_with_pointer_increment`: Fetches a 16-bit word from a table pointed to by a5, increments the pointer in a5 by 2, pushes the incremented pointer to the stack, then fetches another 16-bit word from the original word's location and jumps to that address.
- `dispatch_via_pointer_on_stack`: Loads a long pointer from the stack into a5, fetches a 16-bit word from a table pointed to by a5, and jumps to the address stored in that word.
- `dispatch_with_bit_manipulation`: Fetches a 16-bit word from a table pointed to by a5, performs arithmetic and bitwise operations using d3, d7, and d0, and then jumps to an address fetched from the table.
- `dispatch_with_rotation`: Performs arithmetic and bitwise rotation on d3 and d0, moves the result to d4, and jumps to an address fetched from a table pointed to by a5.
- `dispatch_via_swapped_bytes`: Fetches a 16-bit word from a table pointed to by a5, performs byte-level manipulation and rotation on the data at that address, and jumps to a new address derived from those bytes.
- `execute_indirect_jump`: Fetches a pointer from a table, reads a byte from that address, performs an arithmetic operation on a register using the byte and the carry flag, and then jumps to a second pointer fetched from the table.
- `shift_and_jump_2`: Fetches a pointer from a table, reads a byte from that address, performs a right shift on that byte using the carry flag, stores the result back to the same address, and jumps to a second pointer fetched from the table.
- `update_and_jump`: Fetches a pointer from a table, stores two bytes from that address into offsets 2 and 3 of a structure pointed to by a6, performs an arithmetic operation using a byte from the table and a value at an offset in a6, and jumps to a second pointer fetched from the table.
- `execute_offset_jump`: Adds a 16-bit value to a pointer from a table, reads a byte from the resulting address, performs an arithmetic operation using the byte and the carry flag, and jumps to a second pointer fetched from the table.
- `shift_offset_jump`: Adds a 16-bit value to a pointer from a table, reads a byte from the resulting address, performs a right shift on that byte using the carry flag, stores the result back to that address, and jumps to a second pointer fetched from the table.
- `transfer_d1_and_jump`: Copies a byte from d1 to d4 and jumps to a pointer fetched from a table.
- `transfer_d2_and_jump`: Copies a byte from d2 to d4 and jumps to a pointer fetched from a table.
- `transfer_d0_and_jump`: Copies a byte from d0 to both d2 and d4, then jumps to a pointer fetched from a table.
- `transfer_d0_to_d1_and_jump`: Copies a byte from d0 to both d1 and d4, then jumps to a pointer fetched from a table.
- `transfer_d1_to_d2_and_jump`: Copies a byte from d1 to d2 and a word from d1 to d4, then jumps to a pointer fetched from a table.
- `write_byte_and_jump`: Stores the byte from d2 into a memory location pointed to by a value from a table, then jumps to another address from that same table.
- `write_d0_byte_and_jump`: Stores the byte from d0 into a memory location pointed to by a value from a table, then jumps to another address from that same table.
- `write_d1_byte_and_jump`: Stores the byte from d1 into a memory location pointed to by a value from a table, then jumps to another address from that same table.
- `update_structure_and_jump`: Copies two bytes from a table into a structure pointed to by a6 at specific offsets, stores the byte from d0 into a location at an offset determined by d2, and then jumps to another address from a table.
- `write_offset_byte_and_jump`: Stores the byte from d2 into a memory location calculated by adding a 16-bit offset from a table to a base address, then jumps to another address from that table.
- `write_d0_offset_byte_and_jump`: Stores the byte from d0 into a memory location calculated by adding a 16-bit offset from d2 to a base address from a table, then jumps to another address from that table.
- `write_d0_offset_byte_and_jump_alt`: Stores the byte from d0 into a memory location calculated by adding a 16-bit offset from d1 to a base address from a table, then jumps to another address from that table.
- `clear_byte_and_jump`: Clears the byte at a memory location pointed to by a value from a table, then jumps to another address from that same table.
- `clear_offset_byte_and_jump`: Clears the byte at a memory location calculated by adding a 16-bit offset from d1 to a base address from a table, then jumps to another address from that table.
- `jump_with_masked_d2`: Masks the lower 8 bits of d2 and moves the result to d4, then jumps to an address retrieved from a table.
- `dispatch_with_data_transfer`: Calculates a pointer by adding a word from a table to a long value in d1, stores two bytes from that pointer into a structure at offsets 2 and 3 using a6, and then jumps to a second address retrieved from the table.
- `jump_with_masked_word`: Masks the lower byte of a word from a table, moves it to d4, and jumps to an address retrieved from the next word in the table.
- `jump_with_masked_byte`: Masks the lower byte of a word from a table, moves the result to d4, and jumps to an address retrieved from the next word in the table.
- `jump_with_byte_transfer`: Loads a byte from the address pointed to by a word in a table into d4, then jumps to a second address retrieved from the next word in the table.
- `jump_with_byte_transfer_d0`: Loads a byte from the address pointed to by a word in a table into d0 and d4, then jumps to a second address retrieved from the next word in the table.
- `jump_with_word_transfer`: Loads a word from the address pointed to by a word in a table into d4, then jumps to a second address retrieved from the next word in the table.
- `dispatch_with_indexed_byte`: Loads an address from a table, stores two bytes from that address into a structure at offsets 2 and 3 using a6, loads a byte from that address using d2 as an index into d0 and d4, and then jumps to a second address retrieved from the table.
- `jump_via_table_pointer`: Loads a pointer from a table and jumps to it.
- `jump_with_offset_d2`: Calculates a pointer by adding a word from a table to d2, loads a byte from that pointer into d4, and then jumps to a second address retrieved from the next word in the table.
- `jump_with_offset_d1`: Calculates a pointer by adding a word from a table to d1, loads a byte from that pointer into d4, and then jumps to a second address retrieved from the next word in the table.
- `dispatch_via_d1`: Adds a 16-bit value from a table to the contents of d1, loads a pointer from the resulting address, and jumps to the address stored at that pointer.
- `dispatch_via_d2`: Adds a 16-bit value from a table to the contents of d2, loads a pointer from the resulting address, and jumps to the address stored at that pointer.
- `dispatch_with_subtraction`: Subtracts the lower byte of a 16-bit value from a table from d2, performs a bitwise negation on the result, and jumps to a pointer loaded from the table.
- `dispatch_and_store`: Calculates a pointer using d1 and a table, stores two bytes from that pointer into a structure in a6, and then jumps to a pointer loaded from the table after performing a subtraction and bitwise negation.
- `increment_d2_and_jump`: Increments d2 by one, copies it to d4, and jumps to a pointer loaded from a table.
- `decrement_d2_and_jump`: Decrements d2 by one, copies it to d4, and jumps to a pointer loaded from a table.
- `increment_d1_and_jump`: Increments d1 by one, copies it to d4, and jumps to a pointer loaded from a table.
- `decrement_d1_and_jump`: Decrements d1 by one, copies it to d4, and jumps to a pointer loaded from a table.
- `increment_d0_and_jump`: Increments d0 by one, copies the byte to d4, and jumps to a pointer loaded from a table.
- `dispatch_with_d0_subtraction`: Subtracts the lower byte of a 16-bit value from a table from d0, performs a bitwise negation on the result, and jumps to a pointer loaded from the table.
- `dispatch_via_pointer_table`: Loads a pointer from a table in a5, subtracts a byte from the value at that pointer using d2, and jumps to a second pointer loaded from the table.
- `dispatch_via_byte_subtraction`: Loads a pointer from a5, subtracts a byte from the value at that pointer using d0, and jumps to a second pointer loaded from the table.
- `decrement_and_jump`: Loads a pointer from a5, decrements the byte at that location, and jumps to a second pointer loaded from the table.
- `store_bytes_and_dispatch`: Loads a pointer from a5, stores two bytes from that location into a structure in a6, and jumps to a second pointer loaded from the table using a value calculated from d0 and a table entry.
- `dispatch_with_offset_d2`: Adds a word from a5 to d2 to form a pointer, subtracts a byte from the value at that pointer using d0, and jumps to a second pointer loaded from the table.
- `dispatch_with_offset_d1`: Adds a word from a5 to d1 to form a pointer, subtracts a byte from the value at that pointer using d0, and jumps to a second pointer loaded from the table.
- `decrement_offset_pointer_and_jump`: Adds a word from a5 to d1 to form a pointer, decrements the byte at that location, and jumps to a second pointer loaded from the table.
- `dispatch_with_masked_d1`: Subtracts the lower byte of a word from d1 from d4, then jumps to a second pointer loaded from the table in a5.
- `store_bytes_with_subtraction_dispatch`: Adds a word from a5 to d1 to form a pointer, stores two bytes from that location into a structure in a6, and jumps to a second pointer loaded from the table using a value calculated via subtraction with borrow.
- `dispatch_with_masked_a5_byte`: Loads a word from a5, masks it to a byte, subtracts that byte from d0 using subtraction with borrow, and jumps to a second pointer loaded from the table.
- `execute_subtraction_jump`: Subtracts a byte from a value at a pointer in a0, performs bitwise operations on d3, and jumps to a new address loaded from a5.
- `execute_subtraction_with_carry_jump`: Subtracts a byte from d0 with carry from a bitwise operation on d3, then jumps to a new address loaded from a5.
- `increment_and_jump`: Increments a byte at a pointer in a0 and then jumps to a new address loaded from a5.
- `store_bytes_and_jump`: Stores two bytes from a pointer in a0 into a structure at a6 offset 2 and 3, then jumps to a new address loaded from a5.
- `execute_offset_subtraction_jump`: Adds d1 to a word from a5 to form a pointer, subtracts a byte at that pointer from d0 using bitwise operations on d3, and jumps to a new address loaded from a5.
- `increment_offset_and_jump`: Adds d1 to a word from a5 to form a pointer, increments the byte at that pointer, and jumps to a new address loaded from a5.
- `double_and_jump`: Adds d2 to itself and jumps to a new address loaded from a5.
- `copy_byte_and_jump`: Copies a byte from a pointer in a0 to a destination pointer in a1, then copies a second byte from offset 1 of the source to offset 1 of the destination, and jumps to a new address loaded from a5.
- `copy_two_bytes_and_jump`: Copies two consecutive bytes from a source pointer to a destination pointer and jumps to a new address loaded from a5.
- `negate_and_jump`: If d0 is negative, negates it, then jumps to a new address loaded from a5; otherwise it jumps to a new address loaded from a5.
- `copy_and_execute_data`: Copies three bytes from a source pointed to by a5 to a destination pointed to by a1, stores a byte from the source into d4 and d0, copies that byte to the destination, then loads a new address from a5 and jumps to it.
- `call_offset_14adc`: Calls a routine at an offset of 85948 from a base pointer in a4 and then jumps to an address stored at offset 2406 of that same base pointer.
- `call_offset_14afa`: Calls a routine at an offset of 86170 from a base pointer in a4 and then jumps to an address stored at offset 2414 of that same base pointer.
- `fetch_indexed_byte`: Calculates an address by adding a 16-bit value from a table at offset 98304 of a4 to a base pointer, rotates that value, and then loads a byte from the resulting address into d0.
- `call_offset_14634`: Calls a routine at an offset of 83508 from a base pointer in a4 and then jumps to an address stored at offset 2470 of that same base pointer.
- `write_word_to_offset`: Writes the word value 96 to an offset of 5778 in a4 and then jumps to an address stored at offset 2470 of that same base pointer.
- `call_offset_14b16`: Calls a routine at an offset of 84758 from a base pointer in a4 and then jumps to an address stored at offset 2456 of that same base pointer.
- `call_offset_16018`: Calls a routine at an offset of 90136 from a base pointer in a4 and then jumps to an address stored at offset 2494 of that same base pointer.
- `call_offset_1461c`: Calls a routine at an offset of 83484 from a base pointer in a4 and then jumps to an address stored at offset 2460 of that same base pointer.
- `call_offset_14be4`: Calls a routine at an offset of 84964 from a base pointer in a4 and then jumps to an address stored at offset 2482 of that same base pointer.
- `call_offset_1607c`: Calls a routine at an offset of 90236 from a base pointer in a4 and then jumps to an address stored at offset 2568 of that same base pointer.
- `call_offset_146e6`: Calls a routine at an offset of 83654 from a base pointer in a4, loads an address from offset 6260 of the base pointer into a5, and performs an indirect jump using an address at offset 1542 of the base pointer.
- `call_offset_146e6_alt`: Calls a routine at an offset of 83654 from a base pointer in a4, loads an address from offset 6880 of the base pointer into a5, and performs an indirect jump using an address at offset 1542 of the base pointer.
- `call_offset_14988`: Calls a routine at an offset of 84360 from a base pointer in a4, loads an address from offset 6902 of the base pointer into a5, and performs an indirect jump using an address at offset 1542 of the base pointer.
- `call_offset_15edc`: Calls a routine at an offset of 89820 from a base pointer in a4, loads an address from offset 7000 of the base pointer into a5, and performs an indirect jump using an address at offset 1542 of the base pointer.
- `call_offset_15ec6`: Calls a routine at an offset of 89798 from a base pointer in a4, loads an address from offset 6990 of the base pointer into a5, and performs an indirect jump using an address at offset 1542 of the base pointer.
- `call_offset_16130`: Calls a routine at an offset of 90416 from a base pointer in a4, loads an address from offset 7056 of the base pointer into a5, and performs an indirect jump using an address at offset 1542 of the base pointer.
- `call_offset_16130_alt`: Calls a routine at an offset of 90416 from a base pointer in a4, loads an address from offset 7078 of the base pointer into a5, and performs an indirect jump using an address at offset 1542 of the base pointer.
- `clear_and_set_table`: Loops to clear 7 words starting at offset 6832 of the base pointer in a4 using a decrementing counter, writes 1 to offset 6690 and 7 to offset 6692 of the base pointer, then performs an indirect jump using an address at offset 6886 of the base pointer.
- `call_offset_160ca`: Calls a routine at an offset of 90282 from a base pointer in a4, loads an address from offset 7166 of the base pointer into a5, and performs an indirect jump using an address at offset 1542 of the base pointer.
- `rotate_and_store_offset_1378`: Calculates the difference between a longword at offset 6886 and the base pointer in a4, performs two byte-swapping operations on this value, stores the result in a global variable at offset 5000, and performs an indirect jump using an address at offset 7208 of the base pointer.
- `process_and_check_value`: Calls a routine using a pointer at an offset from a4, then jumps to another location using a pointer at a different offset from a4.
- `decrement_and_jump_if_zero`: Decrements a 16-bit word at offset 18d8 of the structure in a4 and, if the result is zero, calls a routine through a pointer at offset 606 of a4; otherwise it returns.
- `update_and_jump_2`: Calls a routine at an offset from a pointer in a4, copies a longword from offset 2c of that pointer to offset 30, and then jumps to an address stored at offset 606 of the pointer.
- `call_offset_14d50`: Calls a routine at offset 14d50 relative to a4 and then jumps to a routine through a pointer at offset 606 of a4.
- `call_offset_14832`: Calls a routine at offset 14832 relative to a4 and then jumps to a routine through a pointer at offset 606 of a4.
- `update_table_indices`: Calculates an address using values from offsets 132 and 134 of a4 and a pointer at offset 5016 of a4, reads two 16-bit words from that location, performs bitwise rotations and shifts on them, and stores the results back into offsets 132 and 134 of a4 before jumping to a routine through a pointer at offset 1542 of a4.
- `load_byte_and_jump`: Loads a byte from offset 1894 of a4 into d0 and then jumps to a routine through a pointer at offset 606 of a4.
- `check_ciaa_bit`: Checks if bit 7 is set in the CIAA_PRA register and then jumps to a routine through a pointer at offset 606 of a4.
- `call_offset_147e0`: Calls a routine at offset 147e0 relative to a4 and then jumps to a routine through a pointer at offset 606 of a4.
- `call_offset_16f54`: Calls a routine at offset 16f54 relative to a4 and then jumps to a routine through a pointer at offset 606 of a4.
- `call_offset_351c`: Calls a routine at offset 351c relative to a4 and then jumps to a routine through a pointer at offset 606 of a4.
- `call_offset_1661e`: Calls a routine at offset 1661e relative to a4 and then jumps to a routine through a pointer at offset 606 of a4.
- `call_offset_16a2a`: Calls a routine at an offset of 16a2a from a base pointer in a4, then performs an indirect jump using an offset of 606 from that same base pointer.
- `call_and_clear_byte`: Calls a routine at offset 34e0 from a4, clears a byte at offset 2124 from a4, calls another routine at offset 16a1c from a4, and performs an indirect jump using an offset of 606 from a4.
- `call_offset_15f3e`: Calls a routine at an offset of 15f3e from a4, then performs an indirect jump using an offset of 606 from a4.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_offset_1615e`: Calls a routine at an offset of 1615e from a4, then performs an indirect jump using an offset of 606 from a4.
- `call_offset_16460`: Calls a routine at an offset of 16460 from a4, then performs an indirect jump using an offset of 606 from a4.
- `call_offset_16478`: Calls a routine at an offset of 16478 from a4, then performs an indirect jump using an offset of 606 from a4.
- `load_table_entry`: Loads a 32-bit word from a table at offset 2892 from a4, using an index in d0 scaled by 4, and stores it at offset 2c from a4, then performs an indirect jump using an offset of 606 from a4.
- `clear_table_enable_interrupts`: Calls another routine, clears 128 bytes of a table at an offset from a4 using a loop, fills five longword slots starting at a pointer in a2 with addresses derived from a4, and writes a value to the interrupt enable register to enable interrupts.
- `clear_matching_table_entries`: Stores the low byte of d1 in a base-relative field at offset 13592 from a4, then clears each word entry of a 12-entry table at offset 12842 from a4 that either equals that byte or exceeds 254.
- `update_dynamic_pointer_list`: Performs a byte-swapping operation on a pointer stored at offset 40 of a structure, then enters a loop that iterates through a list of data, updating various fields in a structure at offset 22 of the base pointer using values from the swapped pointer and incrementing a counter.
- `process_dynamic_pointer_list`: Iterates through a list of data structures in a loop, performing byte-swapping on 16-bit values read from a pointer at offset 48 of a structure and storing them at specific offsets within the current structure.
- `update_accumulator_records`: Loops over a list of 16-bit accumulator records, advancing each accumulator with carry and comparing it against limits.
- `extend_and_store_byte`: Extends a byte from offset 30f8 of a4 to a word and stores it at offset 32ea from a0.
- `copy_byte_to_offset`: Copies a byte from offset 30f8 of a4 to offset 325a of a0 and returns.
- `keep_smaller_word_pair`: Compares a pair of byte-swapped words taken from a table with a pair stored in a record, and replaces the record's pair when the table pair is smaller.
- `shift_byte_from_record`: Calls another routine, then loads a byte from an offset of 12552 in a6, shifts it left by one, and returns.
- `dispatch_channel_event`: Stores a pointer in a base-relative field and calls three other routines.
- `divide_difference_into_fields`: Computes the 32-bit difference between a byte-swapped value from a table and a stored value, divides it by a 16-bit divisor from a record, and stores the quotient and a remainder-derived value in the record's fields.
- `initialise_step_counters`: Copies two 16-bit values from a record into a base-relative field, clears a counter, and calls another routine.
- `clear_matching_count_byte`: Compares a byte derived from a 16-bit input with a counter byte in a base-relative table, and clears the counter byte when they match.
- `store_halved_count_when_flagged`: Returns the masked value of a flag bit.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_and_increment_byte`: Stores a pointer into a table at an offset determined by a value in a register, increments a byte in another table, and then loads a 16-bit value from a third table to store into the pointer's destination before calling another routine.
- `copy_stack_to_table`: Compares a pointer from a table with a calculated address and, if they differ, copies words from the stack to a table location in a loop before calling another routine.
- `call_via_swapped_bytes`: Loads a 16-bit value by combining two bytes from a table, checks if it is at least an address, and if so, pushes it to the stack and calls another routine.
- `call_via_swapped_bytes_direct`: Loads a 16-bit value by combining two bytes from a table, pushes it to the stack, and calls another routine.
- `copy_stack_to_table_and_store`: Copies words from the stack to a table location in a loop, stores the resulting pointer in a table, and calls another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_table_and_update_pointer`: Iterates through a table using byte-swapped offsets, updates two bytes in a record with values from the table, and if a specific byte is encountered, updates a global pointer and calls another routine.
- `update_and_copy_byte_table`: Calculates a new pointer for a table in a structure using a word from a structure, updates that pointer, and then copies 16 bytes from the new table location to a destination table at an offset of 57376 from a base pointer using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_and_call_routine`: Clears a 16-bit register and calls another routine before returning.
- `process_matching_record`: Copies a string from a table to a destination, calls another routine to find a matching record, and updates several offsets in a structure based on the result.
- `call_routine_return_input`: Calls another routine and returns its input value.
- `find_prefix_matching_record`: Walks a table of 18-byte records, comparing each record's text with a zero-terminated input string.
- `save_restore_hardware_state`: Saves the current DMA and interrupt state by calling another routine, calls a third routine, calls a fourth routine, and then calls a fifth routine to restore the hardware state before returning.
- `wait_dma_idle_and_enable`: Waits until a status bit in the DMA control register is clear, then clears all interrupt enable bits and pending requests, writes a DMA control value, and calls another routine.
- `save_dma_and_interrupt_state`: Saves the current DMA control and interrupt enable register values into shared globals.
- `restore_dma_and_interrupts`: Writes the saved DMA control and interrupt enable values back to the hardware with the top bit set, which enables those channels and sources.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `load_address_and_branch`: Loads a value into a register and branches to another routine.
- `load_address_and_branch_2`: Loads a constant value into a register using the program counter and performs a short branch to another location.
- `load_address_and_branch_3`: Loads a constant value into a register using the program counter and performs a short branch to another routine.
- `load_address_and_branch_4`: Loads the address of a constant into a register and performs a short branch to another location.
- `update_sprite_attribute_tables_2`: Calls another routine, writes to three offsets in a structure, and then performs a series of conditional calls based on the value of a register.
- `update_sprite_attribute_tables`: Rotates a 16-bit value in a register, stores it into a structure at offset 66 and 64 with a bitwise OR, and then performs a series of calls to another routine to update multiple entries in a table by incrementing pointers and storing various register values at specific offsets.
- `wait_dma_status_bit`: Waits until bit 6 of a byte at an offset of 2 from a pointer in a0 is set.
- `execute_sequential_callback_chain`: Calls initialize_system_memory_and_hardware, call_three_subroutines, execute_callback_with_flag, and empty_stub_9 in sequence, then performs an indirect jump using a pointer from a table offset by a value in a4.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `delay_loop_by_count`: Busy-waits for a number of passes equal to its byte input, each pass counting down through a long inner loop.
- `delay_loop_short`: Busy-waits for a number of passes equal to its byte input, each pass counting down through a slightly shorter inner loop.
- `copy_nonempty_table_words`: Reads sixteen byte-swapped words from a table reached through a base-relative pointer and stores each word that is not the all-ones sentinel into a shared array at a four-byte stride.
- `initialise_bitplane_copper_registers`: Initialises the bitplane and copper hardware registers, sets DMA with the top bit set, and fills four pointer entries with addresses spaced 8,000 bytes apart.
- `clear_long_block_at_pointer`: Zeroes 8,000 longwords starting at the address held in a base-relative field.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `call_three_subroutines`: Calls three different subroutines in sequence and then returns.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `read_joystick_mapped_byte`: Processes the joystick data register through bitwise operations and uses the result to index into a table to return a byte.
- `update_structure_pointers_and_copy_data`: Calculates several new pointer values based on a structure in a4 and the result of a routine call, stores them in the structure, and then enters a loop that copies and transforms words from two source locations into two destination locations.
- `copy_and_call_routine`: Copies a block of bytes from a source pointed to by a value in a structure to a destination offset in that structure, updates the structure with a new value and a calculated offset, and then calls another routine.
- `increment_and_set_values`: Adds 16 to a 16-bit value at offset 6834 of a structure and sets the 16-bit value at offset 6834 to 8.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `store_table_values`: Reads a value from a table using a register and an index, then stores that value into two consecutive memory locations.
- `calculate_swapped_pointer`: Calculates an address by performing a byte swap on a value from a table and adding it to a base address and an offset.
- `store_shifted_values`: Stores high bytes from two memory locations into a destination table, applying a bitwise OR if a value exceeds a threshold.
- `decompose_integer_recursively`: Performs a recursive digit decomposition of a value by calculating quotients and remainders to process individual digits.
- `parse_compressed_instruction_stream`: Reads a byte-swapped index from a table to determine a command, then updates a local offset based on the command type.
- `update_entity_state_and_dispatch`: Updates a table of values by iterating through a list and copying data from a secondary table, then calculates a new index based on two structure members to access a third table and calls a routine via an indirect jump.
- `transfer_table_values`: Loads a 16-bit word from a table using a pointer in a4 and an index in d2, then uses that value to fetch a second word from another table and stores it into a third table at the same index.
- `calculate_and_store_offset`: Fetches a 16-bit word from a table, uses it to index into a second table with a bit-shifted offset to calculate a new 32-bit value, and stores a word from a third table into a fourth table at the index specified by the original word.
- `update_entity_state_and_dispatch_2`: Copies data from a secondary table into a primary table based on an index, and then either iterates through a list to update values or calls a function pointer from a table.
- `branch_to_offset`: Moves a word from d6 to d1 and branches to another routine.
- `update_entity_offset_table`: Writes four 16-bit values to a table at offsets 0, 2, 4, and 6 relative to a base address, where each value is derived from a register by adding or subtracting 2 or 40.
- `update_entity_attribute_offsets`: Updates three 16-bit values in a table relative to a base pointer using offsets derived from a register, then performs a loop that modifies values in a secondary table based on conditions and calls another routine.
- `update_entity_offset_table_2`: Writes three modified values derived from a register into a table at specific offsets relative to a base address and an index.
- `update_entity_state_and_dispatch_3`: Stores three 16-bit values into a table at offsets 0, 2, and 4 relative to a base address calculated from a pointer in a6 and a constant, then branches to another routine.
- `update_entity_state_and_dispatch_4`: Stores two 16-bit values into a table at offsets 0 and 2 relative to a base address calculated from a pointer in a6 and a constant, then branches to another routine.
- `update_entity_state_and_dispatch_5`: Writes two 16-bit values to a table at offsets an address and an address relative to a base pointer, where the first value is a register plus 40 and the second is the same register minus 2.
- `update_entity_state_and_dispatch_6`: Stores two 16-bit values into a table at offsets 2 and -26 relative to a base address calculated from a pointer in a6 and a constant, then branches to another routine.
- `update_entity_state_and_dispatch_7`: Stores two 16-bit values into a table at offsets 2 and 40 relative to a base address calculated from a pointer in a6 and a constant, then branches to another routine.
- `update_dma_parameters`: Updates several global variables with incremented values and modifies a DMA controller register.
- `update_table_indices_3`: Saves registers to the stack, performs bitwise operations on values from a table pointed to by a4 to calculate and store new values at various offsets in that same table, calls eight different routines, and then restores the registers before returning.
- `update_object_attribute_tables`: Initializes several fields in a structure using values from a table and performs a series of calculations to update coordinate or attribute offsets.
- `sort_and_swap_tables`: Performs a bubble sort on a table of 16-bit values and swaps corresponding values in another table based on comparison results.
- `update_entity_indices_and_pointers`: Iterates through a table of data, performing bounds checks and arithmetic on various offsets to validate entries.
- `populate_descending_table`: Fills a table with descending 16-bit values and a second table with bit-manipulated values in a loop that runs eight times.
- `iterate_nested_table_pointers`: Performs a nested loop structure that iterates through a table to retrieve indices, uses those indices to look up values in two separate tables, and calls one of three different routines based on the resulting values.
- `execute_indirect_jump_2`: Performs an indirect jump to an address stored at an offset in a structure.
- `update_and_copy_attribute_tables`: Updates two counters in a global structure and then performs a nested loop to copy data from three source tables into three destination buffers, optionally calling another routine to process specific values during the copy.
- `calculate_offset_from_range`: Compares a register against several constants and, depending on the result, adds or subtracts values from a global variable and returns.
- `swap_two_words`: Performs a byte swap on two 16-bit words located at offsets 84 and 86 of the structure in a4 and stores the results at offsets 16ba and 16bc of the same structure.
- `rotate_and_store_words`: Performs a bitwise rotation and shift on a 16-bit word from offset 137c of the structure in a4, stores the result at offset 84, and copies a word from offset 137e to offset 86.
- `perform_bit_extraction`: Iterates through a source table to extract specific bit fields from 32-bit integers and stores them into a large destination buffer.
- `copy_memory_block_2`: Performs a series of conditional calls to another routine and then copies a block of 59 words from one memory location to another using a loop.
- `initialize_data_structure`: Sets several fields in a structure, including byte-swapped values derived from a table, and calls another routine.
- `clear_memory_buffer`: Sets two specific values in a table and then fills a large memory area with zeros using nested loops.
- `copy_memory_block`: Calculates a size from a global variable, performs two function calls, and then copies a block of memory from one location to another using a loop.
- `reset_flag_byte`: Reads a byte from a specific location, stores it in the return value, and then clears that byte to zero.
- `wait_for_byte`: Polls a memory location until the byte is non-zero, then clears it and returns the value.
- `store_swapped_bytes`: Calculates a destination address using a base, an offset, and a byte-swapped value, then stores high bytes from two other locations there.
- `update_offset_values`: Calls another routine, stores a 16-bit value into a structure, and then calls another routine before returning a masked value.
- `update_base_pointer`: Fetches a 32-bit value from a table using an index and stores a byte-swapped version of it into a global variable.
- `update_table_indices_2`: Extracts byte-swapped values from a table and uses them to update several offset and pointer variables.
- `decompress_bitstream_data`: Reads a compressed data stream from a table and expands it into a destination buffer using bitwise shifts and masking.
- `clear_memory_block`: Sets a byte to zero at a specific offset and then fills a subsequent block of memory with zeros using a loop.
- `compare_and_branch`: Saves registers, copies a 16-bit value from a global variable into d1 and d5, and branches if d1 is not equal to 64.
- `process_table_data`: Reads bytes from a table, performs arithmetic and bitwise operations on them, and writes the results into several different memory locations.
- `execute_callback_with_flag`: Sets a flag in a global variable, calls another routine, resets the flag, and then executes a function pointer stored in a table.
- `update_structure_counters_2`: Calls several routines to process data, then enters a loop that copies values from a table into a structure based on a counter.
- `update_structure_counters`: Stores a 32-bit constant into a local variable, calls another routine, compares a value in a0 to a local variable, and then conditionally increments several bytes in a structure in a4.
- `tiny_subq_subq`: Small routine of 2 instructions that subtracts 1 from a5, then subtracts 2 from (a3).
- `calculate_pointer_offset_4`: Iterates through a list of pointers, performs bitwise rotations and swaps on calculated offsets, and stores the results in a structure.
- `calculate_pointer_offset`: Loads a 32-bit value from an offset of a structure in a6, performs bitwise rotations and a swap, adds a6 to the result, and stores the final address in a0.
- `calculate_pointer_offset_2`: Loads a 32-bit value from a different offset of a structure in a6, performs bitwise rotations and a swap, adds a6 to the result, and stores the final address in a0.
- `calculate_pointer_offset_3`: Loads a 32-bit value from another offset of a structure in a6, performs bitwise rotations and a swap, adds a6 to the result, and stores the final address in a0.
- `update_system_state`: Increments a global variable, conditionally increments a byte in a table and calls another routine, and then enters a loop that calls four different routines before updating a hardware register and returning via an interrupt return.
- `process_keyboard_input`: Reads a value from a keyboard serial register, performs bitwise transformations, and updates a table or a global variable based on the resulting character code.
- `enable_dma_channels`: Writes the value fifteen to the DMA control register at offset 96 of the hardware base address.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `configure_audio_parameters`: Initializes multiple global variables and memory locations, likely corresponding to audio channel settings, and then calls another routine.
- `update_indexed_values_2`: Clears three registers, loads three bytes from a table into those registers, and calls another routine.
- `fetch_three_bytes`: Uses a byte from d7 as an index to fetch three consecutive bytes from three different tables relative to a6 and stores them in d7, d6, and d5.
- `update_indexed_values`: Calls another routine, fetches three bytes from tables using an index, and stores them into a local buffer after performing conditional increments.
- `update_lookup_table`: Iterates through a series of values to perform lookups in a table and stores the resulting bytes into a global array.
- `decrement_table_values`: Iterates through a small loop to decrement a byte and increment another byte at specific offsets in a table.
- `delay_loop`: Executes a loop that counts down from six hundred to zero.
- `update_audio_channels`: Updates the hardware registers for multiple audio channels by calculating new period and volume values from a data table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_bitmask_patterns_iteratively`: Sets up three pairs of pointers and a loop counter, then repeatedly calls apply_bitmask_patterns while incrementing the pointer offsets by 640 bytes after each call until the loop counter reaches zero.
- `apply_bitmask_patterns`: Performs nested loops to test bits in a table and, if a bit is set, performs bitwise operations to update a destination buffer and the color register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_audio_channels_and_music`: Depending on a control value in d0, this routine either initializes a channel structure by setting its length and volume, updates a music sequence using a linear congruential generator to populate a table, or performs a full music system reset by clearing channel structures, loading new sequence data, and configuring the Amiga audio hardware registers.
- `initialize_channel_structure`: Clears several fields of a channel structure, stores a pointer taken from a second structure, and sets a fixed longword, a mask byte and a flag byte.
- `update_music_sequence_and_channels`: Generates sixteen pseudo-random values using a linear congruential generator and stores them in a table, then decrements a counter and calls another routine multiple times to update four independent audio channels based on their respective remaining event counts.
- `step_music_channel_sequence`: Reads the next sequence event for one channel, loading its instrument and state from data tables, and updates the channel's pitch and volume counters.
- `apply_bitwise_or_constants`: Performs a series of bitwise OR operations on the d0 register using eight different immediate byte values.
- `mask_and_store_byte`: Masks the low nibble of a byte in d1 and stores the result in a byte at an offset of 47 from a6.
