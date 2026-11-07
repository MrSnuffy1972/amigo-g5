# Game 823 (flashback (1992)(u. s. gold)[cr infect][hd]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00823). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3655984 top-level bytes in 198 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 23 of 23 units identical (bytes and relocations), 35 raw-word lines |
| Functions named and described | pass, 265 of 265 named (0 author symbols), 265 of 265 described (24 by rule, 164 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4629 of 4630 (99.98%) |
| Recovered without load address (counted in proof 1, flagged) | 42104 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9475 of 9486 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2966577 |
| code | 33286 |
| data | 534622 |
| padding | 51355 |
| unused | 70144 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| jtype | 2 | 156 | 3 |
| Type | 1 | 426 | 2 |
| Dir | 2 | 117 | 2 |
| Disk-Validator | 1 | 386 | 1 |
| Ram-Handler | 2 | 10 | 1 |
| Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:ExpLoad | 1 | 257 | 26 |
| Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:INTERPOL | 2 | 267 | 8 |
| Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:intro | 2 | 214 | 7 |
| Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:flashback | 9 | 251 | 6 |
| explode.library | 1 | 475 | 30 |
| Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:Failat | 2 | 117 | 2 |
| Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:endrun | 1 | 117 | 4 |
| Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 500 | 14 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 714 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 99 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2837 | 91 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2398 | 40 |
| memo.cmp (raw code file, load address unknown) | 1 | 0 | 0 |
| memo.cmp (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 473 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 457 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 9 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 90 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1301 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 1559 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- flashback (1992)(u. s. gold)[cr infect](disk 2 of 4)[hd]/Flashback (1992)(U. S. Gold)[cr Infect](Disk 2 of 4)[HD].adf:cine/memo.cmp: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- flashback (1992)(u. s. gold)[cr infect](disk 4 of 4)[hd]/Flashback (1992)(U. S. Gold)[cr Infect](Disk 4 of 4)[HD].adf:cine/memo.cmp: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- flashback (1992)(u. s. gold)[cr infect](disk 2 of 4)[hd]/Flashback (1992)(U. S. Gold)[cr Infect](Disk 2 of 4)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr infect](disk 3 of 4)[hd]/Flashback (1992)(U. S. Gold)[cr Infect](Disk 3 of 4)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr infect](disk 4 of 4)[hd]/Flashback (1992)(U. S. Gold)[cr Infect](Disk 4 of 4)[HD].adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr infect](disk 1 of 4)[hd]/Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- flashback (1992)(u. s. gold)[cr infect](disk 1 of 4)[hd]/Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:flashback: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 2 never-loaded region(s), 42104 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:intro

## Copy protection found

- `initialize_system_state_2` (runtime segment an address (play-rs-m1500)): Initializes collision tables, manipulates interrupt enable bits and CIA registers, and decrements a register to zero. Kind: disk hardware/track check. Pass path: The loop continues until iVar2 != 0, at which point it proceeds to execute the loaded code/data via the function pointers in iVar1..
- `initialize_system_state` (runtime segment an address (play-rs-m1500)): Initializes collision tables and decrements a register to zero while manipulating interrupt enable bits and CIA registers. Kind: disk hardware/track check. Pass path: the branch that continues to the loop/data loading logic after the initial check, specifically avoiding the `u19__sub_4BAA` error/exit path.

## Functions


### Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `move_and_branch`: Moves a long word from one register to another and then performs a relative branch.

### jtype

- `parse_command_line_arguments`: Parses a command-line string to extract a numeric parameter following a space.
- `perform_dos_write_operation`: Executes a sequence of instructions that uses a DOS library function to write data.
- `execute_write_operation`: Performs a write operation using a DOS library function via a sequence of six instructions.

### Type

- `parse_type_command_arguments`: Parses command-line arguments for a type utility to identify specific flags such as hex or number modes.
- `format_output_string`: Formats strings and numbers into a character buffer using specific text patterns and hexadecimal characters.

### Dir

- `update_linked_list_buffer`: Iterates through a linked list to find the maximum value in a specific field, then expands a buffer by writing a header and copying data from a source to a destination.
- `move_and_branch_2`: Moves the contents of register d1 into register d0 and then performs a short relative branch.

### Disk-Validator

- `update_display_buffer`: Calculates character offsets and populates a memory buffer with numeric string representations of values.

