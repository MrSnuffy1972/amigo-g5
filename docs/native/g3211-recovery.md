# Game 3211 (Cortex (Europe)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03211). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4896675 top-level bytes in 60 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 156 of 156 named (0 author symbols), 156 of 156 described (75 by rule, 73 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1232 of 1232 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 65408 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 8276 of 8276 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2501918 |
| code | 35462 |
| data | 1169073 |
| padding | 16718 |
| unused | 1173504 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Cortex (Europe).adf:LOADER | 2 | 465 | 43 |
| Cortex (Europe).ipf#adf:LOADER | 2 | 465 | 43 |
| Cortex (Europe).adf:bootblock | 1 | 10 | 1 |
| Cortex (Europe).ipf#adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2862 | 66 |
| Cortex (Europe).adf:SPACE (raw code file, load address unknown) | 1 | 2232 | 1 |
| Cortex (Europe).ipf#adf:SPACE (raw code file, load address unknown) | 1 | 2232 | 1 |

## Cases handled

- Cortex (Europe)/Cortex (Europe).ipf decoded by disk-analyse to a 901120-byte disk image (tracks: T0.0-79.1: AmigaDOS)
- runtime segment an address: 939 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Cortex (Europe)/Cortex (Europe).adf:SPACE: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- Cortex (Europe)/Cortex (Europe).ipf#adf:SPACE: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 executed PCs start inside another executed instruction (overlapping-instruction idiom): decoded and counted as instruction starts

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Cortex (Europe).adf:LOADER

- `initialize_loader`: Initializes the loader by allocating memory, copying data from a fixed source to a destination buffer, and setting up system registers before jumping to the main game routine.
- `frag_tst_beq_jsr_movea_moveq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then calls another routine, then copies a fixed memory address to a1, then calls another routine, then loads 0 into d0, then copies a fixed memory address to a7, then returns.
- `combines_bitplane_rows_into_buffer`: Processes a block of 16-bit words by ORing five source rows together, inverting the result, and storing it in one destination row while copying the source rows to five further destination rows at fixed offsets.
- `call_super_state`: Wrapper that calls the exec library function SuperState and returns its result to the caller.
- `call_user_state`: Wrapper that calls the exec library function UserState and returns its result to the caller.
- `frag_movea_lea_moveq_jsr_move_beq_rts`: Short code fragment that copies a fixed memory address to a6, then loads the address of a location in the code into a1, then loads 0 into d0, then calls another routine, then copies d0 to a fixed memory address, then branches if the last result was zero, then returns.
- `scan_directory_entries_2`: Scans directory entries using DOS calls and stores the lock handle in a global variable.
- `lists_directory_entries_until_done`: Repeatedly calls the DOS directory-scan call on a directory handle, and for each entry found calls two other routines; when the scan returns zero it reads the DOS error code, then closes the handle.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `frag_move_movea_jsr_tst_rts`: Short code fragment that copies a1 to d1, then copies d0 to d2, then copies a fixed memory address to a6, then calls another routine, then tests d0, then returns.
- `writes_null_terminated_string`: Counts the bytes of a null-terminated string starting at the address in d0, then passes the string and its length to the DOS write call on the standard output handle.
- `frag_move_bsr`: Short code fragment that copies 10 to d0, then calls another routine, then copies 13 to d0.
- `frag_movem_move_lea_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d1, then loads the address of a fixed memory address into a1, then copies d0 to the memory at a1, then copies a1 to d2, then copies 1 to d3, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_move_movea_jsr_tst_rts_2`: Short code fragment that copies a fixed memory address to d1, then copies 500 to d2, then copies a fixed memory address to a6, then calls another routine, then tests d0, then returns.
- `process_input_characters`: Processes character input from user input, storing characters in a buffer until a carriage return is encountered.
- `frag_clr_suba_move_movem_rts`: Short code fragment that clears the memory at a2 (advancing a2), then subtracts 1036 from a2, then copies a2 to a fixed memory address, then restores saved registers from the stack, then returns.
- `decrement_and_clear_byte`: Decrements a pointer in a2 by one word, then clears the byte at that address.
- `frag_move_lea_movea_jsr_clr_rts`: Short code fragment that copies 1 to d3, then copies a fixed memory address to d1, then loads the address of a fixed memory address into a1, then copies a1 to d2, then copies a fixed memory address to a6, then calls another routine, then clears d0, then copies a fixed memory address to d0, then returns.
- `frag_move_bsr_beq_rts`: Short code fragment that copies 1006 to d2, then calls another routine, then branches if the last result was zero, then copies an address to d2, then copies an address to d3, then calls another routine (2 times in a row), then returns.
- `load_game_data`: Loads multiple blocks of game data from fixed memory locations into destination buffers, with error checking after each load operation.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_2`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_3`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_4`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_5`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_6`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_7`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_8`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_9`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_10`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_11`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_12`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_13`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_14`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_15`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_16`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_17`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_18`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_19`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_20`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.

