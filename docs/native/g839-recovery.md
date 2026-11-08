# Game 839 (thunderstrike (1990)(millennium)[cr angels][t +2 adept]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00839). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (918896 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 48 of 48 named (0 author symbols), 48 of 48 described (2 by rule, 46 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1246 of 1246 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 27136 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1391 of 1391 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 18231 |
| code | 5698 |
| data | 887589 |
| padding | 210 |
| unused | 7168 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Thunderstrike (1990)(Millennium)[cr Angels][t +2 Adept].adf:bootblock | 1 | 179 | 10 |
| logged disk load from Thunderstrike (1990)(Millennium)[cr Angels][t +2 Adept].adf boot-block load | 1 | 163 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 165 | 13 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 40 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 844 | 20 |
| Thunderstrike (1990)(Millennium)[cr Angels][t +2 Adept].adf bytes 43008-70144 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- thunderstrike (1990)(millennium)[cr angels][t +2 adept]/Thunderstrike (1990)(Millennium)[cr Angels][t +2 Adept].adf: custom-format (non-DOS) disk, 1738 non-empty blocks: matched against track-loaded segments
- thunderstrike (1990)(millennium)[cr angels][t +2 adept]/Thunderstrike (1990)(Millennium)[cr Angels][t +2 Adept].adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- runtime segment an address: 125 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 40 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 772 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- thunderstrike (1990)(millennium)[cr angels][t +2 adept]/Thunderstrike (1990)(Millennium)[cr Angels][t +2 Adept].adf: bytes 43008-70144 look like code no run loaded: recovered position-independently (load address unknown)
- thunderstrike (1990)(millennium)[cr angels][t +2 adept]/Thunderstrike (1990)(Millennium)[cr Angels][t +2 Adept].adf: 2 logged loads cover 38400 bytes of the image
- thunderstrike (1990)(millennium)[cr angels][t +2 adept]/Thunderstrike (1990)(Millennium)[cr Angels][t +2 Adept].adf: blocks 2-75 (bytes 1024-38912) outside the filesystem hold data, not code (opcode density 0.060, entropy 6.13)
- thunderstrike (1990)(millennium)[cr angels][t +2 adept]/Thunderstrike (1990)(Millennium)[cr Angels][t +2 Adept].adf: blocks 143-1737 (bytes 73216-889856) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.65)
- 1 never-loaded region(s), 27136 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_disk_dma_wait` (bootblock): Clears bit 2 of the global byte at a fixed address using the joystick fire state from offset 6 of the structure in a6, then configures the disk controller by writing the sync word, pointer and control registers and polls the interrupt request register with a countdown timeout before returning or writing the disk length register on failure. Kind: disk sync/track check. Pass path: the branch at an address (after the bsr.b an address call).

## Functions


### Thunderstrike (1990)(Millennium)[cr Angels][t +2 Adept].adf:bootblock

- `branch_to_offset_2`: Performs a short branch to an offset.
- `initialize_disk_io`: Clears a memory range, modifies CIA registers, calls several routines to handle disk status and IO requests, and manages a loop that increments a counter until it reaches 30.
- `increment_counter_and_poll`: Calls another routine, increments a byte at an offset of a structure in a6, and loops until that byte reaches 11; if a word at offset 6 of the structure is even, it increments that word and polls a byte at an offset of a structure until bit 0 is set.
- `setup_disk_dma_wait`: Clears bit 2 of the global byte at a fixed address using the joystick fire state from offset 6 of the structure in a6, then configures the disk controller by writing the sync word, pointer and control registers and polls the interrupt request register with a countdown timeout before returning or writing the disk length register on failure.
- `scan_sync_word_read_block`: Scans memory for a sync word, calls another routine to verify the match, and then loops 128 times to store results from that routine into a buffer.
- `combine_masked_pointer_bits`: Returns the data register ANDed with the longword at the second pointer, ORed with the data register ANDed with the first pointer's longword shifted left by one bit.
- `modify_cia_register`: Reads a byte from a CIA register, clears its top two bits, sets bit 3, and writes the result back to the same register, then calls another routine and loads a word into d0.
- `write_word_bytes_then_poll`: Writes the low and high bytes of the data register word to two consecutive memory locations, then polls a status byte until its low bit is set.
- `mask_byte_and_store_value`: Keeps the top two bits of one memory byte, sets its bit 3, and writes the value 127 to a second memory byte.
- `perform_io_operation`: Sets up an IO request structure with a command, buffer address, and length, calls a routine via a pointer in a global variable, and then calls another routine.

### logged disk load from Thunderstrike (1990)(Millennium)[cr Angels][t +2 Adept].adf boot-block load

- `expand_bitstream_data`: Copies a block of memory to a new location and then processes a bitstream to decode values, which are used as offsets to write bytes from a table into a destination buffer.
- `shift_and_store_word`: Loads a longword from a pointer in a0, moves its byte component to d7, stores that byte at an offset of a5, sets the carry flag, and performs a logical shift right on d0.
- `process_bit_shifts`: Performs a loop that shifts d0 right and calls another routine if the result is zero, then executes a sequence of conditional branches and loops that manipulate memory and call other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_scaled_values`: Stores two 16-bit values into global variables, then calculates two new values by shifting the inputs left by 3 and dividing them by two different global constants before storing the results.
- `update_scaled_values`: Stores two 16-bit values into global variables, then calculates two new values by shifting two other global variables left by 3 and dividing them by the first two stored values before storing the results.
- `store_word_pair`: Stores two 16-bit values from d1 and d2 into two consecutive global memory locations.
- `initialize_input_parameters`: Stores two 16-bit constants into global variables and copies the contents of the mouse port into a third global variable.
- `store_scaled_values`: Loads two words from memory, divides them by four, increments them by one, and calls another routine before returning.
- `prepare_branch_target`: Adds a constant to d0, moves a constant into d1, and branches to another location.
- `execute_four_calls`: Performs four calls to another routine, varying the values in d2 and d3 by adding or subtracting 6 before each call.
- `write_hardware_registers`: Masks the word in d0 with 7967 and writes the result to two consecutive memory locations starting at 51488.
- `tiny_ori_ori_clr`: Small routine of 3 instructions that merges 2 into (a3)+, then merges 7 into d0, then clears d4.
- `branch_to_offset`: Loads a word into d2 and branches to another routine.
- `update_pointer_and_process_table`: Calls another routine, updates a pointer in a structure with a new address, and then calls another routine to process a table of bytes, updating a global variable and a structure based on the table contents.
- `process_byte_table`: Reads a byte from a table in a0, sets a bit in a global variable, and if the byte is 13, calls two other routines and branches to a new address; otherwise, it loops through a counter to perform similar operations and increments a global word.
- `process_and_distribute_bytes`: Reads four bytes from a table using an offset derived from d0, performs a bitwise OR on them, and then writes the result or zero to five different memory locations based on a bitmask in d4.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_audio_channel_state`: Clears five per-channel state bytes at offsets 0, 28, 54, 80, and ac from a pointer in a0, then writes 255 to one register and 15 to another.
- `reset_and_load_channel_table`: Clears the channel state table of sixteen records, marks two global bytes with a full-set sentinel value, then loads four channel start entries from a table selected by the argument, setting each channel's pointer, length and sentinel fields.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_state`: Clears a memory location, sets bit 1 of the CIAA PRA register, calls three other routines, and then copies a block of data from a routine to a memory address using a loop that runs 240 times.
- `update_memory_patterns`: Checks if a byte at a specific memory location is 90, and if so, writes a repeating pattern of 20081 and a fixed address to several memory locations before calling a routine at address 1024.
- `update_loop_parameters`: Checks a bit in a global variable to write specific longword values to three global locations, then enters a loop that calls three other routines and updates two global counters with increments of 2 and 3 respectively, repeating until a global variable equals negative one.
- `tiny_ori`: Small routine of 1 instruction that merges 255 into d0.
- `store_obfuscated_values`: Loops a number of times determined by a word in memory, calling another routine three times per iteration to store a word, a word, and a masked word into a table.
- `update_object_coordinates`: Iterates through a table of data to calculate and update object positions within a structure.
- `update_obfuscated_value`: Multiplies a word from a structure at offset 6 of a5 by a value at a global address, XORs the result with 4681, and stores the longword result back into that global address.
- `initialize_memory_structure`: Calls another routine, sets a longword at offset 84 of a structure in a5 based on a bit test of a global variable, sets three words at offsets 64, 88, and 102 of that structure, and then performs a loop that clears 144 bytes of memory starting at a specific address.
- `calculate_table_index`: If d0 is negative, it adds 360 to d0, uses the result to fetch a word from a table, and performs arithmetic to update d2 based on a comparison with 270.
- `update_data_table_and_reset_state`: Increments a counter in a global variable and, upon reaching a threshold, executes a series of loops to populate a data table with specific word and long values, eventually resetting a state variable to a negative value.
- `update_state_and_copy_data`: Calculates a value from a structure, clamps it between zero and two hundred seventy-eight, and uses it to set a state variable; it then checks a hardware register and calls another routine to copy bytes from a table to a destination buffer based on the current state.
- `calculate_table_offset`: Calls another routine within a loop that iterates as long as the byte pointed to by a2 is not zero, calculating a memory offset using division and multiplication before calling another routine.
- `update_structure_values`: Calls another routine three times, updating various offsets of a structure in a5 with specific word and longword values and incrementing pointer values in a3 and a4 between calls.
- `parse_command_string_and_update_tables`: Decrements a global counter and, if zero, iterates through a string to set various global word values based on character matches.
- `iterate_pointer_table`: Increments a global value and iterates through a table of pointers, calling another routine if a byte at the current pointer is not zero or is negative one.
- `reset_structure_parameters`: Calls another routine three times, setting offset 66 of a structure in a5 to zero and updating offsets 64, 80, 84, and 88 with specific values and incrementing pointers in a3 and a4.
- `toggle_memory_value`: Writes 33792 to an offset of 96 from a5, then loops until bit 6 of a byte at offset 2 of a5 is cleared, after which it writes 29696 to the same offset of 96.
- `wait_for_byte`: Enters a loop that repeatedly checks a byte at offset 6 of a5 until that byte equals 48.
- `configure_system_tables`: Calls another routine, checks a library return value, and if non-zero, performs a loop to write a sequence of longwords and words to a table, then updates several global word values using swap operations and additions.
- `execute_library_calls`: Copies a longword from a global address to an offset of 80 in a5, sets two words at offset 96 and a word at offset 9a in a5, and then calls two routines via pointers stored in a global address.
