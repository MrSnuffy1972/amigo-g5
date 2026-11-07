# Game 1218 (leisure suit larry 3 v1.003 (1989)(sierra)[cr sac]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01218). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3605072 top-level bytes in 25 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 174 of 174 named (0 author symbols), 174 of 174 described (39 by rule, 134 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 834 of 834 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3232 of 3232 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 7194 |
| code | 10648 |
| data | 2267097 |
| padding | 8389 |
| unused | 1311744 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Type | 2 | 117 | 2 |
| ENDCLI | 2 | 117 | 2 |
| ECHO | 2 | 117 | 2 |
| ADD21K | 3 | 480 | 33 |
| Disk-Validator | 1 | 386 | 1 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 1 of 4).adf:LARRY3 | 3 | 942 | 55 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 1 of 4).adf:PROG | 66 | 29 | 2 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 1 of 4).adf:AMIGASND.DRV | 3 | 1030 | 74 |
| Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 1 of 4).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |

## Cases handled

- leisure suit larry 3 v1.003 (1989)(sierra)[cr sac](disk 3 of 4)/Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 3 of 4).adf: file RESOURCE.004 (header 513337 bytes, blocks hold 407968): its blocks are judged as raw blocks outside the filesystem
- leisure suit larry 3 v1.003 (1989)(sierra)[cr sac](disk 1 of 4)/Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 1 of 4).adf:RESOURCE.001: high-entropy data file (7.68 bits/byte), classified as packed data
- leisure suit larry 3 v1.003 (1989)(sierra)[cr sac](disk 2 of 4)/Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 2 of 4).adf:RESOURCE.002: high-entropy data file (7.84 bits/byte), classified as packed data
- leisure suit larry 3 v1.003 (1989)(sierra)[cr sac](disk 4 of 4)/Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 4 of 4).adf:RESOURCE.004: high-entropy data file (7.85 bits/byte), classified as packed data
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- leisure suit larry 3 v1.003 (1989)(sierra)[cr sac](disk 1 of 4)/Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 1 of 4).adf:PROG: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- 3 packed track region(s), 532992 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_data_block`: Copies sixteen bytes from a source offset to a destination pointer if the destination pointer is not null, then calls another routine.

### ENDCLI

- `update_linked_list_and_copy_data`: Traverses a linked list to find the maximum value in a specific field, then updates a buffer with a header and performs a series of conditional memory copies between a source and a destination.
- `copy_memory_block`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### ECHO

- `traverse_list_and_populate_buffer`: Traverses a linked list to find the maximum value in a specific field and then populates a data structure with a header and a sequence of incrementing values.
- `copy_memory_block_2`: Copies sixteen bytes from a memory location offset from a pointer to a destination address.

### ADD21K

- `initialize_and_traverse_data_structures`: Calls a routine to clear a block of memory and store a pointer, then performs conditional checks and nested pointer dereferencing to update a byte and call another routine with specific parameters.
- `configure_display_surface`: Looks up two handles through a lookup routine, releasing them via library stubs if a lookup fails, then sets a flag byte in an associated display structure and calls a routine with the sizes 640 and 256.
- `clear_block_and_store_pointers`: Zeroes forty-one 32-bit words starting at an offset from a base pointer taken from the stack, then stores the address of a small stack buffer and a global value into two fields of that same block.
- `call_optional_handler_and_recurse`: Checks if a global variable is non-zero to call a function pointer, then calls itself recursively.
- `parse_script_and_initialize_objects`: Splits a text buffer into a null-terminated array of word pointers using a character class table and replaces separators with null bytes.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `initialize_state_then_return`: Calls another routine ten times, then calls an optional handler from a global and conditionally calls routines with values taken from globals.
- `append_string_with_count_2`: Finds the end of a destination string and appends bytes from a source string until a null terminator is reached or a specified count is exhausted.
- `append_string_with_count`: Finds the end of a destination string and appends bytes from a source string until a null terminator is reached or a specified count is exhausted, then null-terminates the result.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `compute_string_length`: Walks forward through a zero-terminated byte string to calculate the number of bytes preceding the terminator.
- `copy_bytes_zero_padded`: Copies bytes from a source to a destination up to a specified count or until a zero byte is found, then fills the remaining destination bytes up to that count with zeros.
- `lookup_and_clear_table_entry`: Validates an index against a range, checks a table for a non-zero value, and if valid, calls another routine with that value before clearing the table entry.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_indirect_library_function`: Performs an indirect call to a function located via an offset within a pointer retrieved from a global variable.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `execute_indirect_function_pointer`: Loads a pointer from a location calculated using an offset from a global variable and executes the function at that address.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `call_indirect_function_via_offset`: Fetches a function pointer from a table using an offset from a base address and executes it.

