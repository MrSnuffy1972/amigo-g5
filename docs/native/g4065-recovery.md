# Game 4065 (jet v1.01 (1988)(sublogic)[v sca]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04065). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901472 top-level bytes in 23 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 14 of 14 units identical (bytes and relocations), 808 raw-word lines |
| Functions named and described | pass, 72 of 72 named (0 author symbols), 72 of 72 described (25 by rule, 46 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 218 of 218 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 363342 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2789 of 2791 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 145648 |
| code | 9662 |
| data | 423648 |
| padding | 5586 |
| unused | 316928 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| assign | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| copy | 2 | 286 | 18 |
| cd | 2 | 294 | 5 |
| disk-validator | 1 | 386 | 1 |
| ram-handler | 2 | 475 | 31 |
| Jet v1.01 (1988)(SubLogic)[v SCA].adf:JET | 1 | 226 | 5 |
| Jet v1.01 (1988)(SubLogic)[v SCA].adf:bootblock | 1 | 13 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 29 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 13 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 1 |
| Jet v1.01 (1988)(SubLogic)[v SCA].adf:f1 (raw code file, load address unknown) | 1 | 814 | 1 |
| f1 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- reference run ref ends at a 68000 address error (odd PC an address after an address): the harness halted there, nothing after it ran (68000 HALTED: address error at odd PC an address after an address, vector 3 an address is not a handler in RAM (the 68000 double-faults and halts); run ended at frame 50)
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 14 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- jet v1.01 (1988)(sublogic)[v sca]/Jet v1.01 (1988)(SubLogic)[v SCA].adf:f1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- jet v1.01 (1988)(sublogic)[v sca]/Jet v1.01 (1988)(SubLogic)[v SCA].adf:s/f1: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 2 never-loaded region(s), 363342 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### assign

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `frag_move_bra`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### echo

- `find_max_and_prepare_buffer_2`: Walks a linked list to find the maximum value in a field, writes a header with that value, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_2`: Short code fragment that copies d1 to d0, then jumps to another point in the code.

### copy

- `find_max_and_prepare_buffer_3`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_3`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `update_counter`: Increments a counter in memory and stores a value from a register to a memory location, then compares values and branches based on the comparison result.
- `frag_addq_move_add_lsl_jmp`: Short code fragment that adds 1 to the memory at an offset from a4, then copies the memory at an offset from a4 to d1, then adds the memory at an offset from a4 to d1, then shifts d1 left by 2, then copies the memory at a1 to an indexed entry from a0, then jumps to the address in a6.
- `toggle_flag_and_compare`: Toggles a bit in memory at a specific offset, loads values from memory locations into registers, performs a comparison, and branches conditionally based on that comparison.
- `frag_addq_move_lsl_add_moveq_jmp`: Short code fragment that adds 1 to the memory at an offset from a4, then copies the memory at an offset from a4 to d1, then shifts d1 left by 2, then adds the memory at an offset from a4 to d1, then loads 0 into d2, then copies an indexed entry from a0 to d2, then copies d2 to the memory at an offset from a4, then jumps to the address in a6.
- `check_condition_and_call`: Toggles a bit in memory, compares a value with a constant, and calls another routine if the values are equal, then calls the same routine twice before jumping to another location.
- `toggle_and_branch`: Toggles a bit in a global variable and branches if the result is negative, then performs several memory operations including loading and storing long integers from offsets of a4 to a1, and calls another routine using a function pointer in a5.
- `frag_moveq_move_jmp`: Short code fragment that loads 255 into d1, then copies d1 to the memory at an offset from a4, then copies the memory at a1 to d1, then jumps to the address in a6.
- `frag_moveq_lea_jsr_move_jmp`: Short code fragment that loads 32 into d0, then loads the address of the memory at an offset from a4 into a4, then calls the routine whose address is in a5, then copies the memory at a1 to d2, then loads 20 into d0, then loads the address of the memory at an offset from a4 into a4, then calls the routine whose address is in a5, then copies the memory at a1 to d1, then jumps to the address in a6.
- `process_data_and_jump`: Loads data from memory, calls a routine, stores the loaded data back to memory, compares a value with a constant, and calls another routine if they are equal before jumping to another location.
- `check_character_code`: Loads a long integer from a pointer, compares it with several constants, and branches to different locations based on the comparison results.
- `process_data_with_flags`: Processes data using bit manipulation, performs conditional comparisons, and calls routines through function pointers based on memory values.
- `copy_and_store_bytes`: Calculates an address using shift and add operations, copies a byte from memory at that address to a register, stores the byte to another memory location, then calls a routine before branching.
- `frag_move_moveq_lea_jsr_bra`: Short code fragment that copies the memory at an offset from a4 to d2, then copies the memory at an offset from a1 to d1, then loads 24 into d0, then loads the address of the memory at an offset from a4 into a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `load_and_loop`: Loads a byte from memory using a calculated address, stores it in a structure, then continues looping while the value is non-zero.

### cd

- `find_max_and_prepare_buffer_4`: Walks a linked list to find the maximum value, writes a header, fills a buffer with sequential words, and sets up a stack frame for a subsequent call.
- `frag_move_bra_4`: Short code fragment that copies d1 to d0, then jumps to another point in the code.
- `toggle_and_update`: Toggles a bit in a data structure based on a register value, updates multiple fields in a global structure, and performs conditional operations including calls to a function pointer table.
- `update_game_state_machine`: Updates game state by reading from memory locations, performing conditional operations, and calling routines through function pointers based on data values.
- `update_game_state`: Updates a global variable with a value from another global, then performs several conditional operations based on data in memory before making indirect jumps.

### disk-validator

- `initialize_disk_validator`: Sets up disk validation parameters including counters and memory pointers, initializes data structures, and prepares for disk access operations.

### ram-handler

- `frag_ori_movem_movea_move_jsr_suba_jmp`: Short code fragment that merges 12 into d0, then saves registers on the stack, then copies d1 to a0, then copies d2 to a1, then copies d3 to d0, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then subtracts a0 from a0, then jumps to the address in a6.
- `update_object_state_17`: Updates an object's state in a table and performs conditional operations based on the state value, including clearing memory and calling another routine.
- `update_object_state_16`: Updates an object's state in a table and performs conditional operations based on the state value, including clearing memory and calling handlers.
- `update_object_state_15`: Reads an object's state from a table using a calculated index, then branches to different handlers based on the state value.
- `update_object_state`: Reads a value from a structure, uses it as an index to determine actions, and updates object state accordingly.
- `update_object_state_2`: Reads an object state value from a structure, uses it as an index to determine actions, and updates related fields in the same structure based on the state value.
- `frag_moveq_move_movea_jsr_bra`: Short code fragment that loads 0 into d3, then copies the memory at an offset from a1 to d2, then copies the memory at an offset from a1 to d1, then copies 216 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `update_object_state_3`: Reads an object's state from a table using a calculated index, updates global variables based on the state value, and performs conditional subroutine calls depending on the state.
- `update_object_state_4`: Reads an object's state from a table using a shifted index, updates global variables based on the state value, and performs conditional jumps or calls to another routine depending on the state.
- `update_object_state_5`: Reads a value from a structure, uses it as an index to access a table, updates multiple fields in the structure based on the table entry, and performs conditional calls to another routine depending on the table value.
- `update_object_state_6`: Reads an object state value from a table using a shifted index, updates a global variable with this value, and performs conditional jumps based on the state value.
- `update_object_state_7`: Updates an object's state based on a value read from a table, modifying global variables and calling subroutines conditionally depending on the state value.
- `update_object_state_8`: Reads a value from a structure, uses it as an index to access a table, updates multiple fields in the structure based on the table entry, and performs conditional calls depending on the value read.
- `update_sprite_data`: Loads sprite-related data from a table using an index from a global structure, updates multiple fields in the structure based on the loaded values, and performs conditional operations including calls to a handler routine depending on the data read.
- `update_object_state_9`: Processes an object's state based on a lookup table indexed by a value from memory, updating various object properties and calling handler routines accordingly.
- `update_object_state_10`: Reads a value from a structure, uses it as an index to access a table, and performs different operations based on the table entry.
- `update_object_state_11`: Reads a value from a structure, uses it as an index to access a table, updates multiple fields in the structure based on the table entry, and performs conditional calls depending on the value read.
- `update_object_state_12`: Reads an object state value from a structure, uses it as an index to determine actions, and updates related data based on the state.
- `handle_ram_operations`: Loads data from memory using offsets, performs shift operations, calls a routine through a function pointer, stores results to memory, and branches conditionally based on the result.
- `update_ram_handler`: Loads data from memory locations using offsets and a pointer from a4, then stores values into global variables at specific offsets.
- `update_object_state_13`: Reads a value from a table using an index from a structure, updates multiple global variables based on the read value, and performs conditional jumps to other parts of the routine depending on the value.
- `initialize_rendering`: Initializes rendering parameters and sets up graphics state before drawing operations, then branches to a rendering loop.
- `update_object_state_14`: Updates an object's state based on a value read from a table, modifying global variables and calling handler routines depending on the state value.
- `frag_move_lea_jsr_bra`: Short code fragment that copies the memory at an offset from a1 to d1, then copies 216 to d0, then loads the address of the memory at an offset from a4 into a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `frag_moveq_move_movea_jsr_bra_2`: Short code fragment that loads 255 into d2, then copies the memory at an offset from a1 to d1, then copies 216 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `frag_move_moveq_movea_jsr_bra`: Short code fragment that copies 209 to d3, then loads 0 into d2, then copies the memory at an offset from a1 to d1, then copies 216 to d0, then copies the memory at an offset from a2 to a4, then calls the routine whose address is in a5, then jumps to another point in the code.
- `process_object_states`: Reads an object's state from a table using a calculated index, then branches to different handlers based on the state value.
- `handle_state_transitions`: Reads a value from a table using a shifted index, updates global variables based on the state value, and performs conditional jumps to different handlers depending on the value.
- `handle_object_state_2`: Processes an object's state by comparing a value against multiple constants and branching to different handlers based on the match.
- `handle_object_state`: Processes an object's state by comparing a value against multiple constants and branching to different handlers based on the match.
- `frag_bra`: Short code fragment that jumps to another point in the code.

### Jet v1.01 (1988)(SubLogic)[v SCA].adf:JET

- `initialize_game`: Initializes game resources by opening system libraries, setting up screen colors, and preparing bitmap data for display.
- `frag_lea_swap_add_movea_clr_adda_rts`: Short code fragment that loads the address of a fixed memory address into a1, then swaps the halves of d1, then adds d1 to d1 (2 times in a row), then copies an indexed entry from a1 to a5, then clears d1, then swaps the halves of d1, then adds d1 to a5, then returns.
- `read_byte_from_buffer`: Reads a byte from a buffer pointed to by a global variable and advances the buffer pointer, then returns the read byte.
- `frag_btst_beq_addi_bsr_lsl_move_or_rts`: Short code fragment that tests bit 0 of a fixed memory address, then branches if the last result was zero, then adds 1 to a fixed memory address, then calls another routine, then shifts d0 left by 8, then copies d0 to the stack, then calls another routine, then merges the top of the stack into d0, then returns.
- `frag_bsr_move_swap_rts`: Short code fragment that calls another routine, then copies d0 to d4, then calls another routine, then swaps the halves of d0, then copies d4 to d0, then swaps the halves of d0, then returns.

### Jet v1.01 (1988)(SubLogic)[v SCA].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_dma_and_interrupts`: Configures DMA and interrupt registers, calls several routines through function pointers, and performs memory operations using shift and add calculations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `frag_move_rts`: Short code fragment that copies 34817 to a fixed memory address, then returns.
- `frag_move_divu_subq_rts`: Short code fragment that copies 1200 to d0, then copies an address to d1, then divides d1 by d0 (unsigned), then subtracts 1 from d1, then copies d1 to a fixed memory address, then returns.
- `frag_movea_move_jsr_rts`: Short code fragment that copies a1 to a6, then copies a fixed memory address to d0, then copies 6144 to a fixed memory address, then calls another routine, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_copy`: Loads a 32-bit value from a memory location using a pc-relative addressing mode and copies it to another memory location through a pointer in register a0.

### Jet v1.01 (1988)(SubLogic)[v SCA].adf:f1 (raw code file, load address unknown)

- `initialize_structure`: Initializes a data structure by setting multiple fields to zero, including a 16-bit counter and several 32-bit values.
