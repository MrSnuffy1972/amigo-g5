# Game 999 (universal warrior (1993)(zeppelin)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00999). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (914352 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 59 of 59 named (0 author symbols), 59 of 59 described (3 by rule, 40 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1556 of 1556 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1844 of 1844 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4396 |
| code | 7734 |
| data | 753872 |
| padding | 894 |
| unused | 147456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Universal Warrior (1993)(Zeppelin)[cr QTX].adf:bootblock | 1 | 31 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 511 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 967 | 35 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 335 | 4 |

## Cases handled

- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: custom-format (non-DOS) disk, 1470 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 481 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 881 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 194 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 720-975 (bytes 368640-499712) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.16)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 984-999 (bytes 503808-512000) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.71)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1008-1023 (bytes 516096-524288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.38)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1032-1047 (bytes 528384-536576) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.63)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1056-1071 (bytes 540672-548864) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.69)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1080-1095 (bytes 552960-561152) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.42)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1104-1119 (bytes 565248-573440) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.51)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1128-1143 (bytes 577536-585728) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.21)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1152-1167 (bytes 589824-598016) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.65)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1176-1192 (bytes 602112-610816) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.41)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1200-1215 (bytes 614400-622592) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.78)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1224-1239 (bytes 626688-634880) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.17)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1248-1263 (bytes 638976-647168) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.66)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1272-1287 (bytes 651264-659456) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.54)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1296-1312 (bytes 663552-672256) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.38)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1320-1336 (bytes 675840-684544) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.26)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1344-1360 (bytes 688128-696832) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.31)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1368-1383 (bytes 700416-708608) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.87)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1392-1407 (bytes 712704-720896) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.28)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1416-1431 (bytes 724992-733184) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.84)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1440-1455 (bytes 737280-745472) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.57)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1464-1479 (bytes 749568-757760) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.41)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1488-1504 (bytes 761856-770560) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.56)
- universal warrior (1993)(zeppelin)[cr qtx]/Universal Warrior (1993)(Zeppelin)[cr QTX].adf: blocks 1512-1528 (bytes 774144-782848) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.67)
- 5 packed track region(s), 430080 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_hardware_registers` (runtime segment an address): Sets a global variable and modifies a bit in a hardware register, specifically affecting interrupt and DMA control bits. Kind: hardware register modification/timing check. Pass path: the instruction immediately following the call (e.g., bsr.w an address or the subsequent clr.w/bsr.w sequence).

## Functions


### Universal Warrior (1993)(Zeppelin)[cr QTX].adf:bootblock

- `clear_colour_load_and_jump`: Clears the COLOR00 register, calls one unnamed routine, reads a block with an exec I/O request, sends a motor-off request, then jumps to the loaded code.
- `read_disk_blocks_and_jump`: Reads a run of disk blocks into an address passed in a register with an exec I/O request, sends a motor-off request, and jumps to the loaded code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_store_constants_forever`: Loops forever, storing three fixed constant values per step into a block and calling unnamed routines in between.
- `initialize_system_state`: Clears a block of memory, copies three tables from different locations into new destinations, and sets several global values and hardware registers.
- `seed_random_from_cia`: Waits for a counter to reach zero, repeatedly invoking another routine and resetting a global variable while doing so.
- `call_six_routines_and_reset`: Calls six unnamed routines, then writes a count of sixteen and clears a word in the object record.
- `toggle_flag_select_value_set`: Inverts a flag word, then stores one of two sets of three fixed values into a table depending on whether the flag is now zero.
- `clear_words_at_pointer_list`: Writes zero into a word at each pointer in a list, for a count taken from a global.
- `plot_points_into_bitmap`: For each point in a list, checks it lies within fixed screen bounds, computes its row and column in a bitmap, sets the corresponding bit and stores the address written.
- `update_point_positions`: Steps a list of points by a fixed offset, resets points that leave a range by calling an unnamed routine, and when a flag is set rotates the coordinates through a lookup table.
- `project_points_by_depth`: For each point, scales its x and y coordinates by a factor and divides by its depth value, storing the results in a list and skipping points with zero depth.
- `read_point_fields_from_stream`: Reads three values through an unnamed routine, stores two centred and doubled as coordinates and the third offset by one hundred, resets the third to one if it exceeds 319, and writes two bytes to an output.
- `next_pseudorandom_byte`: Updates a 32-bit state with a rotate, add and xor sequence and returns its low byte as a pseudo-random value.
- `step_nibbles_toward_target`: Waits on a port line, then sixteen times steps each 4-bit channel of a colour table one unit toward a source table, in pairs of words.
- `fade_nibbles_toward_zero`: Waits on a status bit, then sixteen times decrements each 4-bit channel of a colour table by one unit, returning the last status value.
- `advance_effect_script_step`: Counts down a timer and, when it expires, reads the next command byte from a script, then sets step, timing and direction parameters according to the command value.
- `call_three_routines_in_order`: Calls three unnamed routines in sequence and returns.
- `call_setup_three_times`: Invokes another routine three times in succession and then returns.
- `call_setup_three_times_again`: Invokes another routine three times in succession and then returns.
- `draw_text_string_glyphs`: Walks a string of characters, looks up each glyph's source offset from a table, and calls an unnamed routine to blit it at a computed screen position, stopping at a carriage return or the end marker.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_table_values`: Iterates through a nested loop to increment nibble-based values within a table pointed to by a register.
- `decrement_table_values`: Iterates through a nested loop to decrement nibble-based values within a table pointed to by a register.
- `decrement_and_update_table`: Decrements nibble-based values in a specific table and updates several offsets in a structure based on a loop counter.
- `manage_nibble_fade_sequences`: Iterates through nested loops to decrement nibble-based values in a table and performs sequences of fading and stepping values toward targets.
- `execute_script_step`: Sets a global variable to a specific value, calls another routine, and then writes a value to a structure offset.
- `write_structure_constants`: Writes several 16-bit and 32-bit constants, a function pointer, and a zero value into a structure at specific offsets.
- `update_structure_parameters`: Writes multiple values to a structure, including calculated offsets and incrementing values, based on several input parameters.
- `configure_complex_parameters`: Updates a structure with various values and uses a loop to write a sequence of 32-bit and 16-bit values into a table based on an incrementing counter.
- `set_structure_parameters`: Writes several values to a structure, including calculated offsets and incrementing values, based on several input parameters.
- `calculate_input_state`: Reads a 16-bit value from a structure and performs bitwise logic to return a processed status byte.
- `process_encoded_data`: Calculates an encoded value using arithmetic and bits from a global variable, then stores a 16-bit result into a structure by splitting it into two bytes.
- `calculate_encoded_value`: Performs arithmetic operations on a value and combines it with bits from a global variable to return a concatenated result.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `store_word_split`: Splits a 16-bit value into two bytes and stores them at specific offsets within a structure.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_bitmask_via_pointer`: Retrieves a value from another routine and stores it at a memory location pointed to by a register, applying a bitwise OR with a specific bit set.
- `process_data_stream`: Iterates through a data structure, performing calculations and updating multiple fields in a structure based on values retrieved from a table and a pointer.
- `wait_while_flag_bit_set`: Checks a specific bit in a byte at a fixed offset from a pointer in an address register.
- `execute_infinite_loop`: The routine is empty and consists only of an infinite loop.
- `initialize_memory_buffers_and_tables`: Clears several memory regions with zeros and fills others with specific bit patterns.
- `decrement_global_counter`: Decrements a global variable and performs a conditional reset to a specific value if a second global variable is non-zero and the counter reaches four.
- `increment_global_counter`: Increments a global variable and performs a conditional reset to a specific value if a second global variable is non-zero and the counter reaches four.
- `initialize_data_tables`: Fills several consecutive memory areas with repeating sequences of four 16-bit values and then writes a set of 16-bit values to a structure pointed to by a global variable.
- `decrement_table_values_2`: Iterates through a table of 16-bit values to subtract a constant from each, clamping the result to a minimum value if it becomes negative.
- `update_table_and_counter`: Iterates through a table of 16-bit values to add a constant to each, clamping the result to zero if it exceeds a specific threshold, and increments a global byte variable with a wrap-around check.
- `update_structure_offsets`: Calculates a base value from a global variable and a pointer, then stores several derived 16-bit values into a structure at specific offsets.
- `transform_string_data`: Iterates through a sequence of bytes, modifies each byte based on specific character matches, and writes the results to a new memory location.
- `update_loop_counters`: Decrements a global variable and a loop counter repeatedly, resetting the loop counter to a specific value if it becomes negative.
- `store_calculated_pointers`: Calculates a memory address based on a value from a pointer and stores the resulting address and its high word into several global variables.
- `tiny_ori_2`: Small routine of 1 instruction that merges 23 into d4.
- `perform_recursive_nullification`: Checks a global variable and, if zero, clears a memory location before calling itself recursively.
- `tiny_ori_sub`: Small routine of 2 instructions that merges 3 into d0, then subtracts d0 from d0.
- `check_global_flag`: Checks if a global variable is zero and, if so, clears a memory location before calling another routine twice.
- `copy_memory_block`: Copies a block of data from a source location to a destination location using a loop that processes 20 bytes per iteration.
- `modify_hardware_registers`: Sets a global variable and modifies a bit in a hardware register, specifically affecting interrupt and DMA control bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_parameters`: Updates several audio hardware registers by copying values from a table and manages a global counter through a loop.
- `update_audio_channel_parameters`: Updates audio hardware registers by fetching period, volume, and frequency data from a lookup table based on a mode identifier.
- `compare_period_to_target`: Masks a 12-bit period word and stores it in a second field, clearing a flag byte.
- `update_timer_control_structure`: Updates a control structure based on a mode field, performing arithmetic operations such as decrementing a value, adding a value, or clamping it within a specific range.
