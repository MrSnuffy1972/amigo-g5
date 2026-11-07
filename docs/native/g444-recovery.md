# Game 444 (Goal!): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00444). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1829680 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 240 of 240 named (0 author symbols), 240 of 240 described (5 by rule, 210 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2567 of 2567 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 111104 bytes, 3 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5008 of 5130 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 29266 |
| code | 26948 |
| data | 1758104 |
| padding | 2 |
| unused | 15360 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf:bootblock | 1 | 156 | 2 |
| Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| logged disk load from Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| logged disk load from Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf boot-block load | 1 | 338 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 39 | 39 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 492 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 514 | 17 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 170 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3331 | 147 |
| Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf bytes 1024-5120 (code never loaded by a reference run) | 1 | 0 | 0 |
| Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf bytes 5632-6144 (code never loaded by a reference run) | 1 | 0 | 0 |
| Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf bytes 8192-111616 (code never loaded by a reference run) | 1 | 0 | 0 |
| Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf bytes 114688-115200 (code never loaded by a reference run) | 1 | 66 | 2 |
| Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf bytes 121856-123904 (code never loaded by a reference run) | 1 | 14 | 1 |
| Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 2 of 2).adf bytes 1024-1536 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1742 non-empty blocks: matched against track-loaded segments
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 2 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1744 non-empty blocks: matched against track-loaded segments
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: the boot block reads 24576 bytes from disk offset 155648 to an address (found statically); recovered as a unit
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: the boot block reads 1024 bytes from disk offset 154624 to an address (found statically); recovered as a unit
- runtime segment an address: 24 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 335 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 461 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 151 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1596 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: bytes 1024-5120 look like code no run loaded: recovered position-independently (load address unknown)
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: bytes 5632-6144 look like code no run loaded: recovered position-independently (load address unknown)
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: bytes 8192-111616 look like code no run loaded: recovered position-independently (load address unknown)
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: bytes 114688-115200 look like code no run loaded: recovered position-independently (load address unknown)
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: bytes 121856-123904 look like code no run loaded: recovered position-independently (load address unknown)
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 2 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 2 of 2).adf: bytes 1024-1536 look like code no run loaded: recovered position-independently (load address unknown)
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: 7 logged loads cover 136192 bytes of the image
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: 36 track-loaded blocks found in recovered runtime segments and classified through them
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: blocks 247-301 (bytes 126464-154624) outside the filesystem hold data, not code (opcode density 0.042, entropy 5.61)
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: blocks 620-1495 (bytes 317440-765952) outside the filesystem hold data, not code (opcode density 0.009, entropy 6.26)
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 1 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf: blocks 1503-1504 (bytes 769536-770560) outside the filesystem hold data, not code (opcode density 0.023, entropy 3.78)
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 2 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 2 of 2).adf: 1 logged loads cover 512 bytes of the image
- goal! (1993)(virgin)(it)[cr dynamix][tr en](disk 2 of 2)/Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 2 of 2).adf: blocks 22-1759 (bytes 11264-901120) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.54)
- 5 never-loaded region(s), 110592 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 3 packed track region(s), 262144 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `set_cia_bit` (runtime segment an address): Sets the highest bit of a global variable, which corresponds to a hardware register bit. Kind: disk hardware/timing check. Pass path: bra.b an address.
- `toggle_global_flag` (runtime segment an address): Checks if a global variable is zero, and if so, sets it to a specific value before returning. Kind: disk hardware/status check. Pass path: the branch where DAT_0000ed4a is non-zero (skipping the write/toggle logic).
- `process_joystick_input_and_halt` (runtime segment an address): Reads input from the CIA hardware registers and updates a global variable based on bitwise transformations. Kind: manual/code-wheel/color-card lookup. Pass path: the branch where d0 == 0 (after tst.w d0, bne.b an address).
- `toggle_cia_bits` (runtime segment an address): Sequentially sets and clears specific bits in a hardware register through a global variable. Kind: disk hardware timing/sync check. Pass path: the branch following the call that continues normal execution (e.g., the 'bgt.b' or 'bra.b' paths).
- `modify_bit_and_loop` (runtime segment an address): Clears a bit in a global variable, then sets a bit in the same variable before entering an infinite loop. Kind: disk hardware status check. Pass path: btst.b #an address, an address.l results in zero flag (bit 4 is 0).
- `poll_hardware_status` (runtime segment an address): Iteratively checks bits in a value, likely representing hardware status, until a condition is met or a counter expires. Kind: disk hardware/sync status check. Pass path: the branch where the loop exits via the 'if ((DAT_00dff01f & 2) != 0)' condition, returning 0.
- `update_bitmask_data` (runtime segment an address): Manipulates a word by performing bitwise operations and masking based on a flag found in the byte preceding a pointer. Kind: bitmask/interleave de-scrambling. Pass path: the branch where the bit manipulation is applied to the data (the 'after' instructions following the call).

