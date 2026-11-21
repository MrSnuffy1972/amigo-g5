# Game 4482 (murder (1990)(u. s. gold)[cr john]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04482). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (912848 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 69 of 69 named (0 author symbols), 69 of 69 described (37 by rule, 32 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 67 of 67 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 886 of 886 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 6245 |
| code | 3510 |
| data | 5495 |
| padding | 62 |
| unused | 897536 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Murder (1990)(U. S. Gold)[cr John].adf:bootblock | 1 | 84 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 802 | 65 |

## Cases handled

- runtime segment an address: 67 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_system` (bootblock): Initializes system registers by setting specific bits, sets up the stack pointer, copies data from program memory to another location in chunks, and jumps to a final address. Kind: disk hardware check. Pass path: ADKCON register setup and disk drive initialization continues normally.

## Functions


### Murder (1990)(U. S. Gold)[cr John].adf:bootblock

- `initialize_boot_system`: Initializes the boot system by allocating memory, setting up I/O requests, copying data between memory regions, and configuring system registers before jumping to a setup routine.
- `initialize_system`: Initializes system registers by setting specific bits, sets up the stack pointer, copies data from program memory to another location in chunks, and jumps to a final address.
- `copy_and_wait`: Copies data from one memory location to another in chunks of four bytes, then waits for a specific hardware register to reach a certain state before returning.
- `setup_io_request`: Initializes an I/O request structure with specific command, data pointers, and status fields before calling the operating system's DoIO function.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `handle_interrupts`: Processes interrupt requests from hardware by checking bits in the interrupt status and enable registers, clearing specific interrupt flags based on conditions, and calling different routines depending on a global variable's value.
- `tiny_add_rts`: Small routine of 2 instructions that adds d2 to -(a6), then returns.
- `update_display_character`: Updates a display character by reading from a lookup table, adding a constant value, storing the result in memory, and calling a graphics routine with specific parameters.
- `frag_movea_moveq_bra`: Short code fragment that copies an address to a3, then loads 12 into d2, then loads 84 into d3, then loads 1 into d4, then jumps to another point in the code.
- `frag_movea_moveq_bra_2`: Short code fragment that copies an address to a3, then loads 12 into d2, then loads 84 into d3, then loads 255 into d4, then jumps to another point in the code.
- `bitwise_or_operations`: Performs bitwise OR operations between bytes from two memory locations and stores the results at specific offsets in a third location, iterating through multiple rows and columns.
- `process_table_entries`: Processes a table of entries by calculating an offset based on input data, then iterates through the table entries to call another routine and update a counter.
- `wait_for_dma`: Waits for a specific DMA bit to clear in a global variable, then calls another routine and returns.
- `or_operations_with_loop`: Executes a nested loop performing bitwise OR operations between words from two memory locations and stores results at specific offsets in a third location, repeating this process for multiple iterations.
- `frag_lea_move_mulu_adda_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies the memory at an offset from a1 to d0, then multiplies d0 by 64 (unsigned), then adds d0 to a0, then copies the memory at an offset from a1 to d0, then multiplies d0 by 4 (unsigned), then adds d0 to a0, then copies a0 to a fixed memory address, then returns.
- `dma_operations_with_loop`: Repeatedly waits for DMA readiness, calls an operation routine with parameters derived from fixed addresses and registers, then updates memory pointers and repeats until a counter reaches zero.
- `frag_movea_move_clr_dbra_rts`: Short code fragment that copies an address to a0, then copies 2652 to d7, then clears the memory at a0 (advancing a0), then loops back while counting down d7, then returns.
- `frag_addx_bne_move_eori_rts`: Short code fragment that adds d2 and the extend flag to d7, then branches if the last result was not zero, then copies a fixed memory address to d0, then copies a fixed memory address to a fixed memory address, then copies d0 to a fixed memory address, then exclusive-ors 128 into a fixed memory address, then returns.
- `search_memory_blocks`: Searches memory blocks for non-zero bytes using nested loops, sets a global flag to 1 when found, and otherwise clears the flag before returning.
- `memory_search_and_write`: Searches through memory locations for non-zero bytes using nested loops, and if found, sets a global flag to 1 and returns; otherwise, it writes values to memory locations in a similar nested pattern.
- `frag_move_movem_rts`: Short code fragment that copies d6 to d1, then copies 1 to a fixed memory address, then restores saved registers from the stack, then returns.
- `frag_move_dbra_lea_rts`: Short code fragment that copies 0 to the memory at a0 (advancing a0), then loops back while counting down d7, then copies 1023 to d7, then loads the address of a fixed memory address into a0, then copies 0 to the memory at a0 (advancing a0), then loops back while counting down d7, then returns.
- `frag_moveq_lea_add_move_dbra_rts`: Short code fragment that loads 0 into d0, then loads the address of a fixed memory address into a1, then loads 56 into d7, then adds the memory at an offset from a1 to d0, then copies d0 to the memory at an offset from a1, then loads the address of the memory at an offset from a1 into a1, then loops back while counting down d7, then returns.
- `update_game_flags`: Reads two 16-bit values from global memory, adds immediate constants to them, then calls a routine twice while modifying a global variable with bitwise OR operations, and finally performs conditional checks on global bytes before calling another routine.
- `load_and_modify_flags`: Reads two 16-bit values from global memory locations, adds immediate constants to them, then calls a routine twice while modifying a global variable with an OR operation between the calls.
- `frag_move_bsr_rts`: Short code fragment that copies an address to a fixed memory address, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then calls another routine, then returns.
- `frag_move_bsr_rts_2`: Short code fragment that copies an address to a fixed memory address, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then calls another routine, then returns.
- `frag_move_adda_bsr_rts`: Short code fragment that copies a1 to a fixed memory address, then adds 108 to a1, then copies a1 to a fixed memory address, then adds 108 to a1, then copies a1 to a fixed memory address, then adds 108 to a1, then copies a1 to a fixed memory address, then calls another routine, then returns.
- `frag_move_bsr_rts_3`: Short code fragment that copies an address to a fixed memory address, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then copies an address to a fixed memory address, then calls another routine, then returns.
- `frag_move_adda_bsr_rts_2`: Short code fragment that copies a1 to a fixed memory address, then adds 108 to a1, then copies a1 to a fixed memory address, then adds 108 to a1, then copies a1 to a fixed memory address, then adds 108 to a1, then copies a1 to a fixed memory address, then calls another routine, then returns.
- `frag_move_bsr_rts_4`: Short code fragment that copies an address to a fixed memory address, then copies an address to a fixed memory address, then calls another routine, then returns.
- `frag_move_bsr_rts_5`: Short code fragment that copies 59884 to a fixed memory address (2 times in a row), then calls another routine, then returns.
- `copy_word_values`: Copies word-sized values from a global variable to specific offsets within a structure pointed to by register A6, writing twelve separate words to consecutive memory locations.
- `update_flags_and_values`: Modifies two 16-bit values pointed to by a0, combining input registers d0 and d1 through bitwise operations and shifts, while updating specific bit positions in the target memory locations.
- `memory_or_operations`: Calculates memory addresses based on input parameters, performs bitwise OR operations between words from three different memory locations, and stores the results sequentially in another memory area.
- `process_character_data`: Processes character input by validating it against a range, converting it through multiple operations, and applying bitwise manipulations to memory locations based on bit tests.
- `frag_moveq_move_add_asl_lea_movea_rts`: Short code fragment that loads 0 into d1, then copies a fixed memory address to d1, then adds d1 to d1, then copies d1 to d2, then shifts d2 left by 1, then loads the address of a fixed memory address into a1, then copies an indexed entry from a1 to a1, then returns.
- `parse_special_sequences`: Parses input sequences looking for specific markers, handles special character processing, and manages a lookup table for character transformations.
- `parse_input_sequence`: Parses input character sequences, handles special markers like '^', processes matching entries through a lookup table, and calls corresponding routines while managing a loop.
- `calculate_string_value`: Calculates a cumulative value from string characters by indexing into memory tables, performs bounds checking against predefined limits, and calls another routine when conditions are met.
- `write_pattern_data`: Writes pattern data to multiple memory locations using indexed addressing, performs bit shifting operations, and manages loops for repetitive writes based on calculated values.
- `frag_move_asl_lea_addq_rts`: Short code fragment that copies a fixed memory address to d1, then shifts d1 left by 1, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a2, then loads the address of a fixed memory address into a3, then copies an indexed entry from a3 to an indexed entry from a1, then adds 8 to an indexed entry from a2, then adds 1 to a0, then returns.
- `frag_moveq_move_subi_mulu_addi_rts`: Short code fragment that loads 0 into d1, then copies the memory at a0 (advancing a0) to d1, then subtracts 48 from d1, then multiplies d1 by 1080 (unsigned), then adds an address to d1, then copies d1 to a fixed memory address, then returns.
- `frag_move_asl_lea_clr_addq_rts`: Short code fragment that copies a fixed memory address to d1, then shifts d1 left by 1, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a2, then loads the address of a fixed memory address into a3, then copies an indexed entry from a3 to an indexed entry from a1, then clears an indexed entry from a2, then adds 1 to a0, then returns.
- `frag_move_asl_lea_subi_ext_rts`: Short code fragment that copies a fixed memory address to d1, then shifts d1 left by 1, then loads the address of a fixed memory address into a1, then copies the memory at a0 (advancing a0) to d2, then subtracts 48 from d2, then sign-extends d2, then copies d2 to an indexed entry from a1, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_subi_rts`: Short code fragment that copies the memory at a0 (advancing a0) to d1, then subtracts 48 from d1, then copies d1 to a fixed memory address, then returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `frag_move_subi_ext_rts`: Short code fragment that copies the memory at a0 (advancing a0) to d0, then subtracts 48 from d0, then sign-extends d0, then copies d0 to a fixed memory address, then returns.
- `update_coordinates`: Updates coordinate values in two memory tables by reading from input, converting characters to numeric values, performing bit shifts, and storing results at indexed locations.
- `frag_moveq_bra`: Short code fragment that loads 0 into d1, then jumps to another point in the code.
- `frag_moveq_bra_2`: Short code fragment that loads 1 into d1, then jumps to another point in the code.
- `frag_moveq_bra_3`: Short code fragment that loads 2 into d1, then jumps to another point in the code.
- `frag_moveq_bra_4`: Short code fragment that loads 3 into d1, then jumps to another point in the code.
- `compare_and_execute`: Compares a character value with data at a specific memory location, branches to another routine when values don't match, and calls a graphics routine with register values.
- `call_function_offset_1`: Loads a value from a global variable, performs arithmetic operations on it, uses the result as an address offset, and calls a function with data from memory.
- `call_function_offset_2`: Loads a value from a global variable, performs arithmetic operations on it, uses the result as an address offset, and calls a function with data from memory.
- `load_and_process_data`: Reads a byte from a table at offset 99351, uses it as an index to access another table at offset 51935, then reads a byte from that second table and passes it to another routine for processing.
- `validate_character_data`: Validates character data by reading from memory locations, performing arithmetic operations to calculate addresses, and calling another routine when a condition is met.
- `frag_movea_bsr_rts`: Short code fragment that copies 60331 to a0, then calls another routine, then copies the top of the stack to a0, then returns.
- `frag_move_movea_jsr_rts`: Short code fragment that copies a fixed memory address to d0, then copies 55330 to a1, then copies d0 to d1, then calls another routine, then returns.
- `frag_move_movea_jsr_rts_2`: Short code fragment that copies a fixed memory address to d0, then copies 57784 to a1, then copies d0 to d1, then calls another routine, then returns.
- `lookup_character`: Reads a byte from a global variable, shifts it right by four bits, uses the result as an index to access a table, then calls another routine with the table entry and the original byte's lower four bits.
- `frag_movem_move_subq_bpl_rts`: Short code fragment that saves registers on the stack, then copies the memory at a1 (advancing a1) to d0, then subtracts 1 from d1, then branches if the last result was not negative, then restores saved registers from the stack, then returns.
- `frag_move_bra`: Short code fragment that copies d0 to the memory at a6 (advancing a6), then jumps to another point in the code.
- `frag_movem_andi_beq_clr_move_adda_subq`: Short code fragment that saves registers on the stack, then masks d1 with 255, then branches if the last result was zero, then clears d0, then copies the memory at a1 (advancing a1) to d0, then adds d0 to a1, then subtracts 1 from d1, then jumps to another point in the code.
- `frag_move_bsr_movem_rts`: Short code fragment that copies the memory at a1 (advancing a1) to d1, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_mulu_adda_andi_move_asr_rts`: Short code fragment that multiplies d3 by 40 (unsigned), then adds d3 to a1, then masks d2 with 65535, then copies d2 to d4, then masks d4 with 15, then shifts d2 right by 3, then masks d2 with 254, then adds d2 to a1, then returns.
- `update_display_buffer`: Clears three longword locations in memory, reads a word from a hardware register, masks it to 5 bits, uses that value as a loop counter, and performs byte operations on data from a table to update display buffer locations.
- `empty_routine`: Contains no executable instructions and returns immediately.
