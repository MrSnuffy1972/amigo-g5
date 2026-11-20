# Game 2725 (archon 2 - adept (1986)(electronic arts)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02725). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905776 top-level bytes in 3 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 6 raw-word lines |
| Functions named and described | pass, 34 of 34 named (0 author symbols), 34 of 34 described (12 by rule, 16 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 104 of 104 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 10752 bytes, 10 functions |
| Independent decoder cross-check (Ghidra against capstone) | 981 of 982 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3093 |
| code | 4052 |
| data | 421474 |
| padding | 485 |
| unused | 476672 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Archon 2 - Adept (1986)(Electronic Arts).adf:bootblock | 1 | 50 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 392 | 22 |
| Archon 2 - Adept (1986)(Electronic Arts).adf bytes 885760-887808 (code never loaded by a reference run) | 1 | 25 | 2 |
| Archon 2 - Adept (1986)(Electronic Arts).adf bytes 889856-890368 (code never loaded by a reference run) | 1 | 2 | 1 |
| Archon 2 - Adept (1986)(Electronic Arts).adf bytes 890880-891392 (code never loaded by a reference run) | 1 | 0 | 0 |
| Archon 2 - Adept (1986)(Electronic Arts).adf bytes 891904-899584 (code never loaded by a reference run) | 1 | 513 | 7 |

## Cases handled

- runtime segment an address: 104 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- archon 2 - adept (1986)(electronic arts)/Archon 2 - Adept (1986)(Electronic Arts).adf: bytes 885760-887808 look like code no run loaded: recovered position-independently (load address unknown)
- archon 2 - adept (1986)(electronic arts)/Archon 2 - Adept (1986)(Electronic Arts).adf: bytes 889856-890368 look like code no run loaded: recovered position-independently (load address unknown)
- archon 2 - adept (1986)(electronic arts)/Archon 2 - Adept (1986)(Electronic Arts).adf: bytes 890880-891392 look like code no run loaded: recovered position-independently (load address unknown)
- archon 2 - adept (1986)(electronic arts)/Archon 2 - Adept (1986)(Electronic Arts).adf: bytes 891904-899584 look like code no run loaded: recovered position-independently (load address unknown)
- archon 2 - adept (1986)(electronic arts)/Archon 2 - Adept (1986)(Electronic Arts).adf: 4 logged loads cover 10752 bytes of the image
- archon 2 - adept (1986)(electronic arts)/Archon 2 - Adept (1986)(Electronic Arts).adf: 6 track-loaded blocks found in recovered runtime segments and classified through them
- archon 2 - adept (1986)(electronic arts)/Archon 2 - Adept (1986)(Electronic Arts).adf: blocks 22-447 (bytes 11264-229376) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.66)
- archon 2 - adept (1986)(electronic arts)/Archon 2 - Adept (1986)(Electronic Arts).adf: blocks 881-881 (bytes 451072-451584) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.07)
- archon 2 - adept (1986)(electronic arts)/Archon 2 - Adept (1986)(Electronic Arts).adf: blocks 884-893 (bytes 452608-457728) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.21)
- 3 never-loaded region(s), 3072 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 183808 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Archon 2 - Adept (1986)(Electronic Arts).adf:bootblock

