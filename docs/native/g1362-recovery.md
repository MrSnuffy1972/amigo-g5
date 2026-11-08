# Game 1362 (celtic legends (1991)(ubi soft)(m4)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01362). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802240 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 44 raw-word lines |
| Functions named and described | pass, 204 of 204 named (0 author symbols), 204 of 204 described (2 by rule, 200 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 151040 bytes, 196 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9956 of 10652 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 22954 |
| code | 45076 |
| data | 1453151 |
| padding | 995 |
| unused | 280064 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf:bootblock | 1 | 26 | 1 |
| logged disk load from Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf boot-block load | 1 | 88 | 4 |
| logged disk load from Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf boot-block load | 1 | 94 | 3 |
| logged disk load from Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf bytes 457728-549376 (code never loaded by a reference run) | 1 | 9811 | 182 |
| Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf bytes 639488-642048 (code never loaded by a reference run) | 1 | 633 | 14 |
| Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf bytes 694272-708096 (code never loaded by a reference run) | 1 | 0 | 0 |
| Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf bytes 722944-757248 (code never loaded by a reference run) | 1 | 0 | 0 |
| Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf bytes 816640-825344 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1435 non-empty blocks: matched against track-loaded segments
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1527 non-empty blocks: matched against track-loaded segments
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: the boot block reads 22528 bytes from disk offset 799744 to an address (found statically); recovered as a unit
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: the boot block reads 11264 bytes from disk offset 822272 to an address (found statically); recovered as a unit
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: the boot block reads 1024 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: bytes 457728-549376 look like code no run loaded: recovered position-independently (load address unknown)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: bytes 639488-642048 look like code no run loaded: recovered position-independently (load address unknown)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: bytes 694272-708096 look like code no run loaded: recovered position-independently (load address unknown)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: bytes 722944-757248 look like code no run loaded: recovered position-independently (load address unknown)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: bytes 816640-825344 look like code no run loaded: recovered position-independently (load address unknown)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: 7 logged loads cover 177152 bytes of the image
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: blocks 2-893 (bytes 1024-457728) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.18)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: blocks 1079-1136 (bytes 552448-582144) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.73)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: blocks 1167-1246 (bytes 597504-638464) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.40)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: blocks 1258-1309 (bytes 644096-670720) outside the filesystem hold data, not code (opcode density 0.001, entropy 0.40)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: blocks 1346-1354 (bytes 689152-693760) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.50)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 1 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf: blocks 1406-1410 (bytes 719872-722432) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.85)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: 1 logged loads cover 8704 bytes of the image
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 2-11 (bytes 1024-6144) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.10)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 25-441 (bytes 12800-226304) outside the filesystem hold data, not code (opcode density 0.006, entropy 5.95)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 525-1071 (bytes 268800-548864) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.39)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1074-1078 (bytes 549888-552448) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.59)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1081-1084 (bytes 553472-555520) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.74)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1091-1095 (bytes 558592-561152) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.92)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1098-1102 (bytes 562176-564736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.84)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1104-1107 (bytes 565248-567296) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.84)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1114-1115 (bytes 570368-571392) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.09)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1117-1118 (bytes 571904-572928) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.21)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1121-1125 (bytes 573952-576512) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.00)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1128-1131 (bytes 577536-579584) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.32)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1137-1141 (bytes 582144-584704) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.77)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1144-1149 (bytes 585728-588800) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.52)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1151-1154 (bytes 589312-591360) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.77)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1161-1189 (bytes 594432-609280) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.76)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1191-1196 (bytes 609792-612864) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.09)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1198-1201 (bytes 613376-615424) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.27)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1208-1212 (bytes 618496-621056) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.88)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1214-1219 (bytes 621568-624640) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.66)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1222-1224 (bytes 625664-627200) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.39)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1231-1232 (bytes 630272-631296) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.09)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1234-1235 (bytes 631808-632832) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.21)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1238-1242 (bytes 633856-636416) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.84)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1245-1248 (bytes 637440-639488) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.26)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1255-1257 (bytes 642560-644096) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.68)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1259-1259 (bytes 644608-645120) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.02)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1261-1266 (bytes 645632-648704) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.44)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1268-1271 (bytes 649216-651264) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.78)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1278-1548 (bytes 654336-793088) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.35)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1585-1593 (bytes 811520-816128) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.50)
- celtic legends (1991)(ubi soft)(m4)[cr skr](disk 2 of 2)/Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 2 of 2).adf: blocks 1635-1759 (bytes 837120-901120) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.41)
- 3 never-loaded region(s), 56832 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_engine_state_and_hardware` (Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf bytes 457728-549376 (code never loaded by a reference run)): Initializes several global pointers and hardware registers, including audio volume and display control, then performs a series of table updates and calls multiple subroutines to set up the system state. Kind: disk hardware/sync check. Pass path: continuation of execution after func_0x00007a9a.

## Functions


### Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf:bootblock

- `perform_sequential_dma_transfers`: Performs three sequential DMA transfers by configuring a structure with specific source, destination, and length values, then calling a routine at a pointer within that structure.

### logged disk load from Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf boot-block load

- `process_bit_counts`: Reads a longword from a buffer, determines a count based on its value, and uses a loop to write bytes to a destination pointed to by a3.
- `test_d5_then_return`: Tests whether the D5 register shifted right by one is nonzero, then returns in either case without doing anything else.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `count_set_bits_in_register`: Iterates a loop to count the number of set bits in a 32-bit value held in a register by shifting the value and decrementing a counter until the value becomes zero or the counter reaches zero.

### logged disk load from Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf bytes 457728-549376 (code never loaded by a reference run)

- `initialize_engine_state_and_hardware`: Initializes several global pointers and hardware registers, including audio volume and display control, then performs a series of table updates and calls multiple subroutines to set up the system state.
- `update_state_parameters`: Performs a series of conditional checks and data updates involving a table, a global variable, and several other global variables, including a call to another routine and a call to a function at a fixed address.
- `configure_blitter_and_tables`: Calls a blitter routine, performs several subroutines, and then enters a loop that calculates an index to fetch a byte from a table and writes it to a global variable, potentially calling another routine.
- `configure_blitter_chain`: Configures Blitter registers by writing to hardware addresses and sets up a sequence of four Blitter operations using a table of source and destination pointers.
- `filter_index_table`: Iterates through a range of indices and writes them to one of two destination tables if the value at an offset in a table is less than a global word.
- `mask_attribute_data`: Iterates through a table to perform bitwise AND operations on sixteen 16-bit words at specific offsets within a structure pointed to by a global variable, then calls another routine.
- `copy_data_blocks_from_table`: Reads a value from a table, calculates an offset into a structure using a global variable, and then copies sixteen 16-bit words from a source table to that structure in a loop.
- `update_global_pointer_and_call_routines`: Calls a routine, then checks a global variable to either clear a value at a specific address or update it with a value from a table using a shifted index, before calling two other routines.
- `update_entity_attribute_table`: Iterates through a list of entities to update their attributes in a table, performing byte-swapping and bit-clearing operations on specific structure offsets.
- `fill_array_with_swapped_bytes`: Loops eight times to write values into a table at a global address, where each value is constructed by swapping bytes and combining them with a byte from a structure, then calls another routine if a specific memory location is not negative.
- `handle_scaled_value`: Performs arithmetic on a global word, calls another routine, and if certain conditions are met, populates a structure with values from two different tables.
- `sync_structure_pointers`: Saves registers, calls a subroutine, and if a global variable is non-zero, copies several fields from one structure to another, performs a conditional subroutine call, and then calls several other subroutines.
- `process_state_and_swap_values`: Checks several global conditions and, if certain criteria are met, swaps two 16-bit values between different global locations and calls multiple routines.
- `wait_and_reset_flag`: Loops until a byte in a global variable is zero, then sets a global word to 1, stores a byte in a global variable, calls another routine, sets the sign bit of that global variable, clears the global word, and returns.
- `check_conditions_and_update_state`: Checks several global variables and a hardware register, calls a subroutine, increments a global word in a loop, and if a condition is met, updates several global words and calls two other routines.
- `iterate_and_update_asset_table`: Checks a global word, iterates through a table of structures using a loop, modifies bytes within those structures based on specific conditions, and calls two other routines if the loop completes.
- `calculate_and_store_table_pointers`: Calculates two new addresses by multiplying values from two global variables by 88 and adding a base address, then stores these addresses and two 32-bit words from the resulting locations into global variables.
- `process_entity_attribute_tables`: Iterates through a table of eight structures, performing conditional updates on their internal fields based on bitwise checks of a status byte at offset 11.
- `calculate_scaled_value`: Saves registers to the stack, loads a pointer from a global variable, extends a word in d1, multiplies d1 by 12, compares a global word to zero, and branches based on the result.
- `set_pointer_based_on_global_condition`: Sets a global pointer to one of two different addresses based on the value of a global variable and the sign of a 16-bit register.
- `call_routine_with_sequence_of_values`: Writes a sequence of six different 16-bit values into a global variable and calls another routine after each write.
- `manage_asset_table_and_audio`: Checks a global variable and, if non-negative, processes a structure by retrieving values from a table and updating its fields.
- `update_structure_from_table`: If a global variable is nonzero, it calculates an offset from a pointer, reads a value from a table using a shifted index, and writes it to the structure, then clears three fields in the structure if a specific field equals two.
- `process_scaled_values`: Calls a routine to handle scaled values, performs several conditional subroutine calls based on global variables, and updates a global word with the result of a routine call.
- `update_asset_metadata_and_pointers`: Clears a 16-bit global variable and, if a global variable is non-zero, calculates a new pointer by adding 18 to the existing pointer, then updates a structure with a value from a table at an offset determined by a shifted 16-bit value.
- `load_audio_parameters`: Saves registers to the stack, loads a pointer from a global variable, checks if the pointer equals a specific value, calls another routine, loads a new pointer from a global variable, calls a third routine, and updates the original global variable with the new pointer before returning.
- `call_subroutines`: Calls four different subroutines in sequence and then returns.
- `update_entity_states`: Iterates through a list of entities, compares their properties against a value from a global variable and another table, and updates a third table with new values and flags based on the results.
- `save_and_restore_globals`: Clears a 16-bit value in a global variable, calls two other routines, and restores two previously saved 16-bit values from the stack to global variables.
- `update_entity_audio_and_pointers`: Initializes a table of pointers from a data source, updates entity structure fields using a loop, and manages audio playback by setting hardware registers and calling audio-related routines.
- `update_entity_asset_and_audio_state`: Calculates a pointer into a table using a global variable multiplied by 88, stores a word from a register at that location, and updates several structure fields including a pointer, a status byte with its high bit cleared, and two offset-based values derived from other tables.
- `process_complex_asset_logic`: Performs multiple conditional checks on global variables, updates a table of pointers, modifies hardware registers, and executes a sequence of subroutines including several loops and conditional jumps.
- `execute_sequence`: Checks a global variable and, depending on its value, performs a series of subroutine calls and hardware register writes, including setting a value at a specific memory location.
- `update_asset_pointers`: Calculates offsets into two tables using a global variable, copies data between memory locations, and performs a loop to update a structure based on a value from a table.
- `update_asset_state_and_counters`: Updates various fields within a structure pointed to by a global variable based on a state value, including decrementing byte counters at specific offsets and clearing bits in a status byte.
- `load_audio_sequence`: Loads two values from tables, configures hardware registers, and enters a loop that calls three different routines sixteen times.
- `play_audio_sequences`: Sets hardware registers, calls another routine, and then executes two separate loops to call three different routines for a specific number of iterations.
- `check_and_play_audio`: Saves a0 to the stack, loads a pointer into a0, compares a global word to zero, and either calls another routine and branches or calls a third routine before returning.
- `update_asset_status_and_trigger_event`: Checks a global variable to determine if an asset structure should be modified, then clears several bytes at specific offsets within that structure and calls another routine to process a state change.
- `update_global_counter`: Saves registers to the stack, loads a pointer from a global variable, calls another routine, compares a byte in a structure to d0, and if the comparison fails, sets d0 to 1; otherwise, it calculates a new value using d2, calls another routine, masks d0 with 127, and stores the result in a global word.
- `process_table_entries`: Saves registers to the stack, initializes a global word to 65535, loads three pointers, copies a word from a global variable, calls another routine, and enters a loop that checks bytes from two tables, calls a third routine if conditions are met, and decrements a counter.
- `update_entity_attribute_tables`: Iterates through a table of entities to copy attribute data and reset specific status bits, while also updating a secondary table of values by writing 65535 to various memory locations and calling another routine.
- `update_structure_elements`: Loads a pointer from a global variable, retrieves a value from a table using an offset derived from a word in the structure, and updates two different structure fields based on a loop that runs up to four times.
- `check_byte_presence`: Clears a byte at a specific address and then loops up to four times to check if a value from a global variable matches any byte in a table, setting a byte to one if a match is found.
- `process_entity_table_and_audio`: Iterates through an entity table, updating structure fields from a data table and managing audio state.
- `setup_audio_structure`: Saves registers to the stack, loads a pointer from a table using an offset in d0, copies values into a structure pointed to by a0, calculates a new pointer using a multiplier, and calls another routine or a third routine based on a global word.
- `access_audio_data`: Saves registers to the stack, loads a pointer from a global variable, adds d1 to itself twice, loads a new pointer from an offset of that pointer, and calls another routine or a third routine based on a global word.
- `execute_conditional_sequence`: Saves several values to the stack, checks if a word in d0 is non-negative, and if so, updates a global variable, calls another routine, sets a value at a pointer in a0 to 65535, and calls two more routines before restoring the stack.
- `save_and_reset_blitter_state`: Saves three values from the stack and a global variable, then checks if a register is negative; if not, it stores that register into a global variable, calls three different routines, sets the value pointed to by a0 to 65535, and restores the previously saved values before returning.
- `set_status_value`: Checks four different word values at specific offsets within a structure and writes a corresponding constant to a global variable, or writes a default value if no conditions are met.
- `process_entity_data`: Saves registers, checks a byte in a global variable, and if non-zero, performs a series of operations including a loop, table lookups, and multiple subroutine calls to update structure fields and global pointers.
- `process_table_data`: Checks a global byte, and if zero, enters a loop that calculates table offsets using a global word, updates structure fields with values from a table, and calls several routines including one that performs data verification.
- `update_state_variable_from_structure`: Checks four specific 16-bit words at fixed offsets within a structure pointed to by a global address and writes a corresponding constant value to a global variable based on which word is negative; otherwise, it writes a default value of negative one.
- `select_parameter_index`: Saves registers, checks several word values at specific offsets within a structure pointed to by a0, sets d0 to a specific constant based on those checks, and calls several subroutines to update global values.
- `select_entity_type_and_update_state`: Checks several pairs of 16-bit values within a structure to determine a type constant, then calls two routines to process data from that structure, stores the resulting constant into two global variables, and calls three other routines while updating a global pointer with a value from a different global variable.
- `process_structure_loop`: Sets a status bit in a global variable, copies d0 and d1 into a structure, retrieves a value from a table using an offset from the structure, and enters a loop that calls another routine based on a value in d7.
- `update_entity_attributes_and_states`: Iterates through a list of entities, updating their properties by reading values from a table and applying offsets.
- `load_structure_data_2`: Saves registers to the stack, sets several global words to -1, calculates an offset into a table using d0 multiplied by 6, loads a byte from a second table, and if the byte is positive, calls another routine after updating several global variables.
- `update_flags_from_table`: Reads two bytes from a table using indices from global variables, compares these bytes against specific values, and updates two different global variables with either one or zero based on the comparison.
- `copy_data_and_set_flag`: Copies a longword from a table to a destination address using an offset derived from a word at a specific offset, then checks a byte at that offset and clears a global variable if it equals two.
- `iterate_and_copy_data`: Saves registers to the stack, clears a global word, and enters a loop that checks bit flags and byte values in two different tables, calling another routine and writing a value to a destination structure if specific conditions are met.
- `update_memory_masks`: Calculates a value based on a byte from a table, then uses a loop to write zero or a bit-shifted value to four different memory locations.
- `write_bytes_with_shift`: Checks a global variable and either writes the value zeroxFF to four memory locations using either a logical shift right or a logical shift left based on a register value, or performs simple pointer increments and writes.
- `wait_for_condition`: Enters a loop that repeatedly checks a value derived from a global variable and a hardware register against two input values.
- `update_global_byte`: Reads a byte from a global variable, adds a byte from a hardware register to it, masks the result with 127, and stores it back to the global variable if the result is between two values provided in registers.
- `fetch_table_value`: Loads a byte from a table at an offset determined by a second table and a value in d0, then returns it as a signed word.
- `load_structure_data`: Initializes four memory locations with 65535, calculates an offset into a table using a multiplier, and then either copies four words from that table location to the memory locations or copies them in a different order based on a comparison with a global variable.
- `update_pointer_list`: Moves a pointer from the stack to a0, reads a word from an offset of a pointer at a0, and writes that word to two different offsets of the same pointer.
- `copy_data_block_2`: Populates a structure with several constants and a pointer from a global variable, then calls another routine to perform a data copy.
- `reset_blitter_state_2`: Clears a word in a global variable, calls another routine, clears another global variable, and finally sets the first global variable to one.
- `reset_blitter_state`: Calls another routine, clears a longword in a global variable, calls another routine, and sets that same global variable to one before returning.
- `check_status_conditions`: Clears the interrupt bit in the status register, compares two global words, and if the difference is negative, performs bitwise operations and potentially calls another routine based on several global word comparisons.
- `check_status_and_return`: Calls another routine and returns if a global variable is zero, otherwise it returns immediately.
- `initialize_asset_table_pointers`: Calculates a value based on the difference between a global variable and a constant, then performs conditional writes to two global variables and calls another routine.
- `initialize_bitmask_tables`: Stores two specific values into a global variable, increments it if it is non-zero, and calls another routine.
- `set_bitmask_pointers`: Stores two values into a global variable, then increments that variable if it is non-zero before calling another routine.
- `assign_table_addresses`: Writes two values to a global variable, increments it if it is non-zero, and calls another routine.
- `wait_and_set_global`: Loops until a global variable is non-zero, then calls another routine and stores a value into a global variable.
- `setup_blt_registers`: Assigns multiple values to a table in memory and calls another routine while interacting with hardware registers.
- `wait_and_reset_global`: Polls a global variable until it is non-zero, performs a sequence of writes to that same global variable, and then calls another routine.
- `wait_and_assign_constants`: Polls a global variable until it is non-zero, then calls a routine, assigns a value from a pointer to several global variables, sets three specific constant values to global variables, and optionally increments a global variable before calling another routine.
- `wait_and_assign_constants_alt`: Waits for a global variable to become non-zero before storing a different set of constant values into global variables and calling another routine.
- `wait_and_copy_table_data`: Polls a global variable until it is non-zero, calls a routine, assigns constants to global variables, calls another routine, and then enters a loop that copies specific words from a source table to four destination tables at regular intervals.
- `update_table_pointers_and_copy_data`: Checks a global flag, calls another routine, and updates several global pointers with new values or offsets.
- `wait_and_assign_constants_v3`: Polls a global variable for a non-zero value and then writes a specific set of constants to global memory.
- `wait_and_configure_blt`: Polls a global variable until it is non-zero, calls a routine, assigns constants to global variables, calls another routine, and then configures the Blitter registers using two different sets of values based on a bit test of a hardware register.
- `assign_and_call_blt`: Stores three constant values into global variables, calls a routine, and then stores two more constant values into global variables.
- `store_constant_pointers`: Calls another routine and then stores two constant longwords into two specific global variables.
- `setup_parameters_and_call_routine`: Stores three longwords into consecutive global memory locations, calls another routine, and then stores two more longwords into separate global memory locations before returning.
- `set_global_pointers_and_call_subroutine`: Stores three longwords into consecutive memory locations, calls another routine, and then stores two more longwords into a separate memory area before returning.
- `update_nibble_table`: Iterates through a table, compares nibbles from a source at a pointer in a6 with values in a structure, and updates the structure with modified nibbles or copies values from another memory area.
- `increment_table_values`: Enters a loop that iterates a number of times specified by a value from a global variable, and for each iteration, adjusts the nibbles of a word in a destination table to match the nibbles of a word in a source table by incrementing or decrementing them.
- `decrement_table_nibbles`: Decrements the nibbles of values in a table through a loop, then optionally copies several words between memory locations and calls another routine.
- `update_nibble_values`: Iterates through a list of 26 words, comparing four nibbles of each word from a source pointer to a target table and adjusting the nibbles in the table by increments of 1, 10, or 100 based on the comparison results.
- `clear_and_update_nibbles`: Clears 26 words in a table starting at an offset of 6 from a pointer, then iterates 32 times to update nibbles in a source list based on a target table.
- `clear_word_table`: Fills 32 words at a location pointed to by a global variable with zero, then clears five specific global word variables.
- `copy_and_conditional_clear`: Copies 32 words from a source to a table, then either clears five global word variables if a global variable is non-zero or loads them from specific memory locations if it is zero.
- `copy_long_words`: Copies 16 long words from a source pointer to a destination pointer starting at a fixed address.
- `rotate_bitmask_loop`: Performs a loop eight times, calling another routine and rotating a longword in d2 for each iteration.
- `calculate_offset_values_2`: Reads values from two tables, performs arithmetic to calculate several new values, and then executes a loop that calls a routine eight times.
- `apply_bitmask_to_planes`: Performs a bitwise operation on four different memory areas using a rotating bitmask and a value from a source pointer, controlled by two nested loops.
- `apply_bitmask_loop`: Performs a bitwise operation to set or clear bits in four separate memory areas using a rotating bitmask through a loop of 1999 iterations.
- `apply_rotated_mask`: Performs a loop seven times, rotating a longword in d2 and calling another routine.
- `mask_memory_with_bits`: Performs a bitwise AND operation on four memory areas using a value from a register and then rotates that register through a loop of 1999 iterations.
- `play_audio_sequence`: If certain conditions are met, sets audio registers to play a sequence of values from a pointer and then restores the original register settings via a call.
- `load_audio_sample_parameters`: Checks if a global flag is zero and a pointer is non-null, then calls another routine to load a sample, copies five values from the resulting pointer into audio hardware registers and a global variable, and calls that same routine again after updating the audio control and volume registers.
- `duplicate_pointer_array_segments`: Splits two 32-bit values into high and low words and stores them in two separate locations, then calculates and stores three subsequent offsets based on the original value for each location.
- `configure_blt_source`: Waits for a hardware flag and then configures Blitter registers with two different source pointers and waits for a hardware flag to clear.
- `setup_blitter_registers`: Waits for a hardware flag and then sets Blitter registers with two specific source pointers and a control word, then waits for a hardware flag to clear.
- `wait_for_flag`: Polls a word at a global variable address until it is non-zero, then calls another routine.
- `execute_blitter_draws`: Iterates through a table, calculates Blitter parameters including source and destination addresses, and performs four Blitter operations per valid entry.
- `update_blitter_sprite_table`: Iterates through a data table to update a sprite structure, calculating new source and destination coordinates and color attributes for the Blitter.
- `copy_data_table`: Calls another routine, then enters a loop that runs 15 times to fetch two words from a table based on values at a specific memory location, adds them to values from another table, calls another routine, and stores the result into a global array.
- `configure_blitter_graphics`: Performs a nested loop to configure Blitter registers and DMA settings by reading from a table, calculating source and destination addresses, and writing values to hardware registers including color and area parameters.
- `draw_blitter_sprite_tables`: Iterates through several tables of data to perform multiple Blitter area fill operations.
- `clear_memory_blocks`: Clears several memory blocks by zeroing out 2457 bytes at offset 342d6, 64 bytes at offset 341d6, 56 bytes at offset 16636, and 1024 bytes at offset 38cd6 with all ones, after clearing all data registers.
- `update_data_tables`: Iterates through a loop of 256 iterations, using a global variable as a base offset to write specific byte values to two different memory locations based on the contents of a third location.
- `write_bytes_to_table`: Calculates a table address using a global variable multiplied by 37, then iterates through 40 bytes of that table, writing a byte from a register if the current byte is zero.
- `update_state_buffers`: Calculates offsets into two buffers based on a global value, then performs several loops to conditionally update bytes in those buffers or a third buffer based on values in a global variable and a source buffer.
- `update_status_bytes`: Iterates through several loops to check bytes in two tables and writes specific values to those tables based on the results of another routine.
- `process_buffer_updates`: Performs a loop nine times to conditionally copy a byte from one buffer to another, then performs a loop three times to update a byte in a buffer if a value in another buffer is zero.
- `update_conditional_values`: Executes multiple loops to update bytes in two different buffers based on comparisons with a constant, and conditionally updates a third buffer if a global variable is zero.
- `modify_buffer_elements`: Runs several loops to update bytes in two buffers based on values in a third buffer, and conditionally updates a fourth buffer if a global variable is zero.
- `set_table_values`: Writes specific byte values to various offsets in two tables and, if a global variable is zero, calls another routine to set bytes in a third table.
- `initialize_data_tables`: Writes a series of constant bytes to two tables at offsets calculated from a global variable, then calls another routine if a global variable is zero.
- `check_global_condition`: Calls another routine if a global variable is equal to zero, otherwise it returns.
- `recurse_on_zero_flag`: Checks if a global variable is zero and, if so, calls itself recursively.
- `recurse_on_zero_global`: Checks if a global variable is zero and, if so, calls itself recursively.
- `execute_subroutine`: Calls another routine and returns its result.
- `process_flag_table`: Clears data registers, then iterates through a loop to copy a byte from another routine into a table and sets a bit in a second table based on values from a third table.
- `setup_graphics_parameters`: Performs a series of assignments to global variables and calls another routine multiple times to initialize various state flags and constants.
- `update_status_and_call_routines`: Checks a global variable and a byte at an offset from it, then optionally writes a byte to that offset, updates two global variables, and calls several other routines.
- `execute_blitter_sequence`: Checks a global variable, sets hardware registers for the custom chip, and performs a sequence of blitter operations using data from a table to update memory areas.
- `render_sprite_and_layer_data`: Initializes graphics parameters and updates status flags before performing a series of blitter operations to render sprite tiles and display layers.
- `fill_memory_with_byte`: Fills a block of memory starting at a specific address with the value 255 using a loop.
- `swap_nibbles`: Performs a mathematical operation on a global variable to swap its high and low nibbles.
- `dereference_table_indices`: Uses a global variable as an index into a table to load a new value, then uses that value as an index into a second table to load a final value into the global variable.
- `initialize_hardware_pointers`: Saves registers, calls two routines, and sets up three pointers in a5 and a6 by adding an offset to global addresses before branching.
- `configure_hardware_parameters`: Clears data registers, modifies values in a structure at a5, and performs several loops that wait for a hardware bit to clear before writing values to a structure.
- `branch_to_subroutine`: Saves registers to the stack, calls two subroutines, loads a pointer into a0, sets d4 to 107, and branches to another routine.
- `update_blitter_parameters_and_render`: Iterates through a table of sixteen entries, calculating blitter parameters and updating hardware registers including color modes and DMA control.
- `copy_data_block`: Copies 16 words from a source table to a destination table, or clears the destination if a global variable is zero, or uses a different source if a global variable is five.
- `sort_data_block`: Performs a bubble sort on a block of bytes using two input parameters to define the range and step.
- `assign_incremental_values`: Iterates through a list of words and, if a word is non-negative, replaces it with an incrementing counter.
- `fetch_nested_byte`: Calculates an offset using two input parameters to fetch a byte from a table and then fetches another byte from an offset relative to that value.
- `calculate_offset_values`: Calculates and stores two values into global variables based on a global variable and a conditional check.
- `copy_table_data`: Iterates through a nested loop to fetch bytes from a table and store them into three different memory locations.
- `clear_memory_buffers`: Clears two memory buffers by filling them with zeros in a loop, then fills a third memory area with bytes set to 255.
- `update_data_buffers`: Checks a global variable and, if it is greater than 22, performs a series of memory copies and blitter operations using values from a table and specific source/destination addresses.
- `copy_data_blocks`: Performs multiple nested loops to copy bytes from one source address to a destination address using specific offsets and increments.
- `setup_blitter_transfers`: Configures several hardware registers for the blitter and performs four consecutive blitter transfers using specified source and destination addresses.
- `setup_blitter_loop`: Calls another routine, then checks a bit in a custom register to either clear several hardware registers or enter a loop that writes specific values to hardware registers and updates two pointers four times.
- `update_graphics_parameters`: Calculates several values based on a table and a global variable, then configures blitter registers and performs multiple transfers to update memory.
- `update_display_layers`: Sets hardware registers for multiple blitter passes to draw different data sets based on a global value.
- `setup_blitter_parameters`: Configures blitter registers with specific color and address values after waiting for a synchronization flag.
- `toggle_boolean_flag`: Checks if a value shifted left by two matches a global variable, then toggles a global byte between zero and one.
- `render_blt_sprite_table`: Iterates through a structured data table to perform multiple Blitter operations.
- `render_sprite_tiles`: Iterates through a data table to fetch sprite or tile parameters and performs multiple blitter operations to draw graphics to screen memory.
- `clear_all_data_registers`: Clears all unused or temporary data registers to zero and returns.
- `process_input_and_state`: Waits for a bit in a global variable, then performs complex logic involving joystick/mouse input, updating several global counters and values based on bitwise checks and table lookups.
- `update_pointer_data`: Updates two identical memory locations by writing a character derived from a global variable and performing arithmetic on the resulting bytes.
- `update_data_structure`: Clears all data registers, then performs arithmetic on values from a pointer in a1 and updates a global variable based on a bit test of d0.
- `wait_for_bits`: Enters a loop that waits for specific bits to be set in a global variable, which is checked against the CIAA_PRA register status.
- `manage_game_state`: Checks a global variable against specific constants and updates two other global variables based on the result.
- `calculate_and_draw_blitter_rects`: Performs arithmetic to calculate dimensions and positions, then enters a loop that configures and executes four blitter operations using values from a table and hardware registers.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `configure_blitter_parameters`: Waits for a specific raster line, then loads several values from a table into registers and uses them to configure blitter registers and memory pointers.
- `process_pointer_table`: Initializes a global variable and enters a loop that waits for a pointer in a memory location to change from a specific value before accessing data through that pointer.
- `set_ram_pointers`: Stores a constant into a global variable, loads that value into a0, sets three other global variables to zero, and branches to another routine.
- `set_ram_pointers_alt`: Stores a constant into a global variable, loads that value into a0, sets three other global variables to one, and branches to another routine.
- `initialize_ram_pointer`: Sets a global variable to a specific value and then loads it with data from a memory address.
- `update_color_palette`: Updates color registers based on the value of a global variable and contains a loop that copies a block of data from one memory location to another.
- `fetch_table_value_2`: Sets two global variables to fifteen, calls another routine, and then uses the resulting global variable to index into a table to update itself.
- `reset_global_variable`: Evaluates a global variable against several constant values and sets it to zero if certain conditions are met.
- `accumulate_table_values`: Iterates backwards through a table, adding bytes to a global variable if specific conditions are met, and clamps the result to a maximum value.
- `update_color_palette_registers`: Checks a global variable, calls another routine, and if not equal to one, performs three separate data transfers using a sub-routine to update specific memory locations before finally filling the first three color registers with 10000 in a loop.
- `process_pointer_and_buffers`: Waits for a pointer in a memory location to change, then performs several loops to copy data between buffers and update color registers based on a global variable.
- `search_and_set_globals`: Searches through a memory range for the value seven and, upon finding it, populates several consecutive global variables with specific values.
- `populate_lookup_tables`: Selects a source table based on a conditional check and copies its contents into several global memory locations.
- `search_and_initialize_ram`: Iterates through a memory range looking for a specific byte sequence, then populates several consecutive global bytes with a constant value.
- `copy_character_data_2`: Copies a block of data from a source buffer to a destination based on a count derived from global variables, and contains several loops that iterate through tables.
- `copy_character_data`: Parses a source buffer for specific byte values to select data from various tables and copies blocks of data to a destination buffer.

### Celtic Legends (1991)(UBI Soft)(M4)[cr SKR](Disk 1 of 2).adf bytes 639488-642048 (code never loaded by a reference run)

- `execute_complex_logic_sequence`: Sets hardware registers for the custom chip, performs a series of subroutine calls to update various tables and states, and finally writes specific bytes to a structure at offset 2 of a pointer in a0.
- `update_status_table`: Iterates through a table and, if a specific byte is negative one, searches for a matching value in a secondary table to set that entry to negative one.
- `update_entity_state_table`: Iterates through a table of data structures to increment counters and update state values based on indexed lookups.
- `search_sorted_byte_array`: Retrieves a target byte from a table and performs a linear search through six signed short values stored in another table to determine if the target byte is less than, equal to, or greater than the values, returning the comparison result or an index to a global variable.
- `update_global_via_table`: Multiplies a global variable by eight, uses the result as an offset into a table, and stores the retrieved byte back into a global variable.
- `search_table_for_match`: Iterates through a table using a global variable as an index and compares a byte at a calculated offset with the index value, returning negative one if no match is found within six iterations.
- `set_table_flags`: Sets a byte in a table to zero using a global variable as an index, then iterates through a list to set specific entries to one.
- `increment_table_values_2`: Compares a global variable against values in a table and, if they match and certain conditions are met, increments a byte in a separate table.
- `mark_table_entries`: Performs nested loops to compare values within a table against a global variable and another value, setting specific table entries to a specific byte if a match occurs.
- `flag_table_entries`: Iterates through a range of one hundred and twenty-eight entries and sets a byte to a specific value in a table if the entry matches a global variable and is not equal to another value.
- `update_flag_table`: Iterates through one hundred and twenty-eight entries in a table and sets a byte to a specific value if the entry matches a global variable and the index does not match the global variable.
- `initialize_status_bytes`: Sets a global variable to one and iterates through a table, setting bytes based on the values found in another memory location.
- `copy_memory_block`: Copies two thousand fifty-six bytes from the memory location at a fixed address to the memory location at a fixed address.
- `copy_2048_bytes`: Copies 2048 bytes from one memory location to another using a loop that decrements a counter initialized to 2047.
