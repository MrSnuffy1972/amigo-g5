# Game 2398 (TotalFootball v1.2 0154): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02398). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4612726 top-level bytes in 42 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 20 of 20 units identical (bytes and relocations), 43 raw-word lines |
| Functions named and described | pass, 665 of 665 named (0 author symbols), 665 of 665 described (5 by rule, 639 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 11427 of 11427 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 1026650 bytes, 5 functions |
| Independent decoder cross-check (Ghidra against capstone) | 23507 of 23795 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 988526 |
| code | 92296 |
| data | 3531801 |
| padding | 103 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 156 | 5 |
| boot | 1 | 690 | 44 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 9946 | 445 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7475 | 115 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2803 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 386 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 1 |
| CHIPRAM.EXE (raw code file, load address unknown) | 1 | 2261 | 4 |
| FASTRAM.EXE (raw code file, load address unknown) | 1 | 59 | 1 |
| happy0.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| happy1.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| happy2.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| keeper0.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| keeper1.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| keeper2.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| player0.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| player1.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| player2.spr (raw code file, load address unknown) | 1 | 0 | 0 |
| stadium2.blk (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154.hdf: hard-disk image (AmigaDOS OFS, 4896 blocks): its filesystem parses and all 34 files in it are byte-identical to the extracted tree TotalFootball_v1.2_0154_hdf, which is recovered file by file; no block outside the filesystem holds data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 6846 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3065 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1147 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 219 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/CHIPRAM.EXE: 5 executed PCs whose RAM was overwritten before any snapshot match its bytes 0-80: recovered as a unit run at an address
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/CHIPRAM.EXE: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/FASTRAM.EXE: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/happy0.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/happy1.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/happy2.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/keeper0.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/keeper1.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/keeper2.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/player0.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/player1.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/player2.spr: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- TotalFootball_v1.2_0154/TotalFootball_v1.2_0154_hdf/TotalFootball/data/stadium2.blk: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 12 never-loaded region(s), 1026650 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `disable_audio_and_dma` (game.slave): The routine calls a library routine to query status, disables interrupts and DMA channels, clears a global variable four times, writes constants to two memory locations, recursively calls another routine, and then writes the result of the earlier library call to memory before invoking a final library routine. Kind: sabotage path. Pass path: The branch where iVar1 != 0 (the 'else' block) is the normal execution path; the 'if (iVar1 == 0)' block is the failure path that disables hardware and halts the CPU..
- `process_input_and_update` (runtime segment an address): Modifies a global variable using bitwise operations, polls a routine, and performs bitwise updates on two different memory locations based on hardware register states. Kind: disk hardware/track check. Pass path: the branch at an address is taken (bne.b an address) if the check fails, so the pass path is the fall-through/bsr.w an address.
- `wait_for_button_release` (runtime segment an address): Modifies a global variable using bitwise operations and loops until a specific bit in that variable is cleared. Kind: manual/code-wheel/colour-card lookup. Pass path: the branch following the bsr.w an address call (the code continues execution after the subroutine returns).

## Functions


### game.slave

- `return_negative_one`: The routine is the program entry point and immediately returns a constant value with all bits set.
- `disable_audio_and_dma`: The routine calls a library routine to query status, disables interrupts and DMA channels, clears a global variable four times, writes constants to two memory locations, recursively calls another routine, and then writes the result of the earlier library call to memory before invoking a final library routine.
- `reset_game_state_flags`: The routine clears a set of global variables and installs a self-reference in two function pointer slots, then writes constant values to several memory locations.
- `compare_global_and_return`: The routine tests whether a byte derived from a global variable by shifting it left by seven bits and combining it with a right shift by one bit equals a specific constant.
- `select_minimum_and_call`: The routine reads a global variable, calls a function through a table entry to obtain a value, keeps the smallest value seen so far, and then calls another function through the same table with that minimum.

### boot

- `initialize_boot_memory`: The routine sets a global variable to point to itself, triggers a trap, then reads values from a memory region referenced by that variable into RAM locations and checks a condition involving a RAM integer.
- `initialize_hardware_and_copy_table`: The routine sets up a global variable, reads values from it, checks a condition and loops forever if the condition is met, then copies a block of words from the routine's own code into a table at a fixed address and writes a value to a global variable before calling another routine.
- `initialize_memory_table`: The routine sets a global variable, writes constants to two memory locations and a byte location, calls another routine three times, then copies 128 longwords from one table to another table.
- `set_flag_and_wait_loop`: The routine sets a flag byte in memory and then enters a loop that repeatedly toggles bits in that byte while polling a status byte for a condition, calling another routine on each iteration.
- `set_byte_flags`: The routine computes an address from an uninitialized register plus a constant offset, then sets several bits in the byte at that address and clears two other bits, leaving the remaining bits unchanged.
- `set_flag_bits_in_byte`: The routine computes a pointer from an uninitialized register plus a constant offset and treats it as a byte.
- `subroutine_call_pair`: The routine receives a value in a register and a pointer in another register.
- `set_flag_byte_and_wait`: The routine sets bits in a byte at an offset from a pointer, clears or sets bits based on a flag and a comparison loop that repeatedly calls another routine until two values match, then writes a word from a register to a global variable.
- `wait_for_hardware_flag`: The routine writes bytes to offsets in a memory region referenced by an uninitialized pointer, then spins until a flag byte at one of those offsets has its bit set, clears a bit in another byte at an offset, and returns.
- `set_flags_and_wait_for_bit`: The routine writes constant byte values to four offsets of a structure pointed to by a register, then spins until a specific bit in one of those bytes is set, and finally clears a bit in another byte before returning.
- `initialize_sprite_state`: The routine writes several fields of a structure pointed to by a register, overwriting some fields multiple times, then waits in a loop until a flag bit in the structure is set, and finally writes two more fields before returning.
- `deobfuscate_data_block`: The routine repeatedly selects a 512-byte block from a table of blocks, XORs its words with a running checksum, and deobfuscates the block by reordering its nibbles using a mask and shift.
- `execute_indirect_loop`: Repeatedly calls a routine via a pointer retrieved from a table until the called routine returns zero.
- `set_memory_flags`: The routine sets bits in a byte located at an offset from a register value, then clears specific bits in the same byte.
- `set_flags_in_byte`: The routine computes a pointer from an input register plus a constant offset and then modifies the byte at that address by setting and clearing specific bits.
- `validate_and_copy_string_data`: Compares a string against a target buffer using case-insensitive character matching.
- `compare_and_copy_bytes`: The routine compares a zero-terminated string at an offset in a structure pointed to by a0 with a pattern pointed to by a table entry, normalising letters to upper case.
- `set_global_pointer_and_call`: The routine stores a constant value into a word located four bytes before the value pointed to by a global variable and then calls another routine.
- `divide_index_and_compare`: Divides a value in a data register by eleven, compares the quotient's low word against a word in a structure pointed to by an address register, and if they differ, calls another routine with a value combining the remainder and quotient before calling a second routine with no arguments.
- `update_flag_and_wait`: The routine sets a flag byte in a structure, updates a word at an offset in another structure, and then loops calling another routine until a counter matches.
- `process_input_loop`: Modifies bytes in memory through a pointer based on the return value of another routine, updates a structure, and writes several bytes to specific offsets in a table before waiting for a bit change.
- `set_player_characteristics_and_wait`: The routine stores a value from a register into memory via a pointer and compares a register with a stack value to possibly exit early.
- `wait_for_hardware_flag_2`: The routine writes values to several byte locations relative to an address in a register, then spins until a flag byte at an offset has its bit set, and finally clears a bit in another byte before returning.
- `clear_flags_and_wait`: The routine writes constant values to several byte locations referenced through a base pointer, then spins until a specific flag bit in one of those locations becomes set, and finally clears a bit in another location before returning.
- `clear_global_variable`: The routine calls another routine and then stores zero into a 16-bit field of a global variable via an address in A4.
- `initialize_player_state`: The routine writes several fixed values into a structure pointed to by a register, copies a word from another structure, and then waits until a flag byte in the structure has a specific bit set before writing a couple more values and returning.
- `decode_and_verify_block`: The routine decodes a 64-word block in place using a key stored in a table and then verifies a checksum.
- `checksum_verify_loop`: A routine that mixes words from a source buffer with a mask and builds a checksum, compares it to a stored value and if it mismatches loops, invoking another routine repeatedly, decrementing a counter stored in a global structure and exiting via a jump table entry when the counter reaches zero.
- `verify_buffer_integrity`: Calculates a checksum or length from a null-terminated string and compares it against a stored value.
- `set_memory_flags_2`: The routine takes an address from a register, adds a constant offset to obtain a pointer, and then modifies the byte at that location by setting several bits and clearing others.
- `set_byte_flags_2`: The routine computes a pointer from an uninitialized register plus a constant offset and writes to the byte at that address.
- `validate_and_copy_buffer`: Calculates a checksum or length from a null-terminated string and compares it against a stored value.
- `compare_and_rotate_buffer`: The routine compares a normalized string at a fixed offset in a structure pointed to by A0 against a pattern pointed to by A4.
- `set_global_pointer_and_call_2`: The routine stores a constant value into a global variable accessed via a pointer and then calls another routine.
- `recovered_routines`: The routine receives a value in register D0 and a pointer in A4.
- `set_flag_byte_with_loop`: The routine manipulates a byte at an offset from a pointer and updates a word at an offset from another pointer.
- `update_byte_flags`: Performs bitwise operations on a byte through a pointer within a loop and waits for a flag change.
- `other_routine`: The routine stores a word from a register into a location pointed to by a pointer plus an offset.
- `set_memory_flags_and_wait`: The routine writes bytes to four offsets from a base pointer, then spins until a flag byte at one offset has its bit set, and finally clears a bit in another flag byte.
- `set_flags_and_wait`: The routine writes bytes to several offsets from a base pointer, then spins until a flag byte at one offset has its bit set, and finally clears a bit in another flag byte.
- `clear_flag_in_table`: The routine calls another routine and then stores zero into a word at an offset from an address in A4.
- `initialize_player_state_2`: The routine writes several fixed values into a structure pointed to by a register, copies a word from another structure pointed to by a second register into the first, and then waits until a flag byte in the structure has a specific bit set before writing a final value and returning.
- `verify_and_transform_data`: Searches a table for a specific marker, performs bitwise transformations on data blocks, and verifies a checksum.
- `verify_checksum_and_retry`: The routine mixes bytes from two input buffers with a mask and accumulates a checksum, then compares the masked checksum against a stored value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_unpack_data`: Configures hardware interrupts, DMA, and color registers while performing data decompression and bitfield extraction.
- `tiny_ori`: Small routine of 1 instruction that merges 56 into d2.
- `hang_in_infinite_loop`: Loops forever with no operations and never returns.
- `update_interrupt_settings`: Increments a global variable three times, calls another routine, and writes values to a global variable and hardware interrupt registers.
- `perform_repeated_additions`: Performs a series of additions using a value from one register and a value from another, storing the results as word pairs into a table.
- `clear_memory_block`: Fills a block of memory starting at a specific address with zeros using a loop.
- `write_formatted_string`: Calculates two values based on a loop and stores them as bytes into a memory location, followed by a terminator byte.
- `execute_infinite_loop`: Enters an infinite loop that prevents further execution.
- `nop_padding_sequence`: This routine is an empty block of instructions that results in an infinite loop.
- `update_hardware_state`: Checks a global variable, calls another routine, modifies a value at a specific memory address, and writes to hardware interrupt registers.
- `perform_repeated_additions_2`: Performs a series of additions using values from two registers and stores the results as word pairs into a table.
- `perform_blt_area_fill`: Configures and executes a Blitter operation to fill or copy a rectangular area of memory.
- `populate_event_queue`: Iterates through a list of data structures to calculate specific values based on arithmetic operations and stores the results into a queue.
- `sort_integer_table_2`: Performs a nested loop comparison to sort a table of integers in ascending order.
- `call_indirect_pointer`: Checks a global variable and, if non-negative, executes a function via a pointer stored at an offset within that variable.
- `update_sprite_attribute_list_3`: Calculates new coordinate and property values based on a lookup table and writes them into a structured data block.
- `update_attribute_list`: Calculates new coordinate and property values based on a lookup table and writes them into a structured data block.
- `update_object_attribute_list`: Calculates new coordinate and property values based on a relative offset and updates a linked list or structure.
- `update_sprite_display_attributes`: Calculates sprite position and attribute data based on coordinate offsets and table lookups.
- `update_sprite_attributes`: Writes calculated coordinate and attribute data into a structure using values derived from a lookup table and bitwise operations.
- `update_sprite_attribute_list`: Calculates new coordinate and attribute values for a sprite or object based on a lookup table and writes them into a structured data block.
- `update_sprite_attribute_buffer`: Calculates sprite position and attribute data based on coordinate offsets and updates a structured buffer.
- `update_sprite_position`: Calculates and stores new position and attribute values into a structure based on a coordinate offset and a lookup table.
- `update_sprite_attribute_list_2`: Calculates new position and attribute values for a sprite or object based on a coordinate offset and a lookup table.
- `wait_and_call`: Polls a bit in a global variable until it is cleared, sets that variable to a specific value, and then calls a function using an offset.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `execute_blitter_sequence`: Writes parameters from a structure into a command table and waits for a hardware status flag to indicate completion of multiple operations.
- `execute_blitter_sequence_2`: Configures and triggers a series of Blitter operations by writing parameters to a table and waiting for a completion flag.
- `execute_blt_sequence`: Configures and triggers a series of Blitter operations by writing parameters to a table and waiting for a completion flag.
- `update_blt_graphics_parameters`: Updates Blitter registers and data pointers using values from a structured table.
- `execute_offset_function`: Executes a function located at an address determined by adding a value from a pointer to a global variable.
- `update_collision_event_queue`: Iterates through a linked list of objects to detect spatial overlaps based on dynamic boundary constraints.
- `nop_placeholder`: This routine is empty and performs no operations.
- `update_global_pointer`: Updates a global variable by using its lower bits to index into a table and then adds a constant value to the result.
- `unpack_data_stream`: Searches for a specific four-byte signature in a buffer, performs bitwise manipulation to unpack data into a table, and then copies bytes from one location to another.
- `unpack_and_decompress_data`: Searches for a specific magic constant in memory to locate a data block, performs a bitwise transformation to unpack values into a global variable, and then executes a decompression loop to expand data into a target memory area.
- `copy_and_wait_input`: Copies a block of data and then enters a loop that waits for specific bits in a hardware register to be set before returning.
- `execute_conditional_function`: Checks if a global variable is zero and, if so, calls the function pointed to by that variable before resetting it.
- `initialize_global_state`: Sets a global variable to a specific value and calls another routine four times.
- `assign_constant_and_recurse`: Sets a global variable to a specific constant value and then calls itself four times.
- `reset_and_recursively_call`: Sets a global variable to a specific value and then clears it to zero before calling itself multiple times.
- `check_null_global`: Checks if a global variable is zero and calls another routine if it is.
- `modify_global_value`: If a global variable is zero, it updates that variable with a new value derived from a previous state and a constant offset.
- `add_offset_to_table`: If a global variable is zero, it iterates through a table and adds a base address to each entry.
- `initialize_data_tables_2`: If a global variable is zero, it unpacks data and sets several global variables to a specific constant value.
- `add_to_table`: Adds a signed short value to every element in a table of 28,671 words.
- `setup_memory_offsets`: If a global variable is zero, it unpacks data and initializes several global variables with offsets based on a constant value.
- `add_value_to_array`: Adds a constant value from a register to each word in a memory block.
- `sequence_data_checks`: Performs a series of calls to another routine and checks if a specific global variable is zero before continuing a sequence of calls.
- `extended_sequence_checks`: Executes a long sequence of calls to another routine and performs conditional checks on a global variable to determine whether to continue the sequence.
- `update_callback_table`: Iterates through a table to update entries with a function pointer and an incremented integer value.
- `update_attribute_list_2_2`: Iterates through a table to update entries by storing a function pointer and incrementing a value from a global variable.
- `update_callback_table_2`: Iterates through a table to update entries with a function pointer and an incremented value from a global variable.
- `update_callback_table_3`: Iterates through a table to update each entry with a function pointer and an incremented integer value.
- `update_callback_table_4`: Iterates through a table to replace entries with a pointer to a specific routine and increment a corresponding integer value.
- `update_attribute_list_2_3`: Iterates through a table to update entries by storing a routine pointer and incrementing a value using a global variable.
- `update_callback_table_5`: Iterates through a table to replace entries with a pointer to a specific routine and increments an associated integer value.
- `update_callback_table_6`: Iterates through a table to update entries with a function pointer and an incremented integer value.
- `update_callback_table_7`: Iterates through a table to update entries with a function pointer and an incremented value from a global variable.
- `update_callback_table_8`: Iterates through a table to update entries with a function pointer and an incremented value.
- `update_attribute_list_2_4`: Iterates through a table to update entries by storing a routine pointer and incrementing a value using a global variable.
- `update_attribute_list_2`: Iterates through a table to update entries with a pointer to another routine and an incremented value.
- `unknown_routine`: A routine consisting of five instructions that is called by nine different routines.
- `dispatch_calls_and_update_state`: This 22-instruction routine is not reached by normal control flow and was found only because its gap decodes as code.
- `iterate_and_copy_data`: Resets a global variable to zero and enters a loop that calls another routine and increments the variable until it reaches a specific value.
- `fill_table_and_wait`: Fills a memory block with incrementing values, updates a global variable with a character, and waits for a global variable to become non-zero.
- `clear_memory_block_2`: The routine writes zero to a series of longwords in a table until a counter reaches zero.
- `increment_and_recurse_globally`: Resets a global variable to zero and enters a loop that calls another routine and increments the global variable until it reaches a specific value or the called routine returns zero.
- `increment_and_recurse`: Increments a global variable while recursively calling another routine until the variable reaches a specific value or the returned value is zero.
- `copy_memory_block_6`: Copies a block of data from one memory location to another using a loop.
- `update_byte_bits`: Sets four specific bits and clears one or two other bits in a byte through a pointer, depending on the value of a global variable.
- `set_byte_bits`: The routine sets four specific bits and then clears two other bits in a byte through a pointer.
- `set_byte_bits_alt`: The routine sets four specific bits and then clears a different bit in a byte through a pointer.
- `modify_byte_flags_2`: Sets the most significant bit and one of two other bits in a byte through a pointer, while clearing a different bit based on a global variable.
- `modify_byte_pattern`: The routine sets two bits and clears one bit in a byte through a pointer.
- `modify_byte_flags`: Sets the most significant bit and the fifth bit of a byte pointed to by a register, while simultaneously clearing the fifth bit before re-setting it.
- `process_and_copy_string_data`: Calculates a checksum or hash from a null-terminated string and performs a case-insensitive comparison against a stored sequence.
- `compare_and_copy_string_data`: Performs a case-insensitive string comparison between two sequences of characters.
- `store_constant_value`: Stores a constant value into a memory location pointed to by an offset from a pointer.
- `divide_and_conditional_branch`: Divides a value in a data register by eleven and compares the quotient to a word in a structure.
- `control_hardware_output`: Performs bitwise operations on a byte through a pointer and waits for a hardware status bit to be set before writing constant bytes to several offsets.
- `poll_and_write_data`: Performs bitwise operations on a byte through a pointer and enters a loop that polls a status bit before writing several constant bytes to various offsets and returning.
- `poll_hardware_status`: Writes four constant byte values to specific offsets from a base pointer and waits for a bit in one of those locations to be set before clearing it.
- `write_and_poll_status`: Writes four constant byte values to specific offsets from a base pointer and waits for a bit in one of those locations to be set before clearing a bit in another location.
- `set_flags_and_poll_status`: The routine writes constant byte values to four locations relative to a base pointer, polls a specific bit in one location until it is set, and then clears a different bit in another location before returning.
- `poll_status_and_clear`: Sets a bit in a byte, polls a specific bit in a memory location, and then clears a word at a specific offset.
- `clear_memory_word`: Calls another routine and then stores a zero word at an offset from a pointer.
- `update_object_status`: Writes multiple values to a structure pointed to by a register, including a value copied from another structure, then polls a bit in the structure until it is set before writing final values to the structure.
- `process_bitplane_data_3`: Searches for a specific marker in a buffer and performs bitwise transformations on data blocks using bitwise masks.
- `process_bitplane_data_2`: Iterates through a structure to perform bitwise operations and bit-shifting on data while checking for a specific marker and searching for a constant value in a table.
- `process_bitplane_data`: Iterates through a structure to perform bitwise operations and bit-shifting on data while checking for a specific marker.
- `process_input_and_update`: Modifies a global variable using bitwise operations, polls a routine, and performs bitwise updates on two different memory locations based on hardware register states.
- `wait_for_button_release`: Modifies a global variable using bitwise operations and loops until a specific bit in that variable is cleared.
- `store_word_to_global`: Stores a word from a register into a global variable.
- `fill_memory_and_dispatch_indexed`: Repeatedly writes a single byte from a register to eight consecutive memory locations pointed to by a register.
- `fill_memory_and_dispatch`: Fills a memory range starting at a pointer with a repeated byte value.
- `fill_and_call_table_entry`: Fills a memory block with a single byte at the first index and a different byte at the second index, then executes a routine from a table.
- `fill_and_call_table_entry_2`: Fills a memory block with a single byte at the first two indices and a different byte elsewhere, then executes a routine from a table.
- `fill_and_call_table_entry_3`: Fills a memory block with a single byte at the third index and a different byte elsewhere, then executes a routine from a table.
- `fill_and_call_table_entry_4`: Fills a memory block with a single byte at the first and third indices and a different byte elsewhere, then executes a routine from a table.
- `copy_partial_data`: Copies two bytes from a source table into a destination buffer and fills the remaining bytes with a constant value.
- `copy_three_bytes`: Copies three bytes from a source table into a destination buffer and fills the remaining bytes with a constant value.
- `copy_single_byte`: Copies one byte from a source table into a destination buffer and fills the remaining bytes with a constant value.
- `copy_two_bytes_offset`: Copies the first and fourth bytes from a source table into a destination buffer and fills the remaining bytes with a constant value.
- `copy_two_bytes_interleaved`: Copies the first and second bytes from a source table into a destination buffer and fills the remaining bytes with a constant value.
- `copy_three_bytes_interleaved`: Copies the first, second, and fourth bytes from a source table into a destination buffer and fills the remaining bytes with a constant value.
- `copy_two_bytes_offset_alt`: Copies the third and fourth bytes from a source table into a destination buffer and fills the remaining bytes with a constant value.
- `copy_three_bytes_sequential`: Copies three consecutive bytes from a source table into a destination buffer and fills the remaining bytes with a constant value.
- `write_record_then_dispatch_handler`: Copies the 8-byte record at the pointed-to input into the output buffer: the first byte is cleared, the next three bytes are taken from the source, and the last four bytes are cleared.
- `copy_four_bytes`: Copies four consecutive bytes from a source table into a destination buffer and fills the remaining bytes with a constant value.
- `fill_memory_and_dispatch_callback`: Fills a block of memory with a repeated byte value, except for one specific offset, and then executes a function from a table using an index.
- `copy_data_and_dispatch_call`: Copies a sequence of bytes from one memory location to another and then executes a subroutine via an indexed jump from a table.
- `write_buffer_and_dispatch`: Writes a sequence of bytes to a memory location using a pointer and then executes a subroutine from a table based on a calculated offset.
- `copy_data_and_dispatch_callback`: Copies a sequence of bytes from one memory location to another and then executes a function pointer retrieved from a table using an index.
- `update_memory_and_dispatch_callback`: Writes a sequence of bytes to a memory location using a pointer and then executes a subroutine from a table based on an index and an offset.
- `copy_data_and_dispatch_call_2`: Copies a sequence of bytes from one memory location to another and then executes a subroutine via a calculated offset from a table.
- `copy_data_and_dispatch_callback_2`: Copies a sequence of bytes from one memory location to another and then executes a function from a table using an index.
- `copy_data_and_dispatch_callback_3`: Copies a block of data from one memory location to another and then executes a routine selected from a table via an index.
- `update_buffer_and_dispatch_callback`: Copies a sequence of bytes from a source to a destination buffer and then executes a function from a table using an index derived from a pointer.
- `copy_data_and_dispatch_callback_4`: Copies a sequence of bytes from one memory location to another and then executes a function from a table using an index value.
- `write_pattern_one`: Copies a specific sequence of bytes from a source to a destination, alternating between a register value and source values.
- `write_pattern_two`: Copies bytes from a source to a destination, using a register value for specific indices in the sequence.
- `write_pattern_three`: Writes a sequence of bytes to a destination, starting with a register value followed by source bytes.
- `write_pattern_four`: Copies a sequence of bytes from a source to a destination, interspersed with a register value.
- `write_pattern_five`: Writes a sequence of bytes to a destination, combining a register value and several consecutive source bytes.
- `copy_block_and_dispatch_2`: Copies a sequence of bytes from a source to a destination and then executes a subroutine selected from a table using an index.
- `write_pattern_seven`: Fills a destination with a register value, except for one byte taken from a source.
- `write_pattern_eight`: Writes a sequence to a destination where the first byte is from a source and the sixth byte is also from a source.
- `write_pattern_nine`: Writes a sequence to a destination where the second byte is from a source and the sixth byte is also from a source.
- `write_pattern_ten`: Writes a sequence to a destination where the first and sixth bytes are taken from a source.
- `write_pattern_one_2`: Writes a specific pattern of bytes to a destination buffer, using one byte from a source table and others from a register.
- `write_pattern_two_2`: Writes a pattern of bytes to a destination buffer, alternating between bytes from a source table and a register.
- `write_pattern_three_2`: Writes a pattern of bytes to a destination buffer, using two consecutive bytes from a source table and others from a register.
- `write_pattern_four_2`: Writes a pattern of bytes to a destination buffer, using three consecutive bytes from a source table and others from a register.
- `write_pattern_five_2`: Writes a pattern of bytes to a destination buffer, placing a byte from a source table at the fourth position.
- `write_pattern_six`: Writes a pattern of bytes to a destination buffer, placing bytes from a source table at the first, fourth, and fifth positions.
- `write_pattern_seven_2`: Writes a pattern of bytes to a destination buffer, placing bytes from a source table at the second, fourth, and fifth positions.
- `write_pattern_eight_2`: Writes a pattern of bytes to a destination buffer, using four bytes from a source table at specific intervals.
- `write_pattern_nine_2`: Writes a pattern of bytes to a destination buffer, placing bytes from a source table at the third, fourth, and fifth positions.
- `write_pattern_ten_2`: Writes a pattern of bytes to a destination buffer, using four consecutive bytes from a source table.
- `update_byte_sequence_and_dispatch`: Copies a sequence of bytes from one memory location to another and then executes a function from a table using an index and an offset.
- `copy_data_and_dispatch_callback_5`: Copies a block of data from one memory location to another and then executes a function from a table using an index.
- `fill_memory_and_dispatch_callback_2`: Fills a memory range with a repeated byte value and then executes a function pointer retrieved from a table using an index.
- `copy_data_and_dispatch_callback_6`: Copies a sequence of bytes from one memory location to another and then executes a routine selected from a table using an index.
- `update_buffer_and_dispatch_callback_2`: Copies a sequence of bytes from one memory location to another and then executes a function from a table using an index derived from a memory value.
- `copy_record_fields_and_dispatch`: Copies four bytes from the source pointer into the destination buffer and zero-fills the remaining four bytes (the D2 low byte is used as filler).
- `update_buffer_and_dispatch_callback_3`: Copies a sequence of bytes from a source pointer to a destination buffer and then executes a function from a table based on an index.
- `copy_data_and_dispatch_callback_7`: Copies a sequence of bytes from one memory location to another and then executes a function pointer retrieved from a table using an index and an offset.
- `copy_data_and_dispatch_callback_8`: Copies a sequence of bytes from one memory location to another and then executes a routine retrieved from a table using an index and an offset.
- `copy_block_and_dispatch_3`: Copies a sequence of bytes from one memory location to another and then executes a routine selected from a table via an indexed indirect jump.
- `write_mixed_bytes`: Copies three bytes from a source to a destination, interspersed with a single byte from a register.
- `write_partial_bytes`: Copies four bytes from a source to a destination, interspersed with two bytes from a register.
- `write_interleaved_bytes`: Copies four bytes from a source to a destination, interspersed with four bytes from a register.
- `copy_interspersed_bytes_and_dispatch`: Copies a sequence of bytes from a source to a destination, inserting specific register values at fixed intervals.
- `copy_data_and_dispatch_callback_9`: Copies a sequence of bytes from a source to a destination and then executes a routine selected from a table using an index and an offset.
- `write_shifted_bytes`: Copies five bytes from a source to a destination, starting with one byte from a register.
- `write_bulk_bytes`: Copies five bytes from a source to a destination, interspersed with three bytes from a register.
- `write_full_source`: Copies six bytes from a source to a destination, interspersed with two bytes from a register.
- `write_single_source_byte`: Copies one byte from a source to a destination, surrounded by seven bytes from a register.
- `copy_buffer_and_dispatch_callback`: Copies a sequence of bytes from a source to a destination, where the sequence consists of two bytes from the source interleaved with bytes from a register.
- `write_byte_pattern_one`: Writes a specific sequence of bytes to a memory location, using a single byte from a source table and a constant byte for the remaining positions.
- `write_byte_pattern_two`: Writes a sequence of bytes to a memory location, copying two bytes from a source table and filling the rest with a constant byte.
- `write_byte_pattern_three`: Writes a sequence of bytes to a memory location, copying one byte from a source table into the third position and another into the seventh position.
- `write_byte_pattern_four`: Writes a sequence of bytes to a memory location, copying two bytes from a source table into the first and third positions.
- `write_byte_pattern_five`: Writes a sequence of bytes to a memory location, copying three bytes from a source table into the second, third, and seventh positions.
- `write_byte_pattern_six`: Writes a sequence of bytes to a memory location, copying three bytes from a source table into the first, second, and sixth positions.
- `write_byte_pattern_seven`: Writes a sequence of bytes to a memory location, copying one byte from a source table into the fourth and seventh positions.
- `write_byte_pattern_eight`: Writes a sequence of bytes to a memory location, copying two bytes from a source table into the first and fourth positions.
- `write_byte_pattern_nine`: Writes a sequence of bytes to a memory location, copying three bytes from a source table into the second, fourth, and seventh positions.
- `write_byte_pattern_ten`: Writes a sequence of bytes to a memory location, copying four bytes from a source table into the first, second, fourth, and seventh positions.
- `write_pattern_one_3`: Writes a specific sequence of bytes to a destination buffer, using a register value for most bytes and a source table for two specific positions.
- `write_pattern_two_3`: Writes a sequence of bytes to a destination buffer, alternating between values from a source table and a register value.
- `write_pattern_three_3`: Writes a sequence of bytes to a destination buffer, where most bytes are from a register and four bytes are taken from a source table.
- `write_pattern_four_3`: Writes a sequence of bytes to a destination buffer, copying four consecutive bytes from a source table and filling the rest with a register value.
- `write_pattern_five_3`: Writes a sequence of bytes to a destination buffer, where the first three and remaining bytes are from a register, except for two bytes from a source table.
- `write_pattern_six_2`: Writes a sequence of bytes to a destination buffer, using a register value for most positions and three bytes from a source table at specific offsets.
- `write_pattern_seven_3`: Writes a sequence of bytes to a destination buffer, alternating between a source table and a register value at specific intervals.
- `write_pattern_eight_3`: Writes a sequence of bytes to a destination buffer, using four bytes from a source table at various positions and filling the rest with a register value.
- `populate_interleaved_buffer`: Writes a sequence of bytes to a memory location by interleaving a single byte value with bytes from another source.
- `copy_alternating_bytes`: copies a byte from a source pointer to a destination pointer and then copies a byte from a register to the destination pointer, repeating this pattern while decrementing a counter.
- `copy_pattern_variant_one`: Copies a sequence of bytes from a source to a destination, alternating between source values and a register value.
- `copy_pattern_variant_two`: Copies a sequence of bytes from a source to a destination, with specific bytes replaced by a register value.
- `copy_pattern_variant_three`: Copies a sequence of bytes from a source to a destination, using a register value for the first three bytes.
- `copy_data_and_dispatch_callback_10`: Copies a sequence of bytes from one memory location to another and then executes a function from a table based on an index value.
- `copy_and_dispatch_data`: Copies a sequence of bytes from one memory location to another using a series of indexed stores.
- `copy_bytes_with_register_fill`: copies bytes from a source pointer to a destination pointer and writes a byte from a register to the destination pointer, repeating this pattern while decrementing a counter.
- `copy_pattern_variant_seven`: Copies a sequence of bytes from a source to a destination, starting with two register values.
- `copy_data_and_dispatch_call_3`: Copies a sequence of bytes from one memory location to another and then executes a subroutine via an indexed jump through a table.
- `copy_record_fields_to_buffer`: Copies a fixed set of eight bytes from the source pointer into the destination buffer, writing zero into two of the destination slots.
- `copy_block_and_dispatch_4`: Copies a sequence of bytes from one memory location to another and then executes a subroutine selected from a table using an index.
- `write_byte_pattern_one_2`: Writes a sequence of eight bytes to a destination, using two bytes from a source and filling the rest with a constant value.
- `write_byte_pattern_two_2`: Writes a sequence of eight bytes to a destination, using three bytes from a source and filling the rest with a constant value.
- `write_byte_pattern_three_2`: Writes a sequence of eight bytes to a destination, using three bytes from a source with a different distribution than the previous routine.
- `write_byte_pattern_four_2`: Writes a sequence of eight bytes to a destination, using four bytes from a source and filling the rest with a constant value.
- `write_byte_pattern_five_2`: Writes a sequence of eight bytes to a destination, using three bytes from a source with a different distribution than the previous routines.
- `write_byte_pattern_six_2`: Writes a sequence of eight bytes to a destination, using four bytes from a source with a different distribution than the previous routines.
- `copy_data_and_dispatch_callback_11`: Copies eight bytes of data from one memory location to another using a pointer and then executes a subroutine from a table based on an index.
- `write_byte_pattern_eight_2`: Writes a sequence of eight bytes to a destination, using five bytes from a source and filling the rest with a constant value.
- `update_memory_block_and_dispatch`: Copies a sequence of bytes from one memory location to another using a specific pattern.
- `copy_and_dispatch_data_2`: Copies a sequence of bytes from one memory location to another and then executes a routine selected from a table via an offset calculation.
- `interleave_and_dispatch_callback`: Writes a sequence of bytes to a destination by alternating between a single byte value and bytes from a source memory location, then executes a function from a table based on an indexed offset.
- `write_mixed_bytes_2`: Writes a sequence of bytes to a destination, mixing values from a source table and a register value.
- `write_patterned_bytes`: Writes a sequence of bytes to a destination, using a register value for several positions and source table values for others.
- `write_sequential_bytes`: Writes a sequence of bytes to a destination, using a mix of source table values and a register value.
- `write_shifted_bytes_2`: Writes a sequence of bytes to a destination, starting with a register value and then copying consecutive values from a source table.
- `write_contiguous_bytes`: Writes a sequence of bytes to a destination, copying several consecutive values from a source table and padding with a register value.
- `write_trailing_bytes`: Writes a sequence of bytes to a destination, using a register value for the first several positions and then copying values from a source table.
- `write_offset_bytes`: Writes a sequence of bytes to a destination, starting with a source table value and then using a register value for several positions before copying more from the table.
- `write_sparse_bytes`: Writes a sequence of bytes to a destination, alternating between a source table value and a register value at specific intervals.
- `write_grouped_bytes`: Writes a sequence of bytes to a destination, using a source table value followed by a register value and then a sequence from the table.
- `populate_buffer_and_dispatch_callback`: Writes a sequence of eight bytes to a memory location using a mix of a register value and source data, then conditionally executes a function from a table based on an index.
- `copy_bytes_with_padding`: Copies a sequence of bytes from a source to a destination, interspersed with a specific byte value.
- `copy_data_and_dispatch_callback_12`: Copies a sequence of bytes from one memory location to another and then executes a routine from a table using a calculated offset.
- `copy_and_append_bytes`: copies a sequence of bytes from a source pointer to a destination pointer and appends a specific byte from a register.
- `update_buffer_and_dispatch_callback_4`: Copies a sequence of bytes from one memory location to another, partially overwriting the destination with a single byte value.
- `copy_block_and_dispatch_5`: Copies a sequence of bytes from one memory location to another and then executes a subroutine via a table lookup based on a value in a register.
- `copy_eight_byte_record_then_dispatch`: Copies an 8-byte record from a source buffer into a destination buffer, writing the two unused bytes as a fixed filler value taken from a register.
- `copy_record_and_dispatch`: Copies a fixed-size block of eight bytes from one buffer to another, writing the sixth and eighth fields from the first byte of the source and a register value, then returns early if a counter register equals one; otherwise it indexes a jump table by the byte at the source-pointer and a multiple of twenty-four, and transfers control to the selected handler.
- `copy_data_and_dispatch_callback_13`: Copies a sequence of bytes from one memory location to another and then executes a function from a table using an index derived from a pointer's value.
- `copy_data_and_call_indexed`: Copies a block of data from one memory location to another using a series of byte-sized assignments.
- `copy_buffer_and_dispatch`: Copies a sequence of bytes from one memory location to another using a series of word and byte moves.
- `copy_block_and_dispatch_6`: Copies eight consecutive bytes from one memory location to another and then executes a subroutine via a table lookup based on a value at a pointer.
- `fill_memory_and_dispatch_callback_3`: Fills a block of memory with a repeated byte value and then executes a function pointer from a table based on an indexed lookup.
- `fill_memory_and_dispatch_2`: Copies a sequence of bytes from a source address to a destination address and then executes a routine selected from a table via an offset calculation.
- `update_buffer_and_dispatch_callback_5`: Writes a sequence of bytes to a memory location using a pointer and then executes a function from a table based on an index and an offset.
- `copy_block_and_call_indirect`: Copies eight consecutive bytes from one memory location to another and then executes a routine via an indirect jump calculated from a table offset.
- `update_memory_and_dispatch_callback_2`: Copies a sequence of bytes from a source pointer and a register into a destination memory buffer.
- `copy_data_and_dispatch_callback_14`: Copies eight bytes of data from a source pointer to a destination pointer and then executes a function from a table using an index.
- `copy_data_and_dispatch_via_table`: Copies eight bytes of data from two source locations into a destination buffer and then executes a routine selected from a table using an index value.
- `copy_block_and_dispatch_7`: Copies a block of eight bytes from one memory location to another and then executes a subroutine selected from a table using an index.
- `write_sparse_bytes_2`: Stores a single byte from a source at a specific offset into a destination buffer, followed by several other bytes from the same source at different offsets.
- `write_sparse_bytes_3`: Writes a byte from a source to the start of a destination buffer and places two other bytes from the source at later offsets.
- `write_sparse_bytes_4`: Copies three bytes from a source buffer to a destination buffer at specific, non-contiguous indices.
- `write_sparse_bytes_5`: Copies four bytes from a source buffer to a destination buffer at specific, non-contiguous indices.
- `copy_data_and_dispatch`: Copies a sequence of bytes from two source locations into a destination buffer.
- `copy_data_and_dispatch_callback_15`: Copies eight bytes of data from one memory location to another using a pointer and then executes a routine from a table based on an index.
- `copy_data_and_dispatch_callback_16`: Copies a sequence of bytes from one memory location to another using a pointer.
- `copy_record_and_dispatch_2`: Copies a fixed-size block of bytes from the source pointer to the destination pointer, with the last three destination bytes filled from a single byte register, then unless a flag register equals one, indexes a table by the first source byte times 24 and jumps through the resulting entry.
- `write_sparse_bytes_6`: Writes two bytes from a source buffer to a destination buffer at specific, non-contiguous indices.
- `write_sparse_bytes_7`: Writes three bytes from a source buffer to a destination buffer at specific, non-contiguous indices.
- `copy_bytes_with_register_fill_variant`: writes a byte from a register to a destination pointer and copies a byte from a source pointer to a destination pointer, repeating this pattern while decrementing a counter.
- `copy_and_dispatch_data_3`: Copies a sequence of bytes from one memory location to another and then executes a routine selected from a table using an index and an offset.
- `update_memory_and_dispatch_callback_3`: Copies a sequence of bytes from a source pointer to a destination pointer and then executes a function from a table using an index derived from a memory value.
- `copy_data_and_dispatch_callback_17`: Copies a sequence of bytes from one memory location to another and then executes a function from a table using an index and a base address.
- `write_pattern_five_4`: Writes a sequence of bytes from a source table and a register into a destination buffer before calling another routine through a pointer.
- `copy_data_and_dispatch_callback_18`: Copies a block of data from one memory location to another and then executes a routine selected from a table via an indirect jump.
- `fill_record_and_dispatch`: Copies eight bytes from a source record into a destination buffer, filling most fields with a zero-valued register, then returns early if a counter register equals one.
- `copy_and_dispatch_data_4`: Copies a sequence of bytes from one memory location to another and then executes a routine from a table using an offset calculated from a pointer and a multiplier.
- `copy_pattern_and_dispatch_callback`: Copies a sequence of eight bytes from one memory location to another and then executes a routine selected from a table using an index.
- `copy_data_and_dispatch_callback_19`: Copies eight consecutive bytes from one memory location to another.
- `copy_bytes_pattern_a`: Copies a specific pattern of bytes from a source to a destination, using a register for some bytes and a source table for others.
- `copy_bytes_pattern_b`: Copies a specific pattern of bytes from a source to a destination, where the source provides the first and last bytes of the sequence.
- `copy_bytes_pattern_c`: Copies a specific pattern of bytes from a source to a destination, using a register for the first and last bytes and a source table for the middle.
- `copy_memory_and_dispatch_indirect`: Copies a block of eight bytes from a source address to a destination address.
- `write_pattern_and_dispatch_via_table`: Writes a repeating byte pattern to a memory location, interspersed with two bytes from a source pointer, and then executes a subroutine from a table using an index derived from a memory value.
- `copy_bytes_pattern_f`: Copies a specific pattern of bytes from a source to a destination, where the source provides the first, sixth, and seventh bytes.
- `copy_bytes_pattern_g`: Copies a specific pattern of bytes from a source to a destination, where the source provides the second, sixth, and seventh bytes.
- `copy_bytes_pattern_h`: Copies a specific pattern of bytes from a source to a destination, where the source provides the first, second, sixth, and seventh bytes.
- `copy_bytes_pattern_i`: Copies a specific pattern of bytes from a source to a destination, where the source provides the third, sixth, and seventh bytes.
- `copy_bytes_pattern_j`: Copies a specific pattern of bytes from a source to a destination, where the source provides the first, third, sixth, and seventh bytes.
- `write_mixed_bytes_3`: Writes a sequence of bytes to a destination, alternating between a register value and values from a source table.
- `write_source_bytes`: Copies a sequence of bytes from a source table to a destination, interspersed with a register value.
- `populate_interleaved_buffer_2`: Writes a sequence of bytes to a memory location, alternating between a register value and values from a source buffer.
- `copy_interleaved_bytes`: Copies a sequence of bytes from a source to a destination, interleaving a byte from a register between each byte from the source.
- `interleave_bytes_and_dispatch`: Interleaves bytes from two source locations into a destination buffer and then executes a subroutine from a table using an index.
- `write_sparse_bytes_8`: Copies bytes from a source table to a destination, with a register value placed at specific intervals.
- `write_offset_bytes_2`: Writes a sequence of bytes to a destination, starting with a register value and then using source table values.
- `copy_data_and_dispatch_callback_20`: Copies a sequence of bytes from one memory location to another and executes a function from a table using a calculated offset.
- `copy_data_and_dispatch_callback_21`: Copies a sequence of bytes from one memory location to another using a pointer.
- `write_extended_bytes`: Copies a longer sequence of bytes from a source table to a destination, interspersed with a register value.
- `write_pattern_one_4`: Writes a specific pattern of bytes to a destination, using a single byte from a source table and a constant byte for most other positions, before potentially calling another routine via a table lookup.
- `write_pattern_two_4`: Writes a pattern of bytes to a destination, incorporating three bytes from a source table and a constant byte for the remaining positions, before potentially calling another routine via a table lookup.
- `write_pattern_three_4`: Writes a pattern of bytes to a destination, using three bytes from a source table and a constant byte for the remaining positions, before potentially calling another routine via a table lookup.
- `write_pattern_four_4`: Writes a pattern of bytes to a destination, using four bytes from a source table and a constant byte for the remaining positions, before potentially calling another routine via a table lookup.
- `copy_data_and_dispatch_callback_22`: Copies a sequence of bytes from one memory location to another using a pointer.
- `write_pattern_six_3`: Writes a pattern of bytes to a destination, using five bytes from a source table and a constant byte for the remaining positions, before potentially calling another routine via a table lookup.
- `expand_packed_bytes_to_buffer`: Copies eight bytes from a source table into a destination buffer, writing a constant byte to every third position and taking the remaining five bytes from the source in sequence.
- `write_pattern_eight_4`: Writes a pattern of bytes to a destination, using six bytes from a source table and a constant byte for the remaining positions, before potentially calling another routine via a table lookup.
- `update_memory_block_and_dispatch_2`: copies a sequence of bytes from one memory location to another and then executes a function selected from a table via an indirect jump.
- `copy_buffer_and_dispatch_2`: Copies a sequence of bytes from one memory location to another using a series of move instructions.
- `copy_interleaved_bytes_2`: Copies bytes from a source table to a destination, alternating between a register value and source values.
- `copy_shifted_bytes`: Copies bytes from a source table to a destination, inserting a register value at specific offsets.
- `copy_bytes_and_dispatch_via_table`: Copies a sequence of bytes from a source memory location to a destination, interspersed with a specific register value, and then executes a routine from a table using an index and an offset.
- `copy_offset_bytes`: Copies bytes from a source table to a destination, inserting a register value at the second position.
- `copy_patterned_bytes`: Copies bytes from a source table to a destination, inserting a register value at the first and seventh positions.
- `copy_sequential_bytes`: Copies a sequence of bytes from a source table to a destination, inserting a register value at the seventh position.
- `copy_suffix_bytes`: Copies bytes from a source table to a destination, filling the first six positions with a register value.
- `copy_trailing_bytes`: Copies bytes from a source table to a destination, filling the middle positions with a register value.
- `copy_sparse_bytes`: Copies bytes from a source table to a destination, inserting a register value at the first, third, fourth, and fifth positions.
- `copy_split_bytes`: Copies bytes from a source table to a destination, inserting a register value in the middle and placing source bytes at the end.
- `write_pattern_one_5`: Writes a sequence of bytes to a destination buffer using a mix of a register value and bytes from a source table.
- `write_pattern_two_5`: Copies bytes from a source table to a destination buffer, interspersed with a register value.
- `write_pattern_three_5`: Writes a specific byte pattern to a destination buffer using values from a source table and a register.
- `write_pattern_four_5`: Stores a sequence of bytes from a source table into a destination buffer, with some bytes replaced by a register value.
- `write_pattern_five_5`: Writes a byte sequence to a destination buffer where the first three bytes are a register value and the rest are from a source table.
- `write_pattern_six_4`: Copies a subset of bytes from a source table to a destination buffer, separated by a register value.
- `write_pattern_seven_4`: Writes a pattern to a destination buffer by alternating between a register value and bytes from a source table.
- `write_pattern_eight_5`: Stores bytes from a source table into a destination buffer, using a register value as a separator.
- `write_pattern_nine_3`: Writes a sequence of bytes to a destination buffer, starting with a register value and followed by bytes from a source table.
- `write_pattern_ten_3`: Copies a contiguous block of bytes from a source table to a destination buffer, interspersed with a register value.
- `copy_bytes_pattern_a_2`: Copies a specific sequence of bytes from a source to a destination, including two instances of a single register value.
- `copy_bytes_and_dispatch_via_table_2`: Copies a sequence of bytes from a source to a destination, including two repeated bytes from a register, and then executes a routine from a table using an index derived from a memory value.
- `copy_bytes_pattern_c_2`: Copies a sequence of bytes from a source to a destination, using a single register value for most positions except two.
- `copy_bytes_pattern_d`: Copies a sequence of bytes from a source to a destination, where three positions are filled with a single register value.
- `copy_bytes_pattern_e`: Copies a sequence of bytes from a source to a destination, using a single register value for four positions.
- `copy_bytes_pattern_f_2`: Copies a sequence of bytes from a source to a destination, where three positions are filled with a single register value at specific offsets.
- `copy_bytes_pattern_g_2`: Copies a sequence of bytes from a source to a destination, using a single register value for four positions including the first.
- `copy_bytes_pattern_h_2`: Copies a sequence of bytes from a source to a destination, where three positions are filled with a single register value at alternating offsets.
- `copy_bytes_and_dispatch_via_table_3`: Copies a sequence of seven bytes from a source pointer to a destination pointer and then executes a routine from a table using an index multiplied by a constant offset.
- `copy_bytes_pattern_j_2`: Copies a sequence of bytes from a source to a destination, where two positions are filled with a single register value.
- `write_mixed_bytes_4`: Writes a sequence of bytes to a destination, using a mix of a register value and values from a source table.
- `write_offset_bytes_3`: Writes a sequence of bytes to a destination, starting with a value from a source table and followed by a register value.
- `copy_data_and_dispatch_callback_23`: Copies a sequence of bytes from one memory location to another using a series of move instructions.
- `write_shifted_bytes_3`: Writes a sequence of bytes to a destination, starting with two values from a source table and followed by a register value.
- `write_delayed_source_bytes`: Writes a sequence of bytes to a destination, using a register value for the first two entries before reading from a source table.
- `write_leading_source_bytes`: Writes a sequence of bytes to a destination, starting with a value from a source table and followed by several values from that same table.
- `write_consecutive_source_bytes`: Writes a sequence of bytes to a destination, using a register value followed by a continuous block of values from a source table.
- `write_extended_source_bytes`: Writes a sequence of bytes to a destination, starting with several values from a source table and ending with a register value.
- `write_trailing_source_bytes`: Writes a sequence of bytes to a destination, filling the first five entries with a register value and the remainder from a source table.
- `write_late_source_bytes`: Writes a sequence of bytes to a destination, filling the first five entries with a register value and the final three from a source table.
- `copy_byte_pattern_a`: Copies a specific sequence of bytes from a source to a destination, using a register value for several positions.
- `copy_byte_pattern_b`: Writes a sequence of bytes to a destination, mixing values from a source and a register.
- `copy_byte_pattern_c`: Populates a destination buffer with a mix of register values and bytes from a source table.
- `copy_interleaved_bytes_3`: Copies a sequence of bytes from a source to a destination, inserting a byte from a register at specific intervals.
- `copy_byte_pattern_e`: Copies bytes from a source to a destination, where some bytes are replaced by a register value.
- `copy_byte_pattern_f`: Writes a sequence of bytes to a destination, using consecutive bytes from a source and a register.
- `copy_byte_pattern_g`: Fills a destination buffer with a register value at several indices and source bytes at others.
- `copy_byte_pattern_h`: Copies a pattern of bytes from a source to a destination, using a register for padding.
- `copy_byte_pattern_i`: Writes a sequence of bytes to a destination, alternating between source bytes and a register value.
- `copy_memory_block_and_dispatch`: Copies a sequence of eight bytes from a source address to a destination address.
- `copy_data_and_dispatch_callback_24`: Copies a block of eight bytes from one memory location to another via a pointer.
- `copy_block_and_dispatch_8`: Copies a sequence of bytes from one memory location to another via a pointer.
- `copy_data_and_dispatch_callback_25`: Copies a sequence of bytes from one memory location to another using a pointer.
- `copy_block_and_dispatch_9`: Copies a sequence of bytes from one memory location to another and then executes a routine selected from a table via an indirect offset calculation.
- `update_memory_block_and_dispatch_3`: Copies a sequence of bytes from one memory location to another, filling the first four bytes with a specific value before appending four bytes from the source.
- `copy_block_and_dispatch_10`: Copies a sequence of bytes from one memory location to another and then executes a subroutine via a calculated offset in a table.
- `copy_data_and_dispatch_via_table_2`: Copies a sequence of bytes from one memory location to another and then executes a routine selected from a table using an index and a fixed offset.
- `copy_header_and_dispatch`: Copies eight bytes from the source buffer into the destination buffer, writing the source's first two bytes, a duplicated fill byte twice, and then source bytes 2 through 5 and the next two bytes at offsets 4 to 7.
- `copy_data_and_dispatch_callback_26`: Copies a sequence of bytes from one memory location to another and then executes a function from a table using an index derived from a pointer.
- `copy_data_and_dispatch_callback_27`: Copies a sequence of bytes from one memory location to another and then executes a routine selected from a table using an index and a base address.
- `copy_bytes_with_offset`: Copies a sequence of bytes from one location to another, inserting a specific byte value at the first and fourth positions of the destination.
- `copy_bytes_with_offset_2`: Copies a sequence of bytes from one location to another, inserting a specific byte value at the fourth position of the destination.
- `copy_bytes_with_offset_3`: Copies a sequence of bytes from one location to another, inserting a specific byte value at the first, second, and third positions of the destination.
- `copy_bytes_with_offset_4`: Copies a sequence of bytes from one location to another, inserting a specific byte value at the second and third positions of the destination.
- `copy_bytes_with_offset_5`: Copies a sequence of bytes from one location to another, inserting a specific byte value at the first, third, and fourth positions of the destination.
- `copy_bytes_with_offset_6`: Copies a sequence of bytes from one location to another, inserting a specific byte value at the third position of the destination.
- `copy_bytes_with_offset_7`: Copies a sequence of bytes from one location to another, inserting a specific byte value at the first and second positions of the destination.
- `copy_bytes_with_offset_8`: Copies a sequence of bytes from one location to another, inserting a specific byte value at the second position of the destination.
- `copy_block_and_dispatch_11`: Copies a sequence of bytes from one memory location to another using a pointer.
- `copy_block_and_dispatch`: Copies eight consecutive bytes from one memory location to another.
- `return_no_operation_3`: The routine consists of a single instruction that immediately returns to its caller and performs no other work.
- `nop_padding`: This routine is empty and contains no executable instructions.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_hardware_and_tables`: Configures color registers, updates a table of pointers using additions, and populates a memory area with a repeating pattern of values.
- `validate_and_copy_data`: Checks if a value from a global variable matches a specific set of constants.
- `execute_looping_calls`: Resets a global variable to zero and calls another routine a fixed number of times in a loop.
- `execute_repeated_calls`: Sets a global variable to a specific value and executes another routine multiple times within a loop.
- `execute_conditional_loop`: Executes a loop that repeatedly calls other routines and performs conditional calls based on the value of a global variable.
- `modify_global_value_2`: Performs bitwise and arithmetic operations on a global variable and conditionally calls another routine.
- `update_indexed_data_2`: Checks bit flags in a global variable to decrement it or modify a byte at a location determined by an index.
- `calculate_and_call`: Resets a global variable to zero, performs arithmetic operations on it, adds a constant, and then calls another routine.
- `process_data_block`: Extracts values from a memory block and performs a loop to calculate a sum based on those values.
- `copy_and_clear_data`: Copies a block of data from a source to a global variable, clears a memory range with zeros, and then fills another memory range with a byte from a register.
- `increment_global_variable`: Increments a global variable if it is non-negative.
- `nop_padding_byte`: This routine is empty and consists of a single instruction that performs no operation.
- `fill_memory_pattern`: copies a block of data and then fills a larger memory area with a repeating pattern of bytes and words.
- `merge_memory_tables`: iterates through nested loops to perform bitwise OR operations between data from a source table and a destination buffer.
- `process_command_list`: waits for a DMA flag to clear and then parses a list of commands, executing specific subroutines based on command IDs.
- `setup_blitter_transfer`: calculates a destination address and configures blitter registers for a specific data transfer operation.
- `setup_blitter_transfer_alt`: calculates a destination address and configures blitter registers with different control values than the previous routine.
- `expand_data_table`: Iterates through a source table and writes repeating sequences of two words into a destination table at fixed intervals.
- `reconstruct_sprite_data_table`: Iterates through a source data structure to rebuild a table of interleaved color and attribute bytes.
- `copy_and_modify_table_entries`: Copies data from a source table to a destination, modifies specific bytes within the copied words, and stores the resulting pointers into a global list.
- `copy_table_block`: Copies a fixed number of longwords from a source table to a destination buffer based on an offset derived from a global variable.
- `expand_indexed_data`: Reads indices from a table and performs a non-contiguous copy of multi-word data structures into a destination buffer.
- `transfer_structured_data`: Fills a memory range with a single byte and then copies a structured block of data from a source table to a destination table.
- `parse_resource_chunk`: Searches for a specific four-byte signature in memory, performs bitwise manipulation on subsequent data, and then processes a variable-length string or byte block.
- `clear_sprite_attribute_table`: Calculates a memory address based on a global variable and performs a bulk clear of specific bitfields and bytes within a structured table.
- `calculate_address_offset`: calculates a new address in a register by performing arithmetic on a value from a global variable and a constant, then adds an offset derived from another global variable.
- `copy_data_table`: Performs a structured copy of data from a source table to a destination using three distinct loop patterns.
- `setup_blt_parameters`: Polls a status bit, calculates offsets from a table, and prepares values for a hardware bitblock transfer.
- `fill_memory_patterns`: Fills several memory blocks with alternating patterns of zero and specific bit patterns based on input values.
- `clear_memory_structures`: Clears specific offsets within multiple memory blocks based on a calculated offset and a loop counter.
- `clear_memory_blocks`: Clears multiple memory blocks by writing zeros to specific offsets within a loop.
- `extract_bitfield`: Extracts and shifts a specific bitfield from a global variable.
- `fetch_audio_parameters`: Uses a bitmask to index into a table and retrieve multiple values, likely for audio hardware configuration.
- `update_hardware_registers_2`: Performs bitwise operations on a global variable and manipulates various hardware registers including audio and DMA controllers.
- `update_pointer_table`: Iterates through a list of values to add a base address to each entry, effectively updating a table of pointers.
- `clear_global_variable_2`: Sets a global variable to zero.
- `traverse_linked_data`: Decrements a counter and follows a series of offsets stored within a data structure to traverse a linked list or tree.
- `update_input_state_bitfield`: Reads joystick, mouse, and button hardware registers to update a global bitfield.
- `update_counter_state`: Increments or resets bytes in a three-byte structure based on a bitmask in a register.
- `update_counter_state_variant`: Modifies a three-byte structure by incrementing specific bytes and returns a value with bits set based on the value of the first byte.
- `update_counter_state_alt`: Updates a three-byte structure using a bitmask and returns a value with different bit flags set compared to other similar routines.
- `process_bitfield_and_counters`: Updates a bitfield in a table and manages several counters and status flags based on the input register and existing values.
- `initialize_data_tables`: Populates two large memory areas with incrementing values and sets various global constants.
- `update_color_and_tables`: Updates hardware color registers and populates two separate memory tables with incrementing values using a loop.
- `execute_sequential_subroutines`: Calls three distinct subroutines in succession.
- `increment_global_pointer`: Adds a value and a variable offset to a global variable.
- `sync_audio_and_color_parameters`: Copies a block of six words from one memory location to another and updates several global variables by loading values from a table using an offset derived from a global variable.
- `fetch_audio_parameters_2`: This routine is a single instruction that serves as an entry point for retrieving audio configuration data.
- `return_no_op`: The routine consists of a single return instruction and performs no work, so it is an empty stub.
- `copy_memory_block_4`: Checks a global variable and, if it matches a specific value, copies a block of memory to itself using a loop and modifies a hardware color register.
- `assign_function_pointer_2`: Stores the address of the current routine into a global variable and then executes several other routines.
- `update_entity_attributes`: Copies a block of memory from a fixed source into a data structure and executes a series of repeated calls to update various entity properties.
- `convert_number_to_string`: Converts a global integer into a decimal string representation stored in memory and performs bitwise shifts.
- `format_numeric_string`: Converts a global numeric value into a two-digit ASCII string stored in a specific memory buffer.
- `decrement_loop_constant`: Performs two identical loops that decrement a register from a fixed value down to zero.
- `reset_sprite_object_tables`: Clears and reinitializes two sets of object data structures within a loop.
- `nested_decrement_loop`: Executes a nested loop structure that decrements two registers multiple times.
- `initialize_object_tables`: Clears several blocks of memory and populates two distinct tables with structured data entries.
- `execute_nested_loops`: Performs two nested loops that decrement counters until they reach negative one.
- `fetch_word_from_table`: Returns a word from a table using an index provided in a register multiplied by two.
- `fetch_word_with_mask`: Returns a word from a table using an index that is masked and offset by a constant.
- `get_mirrored_byte`: Determines a quadrant based on the signs of two inputs, scales them down, and returns a value from a table that is potentially inverted or offset based on the quadrant.
- `calculate_scaled_value`: Calculates a value based on the relationship between two inputs and applies various bit-shift subtractions depending on the magnitude of the result.
- `return_no_operation`: This routine consists of a single instruction that returns immediately, performing no work.
- `return_no_op_2`: The routine consists of a single return instruction and performs no work.
- `store_constant_values`: Stores two specific 32-bit constant values into a global variable.
- `update_global_math`: Performs complex arithmetic involving multiplication and addition on a global variable and stores the result back into it.
- `decrement_or_reset_counter`: Decrements a global variable if it is non-zero, otherwise sets it to one.
- `update_indexed_data`: Fetches a value from a nested table structure using multiple offsets and updates a field in a data block.
- `update_nested_data_block`: Fetches a pointer from a nested table structure and updates a data block with a value from another table based on a calculated index and a global flag.
- `update_data_via_index`: Fetches a longword from a table using an index and stores it in a data block, then optionally updates a short value in the same block.
- `store_word_to_block`: Stores a word from a register into a specific offset within a data block.
- `update_if_equal`: Compares two values from a table and, if they are equal, updates a data block with a value from another table and modifies an index.
- `update_if_not_equal`: Compares two values from a table and, if they are not equal, updates a data block with a value from another table and modifies an index.
- `update_word_and_index`: Writes a word from a table into a data block and then updates the data block with a value from another table using an index from the same table.
- `update_long_and_index`: Writes a longword from a table into a data block and then updates the data block with a value from another table using an index from the same table.
- `set_bit_and_update`: Sets a specific bit in a word within a data block and then updates the data block with a value from another table using an index.
- `clear_bit_and_update`: Clears a specific bit in a word within a data block and then updates the data block with a value from another table using an index.
- `toggle_bits_and_update`: Performs an XOR operation on a word in a data block using a value from a table and then updates the data block with a value from another table.
- `update_indexed_data_short`: Copies a word from one location to another and updates a data block using an index from a nested table.
- `update_indexed_data_long`: Copies a longword from one location to another and updates a data block using an index from a nested table.
- `return_immediately_stub_2`: The routine consists of a single return instruction and performs no work, so it only transfers control back to its caller.
- `return_no_operation_stub`: The routine consists of a single instruction that immediately returns to its caller, performing no computation, no memory or hardware access, and no library calls.
- `initialize_system_state`: Configures hardware color registers, populates a memory area with a repeating pattern, and updates a table of pointers using arithmetic.
- `set_global_value_2`: Sets a global variable to a specific value and returns.
- `decrement_global_variable_repeatedly`: Performs a series of subtractions from a global variable using specific byte values.
- `initialize_and_populate_data_structures`: Clears a memory block, executes a series of mathematical loops that call another routine with incrementing values, and performs a non-contiguous copy of data from a global array into a destination buffer.
- `execute_loop_with_math`: Executes a loop that performs arithmetic on a global variable and calls other routines multiple times based on values read from a table.
- `update_entity_attribute_structures`: Updates a data structure by copying a block of memory from a fixed source, then executes a series of repeated calls to another routine using incrementing offsets.
- `process_entity_attribute_data`: Iterates through a character string to look up indices in a table and copies large blocks of attribute data based on those indices.
- `format_and_display_numeric_table`: Iterates through a table of seven entries, converting each entry's associated numeric values into two-digit decimal character strings.
- `load_entity_attributes_from_table`: Iterates through a string of characters to look up indices in a table, then uses those indices to copy a large block of attribute data from a source table into a destination structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `expand_encoded_data_table`: Reads a sequence of encoded bytes and uses them as indices into a lookup table to retrieve and expand data.
- `initialize_data_table_set_a`: Stores a series of specific values into a large table starting at a global variable.
- `initialize_data_table_set_b`: Stores a different series of specific values into a large table starting at a global variable.
- `initialize_data_table`: Copies a series of values from memory into a large table starting at a global variable.
- `process_audio_data`: Iterates through a data buffer to decrement byte values and updates audio volume registers.
- `clear_memory_block_3`: Fills a block of memory starting at a global variable with zeros.
- `copy_memory_block`: Copies a block of memory from a global variable to another location using a loop.
- `process_entity_state_transitions`: Evaluates a global state variable to determine which conditional logic branch to execute.
- `update_recursive_coordinate_offset`: Recursively traverses a linked structure to update coordinate or position offsets based on a scaling factor.
- `validate_and_reset_coordinates`: Checks if specific coordinate values within a data structure fall within predefined bounds.
- `update_entity_collision_physics`: Checks if an entity's coordinates fall within specific boundary ranges and, if a collision is detected, modifies its velocity, position, and direction.
- `validate_and_clamp_entity_bounds`: Checks if an entity's coordinates are within defined screen or field boundaries and performs clamping or resets state if they exceed limits.
- `reset_global_variable`: Sets a global variable to zero multiple times before setting it to one.
- `return_no_op_4`: The routine consists of a single instruction, a return, and performs no work: it reads no registers or memory, makes no library calls, and touches no hardware registers.
- `validate_state_transitions`: Evaluates a global variable against multiple specific values and numeric ranges, updating it with different sets of constants based on the result.
- `check_and_update_variable`: Checks if a global variable is zero, calls another routine, and then writes three specific values to that global variable.
- `validate_variable_range`: Performs multiple range and equality checks on a global variable and writes four specific values to it if certain conditions are met.
- `verify_variable_state`: Checks if a global variable matches specific ranges or values and writes three specific values to it if conditions are met.
- `test_variable_bounds`: Evaluates a global variable against several numeric boundaries and writes different sets of values to it depending on which range it falls into.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `copy_memory_block_2`: Copies sixty-seven bytes from a global variable to a fixed memory location.
- `initialize_variable_table`: Writes two values to a global variable and a subsequent memory location, incrementing the value by a constant each time.
- `copy_and_mask_data`: Copies two hundred and three bytes to a memory location and then performs a loop to write a masked value from a function return into a table.
- `rotate_global_pointer`: Rotates a pointer through a loop five times by storing a value from a register and incrementing the global pointer.
- `copy_memory_block_5`: Copies a fixed number of bytes from one memory location to another using a loop.
- `extract_table_data`: Extracts data from two different locations in a table into global variables and performs calculations to determine an index and a value.
- `copy_and_mix_data`: Copies three consecutive blocks of seven words from a source table to different destinations and writes alternating byte sequences using a register and a source table.
- `execute_empty_recursion`: The routine calls itself repeatedly before returning.
- `copy_data_blocks`: Copies three consecutive blocks of seven words each from a source table to three different destination memory locations.
- `transfer_data_blocks`: Performs three consecutive loops to move seven words of data between various memory locations and tables.
- `copy_attribute_data`: Copies a large block of byte-sized data from a source table to a destination buffer using two calculated offsets.
- `transfer_data_buffer`: Copies a large block of data from a global variable to a specific memory address and then calls another routine based on the value of that global variable.
- `rearrange_word_data`: Copies words from a source buffer to a destination buffer while applying a specific offset to the destination pointer in each iteration.
- `transform_word_data`: Iterates through a source buffer and stores the result of subtracting each word from two different constant values into a destination buffer.
- `process_linked_structures`: Iterates through two linked lists to execute function pointers stored in structures and clamps specific word values within a second list to defined boundaries.
- `update_linked_structures`: Iterates through a list of structures, checking a bit flag and updating internal word values based on whether a pointer matches a specific value or by calling another routine.
- `find_nearest_entity_by_proximity`: Iterates through a list of entities to find the one with the lowest priority value and minimum distance to a target, using a wrap-around distance calculation.
- `update_entity_state_and_interpolate_2`: Checks entity status flags and state consistency before calculating interpolated coordinates.
- `reset_entity_state_and_callback`: Checks if an entity's identifier matches a stored value or if a specific flag is set, then resets its status, clears its coordinate data, and updates its function pointer.
- `update_entity_state_and_interpolate`: Checks entity status flags and state consistency before calculating interpolated coordinates.
- `update_entity_interpolation_state`: Checks if an entity's state requires a reset and updates its internal status.
- `interpolate_and_reset_entity_state`: Calculates interpolated coordinate values using a global progress variable and various offset pairs.
- `search_and_reset_entity_list`: Iterates through a linked list of structures to find an entry that meets specific bitmask and value criteria.
- `update_entity_state_3`: Modifies various fields within a structure, including counters, flags, and function pointers, based on conditional checks of structure members.
- `initialize_structure_and_global`: Sets several fields in a structure pointed to by a register and assigns a constant value to a global variable based on a comparison.
- `set_global_value`: Sets a global variable to a specific constant value.
- `update_entity_state_2`: Modifies various fields within structures, including counters, flags, and function pointers, based on conditional checks of structure members.
- `update_entity_state`: Modifies structure fields and function pointers based on a conditional check of a structure member.
- `initialize_structure_fields_2`: Updates several fields in two different structures, including increments, clears, and a function pointer assignment.
- `decrement_and_check_structure`: Decrements a byte in a structure and conditionally writes values to another structure based on a comparison.
- `set_structure_fields`: Writes a zero and a constant value to specific offsets within a structure if a returned value meets a bitmask condition.
- `initialize_structure_fields`: Writes a zero and a constant value to specific offsets within a structure pointed to by a register.
- `clear_and_update_structure`: Clears multiple fields in a structure, performs a bitwise clear on a word, and conditionally stores a value from another routine.
- `clear_structure_data`: Clears several fields in a structure to zero and performs a bitwise clear on a word at a specific offset.
- `store_returned_word`: Calls another routine and stores its two-byte return value into a structure offset.
- `process_recursive_structure_state`: Decrements a counter within a structure and recursively calls itself until the counter reaches zero.
- `store_returned_word_2`: Retrieves a word from a function call and stores it into a memory location offset from a pointer held in a register.
- `initialize_and_call_routine`: Sets two fields in a structure, calls another routine, and then populates several more fields in the structure with the result and constants.
- `validate_structure_index_2`: Checks if a structure field and a global variable fall within specific ranges before comparing a value against a table.
- `validate_structure_index`: Compares a value from a structure against a table to set a global variable and several other memory locations.
- `populate_structure_from_source`: Stores a shifted version of a word from one structure into another and populates several fields with constants and a pointer.
- `populate_structure_with_mask`: Calculates a value using a bitwise mask on a structure field, shifts it, and stores it into a new structure along with several constants.
- `calculate_and_set_structure`: Performs a calculation based on a structure field, stores the result in a different structure, and sets a global variable to a constant.
- `process_recursive_data_structure`: Recursively traverses a linked or nested data structure by decrementing a counter field.
- `store_returned_word_3`: Calls another routine and stores the resulting word into a memory location offset from a pointer held in a register.
- `initialize_data_structure`: Clears specific fields within a data structure and populates them with a constant value and a zeroed word.
- `validate_complex_data`: Compares a structure field against a table and updates several memory locations with constants and a pointer if conditions are met.
- `validate_and_set_complex_data`: Compares a structure field against a table and populates multiple memory locations with constants and a pointer.
- `set_complex_structure_data`: Populates a large number of fields in a structure with constants and a shifted value from another structure.
- `set_complex_structure_masked`: Calculates a masked value from a source structure and populates multiple fields in a destination structure with constants.
- `calculate_and_set_complex_structure`: Calculates a value from one structure to store in another, then populates multiple fields in that second structure with constants and sets a global variable.
- `update_entity_state_and_timer`: Decrements a counter field within a structure and, if it reaches zero, initializes specific data fields and resets a timer.
- `store_word_result`: Stores a two-byte value returned from another routine into a memory location offset from a pointer.
- `initialize_data_structure_2`: Clears several fields and sets specific constant values within a memory block pointed to by a register.
- `validate_structure_state_2`: Performs range checks on a global variable and a structure field, then compares a value against a table to update memory.
- `validate_structure_state`: Checks a value against a table and updates a global variable with a specific bit-shifted value from a structure field if certain conditions are met.
- `update_structure_with_shift`: Stores a shifted word from a source structure into a destination structure and sets several constant fields.
- `update_structure_with_mask_shift`: Performs a bitwise mask and shift on a source structure field before storing it into a destination structure with constants.
- `update_complex_structure`: Performs bitwise manipulations on a value from a table and populates multiple fields in a memory block while updating a global variable.
- `process_recursive_entity_state`: A recursive routine that decrements a counter within a structure and performs conditional updates to various fields based on the result of the recursion.
- `reset_entity_state_and_interpolate`: Checks if an entity's state has changed, and if so, clears its current properties and reinitializes them using data from a global table.
- `reset_entity_state_2`: Clears specific fields within a structure and populates it with coordinate and function pointer data retrieved from a global table.
- `initialize_entity_from_table`: Retrieves coordinate and function pointer data from a global table using offsets stored in a source structure.
- `validate_and_initialize_entity`: Performs bounds checking on two calculated coordinate values before populating a data structure with a function pointer, specific constants, and the validated coordinates.
- `initialize_entity_structure`: Populates a data structure with a constant, a value from another structure, and a function pointer based on a comparison.
- `initialize_entity_data_2`: Populates a structure with a constant, a value from another structure, and a function pointer based on a comparison.
- `initialize_entity_data`: Stores a constant value, a value from a structure, and a function pointer into a memory block.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `decrement_and_initialize_structure`: Decrements a short value at a specific offset in a structure if it is non-zero, otherwise initializes the structure fields based on a returned value.
- `initialize_structure_by_type`: Sets various fields in a structure, including a long value, a word, and a function pointer, based on the lower two bits of a returned value.
- `initialize_data_structure_3`: Clears several fields in a structure pointed to by a register and stores two specific constant values.
- `initialize_structure_fields_4`: Sets several fields within a structure pointed to by a register, including a bitwise XOR operation on a word and a function pointer assignment.
- `update_structure_data`: Writes specific values to a structure, performs a bitwise XOR on a word, and stores a function address into a pointer field.
- `initialize_structure_fields_3`: Clears two fields, sets a long value, sets a word to all bits high, and stores a function pointer into a structure.
- `decrement_or_reset_structure`: Decrements a short integer field within a structure if it is non-zero.
- `modify_structure_fields`: Clears two fields, sets a long value, performs a bitwise XOR on a word, sets a word to all bits high, and stores a function pointer into a structure.
- `update_interpolation_timer_and_state`: Decrements a counter within a structure and performs conditional resets of its fields based on specific constant values.
- `update_structure_interpolation_state_2`: Decrements a counter within a structure and performs conditional resets of structure fields based on specific constant values.
- `interpolate_structure_coordinates`: Performs linear interpolation on coordinate fields within a structure based on a global weight variable.
- `update_structure_interpolation_state`: Updates structure fields by performing linear interpolation of coordinate values based on a global variable acting as a progress or weight factor.
- `update_structure_position`: Calculates new coordinate values using a global variable and structure fields, then either resets the structure or updates its fields.
- `copy_and_clear_fields`: Clears specific offsets in a structure, copies a value from one offset to another, and copies two values from the start of the structure to later offsets.
- `update_structure_offset`: Performs conditional updates to a structure's fields based on a global variable and calculated offsets.
- `interpolate_structure_fields_2`: Calculates two interpolated short integer values based on a global variable and a set of source/destination offsets within a structure.
- `interpolate_structure_fields`: Calculates new values for structure fields using linear interpolation based on a global variable and then updates the structure.
- `update_structure_field`: Updates specific fields in a structure using arithmetic operations involving a global variable and existing structure data.
- `calculate_interpolated_position`: Computes interpolated coordinates using a global variable and structure values, applying clamping logic before updating the structure.
- `clamp_structure_coordinates`: Calculates new coordinates and clamps them within a specific range before updating the structure fields.
- `process_structure_conditional`: Performs conditional logic based on a global variable and a returned value to either reset structure fields or update them with new data.
- `update_structure_with_return`: Stores a constant value in a structure, calls another routine, and then stores the returned value and two other values into the structure.
- `update_structure_fields`: Compares a returned value against a field in a structure and, if the condition is met, resets several fields and copies values from other offsets within that structure.
- `store_returned_value`: Calls another routine and stores the resulting value into a specific offset within a structure.
- `reset_entity_state_and_attributes`: Resets an entity's status flags, coordinates, and function pointers based on its current state.
- `reset_entity_state`: Checks a global variable and, if zero, resets a structure field and a function pointer; otherwise, it performs conditional field updates based on a returned value.
- `validate_and_reset_entity_state`: Checks if an entity's identifier matches a stored value and verifies a status flag before resetting its state and function pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_stub_3`: The routine consists of a single return instruction and performs no work: it loads no registers, calls no library functions, touches no hardware registers and references no text.
- `update_entity_state_and_dispatch`: Checks a status flag in a structure to trigger a state transition.
- `set_structure_callback`: If a global variable is zero, it clears an offset in a structure and stores the address of this routine into another offset.
- `assign_function_pointer`: If a global variable is zero, it clears an offset in a structure and stores the address of this routine into a different offset.
- `update_entity_collision_state`: Checks a list of entities against a primary object to detect proximity or specific attribute matches.
- `reset_entity_state_and_tables`: Resets specific fields in an entity structure and iterates through two separate tables to clear status values if certain bit flags and function pointers match.
- `update_status_flags`: Iterates through two tables to check specific bit flags and a function pointer, then writes a value to a table offset if conditions are met.
- `process_conditional_updates`: Evaluates a global variable against multiple thresholds to perform different structure resets, pointer clearing, or value assignments.
- `clear_object_list_3`: Iterates through a table of structures to nullify pointers and reset specific fields within those structures.
- `clear_object_list_2`: Iterates through a table to nullify pointers and reset fields in a series of linked structures.
- `clear_object_list`: Sets a specific value in a structure and iterates through a table to clear pointers and reset fields in several linked objects.
- `clear_object_pointers`: Resets specific fields in a structure and iterates through a list to clear pointers and reset values in a table.
- `validate_distance_threshold`: Compares a global value against two elements in a list, and if within a specific range, updates a structure and clears several pointers in a table.
- `clear_linked_table_entries_2`: Checks if a global variable falls within a specific range and then iterates through a linked list of objects in a table.
- `clear_linked_table_entries`: Checks if a global variable falls within a specific range and then iterates through a linked list of objects in a table.
- `reset_table_entries`: Updates a structure field with a constant and clears a list of linked objects in a table if a condition is met.
- `reset_structure_table`: Updates a structure field with one of two constant values and then clears a list of linked objects in a table.
- `reset_structure_and_table`: Assigns a value to a structure based on an existing value and then iterates through a table to clear pointers and reset fields in multiple entries.
- `reset_structure_and_list`: Sets specific values in a structure and iterates through a table to clear several pointers and reset values.
- `reset_object_callback_states`: Iterates through two separate tables of data structures to check specific bitmask conditions and function pointer matches, resetting a short value to zero if the conditions are met.
- `update_object_status_flags`: Iterates through two separate tables of data structures to check specific bitmask conditions and function pointer matches.
- `update_structure_byte`: Checks a bit in a structure and a global variable to either set a global variable to one or decrement a byte in a structure after initializing it with specific values.
- `set_global_one`: Sets a global variable to one.
- `decrement_structure_byte`: Decrements a byte in a structure if it is non-zero, or sets it to a constant if it is zero, provided a global bit is not set.
- `decrement_or_set_byte`: Decrements a byte in a structure if it is non-zero, otherwise sets a global byte to one.
- `set_global_byte`: Sets a global byte to one.
- `get_table_value`: Returns a word from a table using an index provided in a register.
- `find_minimum_difference`: Iterates through a table to find the smallest absolute difference between a value and a table entry, using a secondary value as a tie-breaker.
- `find_maximum_difference`: Iterates through a table to find the largest absolute difference between a value and a table entry, using a secondary value as a tie-breaker.
- `find_minimum_value`: Iterates through a list of structures to find the smallest value returned by another routine, based on specific bit flags in a table.
- `calculate_distance_difference`: Performs conditional checks on a pointer's contents and calculates the absolute difference between a value from another routine and a value stored in a table.
- `find_best_match`: Searches a list of structures for the entry that minimizes a calculated value derived from a bitwise XOR operation and a table entry.
- `find_minimum_value_2`: Iterates through a list of structures using a fixed stride to find the smallest value returned by a subroutine.
- `update_position_and_copy`: Updates coordinate values in a structure using multiplication and addition, modifies a byte based on a threshold, and copies a block of memory to a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_memory_block_3`: Copies thirty-three bytes from a global variable to a fixed memory location.
- `set_table_pointers`: If a global variable is non-zero, it stores two calculated offsets derived from that variable into two different memory locations.
- `compress_and_blit_data`: The routine performs a bitwise OR reduction on a block of data to compress multiple words into a single word, then copies the resulting data to a new location.
- `clear_global_variable_3`: Sets a global variable to zero after assigning it a specific value twice.
- `nop_padding_instruction`: The routine is empty and contains no executable instructions.
- `return_no_operation_2`: The routine consists of a single instruction that immediately returns to its caller, performing no computation, no memory or hardware access, and no library calls.
- `update_global_coordinate_offset`: Calculates a new value for a global variable by computing a weighted average of two input values and applying clamped incremental adjustments.
- `clear_global_word`: Sets a global word variable to zero.
- `initialize_entity_data_tables`: This routine populates two distinct sets of data structures by reading values from a source object and writing them into a series of memory locations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `return_immediately_2`: This routine is empty and performs no operations before returning.
- `null_operation`: This routine is empty and performs no actions before returning.
- `return_immediately_stub`: The routine consists of a single instruction that returns straight to its caller, performing no work, no register or hardware access, and no library calls.
- `perform_no_operation`: The routine is empty and performs no actions before returning.
- `return_no_op_3`: The routine is empty: its decompiled body is only a return, and it consists of two instructions, which is consistent with a bare return from subroutine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_hardware_and_initialize_memory`: Configures the interrupt and DMA controllers before performing bulk memory copies and clearing specific data blocks.

### CHIPRAM.EXE (raw code file, load address unknown)

- `initialize_hardware_and_copy_data`: Configures hardware registers including CIA, DMA, and interrupt settings, then performs several memory block copies and clears specific memory regions.
- `update_hardware_registers`: Performs multiple word-sized writes to a memory block, which includes updating bitplane or color data and interacting with hardware registers like the DMA controller and CIAA.
- `sort_integer_table`: Iterates through a table of values to perform a sorting operation.
- `execute_indirect_call`: Calculates an offset from a global variable using a value in a register and performs an indirect call to the resulting address.

### FASTRAM.EXE (raw code file, load address unknown)

- `initialize_hardware_registers`: Configures various hardware components including CIA timers, color registers, copper lists, DMA controllers, and interrupt enable bits.
