# Game 973 (north & south (1989)(infogrames)(m5)[cr cp]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00973). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (921760 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 261 of 261 named (0 author symbols), 261 of 261 described (24 by rule, 181 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2064 of 2064 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 5632 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5562 of 5562 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 29424 |
| code | 28910 |
| data | 863399 |
| padding | 27 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| North & South (1989)(Infogrames)(M5)[cr CP].adf:bootblock | 1 | 150 | 8 |
| logged disk load from North & South (1989)(Infogrames)(M5)[cr CP].adf boot-block load | 1 | 83 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 22 | 22 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1138 | 28 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 38 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 127 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 38 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3966 | 193 |
| North & South (1989)(Infogrames)(M5)[cr CP].adf bytes 91136-96768 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- north & south (1989)(infogrames)(m5)[cr cp]/North & South (1989)(Infogrames)(M5)[cr CP].adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- north & south (1989)(infogrames)(m5)[cr cp]/North & South (1989)(Infogrames)(M5)[cr CP].adf: the boot block reads 29696 bytes from disk offset 844800 to an address (found statically); recovered as a unit
- runtime segment an address: 22 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 560 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 115 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 33 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1315 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- north & south (1989)(infogrames)(m5)[cr cp]/North & South (1989)(Infogrames)(M5)[cr CP].adf: bytes 91136-96768 look like code no run loaded: recovered position-independently (load address unknown)
- north & south (1989)(infogrames)(m5)[cr cp]/North & South (1989)(Infogrames)(M5)[cr CP].adf: 2 logged loads cover 35328 bytes of the image
- north & south (1989)(infogrames)(m5)[cr cp]/North & South (1989)(Infogrames)(M5)[cr CP].adf: 24 track-loaded blocks found in recovered runtime segments and classified through them
- north & south (1989)(infogrames)(m5)[cr cp]/North & South (1989)(Infogrames)(M5)[cr CP].adf: blocks 54-174 (bytes 27648-89600) outside the filesystem hold data, not code (opcode density 0.048, entropy 6.54)
- north & south (1989)(infogrames)(m5)[cr cp]/North & South (1989)(Infogrames)(M5)[cr CP].adf: blocks 1708-1759 (bytes 874496-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 5.16)
- 1 never-loaded region(s), 5632 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 763904 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### North & South (1989)(Infogrames)(M5)[cr CP].adf:bootblock

- `read_disk_block_via_doio`: Boot-block entry that fills an I/O request with a disk offset, a byte length and a destination buffer, performs the read through the exec library DoIO call, and then continues into the loader.
- `load_amigados_hunk_executable`: Loads an AmigaDOS executable by allocating memory through the exec library, reading each hunk's size and memory flags, and processing code, data, BSS, 32-bit relocation and symbol blocks.
- `update_buffer_metadata`: Updates a structure with a new pointer, size, and offset values based on a condition.
- `read_next_buffered_long`: Calls a helper, decrements a remaining-count field, and returns the next 32-bit value from a buffer while advancing the buffer pointer.
- `call_helper_repeatedly`: Calls its helper once for each unit of its argument and stops when the count reaches zero.
- `call_free_mem`: Wrapper that calls the exec library function FreeMem and returns its result to the caller.
- `call_do_i_o`: Wrapper that calls the exec library function DoIO and returns its result to the caller.
- `call_alloc_mem`: Wrapper that calls the exec library function AllocMem and returns its result to the caller.

### logged disk load from North & South (1989)(Infogrames)(M5)[cr CP].adf boot-block load

- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `check_range_bounds`: Compares a range of values provided via a pointer against a global range or a fixed value, returning a counter or a status code.
- `swap_global_pointers`: Checks a global variable against a constant, then swaps two 32-bit values at a fixed offset from a global pointer before calling another routine.
- `toggle_global_state`: Toggles a bit in a global variable and swaps two 32-bit values at a fixed offset, then calls a routine to initialize channel slots.
- `update_pointer_list`: Performs a series of pointer-based searches and updates a list of pointers with calculated offsets or new addresses.
- `calculate_offset_list`: Executes a sequence of pointer-based searches and updates a temporary array with calculated address offsets or new pointers.
- `apply_bitmask_to_buffer`: Iterates through a list of bitmasks, applying bitwise AND and OR operations to a target buffer using a calculated pointer and bit-shifting logic.
- `set_buffer_pointers`: Updates several global pointers or buffer-related variables and initializes a 4-element array with a specific bit-pattern or zeroed values.
- `populate_address_arrays`: Fills two arrays with a sequence of incrementing addresses and calls a routine via a pointer-to-function before waiting for a hardware blitter.
- `initialize_blitter_params`: Calls a library routine, waits for a blitter, writes several constant values and parameters to specific RAM locations, and calls another library routine.
- `return_from_subroutine`: The routine contains a single instruction that exits the current execution flow and returns to a caller.
- `process_record_bitmask`: Iterates through a bitmask extracted from a record lookup to conditionally update or clear specific bytes and bit-flags within a memory-resident structure.
- `update_audio_channel_parameters`: Iterates through a table of audio channel structures to modify volume, or certain bit-flags, and update certain memory-mapped or pointer-based properties based on bits set in the input parameter.
- `copy_memory_block`: Copies a fixed number of words from a source pointer to a destination pointer and a second destination pointer.
- `insert_node_list`: Writes a function pointer to a table and inserts a new node containing a value and a pointer into a linked list.
- `remove_node_list`: Traverses a linked list to find a specific node, removes it by updating pointers, and calls a function via a pointer from a table.
- `update_blitter_rectangles`: Calculates new source and destination coordinates or dimensions and programs the blitter or chip registers to draw or clear rectangular areas.
- `update_bitmask_and_draw_rect`: Updates a bitmask or mask-table and performs a bitwise AND-masking operation on a block of video memory or a buffer.
- `memmove_range_with_length`: Copies a block of memory from a source pointer to a destination pointer using a specified length, choosing between a forward or backward copy direction to prevent overwriting data if the ranges overlap.
- `calculate_product_sum`: Calculates a value based on the product of high and low words from two registers and returns the result.
- `execute_indirect_call`: Peres a pointer lookup from a table and calls the resulting address.
- `execute_indirect_call_offset`: Performs an indirect call using a pointer retrieved from a table with a fixed offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `apply_descending_scaling`: Iterates through a table of sixteen words, applying a descending multiplier to specific bitfields within each word and passing the resulting table to another routine.
- `apply_ascending_scaling`: Iterates through a table of sixteen words, applying an ascending multiplier to specific bitfields within each word and passing the resulting table to another routine.
- `update_global_pointers`: Fetches a pointer from a global variable and stores the pointer and its subsequent data elements into several global locations.
- `fetch_and_pass_struct_members`: Retrieves a pointer from a table using a constant index, then extracts and stores three specific members from the pointed-to structure into local memory before passing two of those members as arguments to another routine.
- `pass_pointer_to_routine`: Passes a pointer stored in a global location to another routine.
- `extract_struct_members`: Retrieves a pointer from a global variable and copies several consecutive data elements from that pointer into a set of global variables.
- `execute_multiple_calls`: Performs several indirect function calls, including calls that pass certain values or pointers, and modifies a global variable.
- `fetch_and_scale_data`: Fetches several values from a table via indirect calls, performs arithmetic on a pointer, and stores the results into various global variables.
- `reverse_string_into_stack`: Reads a null-terminated string and stores its characters in reverse order into a destination buffer.
- `update_global_state`: Calls several subroutines and updates a global variable using a value returned from a function call, then executes a loop that calls a routine multiple times.
- `update_menu_display_parameters`: Updates global menu state flags and pointer-based table parameters while toggling display or string-related flags.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: removes the stack frame with unlk, then returns to the caller.
- `search_byte_in_table`: Iterates through a table of bytes to check if any byte matches a provided parameter.
- `advance_byte_stream_pointer`: Calls a helper with a fixed argument, then advances a byte-stream pointer by one and wraps it back to a fixed start when the next byte is a negative terminator.
- `search_word_in_table`: Iterates through a table of words to check if any word matches a provided parameter.
- `update_loop_and_counters`: Iterates a fixed number of times, checking for certain values in a table and updating a global counter or pointer-based index.
- `update_global_byte_from_offset`: Calculates an offset using bit-shifted values from two global shorts and stores a decremented byte from a table into a global variable.
- `update_byte_from_offset`: Calculates an offset using a bit-shifted value from two global shorts and stores a decremented byte from a table into a global variable.
- `initialize_data_table`: Clears a memory range and copies data from a source table to a destination table, then iterates through a pointer-based list to copy further data.
- `initialize_data_table_2`: Initializes a memory range by copying data from a source table and resets several global variables or pointers.
- `copy_global_data_1`: Copies a 32-bit value from one global memory location to another and branches to a later address.
- `copy_global_data_2`: Copies a 32-bit value from another global memory location to the same destination and branches to a later address.
- `copy_global_data_3`: Copies a 32-bit value from a third global memory location to the same destination and branches to a later address.
- `copy_global_data`: Copies a 32-bit value from one global memory location to another.
- `toggle_menu_state_and_update_display`: Toggles a global flag and updates a pointer to a global variable while checking for string termination.
- `traverse_and_update_tree`: Recursively traverses a tree-like structure, updating a byte in a table if the new value is smaller than the existing value or if the existing value is zero.
- `clear_table_entries`: Iterates through a series of memory offsets and sets a byte at a fixed offset from each pointer to zero.
- `update_table_parameters`: Updates specific bytes and words within a table or memory structure using offsets from a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `generate_random_quotient_remainder`: Advances a 32-bit seed using a timing value from a helper and a constant, then returns the seed's quotient and remainder by a divisor taken from the argument, packed into the two halves of the result.
- `compare_and_return`: Loads a byte from memory, extends it to word size, compares it with another value, and returns zero if they don't match or continues execution otherwise.
- `load_and_compare`: Loads a byte from a memory location with a negative offset from a4 into d3, extends it to word size, compares it with a word value at offset 8 of a5, and branches to an exit label if they are not equal.
- `fall_through_entry`: Entry point part-way through straight-line code: runs two instructions (clr.w, moveq) and then falls through into the code that follows, with no return of its own.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer_from_input`: Reads keyboard and mouse input or button states to update a stored pointer position, applying clamping to keep the coordinates within the screen boundaries, and writes the new position into a display-list word.
- `loop_and_clear_flag`: Repeatedly calls two different functions using pointers from a global structure until a non-zero return value is received, then clears a flag in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scan_array_for_bounds`: Iterates through an array of shorts, checking if any value falls outside a range defined by a global variable, and returns a counter or zero if a limit is reached.
- `store_pointer_address`: Stores the address of a data structure in a global variable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_raster_then_swap_pointers`: Waits for the blitter to go idle and for the raster position to pass a fixed scanline, then swaps two saved pointers in the data area and calls a helper.
- `wait_raster_then_swap_pointers_with_param`: Waits for a bit in a global variable to clear and for a certain value to be reached, then swaps two 32-bit values in a global structure and calls a helper routine.
- `toggle_and_update_pointer`: Toggles a bit in a global word and, depending on the previous state, either copies a 32-bit value or updates a pointer to a new memory address before calling a helper routine.
- `copy_table_entries_2`: Copies a 32-bit value from a table into two different locations in a global structure and calls another routine.
- `copy_table_entries`: Copies a 32-bit value from a table into two different locations in a global structure and calls a helper routine.
- `update_pointer_table`: Parses a data structure to retrieve a pointer and then iterates through a table of values, adding each value to its own memory address.
- `process_segmented_data`: Retrieves a data structure and copies a block of memory into a local buffer before performing calculations to determine a memory offset.
- `reorder_and_sort_index_table`: Copies a source buffer to a destination buffer and then performs a priority-based sort or reordering of an index table using a linked-list or lookup-table-based redirection mechanism.
- `fetch_table_value`: Retrieves a value from a data structure and uses it to look up an entry in a table via a function pointer.
- `process_buffer_operations`: Performs multiple buffer operations including assignment, range splitting, and structure updates using function pointers.
- `store_word_pair`: Stores two 16-bit values from the stack into global memory locations.
- `initialize_bitmask_table`: Calculates a pointer to a data source and populates a four-element table with bitmask values derived from a provided integer.
- `apply_bitmask_to_memory`: Iterates through a list of values to perform bitwise operations on multiple memory locations using bit-shifting and masking logic.
- `sum_glyph_advance_widths`: Walks a zero-terminated list of byte codes and returns the total advance, using each code's high four table bits or a fallback value when those bits are zero, plus a base offset per code.
- `nop_placeholder`: This routine is empty and performs no operations.
- `execute_blitter_sequence`: Performs a sequence of operations involving blitter hardware registers, waits for the blitter to become idle, and executes a library function and an indirect function call.
- `setup_bitplane_pointer_table_2`: Builds two arrays of four bitplane pointers 8000 bytes apart, sets a 40-byte pitch, 200-line height and four-plane count, then calls a library routine through a jump table and a helper.
- `clear_bitplane_with_blit`: Starts a blitter operation with only the destination channel enabled and a zero minterm, writing zeros over a full 200-line, 40-byte-wide bitplane at the stored destination pointer, with library calls before and after.
- `setup_blitter_transfer_parameters`: Configures the blitter control, mode, and size registers and sets the source and destination ort/masking parameters before executing the transfer.
- `decompress_bitstream_to_array_2`: Expands a bit-packed data stream by using bits from a source buffer to index into lookup tables and writing the resulting words into a destination array.
- `expand_compressed_data`: Decompresses a bitstream-based data stream by reading bits from a source to index into lookup tables and writing the resulting words into a destination buffer.
- `decompress_bitstream_to_array`: Iteratively extracts bit-packed or compressed data from a source buffer and reconstructs it into a destination array.
- `unpack_bitstream_data`: Extracts bit-packed or compressed data from a source buffer and reconstructs it into a destination array.
- `register_interrupt_handler`: Configures hardware registers and registers an interrupt service routine with the system.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `update_audio_entity_parameters`: Iterates through a table of pointers to update or clear specific flags and byte-values in an audio-related data structure based on bits set in an input parameter.
- `find_record_by_key`: Searches a zero-terminated list of three-word records for the key passed in a data register and leaves its address register pointing at the matching record.
- `store_channel_byte_pair`: Stores one byte value into two adjacent per-channel bytes that are both referenced from the channel record.
- `store_byte_with_clear`: Stores a byte value at a specific offset in memory and clears the carry flag.
- `store_byte_with_clear_2`: Stores a byte value at another specific offset in memory and clears the carry flag.
- `process_data_sequence`: A routine consisting of 49 instructions that performs a series of operations involving multiple registers and memory accesses.
- `configure_audio_hardware`: Processes a bitfield to set various hardware registers and control bits, including volume and channel settings.
- `update_audio_channels_and_dma`: On each tick where a pending flag is set, updates the four audio channels: writes volume and period to the audio registers, loads sample pointers and lengths from channel records, and sets DMA enable bits to start playback.
- `start_audio_channels_with_dma`: Iterates over the four audio channels and starts any with a pending sample by writing its pointer, length and period and setting its DMA enable bit, then marks the channel state after the loop.
- `start_audio_channels_entry_point`: Same four-channel start loop as its neighbour, entered partway through with the first channel's sample already loaded.
- `advance_modulation_step_return_offset`: Advances a channel's modulation position: adds a table delta when a flag is set, counts a repeat and on completion steps the envelope segment index, then returns the channel's position minus its base, clamped at zero.
- `set_channel_flag_bit_two`: Sets bit two in a channel's flag byte.
- `step_channel_sequence_script`: Advances a channel's command script: once its duration counter has expired it reads script bytes, where a high-bit byte selects a sample record or a control command and any other byte sets the note period from a table.
- `tick_channel_sequence_script`: Ticks a channel's sequence by decrementing its duration counter first, then runs the same command-script reading as its neighbour, writing the channel's volume and period values at the end.
- `start_channel_sequence_entry`: Starts a channel on a sequence entry: for each pattern entry it reads note commands until a note byte appears, then sets the channel's volume and period and returns.
- `start_channel_sequence_at_pattern`: Loads the sequence pointer for a given pattern index and reads the first command entries, setting the channel's volume and period and returning once a note starts.
- `step_channel_pattern_commands`: Reads pattern commands for a channel, applying sample and effect bytes and setting the note period from a table, and returns once a note is started.
- `apply_channel_command_then_note`: Applies a command byte already held in a data register, setting an effect or a sample record, then processes the next note and sets the channel's volume and period before returning.
- `process_channel_commands_until_note`: Sets the channel's period from a table index, then processes command bytes and applies control effects until a note byte starts the channel.
- `load_sample_record_then_commands`: Points a channel at a sample record and processes its command bytes until a note byte is reached, then sets the channel's volume and period and returns.
- `return_without_action`: A bare return with no work and no calls.
- `compute_channel_volume_output`: Subtracts an attenuation taken from the channel's field from a helper's level, floored at zero, and writes the result with a second helper value into the channel's two output words.
- `silence_audio_channel_volume`: Writes zero to one audio channel's volume register and sets a global bit mask to one shifted by the channel index.
- `load_channel_transpose_and_commands`: Stores the next script byte as the channel's transpose value, then reads command bytes until a note byte is reached, setting the volume and period and returning.
- `parse_script_data`: Reads bytes from a script buffer and updates state variables in a structure, including clearing and setting bit flags, updating pointers, and handling control flow based on byte values.
- `parse_script_command`: Reads and interprets script commands from a data buffer, updating state variables and memory locations based on command values.
- `jump_to_tick_channel_sequence_script`: Jump stub: passes control straight on to tick_channel_sequence_script.
- `copy_data_to_hardware`: Copies sixteen bytes from a source pointer to both a hardware register address and another memory location.
- `convert_integer_to_string_2`: Converts an integer into a decimal string and stores the characters in a buffer, then copies that string back to a destination address.
- `convert_integer_to_string`: Converts an integer value into a string of decimal characters in a buffer while updating a counter.
- `copy_string_and_increment_counters`: Iterates through a memory range, incrementing a byte and decrementing another, while copying a byte from a source to a destination calculated from a register.
- `zero_byte_at_offset`: Stores a zero byte into a memory location calculated from a register and an unable-to-determine offset.
- `convert_and_store_digits_2`: Converts numeric values into digit characters by performing division operations and stores them in memory locations while managing counters for different data types.
- `convert_and_store_digits`: Converts numbers into digit characters by performing division operations and stores them in memory locations while managing counters for different parts of the conversion process.
- `copy_byte_sequence`: Processes a sequence of bytes by incrementing a counter, decrementing another counter, loading a byte from a table, and storing it at a calculated memory location.
- `store_zero_byte`: Stores a zero byte at a memory location calculated from a register and an offset.
- `traverse_function_pointer_list`: traverses a linked list of pointers and executes a function pointer found at an offset from each element.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `insert_into_linked_list`: Executes a function pointer found in a structure and, if the return value is not null, inserts a new node into a linked list using a provided parameter.
- `remove_linked_list_node`: Searches a linked list for a specific node and removes it by updating pointers, then calls a function via a global pointer.
- `set_four_plane_pointers`: Writes a base address into four per-plane pointer slots, each 8000 bytes further along than the last, splitting each 32-bit address into high and low words.
- `init_seven_channel_slots`: Stores a 32-bit pointer from a data area into a header, then fills seven four-word slots with a fixed constant and zero.
- `call_nonzero_handler_table`: Walks an eight-entry table of routine pointers and calls each non-null entry in turn, returning zero.
- `check_and_store_pointer`: Loads a pointer from the stack, accesses a memory location with a negative offset, checks if it's zero, and stores the pointer in that location if it was zero.
- `decrement_and_loop`: Decrements a register and branches back to an earlier instruction if the result is not zero, then returns immediately.
- `update_flag_and_compare`: Updates a global variable with a value from another global location, then compares the original value with a register and branches if they are not equal.
- `decrement_and_loop_2`: Decrements a register and branches back to an earlier instruction if the result is not zero, then returns immediately.
- `setup_hardware_and_system`: Allocates memory, opens libraries, initializes several data records and hardware registers including DMA and display parameters, sets up interrupt servers, and configures bitplane and display control registers.
- `release_memory_resources`: Calls functions via pointers stored in a global variable and updates a global pointer with a value from a table.
- `clear_and_open_library`: Fills a memory range with zeros, opens a library, and executes a function from the returned library base if it is not null.
- `allocate_and_free_memory`: Allocates memory, calls a routine, then frees the allocated memory before returning.
- `wait_blitter_idle`: Spins until the blitter busy bit in the custom chip status register is clear.
- `enable_blitter_dma`: Writes the DMA control register with the set bit and the blitter enable bit, turning blitter DMA on.
- `disable_blitter_dma`: Writes the DMA control register with the blitter enable bit cleared, turning blitter DMA off.
- `perform_blitter_area_transform`: Calculates dimensions and source/destination offsets to perform a series of blitter operations.
- `swap_and_translate_byte_pairs`: Swaps bytes between two positions of a buffer, mapping each through a translation table as it goes, over a block whose dimensions come from the caller's registers.
- `compare_swapped_word`: Loads a longword from a pointer in a0, swaps its halves, compares the result with a word in a2, and sets d4 based on the comparison result.
- `outputclip`: Swaps the upper and lower 16-bit halves of a register and stores the result at a memory location pointed to by a register.
- `process_bitblt_data`: Initializes a bitblt operation by setting a flag in memory, performs a loop that processes data using arithmetic operations and bitblt functions, and finally copies data between memory locations before returning.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `update_bounding_box`: Updates minimum and maximum coordinates from input data, stores results in global variables, and performs bitblt operations based on coordinate comparisons.
- `configure_bitblt_registers`: Configures multiple bitblt hardware registers with specific values and performs bitblt operations using those registers.
- `process_scanline_data`: Processes word-sized elements from two memory locations, compares them, and updates pointers based on comparison results with conditional pointer swapping.
- `compare_scanlines`: Compares word-sized elements from two memory locations and updates pointers based on comparison results, with conditional pointer swapping and loop control.
- `process_bitmask_operations`: Processes bitmask operations using data from global structures, updates values based on bit comparisons, and performs bitwise AND operations with memory addresses derived from table lookups.
- `bitblt_mask_operation_1`: Performs bitwise AND operations on longword values at multiple memory locations with a register value, updates a pointer to point to the next memory location, decrements a counter, and branches back if the counter is not zero.
- `bitwise_operation_with_loop`: Performs bitwise XOR operations on word values at multiple memory locations based on bits in a register, then executes a loop that processes data from two pointers and applies AND operations with values from tables.
- `bitblt_mask_operation_2`: Performs bitwise AND operations on word values at multiple memory locations with a register value, updates a pointer to point to the next memory location, decrements a counter, and branches back if the counter is not zero.
- `bitwise_and_with_shifts`: Executes bitwise XOR operations on word values at multiple memory locations based on bits in a register, then performs a loop that processes data using shifts and AND operations with values from two tables.
- `bitblt_mask_operation_3`: Performs bitwise AND operations on longword and word values at multiple memory locations with register values, updates a pointer to point to the next memory location, decrements a counter, and branches back if the counter is not zero.
- `write_and_process_data`: Performs bitwise XOR operations on word values at multiple memory locations based on bits in a register, then executes a loop that processes longword data using shifts and AND operations with values from two tables.
- `write_bitblt_values`: Writes longword values from two data registers to multiple memory locations with specific offsets relative to a base address, then increments the base address and loops back if a counter is not zero.
- `call_function_with_parameters_2`: Saves registers on the stack, loads parameters from the stack, pushes them onto the stack in reverse order, calls a function through a pointer, stores results back to the stack, and restores registers before returning.
- `call_function_with_parameters`: Loads parameters from the stack, pushes them onto the stack in reverse order, then calls a function through a pointer.
- `update_collision_mask`: Calculates collision boundaries from input coordinates and updates graphical display masks based on coordinate comparisons and bit operations.
- `calculate_collision_flags`: Calculates movement coordinates using multiplication, division and addition, compares the results against boundary values, sets collision flags based on these comparisons, then returns with updated register values.
- `update_collision_flags`: Updates collision detection flags based on movement coordinates and boundary checks, then performs multiple library function calls with adjusted parameters.
- `update_display_mask`: Updates graphical display masks based on coordinate comparisons and bit operations, then configures hardware registers for DMA operations before returning.
- `update_structure_fields_2`: Repeatedly sets a word to zero and updates three long integer fields in a structure with increasing offsets, then calls another routine and sets a value at offset 150.
- `update_mask_bits`: Performs a bitwise AND operation between a word value and four specific memory locations, then updates those locations with the result.
- `and_mask_locations`: Performs a bitwise AND between a word in d1 and four specific word locations in a structure, updating those locations with the result.
- `validate_and_clear_region`: Validates coordinate boundaries and clears bits in a memory region by iterating through a range of values, using bit manipulation to modify specific locations in memory.
- `clear_bits_in_table`: Iterates through a table, clearing a specific bit in four different word locations per entry based on a bit index in d1.
- `copy_sequential_words`: Calculates a source address by adding an offset to a routine address, then copies a word and three words from that source and destination to another location before returning.
- `clear_bit_in_table_entries`: Calculates memory addresses based on parameters and clears a specific bit in four table entries using bitwise operations.
- `clear_bit_four_plane_bytes`: Clears one bit in four bitplane bytes at an address held in a pointer register, with the bit number taken from a register.
- `clip_bitplane_rectangle`: Calculates the intersection of a rectangular area with a bounding box and performs bitwise AND operations across four bitplanes to mask out specific regions.
- `and_mask_column_four_planes`: ANDs a 16-bit mask into a column of words across four bitplanes, stepping down the rows with a 40-byte pitch to clear the masked bits.
- `store_clip_bounds_pair`: Stores the two 16-bit halves of its argument into a pair of clip-bound words in the data area.
- `store_scaled_values`: Writes a 16-bit value from the stack to multiple memory locations with different bit shifts and additions, then returns.
- `wait_blitter_idle_keep_value`: Spins while the blitter busy flag is set, then returns its data register unchanged.
- `compare_strings_limited`: Compares two zero-terminated byte strings up to a count limit and returns the difference between the first mismatching bytes, or zero if they match or the count runs out.
- `compare_string_fields_limited`: Compares two strings reached through pointers in a parameter block, advancing both one byte at a time within a count limit, and returns the first byte difference or zero.
- `compare_strings_after_advance`: Advances two string pointers and a count, then compares the next bytes, returning their difference or zero at a terminator or when the count is exhausted.
- `compare_strings_advance_first`: Advances both string pointers and the count before each comparison, returning the difference of the first mismatching bytes or zero at a terminator or when the count runs out.
- `compare_string_case_insensitive`: Compares two strings by checking if characters fall within a certain range and adjusting them before comparison.
- `empty_routine`: The routine is empty and performs no operations before returning.
- `compare_strings_and_update`: Compares two string values using a limited comparison function and returns a word value of either zero or one based on the result.
- `compare_and_return_2`: Compares two string values using a limited comparison function and returns a word value of either zero or one based on the result.
- `clear_and_branch`: Clears a word register and branches to an earlier location in the code, creating a loop that continues execution from that point.
- `search_string_list`: Iterates through a list of strings, comparing each one to a parameter, and returns the number of successful matches or a failure code.
- `fetch_table_entry`: Retrieves a 32-bit value from a table using an index and a base pointer.
- `fetch_and_store_table_value`: Retrieves a 32-bit value from a table using an index and a base pointer, then stores the result into a memory location.
- `validate_string_search_result`: Checks if a string search returned a failure and returns either zero or the result of a subsequent recursive call to the same routine.
- `fetch_table_entry_wrapper`: Calls a routine to retrieve a 32-bit value from a table using an index and a base pointer.
- `update_structure_and_call`: Checks a value in a structure, calls a function with parameters from that structure, and then resets specific fields.
- `update_table_value`: Updates a value in a table using one of three modes: direct assignment, addition, or addition using a value from another table.
- `check_and_update_state`: Checks if a masked input value matches a stored global value, otherwise updates the global value and recursively calls another routine with a new parameter and a constant offset.
- `copy_string_block`: Copies a specified number of bytes from a source pointer to a destination pointer if the destination pointer is not null.
- `split_range_recursively`: Recursively splits or processes a range of memory or a pointer-based structure, updating a global pointer or table entry during the process.
- `execute_empty_routine`: the routine is empty and performs no operations before returning.
- `round_up_to_multiple`: Rounds up a 32-bit value to the nearest multiple of 512 by adding 511 and shifting right by 9 bits, then left by 9 bits.
- `process_data_chunks`: Processes data chunks by copying memory blocks, updating state flags, and adjusting pointers until a specific condition is met, then continues execution.
- `process_data_chunks_and_update_state`: Processes data chunks by copying blocks of memory, updating state flags, and adjusting pointers until a condition is met, then continues execution.
- `copy_and_update_loop`: Processes data chunks by copying blocks of memory, updating state flags, and adjusting pointers until a condition is met, then continues execution from a specific location.
- `copy_and_update_state`: Copies data using a string block function, updates state flags, and adjusts memory pointers while processing chunks of data until a condition is met.
- `jump_to_sub_33ce`: Unconditionally branches to address 13262 and does nothing else.
- `find_and_assign_buffer_entry`: Searches a list for a specific identifier and, if not found, attempts to assign a new entry or update a structure field.
- `update_structure_from_index`: Updates a structure field using a value from a table-based index and executes a function pointer if a certain condition is met.
- `setup_and_call_function`: Populates a structure with four values, calls a function via a pointer, and returns a boolean based on a byte in that structure.
- `update_structure_fields`: Stores four parameters into a structure and then calls a function pointer using a pointer from that structure.
- `update_structure_and_call_function`: Sets a four-byte value to zero and a two-byte value to nine within a structure, copies a four-byte value from a global variable into another field of that structure, and then executes a function via a pointer located in a certain memory area using the structure's base address.
- `update_structure_and_call_function_2`: Writes a constant value and a value from a global variable into specific offsets of a structure, then executes a function pointer located within another structure, returning a boolean based on whether a byte at a specific offset in the structure is zero.
- `set_and_get_value`: Writes a value to a structure field, calls a function pointer, and returns a value from a different structure field.
- `release_active_records`: Checks several global pointers and, if non-zero, calls other routines to reset or clear the data or resources associated with those pointers.
- `initialize_and_check_condition`: Initializes multiple data structures with specific values, performs library calls to retrieve long values and store them, then tests a condition and returns zero or one based on the result.
- `check_and_release_2`: Tests a global variable and returns immediately if it is non-zero, otherwise it performs a library call to check a condition, sets a word flag based on the result, and releases active records if the condition is not met.
- `check_and_release`: Performs a library call to check a condition, sets a word flag based on the result, and releases active records if the condition is not met.
- `set_and_branch`: Sets a word register to one and branches to an earlier location in the code, creating a loop that continues execution from that point.
- `safe_memory_copy`: The routine copies a specified number of bytes from a source to a destination, handling overlapping memory regions by copying backwards if the destination address is higher than the source.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `perform_signed_division`: Determines the sign of the quotient from the signs of both operands, calls an unsigned division helper, and negates the result when the signs differ.
- `compute_signed_division`: Performs signed division of two 32-bit values with sign handling, storing the quotient in a register and returning it.
- `return_unsigned_division_result`: Calls an unsigned 32-bit division routine and returns the result from a specific register.
- `divide_unsigned_32bit`: Divides an unsigned 32-bit value by a divisor using shift-and-subtract steps, returning the quotient and remainder; a fast path handles divisors that fit in 16 bits.
- `divide_unsigned_shift_subtract`: Performs a shift-and-subtract division of the data registers, leaving the quotient in one register.
- `divide_unsigned_bit_loop`: Runs a bit-by-bit division loop over the data registers with a step counter, stopping early when the counter is exhausted.
- `copy_string_with_limit`: Copies a string from a source to a destination buffer up to a maximum number of characters.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `initialize_and_compare`: Initializes structure fields with specific values, performs a library call to get a long value, stores that value in the structure, and compares it with -1.
- `initialize_structure_fields_and_call`: Initializes structure fields with specific values, performs a library call to get a long value, stores that value in the structure, and branches to another routine.
- `setup_and_branch`: Initializes structure fields with specific values, performs a library call to get a long value, stores that value in the structure, and branches based on a conditional test.
- `setup_pointer_chain_2`: Sets up a pointer chain by storing an address, adding four to it, clearing the next memory location, and then branching back to an earlier point in code.
- `reset_record_and_release_resources`: Releases a record's sub-block when one is present, marks the record's state as inactive and clears its counter field, then calls two routines with a stored index and a fixed code.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `check_condition_and_return`: Tests a global variable and returns immediately if it is non-zero, otherwise it returns without setting the return value.
- `initialize_and_loop`: Initializes a structure with predefined field values, calls a library function, and manages a loop that continues until a specific condition is satisfied.
- `initialize_structure_fields`: Initializes various fields in a structure with specific byte, word, and long values, then returns the address of that structure.
- `reset_record_fields`: Checks if a pointer is non-zero, then sets a byte and two dwords at specific offsets within a structure to their maximum possible values before calling another routine with a modified pointer.
- `clear_record_fields`: Sets a byte and two dwords in a structure to certain maximum values and then calls another routine.
- `call_library_function_198`: Indirectly calls a library function at offset minus 198 from a base pointer loaded from a global variable, then returns.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_indirect_function`: Calls a function pointer located at an offset within a structure.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_function_294`: Indirectly calls a library function at offset minus 294 from a base pointer loaded from a global variable, then returns.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base from a global variable and calls a library function at a specific offset.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `setup_pointer_chain`: Stores the address of its pointer argument into the memory location pointed to by the argument, adds four to that stored value, clears the next word in the pointed-to block, and stores the original pointer value into the following word.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
