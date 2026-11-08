# Game 879 (hong kong phooey (1990)(hitec software)[cr angels - defjam - genesis]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00879). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908256 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 53 of 53 named (0 author symbols), 53 of 53 described (8 by rule, 44 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 777 of 777 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 949 of 952 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3024 |
| code | 3414 |
| data | 221414 |
| padding | 980 |
| unused | 679424 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Hong Kong Phooey (1990)(HiTEC Software)[cr Angels - Defjam - Genesis].adf:load | 7 | 281 | 10 |
| Hong Kong Phooey (1990)(HiTEC Software)[cr Angels - Defjam - Genesis].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 392 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 269 | 20 |

## Cases handled

- runtime segment an address: 294 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 254 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- hong kong phooey (1990)(hitec software)[cr angels - defjam - genesis]/Hong Kong Phooey (1990)(HiTEC Software)[cr Angels - Defjam - Genesis].adf:load: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `disable_interrupts_and_dma` (load): Sets a0 to a PC-relative address, writes 32767 to four consecutive word locations at offset 150 of a structure in a6, writes 135 to a specific memory location, and moves a pointer into a global variable. Kind: loader/protection initialization. Pass path: the code continues execution at the address pointed to by pcRam0007f140 after the memory copy loop completes.

## Functions


### Hong Kong Phooey (1990)(HiTEC Software)[cr Angels - Defjam - Genesis].adf:load

- `initialize_and_copy_memory`: Saves registers to the stack, calls another routine, restores registers, and then jumps to a fixed address.
- `execute_callback_loop`: Calls a routine to set a value in d6, executes a function at the address in a1, and then enters a loop that runs two passes, calling a routine and invoking a function pointer from a global variable at offset negative 210 on each pass.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `process_decompression_data`: Calls a routine via a pointer in a4 multiple times and uses the results to perform additions and memory writes within a loop.
- `load_address_and_constant`: Loads a 32-bit address into a1 using a PC-relative offset and moves the constant 2 into d0.
- `decompress_bitstream_data`: Decompresses a bitstream from a source buffer into a destination buffer using a variable-length coding scheme involving bit-shifting and conditional additions.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `disable_interrupts_and_dma`: Sets a0 to a PC-relative address, writes 32767 to four consecutive word locations at offset 150 of a structure in a6, writes 135 to a specific memory location, and moves a pointer into a global variable.
- `copy_and_jump_to_stream`: Copies 645 bytes from a source to a destination, updates two values on the stack, and performs an indirect jump to an address in a3.
- `copy_data_stream`: Reads bytes from a source pointer, repeating a byte a specified number of times into a destination starting at 8192, and updates a global variable with the last byte processed until a pointer reaches a fixed address.