### Disk-Validator

- `format_display_string`: Converts a numeric value into a character representation and stores it in a buffer.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 1 of 4).adf:LARRY3

- `clear_and_call_function`: Zeroes a series of words in a local memory area and then executes a function pointer retrieved from a global variable.
- `initialize_object_state`: The routine writes a value from a parameter into an object field, sets a word field to a constant, stores the address of a global variable into a pointer field, and clears two dword fields.
- `delay_and_configure_game_object`: This routine contains a short busy wait and then queries a global structure, storing several fields into a stack frame.
- `initialize_resource_entry`: Prepares a resource entry in a global structure, writes a constant marker into the entry, and then either loads a resource descriptor from a table or forwards parameters to another routine.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `parse_resource_block_metadata`: Parses a resource block to extract string data and associated metadata.
- `decode_resource_block`: This routine decodes a block from a resource file.
- `memmove_block`: The routine copies a block of memory from a source address to a destination address, handling overlapping memory regions by checking the relative positions of the pointers.
- `copy_memory_block_3`: The routine copies a block of bytes from a source pointer to a destination pointer, handling both forward and reverse copy directions depending on the relative order of the pointers.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `parse_quoted_string_list`: Parses a string into a list of individual tokens, handling whitespace and text enclosed in double quotes.
- `copy_string_with_limit_2`: Copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument is reached.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `copy_string_pad_zeros`: The routine copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `open_and_load_resource`: Opens a file and performs a series of nested indirect calls to retrieve data from a table.
- `execute_conditional_function_pointer`: Checks if a value at a specific offset from a pointer is non-zero, and if so, executes the function located at that address.
- `call_indirect_if_flagged_then_recurse`: Checks a global variable and, if it is non-zero, calls the routine whose address it holds.
- `process_resource_descriptor`: The routine reads a descriptor stored in a register-based structure and conditionally invokes other routines with fields from that descriptor.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `validate_and_clear_table_entry`: The routine checks if an index is within the bounds of a table and ensures the target entry is not null.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_dos_function`: The routine performs an indirect call to a function located at an offset within a table.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_exit`: Wrapper that calls the dos library function Exit and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_indirect_routine_via_table`: The routine loads a function pointer from a table stored in a global variable and calls the referenced routine with no arguments.
- `call_library_offset_19e`: Performs an indirect call to a routine located at a specific offset within a table.
- `call_library_function`: The routine performs an indirect call to a function located at a specific offset within a library table.
- `execute_indirect_function_pointer_2`: Retrieves a function pointer from a nested table structure using an offset from a base address and executes it.
- `call_indirect_function`: Performs an indirect call to a function pointer retrieved from a table using an offset.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `execute_indirect_table_function`: Accesses a global variable to retrieve a pointer, then follows a secondary offset within a table to execute a function located at that destination.
- `call_indirect_function_2`: Executes a function located at an offset within a table pointed to by a global variable.
- `call_indirect_function_3`: Calls a subroutine by dereferencing a pointer found at a specific offset in a table.
- `call_indirect_function_4`: Uses a pointer from a table to jump to another routine.
- `execute_indirect_table_function_2`: Performs a double indirect jump by traversing two pointers stored in memory relative to a base address.
- `call_indirect_function_5`: Retrieves a function address from a table and transfers control to it.
- `execute_indirect_table_entry`: Performs a double-dereference to call a routine located via a pointer within a table.
- `call_indirect_function_6`: Performs an indirect jump to a routine based on a table lookup.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_indirect_function_7`: Calls a routine via a pointer stored in a table at a specific offset.
- `execute_indirect_function`: Performs an indirect call to a function address retrieved from a table using an offset from a pointer.
- `call_indirect_function_8`: Transfers execution to a function address obtained from a table.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_indirect_function_9`: Executes a function by dereferencing a pointer from a table.
- `call_indirect_function_10`: Performs an indirect call to a routine using a table-based pointer.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 1 of 4).adf:PROG

- `run_startup_and_free_memory`: Performs program startup tasks, executes a function from an address register, runs a loop that calls a memory free routine, and clears a longword.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 1 of 4).adf:AMIGASND.DRV

- `deferred_callback_dispatcher`: Checks a one-bit flag in a global variable and, if clear, stores a short value and calls a routine via an indirect pointer before clearing the flag.
- `initialize_sound_command_structure`: Initializes a sound command structure by parsing a data buffer to determine playback parameters such as frequency, length, and sample data offsets.
- `initialize_sound_structure`: Sets a byte at a specific offset in a structure to zero and writes a constant word to another offset.
- `parse_sound_command`: Searches for a marker in a buffer, calculates lengths, determines frequency constants, and copies data to a destination structure.
- `decode_sound_command_block`: The routine initializes a command block in a provided structure, decodes a stream of bytes from a pointer by selecting bits based on a mask from a global variable, and writes up to four selected bytes into a buffer.
- `check_bell_and_write_byte`: The routine checks if the first byte pointed to by a pointer is a bell character and a byte at an offset from another pointer is zero.
- `store_byte_conditional`: The routine checks a global variable for the value 4 and, if it is not equal, stores a byte from a pointer into a byte in a table addressed by a global variable and an index register.
- `update_sound_parameter`: Checks a byte value and either calculates a distance from a base pointer or stores the byte as a word into a structure.
- `store_word_to_structure`: Stores a 16-bit value from a register into a 16-bit field at an offset within a structure.
- `parse_audio_command_stream_12`: Iterates through a byte stream to extract control parameters and update an internal audio playback state structure.
- `reset_sound_structure`: Sets a global variable to zero and updates a structure by writing a word and a byte to specific offsets.
- `update_audio_state`: Manages state transitions and decrements counters within a structure to control sound playback logic.
- `update_sound_playback_counters`: Decrements a byte-sized counter and, upon reaching zero, decrements a word-sized counter and resets specific fields in a structure.
- `decrement_resource_counters`: Decrements byte and word values within two different structures and clears specific fields in a global structure if a counter reaches zero.
- `clear_sound_state`: Calls another routine and, if a global variable is zero, writes a constant word and a constant byte to a structure.
- `parse_audio_command_stream_11`: Iterates through a byte stream to extract control parameters and update an internal audio playback state structure.
- `parse_audio_command_stream_10`: Iterates through a byte stream to extract control parameters and update an internal audio playback state structure.
- `modify_structure_fields`: Decrements a word at one offset, writes two bytes to other offsets, and performs arithmetic on a word at a different offset.
- `parse_audio_command_stream_9`: Iterates through a byte stream to extract control parameters and update an internal audio playback state structure.
- `decode_audio_command_stream`: Parses a byte stream to extract control parameters and update an internal audio playback state structure.
- `parse_audio_command_stream_3`: Iterates through a data stream to decode command bytes into control parameters.
- `parse_audio_command_stream_7`: Iterates through an encoded byte stream to decode control parameters and update an internal audio playback state structure.
- `update_audio_command_state`: Parses an encoded byte stream to update an internal audio playback state structure.
- `parse_audio_command_stream_4`: Parses a stream of encoded bytes to extract control commands and lookup values.
- `parse_audio_command_stream`: Parses a stream of encoded bytes to extract control commands and lookup values.
- `parse_audio_command_stream_8`: Iterates through a command stream to decode control bytes and update an audio channel structure.
- `parse_audio_command_stream_6`: Iterates through a data stream to decode command bytes into control parameters.
- `parse_audio_command_stream_5`: Iterates through a data stream to decode command bytes into control parameters.
- `parse_audio_command_stream_2`: Iterates through a data stream to decode command bytes into control parameters.
- `write_structure_field`: Writes two bytes from registers into consecutive memory locations within a structure.
- `update_structure_field_via_offset`: The routine writes two bytes from registers into consecutive memory locations within a structure.
- `reset_sound_driver_state`: The routine clears a word in a structure pointed to by a register, copies a word from an offset within the same structure to another offset, calls another routine, then writes a byte and a word to further offsets in the structure.
- `clamp_and_scale_value`: The routine clamps a 16-bit value at an offset in a structure pointed to by a register to a maximum and, if a flag byte at another offset is zero, copies the clamped value scaled by four into a 16-bit field in a second structure pointed to by another register.
- `restore_saved_word_and_call`: The routine saves a word from a structure pointed to by a2, clears that word, calls another routine four times, then restores the saved word.
- `set_status_byte`: The routine calls another routine and then writes a byte to a specific offset in a structure pointed to by a register.
- `set_structure_bytes`: Stores two specific byte values into consecutive offsets of a structure pointed to by a register.
- `configure_audio_channel`: The routine writes to Amiga audio channel 0 registers and DMA control registers, disables and re-enables interrupts, and copies values from a table pointed to by a global variable into hardware registers.
- `configure_audio_channel_one`: The routine disables interrupts and DMA, then writes to the audio channel one registers to set period, volume and length, and writes to DMACON and INTENA to re-enable DMA and interrupts.
- `stop_audio_channel_two`: The routine disables interrupts and DMA, then writes to the audio channel two registers to stop its output and clears the channel’s length and period, and finally re-enables interrupts and DMA. It also writes a few values to memory locations that appear to be hardware registers and sets a flag in a structure pointed to by a register.
- `configure_audio_channel_three`: The routine disables interrupts, programs the Amiga audio channel three registers for period, volume and length, enables DMA for the channel, then re-enables interrupts.
- `set_audio_dma_control`: The routine writes a mask derived from an input value to a global variable and then writes a shifted version of that mask to another global variable.
- `update_audio_sample_buffer_2`: Iterates through a data stream to perform sample rate conversion and volume adjustments.
- `process_audio_stream`: Iterates through a data buffer to perform arithmetic on sample values and update a destination structure.
- `update_audio_sample_buffer`: Iterates through a data stream to perform arithmetic on sample values and update a destination structure.
- `process_audio_data_stream`: The routine iterates through a data buffer, performing conditional checks and arithmetic on bytes to update values in a destination structure.
- `write_scaled_audio_register`: Multiplies an input value by a factor from a global variable, shifts the result right by six bits, and stores it in a hardware audio register calculated from another input value.
- `set_audio_channel_volume`: The routine calculates a value by multiplying two shorts from a global variable, shifting the result, multiplying it by a byte from a table indexed by an input value, and shifting again before writing the final result to a custom hardware register.
- `clear_sound_flags`: The routine checks a byte in a structure pointed to by a register for four individual bits.
- `update_sound_parameter_structure_3`: If a specific flag in a source structure is set, it clears a field in that structure and writes three values to a destination structure.
- `update_sound_parameter_structure`: If a specific flag in a source structure is set, it clears a field in that structure and writes three values to a destination structure.
- `copy_sound_driver_fields`: The routine copies a 32-bit value and two 16-bit values from a source structure pointed to by a1 into fields at fixed offsets within a destination structure pointed to by a0.
- `update_sound_parameter_structure_2_2`: If a condition in a source structure is met, it clears a field and sets specific values in a destination structure; otherwise, it copies a 32-bit value and two 16-bit values from a source structure.
- `update_sound_parameter_structure_2`: If a condition in a source structure is met, it clears a field and sets specific values in a destination structure; otherwise, it copies a 32-bit value and two 16-bit values from the source structure to the destination structure before setting two fixed values.
- `copy_sound_header_fields`: The routine copies a 32-bit value and two 16-bit values from a source structure pointed to by a1 into fields of a destination structure pointed to by a0, then writes two constant 16-bit values into additional fields of the destination structure.
- `update_sound_parameters_3`: If a value in the source structure is one, it resets specific fields in the destination structure and sets two word values to a constant.
- `update_sound_parameters`: If a value in the source structure is one, it resets specific fields in the destination structure and sets two word values to a constant.
- `copy_sound_parameters`: The routine copies a long word and two word values from a source structure pointed to by a1 into fields of a destination structure pointed to by a0, then writes constant word values into two other fields of the destination structure.
- `update_sound_parameters_2_2`: Checks a flag in a source structure and either clears a field and sets two specific values or copies a 32-bit value and two 16-bit values from a source structure.
- `update_sound_parameters_2`: Checks a flag in a source structure and either clears a field and sets two specific values or copies a 32-bit value and two 16-bit values from the source into a destination structure before setting those same two values.
- `copy_sound_params`: The routine copies a 32-bit value and two 16-bit values from a source structure pointed to by a register into specific offsets of a destination structure pointed to by another register.
- `execute_cleanup_callbacks`: The routine calls several function pointers stored in a structure and then calls another routine that takes a single argument.
- `restore_audio_state`: Saves a two-byte value from a structure, calls another routine, and if the result is not three sets a flag byte.
- `update_audio_sample_buffer_3`: Updates a per-channel audio structure by fetching sample data from a source table based on a pointer and an index.
- `update_audio_channel`: Updates a structure by copying bytes from various tables and memory locations based on conditional checks.
- `update_audio_channel_state`: The routine calculates an index into a table, updates a value in that table, sets a bit in a flag byte, and writes a word to a structure offset.
- `set_flag_and_write_byte`: The routine sets a bit in a byte located via a pointer and writes a 16-bit value from a register to a table entry, then calls another routine.
- `update_conditional_byte`: The routine compares a byte from a pointer with a byte from a table and, if they match, copies a byte from one memory location to another based on an index.
- `compare_and_copy_byte`: The routine compares a byte pointed to by a pointer with a byte from a table indexed by a word offset.
- `tiny_moveq_move_rts`: Small routine of 3 instructions that loads 255 into d0, then copies 4 to d2, then returns.
- `initialize_audio_channels`: Allocates memory for audio control structures, copies initialization data from a global table, and populates the structures with hardware parameters and interrupt handler pointers.
- `configure_sound_channel`: Initializes or resets a specific sound channel by loading parameters from a sound definition table into a global channel state structure.
- `initialize_sound_channel`: The routine initializes a sound channel entry in a global table and sets up related per-channel state bytes in a global structure.
- `initialize_sound_driver_state_2`: Writes several constants and values from a global structure into a target structure, including a clamped value and a conditional field assignment.
- `initialize_sound_driver_state`: The routine writes constants and values read from a global structure into fields of a structure pointed to by a pointer, clamping one field to a maximum value.

### Leisure Suit Larry 3 v1.003 (1989)(Sierra)[cr SAC](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `execute_indirect_call`: This routine performs an indirect call to an address stored in a memory location.
