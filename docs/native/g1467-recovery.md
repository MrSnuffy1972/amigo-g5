# Game 1467 (body blows (1993)(team 17)[cr interpol]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01467). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3645824 top-level bytes in 265 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 44 raw-word lines |
| Functions named and described | pass, 146 of 146 named (0 author symbols), 146 of 146 described (8 by rule, 112 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1068 of 1068 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 9216 bytes, 15 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3774 of 3971 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10368 |
| code | 15054 |
| data | 3612216 |
| padding | 506 |
| unused | 7680 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf:bootblock | 1 | 38 | 1 |
| logged disk load from Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf boot-block load | 1 | 185 | 11 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 369 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 46 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2794 | 94 |
| Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf bytes 1024-10240 (code never loaded by a reference run) | 1 | 539 | 15 |

## Cases handled

- body blows (1993)(team 17)[cr interpol](disk 1 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf: custom-format (non-DOS) disk, 1734 non-empty blocks: matched against track-loaded segments
- body blows (1993)(team 17)[cr interpol](disk 2 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 2 of 4).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- body blows (1993)(team 17)[cr interpol](disk 3 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 3 of 4).adf: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- body blows (1993)(team 17)[cr interpol](disk 4 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 4 of 4).adf: custom-format (non-DOS) disk, 1757 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 552 of the 552 PCs the canonical run logged (121382 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- body blows (1993)(team 17)[cr interpol](disk 1 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf: the boot block reads 11264 bytes from disk offset 889856 to an address (found statically); recovered as a unit
- runtime segment an address (play-rs-m1500): 210 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 24 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500): 834 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- body blows (1993)(team 17)[cr interpol](disk 1 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf: bytes 1024-10240 look like code no run loaded: recovered position-independently (load address unknown)
- body blows (1993)(team 17)[cr interpol](disk 1 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf: 2 logged loads cover 20480 bytes of the image
- body blows (1993)(team 17)[cr interpol](disk 1 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf: 1 track-loaded blocks found in recovered runtime segments and classified through them
- body blows (1993)(team 17)[cr interpol](disk 1 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf:blocks 129-1726 (bytes 66048-884224)+748032#unpacked: high-entropy data file (7.69 bits/byte), classified as packed data
- body blows (1993)(team 17)[cr interpol](disk 1 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf:blocks 129-1726 (bytes 66048-884224)+752128#unpacked: high-entropy data file (7.65 bits/byte), classified as packed data
- body blows (1993)(team 17)[cr interpol](disk 1 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf:blocks 129-1726 (bytes 66048-884224)+768512#unpacked: high-entropy data file (7.63 bits/byte), classified as packed data
- body blows (1993)(team 17)[cr interpol](disk 1 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf:blocks 129-1726 (bytes 66048-884224)+783872#unpacked: high-entropy data file (7.69 bits/byte), classified as packed data
- body blows (1993)(team 17)[cr interpol](disk 1 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf: blocks 129-1726 (bytes 66048-884224) hold 139 packed streams (C: Compact, IMP: File Imploder) found by scanning, 88% of the bytes
- body blows (1993)(team 17)[cr interpol](disk 2 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 2 of 4).adf:blocks 2-1759 (bytes 1024-901120)+820224#unpacked: high-entropy data file (7.66 bits/byte), classified as packed data
- body blows (1993)(team 17)[cr interpol](disk 2 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 2 of 4).adf:blocks 2-1759 (bytes 1024-901120)+864768#unpacked: high-entropy data file (7.65 bits/byte), classified as packed data
- body blows (1993)(team 17)[cr interpol](disk 2 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 2 of 4).adf: blocks 2-1759 (bytes 1024-901120) hold 91 packed streams (F: Freeze/Melt 1.x, IMP: File Imploder) found by scanning, 89% of the bytes
- body blows (1993)(team 17)[cr interpol](disk 3 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 3 of 4).adf: blocks 2-1759 (bytes 1024-901120) hold 28 packed streams (C: Compact, F: Freeze/Melt 1.x, IMP: File Imploder) found by scanning, 81% of the bytes
- body blows (1993)(team 17)[cr interpol](disk 4 of 4)/Body Blows (1993)(Team 17)[cr Interpol](Disk 4 of 4).adf: blocks 417-1759 (bytes 213504-901120) hold ASCII text
- 2 packed track region(s), 265728 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_joystick_register` (runtime segment an address (play-rs-m1500)): Calculates a bitmask based on a value from a table and stores the inverted result into a global variable that affects a hardware register. Kind: joystick/input-based check. Pass path: the branch at an address (the bne.b instruction is skipped if the joystick value is an address).
- `set_joystick_register` (runtime segment an address (play-rs-m1500)): Stores a value of all bits set into a global variable used to control a hardware register. Kind: joystick/input hardware check. Pass path: moveq #an address, d0.
- `wait_for_bit_clear` (runtime segment an address (play-rs-m1500)): Enters an infinite loop if a specific bit in a global variable is set. Kind: infinite loop trap. Pass path: beq.w an address.
- `synchronize_joystick_input` (runtime segment an address (play-rs-m1500)): Reads joystick input and adjusts a value through a loop to match the hardware state. Kind: checksum/data integrity trap. Pass path: the loop continues or the code proceeds to the next instruction after the check fails to trigger a jump/error.
- `compute_ciab_bit_mask` (Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf bytes 1024-10240 (code never loaded by a reference run)): Computes a clear mask for one bit selected by an index read from memory and stores it in a global variable. Kind: disk hardware/track check. Pass path: the branch at an address (the bne.b instruction is skipped if the check passes).
- `set_global_to_all_ones` (Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf bytes 1024-10240 (code never loaded by a reference run)): Sets a global variable to all eight bits set and returns. Kind: sabotage path. Pass path: not determined.
- `return_unless_port_bit_set` (Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf bytes 1024-10240 (code never loaded by a reference run)): Returns when bit five of a CIA port register is clear; otherwise it loops endlessly. Kind: disk hardware/track check. Pass path: beq.w an address.
- `read_disk_track_via_dma` (Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf bytes 1024-10240 (code never loaded by a reference run)): Sets up disk DMA with a pointer, a length and a sync word, enables disk transfers through the DMA control register, and calls five unnamed routines. Kind: custom disk format/track check with checksum validation. Pass path: the loop completes and returns via the u5__l0_07AC return path (CONCAT44(in_D0,in_D1)).

## Functions


### Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf:bootblock

- `allocate_memory_and_issue_io`: Boot block entry that uses exec to allocate memory, issue DoIO requests, and forbid or enter supervisor state, and writes the DMA control register.

### logged disk load from Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf boot-block load

- `forward_through_jump_vector`: Calls through a jump-table entry that passes control to a fixed address, and calls one unnamed routine.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `rotate_buffer_pointers_and_read`: Swaps two buffer pointers stored in a state record and reads byte values from the buffers into short-word tables held on the stack.
- `read_bits_from_backward_stream`: Reads bits from a byte stream that is consumed backward, shifting bits out with carry and refilling from the previous byte when empty.
- `read_bit_count_from_backward_stream`: Reads a count of bits from a byte stream consumed backward, using carry shifts and refilling from the previous byte when empty, then calls one unnamed routine.
- `decode_backward_stream_width_five`: Decodes values from a backward-consumed bit stream with a fixed width of five and a count of three, refilling from the previous byte when empty.
- `decode_backward_stream_width_four`: Decodes values from a backward-consumed bit stream with a fixed width of four and a count of two, refilling from the previous byte when empty.
- `decode_backward_stream_width_three`: Decodes values from a backward-consumed bit stream with a fixed width of three and a count of one, refilling from the previous byte when empty.
- `decode_backward_stream_width_two`: Decodes values from a backward-consumed bit stream with a fixed width of two and no extra count, refilling from the previous byte when empty.
- `decode_backward_stream_variable_width`: Decodes values from a backward-consumed bit stream with a width taken from a register, refilling from the previous byte when empty.
- `decode_backward_stream_in_loop`: Decodes values from a backward-consumed bit stream in a loop, refilling from the previous byte when empty, and calls one unnamed routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `wait_for_memory_value`: Enters a loop that polls a specific byte offset within a memory block until it matches a specific value.
- `dispatch_function_pointer_2`: Checks a global variable and executes a routine from a table if a specific condition is met.
- `increment_global_counter`: Increments a global variable and calls another routine if the value wraps to zero after meeting specific conditions.
- `populate_data_table`: Calculates a series of values and stores them into a table, then copies a block of data from one memory location to another.
- `populate_lookup_tables`: Calculates base offsets using arithmetic on a global value and populates several memory tables with derived data.
- `initialize_lookup_tables`: Performs arithmetic on a global value to calculate offsets and populates two separate tables with derived data.
- `update_interrupt_state`: Modifies a global value and stores the result into two specific memory locations.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `dispatch_function_pointer`: Compares a global variable against a value in a table and executes a routine if a match is found.
- `transform_bit_pattern`: Iterates through a source buffer and performs bitwise shifts to transform and store a new pattern into a destination buffer.
- `merge_memory_blocks`: Reads data from a source table in chunks and performs bitwise OR operations to combine them into a destination table.
- `scale_integer_table`: Fills a table with values calculated by multiplying an incrementing index by a constant factor.
- `fill_interleaved_table`: Populates a table with two different scaled values in an interleaved pattern using a loop.
- `initialize_pattern_table`: Fills a table with repeating sequences of decreasing values in blocks of eight words.
- `initialize_memory_buffers`: Clears a memory block with zeros and then merges multiple data chunks from a source table into a destination table using bitwise OR operations.
- `initialize_system_tables`: performs several table initializations including clearing memory, scaling integer values, and copying data blocks.
- `clear_memory_blocks`: Clears several large, contiguous blocks of memory by writing zeros to them using multiple loops.
- `clear_memory_block`: Fills a specific block of memory with zeros using a loop.
- `shift_left_seven`: Returns the input value shifted left by seven bits.
- `shift_left_register`: takes the value in a data register and shifts it left by seven bits before returning it.
- `shift_register_left`: Takes a value from a data register and shifts it left by seven bits before returning it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_ori_rts`: Small routine of 2 instructions that merges 3 into d6, then returns.
- `clear_memory_blocks_2`: Compares two values at a specific offset and, if they differ, fills two separate memory blocks with zeros using a loop.
- `copy_data_to_offset_table`: calculates a destination address based on two input values and copies a block of sixteen words to that location.
- `write_two_value_pairs`: Writes two fixed value pairs into shared variables and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `calculate_offset_value`: Performs arithmetic operations involving a global variable and returns a bitwise AND of the result with an input value.
- `interpolate_data_structures`: iteratively updates a destination table with values derived from a source table and a calculated index.
- `interpolate_table_values_3`: Iteratively processes a source table to calculate and store new values into a destination table, applying a bitwise transformation to certain elements based on a global variable.
- `zero_memory_range`: Fills a range of memory with zeros and then calls another routine, returning a boolean based on a global variable.
- `copy_data_block`: Copies a block of sixteen words from a source location identified through a pointer into a destination memory range.
- `copy_source_to_destination`: copies sixty-two words from a source table to a destination table.
- `interpolate_table_values`: Iteratively adjusts values in a table to match a target table by incrementing or decrementing nibbles and bytes.
- `interpolate_table_values_2`: Iteratively adjusts values in a table to match a target table until a condition is met or a limit is reached.
- `tiny_ori_2`: Small routine of 1 instruction that merges 3 into d1.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 252 into d0, then merges 0 into d0.
- `execute_indirect_jump`: An empty routine that performs an indirect jump to an address stored in a jump table.
- `execute_indirect_call`: performs an indirect jump to an address stored in a jump table.
- `process_input_and_decompress_stream`: Updates a global state table based on hardware input and performs bitstream decompression of a data block into a destination buffer.
- `perform_recursive_calls`: Executes two consecutive calls to itself before returning.
- `update_state_from_input`: monitors a global variable and uses input from a hardware register to update a table with new values.
- `copy_data_blocks`: Copies two separate blocks of data from source locations to a global destination.
- `initialize_data_block`: Clears a global variable and then copies a block of data from a fixed source to that location.
- `copy_on_bit_test_2`: Iteratively copies bytes from a source to a destination only when a corresponding bit in a bitmask is set.
- `copy_on_bit_test`: iteratively copies bytes from a source to a destination only when a specific bit is set in a bitmask.
- `decompress_and_update_palette`: Locates a tagged data block, expands a compressed stream using run-length encoding, and writes the resulting bytes into a color palette by shifting and masking values into the hardware color registers.
- `expand_and_pack_data_stream`: Locates a tagged data block by searching for a specific magic identifier, expands the compressed stream by repeating bytes according to a count, and then packs the resulting bytes into a bit-shifted format.
- `expand_and_transform_data_2`: Expands a data stream by repeating bytes according to a count and then performs bitwise transformations on the resulting data.
- `expand_and_transform_data`: expands data by repeating bytes based on a count and then transforms the result into a new table using bitwise operations.
- `transform_byte_data`: Reads data from a global buffer in three-byte increments and performs bitwise shifts and concatenations to store the results into a new table.
- `decompress_atnt_stream`: Checks for a specific magic signature and performs a bitstream decompression of a data block into a destination buffer.
- `dispatch_callback_pointer`: Checks a global variable and a pointer; if the pointer is not one, it increments the pointer by three and calls the function at the new address, otherwise it calls the function at the pointer and increments it by three if the result is non-zero.
- `set_global_values`: Calls another routine and then writes two specific constant values to global variables.
- `process_indexed_state`: Updates a global variable based on a state value and executes a routine from a table using an index derived from a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `execute_indirect_function_pointer`: Calculates an offset into a table of function pointers using a value from a structure and executes the resulting routine.
- `call_indexed_function`: Executes a function via a pointer calculated by adding a scaled index from a table to a global base address.
- `dispatch_indexed_functions`: uses a global value to calculate offsets and executes several routines via a table of function pointers.
- `execute_indexed_function_pointers`: Checks a global variable and, if it does not equal three, writes two values to specific memory locations and calls another routine multiple times before updating two global variables.
- `search_and_update_table`: Iterates through a table to find a matching integer value and, if a match is found after several attempts, updates a structure with specific values and calls another routine.
- `set_control_state`: Stores a specific constant into a global variable and sets another global variable to one.
- `clear_and_initialize_table`: Clears a large memory block with zeros and then populates a table by performing multiple indirect calls to fetch data from various memory locations.
- `execute_table_commands`: Calculates an offset into a table and performs multiple indirect calls to other routines using data extracted from the table.
- `update_sprite_attribute_table`: Iterates through a sequence of data structures to fetch pointers from a table and update specific global attributes.
- `sort_and_initialize_table`: Sorts a list of short integers in descending order using an insertion-style algorithm and then iterates through a table to initialize global function pointers and data values based on specific memory conditions.
- `copy_memory_until_sentinel`: Copies data from a source pointer to a destination pointer until a sentinel value of negative one is encountered.
- `copy_memory_block`: Performs a memory copy operation using a loop, updating several global variables to track progress and source/destination pointers.
- `copy_memory_block_2`: Copies data from one memory location to another while managing various state variables and loop counters.
- `copy_memory_block_3`: Copies a block of data from a source address to a destination address using a decrementing counter.
- `copy_memory_blocks_with_loop`: Copies data from a source address to a destination address using a nested loop structure.
- `validate_and_copy_data_blocks`: Validates data block headers against specific magic constants and lengths before performing a nested loop to copy memory contents into a destination structure.
- `process_os_data_block`: Checks for a specific magic constant and length, then performs a nested loop to copy data blocks into a structure.
- `process_atn_data_block`: Validates a specific header signature and then executes a nested loop to transfer memory blocks into a structure.
- `process_atn_variant_block`: Verifies a unique header value and proceeds to copy data using a nested loop structure into a destination structure.
- `process_atn_custom_block`: Identifies a specific data signature and performs a multi-stage memory copy operation into a structure.
- `copy_data_with_loop`: Writes several values to a structure and then performs a nested loop to copy data from one memory location to another based on a calculated count.
- `copy_data_with_mask`: Initializes a structure with values including a masked input and then executes a loop to transfer data between memory locations.
- `update_joystick_register`: Calculates a bitmask based on a value from a table and stores the inverted result into a global variable that affects a hardware register.
- `set_joystick_register`: Stores a value of all bits set into a global variable used to control a hardware register.
- `wait_for_bit_clear`: Enters an infinite loop if a specific bit in a global variable is set.
- `synchronize_joystick_input`: Reads joystick input and adjusts a value through a loop to match the hardware state.
- `load_and_discard_register`: It loads the value at the address held in a register into another register and then returns without using it.
- `modify_and_wait_flag`: Performs bitwise operations on a byte at a pointer and then enters a loop that waits for a specific offset from that pointer to become non-zero.
- `modify_and_wait_on_memory`: It performs bitwise operations on a byte at a pointer, stores a zero at an offset from that pointer, and enters a loop until that offset contains a non-zero value.
- `adjust_table_values`: It checks a value at an offset within a structure and, depending on its content, subtracts a constant or adds a constant to the value at the start of the structure.
- `check_joystick_state`: Resets a global variable and then performs several conditional checks against other global values to determine if a specific bitwise state is met.
- `process_joystick_input`: It performs bitwise transformations on a global variable to calculate an index, waits for a specific bit to be set, and then updates the global variable with a value from a table.
- `process_second_joystick_input`: It performs bitwise transformations on a global variable to calculate an index, waits for a different specific bit to be set, and then updates the global variable with a value from a table.
- `call_indirect_via_table`: It uses a value from a structure to index into a table of pointers and calls the resulting subroutine.
- `reset_audio_and_memory`: Sets several global variables and memory locations to zero and loads a value from a memory offset.
- `update_audio_playback_state`: Updates audio channel parameters and sample playback structures by modifying hardware registers and internal memory blocks.
- `update_audio_channel_parameters`: Updates audio hardware registers and internal sample playback structures based on command nibbles.
- `update_audio_sample_parameters`: Updates a structure with sample data, pitch, and duration values retrieved from a lookup table based on a command nibble.
- `loop_until_zero`: Repeatedly calls another routine in a loop until a value returned by that routine is zero.
- `calculate_offset_value_2`: It performs arithmetic on bytes from a structure and stores the result at an offset within another structure if certain conditions are met.
- `clear_memory_block_2`: It iterates through a structure and sets a specific byte at a fixed offset to zero for a set number of iterations.
- `update_parameter_via_nibble_command`: Reads a nibble from a structure to select one of several arithmetic or lookup operations.
- `copy_memory_word`: Copies a word from one memory location to another using offsets from two pointers.
- `search_and_update_table_value`: Iterates through a table to find an entry where a value is greater than or equal to a threshold, then updates a structure member with the retrieved value.
- `update_scaled_value`: Modifies a value in a structure by adding or subtracting a byte based on a flag, then performs a lookup in a table to update another value.
- `calculate_scaled_value`: Reads bytes from a structure to calculate a scaled value, which is then added to a value at an offset and stored in another memory location.
- `process_input_command_nibble`: Reads a nibble from a structure to determine a command type and performs specific arithmetic or bitwise operations on associated data.
- `mask_byte_value`: Extracts the lower nibble from a byte at a specific offset within a structure and stores it in another offset within the same structure.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `initialize_and_set_pointer`: It clears a block of memory, updates a global variable with a pointer, and sets a global variable to a constant value.
- `tiny_subi`: Small routine of 1 instruction that subtracts 0 from d3.
- `clear_memory_block_3`: Initializes a memory region by zeroing out a sequence of words.
- `clear_memory_block_4`: Clears a block of memory by writing zero to a series of consecutive words.
- `clear_memory_block_5`: Clears a block of memory by writing zero to a sequence of words.
- `tiny_andi_move`: Small routine of 2 instructions that masks -(a0) with 0, then copies d0 to d0.
- `clear_memory_block_6`: Fills a block of memory with zeros using a loop and then updates a global variable with a specific value.
- `configure_hardware_registers`: Writes specific values to audio volume, DMA control, and interrupt enable registers.
- `execute_empty_calls`: This routine performs two recursive calls to itself before returning.
- `rotate_audio_sample_buffer`: Manages a circular buffer by decrementing a counter and shifting blocks of data to new memory locations.
- `update_audio_parameter_small`: Updates a global variable by adding a constant and then sets several global values to specific constants.
- `copy_memory_block_4`: Copies a block of data from a source address to a destination address using a loop-unrolled sequence of word transfers.
- `rotate_audio_sample_buffer_2`: Shifts a circular buffer of audio data by decrementing a counter and performing a block memory copy to wrap the data.
- `update_audio_parameter_large`: Updates a global variable by adding a constant and then sets several global values to larger specific constants.
- `copy_memory_block_5`: Copies a block of data from a source address to a destination address using a loop-unrolled method.
- `increment_and_mask_global_state`: Increments a global variable and applies a bitwise AND mask to clear the most significant bit.

### Body Blows (1993)(Team 17)[cr Interpol](Disk 1 of 4).adf bytes 1024-10240 (code never loaded by a reference run)

- `write_dma_and_interrupt_control_raw`: Raw code region of 19 instructions that writes the DMA and interrupt enable registers and calls one unnamed routine.
- `call_routines_then_jump_pointer`: Calls unnamed routines repeatedly, sets a global value in between, and then transfers control through a function pointer.
- `add_base_to_longword_run`: Adds a base value to a run of 34 longwords starting at a global address, and stores the base plus a fixed offset into a function-pointer global.
- `copy_longwords_backward_to_fixed_address`: Copies a run of 60 longwords from a global address into memory just below a fixed address, moving backward, and touches the DMA control and vertical position registers.
- `check_signature_and_unpack_block`: Checks a four-character signature at its input, then processes a packed block.
- `store_derived_state_and_call`: Stores several values derived from its inputs into global state variables, then calls three unnamed routines.
- `compute_ciab_bit_mask`: Computes a clear mask for one bit selected by an index read from memory and stores it in a global variable.
- `set_global_to_all_ones`: Sets a global variable to all eight bits set and returns.
- `return_unless_port_bit_set`: Returns when bit five of a CIA port register is clear; otherwise it loops endlessly.
- `read_disk_track_via_dma`: Sets up disk DMA with a pointer, a length and a sync word, enables disk transfers through the DMA control register, and calls five unnamed routines.
- `stub_return_only`: Routine whose decompiled body is a bare return, so no behaviour is recovered.
- `set_status_and_wait_for_flag`: Sets bit one and then sets bit zero of a status byte at an address register, clears a flag at a fixed offset, and waits until that flag becomes nonzero.
- `clear_status_and_wait_for_flag`: Clears bit one and sets bit zero of a status byte at an address register, clears a flag at a fixed offset, and waits until that flag becomes nonzero.
- `process_hunk_relocations`: Parses a data structure identified by a specific hunk type code to calculate relocation offsets and execute subsequent code segments.
- `load_hunk_sections_then_call`: Compares values with AmigaDOS hunk type codes, which points to executable loading, and walks the hunk structure to place sections.
