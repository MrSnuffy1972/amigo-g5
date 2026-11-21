# Game 4697 (Piece 'n' Space): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04697). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 102 of 102 named (0 author symbols), 102 of 102 described (41 by rule, 47 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 5 of 5 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1968 of 1968 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 353633 |
| code | 8140 |
| data | 33451 |
| padding | 1128 |
| unused | 504832 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Piece 'n' Space.adf:TEXT | 2 | 14 | 1 |
| Piece 'n' Space.adf:game | 1 | 1942 | 99 |
| Piece 'n' Space.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- reference run ref ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 0)
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate+address

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Piece 'n' Space.adf:TEXT

- `open_library_and_close`: Opens a library using OldOpenLibrary and then calls two function pointers from the library before closing it with CloseLibrary.

### Piece 'n' Space.adf:game

- `wait_for_vbl_and_init`: Waits for vertical blank by polling two hardware registers until specific values are read, then initializes a global variable with a value from memory and writes a constant to another global variable.
- `initialize_game_state`: Initializes global variables and hardware registers including DMA control, interrupt settings, and coprocessor pointers.
- `frag_btst_bne_rts`: Short code fragment that tests bit 14 of the memory at an offset from a6, then branches if the last result was not zero, then returns.
- `frag_movem_move_btst_bne_jsr_rte`: Short code fragment that saves registers on the stack, then copies a fixed memory address to d0, then tests bit 4 of d0, then branches if the last result was not zero, then calls another routine (2 times in a row), then copies 112 to a fixed memory address, then restores saved registers from the stack, then returns from the exception.
- `convert_number_to_string`: Converts a signed integer into a string representation in base 10, storing the result in a global buffer and returning the original value.
- `read_mouse_movement`: Reads mouse movement data from a hardware register, processes the horizontal and vertical deltas, updates global position variables, and performs boundary checks on the resulting values.
- `frag_move_sub_neg_bra`: Short code fragment that copies 255 to d6, then subtracts d1 from d6, then negates d6, then jumps to another point in the code.
- `frag_move_sub_neg_asr_cmpi_bge_bra`: Short code fragment that copies an address to d6, then subtracts d1 from d6, then negates d6, then shifts d6 right by 1, then subtracts d6 from a fixed memory address, then compares a fixed memory address with 10, then branches if the comparison was greater or equal, then copies 10 to a fixed memory address, then jumps to another point in the code.
- `update_position_and_validate`: Updates a position value based on input coordinates and validates the resulting values against specific ranges, normalizing them using division and multiplication operations before storing the results.
- `frag_move_sub_neg_bra_2`: Short code fragment that copies 255 to d6, then subtracts d1 from d6, then negates d6, then jumps to another point in the code.
- `frag_move_sub_neg_asr_bpl_clr_bra`: Short code fragment that copies an address to d6, then subtracts d1 from d6, then negates d6, then shifts d6 right by 1, then subtracts d6 from a fixed memory address, then branches if the last result was not negative, then clears a fixed memory address, then jumps to another point in the code.
- `validate_and_normalize_coordinates`: Validates coordinate values against specific ranges and normalizes them using division and multiplication operations, then stores the results in multiple global variables.
- `update_graphics_parameters`: Calculates screen coordinates and graphics buffer addresses based on input values, then stores these parameters into a global structure.
- `calculate_and_store_graphics_data_2`: Performs identical calculations and storage operations as routine R4, calculating graphics buffer addresses and storing parameters including coordinates, colors, and pointers to a global structure.
- `calculate_coordinates`: Processes input values to calculate screen coordinates and graphics buffer addresses, storing the results into a global structure.
- `frag_move_addq_jsr_lea_moveq_bra`: Short code fragment that copies a fixed memory address to d2, then adds 1 to d2, then calls another routine, then loads the address of a fixed memory address into a0, then loads 34 into d0, then loads 23 into d1, then jumps to another point in the code.
- `frag_move_jsr_lea_moveq_bra`: Short code fragment that copies a fixed memory address to d2, then calls another routine, then loads the address of a fixed memory address into a0, then copies 128 to d0, then loads 23 into d1, then jumps to another point in the code.
- `frag_move_jsr_lea_moveq_bra_2`: Short code fragment that copies a fixed memory address to d2, then calls another routine, then loads the address of a fixed memory address into a0, then copies 99 to d0, then loads 65 into d1, then jumps to another point in the code.
- `format_number_string`: Loads a long value from a global address into register d2, then calls a conversion routine to transform a signed integer into a decimal string representation.
- `process_digit_input`: Reads a byte from memory, converts it from ASCII to numeric value, then calculates screen positions using a lookup table and calls a graphics routine.
- `iterate_graphics_data`: Iterates through graphics data using a loop counter, performs calculations on each element, and updates screen coordinates through a graphics routine call.
- `frag_lea_move_mulu_adda_moveq_dbra`: Short code fragment that loads the address of a fixed memory address into a0, then copies a fixed memory address to d0, then multiplies d0 by 25 (unsigned), then adds d0 to a0, then loads the address of a fixed memory address into a1, then loads 24 into d7, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then loops back while counting down d7.
- `calculate_graphics_addresses`: Processes data from multiple memory locations to calculate graphics buffer addresses and parameters.
- `update_display_parameters`: Checks a global flag and if positive, calls a graphics parameter update routine multiple times with different memory addresses.
- `populate_graphics_parameters`: Populates a global graphics structure with specific values including color data, buffer addresses, and coordinate offsets through multiple iterations of the same operation.
- `validate_and_place_piece`: Validates coordinates and checks if a piece can be placed at a specific location in a grid, updating the grid with piece data when valid.
- `check_and_clear_piece`: Reads a value from a grid-based table using input coordinates, checks if the value is positive, and clears matching entries in the table while updating a global variable.
- `update_piece_position`: Updates piece position coordinates based on comparison results and performs boundary checks, then clears a global buffer and updates adjacent piece states.
- `decrement_and_format_counter`: Decrements multiple counters stored in global variables and handles overflow conditions by resetting them to specific values.
- `frag_lea_move_beq_dbra_moveq_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 149 to d7, then copies the memory at a0 (advancing a0) to d0, then branches if the last result was zero, then loops back while counting down d7, then loads 1 into d7, then returns, then loads 0 into d7, then returns.
- `wait_for_flag_clear`: Waits for a specific bit in memory to become clear, then initializes several global variables with fixed values and sets a word at a specific offset.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `initialize_and_clear_data`: Copies data from a table into a global structure, clears a portion of that structure, and performs arithmetic operations on elements based on conditions from the table.
- `update_input_and_audio`: Processes input data by comparing bytes in a buffer, updates audio registers, and initializes several sound channel parameters when a condition is met.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `frag_movem_addq_move_cmp_bcs_clr_tst_be`: Short code fragment that saves registers on the stack, then adds 1 to a fixed memory address, then copies a location in the code to d0, then compares d0 with a location in the code, then branches if the carry flag is set, then clears a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_audio_registers`: Updates audio period registers for four sound channels based on data from a table, with conditional logic affecting channel 3's behavior and returning a modified value.
- `update_audio_dma`: Configures audio DMA registers with values from tables and updates a global counter, then modifies global bytes based on conditional checks.
- `update_piece_data`: Processes piece movement data based on input and updates position coordinates, then configures DMA settings and returns.
- `update_position_and_input`: Updates position data based on input processing and handles DMA configuration.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_game_state`: Updates game state variables based on input processing and handles DMA configuration.
- `update_audio_and_counters`: Updates audio DMA registers with values from tables, increments a global counter, and modifies global bytes based on conditional checks and arithmetic operations.
- `process_input_byte_2`: Reads a byte from a structure, extracts its lower nibble, uses it as a switch case to determine which routine to call for handling input, then stores a word value at an offset in another structure.
- `frag_move_rts`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `load_and_divide_value`: Loads a byte from memory, divides it by three, swaps the result, compares it to zero or two, and branches based on the comparison results.
- `frag_moveq_move_andi_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `lookup_and_compare_values`: Performs table lookups using indexed offsets, compares values, and stores results in memory when a comparison condition is met.
- `frag_move_rts_2`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `clamp_and_update_position`: Checks a global flag, updates a value in memory with 15, performs bit operations on another memory location, then clamps and stores a calculated result.
- `update_position`: Updates a position value with input data, clamps it to a range, and stores the result in a table.
- `update_input_state`: Processes input data from a structure by reading a value at offset 18, multiplying it by 74, adding that to a table address, then looping through word pairs in the table to find a match with a value at offset 0 of the structure.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_beq_clr`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies d0 to the memory at an offset from a6, then clears the memory at an offset from a6.
- `adjust_position_value`: Checks if a value at offset 24 of a structure is zero, and if not, reads a byte from offset 23 and adds it to a word at offset 16.
- `calculate_index_and_store`: Subtracts a value from a word, performs a loop to find a value in a table based on a calculated index, and stores the result into a structure.
- `combine_and_store_bits`: Processes byte values from memory using bitwise AND and OR operations to combine bits, then stores the combined result back to memory.
- `calculate_and_store_result`: Processes byte values through bit manipulation and arithmetic operations, then stores the result in a structure with additional updates to other bytes.
- `frag_tst_bpl_move_sub_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `frag_move_andi_mulu_lsr_tst_bmi_add_bra`: Short code fragment that copies an indexed entry from a4 to d2, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then multiplies d2 by d0 (unsigned), then shifts d2 right by 7, then copies the memory at an offset from a6 to d0, then tests the memory at an offset from a6, then branches if the last result was negative, then adds d2 to d0, then jumps to another point in the code.
- `subtract_and_advance_phase`: Subtracts d2 from d0 and stores the result in offset 6 of a structure in a5, then advances a phase byte in the structure by an amount taken from the high nibble of a neighbouring byte.
- `frag_bsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_5`: Short code fragment that calls another routine, then jumps to another point in the code.
- `process_and_clamp_input`: Processes byte values from memory using bitwise operations and lookup tables, performs arithmetic calculations, then clamps and stores a final result.
- `frag_tst_bpl_move_sub_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `clamp_and_update_value_2`: Calculates a value from memory using multiplication and bit shifting, adds it to another value, and stores the result after conditional checks.
- `clamp_and_update_value`: Calculates the difference between two values, clamps the result between zero and sixty-four, stores it in a structure, and updates a byte by merging a shifted nibble from a neighboring byte.
- `update_and_compare_values`: Loads a byte value, performs arithmetic operations on it, compares against another value, and updates memory locations based on the comparison result.
- `frag_move_rts_3`: Short code fragment that copies 1 to the memory at an offset from a6, then returns.
- `decrement_and_clamp_byte_2`: Reads a byte from memory, shifts its high nibble to low position, adds it to another byte, clamps the result to 64, and stores it back.
- `decrement_and_clamp_byte`: Subtracts the low nibble of the byte at offset 3 of the structure in a6 from the byte at offset 19, clears that byte to zero when the result is negative, and stores the resulting byte as a word at offset 8 of the structure in a5.
- `decrement_and_store_byte`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.
- `clamp_and_store_byte`: Reads a byte from offset 3 of a structure in a6, clamps its value to a maximum of sixty-four, and stores the result in both a byte at offset 19 and a word at offset 8 of a structure in a5.
- `decrement_and_store_byte_2`: Loads a byte value, performs bit manipulation and arithmetic operations, compares against a threshold, and stores the result in global memory locations.
- `frag_move_beq_clr_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then clears a fixed memory address, then copies d0 to a fixed memory address, then returns.
- `process_input_byte_3`: Reads a byte from a structure, extracts its upper nibble, and uses it as an index to determine which routine to call for handling input.
- `process_input_byte`: Reads a byte from offset 3 of a structure, extracts its upper nibble, and uses it as a switch case to determine which routine to call for handling input.
- `update_global_from_ciaa_pra`: Masks bit zero of the byte at offset 3 of the structure in a6, shifts it left by one, and ORs the result into a global variable after clearing bit one of that variable.
- `mask_and_merge_byte`: Masks the low nibble of a byte at offset 31 from a pointer in a6 and then merges the low nibble of a byte at offset 3 from the same pointer into it.
- `clear_low_nibbles_of_byte`: Clears the low nibble of a byte at offset 30 from an address in a6 and then ORs in the low nibble of another byte from the same base.
- `store_masked_byte`: The routine reads a byte from an offset in a structure pointed to by a register, masks it with a constant, and stores the result into a different offset in the same structure.
- `validate_and_update_counter`: Checks a global flag and another memory location, decrements a counter if it's not zero, updates memory locations with specific values, and returns.
- `frag_move_bra_4`: Short code fragment that copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_lsr_rts`: Short code fragment that copies a location in the code to d0, then shifts d0 right by 4, then copies d0 to the memory at an offset from a6, then returns.
- `mask_and_merge_nibble`: Masks the low nibble of a byte at offset 30 from a pointer in a6 and merges the high nibble from a byte at offset 3 of the same pointer, storing the result back at offset 30.
- `update_dma_parameters`: Checks if a value in a structure is divisible by a byte from the same structure, then updates a destination pointer and two word values from the structure, calls another routine twice with different bit settings in a global variable, and finally updates the destination pointer again.
- `frag_tst_bne_moveq_move_andi_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_andi_bra_2`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_moveq_move_andi_cmp_bne_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then clears the memory at an offset from a6, then copies 0 to the memory at an offset from a5, then returns.
- `frag_moveq_move_andi_cmp_bne_beq_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a fixed memory address, then branches if the last result was not zero, then copies the memory at a6 to d0, then branches if the last result was zero, then copies d1 to the stack, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_andi_addq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then tests a fixed memory address, then branches if the last result was not zero, then adds 1 to d0, then copies d0 to a fixed memory address, then returns.
- `frag_tst_bne_move_andi_lsl_or_beq`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 4, then masks the memory at an offset from a6 with 15, then merges d0 into the memory at an offset from a6, then tests d0, then branches if the last result was zero.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `frag_movem_lea_move_addi_cmp_bne_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a0, then copies the memory at a0 to d0, then adds 4 to d0, then copies the memory at a0 to d1, then compares d1 with d0, then branches if the last result was not zero, then restores saved registers from the stack, then returns.

### Piece 'n' Space.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
