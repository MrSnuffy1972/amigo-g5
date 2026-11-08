# Game 1197 (heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01197). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (911120 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 24 raw-word lines |
| Functions named and described | pass, 38 of 38 named (0 author symbols), 38 of 38 described (3 by rule, 35 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 393 of 393 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 7168 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 843 of 843 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4801 |
| code | 3084 |
| data | 896017 |
| padding | 50 |
| unused | 7168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf:bootblock | 1 | 102 | 3 |
| logged disk load from Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 741 | 35 |
| Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf bytes 1536-8704 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: custom-format (non-DOS) disk, 1744 non-empty blocks: matched against track-loaded segments
- reference crashes at an address (OSHLE RUNAWAY: PC reached an address after an address (op 4E75) A7=an address (A7)=an address, slid to exec.library -an address): 557 executed addresses of the slide through empty memory after it (an address) are not game code and are excluded from proof 4
- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: the boot block reads 512 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 299 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: bytes 1536-8704 look like code no run loaded: recovered position-independently (load address unknown)
- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: 2 logged loads cover 7680 bytes of the image
- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: blocks 23-157 (bytes 11776-80896) outside the filesystem hold data, not code (opcode density 0.042, entropy 6.88)
- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: blocks 159-159 (bytes 81408-81920) outside the filesystem hold data, not code (opcode density 0.031, entropy 1.37)
- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: blocks 161-806 (bytes 82432-413184) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.99)
- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: blocks 809-821 (bytes 414208-420864) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.16)
- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: blocks 825-879 (bytes 422400-450560) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.92)
- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: blocks 882-894 (bytes 451584-458240) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.16)
- heroes of the lance v1.0 (1988)(ssi)[cr ackerlight](disk 1 of 2)[a]/Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf: blocks 898-1759 (bytes 459776-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.76)
- 1 never-loaded region(s), 7168 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Heroes of the Lance v1.0 (1988)(SSI)[cr Ackerlight](Disk 1 of 2)[a].adf:bootblock