### Hong Kong Phooey (1990)(HiTEC Software)[cr Angels - Defjam - Genesis].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_and_jump_routine`: Saves registers to the stack, calls a routine at a long address, restores the registers, and performs an indirect jump to a long address.
- `tiny_ori_2`: Small routine of 1 instruction that merges 31 into d0.
- `free_memory_blocks_in_loop`: Calls a routine before entering a loop that runs three passes, invoking a routine and dereferencing a pointer stored in a global variable at offset negative 210 on each pass.
- `decrement_until_minus_one_2`: A busy-wait loop that decrements a 16-bit register until it reaches minus one, then returns.
- `store_pointer_and_trap`: Stores the address of this routine in a global variable, then executes trap instruction zero and returns the register argument unchanged.
- `configure_hardware_and_update_tables`: Sets several hardware registers including DMA and interrupt control, calls several routines to process tables and keyboard input, and performs a loop that clears 16 bytes of memory starting at a specific address.
- `process_table_data`: Decrements a 16-bit value in a global variable if it is non-zero, otherwise it performs a loop that reads from a table to perform bitwise operations and writes results to a destination buffer.
- `test_and_call_routine`: Tests bit 2 of a byte at offset 22 of a structure in a5, and if it is set, calls another routine.
- `update_pointer_and_branch`: Stores the value an address in a global variable, loads a pointer into a4, loads two addresses into a3 and a6, and branches to another routine.
- `shift_data_block`: Performs a loop eight times that shifts every byte in a memory range between a fixed address and a fixed address right by one bit.
- `process_data_loop_2`: Polls bit 6 of a hardware register until it is set, calls another routine, stores a value in a global variable, and then enters a loop that calls another routine 39 times while updating a pointer and a global variable.
- `calculate_and_store_address`: Subtracts 32 from a value in d0, and if the result is not zero, shifts d0 left by 3, adds it to an address at offset 4464, calls another routine, and stores the returned address in a global variable at offset 18272.
- `tiny_ori_3`: Small routine of 1 instruction that merges 117 into d1.
- `increment_and_call_buffer`: Copies the address in a0 to a2, calls another routine, increments a0, and returns.
- `bitwise_or_buffer`: Performs a loop seven times that reads a byte from a source and bitwise ORs it into a destination byte, advancing the destination pointer by 48 bytes each iteration.
- `toggle_global_state`: Compares a value at a global pointer to a constant and, depending on the result, writes a set of different 16-bit and 32-bit values to various global locations.
- `tiny_ori_4`: Small routine of 1 instruction that merges 98 into d1.
- `tiny_ori_5`: Small routine of 1 instruction that merges 226 into d0.
- `handle_keyboard_input`: Checks bit 3 of a 16-bit value at an offset of 30 from a pointer; if set, it performs a busy-wait loop and then clears bit 5 of a global byte.
- `populate_attribute_tables`: Calls another routine and then writes a series of 16-bit values into several memory locations at various offsets, using the swap instruction to write both halves of the values.
- `increment_table_values`: Executes three separate loops that increment bytes at specific offsets within three different memory tables by 3, 2, and 1 respectively.
- `populate_graphics_attribute_tables`: Executes three sequential loops that write series of constant values into three distinct tables in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `write_constant_values`: Writes three 32-bit constants into a series of memory locations by splitting them into 16-bit halves and storing them at offsets 4, 8, 12, 16, 20, and 24 from a base address.
- `execute_repeated_calls`: Pushes the value 2 onto the stack and enters a loop that runs three times, calling two different routines in each iteration before returning.
- `process_data_loop`: Performs a loop four times that reads a byte from a calculated address, adds a constant to it, writes the result to offset 1 of a structure in a4, and updates three pointers including one that is reset to zero if it reaches 512.
- `update_table_via_loop`: Runs a loop four times to copy bytes from a source table to a destination table, updating a pointer and a counter in each iteration.
- `apply_bitwise_or_mask`: Calls another routine three times, each time passing a different pair of memory addresses and the constant 153 in d0.
- `bitwise_or_buffer_loop`: Performs a bitwise OR operation between a longword at a pointer and a longword at an offset of 644 from that pointer, storing the result in a destination buffer and repeating for a specified count.
- `initialize_scaled_table`: Fills a table starting at a specific memory location with values calculated by multiplying an incrementing counter by 48.
- `perform_triple_setup`: Performs three consecutive sequences where it loads a pointer into a6, loads a pointer into a0, clears d6 and d7, loads two bytes from a table into d7 and d6, and calls another routine.
- `setup_blt_parameters`: Calculates several values based on d6 and d7, calls another routine, and then writes a series of values to the hardware registers starting at offset 64 of a structure in a5.
- `execute_three_blt_configs`: Performs three consecutive sequences where it loads a pointer into a1, clears d6 and d7, loads two bytes from a table into d7 and d6, loads a pointer into a2, sets d0 to 2, and calls another routine.
- `configure_transfer_parameters`: Calls another routine, then adds 10116 to a2 and 676 to a1, and calls another routine before returning.
- `configure_blt_transfer`: Saves registers to the stack, calculates a destination address using d6 and d7, calls another routine, and writes several values to the hardware registers starting at offset 64 of a structure in a5 before restoring registers and returning.
- `execute_three_hardware_configs`: Performs three consecutive sequences where it loads a pointer into a2, clears d6 and d7, loads two bytes from a table into d7 and d6, loads a pointer into a1, sets d0 to 2, and calls another routine.
- `configure_transfer_addresses`: Calls another routine, adds 10116 to a1 and 676 to a2, and calls another routine before returning.
- `configure_blt_hardware`: Calculates source and destination addresses using a table and bit shifts, calls another routine, and then writes several parameters to the Blitter chipset registers via the hardware base address.
- `wait_for_bit_clear`: Enters a loop that repeatedly tests bit 14 of a byte at offset 2 of a structure in a5 until it is clear.
- `clear_bit_and_set_flag`: Clears bit 0 of a byte at the address pointed to by a0 and then sets bit 21 of a byte at offset 318 of a4.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 1 into d2, then merges 40 into d1.
- `fill_zero_data`: Executes eight consecutive instructions that perform a bitwise OR of register d0 with zero.
