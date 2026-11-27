# Game 4605 (orb (19xx)(u. anfang)(de)[cr avantgarde]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04605). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (907872 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 106 of 106 named (0 author symbols), 106 of 106 described (30 by rule, 52 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 681 of 681 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2361 of 2361 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2627 |
| code | 4878 |
| data | 351317 |
| padding | 1210 |
| unused | 547840 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Orb (19xx)(U. Anfang)(De)[cr Avantgarde].adf:pUSSy | 2 | 214 | 7 |
| Orb (19xx)(U. Anfang)(De)[cr Avantgarde].adf:ORB | 1 | 242 | 5 |
| Orb (19xx)(U. Anfang)(De)[cr Avantgarde].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 950 | 21 |
| Orb (19xx)(U. Anfang)(De)[cr Avantgarde].adf:pUSSy#unpacked | 1 | 945 | 72 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 407 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- orb (19xx)(u. anfang)(de)[cr avantgarde]/Orb (19xx)(U. Anfang)(De)[cr Avantgarde].adf:pUSSy: crunched executable decrunched by XFDMaster (Spike Cruncher (xfd), 38036 -> 96296 bytes), 29 of 29 windows confirmed in reference RAM: recovered as a hunk executable
- orb (19xx)(u. anfang)(de)[cr avantgarde]/Orb (19xx)(U. Anfang)(De)[cr Avantgarde].adf:ORB: Titanics Cruncher 1.2 (xfd) stream found by ancient scan (287960 -> 413228 bytes) but only 1 of 167 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- orb (19xx)(u. anfang)(de)[cr avantgarde]/Orb (19xx)(U. Anfang)(De)[cr Avantgarde].adf:pUSSy: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Orb (19xx)(U. Anfang)(De)[cr Avantgarde].adf:pUSSy

- `process_data_with_loop_2`: Processes data through pointer arithmetic in a loop with decrementing counters, copies bytes between memory locations, and performs conditional operations based on register values.
- `process_data_with_loop`: Performs conditional operations based on register values, executes a loop with decrementing counters, and processes data through pointer arithmetic and memory operations.
- `decrement_word_until_wrap`: Decrements a 16-bit register in a loop until it wraps past zero, then returns.
- `initialize_movement`: Initializes movement parameters by loading data from memory tables and setting up counters, then performs a series of conditional operations based on comparison results before copying bytes between memory locations.
- `frag_add_bne_move_addx_rts`: Short code fragment that adds d3 to d3, then branches if the last result was not zero, then copies the memory at a0 (advancing a0) to d3, then copies d3 to a fixed memory address, then adds d3 and the extend flag to d3, then returns.
- `initialize_game_loop`: Initializes a loop counter in d1, calls another routine, adds the carry flag to d1, decrements d4, and continues the loop while d4 is positive.
- `expand_run_length_bytes`: Copies bytes from an input to an output, expanding bytes that match a specific marker by reading a count and repeating that byte.

### Orb (19xx)(U. Anfang)(De)[cr Avantgarde].adf:ORB

