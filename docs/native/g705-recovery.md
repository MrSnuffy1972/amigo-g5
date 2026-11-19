# Game 705 (F1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00705). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1002602 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 259 raw-word lines |
| Functions named and described | pass, 890 of 890 named (0 author symbols), 890 of 890 described (438 by rule, 55 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 9817 of 9817 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 23533 of 23533 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 30949 |
| code | 82346 |
| data | 680913 |
| padding | 10 |
| unused | 208384 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| F1.adf:bootblock | 1 | 122 | 7 |
| logged disk load from F1.adf boot-block load | 1 | 1336 | 33 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1076 | 39 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 7266 | 242 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 21 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 17 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12529 | 476 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 11 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 985 | 67 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 122 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 2 |

## Cases handled

- F1/F1.adf: custom-format (non-DOS) disk, 1350 non-empty blocks: matched against track-loaded segments
- 517 executed addresses below an address (an address) run the game's own code, not harness stubs (their opcode words are not the fake OS's): treated as game code (LOWCODE, 15 Nov 2026)
- F1/F1.adf: the boot block reads 22528 bytes from disk offset 788480 to an address (found statically); recovered as a unit
- runtime segment an address: 711 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3046 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5264 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk01): 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk02): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 581 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 116 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- 3 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 1 first-run image unit(s) in 1 version group(s); 4 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (2 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (6 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500-unpk01) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address (play-rs-m1500-unpk02) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (4 bytes change between runtime segment an address and runtime segment an address (play-rs-m1500-unpk02)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 8 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 5 immediate, 2 call target, 1 immediate+displacement
- F1/F1.adf: 1 logged loads cover 22528 bytes of the image
- F1/F1.adf: blocks 2-2 (bytes 1024-1536) hold ASCII text
- F1/F1.adf: blocks 1757-1759 (bytes 899584-901120) outside the filesystem hold data, not code (opcode density 0.044, entropy 3.99)
- 2 packed track region(s), 667136 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `process_input_and_audio` (runtime segment an address): Increments a global counter, performs conditional logic to update a global variable and call two other routines, calculates differences to update structure values, fetches bitwise values from tables to update structure bytes, and sets bits in a structure based on the status of hardware registers before returning via an exception return. Kind: disk hardware check (CIAB_PRB manipulation). Pass path: The check passes if _DAT_0000003a reaches zero after decrementing, allowing the CIAB_PRB bit manipulation to occur and complete successfully..
- `frag_btst_beq_bset_bsr_bra_clr_rts` (runtime segment an address): Short code fragment that tests bit 4 of a fixed memory address, then branches if the last result was zero, then sets bit 1 of a fixed memory address, then calls another routine, then jumps to another point in the code, then clears a fixed memory address, then returns. Kind: disk hardware check. Pass path: bra.w an address.
- `loop_until_zero` (runtime segment an address): Modifies a global variable and enters a loop that calls another routine until a value derived from the global variable reaches zero. Kind: disk hardware check. Pass path: The loop_until_zero routine completes and sets _DAT_0001eaa2 = uRam0100fb18, indicating successful hardware verification..
- `frag_movem_bclr_bset_move_nop_dbra_rts` (runtime segment an address): Short code fragment that saves registers on the stack, then clears bit 0 of a fixed memory address, then sets bit 0 of a fixed memory address, then copies 31 to d1, then copies a fixed memory address to d0, then pauses for one no-operation step, then loops back while counting down d0, then loops back while counting down d1, then restores saved registers from the stack, then returns. Kind: disk hardware check. Pass path: The routine toggles CIAB_PRB bit 0 (bset.b #an address, (a0)) and then checks bit 2 (an address(a0)) with btst.b. If bit 2 is set (bne.b an address), it continues to call an address which likely leads to a successful load path; otherwise, it may jump to an error o.
- `configure_dma_and_wait_for_interrupt` (runtime segment an address): Calls three routines, configures DMA controller registers with specific control and mask values, sets a DMA destination address from a pointer, and performs bit tests on a hardware register to conditionally update a DMA control register before calling another routine and returning. Kind: disk track check. Pass path: The routine waits for a specific interrupt flag (bit 4 of CIAB_ICR) to be set twice, indicating successful disk operation or synchronization. If the check passes, execution continues normally to load data; if not, it may trigger a trap or s.
- `initialize_hardware_and_verify` (runtime segment an address): Sets a counter in a global variable, configures hardware registers including DMA and custom chip registers, calls a routine via a pointer in d0, and verifies the resulting memory content against a global variable. Kind: disk hardware verification. Pass path: The routine initializes disk hardware registers (ADKCON, DSKLEN, DSKPTH, DSKSYNC) and performs a loop checking for specific values in the disk buffer after a read operation. If the check passes (i.e., the expected data is found in the buffe.

## Functions


### F1.adf:bootblock

- `initialize_hardware_and_copy_instructions`: Sets values in certain hardware registers, calls a routine, calls another routine, and then loops to copy a sequence of instructions to a destination pointed to by a register.
- `call_search_and_reorder_buffer_data`: Saves registers to the stack, calls a routine that searches for a marker and reorders pointers, restores registers, and returns.
- `search_and_reorder_buffer_data`: Searches for a specific marker in memory, then recursively calls itself to populate a stack-based array with pointers, sorts those pointers in ascending order, and finally initializes a memory structure with a bitmask and a function pointer before copying the sorted pointers into a destination buffer.
- `tiny_move`: Small routine of 1 instruction that copies (a6)+ to d1.
- `merge_masked_bit_values`: Combines a register value with a memory word under a mask: it takes the register bits selected by the mask, ORs in the masked memory bits shifted left by one, and returns the result.
- `xor_accumulate_longword_table`: Walks a table of 32-bit words a certain number of times, XORing each into an accumulator, then returns the accumulator masked by a register value.
- `frag_cmpi_bne_move_rts`: Short code fragment that compares a fixed memory address with 26616, then branches if the last result was not zero, then copies 20081 to a fixed memory address, then copies 96 to a fixed memory address, then compares a fixed memory address with 26616, then branches if the last result was not zero, then copies 20081 to a fixed memory address, then copies 96 to a fixed memory address, then returns.

### logged disk load from F1.adf boot-block load

- `parse_text_data_stream`: Iterates through a character stream, using newline characters to increment a destination pointer and mapping other characters to specific values.
- `initialize_graphics_buffer_table`: Initializes a data structure by setting various memory offsets and values, then iterates through a list of values to populate a table with calculated memory addresses and parameters.
- `set_register_bits`: Sets a bit in a register and performs a logical OR on a register with a constant.
- `decrement_and_populate_table`: Decrements a global counter and, if it reaches zero, resets it to a specific value and performs a loop to populate a table with values derived from a register.
- `set_bitmask_buffer`: Sets specific bits in a memory buffer by iterating through a table of offsets and values.
- `clear_memory_word`: Writes a zero value to a specific offset within a structure and then enters an infinite loop.
- `initialize_structure_from_string`: Initializes a structure with several constant values, then parses a string to populate a table of properties by calculating an offset from each character's value.
- `scatter_bytes_from_table`: Subtracts thirty-two from a byte in d0, shifts it left by three, and then copies a sequence of bytes from a table at a calculated offset into a destination buffer at a fixed stride of thirty-two bytes, then branches to another routine.
- `process_input_and_update_structure`: Iterates based on a value in a structure, updates a field with an incrementing value, and then performs several structure assignments and calls another routine.
- `check_interrupt_request_bit`: Saves registers to the stack, checks if bit 5 of a hardware register is set, and if not, calls another routine before returning via rte.
- `clear_global_and_set_register`: Clears a global variable, calls another routine, and writes a value to the certain hardware register before returning via rte.
- `initialize_all_subroutines`: Calls a sequence of nine different subroutines that initialize or manipulate various data tables, pointers, and hardware or software settings, then returns.
- `initialize_data_tables`: Populates two separate tables with incrementing values and clears several fields in a structure.
- `populate_function_pointer_table`: Populates a table with a specific routine address by iterating through a memory range, writing the address into consecutive double-word slots within the table, and then returns.
- `load_global_pointers`: Stores several memory addresses into a series of global variables.
- `copy_and_scale_data`: Copies data between memory locations, scales bytes from a table by a constant, and then copies the results to a new destination using a loop.
- `set_global_byte`: Sets a byte in a global variable to one and then returns.
- `setup_blitter_graphics_tables`: Copies a source table into a larger destination buffer and then configures a series of blitter operations.
- `expand_bitmask_to_buffer`: Expands bit-packed values from a source table into a destination buffer by setting bytes to either zero or maximum value, then updates a series of pointers and hardware-related parameters in a structure, waiting for a specific bit flag in a register to clear before proceeding with further pointer updates.
- `reformat_data_table`: Iterates through a table of words and performs bitwise shifts and divisions to store transformed values into a new memory location.
- `copy_data_blocks`: Copies two blocks of data from a source table to a destination table using a fixed stride.
- `initialize_audio_and_random_tables`: If a certain bitmask check passes, it initializes a sequence of random numbers into a table, sets up several pointers or table offsets, and calls step_music_channel_sequence_2 and generate_pseudo_random_sequence multiple times.
- `step_music_channel_sequence_2`: Reads the next sequence event for one channel, loading its instrument and state from data tables, and updates the channel's pitch and volume.
- `step_music_channel_sequence`: Reads the next sequence event for one channel, loading its instrument and state from data tables, and updates the channel's pitch and volume counters.
- `mask_and_store_byte`: Masks the lower 4 bits of register d1 and stores the result into a byte at an offset of 47 from the address in a6.
- `frag_tst_bne_bset_rts`: Short code fragment that tests d1, then branches if the last result was not zero, then sets bit 1 of a fixed memory address, then returns.
- `frag_bclr_rts`: Short code fragment that clears bit 1 of a fixed memory address, then returns.
- `store_negated_low_byte`: Stores the negated low byte of its input into a word field of the structure addressed by the base register.
- `frag_andi_move_rts`: Short code fragment that masks d1 with 255, then copies d1 to the memory at an offset from a0, then returns.
- `frag_move_rts_2`: Short code fragment that copies d1 to the memory at an offset from a0, then returns.
- `frag_andi_move_rts_2`: Short code fragment that masks d1 with 63, then copies d1 to the memory at an offset from a0, then returns.
- `frag_andi_move_rts_3`: Short code fragment that masks d1 with 63, then copies d1 to the memory at an offset from a6, then returns.
- `store_table_pointer`: Masks its input to six bits, multiplies it by sixteen, adds a fixed base address, and stores the result as a pointer in a structure field.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_engine_data_tables`: Initializes system parameters and populates several memory tables by clearing regions with zeros and negative ones, then performs a large-scale data copy loop that distributes values from a source table into multiple destination tables using specific word offsets.
- `initialize_system_parameters`: Writes values to specific memory locations, calls two routines, and then enters a loop that iterates 65544 times, performing various memory writes and bitwise operations using values from a table and a pointer.
- `initialize_system_tables`: Checks if a word at a0 is 5555; if so, it enters a loop that calls a routine, updates several global pointers, and processes a table to fill a destination buffer with bit-shifted values and their complements.
- `initialize_hardware_and_interrupts`: Sets several hardware registers including BPLCON0 and DIWSTRT, clears a range of color registers using a loop, writes specific values to a table at a0, and updates interrupt vectors and mouse data registers.
- `copy_structured_data_3`: Performs a nested loop to copy multiple blocks of 19 words from a source table to a destination table, using various offsets and increments to move data through memory.
- `execute_blt_transfers`: Calculates source and destination addresses for several Blitter transfers based on a global structure, waits for DMA completion, and performs multiple Blitter operations with specific widths and modes before clearing several memory locations.
- `initialize_and_populate_data_tables`: Clears several memory regions with zeros and negative ones, populates a table with incrementing values, and performs multiple loops to copy word pairs from various source tables into a destination buffer pointed to by a global variable.
- `process_nested_data_blocks`: Performs a triple-nested loop that reads pairs of longwords from a table, performs bitwise OR and swap operations on them, and writes the results to a destination buffer.
- `populate_and_update_lookup_tables`: Clears several tables and a global word, then enters a loop that copies data from one table into multiple locations within a larger memory structure.
- `initialize_global_structure`: Calls several routines and stores the long value a fixed address into a global variable at offset 34436.
- `initialize_global_structure_2`: Calls several routines and stores the long value a fixed address into a global variable at offset 34436.
- `initialize_global_word`: Calls several routines and stores the word value 32768 into a global variable at offset 34436.
- `frag_bsr_rts`: Short code fragment that calls another routine (9 times in a row), then returns.
- `frag_bsr_rts_2`: Short code fragment that calls another routine (4 times in a row), then returns.
- `frag_bsr_rts_3`: Short code fragment that calls another routine (4 times in a row), then returns.
- `frag_bsr_rts_4`: Short code fragment that calls another routine, then returns.
- `frag_bsr_rts_5`: Short code fragment that calls another routine, then returns.
- `frag_lea_move_bra`: Short code fragment that loads the address of a location in the code into a5, then copies 24 to a fixed memory address, then jumps to another point in the code.
- `frag_cmpi_ble_clr_move_lea_bra`: Short code fragment that compares a fixed memory address with 16, then branches if the comparison was less or equal, then clears a fixed memory address, then copies 32768 to a fixed memory address, then loads the address of a location in the code into a5, then jumps to another point in the code.
- `reset_counter_and_call_functions`: If a value at a global variable is greater than 8, it clears that variable and sets several other global variables to zero before calling two routines via a pointer.
- `frag_clr_lea_bra`: Short code fragment that clears a fixed memory address, then loads the address of a location in the code into a5, then jumps to another point in the code.
- `set_high_bit_and_call_functions`: Sets a global variable to a specific bitmask and populates a structure with values from other global variables before calling two routines.
- `frag_clr_lea_bra_2`: Short code fragment that clears a fixed memory address, then loads the address of a location in the code into a5, then jumps to another point in the code.
- `set_word_and_call_functions`: Sets a global variable to a specific word value and populates a structure using values from a table before calling two routines.
- `frag_clr_lea_bra_3`: Short code fragment that clears a fixed memory address, then loads the address of a location in the code into a5, then jumps to another point in the code.
- `set_word_and_call_functions_2`: Sets a global variable to a specific word value and populates a structure using values from a table before calling two routines.
- `frag_clr_lea_bra_4`: Short code fragment that clears a fixed memory address, then loads the address of a location in the code into a5, then jumps to another point in the code.
- `set_word_and_populate_structure`: Sets a global variable to the value 32768 and copies data from specific memory locations into a structure at offset 10 of a pointer, then calls a routine twice.
- `frag_clr_lea_bra_5`: Short code fragment that clears a fixed memory address, then loads the address of a location in the code into a5, then jumps to another point in the code.
- `set_word_and_populate_structure_alt`: Sets another global variable to the value 32768 and copies data from different memory locations into a structure at offset 10 of a pointer, then calls a routine twice.
- `frag_andi_lea_bra`: Short code fragment that masks a fixed memory address with an address, then loads the address of a location in the code into a5, then jumps to another point in the code.
- `call_functions_with_table_data`: Populates a structure with values retrieved from a global variable and calls two routines.
- `frag_lea_move_clr_cmpi_bge_bra`: Short code fragment that loads the address of a location in the code into a5, then copies an address to a fixed memory address, then clears a fixed memory address, then compares a fixed memory address with 16, then branches if the comparison was greater or equal, then copies 16 to a fixed memory address, then jumps to another point in the code.
- `frag_clr_lea_move_cmpi_ble_bra`: Short code fragment that clears a fixed memory address, then loads the address of a location in the code into a5, then copies an address to a fixed memory address, then compares a fixed memory address with 8, then branches if the comparison was less or equal, then clears a fixed memory address, then jumps to another point in the code.
- `set_high_bit_and_call_functions_2`: Sets a global variable to a specific bitmask and populates a structure with values from other global variables before calling two routines.
- `frag_move_lea_clr_cmpi_bge_bra`: Short code fragment that copies an address to a fixed memory address, then loads the address of a location in the code into a5, then clears a fixed memory address, then compares a fixed memory address with 8, then branches if the comparison was greater or equal, then copies 8 to a fixed memory address, then jumps to another point in the code.
- `frag_move_lea_clr_cmpi_bge_bra_2`: Short code fragment that copies an address to a fixed memory address, then loads the address of a location in the code into a5, then clears a fixed memory address, then compares a fixed memory address with 8, then branches if the comparison was greater or equal, then copies 8 to a fixed memory address, then jumps to another point in the code.
- `frag_move_lea_clr_cmpi_ble_bra`: Short code fragment that copies 4 to a fixed memory address, then loads the address of a location in the code into a5, then clears a fixed memory address, then compares a fixed memory address with 4, then branches if the comparison was less or equal, then clears a fixed memory address, then jumps to another point in the code.
- `set_bitmask_and_call_functions_alt`: Sets a global variable to a specific bitmask and calls a routine using values from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_transform_data_blocks`: Performs multiple loops to copy, XOR, and clear various data blocks from source tables into a destination buffer, updating several global pointers along the way.
- `frag_bsr_jsr_bra`: Short code fragment that calls another routine (2 times in a row), then jumps to another point in the code.
- `frag_bsr_bra_lea_move_asr_movea_jmp`: Short code fragment that calls another routine, then jumps to another point in the code, then loads the address of a location in the code into a0, then copies a location in the code to d0, then shifts d0 right by 1, then copies an indexed entry from a0 to a0, then jumps to the address in a0.
- `initialize_data_buffers_and_tables`: Iterates through a source table to populate a destination structure with three-word entries, applying a bitwise right shift and an offset of 80 to the second word of each entry if a specific condition is met.
- `initialize_and_branch_data`: Stores the value 8 in a long word at a global address, clears two 16-bit values at specific global addresses, stores 8 in another global 16-bit variable, stores 0 in a fourth global 16-bit variable, and then uses a value from a global word to perform a table lookup and branch to a routine or a specific address.
- `process_complex_data_buffers`: Performs multiple data transformations including bitwise OR operations in a loop, calls several subroutines to manipulate memory blocks, and updates various global variables and tables based on conditional checks.
- `process_small_data_and_copy_buffers`: Checks a value in a global variable and, if it is less than 17, calls two routines and executes a loop that performs arithmetic on a table of values using a constant.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `update_display_and_buffer_state`: Calls two routines to update memory and sprite tables, then checks a global value against sixteen; if the value is sixteen or less, it performs a series of arithmetic operations on a table and calls a routine to process them, otherwise it calls a routine to copy 4000 words from one memory block to another and updates several global pointers and counters.
- `update_memory_buffers_and_jump`: Iterates through a table to perform arithmetic on values and calls a routine to process them, then copies 4000 words from one memory block to another using a loop that also copies specific word offsets, and finally jumps to a destination address.
- `update_sprite_attribute_tables`: Clears and populates several attribute tables by copying data from a source table into structured memory blocks using nested loops.
- `frag_move_bra`: Short code fragment that copies 1 to the stack, then jumps to another point in the code.
- `frag_move_jsr_adda_rts`: Short code fragment that copies an address to the stack (2 times in a row), then calls another routine, then adds 10 to a7, then returns.
- `clear_multiple_tables`: Clears three separate tables of varying lengths with zeros, sets two global longwords to all ones, and clears the first eight bytes of the hardware color registers.
- `frag_move_cmp_beq_rts`: Short code fragment that copies d0 to the stack, then copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the last result was zero, then copies the top of the stack to d0, then returns.
- `frag_move_dbra_rts`: Short code fragment that copies 10 to d1, then copies 32000 to d0, then loops back while counting down d0, then loops back while counting down d1, then returns.
- `merge_and_increment_global`: Increments a global variable, calls several routines, and then performs a loop that bitwise-ORs five different memory sources into a destination table.
- `store_argument_into_global`: Stores one argument into a shared global variable, choosing the source by the sign of a third argument.
- `update_color_registers`: Reads a series of words from a table, masks and doubles each value, and writes the results to the hardware color registers in a loop.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (bset.b) and then falls through into the code that follows, with no return of its own.
- `initialize_audio_hardware`: Clears several memory locations, sets up audio period and frequency values in a table, and configures DMA and interrupt registers.
- `frag_move_rts`: Short code fragment that copies 43068 to a fixed memory address, then copies 15 to a fixed memory address, then copies 1920 to a fixed memory address (2 times in a row), then returns.
- `update_audio_hardware_3`: Updates audio hardware registers based on global variables and performs conditional configuration of audio channels, including setting period, volume, and enabling DMA transfers.
- `frag_bra_2`: Short code fragment that jumps to another point in the code.
- `frag_bra_3`: Short code fragment that jumps to another point in the code.
- `frag_bra_4`: Short code fragment that jumps to another point in the code.
- `update_audio_hardware_2`: Performs conditional updates to a global variable and a table, and if certain conditions are met, calls configure_hardware_via_table and writes values to the audio hardware registers at offset d0 multiplied by 8 from a base address.
- `check_and_clear_memory`: Loads an address into a4, checks a word at an offset from that address, and if non-zero branches to another routine; otherwise it clears two memory locations and calls another routine if a word from a constant table is non-zero.
- `frag_tst_bne_move_blt_lsl_or_jmp`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero, then copies a location in the code to d0, then branches if the comparison was less, then shifts d0 left by 4, then copies a location in the code to d1, then shifts d1 left by 2, then merges d1 into d0, then jumps to another point in the code.
- `frag_bra_5`: Short code fragment that jumps to another point in the code.
- `frag_bra_6`: Short code fragment that jumps to another point in the code.
- `frag_bra_7`: Short code fragment that jumps to another point in the code.
- `frag_bra_8`: Short code fragment that jumps to another point in the code.
- `frag_bra_9`: Short code fragment that jumps to another point in the code.
- `frag_bra_10`: Short code fragment that jumps to another point in the code.
- `frag_bra_11`: Short code fragment that jumps to another point in the code.
- `frag_bra_12`: Short code fragment that jumps to another point in the code.
- `frag_bra_13`: Short code fragment that jumps to another point in the code.
- `frag_bra_14`: Short code fragment that jumps to another point in the code.
- `frag_bra_15`: Short code fragment that jumps to another point in the code.
- `frag_bra_16`: Short code fragment that jumps to another point in the code.
- `frag_bra_move_lsl_jmp`: Short code fragment that jumps to another point in the code, then copies a location in the code to d0, then shifts d0 left by 2, then jumps to another point in the code.
- `frag_bra_17`: Short code fragment that jumps to another point in the code.
- `frag_bra_18`: Short code fragment that jumps to another point in the code.
- `frag_bra_19`: Short code fragment that jumps to another point in the code.
- `check_audio_conditions_2`: Branches to another routine, calls a routine, loads a pointer into a5, checks a 16-bit value at an offset from that pointer, and if non-zero calls another routine, then checks a second 16-bit value at a different offset and calls a third routine if non-zero before branching.
- `check_audio_conditions`: Checks a value at an offset from a pointer, calls a routine and clears a global word if the value is negative, then checks two other offsets in a structure to conditionally call different routines.
- `frag_move_bsr_clr_bra`: Short code fragment that copies a location in the code to d0, then calls another routine, then clears a fixed memory address, then jumps to another point in the code.
- `frag_move_lea_tst_bge_bsr_clr_bra`: Short code fragment that copies a location in the code to d0, then loads the address of a location in the code into a5, then tests an indexed entry from a5, then branches if the comparison was greater or equal, then calls another routine, then clears a fixed memory address, then jumps to another point in the code.
- `frag_move_bsr_bra`: Short code fragment that copies a location in the code to d6, then copies 1441 to d7, then copies 52 to d5, then copies 410 to d4, then calls another routine, then copies 1 to a fixed memory address, then jumps to another point in the code.
- `frag_bra_20`: Short code fragment that jumps to another point in the code.
- `frag_move_bsr_bra_2`: Short code fragment that copies a location in the code to d6, then copies 769 to d7, then copies 38 to d5, then copies 531 to d4, then calls another routine, then copies 2 to a fixed memory address, then jumps to another point in the code.
- `frag_move_bsr_bra_3`: Short code fragment that copies a location in the code to d0, then calls another routine, then jumps to another point in the code.
- `frag_bra_21`: Short code fragment that jumps to another point in the code.
- `frag_move_bsr_bra_4`: Short code fragment that copies a location in the code to d6, then copies 3868 to d7, then copies 58 to d5, then copies 450 to d4, then calls another routine, then copies 3 to a fixed memory address, then jumps to another point in the code.
- `update_audio_parameters_2`: Loads a 16-bit value from a table into d0, calls a routine to set DMA parameters, branches to another routine, loads a value from a table, and if it is not negative calls two different routines, clears d5, modifies d0 and d1 using shifts and an OR operation, and performs a jump to a routine using the result in d0.
- `frag_bra_22`: Short code fragment that jumps to another point in the code.
- `frag_bra_23`: Short code fragment that jumps to another point in the code.
- `frag_bra_24`: Short code fragment that jumps to another point in the code.
- `frag_bra_25`: Short code fragment that jumps to another point in the code.
- `frag_bra_26`: Short code fragment that jumps to another point in the code.
- `frag_bra_27`: Short code fragment that jumps to another point in the code.
- `frag_bra_28`: Short code fragment that jumps to another point in the code.
- `frag_bra_29`: Short code fragment that jumps to another point in the code.
- `frag_bra_30`: Short code fragment that jumps to another point in the code.
- `frag_bra_31`: Short code fragment that jumps to another point in the code.
- `frag_bra_32`: Short code fragment that jumps to another point in the code.
- `frag_bra_bsr`: Short code fragment that jumps to another point in the code, then calls another routine (2 times in a row), then jumps to another point in the code.
- `frag_clr_move_bsr_bra`: Short code fragment that clears a fixed memory address, then copies 6 to d0, then calls another routine, then jumps to another point in the code.
- `frag_move_bne_bsr_bra`: Short code fragment that copies a location in the code to d0, then branches if the last result was not zero, then copies 1 to a fixed memory address, then copies a location in the code to d6, then copies 1441 to d7, then copies 52 to d5, then copies 410 to d4, then copies 6 to d0, then calls another routine, then jumps to another point in the code.
- `frag_move_bsr_bra_5`: Short code fragment that copies 3 to a fixed memory address, then copies a location in the code to d6, then copies 3868 to d7, then copies 58 to d5, then copies 450 to d4, then copies 6 to d0, then calls another routine, then jumps to another point in the code.
- `execute_indirect_jump`: Stores the word value 2 into a global variable, loads several constants into registers, calls another routine, and performs an indirect jump using a value calculated from two constants and a result in d0.
- `frag_bra_33`: Short code fragment that jumps to another point in the code.
- `frag_bra_34`: Short code fragment that jumps to another point in the code.
- `frag_bra_35`: Short code fragment that jumps to another point in the code.
- `frag_bra_36`: Short code fragment that jumps to another point in the code.
- `frag_bra_37`: Short code fragment that jumps to another point in the code.
- `frag_bra_38`: Short code fragment that jumps to another point in the code.
- `frag_bra_39`: Short code fragment that jumps to another point in the code.
- `frag_bra_40`: Short code fragment that jumps to another point in the code.
- `frag_bra_41`: Short code fragment that jumps to another point in the code.
- `frag_bra_42`: Short code fragment that jumps to another point in the code.
- `frag_bra_43`: Short code fragment that jumps to another point in the code.
- `frag_clr_move_bsr_bra_2`: Short code fragment that clears a fixed memory address, then copies 4 to d0, then calls another routine, then jumps to another point in the code.
- `frag_move_bne_bsr_bra_2`: Short code fragment that copies a location in the code to d0, then branches if the last result was not zero, then copies 1 to a fixed memory address, then copies a location in the code to d6, then copies 1441 to d7, then copies 52 to d5, then copies 410 to d4, then copies 4 to d0, then calls another routine, then jumps to another point in the code.
- `frag_move_bsr_bra_6`: Short code fragment that copies 3 to a fixed memory address, then copies a location in the code to d6, then copies 3868 to d7, then copies 58 to d5, then copies 450 to d4, then copies 4 to d0, then calls another routine, then jumps to another point in the code.
- `frag_move_bsr_bra_7`: Short code fragment that copies 2 to a fixed memory address, then copies a location in the code to d6, then copies 769 to d7, then copies 38 to d5, then copies 531 to d4, then copies 4 to d0, then calls another routine, then jumps to another point in the code.
- `frag_bsr_move_rts`: Short code fragment that calls another routine, then copies d0 to a fixed memory address, then calls another routine, then returns.
- `frag_bsr_move_lea_clr_addq_bset_rts`: Short code fragment that calls another routine, then copies d0 to a fixed memory address, then loads the address of a location in the code into a0, then clears an indexed entry from a0, then calls another routine, then clears d1, then adds 7 to d2, then sets bit d2 of d1, then copies d1 to a fixed memory address, then returns.
- `frag_lea_tst_beq_bsr_cmp_bne_clr_move_r`: Short code fragment that loads the address of a location in the code into a5, then tests an indexed entry from a5, then branches if the last result was zero, then calls another routine, then compares d0 with a location in the code, then branches if the last result was not zero, then clears a fixed memory address, then copies 1 to an indexed entry from a5, then calls another routine, then returns.
- `update_audio_and_flags`: Loads a pointer into a5, checks a 16-bit value at an offset from that pointer, and if non-zero calls a routine, then compares d0 to a value from a table and if they are not equal clears a 32-bit global variable, sets a 16-bit value at an offset from the pointer to 1, clears a 16-bit value at an offset from a second pointer, calls a routine, and sets a bit in a 16-bit value at a global address.
- `configure_hardware_via_table`: Tests a word in a global variable and, if non-zero, performs a series of checks against a table to set a bit in a global variable, updates a global word with the value from d0, modifies a bit in a global word based on d0, and finally writes a long and a word to hardware registers at an offset determined by d0.
- `update_audio_register_and_dff_data`: Sets a bit in a global variable based on a shifted value from a register, writes that variable to a specific hardware register, and then updates a four-word block within the audio hardware registers using values from four different registers.
- `configure_dma_and_interrupts`: Checks a global variable to either return a value or update several hardware registers including the DMA controller and interrupt registers based on values in a table.
- `frag_tst_bne`: Short code fragment that tests a fixed memory address, then branches if the last result was not zero.
- `frag_move_clr_lsr_bset_lsl`: Short code fragment that copies d0 to d1, then clears d2, then shifts d1 right by 1, then sets bit d1 of d2, then copies d2 to a fixed memory address, then shifts d2 left by 7, then copies d2 to a fixed memory address (2 times in a row).
- `update_audio_structure`: Clears a word at an offset from a5, modifies a bit in a value from a table, and if a value in d0 is greater than four, it writes a longword, a word, a byte, and two longwords to an audio structure and a separate memory location.
- `configure_dma_and_interrupts_2`: Sets bits in a register based on a shifted version of d1, writes to the DMA control, interrupt enable, and interrupt request registers, and clears a word at an offset from a5.
- `update_table_values`: Loads a pointer to a table, compares values, and if not equal, updates several offsets in the table using arithmetic derived from a value in d7.
- `update_audio_period_and_registers`: Shifts a value from a table, updates several global variables, and writes values to the audio period, interrupt enable, and interrupt request registers based on various conditional checks.
- `frag_move_bra_2`: Short code fragment that copies d6 to d7, then jumps to another point in the code.
- `update_audio_frequency_and_volume`: Compares a value in a table with a stored value, and if they differ, updates the table and calculates new period and volume values using bit shifts and subtractions.
- `update_audio_parameters`: Shifts a value from a table by two bits, then uses it to fetch and write values to the audio period and volume registers or other global variables based on comparisons and a global flag.
- `frag_move_bra_3`: Short code fragment that copies d6 to d7, then jumps to another point in the code.
- `process_audio_queue`: Updates a table of four values, performs arithmetic to calculate new values for a structure, and writes a value from a table to an audio period register.
- `update_audio_period_and_volume`: Updates four values in a table, then if a value in d0 is not negative, uses it to fetch a value from a table and write it to an audio period register and calculates a volume value for the audio volume register.
- `update_audio_structure_2`: Checks a value in a structure, potentially updates a counter in a global variable, fetches a value from a table to update structure offsets, and performs arithmetic to update several 16-bit values in a structure based on a difference calculation.
- `set_audio_hardware`: Updates a structure of four values, then either writes a value from a table to the audio period register and calculates a volume value, or writes specific constants to the audio period and volume registers.
- `update_audio_hardware`: Writes a value to the audio period register, updates a value from a table into the audio frequency register, and performs a memory copy from a table to the audio hardware registers before returning from an interrupt.
- `copy_audio_parameters`: Writes a word to the audio period register, writes a word from a table to the audio frequency register, writes a word to a memory location, clears a word at a specific address, and branches to another routine.
- `frag_movea_move_lea_movem_rte`: Short code fragment that copies an address to a0, then copies a location in the code to d0, then loads the address of a location in the code into a1, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then copies the memory at a1 to the memory at a0, then copies 1 to a fixed memory address, then restores saved registers from the stack, then returns from the exception.
- `frag_lsr_lea_tst_bgt_move_movea_lsl_bra`: Short code fragment that shifts d0 right by 1, then loads the address of a location in the code into a0, then tests the memory at a0, then branches if the comparison was greater, then copies 1 to the memory at a0, then copies an address to a0, then shifts d0 left by 3, then copies a location in the code to an indexed entry from a0, then copies 50 to an indexed entry from a0, then jumps to another point in the code.
- `frag_move_lsr_lea_clr_movem_rte`: Short code fragment that copies d1 to a fixed memory address, then shifts d1 right by 7, then copies d1 to a fixed memory address, then copies 65535 to the memory at a0, then loads the address of a location in the code into a0, then clears an indexed entry from a0, then restores saved registers from the stack, then returns from the exception.
- `dispatch_interrupt_handler`: Reads the interrupt status register, performs bitwise operations to find an index, fetches a value from a table, sets a bit in d1, writes it to a hardware register, and jumps to an address found in another table.
- `update_dual_interpolated_values`: Calculates the difference between two values retrieved from a table and updates two separate 16-bit structures by applying a scaled delta based on the sign and magnitude of those differences.
- `calculate_scaled_value`: Subtracts 148 from a 16-bit value in d6, and if the result is non-negative, performs a series of arithmetic operations involving multiplication, bit rotation, and bitwise masking to calculate a new value, which is then used to subtract from 112 and returned via the stack.
- `calculate_decaying_value`: Clamps a 16-bit value in d6 between 0 and 168, then performs a loop that iteratively reduces a value in d1 by a quarter based on a counter in d3, returning the result via the stack.
- `frag_move_bge_lea_sub_blt_rts`: Short code fragment that copies the memory just below a0 (moving a0 down) to d0, then branches if the comparison was greater or equal, then loads the address of the memory at an offset from a0 into a0, then subtracts a fixed memory address from d7, then copies the memory just below a0 (moving a0 down) to d0, then branches if the comparison was less, then returns.
- `frag_move_bge_lea_add_rts`: Short code fragment that copies the memory at a1 (advancing a1) to d0, then branches if the comparison was greater or equal, then loads the address of a fixed memory address into a1, then adds a fixed memory address to d6, then copies the memory at a1 (advancing a1) to d0, then returns.
- `frag_movem_lea_move_btst_beq_and_movea`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a0, then copies a fixed memory address to d0, then tests bit 14 of d0, then branches if the last result was zero, then masks d0 with a fixed memory address, then tests bit 3 of d0, then branches if the last result was zero, then copies the memory at an offset from a0 to a0, then jumps to the address in a0.
- `frag_movem_rte`: Short code fragment that restores saved registers from the stack, then returns from the exception.
- `dispatch_interrupt_request_bits`: Reads the interrupt enable and request registers, testing the master enable bit and the request masks 64, 32 and 16 to dispatch on the first set bit through a handler call or a counter update.
- `handle_interrupt_request`: Saves registers to the stack, reads a value from a hardware register into d0, and if bit 14 is set, performs an AND operation with a hardware register and jumps to an address stored in a table; otherwise, it checks bit 7 and performs a jump or writes a value to a hardware register.
- `frag_btst_beq_move_movea_jmp`: Short code fragment that tests bit 8 of d0, then branches if the last result was zero, then copies 256 to a fixed memory address, then copies the memory at an offset from a0 to a0, then jumps to the address in a0.
- `frag_btst_beq_move_movea_jmp_2`: Short code fragment that tests bit 9 of d0, then branches if the last result was zero, then copies 512 to a fixed memory address, then copies the memory at an offset from a0 to a0, then jumps to the address in a0.
- `frag_btst_beq_move_movea_jmp_3`: Short code fragment that tests bit 10 of d0, then branches if the last result was zero, then copies 1024 to a fixed memory address, then copies the memory at an offset from a0 to a0, then jumps to the address in a0.
- `frag_movem_rte_2`: Short code fragment that restores saved registers from the stack, then returns from the exception.
- `handle_interrupt_request_2`: Saves d0 and a0 to the stack, loads a pointer into a0, reads a 16-bit value from a hardware register, checks two specific bits in that value, and if both bits are set, writes a value to a hardware register and jumps to a routine using a pointer from a table.
- `frag_movem_rte_3`: Short code fragment that restores saved registers from the stack, then returns from the exception.
- `frag_move_rts_3`: Short code fragment that copies 8 to a fixed memory address, then returns.
- `frag_clr_rts`: Short code fragment that clears a fixed memory address, then returns.
- `frag_move_rts_4`: Short code fragment that copies 4 to a fixed memory address, then returns.
- `process_state_table`: Loads a value from a global variable and, if non-negative, writes 65535 to it, fetches a pointer from a table, and jumps to it; if the value is negative, it checks another global variable and, if non-negative, calls another routine and sets that variable to negative one; otherwise it performs a series of operations involving a bitwise OR routine and returns.
- `execute_conditional_jump`: Checks a 16-bit value from a global variable; if non-negative, it sets that global variable to 65535, loads a pointer from a table, and performs an indirect jump.
- `frag_move_blt_lea_moveq_bsr_rts`: Short code fragment that copies a location in the code to d1, then branches if the comparison was less, then loads the address of a location in the code into a1, then loads 1 into d0, then copies 16000 to d3, then calls another routine, then copies an address to a fixed memory address, then returns.
- `update_and_copy_data`: Increments a global variable if its value is non-negative, then performs a series of checks on global variables to either jump to a table-based address or call a routine; finally, it checks two separate global variables and, if they are non-negative, copies 15 words from a source table to a global variable using a loop.
- `frag_move_swap_addq_add_dbra_rts`: Short code fragment that copies d1 to d2, then copies d2 to the memory at an offset from a1, then swaps the halves of d2, then copies d2 to the memory at a1, then adds 8 to a1, then adds d3 to d1, then loops back while counting down d0, then returns.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `start_copper_list_2`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `start_copper_list_3`: Points the copper at a new copper list by writing its address to COP1LC, then returns.
- `set_display_fetch_registers`: Writes the bitplane control register and the display data fetch start and stop registers with fixed values.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 41472 to the memory at an offset from a0, then copies 60 to the memory at an offset from a0, then copies 212 to the memory at an offset from a0, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `process_input_and_audio`: Increments a global counter, performs conditional logic to update a global variable and call two other routines, calculates differences to update structure values, fetches bitwise values from tables to update structure bytes, and sets bits in a structure based on the status of hardware registers before returning via an exception return.
- `process_metadata_stream_3`: Initializes several pointers and global variables, then enters a loop that processes a stream of words by performing bitwise operations, table lookups, and calling various routines to update system state and hardware parameters.
- `update_complex_structures`: Performs a series of calculations involving multiplication and bit shifts to update multiple long words in a table, copies bytes from a table to a destination using a loop, and executes several conditional branches that modify global variables and call other routines.
- `update_velocity_and_state`: Updates a velocity value in a structure and modifies related state flags based on conditions involving other values in the same structure.
- `adjust_velocity_value`: Adjusts a velocity value in a structure based on conditions involving other values in the same structure, potentially clamping or modifying the value depending on comparison results.
- `adjust_positive_velocity`: Processes a positive velocity value by applying bit shifts, clamping operations, and conditional updates to a structure field based on multiple comparison checks.
- `adjust_negative_velocity`: Processes a negative velocity value by applying arithmetic operations, conditional updates, and clamping to a structure field based on comparison checks.
- `increment_and_clamp_counter`: Increments a counter in a structure and clamps its value to a maximum limit, while also updating another field based on conditional logic.
- `reset_and_update_counters`: Resets certain counters in a structure and updates velocity values based on conditional checks and arithmetic operations involving multiple structure fields.
- `validate_event_condition`: Checks several global words and a constant against specific ranges, then sets d0 to one if all conditions are met; otherwise it returns.
- `process_structure_event`: Performs complex arithmetic and table lookups to update various values in a structure, including conditional calls to other routines.
- `frag_bra_44`: Short code fragment that jumps to another point in the code.
- `frag_bra_45`: Short code fragment that jumps to another point in the code.
- `frag_bra_46`: Short code fragment that jumps to another point in the code.
- `process_structure_event_data`: Validates a range within a structure, updates a status byte at offset 44, and performs complex arithmetic and comparisons on structure fields.
- `decrement_and_search_table`: Decrements a 16-bit value at offset 70 of a structure in a1 and, if it reaches zero, performs a loop to find the entry with the maximum byte value in a table.
- `update_scaled_structure_field`: Calculates a value based on the difference between a table entry and a value in a register, then updates a 16-bit field at offset 14 of a structure in a1 using a scaled version of a value at offset 48.
- `frag_move_bra_4`: Short code fragment that copies an indexed entry from a1 to an indexed entry from a1, then jumps to another point in the code.
- `update_scaled_value_and_clamp`: Calculates a new 16-bit value by adding a 16-bit value from a table to a scaled version of itself and a byte from an offset, stores the result in a table, and then clamps a 16-bit value from another table between 0 and 80.
- `frag_cmp_bge_move_bra`: Short code fragment that compares d3 with 65336, then branches if the comparison was greater or equal, then copies 65336 to d3, then copies d3 to an indexed entry from a1, then jumps to another point in the code.
- `update_structure_field_with_scaling`: Calculates a value based on the difference between a register and a byte at offset 59 of a structure, then updates a 16-bit field at offset 14 of the structure using a scaled version of the value at offset 48.
- `update_structure_range_and_counters_2`: Calculates a new value for a 16-bit field at offset 22 of a structure in a1 by adding a scaled version of a value at offset 48 to a base value, then enters a loop that iterates through a linked list or table to compare structure bounds and updates global variables with minimum and maximum range values.
- `update_distance_check`: Updates a distance value based on comparisons with structure fields and adjusts it if certain conditions are met.
- `frag_move_subi_bra`: Short code fragment that copies an indexed entry from a1 to d2, then subtracts 160 from d2, then jumps to another point in the code.
- `frag_move_sub_blt_cmpi_bgt_bra`: Short code fragment that copies an indexed entry from a1 to d2, then subtracts an indexed entry from a1 from d2, then branches if the comparison was less, then compares d2 with 100, then branches if the comparison was greater, then jumps to another point in the code.
- `update_structure_range_and_counters`: Calculates a range based on input values, updates a global variable with the minimum value found, and modifies several fields in a structure pointed to by a1, including incrementing or decrementing counters and adjusting values at specific offsets.
- `validate_range_parameters`: Compares a value from a0 against a table in a1 to perform various conditional checks and updates a structure field based on the result.
- `process_complex_data_structures`: Performs extensive data manipulation involving multiple loops, conditional branches, and arithmetic on various structure members and tables, including calls to other routines and updates to several global and structure-based values.
- `update_physics_and_interpolation`: Calculates physics-related values by performing division and modulo operations on structure members, updates several counters and flags within a structure, and executes a loop to interpolate values based on a global variable.
- `frag_muls_ori_move_lea_bsr_jmp`: Short code fragment that multiplies d7 by the memory just below a0 (moving a0 down) (signed), then merges 16 into d0, then merges 38 into d0, then copies 24 to a fixed memory address, then loads the address of a fixed memory address into a5, then calls another routine, then jumps to another point in the code.
- `update_structure_data_and_sort`: Iterates through a table to calculate a value based on a division involving a constant and a table entry, then stores the result and a modified version of the original entry.
- `calculate_and_update_structure_data`: Calculates a value based on a division of a register by a product of a constant and a table value, then stores this value and a related word into a structure.
- `update_structure_data`: Updates a table of structures by writing a word from a register into a table entry, then writes a byte value at an offset from another table entry.
- `modify_memory_and_branch`: Adds d0 to a location at offset d0 of a structure in a6, adds d0 to a location at offset d0 minus 4 of a structure in a6, performs a bitwise OR on d0, writes a longword to a global variable, loads a pointer into a5, calls another routine, and then performs a jump and a branch.
- `frag_add_adda_ori_move_lea_bsr_jmp`: Short code fragment that adds an indexed entry from a2 to d1, then adds the memory at a0 (advancing a0) to a1, then merges 46 into d0, then copies 24 to a fixed memory address, then loads the address of a fixed memory address into a0, then copies a0 to a fixed memory address, then loads the address of a fixed memory address into a5, then calls another routine, then jumps to another point in the code.
- `execute_indirect_calls`: Calls two routines using a pointer from a table, then calls frag_jsr_rts and performs an indirect jump through a table using a value from a global variable.
- `frag_jsr_rts`: Short code fragment that calls another routine (3 times in a row), then returns.
- `update_game_state_and_graphics`: Updates a structure by performing various conditional checks, modifies a memory block through a loop, and updates a global variable based on a comparison of values in a table.
- `update_structure_state`: Clears two words in a structure at a5, then uses d0 to call another routine multiple times with different d5 values or modifies a structure pointed to by a value loaded from the program counter.
- `process_indexed_state`: Clears two words in a structure at a5, then performs conditional calls to another routine based on d0 and modifies a structure field at offset 244 of a5 or increments a value at offset 230 of a structure pointed to by the program counter.
- `frag_move_subi_bra_2`: Short code fragment that copies 55798 to the memory at an offset from a5, then subtracts 10 from the memory at an offset from a5, then jumps to another point in the code.
- `reset_structure_fields_2`: Stores a routine pointer in a structure, subtracts one hundred from a word at offset one hundred eighty, clears several fields in a structure, and clears a global longword if a specific word in the structure is zero.
- `update_structure_state_3_2`: Checks a 16-bit value at offset 62 of a structure; if it is not 35, it performs calculations involving table lookups and pointer arithmetic to update values at offsets 250 and 258, then calls two other routines and increments the value at offset 62.
- `update_structure_state_3`: Clears a 16-bit value at offset 218 of the structure in a5, conditionally negates a value at offset 214 using a value from a table, and may write several values to the structure based on a bit test and comparisons.
- `frag_tst_bne_bclr_clr_move_rts`: Short code fragment that tests the memory at an offset from a5, then branches if the last result was not zero, then clears bit 0 of the memory at an offset from a5, then clears the memory at an offset from a5, then copies 65535 to the memory at an offset from a5 (2 times in a row), then returns.
- `update_structure_values_2`: Updates a structure in a5 by modifying several word and long values at specific offsets, potentially setting a function pointer at offset 70, and updates a value in a table using a pointer and index from a1 and d1.
- `frag_movea_move_bclr_bsr_rts`: Short code fragment that copies a location in the code to a5, then copies 65535 to the memory at an offset from a5, then clears bit 0 of the memory at an offset from a5, then calls another routine, then returns.
- `calculate_and_store_offset`: Calculates a 32-bit value using a word from a5, a constant, and a byte from a5, then stores this value into a structure at an offset determined by a pointer in a global variable and calls another routine.
- `reset_and_update_structure_state`: Checks two conditions in a structure pointed to by a5; if met, it sets a function pointer at offset 70, modifies several numeric values and counters at specific offsets, and updates a value in a table using an index and a calculated offset.
- `calculate_table_offset`: Increments a word at offset 62 of a5, calculates an offset based on that value and a byte at offset 27 of a new address, and stores a long value at an offset in a table before calling another routine.
- `frag_tst_blt_movea_subi_bpl_clr_rts`: Short code fragment that tests the memory at an offset from a5, then branches if the comparison was less, then copies a location in the code to a4, then subtracts 30 from the memory at an offset from a4, then branches if the last result was not negative, then clears the memory at an offset from a4, then returns.
- `frag_move_bclr_rts`: Short code fragment that copies 65535 to the memory at an offset from a5, then clears bit 0 of the memory at an offset from a5, then returns.
- `frag_move_tst_bpl_blt_cmpi_bra`: Short code fragment that copies an indexed entry from a1 to d5, then tests the memory at an offset from a5, then branches if the last result was not negative, then tests the memory at an offset from a5, then branches if the comparison was less, then compares the memory at an offset from a5 with 65348, then branches if the comparison was less, then jumps to another point in the code.
- `update_structure_values_3`: Tests values at offsets 98 and 116 of a5 and a value at offset 528; if certain conditions are met, it branches to another routine, otherwise it performs arithmetic on d2, subtracts 10 from a word at offset 180 of a5, clears words at offsets 116 and 118 of a5, and stores a shifted d5 value into offset 120 of a5 before returning.
- `update_structure_state_2`: Checks a 16-bit value at offset 216 of a structure in a5; if it is zero, it modifies several fields in the structure, including subtracting 90 from offset 180 and setting bit 1 of offset 242, and calls another routine twice using values derived from a table and a pointer.
- `frag_move_addi_subi_bpl_clr_rts`: Short code fragment that copies d3 to d2, then adds 6 to d2, then subtracts 60 from the memory at an offset from a5, then branches if the last result was not negative, then clears the memory at an offset from a5, then returns.
- `frag_eori_beq_addi_rts_subi`: Short code fragment that exclusive-ors 4 into a fixed memory address, then branches if the last result was zero, then adds 32000 to a fixed memory address (2 times in a row), then returns, then subtracts 32000 from a fixed memory address (2 times in a row), then returns.
- `frag_movea_cmpi_beq_btst_bne_clr_rts_mo`: Short code fragment that copies a location in the code to a3, then compares the memory at an offset from a3 with 1, then branches if the last result was zero, then tests bit 7 of the memory at an offset from a3, then branches if the last result was not zero, then clears d0, then returns, then copies 1 to d0, then returns.
- `process_input_data`: Checks bit 3 of a word pointed to by a0; if clear, it updates a global variable and a value at a specific address using bitwise operations, otherwise it performs several assignments to a global variable.
- `subtract_scaled_value`: Calculates a difference between a value in a3 and a global variable, uses a table to find a multiplier, subtracts a scaled product from a 16-bit value at offset -8 of a0, and writes a byte to offset -2 of a0 if a byte at offset -1 is zero.
- `subtract_scaled_value_with_bounds`: Calculates a difference between a global variable and a value in a3, clamps the result, uses a table to find a multiplier, subtracts a scaled product from a 16-bit value at offset -8 of a0, and writes a byte to offset -2 of a0 if a byte at offset -1 is zero.
- `frag_tst_bne_move_sub_movem_rts`: Short code fragment that tests the memory at an offset from a0, then branches if the last result was not zero, then copies 254 to the memory at an offset from a0, then subtracts d2 from the memory at an offset from a0, then restores saved registers from the stack, then returns.
- `add_scaled_value`: Calculates a difference between a value in a3 and a global variable, clamps the result, uses a table to find a multiplier, adds a scaled product to a 16-bit value at offset -6 of a0, and writes a byte to offset -2 of a0 if a byte at offset -1 is zero.
- `add_scaled_value_with_bounds`: Calculates a difference between a global variable and a value in a3, clamps the result, uses a table to find a multiplier, adds a scaled product to a 16-bit value at offset -6 of a0, and writes a byte to offset -2 of a0 if a byte at offset -1 is zero.
- `frag_tst_bne_move_add_movem_rts`: Short code fragment that tests the memory at an offset from a0, then branches if the last result was not zero, then copies 255 to the memory at an offset from a0, then adds d2 to the memory at an offset from a0, then restores saved registers from the stack, then returns.
- `add_scaled_value_fixed_base`: Calculates a difference between a value in a3 and a global variable, clamps the result, uses a table to find a multiplier, adds a scaled product to a 16-bit value at offset -8 of a0, and writes a byte to offset -2 of a0.
- `add_scaled_value_with_bounds_and_flag`: Calculates a difference between a global variable and a value in a3, clamps the result, uses a table to find a multiplier, adds a scaled product to a 16-bit value at offset -8 of a0, and writes a byte to offset -2 of a0.
- `frag_add_move_movem_rts`: Short code fragment that adds d2 to the memory at an offset from a0, then copies 1 to the memory at an offset from a0, then restores saved registers from the stack, then returns.
- `populate_lookup_tables`: Generates several lookup tables by performing arithmetic operations on values from a source table and storing the results into a destination buffer.
- `process_metadata_stream_2`: Reads a stream of metadata from a pointer in a0, using a 16-bit word to select operations via a table and updating various global state variables.
- `frag_bra_47`: Short code fragment that jumps to another point in the code.
- `frag_bra_48`: Short code fragment that jumps to another point in the code.
- `frag_bra_49`: Short code fragment that jumps to another point in the code.
- `frag_bra_50`: Short code fragment that jumps to another point in the code.
- `frag_move_andi_bra`: Short code fragment that copies the memory at a0 to d1, then masks d1 with 65532, then copies a location in the code to a fixed memory address (2 times in a row), then copies d0 to the memory at an offset from a5, then copies 1 to the memory at an offset from a5, then copies d1 to the memory at an offset from a5, then jumps to another point in the code.
- `frag_move_lsr_andi_bra`: Short code fragment that copies the memory at a0 to d1, then shifts d1 right by 2, then masks d1 with 3, then copies the memory at a0 to the memory at an offset from a5, then copies a location in the code to the memory at an offset from a5, then jumps to another point in the code.
- `adjust_coordinate_by_scaled_offset`: Reads a 16-bit value from a pointer in a0 and uses its lower bits to index into a table to determine a scale factor and sign.
- `frag_addq_move_bra_add`: Short code fragment that adds 8 to a0, then copies a0 to the memory at an offset from a5, then jumps to another point in the code, then adds d3 to the memory at an offset from a5, then jumps to another point in the code.
- `update_interpolated_delta_values`: Calls update_dual_interpolated_values, then loads a pointer from offset 0 of a structure into a register and writes a value from offset 16 of the structure to a global variable.
- `update_physics_and_clamping`: Updates a 16-bit value at offset 48 of a structure by subtracting a value, clamps it within specific bounds, and performs complex scaling and interpolation calculations using lookup tables to update various fields in the structure, including offsets 30, 36, 52, 54, and 56.
- `frag_bra_51`: Short code fragment that jumps to another point in the code.
- `frag_bra_52`: Short code fragment that jumps to another point in the code.
- `frag_bra_tst_bge_movem_movea_clr_jsr`: Short code fragment that jumps to another point in the code, then tests the memory at an offset from a5, then branches if the comparison was greater or equal, then saves registers on the stack, then copies a0 to a4, then clears d0, then copies a location in the code to a1, then calls another routine, then restores saved registers from the stack, then jumps to another point in the code.
- `load_values_to_memory`: Loads a word from a location pointed to by a0 into a global variable, clears the lowest two bits of that global variable, loads two longwords from PC-relative addresses into other global variables, writes d0 to a global variable, and writes three words from PC-relative addresses into offsets 32, 30, and 2e of a structure in a5 before branching.
- `frag_move_lsr_andi_tst_bne_muls_asr_add`: Short code fragment that copies the memory at a0 to d1, then shifts d1 right by 2, then masks d1 with 3, then copies d0 to d6, then tests the memory at an offset from a5, then branches if the last result was not zero, then multiplies d6 by a location in the code (signed), then shifts d6 right by 8, then adds d6 to the memory at an offset from a5, then jumps to another point in the code.
- `update_scaled_offset_and_delta_2`: Calculates a scaled offset by subtracting a byte at offset 43 of the structure in a5 from 7, shifts a negated word from d6 by this result, adds it to a longword at offset 36 of the structure, calls another routine, and then updates a longword at offset 30 of the structure using the difference between two longwords.
- `update_scaled_offset_and_delta`: Calculates a scaled offset by shifting a value derived from subtracting a byte at offset 43 of a structure from 7, adds this to a longword at offset 36 of the structure, and calls another routine.
- `apply_scaled_delta_to_offset`: Calculates a scaled delta by negating a value, subtracting a byte from offset 43 of a structure, and shifting the result left by a variable amount determined by that byte.
- `update_scaled_offset_value`: Calculates a scaled value by subtracting a byte at offset 43 of a structure from 7, shifting the result left by one bit, and subtracting this from a longword at offset 36 of the structure.
- `update_scaled_offset_and_delta_3`: Calculates a scaled delta value by shifting a 16-bit integer based on a stored scale factor and updates a structure's offset and delta fields.
- `calculate_table_sum`: Fetches a value from offset 36 of a structure in a5, masks and shifts it, uses it as an index into a table starting at a constant address, performs a series of additions and subtractions using 16-bit values from that table, and stores the final result in a global variable.
- `frag_lea_clr_move_lsl`: Short code fragment that loads the address of a location in the code into a4, then clears d4, then copies the memory at an offset from a5 to d4, then shifts d4 left by 1, then copies an indexed entry from a4 to the memory at an offset from a4.
- `frag_move_andi_asr_movea_ext_nop_rts`: Short code fragment that copies the memory at an offset from a5 to d4, then masks d4 with 65472, then shifts d4 right by 4, then copies an address to a4, then copies an indexed entry from a4 to d4, then sign-extends d4, then pauses for one no-operation step, then returns.
- `frag_asr_nop_add_asl`: Short code fragment that shifts d4 right by 5, then shifts d4 right by 4, then shifts d4 right by 3, then shifts d4 right by 2, then shifts d4 right by 1, then pauses for one no-operation step, then adds d4 to d4, then shifts d4 left by 2.
- `frag_move_cmp_ble_rts_bge`: Short code fragment that copies the memory at an offset from a5 to d5, then compares d5 with an address, then branches if the comparison was less or equal, then copies an address to the memory at an offset from a5, then returns, then compares d5 with an address, then branches if the comparison was greater or equal, then copies an address to the memory at an offset from a5, then returns.
- `clamp_and_update_values`: Clamps a long value at offset 36 of a structure between two bounds, updates a 16-bit value at offset 54 and another at offset 56 based on arithmetic involving a third value, modifies a long value at offset 14, and performs complex table manipulations and updates several other structure offsets including 8, 30, and 162.
- `clamp_and_shift_value`: Compares a word in d1 to a word at offset 166 of a5; if the value is smaller, it increments a word at offset 166 of a5 by one, otherwise if it is greater than a word at offset 168 of a5, it decrements that word by one.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `update_structure_fields`: Performs arithmetic operations involving a value from a table, a constant, and registers, then either writes a result to an offset in a structure or updates three different 16-bit values at specific offsets within that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_fixed_point_interpolation`: Performs fixed-point arithmetic and interpolation using values from a table and a structure, updating two 16-bit values at specific offsets within that structure.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_interpolated_coordinate_table`: Calculates a new value based on a scaled difference and stores it into a table starting at a pointer in a0, then enters a loop that updates subsequent entries in the table using a pointer and a switch statement.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_fixed_point_accumulator`: Performs fixed-point arithmetic using 32-bit values to update a counter in a structure, involving signed multiplication, division, and a loop that accumulates values into a memory buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `transform_and_validate_data_blocks_2`: Calculates a value by multiplying a 16-bit word from a source pointer by the sum of two 16-bit registers, subtracts a constant, and compares the result to zero.
- `frag_bra_53`: Short code fragment that jumps to another point in the code.
- `frag_subq_ble_addq_dbra_bra_move_cmp_bg`: Short code fragment that subtracts 2 from the memory at an offset from a4, then branches if the comparison was less or equal, then adds 4 to a3, then loops back while counting down d2, then jumps to another point in the code, then copies a location in the code to d7, then compares d7 with the memory at a3, then branches if the comparison was greater or equal, then jumps to another point in the code.
- `frag_bra_54`: Short code fragment that jumps to another point in the code.
- `transform_and_validate_data_blocks_3`: Iterates through a series of 16-bit words, performing a calculation involving multiplication, bit shifting, and subtraction for each element, then updates a value in a structure and calls a jump table via an indirect jump.
- `frag_bra_55`: Short code fragment that jumps to another point in the code.
- `process_scaled_data_block`: Iterates through a list of 16-bit words, performing a fixed-point multiplication and scaling operation on each element before comparing the result to a threshold.
- `frag_bra_56`: Short code fragment that jumps to another point in the code.
- `interpolate_and_rescale_table_entry`: Performs linear interpolation between two values in a table using a fractional index, then scales the result and calls rescale_and_interpolate_data.
- `frag_move_mulu_swap_neg_subq_sub_dbra_a`: Short code fragment that copies d7 to d6, then multiplies d6 by the memory at an offset from a3 (unsigned), then swaps the halves of d6, then swaps the halves of d7, then negates d7, then subtracts 1 from d7, then subtracts the memory at an offset from a3 from d6, then loops back while counting down d7, then adds a location in the code to d6, then jumps to another point in the code.
- `frag_sub_add_bra`: Short code fragment that subtracts the memory at an offset from a3 from d6, then adds a location in the code to d6, then jumps to another point in the code.
- `frag_bra_57`: Short code fragment that jumps to another point in the code.
- `interpolate_table_data`: Calculates a value by performing a weighted average between two entries in a table using a scale factor, then calls rescale_and_interpolate_data.
- `scale_and_subtract_data`: Multiplies a 16-bit value from a table by a value in a register, performs a series of swaps and subtractions, and executes a loop that decrements a counter until it reaches zero before calling rescale_and_interpolate_data.
- `frag_sub_add_bra_2`: Short code fragment that subtracts the memory at an offset from a3 from d6, then adds a location in the code to d6, then jumps to another point in the code.
- `frag_bra_58`: Short code fragment that jumps to another point in the code.
- `interpolate_rescaled_values`: Calculates a rescaled value using a scale factor and an offset, then enters a loop that performs linear interpolation between source values to populate a destination buffer.
- `rescale_and_interpolate_data`: Performs a loop to rescale a series of 16-bit words by calculating a scale factor using division and then interpolates values into a destination buffer using a combination of arithmetic operations and a call to another routine.
- `transform_and_validate_data_blocks`: Performs a loop to transform a series of 16-bit words by adding values from two different offsets and subtracting a third, then compares these values against a table to set a bitmask in a global variable before returning.
- `frag_move_movem_jmp_andi_or`: Short code fragment that copies a location in the code to the memory at a0 (advancing a0), then saves registers on the stack, then copies a location in the code to d5, then jumps to another point in the code, then copies the memory at a0 to d2, then masks d2 with 12, then merges a location in the code into d2, then jumps to another point in the code.
- `frag_bra_59`: Short code fragment that jumps to another point in the code.
- `frag_bra_60`: Short code fragment that jumps to another point in the code.
- `frag_bra_61`: Short code fragment that jumps to another point in the code.
- `frag_bra_62`: Short code fragment that jumps to another point in the code.
- `frag_bra_63`: Short code fragment that jumps to another point in the code.
- `frag_bra_64`: Short code fragment that jumps to another point in the code.
- `frag_bra_65`: Short code fragment that jumps to another point in the code.
- `frag_bra_66`: Short code fragment that jumps to another point in the code.
- `frag_bra_67`: Short code fragment that jumps to another point in the code.
- `frag_bra_68`: Short code fragment that jumps to another point in the code.
- `frag_bra_69`: Short code fragment that jumps to another point in the code.
- `frag_bra_70`: Short code fragment that jumps to another point in the code.
- `update_render_state`: Updates rendering parameters based on input values and stores indexed data.
- `calculate_and_store_scaled_index`: Performs a signed comparison and a scaled division using a constant, then checks the result against two thresholds before storing the value.
- `scale_and_store_index`: Performs scaled arithmetic on a value from register d4 and stores the result in a global variable at offset -8010 relative to a4.
- `dispatch_indexed_table_data`: Calculates a scaled index using multiplication and bit shifting, updates a structure with values from a table, and calls routines to clamp an index, perform an indirect jump, and validate data blocks.
- `clamp_and_store_index`: Calculates an index by shifting and adding a constant to a value in d4, clamps the result to a maximum of 32767, stores it in a global variable, and then either clears or sets a 16-bit value at a global variable location based on a comparison.
- `process_dynamic_table`: Calculates a clamped index, manages a table of structures by updating a global pointer and various offsets, performs arithmetic involving multiplication and division, and eventually copies a sequence of words from a table to a destination via a pointer in a1.
- `update_dynamic_table_and_dispatch`: Calculates a scaled index using multiplication and bit shifting, then updates a structure with values from a table.
- `calculate_scaled_index_and_update_table`: Calculates a scaled value using multiplication and division by a constant, then uses a derived index to copy four words from a table into global variables.
- `calculate_scaled_table_index`: Performs a signed comparison and a scaled division using a constant, then checks the result against two thresholds before storing the value into a structure at a negative offset from a pointer and calling process_dynamic_table.
- `copy_table_data_and_update_globals`: Adds a word from the stack to 21000 in d4, calls another routine, shifts d2 left by 3, uses d2 to index into a table via a0, copies four words from that table into a structure at a1, stores a longword into a location relative to a4, writes two words into a structure at a1, and branches to another location.
- `update_dynamic_table_structure`: Calculates a scaled index using bit shifts and subtraction, then copies a four-word data block from a source table into a destination structure.
- `update_scaled_index_and_dispatch`: Calculates a scaled index by shifting a value based on a byte read from a global variable, updates a structure with this value, and then calls frag_move_add_jmp and sum_table_offsets before branching to another routine.
- `frag_move_andi_or_jmp`: Short code fragment that copies the memory at a0 to d2, then masks d2 with 12, then merges a location in the code into d2, then jumps to another point in the code.
- `frag_bra_71`: Short code fragment that jumps to another point in the code.
- `frag_bra_72`: Short code fragment that jumps to another point in the code.
- `frag_bra_73`: Short code fragment that jumps to another point in the code.
- `frag_bra_74`: Short code fragment that jumps to another point in the code.
- `frag_bra_75`: Short code fragment that jumps to another point in the code.
- `frag_bra_76`: Short code fragment that jumps to another point in the code.
- `frag_bra_77`: Short code fragment that jumps to another point in the code.
- `frag_bra_78`: Short code fragment that jumps to another point in the code.
- `frag_bra_79`: Short code fragment that jumps to another point in the code.
- `frag_bra_80`: Short code fragment that jumps to another point in the code.
- `frag_bra_81`: Short code fragment that jumps to another point in the code.
- `frag_bra_82`: Short code fragment that jumps to another point in the code.
- `update_table_entry_and_dispatch`: Calculates a bit-shifted index to access a table, copies four 16-bit words from that table into a destination structure, and then performs an indirect jump based on a value retrieved from the table.
- `frag_asr_add_cmp_ble_move_bge_rts`: Short code fragment that shifts d4 right by d7, then adds a location in the code to d4, then compares d4 with 32767, then branches if the comparison was less or equal, then copies 32767 to d4, then copies d4 to a fixed memory address, then compares d4 with a location in the code, then branches if the comparison was greater or equal, then copies 4 to a fixed memory address, then returns.
- `frag_clr_rts_2`: Short code fragment that clears a fixed memory address, then returns.
- `update_table_entry_and_dispatch_3`: Calculates a scaled index using a value from a table, updates a structure with four 16-bit words from a source buffer, and performs an indirect jump via a jump table.
- `fetch_and_copy_table_data`: Calculates a scaled index using a multiplication and bit shift, then copies four 16-bit words from a table into a structure at a location determined by a byte read from a0.
- `update_sprite_attribute_table_5`: Calculates a scaled index using a multiplication and bit shift, then fetches a byte from a source buffer to use as an index into a table.
- `update_table_entry_and_dispatch_4`: Calculates a scaled index using a value from a table, updates a structure with four 16-bit words from a memory location, and performs an indirect jump through a table based on a calculated offset.
- `update_table_entry_and_dispatch_2`: Calculates a bit-shifted index based on a value in a global variable, copies four 16-bit words from a table into a structure at a calculated offset, and performs an indirect jump to a routine selected from a jump table.
- `calculate_scaled_index_and_initialize`: Calculates a scaled index by shifting a value based on a subtracted byte, stores the result in a global pointer, and initializes a structure with a cleared 16-bit value, a value of 7, and a constant of 32767 before branching to another routine.
- `process_dynamic_table_entries`: Calculates a scaled index using multiplication and bit shifting, then updates a structure with values from a table and calls interpolate_rescaled_values, update_dynamic_table_and_dispatch, frag_move_add_jmp, and sum_table_offsets_2.
- `frag_move_sub_add_clr_bsr_muls_bra`: Short code fragment that copies d1 to d4, then subtracts a location in the code from d4, then adds d7 to d4, then clears d2, then copies a location in the code to d2, then calls another routine, then adds d3 to d7, then multiplies d7 by d6 (signed), then jumps to another point in the code.
- `update_structure_and_jump`: Calculates a value using multiplication and bit shifting, stores a 16-bit result at offset 2 of a structure in a5 and a byte at offset 4 of the same structure, then performs an indirect jump using a pointer from a2.
- `frag_move_sub_ext_add_clr_bsr_bra`: Short code fragment that copies a location in the code to d4, then subtracts a location in the code from d4, then sign-extends d4, then adds d5 to d4, then clears d2, then copies a location in the code to d2, then calls another routine, then jumps to another point in the code.
- `interpolate_table_value_and_loop`: Calculates a scaled index using multiplication and bit shifting, updates a 16-bit value in a structure via a table lookup, and then enters a loop that performs arithmetic on a global variable and executes an indirect jump through a table.
- `dispatch_indirect_jump_table`: Compares a value in a register with a threshold, then performs an indirect jump through a table.
- `frag_move_bra_5`: Short code fragment that copies the memory just below a0 (moving a0 down) to the memory at a0 (advancing a0), then jumps to another point in the code.
- `frag_bra_83`: Short code fragment that jumps to another point in the code.
- `frag_bra_84`: Short code fragment that jumps to another point in the code.
- `undecodable_bytes_3`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `frag_add_move_jmp`: Short code fragment that adds d2 to d2 (2 times in a row), then copies d4 to d7, then jumps to another point in the code.
- `frag_bra_85`: Short code fragment that jumps to another point in the code.
- `frag_bra_86`: Short code fragment that jumps to another point in the code.
- `frag_bra_87`: Short code fragment that jumps to another point in the code.
- `frag_bra_88`: Short code fragment that jumps to another point in the code.
- `frag_bra_89`: Short code fragment that jumps to another point in the code.
- `frag_bra_90`: Short code fragment that jumps to another point in the code.
- `frag_bra_91`: Short code fragment that jumps to another point in the code.
- `frag_bra_92`: Short code fragment that jumps to another point in the code.
- `frag_asl_move_asr_rts_2`: Short code fragment that shifts d4 left by 3, then copies an indexed entry from a4 to d7, then shifts d7 right by 5, then returns.
- `frag_asl_move_asr_rts`: Short code fragment that shifts d4 left by 2, then copies an indexed entry from a4 to d7, then shifts d7 right by 4, then returns.
- `frag_andi_sub_add_muls_asr_rts`: Short code fragment that masks d4 with 65534, then subtracts d4 from d7, then adds d4 to d4, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 1, then adds an indexed entry from a4 to d7, then shifts d7 right by 3, then returns.
- `frag_andi_sub_muls_asr_add_rts`: Short code fragment that masks d4 with 65532, then subtracts d4 from d7, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 2, then adds an indexed entry from a4 to d7, then shifts d7 right by 2, then returns.
- `calculate_scaled_value_3`: Masks d4 with 31, subtracts the result from d7, shifts d4 right by 1, multiplies d7 by a 16-bit value at offset 2 of a table indexed by d4, shifts d7 right by 3, adds a 16-bit value from offset 0 of the table, and shifts d7 right by 1 before returning.
- `frag_andi_sub_asr_muls_add_rts`: Short code fragment that masks d4 with 65504, then subtracts d4 from d7, then shifts d4 right by 3, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 5, then adds an indexed entry from a4 to d7, then adds d7 to d7, then returns.
- `frag_andi_sub_asr_muls_add_asl_rts`: Short code fragment that masks d4 with 65472, then subtracts d4 from d7, then shifts d4 right by 4, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 6, then adds an indexed entry from a4 to d7, then shifts d7 left by 2, then returns.
- `frag_move_add_jmp`: Short code fragment that copies d2 to d4, then adds d4 to d4 (2 times in a row), then jumps to another point in the code.
- `frag_bra_93`: Short code fragment that jumps to another point in the code.
- `frag_bra_94`: Short code fragment that jumps to another point in the code.
- `frag_bra_95`: Short code fragment that jumps to another point in the code.
- `frag_bra_96`: Short code fragment that jumps to another point in the code.
- `frag_bra_97`: Short code fragment that jumps to another point in the code.
- `frag_bra_98`: Short code fragment that jumps to another point in the code.
- `frag_bra_99`: Short code fragment that jumps to another point in the code.
- `frag_bra_100`: Short code fragment that jumps to another point in the code.
- `frag_move_asl_asr_rts_2`: Short code fragment that copies d1 to d4, then shifts d4 left by 3, then copies an indexed entry from a4 to d7, then shifts d7 right by 5, then returns.
- `frag_move_asl_asr_rts`: Short code fragment that copies d1 to d4, then shifts d4 left by 2, then copies an indexed entry from a4 to d7, then shifts d7 right by 4, then returns.
- `frag_move_andi_sub_add_muls_asr_rts`: Short code fragment that copies d1 to d4, then copies d1 to d7, then masks d4 with 65534, then subtracts d4 from d7, then adds d4 to d4, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 1, then adds an indexed entry from a4 to d7, then shifts d7 right by 3, then returns.
- `frag_move_andi_sub_muls_asr_add_rts`: Short code fragment that copies d1 to d4, then copies d1 to d7, then masks d4 with 65532, then subtracts d4 from d7, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 2, then adds an indexed entry from a4 to d7, then shifts d7 right by 2, then returns.
- `calculate_indexed_value`: Copies d1 to d4 and d7, masks d4 with 248, subtracts the result from d7, shifts d4 right by 1, multiplies d7 by a 16-bit value at offset 2 of a table indexed by d4, shifts d7 right by 3, adds a 16-bit value from offset 0 of the table, and shifts d7 right by 1 before returning.
- `frag_move_andi_sub_asr_muls_add_rts`: Short code fragment that copies d1 to d4, then copies d1 to d7, then masks d4 with 65504, then subtracts d4 from d7, then shifts d4 right by 3, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 5, then adds an indexed entry from a4 to d7, then adds d7 to d7, then returns.
- `frag_move_andi_sub_asr_muls_add_asl_rts`: Short code fragment that copies d1 to d4, then copies d1 to d7, then masks d4 with 65472, then subtracts d4 from d7, then shifts d4 right by 4, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 6, then adds an indexed entry from a4 to d7, then shifts d7 left by 2, then returns.
- `frag_add_move_jmp_2`: Short code fragment that adds d2 to d2 (2 times in a row), then copies d4 to d7, then jumps to another point in the code.
- `frag_bra_101`: Short code fragment that jumps to another point in the code.
- `frag_bra_102`: Short code fragment that jumps to another point in the code.
- `frag_bra_103`: Short code fragment that jumps to another point in the code.
- `frag_bra_104`: Short code fragment that jumps to another point in the code.
- `frag_bra_105`: Short code fragment that jumps to another point in the code.
- `frag_bra_106`: Short code fragment that jumps to another point in the code.
- `frag_bra_107`: Short code fragment that jumps to another point in the code.
- `frag_bra_108`: Short code fragment that jumps to another point in the code.
- `frag_asl_move_asr_ext_rts`: Short code fragment that shifts d4 left by 3, then copies an indexed entry from a4 to d7, then shifts d7 right by 5, then sign-extends d7, then returns.
- `frag_asl_move_asr_ext_rts_2`: Short code fragment that shifts d4 left by 2, then copies an indexed entry from a4 to d7, then shifts d7 right by 4, then sign-extends d7, then returns.
- `frag_andi_sub_add_muls_asr_ext_rts`: Short code fragment that masks d4 with 65534, then subtracts d4 from d7, then adds d4 to d4, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 1, then adds an indexed entry from a4 to d7, then shifts d7 right by 3, then sign-extends d7, then returns.
- `frag_andi_sub_muls_asr_add_ext_rts`: Short code fragment that masks d4 with 65532, then subtracts d4 from d7, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 2, then adds an indexed entry from a4 to d7, then shifts d7 right by 2, then sign-extends d7, then returns.
- `frag_andi_sub_asr_muls_add_ext_rts`: Short code fragment that masks d4 with 65528, then subtracts d4 from d7, then shifts d4 right by 1, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 3, then adds an indexed entry from a4 to d7, then shifts d7 right by 1, then sign-extends d7, then returns.
- `frag_andi_sub_asr_muls_add_ext_rts_2`: Short code fragment that masks d4 with 65520, then subtracts d4 from d7, then shifts d4 right by 2, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 4, then adds an indexed entry from a4 to d7, then sign-extends d7, then returns.
- `frag_andi_sub_asr_muls_add_ext_rts_3`: Short code fragment that masks d4 with 65504, then subtracts d4 from d7, then shifts d4 right by 3, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 5, then adds an indexed entry from a4 to d7, then sign-extends d7, then adds d7 to d7, then returns.
- `frag_andi_sub_asr_muls_add_ext_asl_rts`: Short code fragment that masks d4 with 65472, then subtracts d4 from d7, then shifts d4 right by 4, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 6, then adds an indexed entry from a4 to d7, then sign-extends d7, then shifts d7 left by 2, then returns.
- `frag_move_add_jmp_2`: Short code fragment that copies d2 to d4, then adds d4 to d4 (2 times in a row), then jumps to another point in the code.
- `frag_bra_109`: Short code fragment that jumps to another point in the code.
- `frag_bra_110`: Short code fragment that jumps to another point in the code.
- `frag_bra_111`: Short code fragment that jumps to another point in the code.
- `frag_bra_112`: Short code fragment that jumps to another point in the code.
- `frag_bra_113`: Short code fragment that jumps to another point in the code.
- `frag_bra_114`: Short code fragment that jumps to another point in the code.
- `frag_bra_115`: Short code fragment that jumps to another point in the code.
- `frag_bra_116`: Short code fragment that jumps to another point in the code.
- `frag_move_asl_asr_ext_rts`: Short code fragment that copies d5 to d4, then shifts d4 left by 3, then copies an indexed entry from a4 to d7, then shifts d7 right by 5, then sign-extends d7, then returns.
- `frag_move_asl_asr_ext_rts_2`: Short code fragment that copies d5 to d4, then shifts d4 left by 2, then copies an indexed entry from a4 to d7, then shifts d7 right by 4, then sign-extends d7, then returns.
- `calculate_extended_value`: Copies d5 to d4 and d7, masks d4 with 65534, subtracts the result from d7, adds d4 to itself, multiplies d7 by a 16-bit value at offset 2 of a table indexed by d4, shifts d7 right by 1, adds a 16-bit value from offset 0 of the table, shifts d7 right by 3, and extends d7 to a 32-bit value.
- `frag_move_andi_sub_muls_asr_add_ext_rts`: Short code fragment that copies d5 to d4, then copies d5 to d7, then masks d4 with 65532, then subtracts d4 from d7, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 2, then adds an indexed entry from a4 to d7, then shifts d7 right by 2, then sign-extends d7, then returns.
- `calculate_shifted_value`: Copies d5 to d4 and d7, masks d4 with 248, subtracts the result from d7, shifts d4 right by 1, multiplies d7 by a 16-bit value at offset 2 of a table indexed by d4, shifts d7 right by 3, adds a 16-bit value from offset 0 of the table, shifts d7 right by 1, and extends d7 to a 32-bit value.
- `frag_move_andi_sub_asr_muls_add_ext_rts`: Short code fragment that copies d5 to d4, then copies d5 to d7, then masks d4 with 65520, then subtracts d4 from d7, then shifts d4 right by 2, then multiplies d7 by an indexed entry from a4 (signed), then shifts d7 right by 4, then adds an indexed entry from a4 to d7, then sign-extends d7, then returns.
- `calculate_large_shifted_value`: Copies d5 to d4 and d7, masks d4 with 224, subtracts the result from d7, shifts d4 right by 3, multiplies d7 by a 16-bit value at offset 2 of a table indexed by d4, shifts d7 right by 5, adds a 16-bit value from offset 0 of the table, extends d7 to a 32-bit value, and doubles it.
- `calculate_left_shifted_value`: Copies d5 to d4 and d7, masks d4 with 192, subtracts the result from d7, shifts d4 right by 4, multiplies d7 by a 16-bit value at offset 2 of a table indexed by d4, shifts d7 right by 6, adds a 16-bit value from offset 0 of the table, extends d7 to a 32-bit value, and shifts it left by 2.
- `sum_table_offsets_2`: Loads a pointer into a2 using d4 and a4, sums nine 16-bit values from various offsets of the structure in a2, subtracts six 16-bit values from various negative offsets of the structure in a2, and shifts the result right by 3.
- `sum_table_offsets`: Loads a pointer into a1 using a4 and d4, sums several 16-bit words at various offsets from that pointer, subtracts several other 16-bit words at different offsets, and returns the result shifted right by three.
- `clear_memory_buffer_blocks`: Compares a global value against eight and, if greater, sets a global variable to a constant value; otherwise, it calculates a loop count based on a memory range and clears blocks of memory using a loop that writes zeros to a series of offsets.
- `update_bitfield_via_table_conditional_2`: Calculates a bitmask based on a value in d1 and a constant, then conditionally pushes a word from d3 onto the stack and performs an indirect.
- `update_bitfield_via_table_conditional`: Calculates a bitmask based on a value in d1 and a constant, then conditionally pushes a word from d3 onto the stack and performs an indirect jump to a location determined by the calculated offset.
- `update_bitfield_via_table`: Pushes four longwords from d3 onto the stack, retrieves a 16-bit value from a table using a masked d1 index, and performs bitwise operations or conditional updates on a bitfield based on the relationship between d2 and d7.
- `store_argument_conditionally`: Saves registers to the stack, masks d1 with 15, doubles it, loads a word from a table at offset 32 of a structure in a6 using the new d1, and performs a bitwise AND between that word and the value at the pointer in a1 before branching to another routine or performing a jump to a relative offset.
- `push_and_jump_table`: Pushes nine longwords from d4 onto the stack, fetches a word from a table using a masked and shifted index from d1, and then performs an indirect jump based on a bit test of d6 and d7.
- `frag_move_rts_bclr_beq_neg_jmp`: Short code fragment that copies d3 to the memory just below a1 (moving a1 down) (4 times in a row), then returns, then clears bit d6 of d2, then branches if the last result was zero, then copies d4 to the memory just below a1 (moving a1 down), then negates d2, then jumps to another point in the code.
- `push_and_loop_data`: Pushes ten longwords from d4 onto the stack, compares a value from a global variable with another, and then enters a loop that performs multiple stack operations based on a calculated count.
- `build_blitter_command_lists`: Calculates a count based on the difference between two global values, then populates a table with multiple command structures containing subroutine pointers, memory addresses, and bitmasks, and finally updates hardware registers for interrupts and display settings.
- `frag_movea_move_movem_rte`: Short code fragment that copies an address to a6, then copies the memory at a0 (advancing a0) to the memory at an offset from a6 (5 times in a row), then copies a0 to a fixed memory address, then copies the top of the stack to a6, then restores saved registers from the stack, then returns from the exception.
- `frag_movea_move_movem_rte_2`: Short code fragment that copies an address to a6, then copies the memory at a0 (advancing a0) to the memory at an offset from a6 (2 times in a row), then copies a0 to a fixed memory address, then copies the top of the stack to a6, then restores saved registers from the stack, then returns from the exception.
- `frag_movea_move_movem_rte_3`: Short code fragment that copies an address to a6, then copies the memory at a0 (advancing a0) to the memory at an offset from a6 (3 times in a row), then copies a0 to a fixed memory address, then copies the top of the stack to a6, then restores saved registers from the stack, then returns from the exception.
- `setup_blitter_extended`: Sets multiple blitter registers including address, mode, height, width, and size using values from a0 and stores the resulting pointer into a global variable before returning from an interrupt.
- `frag_movea_move_movem_rte_4`: Short code fragment that copies an address to a6, then copies the memory at a0 (advancing a0) to the memory at an offset from a6 (3 times in a row), then copies a0 to a fixed memory address, then copies the top of the stack to a6, then restores saved registers from the stack, then returns from the exception.
- `setup_blitter_complex`: Sets a wide range of blitter registers including mode, address, and various control parameters from a0 and stores the resulting pointer into a global variable before returning from an interrupt.
- `frag_movea_move_movem_rte_5`: Short code fragment that copies an address to a6, then copies the memory at a0 (advancing a0) to the memory at an offset from a6 (5 times in a row), then copies a0 to a fixed memory address, then copies the top of the stack to a6, then restores saved registers from the stack, then returns from the exception.
- `update_blt_registers`: Copies six values from a source list pointed to by a0 into the Blitter hardware registers and stores the next pointer in a global variable, then returns via an exception return.
- `frag_move_movea_movem_rte`: Short code fragment that copies 71842 to a fixed memory address, then copies 64 to a fixed memory address, then copies the top of the stack to a6, then restores saved registers from the stack, then returns from the exception.
- `frag_move_btst_bne_rts`: Short code fragment that copies 33792 to a fixed memory address, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then copies 1024 to a fixed memory address, then returns.
- `frag_move_lea_cmpa_bne_rts`: Short code fragment that copies 33792 to a fixed memory address, then loads the address of a location in the code into a6, then compares a6 with a location in the code, then branches if the last result was not zero, then copies 1024 to a fixed memory address, then returns.
- `frag_move_lea_cmpa_bne_btst_rts`: Short code fragment that copies 33792 to a fixed memory address, then loads the address of a location in the code into a6, then compares a6 with a location in the code, then branches if the last result was not zero, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then copies 1024 to a fixed memory address, then returns.
- `frag_move_lea_cmpa_bne_btst_rts_2`: Short code fragment that copies 33792 to a fixed memory address, then loads the address of a location in the code into a4, then compares a4 with a location in the code, then branches if the last result was not zero, then tests bit 6 of a fixed memory address, then branches if the last result was not zero, then copies 1024 to a fixed memory address, then returns.
- `initialize_asset_data_structures`: Copies and expands multiple data blocks from a source pointer, then processes several sequences of color palette data using varying counts.
- `copy_structured_data_blocks`: Depending on a value at an offset of a global variable, it performs nested loops to copy and manipulate word data from a source buffer into a destination table using bitwise OR and NOT operations.
- `process_color_palette_data`: Depending on a value at an offset of a global variable, it performs nested loops to calculate bitwise OR combinations of four words from a source buffer and writes them to a destination table at calculated offsets.
- `expand_data_structures`: Depending on a value at an offset of a global variable, it performs nested loops to copy words from a source buffer to a destination table while calculating a bitwise NOT of the ORed components.
- `update_memory_bits`: Sets bits in several memory locations using a value from d6 and its bitwise complement in d7, then clears d6 and returns.
- `update_graphics_scaling_and_buffers`: Calculates scaling factors and updates graphics buffer parameters by fetching values from a structure and calling update_graphics_buffer_parameters or update_graphics_buffer_parameters_2.
- `update_graphics_buffer_parameters`: Calculates a scaling factor based on a value in a structure and then performs four consecutive calls to another routine to update a parameter block in a6 with varying source and destination pointers, offsets, and constant values.
- `update_graphics_buffer_parameters_2`: Calculates a scaling factor and then performs four consecutive calls to another routine to update a structure with various memory addresses, scaling values, and constants based on a conditional check of a value at offset 20 of a structure.
- `update_blitter_graphics_buffers`: Calculates blitter parameters from a data structure to perform multiple area fills and bitwise operations on four distinct memory regions.
- `update_sprite_attribute_table_2_2`: Calculates an index using bitwise operations on registers, retrieves a pointer from a table, and performs a series of bitwise AND and OR operations to merge data from a source into a destination structure via a loop, then calls another routine.
- `update_sprite_attribute_table_2`: Iterates through a nested loop structure to update a destination table of 16-bit values using a source list of attributes.
- `copy_sprite_data_block`: Clamps a value in d0 by shifting it right and comparing it to 26, uses the result to select a pointer from a table, and copies sixteen 16-bit words from that pointer to a destination structure at an offset derived from a global variable.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs three instructions (moveq, ori.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `copy_byte_blocks`: Uses values from a global structure to select a source table and copies multiple bytes from that table to a destination calculated from two global pointers.
- `update_entity_attributes`: Calculates an index based on a value in d0, then performs a series of word-sized copies from two different tables into a structure located at a global variable offset.
- `update_sprite_attribute_tables_2`: Calculates an index into a table based on an input value, then performs two large series of block transfers to copy 16-bit words from two different source tables into a destination structure using interleaved offsets.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs two instructions (ori.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `frag_ori_lea_eori_nop`: Short code fragment that merges 66 into d1, then merges 70 into d1, then merges 242 into d3, then merges 250 into d3, then merges 2 into d3, then merges 10 into d3, then loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then exclusive-ors 4 into the memory at an offset from a1, then pauses for one no-operation step.
- `distribute_interleaved_data_blocks`: Copies multiple 16-bit words from a source buffer into several interleaved destination offsets within a table, then repeats the process using a second source buffer and a different destination base, finally toggling a bit in a global variable and updating two function pointers.
- `copy_data_blocks_2`: Copies multiple 16-bit words from a source buffer to a destination table using several offset-based memory transfers.
- `execute_three_subroutines`: Loads three different pairs of addresses and constants into registers and calls another routine three times before returning.
- `initialize_three_data_structures`: Calls another routine three times, each time passing a different pointer in a1, a specific offset in a3, and two constant values in d0 and d1.
- `update_interleaved_data`: Calls several routines to add a global pointer, copies four 16-bit words from one location to another using offsets of 0, 8000, 16000, and 24000, and then enters a loop that calls another routine to copy interleaved data.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_global_pointers`: Calls another routine multiple times to add a global pointer, including conditional calls if the value at a specific global variable is not greater than 16.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs one instruction (adda.l) and then falls through into the code that follows, with no return of its own.
- `copy_nested_blocks`: Performs a nested loop to copy 16-bit words from a source to a destination, using specific offsets to distribute data across a large memory range.
- `fall_through_entry_5`: Entry point part-way through straight-line code: runs one instruction (adda.l) and then falls through into the code that follows, with no return of its own.
- `frag_move_movea_dbra_lea_rts`: Short code fragment that copies d1 to d2, then copies a3 to a4, then copies the memory at an offset from a1 to the memory at an offset from a4 (3 times in a row), then copies the memory at a1 to the memory at a4 (advancing a4), then loops back while counting down d2, then loads the address of the memory at an offset from a3 into a3, then loops back while counting down d0, then returns.
- `copy_structured_data_4`: Sets several bytes in d3, then enters a loop that copies four 16-bit words from a table to a destination calculated from a global variable and another global variable, using offsets of 0, 8000, 16000, and 24000.
- `clear_bits_in_planes`: Uses a value from a table to find a pointer, then checks two 16-bit words at an offset in a buffer; if they are non-negative and equal, it calls another routine with a swapped value.
- `clear_bit_in_planes`: Calculates a memory address based on input values and clears a specific bit in four 16-bit words at that location and its corresponding plane offsets.
- `process_table_entry`: Loads data from a table using indices from registers d4 and d5, performs conditional checks on the loaded value, and calls a routine to set bits in planes based on the results.
- `set_bits_in_planes`: Calculates a memory address and sets specific bits in four 16-bit words, using a combination of bitwise OR and NOT operations across plane offsets.
- `modify_table_entry`: Loads data from a table using indices from registers d4 and d5, performs conditional checks on the loaded value, and calls a routine to modify bits in planes based on the results.
- `modify_bits_in_planes`: Calculates a memory address and modifies four 16-bit words by clearing one bit and setting others using bitwise AND, OR, and NOT operations.
- `format_decimal_buffer`: Divides a 16-bit value by various constants to extract digits and stores them as 16-bit words in a buffer starting with the value ten.
- `format_decimal_value`: Divides a 16-bit value by 100 and 10 to extract hundreds, tens, and units digits, storing them as 16-bit words in a buffer.
- `frag_divu_beq_move_bra`: Short code fragment that divides d0 by 60000 (unsigned), then branches if the last result was zero, then copies d0 to the memory at a0 (advancing a0), then jumps to another point in the code.
- `decompose_large_value`: Performs multiple divisions on a 32-bit value to extract and store its constituent parts as a sequence of 16-bit words in a buffer.
- `frag_moveq_move_beq_subq_movea_lea_bsr`: Short code fragment that loads 0 into d2, then copies the memory at a2 to d2, then branches if the last result was zero, then subtracts 1 from d2, then copies d2 to the memory at a2, then copies a location in the code to a4, then loads the address of a location in the code into a3, then calls another routine, then returns.
- `increment_and_conditional_call`: Loads a word from the address in a2 into d2, increments the word at the address in a2 if d2 is not 27, and calls another routine if a calculated value from a global pointer is greater than the value at the address in a2.
- `compare_byte_values`: Tests d0 and branches if zero, otherwise it subtracts d0 from a byte loaded from a0 into d4, tests d1 and branches if zero, otherwise it subtracts d1 from a byte loaded from a1 into d6, and finally branches.
- `frag_move_movea_lea_bsr`: Short code fragment that copies d4 to d2, then copies a location in the code to a4, then loads the address of a location in the code into a3, then calls another routine.
- `frag_move_movea_lea_bsr_rts`: Short code fragment that copies d5 to d2, then copies a location in the code to a4, then loads the address of a location in the code into a3, then calls another routine, then returns.
- `frag_tst_beq_move_sub`: Short code fragment that tests d1, then branches if the last result was zero, then copies the memory at a1 to d7, then copies d7 to d6, then subtracts d1 from d6.
- `frag_move_movea_lea_bsr_2`: Short code fragment that copies d6 to d2, then copies a location in the code to a4, then loads the address of a location in the code into a3, then calls another routine.
- `frag_move_movea_lea_bsr_rts_2`: Short code fragment that copies d7 to d2, then copies a location in the code to a4, then loads the address of a location in the code into a3, then calls another routine, then returns.
- `frag_cmp_beq_bsr_rts`: Short code fragment that compares d7 with d4, then branches if the last result was zero, then compares d6 with d5, then branches if the last result was zero, then calls another routine (2 times in a row), then returns.
- `frag_cmp_bne_bsr_rts`: Short code fragment that compares d6 with d5, then branches if the last result was not zero, then calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_6`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_bsr_rts_7`: Short code fragment that calls another routine (2 times in a row), then returns.
- `frag_move_ext_lsl_movea_adda_bsr_lea_rt`: Short code fragment that copies d2 to d3, then sign-extends d3, then shifts d3 left by 2, then copies an indexed entry from a3 to a3, then adds an address to a3, then calls another routine, then loads the address of the memory at an offset from a3 into a3, then calls another routine, then returns.
- `call_routine_twice`: Saves registers to the stack, calculates an address using a table lookup based on a shifted value in d0, and calls another routine twice using two different addresses derived from that calculation.
- `copy_interleaved_data`: Copies 16-bit words from a source buffer to a destination buffer at various offsets, including negative offsets relative to the destination pointer, and adjusts the source pointer before returning.
- `frag_ori`: Short code fragment that merges 250 into d3, then merges 218 into d3, then merges 226 into d3, then merges 234 into d3, then merges 242 into d3.
- `set_d0_bits`: Performs a series of bitwise OR operations on register d0 using a sequence of specific byte constants.
- `update_bitmask_and_call`: Performs multiple bitwise OR operations on a register, checks a value at a pointer, and calls another routine if a condition is met.
- `frag_bsr_rts_8`: Short code fragment that calls another routine, then returns.
- `update_sprite_attribute_table_4`: Calculates an index based on input values and updates a table of sprite attributes by copying bytes from a source structure to specific offsets.
- `build_bitmask_and_copy_data`: Constructs a bitmask in a register by performing bitwise OR operations with a series of constants, then calls a routine to obtain a destination pointer and copies sixteen words from a data table to that destination.
- `decompress_and_copy_data`: Calls a decompression routine, then if the result is not zero, copies 16 words from a table at offset 11184 of a global pointer to a destination buffer, and finally calls another routine with several parameters.
- `copy_and_interleave_words`: Copies 16-bit words from a source to four destination pointers with increasing offsets, then copies 32-bit words from a source to a destination pointer in a loop.
- `frag_lea_move_eori_movea_dbra_rts`: Short code fragment that loads the address of a fixed memory address into a1, then copies the memory at a1 to d0, then copies d0 to d1, then exclusive-ors 4 into d1, then copies an indexed entry from a1 to a0, then copies an indexed entry from a1 to a1, then copies 7999 to d0, then copies the memory at a1 (advancing a1) to the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `frag_movea_move_clr_dbra_rts`: Short code fragment that copies a fixed memory address to a0, then copies 7999 to d0, then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `frag_move_jsr_adda_cmp_beq_bsr_rts`: Short code fragment that copies 65535 to the stack, then copies a fixed memory address to the stack (2 times in a row), then calls another routine, then adds 10 to a7, then copies a fixed memory address to d0, then compares d0 with a fixed memory address, then branches if the last result was zero, then calls another routine, then returns.
- `frag_lea_move_eori_rts`: Short code fragment that loads the address of a fixed memory address into a5, then copies the memory at a5 to d5, then exclusive-ors 4 into d5, then copies d5 to the memory at a5, then copies an indexed entry from a5 to a fixed memory address, then returns.
- `search_linked_list`: Iterates through a table of 8-byte structures, clearing a byte at offset 1 if specific bit patterns are met, and updates a 16-bit pointer at offset 6 to the next matching entry.
- `update_timer_and_state_2`: Iterates through a table to find entries matching a value in a structure, then updates various fields in that structure, including timer offsets and status bits, and calls another routine.
- `process_list_elements`: Iterates through a table of 8-byte structures and calls a routine via a pointer if the byte at offset 1 of the current structure has its lowest two bits set to 2.
- `update_lap_timing_logic`: Calls two subroutines, then checks status bits in a structure to determine a timing constant.
- `update_structure_values`: Performs complex arithmetic and conditional logic on various fields of a structure pointed to by a1, including bitwise operations, divisions, and multiplications to update several offsets.
- `update_lap_timing_data`: Subtracts a value from two fields in a structure and, if the result is non-positive, iterates through a linked list to find the next valid entry.
- `update_timer_and_state`: Loops through a list until a bit is set, calculates a difference between a value in a structure and a global, updates a field in a1, and copies a value from a table into a structure.
- `modify_list_and_state`: Updates a 32-bit value in a structure, performs a search through a table to find a specific entry, and modifies a byte at offset 58 of the structure in a1.
- `copy_structured_data_2`: Updates several global variables, then performs a series of nested loops to copy blocks of 19 words from a source buffer to a destination buffer, with the source pointer advancing by 2560 or 160 bytes depending on a condition.
- `process_data_blocks`: Executes a loop eight times, calling another routine multiple times with varying pointer offsets derived from the value in a0.
- `convert_data_to_bitmask`: Uses nested loops to iterate through a source table, performs a swap and bitwise operations on the data, and stores the resulting bitmask into a destination table.
- `frag_move_lea_tst_beq_clr_dbra_rts`: Short code fragment that copies a fixed memory address to d0, then loads the address of a location in the code into a0, then copies 670 to d2, then tests a fixed memory address, then branches if the last result was zero, then copies 590 to d2, then copies d2 to the memory at a0 (advancing a0), then clears the memory at a0 (advancing a0), then loops back while counting down d0, then returns.
- `update_sprite_table_and_queue_blitter`: Updates a structure in a table by adjusting a pointer and a counter through arithmetic operations and loops, then populates a list of commands for a blitter routine with various parameters and offsets.
- `calculate_and_execute_blitter_fills`: Calculates a destination address and dimensions from a data structure, then performs a series of blitter operations using DMA to fill specific memory areas.
- `fall_through_entry_6`: Entry point part-way through straight-line code: runs two instructions (lsl.w, adda.l) and then falls through into the code that follows, with no return of its own.
- `configure_blitter_area_and_dma`: Calculates source and destination parameters for a blitter operation based on values from a structure, then configures the blitter registers and enables DMA. It performs a loop to find a bit position in a value derived from a table and calls another routine to calculate a specific offset before updating the blitter control and address registers.
- `calculate_and_execute_blitter_fills_2`: Calculates destination memory addresses and dimensions based on values from a structure, then performs a series of blitter operations to fill or clear areas in memory.
- `configure_blitter_fill_area`: Calculates blitter destination parameters based on input values and configures multiple blitter registers for DMA operations, then executes a series of blitter fills using control and address registers.
- `copy_structured_data`: Uses a nested loop to copy words from a source table to a destination structure based on values retrieved from a global pointer and a secondary table.
- `apply_bitmask_to_buffer_3`: Calculates a source address and bit-shift amount from a table, then performs a loop that copies 16-bit words from a source buffer to a destination buffer while applying a bitwise mask derived from another table.
- `apply_bitmask_to_buffer_2`: Extracts four 16-bit words from a source table using a bit-shift amount stored in a register, performs bitwise OR operations to combine them into two 32-bit values, and writes these values to a destination buffer while applying a bitwise AND mask if the combined result is non-zero.
- `apply_bitmask_to_buffer`: Performs bitwise AND and OR operations on 16-bit words extracted from a source table using a mask, then writes the resulting 32-bit values into a destination buffer.
- `update_bitmask_buffer`: Iterates through a source table using a loop, performing bitwise shifts and logical operations on 16-bit values to combine them into 32-bit words.
- `update_bitmask_buffers`: Reads a structure via a pointer in a5 to determine a base address and a bit-shift amount, then iterates through a series of data buffers to perform bitwise OR operations on 16-bit values based on calculated bitmask offsets.
- `combine_shifted_bitmask_values`: Reads two 16-bit values from a structure in a2, shifts each by a value in d1, and combines the results into d5 and d6 using bitwise OR operations.
- `update_bitmask_values_2`: Performs a nested loop that updates a bitmask table by shifting values from a source table using a bit-shift amount in a register and applying bitwise AND/OR operations to merge them into the destination table.
- `update_bitmask_values`: Performs a nested loop that shifts values from a table in a2 by the amount in d1, performs bitwise operations with values from a3, and stores the results back into a3.
- `update_blitter_area_parameters_2`: Reads three sets of data from a table using indices provided in a list, performs bitwise shifts and rotations to pack these values into five 16-bit integers, and writes these integers into a structure in memory at specific offsets.
- `update_blitter_area_parameters`: Iterates through a series of offsets in a structure to calculate memory addresses and calls configure_blitter_area_and_dma multiple times to write specific constant values and data from the structure into a destination buffer.
- `update_bitmask_buffers_2`: Iterates through several data buffers by calculating offsets from a base address in a5 using a word from a0 and a value from a global variable, then calls another routine to move data from the stack into those buffers.
- `update_bitmask_buffers_3`: If a word at the pointer in a0 is not 10, it calculates an offset using a5 and a value from a0, then writes two longwords to a table at an offset determined by a global pointer; it then calls update_bitmask_buffers.
- `write_inverted_values_2`: Updates a global variable, then iterates through a list to extract bit-shifted values from a table pointed to by a5 and writes their bitwise NOT to a destination table pointed to by a3.
- `write_inverted_values`: Writes the bitwise NOT of d0, d1, d2, d3, and d4 to specific offsets in a table, while also writing the original values to corresponding offsets.
- `frag_tst_blt_movea_adda_bra`: Short code fragment that tests a fixed memory address, then branches if the comparison was less, then copies a fixed memory address to a4, then adds 1040 to a4, then jumps to another point in the code.
- `process_hunk_relocations`: Loops six times to copy data from a structure into a target memory area, then iterates through a list of relocation entries to update memory addresses by calling another routine.
- `update_global_index`: Copies a longword from one memory location to another, modifies a global word by XORing it with 8, and compares a word from a pointer in a0 with a value at a calculated address before potentially incrementing the pointer in a0.
- `process_inverted_data`: Reads two values from a table using an index derived from a pointer in a5, performs bitwise shifts and logical OR operations on bytes and words, and calls write_inverted_values.
- `process_inverted_values`: Performs two separate sequences of reading values from a table based on a5 and a0, performing various bitwise shifts and logical OR operations to combine them into d0 through d4, and then calls write_inverted_values twice with different offsets into a destination pointed to by a global pointer.
- `process_table_data_2`: Updates a global word using a value from a0, and if that value is not 10, it calculates an address using a pointer in a5 and a shifted version of the value, performs several arithmetic operations and swaps on words retrieved from that address, and calls another routine.
- `process_bitmask_table`: Performs multiple sequences of reading values from a table, applying bitwise shifts and logical OR operations to combine word and byte data, and calls write_inverted_values.
- `process_bitmask_data`: Calculates combined bitwise values from a table using shifts and OR operations across two different sets of data, then calls write_inverted_values twice with different offsets into a destination pointed to by a global pointer.
- `update_blitter_command_list`: Calculates a destination address based on a scaled offset and a base pointer, then writes a command length and a source address to a structure.
- `configure_blitter_parameters`: Calculates an offset using a5 and d5, then writes two words to a table at an offset from a global pointer; it modifies these words based on the value of d6 before calling configure_blitter_area_and_dma.
- `update_state_via_calls`: Performs a series of conditional checks on a word at a pointer, then calls another routine multiple times while toggling a bit in a global variable and updating a global pointer.
- `frag_move_movea_bne_bra`: Short code fragment that copies the memory at a5 (advancing a5) to d0, then copies a5 to the stack, then copies the top of the stack to a5, then copies d0 to the stack, then copies a fixed memory address to d3, then branches if the last result was not zero, then copies 314 to the memory at an offset from a5, then copies 320 to the memory at an offset from a5, then jumps to another point in the code.
- `process_structured_data`: Sets two longwords at offsets 8 and 12 of a structure in a5, calls another routine, then retrieves a longword from a structure using a global pointer and performs bounds checking and updates to that longword before calling two other routines.
- `initialize_and_loop_structure`: Loads a pointer into a register, clears a long value at that pointer and a word at an offset of 4, then enters a nested loop structure that decrements a value from the stack and a constant value before returning.
- `frag_move_rts_5`: Short code fragment that copies an address to a fixed memory address (4 times in a row), then returns.
- `frag_move_ext_neg_lea_subq_bge_clr_rts`: Short code fragment that copies the memory at a5 to d0, then sign-extends d0, then negates d0, then loads the address of a fixed memory address into a3, then subtracts 2 from d0, then branches if the comparison was greater or equal, then clears the memory at an offset from a3, then copies 256 to the memory at an offset from a3, then returns.
- `update_animation_data`: Calculates a value based on a byte from a structure, updates several words in a table at a fixed address, and calls another routine before jumping to a location determined by a table lookup.
- `update_hardware_register`: Writes a 16-bit value from d1 to a memory location, adds that value to a 16-bit value in d4 and stores the result at an offset of 2 from the same location, masks and shifts a byte from a global variable into d6, calls another routine, and then writes a byte to a different global variable before returning.
- `calculate_scaled_values`: Computes a value using bit shifts and additions, stores it in a structure, and calls multiple routines to update related data.
- `frag_lsl_movea_adda_bsr_rts`: Short code fragment that shifts d0 left by 2, then copies the memory at an offset from a3 to a0, then adds d0 to a0, then calls another routine, then returns.
- `frag_move_bra_6`: Short code fragment that copies 200 to d7, then jumps to another point in the code.
- `frag_move_bra_7`: Short code fragment that copies 160 to d7, then jumps to another point in the code.
- `frag_move_bra_8`: Short code fragment that copies 120 to d7, then jumps to another point in the code.
- `frag_move_mulu_lsr_add_bra`: Short code fragment that copies the memory at a3 to d0, then multiplies d7 by d0 (unsigned), then shifts d7 right by 8, then copies d7 to the memory at an offset from a3, then copies d0 to d7, then adds d7 to d7, then adds d0 to d7, then shifts d7 right by 5, then copies d7 to the memory at an offset from a3, then jumps to another point in the code.
- `frag_move_lsr_bra`: Short code fragment that copies the memory at a3 to d0, then copies d0 to d7, then shifts d7 right by 3, then copies d7 to the memory at an offset from a3 (2 times in a row), then jumps to another point in the code.
- `frag_move_lsr_bra_2`: Short code fragment that copies the memory at a3 to d0, then copies d0 to d7, then shifts d7 right by 2, then copies d7 to the memory at an offset from a3, then shifts d7 right by 1, then copies d7 to the memory at an offset from a3, then jumps to another point in the code.
- `update_physics_parameters`: Calculates several values using bit shifts and multiplication, updates a structure, and calls multiple routines to perform indirect jumps and table lookups.
- `frag_move_lsr_add_bra`: Short code fragment that copies the memory at a3 to d0, then copies d0 to d7, then shifts d7 right by 3, then copies d7 to d2, then shifts d2 right by 1, then adds d2 to d7, then copies d7 to the memory at an offset from a3, then jumps to another point in the code.
- `frag_move_lsr_bra_3`: Short code fragment that copies the memory at a3 to d0, then copies d0 to d7, then shifts d7 right by 3, then copies d7 to the memory at an offset from a3, then jumps to another point in the code.
- `frag_lsr_subq_bge_clr_bra`: Short code fragment that shifts d0 right by 5, then subtracts 1 from d0, then branches if the comparison was greater or equal, then clears d0, then jumps to another point in the code.
- `limit_and_update_value`: Clamps a value to a maximum of five, stores it in a structure, and calls multiple routines to update related data.
- `frag_lea_move_add_movea_adda_bsr_bra`: Short code fragment that loads the address of a location in the code into a0, then copies d3 to d7, then copies d7 to d0, then adds d0 to d0, then copies an indexed entry from a0 to d0, then copies a location in the code to a0, then adds d0 to a0, then calls another routine, then jumps to another point in the code.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `update_status_flags`: Compares a word in d2 against a global value and a constant, then writes specific values to a structure in a4 and potentially updates values in a6 based on the result.
- `process_conditional_logic`: Checks a bit in a memory location, then performs comparisons on d2 to write values to a structure in a4 and a6, potentially updating a bit in a4 based on a sum of d3 and d5.
- `calculate_differences`: Calculates the difference between a word in a table and d0, stores it in a structure in a4, and then subtracts a global value from a second word in a4.
- `decrement_and_store_values`: Updates a 16-bit value in a structure in a4 using d7, and if a condition is met, decrements d7 in a loop and stores the results into a table pointed to by a5.
- `update_structure_counters`: Updates a 16-bit value at offset 14 of a structure in a4 and a 16-bit value at offset 16 of the same structure based on a comparison with d7, then stores results in offsets 38 and 40 of the structure.
- `frag_move_lea_lsl_ror_rts`: Short code fragment that copies d3 to d6, then loads the address of a location in the code into a2, then shifts d6 left by 2, then copies an indexed entry from a2 to d6, then rotates d6 right by d5, then copies d6 to the memory at an offset from a6, then copies d5 to d7, then rotates d7 right by 4, then copies d7 to the memory at an offset from a6, then returns.
- `frag_jmp`: Short code fragment that jumps to another point in the code.
- `frag_bra_117`: Short code fragment that jumps to another point in the code.
- `frag_bra_118`: Short code fragment that jumps to another point in the code.
- `frag_bra_119`: Short code fragment that jumps to another point in the code.
- `frag_bra_120`: Short code fragment that jumps to another point in the code.
- `frag_bra_121`: Short code fragment that jumps to another point in the code.
- `frag_bra_122`: Short code fragment that jumps to another point in the code.
- `frag_bra_123`: Short code fragment that jumps to another point in the code.
- `frag_bra_124`: Short code fragment that jumps to another point in the code.
- `perform_bulk_bitwise_operations`: Performs a series of bitwise OR operations on memory locations using d6 and a loop controlled by d7, with different offsets and bit-widths depending on values in a0.
- `apply_bitwise_or_loop_2`: Sets several fields in a structure at a6, then enters a loop that bitwise ORs d6 into specific memory locations relative to a1, using a counter derived from d7, and finally updates the structure at a6 with values from a0 and a1.
- `frag_or_lea_dbra_bra`: Short code fragment that merges d6 into the memory at a1, then merges d6 into the memory at an offset from a1, then loads the address of the memory at an offset from a1 into a1, then loops back while counting down d7, then jumps to another point in the code.
- `frag_jmp_2`: Short code fragment that jumps to another point in the code.
- `update_ram_values`: Reads a byte from a5 plus 1, calculates a value in d0, and if d0 is not zero, writes several values to a table at 36032, including a value from a5 plus 6, a value from a5 plus 14 plus a shifted version of d3, and a subtracted value; it then calls frag_clr_move_bra and branches to frag_move_sub_bsr_bra.
- `frag_move_sub_bsr_bra`: Short code fragment that copies d1 to d7, then copies the memory at an offset from a5 to d5, then subtracts d0 from d5, then calls another routine, then jumps to another point in the code.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_and_update_collision_bounds`: Calculates a set of boundary values by performing arithmetic on bytes and words from two structures and writes them to a table.
- `write_ram_parameters`: Writes a word from offset 10 of a structure in a3 to offset 2 of a table at a fixed address, and if a value at offset 8 of a structure in a4 is negative one, writes 319 to offset 2 of that table before calling another routine.
- `compare_and_call_routine`: Compares a value at offset 6 of the structure in a3 with a value at offset 2, and if the first is not less than the second, it writes a value to a global address, calculates a difference between offsets 6 and 2 of the structure, and calls another routine.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `calculate_scaled_offsets`: Calculates a value based on a word at offset 10 of a structure in a3, and if not zero, calls three different routines using various combinations of that value and data from a structure in a5.
- `calculate_scaled_offset_2`: Calculates a value by adding a word from offset 10 of a structure in a3 to a word from offset 14 of a structure in a5, then performs three calls using modified d7 and d6 values before branching to another routine.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `interpolate_and_update_callback`: Calculates interpolated coordinate values based on a global variable and two input structures, then populates and initializes several callback structures with varying header types and coordinate data.
- `process_scaled_loop_data`: Calculates a bit-shifted value from a word at offset 10 of a structure, then enters a loop that compares a value at offset 6 of a structure against a constant, performing arithmetic and calling frag_subq_bge_rts_3 and frag_clr_move_bra until a condition is met.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `update_clipping_rectangles`: Calculates and updates a series of sub-rectangles or clipping boundaries based on dimensions and offsets stored in two structures, then calls a routine to initialize a callback structure for each calculated region.
- `update_clipping_rectangles_3`: Calculates dimensions by shifting values from a structure, calls another routine if a value is non-negative, and calls frag_subq_bge_rts_3 and frag_clr_move_bra based on comparisons.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `recalculate_buffer_segments`: Calculates new segment boundaries and lengths based on values in two structures, then calls a callback routine multiple times to initialize new data structures representing the overlapping or remaining segments.
- `calculate_and_loop_buffer_index`: Calculates a loop counter and an index based on values from a structure in a5 and a pointer in a3, then calls another routine and frag_subq_bge_rts_3 to iterate through a sequence of bytes loaded into d3.
- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `rescale_and_initialize_callback`: Calculates scaled coordinate values based on two input structures, performs linear interpolation to adjust their dimensions, and then populates multiple callback structures with the resulting bounding boxes and parameters before calling frag_subq_bge_rts_3 and calculate_offset_and_pointer.
- `update_callback_parameters`: Performs a series of conditional checks on values within three structures to determine if a new callback configuration should be built.
- `calculate_loop_parameters`: Performs arithmetic on a word from a structure in a3 to derive two values, then calls another routine twice with different parameters and calls a third routine to load a byte into d3.
- `calculate_buffer_parameters`: Calculates two different length values based on a 16-bit word from a structure and a value in d0, calls a routine to decrement a loop counter twice, and then calls a routine to load a byte into d3.
- `calculate_buffer_bounds_and_load_byte`: Calculates two different length values based on a 16-bit word from a structure and a value in d0, then calls a routine to load a byte into d3 using a pointer in a1.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `interpolate_and_update_structures`: Calculates interpolated values for two sets of coordinate-like data based on a global variable and updates several memory structures.
- `interpolate_coordinate_data`: Calculates two values based on a word from a structure, calls a routine twice with different parameters, and then calls a routine to load a byte into d3 after subtracting a value from d5.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `update_clipping_rectangles_2`: Calculates and updates a set of rectangular coordinates by interpolating values between two structures in a4 and a5, then calls initialize_callback_structure multiple times to queue various clipping or drawing operations based on the resulting bounds.
- `initialize_callback_structure`: Loads the address of the current routine into a1 and a2, sets a0 to a table at a fixed address, and performs conditional data movement into a structure based on the difference between values at offsets 2 and 8.
- `execute_callback_loop_2`: Enters a loop that checks if the difference between values at offsets 2 and 8 of a structure in a2 is zero, and if so, calls a routine using a pointer from offset 4 of the structure, decrementing a counter in d0 until it reaches negative one.
- `execute_callback_loop`: Decrements a counter in a loop while calling another routine using a pointer from a table, then performs arithmetic on a value in d7 and calls another routine before returning.
- `frag_move_movem_bsr_rts`: Short code fragment that copies a location in the code to d6, then saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_bra_125`: Short code fragment that jumps to another point in the code.
- `frag_bra_126`: Short code fragment that jumps to another point in the code.
- `frag_bra_127`: Short code fragment that jumps to another point in the code.
- `frag_bra_128`: Short code fragment that jumps to another point in the code.
- `frag_bra_129`: Short code fragment that jumps to another point in the code.
- `frag_bra_130`: Short code fragment that jumps to another point in the code.
- `frag_bra_131`: Short code fragment that jumps to another point in the code.
- `frag_bra_132`: Short code fragment that jumps to another point in the code.
- `frag_bra_133`: Short code fragment that jumps to another point in the code.
- `frag_bra_134`: Short code fragment that jumps to another point in the code.
- `frag_move_moveq_bra`: Short code fragment that copies d6 to d7, then loads 1 into d6, then jumps to another point in the code.
- `frag_move_moveq_bra_2`: Short code fragment that copies d6 to d7, then loads 255 into d6, then jumps to another point in the code.
- `frag_move_moveq_bra_3`: Short code fragment that copies d2 to d4, then copies d6 to d7, then loads 252 into d6, then jumps to another point in the code.
- `frag_move_moveq_bra_4`: Short code fragment that copies d2 to d4, then copies d6 to d7, then loads 253 into d6, then jumps to another point in the code.
- `frag_move_moveq_bra_5`: Short code fragment that copies d2 to d4, then copies d6 to d7, then loads 254 into d6, then jumps to another point in the code.
- `frag_move_moveq_bra_6`: Short code fragment that copies d2 to d4, then copies d6 to d7, then loads 255 into d6, then jumps to another point in the code.
- `frag_move_moveq_bra_7`: Short code fragment that copies d2 to d4, then copies d6 to d7, then loads 1 into d6, then jumps to another point in the code.
- `frag_move_moveq_bra_8`: Short code fragment that copies d2 to d4, then copies d6 to d7, then loads 2 into d6, then jumps to another point in the code.
- `frag_subq_bge_rts`: Short code fragment that subtracts 1 from d6, then branches if the comparison was greater or equal, then returns.
- `frag_lsr_bcc_move_dbra_rts`: Short code fragment that shifts d6 right by 1, then branches if the carry flag is clear, then copies d2 to the memory at a0 (advancing a0) (2 times in a row), then loops back while counting down d6, then returns.
- `populate_buffer_with_interpolated_values`: Calculates and stores a sequence of 16-bit values into a buffer pointed to by a0, using linear interpolation or step-based increments based on the relationship between d5 and d6.
- `increment_buffer_values`: Stores a word from a register into a buffer pointed to by a0, then loops while decrementing d6, adding d5 to d4 and incrementing d2 in each iteration.
- `frag_add_dbra_rts`: Short code fragment that adds d5 to d4, then loops back while counting down d6, then returns.
- `fill_buffer_with_interpolated_values`: Performs a series of arithmetic operations involving division and bit shifts to calculate step values, then loops to store words into a buffer pointed to by a0 while decrementing d2 and d6.
- `decrement_buffer_values`: Stores a word from a register into a buffer pointed to by a0, then loops while decrementing d6, adding d3 to d4 and decrementing d2 in each iteration.
- `frag_add_dbra_rts_2`: Short code fragment that adds d5 to d4, then loops back while counting down d6, then returns.
- `frag_subq_bge_rts_2`: Short code fragment that subtracts 1 from d7, then branches if the comparison was greater or equal, then returns.
- `frag_lsr_bcc_move_add_dbra_rts`: Short code fragment that shifts d7 right by 1, then branches if the carry flag is clear, then copies d2 to the memory at a0 (advancing a0) (2 times in a row), then adds d6 to d2, then loops back while counting down d7, then returns.
- `frag_subq_bge_rts_3`: Short code fragment that subtracts 1 from d7, then branches if the comparison was greater or equal, then returns.
- `frag_lsr_bcc_move_add_dbra_rts_2`: Short code fragment that shifts d7 right by 1, then branches if the carry flag is clear, then copies d4 to the memory at a0 (advancing a0), then adds d6 to d4, then copies d4 to the memory at a0 (advancing a0), then adds d6 to d4, then loops back while counting down d7, then returns.
- `frag_clr_move_bra`: Short code fragment that clears d3, then copies the memory at an offset from a5 to d3, then jumps to another point in the code.
- `calculate_offset_and_pointer`: Calculates a value based on a byte at offset 4 of a structure in a4 and several registers, then updates a pointer in a0 and modifies d7.
- `update_blitter_and_global_state`: Performs several comparisons and arithmetic operations on values from a table and a global variable, updates several global variables, and calls another routine.
- `update_blt_parameters_via_table`: Waits for DMA and interrupts, disables the Blitter by setting a register to all ones, and then enters a loop that iterates through a table to calculate and update Blitter parameters including mode, area, and size based on values read from a buffer, eventually calling another routine or returning.
- `frag_clr_move_bra_2`: Short code fragment that clears d0, then copies d0 to d5, then jumps to another point in the code.
- `frag_lea_dbra_rts`: Short code fragment that loads the address of the memory at an offset from a4 into a4, then loops back while counting down d7, then returns.
- `frag_move_subq_moveq_bra`: Short code fragment that copies a fixed memory address to d6, then subtracts 2 from d6, then loads 30 into d4, then jumps to another point in the code.
- `process_table_data`: Reads a word from a buffer, performs lookups in a table using bits from that word, and if certain conditions are met, updates several fields in a structure pointed to by a6 before jumping to an address in a5.
- `frag_blt_add_adda_not_swap_move_jmp`: Short code fragment that branches if the comparison was less, then adds d0 to d0, then adds d0 to a1, then inverts d4, then swaps the halves of d5, then copies d4 to d5, then copies d5 to d4, then inverts d4, then jumps to another point in the code.
- `frag_not_and_move_add_adda_jmp`: Short code fragment that inverts d4, then masks d5 with d4, then copies d5 to d4, then inverts d4, then adds d0 to d0, then adds d0 to a1, then jumps to another point in the code.
- `frag_or_and_lea_dbra_rts`: Short code fragment that merges d5 into the memory at a1, then merges d5 into the memory at an offset from a1, then masks the memory at an offset from a1 with d4, then merges d5 into the memory at an offset from a1, then loads the address of the memory at an offset from a4 into a4, then loops back while counting down d7, then returns.
- `update_sprite_animation_frame_data`: Iterates through a table of animation data, performing bitwise shifts and comparisons on word values to calculate new dimensions and positions.
- `frag_bra_135`: Short code fragment that jumps to another point in the code.
- `frag_bra_136`: Short code fragment that jumps to another point in the code.
- `frag_bra_137`: Short code fragment that jumps to another point in the code.
- `frag_bra_138`: Short code fragment that jumps to another point in the code.
- `frag_bra_139`: Short code fragment that jumps to another point in the code.
- `frag_bra_140`: Short code fragment that jumps to another point in the code.
- `frag_bra_141`: Short code fragment that jumps to another point in the code.
- `frag_bra_142`: Short code fragment that jumps to another point in the code.
- `frag_bra_143`: Short code fragment that jumps to another point in the code.
- `frag_bra_144`: Short code fragment that jumps to another point in the code.
- `frag_bra_145`: Short code fragment that jumps to another point in the code.
- `process_sprite_animation_frames`: Iterates through animation frame data to update sprite dimensions and positions, calling frag_move_ext_neg_lea_subq_bge_clr_rts, update_animation_data, and update_interpolated_value to manage frame transitions and coordinate interpolation between two structures in a4 and a5.
- `update_interpolated_value`: Calculates a new value based on a target and a current state using various arithmetic modes, then performs a loop to adjust a sequence of bytes in a buffer to match the new value.
- `update_graphics_parameter_structure_2`: Iteratively updates a series of pointers and values within a structure pointed to by a6, using offsets to set memory locations for source, destination, and control parameters based on values read from a list in a4.
- `update_graphics_parameter_structure`: Iteratively updates a series of pointers and values within a structure pointed to by a6, using offsets to set memory locations for source, destination, and size parameters.
- `update_graphics_structure_elements_2`: Iterates through a sequence of bytes from a pointer in a4, using each byte to calculate and update multiple fields within a structure pointed to by a6, including pointer offsets, word values, and bit-flagged status words.
- `update_graphics_structure_elements`: Updates multiple fields within a structure pointed to by a6, including several pointer and word values, by iterating through a series of offsets and applying calculated values based on registers d1, d2, d3, d4, and d7.
- `frag_move_ror_bne_ori_bra`: Short code fragment that copies a5 to the stack, then rotates d6 right by 4, then copies d6 to the memory at an offset from a6, then copies d6 to d0, then copies the memory at a0 (advancing a0) to d7, then branches if the last result was not zero, then merges 2826 into d0, then jumps to another point in the code.
- `frag_ori_swap_move_bne_bra`: Short code fragment that merges 3066 into d0, then swaps the halves of d0, then copies d6 to d0, then copies the memory at a0 to d7, then branches if the last result was not zero, then merges 2826 into d0, then jumps to another point in the code.
- `populate_graphics_parameter_table_2`: Iterates through a byte sequence to populate a table of structures in a6 with various parameters, including pointers, offsets, and bitwise-modified values.
- `populate_graphics_parameter_table`: Populates a series of structures pointed to by a6 with values from registers and memory, using a loop to select different memory offsets based on a bit test.
- `update_sprite_attribute_table`: Iterates through a list of bytes to update a structure in a6, setting various 16-bit and 32-bit fields including pointers, coordinates, and attribute values.
- `frag_move_ror_bne_ori_bra_2`: Short code fragment that copies a5 to the stack, then rotates d6 right by 4, then copies d6 to the memory at an offset from a6, then copies d6 to d0, then copies the memory at a0 (advancing a0) to d7, then branches if the last result was not zero, then merges 2826 into d0, then jumps to another point in the code.
- `frag_ori_swap_move_bne_bra_2`: Short code fragment that merges 3066 into d0, then swaps the halves of d0, then copies d6 to d0, then copies the memory at a0 to d7, then branches if the last result was not zero, then merges 2826 into d0, then jumps to another point in the code.
- `frag_ori_bra`: Short code fragment that merges 3066 into d0, then merges 3898 into d6, then jumps to another point in the code.
- `update_structure_list_elements_data`: Iterates through a byte stream to update a list of structures, calculating new pointer offsets and values based on the input bytes and arithmetic involving multiplication and bit shifts.
- `update_structure_list_elements`: Iterates through a list of data, performing arithmetic on various registers to calculate new pointer and value offsets, and updates multiple fields in a structure pointed to by a6.
- `update_structure_fields_4_2`: Writes a value to offset 150 of a structure in a6, then checks bit 6 of a byte at offset 2 of that structure.
- `update_structure_fields_4`: Checks bit 6 of a byte at offset 2 of a structure in a6; if not set, it writes 400 to offset 96, 3fa to offset 40, the value in d0 to offsets 48 and 54, and the value in d6 to offset 58, then returns.
- `update_structure_fields_3`: Updates multiple fields in a structure including a 16-bit counter, two 32-bit values, and several 16-bit values based on input parameters.
- `update_structure_fields_2`: Updates multiple fields in a structure including a 16-bit counter, two 32-bit values, and several 16-bit values based on input parameters and loop iterations.
- `initialize_structure_fields`: Sets multiple fields in a structure including a 16-bit counter, two 32-bit values, and several 16-bit values.
- `update_structure_fields_4_3`: Checks bit 6 of a byte at offset 2 of a structure in a6; if not set, it writes 400 to offset 96, 3fa to offset 40, the value in d0 to offsets 48 and 54, and the value in d6 to offset 58, then returns.
- `frag_move_ror_bne_ori_bra_3`: Short code fragment that copies a5 to the stack, then rotates d6 right by 4, then copies d6 to the memory at an offset from a6, then copies d6 to d0, then copies the memory at a0 (advancing a0) to d7, then branches if the last result was not zero, then merges 2826 into d0, then jumps to another point in the code.
- `frag_ori_swap_move_bne_bra_3`: Short code fragment that merges 3066 into d0, then swaps the halves of d0, then copies d6 to d0, then copies the memory at a0 to d7, then branches if the last result was not zero, then merges 2826 into d0, then jumps to another point in the code.
- `populate_graphics_structure_parameters`: Iterates through a data stream to populate a structure pointed to by a6 with source and destination pointers, offsets, and various 16-bit values.
- `update_graphics_structure_parameters`: Populates a structure pointed to by a6 with various source pointers, destination pointers, and offsets, performing multiple conditional updates based on a bit test.
- `update_sprite_attribute_table_3`: Iterates through a data stream to update a structure in a6, setting various 16-bit and 32-bit fields including pointers, coordinates, and color attributes.
- `update_sprite_attribute_list`: Calculates sprite position and attribute data based on a byte sequence from a pointer in a4, then updates a structure in a6.
- `frag_moveq_move_movea_jsr_rts`: Short code fragment that loads 0 into d0, then copies a location in the code to d0, then copies a location in the code to a0, then calls another routine, then returns.
- `unpack_and_reconstruct_sprite_data`: Checks a table for a valid pointer, then copies a data stream into a destination buffer and performs a series of bitwise OR operations to reconstruct multi-word sprite pixel data based on flags in a control structure.
- `process_sprite_data`: Calculates sprite dimensions and offsets from a source structure, then iterates through a data stream to call subroutines for sprite rendering or attribute updates.
- `decompress_bitstream_to_memory`: Reads a bitstream from a source buffer and expands it into a destination buffer using a series of bitwise operations and a loop.
- `perform_bitwise_operations_subset`: Performs a series of logical OR operations between registers d7, d5, d4, and d3.
- `perform_bitwise_operations_variant`: Performs a series of logical OR operations between registers d7, d5, d4, and d3 using different instruction orders than other routines.
- `process_byte_stream`: Reads bytes from a buffer in a0, performs bitwise manipulations, and accumulates values into d6 based on the contents of the buffer.
- `unpack_bitstream_to_bytes`: Reads a stream of bits from a source buffer and expands them into a destination buffer by packing nibbles into bytes.
- `copy_memory_blocks`: Copies the next byte from the buffer in a0 to the keyboard queue and calls the queue handler when the byte is the end marker.
- `update_structure_bounds`: Calls another routine, clears two registers, and reads two 16-bit values from offsets 6 and 8 of a structure pointed to by a3 into global variables.
- `update_pointer_and_bits`: Adjusts a pointer in a0 using d7, toggles a bit in d0, and enters a loop that calls another routine and performs bitwise OR operations on the memory at a0 using d4 or d3, while decrementing d5 and d6.
- `update_pointer_and_counters`: Performs a bitwise OR on a byte at the address in a0, then enters a loop that increments or decrements a global variable based on comparisons between d1 and three different values, and modifies a0 using d7 and a 16-bit word from a structure in a3.
- `frag_bchg_beq_addq_bra`: Short code fragment that flips bit 0 of d0, then branches if the last result was zero, then adds 1 to a0, then jumps to another point in the code.
- `frag_cmp_bgt_adda_bra`: Short code fragment that compares d1 with the memory at an offset from a3, then branches if the comparison was greater, then adds d7 to a0, then jumps to another point in the code.
- `frag_bchg_beq_addq_neg_move_bra`: Short code fragment that flips bit 0 of d0, then branches if the last result was zero, then adds 1 to a0, then negates d7, then copies a location in the code to d6, then jumps to another point in the code.
- `write_packed_nibbles`: Iterates through a loop to write nibbles from registers to memory at a destination pointed to by a0, using a table-based count and an offset to determine the write position and value.
- `apply_structured_bit_masks`: Iterates through multiple nested loops to perform bitwise OR operations on memory locations using d4 and d3, with varying offsets and conditions based on d7 and d1.
- `clear_memory_block`: Calculates a count based on values in a table and uses a loop to clear a block of memory starting at a pointer in a0.
- `extract_nibbles`: Iterates through data in a1 or a2, extracting nibbles based on the state of a bit in d2 and updating a counter in d5.
- `frag_bra_146`: Short code fragment that jumps to another point in the code.
- `frag_bra_147`: Short code fragment that jumps to another point in the code.
- `frag_bra_148`: Short code fragment that jumps to another point in the code.
- `frag_bra_149`: Short code fragment that jumps to another point in the code.
- `frag_bra_150`: Short code fragment that jumps to another point in the code.
- `frag_bra_151`: Short code fragment that jumps to another point in the code.
- `frag_bra_152`: Short code fragment that jumps to another point in the code.
- `frag_bra_153`: Short code fragment that jumps to another point in the code.
- `frag_bra_154`: Short code fragment that jumps to another point in the code.
- `frag_bra_155`: Short code fragment that jumps to another point in the code.
- `frag_bra_156`: Short code fragment that jumps to another point in the code.
- `frag_bra_157`: Short code fragment that jumps to another point in the code.
- `frag_bra_158`: Short code fragment that jumps to another point in the code.
- `frag_bra_159`: Short code fragment that jumps to another point in the code.
- `frag_bra_160`: Short code fragment that jumps to another point in the code.
- `frag_bra_161`: Short code fragment that jumps to another point in the code.
- `frag_bra_162`: Short code fragment that jumps to another point in the code.
- `frag_bra_163`: Short code fragment that jumps to another point in the code.
- `frag_bra_164`: Short code fragment that jumps to another point in the code.
- `frag_nop_bra`: Short code fragment that pauses for one no-operation step (2 times in a row), then jumps to another point in the code.
- `frag_nop_bra_2`: Short code fragment that pauses for one no-operation step (2 times in a row), then jumps to another point in the code.
- `empty_routine`: Does nothing but performs an OR operation between two 16-bit registers and returns immediately.
- `empty_routine_2`: Does nothing but performs an OR operation between two 16-bit registers and returns immediately.
- `nop_padding`: Padding: two no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_2`: Padding: two no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_or_rts`: Small routine of 2 instructions that merges d5 into d4, then returns.
- `empty_routine_3`: Does nothing but performs an OR operation between two 32-bit registers and returns immediately.
- `clear_register`: Clears a 16-bit value in a global variable and returns immediately.
- `clears_register`: Clears a 16-bit value in a register and returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_3`: Padding: two no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `nop_padding_4`: Padding: two no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_or_or_rts_2`: Small routine of 3 instructions that merges d5 into d3, then merges d7 into d4, then returns.
- `nop_padding_5`: Padding: two no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `nop_padding_6`: Padding: two no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_and_or_rts`: Does nothing but execute a no-operation instruction followed by an OR operation between two word-sized values, then returns immediately.
- `tiny_or_rts_2`: Small routine of 2 instructions that merges d7 into d4, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding_7`: Padding: two no-op instructions followed by a return to the caller, so calling it has no effect on any register or memory.
- `empty_stub_15`: Empty routine: returns to the caller immediately without doing anything.
- `process_fragment_metadata_and_state`: Reads a byte from a table using an index derived from a value in a4, then updates a global variable with a byte from that table.
- `tiny_clr_rts`: Small routine of 2 instructions that clears d7, then returns.
- `frag_clr_rts_3`: Short code fragment that clears the memory at an offset from a5, then returns.
- `process_fragment_data_2`: Calculates an address using a table and d0, retrieves a byte from a table using d4, and updates a global variable with a value from a5 or d2 depending on a comparison.
- `update_global_state`: Subtracts d0 from d2, modifies d1 via shifts, calls another routine, and if certain conditions are met, updates a global variable with a value from a pointer and sets a flag at a global address.
- `frag_tst_blt_cmp_bgt_move_sub_lsr_rts`: Short code fragment that tests d2, then branches if the comparison was less, then compares d2 with 80, then branches if the comparison was greater, then copies 80 to d0, then subtracts d2 from d0, then shifts d0 right by 1, then copies 80 to d4, then subtracts d0 from d4, then returns.
- `frag_subi_clr_move_rts`: Short code fragment that subtracts 80 from d2, then clears d7, then copies d2 to a fixed memory address, then returns.
- `frag_add_cmp_bge_addi_neg_bra`: Short code fragment that adds d0 to d2, then compares d2 with 65456, then branches if the comparison was greater or equal, then adds 80 to d2, then negates d2, then jumps to another point in the code.
- `frag_cmp_ble_sub_bge_clr_rts`: Short code fragment that compares d2 with 80, then branches if the comparison was less or equal, then subtracts d0 from d2, then compares d2 with 65456, then branches if the comparison was greater or equal, then clears d4, then returns.
- `frag_move_sub_asr_rts`: Short code fragment that copies 65456 to d0, then subtracts d2 from d0, then shifts d0 right by 1, then copies 65456 to d4, then subtracts d0 from d4, then returns.
- `manage_fragment_state`: Compares d0 to a constant, uses d0 to index into a table to set d7, and either clears a value at an offset from a5 or copies values from a table into a structure at a5.
- `process_fragment_data`: If d7 is 2, it clears a value at an offset from a5 and sets d7 to 1.
- `frag_lea_neg_subq_bge_clr_bra`: Short code fragment that loads the address of a location in the code into a0, then negates d0, then subtracts 2 from d0, then branches if the comparison was greater or equal, then clears d7, then jumps to another point in the code.
- `frag_clr_bra`: Short code fragment that clears d7, then jumps to another point in the code.
- `validate_and_update_offset_values`: Compares a 16-bit value in d0 against 48, and if it is less than 48, shifts it left by 3, uses it as an offset to load a new 16-bit value into d0 from a memory location, and then performs conditional updates to d2 and d1 based on the contents of structures pointed to by a5 and a4.
- `calculate_and_update_distance_thresholds`: Calculates a signed 16-bit value using arithmetic on registers and a pointer in a0, then calls another routine.
- `update_global_state_2`: Calculates a value using d0, d1, and d2, calls another routine, and if certain conditions are met, updates a global variable and a table with specific constants.
- `frag_move_bra_9`: Short code fragment that copies d0 to the memory at a0, then copies the memory at an offset from a0 to the memory at an offset from a0, then jumps to another point in the code.
- `frag_subi_add_lea_move_bra`: Short code fragment that subtracts 48 from d0, then adds d0 to d0 (2 times in a row), then loads the address of a fixed memory address into a0, then copies an indexed entry from a0 to d0, then jumps to another point in the code.
- `process_table_entries`: Loads a value from a global variable, uses it as an index into a table starting at a constant address, and if a non-zero byte is found at offset 6, updates a global variable and calls another routine; it then enters a loop that iterates through a list of entries, updating a global variable and calling another routine for each entry where a byte at an offset is non-negative.
- `frag_move_rts_6`: Short code fragment that copies 13400 to a fixed memory address (2 times in a row), then returns.
- `update_global_and_call`: Compares a register to a global variable, updates that global if they differ, and then calls another routine multiple times.
- `update_global_and_loop`: Updates a global variable with a value from a register and calls another routine; if the register value is 11, it enters a loop that increments the global variable by 15256 and 1 respectively, calling another routine each time until the counter reaches 8.
- `calculate_scaled_offset`: Multiplies the value in a0 by 9, adds a value derived from a global variable scaled by 3, and adds a value derived from another global variable scaled by 1/8, with the final result capped by a comparison.
- `frag_move_subq_lsr_mulu_add_addi_rts`: Short code fragment that copies a0 to d2, then subtracts 2 from d2, then copies a fixed memory address to d0, then shifts d0 right by 2, then multiplies d0 by 9 (unsigned), then adds d0 to d2, then adds 18 to d2, then returns.
- `frag_bsr_bra`: Short code fragment that calls another routine, then jumps to another point in the code.
- `call_calculate_scaled_offset`: Calls a routine that calculates a scaled offset by multiplying a value in a0 by 9 and adding values derived from two global variables scaled by 3.
- `frag_bsr_bra_2`: Short code fragment that calls another routine, then jumps to another point in the code.
- `copy_data_block`: Loads two bytes from a table into d0 and d1, calls another routine, and then copies a block of bytes from a source pointer to a destination pointer calculated using d1 and a global variable.
- `frag_move_lsr_add_bsr_rts`: Short code fragment that copies a fixed memory address to d1, then shifts d1 right by 1, then copies a location in the code to d1, then shifts d0 right by 2, then adds d1 to d0, then copies a0 to a fixed memory address, then copies a1 to a fixed memory address, then calls another routine, then returns.
- `frag_move_bsr_rts`: Short code fragment that copies d0 to a fixed memory address, then copies 19 to d0, then copies a0 to a fixed memory address, then calls another routine, then returns.
- `frag_movem_bsr_move_rts_4`: Short code fragment that saves registers on the stack, then calls process_bitstream_and_update_tables, then restores saved registers from the stack, then copies 65535 to a fixed memory address, then returns.
- `frag_movem_bsr_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_bsr_move_rts`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then copies 65535 to a fixed memory address, then returns.
- `frag_movem_bsr_move_rts_3`: Short code fragment that saves registers on the stack, then calls frag_bsr_move_addq_subq_mulu_add_rts, then restores saved registers from the stack, then copies 65535 to a fixed memory address, then returns.
- `frag_movem_bsr_move_rts_2`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then copies 65535 to a fixed memory address, then returns.
- `poll_timer_flag_with_timeout`: Sets a countdown to nine and polls a status bit until it becomes set or the count runs out, then leaves the status variable at a fixed value.
- `toggle_hardware_bit_and_loop`: clears two global variables, modifies a byte at a calculated address, and enters a loop that toggles bit 1 of that address and calls another routine while checking bit 2 of a specific byte in a hardware register.
- `frag_movem_move_nop_dbra_rts`: Short code fragment that saves registers on the stack, then copies 3100 to d1, then copies a fixed memory address to d0, then pauses for one no-operation step, then loops back while counting down d0, then loops back while counting down d1, then restores saved registers from the stack, then returns.
- `frag_btst_beq_bset_bsr_bra_clr_rts`: Short code fragment that tests bit 4 of a fixed memory address, then branches if the last result was zero, then sets bit 1 of a fixed memory address, then calls another routine, then jumps to another point in the code, then clears a fixed memory address, then returns.
- `loop_until_zero`: Modifies a global variable and enters a loop that calls another routine until a value derived from the global variable reaches zero.
- `frag_movem_bclr_bset_move_nop_dbra_rts`: Short code fragment that saves registers on the stack, then clears bit 0 of a fixed memory address, then sets bit 0 of a fixed memory address, then copies 31 to d1, then copies a fixed memory address to d0, then pauses for one no-operation step, then loops back while counting down d0, then loops back while counting down d1, then restores saved registers from the stack, then returns.
- `configure_dma_and_wait_for_interrupt`: Calls three routines, configures DMA controller registers with specific control and mask values, sets a DMA destination address from a pointer, and performs bit tests on a hardware register to conditionally update a DMA control register before calling another routine and returning.
- `initialize_buffer_with_pattern`: Fills a buffer with a repeating pattern, calculates a sum from a buffer, and then fills another portion of a buffer with a repeating pattern.
- `process_bit_interleaving`: iterates through a buffer to perform bitwise masking and bit-reversal-like operations, storing the results back into a buffer.
- `frag_movem_bsr_rts_2`: Short code fragment that saves registers on the stack, then calls another routine, then restores saved registers from the stack, then returns.
- `initialize_hardware_and_verify`: Sets a counter in a global variable, configures hardware registers including DMA and custom chip registers, calls a routine via a pointer in d0, and verifies the resulting memory content against a global variable.
- `process_data_stream_and_update_counters`: Calculates two 16-bit values by dividing a sum of two longwords by 3616 and 160 respectively, storing the results in global variables.
- `frag_clr_move_rts`: Short code fragment that clears a fixed memory address, then copies d0 to a fixed memory address, then copies d1 to a fixed memory address, then copies d2 to a fixed memory address, then returns.
- `frag_bsr_move_addq_subq_mulu_add_rts`: Short code fragment that calls another routine (2 times in a row), then copies a location in the code to d0, then adds 8 to d0, then copies a fixed memory address to d1, then subtracts 1 from d1, then multiplies d1 by 512 (unsigned), then adds d1 to d0, then returns.
- `frag_movem_bsr_rts_3`: Short code fragment that saves registers on the stack, then calls another routine (2 times in a row), then restores saved registers from the stack, then returns.
- `frag_bsr_movea_rts`: Short code fragment that calls another routine, then copies a location in the code to a0, then calls another routine, then returns.
- `process_bitstream_and_update_tables`: Reads a data stream from a pointer, extracts bits to populate a table of 32 bytes, and subsequently performs a series of table updates and bitwise manipulations involving several global arrays and counters.
- `sum_table_entries`: calculates an offset from a 16-bit value in d0, reads two 16-bit values from a table at that offset, and then enters a loop to sum additional 16-bit values from the table at 4-byte intervals until the offset is exhausted.
- `decompress_and_extract_data_stream`: Decompresses a bitstream using a variable-length encoding scheme and performs lookups in tables to extract bytes, which are then stored in a destination buffer.
- `fall_through_entry_7`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_8`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_9`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_10`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_11`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_12`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_13`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_14`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_15`: Entry point part-way through straight-line code: runs two instructions (ori.w, ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_16`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_bra_165`: Short code fragment that jumps to another point in the code.
- `configure_hardware_loops`: Performs two nested loops that decrement a value from ten thousand down to zero, calls a routine, and returns.
- `frag_move_rts_7`: Short code fragment that copies 15 to a fixed memory address, then copies 1920 to a fixed memory address (2 times in a row), then returns.
- `initialize_audio_and_dma`: Calls several routines to copy data, performs a loop to transform bytes using a table, updates a global pointer, and writes specific values to the DMA and interrupt control registers.
- `dispatch_interrupt_handler_2`: checks bits in the interrupt status register, calculates an index, sets a bit in the interrupt mask register, and jumps to an address from a table before returning from an interrupt.
- `update_audio_channel_0`: Reads a byte from a pointer in a global variable, and if it is negative, updates a pointer in that same global variable using a table lookup, then writes a new address and length to the first audio channel hardware registers and returns via an exception return.
- `update_audio_channel_1`: Reads a byte from a pointer in a global variable, and if it is negative, updates a pointer in that same global variable using a table lookup, then writes a new address and length to the second audio channel hardware registers and returns via an exception return.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_or_or_rts`: Small routine of 3 instructions that merges d7 into d3, then merges d5 into d4, then returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
