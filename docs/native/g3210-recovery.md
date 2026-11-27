# Game 3210 (corsarios (19xx)(rainbird)[cr pna]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03210). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 1 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 70 of 70 named (0 author symbols), 70 of 70 described (41 by rule, 29 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 27 of 27 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 793 of 793 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 120 |
| data | 1036 |
| padding | 892 |
| unused | 899072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Corsarios (19xx)(Rainbird)[cr PNA].adf:bootblock | 1 | 21 | 1 |
| logged disk load from Corsarios (19xx)(Rainbird)[cr PNA].adf boot-block load | 1 | 181 | 20 |
| logged disk load from Corsarios (19xx)(Rainbird)[cr PNA].adf boot-block load | 1 | 591 | 49 |

## Cases handled

- corsarios (19xx)(rainbird)[cr pna]/Corsarios (19xx)(Rainbird)[cr PNA].adf: the boot block reads 41984 bytes from disk offset 844800 to an address (found statically); recovered as a unit
- corsarios (19xx)(rainbird)[cr pna]/Corsarios (19xx)(Rainbird)[cr PNA].adf: the boot block reads 65536 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target, 1 address
- corsarios (19xx)(rainbird)[cr pna]/Corsarios (19xx)(Rainbird)[cr PNA].adf: 2 logged loads cover 0 bytes of the image

## Cracktros (kept, not protection)

- none

## Copy protection found

- `initialize_display` (Corsarios (19xx)(Rainbird)[cr PNA].adf boot-block load +an address to an address (41984 bytes)): Sets up display hardware registers by configuring color and interrupt settings, then performs bit manipulation operations on data streams. Kind: disk hardware check. Pass path: CIAB_PRB is manipulated and checked, indicating a hardware-based disk integrity or track verification routine..

## Functions


### Corsarios (19xx)(Rainbird)[cr PNA].adf:bootblock

- `initialize_dma_transfer`: Sets up DMA transfer parameters for the boot block and performs two sequential I/O operations using the exec library, then jumps to a new address loaded from memory.

### logged disk load from Corsarios (19xx)(Rainbird)[cr PNA].adf boot-block load

- `initialize_display`: Sets up display hardware registers by configuring color and interrupt settings, then performs bit manipulation operations on data streams.
- `handle_bit_shift_operations`: Compares a register value with 6, branches if the comparison was less than or equal, performs bit shifting operations, handles conditional jumps based on carry flags, and executes jump instructions to different code locations.
- `frag_subq_bcc_moveq_lea_jmp`: Short code fragment that subtracts 1 from d1, then branches if the carry flag is clear, then loads 15 into d0, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_lea_bra`: Short code fragment that loads the address of a location in the code into a5, then jumps to another point in the code.
- `frag_moveq_lea_jmp`: Short code fragment that loads 3 into d0, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `process_data_stream`: Processes data by shifting bits left, handling conditional branches, moving bytes to memory, and looping through a counter.
- `frag_move_lsl_roxl_addq_dbra_cmpa_bcc_j`: Short code fragment that copies the memory just below a2 (moving a2 down) to d3, then shifts d3 left by 1, then rotates d2 left through the extend flag by 1, then adds 1 to d3, then loops back while counting down d0, then copies d2 to the memory just below a0 (moving a0 down), then loops back while counting down d1, then compares a1 with a0, then branches if the carry flag is clear, then jumps to the address in a5.
- `frag_lea_bra_2`: Short code fragment that loads the address of a location in the code into a5, then jumps to another point in the code.
- `frag_moveq_lea_jmp_2`: Short code fragment that loads 1 into d0, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `manage_data_processing_paths`: Loads specific values into registers, performs conditional comparisons with register values, handles indirect jumps to different execution paths, manages bit operations, and executes jump instructions based on computed values.
- `frag_moveq_lea_jmp_3`: Short code fragment that loads 1 into d0, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_cmpi_bcc_addq_bra`: Short code fragment that compares d1 with 2, then branches if the carry flag is clear, then adds 4 to d1, then jumps to another point in the code.
- `frag_cmpi_beq_moveq_lea_jmp`: Short code fragment that compares d1 with 3, then branches if the last result was zero, then loads 1 into d0, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `frag_addq_bra`: Short code fragment that adds 6 to d1, then jumps to another point in the code.
- `frag_moveq_lea_jmp_4`: Short code fragment that loads 7 into d0, then loads the address of a location in the code into a6, then jumps to the address in a4.
- `handle_data_processing`: Handles data processing with multiple conditional branches, manages bit operations, and jumps to an address based on a computed value.
- `copy_and_process_data_stream`: Copies data from one memory location to another while processing control characters, handling special cases for specific byte values and performing bit operations during the copy process.
- `initialize_cia_port`: Sets and clears a specific bit in a CIA register through a pointer, then writes a fixed value to a memory location at offset -228 from a3, and performs a loop of 999 iterations that toggles the same bit repeatedly.
- `frag_moveq_lsl_beq_roxl_dbra_jmp`: Short code fragment that loads 0 into d1, then shifts d3 left by 1, then branches if the last result was zero, then rotates d1 left through the extend flag by 1, then loops back while counting down d0, then jumps to the address in a6.
- `frag_move_lsl_roxl_addq_dbra_jmp`: Short code fragment that copies the memory just below a2 (moving a2 down) to d3, then copies d3 to the memory at a3, then shifts d3 left by 1, then rotates d1 left through the extend flag by 1, then adds 1 to d3, then loops back while counting down d0, then jumps to the address in a6.