## Functions


### Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf:bootblock

- `allocate_buffers_and_issue_io`: Boot block entry that allocates a buffer through exec, fills an I/O request with a start address and length, and issues it through the DoIO call.
- `nop_routine`: The routine is empty and contains no executable instructions.

### Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 2 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### logged disk load from Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf boot-block load

- `allocate_and_configure_hardware`: allocates memory via a library call and configures hardware registers including color, DMA, and interrupt settings before performing a series of I/O operations and a memory copy loop.
- `forward_io_request_to_exec`: Passes control to exec's DoIO call through the library base and returns.
- `set_colours_then_dispatch`: Writes a DMA control value and two colour register values, issues a trap instruction, clears the colour register and then calls through a jump table entry that could not be recovered.
- `clear_colour_then_dispatch`: Clears the colour register to zero and then calls through a jump table entry that could not be recovered.
- `copy_relocate_code_block`: Copies two word blocks into a target area, rewrites the leading branch word to one of two fixed forms when it differs from the expected value, and applies a byte-coded relocation list that adds the load base to longwords.
- `copy_blocks_relocate_and_zero`: Copies two word blocks into a target area from a source pointer, then, when a stacked flag is zero, applies the same byte-coded relocation list that adds a base address to longwords.
- `apply_relocation_list_bytes`: Walks a zero-terminated byte list of relocation offsets, adding a base address to the longword at each offset; a code of one advances the position by 254 bytes.
- `zero_fill_word_block`: Clears a block of words to zero, one word at a time, for the byte count held in a register.
- `decrunch_backward_bit_stream`: Checks a four-byte signature at the start of the input, then decodes a backward bit stream of literal bytes and back-references, with lengths and distances taken from small code tables.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `trigger_trap`: Stores a constant value into a memory location and executes a software trap.
- `wait_for_flag`: Sets a byte in memory to one and enters a loop that continues until that same memory location is cleared.
- `call_nested_routines`: An empty routine that calls two other routines.
- `execute_recursive_calls`: This routine calls itself twice before returning a concatenated value from the data registers.
- `nop_padding_instruction`: This routine consists of a single instruction that performs no operation.
- `transfer_and_call_data`: Copies two words between memory locations, calls a function, and performs conditional logic involving further function calls and memory writes.
- `call_external_function`: An empty routine that calls another routine.
- `call_indirect_function`: An empty routine that calls a function through a pointer stored in memory.
- `call_indirect_function_2`: Calls a function through a pointer stored at an offset from a base address.
- `update_state_parameters`: Compares a value against a stored short and updates several table entries and a long value based on the input.
- `update_position_with_bounds`: Checks if two coordinates are within specific bounds and updates them by adding values, resetting to zero if a limit is reached.
- `generate_bitmask_table`: Allocates memory for a table and populates it by performing bitwise operations on data based on a bitmask.
- `perform_infinite_recursion`: The routine calls itself indefinitely without performing any other operations.
- `execute_conditional_calls`: Calls a function once or twice depending on whether a parameter is zero.
- `iterate_and_call_function`: Traverses a linked list and calls a function for each element in the list.
- `copy_string_to_buffer`: Copies a string into a buffer, manages a linked list of buffers, and updates a global pointer.
- `process_string_and_allocate`: Copies a string, performs several conditional checks and function calls, and eventually stores a long value into a table.
- `manage_string_buffer_logic`: Copies a string to a buffer and executes a complex sequence of conditional function calls and memory updates.
- `update_conditional_state`: Checks specific values in a table and updates various offsets based on conditional logic and the results of another routine.
- `execute_single_instruction`: This routine consists of exactly one instruction and is entered from outside the static code flow during execution.
- `null_operation_padding`: This routine is empty and contains a single instruction that does not perform any data processing or branching.
- `calculate_offset_index`: Computes a new index based on input parameters and table values, then updates several offsets in a table.
- `copy_data_conditionally`: Checks a condition in a table and, if met, copies a four-byte value from one memory location to another.
- `shift_bit_sequences`: Performs a loop that shifts bits through three consecutive word-sized values in a table using bitwise operations and XOR.
- `nop_padding_byte`: This routine consists of a single instruction that performs no operation, serving as a padding byte within the executable segment.
- `copy_memory_block`: Copies a block of data between two memory locations using a loop that transfers eight-byte chunks.
- `store_word_via_pointer`: Stores two consecutive words into a memory location calculated by adding an offset to a value held in a register.
- `apply_bitmask_to_buffer`: Performs bitwise AND, OR, or XOR operations on a target memory buffer using a source value and a bitmask.
- `calculate_product`: Performs a mathematical calculation involving division, bit shifting, and multiplication of two input parameters.
- `update_state_and_call`: Validates three input parameters against specific ranges, stores them in a table, and executes a series of function calls.
- `process_bit_stream`: Iterates through a bitmask based on a value from a table, calling another routine for each bit found.
- `store_parameters`: Stores four input parameters into a specific memory table.
- `copy_memory_block_2`: Copies a block of data from a source pointer to both a global variable and a destination table.
- `accumulate_table_values`: Iterates through a table to perform arithmetic on elements and stores the results in two separate global variables.
- `search_and_modify_table`: Searches a table for a value matching a specific condition and modifies table entries if a match is found.
- `remove_table_entry`: Searches a table for a specific value and removes that entry by shifting subsequent elements backward.
- `initialize_data_structures`: Clears a large table of values and initializes several global variables and memory locations with specific constants.
- `access_lookup_table`: Repeatedly performs lookups in a table using values derived from arithmetic and other routine returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_then_spin_when_flagged`: When a RAM word is nonzero, calls one unnamed routine, spins in a loop whose exit test can never succeed, and then calls another unnamed routine.
- `copy_table_and_reset_state`: Clears a RAM word, copies 128 longwords from a fixed table into a buffer pointed to by a global, sets a global control value to fifteen and returns zero.
- `load_sound_module_header`: Copies a 128-longword header block from the given address into a fixed table.
- `init_module_playback_state`: Rebases pointers in a module block by the load address, clears four 20-word state blocks and sets up pattern pointers.
- `initialize_dma_table_and_state`: Clears a global variable, copies 128 longwords from a fixed address into a table pointed to by a global, sets another global to a constant and returns zero.
- `relay_two_unnamed_calls`: Calls two unnamed routines in sequence and returns.
- `advance_music_tick_voices`: Counts down a tick counter and, on each expiry, walks the four voice records and applies each record's effect command to its volume and pitch fields.
- `process_voice_effect_commands`: Iterates through a command list to update voice parameters such as pitch, volume, and envelope values.
- `update_voice_effect_parameters`: Processes music voice effect commands by modifying pitch, volume, or envelope values within a voice structure.
- `run_voice_effect_loop`: Runs a per-tick loop over the voice records that applies slide, volume and pitch commands and advances each voice's counters, returning once every voice is done.
- `enter_voice_effect_loop`: Copies a pitch field from a voice record into a second field, then enters the same per-voice effect loop as its neighbouring routine.
- `process_voice_effect_commands_2`: Iterates through a sequence of effect commands to modify voice parameters such as pitch, volume, and envelope values.
- `read_next_voice_events`: Walks the four voice records and, for each whose countdown has reached zero, reads the next event from its stream and stores longwords from a table into the record.
- `update_voice_outputs_per_frame`: Clears a pending flag, writes a saved DMA word and waits on a loop tied to the beam position before setting an interrupt enable pattern.
- `process_voices_and_set_dma`: Clears a byte in a voice record and enters the per-voice loop, which copies each record's pointer and length into its outputs.
- `parse_voice_event_stream`: Iterates through a sequence of event data structures to update audio parameters and sample pointers.
- `parse_voice_event_stream_2`: Iterates through a sequence of voice event descriptors, updating playback pointers and volume or pitch parameters based on command codes.
- `set_voice_trigger_then_process`: Stores a trigger byte into two fields of a voice block, with a special case for the value one, and then enters the same per-voice processing loop that writes the combined DMA enable bits.
- `update_voice_effect_parameters_2`: Processes music voice effect commands by modifying pitch, volume, or envelope values within a voice structure.
- `update_audio_channel_and_minimums`: Decrements a counter and, when it goes negative, resets it and calls one unnamed routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `smooth_and_update_entities`: Iteratively approaches target coordinates for a list of data structures using bitwise shifts for smooth movement.
- `execute_conditional_callbacks`: Checks a return value from a function and executes a sequence of several other routines if the value is non-negative.
- `interpolate_color_tables`: Iterates through two tables and incrementally adjusts the color components of the first table to match the second table.
- `copy_and_clear_table`: Copies a block of data from one location to another and then fills a subsequent section of a table with zeros.
- `copy_and_fill_table`: Copies a block of data from one location to another and then fills a subsequent section of a table with a specific value before enabling DMA.
- `calculate_bitmask_value`: Performs a calculation involving bitwise shifts and a table lookup to return a 16-bit value.
- `update_system_state`: Writes a value to a memory location and waits for a specific condition in a loop before calling two other routines.
- `update_callback_index`: Checks a global variable and either increments it, decrements it, or calls a function pointer from a table if the value is less than one.
- `wait_for_value`: Enters a loop that continues as long as a value at a specific memory location is less than or equal to the provided argument.
- `allocate_and_process_string`: Allocates memory for a buffer, then enters a loop to find the length of a null-terminated string before freeing the allocated memory.
- `process_coordinate_list`: Iterates through a string to calculate a size, populates a list of data structures with calculated values, and performs several routine calls.
- `increment_list_coordinates`: Iterates through a list of data structures and increments specific coordinate values if they are below a certain threshold.
- `smooth_list_coordinates`: Iterates through a list of data structures and moves coordinate values toward target values using bitwise shifts.
- `execute_recursive_call`: The routine performs a single call to itself before returning.
- `call_indirect_function_3`: Calls a function pointer using an argument retrieved from a memory location.
- `search_string_for_marker`: Iterates through a string, calling a routine on each pointer, until a specific character is encountered.
- `execute_indirect_call`: Executes a function call through a pointer stored in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_status_word`: Clears one status word at a fixed offset from a base pointer and returns.
- `read_joystick_and_mouse_ports`: Reads the mouse and joystick movement registers and the fire-button register, converts their bits into direction and button states through lookup tables, and accumulates movement values clamped to a fixed ceiling.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_conditional_value`: Stores a value from a parameter into a global variable if the current value is not five or if a specific condition is met.
- `reset_state_variables`: Clears several memory locations and registers, then conditionally calls a routine and updates a global variable.
- `search_and_clear_table`: Searches a table for a matching value and, if found, sets that entry to zero and searches for the next non-zero entry in the table.
- `execute_callback_list`: Iterates through a list of function pointers, executing each non-null routine, and then sets a global variable to a specific value.
- `write_value_and_trap`: Stores a constant value into a memory location pointed to by an offset from a register and then executes a software interrupt.
- `wait_for_flag_clear`: Sets a value at a memory location to one and enters a loop that waits until that same location becomes zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `reconstruct_pixel_palette`: Reconstructs pixel data from a source buffer and copies a color palette into a global table.
- `reconstruct_pixel_data_2`: A routine that calls functions to parse metadata, allocate memory, and reconstruct pixel data from a source buffer.
- `copy_and_execute_callback`: Copies two words from one location to another, executes a function via a pointer, and performs further indirect calls based on returned values.
- `parse_string_metadata`: Searches for a pattern in a string and extracts multiple short and long integer values from the resulting offset.
- `reconstruct_pixel_data`: Processes a source buffer by combining nibbles from consecutive bytes to store packed data into a destination buffer.
- `allocate_memory_buffer`: Calculates a size, attempts to allocate memory, and returns a status value based on the success of the allocation.
- `perform_bit_shift_copy`: Copies data from a source to a destination while applying bitwise shifts and logical operations based on values in a table.
- `decompress_bitstream_data`: Expands a compressed input buffer into a destination buffer using a bit-shifting algorithm.
- `search_string_pattern`: Compares a source string against a pattern in a table and returns the offset where the match occurs or a negative value if not found.
- `tokenize_string_into_tokens`: Parses a source string into individual tokens by identifying delimiters such as spaces, newlines, or a specific character sequence.
- `tokenize_and_copy_string`: Parses a source string by identifying delimiters such as spaces, newlines, or specific character sequences, then copies the resulting substrings into a destination buffer.
- `parse_text_tokens_3`: Scans a string for specific character sequences and copies substrings into a destination buffer.
- `parse_text_tokens_2`: Scans a string for specific character sequences like newlines or spaces to extract and copy substrings.
- `extract_text_tokens`: Parses a string by searching for delimiters and copying substrings into a destination buffer.
- `extract_text_token`: Identifies substrings within a string by searching for delimiters like spaces or newlines and copies them.
- `parse_text_substring`: Scans a string for specific character sequences and copies a substring to a new location.
- `parse_text_tokens`: Scans a string for specific character sequences like newlines or spaces and manipulates a pointer to extract or copy substrings.
- `parse_text_string`: Scans a string for specific character sequences and whitespace to manipulate a pointer or return a value.
- `nop_sequence`: This routine is empty and performs no operations.
- `call_indirect_function_4`: Executes a function pointer stored at an offset from a global base address.
- `call_indirect_function_5`: Executes a function pointer stored at a different offset from a global base address.
- `call_routine_per_character`: Calls another routine for every character in a null-terminated string.
- `update_state_parameters_2`: Compares an input value to a stored value and updates several global variables with specific constants and offsets based on the input.
- `update_clamped_coordinates`: Checks if coordinate values are within specific bounds and updates them by adding offsets while ensuring they do not exceed limits.
- `update_clamped_coordinates_2`: Performs boundary checks on coordinate values and resets them to zero if they exceed defined thresholds.
- `iterate_string_pointers`: Iterates through a null-terminated string by calling another routine on each character until the end of the string is reached.
- `return_null_value`: This routine is empty and returns a zero value to the caller.
- `setup_blt_chain_sequence`: Calculates screen coordinates and dimensions to populate a series of Blitter transfer descriptors.
- `apply_bitmask_to_block`: Calculates a destination buffer size and performs bitwise operations on a block of memory, using a bitmask to invert or preserve word values.
- `process_bitmask_block`: Iterates through a memory block and performs bitwise operations using a bitmask to write values to a destination table.
- `update_bitmask_state`: Updates a global variable based on a bit pattern found within a word.
- `process_bitmask_flags`: Iterates through a bitmask to populate a memory location with specific constant values.
- `sync_status_variable`: Compares a value against a global variable and updates that variable if the values are not equal.
- `manage_buffer_and_input`: Initializes a table of values, checks a bit in a hardware register, and performs indirect calls and string formatting.
- `call_indirect_function_6`: Calls a function through a pointer stored at an offset from a global base address.
- `run_repeated_tasks`: Executes a specific routine multiple times within a loop and then performs one final call to another routine.
- `process_bracketed_string`: Copies characters from a source string to a destination buffer if the source starts with an opening bracket and ends with a closing bracket.
- `search_pattern_in_table`: Iterates through a table of short values and compares a string pattern against characters following specific values.
- `execute_conditional_loop`: Updates a memory location and executes a series of routines based on the return values of other routines.
- `call_conditional_function`: Calls a function through a pointer if a specific value is negative and a global pointer is non-zero.
- `execute_indirect_pointer`: Calculates an address by adding a negative offset to a value in a register and then calls the subroutine located at that destination.
- `find_string_in_list`: Searches a linked list of strings for a match and updates several global variables with data from the matching entry.
- `iterate_and_call_function_2`: Traverses a linked list and calls a function for each element using the element's offset as an argument.
- `process_string_data`: Retrieves a pointer from a function call, manages a linked list or table of pointers, and copies string data into a new location.
- `check_bracket_syntax`: Checks if a string starting with an opening bracket contains a matching closing bracket before a null terminator.
- `process_complex_data`: Copies a string to a buffer and performs multiple conditional checks and table searches to update various global variables and status values.
- `update_state_from_string`: Copies a string to a destination and executes a sequence of conditional routines and callbacks to update several memory locations.
- `process_complex_state`: Updates several memory locations and performs conditional logic based on the results of multiple other routines.
- `execute_conditional_callback_2`: Checks if a global pointer is non-zero and a specific value is negative before calling a function through that pointer.
- `execute_conditional_callback_3`: Checks if a specific value is negative and a global pointer is non-zero before calling a function through that pointer.
- `execute_conditional_callback_4`: Checks if a value is negative and a global pointer is non-zero before calling a function through that pointer.
- `execute_conditional_callback`: Checks if a specific value is negative and a global pointer is non-zero before calling a function through that pointer.
- `process_conditional_callbacks`: Checks multiple conditions and global values to execute various function pointers and update a global variable based on bitwise results.
- `copy_data_with_callbacks`: Calls a function through a pointer and copies a sequence of bytes from a source to a destination table.
- `write_table_data`: Clears memory offsets and writes a series of bytes from a source pointer into a table at an offset determined by a global variable.
- `clear_memory_offsets`: Stores zero and all ones into two specific offsets relative to a pointer.
- `update_coordinate_index`: Calculates a new index based on input parameters and updates a global table with values derived from a division operation.
- `shift_and_clear_memory`: Shifts a block of data within a memory range and clears four longwords at a specific offset.
- `zero_memory_block`: Clears four consecutive longwords in a memory block and returns a value.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `format_bracketed_string_buffer`: Checks a source string for specific conditions and wraps its contents in square brackets before copying the result into a destination buffer.
- `match_wildcard_pattern`: Compares a pattern containing a wildcard character against a string, allowing for a limited number of character skips.
- `copy_data_to_table`: Copies a sequence of bytes from a source to a destination table based on a counter and a global index.
- `adjust_global_index`: Increments a global variable by a multiple of a constant value if a specific condition is met.
- `write_bytes_to_table`: Writes a series of bytes from a source pointer into a table at an offset determined by a global variable.
- `update_offset_value`: Calculates an offset based on a value and adds it to a memory location.
- `update_status_with_callback`: Checks a global variable and a hardware register, then calls a function pointer and updates a value in a table.
- `execute_nested_callbacks`: Checks a global variable and a hardware register, then executes a function pointer twice and returns a combined value.
- `update_input_state`: Checks hardware input status and updates a global state and coordinate values based on the result.
- `execute_callback_and_check_flag`: Calls a function pointer from a table and updates a global variable based on a bitwise check.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `search_table_for_value_2`: Iterates through a table to find a matching value and returns a shifted word from a table entry.
- `search_table_for_value`: Iterates through a table of structures to find an entry where a specific word matches a value derived from a table entry or a global variable.
- `insert_table_entry`: Iterates through a table to find an insertion point, shifts existing entries to make room, and copies a new data block into the table.
- `shift_and_copy_data`: Shifts a word left by four bits and copies a block of data from one location to another within a table.
- `copy_and_shift_block`: Copies a block of data from one memory location to another while performing a bitwise left shift on a specific word within the block.
- `return_negative_one`: returns a value representing negative one.
- `validate_and_copy_string`: Checks if a character at a specific position matches a set of allowed characters and copies a value to a table if it does.
- `update_status_values`: Compares a value against a global variable and updates it if they differ.
- `perform_xor_loop`: Executes a nested loop that calls two different function pointers and performs a bitwise XOR operation on a value using a global variable.
- `sum_memory_values`: Calls a function pointer and then calculates the sum of one hundred twenty-eight values stored at a fixed memory address.
- `execute_callback_and_copy`: Calls a function through a pointer and, depending on the return value, performs a data copy or executes another function through a pointer.
- `shift_bit_history`: Shifts three separate two-byte values left by one bit and inserts new bit information derived from XORing two bytes and checking high bits.
- `loop_shift_bit_history`: Repeats a bit-shifting process that updates three two-byte values using bitwise logic for a specified number of iterations.
- `clear_memory_gap`: This routine is empty.
- `shift_memory_block`: Moves a block of data within memory by shifting values from one location to another and clearing a specific word.
- `manage_memory_and_input_state`: Performs block memory transfers in both directions and updates a global variable based on joystick input and specific status flags.
- `set_cia_bit`: Sets the highest bit of a global variable, which corresponds to a hardware register bit.
- `copy_memory_block_3`: Performs a nested loop to copy blocks of data while decrementing a global counter and checking return values.
- `iterate_data_blocks`: Copies data in chunks of eight bytes through nested loops and checks a global variable against a specific value.
- `toggle_global_flag`: Checks if a global variable is zero, and if so, sets it to a specific value before returning.
- `return_from_subroutine_2`: This routine is empty and performs no operations before returning to its caller.
- `manipulate_global_bits`: Performs bitwise operations on a global variable and executes logic based on the resulting bit patterns.
- `update_state_variable`: Increments a global variable and performs a series of bitwise masking and conditional checks.
- `process_joystick_input_and_halt`: Reads input from the CIA hardware registers and updates a global variable based on bitwise transformations.
- `toggle_cia_bits`: Sequentially sets and clears specific bits in a hardware register through a global variable.
- `modify_bit_and_loop`: Clears a bit in a global variable, then sets a bit in the same variable before entering an infinite loop.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `infinite_loop`: Enters an infinite loop that prevents further execution.
- `iterate_with_conditional_logic`: Executes a loop that calls other routines and modifies a global variable based on specific bitwise results and a decrementing counter.
- `scan_and_reconstruct_bitmask`: Scans a memory block for a specific word pattern and, upon finding a sequence of these patterns, performs a bitwise interleaving operation to reconstruct data into a new memory location.
- `xor_memory_block_and_compare`: Computes the XOR sum of a block of words in memory pointed to by a register.
- `compute_bitwise_result`: Performs bitwise operations on two values, one from a register and one from a memory location pointed to by another register, and returns the combined result.
- `poll_hardware_status`: Iteratively checks bits in a value, likely representing hardware status, until a condition is met or a counter expires.
- `update_global_state_via_flags`: Checks bits within a global variable to select a value and performs nested loops that modify the global variable based on bitwise conditions.
- `fill_memory_pattern`: Fills a block of memory with a repeating pattern and performs a loop of calculations.
- `perform_blitter_operations`: Calculates an XOR checksum, performs a blitter DMA transfer, and updates a word using bitwise operations and a flag.
- `xor_checksum_data`: Calculates an XOR sum of a block of memory and returns the result masked by a specific bit pattern.
- `configure_blitter_dma_transfer`: Waits for the blitter to be idle, then configures blitter registers for a DMA transfer by setting source, destination, and size parameters.
- `update_bitmask_data_2`: Manipulates a word by performing bitwise operations and masking based on a flag found in the byte preceding a pointer.
- `clear_flag_byte`: Reads a byte pointed to by a pointer and a flag from the preceding byte.
- `update_bitmask_data`: Manipulates a word by performing bitwise operations and masking based on a flag found in the byte preceding a pointer.
- `search_bitmask_pattern`: Iterates through bits in a word to find a specific pattern, updating a global value based on the bit state and a flag in a nearby byte.
- `configure_blitter_transfer`: Calculates source and destination addresses and dimensions using values from a table and global variables.
- `apply_bitmask_to_vram`: Performs bitwise AND, OR, or XOR operations on specific words within a video memory buffer using a mask.
- `scan_bitmask_flags`: Iterates through bits in a word and stores specific constant values into a memory location based on bitmask matches.
- `scan_bitmask_and_store`: Iterates through bits in a word to find a specific flag and stores a constant into a global variable based on the result.
- `scan_bitmask_for_value`: Loops through bits in a word, checking against flags in a nearby byte to determine which constant to store in a global variable.
- `update_blt_parameters`: Calculates distance and size values based on input coordinates and stores them in several global variables, including parameters for a Blitter operation.
- `clamp_coordinate_values`: Ensures two specific global values do not fall below zero and decrements them if they are greater than another set of global values.
- `return_from_subroutine_3`: The routine is empty and performs no operations before returning.
- `calculate_product_2`: Performs a mathematical calculation involving division and bitwise shifts on two input parameters.
- `setup_dma_transfer`: Validates coordinate and size parameters, stores them in a table, and enables a DMA channel by writing to a hardware register.
- `fill_memory_with_bits`: Iterates through a count specified in a global variable and fills memory with either zeros or all ones based on the state of individual bits in a value.
- `clear_memory_blocks`: Fills multiple memory blocks with either zeros or all ones depending on a flag passed in a parameter.
- `store_parameters_to_table`: Stores three provided parameters into consecutive locations within a table.
- `initialize_lookup_tables`: Calculates a product and populates two different tables with incrementing values and calculated offsets.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `fill_table_with_swapped_words`: Fills a table with values derived from a base address by performing word-level bitwise rotations.
- `configure_display_hardware`: Configures various hardware registers including bitplane modes, display enable, and raster timing, then waits for a specific raster line and fills a table with bit-rotated words.
- `copy_color_data`: Copies a sequence of words from a source pointer to both a global table and another memory location.
- `copy_color_palette`: Copies thirty-two words from a specific memory location to a global table.
- `set_palette_entry`: Writes a word to both a global table and a specific memory location at an offset determined by a parameter.
- `search_and_insert_node`: Searches through a linked list of nodes to find a specific position based on a value and inserts a new node into the list.
- `remove_list_node`: Traverses a linked list to find a node matching a specific value and removes it by updating the surrounding pointers.
- `initialize_memory_buffers`: Writes constant values to a global variable, clears a large memory block using a loop, and sets several memory locations with specific bit patterns.
- `initialize_data_structures_2`: Writes specific constant values to a global variable and updates several memory locations via a pointer.
- `wait_for_value_range`: Enters a busy-wait loop until a global variable satisfies specific numeric conditions.
- `update_video_pointer`: Updates the video hardware pointer register with a value from a memory location.
- `process_table_data`: Enters an infinite loop that repeatedly accesses a table using offsets derived from a global variable and other calculated values.

### Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf bytes 114688-115200 (code never loaded by a reference run)

- `initialize_interrupts_and_run_table`: Enables interrupts via the hardware and executes a series of functions from a table until a counter reaches zero.
- `execute_function_table`: Iterates through a table of function pointers and executes each non-null pointer until a counter reaches zero.

### Goal! (1993)(Virgin)(It)[cr Dynamix][tr En](Disk 1 of 2).adf bytes 121856-123904 (code never loaded by a reference run)

- `sum_table_values`: Calls a function via a pointer and then calculates the sum of a series of values from a table.