### Ram-Handler

- `execute_indirect_call`: Performs an indirect call to a routine located via a pointer in a global table.

### Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:ExpLoad

- `initialize_expansion_loader`: The routine sets up a small stack frame and calls a loader function from a table pointed to by a global variable.
- `perform_conditional_branching`: Stores the stack pointer and a short word into memory offsets before executing a subroutine and performing multiple conditional branches based on the processor status register.
- `store_and_call_subroutine`: Stores a longword from a register into a memory location and then calls another routine after pushing the current program counter onto the stack.
- `search_library_name`: Uses a system library call to find a name and performs several conditional checks and subroutine calls based on the results.
- `execute_conditional_branches`: Performs a series of conditional byte and flag checks followed by a branch to another routine.
- `store_and_call_routines`: Stores a long value into a memory offset and executes two separate subroutines conditionally.
- `execute_conditional_calls`: Performs several conditional branches and calls other routines while manipulating the stack and address registers.
- `call_hook_and_clear_flags`: Checks a pointer stored in a global variable; if non-zero, it invokes a function pointer located at an offset from that address.
- `clear_flag_and_call_hook`: The routine checks a flag stored at an offset from a global variable using a short index in D0.
- `call_indirect_function`: Calls another routine with a modified argument and then executes a function pointer retrieved from an offset within a table located in chip RAM.
- `wrap_subroutine_call`: Pushes the current stack pointer and a register onto the stack, calls another routine, and then restores the register and the stack pointer.
- `call_open_library`: Wrapper that calls the exec library function OpenLibrary and returns its result to the caller.
- `setup_stack_and_call`: Creates a stack frame and calls another routine.
- `test_signal_and_callback`: Calls another routine to test a signal flag and subsequently calls a second routine while checking a zero flag.
- `call_indirect_via_table`: The routine calls another routine through an indirect pointer obtained from a global variable and returns a 64-bit value formed by concatenating the two input registers.
- `store_byte_and_advance_pointer`: Stores a byte from a register into the memory location pointed to by a global pointer.
- `store_return_values`: The routine calls another routine, captures its return value and an extra output pointer, and stores both into two consecutive words of a caller-provided structure via a pointer in A4, then returns the original input value in D0.
- `load_address_from_stack`: Loads an address from an offset relative to a stack pointer into a register.
- `parse_string_table_entry`: The routine reads a zero-terminated string from a pointer table, advances the pointer past the string and returns the byte offset of the next entry.
- `set_signal_flag`: The routine calls another routine and tests a returned value for a specific bit.
- `clear_flag_in_structure`: The routine calls another routine and then writes a constant to a field of a structure pointed to by a register.
- `check_and_invoke_callbacks`: The routine calls another routine and then checks a zero flag.
- `store_parameter_to_offset`: Stores a passed parameter into a memory location at a fixed offset from a register.
- `call_indirect_via_global_table`: The routine calls another routine through a pointer stored in a global variable.
- `count_string_bytes`: The routine receives a pointer in a0 and walks forward byte by byte until it finds a zero byte, then returns the number of bytes from the start pointer to the terminating zero.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.

### Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:INTERPOL

- `parse_amiga_hunk_data`: Parses a memory buffer containing AmigaDOS hunk type codes to allocate and relocate data segments.
- `decrement_word_until_wrap`: Decrements a 16-bit register in a loop until it wraps past zero, then returns.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `interpolate_bitstream_data`: Processes a bitstream by performing bitwise shifts and copying data between memory locations based on conditional logic.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `process_bitstream_interpolation`: Performs bitwise manipulation and shifts on data from a table and updates a color register.
- `perform_trap_operation`: Moves a word from a pointer, performs a bitwise OR, and executes a software interrupt.

### Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:intro

- `manage_memory_and_data_expansion`: Allocates and frees memory while performing conditional data copying and run-length byte expansion.
- `decrement_until_wrap`: Decrements a 16-bit value in a register until it wraps around to zero.
- `decrement_word_until_wrap_2`: Decrements a 16-bit value in a register until it wraps around to zero, then returns.
- `copy_data_conditionally`: Iterates through memory to copy data from one location to another based on conditional checks and register values.
- `store_word_conditionally`: Checks if the lower seven bits of a register are zero and, if so, stores a word from a pointer into a global variable.
- `decrement_and_loop`: Decrements a register in a loop until it reaches a negative value, calling another routine during each iteration.
- `expand_run_length_bytes`: Copies bytes from an input to an output, expanding bytes that match a specific marker by reading a count and repeating that byte.

### Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:flashback

- `free_blocks_and_restore_patch`: Calls two unnamed routines, loops to free five memory blocks, and restores a saved word in a table if a specific masked pattern is matched.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.
- `copy_and_cache_clear`: Copies a block of memory and performs a cache clear using a system library call.
- `load_address_and_value`: Loads an address into a register using the program counter and moves a small constant into another register.
- `copy_memory_block_2`: Copies a block of memory from a source address to a destination address using a loop.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### explode.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `handle_message_request`: Reads a request structure from the caller, resolves a dos.library base via an indirect call and stores a handle, then extracts a short flag and a pointer from the request, and dispatches based on the flag value using table lookups, storing a result pointer and invoking further library routines before returning.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_and_test_bits`: As library code, it decrements a word at an offset from a pointer and branches if the result is non-zero, then tests a bit at another offset from the same pointer and branches if the bit is set.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_update_library`: Library code that checks magic values and function pointers within a structure to verify its integrity.
- `set_library_bit`: As library code, it performs a bitwise OR operation to set a specific bit in a byte at an offset from a pointer.
- `jump_to_handler`: Calls a subroutine and then performs a short relative branch.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `validate_library_string`: This routine retrieves a pointer and checks if the memory at that location contains specific four-byte character sequences.
- `check_library_name`: The routine receives a pointer in a0 and checks whether the four words pointed to by that pointer spell a library name string.
- `decrement_timer_and_trigger_explosion`: Library code that decrements a short timer in a structure, calls a routine to handle an effect, and calls a function pointer from a global table if a threshold is met.
- `initialize_data_structure`: Populates a data structure by copying values from a source buffer and calculating bit-shifted scaling factors.
- `copy_memory_unrolled`: Copies a block of memory using an optimized loop that transfers eight words at a time, followed by a byte-by-byte loop for any remaining data.
- `check_pointer_and_decrunch`: Library code that compares two pointers; if they are equal, it calls a routine to decrunch a block of data.
- `copy_memory_block_unrolled`: Copies a block of memory using an unrolled loop that transfers data in eight-word increments for efficiency.
- `copy_memory_block_4`: As library code, this routine performs a loop that copies data from one memory location to another, decrementing a counter until the transfer is complete.
- `branch_to_subroutine`: As library code, this routine calls another subroutine and then performs an unconditional branch to another location.
- `prepare_decompression_parameters`: As library code, this routine saves registers to the stack, moves a value from a register to another, and loads three bytes from a memory location pointed to by a register before performing conditional branches.
- `decode_variable_length_bitstream`: Reads a sequence of bits from a memory location by shifting a value and decrementing a pointer whenever a zero bit is encountered.
- `expand_bitstream_data`: Processes a bitstream by reading bits from a source pointer and expanding them into a destination buffer using a variable-length decoding scheme.
- `initialize_library_constants`: This library routine loads small constant values into two registers and performs a relative branch.
- `initialize_loop_counters`: Loads two small constant values into registers and performs a relative branch to another location.
- `initialize_loop_counters_2`: Sets two registers to small constant values and performs a relative branch to another location.
- `decode_bitstream_data`: Processes a stream of bits by reading from a source pointer and expanding them into a destination buffer.
- `process_bitstream_data`: This library routine performs arithmetic on bytes and transfers a byte from a pointer into a register.
- `decompress_bitstream_data`: This routine implements a bitstream decompression algorithm, likely a variation of Huffman or LZ-based decoding, to expand compressed data into a destination buffer.
- `decompress_bitstream_data_block`: Expands a compressed bitstream into a destination buffer using variable-length coding.
- `execute_callback_loop`: The routine reads a counter from a structure, increments it, and repeatedly calls another routine and a callback taken from a table indexed by a global variable until the counter condition is met, then calls the routine once more and clears a word in memory.
- `decrement_loop_counter`: The routine receives an unsigned value in an input register, converts it to signed, subtracts one, and if the result is positive, repeatedly decrements it until it reaches -1.

### Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:Failat

- `update_linked_list_and_copy_buffer`: Iterates through a linked list to find the maximum value in a specific field, then writes a header and populates a buffer with incrementing values.
- `transfer_register_and_branch`: Moves a longword from one data register to another and then performs a short relative branch to another location.

### Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:endrun

- `initialize_system_structures`: Initializes various data structures by zeroing specific offsets and setting maximum values.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `execute_infinite_loop`: Enters an infinite loop that performs no actions.
- `initialize_structure_fields`: This routine writes zero and maximum values to specific offsets within a structure based on a register value.

### Flashback (1992)(U. S. Gold)[cr Infect](Disk 1 of 4)[HD].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant_2`: Returns the constant -1 in d0 to the caller without any other effect.
- `configure_hardware_and_globals`: Sets several global variables and writes values to hardware registers to configure DMA, interrupts, and coprocessor registers.
- `wait_for_bit_clear`: Polls a byte at an offset from a pointer until a specific bit is cleared.
- `configure_dma_and_interrupts`: Sets hardware registers to enable DMA and interrupts while updating global state and calling another routine.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 64 into d0, then merges 112 into d0.
- `initialize_system_hardware`: This routine polls a global variable, modifies hardware registers for DMA and interrupts, and calls several subroutines to populate data tables and initialize structures.
- `wait_for_values`: Polls a memory location until a specific bitmask matches a value, and then polls another location until a specific character is no longer present.
- `populate_data_table`: Stores a series of values into a table by iterating through a loop and writing two-byte segments derived from a long word.
- `initialize_data_structure_tables`: Sets a global variable to a constant value and calls three subroutines to populate various data tables through arithmetic and bitwise operations.
- `zero_d0_register`: Performs a logical OR operation on the least significant byte of the d0 register using a zero immediate value.
- `populate_data_table_2`: This routine performs complex arithmetic to calculate offsets and populates a structure with various constant values and pointers.
- `parse_encoded_data_table`: Iterates through a data table using an index, performing arithmetic on bytes to calculate offsets and populate a structure with derived values.
- `update_bitmask_table`: Iterates through a set of coordinate pairs to calculate offsets and performs bitwise XOR operations on a target memory buffer.
- `fill_table_with_increments`: Fills a table with values starting at zero and increasing by a constant amount for a specific number of entries.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_audio_sample_data`: Processes a table of audio sample parameters to populate a buffer with calculated volume and pitch values.
- `initialize_hardware_registers`: Writes several constant values to a global variable and modifies a value in memory using a bitwise AND operation.
- `update_scaled_audio_parameters`: Updates a set of global variables by performing bit-shifted multiplications using a scaled value and a series of pointers from a table.
- `update_global_values`: Stores several constant values and a calculated sum into a global variable.
- `configure_audio_channels`: Writes a series of memory addresses into a table and updates a global variable.
- `handle_arithmetic_overflow`: This routine processes an overflow condition by executing sixteen instructions, performing a two-byte addition, and updating a pointer and a value if a specific condition is met.
- `execute_overflow_logic`: This routine executes three different subroutines related to handling overflow states and overflow calls.
- `handle_overflow_calls`: This routine executes an overflow call and subsequently calls an overflow handler.
- `execute_overflow_call`: This routine consists of eleven instructions and calls an overflow handler routine.
- `update_overflow_state`: This routine calls a routine to update a pointer and a value via two-byte addition and then calls an overflow handler routine.
- `update_overflow_pointer`: Performs a two-byte addition and updates a pointer and a value if a specific condition is met.
- `call_overflow_handler`: Executes sixteen instructions and calls a routine that checks a value, performs a two-byte addition, and updates a pointer and a value if an overflow occurs.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `clamp_and_store_byte`: Reads a byte from a source, clamps its value to a maximum of sixty-four, and stores it in a destination offset.
- `store_byte_with_nonzero_default`: Copies a byte from the global block into a field of the caller's record, storing one instead when that byte is zero.
- `increment_and_offset_values`: Adds an offset to a byte from a source and stores it as a word, then increments a short value at a different offset.
- `update_calculated_values`: Stores several constant values into global variables, calls another routine, and performs bit-shifted multiplications to store results into a table.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `process_overflow_and_pointer`: Checks a value, adds two bytes together, and if an overflow occurs, updates a pointer and modifies a value at that pointer.
- `mask_byte_and_branch`: Loads a byte from a memory location, masks it with a constant value, and branches if the result is non-zero.
- `advance_step_and_latch_entry`: Advances a step counter and, when it reaches the value passed in a data register, copies a table entry into the output pointer and sets a bit in a shared flag word.
- `copy_word_on_condition`: Checks two conditions and, if both are met, copies a word from a source to a destination offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_function`: The routine performs a library call using a function index.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_audio_volume_registers`: Sets multiple audio volume registers to specific values and clears a block of memory using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_global_with_conditional_xor`: Multiplies a global variable by two and performs a conditional bitwise XOR with a constant if the multiplication results in an overflow or zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_protection_screen`: Clears screen buffers, draws a specific shape for copy protection, and handles text output.
- `increment_and_test_byte`: Increments a byte in a global variable and compares it to a constant.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d3.
- `protection_draw_shape`: Copy protection: clears both screen buffers and draws the selected code-wheel shape in the protection screen's colours.
- `decode_message_table`: The routine walks a table of messages indexed by an input value and recursively decodes entries, using flags in each entry to select a character and to set two global variables, then stores the resulting character into a global variable and updates a pointer.
- `clear_and_iterate_table_entries`: The routine clears several global variables and then walks a table pointed to by A6.
- `process_data_table`: Iterates through a table of values, performing arithmetic on each entry and storing modified characters into a destination buffer.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies d0 to (a0)+, then copies (a7)+ to d5, then returns.
- `preserve_and_shift_word`: Pushes a word onto the stack, shifts it right by eight bits, calls another routine, and restores the original word to a register.
- `prepare_nibble_storage`: Pushes a byte to the stack, shifts it right by four bits, calls another routine, and then restores the original byte from the stack.
- `store_nibble_indexed_byte`: The routine stores a byte from a register into a global variable table indexed by the low nibble of another register value.
- `fill_bit_reversal_table`: The routine writes 256 bytes to a global variable, treating it as a table.
- `expand_palette_table`: The routine fills a 256-entry table in memory with 16-bit values derived from an 8-bit index by mapping each bit to a 2-bit nibble in the high and low bytes.
- `generate_palette_from_index_table`: The routine builds a 256-entry table in a global variable by iterating over all 256 byte indices and expanding each index into a 32-bit value whose four bytes are derived from the four high and low nibbles of the index.
- `expand_compressed_data_stream`: Iteratively processes a compressed data stream by reading encoded values and expanding them into a destination buffer.
- `process_compressed_data_stream`: Decompresses a data stream by reading a header and iteratively expanding encoded values into a destination buffer.
- `update_globals_and_copy_memory`: Checks a global variable against a register, copies a memory block if a condition is met, and then increments two global variables by a register value before calling a handler through a table pointer.
- `save_registers_and_set_bit`: Saves multiple registers to the stack, loads a long value from a global variable, and sets a bit in a register before branching.
- `clear_global_variable_on_zero`: The routine calls another routine and tests the returned value.
- `reset_global_flag_and_call_hook`: The routine checks a global variable and, if it is non-zero, calls another routine and then clears the global variable to zero.
- `clear_pending_flags`: The routine calls another routine unconditionally and then checks ten global variables in order.
- `clear_flags_and_return_value`: The routine checks two global variables and, if both are non-zero, calls another routine and clears the first flag.
- `parse_object_property_table_2`: Iterates through a structured data table to extract and validate various properties.
- `parse_object_property_table`: Iterates through a structured data table to extract and validate various properties.
- `process_string_table`: This routine iterates through a table of pointers, performs conditional checks and calls, and stores several constant long words into global variables.
- `parse_and_index_string_table`: Iterates through a null-terminated string list to build a table of pointers and associated length values.
- `parse_string_table`: Iterates through a null-terminated string to build a table of pointers and associated length values.
- `allocate_and_process_list`: Requests memory from the system if a global variable is zero, then iterates through a data list to update a table of pointers and lengths.
- `process_data_list`: Traverses a list of data, performing operations on elements and updating a global variable and a table of pointers and lengths.
- `process_compressed_data`: Checks a global variable against several constants and performs a series of pointer arithmetic operations and conditional branches to update a global variable based on values retrieved from a table.
- `execute_conditional_subroutines`: Pushes a value to the stack, loads two different pointers and calls a subroutine twice, and then branches based on the result of the second call.
- `copy_conditional_data`: This routine loads a pointer and a value from memory, performs arithmetic on a register, and branches based on whether a value is positive.
- `copy_and_branch_data`: Copies a block of bytes from a source pointer to a destination pointer if a value in a table is negative.
- `copy_memory_block`: Copies a block of bytes from a source pointer to a destination pointer using a loop.
- `transfer_and_branch_data`: Performs a series of conditional checks on a global variable and then moves data from one memory location to another before branching.
- `return_uninitialized_stack_value`: This routine returns the value currently located at the top of the stack.
- `copy_and_branch`: Moves two longwords to specific memory locations and then branches to a new address.
- `copy_memory_with_offset`: Calculates an offset from a value, then copies a block of memory using a loop.
- `copy_memory_block_3`: Checks a global variable against a register, then copies a block of memory using a calculated offset.
- `self_modifying_dispatch`: The routine checks a global byte flag and, if set, reads a byte from a table indexed by the flag value.
- `increment_two_globals_and_call_handler`: The routine adds the value in D0 to two global variables and then calls a handler routine through a pointer stored in a table.
- `decrement_counter_and_call_handler`: The routine subtracts the value in D0 from a global variable and then calls a handler routine obtained from a table in RAM. It then returns.
- `write_debug_strings_to_log`: The routine writes a sequence of ASCII strings into memory via an indirect call, using a global variable and several RAM locations as a buffer, and then repeatedly calls another routine until a zero terminator is cleared.
- `remove_unreachable_blocks`: A generated decompiler stub for a non-existent routine.
- `initialize_graphics_subsystems`: Clears a large memory range to zero, initializes several lookup tables for bit reversal and palette expansion, and configures hardware registers and interrupt handlers for double buffering.
- `initialize_ram_table`: Writes a series of values and pointers from registers and the stack into a large block of RAM, then executes a routine at a RAM address.
- `set_ram_pointer`: Sets a global variable and then executes a routine through a pointer stored in RAM.
- `reset_interrupt_state_2`: A routine consisting of eleven instructions that is called by a reset interrupt state handler.
- `execute_ram_callback`: Sets a global variable and then performs an indirect call to a routine via a RAM pointer.
- `clear_screen_buffer`: Clears one 320x224 four-plane screen buffer (a0) with the blitter.
- `update_system_state_and_palette`: Updates hardware status, input flags, and palette entries by reading hardware registers and modifying a table.
- `set_up_sprite_table`: The routine writes a set of 32-bit values split into 16-bit halves into a structure pointed to by a global variable, using a base offset and an index derived from a global flag.
- `update_hardware_status`: Reads hardware registers and memory, modifies specific bits in a table based on those values, and stores a constant in a global variable.
- `write_constant_to_offset`: Stores the value eight into a memory location calculated by adding an offset to a register.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `take_over_hardware`: Saves the system interrupt vectors and INTENA, sets up the double screen buffers, installs the game's level 1 and level 2 handlers and enables its interrupts.
- `copy_word_table_until_marker`: The routine copies 16-bit words from a source pointer to a destination pointer until a sentinel word is encountered, then clears a global variable, stores the destination pointer into a global variable and another global variable, calls another routine, and sets a global variable to a constant value.
- `tiny_and`: Small routine of 1 instruction that masks d0 with d0.
- `tiny_ori_3`: Small routine of 1 instruction that merges 112 into d0.
- `apply_bit_mask`: Performs a bitwise OR operation on a register using a constant mask.
- `set_data_register_bits`: Performs a bitwise OR operation on a byte in a data register using a constant value.
- `tiny_ori_4`: Small routine of 1 instruction that merges 64 into d0.
- `clear_data_d0`: Performs two logical OR operations with zero on a byte in register d0.
- `swap_buffers_and_wait_frames`: Ends a frame: swaps the front and back screen buffers and waits until the vertical-blank counter reaches the frame length (the game's frame pacing), then updates the copper list.
- `call_open_library_2`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `clear_byte_table`: The routine clears a 256-byte block starting at a global variable by writing zero to each byte.
- `update_input_state_flags`: The routine reads a byte from another routine, updates a global state variable through a small state machine, and sets bits in the byte based on mismatches between pairs of RAM bytes.
- `read_input_device_state`: The routine reads the current input device mode from a global variable and combines button and mouse information from CIAA_PRA, JOY0DAT and POTGOR into a single status word.
- `clear_data_d0_2`: Performs a logical OR operation with zero on a byte in register d0.
- `clear_data_register_byte`: Performs a logical OR operation on a byte in a data register using zero to effectively clear the register's contents.
- `wait_for_interrupt_flag`: The routine spins until a specific bit in a global variable is set, then returns the value of that bit.
- `configure_bitplane_dma`: The routine writes a series of values to a set of global variables and then calls another routine multiple times, passing adjusted pointers derived from the outputs of those calls.
- `initialize_interrupt_handlers`: The routine writes constants and pointers into a global variable area, setting up a table of handler addresses and a RAM reference.
- `transform_palette_entries`: Iterates through a source table to bit-swap components of sixteen words and stores the results into a destination table and a global variable.
- `swap_palette_entries`: The routine copies sixteen 16-bit palette entries from a source table into a destination table and writes them to the Amiga custom chip color register.
- `rotate_color_table`: The routine copies sixteen 16-bit color entries from a global table to a hardware color register and to another memory location, rotating the byte order of each entry.
- `swap_nibble_bytes`: The routine copies sixteen 16-bit words from a source pointer to a destination pointer, reordering the bytes within each word.
- `swap_nibble_bytes_2`: The routine copies sixteen 16-bit words from a source pointer to a global table, reordering each word’s bytes and nibbles by masking and shifting the bits.
- `read_joystick_buttons`: The routine reads the joystick/mouse fire buttons from CIAA_PRA, the mouse movement from JOY0DAT and the right/middle buttons from POTGOR, combines the values with a global variable and stores the results into a few global variables.
- `check_byte_range_condition`: The routine compares two byte values and a signed byte value, returning early if they are equal or if certain arithmetic relations between them hold.
- `tiny_move`: Small routine of 1 instruction that copies (a6) to (a3)+.
- `clear_data_register_byte_2`: Performs a logical OR operation on a byte in a data register using a zero value.
- `clear_data_register_byte_3`: Performs a logical OR operation on a byte in a data register using a zero value.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 117 into d0, then returns.
- `call_other_routine_with_global`: The routine calls another routine, passing the address of a global variable twice, and then returns.
- `draw_text`: Draws a zero-terminated string with the 8x8 game font at x, y in a colour into a screen buffer; '/' starts a new line.
- `reset_interrupt_state`: The routine clears a global variable and then writes a series of values into a global variable and a small table of four words, followed by copying a value from another global variable into two locations and finally zeroing the global variable.
- `set_display_dma_control`: The routine writes to Amiga custom chip registers that control display DMA and copper list pointers, and it sets a global variable to a constant value before copying a value from another global variable into a third global variable and then clearing the first global variable three times.
- `load_and_parse_file_table`: This routine loads a file and iterates through a table of structures, calculating offsets based on size fields stored in the data.
- `populate_pointer_table`: The routine walks a byte table pointed to by a global variable and fills an array of pointers with successive entries, advancing the source pointer by a size derived from the entry bytes.
- `calculate_scaled_value`: Saves registers to the stack, writes a value to a memory location, multiplies a value from memory by a constant, and then branches based on a byte test.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_move_rts`: Small routine of 2 instructions that copies (a7)+ to d7, then returns.
- `calculate_blitter_tile_offsets`: Calculates source address offsets and dimensions for blitter operations based on tile indices and screen coordinates.
- `clear_blitter_state`: The routine writes a series of constant and register values to a set of global variables and then calls another routine.
- `clear_sprite_table_entries`: The routine iterates over a table of sprite descriptors and clears their entries.
- `wait_for_cpu_clear_flag`: A routine that spins until a global variable’s bit is cleared and then writes a constant to the same global variable.
- `initialize_system_state_2`: Initializes collision tables, manipulates interrupt enable bits and CIA registers, and decrements a register to zero.
- `initialize_system_state`: Initializes collision tables and decrements a register to zero while manipulating interrupt enable bits and CIA registers.
- `decrement_register_to_zero`: decrements a value in a data register until it reaches zero.
- `call_old_open_library`: Library stub: loads its arguments and the exec library base, calls OldOpenLibrary and returns its result to the caller.
- `initialize_collision_tables`: Clears and seeds a set of collision-related tables referenced through a global variable.
- `bytekiller_unpack`: Depacks a data file packed with the ByteKiller-style coder: reads 32-bit words backwards from the end (unpacked size, checksum, bit stream), copies literal runs and back references to the output backwards and keeps an XOR checksum.
- `update_sine_table_entries`: The routine updates a global sine table using fixed-point arithmetic and a short table pointed to by a global variable.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `multiply_fixed_point`: The routine multiplies a 16.16 fixed-point value in a register with a 16.16 fixed-point value from a global variable and returns a 32-bit result in the high part of the return value while preserving a separate input value in the low part.
- `set_bitmask_range`: Calculates a rectangular bounding box defined by two coordinate pairs and iterates through the area to modify bits within a grid of words.
- `update_bitmask_grid_cells`: Iterates through a two-dimensional grid structure and modifies specific bits within word-sized elements.
- `update_arithmetic_values`: Performs conditional branching based on a word comparison and adds various registers together.
- `apply_bitmask_to_grid`: Iterates through a grid structure and applies bitwise masks or bit-setting operations using a bitmask and a pointer.
- `draw_scanline_segment`: The routine computes a set of integer values from its parameters and then repeatedly calls another routine while updating counters and two accumulators.
- `call_callback_with_threshold_check`: The routine calls another routine via a function pointer and then compares a short value with a short stored at an offset from a pointer.
- `empty_stub_routine`: The routine is 13 instructions long but performs no observable work; it does not read or write memory, does not access hardware registers, and does not call another routine.
- `store_indexed_pair`: The routine checks a signed index against a global limit and a base value, computes a pointer into a table using the index and a base pointer, clamps two input values to non-negative and an upper bound from a global variable, then stores the two clamped values as a short pair at the computed table entry.
- `calculate_matrix_transformation`: Performs a 2D matrix multiplication using values fetched from two tables via indexed offsets and stores the resulting products in global variables.
- `line_draw_noop`: Empty draw routine (rts) chosen by the 3D line setup when a segment needs no drawing; its address is loaded into the draw-routine pointer.
- `validate_value_within_bounds`: Iterates through a table of values to determine the minimum and maximum elements while performing word-swapping operations.
- `check_range_and_return_flag`: The routine calls another routine, then reads a 32-bit value from a global variable table using an index derived from two globals, extracts its high 16 bits as a signed short, and compares that value with another global.
- `find_table_extrema_and_bounds`: Iterates through a table of values to determine the minimum and maximum elements while performing word-swapping operations.
- `analyze_table_extrema_and_bounds`: Iterates through a table of values to determine the minimum and maximum elements while performing word-swapping operations.
- `calculate_word_difference_and_swap`: Loads a long value from a pointer and subtracts its lower word from a value stored at that pointer.
- `check_signed_value`: Checks if a value is negative by testing its sign bit.
- `process_table_entry`: Retrieves a long value from a pointer, performs word-based subtractions, and swaps register word halves.
- `verify_sign_extension`: The routine loads a 16-bit value from a register and tests whether the sign bit is preserved when the value is shifted left by eight bits.
- `calculate_and_populate_data`: Calculates several values based on the difference between two pairs of inputs and uses them to populate a structure by copying entries from a table.
- `load_constant_and_branch`: Loads a word into a register and performs a relative branch.
- `compare_and_exchange_registers`: Negates a word, compares it to another, and conditionally exchanges two registers before branching.
- `populate_data_structure`: Copies entries from a table into a structure and calculates several offset values based on input parameters.
- `copy_table_to_struct`: The routine copies a sequence of entries from a table pointed to by a pointer into fixed offsets of a structure pointed to by a pointer.
- `set_table_pointer_by_index`: The routine receives a value in D0 and uses the low 16 bits as an index.
- `clear_register_d0`: Performs a bitwise OR operation on register d0 with zero.
- `update_blitter_registers`: Modifies various Blitter hardware registers and the DMA control register.
