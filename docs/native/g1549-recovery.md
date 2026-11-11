# Game 1549 (helter skelter (1990)(audiogenic)[cr skr][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01549). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901808 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 214 raw-word lines |
| Functions named and described | pass, 96 of 96 named (11 author symbols), 96 of 96 described (5 by rule, 79 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 336 of 336 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 326122 bytes, 68 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3115 of 3242 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 26237 |
| code | 11286 |
| data | 347441 |
| padding | 1772 |
| unused | 515072 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:Type | 2 | 117 | 2 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs | 1 | 205 | 12 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 159 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs.img (raw code file, load address unknown) | 1 | 2747 | 68 |

## Cases handled

- runtime segment an address: 133 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- helter skelter (1990)(audiogenic)[cr skr][a]/Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs.img: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 326122 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_display_and_copy_buffers` (hs.img (raw code file, load address unknown)): Configures display registers and copies or initializes memory buffers or tables. Kind: disk/track/hardware check (ADKCON/CIAB_PRB). Pass path: an address.w == d0.

## Functions


### Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: Copies sixteen bytes of data from a memory location offset from a pointer to a destination address.

### Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs

- `start`: Program entry that opens the DOS library and allocates memory, clears colour and interrupt-related registers, sets display window and bitplane registers and builds the copper list.
- `copy_trampoline_and_jump`: Copies three longwords from the transfer block into a global area, then jumps to that area as code.
- `trans1`: A four-instruction block that one unnamed caller reaches by a pc-relative address.
- `loadfile`: The routine calls a function through a pointer stored at an offset from a6, tests the returned integer, and if non-zero calls two further functions through pointers at other offsets from a6, passing the first result to the second and its result to the third.
- `dopiccy`: Loads a file, fades the display down, decompresses the picture and then runs sixteen vertical-blank waits, each followed by a fade step.
- `dopiccy1`: Decompresses picture data and then runs sixteen vertical-blank waits, each followed by a fade step.
- `waitabit`: Waits for one hundred and fifty vertical blanks by calling the vertical-blank wait routine in a countdown loop.
- `decompress`: Walks a chunked picture file, reads the palette chunk into a sixteen-entry palette and then run-length decodes the body into the image buffer row by row.
- `fadedown`: The routine waits for scanlines and updates the palette fade repeatedly.
- `dofade`: The routine reads 32 palette entries from chip memory and writes them to the Amiga color registers.
- `wvbl`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `makecoplist`: Writes a copper list of four bitplane-pointer pairs spaced forty bytes apart and terminates it with end-of-list words.

### Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_system_tables`: Initializes several memory tables with pointers and values, while configuring audio hardware, DMA, and interrupts.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `copy_pointer_list`: Traverses a linked list of pointers and copies a value from each node to a destination buffer.
- `unroll_bit_stream_to_registers`: This routine checks a bitmask derived from a global variable and, for every set bit, updates a pair of hardware or memory-mapped registers using a pointer-based lookup or a linked-list-style traversal through a table or routine structure.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `update_audio_buffer_list`: Iterates through a bitmask-controlled sequence of pointers to copy data into audio hardware registers.
- `setup_data_structures`: Populates two different tables with specific values and pointers, and modifies several global variables.
- `update_table_entry`: Searches a table for a specific pointer and, if not found, updates a table entry with a value and its preceding word.
- `initialize_function_pointer_table`: Populates a table with alternating entries of a routine address and a constant value, then assigns that routine's address to a global variable.
- `wait_for_pointer`: Enters a loop that waits until a specific pointer is found within a table.
- `update_list_with_mask`: Iterates through a table and copies entries to a destination, performing bitwise shifts or arithmetic on data based on a threshold.
- `execute_unnamed_routine`: The routine executes a fixed number of instructions and accesses certain memory or registers, but the decompiler output is insufficient to determine its specific logic.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine`: The routine is empty and contains no instructions.

### Helter Skelter (1990)(Audiogenic)[cr SKR][a].adf:hs.img (raw code file, load address unknown)

- `initialize_memory_tables`: Initializes a memory-resident table with incrementing or constant values and clears several global or memory-based flags and pointers.
- `wait_for_condition_and_limit`: Waits for a specific bit in a memory location to become set or checks a global variable against a value in a table, returning if a certain condition is met.
- `clear_global_variable`: Sets a global variable to zero multiple times.
- `clear_global_variable_2`: sets a global variable to zero.
- `update_global_with_math`: Performs arithmetic or bitwise operations using values from a table and stores the result into a global variable.
- `process_graphics_list`: Iterates through a list of pointers, using a lookup table to calculate source and destination addresses, then copies memory blocks between them.
- `update_bitmapped_graphics_layer`: Calculates a destination pointer and bit-shift amount based on a lookup table, then iterates through a source list to perform bitwise OR and masking operations on a target buffer.
- `fill_memory_with_register`: Fills a block of memory with the contents of a register using a pointer.
- `modify_memory_with_mask`: Iterates through a list of memory locations, masking certain bits and multiplying the result by two.
- `expand_memory_block`: Calls another routine and then copies a block of memory from a source pointer to several different destination pointers, followed by a second loop that copies a block or clears a range.
- `calculate_digit_mapping`: Calculates a value based on a global variable, performs a digit-swapping or masking operation, and calls another routine with a constructed parameter.
- `transform_index_and_swap_digits`: Calculates an offset-based index, performs a digit-swapping bitwise transformation on a value derived from that index, and passes the result to another routine.
- `copy_memory_blocks`: Copies a block of data from a source table to two different destination tables.
- `sync_memory_buffers`: Copies a block of data from one source pointer to two different destination pointers.
- `find_null_terminator`: Iterates through a table until a null or zero-valued word is encountered.
- `update_decoding_table`: Calculates a bitwise value and stores the individual bytes of that value into a large memory buffer and a secondary offset buffer.
- `apply_xor_mask_to_buffer`: Applies a bitwise XOR mask and stores the resulting bytes into a primary buffer and a secondary offset buffer.
- `update_value_with_carry`: Performs arithmetic or bitwise operations on a byte or a pointer, involving a carry or a checksum-like adjustment, and updates a certain memory location.
- `repeat_apply_mask`: Executes another routine a fixed number of times in a loop.
- `apply_masked_data`: Decrements a global variable and, if certain conditions are met, writes masked or modified data to a range of memory buffers.
- `apply_masked_buffer_update`: Calls another routine twice and then performs a loop that applies a bitwise mask to data, storing the result into two separate large memory buffers at specific offsets.
- `write_xor_buffer_v_p_ptr`: Iterates through a source pointer, applies a bitwise XOR mask, and writes the resulting bytes to a primary and secondary buffer.
- `write_xor_buffer_derived_ptr`: Calculates a source pointer based on a global variable and a mathematical offset, then writes XORed bytes to a primary and secondary buffer.
- `traverse_string_buffer`: Iterates through a pointer until a null byte is encountered, calling another routine for each byte.
- `traverse_pointer_list`: Iterates through a list of pointers until a negative or terminal value is encountered.
- `patch_hunk_relocation_data`: Iterates through a range of memory defined by a hunk-type structure to apply bitmask-based bit-setting or bit-clearing to a set of destination buffers.
- `decode_string_and_copy`: Decodes a specialized string format by reading or copying bytes based on certain values and writes the result to a destination buffer.
- `clear_interleaved_table_values`: clears a sequence of words to zero, skipping a fixed number of bytes between each write.
- `initialize_struct_members`: Writes a series or constant values to a memory structure pointed to by a register.
- `empty_loop_routine`: The routine is empty and consists only of an infinite loop.
- `empty_loop_routine_2`: the routine executes an infinite loop without performing any operations or modifying any registers.
- `initialize_and_copy_buffer_tables`: Initializes a block of memory with zeroed or constant values and copies data between several memory-mapped or or large buffer structures.
- `run_repeated_routine`: Executes another routine a fixed number of times using a loop.
- `process_digits_loop`: Iteratively calculates a value based on a global variable and calls another routine while processing or splitting digits.
- `execute_repeated_routine`: executes another routine a fixed number of times.
- `iteratively_transform_and_pass_data`: Iteratively applies a mathematical transformation involving division and remainder to a global variable and passes the result to another routine.
- `select_and_store_pointer_value`: updates a flag in a global structure and selects a word from a table or an offset-displaced table based on a conditional check, then stores either the first or second word of the selected location into a target pointer or value.
- `initialize_conditional_struct`: Checks a global variable and writes a set of constant values or values from a pointer-offset to a memory structure.
- `process_pointer_table`: Iterates through a table of pointers, and for each, populates a structure with a calculated pointer and a flag.
- `apply_to_string_data`: Iterates through a null-terminated string and calls another routine with an incrementing value for each character found.
- `traverse_null_terminated_string`: Iterates through a string starting at a fixed address until a null terminator is found.
- `configure_display_and_copy_buffers`: Configures display registers and copies or initializes memory buffers or tables.
- `find_base_address`: Loops through memory until the pointer or index returns to the start of a global variable.
- `execute_recursive_routine_calls`: Executes a routine twice and then, if a certain global variable is non-zero, executes it two additional times, returning the last result in a register.
- `search_string_for_value`: Searches through a string for a terminator and returns a byte from an offset relative to that terminator if found.
- `traverse_string_and_call`: Traverses a null-terminated string and calls another routine for every byte encountered.
- `update_mask_tables`: Copies blocks of data from a source pointer to a destination, then applies a bitwise mask to several global memory locations.
- `mask_table_values`: Performs a bitwise AND between a register and several specific memory locations, while also ORing a value into certain table entries.
- `expand_and_copy_data`: Copies and expands a small block of data from one memory location to a larger, or staggered, destination area.
- `copy_memory_block`: Iteratively copies large blocks of data from a source address to a destination address, where the source address is determined by a value in a table.
- `increment_returned_value`: calls another routine and returns the result incremented by one.
- `transform_and_call_digit`: Constructs a value using a register and a global variable, performs a digit-swapping or masking operation, and calls another routine with a constructed parameter.
- `recursively_call_routine`: This routine calls itself repeatedly before returning.
- `update_decoding_table_2`: Calculates a bitwise value and stores the individual bytes of that value into a large memory buffer and a secondary offset buffer, while also traversing a null-terminated string and calling another routine.
- `clamp_global_variable`: Iteratively checks a global variable, clears it, and then applies a clamping or saturation logic to a value before storing it back to a global variable.
- `copy_and_expand_buffer`: Updates a global variable using a bitmask, then copies a sequence of words from a source table to a destination buffer, repeating the copy or clearing certain memory offsets.
- `traverse_pointer_list_2`: Iterates through a list of pointers in memory until a negative or terminal pointer value is reached.
- `traverse_pointer_triplet_list`: Iterates through a list of pointers, advancing by a fixed offset, and calls another routine.
- `blit_bitmapped_sprite_layer`: Iterates through a table of sprite or tile-like data to apply bitwise masks and ORs to a destination bitmapped buffer.
- `blit_sprite_to_bitmapped_layer`: Iterates through a source list to apply bitwise OR and AND operations, masking or shifting sprite pixel data into a destination bitmapped buffer.
- `reset_global_variable`: Sets a global variable to a specific constant value and calls another routine.
- `read_memory_indirect`: Returns a 32-bit value read from a memory location calculated by adding an offset to a register.
- `tiny_ori_rts`: Small routine of 2 instructions that merges 0 into d0, then returns.
- `set_memory_pattern`: Writes a specific sequence of bytes into a memory range and updates a global variable.
- `set_audio_registers`: Checks a global variable and, if it does not match a specific value, performs a loop to decrement a counter and writes several values to audio-related hardware registers and certain memory locations.
- `initialize_data_pattern`: Writes a specific pattern of 32-bit or 16-bit values to a range of memory starting from an offset relative to a register.
- `find_and_insert_value`: Searches a list of memory locations for a specific 32-bit pattern and, if found, replaces a value or inserts a new value from a register.
- `search_for_specific_word`: Iterates through a memory-resident table of 32-bit words, checking if any entry matches a specific constant value, and continues for a fixed number of iterations or until the value is found.
