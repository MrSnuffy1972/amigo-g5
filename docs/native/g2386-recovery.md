# Game 2386 (zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02386). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (935216 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 34 raw-word lines |
| Functions named and described | pass, 237 of 237 named (0 author symbols), 237 of 237 described (13 by rule, 224 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1064 of 1064 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 67072 bytes, 11 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4012 of 4316 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28966 |
| code | 20322 |
| data | 885398 |
| padding | 18 |
| unused | 512 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf:bootblock | 1 | 55 | 5 |
| logged disk load from Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf boot-block load | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2418 | 172 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 141 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 970 | 27 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 57 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf bytes 425472-438272 (code never loaded by a reference run) | 1 | 0 | 0 |
| Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf bytes 438784-440320 (code never loaded by a reference run) | 1 | 200 | 7 |
| Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf bytes 443392-443904 (code never loaded by a reference run) | 1 | 0 | 0 |
| Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf bytes 444928-495104 (code never loaded by a reference run) | 1 | 85 | 1 |
| Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf bytes 503808-504832 (code never loaded by a reference run) | 1 | 160 | 2 |
| Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf bytes 882176-883200 (code never loaded by a reference run) | 1 | 99 | 1 |

## Cases handled

- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: the boot block reads 6656 bytes from disk offset 890368 to an address (found statically); recovered as a unit
- runtime segment an address: 553 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 12 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 107 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 350 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 35 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: bytes 425472-438272 look like code no run loaded: recovered position-independently (load address unknown)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: bytes 438784-440320 look like code no run loaded: recovered position-independently (load address unknown)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: bytes 443392-443904 look like code no run loaded: recovered position-independently (load address unknown)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: bytes 444928-495104 look like code no run loaded: recovered position-independently (load address unknown)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: bytes 503808-504832 look like code no run loaded: recovered position-independently (load address unknown)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: bytes 882176-883200 look like code no run loaded: recovered position-independently (load address unknown)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: 7 logged loads cover 73728 bytes of the image
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: 32 track-loaded blocks found in recovered runtime segments and classified through them
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: blocks 2-150 (bytes 1024-77312) outside the filesystem hold data, not code (opcode density 0.044, entropy 5.55)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: blocks 152-829 (bytes 77824-424960) outside the filesystem hold data, not code (opcode density 0.032, entropy 5.80)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: blocks 862-862 (bytes 441344-441856) outside the filesystem hold data, not code (opcode density 0.047, entropy 5.58)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: blocks 970-972 (bytes 496640-498176) outside the filesystem hold data, not code (opcode density 0.057, entropy 5.79)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: blocks 974-981 (bytes 498688-502784) outside the filesystem hold data, not code (opcode density 0.023, entropy 4.28)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: blocks 994-1720 (bytes 508928-881152) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.59)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: blocks 1732-1738 (bytes 886784-890368) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.10)
- zeewolf 2 - wild justice (1995)(binary asylum)[cr atx][t +18 atx]/Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf: blocks 1752-1759 (bytes 897024-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.17)
- 3 never-loaded region(s), 63488 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_bitstream_and_hardware` (runtime segment an address): Performs two loops that manipulate bits in a destination table using values from a source table and a mask, sets a hardware register, and then performs several conditional hardware register modifications and calls another routine. Kind: bitstream manipulation/decryption. Pass path: the instructions following the call sites, specifically where the code continues to process data (e.g., movea.l a2, a0 or dbra loops).
- `configure_dma_and_wait` (runtime segment an address): Writes values to custom chip registers, waits for bit 4 of a byte at offset 3328 of a custom chip register to be set, writes a pointer and two words to custom chip registers, and then waits for bit 1 of a custom chip register to be set before writing a final value. Kind: disk hardware/sync check. Pass path: the instruction immediately following the bsr.w an address call.

## Functions


### Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf:bootblock

- `initialize_system_environment`: Retrieves a pointer from a system list, calls two routines via a6, populates a structure with specific values, performs a memory allocation, and modifies hardware registers before calling a routine at a calculated address.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `perform_logical_or`: Performs a bitwise logical OR of zero onto register d0.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `set_structure_and_call`: Stores three 32-bit values into a structure at offsets 36, 40, and 44 using a pointer in a1, calls a routine at an offset from a6, then calls a routine at address an address, and finally jumps to a location stored at a2 plus offset 64.

### logged disk load from Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf boot-block load

- `store_and_rotate_long`: Stores a byte from a long value in a0 into a memory location at a5, sets the carry flag, and returns a long value with the bits rotated right.
- `process_data_stream`: Performs a loop that shifts a long value right and calls another routine when the value reaches zero, or executes a block that reads bytes from a source pointer, compares them against a constant, and writes them to a destination pointer or a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_callback_table`: Copies three longwords from a list pointed to by a5 into global variables, then enters a loop that executes a pointer from a table and branches based on the returned value in d0.
- `branch_to_offset_2`: Loads an address from a relative offset into a0 and branches to another location.
- `update_callback_pointer`: Adds register d0 to a word from a table, and depending on the result, either clears a global word, updates the table with a new value, or stores the result back into the table.
- `tiny_ori_2`: Small routine of 1 instruction that merges 252 into d0.
- `tiny_ori_3`: Small routine of 1 instruction that merges 150 into d0.
- `initialize_and_execute_callback_table`: Loads a table of pointers and several state variables from a list pointed to by a5, clears a byte and a word in the newly stored global pointers, and then calls a function pointer retrieved from the table if it is not null.
- `process_input_character`: Calls two routines via pointers, compares d0 to several values, and if d0 is between decimal eight and decimal thirty-one, calls three routines, increments a global word, and calls a final routine.
- `branch_to_offset`: Performs a short branch to an offset.
- `check_conditional_logic`: Compares a byte in d0 to 8, and if equal, checks a 16-bit value in a global variable; if that value is not zero, it calls three different routines and decrements the global variable before branching.
- `execute_conditional_branch`: Compares a byte in d0 to 4, and if not equal, branches; otherwise it checks a 16-bit value in a global variable, and if it is not zero, calls two different routines and decrements the global variable before branching.
- `execute_conditional_logic`: Compares a byte in d0 to two, and if not equal, branches; otherwise it calls three different routines sequentially, increments a long word at a global address, calls a fourth routine, and branches.
- `execute_conditional_sequence`: Compares a byte in d0 to sixteen and branches if not equal, otherwise it calls four different routines sequentially and then branches to another location.
- `branch_to_offset_alt`: Performs a short branch to a different offset.
- `call_and_restore_registers`: Calls another routine and restores a5 from the stack before returning.
- `shift_string_bytes`: Loads two pointers and an index from global variables, then enters a loop that shifts bytes within a memory range until a null terminator is encountered.
- `copy_byte_with_offset`: Loads a pointer and an index from global variables, then copies a byte from an offset of that pointer to the next byte in the sequence, branching if the byte is non-zero.
- `set_byte_at_offset`: Loads a pointer and an index from global variables and stores the value decimal fifty-nine at the calculated offset.
- `set_byte_at_offset_alt`: Loads a pointer and an index from global variables and stores the value decimal thirty-two at the calculated offset.
- `test_byte_at_offset`: Loads a pointer and an index from global variables and tests the byte at the calculated offset.
- `test_byte_at_pointer`: Loads a pointer from a global variable into a0, loads a word from a second global variable into d0, tests the byte at the address in a0 plus the offset in d0, and returns.
- `clears_data_register_zero`: Performs a logical OR operation on register d0 with zero.
- `clears_data_register_zero_2`: Performs a logical OR operation on data register zero with zero.
- `zeroes_data_byte`: Performs a logical OR operation on a byte in register d0 with zero.
- `lookup_and_call`: Loads a word from a pointer in a0 into d0, doubles d0 twice, uses the result to index a table pointed to by a pointer from a5 to update a0, and then calls a routine at address 55740.
- `store_routine_address_3`: Loads the address of this routine into a0 and stores it into a global variable.
- `store_routine_address_2`: Loads a PC-relative address into a0 and stores it into a global long word.
- `update_signed_counter`: Loads a 16-bit value from a memory location, increments it if it is not negative, stores it in a global variable, and then either negates that global variable or continues.
- `update_global_counters`: Increments d0 and stores it in a global variable, negates d0, then checks a value from a memory location to conditionally increment and store another value in a different global variable.
- `swap_global_pointers`: Increments a 16-bit value in a global variable, stores the result in another global variable, and then swaps the contents of two other global variables.
- `copy_memory_block`: Calls another routine, then copies several longwords from a source table at a specific address to a destination table pointed to by a global variable using a loop that runs 2000 times.
- `process_complex_logic_flow`: Checks a global byte, and if it is negative, enters a loop that calls another routine and stores the returned byte into a buffer until a zero byte is encountered; otherwise it executes a sequence of calls to other routines and modifies a byte at an offset of a structure in a1.
- `modify_and_call_routine`: Performs bitwise operations on d0, moves a word from the address in a0 to the address in a1, and calls another routine, branching to another routine if the zero flag is not set.
- `compare_and_branch`: Compares the byte at the address in a0 with 101 and branches to another routine if not equal, otherwise it loads a new address into a0 and branches to another routine.
- `update_stack_pointer_and_branch`: Loads a value from a memory location into d0, copies the stack pointer to a global variable, and branches to another routine if d0 is not zero; otherwise it calls another routine, and if the zero flag is set, stores zero in d0 and writes it to the global variable before returning.
- `compare_and_jump`: Compares the byte at the address in a0 with 101 and branches to another routine if not equal, otherwise it loads a new address into a0 and performs a branch to another routine.
- `loop_and_copy_bytes`: Decrements a 16-bit value in a global variable and, if it is not negative, calls another routine and loops back to the start of the routine; otherwise it loads a byte from the address in a0 into d0, updates a pointer in a global variable, clears d7, and returns.
- `loop_and_store_data`: Decrements a 16-bit value in a global variable in a loop, calling another routine each iteration, and finally stores the resulting byte from d0 into a location pointed to by a0 and updates a global pointer.
- `loop_and_call_routine_3`: Decrements a 16-bit value in a global variable in a loop, calling another routine each iteration, if the result is not negative.
- `store_and_update_pointer`: Stores a longword from d0 to the location pointed to by a0, increments a0, stores the new value of a0 into a global variable, and clears d7.
- `save_stack_and_call`: Stores the current stack pointer and the value in a1 into global variables, then calls another routine and branches to another routine if the zero flag is not set.
- `copy_data_block_2`: Calls another routine and, if the zero flag is set, copies eight 32-bit words from the address in a0 to a new destination in a1 using a loop, then updates two global variables.
- `initialize_and_store_pointer`: Calls another routine and, if the zero flag is set, stores a local address into a global variable, writes 128 to another global variable, and clears d0.
- `fill_data_with_loop`: Checks if a 16-bit value in a global variable is negative; if not, it enters a loop to store the value 26 into a buffer pointed to by a0, then calls two different routines and clears d0.
- `update_conditional_pointer`: Calls another routine and, if the zero flag is set, stores a local address into a global variable and writes the value 128 to another global variable; otherwise, it returns the value 255 in register d0.
- `perform_zero_or_instruction`: Performs a bitwise OR operation with zero on register d0.
- `zeroes_data_register_zero`: Performs a bitwise OR operation on data register zero with zero.
- `perform_double_zero_or`: Performs two consecutive bitwise OR operations with zero on register d0.
- `perform_null_operation`: Performs a logical OR operation on a byte in a data register with zero, effectively doing nothing.
- `clear_data_register_d0`: Performs a logical OR operation on data register d0 with zero.
- `perform_multiple_zero_ors`: Performs thirty-two consecutive bitwise OR operations with zero on register d0.
- `clear_and_store_pointers`: Clears register d0 through multiple operations and then stores four specific longwords into a table and other global variables.
- `tiny_ori_4`: Small routine of 1 instruction that merges 224 into d5.
- `search_and_encode_string`: Calculates an offset based on a byte from a structure, then enters a loop to search for a null terminator in a string, eventually storing a processed value into a memory location.
- `initialize_and_populate_lookup_table`: Clears a block of memory with zeros, sets the first entry to all ones, and then executes nested loops to parse a data structure; it reads bytes to determine offsets and writes the value two hundred fifty-five to specific locations in the cleared memory block based on values extracted from the data.
- `modify_hardware_register`: Loads a byte from a memory location, calls another routine, increments the byte by two, and then performs a sequence of bit set and bit clear operations on a specific offset of a custom hardware register.
- `modify_hardware_register_2`: Loads a byte from a global variable, calls another routine, increments the byte, modifies a bit in a hardware register at offset 256 of the address in a0 using the modified byte, and then calls another routine with d1 set to 1000.
- `reset_and_store_byte`: Clears a byte at a global variable location, calls another routine, and stores the resulting byte from d0 into another global variable location before returning.
- `check_d0_value`: Loads a pointer into a0, tests the byte in d0, and branches to another routine if d0 is zero or if d0 is less than three.
- `initialize_hardware_status`: Sets two 16-bit words to 65535 and clears two bytes in two different global variables, then checks bit 5 of the CIAA_PRA register; if bit 5 is set, it calls another routine, sets d0 to 2, and calls another routine; if bit 2 of CIAA_PRA is set, it loads a pointer from a relative address and calls another routine before returning.
- `check_hardware_status_2`: Calls another routine, calls a second routine, and then checks bit 2 of a hardware register before returning.
- `check_disk_write_protect`: Tests bit 3 of the CIAA_PRA register and branches to another routine if the bit is zero.
- `check_disk_status`: Tests bit 5 and bit 2 of the CIAA_PRA register, then either branches to another routine or clears d0 and returns.
- `process_structure_updates`: Stores the stack pointer in a global variable, calls four different routines, and then modifies a byte at offset 12 of a structure in a1.
- `initialize_hardware_and_status`: Loads a byte from a global variable, calls three separate routines, and then branches to another routine.
- `fetch_byte_and_call_subroutine`: Loads a byte from the address pointed to by the first element of a stack-saved list into register d0, calls a subroutine, restores the saved registers, and branches to another routine.
- `initialize_and_patch_data`: Clears two bytes at offsets 12 and 32 of a structure pointed to by a register, then calls three routines to modify hardware, reset a global variable, and initialize hardware status before performing a conditional string comparison and patch operation using a wildcard character.
- `execute_callback_loop`: Sets a stack pointer from a global variable, calls three different routines, and then enters a loop that calls another routine while incrementing a value in d0 until d0 equals 4.
- `set_d0_to_ff_2`: Sets the stack pointer to a value from a memory location, sets d0 to 255, and returns.
- `update_table_index_and_counter`: Checks a byte at offset 32 of a structure; if it is less than 128 and greater than or equal to the byte at offset 15, it increments the byte at offset 32 and returns 0, otherwise it increments the byte at offset 12, clears the byte at offset 32, and returns 255.
- `process_structure_data`: Loads a byte from a structure in a1, calls a routine to search for a null terminator, and then either increments bytes at offsets 15 and 32 of the structure or enters a loop that increments a byte at offset 12 and clears offset 32.
- `perform_logical_or_operation`: Performs a logical OR operation on data register zero using a zero constant.
- `set_status_byte_and_branch`: Writes the value 127 to a byte at a specific memory location and then branches to another routine.
- `set_flag_and_branch`: Writes the value 255 to a byte at a specific memory location and then performs an unconditional branch to another routine.
- `zero_initialize_buffer_and_patch_memory`: Clears a 20-byte buffer starting at an offset from a pointer in a1 with zeros using a loop, then calls compare_and_patch_string_data to conditionally overwrite a memory range, and finally writes the byte 229 to a global variable before returning.
- `compare_and_patch_string_data`: Compares a sequence of bytes from a source buffer to a destination buffer, allowing for a wildcard character, and performs conditional byte-wise copying or patching of a global variable based on the comparison results and specific byte markers.
- `compare_and_copy_strings`: Compares two strings byte by byte using a loop, and if a mismatch is found or a specific character is encountered, it copies a sequence of bytes from one memory location to another.
- `perform_eight_zero_ors`: Performs eight consecutive bitwise OR operations with zero on register d0.
- `set_d0_to_ff`: Performs a bitwise OR with zero on register d0, sets register d0 to a byte value of 255, and then returns.
- `extract_table_values`: Loads a byte from an offset of 32 of a structure in a1, uses its high nibble to calculate an index, and then loads two bytes from a table at that index into d0 and d1.
- `calculate_offset_address`: Calls another routine, shifts the returned value in d1 left by 7 bits, and adds it to a0.
- `extract_table_values_duplicate`: Reads a byte from offset 32 of a structure in a1, uses the high nibble to index into a table to load two bytes into d0 and d1, and masks d1 with 15.
- `copy_data_block`: Calls another routine, calculates an address by adding a shifted d1 value to a0, copies eight longwords from a source pointed to by a2 to the destination at a0 using a loop, and stores a byte into a global variable.
- `execute_repeated_callback`: Compares d0 to a word in a global variable and, if they are not equal, calls another routine, performs arithmetic on d0, and enters a loop that calls another routine four times before updating the global variable and clearing a byte at an offset.
- `initialize_structure_and_hardware`: Clears bytes at offsets 12 and 32 of a structure in a1, calls five different routines, and then either returns or calls two more routines and branches to another routine.
- `call_sequential_routines`: Loads a byte from a structure in a1 into d0 and calls four different routines before clearing d0 and returning.
- `loop_and_call_routine_2`: Loads a byte from a global variable, and if it is non-zero, enters a loop that calls another routine four times with an incremented value derived from a word in a global variable.
- `divide_and_copy_buffer`: Divides register d0 by decimal eleven, calls another routine, and if the result is zero, copies thirty-two longwords from the buffer in a0 to the destination in a0 using a loop.
- `divide_and_offset_pointer`: Divides register d0 by decimal eleven, calls another routine, and if the result is zero, adds decimal 512 to the pointer in a0 before returning.
- `set_data_bit`: Sets bits 7, 6, and 5 of d5 by performing a bitwise OR with 224.
- `copy_buffer_via_division`: Sets register d0 to decimal 255, calls another routine, and if the result is zero, copies thirty-two longwords from the buffer in a1 to the destination in a0 using a loop.
- `divide_and_call_routine`: Divides register d0 by decimal eleven, swaps the high and low words of d0, calls another routine, and returns.
- `update_status_and_return`: Compares d0 to a global word, and if they differ, calls three routines and updates the global word; then, if d1 is less than decimal eleven, it calculates an address by adding a shifted version of d1 to a pointer and returns.
- `copy_data_and_update_status`: Compares d0 to a global word, and if they differ, calls three routines and updates the global word; if d1 is less than decimal eleven, it copies thirty-two words from a source to a destination and sets a global byte to decimal two hundred fifty-five.
- `update_hardware_registers`: Checks a byte at a specific offset; if it is not zero, it loads a hardware register address into a0, writes a 16-bit value to an offset of that register, calls another routine, writes a different 16-bit value to the same register offset, and branches.
- `process_bitstream_data`: Sets a bit in a table based on a value from a global variable, then enters a loop that performs several operations including writing a constant word to a table, calling various routines to update table entries, and performing bitwise shifts and additions to calculate new table values.
- `fill_buffer_with_xor_pattern`: Decrements d6 and enters a loop that writes a word to the address in a1, where the word is the result of XORing 43690 with the bit-rotated value of the word previously at a1 minus 2, then increments a1 and repeats until d6 is zero.
- `fill_buffer_with_constant`: Decrements d6 and enters a loop that writes the word 43690 to the address in a1 and increments a1 until d6 is zero.
- `xor_checksum_and_mask`: Decrements d6 and enters a loop that XORs the value at the address in a0 with d1, then after the loop, performs bitwise operations on d1 to return a masked value.
- `process_bitstream_and_hardware`: Performs two loops that manipulate bits in a destination table using values from a source table and a mask, sets a hardware register, and then performs several conditional hardware register modifications and calls another routine.
- `process_bitmask_data_2`: Checks a bit in a table based on a value in d0, then either fills a buffer with a constant pattern and writes bytes to a destination or performs a complex sequence of data transfers, XOR operations, and bitwise checks involving multiple calls to another routine.
- `process_bitmask_data`: If a subtraction of two 16-bit values is non-negative, it enters a loop that reads a longword from a source pointer in a0 and a longword from a second pointer calculated from a0 and d6, performs bitwise operations to combine them, and stores the result in a destination pointer in a1, repeating the process d6 times.
- `rotate_and_store_bits`: Decrements d6 and enters a loop that processes words from a0 by performing bit rotations and bitwise operations, then stores a byte in a1 and increments a1 until d6 is zero.
- `compare_and_branch_data`: Compares a 16-bit value at a0 to 17545, and based on the result, either branches to another routine, decrements a counter in d7 and loops, or decrements a0 by 2 and returns.
- `zeroes_data_byte_2`: Performs a logical OR operation on a byte in a data register using a zero constant.
- `configure_dma_and_wait`: Writes values to custom chip registers, waits for bit 4 of a byte at offset 3328 of a custom chip register to be set, writes a pointer and two words to custom chip registers, and then waits for bit 1 of a custom chip register to be set before writing a final value.
- `update_status_and_hardware`: Checks a byte from a global variable; if it is not zero, it returns; otherwise it sets a bit in a hardware register, calls another routine, checks a bit in a hardware register, and performs several bitwise operations on a hardware register and global variables before calling another routine.
- `perform_logical_or_operation_2`: Performs a logical OR operation on the contents of a data register using zero as the operand.
- `update_hardware_bit_2`: Checks if a byte from a global variable is less than a constant, and if so, calls a routine, performs a bitwise AND on d0, and then either sets or clears bit 2 of a hardware register at offset 256 of a pointer in a5.
- `update_counter_and_hardware`: Compares d0 with a 16-bit value from a global variable; if d0 is not equal and not less than the global value, it updates the global variable and performs a loop that modifies hardware registers and calls another routine; otherwise it performs different updates to the global variable and hardware registers.
- `clear_hardware_bit`: Increments d0 by three, loads a hardware register address into a4, loads another address into a5, clears bit 7 of the register at offset 256 of a5, clears a bit in the register at offset 256 of a5 using d0, and calls another routine.
- `update_hardware_bit`: Increments a value in d0 by 3, then sets and clears a bit at an offset determined by d0 in a table pointed to by a5, while also setting bit 7 of the same table entry.
- `save_and_call_routine`: Pushes d0, a4, and a5 onto the stack, calls another routine, and then restores the original values of a5, a4, and d0 from the stack before returning.
- `loop_and_call_routine`: Calls another routine and then loops based on a counter in d1 until it reaches negative one.
- `modify_hardware_table`: Reads a byte from a table, masks it to preserve the top two bits and set the third bit, then writes several constant values to offsets of that table and returns if a specific bit is not set.
- `set_hardware_parameters`: Saves registers to the stack, modifies a byte in a table using a bitwise mask, and writes three constant values to specific offsets in that table before restoring registers and returning d0.
- `clear_bitmask_table_entries`: Iterates through a list of 100 entries, clearing multiple longwords at specific offsets within a structure pointed to by a pointer retrieved from a global variable, and calls update_bitmask_table.
- `perform_logical_or_2`: Performs a bitwise OR operation on d0 with zero.
- `update_blitter_settings`: Checks a bit in a hardware register and a global variable, then either decrements the global variable or updates several bitplane and DMA registers and a global byte with specific values.
- `set_pointer_offset_eight`: Adds 8000 to a global variable, stores the result in a global pointer, and updates a structure at offset 54 with that pointer, offset 58 with 4225, and a global byte with 8.
- `set_pointer_offset_nine`: Adds 8000 to a global variable, stores the result in a global pointer, and updates a structure at offset 54 with that pointer, offset 58 with 4225, and a global byte with 9.
- `set_pointer_offset_six`: Adds 8000 to a global variable, stores the result in a global pointer, and updates a structure at offset 54 with that pointer, offset 58 with 4225, and a global byte with 6.
- `set_pointer_offset_ten`: Adds 8000 to a global variable, stores the result in a global pointer, and updates a structure at offset 54 with that pointer, offset 40 with 511, offset 58 with 6409, and a global byte with 10.
- `clear_blitter_structure`: Adds 8000 to a global variable, stores the result in a global pointer, and updates a structure at offset 54 with that pointer, offset 58 with 4225, offset 66 with 0, and clears a global byte.
- `configure_blitter_setup`: Checks a bit in a hardware register; if not set, it adds a constant to a value from a pointer, writes it to a hardware register, and sets several other hardware registers and a global variable.
- `set_blitter_address`: Saves registers, calculates an address by adding a constant to a value from a table, writes it to a hardware register, and sets two other hardware registers and a global variable.
- `configure_blitter_offset`: Checks a bit in a hardware register; if not set, it clears a hardware register, subtracts a constant from a value in a pointer, writes the result to a hardware register, and sets several other hardware registers and a global variable.
- `configure_blt_parameters`: Calculates a destination address from a table entry and writes it to the bitplane pointer register, then sets the bitplane depth and the bitplane size registers before setting a byte in a hardware register.
- `update_table_pointers`: Performs a bitwise AND on a byte at an offset of 5 plus d0 from a pointer in a2, loads a value from a PC-relative address into d4, and then writes that value to three consecutive global variables.
- `recalculate_table_pointers`: Calls another routine, rotates three global pointers, updates several bytes in a structure based on comparisons, and calculates a new address using multiplication and addition to store in a structure.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `check_and_return`: Clears a byte at a location relative to the program counter and returns if that byte is zero; otherwise it enters an infinite loop.
- `dispatch_subroutines`: Stores a PC-relative address into a global long word and then calls two different routines.
- `transfer_joypad_data`: Reads a word from a hardware register, splits it into two bytes, stores those bytes into two different global variables, and sets two other global variables to a constant value.
- `clear_long_words`: Clears two long words at a specific memory location.
- `wait_for_hardware`: Loops until a byte in a global variable is zero, then loops until bit 6 of a specific bitplane register is cleared.
- `check_hardware_status`: Checks bit 5 of a custom chip register, and if set, branches; otherwise it checks bit 6, and if set, branches; if neither is set, it adds 4 to a pointer from a global variable, stores that pointer into a custom chip register, and calls another routine.
- `update_hardware_and_dispatch_interrupt`: Checks hardware status bits to determine whether to update a pointer and modify bitplane-related global variables, then performs arithmetic to clamp color palette values and updates hardware registers before calling several subroutines and returning via an exception.
- `return_from_exception`: Pushes a 16-bit value from a memory location onto the stack and returns from an exception or interrupt.
- `store_byte_ten`: Sets register d0 to 10, d1 to 3, and d2 to 1, then calls another routine to store a byte into a table.
- `store_byte_thirty_three`: Sets register d0 to 33, d1 to 2, and d2 to 1, then calls another routine to store a byte into a table.
- `execute_and_decrement_pointer`: Calls a function pointer stored in a global variable, then decrements that pointer and updates it with a byte from a table, or sets it to negative one if it reaches zero.
- `store_byte_to_table`: Stores a byte from register d0 into a table at an offset calculated from register d1 and register d2 multiplied by four.
- `scale_color_palette`: Reads a word from a table, extracts its nibbles, divides them by the value in register d1, and writes the resulting nibbles back to the hardware color registers in a loop.
- `decrement_and_copy_color_palette`: Decrements a value in a global variable if it is not zero and satisfies a comparison against another global value, then calls another routine to fetch a data pointer and copies sixteen words from that pointer to the hardware color palette registers.
- `copy_word_to_multiple_locations`: Copies a 16-bit word from an offset of a structure in a0 to three different locations in memory.
- `clear_memory_block`: Clears a long word at a location from a table and then enters a loop to clear 15 more long words at that location.
- `copy_and_update_tables`: Copies 16 words from a source table to a destination table using a loop, then copies a word from a source table to a specific memory location, updates a global variable with a word from a table, copies 32 bytes from a source table to the hardware audio registers using a loop, and finally stores a word from a table into a hardware register.
- `update_bitmask_table`: Iterates through a list of values to perform bitwise operations on multiple bytes at specific offsets within a structure pointed to by a2.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `check_bit_and_store_pointer`: Tests bit zero of a long value at a memory location; if it is set, it loads a pointer from a table into a2 and stores that pointer into another memory location.
- `apply_relocation_hunks`: Calculates a destination address based on a base pointer and two bytes from a buffer, then enters a loop that reads a sequence of bytes to update a series of bitmasks in a destination structure using bitwise OR and AND operations.
- `process_relocation_hunks`: Reads a sequence of bytes from a source buffer and updates a series of bitmasks in a destination structure using bitwise OR and AND operations based on the byte values.
- `dispatch_indexed_call`: Calls a routine, adds a pointer in a3 to a2, loads a byte from a0, and if the byte is not zero and not 27, either calls another routine or calculates an offset to call a routine via a pointer in a4.
- `set_a3_zero_and_branch`: Sets address register a3 to zero, loads a constant address into a4, and branches to another routine.
- `mask_hunk_type_bits_2`: Reads a byte from a pointer in a3 and performs bitwise operations to extract specific bit patterns, then uses those patterns to clear or mask bits in various locations within a memory structure pointed to by a2.
- `initialize_jump_parameters`: Clears register a3, loads the address of a nearby constant into register a4, and branches to another routine.
- `initialize_jump_parameters_2`: Clears register a3, loads the address of the instruction following the branch into register a4, and branches to another routine.
- `call_bitmask_modifier`: Calls another routine seven times and then branches to that same routine.
- `set_a3_negative_and_branch`: Sets address register a3 to a constant value of negative 56, loads a constant address into a4, and branches to another routine.
- `update_pointer_table`: Checks a bit in a global variable; if set, it stores a value from a global variable and a register into two global variables, otherwise it calculates an offset using two bytes from a0 and a base address, then stores the resulting pointer and a register into two global variables.
- `load_value_from_table`: Subtracts 65 from a byte in a2, multiplies the result by two three times, and uses the result to load a long word from a table into d5.
- `call_indexed_routine`: Loads a byte from a pointer in a0, subtracts 65, multiplies the result by 8, and calls a routine at the address found in a table at an offset determined by the modified byte.
- `fetch_byte_from_pointer`: Performs several bitwise OR operations on register d0, calls another routine, adds the result to a2, and then loads a byte from the address in a0 into d0 before returning.
- `calculate_and_store_offset`: Reads two bytes from a0 to calculate an offset, updates a global pointer using this offset, updates another global variable using a third byte from a0, and returns that third byte.
- `set_hardware_bit_and_clear_d0`: Sets bit 1 of a bitfield located at a fixed hardware address and clears register d0 before returning.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 177 into d4, then merges 0 into d0.
- `clear_d0_byte`: Performs a bitwise OR on register d0 with zero.
- `modify_bitfield_in_structure`: Modifies several bits within a structure pointed to by a2 using a bitmask derived from register d0.
- `modify_bitmask_flags`: Reads a byte from a pointer in a3 and performs bitwise operations to clear or set specific bits at various offsets within a structure pointed to by a2, then updates the pointer in a2 by adding 40.
- `mask_hunk_type_bits`: Reads a byte from a pointer in a3 and performs bitwise operations to extract specific bit patterns, then uses those patterns to clear or mask various bits across multiple locations in a memory structure pointed to by a2.
- `fetch_and_update_pointer`: Fetches a byte from a location pointed to by a3, increments a3, sets a2 to a pointer at offset 40 of a2, and returns.
- `modify_bitmask_data`: Fetches a byte from a3 and increments a3, then performs bitwise operations to clear or set bits at specific offsets within a structure pointed to by a2.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_table_function`: Clears a bit in a global variable, fetches a byte from a0 and increments a0, uses that byte to index into a table of pointers to call a routine, and then updates a global pointer based on the result of that call.
- `call_and_branch`: Calls a routine at a specific address and then performs a short branch to another location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `iterate_function_list`: Iterates through a list of pointers starting at a5, calling each routine pointed to by the list until a null pointer is encountered.
- `set_hardware_parameters_and_call`: Writes two words to specific memory locations, then calls a routine at a fixed address.
- `set_hardware_parameters_and_call_2`: Writes two different words to specific memory locations, then calls a routine at a fixed address.
- `execute_subroutine_288`: Saves register a5 to the stack, calls another routine, restores a5, and returns.
- `execute_blitter_configuration`: Saves address register a5 to the stack, calls configure_blitter_and_run_2, restores the register, and returns.
- `call_subroutine_with_pointer`: Loads a pointer from a5 and increments a5, then calls a subroutine at a fixed address.
- `configure_blitter_and_run`: Calls a routine, then configures several hardware registers including the blitter via a6, calls the same routine again, and finally writes a value to a register.
- `configure_blitter_and_run_2`: Calls a routine, then configures several hardware registers including the blitter via a6 using different source values than the previous routine, calls the same routine again, and finally writes a value to a register.
- `configure_hardware_and_return`: Writes values to hardware registers at offsets 320 and 3584 of the structure in a5, updates two global variables, and executes an interrupt return.
- `restore_registers_and_return`: Branches to a nearby address, then loads a0 and d1 from the stack before returning.
- `execute_infinite_loop`: Enters an infinite loop that prevents further execution.
- `process_status_loop`: Writes a byte to a global variable, calls another routine twice, sets d0 to a constant, calls a third routine, and returns if the third routine does not set the zero flag.
- `check_global_status`: Checks three global variables and a global word; it either returns a specific value, calls another routine, or sets the global word and three other globals to zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_address_2`: Performs a short branch to a specific address.
- `branch_to_address`: Performs a relative branch to a specific address.
- `initialize_and_branch_routine`: Clears the d1 register and performs a relative branch to another location.
- `update_object_state_and_hardware`: Updates a structure by copying values from a source table, calculating differences between two sets of coordinates, and storing the results in a destination structure.
- `set_state_to_six`: Sets a global variable to six and returns.
- `update_object_table_and_hardware`: Iterates through a table of structures, updating their internal counters and coordinates based on complex arithmetic involving multiplication and division.
- `initialize_dma_and_memory`: Clears d0 through many operations, writes zero to a memory location pointed to by a0, sets a hardware register to one, and writes 255 to a location relative to the program counter.
- `copy_table_and_set_global_2`: Sets bit fifteen of a hardware register, copies sixty-four words from a table to a destination if a specific memory location is non-zero, and then writes a constant to a hardware register based on several sentinel checks in a table.
- `tiny_ori_5`: Small routine of 1 instruction that merges 96 into d6.
- `check_global_table_entries`: Walks a small table pointed to by a global, calling another routine for each entry that is not minus one, and makes a final check on a flag.
- `execute_conditional_branch_2`: Checks a word at a relative address, and if it is zero, performs a relative branch; otherwise it calls another routine, saves registers to the stack, calls another routine, restores registers, saves a block of registers to a global variable, and returns.
- `dispatch_hardware_parameter_update`: Increments a counter in a local variable and compares it against a limit; if the limit is reached, it performs a jump table dispatch based on a value masked from a structure in a5.
- `update_parameter_counter_and_limit`: Subtracts a byte from a value at offset 15 of a structure in a5 and updates a 16-bit value at offset 8 of a structure in a4 based on the result.
- `update_hardware_parameter_tables`: Iterates through a table of structures to update various fields based on a command type, including incrementing or decrementing counters, performing arithmetic on values, and updating pointers or offsets within the structures.
- `update_structure_value`: Subtracts a byte from a memory location at offset 12 of a structure in a5, compares the result to 113, and if the result is greater than 113, updates the value at offset 12 of the structure and copies it to offset 6 of a structure in a4 before branching to another location.
- `transfer_word_to_structure_offset`: Copies a 16-bit word from an offset of 12 in a structure pointed to by a5 to an offset of 6 in a structure pointed to by a4, then branches to another routine.
- `process_hardware_update_tables`: Decrements counters in a global structure and, if non-zero, iterates through a table to update various fields, including pointer manipulation, bitwise masking, and conditional data copying from a secondary table to a destination structure.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_hardware_and_tables`: Sets bit fifteen of the INTENA register, modifies a global word, and conditionally copies various blocks of data from tables to destination pointers based on several byte-checked flags.
- `transfer_byte_and_branch`: Copies a byte from offset 2 of a structure in a5 to offset 15 of the same structure, then branches to another routine.
- `initialize_object_pointer_table`: Calculates an index from a byte at offset 2 of a structure, scales it, and uses it to retrieve a pointer from a table to store at offset 20 of a structure.
- `update_hardware_table_entries`: Iterates through a table of entries, updating pointers and values based on bitwise operations and conditional checks.
- `update_entity_state_counters`: Checks a byte at offset 2 of a structure; if it is 1, it increments a value, updates bytes at offsets 9 and 11 of a second structure, and sets a word at offset 12 to 255.
- `update_parameter_table_entry`: Iterates through a table of structures to update various fields based on a command type, including incrementing or decrementing counters, adjusting values using a lookup table, and clamping results within specific bounds.
- `decrement_and_copy_register_state`: Decrements a global counter and, if non-zero, updates a set of limit-check variables using a bit-masked value from that counter.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_ori_6`: Small routine of 1 instruction that merges 6 into d0.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_interrupt_logic`: Checks a bit in a custom register, then either writes a value to a specific address and calls another routine before returning via an exception return, or writes a value to a global variable and returns.
- `store_routine_address`: Stores the current routine's address into a global variable and returns.
- `increment_wrapped_counters`: Increments two global counters with specific wrap-around logic at 128 and 298 respectively, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_pointers_to_globals`: Stores the value in a0 into a global variable, then stores two different addresses relative to the program counter into two other global variables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.

### Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf bytes 438784-440320 (code never loaded by a reference run)

- `initialize_memory_and_branch`: Clears a long value at a memory location using a loop that runs sixteen times, then branches to different locations based on the value of a0.
- `calculate_audio_averages`: Calculates averages from three source tables using a loop, stores the results in a destination table, and performs several memory copies and hardware register updates including a loop that copies 31 words to the hardware audio registers.
- `calculate_and_store_averages`: Calculates a value based on the sum of three 16-bit registers, performs a loop to copy 31 words from a source table to the hardware audio registers, and then performs another loop to call a routine for 32 words from a source table to a destination table.
- `calculate_scaled_values`: Calls another routine, calculates an average of three 16-bit values in d1, d2, and d3, performs a loop that calls another routine to copy 32 words from a source table to a destination table, and finally stores two words into global variables and a word into a third global variable.
- `calculate_scaled_average`: Calls another routine, sums three 16-bit values, adds two, divides the result by three, and then executes a loop that calls another routine to fill a table with 32 values from a source.
- `calculate_scaled_sum`: Calls another routine, then performs arithmetic on d1, d2, d3, and d4 to calculate a value involving division and bit shifting, and finally branches to one of two locations.
- `calculate_bitmask_logic`: Performs a series of logical AND and bit-shift operations on values in d0, d1, d2, and d3, then returns.

### Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf bytes 444928-495104 (code never loaded by a reference run)

- `update_bitmask_fields`: Reads a byte from a source pointer in a3 and performs a series of bitwise AND and OR operations on five specific byte offsets within a structure pointed to by a2.

### Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf bytes 503808-504832 (code never loaded by a reference run)

- `clear_tables_and_fill_pointers`: Zero-fills four runs of twenty words, then fills a block of addresses computed from a structure's base pointer and sets a few counter fields.
- `copy_table_and_set_global`: Copies a 64-byte table from a global variable to a table pointed to by another routine when a flag is cleared, then sets a global variable based on several sentinel checks and finally writes a constant to the global variable.

### Zeewolf 2 - Wild Justice (1995)(Binary Asylum)[cr ATX][t +18 ATX].adf bytes 882176-883200 (code never loaded by a reference run)

- `initialize_data_structures`: Clears three 20-word areas using loops, then populates a structure at a3 with values from a pointer in a0, including several calculated offsets, and finally calls a routine at a specific address.
