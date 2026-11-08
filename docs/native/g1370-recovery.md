# Game 1370 (super twintris (1994)(-)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01370). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 7 raw-word lines |
| Functions named and described | pass, 85 of 85 named (0 author symbols), 85 of 85 described (7 by rule, 75 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 77 of 77 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1459 of 1461 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 4212 |
| data | 127185 |
| padding | 2003 |
| unused | 767488 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 4 | 18 | 1 |
| M | 6 | 29 | 2 |
| powerpacker.library | 2 | 756 | 49 |
| explode.library | 1 | 402 | 28 |
| Super Twintris (1994)(-).adf:SuperTwintris | 1 | 99 | 3 |
| Super Twintris (1994)(-).adf:bootblock | 1 | 157 | 2 |

## Cases handled

- super twintris (1994)(-)/Super Twintris (1994)(-).adf:c/M: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- super twintris (1994)(-)/Super Twintris (1994)(-).adf:SuperTwintris: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `init_password_decompression` (powerpacker.library): Clears register d3, loads the address of the 'Password' text into a6, clears a byte at the structure in a1, and calls the decompression setup routine, then branches to the decompression loop. Kind: password check. Pass path: bne.b an address taken when d0 != 0 (success); routine decompresses the password-protected data and returns a nonzero result.

## Functions


### Run

- `loop_until_nonzero_call`: Repeatedly calls a function pointer stored at offset -552 of a global variable until it returns a nonzero value, then calls the function pointer at offset -30 of that returned value.

### M

- `execute_init_and_free`: Calls an initial routine and a loop that executes a function pointer from a global variable, then clears a value at an offset from a pointer and returns a concatenated register value.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### powerpacker.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `library_execute_indirect`: As library code, it stores a value into a global variable, writes a value to offset 36 of that pointer, then calls the function pointer at offset -408 of a6 twice and returns.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `check_flags_return`: Sets d0 to zero, decrements the word at offset 32 of the structure in a6, and returns immediately if that value is nonzero or if bit 3 of the byte at offset 34 is set, otherwise it branches into another routine.
- `check_flag_then_prepare`: Saves registers on the stack, checks whether the word at offset 32 of the structure is zero, and if so it returns immediately, otherwise it sets bit 3 of the byte at offset 34 and loads zero into the return register.
- `library_execute_indirect_2`: As library code, it retrieves a value from offset 36 of a pointer, calls the function pointers at offsets -252, -414, -414 and -210 of a6 in turn, then returns the retrieved value.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `invoke_packer_version_validation`: Saves registers d2 and d3 on the stack, copies the pointer in a2 into d2 and the pointer in a3 into d3, then calls a validation routine and restores the saved registers before returning.
- `decode_powerpacker_block`: Loads a decompressed size from a stack longword, branches on its value, then repeatedly decodes literal and back-references by shifting bits and copying bytes backward through a pointer until the buffer is exhausted.
- `shift_and_branch_zero`: Shifts the value in the register right by one bit, and if the result is zero it returns, otherwise it falls through into the following routine.
- `pop_and_rotate_longword`: Pops a longword from the stack into d5, rotates it right by one bit, then returns.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `rotate_loop`: Sets d1 to zero, shifts d5 right and rotates d1 left in a loop while decrementing d0, then returns, otherwise it branches into the middle of another routine.
- `pop_and_rotate_longword_2`: Pops a longword from the stack into the register, rotates it right by one bit, and branches into the middle of the following routine.
- `decode_lzss_block`: Sets d0 to one and calls a routine, then loads a byte from a table indexed by d1 into d0 and copies a run of bytes backward through a pointer in a2 while decrementing a counter.
- `store_byte_loop_pointer`: Decrements a loop counter in d2, increments a byte offset in d3, and stores a byte from a table through a pointer in a1 onto the stack, repeating until the counter reaches zero.
- `compute_checksum`: Computes a 16-bit checksum over a zero-terminated byte stream by accumulating bytes into a value that is rotated left by one bit during each iteration.
- `compute_checksum_2`: As part of the PowerPacker shared library, computes a checksum over a zero-terminated byte stream pointed to by a0.
- `xor_decrypt_block`: As part of the PowerPacker shared library, XORs a block of memory in place using a 32-bit key supplied in a register.
- `init_password_decompression`: Clears register d3, loads the address of the 'Password' text into a6, clears a byte at the structure in a1, and calls the decompression setup routine, then branches to the decompression loop.
- `call_routine_then_return`: As library code, it saves registers d3 and a6 to the stack, calls another routine using a pointer in a6, then restores the registers and returns.
- `setup_stack_frame`: As library code, it reserves stack space, copies an input pointer into a stack buffer, clears a byte there, calls a routine with the prepared frame, then restores the stack and returns.
- `initialize_stack_and_call_routine`: Initializes a stack frame and calls a routine that returns a pointer, then branches to another routine passing that pointer in a register.
- `decrypt_adjusted_block`: Sets up a stack frame and calls a checksum routine over a zero-terminated byte stream, then adjusts the stack pointer and computes a pointer from a value in a global variable.
- `process_powerpacker_hunks`: Walks a hunk stream, detecting and merging consecutive 128-byte-aligned executable or relocation hunks, copying their data into newly allocated memory while adjusting their sizes and relocation pointers.
- `process_amigados_hunks`: Processes an AmigaDOS hunk stream, copying executable and relocation hunks into newly allocated memory and copying data for other hunk types, then relocates the loaded code and returns.
- `merge_powerpacker_hunks`: Walks a hunk stream, detecting 128-byte-aligned executable or relocation hunks and merging consecutive ones by copying their data into a newly allocated buffer, then updates the hunk table to reflect the merged blocks.
- `adjust_pointer_and_branch`: Copies a5 into a1, subtracts 4 from a1, decrements d1, and branches to a return routine when the subtract produces a carry.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 4 to a1, then returns.
- `merge_powerpacker_hunks_2`: Walks a hunk stream, skipping non-executable and non-relocation hunks, and for each matching hunk computes its size by subtracting a base pointer from a stored value before calling a library function to process it.
- `tiny_clr`: Small routine of 1 instruction that clears (a0)+.
- `decrement_and_loop`: Decrements d0 and branches back to the start of the loop until d0 reaches zero, then returns.
- `copy_and_verify_powerpacker_header`: Copies a null-terminated string from the source buffer into a stack buffer, then calls a routine to read from the source and compares the decompressed data header against the known PowerPacker magic values.
- `copy_null_terminated_string`: Copies a null-terminated string from a source buffer into a destination buffer, then calls a routine and branches back to a shared handler.
- `dispatch_powerpacker_decompress`: Calls two library functions to read from a source buffer, then compares the decompressed data header against known PowerPacker magic values and branches to the matching decompression routine, updating the source pointer and shift register as it goes.
- `process_powerpacker_byte_2`: Computes a stack offset, saves a value into register d1, and calls a library function using a base pointer, then decrements the result and returns immediately when the value is zero.
- `process_powerpacker_byte`: Clears a byte at offset 37 of the structure in a5, then calls a routine using that byte as an argument when a pointer at offset 184 of the task is non-zero, otherwise it returns a negative value.
- `read_byte_and_shift_longword`: Reads a byte from a source buffer using the library's read functions, then shifts a stored longword eight bits to the right when the read value is nonzero, and finally calls a function pointer stored in a global variable.
- `library_read_and_shift`: As library code, it calls two library functions to read from a source buffer, shifts a stored longword right by eight bits when the result equals four, and otherwise calls a second library function before jumping to a shared exit.
- `shift_and_store_bits`: Loads a longword from a global variable at offset -50 of a5, shifts it right by eight bits, stores the result back to the same location, copies it into d6, increments d6 by eight, subtracts d7 from d5, and branches into another routine.
- `check_magic_and_branch`: Loads a word from a global variable at offset -42 of a5, compares it with the value 20560, and branches to a different routine when the values differ.
- `call_and_branch`: Saves d5 into a global variable, calls a function pointer at offset -198 of a6, and branches into another routine when the result is nonzero, otherwise it sets d7 to 253 and continues.
- `dispatch_decode_command`: Calls a function pointer at offset -42 of the structure in a4 and compares its result with the value in d7, otherwise it calls a function pointer at offset -36 and returns its result.
- `call_library_functions`: As library code, it calls the function pointer at offset -210 of a global variable, reads a value from offset -46 of a5, calls the function pointer at offset -36 of a structure, and returns the previously read value.
- `process_decompressed_buffer`: As library code, it saves registers, initializes several stack fields, calls a setup routine three times, then repeatedly pulls and processes decompressed data through library calls until a termination condition is met.
- `process_powerpacker_buffer`: Reads a decompressed size from a library call, stores a computed width and height into a structure, then repeatedly pulls and processes data from a buffer until a terminator is reached, returning a flag from that buffer.
- `check_flag_and_call`: Sets d0 to 2 and returns immediately if it does not equal d6, otherwise it sets bit 11 of the word at offset -70 of a5, calls a function pointer at offset -204, and branches into another routine when the result is nonzero, otherwise it returns zero.
- `process_message_loop`: Calls a function pointer at offset 8 of the structure in a6 and loops while its result equals 8, then checks a byte at a3 and increments a counter at offset -134 of a5, otherwise it returns that byte.
- `call_indirect_return_byte`: Calls the function pointer stored at offset 8 of the structure in a6, then loads a byte from the memory location pointed to by a3 into the return register and returns it.

### explode.library

- `return_negative_one`: Loads the value negative one into the return register and returns immediately.
- `open_library_and_dispatch`: Saves a register, stores the first argument into a structure, writes a one at offset 32 and the pointer at offset 38, calls an exec routine to open a library, and when the result is nonzero loads a constant and jumps to a library routine.
- `clear_fields_check_trap_instruction`: Clears two longwords and a byte in the structure in a5, loads a pointer from offset 34, and when the word at that pointer equals the trap instruction 20217 stores a pointer at offset 4 and calls a routine, otherwise it branches to another routine.
- `compare_byte_branch_on_match`: Compares a byte from the input pointer to 112 and branches to another routine when they match, otherwise it pushes a register, loads a constant, calls a library routine, restores the register, and falls through to the next routine.
- `copy_byte_dispatch_on_word`: Copies a byte from the input pointer to offset 1 of the structure in a1, then advances the input pointer and checks the following word: when it equals 24576 it stores an offset-adjusted pointer at offset 4 and calls one routine, otherwise it calls another routine.
- `increment_and_return_value`: Increments the 32-bit value at offset 42 of the structure in a6 by one, copies that value into the return register, and returns.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `verify_library_name_and_count`: Calls a routine to obtain a pointer, multiplies the result by four, and compares five consecutive longwords at offsets 4 through 52 against the ASCII string explode.library, incrementing a library counter and calling one routine when all five match.
- `increment_counter_and_call`: Increments a 32-bit counter in the explode library data, calls one routine, and returns the saved first argument unchanged.
- `find_task_set_priority_around`: Finds the current task, calls the priority-setting exec routine on it, makes two more routine calls, sets the priority again, and returns its first argument unchanged.
- `initialize_explode_structure`: Initializes a 48-byte structure in a4 from a packed input in a5 by decoding its nibbles into a lookup table, computing cumulative offsets and a bitmask, then builds a linked list of pointers and calls a helper routine once for each entry.
- `process_explode_counter_loop`: Doubles the input value and branches to a helper when it is negative, otherwise calls a helper and stores the result into a word register.
- `copy_four_bytes_via_helper`: Copies four bytes from the address in a2 to the address in a1 via a helper routine, then jumps back to the caller.
- `load_address_and_value`: Computes the address of a global variable at offset 48 of a4 into a register and loads the value two into the return register, then falls through into the following routine.
- `copy_four_bytes_loop`: Compares register a2 with a6 and returns immediately if they are equal, otherwise copies four bytes from the address in a2 to the address in a1, increments a3a4 by four, decrements the counter in d0, and returns when the counter reaches zero.
- `copy_explode_run_byte`: Copies a run of bytes from the source buffer in a0 to the destination in a3, decrementing the count, and when the run is exhausted it reads a bit from a flag byte in the structure at a4, loading a new flag byte from the source when needed, then returns when the remaining count in the structure reaches zero.
- `decode_run_length_and_copy`: Decodes a run-length byte from the source buffer in a0 into a byte count, then calls copy_explode_run to copy that many bytes from the source to the destination.
- `setup_run_copy_five`: Loads the decimal constant 5 into register d0 and the decimal constant 3 into register d7, then branches to a shared routine that copies a decoded run-length of bytes from the source buffer in a0 to the destination.
- `setup_run_copy_four`: Loads the decimal constant 4 into register d0 and the decimal constant 2 into register d7, then branches to a shared routine that copies a decoded run-length of bytes from the source buffer in a0 to the destination.
- `setup_run_copy_three`: Loads the decimal constant 3 into register d0 and the decimal constant 1 into register d7, then branches to a shared routine that copies a decoded run-length of bytes from the source buffer in a0 to the destination.
- `copy_explode_run`: Reads a run-length from the source buffer in a0, decodes it into a byte count, then copies that many bytes from the source to the destination using a loop that wraps around when the destination pointer reaches its end.
- `accumulate_explode_bits`: Accumulates bits from a source byte into a register using carry propagation, branches into another routine, and then returns.
- `decode_and_copy_explode_run_2`: Decodes a run of bits from the source buffer in a0 into a 16-bit value in d1, then copies the resulting number of bytes from the source to the destination, using the table in a4 to look up lengths and offsets for the copy.
- `decode_and_copy_explode_run`: Decodes a run of bits from the source buffer in a0 into a 16-bit value in d1, then copies the resulting number of bytes from the source to the destination, calling another routine when the counter reaches zero.
- `store_and_branch`: Stores d0 at offset 3 of the structure in a4, stores d1 at offset 2, stores d2 as the longword at the base of that structure, restores the caller's registers, and branches into the middle of another routine.
- `run_delay_loop`: Reads a counter at offset 56 of a5, calls a routine at offset -210 of a6 in a loop while decrementing that counter, then clears a stack slot and adds a value at offset 58 of a5 to a global variable.
- `add_to_global_longword`: Adds the value in the return register to the 32-bit global variable at offset zero and returns.
- `delay_countdown_loop`: Copies a register into another, decrements it by one, and if it is not negative enters a loop that decrements it until it reaches minus one, otherwise it returns.

### Super Twintris (1994)(-).adf:SuperTwintris

- `copy_byte_block`: Copies a block of bytes from a source table to a destination table, choosing between two source pointers based on a comparison, then returns.
- `store_byte_and_shift_right`: Loads a longword from just below the pointer in a0 into d0, saves its low byte in d7, stores that byte at the address in a5, shifts d0 right by one bit through the carry flag, and returns.
- `shift_and_rotate`: Loops while decrementing d1, shifting d0 right and rotating d2 left until d0 becomes zero, then returns, otherwise it branches into the middle of another routine.

### Super Twintris (1994)(-).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `build_copper_gradient_list`: Writes a copper list into memory as two blocks of fifteen entries, where each entry consists of a wait for a scanline followed by a write to the background colour register, and the second block steps the colour value down.
