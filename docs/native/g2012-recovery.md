# Game 2012 (populous (1989)(electronic arts)[cr untouchables]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02012). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (946000 top-level bytes in 33 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 164 of 164 named (0 author symbols), 164 of 164 described (35 by rule, 126 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2777 of 2777 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4305 of 4305 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 747270 |
| code | 12592 |
| data | 153274 |
| padding | 9824 |
| unused | 23040 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Populous (1989)(Electronic Arts)[cr Untouchables].adf:run | 2 | 117 | 2 |
| Populous (1989)(Electronic Arts)[cr Untouchables].adf:populous.prg | 2 | 207 | 8 |
| Populous (1989)(Electronic Arts)[cr Untouchables].adf:KEFLOAD | 2 | 33 | 1 |
| Populous (1989)(Electronic Arts)[cr Untouchables].adf:1 | 2 | 207 | 8 |
| Populous (1989)(Electronic Arts)[cr Untouchables].adf:bootblock | 1 | 19 | 1 |
| logged disk load from Populous (1989)(Electronic Arts)[cr Untouchables].adf boot-block load | 1 | 83 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 77 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2199 | 132 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1363 | 7 |

## Cases handled

- populous (1989)(electronic arts)[cr untouchables]/Populous (1989)(Electronic Arts)[cr Untouchables].adf: the boot block reads 10240 bytes from disk offset 1024 to an address (found statically); recovered as a unit
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 41 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1221 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- RAM snapshot an address: 1346 executed PCs whose RAM was overwritten before any snapshot match its bytes 1088-4224: recovered as a unit run at an address
- populous (1989)(electronic arts)[cr untouchables]/Populous (1989)(Electronic Arts)[cr Untouchables].adf: 1 logged loads cover 10240 bytes of the image
- populous (1989)(electronic arts)[cr untouchables]/Populous (1989)(Electronic Arts)[cr Untouchables].adf:populous.prg: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- populous (1989)(electronic arts)[cr untouchables]/Populous (1989)(Electronic Arts)[cr Untouchables].adf:1: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Populous (1989)(Electronic Arts)[cr Untouchables].adf:run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_entries_from_table`: Copies a fixed number of four-byte entries from a table into a caller-provided buffer, advancing both pointers until the counter reaches zero.

### Populous (1989)(Electronic Arts)[cr Untouchables].adf:populous.prg

- `process_packed_stream`: Processes a packed data stream by unpacking it into a destination buffer and then decoding a run-length-encoded table.
- `return_empty_stub`: Consists of five instructions that perform no observable work before returning.
- `decrement_counter_register`: Loops a fixed number of times decrementing a counter held in a register, making no memory accesses and no calls.
- `tiny_ori`: Small routine of 1 instruction that merges 196 into d0.
- `unpack_run_length_stream`: Reads a packed byte stream and reconstructs it into memory, writing backwards from a destination pointer.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_with_space_wrap`: Loops a fixed number of times while decrementing a byte held in a register, replacing it with a space character when it underflows to zero.
- `return_empty_stub_2`: Consists of three instructions that perform no observable work before returning.

### Populous (1989)(Electronic Arts)[cr Untouchables].adf:KEFLOAD

- `store_digit_string_and_init`: Builds a two-character ASCII string from the digits of a global counter and stores it in a global buffer, clearing the counter afterward.

### Populous (1989)(Electronic Arts)[cr Untouchables].adf:1

- `process_relocation_table`: Processes a table of 32-bit entries by computing offsets and calling another routine for each item, storing results into a destination buffer.
- `return_empty_stub_3`: It executes five instructions that perform no observable work before returning.
- `decrement_register_counter`: Loops decrementing a value held in a register until it reaches zero, making no memory accesses and no calls.
- `tiny_ori_2`: Small routine of 1 instruction that merges 28 into d0.
- `copy_memory_backward`: Copies a block of bytes from one location to another while walking backward through both the source and destination pointers, decrementing a counter until the entire region has been transferred.
- `decrement_byte_loop`: It decrements a byte held in a register, wrapping it to a space character when it underflows to zero, and repeats this a fixed number of times.
- `decrement_byte_with_space_wrap_2`: Loops a fixed number of times while decrementing a byte held in a register, replacing it with a space character whenever it underflows to zero.
- `halt_baddata`: This routine contains no executable instructions, only data bytes that were misidentified as code.

