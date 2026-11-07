# Game 1000 (universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01000). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (914352 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 60 of 60 named (0 author symbols), 60 of 60 described (3 by rule, 41 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1555 of 1555 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1880 of 1943 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4396 |
| code | 7972 |
| data | 777564 |
| padding | 4 |
| unused | 124416 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf:bootblock | 1 | 130 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 511 | 18 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 967 | 35 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 335 | 4 |

## Cases handled

- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: custom-format (non-DOS) disk, 1515 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 481 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 880 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 194 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 720-975 (bytes 368640-499712) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.16)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 984-999 (bytes 503808-512000) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.71)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1008-1023 (bytes 516096-524288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.38)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1032-1047 (bytes 528384-536576) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.63)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1056-1071 (bytes 540672-548864) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.69)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1080-1095 (bytes 552960-561152) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.42)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1104-1119 (bytes 565248-573440) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.51)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1128-1143 (bytes 577536-585728) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.21)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1152-1167 (bytes 589824-598016) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.65)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1176-1192 (bytes 602112-610816) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.41)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1200-1215 (bytes 614400-622592) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.78)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1224-1239 (bytes 626688-634880) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.17)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1248-1263 (bytes 638976-647168) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.66)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1272-1287 (bytes 651264-659456) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.54)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1296-1312 (bytes 663552-672256) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.38)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1320-1336 (bytes 675840-684544) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.26)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1344-1360 (bytes 688128-696832) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.31)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1368-1383 (bytes 700416-708608) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.87)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1392-1407 (bytes 712704-720896) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.28)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1416-1431 (bytes 724992-733184) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.84)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1440-1455 (bytes 737280-745472) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.57)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1464-1479 (bytes 749568-757760) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.41)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1488-1504 (bytes 761856-770560) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.56)
- universal warrior (1993)(zeppelin)[cr qtx][t +16 znt]/Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf: blocks 1512-1528 (bytes 774144-782848) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.67)
- 5 packed track region(s), 453120 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_hardware_registers` (runtime segment an address): Writes values to specific hardware registers to modify system control settings. Kind: disk hardware/timing check. Pass path: the code following the call to sub_01EA (the instruction after the BSR/CALL).

## Functions


### Universal Warrior (1993)(Zeppelin)[cr QTX][t +16 ZNT].adf:bootblock

- `bootblock_entry_setup_dma`: Boot block entry called by the Kickstart strap.
- `boot_issue_io_request`: Issues an exec I/O request through its library call, then calls one other routine.
- `bootblock_block_undecoded`: Known only from its instruction count, its single caller and its use as an address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `loop_write_word_triplets`: Loops endlessly, writing three 16-bit words into fixed table slots and then making a run of calls to unnamed routines.
- `initialize_system_state`: Clears a block of memory, copies three tables from different locations into new destinations, and sets several global values and hardware registers.
- `check_button_status`: Checks the status of a button or disk status by reading a hardware register.
- `write_fixed_values`: Writes certain constant values into a table at a fixed offset relative to a register or pointer.
- `toggle_and_select_word_triplet`: Inverts a toggle word in a table and writes one of two sets of three 16-bit values into fixed slots, chosen by whether the toggled word is now zero.
- `zero_words_at_pointers`: Stores a zero word at each address in a pointer list, walking the list for a count held in a global plus one.
- `plot_visible_points_into_bitmap`: For each point in a table, it checks that the point lies inside a 320 by 200 area, computes the byte and bit position in a 40-byte-wide bitmap, and sets that pixel bit.
- `update_point_positions_and_angles`: Walks a table of point records, moving each by a step value and resetting it when it leaves the valid range.
- `scale_point_coordinates_by_ratio`: For each record with a nonzero divisor, scales its two coordinates by a ratio taken from a global and stores them in the output table.
- `decode_three_coordinate_bytes`: Reads three bytes from the input and converts them into two offset coordinates and a third value.
- `next_pseudo_random_byte`: Updates a 32-bit state word by rotating it left one bit, adding, XOR-ing with an input word and subtracting that word again.
- `fade_palette_towards_target`: Waits for the vertical blank flag, then over sixteen intervals steps the red, green and blue nibbles of each colour entry one unit toward a target table and writes the results back.
- `fade_palette_to_black`: Waits for the vertical blank flag sixteen times, and each time reduces the red, green and blue nibbles of two palette entries by one, stopping at zero.
- `advance_sequence_script_step`: Counts down a timer and, when it expires, reads the next command from a script table, wrapping to the start at the end.
- `run_three_update_steps`: Calls three update routines in order and returns.
- `execute_nested_routines`: Executes a sequence of three calls to another routine.
- `recurse_indefinitely`: This routine calls itself recursively without any conditional checks or state changes.
- `build_shifted_row_masks`: Walks a list of records that index a width table, builds a bit mask for each row by shifting a 32-bit word, and combines the mask into bitmap rows at a 40-byte stride.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `increment_table_values`: Iterates through a nested loop to increment nibble-based values within a table pointed to by a register.
- `decrement_table_values`: Iterates through a nested loop to decrement nibble-based values within a table pointed to by a register.
- `decrement_and_update_table`: Decrements nibble-based values in a specific table and updates several offsets in a structure based on a loop counter.
- `configure_hardware_interrupts`: Modifies system control settings by writing to hardware registers, including interrupt enable and DMA control registers.
- `execute_script_step`: Sets a global variable to a specific value, calls another routine, and then writes a value to a structure offset.
- `write_structure_constants`: Writes several 16-bit and 32-bit constants, a function pointer, and a zero value into a structure at specific offsets.
- `update_entity_attribute_table`: Updates a block of memory using values derived from registers and a counter.
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
- `wait_while_flag_bit_set`: Returns at once when bit six of a byte at a fixed offset in an address register is clear.
- `execute_infinite_loop`: This routine consists of an infinite loop that performs no operations.
- `initialize_memory_buffers_and_tables`: Clears several memory regions with zeros and fills others with specific bit patterns.
- `decrement_global_counter`: Decrements a global variable and performs a conditional reset to a specific value if a second global variable is non-zero and the counter reaches four.
- `increment_global_counter`: Increments a global variable and performs a conditional reset to a specific value if a second global variable is non-zero and the counter reaches four.
- `initialize_data_tables`: Fills several consecutive memory areas with repeating sequences of four 16-bit values and then writes a set of 16-bit values to a structure pointed to by a global variable.
- `update_structure_values`: Decrements values within a structure based on a global variable and a counter, applying a minimum threshold to the results.
- `update_structured_counters`: Iterates through a table of 16-bit values to increment or add them to a base value, applying a wrap-around threshold.
- `update_structure_offsets`: Calculates a base value from a global variable and a pointer, then stores several derived 16-bit values into a structure at specific offsets.
- `transform_string_data`: Iterates through a sequence of bytes, modifies each byte based on specific character matches, and writes the results to a new memory location.
- `update_loop_counters`: Decrements a global variable and a loop counter repeatedly, resetting the loop counter to a specific value if it becomes negative.
- `store_calculated_pointers`: Calculates a memory address based on a value from a pointer and stores the resulting address and its high word into several global variables.
- `tiny_ori_2`: Small routine of 1 instruction that merges 23 into d4.
- `perform_recursive_calls`: This routine executes itself multiple times via recursion.
- `tiny_ori_sub`: Small routine of 2 instructions that merges 3 into d0, then subtracts d0 from d0.
- `check_global_flag`: Checks if a global variable is zero and, if so, clears a memory location before calling another routine twice.
- `copy_memory_block`: Copies a block of data from a source location to a destination location using a loop that processes 20 bytes per iteration.
- `modify_hardware_registers`: Writes values to specific hardware registers to modify system control settings.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_audio_registers`: Writes frequency and volume data from a table into the Amiga audio hardware registers.
- `update_audio_hardware_registers`: Parses a data structure to retrieve frequency and volume parameters from a lookup table and writes them to the Amiga audio hardware registers.
- `compare_period_to_target`: Masks a 12-bit period word and stores it in a second field, clearing a flag byte.
- `update_timer_state`: Updates a 16-bit timer value based on a 4-bit mode selector.
