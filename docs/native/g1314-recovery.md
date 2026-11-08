# Game 1314 (world class leader board (1988)(access)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01314). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 52 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 81 of 81 named (0 author symbols), 81 of 81 described (28 by rule, 49 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 85 of 85 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 2140 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1285 of 1285 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 758515 |
| code | 4284 |
| data | 113574 |
| padding | 12459 |
| unused | 12288 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| World Class Leader Board (1988)(Access).adf:Filter | 1 | 2 | 1 |
| World Class Leader Board (1988)(Access).adf:Golf | 7 | 29 | 2 |
| World Class Leader Board (1988)(Access).adf:    | 1 | 641 | 35 |
| World Class Leader Board (1988)(Access).adf:nofastmem | 3 | 603 | 42 |
| World Class Leader Board (1988)(Access).adf:bootblock | 1 | 10 | 1 |
| HighScores (raw code file, load address unknown) | 1 | 0 | 0 |
| font (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- world class leader board (1988)(access)/World Class Leader Board (1988)(Access).adf: file s/startup-sequence (header 22 bytes, blocks hold 29): its blocks are judged as raw blocks outside the filesystem
- world class leader board (1988)(access)/World Class Leader Board (1988)(Access).adf:WCLB/Data/HighScores: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- world class leader board (1988)(access)/World Class Leader Board (1988)(Access).adf:WCLB/Data/font: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- world class leader board (1988)(access)/World Class Leader Board (1988)(Access).adf: blocks 884-884 (bytes 452608-453120) outside the filesystem hold data, not code (opcode density 0.008, entropy 1.89)
- world class leader board (1988)(access)/World Class Leader Board (1988)(Access).adf:Golf: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 2140 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_hardware_and_call_subroutines` (   ): Reads a byte from a custom hardware register, performs several conditional calls to other routines, and writes a byte to a memory location via a pointer in a1. Kind: disk hardware/integrity check. Pass path: The branch taken immediately following the call to `update_hardware_and_call_subroutines` (likely the path that continues normal execution rather than a crash or error loop)..

## Functions


### World Class Leader Board (1988)(Access).adf:Filter

- `toggle_disk_status_bit`: Toggles bit 0 of the CIAA Port A Data Register and returns.

### World Class Leader Board (1988)(Access).adf:Golf

- `free_blocks_and_clear_field`: Runs a short loop that releases memory blocks through the system FreeMem entry, then zeroes a longword field in the caller's structure.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### World Class Leader Board (1988)(Access).adf:   

- `allocate_and_copy_table`: Calls a routine and, if successful, calls another routine to allocate memory, stores the result in a global variable, and if the allocation succeeds, copies 2875 bytes from a source to the new memory before calling a routine via an indirect jump.
- `execute_branch_sequence`: Calls three different routines in succession and then performs a branch to another location.
- `xor_table_with_mask`: Copies a block of data from a global variable to a destination table while xor-ing each word with a constant mask.
- `update_counter_and_hardware`: Performs bitwise operations on a hardware serial register and updates a global pointer or counter.
- `initialize_data_structure`: Writes a 16-bit constant to an offset of a calculated address, stores a 32-bit constant at another offset, and populates several offsets of the structure with various addresses.
- `call_library_and_copy_data`: Stores a pointer into a location at offset 550 of a6, calls a library routine, and then copies 2875 bytes from a source table to a destination pointed to by the returned value before jumping to an address calculated from that value.
- `update_global_pointer`: Compares a value from a library function to a range and, if within bounds, stores it in a global variable at offset 2 of a table, then performs an indirect jump using a value from a6.
- `check_structure_id`: Loads a pointer from an offset of a6, follows it to another pointer, compares a value at that location to a constant, and if not equal, compares a value at a different offset to a global variable before returning.
- `calculate_checksum_and_verify`: Clears two longwords at offsets 42 and 46 of a structure in a6, sums 23 words from a table starting at offset 34 of a6, stores the negated sum at the end of that table, sums 255 longwords from a table starting at the current PC, and returns if the sum matches a specific constant.
- `access_hardware_registers`: Accesses the CIAA or POT hardware registers.
- `wait_for_input_and_call_library`: Checks a byte at a PC-relative address, runs two loops that modify a value at offset 50 of a hardware register, and calls a library routine using a pointer in a6.
- `update_hardware_and_call_subroutines`: Reads a byte from a custom hardware register, performs several conditional calls to other routines, and writes a byte to a memory location via a pointer in a1.
- `branch_to_offset`: Performs a relative branch to a specific offset.
- `handle_library_and_keyboard`: Checks a byte at a PC-relative address, calls a library routine using a pointer in a6, and performs conditional logic involving the keyboard serial register.
- `process_byte_sequence`: Compares a value at a pointer to a3, runs a loop that sums bytes from a pointer into d2, and calls a routine to process bytes if certain conditions are met.
- `calculate_offset_and_branch`: Multiplies d2 by 22, adds the result to a0, and branches to a PC-relative address.
- `process_data_and_call_subroutines`: Iterates through data pointed to by d6, performs several comparisons and conditional calls to other routines, and modifies a value at offset 40 of a structure in a1.
- `process_structure_pointer`: Loads a pointer from offset 40 of a structure in a1, uses it to fetch a longword at an offset determined by d0, and calls two routines before checking a longword at offset 496 of the same structure.
- `process_structure_data`: Loads a pointer from offset 40 of a structure in a1, calls two routines, then performs a loop that clears a byte at an offset determined by a value from offset 12 of the structure and copies a byte from offset 0 to offset 7 of a secondary structure.
- `update_game_state_and_graphics`: Loads data from a structure, performs several loops to copy and manipulate values, calls multiple routines to update internal state, and uses library calls to interact with hardware or system functions.
- `shift_and_store_register`: Shifts the value of a register left by nine bits and stores the result into a memory location relative to a pointer.
- `store_accumulated_value`: Clears a longword at offset 20 of a structure in a0, calls another routine to calculate a sum, and stores the result in the previously cleared location.
- `accumulate_pointer_values`: Iterates through a list of values pointed to by a register, subtracting them from an accumulator and returning the final sum.
- `compare_string_with_case_insensitivity`: Iterates through a memory buffer, converting certain characters to lowercase and comparing them against a global variable until a mismatch or end-of-buffer occurs.
- `calculate_absolute_difference`: Calculates the absolute difference between a register value and a constant and returns the result.
- `clear_bits_based_on_string_nulls`: Iterates through a string, and if a null terminator is found, clears a corresponding bit in a global bitmask, repeating this for a range of memory.
- `find_string_length_and_set_pointer`: Calculates the length of a string located via a pointer and stores the resulting address into a memory location relative to another pointer.
- `call_library_with_parameter`: Calls a routine to process a value, loads a library base from a global variable, calls a library function at a specific offset, and then performs an indirect jump to a library function.
- `perform_repeated_divisions`: Performs four consecutive unsigned word divisions by 10000, 1000, 100, and 10, calling another routine after each division.
- `convert_byte_to_ascii`: Adds 48 to a byte in d0, stores the result at the memory location pointed to by a0, and returns the high word of d0.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `clear_memory_loop`: Clears a longword at a memory location pointed to by a0 and then performs a branch to another location.
- `tiny_ori`: Small routine of 1 instruction that merges 27 into d0.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 208 into d2, then merges 24 into d0.

### World Class Leader Board (1988)(Access).adf:nofastmem

- `jump_to_address`: Performs an absolute jump to an address stored at a PC-relative offset.
- `call_library_branch_on_result`: Loads a library base from a global variable and calls a library function through it, then passes the result to one of two routines depending on whether it is zero.
- `process_nested_structure_elements`: Checks a pointer in a2, and if non-zero, enters a loop that iterates up to ten times to process elements from a linked list, calling several routines to manipulate data and library functions to handle resource pointers.
- `initialize_and_manage_resources`: Calls a routine with two arguments, and if the result is non-zero, performs a series of library calls and data manipulations including setting specific byte values in a structure and iterating through a list of pointers to clear them.
- `initialize_dos_and_clear_memory`: Initializes a DOS library handle and clears a block of memory using a loop.
- `load_address_and_return`: Loads a longword from a PC-relative address into a4 and returns immediately.
- `initialise_dos_library_handle`: Looks up the dos library and, if it is found, allocates a block, writes a four-character tag and a handle into it and calls library routines to set it up.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `close_library_resources_at_exit`: Runs a cleanup for ten indexed slots, then calls a stored close function and releases up to three stored resource pointers through library calls.
- `set_and_branch`: Sets d0 to 32767 and performs a relative branch.
- `strncat`: Appends characters from the second string to the end of the first, stopping at a zero terminator or after a given count, and terminates the result.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string and returns the number of bytes before the terminator.
- `copy_limited_string_with_pad`: Copies bytes from a source pointer to a destination pointer for a given count, stopping on a zero byte, and fills any remaining space with zero bytes.
- `process_structure_element`: Calls another routine, calculates an address based on the result, and if a value is within a specific range and not zero, checks a bit in a word at that address, potentially calling another routine before clearing the longword at that address.
- `multiply_32_bit_values`: Computes the full 64-bit product of two 32-bit values using the cross terms of their 16-bit halves.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_function`: Performs an indirect jump to a library function via a pointer stored in a location relative to the program counter.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `jump_to_indirect_address`: Loads a pointer from a global variable, applies an offset of 212, and jumps to the address contained at that location.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_library_function`: Loads a library base from a global variable and calls a function at a specific offset within that library.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `execute_indirect_jump`: Performs an indirect jump to a function address retrieved by following a pointer from a global variable and applying two offsets.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### World Class Leader Board (1988)(Access).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
