# Game 4641 (Paperboy 2): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04641). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902944 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 97 of 97 named (0 author symbols), 97 of 97 described (56 by rule, 41 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 11 of 11 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 62464 bytes, 72 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1318 of 1318 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 29602 |
| code | 5230 |
| data | 861658 |
| padding | 310 |
| unused | 6144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Paperboy 2.adf:bootblock | 1 | 37 | 1 |
| logged disk load from Paperboy 2.adf boot-block load | 1 | 99 | 2 |
| logged disk load from Paperboy 2.adf boot-block load | 1 | 107 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 226 | 20 |
| Paperboy 2.adf bytes 11264-12800 (code never loaded by a reference run) | 1 | 221 | 12 |
| Paperboy 2.adf bytes 14336-75264 (code never loaded by a reference run) | 1 | 628 | 60 |

## Cases handled

- Paperboy 2/Paperboy 2.adf: custom-format (non-DOS) disk, 1744 non-empty blocks: matched against track-loaded segments
- Paperboy 2/Paperboy 2.adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- Paperboy 2/Paperboy 2.adf: the boot block reads 24064 bytes from disk offset 877056 to an address (found statically); recovered as a unit
- runtime segment an address: 11 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Paperboy 2/Paperboy 2.adf: bytes 11264-12800 look like code no run loaded: recovered position-independently (load address unknown)
- Paperboy 2/Paperboy 2.adf: bytes 14336-75264 look like code no run loaded: recovered position-independently (load address unknown)
- Paperboy 2/Paperboy 2.adf: 4 logged loads cover 92160 bytes of the image
- Paperboy 2/Paperboy 2.adf: 3 track-loaded blocks found in recovered runtime segments and classified through them
- Paperboy 2/Paperboy 2.adf: blocks 159-1712 (bytes 81408-877056) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.66)
- 1 never-loaded region(s), 60928 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 1 packed track region(s), 4608 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Paperboy 2.adf:bootblock

- `initialize_io_request`: Sets up an I/O request structure with command code 2 and various data values, then calls the OS I/O handler twice with different parameters before returning.

### logged disk load from Paperboy 2.adf boot-block load

- `bit_rotation_and_store`: Rotates bits in a value and stores the result to a memory location while updating a status register.
- `bit_shift_loop`: Performs bit shifting operations in a loop that processes input values and calls another routine when a condition is met.

### logged disk load from Paperboy 2.adf boot-block load

- `bit_rotation_and_store_v2`: Performs bit rotation operations on data and stores results to memory locations with status updates.
- `bit_shift_loop_v2`: Processes data through bit manipulation and looping operations, calling another routine under specific conditions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `compare_and_advance`: Compares a value from memory with a parameter, advances a pointer when they don't match, and recursively calls another routine when the value equals -1.
- `setup_linked_list_node`: Initializes a linked list node structure by copying data between pointers, setting various fields including a timestamp, and calling library functions to manage the node.
- `update_animation_state`: Updates animation state variables based on input parameters, performs bit manipulation operations, and modifies global flags related to animation processing.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `configure_render_target`: Sets up rendering target parameters including a command byte, memory addresses, and various data fields, then calls library functions based on a comparison result.
- `frag_movea_jsr_move_adda_rts`: Short code fragment that copies the memory at an offset from a4 to a6, then calls another routine, then copies the memory at an offset from a7 to d0, then adds 8 to a7, then returns.
- `frag_cmpi_beq_clr_move_movea_jsr_rts`: Short code fragment that compares the memory at an offset from a7 with 0, then branches if the last result was zero, then clears d0, then copies the memory at an offset from a7 to d0, then copies the memory at an offset from a7 to a1, then copies the memory at an offset from a4 to a6, then calls another routine, then returns.
- `frag_movea_move_lsl_or_jsr_rts`: Short code fragment that copies the memory at an offset from a4 to a6, then copies an address to d1, then copies the memory at an offset from a7 to d2, then shifts d2 left by 1, then merges d2 into d1, then calls another routine, then returns.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `frag_movea_move_clr_lsl_jsr_rts`: Short code fragment that copies the memory at an offset from a4 to a6, then copies the memory at an offset from a7 to d0, then clears d1, then copies the memory at an offset from a7 to d1, then shifts d1 left by 1, then calls another routine, then returns.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `frag_movea_cmpi_beq_jsr_tst_moveq_rts`: Short code fragment that copies the memory at an offset from a4 to a0, then compares the memory at an offset from a0 with 0, then branches if the last result was zero, then copies the memory at an offset from a4 to a1, then copies the memory at an offset from a4 to a6, then calls another routine, then tests d0, then branches if the last result was zero, then loads 1 into d0, then returns.
- `tiny_move_rts`: Small routine of 2 instructions that copies 1 to d0, then returns.
- `clear_flag_and_call`: Checks a byte at offset 56 of a structure, and if it is non-zero, calls two library routines and then clears that byte.
- `frag_move_bne_bra`: Short code fragment that copies the memory at an offset from a1 to d0, then branches if the last result was not zero, then copies 8 to a fixed memory address, then copies 1024 to a fixed memory address, then jumps to another point in the code.
- `frag_subq_move_bne_rts`: Short code fragment that subtracts 1 from d0, then copies d0 to the memory at an offset from a1, then branches if the last result was not zero, then copies 1 to a fixed memory address, then copies 1024 to a fixed memory address, then returns.
- `frag_suba_cmpi_bne_beq_adda_rts`: Short code fragment that subtracts 6 from a7, then compares the memory at an offset from a7 with 3, then branches if the last result was not zero, then compares the memory at an offset from a4 with 0, then branches if the last result was zero, then adds 6 to a7, then returns.
- `setup_audio_dma`: Configures audio DMA settings by writing values to hardware registers, updates a table entry, waits for a period, and enables specific interrupt and DMA channels.
- `frag_clr_move_lsl_rts`: Short code fragment that clears d0, then copies the memory at an offset from a7 to d0, then copies 1 to d1, then shifts d1 left by d0, then copies d1 to a fixed memory address, then returns.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.

