# Game 805 (aaargh! (1987)(electronic arts)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00805). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (912256 top-level bytes in 58 containers) | pass, 100.000%, 0 unknown |
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
| data | 148768 |
| padding | 13790 |
| unused | 3072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Aaargh! (1987)(Electronic Arts).adf:pc | 3 | 214 | 16 |
| add21k | 3 | 480 | 33 |
| disk-validator | 1 | 386 | 1 |
| Aaargh! (1987)(Electronic Arts).adf:ex | 3 | 161 | 13 |
| Aaargh! (1987)(Electronic Arts).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 574 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 50 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| Aaargh! (1987)(Electronic Arts).adf:gwave.dat (raw code file, load address unknown) | 1 | 0 | 0 |
| Aaargh! (1987)(Electronic Arts).adf:arg.bin (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- runtime segment an address: 189 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 28 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- aaargh! (1987)(electronic arts)/Aaargh! (1987)(Electronic Arts).adf:gwave.dat: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- aaargh! (1987)(electronic arts)/Aaargh! (1987)(Electronic Arts).adf:arg.bin: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 66004 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_registers` (pc): Reads four 16-bit values from a table and stores them in specific offsets of a structure in a4, writes several 16-bit constants to hardware registers, and sets bit 6 and bit 7 of a hardware register via an OR operation. Kind: disk hardware register configuration/initialization. Pass path: the execution continues to the subsequent JSR calls (jsr an address(pc) and jsr an address(pc)).
- `clear_bits_then_call_routine` (pc): Clears bit 7 in a global variable, then loops 100 times clearing bit 3 of the same global variable before calling another routine. Kind: checksum/bit-manipulation trap. Pass path: u0__sub_0_01FC.
- `wait_while_port_bit_set` (pc): While bit 4 of a global status byte is set, sets the low two bits to three and calls one further routine, touching the CIA port A and port B registers. Kind: disk hardware/timing check. Pass path: the loop terminates when the bit is cleared, allowing execution to proceed to the instructions following the call site.
- `spin_on_status_then_skip` (pc): Spins while bit 4 of a CIA port A register is set, otherwise it enters a loop that clears bits 0 and 1 of a global status byte and calls another routine. Kind: status spin loop / hardware sync check. Pass path: the instructions following the call (bclr.b #an address, an address.l).
- `initialize_hardware_and_wait` (pc): Calls a routine, clears a bit in a CIA register, writes values to a structure, calls another routine, and then checks a bit in a structure to write a value to an offset of that structure. Kind: disk hardware/track check. Pass path: the loop continues (iVar1 reaches 0) and the code proceeds to the next instruction after the call.
- `update_hardware_control_bits` (pc): Sets and clears bits in a hardware register, then reads four 16-bit values from a table and stores them in a hardware register after setting the high bit of each value. Kind: disk hardware status/control bit update. Pass path: the routine continues execution normally (the instructions following the call are the standard flow).
- `set_hardware_registers` (ex): Writes a 16-bit value to an offset of the stack pointer, performs a bitwise OR on a global variable, and returns. Kind: hardware register configuration/sync check. Pass path: execution continues to the next instruction after the call.
- `fill_table_with_random_bytes` (runtime segment an address): Writes 255 to the blitter OR plane register and then fills a 2047-byte table at offset an address with random bytes by calling another routine in a loop. Kind: checksum/integrity trap. Pass path: the loop continues normally (sVar2 decrements and loop repeats).

## Functions


### Aaargh! (1987)(Electronic Arts).adf:pc

- `initialize_system_and_strings`: Calls three different routines using a library base, then calls another routine and writes a byte to a memory location.
- `call_library_function`: Copies a byte from a pointer in a0 to d3, stores a pointer in d2, and calls a function at an offset of negative 48 from a library base address found via a PC-relative offset.
- `open_dos_library`: Calls a function at an offset of negative 552 from a library base address and stores the returned pointer in a global variable.
- `execute_multiple_subroutines`: Saves registers to the stack, calls eight different routines, loads a byte from an offset of a structure in a4 into register d0, and restores the registers before returning.
- `configure_hardware_registers`: Reads four 16-bit values from a table and stores them in specific offsets of a structure in a4, writes several 16-bit constants to hardware registers, and sets bit 6 and bit 7 of a hardware register via an OR operation.
- `clear_bits_then_call_routine`: Clears bit 7 in a global variable, then loops 100 times clearing bit 3 of the same global variable before calling another routine.
- `delay_then_wait_port_bit`: Runs a fixed countdown of three thousand iterations, then spins until bit five of a global variable that holds the CIA port A status is clear.
- `wait_while_port_bit_set`: While bit 4 of a global status byte is set, sets the low two bits to three and calls one further routine, touching the CIA port A and port B registers.
- `spin_on_status_then_skip`: Spins while bit 4 of a CIA port A register is set, otherwise it enters a loop that clears bits 0 and 1 of a global status byte and calls another routine.
- `initialize_hardware_and_wait`: Calls a routine, clears a bit in a CIA register, writes values to a structure, calls another routine, and then checks a bit in a structure to write a value to an offset of that structure.
- `clear_and_set_values`: Clears a 16-bit value at an offset of 36 from a6 and sets a 16-bit value at an offset of 150 from a6 to 16.
- `wait_for_status_bit_set`: Spins until bit four of a global status byte is set, where the byte mirrors the CIA B interrupt control status.
- `update_hardware_control_bits`: Sets and clears bits in a hardware register, then reads four 16-bit values from a table and stores them in a hardware register after setting the high bit of each value.
- `search_bit_pattern_in_table`: Iterates through a table of 16-bit words, performing bit shifts and comparisons to find a specific pattern, then sets a global byte to 0 or 1 based on the search result.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.

### add21k

- `call_clear_block_and_store_pointers`: Calls clear_block_and_store_pointers, which zeroes forty-one 32-bit words at an offset from a base pointer retrieved from the stack and then stores the address of a small stack buffer.
- `configure_display_surface`: Looks up two handles through a lookup routine, releasing them via library stubs if a lookup fails, then sets a flag byte in an associated display structure and calls a routine with the sizes 640 and 256 before passing the stored handles to further calls.
- `clear_block_and_store_pointers`: Zeroes forty-one 32-bit words starting at an offset from a base pointer taken from the stack, then stores the address of a small stack buffer and a global value into two fields of that same block.
- `call_optional_handler_and_recurse`: Checks if a value at a negative offset from a register is non-zero to call a function pointer, then calls itself recursively.
- `parse_script_and_initialize_objects`: Splits a text buffer into a null-terminated array of word pointers using a character class table and replaces separators with null bytes.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `initialize_state_then_return`: Calls another routine ten times, then calls an optional handler from a global and conditionally calls routines with values taken from globals.
- `set_d0_and_branch`: Sets register d0 to 32767 and branches to another routine.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination and returns it.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `process_indexed_table_entry`: Calculates an address by multiplying a word from a stack offset by 6 and adding it to a base address in a4.
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
- `jump_to_library_function_2`: Performs an indirect jump to a function located via a library base and a different offset.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_library_vector_384_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### disk-validator

- `format_and_update_display_buffer`: Converts a value from a structure into a character string and writes it to a buffer at a calculated offset, then performs a series of bitwise operations and memory writes to update a data structure using multiple calls to other routines.

### Aaargh! (1987)(Electronic Arts).adf:ex

- `parse_filename_and_open_library`: Copies bytes from a source buffer to a destination buffer, appending a specific five-byte string if no period is found in the source, then calls a function via a pointer in a global variable.
- `open_file_and_check_status`: Stores a value in a local structure, calls a routine using a pointer from that structure, and then calls another routine using a pointer from the structure if the first call returns zero.
- `execute_conditional_handler`: If the negative flag is not set, loads three pointers from a table, calls a library function using a pointer from a second table, and if the result is non-negative, clears a global word; otherwise it clears a word at an offset of a third table and calls another routine.
- `execute_library_function`: Loads a longword from offset 4 of the structure in a0 into d1, calls a library function via a pointer in a6, and then performs conditional calls to another routine based on values loaded from offset 8 of a structure pointed to by the program counter.
- `perform_nested_loops`: Writes the contents of register d0 to the color register, then executes two nested loops that perform bitwise OR operations with zero on a global variable before returning.
- `fill_zero_data`: Performs five consecutive bitwise OR operations with zero on register d0.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `test_processor_status_flags`: Checks the state of the negative, zero, overflow, and carry flags in the status register and returns based on their combination.
- `fill_zero_data_long`: Performs twenty-nine consecutive bitwise OR operations with zero on register d0.
- `fill_zero_data_short`: Performs three consecutive bitwise OR operations with zero on register d0.
- `protec`: Calls a sequence of eight different routines and returns a value formed by combining a byte from memory with the contents of register d1.
- `write_stack_values`: Performs a bitwise OR on the byte at the address in a0, sets register d0, and writes four 16-bit values to offsets 22, 26, 28, and 30 of the stack pointer.
- `set_hardware_registers`: Writes a 16-bit value to an offset of the stack pointer, performs a bitwise OR on a global variable, and returns.

### Aaargh! (1987)(Electronic Arts).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_clear_memory`: Checks a global variable and, if not zero, clears a block of memory starting at a specific address, then calls initialize_audio_and_dma, update_interrupt_register, and fill_table_with_random_bytes before performing several recursive calls to itself and other routines.
- `initialize_audio_and_dma`: Clears four 16-bit values at offsets 32, 96, 160, and 224 of a table, writes a 32-bit value to an audio register, writes four 16-bit values to an audio register, writes a 16-bit value to an offset of a table, writes a 16-bit value to a table offset, clears a global variable, and writes a 16-bit value to an audio register.
- `process_audio_command_list`: Iterates through a linked list of command structures, updating a global index and extracting parameters to modify audio hardware registers and DMA settings.
- `update_interrupt_register`: Copies a long value to a global variable, sets that global variable to a new long value, and writes the word 32880 to the INTENA hardware register.
- `copy_global_pointer`: Copies a 32-bit value from a global variable to another global variable and returns.
- `handle_interrupt_request`: Checks the interrupt request register to set specific values in the audio hardware registers, calls the random generator, the audio routine, and the joystick routine, then decrements three values in a table and handles a blitter interrupt.
- `read_joystick_buttons`: Reads the joystick data register and sets bits of a result depending on which button bits are set, setting bit 0 for bit 1, bit 1 for bit 9, bit 2 for bit 1, and bit 3 for bit 9 when the corresponding conditions hold.
- `process_joystick_buttons`: Calls a routine to read joystick data, then checks the resulting bits to set various flag bytes at offsets 27570, 27572, 27578, 27582, 27584, 27586, 27588, 2758a, 2758e, and 27590 of a structure, while also writing to offset 27ea4.
- `configure_audio_channel`: Writes an audio register, sets a timer period and a DMA enable word, then clears a global variable, configuring audio channel three.
- `update_audio_channels`: Checks two flags, then loads sound data from several tables into audio hardware registers, advancing per-channel pointers and decrementing their counters, while also configuring the noise channel and writing a random value to the third audio channel.
- `fill_table_with_random_bytes`: Writes 255 to the blitter OR plane register and then fills a 2047-byte table at offset an address with random bytes by calling another routine in a loop.
- `advance_random_state`: Reads a global variable, increments it when zero, shifts it left by one, XORs it with a constant when negative, and stores the result back.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_d0_to_global`: Stores the contents of register d0 into a global variable.
- `wait_and_set_interrupt_bit`: Waits until bit 5 of the interrupt request status register is set, then sets bit 5 of the interrupt request register and returns.
- `call_super_state`: Wrapper that calls the exec library function SuperState and returns its result to the caller.
- `call_library_function_2`: Loads a function pointer from an offset within a library structure and executes it.
- `clear_audio_and_dma`: Clears four audio volume registers, sets a global word to 65535, clears a global word, and if a global word is not less than 2, clears several DMA and audio registers and sets a global word to 15.
- `set_audio_volumes`: Writes the value 65535 to six different memory locations and then branches to another routine.
- `setup_blitter_and_dma`: Calls a routine, then writes values to the bitplane modification registers, the bitplane control registers, the display start address register, and the DMA control register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
