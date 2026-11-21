# Game 4038 (iron trackers (1989)(microids)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04038). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903264 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 5 raw-word lines |
| Functions named and described | pass, 80 of 80 named (0 author symbols), 80 of 80 described (28 by rule, 52 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 213 of 213 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1724 of 1724 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 48346 |
| code | 7290 |
| data | 843324 |
| padding | 720 |
| unused | 3584 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Iron Trackers (1989)(Microids).adf:bootblock | 1 | 93 | 1 |
| logged disk load from Iron Trackers (1989)(Microids).adf boot-block load | 1 | 1295 | 62 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 316 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |

## Cases handled

- iron trackers (1989)(microids)/Iron Trackers (1989)(Microids).adf: custom-format (non-DOS) disk, 1750 non-empty blocks: matched against track-loaded segments
- iron trackers (1989)(microids)/Iron Trackers (1989)(Microids).adf: the boot block reads 64000 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 114 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 13 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- iron trackers (1989)(microids)/Iron Trackers (1989)(Microids).adf: 1 logged loads cover 64000 bytes of the image
- iron trackers (1989)(microids)/Iron Trackers (1989)(Microids).adf: blocks 127-935 (bytes 65024-479232) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.22)
- iron trackers (1989)(microids)/Iron Trackers (1989)(Microids).adf: blocks 939-1417 (bytes 480768-726016) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.63)
- iron trackers (1989)(microids)/Iron Trackers (1989)(Microids).adf: blocks 1420-1421 (bytes 727040-728064) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.15)
- iron trackers (1989)(microids)/Iron Trackers (1989)(Microids).adf: blocks 1424-1759 (bytes 729088-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.56)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Iron Trackers (1989)(Microids).adf:bootblock

- `load_and_relocate_hunks`: Allocates memory for hunk data structures and processes relocation information, handling different hunk types including code and data hunks.

### logged disk load from Iron Trackers (1989)(Microids).adf boot-block load