### Cortex (Europe).ipf#adf:LOADER

- `initialize_system_3`: Initializes system memory and hardware state by allocating memory blocks, clearing memory regions, setting up interrupt handlers, and preparing the execution environment before transferring control to the main game routine.
- `frag_tst_beq_jsr_movea_moveq_rts_2`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then calls another routine, then copies a fixed memory address to a1, then calls another routine, then loads 0 into d0, then copies a fixed memory address to a7, then returns.
- `combines_bitplane_rows_into_buffer_2`: Processes a block of 16-bit words by ORing five source rows together, inverting the result, and storing it in one destination row while copying the source rows to five further destination rows at fixed offsets.
- `call_super_state_2`: Wrapper that calls the exec library function SuperState and returns its result to the caller.
- `call_user_state_2`: Wrapper that calls the exec library function UserState and returns its result to the caller.
- `frag_movea_lea_moveq_jsr_move_beq_rts_2`: Short code fragment that copies a fixed memory address to a6, then loads the address of a location in the code into a1, then loads 0 into d0, then calls another routine, then copies d0 to a fixed memory address, then branches if the last result was zero, then returns.
- `scan_directory_entries_3`: Scans directory entries using DOS calls and stores the lock handle in a global variable, then performs another DOS examine operation on the same lock.
- `scan_directory_entries`: Repeatedly scans directory entries using a handle, calling two routines for each entry until no more entries are found, then closes the handle.
- `call_close_library_2`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `frag_move_movea_jsr_tst_rts_3`: Short code fragment that copies a1 to d1, then copies d0 to d2, then copies a fixed memory address to a6, then calls another routine, then tests d0, then returns.
- `writes_null_terminated_string_2`: Counts the bytes of a null-terminated string starting at the address in d0, then passes the string and its length to the DOS write call on the standard output handle.
- `frag_move_bsr_2`: Short code fragment that copies 10 to d0, then calls another routine, then copies 13 to d0.
- `writes_byte_to_output_handle`: Saves all registers on the stack, stores the low byte of d0 into a global variable, and writes that single byte to the standard output handle via the DOS write call.
- `frag_move_movea_jsr_tst_rts_4`: Short code fragment that copies a fixed memory address to d1, then copies 500 to d2, then copies a fixed memory address to a6, then calls another routine, then tests d0, then returns.
- `read_input_characters`: Reads input characters from a source, handles backspace and delete key presses by decrementing a pointer and clearing bytes, and stores valid characters until a carriage return is encountered.
- `frag_clr_suba_move_movem_rts_2`: Short code fragment that clears the memory at a2 (advancing a2), then subtracts 1036 from a2, then copies a2 to a fixed memory address, then restores saved registers from the stack, then returns.
- `clear_input_buffer`: Compares an address with a fixed value, branches if equal, writes a byte to a memory location, calls another routine with different values, clears a byte at an address, decrements the address, and then branches back.
- `frag_move_lea_movea_jsr_clr_rts_2`: Short code fragment that copies 1 to d3, then copies a fixed memory address to d1, then loads the address of a fixed memory address into a1, then copies a1 to d2, then copies a fixed memory address to a6, then calls another routine, then clears d0, then copies a fixed memory address to d0, then returns.
- `frag_move_bsr_beq_rts_2`: Short code fragment that copies 1006 to d2, then calls another routine, then branches if the last result was zero, then copies an address to d2, then copies an address to d3, then calls another routine (2 times in a row), then returns.
- `copy_data_blocks`: Copies multiple blocks of data from source to destination addresses using a series of memory copy operations with error checking, and jumps to an exit routine if any copy operation fails.
- `call_open_21`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_22`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_23`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_24`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_25`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_26`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_27`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_28`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_29`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_30`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_31`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_32`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_33`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_34`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_35`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_36`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_37`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_38`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_39`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_open_40`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close_2`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_write_2`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_read_2`: Wrapper that calls the dos library function Read and returns its result to the caller.

