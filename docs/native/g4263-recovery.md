# Game 4263 (Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/04263). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4858239 top-level bytes in 17 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 70 of 70 named (0 author symbols), 70 of 70 described (45 by rule, 22 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 58 of 58 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1678 of 1678 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2010746 |
| code | 6390 |
| data | 2690633 |
| padding | 3014 |
| unused | 147456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).adf:golf153 | 3 | 380 | 7 |
| Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).adf:instalcrs | 3 | 471 | 30 |
| Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).ipf#adf:golf153 | 3 | 380 | 7 |
| Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).ipf#adf:instalcrs | 3 | 447 | 26 |

## Cases handled

- Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon)/Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).ipf decoded by disk-analyse to a 901120-byte disk image (tracks: T0.0-79.1: AmigaDOS)
- Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon)/Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).adf:fireston.crs: high-entropy data file (7.89 bits/byte), classified as packed data
- Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon)/Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).ipf#adf:fireston.crs: high-entropy data file (7.89 bits/byte), classified as packed data
- Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon)/Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon)/Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).ipf#adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon)/Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).adf:golf153: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon)/Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).ipf#adf:golf153: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).adf:golf153

- `initialize_and_process_data`: Initializes library connections and processes a table of data entries, performing file operations and memory management for each entry.
- `frag_movea_tst_bne_suba_move_jsr_rts`: Short code fragment that copies a0 to a1, then tests the memory at a1 (advancing a1), then branches if the last result was not zero, then subtracts a0 from a1, then copies a1 to d3, then copies a0 to d2, then copies a fixed memory address to d1, then copies a fixed memory address to a6, then calls another routine, then returns.
- `format_number_string`: Converts a value into a decimal string representation with leading zeros, storing the result in memory.
- `frag_movem_move_movea_jsr_tst_rts`: Short code fragment that saves registers on the stack, then copies 1 to d1, then copies a fixed memory address to a6, then calls another routine, then tests d0, then restores saved registers from the stack, then returns.
- `frag_movem_pea_bsr_move_movea_sub_rts`: Short code fragment that saves registers on the stack, then pushes the address of the memory at a0 onto the stack, then calls another routine, then copies a1 to d0, then copies the top of the stack to a0, then subtracts the memory at an offset from a0 from d0, then copies d0 to the memory at an offset from a0, then restores saved registers from the stack, then returns.
- `copy_byte_sequence`: Copies 60 bytes from a source buffer to a destination buffer using post-increment addressing modes.
- `initialize_course_data`: Initializes course-related data structures by setting up counters and offsets, then performs iterative updates on game state variables based on input values from memory locations.

### Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).adf:instalcrs