- `initialize_boot_environment`: Clears a hardware register, performs two synchronous I/O operations via a library call, allocates memory, and then either loops through a table to update a structure or configures hardware registers and jumps to a routine.
- `send_zero_length_io_command`: Sets a device I/O request to command code nine with a zero length and sends it synchronously.
- `apply_relocation_table_offsets`: Walks a byte-coded table and adds a base value to each longword it addresses.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_system_initialization`: Sets up a stack and a global pointer, performs two system library calls, and then enters a loop that repeatedly calls a routine to process an I/O request and executes a function from a structure at offset 28.
- `execute_indirect_calls`: Loads a pointer from a global variable, calls the routine at the address found by adding an offset of negative 330 to that pointer, and repeats this process with several other offsets and values stored in the same structure.
- `set_color_white_and_loop`: Sets the first word of the color register to all ones and enters an infinite loop.
- `set_color_gray_and_loop`: Sets the first word of the color register to 3855 and enters an infinite loop.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `call_indexed_routine`: Disables interrupts, shifts the value in d0 left by two, adds it to a base address, and calls the routine located at the resulting address before returning from an interrupt.
- `perform_multiple_bitwise_ors`: Performs a series of bitwise OR operations on the value in register d0 using various constant byte values and then returns.
- `clear_memory_blocks`: Calculates a loop count based on a 16-bit value in register d1, then repeatedly clears specific offsets within a structure pointed to by a global variable until the counter reaches zero.
- `calculate_buffer_offset`: Saves registers to the stack, performs arithmetic using values from two global variables and a constant, and branches based on the result.
- `apply_bitmask_to_buffer_2`: Calculates a memory offset and a bitwise rotation amount based on input values, then enters a loop that applies a bitwise-inverted and rotated mask to a series of 32-bit values stored in a buffer, updating a destination buffer through a pointer.
- `apply_bitmask_to_buffer`: Iterates through a list of four 32-bit values, applies a bitwise rotation to each based on a shift amount derived from a register, and updates a destination buffer by combining the rotated values with the existing buffer content using a calculated bitmask.
- `initialize_memory_range`: Sets three global variables to specific memory addresses and clears a block of memory at the third address using a loop of 8192 iterations.
- `configure_hardware_registers`: Writes specific values to several hardware registers, including bit manipulation on a register read from a hardware address, and stores a value from a hardware address into a global variable.
- `toggle_and_add_constant`: Toggles the least significant bit of a global variable and then adds 48001 to it.
- `transform_and_store_data`: Reads a word from a list in a0, doubles it, adds 273, and stores the result into a series of words at a hardware register address, repeating this 16 times.
- `unpack_bit_shifted_word_data`: Validates a word value against a range to determine a destination offset and a bit-shift amount, then enters a loop to extract and store bit-shifted words from a source table into a destination buffer.
- `distribute_bit_shifted_word_data_2`: Validates an input value against a range, calculates a destination offset and a bit-shift amount, and then enters a loop to extract words from a source table and distribute them into four separate destination buffers using the calculated shift.
- `distribute_bit_shifted_word_data`: Extracts words from a source table, applies a bitwise right shift based on a calculated value, and distributes the resulting words into four separate destination memory areas using nested loops.
- `distribute_word_data`: Performs a nested loop to distribute word data from a source into four different memory destinations with increasing offsets.
- `distribute_byte_data`: Performs a nested loop to distribute byte data from a source into four different memory destinations with increasing offsets and step increments.
- `execute_disk_io`: Sets up a structure with a command, a count, and two buffer addresses, then calls a function via a pointer in a global variable to perform an I/O operation.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_do_i_o_2`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `setup_io_request`: Sets a 16-bit value at offset 28 of a global variable, writes 3 to offset 1c of a structure pointed to by a global variable, sets offsets 24 and 2c of that structure to 512 and 1024 respectively, and calls another routine.
- `compare_null_terminated_strings`: Compares two strings pointed to by a0 and a1, returning 65535 if a mismatch occurs or if the loop counter reaches -1, otherwise it returns a 16-bit value composed of the last two bytes read.
- `iterate_string_table`: Iterates through a table of pointers, calling a routine to compare two strings at each entry, and returns -1 if a null pointer is encountered, a mismatch occurs, or the loop counter reaches -1.
- `update_structure_values`: Updates several fields in a structure pointed to by a global variable using values from other offsets within that structure, calls a routine via a pointer, and may recursively call itself if a specific value is found at a global variable.
- `setup_io_request_scaled`: Sets offset 28 of a global variable to itself, writes 3 to offset 1c of a structure pointed to by a global variable, and sets offsets 24 and 2c of that structure using values from offsets 0e and 0c of a global variable multiplied by 512, then calls another routine.
- `process_offset_table`: Reads a value from a calculated pointer, then enters a loop that adds a constant to values in a table and clears bytes, using the cleared byte to determine the next offset to jump to until a zero byte is encountered.
- `clear_io_request`: Writes 9 to offset 36 of a structure pointed to by a global variable, clears the 32-bit value at offset 36 of that structure, and calls another routine.
- `process_io_sequence`: Calls a routine to return a value to a global variable, then calls another routine to check a string table; if the result is non-negative, it updates a global variable and calls a third routine before finally calling a fourth routine to clear an I/O request.
- `initialize_disk_io_request`: Stores a pointer from a1 into the stack, updates a global variable with a pointer from a0, and calls another routine.
- `copy_string_and_setup_io`: Sets up an I/O request, then if a value from a routine is negative, it copies a string from one location to another using a loop; it then updates a 16-bit value at an offset of 14 in a global variable and calls three separate routines.
- `apply_font_mask_to_buffers`: Iterates through a string of characters, looking up each character in a font table and applying its bitmask to four separate memory buffers using bitwise OR or AND operations based on flags in a register.
- `execute_system_traps`: Performs a sequence of five system traps with various parameters in d0, d1, and a0, including one call to a routine at an address in a register.
