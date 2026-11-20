# Game 4159 (king of the barbarians (19xx)(unicorns)[cr trilogy][t trilogy]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04159). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (922370 top-level bytes in 9 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 16 raw-word lines |
| Functions named and described | pass, 91 of 91 named (0 author symbols), 91 of 91 described (25 by rule, 63 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1523 of 1523 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3054 of 3055 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 7706 |
| code | 13174 |
| data | 163547 |
| padding | 1175 |
| unused | 736768 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| King of the Barbarians (19xx)(Unicorns)[cr Trilogy][t Trilogy].adf:KOTB | 2 | 177 | 6 |
| King of the Barbarians (19xx)(Unicorns)[cr Trilogy][t Trilogy].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2764 | 70 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 92 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |

## Cases handled

- runtime segment an address (play-rs-m1500): 1285 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 53 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 5 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 2 first-run image unit(s) in 1 version group(s); 10 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- king of the barbarians (19xx)(unicorns)[cr trilogy][t trilogy]/King of the Barbarians (19xx)(Unicorns)[cr Trilogy][t Trilogy].adf:KOTB: emu (own depacker stub) stream found by ancient scan (148944 -> 19984 bytes) but only 2 of 2 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 3 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 3 address
- king of the barbarians (19xx)(unicorns)[cr trilogy][t trilogy]/King of the Barbarians (19xx)(Unicorns)[cr Trilogy][t Trilogy].adf:KOTB: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### King of the Barbarians (19xx)(Unicorns)[cr Trilogy][t Trilogy].adf:KOTB

- `decode_compressed_data`: Decompresses bitstream data using a recursive decoding process, writing output bytes through a pointer while managing internal state and checksums.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `decode_bitstream`: Processes compressed bit data using recursive manipulation and decoding routines, writing output bytes through a pointer while managing internal state and control flow based on bit values.
- `bit_manipulate_recursive`: Performs bit manipulation operations on input data using right shifts and rotates, recursively calling another routine based on certain conditions.
- `decode_bits_recursive`: Performs bit manipulation on input data using right shifts and rotates, recursively calling another routine when certain conditions are met, with a loop controlling the number of iterations.

### King of the Barbarians (19xx)(Unicorns)[cr Trilogy][t Trilogy].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system`: Initializes system memory regions by clearing large blocks of memory starting at 61440 and a fixed address, sets up DMA control registers, and configures interrupt enable bits.
- `dma_control_sequence`: Manages DMA control registers by incrementing a counter, checking conditions, and updating hardware registers while processing sprite data and palette information.
- `initialize_dma_and_counters`: Initializes DMA control registers and clears memory in two address ranges, then sets up counter variables and performs conditional logic based on input states before returning.
- `initialize_game_state`: Initializes multiple global variables to zero or specific values, sets up tile data pointers based on a state variable, and performs a loop of memory clearing operations before returning.
- `copy_tile_data`: Copies tile data from two different source addresses to a destination address, using loop counters and offset calculations.
- `loop_move_copy`: Executes a loop that calls another routine 8 times while incrementing a pointer, then returns.
- `frag_move_lea_dbra_rts`: Short code fragment that copies 7 to d0, then copies the memory at an offset from a0 to the memory at an offset from a1 (4 times in a row), then copies the memory at a0 (advancing a0) to the memory at a1, then loads the address of the memory at an offset from a1 into a1, then loops back while counting down d0, then returns.
- `frag_move_lea_dbra_rts_2`: Short code fragment that copies 7 to d0, then copies the memory at an offset from a0 to the memory at an offset from a1 (4 times in a row), then copies the memory at a0 (advancing a0) to the memory at a1, then loads the address of the memory at an offset from a0 into a0, then loads the address of the memory at an offset from a1 into a1, then loops back while counting down d0, then returns.
- `check_collision`: Processes tile data entries to determine collision conditions between game objects and coordinates, updating object states and calling handlers based on comparison results.
- `process_input_state`: Updates global variables based on input flags and state transitions, handling different movement and action conditions while managing counters and status values.
- `update_global_value`: Iterates through a table until finding a matching value, then updates a global variable with a calculated result and returns it.
- `validate_and_clear`: Validates data against a global variable and clears a return value if the condition is met, otherwise returns 255 after iterating through an array.
- `check_and_increment_counter`: Checks if a global variable equals zero or has specific bits set, and if not, increments another global variable.
- `adjust_global_value`: Adjusts a global variable based on conditions involving zero checks and bit masking, returning either the modified value or zero.
- `update_player_state`: Updates player state variables based on input conditions and performs collision detection with specific boundaries, then triggers a sequence of global variable modifications and hardware register updates before returning.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_movea_move_cmpi_beq_lea_dbra_rts`: Short code fragment that copies an address to a0, then copies 15 to d7, then compares the memory at a0 with 65535, then branches if the last result was zero, then copies 255 to the memory at a0, then loads the address of the memory at an offset from a0 into a0, then loops back while counting down d7, then returns.
- `process_object_entries`: Iterates through object entries starting at address a fixed address, decrementing counters and updating entry states based on conditional logic.
- `initialize_and_process_entries`: Initializes an entry in a table to -1, then iterates through entries, processing data from a source table and updating destination entries with specific values.
- `manage_object_state`: Manages object state by decrementing a counter, initializing object properties, and calling other routines to update object coordinates and flags.
- `decrement_counter`: Decrements a word value in a global variable and branches to a label if the result is negative.
- `update_character_state`: Updates character state based on global flags and conditions, managing counters and transitions between different states while handling movement and action logic.
- `frag_move_and_asl_add_movea_rts`: Short code fragment that copies d0 to the stack, then masks d0 with 15, then shifts d0 left by 2, then adds an address to d0, then copies d0 to a2, then adds the memory at a2 (advancing a2) to d2, then adds the memory at a2 to d3, then copies the top of the stack to d0, then returns.
- `copy_tile_data_4`: Clears memory in a designated area, calculates address offsets based on input parameters, and copies tile data between multiple locations using specific addressing modes and loop counters.
- `update_string_characters`: Updates characters in a string by incrementing values and handling overflow conditions, with a loop that decrements a counter until it reaches zero.
- `frag_movea_jsr_cmpa_bne_rts`: Short code fragment that copies an address to a0, then copies 49617 to a3, then calls another routine, then compares a0 with an address, then branches if the last result was not zero, then returns.
- `copy_character_data`: Copies character data from a source table to a destination address using bit manipulation and multiplication to calculate offsets, then copies 8 rows of data.
- `process_tile_data_3`: Processes tile data by copying character values, updating coordinates, and handling state transitions based on input flags, then returns.
- `frag_jsr_rts`: Short code fragment that calls another routine (4 times in a row), then returns.
- `initialize_tile_data`: Clears a global variable, checks if it's zero, and if not, calls another routine with specific parameters before entering a loop that processes data elements.
- `iterate_mapped_data`: Processes sequential data elements from a source table, performing calculations to determine target addresses, calling another routine with these values, and continuing until encountering an end marker byte.
- `copy_mapped_data`: Copies mapped data from a source table to a destination address using bit manipulation and multiplication to calculate offsets, then copies 8 rows of data with specific memory offsets.
- `frag_move_dbra_rts`: Short code fragment that copies 65535 to d0, then loops back while counting down d0, then returns.
- `frag_move_sub_bne_rts`: Short code fragment that copies an address to d0, then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `update_pointer_table`: Updates a pointer table by reading values from a structure and writing them to another location, then advances the pointer table entry.
- `copy_tile_data_2`: Copies tile data from one memory location to another in a grid pattern, handling multiple rows and columns of data.
- `process_sprite_data`: Processes sprite data by copying and combining bitplane information for multiple sprite entries, updating destination buffers with masked and OR'd values from source data.
- `transfer_sprite_data`: Transfers sprite data between memory locations, processing a fixed number of entries and copying data with specific offsets.
- `update_sprite_data`: Processes sprite position and transformation data, updating memory locations with computed values based on input coordinates and bit shifts.
- `calculate_offset`: Processes two coordinate differences and a global table index to compute an offset into a table, then reads a word from that location.
- `frag_movem_clr_cmp_ble_or_sub_bra`: Short code fragment that saves registers on the stack, then clears d4, then compares d1 with d3, then branches if the comparison was less or equal, then merges 2 into d4, then subtracts d3 from d1, then jumps to another point in the code.
- `frag_sub_move_cmp_ble_or_bra`: Short code fragment that subtracts d1 from d3, then copies d3 to d1, then compares d0 with d2, then branches if the comparison was less or equal, then merges 1 into d4, then subtracts d2 from d0, then jumps to another point in the code.
- `compute_shift_mask`: Calculates a bit mask based on the shift amount and compares masked values from two registers, returning a word from a table based on the comparison result.
- `compute_table_index`: Adjusts a table index based on comparisons between shifted and unshifted values, then uses the index to read a word from a table.
- `initialize_entries`: Iterates through a table of 64 entries, setting a specific field to 1 for entries that meet certain conditions.
- `process_projectiles`: Iterates through a projectile list, updates their states, and copies graphic data to display addresses based on calculated offsets, while handling projectile expiration and state transitions.
- `process_tile_interactions`: Iterates through tile data to calculate positions and interactions between game elements, storing results in a global attribute table while managing counters and offsets.
- `process_tile_data`: Processes tile data by performing bitwise AND operations between source and destination data, then OR operations to combine results, storing the final values back to the destination structure.
- `copy_sprite_attributes`: Copies sprite attribute data from a calculated memory location to specific global variables based on input parameters.
- `initialize_memory_blocks`: Initializes multiple memory blocks with a fixed address values, first clearing 64 blocks of 4 words each, then 128 words starting at address a fixed address, and finally 16 blocks of 8 words each.
- `copy_animation_data`: Copies animation data from source structures to a target buffer, handling both fixed and variable-length segments while maintaining proper offsets and termination conditions.
- `frag_addq_ori`: Short code fragment that adds 2 to the memory just below a4 (moving a4 down), then merges 252 into d7, then merges 4 into d7 (2 times in a row).
- `update_tile_coordinates`: Processes coordinate data from global variables, performs bit shifts and additions to calculate tile positions, then stores the results in memory locations relative to a fixed address.
- `shift_and_combine_coordinates`: Shifts the value in d0 right by one bit, combines it with values from d1 through bit operations, and returns a 32-bit result in d2.
- `copy_tile_rows`: Copies tile row data from two source addresses to two destination buffers, processing 16 rows of 4 words each with specific offset calculations.
- `adjust_counter_value`: Reads a value from a global variable, increments it by two, masks the result with 65534, and writes it back to the same variable before adding another value from a different global variable.
- `process_tile_data_2`: Iterates through tile data entries, performs arithmetic operations on coordinates, calculates memory addresses using bit shifts, and calls another routine to process the tile information.
- `update_object_data`: Updates object data by copying values from a source buffer to an object structure, handling termination conditions and maintaining proper indexing through a loop.
- `copy_pattern_data`: Copies pattern data from a source table to a destination buffer, processing multiple rows with specific offset calculations and maintaining proper data alignment.
- `frag_cmp_ori`: Short code fragment that compares d0 with d0, then merges 0 into d7 (3 times in a row).
- `copy_tile_data_3`: Copies tile data from a source table to a destination buffer, processing 32 words of data with specific addressing calculations.
- `process_tile_mapping`: Processes tile mapping data by calculating memory addresses based on loop counters and copying attribute data between multiple buffers with specific offset calculations.
- `increment_and_reset_counters`: Increments a counter and checks if it reaches zero after masking with 15; if so, increments another counter and continues with a sequence of operations including resets and calls to other routines.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `handle_joystick_input`: Reads joystick data from a hardware register, processes bitwise operations to determine button states, and stores the result in a global variable.
- `process_joystick_input`: Reads joystick data from a hardware register, performs bitwise operations to determine button states, and stores the result in a global variable.
- `update_color_palette`: Calculates color values based on input data, writes these values to multiple color registers, and updates a palette table with 32 color entries.
- `initialize_display_registers`: Waits for a specific interrupt signal, configures display timing and DMA control registers, clears memory areas, and sets up copper list pointers for video rendering.
- `frag_move_rts`: Short code fragment that copies 2 to a fixed memory address, then copies an address to a fixed memory address, then copies 22400 to a fixed memory address, then copies 64 to a fixed memory address, then copies 500 to a fixed memory address, then copies 33282 to a fixed memory address, then returns.
- `frag_move_dbra`: Short code fragment that copies 2 to a fixed memory address, then copies 4096 to d0, then loops back while counting down d0, then copies an address to a fixed memory address, then copies 8960 to a fixed memory address, then copies 64 to a fixed memory address, then copies 600 to a fixed memory address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_input_state`: Updates input state variables including joystick and mouse button statuses, increments a counter in a global structure, and manages interrupt enable flags before returning.
- `frag_bsr_move`: Short code fragment that calls another routine, then copies an address to a fixed memory address, then copies an address to a fixed memory address.
- `update_counters`: Increments two global counters and processes data from a buffer until it encounters a zero byte, then calls another routine.
- `initialize_blitter`: Configures multiple blitter registers including address pointers, control words, and size parameters while waiting for a specific bit in a hardware status register to clear.
- `blitter_init_loop`: Increments a global counter and checks if it reaches a specific value, clearing the counter and setting up blitter registers when the condition is met.
- `frag_addq_bsr_bra`: Short code fragment that adds 1 to a fixed memory address, then calls another routine, then jumps to another point in the code.
- `process_data_block`: Processes data through multiple shift and rotate operations, writing transformed values to an output buffer while iterating through nested loops with decrementing counters.
- `frag_movea_cmpa_bne_move_bra`: Short code fragment that copies a fixed memory address to a2, then compares a2 with an address, then branches if the last result was not zero, then copies an address to a fixed memory address, then jumps to another point in the code.
- `frag_addq_lea_movea_moveq_move`: Short code fragment that adds 8 to a fixed memory address, then loads the address of a fixed memory address into a0, then copies a fixed memory address to a3, then loads the address of a fixed memory address into a5, then loads 24 into d3, then copies 256 to the memory at an offset from a5.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_jsr_tst_beq_jmp`: Short code fragment that copies 288 to a fixed memory address, then calls another routine, then tests a fixed memory address, then branches if the last result was zero, then copies 24580 to a fixed memory address, then copies 33056 to a fixed memory address, then jumps to another point in the code.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_tst_beq`: Short code fragment that tests a fixed memory address, then branches if the last result was zero.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_4`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
