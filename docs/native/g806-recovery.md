# Game 806 (aaargh! (1987)(electronic arts)[cr htl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00806). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (912256 top-level bytes in 57 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 20 raw-word lines |
| Functions named and described | pass, 84 of 84 named (0 author symbols), 84 of 84 described (25 by rule, 57 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 617 of 617 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 66004 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1851 of 1876 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 739468 |
| code | 7158 |
| data | 148224 |
| padding | 13310 |
| unused | 4096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Aaargh! (1987)(Electronic Arts)[cr HTL].adf:pc | 3 | 214 | 16 |
| add21k | 3 | 480 | 33 |
| disk-validator | 1 | 386 | 1 |
| Aaargh! (1987)(Electronic Arts)[cr HTL].adf:ex | 3 | 161 | 13 |
| Aaargh! (1987)(Electronic Arts)[cr HTL].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 574 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 50 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Aaargh! (1987)(Electronic Arts)[cr HTL].adf:gwave.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| Aaargh! (1987)(Electronic Arts)[cr HTL].adf:arg.bin (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 189 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 28 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- aaargh! (1987)(electronic arts)[cr htl]/Aaargh! (1987)(Electronic Arts)[cr HTL].adf:gwave.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- aaargh! (1987)(electronic arts)[cr htl]/Aaargh! (1987)(Electronic Arts)[cr HTL].adf:arg.bin: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 66004 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_registers` (pc): Reads several hardware registers into a stack-based structure and writes specific constant values to various hardware registers including DMA and interrupt control. Kind: disk hardware register configuration. Pass path: jsr an address(pc).
- `clear_bits_then_call_routine` (pc): Clears bit 7 in a CIA port B register, loops 100 times clearing bit 3 in the same register, and then calls another routine. Kind: checksum/bit-manipulation trap. Pass path: u0__sub_0_01FC.
- `wait_while_port_bit_set` (pc): While bit 4 of CIA port A is set, sets the low two bits of CIA port B to three and calls another routine. Kind: disk hardware/timing check. Pass path: the loop terminates when the bit is cleared, allowing execution to proceed to the instructions following the call site.
- `spin_on_status_then_skip` (pc): Spins while bit 4 of a CIA port A register is set, otherwise it enters a loop that clears bits 0 and 1 of a CIA port B register and calls another routine. Kind: status polling loop/retry mechanism. Pass path: the loop terminates when the status bit is cleared, proceeding to the instructions following the call site.
- `initialize_hardware_state` (pc): Calls another routine, clears bit 2 of a CIA register, writes values to several memory locations, calls another routine, and then checks a bit in a memory location to write a value to an offset of a structure. Kind: disk hardware/track check. Pass path: the loop continues or exits via the break condition, allowing the code to proceed to the next instruction after the JSRs.
- `set_register_bits` (pc): Sets and clears specific bits in a hardware register and then sets the high bit of four different 16-bit values stored in a stack-based structure before writing them to hardware registers. Kind: checksum or data integrity trap. Pass path: the routine continues execution normally if the bitwise operations do not trigger a crash or incorrect state.
- `set_hardware_bits` (ex): Writes a 16-bit value to an offset of a stack pointer, performs a logical OR on a global variable, and returns. Kind: hardware register configuration/timing setup. Pass path: execution continues to the disk read/DMA routine.
- `fill_table_with_random_bytes` (runtime segment an address): Writes 255 to a hardware register and then fills a 2047-byte table at offset an address with random bytes by calling another routine in a loop. Kind: checksum/integrity trap. Pass path: the loop completes and execution continues to the next instruction after the call site.

## Functions


### Aaargh! (1987)(Electronic Arts)[cr HTL].adf:pc

- `initialize_system_libraries`: Calls a routine to save the stack pointer, calls two library functions using a pointer from a global variable, calls a third routine, and then either writes a byte to a memory location and calls another routine, or writes the string 'Tammy' followed by 'y   ' to a memory location and returns.
- `call_library_function`: Copies a byte from a pointer in a0 into d3, stores the pointer in d2, and calls a function at an offset from a library base address retrieved from a table.
- `open_dos_library`: Calls a library function to open a library and stores the resulting pointer in a global variable.
- `execute_initialization_sequence`: Saves registers to the stack, calls eight different routines in sequence, moves a byte from an offset of a structure in a4 into register d0, and restores the registers before returning.
- `configure_hardware_registers`: Reads several hardware registers into a stack-based structure and writes specific constant values to various hardware registers including DMA and interrupt control.
- `clear_bits_then_call_routine`: Clears bit 7 in a CIA port B register, loops 100 times clearing bit 3 in the same register, and then calls another routine.
- `delay_then_wait_port_bit`: Runs a fixed countdown of three thousand iterations, then spins until bit five of a global variable that holds the CIA port A status is clear.
- `wait_while_port_bit_set`: While bit 4 of CIA port A is set, sets the low two bits of CIA port B to three and calls another routine.
- `spin_on_status_then_skip`: Spins while bit 4 of a CIA port A register is set, otherwise it enters a loop that clears bits 0 and 1 of a CIA port B register and calls another routine.
- `initialize_hardware_state`: Calls another routine, clears bit 2 of a CIA register, writes values to several memory locations, calls another routine, and then checks a bit in a memory location to write a value to an offset of a structure.
- `clear_and_set_values`: Clears a word at an offset of 36 from a6 and sets a word at an offset of 150 from a6 to 16.
- `wait_for_status_bit_set`: Spins until bit four of a global status byte is set, where the byte mirrors the CIA B interrupt control status.
- `set_register_bits`: Sets and clears specific bits in a hardware register and then sets the high bit of four different 16-bit values stored in a stack-based structure before writing them to hardware registers.
- `search_bit_pattern`: Iterates through a table of 14,808 words, performing bit shifts and comparisons to find a specific 16-bit pattern, then updates a global byte based on the result and a counter.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.

### add21k

- `call_clear_block_and_store_pointers`: Calls a routine that zeroes forty-one 32-bit words at an offset from a base pointer taken from the stack and stores the address of a small stack buffer.
- `configure_display_surface`: Looks up two handles through a lookup routine, releasing them via library stubs if a lookup fails, then sets a flag byte in an associated display structure and calls a routine with the sizes 640 and 256.
- `clear_block_and_store_pointers`: Zeroes forty-one 32-bit words starting at an offset from a base pointer taken from the stack, then stores the address of a small stack buffer and a global value into two fields of that same block, and finally calls another routine.
- `call_optional_handler_and_recurse`: Checks if a value at an offset from a pointer is non-zero, calls the function at that pointer, pushes a value from a stack to the stack, and calls another routine.
- `parse_script_and_initialize_objects`: Splits a text buffer into a null-terminated array of word pointers using a character class table and replaces separators with null bytes.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `initialize_state_then_return`: Calls another routine ten times, then calls an optional handler from a global and conditionally calls routines with values taken from globals.
- `set_d0_and_branch`: Sets register d0 to 32767 and branches to another routine.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination and returns it.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `process_indexed_structure`: Calculates an address based on a value from a structure at offset 8 of the stack frame, checks if that value is between 0 and 19, and if the pointer at the calculated address is non-zero, it either calls another routine or clears the memory at that address.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `jump_to_library_function`: Performs an indirect jump to a function located via a library base and an offset.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `jump_to_library_function_alt`: Performs an indirect jump to a function located via a library base and a different offset.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### disk-validator

- `update_disk_validation_status`: Calculates a character representation of a value from a global structure, writes it to a buffer, and performs a series of bitwise operations and memory writes to update a bitmask and status flags within a data structure.

### Aaargh! (1987)(Electronic Arts)[cr HTL].adf:ex

- `parse_filename_and_open_library`: Copies characters from a source buffer to a destination buffer, appending a specific five-character extension if no period is found, then calls a routine via a pointer in a global variable.
- `handle_file_error`: Stores a value in a structure, calls a library function using a pointer from that structure, and if the result is zero, calls another library function using a pointer from the same structure and then calls a third routine.
- `execute_conditional_handler`: If register d0 is not negative, loads three values from a table, calls a library function, and if d0 remains non-negative, clears a global word and calls another routine.
- `execute_library_function`: Calls a library function using a pointer from a structure in a0, then loads a word from a table into d0 and calls another routine; if a long value at the same table offset is non-zero, it loads that long value into d0 and calls the same routine again before returning.
- `write_color_and_loop`: Writes the contents of register d0 to the color register, then enters two nested loops that perform a total of 32,768 iterations before calling another routine and returning.
- `fill_zero_data`: Performs five consecutive logical OR operations on register d0 with zero.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `perform_zero_operations`: Performs twenty-eight consecutive logical OR operations on register d0 with zero and returns.
- `fill_zero_table`: Performs twenty-eight consecutive logical OR operations on register d0 with zero.
- `fill_zero_small_table`: Performs three consecutive logical OR operations on register d0 with zero.
- `protec`: Calls a sequence of eight different routines and then returns a 32-bit value formed by a byte from a memory location and the contents of register d0.
- `write_hardware_parameters`: Sets a byte at the address in a0, sets register d0, and writes four 16-bit values to offsets of a stack pointer.
- `set_hardware_bits`: Writes a 16-bit value to an offset of a stack pointer, performs a logical OR on a global variable, and returns.

### Aaargh! (1987)(Electronic Arts)[cr HTL].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_randomize_table`: Initializes audio and DMA settings, updates the interrupt mask by writing to the interrupt enable register, and fills a two-thousand-and-forty-seven-byte table with random bytes after writing two hundred and fifty-five to a hardware register.
- `initialize_audio_and_dma`: Clears four 16-bit values at offsets of a global table, writes four 32-bit values to a hardware register table, writes four 16-bit values to that same hardware register table, writes a 16-bit value to an offset of a global table, clears a global variable, and writes a 16-bit value to a hardware register.
- `process_audio_command_list`: Iterates through a list of command structures, updating a counter and fetching data from a table to configure audio hardware and DMA settings.
- `update_interrupt_mask`: Stores a value into a global variable and then writes the value 32880 to the INTENA register.
- `copy_long_word`: Copies a 32-bit value from a global variable to another global variable and returns.
- `handle_interrupt_request`: Checks the interrupt request register to set specific hardware register values, calls the random state, audio, and joystick routines, decrements three values in a table, and handles a blitter interrupt by updating a pointer and a counter.
- `read_joystick_buttons`: Reads the joystick data register and sets bits of register d1 depending on which button bits are set, setting bit 0 for bit 1, bit 1 for bit 9, bit 2 for bit 1, and bit 3 for bit 9 when the corresponding conditions hold.
- `process_joystick_buttons`: Calls a routine to read joystick data, then checks various bits in the result to set or clear specific flag bytes at offsets 27570, 27572, 27578, 27580, 27582, 27584, 27586, 27588, 2758a, 2758c, 2758e, and 27590 of a structure, while also clearing bit 7 of a CIA register.
- `configure_audio_channel`: Writes an audio register, sets a timer period and a DMA enable word, then clears a global variable, configuring audio channel three.
- `update_audio_channels`: Checks two flags, then loads sound data from several tables into audio hardware registers, advancing per-channel pointers and decrementing their counters, while also configuring the noise channel and writing a random value to the third audio channel.
- `fill_table_with_random_bytes`: Writes 255 to a hardware register and then fills a 2047-byte table at offset an address with random bytes by calling another routine in a loop.
- `advance_random_state`: Reads a global variable, increments it when zero, shifts it left by one, XORs it with a constant when negative, and stores the result back.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_d0_to_global`: Stores the contents of register d0 into a global variable and returns.
- `wait_and_set_interrupt_bit`: Waits until bit 5 of the interrupt request status register is set, then sets bit 5 of the interrupt request register and returns.
- `call_super_state`: Wrapper that calls the exec library function SuperState and returns its result to the caller.
- `call_library_function_2`: Loads a function pointer from an offset within a library structure and executes it.
- `clear_audio_and_dma`: Clears four audio volume registers, sets a global word to 65535, clears a global word, and if a global word is not less than two, clears several DMA and audio registers and sets a global word to 15.
- `set_audio_volumes`: Writes the value 65535 to six different memory locations and then branches to another routine.
- `configure_hardware_registers_2`: Calls another routine, then writes values to three hardware registers and two additional registers at offsets 104 and 106.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