- `initialize_boot_block`: Sets up initial boot block parameters including memory allocation and status flags, then calls system routines for memory management and buffer handling.
- `frag_movem_move_dbra_rts`: Short code fragment that saves registers on the stack, then copies 2048 to d0, then copies 256 to d1, then copies the memory at an offset from a7 to a fixed memory address, then loops back while counting down d1, then loops back while counting down d0, then restores saved registers from the stack, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_and_refill_buffer`: Copies bytes from a source pointer to a destination pointer while decrementing a counter, and if the source pointer reaches the end pointer, it calls a routine to read 512 bytes into the buffer and updates the end pointer.
- `update_buffer_xor`: Performs XOR operation between a global variable and a parameter value when the global variable is non-zero, then returns immediately.
- `process_data_with_loop`: Processes data through a loop that calls other routines and updates internal counters, continuing until a condition is met.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: removes the stack frame with unlk, then returns to the caller.
- `process_data_recursive`: Processes data recursively by calling other routines and updating internal counters, continuing until a condition is met.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: removes the stack frame with unlk, then returns to the caller.
- `frag_link_jsr_unlk_rts`: Short code fragment that creates a stack frame in a6, then calls another routine, then removes the stack frame held in a6, then returns.
- `process_buffer_loop`: Creates a stack frame, calls a routine with a parameter, shifts a value left by two bits, calls another routine with the shifted value, and repeats until a condition is met.
- `process_buffer_size_and_pointer`: Calls another routine using a pointer from a structure, then calls another routine using a value from the stack shifted left by two bits and the original pointer.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `execute_conditional_calls`: Creates a stack frame, calls a routine with a parameter, checks if a value is zero, and if not, calls routines in a loop with shifted parameters before calling another routine one final time.
- `frag_pea_move_bsr_addq_unlk_rts`: Short code fragment that pushes the address of the memory at an offset from a6 onto the stack, then copies the memory at an offset from a6 to the stack, then calls another routine, then adds 8 to a7, then copies the memory at an offset from a6 to d0, then removes the stack frame held in a6, then returns.
- `execute_fixed_calls`: Creates a stack frame, calls a routine with the constant eight, calls another routine using a value from the stack, and then calls a third routine using that same stack value shifted left by two bits.
- `allocate_and_initialize_buffer`: Creates a stack frame, calls a routine with a parameter, calculates a size, calls a routine to allocate memory, updates a global pointer with an even-aligned address, and then calls another routine to fill a portion of that memory.
- `frag_link_tst_bgt_moveq_unlk_rts`: Short code fragment that creates a stack frame in a6, then tests the memory at an offset from a6, then branches if the comparison was greater, then loads 0 into d0, then removes the stack frame held in a6, then returns.
- `manage_buffer_limit`: Checks if a memory offset exceeds a limit, and if so, copies fixed values to the stack before decrementing the offset and looping back.
- `frag_move_bsr_lea_unlk_rts`: Short code fragment that copies the memory at an offset from a6 to the stack, then copies a fixed memory address to the stack, then copies the memory at an offset from a6 to the stack, then calls another routine, then loads the address of the memory at an offset from a7 into a7, then removes the stack frame held in a6, then returns.
- `execute_function_pointer`: Writes a byte to a specific memory location, then executes a function pointer calculated from a parameter value, returning after execution.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `fall_through_entry`: Entry point part-way through straight-line code: runs three instructions (asl.l, ori.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `merge_and_update_tables`: Adds a longword from one table to another, updates the two table pointers by adjusting them based on a boundary check, and returns the longword at the updated first pointer.
- `copy_and_process_data`: Copies data from one buffer to another while updating counters and handles boundary conditions, returning the total bytes processed.

### Archon 2 - Adept (1986)(Electronic Arts).adf bytes 885760-887808 (code never loaded by a reference run)

- `setup_executive_base`: Initializes global variables with execution base values and enters an infinite loop calling other routines until terminated.
- `frag_movem_move_dbra_rts_2`: Short code fragment that saves registers on the stack, then copies 2048 to d0, then copies 256 to d1, then copies the memory at an offset from a7 to a fixed memory address, then loops back while counting down d1, then loops back while counting down d0, then restores saved registers from the stack, then returns.

### Archon 2 - Adept (1986)(Electronic Arts).adf bytes 889856-890368 (code never loaded by a reference run)

- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: removes the stack frame with unlk, then returns to the caller.

### Archon 2 - Adept (1986)(Electronic Arts).adf bytes 891904-899584 (code never loaded by a reference run)

- `copy_and_update_pointer`: Copies data from one location to another, updates a pointer, and returns a value based on the operation performed.
- `mfill_buf`: Computes a sector number from the file position shifted right by nine bits plus a base value, reads four sectors into the 2048-byte buffer with a read call, then sets the read pointer to the buffer start plus the position's low nine bits and sets the buffer end 2048 bytes past the start.
- `manage_memory_blocks`: Handles memory block allocation and updates pointers for data processing, returning a value from a system call after updating internal state.
- `initialize_global_data`: Checks a global variable, and if it's zero, returns immediately.
- `toupper`: Walks a null-terminated string at the pointer argument, replacing each byte in the range 97 to 122 with its value minus 32 in place, and stops at the terminating zero byte.
- `compare_string_segments`: Compares character segments from two memory locations, returning -1 if first is less, 0 if equal, or 1 if first is greater.
- `strcopy`: Copies a null-terminated string from a source to a destination and returns the number of bytes copied.
