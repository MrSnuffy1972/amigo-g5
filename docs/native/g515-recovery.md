# Game 515 (Kgb): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00515). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4537392 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 23 raw-word lines |
| Functions named and described | pass, 176 of 176 named (0 author symbols), 176 of 176 described (18 by rule, 147 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1357 of 1357 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4264 of 4276 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 17133 |
| code | 16058 |
| data | 4146516 |
| padding | 1845 |
| unused | 355840 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf:install | 2 | 646 | 34 |
| KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf:KGB | 6 | 233 | 7 |
| KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf:intro | 5 | 233 | 7 |
| KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 24 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 460 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1664 | 72 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 973 | 30 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- runtime segment an address: 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 181 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-long-joy): 75 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 622 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- kgb (1992)(virgin)[cr trsi](disk 1 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf:dir.0: raw file found loaded verbatim in runtime segment an address (play-long-joy) at an address; classified through the recovered segment
- kgb (1992)(virgin)[cr trsi](disk 1 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf: blocks 2-625 (bytes 1024-320512) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.99)
- kgb (1992)(virgin)[cr trsi](disk 1 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf: blocks 627-829 (bytes 321024-424960) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.55)
- kgb (1992)(virgin)[cr trsi](disk 1 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf: blocks 831-879 (bytes 425472-450560) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.30)
- kgb (1992)(virgin)[cr trsi](disk 1 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf: blocks 1466-1466 (bytes 750592-751104) outside the filesystem hold data, not code (opcode density 0.035, entropy 3.08)
- kgb (1992)(virgin)[cr trsi](disk 2 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 2 of 5).adf: blocks 207-284 (bytes 105984-145920) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.86)
- kgb (1992)(virgin)[cr trsi](disk 2 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 2 of 5).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- kgb (1992)(virgin)[cr trsi](disk 3 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 3 of 5).adf: blocks 1137-1218 (bytes 582144-624128) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.78)
- kgb (1992)(virgin)[cr trsi](disk 3 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 3 of 5).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- kgb (1992)(virgin)[cr trsi](disk 4 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 4 of 5).adf: blocks 39-95 (bytes 19968-49152) outside the filesystem hold data, not code (opcode density 0.030, entropy 6.22)
- kgb (1992)(virgin)[cr trsi](disk 4 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 4 of 5).adf: blocks 103-127 (bytes 52736-65536) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.13)
- kgb (1992)(virgin)[cr trsi](disk 4 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 4 of 5).adf: blocks 884-1237 (bytes 452608-633856) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.10)
- kgb (1992)(virgin)[cr trsi](disk 4 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 4 of 5).adf: blocks 1239-1681 (bytes 634368-861184) outside the filesystem hold data, not code (opcode density 0.003, entropy 7.16)
- kgb (1992)(virgin)[cr trsi](disk 4 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 4 of 5).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- kgb (1992)(virgin)[cr trsi](disk 5 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 5 of 5).adf: blocks 37-137 (bytes 18944-70656) outside the filesystem hold data, not code (opcode density 0.036, entropy 6.90)
- kgb (1992)(virgin)[cr trsi](disk 5 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 5 of 5).adf: blocks 145-174 (bytes 74240-89600) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.54)
- kgb (1992)(virgin)[cr trsi](disk 5 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 5 of 5).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- kgb (1992)(virgin)[cr trsi](disk 1 of 5)/KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf:KGB: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 14 packed track region(s), 2988032 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf:intro

## Copy protection found

- none found

## Functions


### KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf:install

