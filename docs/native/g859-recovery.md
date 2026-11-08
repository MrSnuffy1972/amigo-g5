# Game 859 (air support (1992)(psygnosis)[cr skr]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00859). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1806496 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 28 raw-word lines |
| Functions named and described | pass, 76 of 76 named (0 author symbols), 76 of 76 described (1 by rule, 73 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 284 of 284 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 29184 bytes, 31 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2953 of 3214 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 32177 |
| code | 11950 |
| data | 1486896 |
| padding | 17 |
| unused | 275456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf:bootblock | 1 | 16 | 1 |
| Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf:bootblock | 1 | 16 | 1 |
| logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf boot-block load | 1 | 94 | 3 |
| logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf boot-block load | 1 | 88 | 4 |
| logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 770 | 36 |
| Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf bytes 6144-8192 (code never loaded by a reference run) | 1 | 0 | 0 |
| Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf bytes 12288-39424 (code never loaded by a reference run) | 1 | 2230 | 31 |

## Cases handled

- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1697 non-empty blocks: matched against track-loaded segments
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1281 non-empty blocks: matched against track-loaded segments
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: the boot block reads 22528 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: the boot block reads 4608 bytes from disk offset 867328 to an address (found statically); recovered as a unit
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: the boot block reads 22528 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: the boot block reads 1536 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- runtime segment an address: 284 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: bytes 6144-8192 look like code no run loaded: recovered position-independently (load address unknown)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: bytes 12288-39424 look like code no run loaded: recovered position-independently (load address unknown)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: 2 logged loads cover 27136 bytes of the image
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 2-1123 (bytes 1024-575488) outside the filesystem hold data, not code (opcode density 0.004, entropy 7.18)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1146-1157 (bytes 586752-592896) hold ASCII text
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1163-1167 (bytes 595456-598016) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.50)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1173-1173 (bytes 600576-601088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1177-1184 (bytes 602624-606720) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.73)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1192-1200 (bytes 610304-614912) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.13)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1208-1215 (bytes 618496-622592) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1224-1231 (bytes 626688-630784) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.78)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1236-1693 (bytes 632832-867328) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.81)
- air support (1992)(psygnosis)[cr skr](disk 1 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf: blocks 1703-1715 (bytes 871936-878592) hold ASCII text
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: 4 logged loads cover 53248 bytes of the image
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 5-11 (bytes 2560-6144) hold ASCII text
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 80-332 (bytes 40960-170496) outside the filesystem hold data, not code (opcode density 0.052, entropy 6.01)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 338-338 (bytes 173056-173568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.15)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 341-341 (bytes 174592-175104) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 345-345 (bytes 176640-177152) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 375-508 (bytes 192000-260608) outside the filesystem hold data, not code (opcode density 0.008, entropy 6.35)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 511-517 (bytes 261632-265216) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.37)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 559-596 (bytes 286208-305664) outside the filesystem hold data, not code (opcode density 0.047, entropy 6.08)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 610-611 (bytes 312320-313344) outside the filesystem hold data, not code (opcode density 0.031, entropy 3.13)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 613-633 (bytes 313856-324608) outside the filesystem hold data, not code (opcode density 0.024, entropy 5.36)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 688-725 (bytes 352256-371712) outside the filesystem hold data, not code (opcode density 0.050, entropy 6.08)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 739-740 (bytes 378368-379392) outside the filesystem hold data, not code (opcode density 0.031, entropy 3.15)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 742-762 (bytes 379904-390656) outside the filesystem hold data, not code (opcode density 0.024, entropy 5.36)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 774-814 (bytes 396288-417280) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.47)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 936-999 (bytes 479232-512000) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.78)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 1080-1154 (bytes 552960-591360) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.35)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 1200-1274 (bytes 614400-652800) outside the filesystem hold data, not code (opcode density 0.008, entropy 4.19)
- air support (1992)(psygnosis)[cr skr](disk 2 of 2)/Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf: blocks 1320-1715 (bytes 675840-878592) hold ASCII text
- 1 never-loaded region(s), 2048 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_interrupt_settings` (Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf bytes 12288-39424 (code never loaded by a reference run)): Iterates through a loop to call three subroutines, then sets two 16-bit values at offsets 250 and 254 of a structure in a6 before returning. Kind: sabotage path. Pass path: u8__sub_09D8() is called.
- `configure_hardware_and_interrupts` (Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf bytes 12288-39424 (code never loaded by a reference run)): Saves registers to the stack, performs several memory and hardware register writes, calls multiple routines, and eventually jumps to another routine. Kind: disk hardware/track check. Pass path: sub_08FA.

## Functions


### Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf:bootblock

- `perform_dma_transfer_and_jump`: Sets up and executes two DMA transfers using the exec.DoIO routine, first transferring data from a memory location to a hardware address and then transferring data from a different memory location to another hardware address, before jumping to a new location.

### Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf:bootblock

- `configure_dma_and_load_code`: Configures two DMA transfers using a structure in a1 to move data from a source to a destination, calls a routine at a specific address, updates a global variable, and then performs a second DMA transfer before jumping to a new location.

### logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 1 of 2).adf boot-block load

- `write_decrementing_global_until_wrap`: Writes a 16-bit countdown into a global variable on each pass, looping until the counter wraps back to its starting value, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf boot-block load

- `decompress_and_copy_data`: Reads a byte from a source, performs bitwise shifts, and enters a loop to call another routine and copy bytes to a destination buffer.
- `test_d5_then_return`: Tests whether the D5 register shifted right by one is nonzero, then returns in either case without doing anything else.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_and_loop_data`: Performs a bitwise right shift on d5 using a loop controlled by d0, while simultaneously performing a rotate-left through the carry bit into d1.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_event_handler`: Checks a 16-bit value in a structure and several hardware/global flags, then performs a series of operations including a call to another routine and an indirect jump through a table.
- `execute_conditional_jump`: Moves a word from a structure into d0, and if d0 is not negative, uses it to index into a table to perform a call and a branch.
- `decrement_and_clear_counters`: Decrements a 16-bit value at an offset in a structure and clears another offset if the value reaches zero, otherwise it performs a comparison and branches.
- `rotate_linked_pointers`: Shifts three 32-bit pointers stored in a structure at a5 by moving the value from one offset to the next in a circular fashion.
- `jump_to_indirect_address`: Sets the processor status register to a specific value, loads a pointer from a global variable into a7, and jumps to the address contained in a0 which was loaded from a global variable.
- `clear_and_test_word`: Clears a 16-bit word at an offset of 1412 from a6 and returns if the resulting value is zero, otherwise it returns.
- `generate_dynamic_data_table`: Constructs a complex data structure in memory by performing arithmetic on values from a structure in a5, including bitwise operations, scaling, and loops that populate a series of word-sized entries with calculated values and constants.
- `scale_rgb_components_loop`: Clears a 16-bit value at offset 1474 of a structure in a5, then enters a loop that calls scale_rgb_components to process color data at offset 1518 of the structure, repeating the call a number of times determined by a value at offset 1612.
- `update_structure_from_pointer`: Loads a word from a pointer in a4 and, if the value is negative, writes 2 to offset 1668 of a5; otherwise, it writes the value to offset 1672 of a5 and writes 4 to offset 1668 of a5.
- `store_word_at_offset`: Stores a 16-bit word from a pointer in a4 into offset 1674 of a5.
- `store_word_and_constant`: Stores a 16-bit word from a pointer in a4 into offset 1672 of a5 and writes 6 to offset 1668 of a5.
- `clear_and_fill_structure`: Clears a 16-bit word at offset 1660 of a5, then stores three consecutive words from a pointer in a4 into offsets 1682, 1686, and 1690 of a5, and finally clears offset 1684 of a5.
- `store_word_at_indexed_offset`: Looks up an offset from a table indexed by the first word addressed by the pointer, then stores the second word at that offset in the slot record.
- `scale_rgb_components`: Processes three nibbles from a word at a0 by multiplying each by a value in d0, scaling the result, clamping it to a maximum of 15, and storing the combined 16-bit result at a pointer in a1.
- `initialize_structure_pointers`: Clears two 16-bit values in a structure, sets a pointer at an offset, and calls three different routines if a test of a pointer succeeds.
- `increment_and_rotate_pointers`: Increments a 16-bit value in a structure and enters a loop that calls another routine to update a pointer at a specific offset.
- `unpack_compressed_data_stream`: Reads a stream of bytes from a source pointer in a0 and writes them to a destination pointer in a1 using a custom decompression scheme.
- `decompress_bitmask_encoded_data`: Reads a header from a source buffer to update several values in a structure, copies sixteen 32-bit words from the source to a destination, and then performs a bitmask-based expansion to write variable-sized chunks of data into a destination buffer.
- `expand_run_length_encoded_data`: Decompresses run-length encoded data from a source buffer in a0 to a destination buffer in a1 using a bitmask-based expansion technique.
- `update_structure_pointer`: Loads an address from a global variable into a0 and branches to another location.
- `update_structure_data`: Stores a byte value of 255 at offset 24 of a structure in a0, updates several 16-bit fields in a0 using values from a2 and d1 or d4, and sets a bit in a global variable via a5.
- `calculate_bitmask_values`: Performs bitwise manipulations on a 16-bit value from a6 plus offset 12 to determine two 16-bit results, which are then stored at offsets 1436 and 1438 of a5.
- `adjust_clamped_counters`: Updates two 16-bit counters at offsets 1440 and 1442 of a5 by calculating differences between a byte from a6 plus offset 10 and a byte from a5 plus offset 1448, then repeats a similar process for a second byte at offset 1450, applying specific minimum bounds to the results.
- `handle_structure_flags`: Checks bitwise flags in a structure, and if specific conditions are met, increments a counter in a structure and performs an indirect jump through a table.
- `manage_buffer_and_counters`: Calls three routines, updates a longword at an offset in a structure if a condition is met, and manages two 16-bit counters in a structure.
- `write_constant_bytes`: Writes the bytes 127, 136, and 0 to specific offsets relative to a pointer derived from the program counter.
- `increment_conditional_value`: Checks a 32-bit value at offset 1602 of a5; if bit 23 is not set and bit 1 is not set, it stores the value doubled; otherwise, if bit 1 is set, it increments the value and stores it back at offset 1602.
- `dispatch_status_and_update_structure`: Tests bits 7 through 10 of a bitmask formed by masking two 16-bit words from a structure in a6, then branches to different routines based on which bit is set.
- `set_status_and_branch`: Writes a word to an offset in a structure and branches to another routine.
- `set_status_and_branch_2`: Writes the word 256 to an offset of 156 in a structure pointed to by a6, loads the address of a field in a5 into a0, and branches to another routine.
- `set_status_and_branch_3`: Writes the word 512 to an offset of 156 in a structure pointed to by a6, loads the address of a field in a5 into a0, and branches to another routine.
- `clear_status_byte`: Writes a word to an offset in a structure, checks a byte at an offset from a pointer in a5, and either clears that byte or branches to another routine.
- `decrement_and_copy_pointer`: Decrements a 16-bit value at an offset in a structure and, if non-zero, copies a longword to a destination address and a word to another offset.
- `clear_structure_fields`: Clears several words and a byte at offsets 8, 12, and 25 of a structure pointed to by a0, moves two words from a0 to a6, and branches to another location.
- `process_and_call_subroutines`: If a 16-bit value at offset 128 of a structure in a5 is non-zero, it sets two 16-bit values in a6 using that value and its bit-shifted version, clears the value at offset 128, and calls another routine four times with different offsets from a5.
- `update_limit_and_offset`: Updates a 16-bit value at offset 10 of a structure in a0 based on its sum with a value at offset 3, applying a limit of 512 and setting an offset at offset 8 of a pointer in a0, while also updating a 16-bit value at offset 14 of a0 and offset 6 of a1.

### Air Support (1992)(Psygnosis)[cr SKR](Disk 2 of 2).adf bytes 12288-39424 (code never loaded by a reference run)

- `process_input_and_update_camera_2`: Branches to another routine, performs several memory and hardware register writes, calls another routine, and contains several loops that manipulate data and hardware registers.
- `copy_color_palette`: Copies 32 words from a source table to the hardware color registers starting at the address in a0.
- `configure_interrupt_settings`: Iterates through a loop to call three subroutines, then sets two 16-bit values at offsets 250 and 254 of a structure in a6 before returning.
- `configure_interrupt_registers`: Sets various bits in the interrupt mask and enable registers by performing bitwise operations on d0 and d1 and writing the results to hardware registers.
- `clear_table_entry`: Calculates an offset into a table using the lower two bits of d0, shifts the result, and clears the 16-bit word at that location.
- `configure_hardware_and_interrupts`: Saves registers to the stack, performs several memory and hardware register writes, calls multiple routines, and eventually jumps to another routine.
- `update_hardware_and_structures`: Checks a global variable, writes values to hardware registers and various memory locations, calls several routines, and performs arithmetic on values from a global variable before returning.
- `check_and_call_routines`: Checks a longword at a specific address and, if non-zero, calls two different routines before returning.
- `run_nested_interpolation`: Sets a global variable to negative one and executes a nested loop that calls another routine one hundred and twenty-one times.
- `interpolate_two_points`: Calculates a base address using d6 and d7, then calls another routine twice with different pairs of pointers derived from that address.
- `calculate_interpolated_coordinate`: Checks bit flags in two structures to determine an offset for a global pointer, then performs a complex fixed-point interpolation calculation using values from the structures before calling another routine.
- `extract_and_store_bits`: Extracts bitwise components from two 16-bit values read from hardware registers and stores them into several consecutive word locations in a global table, then enters a nested loop that calls two different routines based on a global flag.
- `check_collision_and_copy`: Calculates a base address using d6 and d7, calls another routine twice with specific pointer offsets, and then copies a 16-bit word from a global variable to another global variable.
- `validate_and_check_collisions`: Performs multiple range and bitwise checks on values derived from d6 and d7 and global tables, then calls another routine twice with specific pointer offsets if the checks pass.
- `evaluate_entity_collision_states`: Checks if three entities have positive values at offset 2304 and exceed a threshold of 2000 at offset 5184, then updates a global variable and a secondary global variable based on shared bit flags in their status words.
- `update_structure_elements`: Iterates 144 times through a structure in a6, performing arithmetic on values at various offsets and writing results to offsets 2880, 3456, and 5184, or setting bit 15 of the value at offset 2304.
- `calculate_scaled_products`: Fetches two 16-bit values from tables at a1 and a2 using an index in d6, multiplies them by d0 and d1 respectively, and returns the results in d4 and d5 as 32-bit values.
- `process_input_and_update_camera`: Reads the vertical scroll position from a hardware register and updates several global variables based on joystick or mouse movement.
- `combine_bit_fields`: Combines several bit-manipulated values derived from a table and a global variable into a single 32-bit result returned in d0.
- `update_data_tables`: Copies three longwords from specific memory locations to a global variable, performs arithmetic on values from hardware registers, and uses a loop to call another routine and update a table of values.
- `scale_and_swap_values`: Multiplies d0 and d1 by zero, adds them to values derived from d0 and d1, and performs swaps on the resulting 32-bit registers before returning.
- `load_pointers_and_call`: Loads two addresses into a1 and a2 and calls another routine.
- `process_scaled_data`: Clears a memory block using a loop, updates a global value, and enters a nested loop that performs calculations using values from a table and hardware registers to update a structure at offset 2304 of a1.
- `execute_nested_loops`: Calculates a value based on hardware registers and enters a nested loop that calls another routine eleven times.
- `copy_data_block`: Calculates an index using d5 and d6, fetches a pointer from a table at 26e9c, and copies eight 32-bit words from that pointer to a destination at a1.
- `calculate_and_store_interpolated_values_2`: Writes the word 24000 to an offset of 76 of a structure, loads the address of a table into a5, and calls another routine before returning.
- `calculate_and_store_interpolated_values`: Performs complex arithmetic on values from two tables, stores the results in a buffer, calls a routine via a longword pointer, and then executes a loop that calls two different routines.
- `execute_scaled_calls`: Enters a loop that calls another routine eight times, incrementing a pointer by ten each iteration.
- `process_scaling_parameters`: Updates several global variables based on a value from a structure, then performs two separate calls to another routine after modifying a global pointer.
- `process_conditional_scaling`: Performs division-based scaling on two sets of values from a structure in a0 and, depending on bit tests of values at offsets 6 and 16, calls another routine with a pointer from a global variable.
- `clear_three_words`: Writes three 16-bit zero values to the memory locations pointed to by a1.
