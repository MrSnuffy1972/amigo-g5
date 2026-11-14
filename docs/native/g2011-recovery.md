# Game 2011 (populous (1989)(electronic arts)[cr unicorn]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02011). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901312 top-level bytes in 40 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 259 of 259 named (0 author symbols), 259 of 259 described (47 by rule, 44 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1339 of 1339 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5754 of 5754 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 625444 |
| code | 19238 |
| data | 243384 |
| padding | 10686 |
| unused | 2560 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Populous (1989)(Electronic Arts)[cr Unicorn].adf:more | 2 | 206 | 7 |
| Disk-Validator | 1 | 386 | 1 |
| Populous (1989)(Electronic Arts)[cr Unicorn].adf:Populous | 3 | 3625 | 132 |
| UNICORN | 2 | 24 | 4 |
| Populous (1989)(Electronic Arts)[cr Unicorn].adf:populous.prg | 7 | 78 | 1 |
| icon.library | 10 | 1421 | 110 |
| Populous (1989)(Electronic Arts)[cr Unicorn].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- populous (1989)(electronic arts)[cr unicorn]/Populous (1989)(Electronic Arts)[cr Unicorn].adf:more: PowerPacker 2.3 (xfd) stream found by ancient scan (5096 -> 8640 bytes) but only 0 of 5 sampled windows are in reference RAM: not used
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 jump target
- populous (1989)(electronic arts)[cr unicorn]/Populous (1989)(Electronic Arts)[cr Unicorn].adf:more: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- `disable_audio_channels_dma` (Populous): Clears a global status byte and writes masks to the audio control and DMA control registers, which clears the audio DMA enable bits. Kind: disk hardware/ADKCON check. Pass path: sub_0_2DD8 (the routine itself).

## Functions


### Populous (1989)(Electronic Arts)[cr Unicorn].adf:more