### Paperboy 2.adf bytes 11264-12800 (code never loaded by a reference run)

- `copy_pattern_data`: Copies byte data from source to multiple destination locations using indexed addressing, performs boundary checks, and advances pointers in a loop.
- `validate_and_copy_data`: Validates a value against a threshold, calls the debug function when exceeded, then copies byte data to multiple destinations using indexed addressing in a loop.
- `coordinate_validation`: Validates coordinate boundaries and performs memory copying operations when coordinates exceed certain limits.
- `swap_pattern_data`: Swaps byte values between two memory locations using indexed addressing, performs boundary checks, and advances pointers in nested loops to process multiple rows.
- `update_flag_byte`: Updates a byte value in memory and sets two other bytes to one when the input does not match the current value, then returns.
- `bitwise_memory_operations`: Performs bitwise AND and OR operations on memory locations based on bit patterns from input data, with a loop that processes multiple iterations.
- `frag_cmpi_bcs_subi_move_andi_rts`: Short code fragment that compares the memory at an offset from a7 with 7680, then branches if the carry flag is set, then subtracts 7680 from the memory at an offset from a7, then copies the memory at an offset from a7 to d0, then copies d0 to the memory at an offset from a4, then copies the memory at an offset from a7 to d0, then masks d0 with 7, then copies d0 to the memory at an offset from a4, then returns.
- `boundary_check_and_copy`: Checks boundary conditions and performs memory copying operations when specific thresholds are exceeded.
- `frag_bchg_movea_move_jsr_rts`: Short code fragment that flips bit d3 of the memory at an offset from a2, then copies the memory at an offset from a4 to a6, then copies the memory at an offset from a7 to d1, then calls another routine, then returns.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.

### Paperboy 2.adf bytes 14336-75264 (code never loaded by a reference run)