### Cortex (Europe).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Cortex (Europe).ipf#adf:bootblock

- `boot_entry_2`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_game`: Initializes global variables, clears memory blocks, configures display timing and DMA control registers, and sets up game state parameters including counters and flags.
- `update_display_timing`: Updates display timing counters and DMA control registers based on various global variables, copies data between memory addresses, and handles screen update operations using hardware registers.
- `initialize_game_state_2`: Initializes multiple global variables with fixed values, clears memory blocks, and sets up game state parameters including counters, flags, and pointers.
- `initialize_game_state`: Initializes multiple global variables with fixed values, clears memory blocks, and sets up game state parameters including counters, flags, and address pointers.
- `copies_data_with_offset`: Copies data from source addresses to destination addresses with fixed offsets, processing multiple rows and updating memory locations based on loop counters.
- `processes_table_entries`: Checks conditions on global variables and calls another routine to process table entries, performing conditional jumps and function calls based on the values of these variables.
- `frag_move_lea_dbra_rts`: Short code fragment that copies 7 to d0, then copies the memory at an offset from a0 to the memory at an offset from a1 (4 times in a row), then copies the memory at a0 (advancing a0) to the memory at a1, then loads the address of the memory at an offset from a1 into a1, then loops back while counting down d0, then returns.
- `update_game_state`: Updates game state variables based on input conditions and manages sprite placement logic, including boundary checks and memory updates for game objects.
- `check_collision_entries`: Reads word values from a global variable at fixed offsets, compares them with specific constants, and calls another routine when a match is found, advancing the pointer through a loop until a termination condition is met.
- `update_object_collision`: Processes object collision detection by comparing coordinates with tile data entries and updating object state when overlaps occur.
- `process_table_entries`: Iterates through a table of entries, checking conditions based on entry values and global variables, then updates global flags and calls other routines when conditions are met.
- `frag_movea_move_cmpi_beq_lea_dbra_rts`: Short code fragment that copies an address to a0, then copies 15 to d7, then compares the memory at a0 with 65535, then branches if the last result was zero, then copies 255 to the memory at a0, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d7, then returns.
- `process_entry_data`: Updates entry counters and status flags based on conditional checks, then processes table entries by copying data and modifying fields according to computed values.
- `update_entry_counter`: Increments a counter at offset 10 in a structure, checks if it reaches 6, and updates another field at offset 12.
- `increment_and_process`: Increments a counter at offset 10 in a structure, checks if it reached 8, and updates a status flag at offset 12.
- `initialize_and_process_entries`: Initializes an entry in a table to -1, then iterates through entries, processing data from a source table and updating destination entries with specific values.
- `updates_table_with_data`: Searches for an empty entry in a table, copies data from a source to the found entry, and updates various fields of that entry with specific values before calling other routines.
- `manages_table_entry_counter`: Decrements a counter in a global variable, and if it becomes negative, resets it to 2; otherwise, processes table entries by copying data from source to destination with specific updates.
- `update_object_state`: Updates object state based on current mode and handles state transitions, including decrementing counters and modifying coordinates.
- `frag_and_asl_add_movea_rts`: Short code fragment that masks d0 with 15, then shifts d0 left by 2, then adds an address to d0, then copies d0 to a2, then adds the memory at a2 (advancing a2) to d2, then adds the memory at a2 to d3, then returns.
- `frag_and_asl_add_movea_rts_2`: Short code fragment that masks d0 with 15, then shifts d0 left by 2, then adds an address to d0, then copies d0 to a2, then adds the memory at a2 (advancing a2) to d2, then adds the memory at a2 to d3, then returns.
- `update_entity_data`: Updates entity data in a table, decrementing counters and copying values from global variables, then clears a flag when a counter reaches zero.
- `clear_color_buffers`: Clears four longword locations starting at address a fixed address until reaching address a fixed address, then calls another routine and performs a series of operations involving tile data copying and color palette updates.
- `update_string_characters`: Updates characters in a string by incrementing values and handling overflow conditions, with a loop that decrements a counter until it reaches zero.
- `frag_movea_jsr_cmpa_bne_rts`: Short code fragment that copies an address to a0, then copies 49617 to a3, then calls another routine, then compares a0 with an address, then branches if the last result was not zero, then returns.
- `copy_character_data`: Copies character data from a source table to a destination address using bit manipulation and multiplication to calculate offsets, then copies 8 rows of data.
- `initialize_tile_data_2`: Clears a global variable, checks if it's zero, and if not, calls another routine with specific parameters before entering a loop that processes data elements.
- `frag_jsr_rts`: Short code fragment that calls another routine (4 times in a row), then returns.
- `initialize_tile_data`: Clears a global variable, checks if it's zero, and if not, calls another routine with specific parameters before entering a loop that processes data elements.
- `process_data_elements`: Processes sequential bytes from a buffer, calculates addresses based on input values, calls another routine with these addresses, and continues until it encounters a byte with the value 255.
- `copy_mapped_data`: Copies mapped data from a source table to a destination address using bit manipulation and multiplication to calculate offsets, then copies 8 rows of data with specific memory offsets.
- `frag_move_dbra_rts`: Short code fragment that copies 65535 to d0, then loops back while counting down d0, then returns.
- `frag_move_sub_bne_rts`: Short code fragment that copies an address to d0, then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `update_linked_list`: Updates a linked list by traversing nodes and copying data from one location to another until reaching a specific value, then returns.
- `copy_tile_data`: Copies tile data from one memory location to another in a grid pattern, handling multiple rows and columns of data.
- `update_screen_data`: Processes screen data by copying and combining bitmaps from multiple source addresses into destination addresses, handling 16 rows of data with specific memory offsets.
- `copy_screen_data`: Copies screen data from a source table to multiple destination addresses using indexed offsets, processing 16 rows of data with specific memory layout.
- `process_sprite_data`: Processes sprite transformation data by calculating screen positions, copying transformed coordinates, and performing bit rotation operations on sprite data arrays.
- `calculate_offset`: Processes two coordinate differences and a global table index to compute an offset into a table, then reads a word from that location.
- `compute_direction`: Evaluates coordinate differences using bit operations and a lookup table to determine a direction value, then returns a word from the table based on computed indices.
- `initialize_entries`: Iterates through a table of 64 entries, setting a specific field to 1 for entries that meet certain conditions.
- `process_object_data`: Iterates through object data entries, updates their state based on calculated values, and writes transformed data to memory locations determined by object properties and a global counter.
- `process_tile_data_2`: Processes tile data by performing bitwise AND operations between source and destination data, then OR operations to combine results, storing the final values.
- `process_tile_data`: Processes tile data by performing bitwise AND operations between source and destination data, then OR operations to combine results, storing the final values back to the destination structure.
- `copy_sprite_attributes`: Copies sprite attribute data from a calculated memory location to specific global variables based on input parameters.
- `initialize_memory_blocks`: Initializes multiple memory blocks with a fixed address values, first clearing 64 blocks of 4 words each, then 128 words starting at address a fixed address, and finally 16 blocks of 8 words each.
- `copy_object_data`: Copies object data from a source buffer to a destination structure, handling termination conditions and maintaining proper indexing through a loop.
- `frag_subq_bcc`: Short code fragment that subtracts 2 from a fixed memory address, then subtracts 4 from the memory at an offset from a2, then subtracts 7 from an indexed entry from a0, then branches if the carry flag is clear.
- `copy_pattern_data_3`: Copies pattern data from two source buffers to destination buffers, processing multiple rows with specific offset calculations and maintaining a counter.
- `shift_and_combine_coordinates`: Shifts the value in d0 right by one bit, combines it with values from d1 through bit operations, and returns a 32-bit result in d2.
- `copy_pattern_data`: Copies pattern data from two source buffers to destination buffers, processing multiple rows with specific offset calculations and maintaining proper data alignment.
- `update_global_counter`: Reads a 16-bit value from a global variable, adds two to it, masks the result with 65534, writes it back to the same variable, then uses this modified value as an offset to read another 16-bit value from a table and adds it to a second global variable.
- `iterate_tile_entries`: Iterates through tile data entries, performs arithmetic operations on coordinates, calculates memory addresses using bit shifts, and calls another routine to process the tile information.
- `update_object_data`: Updates object data by copying values from a source buffer to an object structure, handling termination conditions and maintaining proper indexing through a loop.
- `copy_pattern_data_2`: Copies pattern data from a source table to a destination buffer, processing multiple rows with specific offset calculations and maintaining proper data alignment.
- `frag_cmp_ori`: Short code fragment that compares d0 with d0, then merges 0 into d7 (3 times in a row).
- `copy_tile_data_2`: Copies tile data from a source table to a destination buffer, processing 32 words of data with specific addressing calculations.
- `process_tile_mapping`: Processes tile mapping data by calculating memory addresses based on loop counters and copying attribute data between multiple buffers with specific offset calculations.
- `increment_and_reset_counters`: Increments a counter and checks if it reaches zero after masking with 15; if so, increments another counter and continues with a sequence of operations including resets and calls to other routines.
- `copy_color_data`: Copies color data from a source table to multiple destination locations using indexed addressing and performs bit manipulation on the data before storing it.
- `process_joystick_input`: Reads joystick input from hardware registers, processes button states using bitwise operations, and stores the result in a global variable.
- `read_joystick_input`: Reads joystick input from hardware registers, performs bitwise operations to determine button states, and stores the result in a global variable.
- `update_color_palette`: Calculates color values based on input data, writes these values to multiple color registers, and updates a palette table with 32 color entries.
- `initialize_display_registers`: Waits for a specific interrupt signal, configures display timing and DMA control registers, clears memory areas, and sets up copper list pointers for video rendering.
- `frag_move_rts`: Short code fragment that copies 2 to a fixed memory address, then copies an address to a fixed memory address, then copies 22400 to a fixed memory address, then copies 64 to a fixed memory address, then copies 500 to a fixed memory address, then copies 33282 to a fixed memory address, then returns.
- `frag_move_dbra`: Short code fragment that copies 2 to a fixed memory address, then copies 4096 to d0, then loops back while counting down d0, then copies an address to a fixed memory address, then copies 8960 to a fixed memory address, then copies 64 to a fixed memory address, then copies 600 to a fixed memory address.

### Cortex (Europe).adf:SPACE (raw code file, load address unknown)

- `initialize_system`: Initializes system memory regions by clearing large blocks of RAM, configures interrupt and DMA control registers, and performs a series of subroutine calls with specific register values before returning.

### Cortex (Europe).ipf#adf:SPACE (raw code file, load address unknown)

- `initialize_system_2`: Initializes system memory regions by clearing large blocks of RAM, configures interrupt and DMA control registers, and performs a series of subroutine calls with specific register values before returning.
