# Game 1987 (micro machines (1992)(codemasters)[cr flt][t +6 cherry][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01987). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903616 top-level bytes in 28 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 17 of 17 units identical (bytes and relocations), 154 raw-word lines |
| Functions named and described | pass, 109 of 109 named (0 author symbols), 109 of 109 described (9 by rule, 99 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 445 of 445 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2362 of 6330 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 25131 |
| code | 23180 |
| data | 777026 |
| padding | 4551 |
| unused | 73728 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:GAM | 1 | 0 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:ChErRy | 1 | 5236 | 50 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:FLTINTRO | 2 | 267 | 8 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:FLTBOOT | 1 | 341 | 19 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV1#unpacked | 1 | 18 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV2#unpacked | 1 | 18 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV3#unpacked | 1 | 21 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV4#unpacked | 1 | 22 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV5#unpacked | 1 | 21 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV6#unpacked | 1 | 19 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV7#unpacked | 1 | 20 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV8#unpacked | 1 | 22 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV9#unpacked | 1 | 19 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:FRT | 1 | 0 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:MID | 1 | 0 | 1 |
| Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 288 | 19 |

## Cases handled

- micro machines (1992)(codemasters)[cr flt][t +6 cherry][a]/Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV1: IMP: File Imploder-packed, unpacked with ancient (86532 -> 103120 bytes)
- micro machines (1992)(codemasters)[cr flt][t +6 cherry][a]/Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV2: IMP: File Imploder-packed, unpacked with ancient (60366 -> 77124 bytes)
- micro machines (1992)(codemasters)[cr flt][t +6 cherry][a]/Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV3: IMP: File Imploder-packed, unpacked with ancient (51974 -> 64272 bytes)
- micro machines (1992)(codemasters)[cr flt][t +6 cherry][a]/Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV4: IMP: File Imploder-packed, unpacked with ancient (76090 -> 88296 bytes)
- micro machines (1992)(codemasters)[cr flt][t +6 cherry][a]/Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV5: IMP: File Imploder-packed, unpacked with ancient (39336 -> 50772 bytes)
- micro machines (1992)(codemasters)[cr flt][t +6 cherry][a]/Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV6: IMP: File Imploder-packed, unpacked with ancient (65154 -> 75932 bytes)
- micro machines (1992)(codemasters)[cr flt][t +6 cherry][a]/Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV7: IMP: File Imploder-packed, unpacked with ancient (43110 -> 54964 bytes)
- micro machines (1992)(codemasters)[cr flt][t +6 cherry][a]/Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV8: IMP: File Imploder-packed, unpacked with ancient (49834 -> 62796 bytes)
- micro machines (1992)(codemasters)[cr flt][t +6 cherry][a]/Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV9: IMP: File Imploder-packed, unpacked with ancient (67428 -> 80716 bytes)
- runtime segment an address: 174 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:FLTINTRO

## Copy protection found

- `copy_and_jump_table` (FLTBOOT): Calls another routine, disables interrupts, writes values to custom hardware registers, and copies 256 words from a routine to a table at 32768 before jumping to an address in that table. Kind: loader/jump table relocation. Pass path: the loop completes and execution continues to u16__sub_082C.
- `initialize_hardware_and_jump` (FLTBOOT): Sets up several pointers in globals, calls another routine, performs a trap, modifies bits in a memory location, and jumps to an address stored in a table. Kind: loader/protection entry point. Pass path: the sequence of calls to u16__sub_0880, u16__sub_07E0, u16__sub_0806 following the trap.
- `initialize_hardware_and_wait` (runtime segment an address): Configures various hardware registers including DMA, interrupts, and custom registers, then enters a loop checking hardware status and global flags. Kind: disk hardware/sync check. Pass path: the loop continues until the disk status/sync condition is met, eventually exiting the loop to proceed with the main program execution.

## Functions


### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:GAM

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:ChErRy

- `initialize_system_and_check_hardware`: Clears a block of memory, calls two subroutines, and initializes a global structure with a pointer and a zero value.
- `copy_memory_block`: Calls two routines, then copies a sequence of bytes from a source location in a0 to a destination location in a2 using a loop.
- `update_state_variables`: Checks a byte at offset 17 and, if non-zero, updates various values at specific memory offsets based on the value found at offset 6d00, finally performing a jump to an address in a table.
- `fill_zero_data`: Contains five identical instructions that perform a bitwise OR on register d0 with zero.
- `setup_system_environment`: Initializes system libraries and hardware registers while configuring task priorities and screen displays.
- `initialize_system_resources`: Sets several word and long values in hardware registers, calls multiple library routines to manage tasks and screens, and copies a block of data from a source to a destination pointed to by a1.
- `initialize_data_block`: Stores several byte values into various offsets of a memory block pointed to by a register.
- `test_bits_and_return`: Saves registers to the stack, checks bits 6 and 5 of a word at offset 28 of a structure in a5, calls another routine if bit 5 is set and bit 6 is clear, writes 32 to offset 154 of the structure, and returns via an exception return.
- `restore_context_and_return`: Tests bit 4 of d0; if the bit is not set, it restores d0 through d7 and a0 from the stack and returns via an exception return; otherwise it returns immediately.
- `set_control_word_small`: Stores the word value 16 into a memory location at offset 154 of a structure in a5 and returns via an exception return.
- `set_control_word_large`: Stores the word value 64 into a memory location at offset 154 of a structure in a5 and returns via an exception return.
- `update_hardware_registers`: Writes specific values to two global variables, which correspond to hardware control registers.
- `read_hardware_mask`: Performs a bitwise AND on two hardware register values and stores the result in a global variable.
- `copy_data_loop`: Iteratively stores 32-bit values into a destination buffer by splitting them into two 16-bit parts across 8-byte increments.
- `execute_conditional_function`: Checks a bit in a global variable and, if set, calls a function pointer stored in another global variable.
- `scan_flag_bits_into_field`: Counts a value down by ten for each bit of a flag byte in global memory that it tests, stopping at the first bit that is set or when the counter reaches a fixed limit.
- `decrement_and_test`: Tests a bit in a global variable; if set, it moves a value from d1 to d0 and decrements a 32-bit counter in d2 before returning, otherwise it returns.
- `countdown_write_cia_registers`: Runs a countdown loop that writes constant values to CIA chip registers on each pass.
- `initialise_music_player`: Checks a four-byte module tag in a data block and returns if it matches neither of two known tags; otherwise it copies per-channel sample records into global tables, converts packed volume fields, primes the audio channel volume and DMA registers, and enables the playback interrupt.
- `toggle_joystick_fire_interrupt`: Enables interrupts by setting bit 15 of the hardware custom register and checks bit 1 of a value loaded from a PC-relative address; if that bit is not set, it sets bit 1 of a CIA register via a pointer in a5, otherwise it returns.
- `clear_cia_register_bit`: Clears bit 1 of a CIA register and returns.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into -(a0).
- `increment_and_clamp_value`: Adds a word from a memory location to a value at a pointer in a0, then clamps that value to 32 if it is greater than 32 or less than 0.
- `write_custom_registers`: Writes 32 and 0 to offsets 0 and 2 of a structure in a0, loads the address of a custom hardware register into a5, and branches to another routine if a value at a calculated offset is zero.
- `compare_and_call`: Compares a byte at offset 3 of a structure in a2 to 1, and if it is not less than 1, calls another routine.
- `update_entity_attributes`: Iterates through a table of four data entries, calling a subroutine for each to update specific values in a structure and performing a scaled multiplication to store a result at an offset in that structure.
- `update_structure_counters`: Decrements a 16-bit value in a structure, increments a global counter, and performs arithmetic on a pointer-based value within the structure based on a comparison with a global variable.
- `write_globals_and_table`: The routine writes constants and a value from a global variable into global variables, adds a constant to a word in a table pointed to by a pointer in memory, and touches hardware registers CIAB_CRA, CIAB_ICR, DMACON and INTREQ.
- `write_globals_and_table_2`: The routine writes values from several global variables into global variables, writes a constant into a global variable, stores a value from a global variable into a table entry, and touches hardware registers AUD0LCH, AUD0LEN, AUD1LCH, AUD1LEN, AUD2LCH, AUD2LEN, AUD3LCH, AUD3LEN, CIAB_ICR, INTREQ and custom+0.
- `push_data_to_stack`: Pushes several bytes onto the stack, clears a 16-bit value at offset 532 of a structure in a2, and if a condition is met, updates a value at offset 4 of a4 and copies several longwords and a word from a table into a structure in a4.
- `update_structure_from_table`: Ors a word from d7 into a memory location at offset 788 of a2, and if a byte at offset 532 of a2 is zero, it calculates an offset using d0 to copy a word from offset 6 of a table to offset 12 of a4, a longword from offset 8 of the table to offset 6 of a4, and a word from offset 14 of the table to offset 10 of a4.
- `fetch_table_data`: Stores a word from d7 into a structure, calculates an offset based on d0, and then copies several values from a table into a structure pointed to by a4.
- `fetch_table_entry`: Performs several operations including bitwise OR on a register and a memory location, then calculates an offset using d0 to fetch a longword and a word from a table in a0, storing them in a structure in a4 before jumping to another routine.
- `copy_table_entries`: If a byte in d0 is non-zero, it stores that byte into a structure in a4, and if d1 is zero, it calculates an offset to fetch data from a table to populate several fields in a structure in a4.
- `process_table_data_and_store`: Ors a word from d7 into offset 790 of a2 and stores a word from d7 into offset 150 of a5, then uses a byte from offset 5 of a4 to calculate a value in d6, uses d0 to find an offset in a0 to fetch a longword and a word, and if the word is less than d6, it performs arithmetic to store a longword and a word into a structure pointed to by a3 and updates several fields in a4.
- `copy_and_store_data`: Copies a word from a memory location to a structure, stores a longword from a memory location into a structure, and then populates several fields in a structure pointed to by a4 using values from a memory location.
- `initialize_and_process_table`: Clears a longword at offset 20 of a4, and if d1 is non-zero, it performs a series of operations including bitwise OR on a memory location, calculating an offset from d0 to fetch data from a table, and updating a structure at a3 and several fields in a4 with values from the table.
- `copy_and_store_data_offset`: Copies a longword from a memory location into a structure, copies a word from a memory location into a structure, and then populates several fields in a structure pointed to by a4 using values from a memory location.
- `jump_via_table_offset`: Decrements d1, uses d0 to calculate an offset into a table in a1, and jumps to the address found at that offset while storing a word from the table into a structure in a4.
- `jump_via_table`: Calls another routine, doubles a byte from a4, uses the result to fetch an address from a table in a0, and jumps to that address.
- `write_four_channel_volumes`: Runs a table-selected effect handler for each of four channels and writes each channel's volume, scaled by a value taken from a structure, to the audio volume registers.
- `update_buffer_value`: Adds a byte from a table to a byte at offset 23 of a structure in a4, then stores the result back to that offset if the result is not negative.
- `decrement_buffer_value`: Sets a byte at offset 35 of a structure to zero, then if a pointer in the structure is not null, it calculates a new pointer position and updates the byte at that location by subtracting its current value from negative one.
- `calculate_table_offset`: Saves registers to the stack, calculates an offset by multiplying d0 by a constant, adds it to a pointer in a1, and enters a loop that modifies a pointer in a1.
- `draw_string_fixed_origin`: Draws a null-terminated string into two bitplane buffers from a fixed screen origin, using the same glyph and control-byte rules as its sibling.
- `decrease_value_clamped_sixteen`: Copies a word from offset 6 of a structure to offset 2 of another structure, calls another routine, subtracts the result from a field in the first structure, and clamps the result between zero and sixteen before storing it back in that field.
- `calculate_signed_difference`: Subtracts d1 from d0 and performs range checks to adjust the result, then either calls another routine to store a value at an offset from a3 or calls another routine to store the bitwise complement of a value at an offset from a3.
- `split_value_by_eight`: Divides a value by eight and returns the quotient and the remainder packed into one register pair, with part of the lower word masked off.
- `parse_amigados_hunk_table`: Iterates through a table of data structures, identifying specific hunk types and performing memory allocation and relocation by copying data blocks and calling external routines to manage memory segments.
- `parse_amigados_hunk_table_2`: Iterates through a table of data containing AmigaDOS hunk type codes to perform memory relocation and segment loading.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:FLTINTRO

- `process_amiga_hunk_relocations`: Parses a data stream containing AmigaDOS hunk types to perform memory allocation and relocation.
- `decrement_counter_and_allocate`: Decrements a word in a loop, performs memory calculations, and calls two library routines to handle memory or system tasks.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `expand_data_stream`: Reads a value from a source in a0, calculates an offset to a destination in a3, and uses a loop to write bytes to the destination based on values processed by another routine.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_shifted_data`: Performs bitwise shifts and rotations on data from a source pointer in a0, using a count from d0 and a shift amount from d4, while calling another routine and potentially copying bytes from a table in a3 to a decreasing pointer in a3.
- `perform_trap_operation`: Moves a 16-bit word from an offset of 1773 in a2 to d4, performs a bitwise OR on d0 with a constant, calls a trap, and performs a bitwise OR on d1 with the result of the trap.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:FLTBOOT

- `initialize_hardware_registers`: Sets specific values in the custom chip registers for color, DMA, and interrupts, copies fifty bytes from a memory location to another, and then jumps to an address stored in a memory location.
- `load_cop_registers`: Loads a value from a memory location into both COP1LCH and COP2LCH hardware registers, then jumps to a destination address stored in a memory location.
- `copy_bytes_conditional`: Compares a2 and a3, and if a3 is not greater than a2, copies a byte from a2 to a3 and increments both pointers in a loop controlled by d7.
- `copy_bytes_backward_block`: Copies a block of bytes backward, from its last byte to its first, using a count held in a register.
- `copy_and_jump_table`: Calls another routine, disables interrupts, writes values to custom hardware registers, and copies 256 words from a routine to a table at 32768 before jumping to an address in that table.
- `initialize_hardware_and_jump`: Sets up several pointers in globals, calls another routine, performs a trap, modifies bits in a memory location, and jumps to an address stored in a table.
- `initialize_hardware_and_copy_block`: Writes constants to a global and to a CIA-B port register, saves two address values into globals, and calls several routines.
- `set_clear_byte_flags`: Computes a byte address from an uninitialized address register plus a constant offset, sets four bits in that byte and clears two others, leaving the remaining bits unchanged.
- `set_clear_byte_bits`: Computes a byte address from an input address register plus a constant offset, sets two bits in that byte, clears one bit and sets it again, leaving the other bits unchanged.
- `calculate_checksum_and_verify_string`: Calculates a checksum from a null-terminated string and uses it to index into a table, then performs a case-insensitive comparison of a string against a target buffer and copies data blocks if a match is found.
- `verify_and_copy_data_blocks`: Compares a sequence of case-insensitive characters from a source buffer against a target table, and if a match is found, copies blocks of data using a combination of longword and single-byte transfers based on a length field.
- `divide_index_and_compare`: Divides a value in d6 by eleven, compares the quotient's low word against a word in a structure pointed to by a4, and if they differ, calls another routine with a value combining the remainder and quotient before calling a second routine.
- `update_structure_flags`: Sets bit 2 of a byte at offset 256 of a structure in a3, then enters a loop that compares values from d6 and a4 to set or clear bits 0 and 1 at offset 256, finally calling another routine and writing several constants to offsets 2432, 2560, 3328, and 3840 of a3.
- `set_hardware_constants`: Sets bit 3 of a byte at offset 3840 from a3, writes 2 to offset 3328 from a3, writes 84 to offset 2432 from a3, writes 64 to offset 2560 from a3, and branches to another location.
- `sets_flags_waits_clears_bit`: Sets bit 3 in a byte at offset 3840 from a base register, writes the value 2 to a byte at offset 3328, and writes 100 and 8 to bytes at offsets 1536 and 1792.
- `toggle_bit_while_status_set`: Sets bit 1 of a byte at offset 256 of a structure, then repeatedly toggles bit 0 of that byte and calls another routine while bit 4 of a hardware register remains set, finally calling another routine and clearing a word at offset 26 of a structure.
- `configures_disk_sync_transfer`: Writes a set of 16-bit control words to hardware registers offset from a base register, including a sync word and a DMA-related value, and stores a 32-bit pointer from a second base-relative location into a register-offset slot.
- `verify_and_xor_data`: Validates a data block by checking for a specific word, then performs a series of XOR and OR operations on data from two tables to populate a destination buffer in a1.
- `calculate_negative_value`: Performs several bitwise OR operations on d0 with various constants, negates d0, loads the address of a local constant into a0, and sets d0 to zero.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV1#unpacked

- `process_obfuscated_data_level_1`: Performs a series of bitwise OR operations on d0 and d1 with various constants.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV2#unpacked

- `manipulate_bit_flags_and_memory`: Performs a series of bitwise OR operations on data registers and writes specific byte values to memory locations derived from register contents and offsets.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV3#unpacked

- `process_obfuscated_data_level_3`: Performs a series of bitwise OR operations on d0 with various constants, tests bit 7 of d6, performs a bitwise OR on a word at a location pointed to by a1, and performs a bitwise XOR on a longword at an offset of 2928 from a2.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV4#unpacked

- `modify_register_values`: Performs multiple bitwise OR operations on registers d0 and d1, clears a bit in a byte pointed to by a1 using d4, and toggles a bit in a byte pointed to by a5 using d2.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV5#unpacked

- `toggle_and_clear_bits`: Performs multiple bitwise OR operations on d0, toggles a bit in a byte pointed to by a5 using d2, clears a bit in a byte pointed to by a3 using d1, and compares a byte at an offset of a4 with 255.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV6#unpacked

- `subtract_from_memory`: Performs multiple bitwise OR operations on d0 and d1, then subtracts zero from a 16-bit word at an offset of 4095 from the pointer in a1.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV7#unpacked

- `modify_data_via_logic`: Performs a series of bitwise OR operations on register d0, then XORs a word at the address in a3 with a constant, and finally performs an AND operation on a value from a pointer in a7.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV8#unpacked

- `xor_and_subtract_data`: Performs a series of bitwise OR operations on register d0, XORs a longword at an offset from a2 with a constant, and subtracts a longword from a location calculated using a4 and d0.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:LV9#unpacked

- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:FRT

- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:MID

- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### Micro Machines (1992)(Codemasters)[cr FLT][t +6 Cherry][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_conditional_callback`: Calls three different routines via branch instructions and performs a supervisor call using a pointer from a global table.
- `initialize_hardware_and_wait`: Configures various hardware registers including DMA, interrupts, and custom registers, then enters a loop checking hardware status and global flags.
- `wait_for_flag_and_set_flag`: It spins until a specific bit in a word at a certain offset from a pointer in a register is set, then writes a constant word to another offset from that same pointer and returns.
- `wait_for_flag_toggle`: It spins on a flag byte in a global structure until the flag is set, then spins until the flag is cleared again, and then returns.
- `increment_and_set_values`: Increments a word at an offset from a pointer and writes a constant word to two subsequent offsets within a loop.
- `decrement_and_set_values`: Decrements a word at an offset from a pointer and writes a constant word to two subsequent offsets within a loop.
- `increment_until_limit`: Repeatedly calls another routine and increments a word at an offset from a pointer until it reaches a specific value.
- `decrement_until_limit`: Repeatedly calls another routine and decrements a word at an offset from a pointer until it reaches a specific value.
- `wait_for_register_match`: It loads an address into a register and then loops while the high byte of a word at offset six from that address is equal to itself, effectively creating an infinite loop.
- `check_serial_input_and_wait`: It tests a flag in a structure via a pointer and, if set, toggles a global variable while repeatedly calling another routine until a short condition holds, then restores the global variable.
- `invoke_callback_on_condition`: The routine calls a function obtained from a global variable table, and if the return value is non-zero, it calls a second function using that return value as an index and then calls a third function obtained from a global variable table.
- `call_supervisor`: Wrapper that calls the exec library function Supervisor and returns its result to the caller.
- `initialize_system_state`: Sets a global variable based on a comparison, calls a function from a global table, stores three values from the returned pointer into global variables, and then calls three more functions via offsets from that pointer.
- `execute_indirect_table_sequence`: It reads a global variable, calls three routines via offsets from that value and from a second global, then stores a different global variable into the first global variable.
- `calculate_offset_from_strings`: Iterates through null-terminated strings to calculate a value based on string lengths and stores the result in a memory location.
- `copy_word_to_offset`: Calls another routine and stores the resulting word into a location at an offset from a pointer.
- `copy_struct_fields`: Copies one byte from the source pointer to the destination pointer, then copies seven more bytes from successive source offsets to fixed destination offsets.
- `store_pointer_parts`: It splits two pointers into high and low words and stores these four words into a table at specific offsets.
- `clear_zero_table`: Clears a block of memory by writing zero to a sequence of longwords, iterating a fixed number of times while decrementing a counter and advancing a pointer.