- `decompress_and_initialize_data_stream`: Allocates memory and opens a library, then processes a compressed data stream by reading values from a source, performing bitwise manipulations, and writing the resulting data into a destination buffer.
- `frag_movem_move_jsr_movea_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to d1, then copies a location in the code to d2, then copies 5120 to d3, then calls another routine, then restores saved registers from the stack, then copies a location in the code to a0, then returns.
- `process_data_stream`: Processes a data stream by loading values into registers, performing bit shifts, and calling other routines based on conditional branches, while managing loop counters and memory pointers.
- `frag_moveq_lsl_bne_bsr_roxl_dbra_rts`: Short code fragment that loads 0 into d2, then shifts d0 left by 1, then branches if the last result was not zero, then calls another routine, then rotates d2 left through the extend flag by 1, then loops back while counting down d1, then returns.
- `frag_move_cmpa_bne_bsr_roxl_rts`: Short code fragment that copies the memory at a0 (advancing a0) to d0, then compares a0 with a location in the code, then branches if the last result was not zero, then calls another routine, then copies 16 to the condition codes, then rotates d0 left through the extend flag by 1, then returns.

### Orb (19xx)(U. Anfang)(De)[cr Avantgarde].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `sets_global_pointer_returns_minus_one`: Stores a constant into a global variable, removes 4 bytes from the stack, restores registers from memory, and returns the value -1 in d0.
- `setup_video_system`: Sets up video hardware registers including DMA control and copper pointers, initializes color registers, and configures interrupt handlers for the graphics system.
- `frag_movem_addi_bsr_moveq_rts`: Short code fragment that saves registers on the stack, then adds 1 to a fixed memory address (2 times in a row), then calls another routine, then restores saved registers from the stack, then loads 1 into d0, then returns.
- `copy_pixel_data`: Copies pixel data from multiple source locations to a destination buffer, processing 39 rows of 6 columns each with specific offset calculations.
- `copy_buffer_data`: Copies data from three different buffers to two output buffers, processing 40 rows of 39 bytes each with specific address increments.
- `initialize_color_registers`: Initializes multiple color registers by writing specific values to various memory locations with byte-swapped operations and incremental additions.
- `setup_interrupt_handler`: Sets up an interrupt server by configuring a structure with pointer values, then calls the system routine to register it as an interrupt handler.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `initialize_game_data`: Initializes game data structures by setting up pointers, performing comparisons, and configuring hardware registers including joystick status and audio volumes.
- `mt_end`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `manage_audio_sync`: Saves all general-purpose and address registers to the stack, increments a byte value at a fixed memory location, reads bytes from specific memory locations, compares them, and branches conditionally based on the comparison result.
- `update_audio_channels`: Updates audio channel registers based on object state data and movement codes, then processes audio period values for sound generation.
- `update_object_state`: Updates object position and state data based on input parameters and movement flags, then stores the resulting position and state information to global memory locations.
- `process_movement_code`: Processes a movement code from a structure, updates position data based on the code's value, and handles boundary checks.
- `store_position_data`: Copies a word value from a structure at offset 16 to another location at offset 6, then returns immediately without performing any other operations.
- `adjust_position_2`: Adjusts a position value based on movement parameters and boundary checks, updating the position and clearing movement flags when boundaries are exceeded.
- `adjust_position`: Adjusts a position value based on movement parameters and boundary checks, updating the position and clearing movement flags when boundaries are reached.
- `process_movement_data`: Processes movement parameters from a structure, calculates shifted values using lookup tables, and updates position data with addition or subtraction based on sign flags.
- `process_input_data`: Reads input parameters from a structure and updates position or state data based on control codes, with specific handling for joystick/mouse input and bit manipulation.
- `copy_low_nibble`: Copies the lower four bits of a byte value from offset 3 of a structure to offset 18 of the same structure.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.

### Orb (19xx)(U. Anfang)(De)[cr Avantgarde].adf:pUSSy#unpacked

- `initialize_video_system`: Initializes video hardware registers including DMA control and copper pointers, sets up color registers, and configures interrupt handlers for the graphics system.
- `frag_movem_addi_bsr_moveq_rts_2`: Short code fragment that saves registers on the stack, then adds 1 to a fixed memory address (2 times in a row), then calls another routine, then restores saved registers from the stack, then loads 1 into d0, then returns.
- `copy_pixel_rows`: Processes 6 rows of pixel data, copying 39 columns per row from a source table to a destination buffer with calculated offsets, then updates a global pointer.
- `copy_buffer_data_2`: Copies data from three different buffers to two output buffers, processing 40 rows of 39 bytes each with specific address increments.
- `initialize_color_registers_2`: Writes specific 16-bit values to multiple global variables using byte-swapped operations and incremental additions based on an initial value.
- `setup_interrupt_handler_2`: Sets up an interrupt server by configuring a structure with pointer values, then calls the system routine to register it as an interrupt handler.
- `call_rem_int_server_2`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `initialize_audio_data`: Initializes audio data structures by copying a pointer to a buffer into a global variable, then sets up a loop to clear and populate memory locations with pointers from a table based on a counter value.
- `mt_end_2`: Stops the ProTracker music replay by silencing all four audio channels and switching audio DMA off.
- `frag_movem_addq_move_cmp_bcs_clr_tst_be`: Short code fragment that saves registers on the stack, then adds 1 to a fixed memory address, then copies a location in the code to d0, then compares d0 with a location in the code, then branches if the carry flag is set, then clears a fixed memory address, then tests a fixed memory address, then branches if the last result was zero, then calls another routine, then jumps to another point in the code.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_audio_registers_2`: Updates audio period registers based on structured data and global state, with conditional logic affecting different audio channels.
- `update_orb_audio`: Processes orb state data based on input values, performs table lookups, updates position and related registers, and handles conditional operations.
- `update_orb_state`: Processes orb state data based on input values, performs table lookups, updates position and related registers, and handles conditional operations including hardware register updates.
- `update_position`: Updates position and state data based on input values and performs conditional operations including table lookups and hardware register updates, then returns.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_3`: Short code fragment that calls another routine, then jumps to another point in the code.
- `update_state_entry`: Reads a value from a table using an index derived from a structure field, stores the result in another structure field, and performs conditional updates to global variables based on the value read.
- `update_audio_registers`: Updates audio hardware registers with values from a table, modifies a global counter, and performs conditional logic based on byte values to control audio playback parameters.
- `load_and_compare_value`: Loads a word value from a structure, masks it to 12 bits, and branches based on the result.
- `frag_move_rts`: Short code fragment that copies the memory at an offset from a6 to the memory at an offset from a5, then returns.
- `calculate_division_result`: Loads a byte value from memory, divides it by 3 using signed division, swaps the word result, and compares it with zero or two to determine branch targets.
- `frag_moveq_move_and_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_move_bra`: Short code fragment that copies the memory at an offset from a6 to d2, then jumps to another point in the code.
- `lookup_and_compare`: Calculates an address using a multiplier and offset, then performs a loop to compare values at that address with another value, storing the result when a condition is met.
- `frag_move_rts_2`: Short code fragment that copies d2 to the memory at an offset from a5, then returns.
- `decrease_value_with_limit`: Decreases a value in a structure with masking and comparison against a limit, then stores the result back to another location in the structure.
- `increase_value_with_limit`: Increases a value in a structure with masking and comparison against a limit, then stores the result back to another location in the structure.
- `update_position_value`: Loads a word value from a table based on a calculated offset, compares it with a masked value from a structure, and updates a position field in the structure when certain conditions are met.
- `frag_clr_rts`: Short code fragment that clears the memory at an offset from a6, then returns.
- `frag_move_beq_clr`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then copies d0 to the memory at an offset from a6, then clears the memory at an offset from a6.
- `adjust_and_clamp_value`: Adjusts a value in a structure based on another value, clamps it to a maximum limit, and updates a global variable with the result of a table lookup operation.
- `subtract_and_clamp_offset`: Subtracts the low word of d0 from the word at offset 16 of the structure in a6, and if that value is at or below the word at offset 24 it copies offset 24 into offset 16 and clears offset 24.
- `combine_byte_values`: Combines nibble values from different locations in a structure using bitwise AND and OR operations, then stores the combined result back to the structure.
- `calculate_shifted_value`: Processes byte values from a structure to calculate a shifted and masked result, then uses conditional logic to determine the final output value based on comparison results.
- `frag_tst_bpl_move_sub_bra`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `frag_move_and_mulu_lsr_tst_bmi_add_bra`: Short code fragment that copies an indexed entry from a4 to d2, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then multiplies d2 by d0 (unsigned), then shifts d2 right by 7, then copies the memory at an offset from a6 to d0, then tests the memory at an offset from a6, then branches if the last result was negative, then adds d2 to d0, then jumps to another point in the code.
- `frag_sub_move_lsr_and_add_rts`: Short code fragment that subtracts d2 from d0, then copies d0 to the memory at an offset from a5, then copies the memory at an offset from a6 to d0, then shifts d0 right by 2, then masks d0 with 60, then adds d0 to the memory at an offset from a6, then returns.
- `frag_bsr_bra_4`: Short code fragment that calls another routine, then jumps to another point in the code.
- `frag_bsr_bra_5`: Short code fragment that calls another routine, then jumps to another point in the code.
- `calculate_accumulate`: Processes input data through bitwise operations and table lookups, performs arithmetic calculations with shift operations, and accumulates results into a destination value while applying boundary constraints.
- `frag_tst_bpl_move_sub_bra_2`: Short code fragment that tests the memory at an offset from a6, then branches if the last result was not negative, then copies 255 to d2, then subtracts d0 from d2, then jumps to another point in the code.
- `frag_move_bra_3`: Short code fragment that copies d0 to d2, then jumps to another point in the code.
- `calculate_and_accumulate`: Loads a byte value from memory, multiplies it by a shifted value, shifts the result right by 6 bits, and accumulates the result into another memory location with conditional addition.
- `clamp_and_accumulate`: Clamps a calculated difference to a maximum value, stores it, and accumulates a shifted byte value into another memory location.
- `update_counter_with_threshold`: Loads a byte value from a structure, performs conditional operations based on that value, and updates a counter with the result of arithmetic operations.
- `frag_move_rts_3`: Short code fragment that copies 1 to the memory at an offset from a6, then returns.
- `accumulate_and_limit`: Reads a byte value, shifts it right by four bits, adds it to another byte in the structure, and limits the result to a maximum value using conditional branching.
- `frag_moveq_move_and_sub_bpl_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then subtracts d0 from the memory at an offset from a6, then branches if the last result was not negative, then clears the memory at an offset from a6, then copies the memory at an offset from a6 to d0, then copies d0 to the memory at an offset from a5, then returns.
- `decrement_and_store_byte`: Reads a byte from offset 3 of the structure in a6, subtracts one from it, and stores the result in a global byte, then clears two subsequent global bytes.
- `clamp_byte_and_store`: Reads a byte from offset 3 of the structure in a6, clamps it to a maximum of sixty-four, stores the clamped value as a byte at offset 19 of that structure, and stores it as a word at offset 8 of a structure in a5.
- `update_and_store_value`: Processes a byte value by shifting and masking its bits, performs arithmetic operations, and stores the result in a global variable with conditional checks.
- `frag_move_beq_clr_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then branches if the last result was zero, then clears a fixed memory address, then copies d0 to a fixed memory address, then returns.
- `process_state_entry`: Reads a nibble from a structure byte at offset 2, uses it as an index to select and execute different code paths, then returns.
- `process_state_machine`: Reads a nibble from offset 3 of a structure, uses it as an index to select and execute different code paths, then returns.
- `frag_move_and_asl_andi_or_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with 1, then shifts d0 left by 1, then masks a fixed memory address with 253, then merges d0 into a fixed memory address, then returns.
- `replace_low_nibble_at_offset`: Clears the low nibble of the byte at offset 31 of a6 and replaces it with the low nibble of the byte at offset 3 of a6.
- `replace_low_nibble_at_offset_2`: Clears the low nibble of the byte at offset 30 of a6 and replaces it with the low nibble of the byte at offset 3 of a6.
- `store_low_nibble_at_offset`: Stores the low nibble of the byte at offset 3 of a6 into the byte at offset 18 of the same structure, then returns.
- `check_and_update_counter`: Checks a byte value at offset 34 of the structure in a6, decrements it if non-zero, and updates two global variables based on the result.
- `frag_move_bra_4`: Short code fragment that copies d0 to the memory at an offset from a6, then jumps to another point in the code.
- `frag_move_lsr_rts`: Short code fragment that copies a location in the code to d0, then shifts d0 right by 4, then copies d0 to the memory at an offset from a6, then returns.
- `frag_move_and_lsl_andi_or_rts`: Short code fragment that copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 4, then masks the memory at an offset from a6 with 15, then merges d0 into the memory at an offset from a6, then returns.
- `conditionally_write_dma_control`: Tests the low nibble of the byte at offset 3 of the structure in a6 and a global variable modulo that nibble; when the checks pass it writes the DMA control register from offset 20, delays with two countdown loops of 300 iterations, sets bit 15 of the value and writes it again, then copies longwords from offsets 4 and 14 into the structure pointed to by a5.
- `frag_tst_bne_moveq_move_and_bra`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_and_bra_2`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then jumps to another point in the code.
- `frag_moveq_move_and_cmp_bne_clr_rts`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a location in the code, then branches if the last result was not zero, then clears the memory at an offset from a6, then copies 0 to the memory at an offset from a5, then returns.
- `frag_moveq_move_and_cmp_bne_beq_bra`: Short code fragment that loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then compares d0 with a fixed memory address, then branches if the last result was not zero, then copies the memory at a6 to d0, then branches if the last result was zero, then copies d1 to the stack, then jumps to another point in the code.
- `frag_tst_bne_moveq_move_and_addq_rts`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then loads 0 into d0, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then tests a fixed memory address, then branches if the last result was not zero, then adds 1 to d0, then copies d0 to a fixed memory address, then returns.
- `frag_tst_bne_move_and_lsl_andi_or_beq`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies the memory at an offset from a6 to d0, then masks d0 with 15, then shifts d0 left by 4, then masks the memory at an offset from a6 with 15, then merges d0 into the memory at an offset from a6, then tests d0, then branches if the last result was zero.
- `process_sprite_entry`: Reads a nibble from a structure byte, uses it as an index to add a value from a table to another structure byte, clears that byte if its high bit is set, and updates pointers based on comparisons.