- `initialize_installation_3`: Initializes installation data structures, opens the exec library, allocates memory, and handles user input for course destination selection.
- `call_open_library`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `frag_lea_bsr_move_beq_rts`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then copies d0 to a fixed memory address, then branches if the last result was zero, then returns.
- `frag_lea_bsr_move_beq_rts_2`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then copies d0 to a fixed memory address, then branches if the last result was zero, then returns.
- `initialize_installation`: Allocates memory and opens the exec library, then initializes installation data structures and handles user input for course destination selection.
- `check_keyboard_abort`: Reads a byte from the keyboard serial register, checks if it matches the 't' key ASCII value, and if so, calls another routine to handle cleanup operations before returning.
- `copy_until_null`: Copies bytes from one memory location to another until it encounters a null terminator, then updates a global pointer to point to the end of the copied data.
- `handle_file_operations`: Manages file opening and copying operations, checking for specific keyboard input and handling errors during file operations through system calls.
- `frag_movea_tst_bne_suba_move_jsr_rts_2`: Short code fragment that copies a0 to a1, then tests the memory at a1 (advancing a1), then branches if the last result was not zero, then subtracts a0 from a1, then copies a1 to d3, then copies a location in the code to d1, then copies a0 to d2, then copies a location in the code to a6, then calls another routine, then returns.
- `frag_movea_movem_pea_bra`: Short code fragment that copies the memory at an offset from a7 to a0, then copies several registers to or from memory, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_movem_pea_bra`: Short code fragment that copies several registers to or from memory, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_movea_jsr_rts_2`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_movea_jsr_rts_3`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movea_pea_bra`: Short code fragment that copies the memory at an offset from a7 to a0, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_pea_bra`: Short code fragment that copies the memory at an offset from a7 to d0, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_movea_pea_bra_2`: Short code fragment that copies the memory at an offset from a7 to a0, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_movem_movea_jsr_rts_4`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to a0, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_movea_jsr_rts_5`: Short code fragment that saves registers on the stack, then copies several registers to or from memory (2 times in a row), then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_movea_jsr_rts_6`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to a0, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movea_pea_bra_3`: Short code fragment that copies the memory at an offset from a7 to a0, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `link_string_gadget`: Library stub: loads a library base from a global variable and calls the library function at offset minus 150 of that base, returning its result to the caller.
- `link_prop_gadget`: Library stub: loads a library base from a global variable and calls the library function at offset minus 156 of that base, returning its result to the caller.
- `get_string`: Library stub: loads a library base from a global variable and calls the library function at offset minus 162 of that base, returning its result to the caller.
- `frag_movea_pea_bra_7`: Short code fragment that copies the memory at an offset from a7 to a0, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_pea_move_movea_add_rts`: Short code fragment that pushes the address of a fixed memory address onto the stack, then copies the memory at an offset from a7 to the stack, then copies a6 to the memory at an offset from a7, then copies a fixed memory address to a6, then copies d0 to the stack, then copies a6 to d0, then adds d0 to the memory at an offset from a7, then copies the top of the stack to d0, then returns.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `prepare_and_call_structure`: Initializes a stack structure with multiple pointers and a constant value, then invokes a function through a pointer stored in a global variable.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).ipf#adf:golf153

- `initialize_and_process`: Initializes library connections and processes data entries from a table, handling file operations and directory creation.
- `frag_movea_tst_bne_suba_move_jsr_rts_3`: Short code fragment that copies a0 to a1, then tests the memory at a1 (advancing a1), then branches if the last result was not zero, then subtracts a0 from a1, then copies a1 to d3, then copies a0 to d2, then copies a fixed memory address to d1, then copies a fixed memory address to a6, then calls another routine, then returns.
- `convert_and_pad`: Performs arithmetic operations on data from a table, converts results to characters, stores them sequentially, and pads the result with spaces.
- `frag_movem_move_movea_jsr_tst_rts_2`: Short code fragment that saves registers on the stack, then copies 1 to d1, then copies a fixed memory address to a6, then calls another routine, then tests d0, then restores saved registers from the stack, then returns.
- `calculate_difference`: Subtracts a value from a pointer offset and stores the result into another memory location.
- `copy_byte_sequence_2`: Copies 58 bytes from a source buffer to a destination buffer using post-increment addressing.
- `initialize_course_data_2`: Initializes course-related data structures by setting up arrays with specific values and performing iterative updates on game state variables.

### Links Championship Course 1 - Firestone Country Club Akron, Ohio (Europe) (Addon).ipf#adf:instalcrs

- `initialize_system`: Initializes system resources by setting up task pointers and memory structures, then waits for and processes messages from a message port while handling various library function calls and conditional logic based on message content.
- `call_open_library_2`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.
- `call_close_library_2`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `frag_lea_bsr_move_beq_rts_3`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then copies d0 to a fixed memory address, then branches if the last result was zero, then returns.
- `frag_lea_bsr_move_beq_rts_4`: Short code fragment that loads the address of a fixed memory address into a1, then calls another routine, then copies d0 to a fixed memory address, then branches if the last result was zero, then returns.
- `initialize_installation_2`: Allocates memory and opens the exec library, then initializes installation data structures and handles user input for course destination selection.
- `check_keyboard_input`: Reads the keyboard serial register, clears bit 0 of the read value, compares it with the character 't', and branches to different routines based on the comparison result.
- `copy_until_null_2`: Copies bytes from one memory location to another until it encounters a null terminator, then updates a global pointer to point to the end of the copied data.
- `handle_file_operations_2`: Manages file opening and copying operations, checking for errors and updating status flags based on the success or failure of these operations.
- `frag_movea_tst_bne_suba_move_jsr_rts_4`: Short code fragment that copies a0 to a1, then tests the memory at a1 (advancing a1), then branches if the last result was not zero, then subtracts a0 from a1, then copies a1 to d3, then copies a location in the code to d1, then copies a0 to d2, then copies a location in the code to a6, then calls another routine, then returns.
- `frag_movea_movem_pea_bra_2`: Short code fragment that copies the memory at an offset from a7 to a0, then copies several registers to or from memory, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_movem_pea_bra_2`: Short code fragment that copies several registers to or from memory, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_movem_movea_jsr_rts_7`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_movea_jsr_rts_8`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_movea_jsr_rts_9`: Short code fragment that saves registers on the stack, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movea_pea_bra_4`: Short code fragment that copies the memory at an offset from a7 to a0, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_pea_bra_2`: Short code fragment that copies the memory at an offset from a7 to d0, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_movea_pea_bra_5`: Short code fragment that copies the memory at an offset from a7 to a0, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_movem_movea_jsr_rts_10`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to a0, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_movea_jsr_rts_11`: Short code fragment that saves registers on the stack, then copies several registers to or from memory (2 times in a row), then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_movea_jsr_rts_12`: Short code fragment that saves registers on the stack, then copies the memory at an offset from a7 to a0, then copies several registers to or from memory, then copies a fixed memory address to a6, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movea_pea_bra_6`: Short code fragment that copies the memory at an offset from a7 to a0, then pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `adjust_stack_and_return`: Adjusts the stack pointer by adding a value from a register and then returns to the caller.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.
- `prepare_and_call_structure_2`: Clears 23 words on the stack, populates a structure with four pointers and a 16-bit constant, then calls a routine via a pointer stored in a global variable.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d0, then returns.