- `load_executable_hunks`: Loads an AmigaDOS executable's hunks into allocated memory, copying code and data hunks and reserving space for BSS hunks.
- `stub_empty_routine`: Contains no instructions beyond returning, so it is an empty stub.
- `delay_by_counting_down`: Spins in a countdown loop over a register value for the requested count, with no other effects.
- `unpack_backward_compressed_stream`: Expands a packed byte stream from the end of its destination towards the start, writing literal bytes and back-references whose lengths are sums of read codes.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_with_space_wrap`: Decrements a byte register once for each unit of a counter, replacing a zero result with a space character.
- `stub_bad_instruction_data`: Contains only undecodable instruction data, so it is a stub with no usable behaviour.

### Disk-Validator

- `scan_bitmask_transitions`: Walks tables of bit-mask words, comparing each bit with the previous one and calling through a function table when the value changes, while counting the changes in a global.

### Populous (1989)(Electronic Arts)[cr Unicorn].adf:Populous

- `zero_data_block_and_setup`: Clears a block of longwords in the data area and records the global base pointer.
- `allocate_demo_picture_buffers`: Allocates four memory blocks of fixed sizes and stores their addresses, then runs a sequence of calls and branches on a parameter to choose between two call sequences.
- `start_copper_list_2`: The routine points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1, then returns.
- `register_buffers_with_sizes`: Passes four stored buffer addresses with their sizes to three called routines, then stores a fixed value in a global.
- `create_process_from_startup_record`: Allocates a 40-byte structure, sets its name and size fields from the startup text reference and zeroes its other fields, then hands it to a process-creation routine.
- `allocate_and_index_buffer_table`: Allocates a 1152-byte block with a memory-type flag and fills an eight-entry table of addresses spaced 144 bytes apart within it.
- `store_fixed_count_and_call`: Stores the constant 399 in a word slot of the frame and then calls another routine.
- `process_stored_buffer_block`: Passes the buffer allocated earlier and its size of 1152 bytes to another routine.
- `call_with_two_stored_values`: Calls another routine twice, first with a stored pointer and then with a second stored value.
- `call_library_vector_with_word`: Passes a stored value and the low word of its argument to a library stub, which loads a library base and jumps through its vector.
- `call_single_routine`: Calls another routine with no arguments and returns.
- `find_entry_and_dispatch`: Searches a two-entry table for the given value and, on a match, calls another routine with the paired entry.
- `swap_pair_and_call`: Swaps two stored four-byte values in the frame and then calls another routine with the swapped pair.
- `open_system_libraries`: Obtains a base address and then opens the graphics and intuition libraries by name, storing each returned base and calling a routine when one fails.
- `wait_for_selection_then_dispatch`: Calls a routine repeatedly until it returns one, two or three, then calls another routine with a stored value and a frame address.
- `wait_then_classify_stored_values`: Starts a sequence using the text reference load.pic, polls for a flag to clear for a bounded number of iterations, then classifies two stored coordinate values into a code from zero to three.
- `install_handlers_and_run_loop`: Installs two routine pointers and sets initial values for a table of sixteen 14-byte entries.
- `step_byte_coded_event_stream`: Steps through a byte-coded event stream for a set of channels, treating small codes as note updates, middle codes as table updates and the top codes as pattern-control commands, and calling routines to output each event.
- `stub_empty_return`: Contains only a return, so it is an empty stub.
- `setup_entries_and_run_loop`: Initialises eighteen table entries with a fixed value and a three-bit value from a called routine, then loops calling routines to output each entry's offset until a stop flag is set.
- `initialise_entry_pairs_from_routine`: For each entry in the given count, stores a value in the range 232 to 244 and a value from zero to eight, both taken as remainders of results from a called routine.
- `update_entry_pairs_and_return_first`: Updates each entry pair by subtracting its second value from its first after adding one, clamping negative results to zero and halving the step in that case.
- `run_demo_picture_loop`: Sets up state and calls routines while the loop condition holds, checking a stop flag on each pass, then calls a library routine through a stub to finish.
- `compute_plane_buffer_addresses`: Computes four pairs of buffer addresses from stored bases, stepping by 300 and 32000 bytes, and passes each pair with two frame pointers to a called routine.
- `load_32000_bytes_from_file`: Opens the named file for reading, reads 32000 bytes into a stored buffer, passes the buffer on to another routine and closes the file.
- `load_asset_files_into_buffers`: Opens and reads several data files named in its text references, font.dat, land0 and sprites0.dat, into fixed-size buffers, closing each file after its reads.
- `conditional_load_with_params`: The routine calls another routine with a parameter and a constant, and if the result is non-zero it calls another routine with the result and the two parameters, then calls another routine with the result and a global variable, and finally calls another routine with the result.
- `blit_bitplanes_from_source_table`: Draws four bitplanes into the screen buffer at a given position, shifting each 16-bit source word by the low bits of the horizontal coordinate and combining it with masked destination words.
- `blit_sprite_from_fixed_source`: Draws a masked, shifted sprite from a fixed source table into the screen buffer, combining four bitplanes word by word at a given position.
- `call_routine_return_pair`: Calls another routine and returns the register values it leaves as a pair.
- `draw_masked_plane_blocks`: Draws a block of bitplane words into the screen buffer, combining each source word with an AND mask and an OR value for every plane in turn.
- `rotate_entry_table_fields`: Rotates the fields of each of 1680 twenty-byte entries in a global table by one slot, moving the high and low words of the stored long values into neighbouring positions.
- `shift_buffer_bytes_left_three`: Shifts each byte of a fixed 100-byte buffer left by three bits, discarding the overflow.
- `draw_ten_by_ten_grid`: Loops over a ten-by-ten grid of entries, computing a position for each cell from its row, column and a stored offset, and passes it to a called routine.
- `concatenate_registers`: The routine calls another routine and returns a 32-bit value formed by concatenating the contents of two data registers.
- `set_text_pointer_to_global`: Stores the address of a global buffer in a pointer variable.
- `render_text_into_five_planes`: First fills 80 longwords with minus one and 320 longwords with zero, then renders the string held in a pointer variable by copying each character's eight-row glyph from a font table into five bitplane buffers.
- `initialize_system_library_interface`: Attempts to open a library via a vector table and, upon success, initializes several global pointers and sets multiple global variables to a specific constant value.
- `open_library_via_vector_table_2`: Looks up a library through a vector table and, when it is found, calls its setup vectors and stores several global values before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_status_byte_and_return`: Clears one status byte at a fixed offset in a structure and returns the value it was given.
- `restore_interrupt_registers`: Writes the mask that clears all interrupt enable bits, calls a library routine to free memory, then restores the saved register values.
- `dispatch_status_code_and_clear`: Calls a routine for a status code of ten or one, then clears the status byte at a fixed offset in the structure before returning.
- `clear_status_after_code_one`: Calls a routine when the code is one, then clears the status byte at a fixed offset in the structure before returning.
- `reset_structure_status_byte`: Clears a single byte at a fixed offset within a data structure pointed to by a register.
- `clear_structure_status_byte`: Sets a single byte to zero at a fixed offset within a data structure pointed to by a register.
- `clear_status_byte_2`: The routine clears a byte at a fixed offset within a structure and returns the value it was given.
- `clear_status_byte`: The routine clears a byte at a fixed offset in a structure and returns the value it was given.
- `poll_interrupt_flag_bit`: Checks one flag bit in a global word and, when it is set, writes a status byte, updates the word and calls another routine.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `init_interrupts`: Initializes interrupt handling by clearing a control register, setting an interrupt control byte, writing a routine address to a global location, enabling specific interrupts, and setting a byte in memory to ff.
- `handle_input_and_interrupts`: Reads interrupt status from hardware registers, clears specific interrupt bits, reads input data from memory, checks for certain bit conditions, and branches to other routines based on those conditions before returning.
- `update_flags_and_shift`: Stores a byte from a memory location, sets a flag bit, updates multiple memory locations with specific values, performs a bitwise rotation on the stored byte, and writes the result to another memory location.
- `build_dispatch_tables`: The routine builds dispatch tables by reading values from a global variable area and writing computed pointers and constants into memory, then initializes additional tables and global variables.
- `disable_audio_channels_dma`: Clears a global status byte and writes masks to the audio control and DMA control registers, which clears the audio DMA enable bits.
- `update_audio_channel_registers`: Returns early when a global flag byte is clear.
- `blend_pattern_into_bitplanes`: Copies source words into four bitplanes over 64 rows, taking each bit from the source where the pattern bit is clear and keeping the destination bit where it is set.
- `merge_pattern_row_into_planes`: Merges a row of 160 source words with a pattern from a table into four bitplanes, cycling through three pattern words.
- `store_stack_word_and_call`: Stores a word passed on the stack into a global variable and then calls another routine.
- `call_other_routine`: The routine calls another routine and then returns.
- `update_global_pseudorandom`: Updates a global variable with a linear congruential calculation and masks the result.
- `update_position_from_input_bytes`: Updates a horizontal position from the change in the low byte of an input word, wrapping the difference and clamping the result to zero through 639 before halving it.
- `save_input_bytes_and_call`: Saves the low byte and high byte of a global input word into two saved globals, then calls another routine.
- `save_interrupt_state_and_enable`: Saves the current interrupt enable word in two saved globals, then writes a set-bits mask to the interrupt enable register.
- `restore_saved_interrupt_enable`: If a saved interrupt enable value is non-zero, it clears all enable bits, writes back the saved value with its top two bits set, clears the saved value and returns it.
- `build_copper_palette_list`: Builds a copper list in memory by writing display window, data fetch and bitplane control values, choosing one of two window positions from a global.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `write_request_bit_and_wait`: Writes its argument and then a value four higher to a global register, then loops until the request bit is set.
- `wait_for_request_bit_set`: Writes the request bit value to a global register and polls the same register until that bit reads back as set, then returns that bit.
- `append_values_to_copper_list`: Finds the first colour register entry in a copper list and writes the values from a source array into the following entries, two words apart.
- `unknown_routine_no_pseudocode`: Only its call relationship is known: one unnamed routine calls it and it runs 21 instructions.
- `process_table_entry`: Loads data from a table using an index, processes it through several arithmetic operations, and stores results into two different structures based on a conditional check.
- `initialize_struct_from_args`: Clears two words in a structure pointed to by a1, then copies the high word of an argument and the low word of the same argument into four locations of a structure pointed to by a0.
- `write_sprite_control_block`: Looks up a control block from a table indexed by an argument and writes its start positions and flag bits into the block and its duplicate fields.
- `decode_raw_keyboard_code`: Reads a raw keyboard code from the keyboard serial register, ignores key releases and converts the code to a character through a key table.
- `copy_32000_byte_block`: Copies 8000 longwords, which is 32000 bytes, from one buffer to another.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `step_palette_components_toward_zero`: Copies a palette of entries into a local buffer, then repeatedly decrements each 4-bit colour component toward zero until every component is zero, calling routines on each step.
- `call_then_forward_result`: Calls one routine with no arguments, then passes its two arguments through to another routine and returns that routine's result.
- `call_and_conditional_followup`: Passes its argument to another routine and then calls a second routine when a stored value is non-zero.
- `advance_seed_return_high_word`: Adds 12345 to the value returned by a called routine, stores the sum, and returns its high word with the top bit cleared.
- `store_high_word_to_global`: The routine stores the high word of its parameter into a global variable and returns.
- `stub_empty_return_routine`: Contains only a return and does nothing.
- `allocate_tagged_record_block`: Allocates a record sized from a table entry, initialises its fields and writes a four-character tag into its header.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `copy_named_string_to_record`: Copies a null-terminated string from a global table entry into a newly allocated buffer, storing its length and pointer in the record.
- `append_string_bounded`: Appends a null-terminated source string to a destination string, copying no more than 32767 characters and terminating the result when the copy is cut short.
- `append_string_with_limit`: Appends a null-terminated source string to a destination string, stopping after the count in the third argument and adding a terminator when the copy is cut short.
- `copy_string_padded_to_length`: Copies up to a given number of characters from a source string into a destination, then pads the rest of the destination with zero bytes up to that length.
- `multiply_32bit_values`: Multiplies two 32-bit values from the register inputs using 16-bit partial products, returning the combined product.
- `open_window_and_icon_resources`: Opens icon.library and obtains a window-related entry from it, storing the returned pointers in the record.
- `call_stored_callback_then_routine`: Calls a stored routine pointer when it is set, then calls another routine.
- `check_global_flags_and_cleanup`: The routine checks a global variable for non-zero and calls an indirect function obtained from it, then calls another routine.
- `release_stored_resources`: Releases resources held in the record in turn: it frees each entry of a table, calls a stored callback, and releases any stored handles that are non-null.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_table_entry_handle`: Checks that an index is within the table and that its entry is in use, returning minus one and setting error code 2 otherwise.
- `call_dos_close_vector`: Calls the library vector for closing through the stored library base at a fixed offset, and returns.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_load_seg`: Wrapper that calls the dos library function LoadSeg and returns its result to the caller.
- `call_dos_open_vector`: Calls the library vector for opening through the stored library base at a fixed offset, and returns.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_stored_library_vector`: Calls a library vector through a stored base pointer at a fixed offset, passing no arguments of its own, and returns.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_offset_198`: The routine loads a library base from a global variable and calls a function at a specific offset within that library.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `execute_indirect_library_function`: Loads a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `execute_indirect_function`: Fetches a function pointer from a nested structure within a global variable and executes it.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `execute_indirect_library_function_2`: Loads a function address from a nested pointer structure and executes it.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `execute_indirect_function_2`: Loads a function pointer from a nested offset within a structure pointed to by a global variable and executes it.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_stored_library_vector_2`: Calls a library vector through a stored base pointer at a fixed offset from a different stored base, passing no arguments of its own, and returns.

### UNICORN

- `call_library_close_sequence`: Calls a routine and, when the zero flag is clear, calls a second routine and then a library vector through a stored base.
- `open_dos_library`: Opens dos.library through the exec library's open entry and stores the returned handle in a global variable.
- `call_close_library`: Wrapper that calls the exec library function CloseLibrary and returns its result to the caller.
- `call_library_vector`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.

### Populous (1989)(Electronic Arts)[cr Unicorn].adf:populous.prg

- `launch_program_process_from_startup`: Opens dos.library, looks up the task, and creates a process for populous.prg, exchanging messages with it and releasing memory afterwards.

### icon.library

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `increment_counter_clear_bit`: Library code, part of icon.library: increments a 16-bit counter at a fixed offset from a base pointer and clears one bit in a status byte at another offset.
- `decrement_counter_and_dispatch`: Library code, part of icon.library: decrements a 16-bit counter at a fixed offset and, when it reaches zero with a flag bit set, calls another routine and returns its result.
- `unlink_node_and_release`: Library code, part of icon.library: when a counter field is zero, it unlinks a node from a doubly linked list and calls a library vector to release it.
- `set_flag_bit_return_zero`: Library code, part of icon.library: sets one flag bit in a status byte and returns zero.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `store_library_result_in_record`: Library code, part of icon.library: stores its arguments in a structure, calls a library vector and records the result.
- `copy_record_fields_to_globals`: Library code, part of icon.library: copies two stored fields from a structure into two global variables.
- `call_library_handler`: This library routine simply calls another routine and then returns.
- `forward_library_call_stub`: Library code, part of icon.library: calls one routine with no arguments and returns, doing nothing else.
- `call_icon_init_routine`: This library routine invokes a single initialization function to set up icon data structures and then returns.
- `call_icon_data_copy`: This library routine unconditionally calls a helper routine to copy icon data to the stack and then returns.
- `invoke_handler_pointer`: This library routine retrieves a function pointer from a data structure and calls it.
- `call_handler_via_offset`: This library routine calls another routine using a pointer and a second argument retrieved from a fixed offset, then returns.
- `call_library_helper`: Library code that calls another routine to allocate memory and initialize a structure, then returns.
- `invoke_icon_initialization`: This library routine serves as a wrapper that calls a single initialization function to set up icon data structures and then returns.
- `call_library_handler_2`: This library routine calls another routine and then returns.
- `call_handler_via_pointer`: This library routine retrieves a pointer from a data structure and calls the routine located at that address.
- `call_other_routine_2`: The routine calls another routine and then returns.
- `call_match_segment`: This library routine calls another routine and then returns.
- `call_parse_copy_count_text`: This routine calls another routine and then returns.
- `call_vector_and_store_field`: Calls a library vector with its first argument and stores its second argument at a fixed offset in the structure that the vector returns.
- `obtain_record_and_copy_frame`: Obtains a record from a routine and, when a second call succeeds, copies ten saved longwords and several saved values into the record's fields.
- `copy_frame_into_record_links`: Copies ten saved longwords and several saved values from the frame into a record's fields, then links the record back to its owner when its link field is non-zero.
- `return_zero_stub`: The routine is an empty stub that immediately returns the constant zero.
- `copy_icon_data_to_stack`: Builds a stack header from a fixed signature and version, copies ten longwords and several icon fields from the caller's structure into local slots, then passes the header to another library routine with the first two arguments.
- `load_icon_from_info_file`: Opens the icon's .info file by name, checks its header signature and version, and reads the image and tool-type data into the icon structure, allocating memory for the optional sections.
- `read_icon_header_sections`: Checks the icon header signature and version, then reads the optional image and tool-type sections through the library's read calls, allocating memory for each entry.
- `read_icon_extra_sections`: Reads the remaining optional sections of an icon: a pointer list whose length is counted from the stack, plus a further section read only when its pointer is set.
- `read_icon_pointer_list`: Fills an array of longwords by reading each entry through the library read call and storing it, decrementing a byte count kept on the stack until it reaches four or less.
- `call_then_return_zero`: Calls one library routine, discards its result, and returns zero.
- `empty_routine`: The routine contains four instructions and performs no observable work; it ends immediately with a return.
- `save_icon_with_buffer`: Allocates a temporary buffer, reporting an error and returning zero if that fails.
- `write_icon_info_sections`: Writes the optional image and tool-type sections of an icon to a .info file, measuring each entry before writing it through library calls.
- `write_icon_extra_sections`: Writes the remaining optional sections of an icon, including a length-prefixed block when its pointer is set, a pointer list with each entry's length, and a final block, all through library calls.
- `write_icon_list_tail`: Measures the byte length of a null-terminated pointer list by counting its entries, then writes each entry's pointer through library calls, followed by an optional trailing section.
- `write_icon_table_entries`: Walks a null-terminated table of pointers, writing each entry with its length through library calls, and then writes an optional trailing section.
- `walk_node_list_until_head`: Passes each node of a circular list to a library routine, starting after the header node, until the list returns to its head or runs out.
- `iterate_linked_list_nodes`: Passes the current node to a library routine, obtains the next node through another library call, and stops when that call returns nothing or the header link points back to the start.
- `call_with_offset_pointer`: Passes its first argument and a second argument moved forward by a fixed offset to another routine, then returns.
- `allocate_cleared_record`: Allocates a cleared block of fixed size and stores a pointer to its link field, then passes the record's next field to a routine that fills it.
- `call_routine_then_return`: Calls one library routine with no arguments and returns.
- `return_constant_zero`: Returns the constant zero without calling any routine or touching its arguments.
- `store_pair_in_table_entry`: Decrements a 16-bit count and, when it runs out, asks a library routine to expand the table.
- `decrement_counter_and_store_pair`: Decrements a 16-bit counter through its pointer, then stores two values from registers into consecutive entries of the table referenced by a field of the structure, selected by the new counter.
- `allocate_and_link_record`: Allocates a cleared record and stores a pointer to its second field, then passes the record through two library routines that set it up from the caller's arguments.
- `return_immediately_empty`: Contains only a return, with no work and no calls.
- `call_routine_return_zero`: Calls one library routine and then returns the constant zero, ignoring the routine's result.
- `advance_pointer_and_call`: Passes its first argument and the second argument advanced by a fixed step to another routine, then returns.
- `find_tooltype_value`: Converts a base value through a library routine, then searches a null-terminated list of entries for one whose text is followed by an equals sign, and returns a pointer to the value after that sign.
- `locate_value_after_equals`: Walks a null-terminated list of pointers and checks each entry at a selected position, returning the text after the equals sign once a library routine reports success.
- `find_entry_value_after_equals`: Walks a null-terminated list of pointers, passing each entry to a library routine, and returns the text after the equals sign at the offset of the first entry for which that routine returns zero.
- `match_segment_in_list`: Walks a string of entries separated by a delimiter character, locating each separator through a library call and comparing each segment's length with a reference value.
- `find_matching_segment`: Steps through separator-delimited entries with a library routine, returning one when an entry's length equals the reference value and a second library call confirms it.
- `scan_delimited_segments`: Steps through separator-delimited segments, comparing each segment's length with a reference value and confirming a match through a library routine.
- `scan_segments_for_length`: Walks delimiter-separated segments and returns one as soon as a segment has the expected length and the confirming library routine reports zero.
- `find_segment_with_length`: Iterates over delimiter-separated segments, using library calls to find each separator and to test the segment length.
- `parse_copy_count_text`: Parses a text field that starts with a fixed copy phrase, reads a decimal count from it, and records that count with the text that follows in the structure.
- `parse_prefixed_decimal_field`: Checks a three-character prefix at a fixed position, then reads the decimal digits after it and checks a further four-character tag.
- `parse_decimal_digits_then_tag`: Reads decimal digits from a string into a numeric value, then checks a four-character tag that follows them.
- `parse_decimal_count_suffix`: Accumulates decimal digits from a string into a count, then checks the four characters that follow against a text constant.
- `convert_decimal_digit_char`: Returns the numeric value of a character when it is a decimal digit, and minus one for any other character.
- `return_minus_one`: Returns minus one without doing anything else.
- `call_library_function_and_return`: Calls one function of a library whose base is held in a global variable, passing no arguments, and then returns.
- `call_second_library_function`: Calls a different library function through the same global library base, with no arguments, and then returns.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_264`: Library stub: loads a library base from a global variable and calls the library function at offset minus 264 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_42`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `attempt_and_release_on_failure`: Obtains a value from a routine using the last two arguments, then passes it with the first two arguments to a second routine.
- `allocate_block_with_default_count`: Allocates a cleared block, stores a default count of ten in the block and in the caller's word, and passes the caller's buffer and the new block to a library routine.
- `return_zero_value_only`: Returns the constant zero and nothing else.
- `verify_library_result_or_zero`: Calls a library routine with three arguments and returns the fourth argument when the routine's result matches it, or zero when it does not.
- `tiny_move_rts`: Small routine of 2 instructions that copies d1 to d0, then returns.
- `verify_second_library_result`: Calls a different library function with three arguments and returns the fourth argument when the result matches it, or zero otherwise.
- `exit_without_action`: Returns immediately without any work.
- `divide_32_bit_unsigned`: Divides a 32-bit dividend by a divisor and returns the quotient and remainder as a pair.
- `shift_subtract_divide_loop`: Runs a sixteen-step shift-and-subtract loop that divides the value in the low register by a divisor, leaving the quotient and remainder in registers.
- `multiply_32_bit_values`: Multiplies two 32-bit register values using 16-bit partial products and returns the combined 32-bit result.
- `forward_call_to_helper`: Calls one helper and returns, with no other work.
- `shift_and_divide`: Moves data between registers, calls a 32-bit unsigned division routine, and returns the result in a data register.
- `negate_if_nonzero`: Checks if the result from a division operation is non-zero and negates it before returning.
- `call_and_negate_on_flag`: Calls a helper, then returns its first result, or the negation of that result when the second result left in an address register is nonzero.
- `call_raw_format_function`: Calls the exec text formatting function, RawDoFmt, through the library base held in a global variable, using arguments the caller has already placed in registers.
- `tiny_move_rts_2`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `measure_string_length`: Counts the characters of a null-terminated string using a 16-bit counter, stopping at the terminator or when the counter runs out.
- `find_character_in_string`: Scans a null-terminated string for a given character and returns a pointer to its first occurrence.
- `return_zero_status`: Returns the constant zero and does nothing else.
- `copy_c_string`: Copies bytes from a source string to a destination buffer, including the terminating zero byte, advancing both pointers until the terminator has been copied.
- `append_c_string`: Advances to the terminating zero of the destination string, then copies the source string into that position, including its own terminator.
- `compare_bounded_strings`: Compares two strings byte by byte for at most a given count, returning one as soon as they differ.
- `return_one`: This library routine returns a constant value of one.
- `walk_string_up_to_count`: Walks a string for up to a given number of characters, and when it reaches the terminating zero it calls a library routine to handle the end of the string.
- `forward_call_no_result`: Calls one library routine with no arguments and returns, discarding any result.
- `copy_string_with_padding`: Copies a string into a buffer of a given size, stopping after the terminating zero, then fills the rest of the buffer with zero bytes.
- `return_without_effect`: Returns immediately with no effect.
- `initialize_list_header_node`: Sets the first longword of a block to point just past its start, clears the next longword, and stores the block's own address in the third longword.
- `process_icon_request`: Returns one when given no entry.
- `process_entry_list_nodes`: Walks a linked list of entries through their next field, running a library check on each entry and then a second routine with the entry's pointer and a size taken from a byte field.
- `check_entry_chain_pair`: Walks a chain of entries through their next link, calling two library routines on each one.
- `return_zero_result`: Returns the constant zero without any other work.
- `allocate_and_fill_entry`: Allocates a cleared twenty-byte block and fills it through library routines, storing a value from a library call in its pointer-sized field.
- `return_minus_one_status`: Returns minus one with no other work.
- `build_entry_chain`: Allocates cleared entries one at a time, linking each new entry to the previous one through its next field and filling it through library calls.
- `fill_entry_chain_fields`: Walks a chain of entries, linking each to the previous one and filling its pointer field through library calls.
- `fill_entries_from_stack`: Stores a pointer argument into each entry's link field, then fills entries through library calls until one has no further link.
- `return_negative_one`: This library routine returns a constant value of negative one.

### Populous (1989)(Electronic Arts)[cr Unicorn].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_from_exception`: Returns from an exception handler by executing an rte instruction.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_minus_one_stub`: Returns minus one and performs no other work.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_stub`: Returns immediately without any work.
