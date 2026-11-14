# Game 879 (hong kong phooey (1990)(hitec software)[cr angels - defjam - genesis]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00879). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (908272 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 12 raw-word lines |
| Functions named and described | pass, 48 of 48 named (0 author symbols), 48 of 48 described (3 by rule, 24 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1015 of 1015 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1048 of 1048 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3024 |
| code | 3640 |
| data | 221200 |
| padding | 984 |
| unused | 679424 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Hong Kong Phooey (1990)(HiTEC Software)[cr Angels - Defjam - Genesis].adf:load | 7 | 414 | 13 |
| Hong Kong Phooey (1990)(HiTEC Software)[cr Angels - Defjam - Genesis].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 357 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 254 | 16 |

## Cases handled

- runtime segment an address: 342 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 254 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- hong kong phooey (1990)(hitec software)[cr angels - defjam - genesis]/Hong Kong Phooey (1990)(HiTEC Software)[cr Angels - Defjam - Genesis].adf:load: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `disable_interrupts_and_dma` (load): Sets a0 to a PC-relative address, writes 32767 to four consecutive word locations at offset 150 of a structure in a6, writes 135 to a specific memory location, and moves a pointer into a global variable. Kind: loader/protection initialization. Pass path: the code continues execution at the address pointed to by pcRam0007f140 after the memory copy loop completes.

## Functions


### Hong Kong Phooey (1990)(HiTEC Software)[cr Angels - Defjam - Genesis].adf:load

- `initialize_and_copy_memory`: Saves registers to the stack, calls another routine, restores registers, and then jumps to a fixed address.
- `process_decompression_data`: Calls a routine via a pointer in a4 multiple times and uses the results to perform additions and memory writes within a loop.
- `load_address_and_constant`: Loads a 32-bit address into a1 using a PC-relative offset and moves the constant 2 into d0.
- `decompress_bitstream_data`: Decompresses a bitstream from a source buffer into a destination buffer using a variable-length coding scheme involving bit-shifting and conditional additions.
- `or_then_store_registers`: Ors d0 with itself and with a constant, stores the low byte of d0 and the byte of d1 through the predecrementing pointer in a4, stores the word of d2 there as well, then reloads several registers from the stack and branches to another routine.
- `disable_interrupts_and_dma`: Sets a0 to a PC-relative address, writes 32767 to four consecutive word locations at offset 150 of a structure in a6, writes 135 to a specific memory location, and moves a pointer into a global variable.
- `copy_and_jump_to_stream`: Copies 645 bytes from a source to a destination, updates two values on the stack, and performs an indirect jump to an address in a3.
- `copy_data_stream`: Reads bytes from a source pointer, repeating a byte a specified number of times into a destination starting at 8192, and updates a global variable with the last byte processed until a pointer reaches a fixed address.
- `decode_compressed_bitstream_to_buffer`: Copies an initial block of bytes from one buffer to another, then repeatedly reads bits from a 32-bit bit reader and expands them into a decoded byte stream written backwards from a pointer, looping until that pointer reaches a bound.
- `extract_bits_from_long_backwards`: Reads a longword backwards from the address in a0, stores its low byte in the word at the address in a5, sets the extend flag from the value 16 in the condition code register and rotates d0 right through the extend bit.
- `shift_register_bits_loop`: Repeatedly shifts the data register right by one bit, decrementing a loop counter initialised from the 16-bit value in the d1 register until it reaches -1.
- `execute_callback_loop`: Calls a routine to set a value in d6, executes a function at the address in a1, and then enters a loop that runs two passes, calling a routine and invoking a function pointer from a global variable at offset negative 210 on each pass.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Hong Kong Phooey (1990)(HiTEC Software)[cr Angels - Defjam - Genesis].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trap_zero_with_saved_registers`: Saves registers d0-d7 and a0-a6 on the stack, stores its own entry address into the interrupt vector at offset 128 (vector 32, trap #0), issues trap #0 to invoke the handler for that vector, then restores all saved registers and returns with d0 cleared to zero.
- `shutdown_and_restore_system`: Saves the current DMA and interrupt enable words into global variables, disables all DMA and interrupts, resets the display window, data fetch and bitplane modulo registers, clears the sprite pointer registers and the copper jump register, then re-enables the saved DMA and interrupt bits and returns.
- `fill_table_with_words`: Decrements a global counter and returns while it is nonzero; otherwise it reads words from a source pointer, resets the pointer when a marker value is seen, and writes 32-bit values and masked words into a table, looping 177 times before returning.
- `conditionally_shift_bytes_right`: Tests bit 2 of the byte at offset 22 from the a5 register and, if it is clear, branches to the return.
- `stores_constant_and_loads_pointers`: Writes the 32-bit constant an address to a global variable, loads a4 from a table entry, and loads the addresses of two tables into a3 and a6 using pc-relative offsets.
- `shift_bytes_right_seven_passes`: Runs eight passes over a memory range of bytes in a global block, logically shifting each byte right by one bit and writing it back, then returns.
- `wait_then_clear_memory_block`: Polls the CIAA_PRA hardware register until bit 6 is set, then calls another routine, stores a 32-bit constant in a global variable, and walks a memory range one byte at a time, clearing each byte through a pointer and reloading the constant plus 480 after every 40 bytes until the pointer reaches a fixed end address.
- `store_sprite_pointer_by_index`: Subtracts 32 from the value in d0, and if the result is non-zero scales it by eight and computes a pointer into a table, adding the offset to a base address.
- `copy_pointer_and_or_column`: Copies the destination pointer in a0 to a2 and calls another routine that ORs a column of bytes into the table, then advances a0 by one byte and returns.
- `or_column_bytes_into_table`: Saves a1 on the stack, then eight times ORs the byte at the source pointer into the byte at the destination pointer, advancing the destination by 48 bytes and the source by one byte, and restores a1 before returning.
- `swap_two_global_word_sets`: Compares a global word against a fixed constant and, depending on the result, writes one set of constants into several global words and long words, or a second set of constants with different values, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (movea.l) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `pulse_keyboard_serial_handshake`: If bit 3 of the word at offset 30 of the structure in a5 is set, reads the keyboard serial register, sets bit 6 of the keyboard control register, busy-waits for a 16-bit countdown, then clears bit 6 of the keyboard control register.
- `initialise_sprite_table_entries`: Calls another routine to fill the sprite tables, then writes a fixed set of 16-bit words and zero words to successive global variable locations, pairing each constant with a zero word.
- `add_offsets_to_byte_columns`: Adds 3 to one byte in each of 11 entries spaced 64 bytes apart starting at a fixed address, then adds 2 to one byte in each of 18 entries from a second start address, then adds 1 to one byte in each of 18 entries from a third start address, returning afterward.
- `fill_three_sprite_tables`: Writes three successive tables of 16-bit words in memory, each made of 18 blocks of 16 longwords (the first with 11 blocks), where every block stores a fixed set of pattern longwords and a running value that is advanced by adding a fixed address and combining bytes read from the video position register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_keyboard_interrupt_byte`: Checks the CIA-A interrupt flag for a serial data event, and if set, reads the keyboard serial register, toggles its handshake bit in the CIA control register around a write of 255 to the serial register, stores the inverted and rotated byte in a global variable, then clears the handshake bit.

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