### Populous (1989)(Electronic Arts)[cr Untouchables].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from Populous (1989)(Electronic Arts)[cr Untouchables].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_bad_instruction_data`: A 69-instruction routine that executes data flagged by the decompiler as a bad-instruction control-flow error.
- `call_indirect_from_register`: It dereferences a pointer stored at a fixed offset from an unaffiliated register and calls the code pointed to, passing a value loaded from another pointer chain.
- `execute_bad_instruction`: It is a single instruction that executes bad instruction data, which the decompiler flags as a control-flow error.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `run_setup_and_copy_blocks`: It calls a series of setup routines, then copies two blocks of words when a stored value equals three, sets several flag fields, and stores a value from another location.
- `lookup_and_process_resource`: It queries a table for a resource by an identifier, and on success stores the result, computes derived values, calls another routine with them, and finally releases the resource.
- `lookup_table_and_call`: Searches a table of four entries for one whose field matches a parameter, then calls two routines with the matching entry's other fields.
- `swap_and_pass_pair`: Swaps two consecutive four-byte values held in a global structure, then passes the swapped values to another routine.
- `call_table_routine_repeatedly`: Loops eight times calling a routine with successive entries from a parameter table and a fixed argument.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `fill_buffer_with_parameter_data`: It fills a buffer with a sequence of words, writing fixed header entries, a loop of paired values derived from a parameter, and further parameter-derived words before terminating the sequence.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `wait_for_copper_interrupt`: It stores a parameter-derived value into a global variable, then loops, recombining the parameter and the global variable and masking the result, until the global variable has bit five set.
- `called_twenty_one_instructions`: No further behavior can be described from the available information because the decompiler timed out.
- `load_byte_from_pointer`: Loads a pointer from a global variable, reads a byte from the pointed-to location, and stores that byte into a global variable.
- `update_byte_buffer_from_params`: It indexes a table with a parameter to obtain a byte buffer pointer, then writes several bytes into that buffer derived from the two parameters and a global variable, including conditional flag bits.
- `compare_and_store_derived`: Compares two values derived from its parameter against fields in a global structure, and when they qualify, stores a derived value and calls several routines with the derived data, returning a result.
- `store_derived_values_and_compare`: It stores several values derived from its parameters into fields of a structure pointed to by the first parameter, reads a table entry indexed by a second parameter, and returns whether the resulting accumulated value is nonzero.
- `index_and_bump_counters`: Invokes another routine several times with increasing arguments, increments a counter, then compares an indexed structure field against that counter and, on a match, increments a second counter, resets the first, rolls the index over, and stores a combined value into another field.
- `dispatch_pending_calls`: Calls a routine with four values taken from a table, then calls a jump-table entry with three different argument pairs when three global variables are nonzero.
- `setup_audio_resources`: Allocates audio resources by repeatedly requesting a node and filling its fields, recursing into nested structures until four entries are set up.
- `process_table_entries_recursive`: Walks through four consecutive entries of a table, recursing into nested sub-structures when their flags are set, then clears the entry pointer and invokes a callback stored at a fixed offset.
- `allocate_and_map_memory`: Allocates a block of memory and records its size, then maps that block into a table of segments, filling each entry with its address and length.
- `save_and_restore_registers`: Saves the caller's register contents into a global structure, then restores those same registers from that structure.
- `save_registers_and_restore`: Saves several registers onto the stack, runs a sequence of helper routines, then restores the saved registers from a global structure.
- `conditional_byte_swap`: Stores two function pointers, then conditionally byte-swaps and exclusive-or's a stack value or restores a previous value depending on a flag.
- `swap_stack_byte_swap`: Swaps a global variable with a stack value, then overwrites that stack slot with a byte-swapped exclusive-or of its previous contents.
- `run_sequential_helper_routines`: Runs three short helper routines in sequence, one of which writes a masked byte to a global variable, another polls a CIAB bit, and the third performs a fixed spin delay.
- `tiny_moveq_2`: Small routine of 1 instruction that loads 255 into d1.
- `mask_global_with_ciab`: It writes to a global variable a value formed by masking the input byte with the inverse of a bit selected from a global variable and the CIAB_PRB hardware register.
- `wait_for_ciaa_bit`: It polls a global variable for up to 50000 iterations, returning zero when the bit clears and returning the loop counter otherwise.
- `steer_ciab_toward_target`: Reads a bit pattern from a CIAB register, walks a global byte toward a target value by incrementing or decrementing it, and stores the resulting value into a table indexed by a register.
- `empty_routine`: The routine contains no instructions and returns to the caller immediately without touching memory, registers, or calling another routine.
- `wait_for_flag_clear`: Waits for a flag bit to clear in a global byte, writing a value and clearing a register bit while waiting.
- `clear_ciab_bit_and_delay`: Clears a bit selected from a CIAB data register within a global byte, adjusts it based on another CIAB flag, sets the least significant bit, and then burns time in a fixed countdown loop.
- `update_ciab_output_bit`: Reads a bit from the CIAB register, combines it with a flag check into a global variable, then waits out a fixed countdown loop before returning.
- `modify_global_with_ciab`: The routine sets a bit in a global variable based on a value from a global byte, clears that same bit when another global byte is nonzero, and then spins in a fixed countdown loop.
- `spin_delay_loop`: The routine spins in a fixed countdown loop using the input value and then returns without accessing memory or registers.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `tiny_ori_3`: Small routine of 1 instruction that merges 0 into d0.
- `empty_forward_routine`: It is an empty routine that only forwards to another routine without touching its own data.
- `save_context_and_call`: It saves the caller's registers onto the stack and records the return address in a global variable, then invokes another routine through a helper that pushes a chain of words onto the stack through a pointer.
- `push_and_call_routine`: It saves the return address into a global variable, then pushes a chain of words onto the stack through a pointer and calls another routine.
- `conditional_xor_stack_word`: This routine stores a return address into a global variable, then uses a flag byte to conditionally XOR a stack word with the bit-reversed version of an adjacent stack word, saving and restoring the value through a saved pointer.
- `swap_and_xor_stack_words`: It swaps two adjacent words on the stack and then XORs the resulting stack word with a bit-reversed version of the previous stack word.
- `run_serial_helpers`: Runs three helper routines in sequence, one of which masks a byte using a CIAB bit, another of which polls a CIAB bit with a bounded spin loop, and a third of which performs a fixed delay, before returning.
- `clear_bit_in_global`: The routine writes a single-bit mask derived from a global byte into a global variable, clearing that bit.
- `mask_global_with_ciab_2`: The routine writes to a global variable a value formed by masking the input byte with the inverse of a bit selected from a global variable and the CIAB_PRB hardware register.
- `wait_for_ciaa_bit_2`: The routine polls a global variable for up to 50000 iterations, returning zero when the target bit clears and returning the loop counter otherwise, with the global variable reflecting CIAA_PRA.
- `search_table_increment`: Searches a table for a value matching a masked input, incrementing or decrementing a counter until it is found, then stores the result.
- `empty_routine_2`: This routine contains four instructions and performs no observable work; it ends immediately without accessing memory, registers, or calling another routine.
- `wait_for_bit_or_countdown`: Waits for a bit in a global byte to clear while decrementing a counter, or counts down to zero, then stores a result value into a table indexed by a register.
- `modify_global_byte_and_delay`: Modifies a byte in a global structure by clearing a selected bit and setting the least significant bit, then spins through a fixed delay loop before returning.
- `modify_ciab_byte_and_delay`: Clears a bit selected from a CIAB register within a global byte, adjusts it according to another CIAB flag, sets the least significant bit, and then executes a fixed countdown delay before returning.
- `clear_flag_bit_then_delay`: It clears a single bit in a global byte, choosing which bit from a counter value, then clears another bit if a separate flag byte is nonzero, and finally spins through a short fixed-count delay loop.
- `spin_down_counter`: Decrements a register in a loop until it reaches zero, performing a fixed spin delay and then returning.
- `do_nothing_routine`: This routine contains no instructions and performs no operations.
- `copy_byte_between_globals`: Loads a byte from a global variable into a register and stores it back to another global variable, effectively copying one byte from one location to another.
- `dispatch_jumptable`: The routine dispatches to one of several targets through an unrecovered jump table and then returns to the caller.
- `clear_and_dispatch_struct`: Clears a word array to zero, stores a supervisor register into a structure, then invokes a series of function pointers stored in that structure.
- `empty_routine_3`: It contains no instructions and performs no operation.
- `allocate_and_dispatch_dos`: Allocates a buffer, writes a magic marker and several flags into a global structure, then dispatches a series of DOS library operations and records their results.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `tokenize_string_into_words`: Allocates a temporary buffer, copies a prefix from a source string, and tokenizes the remaining text into whitespace-separated words while unescaping quoted segments.
- `concatenate_string_with_limit`: Finds the end of a null-terminated source string, then copies bytes from a second string into the destination until a null terminator is reached or a counter is exhausted, ensuring the result is null-terminated.
- `copy_string_with_limit`: It scans a source string until it finds a null terminator, then copies bytes into a destination buffer while decrementing a counter, stopping when the counter reaches -1.
- `copy_string_pad_zeros`: It copies bytes from a source into a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: It takes two 32-bit values from registers and multiplies their 16-bit halves using the cross terms, then returns the low 32 bits of the product in the result and the high 32 bits in the other register.
- `open_window_and_icon`: Opens a window and its associated icon, then sets up related structures by chaining several open and setup operations.
- `signed_divide_longwords`: Divides a 32-bit value by a 32-bit divisor by calling an unsigned division routine and negating the result when the dividend and divisor have different signs.
- `signed_divide_longword`: Calls an unsigned division routine and negates the result when the dividend is negative, then returns the signed quotient.
- `divide_32bit_values`: Divides a 32-bit value by a 32-bit divisor using binary long division when the high word is nonzero, otherwise falling back to a 16-bit division, and returns the quotient and remainder.
- `divide_32_bit_values`: It performs a binary long division by repeatedly shifting the dividend, comparing the running remainder against the divisor, subtracting when it fits, and shifting the quotient, iterating over the bits of the operand.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `dispatch_table_call`: Loads a value from a table indexed by a global variable and jumps to the routine found there.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `dispatch_io_functions`: It looks up an entry in a library dispatch table by offset and jumps to it, returning the result of the dos library Output, Read and Seek calls to the caller.
- `dispatch_table_call_2`: Reads a pointer from a table indexed by a value stored at an offset within a structure, then jumps to the routine found at that address.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `dispatch_library_calls`: It looks up an entry in a library dispatch table by offset and jumps to it, returning the result of the two library calls identified by their offsets to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_468`: Library stub: loads a library base from a global variable and calls the library function at offset minus 468 of that base, returning its result to the caller.
- `call_indirect_via_table`: Looks up an entry in a jump table indexed by a value read from a global variable, then calls the routine found there.
- `create_and_link_node`: Allocates a node from a library and initializes its fields with the passed parameters, then recursively links the node into a list.
- `check_flag_and_invoke_callback`: Checks a flag in the passed structure and recurses into itself when it is set, then writes a fixed byte and a full word to other fields and finally invokes a stored callback with a constant argument.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `dispatch_via_jumptable`: Reads an offset from a global variable, indexes into a jump table at that offset, and calls the located routine with no arguments.
- `call_jumptable_routine`: Fetches an offset from a global variable, indexes into a jump table at that offset, and calls the routine located there with no arguments.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `invoke_library_function`: Loads a library base pointer from a global variable and invokes the library function located at a fixed offset within that base, then returns its result.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `indirect_call_via_global_ptr`: It reads a pointer stored in a global variable, adds an offset to it, and jumps through the resulting address to invoke a routine, returning without storing anything.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `dispatch_routine_by_number`: Looks up a value in a table indexed by the low byte of a routine number stored at a fixed offset within a structure, then jumps to the routine found at that table slot.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `set_bits_in_register`: Returns the byte currently in a data register with the bits 2, 4 and 16 set.
- `set_flag_bits_in_global`: Reads a byte from a global variable and stores the result of a bitwise OR with 18 back into that same variable.
- `setup_and_copy_blocks`: Runs a series of setup routines, then when a stored counter equals three it shifts two word tables by one slot and copies their tail entries, updating several flag fields and invoking a helper with a pair of values.
- `fill_table_and_copy_values`: It runs a two-iteration loop that stores incremented values into a table and calls another routine each time, then copies two values into a second location.
- `split_param_and_lookup`: It splits a parameter into its upper and lower halves, stores each as a character into a global variable, then looks up a resource twice and passes the result to another routine.
- `lookup_table_and_dispatch`: Searches a four-entry table for a field matching a parameter and calls two routines using the matching entry's other fields.
- `swap_and_pass_pair_2`: Swaps two consecutive four-byte values stored in a global structure and forwards the swapped values to another routine.
- `fill_and_call_table`: Loops eight times, calling a routine from a table with a fixed pair of arguments and storing each returned value into consecutive four-byte slots in a global structure, then invokes another routine eight times.
- `call_table_with_entries`: Loops eight times, calling a routine from a table with successive entries from a global structure and a fixed argument.
- `update_and_clamp_counter`: It adjusts a global counter toward a target by computing wrapped differences and clamping the result, then halves it twice.
- `split_global_byte`: Splits the low byte of a global variable into one location and the high byte into another, then resets that global variable to ten.
- `store_and_set_global`: Stores a passed value into a global variable, then overwrites that global with a fixed word and returns it.
- `save_registers_and_allocate`: Saves the caller's register values into a block of global variables, then jumps to allocate_and_map_memory with those saved values in the registers.
- `merge_zero_into_d0`: This routine is a single instruction that performs a bitwise OR of the value zero into the d0 register, which is a no-op that merges zero into d0 without changing its contents.
- `dispatch_via_jump_table`: Reads a pointer from a table indexed by a value, then jumps to the routine stored at that address.
- `dispatch_via_global_table`: Loads a value from a table indexed by a global variable and jumps to the routine stored there, returning whatever that routine returns.
- `dispatch_via_another_table`: Loads a value from a different table indexed by a global variable and jumps to the routine stored there, returning whatever that routine returns.
- `dispatch_via_io_table`: Reads an offset from a global variable, indexes into a dispatch table at that offset, and jumps to the located routine.
- `dispatch_io_via_table`: Reads an offset from a global variable, indexes into a dispatch table at that offset, and jumps to the referenced routine to run it, returning its result.
- `offset_and_call_indirect`: Adds a fixed offset to a pointer stored in a global variable, then dereferences the adjusted pointer and calls the routine located there.
- `call_jumptable_routine_2`: Reads a value from a global variable, indexes into a jump table at that value, and jumps to the routine found there, returning.
- `dispatch_via_jumptable_2`: Reads an offset from a global variable and jumps through a recovered jump table at that offset, then returns to the caller.
- `call_jumptable_offset`: Fetches an offset from a global variable, indexes into a jump table at that offset, and calls the routine located there with no arguments, returning.
- `invoke_library_function_2`: Loads a library base pointer from a global variable and invokes the library function located at a fixed offset within that base, then returns.
- `call_indirect_via_global_ptr`: Reads a pointer stored in a global variable, adds an offset to it, and jumps through the resulting address to invoke a routine, returning.
- `call_library_vector_negative`: Loads a library base from a global variable and calls the library function at a negative offset from that base, returning its result.
- `dispatch_routine_by_number_2`: Reads a value from a fixed offset within a structure, indexes into a table by its low byte, and jumps to the routine found there, returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_4`: Small routine of 1 instruction that merges 242 into d0.
- `perform_stack_manipulation`: Branches to a nearby instruction, saves registers to a stack frame, performs several moves between the stack and registers, and ends with an illegal instruction.
- `setup_and_call_interpreter`: Stores d0 into a global variable, saves the stack pointer into a global variable, stores several other global values into a table, and then calls another routine.
- `process_nested_data_tables`: Iterates through a series of pointers to execute code at those locations, performs additions to values at offsets within a table, and finally copies 12 bytes from the routine itself to a fixed memory location.
- `execute_complex_setup`: Decrements a value in d0 in a loop, performs several memory operations including a call to another routine, and pushes a value onto the stack before returning.
- `copy_longwords_then_zero_fill`: Copies 32-bit values from a source region into a destination region until the destination reaches a limit, zero-fills the remainder, saves the register values into fixed globals and returns.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