- `initialize_structure_fields`: Sets up various fields in a data structure including width, pointers, and coordinates, then makes two operating system calls before returning.
- `setup_and_call_os_function`: Configures fields in a data structure with width, pointers, and coordinates, then calls an operating system function before returning.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_492`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `frag_movea_lea_move_clr_rts`: Short code fragment that copies the memory at an offset from a7 to a0, then loads the address of the memory at an offset from a0 into a1, then copies a1 to the memory at a0, then clears the memory at an offset from a0, then copies a0 to the memory at an offset from a0, then returns.
- `conditional_os_calls`: Calls one operating system function with parameters, checks the result, and if non-zero calls another operating system function before returning.
- `frag_move_clr_adda_rts`: Short code fragment that copies d6 to the memory at an offset from a1, then clears the memory at an offset from a4, then adds 2 to a7, then returns.
- `copy_and_increment`: Copies bytes from a source location to a destination table, increments an index, and resets it when reaching a limit.
- `fill_and_copy_bytes`: Fills a memory region with spaces, then copies six bytes from a parameter to a specific offset in the same region.
- `byte_comparison`: Compares bytes from two memory locations and returns a value based on the comparison results.
- `compare_and_branch`: Loads a byte from memory, compares it with another value from a different memory location, and branches based on the comparison result.
- `frag_cmpi_beq_addq_bra`: Short code fragment that compares the memory at an offset from a7 with 5, then branches if the last result was zero, then adds 1 to the memory at an offset from a7, then jumps to another point in the code.
- `tiny_move_adda_rts`: Small routine of 3 instructions that copies 1 to d0, then adds 2 to a7, then returns.
- `frag_movea_move_clr_adda_rts`: Short code fragment that copies the memory at an offset from a4 to a0, then copies 208 to the memory at an offset from a0, then copies the memory at an offset from a4 to a0, then clears the memory at an offset from a0, then adds 2 to a7, then returns.
- `coordinate_normalization`: Calculates coordinate differences, applies negation when needed, and scales values while setting flags in a structure.
- `negate_and_store_coordinates`: Negates two word values if corresponding flags are non-zero, then stores the negated values to specific offsets in a structure.
- `load_and_process_coordinates`: Loads coordinate values from a global structure, processes them based on conditional flags, and prepares parameters for subsequent routine calls.
- `frag_move_adda_rts`: Short code fragment that copies 1 to the memory at an offset from a4, then copies the memory at an offset from a7 to d0, then adds 18 to a7, then returns.
- `frag_cmpi_bcc_clr_move_lea_andi_rts`: Short code fragment that compares the memory at an offset from a7 with 52, then branches if the carry flag is clear, then clears d0, then copies the memory at an offset from a7 to d0, then loads the address of the memory at an offset from a4 into a0, then copies an indexed entry from a0 to d0, then masks d0 with 63, then returns.
- `frag_move_subi_rts`: Short code fragment that copies the memory at an offset from a7 to d0, then subtracts 52 from d0, then returns.
- `initialize_table_entries`: Initializes a table with zero values and sets an initial value at a specific offset before returning.
- `update_status_flags`: Sets a word value at a calculated offset in memory and performs bitwise OR operation on a byte pointer, then clears two bytes in a table starting at an offset from a global variable.
- `frag_movea_move_rts`: Short code fragment that copies the memory at an offset from a7 to a1, then copies the memory at an offset from a7 to a2, then copies the memory at an offset from a1 to a3, then copies a1 to the memory at a2, then copies a3 to the memory at an offset from a2, then copies a2 to the memory at a3, then copies a2 to the memory at an offset from a1, then returns.
- `frag_movea_move_rts_2`: Short code fragment that copies the memory at an offset from a7 to a0, then copies a0 to the memory at a0, then copies the memory at an offset from a7 to a0, then copies a0 to the memory at an offset from a0, then returns.
- `frag_movea_cmpa_beq_move_rts`: Short code fragment that copies the memory at an offset from a7 to a1, then copies the memory at a1 to a2, then compares a2 with a1, then branches if the last result was zero, then copies the memory at an offset from a1 to a0, then copies a2 to the memory at a0, then copies a0 to the memory at an offset from a2, then copies a1 to the memory at a1, then returns.
- `rearrange_linked_list`: Reorganizes elements in a linked list structure by updating pointers and swapping node positions based on comparison of short integer values.
- `frag_ori_move_adda_rts`: Short code fragment that merges 14 into d4, then copies the memory at an offset from a7 to d0, then copies d0 to the memory at an offset from a4, then adds 16 to a7, then returns.
- `update_memory_pointer`: Calculates a memory address by adding a word value to a pointer, compares it with another address, and updates the first pointer if they match.
- `frag_movea_move_adda_rts`: Short code fragment that copies the memory at an offset from a7 to a0, then copies the memory at an offset from a7 to a1, then copies a0 to the memory at a1, then adds 16 to a7, then returns.
- `frag_ori_movea_clr_adda_rts`: Short code fragment that merges 14 into d2, then copies the memory at an offset from a7 to a0, then clears the memory at a0, then clears d0, then adds 10 to a7, then returns.
- `store_word_and_update_pointers`: Copies a word from one memory location to another offset, then updates several pointers in a data structure based on values from the stack.
- `frag_move_ext_asl_movea_add_rts`: Short code fragment that copies the memory at an offset from a7 to d0, then sign-extends d0, then shifts d0 left by 8, then copies the memory at an offset from a7 to a0, then adds d0 to the memory at a0, then returns.
- `frag_clr_move_mulu_rts`: Short code fragment that clears d0, then clears d1, then copies the memory at an offset from a7 to d0, then copies the memory at an offset from a7 to d1, then multiplies d0 by d1 (unsigned), then returns.
- `modify_byte_array`: Updates a byte array by setting specific bits and modifying elements based on index calculations from global variables.
- `initialize_and_increment_counter`: Initializes two bytes in memory to specific values, then increments a counter in a loop until it reaches a maximum threshold.
- `clear_and_update_byte_array`: Clears and updates elements in a byte array using calculated indices from stack values and global variables.
- `frag_move_adda_rts_2`: Short code fragment that copies the memory at an offset from a7 to d0, then copies d0 to the memory at an offset from a4, then adds 4 to a7, then returns.
- `frag_sub_adda_addq_rts`: Short code fragment that subtracts the memory just below a0 (moving a0 down) from d1, then adds 8 to a7, then adds 1 to the memory at an offset from a4, then adds 8 to a7, then returns.
- `frag_adda_clr_rts`: Short code fragment that adds 2 to a7, then clears the memory at an offset from a4 (3 times in a row), then returns.
- `frag_ori_move_adda_rts_2`: Short code fragment that merges 252 into d1, then merges 117 into d2, then copies 1 to d0, then adds 2 to a7, then returns.
- `set_flag_and_call_routine`: Sets a flag byte to one if it's zero, stores a value in memory, then calls another routine with parameters from the stack.
- `flag_increment_and_call`: Increments a counter and calls another routine if the counter is non-zero, otherwise returns immediately.
- `setup_data_structure`: Initializes fields in a data structure including a word value and pointers to function addresses, then sets up a jump table entry.
- `frag_move_lea_movea_bsr_adda_rts`: Short code fragment that copies a0 to the memory at an offset from a1, then loads the address of the memory at an offset from a4 into a0, then copies a0 to the stack, then copies the memory at an offset from a7 to a0, then copies a0 to the stack, then calls another routine, then adds 8 to a7, then returns.
- `frag_movea_clr_adda_rts`: Short code fragment that copies the memory at an offset from a7 to a0, then clears the memory at a0, then clears d0, then adds 4 to a7, then returns.
- `validate_and_copy_pointer`: Compares an address with a stored value, updates a pointer if they match, then calls another routine with the updated pointer as parameter.
- `frag_bra`: Short code fragment that jumps to another point in the code.
- `frag_movea_rts`: Short code fragment that copies the memory at an offset from a4 to a7, then returns.
- `frag_move_clr_lea_adda_rts`: Short code fragment that copies the memory at an offset from a7 to d0, then clears d1, then copies the memory at an offset from a7 to d1, then loads the address of the memory at an offset from a4 into a0, then copies d0 to an indexed entry from a0, then adds 32 to a7, then returns.
- `frag_clr_move_lea_adda_rts`: Short code fragment that clears d0, then copies the memory at an offset from a7 to d0, then loads the address of the memory at an offset from a4 into a0, then clears an indexed entry from a0, then adds 10 to a7, then returns.
- `initialize_structure`: Sets up a data structure with maximum values, flags, and copies input parameters to specific offsets while clearing a counter.
- `frag_move_movea_rts`: Short code fragment that copies the memory at an offset from a7 to d0, then copies the memory at an offset from a7 to a0, then copies d0 to the memory at an offset from a0, then returns.
- `frag_move_swap_clr_lsr_add_rts`: Short code fragment that copies the memory at an offset from a7 to d0, then swaps the halves of d0, then clears d0, then shifts d0 right by 2, then adds the memory at an offset from a7 to d0, then returns.
- `frag_movea_clr_adda_rts_2`: Short code fragment that copies the memory at an offset from a7 to a0, then clears the memory at an offset from a0, then adds 10 to a7, then returns.
- `frag_or_adda_movea_clr_rts`: Short code fragment that merges d3 into the memory at a4 (advancing a4), then adds 12 to a7, then copies the memory at an offset from a7 to a0, then clears the memory at an offset from a0, then adds 4 to a7, then returns.
- `frag_ori_adda_rts`: Short code fragment that merges 0 into an indexed entry from a0, then adds 12 to a7, then adds 4 to a7, then returns.
- `frag_move_add_clr_adda_rts`: Short code fragment that copies the memory at an offset from a4 to d0, then adds the memory at an offset from a7 to d0, then copies d0 to the memory at an offset from a4, then copies the memory at an offset from a4 to d0, then copies d0 to the memory at an offset from a4, then copies 1 to the memory at an offset from a4, then clears the memory at an offset from a4, then adds 4 to a7, then returns.
- `frag_cmpi_beq_subq_adda_rts`: Short code fragment that compares the memory at an offset from a4 with 0, then branches if the last result was zero, then subtracts 1 from the memory at an offset from a4, then adds 20 to a7, then returns.
- `frag_movea_clr_adda_rts_3`: Short code fragment that copies the memory at an offset from a7 to a0, then clears the memory at a0, then clears d0, then adds 14 to a7, then returns.