- `frag_ori_jmp`: Short code fragment that merges 243 into d0, then merges 0 into d0, then merges 1 into d0, then merges 0 into d0 (2 times in a row), then merges 117 into d0, then merges 233 into d0, then merges 117 into d0, then jumps to another point in the code.
- `frag_moveq_move_movea_jsr_tst_beq_rts`: Short code fragment that loads 2 into d1, then copies a6 to the stack, then copies a fixed memory address to a6, then calls another routine, then copies the top of the stack to a6, then tests d0, then branches if the last result was zero, then returns.
- `frag_move_asl_ori_dbra_rts`: Short code fragment that copies 15 to d0, then copies the memory at a6 to d1, then shifts d1 left by 1, then merges 17 into d1, then copies d1 to the memory at a6 (advancing a6), then loops back while counting down d0, then returns.
- `compare_and_update`: Compares values in two tables, increments corresponding entries in the second table when certain conditions are met, and updates hardware color registers with the modified values.
- `decrement_and_update_colors`: Decrements values in a table based on bit conditions, updates corresponding entries in another table, and copies the updated values to color registers.
- `initialize_joystick_ports`: Initializes joystick and mouse port hardware registers including timer settings, interrupt control, and port directions for input detection.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_input_bits`: Reads input bits from hardware, performs bitwise operations on them, compares against values from memory, and updates global variables based on the comparison results.
- `process_input_bitwise`: Processes input bits using bitwise operations to determine a result value, updates global variables based on comparison results, and modifies memory locations.
- `read_joystick_status`: Reads joystick movement and fire button data from hardware registers, processes the data through lookup tables, and updates a global variable with the result.
- `update_position_data`: Updates position data based on input bits, performs boundary checks, and clamps values within specified ranges for movement control.
- `frag_cmpi_bls_move_bhi_bra`: Short code fragment that compares the memory at a6 with 234, then branches if the comparison was lower or same (unsigned), then copies 234 to the memory at a6, then copies 243 to the memory at an offset from a6, then compares the memory at an offset from a6 with 64, then branches if the comparison was higher (unsigned), then copies 64 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_cmpi_bls_move_rts`: Short code fragment that compares the memory at an offset from a6 with 221, then branches if the comparison was lower or same (unsigned), then copies 221 to the memory at an offset from a6, then copies the memory at a6 to a fixed memory address, then copies the memory at an offset from a6 to a fixed memory address, then returns.
- `calculate_position_delta_2`: Calculates position deltas by reading mouse movement data from hardware registers, performing arithmetic operations on word values, shifting results right by one bit, and adding them to memory locations while also clamping values to specific ranges.
- `calculate_position_delta`: Calculates position deltas by performing arithmetic operations on word values, shifts results right by one bit, and adds them to memory locations.
- `update_position_coordinates`: Compares a word value with -127, negates it if greater, adds 255 to it, shifts it right by one bit, and adds the result to values in memory while updating a global variable.
- `setup_graphics_registers`: Initializes graphics hardware registers including display modes, color tables, and coprocessor settings for video output configuration.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `initialize_trackers_3`: Initializes tracker data by clearing specific global variables, copying data between memory regions, and setting up tracker states based on predefined values.
- `process_tracker_data`: Reads specific global variables and performs conditional operations based on their values, including calling other routines and updating multiple data structures.
- `copy_track_data`: Copies track data from a source table to a destination based on indexed lookups, then processes global variables and performs conditional operations including I/O requests and memory copies.
- `process_io_completion`: Handles I/O request completion by copying data from a source to a destination, then branches to another routine.
- `handle_io_request`: Checks a condition based on a global variable's value and performs an I/O operation if the condition is met, otherwise continues with other operations.
- `initialize_trackers_2`: Initializes tracker data by clearing specific global variables, copying data between memory regions, and setting up tracker states based on input conditions.
- `frag_move_clr_bra`: Short code fragment that copies 255 to a fixed memory address, then clears a fixed memory address, then jumps to another point in the code.
- `frag_cmp_bne_jsr_movea_bra`: Short code fragment that compares d4 with 4, then branches if the last result was not zero, then calls another routine (2 times in a row), then copies 16246 to a6, then calls another routine, then jumps to another point in the code.
- `update_tracker_data`: Reads a byte from a table using a counter as index, stores the byte at an offset in another table, then calls a routine to process tracker data.
- `initialize_game_objects`: Initializes game object data structures, clears memory areas, performs calculations on sprite data, and sets up various game parameters.
- `process_object_attributes`: Processes object attribute data by shifting bits, updates state variables, and modifies memory locations based on conditional checks.
- `update_sprite_data`: Updates sprite data in lookup tables, handles bit manipulation for sprite positioning, and sets flags based on calculated values.
- `frag_clr_move_mulu_add_rts`: Short code fragment that clears d0, then copies the memory at a0 to d0, then multiplies d0 by 15 (unsigned), then adds the memory at an offset from a0 to d0, then returns.
- `frag_clr_move_divu_swap_rts`: Short code fragment that clears d1, then copies d0 to d1, then divides d1 by 15 (unsigned), then copies d1 to the memory at a0, then swaps the halves of d1, then copies d1 to the memory at an offset from a0, then returns.
- `frag_lea_moveq_cmp_bne_beq_addq_bra`: Short code fragment that loads the address of a fixed memory address into a6, then loads 0 into d5, then compares d7 with the memory at an offset from a6, then branches if the last result was not zero, then compares d6 with the memory at an offset from a6, then branches if the last result was zero, then adds 2 to a6, then adds 1 to d5, then jumps to another point in the code.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `frag_tst_beq_cmpi_blt_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then compares the memory at a6 with 160, then branches if the comparison was less, then returns.
- `frag_move_bra`: Short code fragment that copies 160 to d7, then jumps to another point in the code.
- `frag_cmpi_bge_rts`: Short code fragment that compares the memory at a6 with 160, then branches if the comparison was greater or equal, then returns.
- `frag_move_cmpi_beq_add_addq_bra`: Short code fragment that copies 65376 to d7, then compares the memory at a6 with 65535, then branches if the last result was zero, then adds d7 to the memory at a6, then adds 4 to a6, then jumps to another point in the code.
- `initialize_rendering_data`: Initializes rendering data structures, clears memory areas, processes sprite data through lookup tables, and prepares graphics for display.
- `frag_tst_beq_bne_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was zero, then tests a fixed memory address, then branches if the last result was not zero, then returns.
- `update_tracker_data_2`: Processes coordinate values from two tracker structures, performs arithmetic operations on them, and updates their position data.
- `frag_sub_bmi_move_bra`: Short code fragment that subtracts d0 from d3, then subtracts d3 from d1, then branches if the last result was negative, then copies d3 to d1, then jumps to another point in the code.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `update_tracker_coordinates`: Processes coordinate data for two tracker structures by dividing input values by ten, adding sixteen, and storing the results.
- `update_track_positions`: Processes coordinate data for two track structures by multiplying and adding byte values, then performs division operations to update position components.
- `check_track_status`: Tests a global variable and branches based on its value, then compares a register with two and calls another routine if they are equal, otherwise it returns.
- `frag_cmp_bgt_subq_jsr_bra`: Short code fragment that compares d4 with 6, then branches if the comparison was greater, then subtracts 3 from d4, then calls another routine, then jumps to another point in the code.
- `frag_cmp_bne_move_jsr_bra`: Short code fragment that compares d4 with 7, then branches if the last result was not zero, then copies a fixed memory address to a fixed memory address, then calls another routine, then compares d4 with 8, then branches if the last result was not zero, then calls another routine, then jumps to another point in the code.
- `initialize_tracking_data`: Sets specific 16-bit values in memory locations, conditionally modifies a memory location based on a byte test, and calls another routine with different address pointers depending on the same byte test result.
- `initialize_trackers`: Initializes tracker-related data structures and clears specific global variables before proceeding with further operations.
- `frag_moveq_move_dbra_clr_rts`: Short code fragment that loads 17 into d0, then copies the memory at a6 (advancing a6) to the memory at a5 (advancing a5), then loops back while counting down d0, then clears the memory at a5 (advancing a5) (2 times in a row), then returns.
- `calculate_position_value`: Multiplies a value from a function call by ten, adds a constant based on a bit comparison, then performs a loop of four iterations adding values to the result depending on bits in a byte variable.
- `compare_and_loop`: Executes a loop that compares word values from two memory locations, branches conditionally based on the comparison result, and increments pointers while decrementing a counter until it reaches zero.
- `copy_and_clear`: Copies 20 bytes from a source table to a destination table, then clears a global variable.
- `process_track_data_2`: Reads bits from a buffer and writes coordinate pairs to another location, with conditional logic based on bit values.
- `process_track_data`: Processes track data by reading bits from a buffer and writing coordinate pairs to another location, with conditional logic based on bit values and loop counters.
- `format_number_string`: Takes a word value from memory, divides it by ten repeatedly, stores the resulting digits in a buffer with specific offsets, and writes constants to global variables.
- `update_tracking_data_2_2`: Loads a 16-bit value from memory, adds a constant to it, sets a register to a specific byte value, calls another routine, and then loads a memory address into a register.
- `process_tracking_data`: Reads a byte value from memory, clears the source location, extends it to word size, then uses it as an index to fetch another byte from a buffer.
- `update_tracking_data_2`: Processes tracking information by copying a byte value from a register to memory, then calling another routine to handle the data, and finally updating a counter and memory location before branching back to the beginning.
- `update_tracking_data`: Updates tracking information by copying a byte value from a register to memory, then calling another routine to process the data, and finally incrementing counters.
- `frag_adda_neg_addq_bmi_move_dbra_subi_r`: Short code fragment that adds d7 to a5, then negates d7, then adds 7 to d7, then branches if the last result was negative, then copies 37 to the memory at a5 (advancing a5), then loops back while counting down d7, then subtracts 32000 from a fixed memory address, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `frag_moveq_move_movea_jsr_tst_beq_rts_2`: Short code fragment that loads 2 into d1, then copies a6 to the stack, then copies a fixed memory address to a6, then calls another routine, then copies the top of the stack to a6, then tests d0, then branches if the last result was zero, then returns.
- `frag_move_asl_ori_dbra_rts_2`: Short code fragment that copies 15 to d0, then copies the memory at a6 to d1, then shifts d1 left by 1, then merges 17 into d1, then copies d1 to the memory at a6 (advancing a6), then loops back while counting down d0, then returns (2 times in a row).
- `update_color_components`: Clears a global variable, processes color data from memory, decrements values based on bit masks, updates hardware registers with new color values, and performs a delay loop.
- `initialize_joystick_timers`: Initializes joystick timer registers and status bytes, clears interrupt flags, and updates global variables with joystick input data.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `process_input_bits_2`: Processes input bits from hardware registers, performs bitwise operations on data, and updates global variables based on comparison results.
- `process_bitwise_operations`: Stores a byte value to a global variable, clears another global variable, performs bitwise operations on data from memory, and updates multiple global variables based on comparison results.
- `process_joystick_input`: Reads joystick data from hardware registers, performs bit operations and table lookups to determine input states, updates a global variable with the processed result, and conditionally calls two other routines based on the state of another global variable.
- `handle_movement_input`: Handles movement input by reading from hardware registers, modifying values in memory based on bit tests, and clamping values to specific ranges.
- `clamp_value_range`: Compares a byte value with 234, updates it to 234 if it's lower, stores another value to memory, and compares a byte with 64 to update it accordingly.
- `limit_and_store_values`: Compares a byte value with 221, updates it to 221 if it's lower, stores values from memory to global variables, and returns immediately.
- `handle_input_clamp_2`: Reads input from a hardware register, compares it with a stored value, calculates differences, clamps values to specific ranges, and updates memory locations based on the results.
- `handle_input_clamp`: Compares a word value with a specific constant and adjusts it if it exceeds a threshold.
- `handle_movement_input_2`: Reads input from hardware registers, modifies values in memory based on bit tests, clamps values to specific ranges, and updates memory locations with calculated results before returning to a different routine.
- `initialize_display_dma`: Sets up bitplane DMA configuration including display window and double-density settings, configures copper list pointers and color registers, then enables DMA with bitplane channels active.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_global_variables`: Calls a function multiple times to get values, stores these values to global variables, performs arithmetic operations on them, and stores the results to other global variables.