### logged disk load from Corsarios (19xx)(Rainbird)[cr PNA].adf boot-block load

- `control_io_state_machine`: Copies a register value to a memory location, calls multiple routines in sequence, performs bit tests on a memory value, handles conditional branches based on comparison results, and manages state transitions through counter updates.
- `calculate_and_process_data`: Calculates a memory address offset using values from a table, then calls two routines with that address and another calculated value.
- `calculate_and_copy_data`: Calculates a memory address offset using values from a table, then calls one routine with that address and another calculated value.
- `frag_clr_cmpi_ble_rts`: Short code fragment that clears a fixed memory address, then compares a fixed memory address with 10, then branches if the comparison was less or equal, then returns.
- `loop_and_process_data`: Processes data in a loop for eight iterations, calculating memory addresses and calling a routine with those addresses.
- `copy_and_scale_data`: Copies data from one memory area to another while scaling values by a multiplication factor, processing multiple rows and columns of data with specific offsets and incrementing pointers.
- `frag_move_bra`: Short code fragment that copies 10 to d0, then copies 32 to d1, then copies 0 to d2, then jumps to another point in the code.
- `frag_move_bra_2`: Short code fragment that copies 10 to d0, then copies 32 to d1, then copies 0 to d2, then jumps to another point in the code.
- `frag_clr_cmpi_ble_rts_2`: Short code fragment that clears a fixed memory address, then compares a fixed memory address with 50, then branches if the comparison was less or equal, then returns.
- `frag_lea_move_subq_bne_rts`: Short code fragment that loads the address of a fixed memory address into a0, then loads the address of a fixed memory address into a1, then copies 64 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `frag_lea_move_subq_bne_rts_2`: Short code fragment that loads the address of a fixed memory address into a0, then copies 8000 to d0, then loads the address of a fixed memory address into a1, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `frag_move_lea_clr_subq_bne_rts`: Short code fragment that copies 10000 to d0, then loads the address of a fixed memory address into a1, then clears the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `frag_lea_move_jsr_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies 128 to the memory at an offset from a0, then copies 20992 to the memory at an offset from a0, then copies 0 to the memory at an offset from a0 (2 times in a row), then calls another routine, then copies 33152 to the memory at an offset from a0, then calls another routine, then returns.
- `frag_lea_move_rts`: Short code fragment that loads the address of a fixed memory address into a0, then copies an address to the memory at an offset from a0, then returns.
- `frag_lea_bra_3`: Short code fragment that loads the address of a fixed memory address into a0, then jumps to another point in the code.
- `frag_lea_adda_move_subq_bne_rts`: Short code fragment that loads the address of a fixed memory address into a1, then adds 384 to a1, then copies 32 to d0, then copies the memory at a0 (advancing a0) to the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `execute_conditional_routines`: Executes routines conditionally based on bit tests, performs memory operations, and branches to different execution paths depending on register values.
- `conditional_execution_loop`: Executes a series of subroutine calls based on conditional checks against memory locations, skipping some calls when certain conditions are met, then returns after completing the sequence.
- `handle_io_operations`: Performs I/O operations using the Amiga OS DoIO function, handles errors, and manages DMA channel settings before jumping to a fixed address.
- `manage_dma_and_io`: Sets up I/O request parameters for device communication, performs I/O operations, enables DMA channels, and jumps to a fixed address.
- `execute_io_request`: Loads library base address, sets up I/O request structure with specific values, calls library routine to execute I/O operation, tests a status byte, branches if the test was not equal, and calls another routine.
- `initialize_io_operation`: Clears a memory location, compares its value with 50, branches if the comparison was less than or equal, then performs I/O operations and jumps to another routine.
- `check_and_execute_io`: Clears a memory location, compares its value with 50, branches if the comparison was less than or equal, then performs I/O operations and jumps to a fixed address.
- `frag_move_movea_jsr_rts`: Short code fragment that copies 2 to a fixed memory address, then copies a fixed memory address to a6, then calls another routine, then copies 2 to the memory at an offset from a1, then copies an address to the memory at an offset from a1, then copies 67594 to the memory at an offset from a1, then copies 512 to the memory at an offset from a1, then calls another routine (2 times in a row), then returns.
- `frag_ori_lea_move_cmpm_bne_subq_rts`: Short code fragment that merges 76 into d1, then loads the address of a fixed memory address into a1, then copies 16 to d0, then compares the memory at a1 (advancing a1) with the memory at a0 (advancing a0), then branches if the last result was not zero, then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `frag_subq_bne_bra`: Short code fragment that subtracts 1 from a fixed memory address, then branches if the last result was not zero, then jumps to another point in the code.
- `frag_lea_cmpi_beq_addq_jmp`: Short code fragment that loads the address of a fixed memory address into a0, then compares the memory at an offset from a0 with an address, then branches if the last result was zero, then adds 2 to a0, then jumps to another point in the code.
- `frag_cmpi_bne_adda_jmp`: Short code fragment that compares the memory at an offset from a0 with an address, then branches if the last result was not zero, then adds 1088 to a0, then jumps to another point in the code.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `frag_movea_move_clr_jsr_rts`: Short code fragment that copies a fixed memory address to a6, then copies a fixed memory address to a1, then copies 9 to the memory at an offset from a1, then clears the memory at an offset from a1, then calls another routine, then returns.
- `frag_movea_jsr_move_rts`: Short code fragment that copies a fixed memory address to a6, then calls another routine (2 times in a row), then copies a fixed memory address to a fixed memory address, then copies 68148 to a fixed memory address, then returns.
- `frag_move_rts`: Short code fragment that copies a fixed memory address to a fixed memory address, then returns.
- `handle_dma_interrupt`: Saves all registers to stack, loads hardware register address, tests interrupt bit, branches if the bit was not set, calls routines to process data, updates a counter, checks another memory location for zero value, and executes a function pointer.
- `process_fragment_data`: Checks two memory locations for zero values and branches accordingly, updates counters, calls other routines with specific parameters, and manages conditional execution based on comparison results.
- `divide_and_store_scaled_values`: Performs division operations with scaled values from a source register and stores the results in a destination area, processing multiple 16-bit values with bit shifting and masking operations.
- `pack_and_divide_values`: Packs and divides values from a global variable into memory locations using bit shifts and division operations, with conditional execution based on another global variable.
- `accumulate_and_pack_bits`: Accumulates values by adding components from an array, then packs the resulting bits into a 16-bit value using bit shifts and masks before storing it in memory.
- `subtract_and_pack_bits`: Subtracts components from an array and packs the resulting bits into a 16-bit value using bit shifts and masks, similar to another routine but with subtraction instead of addition.
- `frag_lea_adda_move_clr_subq_bne_rts`: Short code fragment that loads the address of a fixed memory address into a1, then adds 384 to a1, then copies 32 to d0, then clears the memory at a1 (advancing a1), then subtracts 1 from d0, then branches if the last result was not zero, then returns.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `frag_jsr_move_or_rts`: Short code fragment that calls another routine, then copies d0 to a fixed memory address, then calls another routine, then merges d0 into a fixed memory address, then copies a fixed memory address to d0, then returns.
- `frag_movem_lea_move_asr_andi_btst_rts`: Short code fragment that saves registers on the stack, then loads the address of a fixed memory address into a1, then copies d0 to d1, then shifts d1 right by 3, then copies an indexed entry from a1 to d2, then masks d0 with 7, then tests bit d0 of d2, then restores saved registers from the stack, then returns.
- `bitwise_shift_and_or`: Processes data by shifting bits left and performing OR operations, building a result value from multiple iterations while checking for zero conditions.
- `frag_andi_rts`: Short code fragment that masks a fixed memory address with 64, then returns.
- `frag_jsr_movea_lea_clr_rts`: Short code fragment that calls another routine (2 times in a row), then copies a fixed memory address to a6, then loads the address of a fixed memory address into a1, then loads the address of a fixed memory address into a0, then clears d0, then clears d1, then calls another routine, then returns.
- `allocate_signal_and_initialize_task`: Allocates a signal using the operating system, initializes task-related memory fields with specific values, and stores the allocated signal number in memory.
- `frag_lea_move_rts_2`: Short code fragment that loads the address of a fixed memory address into a1, then copies 5 to the memory at an offset from a1, then copies 48 to the memory at an offset from a1, then returns.
- `read_joystick_input`: Reads joystick input from hardware registers, performs bit tests and logical operations on the input data to determine button states and returns a combined status value.