- `init_system_entry`: Reads a global variable and calls a function pointer stored at a negative offset within that structure, then checks a field at offset 172 and, when it is zero, invokes two more function pointers and calls itself again.
- `check_button_bit`: Tests bit 10 of the button state register and returns immediately when that bit is clear.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `load_zero_into_d0`: It contains a single instruction that loads the immediate value zero into the low byte of data register d0.
- `open_dos_library`: Attempts to open the dos.library using a system library call.
- `close_library_pointer`: Calls a function pointer stored in a global variable to close a library if the pointer is non-zero.
- `search_string_and_loop_forever`: Calls a sequence of function pointers stored in a structure, then reads a byte from that structure and searches a null-terminated string in a global variable for a matching entry, and finally enters an infinite loop.
- `resolve_and_call_current_dir`: Loads a pointer from a global variable, checks whether the pointed-to byte is zero, and otherwise calls two library routines, returning when the first result is zero.
- `iterate_callback_table`: Loops over a table of pointers while a global variable is non-negative, calling a routine four times per entry and setting a byte at offset 6 of a global structure to 48 when a pointer equals that global variable.
- `loop_based_on_global_flag`: Pushes a value onto the stack, reads a global variable and calls a routine, then calls another routine twice with different stack values while the global variable is zero before returning.
- `tiny_addq`: Small routine of 1 instruction that adds 4 to d1.
- `call_and_check_result`: Copies a value from a global variable into a register, calls another routine, and returns when that call result is non-zero.
- `traverse_and_recursively_process_table`: Walks a null-terminated list of values in a global table, and when a value points to a null entry it marks that entry with 255 and makes a series of recursive and sibling calls before skipping past the entry.
- `write_null_terminated_string`: Advances a pointer through a null-terminated string until it reaches the zero terminator, then calls the function pointer stored at offset 522 of the global structure and returns its result.
- `terminate_and_run_callbacks`: Retrieves a length from a routine, terminates a string at that length, appends a colon when none is present, and otherwise runs a series of function pointers from a table before returning a stored value.
- `convert_integer_to_decimal_2`: Repeatedly divides the input value by ten, storing each remainder as a decimal character into a buffer that is filled backward, and handles values of ten thousand or more by delegating to another routine.
- `convert_integer_to_decimal`: Repeatedly divides the value in d0 by ten, storing the character for each remainder as a decimal digit into the buffer pointed to by a2, and loops four times before returning.
- `manage_device_io`: Invokes a function pointer stored at offset -354 of a structure, then repeatedly calls a device I/O routine up to three times, checking the returned status and updating a global variable, and finally returns a combined byte value.
- `dispatch_indirect_calls`: Checks specific bits in a byte and calls function pointers retrieved from a table based on those bits, returning the result of one of the calls.
- `call_and_return`: It contains only a call to another routine followed by a return, and is referenced by address from other code.
- `return_immediately_2`: Returns immediately without modifying any data or registers.
- `write_zero_to_d0`: It writes the immediate value zero into the low byte of d0 eight times in a row, then writes the value 102 into d0, loads a word from a global variable into d0, loads the value 46 into d1, and writes d0 back to itself.
- `call_data_copy_routine`: Multiplies a value in d1 by 14, indexes a table at offset 20 with the result, stores d3 on the stack, and calls the drive validation and copy routine before restoring the stack and returning.
- `read_file_data`: Reads a function pointer from a global structure, calls it, and if the result is non-zero calls two other function pointers to perform file operations before returning.
- `write_file_data`: Reads a function pointer from a global structure and calls it; if the result is non-zero it calls two other function pointers and returns their result, otherwise it calls a third function pointer and returns the value minus one.
- `validate_drive_status_and_copy_data`: Reads a series of 16-bit fields from a structure to compute a status value, compares it against a stored value and another routine's return, and loops calling the routine until they match.
- `update_status_from_io`: Calls function pointers from a table when a global flag is set, compares the returned status value with a stored one, and updates the stored value and a status field when they differ.
- `sum_table_elements`: iterates through a table of shorts, summing their values until the total matches a specific constant.
- `perform_io_operation_2`: Fills fields of an I/O request structure with a command value, a length, a buffer address, and a shifted input value, then calls the function pointer at offset 488 of the global structure and sets a status byte to 255, looping until that status byte becomes zero.
- `init_and_check_io`: Writes the byte 128 to a byte at offset 64 of a structure in a6 and the byte 14 to a word at offset 28 of a global variable, then calls the function pointer at offset -464 of a6.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `clear_and_call_indirect`: If a global variable is non-zero it clears it and then calls the function pointer at offset -456 of a register set to that value, followed by the function pointer at offset -456 of the original register value.
- `check_drive_and_execute_callbacks`: Compares the incoming byte in data register d0 against the values 5, 4, 2 and 3, then repeatedly calls a routine while the byte matches none of them before returning the two incoming registers.
- `execute_callbacks_check_button`: Calls a routine that tests a button bit, then calls a routine that updates status from I/O, stores the value 1434 into a global variable, checks a status byte against 8 and branches when equal, and finally loads the value 255 into register d2 and returns.

### KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf:KGB

- `free_memory_blocks_loop`: Calls another routine and a function pointer, then loops four times calling another routine and a memory-free function pointer, clears the longword before a given pointer, and returns.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `loop_and_accumulate_values`: Initializes a short counter to zero, calls a routine twice, then loops twice calling a routine and, based on the sign of its result, either accumulating values into an integer array or calling another routine for nonzero results.
- `load_addr_and_load_d0`: Loads the address of a global variable into a1 and loads the immediate value 2 into d0, then returns.
- `copy_buffer_bytes`: Copies bytes from a source buffer pointed to by a0 into a destination buffer pointed to by a1, decrementing both pointers and repeating until a counter reaches zero, then returns.
- `push_and_branch_forward`: ORs zero into the word pointed to by a3, loads the immediate value 180 into d0, pushes the bytes of d0, d1 and the word of d2 onto the stack, restores the remaining registers, and branches forward.
- `empty_routine`: This routine is empty, containing only a single instruction that does nothing and returns immediately.

### KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf:intro

- `free_memory_blocks_in_loop`: Calls a routine before entering a loop that runs three passes, invoking a routine and dereferencing a pointer stored in a global variable at offset negative 210 on each pass.
- `decrement_until_minus_one_2`: Decrements a local variable until it reaches negative one and then returns.
- `loop_once_accumulate_values`: Initializes a short counter to zero, calls a routine twice, then loops once calling a routine and, based on the sign of its result, either accumulating values into an integer array or calling another routine for nonzero results.
- `load_address_and_load_two`: Loads the address of a routine located 492 bytes ahead into a register and loads the value 2 into a data register, then returns.
- `decompress_bitpacked_stream`: Decompresses a bit-packed data stream from the address in a0 into the buffer pointed to by a1, expanding each run of identical bytes to the count stored in the global variable, then returns.
- `push_and_branch_forward_102`: ORs zero into the word pointed to by a6, loads the immediate value 102 into d0, pushes the bytes of d0, d1 and the word of d2 onto the stack, restores the remaining registers, and branches forward.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### KGB (1992)(Virgin)[cr TRSi](Disk 1 of 5).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_struct_and_call_routine`: Calls a routine and reads a pointer from a global variable, then dereferences a pointer at offset negative 294 of that global to store its result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_register_and_set_byte`: Stores a value from a hardware register into a global variable and calls a routine, then if a chain of conditions holds it stores zero into a byte in a memory area.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_move_rts`: Small routine of 3 instructions that merges 236 into d0, then copies (a7)+ to d0, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_flag_and_call_routines`: Waits for a flag bit to clear, calls a setup routine, then invokes two stored routine pointers from different tables while repeatedly waiting for that flag bit to clear.
- `clear_flag_and_call_routines_2`: Clears a flag byte, invokes a stored routine pointer, waits for a flag bit to clear, invokes another stored routine pointer, then stores a routine address into a global variable.
- `process_and_adjust_buffer`: Calls a routine, decrements a global variable if it is non-zero, then resets it to 255.
- `compute_joystick_delta`: Reads a value from a global variable, applies bit shifts and masks to compute a new value, then subtracts the previous value from it across two byte sizes.
- `clamp_and_or_registers`: Calls a routine, clamps a register value between 336 and 350 if it falls outside that range, and stores the register values into two global variables.
- `or_register_into_global`: Bitwise-ORs a value from a data register into a global variable, then returns.
- `handle_mouse_button_state`: Returns immediately when a data register equals 3.
- `copy_and_normalize_text_2`: Scans a source buffer one byte at a time, advancing a 32-bit counter, and stops when it reaches a null, caret, or space character.
- `copy_and_normalize_text`: Reads characters from a source buffer, handling special markers by fetching more data or returning early.
- `index_table_then_call_routine`: Loads the address of a table into register a4, subtracts 32 from data register d0 and masks it to a byte, uses it as an offset to fetch a byte from the table into d0, restores registers d1 and a0, then calls a routine and returns unless register d7 is zero, in which case it branches away.
- `process_interrupt_flags`: Reads interrupt request registers, then branches on flag bits to either set a global variable to a constant and return, or call a routine.
- `wait_for_dma_bit_then_call`: Sets a global variable to one, then waits in a loop until a hardware register bit clears.
- `fetch_library_base`: Resolves a library function at offset -30 from the library base in a6 and calls it, then stores the returned pointer into a global variable at offset -30.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_offset`: Invokes a library routine via a negative offset.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `init_library_and_check_flag_2`: Clears a global flag and calls two cleanup routines in a loop until a status byte bit clears, then opens a device and resolves a library, setting a flag and storing the returned pointer into a global variable before returning a pair of registers.
- `open_and_io_device`: Stores a command value into a structure, calls a device open routine, and if it succeeds sets a flag bit, sets up request fields, calls a device I/O routine, copies a field between structure members, and returns the I/O result.
- `init_library_and_check_flag`: Loads two immediate values into data registers and calls a library resolver that stores the returned pointer into a global variable, then tests register d7 and jumps away if it is non-zero.
- `set_flag_byte`: Clears a byte at offset 67 of a structure in a global variable and sets a byte at offset 66 to one, then calls a routine and, if a flag is set, sets a bit in that byte based on a counter value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_do_i_o_2`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `perform_io_operation`: Executes six instructions and calls an input/output routine.
- `clear_dmacon_bits`: Writes the value fifteen to a word at offset 28 of the device structure in a1, then calls a routine at offset -456 from a6.
- `open_library_resource`: Calls a system routine at offset -408 from the library base in a6 twice, storing each returned value into a global variable before returning.
- `close_library_handles`: Loads the library base into a6 and calls a routine at offset -414, then repeats the process with a second library base and routine before returning.
- `return_immediately`: Returns from the routine after a very short sequence of instructions.
- `shift_and_clear_table`: Shifts a global variable left by two bits, then runs a sequence of setup routines and clears a block of doublewords by decrementing a counter.
- `read_keyboard_and_load_pointers`: Reads a byte from the keyboard serial data register and loads long words into an address register.
- `read_keyboard_data`: reads a byte from the keyboard serial data register.
- `test_bits_in_registers`: Tests bit 1 of the value in d2, then bit 0 of the value in d6, then bit 2 of the value in d4, using each of the three bit-test instructions in turn.
- `load_word_from_offset`: Copies a word from the address at offset 112 in a0 into d4, then returns.
- `scan_string_and_store_byte_2`: Walks through a string while checking several bytes at fixed offsets of a structure and accumulating a counter, then divides that counter by ten when a loop counter is non-zero and stores the resulting byte into a global variable.
- `validate_structure_bytes`: Checks bytes at offsets 14, 48, and 59 of the structure in a0, adding two to the 16-bit value in d3 when the byte at offset 48 is non-zero, otherwise adding five, then returns.
- `test_byte_advance_counter`: Tests the byte at offset 14 of the structure in a0 and branches away when it is zero, otherwise adds five to d3 and branches to the target, containing no loop and making no call.
- `test_byte_then_advance_counter_2`: Tests the byte at offset 61 of the structure in a0 and branches away when it is zero, otherwise adds four to data register d3 and branches to a later point.
- `test_byte_then_advance_counter`: Tests the byte at offset 34 of the structure in a0 and branches away when it is nonzero, otherwise adds 3 to data register d3 and branches into another routine.
- `test_byte_and_advance_counter`: Tests a byte at offset 38 of a structure indexed by a data register, and if it is nonzero it branches away.
- `scan_string_and_store_byte`: Increments a counter by six and multiplies a byte by ten while scanning a string, checking bytes at offsets 23, 11, 58, 59, 14, 61 and 34 for nulls and branching on each.
- `fill_and_copy_byte`: Fills a range of doublewords starting at a computed address with the value -1 using a decrementing counter loop, then copies a byte from offset 1390 in a0 into two fields of a structure.
- `reset_global_variables`: Assigns the value eight to a global variable, then clears a series of global variables to zero and sets the last one to 255 before returning zero.
- `store_state_and_call_routines`: Clears a global variable, reads a value and indexes a table by its low byte, then stores a flag and calls several routines in sequence before clearing the flag again and returning.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d4.
- `save_state_and_loop_call`: When the value in d7 is zero it stores the 16-bit d0 into a global variable, calls three routines with that value, clears the global variable, invokes the routine pointed to by d4 when that pointer is nonzero, calls another routine, then repeatedly calls a routine returning a byte until either it is nonzero or the global variable becomes nonzero, and finally calls another routine and returns.
- `copy_memory_block`: Copies sixteen doublewords from the source range starting at a0 plus d4 to the destination range starting at a1 plus d5 using a loop counting down from fifteen, then restores the registers and returns.
- `scan_and_update_structure`: Walks a sequence of longs, returning when it finds a zero entry.
- `save_state_and_loop_call_2`: Copies the incoming d0 value into a global variable, calls a routine and marks the global as -1 when it was zero, then branches on the high byte to call different routine pairs and finally resets the global to 15786.
- `process_indexed_string_table`: Reads a table of 32-bit entries indexed by a global byte, adds a negative offset to the selected entry, writes fixed values to offsets 26686, 26688 and 26690 of a structure, then copies characters from the computed string until a zero byte is reached.
- `clamp_and_store_stack_values`: Clamps two 16-bit values from the stack into the range 0 to 255 and stores them at offsets 1326 and 1328 of the same stack frame, then calls a sequence of four routines twice.
- `load_two_and_branch`: Loads the value two into a register and branches to a following routine.
- `test_word_and_pop_register`: Tests a word in a global variable and branches away when it is nonzero.
- `set_flag_and_branch`: Stores the value one into a byte at a fixed global address and branches to a following routine.
- `loop_and_check_flag`: Checks a global byte and, when it is zero, sets it to 200 and repeatedly calls a routine until that routine returns nonzero or the counter reaches zero, then clears the global byte.
- `blend_memory_block`: Computes source and destination pointers from the input registers and uses a loop to blend values from several offsets of the source block into matching offsets of the destination block, masking each word with a per-iteration shift value.
- `update_flag_and_call_routines`: Calls a routine, saves a global value, and if that value is not two it calls another routine and resets the flag.
- `index_table_with_flag`: Reads a byte from a global table, decrements and masks it, doubles it, adds a value from a register, stores the result back into that register, and then loads a word from a table indexed by that value into the register.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d7, then returns.
- `shift_and_update_state`: Clears a global variable and calls two routines, then, when a flag register is zero, left-shifts a long at a structure offset by three in a loop that runs from 63 down to one.
- `compare_and_update_flag`: Compares an incoming byte with a global flag and, when they differ, resets the flag and calls several routines.
- `clear_flag_and_call_routines`: Clears a global variable and sets it to 255 while calling four routines in sequence.
- `call_two_routines_twice`: Calls one routine followed by another routine twice, then returns.
- `update_and_store_value`: Reads a word from a global variable and a long from a pointer, calls a routine, adds the values together, increments the result, clears its low bit, stores it through the pointer, and returns.
- `clear_and_call_routines`: Calls a routine twice, then calls another routine, copies a value from a pointer to the next location, clears a global block of words by looping until a counter reaches zero, calls a string routine twice, invokes a stored routine pointer, and stores the combined result into a global variable.
- `process_input_table_entries`: Reads a sequence of 16-bit values from a table pointed to by a4, and for each value writes it into a buffer while calling a routine that returns a pointer and a string.
- `accumulate_long_values`: Reads a long from a pointer in a0, adds it to the word at hardware register offset 48aa, decrements a counter, stores the even result back through the pointer and advances the pointer and the register pointer, repeating until the counter reaches zero.
- `compare_bytes_in_loop`: Computes an index from a word at offset 20 of a structure and loops while an index stays below a word at offset 22.
- `write_ff_from_table`: Reads a byte from a table indexed by a register, and if that byte is nonzero it writes 255 through another pointer in a loop indexed by a decremented counter.
- `fill_table_with_max`: Fills a table with the value an address, writing one long per iteration until a counter decremented from 255 reaches zero.
- `clear_and_call_struct_handler`: Saves the current registers, clears a byte at offset 44 of a global structure, calls a routine with the value one in the first register, then restores the registers and returns.
- `store_byte_into_field`: Stores a byte passed in a register into a global variable and into a field selected by a word read from that variable, then clears a global byte and returns.
- `check_flag_and_call_routines`: Reads a value from a routine and branches into the middle of another routine.
- `increment_and_clamp_byte`: Increments a global byte by one and clamps it to two when it exceeds four, repeating this until the byte becomes zero.
- `load_immediate_then_branch`: Stores the immediate value 4096 into data register d1, then branches back into the middle of another routine.
- `update_structure_field`: Reads a long from a global variable using an index derived from a byte, selecting between two offsets depending on a global flag.
- `free_if_pointers_valid`: Checks two pointers held in a structure and, if both are nonzero, calls a routine to free memory and then clears both pointers before returning.
- `call_routine_seven_times`: Calls a routine seven times in sequence and then returns.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_routine_with_early_return`: Calls a routine and returns early when a register is nonzero.
- `call_and_load_one`: Calls a routine and then loads the value 1 into a register before returning.
- `update_and_restore_state`: Saves the current value of a global variable, then reads a byte from the pointer in d3; if bit 7 is clear it calls one routine and, when the word in d7 is zero, calls another routine and stores a derived counter, otherwise it calls the first routine again, calls a third routine twice, calls a fourth routine, and stores a derived counter, then restores the saved value.
- `test_bit_zero_d0`: Tests bit 0 of the 32-bit value in d0 and returns without modifying anything.
- `empty_routine_2`: Contains no instructions, does nothing, and returns immediately.
- `tiny_ori_2`: Small routine of 1 instruction that merges 208 into d5.
- `check_s_character_store`: If the byte in d0 is not the character 'S' it calls two routines and returns the register pair, otherwise it stores the value 33868 and the value 5 into two adjacent 16-bit memory locations at offsets 30 and 26 of a structure and returns.
- `initialize_memory_values`: Clears a global variable and stores the value 33868 and the value 5 into two adjacent memory locations before returning.
- `check_s_character_conditional`: If the byte in d0 is not the character 'S' it calls a routine when a global variable is zero, then calls two more routines and returns the register pair, otherwise it stores the value 33868 and the value 5 into two adjacent 16-bit memory locations at offsets 34 and 30 of a structure and returns.
- `store_values_from_a4`: Stores the value 65535 into a global variable, then copies the two halves of the 32-bit value in a4 into two adjacent 16-bit memory locations and the value 72 added to a4 into two more adjacent 16-bit memory locations.
- `compute_and_store_byte`: Computes a 16-bit value from the sum of two registers plus carry and overflow flags, then stores one byte from a register pair through a pointer and the computed byte through the following pointer, returning immediately.
- `compute_byte_offset_store`: Computes a 16-bit value from the signed byte in d0 minus one times 112 plus a byte at offset 4 of a structure, then stores that value and the same value plus 56 into two adjacent 16-bit memory locations at offsets 30 and 26 of the structure and returns.
- `clamp_and_store_value`: Calls another routine and reads a word from a register pair.
- `contains_no_op_instruction`: Contains a single instruction that performs a no-op on a register, and this range is most likely a zero-filled data area that other code refers to by address.
- `test_bit_and_check_character`: Tests bit 0 of the 32-bit value in data register d0, then checks whether the byte in d0 is the character 'S' and branches accordingly, calling a routine when a global variable is zero before invoking two more routines and returning the value 'r'.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_tables_and_wait_vretrace`: Fills two tables of 78 short values from a global table of three bytes each, then waits for the vertical retrace bit in the custom DMA register and calls another routine in a loop until a flag in a global variable is set.
- `noop_d0_register`: Performs a no-op OR of the immediate value zero onto the d0 register, leaving its contents unchanged, then falls through into the following routine.
- `clear_and_search_structure`: Clears a byte at offset 951 of a structure pointed to by a global variable, then runs a nested search loop that decrements a counter and compares bytes, and when it reaches the end it zeroes a block of memory, sets bit 1 of the global variable, and returns.
- `reset_globals_to_constant`: Writes zero to five global variables and writes a constant value to a fifth global variable.
- `increment_and_check_byte`: Increments a byte at offset a fixed address, loads a byte from a second location into a data register and compares it against a byte at offset a fixed address, returning early when the loaded value is not greater.
- `load_and_play_sound_effect_2`: Loads a sound effect definition into a sound channel structure and plays it, then reads a low nibble from a structure to index a table of channel lengths and adjusts the channel's remaining length counter accordingly.
- `load_and_play_sound_effect`: Loads a sound effect definition from a table into a sound channel structure, copying its length, period and pointer fields, then decodes a compressed sample stream by repeatedly reading nibbles and writing the resulting samples into the audio DMA buffer while updating the channel length register.
- `process_structure_nibble_and_store`: Clears the low nibble of a 16-bit value in the structure at a6 and, based on the low nibble of the byte at offset 1, stores a word from the structure, a value from a table indexed by the byte at offset 9, or a decremented or incremented 12-bit counter into a word at offset 6 of a5, clamping the counter within bounds.
- `copy_structure_word`: Copies the 16-bit word at offset 16 of the structure in a6 into the 16-bit value at offset 6 of the structure in a5, then returns.
- `search_table_and_store_word`: Loops through a table of words, comparing each against a masked value and advancing by two until a match is found or the loop ends.
- `clamp_and_search_table`: Reads a short at offset 24 of the structure in a6 and returns the input value d0 when it is zero, otherwise adjusts a short at offset 16 by adding or subtracting the byte at offset 27 and clamps it when the result exceeds the value at offset 18.
- `compute_shifted_value_store`: Reads a byte at offset 27 of a structure, applies a shift and multiply based on the sign of a byte at offset 27, then stores the result plus a word at offset 16 into a word at offset 6 of another structure and updates the byte at offset 27 and returns.
- `process_structure_nibble`: Reads a byte from a structure and branches on its high nibble, updating counters and status fields: nibble 9 adjusts a timer and stores a value, 11 and 13 set a flag and return, 12 clamps a value and stores it, 15 copies a byte, and 0-8 modify various structure bytes and a global variable.
- `store_nibble_and_enable_dma`: Stores the low nibble of the byte at offset 3 of the structure in a6 into the byte at offset 18, then sets bit 15 of the DMA control register to enable DMA channels.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `contains_bit_test_instruction`: Contains a single bit-test instruction on two registers, and this range is most likely a zero-filled data area that other code refers to by address.
- `clears_register_d0_byte`: It clears the lower byte of the data register d0 by OR-ing it with zero, then returns.
- `zero_memory_via_pointer`: This routine is a single instruction that writes a zero byte to the memory location pointed to by register a0, then returns.
- `tiny_ori_3`: Small routine of 1 instruction that merges 15 into (a0).
- `dispatch_and_advance_table`: Calls a routine that dispatches to an indirect address, then calls a routine that advances a table index and returns, and finally returns.
- `jump_to_indirect_address`: Loads a 32-bit address from a global variable into a register and jumps to that address, dispatching to one of several routines.
- `advance_table_index_and_call`: Decrements a 16-bit counter in a global variable and returns while it is nonzero, otherwise advances a table index past entries that are null and calls the first non-null routine found, storing its return value.
- `store_pointer_and_load_d1`: Stores the address a fixed address into a global variable, then when the byte pointed to by another global variable is zero it replaces that pointer with the address of a global variable, and finally loads the value a fixed address into data register d1 and returns.
- `store_constants_return`: Stores the value an address into a global variable and returns the value an address in the register pair, performing no other work.
- `load_constants_and_return`: Loads the decimal constant an address into data register 1 and the decimal constant 200 into data register 0, then returns.
- `init_struct_and_copy_table`: Writes fixed constants into several fields of the structure in a6, then waits until a bit in a global variable is cleared before copying twelve words from one table to another and advancing the source pointer.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `translate_buffer_advance_pointers`: Loads two addresses from global variables into registers, calls a routine that translates and copies a buffer, then increments one address by 48 and the other by 384, stores both updated addresses back into the global variables, and returns.
- `translate_and_copy_buffer`: Copies bytes from a source buffer into a destination buffer, translating each source byte by subtracting 32 and using it as an index into a 220-byte lookup table, repeating for 44 source bytes.
- `fill_table_from_global_bytes`: Fills a table of 78 short values with computed sums of three bytes from a global table, then increments that global table by five bytes.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
