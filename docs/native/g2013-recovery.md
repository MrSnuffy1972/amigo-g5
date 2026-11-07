# Game 2013 (populous (1989)(electronic arts)[cr vf]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02013). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (905616 top-level bytes in 38 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 15 raw-word lines |
| Functions named and described | pass, 664 of 664 named (0 author symbols), 664 of 664 described (87 by rule, 271 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1147 of 1147 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 131072 bytes, 85 functions |
| Independent decoder cross-check (Ghidra against capstone) | 10122 of 10891 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 623075 |
| code | 81332 |
| data | 190120 |
| padding | 11089 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| endcli | 2 | 117 | 2 |
| echo | 2 | 117 | 2 |
| loadwb | 12 | 536 | 30 |
| Disk-Validator | 1 | 386 | 1 |
| Populous (1989)(Electronic Arts)[cr VF].adf:POP | 2 | 83 | 3 |
| Populous (1989)(Electronic Arts)[cr VF].adf:populous | 3 | 4314 | 145 |
| audio.device | 1 | 448 | 42 |
| Populous (1989)(Electronic Arts)[cr VF].adf:populous.prg | 2 | 83 | 3 |
| icon.library | 10 | 1375 | 109 |
| translator.library | 2 | 490 | 50 |
| info.library | 25 | 2597 | 167 |
| version.library | 2 | 41 | 7 |
| Populous (1989)(Electronic Arts)[cr VF].adf:bootblock | 1 | 7 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 297 | 17 |
| Populous (1989)(Electronic Arts)[cr VF].adf:DAD (raw code file, load address unknown) | 1 | 12830 | 85 |

## Cases handled

- runtime segment an address: 266 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- populous (1989)(electronic arts)[cr vf]/Populous (1989)(Electronic Arts)[cr VF].adf:DAD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### endcli

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_structured_words`: Copies four 32-bit words from a fixed offset in a structure to a caller-supplied buffer when that buffer is not null, then jumps to a stored return address.

### echo

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then, when a condition is met, writes that maximum and a fixed header into a small structure.
- `copy_words_then_call`: Copies a fixed number of words from one memory location to another while the source pointer is non-null, then invokes another routine.

### loadwb

- `initialize_and_tokenize`: Initializes a global structure by storing the entry parameters into its fields, then calls a routine that loads the workbench icon.
- `restore_patch_state`: It retrieves a pointer from a global structure and saves a parameter into a slot within a structure pointed to by that pointer.
- `load_workbench_icon`: Processes a global variable by calling several other routines, performing a double XOR decode on a zero-terminated byte string via a pointer, then stores a result from another routine into a buffer obtained earlier.
- `compare_strings`: It compares two null-terminated strings byte by byte.
- `build_linked_list_from_table`: It iterates through a table of pointers, allocating a new node for each entry and linking them into a new list.
- `call_library_vector_96_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_132_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_138_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_198_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_366_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_372_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_414_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_552_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_624`: Library stub: loads a library base from a global variable and calls the library function at offset minus 624 of that base, returning its result to the caller.
- `compare_workbench_version_string`: It compares a fixed string held in a global variable against a version stored in a Workbench identification structure, returning early when the strings match.
- `write_padded_bytes`: This routine writes a fixed sequence of bytes at consecutive offsets from a pointer argument and returns.
- `compare_byte_sequences`: This routine compares two byte sequences one byte at a time, advancing both pointers until the bytes differ or the first sequence ends, then returns.
- `empty_routine_3`: It contains no instructions and immediately returns to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.
- `call_dispatch`: Library stub: loads its arguments and the exec library base, calls Dispatch and returns its result to the caller.
- `call_find_resident`: Library stub: loads its arguments and the exec library base, calls FindResident and returns its result to the caller.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `checksum_stack_buffer`: It invokes a function pointer stored in a global variable and then computes a rolling checksum over a stack buffer until it reaches a null terminator.
- `call_global_and_checksum_buffer`: It invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### Disk-Validator

- `initialize_data_structure`: Initializes a data structure by writing a computed value to its header, formats a numeric field into a text buffer, and then sets up and runs a loop that fills a table with bit-mapped entries.

### Populous (1989)(Electronic Arts)[cr VF].adf:POP

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Populous (1989)(Electronic Arts)[cr VF].adf:populous

- `initialize_at_entry`: It stores a handle from a routine into a register, clears a block of memory, sets a global pointer to a hardware register value, and makes indirect calls based on flags and return values.
- `initialize_demo_resources`: It stores handles obtained from library calls into global variables and initializes several fields of a structure using values derived from a register, conditionally repeating part of the initialization based on a field of its argument.
- `load_argument_and_call`: It reads a pointer stored in a global variable, dereferences that pointer to obtain an argument, and passes the argument to another routine before returning.
- `manage_interrupt_flag_2`: This routine performs several indirect calls through a global variable table, passing a constant argument to each, then writes a constant to a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `setup_memory_table`: It allocates a block of memory, stores the address in a global variable, and fills an array with incremental offsets.
- `store_constant_and_call`: Stores the constant 399 into a global variable via a pointer derived from a register and then calls another routine.
- `call_routine_from_table`: It reads a pointer from a global variable table using an offset and calls another routine indirectly, passing a constant argument.
- `call_two_global_pointers`: It reads two pointers from two different global variables and makes an indirect call through each of them before returning.
- `call_library_vector_300_2`: It extracts a short field from its argument and calls a library function via a vector obtained from a global variable, using a library stub that loads a library base and calls the function at offset minus 300.
- `store_offset_and_extract_bits_2`: This routine calls another routine, adds a constant to the result, stores it into a global variable, and returns the high 15 bits of that stored value.
- `wait_for_bit_and_call`: It stores a parameter into a global variable and loops until a bit in the global variable is set, combining values from the parameter and the global variable.
- `swap_globals_and_call`: This routine swaps two values stored in global variables and calls another routine with those swapped values.
- `initialize_structure_fields`: Initializes a structure by storing values obtained from other routines into its fields, checking those return values and conditionally invoking another routine when a value is zero, then extracting a masked field and storing it back.
- `wait_for_state_then_call`: It loops calling a routine until it returns one of three values, then passes a global variable pointer and a derived offset to another routine before returning.
- `load_pic_and_wait_sync`: Loads the load.pic file and reads its blocks into buffers, then waits for the copper to reach a sync point.
- `initialize_game_state`: Installs two function pointers in a global table, writes three short constants into globals, clears a short array pointed to by a global until a zero terminator, then zero-initialises a 16-entry structure array by copying a short from another global array and setting fixed fields.
- `process_byte_stream_values`: Processes a stream of byte values from a buffer, converting numeric values into their individual digits and storing the results, while handling special marker values that trigger other operations.
- `return_nothing_3`: This routine contains no instructions and simply returns.
- `process_and_update_arrays`: Repeatedly invokes another routine to gather values, then fills a short array with 160 and a second array with the low three bits of those values, and finally drives a series of array updates and swaps while a flag stays clear and a counter is nonzero.
- `fill_short_arrays_modulo`: Loops a number of times given by a parameter and stores computed short values into two arrays referenced via a pointer, deriving each stored value from a short returned by another routine reduced modulo a constant and offset.
- `update_short_arrays`: It loops over a count derived from its argument, reads and updates two short arrays referenced via a global variable, and returns the first updated value.
- `process_demo_pic_sequence`: It performs a sequence of library and helper calls, conditionally processes input and updates tables using global variables, and manipulates several global flags and pointers, referencing the text demo.pic.
- `initialize_dma_list_entries`: It initializes a DMA list structure in memory with constant words and copies data from parameters into the list using loops.
- `load_two_data_blocks`: It opens a file and reads a large block of data into a destination buffer, then reads a second block into a different buffer, and finally closes the file handle.
- `load_three_data_files`: It opens three separate data files, reading a small header block followed by a large block from each one into distinct buffers, and closes each file handle after its reads.
- `conditional_load_blocks`: It opens a file with a parameter and a constant, and if the open succeeds it reads a large block into one buffer, reads a small block into another buffer, and then closes the file handle.
- `rotate_and_combine_word_planes`: Bounds-checks a value and, when valid, rotates each word of a source table by a nibble-derived amount, then merges the rotated words into four interleaved word planes of a destination buffer using bitwise AND and OR. It repeats this process over a small fixed count, advancing through both the source table and the destination planes each iteration.
- `apply_rotated_mask_to_map_tiles`: Conditionally processes a block of map tiles using a mask table indexed by an input value.
- `apply_tile_mask_and_return`: It calls another routine that indexes a table with a global variable and a register value, using the entry to mask and combine words at offsets derived from it.
- `apply_tile_mask_table`: Clamps a row coordinate into range, then indexes a table using a global variable and a nibble value to combine four groups of words in the destination area with AND and OR operations, applying a shifted path when the nibble is nonzero.
- `reorder_struct_fields`: Walks a table of fixed-size entries and reorders the fields within each one by copying words and bytes to new offsets.
- `shift_byte_buffer`: Loops over a fixed-size byte buffer at a constant address and left-shifts each byte by three bits.
- `nested_tile_mask_loops`: It performs nested loops over a range of indices, calling another routine that indexes a table with a global variable and a register value to mask and combine words.
- `concatenate_registers`: It calls another routine and returns a 32-bit value formed by concatenating the contents of two data registers.
- `store_global_pointer`: Stores the address of a global variable into a global pointer and returns.
- `initialize_buffer_from_string`: Clears a buffer with a pattern of bytes and then a run of zeros, then copies a string into it while splitting characters across several interleaved offsets.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `invoke_handler_chain`: Invokes a routine through a pointer in a global table and follows a chain of further pointers.
- `route_from_table_lookup`: Reads a value from a global variable table and, based on whether it is zero, branches to one of two code paths.
- `dispatch_library_vectors`: It calls a function pointer stored in a global slot and branches based on the result.
- `route_global_dispatch`: Reads a pointer from a global table and calls the routine it points to.
- `advance_state_machine`: Reads a pointer from a global table and calls it, then follows a chain of pointers stored in a data block.
- `initialize_populous_dispatch`: It opens libraries and files, allocates memory, and writes to hardware interrupt registers, referencing text about memory and file errors, then builds dispatch tables by reading values from a global variable area and writing computed pointers and constants into memory.
- `empty_return`: Empty routine that returns immediately without performing any work.
- `clear_byte_and_return`: Stores a zero byte at a fixed offset from a pointer argument and returns the value passed on the stack.
- `initialize_dispatch_and_free`: This routine clears global flags, builds a dispatch table from a source table, writes constants to global variables, performs an indirect call, and returns a combined value from registers.
- `branch_on_byte_then_clear`: It inspects a byte argument and branches to one of two different routines depending on whether that byte equals a newline character or the value one.
- `clear_struct_offset_if_flag`: If its first argument equals one, it calls another routine and takes the pointer that routine returns.
- `clear_struct_offset_byte_2`: It stores a zero byte at a fixed offset within the structure pointed to by a register argument and then returns the value that was on the stack.
- `clear_struct_offset_byte`: Clears the byte at a fixed offset within a structure pointed to by register A0 and returns the value that was on the stack.
- `clear_offset_byte_2`: This routine stores a zero byte at a fixed offset from a pointer argument and returns the value passed on the stack.
- `clear_offset_byte`: Writes a zero byte at a fixed offset from the address held in register A0 and returns the caller's stack value.
- `check_flag_and_call`: It masks a byte in a global variable and, when a specific bit is set, writes a constant through a pointer, updates the global variable, and makes an indirect call through another routine.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `init_global_dispatch_table`: Fills a global table with eight entries, computing offsets, shifts and a division from a source table, then stores pointers derived from those values.
- `build_dispatch_tables`: This routine builds dispatch tables by reading values from a global variable area and writing computed pointers and constants into memory, then initializes additional tables and global variables.
- `reset_global_flags`: Clears a global variable and writes two constant values to another global variable.
- `initialize_dma_channels`: Walks through a table of DMA channel descriptors, initializing each one by setting up its source and destination addresses, control registers, and a shift value derived from a global lookup table.
- `expand_and_init_table_3`: Decodes a packed byte stream into a table of records, interpreting special command bytes that switch between data blocks and expand them.
- `expand_and_init_table_2`: Decodes a packed byte stream and writes the expanded data into a table of records, handling special command bytes that switch between data blocks, set pointers, and compute scaled values, then walks the table to initialize each entry.
- `expand_and_init_table`: Reads a packed byte sequence and expands it into a table of pointers and associated data, then walks that table to initialize per-entry fields and update their counters.
- `initialize_dma_channel_table_2`: Initializes a DMA channel table by storing a control value with bit 15 set, then walks a table of entries, decoding command bytes and updating channel state based on the values found.
- `process_stream_entry`: Reads a byte from a stream and stores a scaled value into a structure, then walks a table of entries, decrementing counters and updating related fields based on flags.
- `process_entity_loop`: Initializes a per-entity timer and status flags, then decrements and processes a loop of entity records, updating their timers, pointers, and derived values from a lookup table.
- `process_entity_frames`: Reads a byte from a data stream and stores it into a screen buffer, then decrements a timer for each active entity and updates its position and state.
- `initialize_dma_channel_table`: Stores a DMA control value with bit 15 set to a global variable, then iterates over a table of entries.
- `process_dma_channel_entry`: Updates per-entry DMA bookkeeping by adjusting a pointer offset and a character counter, then configures the corresponding DMA channel registers based on the current entry type.
- `initialize_dma_channel_table_3`: Initializes a DMA control value with a high bit set, then walks through a table of entries, updating channel registers and processing command bytes to configure transfer parameters.
- `process_dma_channel_commands`: Clears a flag bit and walks through a table of DMA channel entries, decrementing per-entry counters and configuring the corresponding DMA control registers.
- `configure_dma_channels`: Writes a DMA control value with bit 15 set to a global variable, then iterates over a table of entries.
- `combine_word_planes`: Iterates over a block of words and combines each word with entries from a global table using bitwise AND, OR and NOT operations.
- `mask_word_planes`: Loops over a source and destination word arrays and masks destination words with source words and a global table using bitwise AND, NOT and OR. The routine updates four spaced locations per iteration.
- `store_word_from_stack_and_call`: It stores a word taken from the stack into a global variable, calls another routine, and returns a 64-bit value formed by concatenating the two input registers.
- `call_and_return_2`: It calls a single other routine and then returns without performing any other observable operation.
- `update_scaled_state_values`: It checks a global flag and, when set, clamps and scales two values from a table into a global variable, then derives a byte from it.
- `read_joy0dat_and_update_state`: Reads the mouse port movement from JOY0DAT, stores a byte from a global variable through a pointer and updates several global variables, then calls another routine.
- `swap_globals_and_enable_interrupts`: It swaps the contents of two global variables, writes a constant into one of them, and enables interrupts by writing to the interrupt enable register with bit 15 set.
- `manage_interrupt_flag`: Checks a global flag and if set writes values to a global variable and clears the flag.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `build_dma_list`: Initializes a DMA list structure in memory with constant words and copies data from parameters into the list using loops.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `wait_copper_sync`: Stores a parameter into a global variable and loops until a bit in the global variable is set, combining values from the parameter and the global variable.
- `wait_for_interrupt_bit`: The routine writes the constant 32 to a global variable and spins until bit five of that global variable is set, then returns the masked bit.
- `patch_dma_entry`: Searches a short array for a marker value and then copies a number of shorts from a source pointer into the array starting after the marker.
- `unknown_routine`: No further details are available because the decompiler timed out.
- `update_byte_buffer_from_params`: The routine indexes a table with a parameter to obtain a byte buffer pointer, then writes several bytes into that buffer derived from the two parameters and a global variable, including conditional flag bits.
- `read_keyboard_serial`: It clears a global variable twice and tests a flag stored in a global variable.
- `copy_memory_block`: It copies 8000 words from a source pointer to a destination pointer by repeatedly storing each source word to the destination and advancing both pointers.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `decrement_nibble_counters`: It copies a block of words from a source pointer to a stack buffer and repeatedly decrements each nibble of the words until all nibbles are zero.
- `call_and_return_result_2`: It calls another routine with no arguments, then calls another routine with two parameters and returns the result of that second call.
- `conditional_indirect_call`: Calls another routine with its parameter, then checks a global variable and conditionally calls another routine, and performs an indirect call through a table derived from a global variable.
- `store_offset_and_extract_bits`: It calls another routine, adds a constant to the result and stores it into a global variable, then returns the high 15 bits of that stored value.
- `store_high_word_to_global`: It stores the high word of its parameter into a global variable and returns.
- `empty_routine`: It is an empty routine that returns immediately without doing anything.
- `initialize_window_resources`: It obtains a handle from another routine using a value from a global variable, stores it in a global variable, and initializes several fields of the pointed structure, then sets up a table entry and calls other routines to process a resource.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `parse_command_line_into_tokens`: Parses a null-terminated command line into individual tokens, stripping surrounding whitespace and handling quoted strings, then stores an array of pointers to each token.
- `copy_string_with_limit_2`: It scans a null-terminated string to find its end, then copies bytes from another string into it, stopping at a null terminator or when a counter reaches zero.
- `copy_string_with_limit`: It copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `copy_string_pad_zeros`: It copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `multiply_32_bit_values`: It takes two 32-bit values from registers and returns a 64-bit product formed from the high and low 16-bit halves of each operand.
- `open_window_and_dispatch`: It opens a window through the icon library and, if successful, dereferences a jump table and calls the routine found there.
- `call_indirect_then_self`: It checks a global variable and, if it is nonzero, calls the routine stored at the address it points to.
- `check_global_flags_and_cleanup`: It reads a global variable and, if that value is non-zero, jumps through it to call the routine it points to.
- `process_global_flags_and_cleanup`: It checks several global variables for non-zero and conditionally calls other routines or indirect functions obtained from a global variable.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `release_table_entry`: It indexes into a table using a parameter and checks whether the value is out of range or null.
- `call_close_via_jumptable`: It reads an offset from a global variable, indexes into a jump table at that offset, and calls the routine found there before returning.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `invoke_indirect_table`: It dereferences an entry from a table indexed by a value derived from a global variable and jumps to the routine found there.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `dispatch_library_vector_414`: Loads a library base from a global slot and calls the library function at a fixed negative offset from it, with no other logic.
- `call_amiga_library_vector_414`: Calls an Amiga library function at a fixed negative offset from a library base loaded from a global slot, with no other logic.
- `call_library_vector_198`: Loads a library base from a global slot and calls the library function at a fixed negative offset from it, with no other logic.
- `call_indirect_jumptable`: Fetches a jump-table base address from a global variable using an unaffiliated register, adds an offset to it, dereferences the resulting entry as a code pointer, and calls the routine it points to.
- `invoke_jumptable_entry_2`: It reads a jump-table base from a global variable via an unaffiliated register, adds an offset to it and calls the routine found at the resulting entry.
- `invoke_indirect_jumptable`: Loads a jump-table base from a global variable via an unaffiliated register, adds a different offset to it, dereferences the computed entry as a code pointer, and invokes the pointed-to routine.
- `invoke_jumptable_entry_offset`: It reads a jump-table base from a global variable via an unaffiliated register, adds a different offset to it and calls the routine found at the resulting entry.
- `call_jumptable_entry`: Reads a jump-table base address from a global variable using an unaffiliated register, adds an offset to it, dereferences the resulting entry as a code pointer, and calls the routine it references.
- `invoke_jumptable_entry`: Loads a jump-table base from a global variable via an unaffiliated register, adds an offset to it, dereferences the computed entry as a code pointer, and calls the routine it points to.
- `call_library_vector_210_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_indirect_entry`: Fetches a jump-table base address from a global variable using an unaffiliated register, adds an offset to it, dereferences the resulting entry as a code pointer, and invokes the pointed-to routine.
- `dispatch_indirect_call`: Reads a jump-table base address from a global variable, adds an offset to it, dereferences the computed entry, and calls the resulting routine.
- `call_indirect_dispatch`: Reads a jump-table base from a global variable via an unaffiliated register, adds a different offset to it, dereferences the computed entry as a code pointer, and calls the routine it references.
- `invoke_indirect_dispatch`: Loads a jump-table base address from a global variable using an unaffiliated register, adds an offset to it, dereferences the resulting entry as a code pointer, and calls the routine it points to.
- `call_amiga_library_vector_366`: Calls an Amiga library function at a fixed negative offset from a library base.
- `call_library_vector_378_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_dispatch_table`: Fetches a jump-table base from a global variable via an unaffiliated register, adds an offset to it, dereferences the computed entry as a code pointer, and invokes the pointed-to routine.
- `call_library_vector_462_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `invoke_indirect_table_2`: It reads a pointer from a table indexed by a value stored in a global variable, then jumps to the routine located at that pointer.
- `call_library_vector_90_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_indexed_routine`: Looks up a routine in a jump table using an index stored in a global variable and calls it.

### audio.device

- `stub_bad_instruction_data`: The code holds only an unimplemented-instruction marker, so the decoder stops at once and no logic is recovered.
- `call_single_routine_and_return`: Calls one other routine and returns, with no register or memory work of its own.
- `call_first_pointer_in_block`: Calls the routine whose address is stored in the first longword of the block passed in an address register, then returns.
- `advance_audio_request_queue`: Decrements a count in the request block and, when the count reaches zero, either clears a field or unlinks the next queued request.
- `finish_audio_request_chain`: Updates the request from its attached state, relinks it into a list when a pointer is present, and decrements its count, unlinking the next request at zero.
- `decrement_request_count`: It decrements a count field in a request block and, when that count reaches zero, copies a value from the request into another structure and unlinks the request from a linked list.
- `store_audio_request_value`: Sets the request's handler pointer to its own first field, then stores a value from the block into a device field with its top bit set.
- `copy_request_fields_and_unlink`: Copies two fields from the request into the device block, unlinks the request from its list when it has a link, and calls a routine from the device base.
- `call_cause`: Wrapper that calls the exec library function Cause and returns its result to the caller.
- `clear_request_and_notify`: Clears the request's link pointer, stores its handler address, and writes a field into the device block.
- `dequeue_request_on_carry`: When the carry flag is set, clears a count field in the request.
- `dequeue_audio_request`: Takes the first entry from the request's list and unlinks it, or clears the request pointer when the list is empty.
- `call_routine_touching_custom_chip`: Calls one other routine and returns.
- `call_routine_with_chip_base`: Calls one other routine and returns, and the decoder records a register access to the custom chip base.
- `call_routine_and_return`: Calls one other routine and returns with no other work.
- `call_gap_routine`: Calls one other routine and returns; the bytes sit between known routines and decode as code.
- `clear_request_flag_and_call`: Clears bit one of a flag byte in the request block, then calls a routine from the device base.
- `link_node_into_list`: Inserts a node into a list by updating neighbouring pointers, then resets the node's own pointers so it heads an empty chain.
- `drain_list_with_interrupts_masked`: Repeatedly removes the first entry from a linked list while interrupts are disabled, calling a routine from the device base for each entry.
- `decrement_counter_and_enable_interrupts`: Decrements a byte counter in the block and, when the value before the decrement was zero or negative, writes the interrupt control register with the enable bit set.
- `init_audio_device_structures`: Initialises a block of audio device structures by writing fixed values into their fields and setting up linked-list headers.
- `mark_request_and_enqueue`: Sets a state byte in the request to minus two, then calls a routine with the request and a list header.
- `set_flag_and_call`: It sets a flag byte in a request block to a fixed value and then calls another routine with swapped arguments.
- `requeue_or_release_request`: If the request has no link or its flag bit is set, it passes the request to a routine.
- `update_interrupt_status_word`: Copies the forward link into the back-link field, sets the handler pointer, and writes status values into interrupt-related globals depending on a bit test.
- `walk_linked_request_nodes`: Calls a routine on the given node and, when the node's link is set and not the head, calls the same routine on the linked node.
- `find_and_detach_request`: Disables interrupts, scans a request list for the given node and, when found, removes it and marks it inactive before re-enabling interrupts.
- `search_list_and_detach_node`: Scans a list for the given node with interrupts disabled and, when the node is found, removes it and marks it inactive.
- `scan_list_and_detach_node`: Scans a list for a node while interrupts are masked, marking the node inactive and calling a routine to remove it when found.
- `mark_all_requests_inactive`: Walks a list and sets the state byte of every entry to minus two while interrupts are disabled, then calls a routine to finish the list.
- `mark_remaining_requests_inactive`: Sets the state byte to minus two for each node in a list, then calls routines under interrupt masking and processes any remaining entry.
- `reset_request_flag_and_dispatch`: Runs a sequence of list and queue routines with interrupts masked, clears bit one of a flags word in the block, and calls a further routine from the device base.
- `reset_flags_and_dispatch`: It runs a sequence of list and queue operations with interrupts masked, writes a value to a structure field, loads a byte from a table into a global variable, and clears a flag bit.
- `copy_table_byte_and_call`: Copies one byte from a table indexed by a register into the byte before a pointer, then calls a library function at a fixed offset from the library base.
- `initialize_empty_list_header`: Initialises a list header in place: its first longword points just past itself, the next longword is cleared, and the last longword points back to the header.
- `call_disable`: Library stub: loads its arguments and the exec library base, calls Disable and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_exec_forbid`: Calls the Exec library Forbid function through the library base stored in a global slot, then returns.
- `call_set_int_vector`: Library stub: loads its arguments and the exec library base, calls SetIntVector and returns its result to the caller.
- `call_exec_free_memory`: Calls the Exec library FreeMem function through the library base stored in a global slot, then returns.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_remove`: Library stub: loads its arguments and the exec library base, calls Remove and returns its result to the caller.

### Populous (1989)(Electronic Arts)[cr VF].adf:populous.prg

- `intro_bytekiller_depack_2`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Increments a 16-bit counter in a library structure and clears bit three of a flag byte in the same structure.
- `decrement_library_refcount`: Decrements a 16-bit reference count in a library structure.
- `release_or_defer_library_node`: If the count is zero, unlinks the structure from its list and calls a library function to free it, returning the saved value.
- `set_deferred_flag_return_zero`: Sets bit three of a flag byte in a library structure and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `open_dos_library_or_alert`: Stores two arguments in a library structure, calls a library function and saves its result.
- `copy_two_pointers_from_table`: Copies two longwords from a structure, addressed through a register, into two global variables.
- `forward_to_shared_routine`: Calls one other routine and returns, with no other work.
- `forward_library_call`: Calls one other routine and returns, with no other work of its own.
- `call_library_handler`: Calls one other routine and then returns, with no other operations.
- `forward_icon_copy_call`: Calls one other routine and returns, with no other work.
- `call_icon_init_routine`: Calls one other routine and returns; it acts as a wrapper for that routine.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `call_library_handler_2`: This library routine simply calls another routine and then returns.
- `call_library_handler_3`: This library routine invokes another routine and then returns without performing any other operations.
- `call_library_helper`: Calls one other routine and returns, with no other work.
- `call_icon_init_routine_2`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `store_byte_via_pointer`: This library routine stores a byte from a register through a pointer, then returns.
- `invoke_indirect_handler`: This library routine loads a routine address from a data location and jumps to it, then returns to its caller.
- `call_other_routine`: The routine calls another routine and then returns.
- `call_other_routine_2`: This routine invokes another routine and then returns without performing any other operations.
- `call_other_routine_3`: This routine calls another routine and then returns.
- `update_data_field`: Calls a routine through a function pointer table entry with the first parameter, then stores the second parameter into a field at a fixed offset from the returned pointer.
- `allocate_and_init_structure`: Calls one routine to obtain a structure and passes it to a second routine.
- `read_icon_record_into_structure`: Allocates a structure, reads a record into a local stack buffer through library calls, copies its fields into the structure, and links it into a list.
- `copy_icon_fields_into_structure`: Copies ten longwords and several fields from a stack record into a structure, and sets a back-link in the structure when its list pointer is non-zero.
- `return_zero_stub_3`: This routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: Copies ten longwords and several fields from a structure into a local stack record, then calls another routine with the first two arguments and a pointer to that record.
- `load_info_icon_file`: Builds a path from the name and the .info suffix, opens the file through library calls, and reads the icon data into the caller's structure.
- `read_icon_image_sections`: Reads an icon header, checks a magic value and version number, and on success reads further sections, each allocated and stored in the structure.
- `read_icon_optional_sections`: Reads optional icon sections whose presence is flagged in the structure, allocating and storing a pointer for each section that is present.
- `read_icon_pointer_table`: Reads a list of pointers into an allocated table one entry at a time, then reads a final pointer when a flag is set.
- `return_zero_after_call`: Calls one other routine and returns zero.
- `return_immediately`: Contains no logic and returns at once.
- `load_icon_with_temp_buffer`: Allocates a 260-byte temporary buffer and reads icon data through library calls, copying several fields into the structure.
- `reread_icon_record_fields`: Saves a field of the structure, reads icon data through library calls, and copies or writes several fields and lists.
- `write_icon_pointer_lists`: Writes a size value and the pointer entries of a list through library calls, returning non-zero when every write succeeds.
- `write_icon_string_table`: Walks a null-terminated list of pointers, writing each string through library calls, and returns non-zero when the writes succeed.
- `iterate_icon_table_entries`: Walks a null-terminated table of pointers, calling a routine for each entry and updating a count, then invokes a callback while possibly processing a secondary entry.
- `walk_icon_list_entries`: Walks a list starting at the given entry, calling a routine on each node until a terminating node is reached, then calls the routine once more for a pending node.
- `iterate_linked_list_nodes`: Walks a chain of nodes through their link field, calling a routine for each node until the chain ends or returns to its start.
- `increment_param_and_recurse`: Calls another routine with the same first argument and the second argument increased by a fixed amount, then returns.
- `allocate_icon_structure`: Allocates a 168-byte cleared structure and links a field to it, then calls routines to initialise it.
- `return_nothing_2`: This routine is empty and immediately returns without performing any operation or touching any data.
- `return_nothing`: This routine is an empty function that immediately returns without performing any operation or touching any data.
- `store_pair_in_entry_list`: Decrements a 16-bit count in a structure.
- `decrement_counter_and_store_pair`: Decrements a 16-bit counter through a pointer, then stores two register values into a table entry selected by the counter.
- `allocate_icon_node_structure`: Allocates a 94-byte cleared structure, links a field to it, and calls routines to initialise it.
- `empty_return_routine_2`: This routine contains no instructions and simply returns without performing any operation.
- `call_library_routine_and_return_zero`: Calls one other routine and then returns zero.
- `call_with_incremented_argument`: Calls another routine with the second argument increased by a fixed amount, then returns.
- `find_value_after_equals`: Walks a null-terminated list of pointers, testing each entry, and returns the character after the equals sign of the first match, or zero when no entry matches.
- `scan_table_for_equals_value`: Scans a pointer table for an entry whose text has an equals sign at the matching position, and returns the text after it or zero when none matches.
- `locate_key_value_separator`: Walks a null-terminated pointer table testing each entry, and returns the position after the separator that follows a matching prefix, or zero when no entry matches.
- `check_chain_length_matches`: Walks a chain of entries, comparing each entry's computed length with an expected value, and returns one when a match passes a check routine.
- `scan_chain_for_length`: Walks a chain of entries computing each length, and returns one as soon as a length matches the expected value and a check routine passes, zero otherwise.
- `find_length_match_in_chain`: Scans a chain of entries for one whose computed length matches the expected value, returning one when a matching entry passes a check and zero otherwise.
- `verify_chain_length_match`: Repeatedly computes entry lengths along a chain and returns one when a length matches the expected value and a check routine passes.
- `search_chain_length_match`: Searches a chain for an entry whose computed length matches the expected value and whose check routine passes, returning one on success.
- `build_copy_name_string`: Clears a flag byte, checks the input for a copy marker text, parses a number, and writes a new name built from the copy text into the target buffer.
- `parse_copy_number`: Compares a three-character marker, parses decimal digits after it, then compares a four-character suffix before finishing.
- `parse_decimal_digits`: Reads decimal digits from a string while each character converts to a digit, then compares the following four characters and returns.
- `parse_copy_number_text`: Accumulates decimal digits from a string and then checks a fixed four-character suffix.
- `convert_char_to_digit`: Returns the decimal value of a character between the digit codes for zero and nine, and minus one for any other character.
- `return_minus_one_stub`: Returns minus one and does nothing else.
- `forward_to_shared_routine_2`: Forwards control to a shared routine via a computed call and then returns.
- `call_indirect_global_hook`: It invokes the routine whose address is stored in a global variable and then returns.
- `call_library_vector_228_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `propagate_icon_change`: Forwards four parameters to another routine and, when its result is non-zero, calls it again with a modified argument list.
- `allocate_icon_object_header`: Allocates a 96-byte cleared structure, sets a size field to ten and stores ten through a caller pointer, then calls another routine to initialise it.
- `return_zero_stub_2`: This routine is an empty stub that immediately returns the constant zero.
- `verify_returned_value_matches`: Calls a routine with three arguments and returns its result when it equals the fourth argument, otherwise zero.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `verify_library_result_matches`: Calls a library routine with three arguments and returns its result when it equals the fourth argument, otherwise zero.
- `return_empty_stub_2`: This routine is an empty stub that immediately returns without performing any operations.
- `divide_unsigned_32bit`: Divides a 32-bit dividend by a divisor, using a shift-and-subtract loop when the divisor is larger than sixteen bits and the native division otherwise, and returns the quotient and remainder as a pair.
- `divide_by_divisor_bit_loop`: Runs sixteen shift-and-subtract steps that divide a 32-bit dividend held in two registers by a divisor, leaving the quotient in the low half.
- `multiply_32_bit_integers`: Computes the 32-bit product of two 32-bit register values from their 16-bit halves and returns it.
- `call_rawdofmt_formatter`: Calls the Exec RawDoFmt function through a library base loaded from a global slot, which formats text.
- `store_byte_through_pointer`: Writes the low byte of a data register to the memory address held in an address register.
- `compute_string_checksum`: Iterates a zero-terminated byte string, updating a 32-bit accumulator for each character with a decrementing pattern, and returns the bitwise complement of the final value.
- `find_character_in_string`: Scans a zero-terminated string for a given character and returns a pointer to its first match, or zero when the string ends first.
- `return_zero_stub`: Returns the constant zero and does nothing else.
- `copy_c_string`: Copies bytes from a source string to a destination until a zero byte is copied, advancing both pointers.
- `append_string_to_string`: Advances to the end of a destination string and copies a source string, including its terminator, onto that end.
- `compare_strings_up_to_length`: Compares two strings character by character for up to a given count, returning one when they differ and zero when they match or the count runs out.
- `return_one_value`: Returns the constant one to its caller without modifying any data.
- `scan_string_with_count_limit`: Walks a string while a count remains and the characters are non-zero, then calls a routine to finish the operation.
- `call_and_return_result`: Invokes another routine and then returns its result to the caller.
- `copy_string_and_zero_pad`: Copies a string into a destination up to a length, then fills the remaining destination bytes with zeros.
- `empty_routine_2`: The routine contains no instructions and returns immediately without performing any operation or modifying any data.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `process_icon_request`: Checks that a parameter is non-zero, calls a routine to validate a value, and when that succeeds calls further routines using a field read through the pointer.
- `walk_records_until_failure`: Walks a chain of records through their link field, calling two routines on each record, and returns one when every record passes and zero otherwise.
- `check_records_all_pass`: Walks a list of records through a fixed link field, calling two routines on each record, and returns one when the walk ends without a failure.
- `load_byte_via_pointer`: Loads the byte stored at the address in the a1 register into the d0 register and returns it.
- `allocate_icon_entry`: Allocates a 20-byte cleared record, calls routines to initialise it and store a value in it, and returns the record.
- `return_negative_one_stub`: This routine is an empty stub that immediately returns the constant value of negative one.
- `build_record_chain`: Allocates records in a loop, links each to the previous one through its link field, and returns the first record, or minus one if allocation fails.
- `link_record_chain`: Allocates a record, stores a pointer to it in the previous record, and repeats while the record's link is non-zero.
- `walk_and_store_record_chain`: Walks a sequence of records through library calls, storing a pointer in each, and returns the saved head or minus one on failure.
- `return_negative_one`: This routine is a library function in icon.library that returns the value -1 (a fixed address) to its caller.

### translator.library

- `initialize_translator_library`: Creates a library structure with MakeLibrary, stores two pointer values in it, and registers it with AddLibrary.
- `increment_counter_clear_bit_five`: Increments a 16-bit counter in a library structure and clears bit five of a flag byte.
- `decrement_counter_and_release`: Decrements a 16-bit counter in a library structure.
- `unlink_and_free_library_node`: If the structure has no pending link, unlinks it from its list and calls a library function to free it, returning the saved value.
- `unlink_and_free_node`: Unlinks a node from its list, then calls a library function to free it and returns the saved value.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `translate_text_line`: Fills a buffer with spaces and copies characters from the input string, stopping at a hash marker or a space-delimited boundary and calling routines for each character.
- `return_low_word`: Returns the low sixteen bits of a data register as a signed value.
- `clear_byte_return_word`: Calls a routine, clears the byte before a pointer in an address register, and returns the 16-bit value from the data register.
- `match_translation_pattern`: Matches characters of a pattern string against the input line, calling routines for each character, and returns the status when the pattern ends or fails.
- `scan_translation_pattern`: Scans the input for characters that match a pattern, calling routines for each character and returning when the input ends or no character matches.
- `scan_escape_markers`: Walks a text buffer looking for backslash or backtick characters, calling a routine at each marker and returning when the text ends.
- `handle_escape_marker`: Handles a backslash or backtick marker by calling routines, and returns when the marker closes a block.
- `stub_immediate_return`: The decompiled body contains only a return, so the routine does no visible work in the library.
- `match_bracketed_tag`: Skips to a left bracket, then compares the following characters with a second string until a right bracket, returning once the two match up to it.
- `scan_backwards_with_table`: Scans backwards through text while a character-class table flags each character, comparing with a second string and calling a routine when the scan ends.
- `stub_empty_return_slot`: Contains only a return in the decompiled output, so it is an empty stub with no visible effect on the library.
- `match_backwards_with_table`: Compares characters backwards from the end of a text against a second string, calling routines on each step and returning when a comparison fails.
- `match_until_equals_sign`: Compares characters of two strings until an equals sign is reached in the first, returning when they differ.
- `match_text_with_table`: Compares characters of a string against a second string, using a character-class table to decide when to stop; returns on mismatch or at an equals sign.
- `copy_text_segment`: Copies a segment of text into the output buffer, stopping at a backquote or backslash marker, and returns the copied length.
- `copy_text_segment_bounded`: Copies text into the output buffer up to a size limit, calling a routine at the end of the copy and returning the new position.
- `copy_fixed_length_bytes`: Copies a fixed count of bytes from source to destination, then sets bit zero of a flag byte when the following source character is a backquote.
- `terminate_text_after_last_word`: Walks backwards from the end of the text using a character-class table to find the last non-space character, and writes a zero terminator after it.
- `dispatch_on_special_character`: Searches a fixed table of special characters for the character in a data register and calls the routine stored at the matching index.
- `empty_stub_return`: This routine is an empty stub that contains only a return and has no visible effect on the library.
- `stub_empty_return_slot_2`: This routine contains only a return in the decompiled output, so it is an empty stub with no visible effect on the library.
- `library_empty_return`: This library code routine performs no visible actions and simply returns.
- `call_while_status_bit_set`: Calls a routine repeatedly while bit five of a status register is set.
- `library_empty_return_2`: This library code routine performs no visible actions and simply returns.
- `check_status_character_pair`: Calls a routine, then checks a status bit and the character returned by a further routine against two fixed letters, returning in every branch.
- `return_immediately_3`: This routine performs no operations and simply returns without touching any data or registers.
- `match_letter_sequence_suffix`: Calls routines to read letters and compares them with a fixed letter sequence, returning early when a letter does not match and the status bit fails.
- `match_letter_pair_suffix`: Compares two letters returned by routines against a fixed pair and, on a match, checks a status bit and a further letter before returning.
- `do_nothing_routine`: This routine contains no instructions and does nothing to its data.
- `empty_return_stub`: This routine is an empty stub that contains only a return instruction and has no visible effect on the library.
- `empty_return_stub_2`: This routine contains only a return instruction and performs no operations on any data or registers.
- `empty_stub_return_2`: This routine is an empty stub that contains only an RTS instruction and has no visible effect on the library.
- `return_immediately_4`: This routine consists of three instructions that return control to the caller.
- `poll_status_bit_until_clear`: It calls a library routine repeatedly while bit 5 of the status value stays set, then returns once that bit is clear.
- `empty_routine_returns`: It executes three instructions and then returns without changing any data or registers.
- `call_while_status_bit_two_set`: Calls a routine repeatedly while bit two of a status register is set.
- `check_status_high_bit`: Calls a routine and then tests bit seven of a status word; both branches return, so the test has no effect on the result.
- `do_nothing_return`: This routine contains no instructions and simply returns without performing any action.
- `format_translated_output`: Copies characters from the source text into the output buffer, mapping control characters to spaces and stopping at a hash marker or a size limit.
- `store_space_before_pointer`: Writes a space character two bytes before the address held in an address register.
- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 253 into d1, then returns.
- `tiny_move_move_rts`: Small routine of 3 instructions that copies 32 to (a2)+, then copies 32 to (a2)+, then returns.
- `replace_listed_letters_with_digit`: Scans a text buffer for characters that appear in a fixed table of letters and replaces each match with the character for the digit four, then stores the new end pointer.
- `insert_char_and_shift_buffer`: Library code that shifts the bytes of a buffer one place towards its end and writes a fixed character into the freed slot.

### info.library

- `stub_bad_instruction_data_2`: Only one instruction could be decoded and it is not valid code, so the body is an empty stub.
- `store_library_state`: Library code that saves the library base and one incoming register value into two globals.
- `forward_call_and_return`: Library code that makes a single call to another routine and then returns, adding nothing of its own.
- `call_helper_and_return`: Library code that calls one helper routine and returns straight afterwards, with no other work in its body.
- `call_and_return`: Invokes an unnamed routine pointed to by the library and then returns without producing a value.
- `call_routine_and_return_2`: Makes one call to an unnamed routine and returns immediately afterwards, with no other work.
- `stub_return_immediately`: Contains a single return instruction and does no work when called.
- `clear_flag_and_increment_counter`: Clears one flag bit in a byte of the structure passed in and increments a 16-bit counter at a later offset in the same structure.
- `branch_on_counter_state`: Calls an unnamed routine and then branches on a 16-bit counter in the structure.
- `call_routines_with_computed_args`: Saves a global value, calls one unnamed routine, then passes a computed offset and sum from two 16-bit structure fields to another routine.
- `decrement_counter_and_dispatch`: Decrements a 16-bit counter in the structure.
- `stub_return_zero`: Returns zero without performing any other work.
- `call_nonzero_pointers_conditionally`: Calls a routine for each non-zero pointer among its arguments and several global pointers.
- `initialize_info_block_from_template`: Requests a block from a library function and, on success, stores fixed offsets into it in several globals.
- `initialize_info_library_struct`: Computes many pointers as fixed offsets from a base pointer and stores them in globals and in a structure.
- `acquire_three_handles_or_fail`: Calls an unnamed routine three times with zero arguments, storing each result in a global.
- `acquire_three_handles_indirect`: Performs the same three acquisitions as its neighbour, but calls through function pointers held in address registers.
- `acquire_two_handles_indirect`: Calls a function pointer twice and stores each result in a global.
- `acquire_one_handle_indirect`: Calls a function pointer and stores the result in a global.
- `stub_return_one`: Returns one without performing any other work.
- `get_cached_or_derived_long`: Returns a 32-bit value stored in a global structure if one is set.
- `get_cached_long_from_pointer`: Returns a stored 32-bit value from the structure in the address register when present.
- `get_stored_or_chained_long`: Returns a stored 32-bit value when one is present.
- `read_long_from_chained_pointer`: Follows a pointer chosen by a type byte, either a structure field or a pointer argument, and returns the 32-bit value at a fixed offset in the target.
- `read_long_via_field_pointer`: Returns a 32-bit value found at a fixed offset in the structure referenced by a field of its argument.
- `tiny_moveq_movea_rts`: Small routine of 3 instructions that loads 0 into d0, then copies (a7)+ to a2, then returns.
- `copy_name_and_resolve_info`: Gets a value from an unnamed routine and stores it in a field of a structure.
- `resolve_name_and_set_flag`: Calls two unnamed routines on a local 116-byte buffer, the second with a code of -2, and clears a global flag.
- `allocate_paired_blocks_and_init`: Obtains two blocks from a library function and checks each is non-zero.
- `allocate_block_and_set_count`: Obtains a block through an unnamed routine and stores it in a pointer argument.
- `configure_block_with_count`: Runs optional status checks through unnamed routines and stores a computed count of at least 256 in a field of a block.
- `link_block_with_count`: Stores a computed count of at least 256 in a field of a block and obtains another block through an unnamed routine.
- `finish_block_setup_and_report`: Stores a computed count of at least 256 into a block, obtains a further block and copies two fields into it when that succeeds.
- `call_routine_then_copy_fields`: Calls one unnamed routine with a field from one structure and a field from a second structure.
- `setup_info_state_or_report`: Clears several global pointers and stores its second argument in a global.
- `setup_info_state_pointer_out`: Performs the same obtain-and-link sequence as its neighbour, storing the new pointer into a caller-supplied slot.
- `setup_info_state_from_handle`: Obtains a stack record and a block through unnamed routines, links them into the structures and stores a value into a caller slot.
- `link_block_and_report_failure`: Obtains a record through an unnamed routine, stores a global-based value in a passed pointer and links the record to the structures.
- `finish_block_link_and_report`: Links a block to the structures through several unnamed calls, using a value from a global structure.
- `finish_info_setup_report`: Links a block and its structures through a sequence of unnamed calls and reports code 1 through a function pointer if the setup fails.
- `finish_setup_with_status_call`: Calls an unnamed routine and, if its result is zero, reports code 1 through a function pointer held in an address register.
- `run_conditional_calls_then_report`: Runs one unnamed routine only when a global flag is clear and an argument is non-zero, then runs four further unnamed routines.
- `call_library_vector_132`: Loads a library base from a global variable and calls the library function 132 bytes before that base.
- `call_library_vector_138`: Loads a library base from a global variable and calls the library function 138 bytes before that base.
- `call_library_vector_198_2`: Loads a library base from a global variable and calls the library function 198 bytes before that base.
- `call_library_vector_210`: Loads a library base from a global variable and calls the library function 210 bytes before that base.
- `call_library_vector_234_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `call_library_vector_246`: Loads a library base from a global variable and calls the library function 246 bytes before that base.
- `call_library_vector_252`: Library stub: loads a library base from a global variable and calls the library function at offset minus 252 of that base, returning its result to the caller.
- `call_library_vector_294`: Loads a library base from a global variable and calls the library function 294 bytes before that base.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_372`: Loads a library base from a global variable and calls the library function 372 bytes before that base.
- `call_library_vector_378`: Loads a library base from a global variable and calls the library function 378 bytes before that base.
- `call_library_vector_414`: Loads a library base from a global variable and calls the library function 414 bytes before that base.
- `call_library_vector_552`: Loads a library base from a global variable and calls the library function 552 bytes before that base.
- `call_library_vector_42`: Loads a library base from a second global variable and calls the library function 42 bytes before that base.
- `call_library_vector_72_3`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_96`: Loads a library base from a second global variable and calls the library function 96 bytes before that base.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_228`: Loads a library base from a second global variable and calls the library function 228 bytes before that base.
- `call_library_vector_270_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_276`: Library stub: loads a library base from a global variable and calls the library function at offset minus 276 of that base, returning its result to the caller.
- `call_library_vector_354_2`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_366`: Loads a library base from a second global variable and calls the library function 366 bytes before that base.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_408`: Library stub: loads a library base from a global variable and calls the library function at offset minus 408 of that base, returning its result to the caller.
- `call_library_vector_462`: Loads a library base from a second global variable and calls the library function 462 bytes before that base.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_vector_72`: Loads a library base from a third global variable and calls the library function 72 bytes before that base.
- `call_library_vector_78`: Loads a library base from a third global variable and calls the library function 78 bytes before that base.
- `call_library_vector_234`: Loads a library base from a third global variable and calls the library function 234 bytes before that base.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_270`: Loads a library base from a third global variable and calls the library function 270 bytes before that base.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_354`: Loads a library base from a third global variable and calls the library function 354 bytes before that base.
- `fill_record_and_call_routine`: Fills a local record with fixed constants, a value from a global and a field from another structure, then passes it to an unnamed routine.
- `dispatch_by_record_type`: Switches on a 16-bit type field of the record in its first argument.
- `return_one`: Returns the constant value one to its caller without touching any data.
- `toggle_flag_bit_by_type`: Maps a record's type value to one of seven single-bit masks and toggles that bit in a stored flag word.
- `store_value_then_call_routine`: Stores its incoming register value into a field of a stored record, then calls an unnamed routine with a field taken from the record passed in an address register.
- `build_info_path_and_validate`: Formats a path from two text pieces ending in an info extension, then checks it through several unnamed routines and library calls.
- `check_disk_path_status`: Formats a path into a stack buffer using a disk text or a stored pointer, then checks it through an unnamed routine.
- `validate_info_entry_fields`: Copies text into a stack buffer and checks a stored record through unnamed routines.
- `check_four_entries_then_run`: Calls an unnamed routine with four different global pointers and the all-ones argument.
- `check_three_calls_then_run`: Calls a function pointer three times and returns 0 if any result is the all-ones value.
- `check_two_calls_then_run`: Calls a function pointer twice and returns 0 if either result is the all-ones value.
- `run_if_call_succeeds`: Calls a function pointer and, unless the result is the all-ones value, runs one unnamed routine.
- `call_routine_and_return_one`: Makes one call to an unnamed routine and returns one afterwards.
- `initialize_info_table_entries`: Calls one unnamed routine 24 times with fixed numeric constants describing ranges and positions.
- `register_seven_table_entries`: Calls one unnamed routine seven times, each time with the same first argument and fixed range constants.
- `register_seven_range_entries`: Calls one unnamed routine seven times, passing the same first argument with fixed numeric constants for flags and ranges.
- `register_nine_table_entries`: Calls one unnamed routine nine times, each time with the same first argument and fixed numeric constants for flags and ranges.
- `register_nine_range_entries`: Calls one unnamed routine nine times with the same first argument and fixed numeric constants for flags and ranges.
- `select_entry_layout_by_type`: Chooses one of two numeric constants from a type byte.
- `process_entry_list_state`: Runs one of two unnamed call sequences depending on whether a stored list has reached its end marker.
- `run_entry_callbacks_with_field`: Calls a library function at a fixed offset from its base, then passes a field of the record to a second routine.
- `allocate_linked_entry_list`: Obtains a block through a library function and makes it the head of a circular list.
- `append_entries_from_table`: Links the existing head block into a list, then for each value in a zero-terminated table obtains a further block, stores the value in it and appends it.
- `append_table_entries_loop`: Loops over a zero-terminated table, obtaining a block for each value, storing the value in it and appending it to the list.
- `append_table_entries_from_block`: Stores a value from a zero-terminated table into a supplied block and appends it to the list, obtaining a new block for each further value.
- `advance_list_entry_or_insert`: If the list head is at an end marker, or the entry before it is the list start, it calls a library function and two routines with fields of the record.
- `advance_list_head_and_notify`: Calls an unnamed routine with a field from the current list head and a stored value, then calls another routine.
- `step_list_head_or_finish`: If the list head is at an end marker, it calls a library function and two routines with fields of the record argument.
- `advance_list_head_via_link`: Calls an unnamed routine with a field from the current list head and a stored value, then calls another routine.
- `insert_entry_and_mark_busy`: Sets a state flag, and if the list is not at its end it calls two routines with stored values.
- `advance_list_head_and_mark`: Sets a state flag, then either calls two routines and a third with a record field, or moves the list head to a neighbouring entry chosen by comparing links.
- `reset_list_head_and_notify`: Calls an unnamed routine with the current list entry, then moves the head to the first entry of the list, or to the last entry when the list is at its end.
- `run_entry_callbacks_in_order`: Makes one unnamed call, then calls two routines with the fields at offsets 46 and 62 of its record argument, in that order.
- `allocate_per_entry_pointer_table`: When a pending flag is set, it rebuilds a pointer table for the list.
- `build_entry_pointer_table`: Rebuilds a pointer table for the list using an address-register record, counting the entries and obtaining one block per entry.
- `rebuild_entry_pointer_table`: Calls an unnamed routine with a stored value, then counts the list entries and obtains a pointer table sized to them.
- `fill_entry_pointer_table`: Counts the list entries and obtains a pointer table sized to them.
- `fill_entry_blocks_from_table`: Copies data into an existing pointer table, then for each list entry obtains a block, stores it in the table and fills it.
- `fill_entry_blocks_loop`: Loops over the list from its current entry, obtaining a block for each entry sized from its value, storing the block in the table and copying data into it.
- `continue_entry_block_fill`: Copies data into the current table slot, advances to the next list entry and obtains a block sized from its value.
- `call_library_vector_84`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `call_library_vector_90`: Loads a library base from a global variable and calls the library function 90 bytes before that base.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_180`: Library stub: loads a library base from a global variable and calls the library function at offset minus 180 of that base, returning its result to the caller.
- `call_library_vector_186`: Library stub: loads a library base from a global variable and calls the library function at offset minus 186 of that base, returning its result to the caller.
- `dispatch_layout_by_mode`: Switches on a mode byte in a stored record and makes a fixed sequence of unnamed routine calls with numeric constants and stored values for each mode.
- `return_immediately_2`: Returns immediately without modifying any data or registers.
- `allocate_mode_dependent_blocks`: Obtains four blocks through a library function and, only if all are non-zero, obtains four more of fixed sizes.
- `allocate_mode_steps_via_pointer`: Calls a function pointer four times, storing the last result in a caller slot.
- `run_mode_dependent_call_sequence`: Makes a mode-dependent number of unnamed routine calls: two for mode 1, three for modes 2 and 5, four for mode 3 and five for mode 4.
- `execute_three_instructions`: This routine executes three instructions and then returns.
- `clear_or_copy_optional_entry`: If a stored pointer is null it clears a global target, otherwise it calls an unnamed routine to copy into that target.
- `copy_entry_then_check_result`: Calls an unnamed routine to copy from a stored pointer into a target, then calls another routine.
- `empty_return_routine`: This routine performs no operations and simply returns to its caller without modifying any data or registers.
- `set_fields_and_check_four`: When the mode is not 4, it writes four 16-bit values into fixed fields of stored records.
- `check_three_calls_succeed`: Calls a function pointer three times and returns 0 if any result is the all-ones value, otherwise 1.
- `check_two_calls_succeed`: Calls a function pointer twice and returns 0 if either result is the all-ones value, otherwise 1.
- `check_single_call_succeeds`: Calls a function pointer once and returns whether its result is not the all-ones value.
- `return_one_constant`: This routine is an empty library function that simply returns the constant value one without performing any operations.
- `process_pending_entries_loop`: Loops forever fetching entries through an unnamed routine.
- `return_zero`: Returns the constant value zero to its caller without touching any data.
- `dispatch_by_status_code`: Compares a status value in the record with special codes: a status of 4 calls a routine with a record field and then continues, 64 returns the result of another routine, and 512 calls a routine and returns 0.
- `dispatch_record_type`: It loads a 16-bit type field from a record, dispatches on it via another routine, and then makes two calls to unnamed routines in sequence while returning one value.
- `call_routine_then_return_zero`: Makes one call to an unnamed routine and then returns zero.
- `call_two_routines_return_one`: Makes two calls to unnamed routines in sequence and returns one.
- `run_entry_callback_sequence`: Calls an unnamed routine three times with the same argument, and once more when the record's type byte is not 1.
- `format_protection_flag_strings`: Derives four bit values from a stored flag word and formats each one through an unnamed routine with a global text pointer.
- `format_flag_status_line`: Selects one of two string fragments according to a flag argument and builds a text from them.
- `format_string_argument_list`: Formats a temporary text buffer from two string pieces and a value taken from the caller's frame.
- `measure_string_length`: Returns the number of bytes before the terminating zero byte of a text at the given address.
- `count_bytes_until_terminator`: Returns the offset of the first zero byte in a text, counted from a base address given in a data register.
- `copy_cstring_to_buffer`: Copies a zero-terminated text from the source address to the destination, including the terminating zero byte.
- `copy_fixed_length_bytes_2`: Copies a given number of bytes from the source address to the destination address, one byte at a time.
- `copy_bytes_from_registers`: Copies bytes from the source to the destination using register inputs, one byte at a time, until the counter reaches the count.
- `compare_strings_for_equality`: Compares two zero-terminated texts, advancing through them while their characters match, and returns whether the characters where they stop are equal.
- `compare_register_strings`: Compares two zero-terminated texts whose addresses are in address registers and returns whether the characters where they stop are equal.
- `measure_string_including_terminator`: Returns the number of bytes in a zero-terminated text, counting the terminating zero byte.
- `run_three_library_operations`: Obtains a value from an unnamed routine, then calls three library-backed routines on a base taken from a field of the first argument.
- `call_string_formatter`: Formats text from a global pointer and an extra argument into a local 12-byte buffer.
- `copy_counted_string_with_terminator`: Reads a length value from a byte at a computed address, copies that many bytes from the source into the destination, and writes a zero byte after them.
- `copy_bytes_and_terminate`: Copies a counted run of bytes from the source to the destination and appends a zero byte after the last one.
- `initialize_entry_record_and_add`: Fills two new structures from its parameters, storing sizes, positions and flags into fixed fields, and calls two unnamed routines with them.
- `call_library_with_record_field`: Passes a field at offset 62 of the record in an address register to a library function at a fixed offset, then returns one.
- `call_library_sequence_with_fields`: Calls library functions in sequence on a base taken from the field at offset 50 of the first argument.
- `call_raw_format_function`: Calls the exec library's formatting function through a global base, using the arguments the caller has already set up.
- `store_byte_at_address`: Writes the low byte of the data register into the byte at the address held in an address register.
- `call_library_vector_36`: Loads a library base from a global variable and calls the library function 36 bytes before that base.
- `call_library_vector_72_2`: Loads a library base from a global variable and calls the library function 72 bytes before that base.

### version.library

- `return_zero_value`: This is an empty routine that returns a constant zero value without performing any operations.
- `store_register_values_in_struct`: Stores a register value and a second register value into two fixed fields of the structure addressed by the data register, then returns.
- `increment_counter_and_clear_flag`: Increments the 16-bit counter at offset 32 of the structure in the frame register and clears bit 3 of the byte at offset 42 in the same structure.
- `decrement_counter_and_call`: Decrements the 16-bit counter at offset 32 of the structure.
- `finish_counter_or_flag_entry`: If the counter at offset 32 is zero, it reads the value at offset 38, calls two library functions on a base stored at offset 34, and returns the value.
- `read_value_around_library_calls`: Reads a 32-bit value from offset 38 of the structure in one address register, then calls two library functions on the base in another register.
- `return_empty_stub`: This routine returns without executing any instructions and produces no side effects.

### Populous (1989)(Electronic Arts)[cr VF].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_custom_chips_open_graphics`: Writes custom-chip registers, including interrupt enable and DMA control, to disable interrupts and enable DMA. It clears a block of memory, waits on a status bit, and then uses exec functions to open the graphics library and close another library.
- `fetch_char_and_blit_glyph`: Fetches the next character from a text pointer when needed and finds its index in a fixed alphabet string.
- `blit_block_grid_from_table`: Programs the blitter across a 20 by 8 grid of blocks, writing source, destination and size registers for each block and waiting for the blitter between blocks.
- `toggle_bit_and_wait_loop`: Toggles one bit in a byte at the address held in an address register and clears all but two bits of a nearby word.
- `clear_masked_word_and_wait`: Keeps only two bits of a word at a fixed negative offset from an address register, clearing the rest.
- `poll_status_thirty_iterations`: Runs a fixed 30-iteration loop, calling an unnamed routine on each pass and polling a second routine until it returns zero.
- `start_blit_with_pointers`: Waits until the blitter is idle, then programs its control, mask, modulo, source and destination pointer registers, and starts a blit by writing its size register.
- `fill_words_from_table_count`: Looks up a count byte in a table, indexed by the data register, and calls two routines around that lookup.
- `fill_words_with_fixed_value`: Writes a fixed value into a run of 32-bit words starting at the address in an address register, with the count given in the data register.
- `repeat_call_until_zero`: Calls an unnamed routine repeatedly until it returns zero.
- `store_sum_through_pointer`: It adds two values held in registers and stores the result through a pointer argument.
- `swap_global_longs_and_split`: Swaps two 32-bit global variables, then stores the low word and the high word of the old first value into two word-sized globals.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 32 into d5, then merges 192 into d5.
- `swap_global_variables`: It swaps the contents of two global variables with each other.
- `setup_four_audio_channels`: Programs four audio channel register blocks with a shared sample pointer, a length of 13, a volume of 64 and periods that rise in steps of 4.
- `increment_byte_triplets_in_table`: Walks 110 records spaced 24 bytes apart and adds 1, 2 and 3 to three bytes in each record.
- `fill_table_records_with_pattern`: Writes fixed values into 140 six-long records starting at one address.

### Populous (1989)(Electronic Arts)[cr VF].adf:DAD (raw code file, load address unknown)

- `initialize_system_subsystems`: Modifies interrupt status in the INTENA register, manages serial I/O, and references graphics and intuition libraries.
- `toggle_interrupt_status`: Modifies the interrupt enable bit in the INTENA register and executes several short instruction sequences while referencing a text string.
- `reference_text_string`: Processes a routine that references a specific text string.
- `execute_short_sequence`: Performs a short sequence of ten instructions.
- `handle_serial_io`: Manages serial data by referencing serial read and serial write functions.
- `toggle_interrupt_enable`: Modifies the interrupt enable bit in the hardware interrupt enable register.
- `execute_minimal_sequence_2`: Runs twelve instructions, including a brief six-instruction sequence.
- `execute_minimal_sequence`: Performs a very brief sequence of six instructions.
- `access_graphics_intuition`: Uses thirty-four instructions to reference graphics and intuition libraries and executes a six-instruction sequence.
- `run_nested_routines_2`: Executes a series of medium and short routines in a nested fashion.
- `execute_nested_logic_tasks`: Executes a series of nested medium and short routines, including several logic sequences and data block processing tasks, while referencing a cheat text string.
- `run_nested_routines`: Consists of one hundred fifty-two instructions and calls two separate medium-length routines.
- `execute_complex_logic_3`: Triggers a large routine and performs fifty-six instructions of logic.
- `execute_complex_logic_2`: Contains fifty-six instructions and triggers a large routine consisting of one hundred twenty-seven instructions.
- `execute_complex_logic`: Executes a large routine of one hundred and twenty-seven instructions that calls another routine.
- `execute_medium_routine`: Performs a routine consisting of one hundred and three instructions.
- `execute_medium_routine_2`: Performs a routine consisting of ninety-five instructions.
- `perform_large_task`: Executes six hundred ninety-three instructions and calls a medium-length routine and a short task routine.
- `execute_medium_routine_3`: Performs a routine consisting of sixty-two instructions.
- `execute_medium_routine_4`: Performs a routine consisting of thirty-seven instructions.
- `execute_subroutine`: A routine consisting of one hundred twenty-seven instructions that calls another routine and is called by three different routines.
- `call_short_task`: Runs forty instructions and invokes a very short eight-instruction routine.
- `process_data_block`: A routine containing three hundred forty-five instructions that is called by one unnamed routine.
- `perform_minimal_task`: A very short routine consisting of eight instructions that is called by two different routines.
- `handle_logic_sequence`: A routine of seventy-three instructions that is called by one unnamed routine.
- `process_extended_logic_2`: A large routine that processes various logic blocks, frequent tasks, and subroutine logic while referencing a text string.
- `manage_multiple_tasks`: Coordinates a wide variety of operations including frequent tasks, extended logic, and intermediate data processing.
- `manage_complex_operation`: A routine consisting of two hundred fifty-four instructions that is called by one unnamed routine.
- `execute_long_sequence`: Comprises five hundred five instructions and calls a sixty-instruction sequence.
- `execute_short_sequence_2`: A routine of sixty instructions that is called by one unnamed routine.
- `process_intermediate_data`: A routine containing ninety-two instructions that is called by one unnamed routine.
- `process_extended_logic`: Executes sixty-nine instructions and calls both an extended logic routine and a subroutine logic routine.
- `dispatch_frequent_task`: A routine of one hundred twelve instructions that is called by six different routines.
- `execute_extended_logic`: A routine consisting of one hundred seventy-eight instructions that is called by four different routines.
- `dispatch_task_logic_2`: Invokes a frequent task dispatcher and an extended logic routine.
- `dispatch_task_logic`: This routine invokes a frequent task dispatcher and an extended logic routine.
- `invoke_frequent_task`: This routine calls a frequent task dispatcher routine.
- `execute_subroutine_logic_2`: This routine calls another subroutine to perform logic operations.
- `execute_complex_tasks`: This routine calls a complex logic routine, a subroutine, and a frequent task dispatcher.
- `run_large_routine`: A routine containing three hundred fifty-seven instructions that is called by one unnamed routine.
- `execute_complex_tasks_2`: Performs complex logic and subroutine execution by calling a frequent task dispatcher and other subroutines.
- `execute_subroutine_logic`: A routine consisting of 223 instructions that calls another routine and is called by four different locations.
- `perform_short_task`: A routine containing 85 instructions that is called by one other routine.
- `process_medium_block`: A routine consisting of 213 instructions that is called by a single caller.
- `handle_routine_calls`: A routine of 145 instructions that is invoked by two separate callers.
- `execute_logic_block`: A routine containing 120 instructions that is called by one unnamed routine.
- `run_instruction_sequence`: A routine consisting of 138 instructions that is called by one other routine.
- `run_small_routine`: This routine calls a small subroutine consisting of sixty-seven instructions.
- `execute_small_routine`: A routine of 67 instructions that is called by one unnamed routine.
- `process_large_block`: A routine containing 266 instructions that is called by one other routine.
- `execute_extended_logic_2`: A routine consisting of 376 instructions that is called by four different routines.
- `perform_fifty_instructions`: This routine executes a sequence of fifty instructions.
- `handle_file_operations`: Performs library calls to manage file loading and saving, referencing text strings for directories, errors, and filenames.
- `perform_brief_task`: A routine of 71 instructions that is called by one unnamed routine.
- `manage_hardware_and_logic`: Performs extended logic and manages hardware by accessing custom registers, serial data, and interrupt request registers.
- `handle_connection_logic`: This routine performs extended logic, updates a memory pointer via register arithmetic, accesses custom hardware and interrupt registers, and interacts with serial data hardware.
- `execute_fifty_instructions`: Performs fifty instructions and is called by four different routines.
- `process_game_state_logic`: Executes various logic blocks and undefined routines while referencing text strings related to game starting and world availability.
- `undefined_routine`: The routine consists of 273 instructions and is invoked by two unnamed callers.
- `call_subroutine`: An extremely short routine that calls another unnamed routine.
- `run_instruction_sequences`: Executes several specific instruction sequences of varying lengths.
- `execute_twenty_three_instructions`: Performs twenty-three instructions and is called by one routine.
- `execute_thirty_eight_instructions_2`: This routine calls a routine that performs thirty-eight instructions.
- `execute_thirty_eight_instructions`: Performs thirty-eight instructions and is called by one routine.
- `invoke_multiple_libraries_2`: This routine performs several calls to various library functions at specific offsets.
- `execute_twenty_instructions`: Performs twenty instructions and is called by one routine.
- `update_memory_pointer`: The routine performs arithmetic on a register and stores the resulting value into a memory location.
- `call_library_functions`: Performs a series of Amiga OS library calls and references a text string.
- `execute_sixteen_instructions`: Performs sixteen instructions and is called by one routine.
- `process_game_data`: Performs two hundred sixty-two instructions and references a data section labeled .GAM.
- `manage_interrupt_requests`: Performs twenty instructions and accesses the interrupt request and interrupt request mask registers.
- `access_custom_registers`: Performs twenty-two instructions and accesses the interrupt request register and a custom hardware register.
- `execute_subroutine_2`: A routine consisting of forty-nine instructions that is called by another routine.
- `access_serdat`: A routine that performs forty-six instructions and interacts with the serial data hardware register.
- `call_library_offset_102`: A short routine of four instructions that invokes a library function at a specific offset.
- `call_multiple_libraries`: A routine of five instructions that invokes two different library functions.
- `call_library_offset_132`: A three-instruction routine that calls a library function.
- `call_multiple_library_functions`: A routine of eighteen instructions that performs five separate library calls.
- `call_library_offset_90`: A four-instruction routine that invokes a library function.
- `execute_library_sequence_2`: This routine performs a series of multiple calls to Amiga OS library functions.
- `call_library_offset_354`: A three-instruction routine that invokes a library function.
- `execute_library_sequence`: An eighteen-instruction routine that performs five library calls in succession.
- `call_two_libraries`: A six-instruction routine that invokes two library functions.
- `call_os_libraries`: Executes nineteen instructions and invokes three different Amiga OS library functions.
- `invoke_multiple_libraries`: Performs twenty-three instructions and makes seven distinct calls to Amiga OS library functions.
