# Game 308 (World Cup 90): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00308). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 31 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 575 of 575 named (35 author symbols), 575 of 575 described (106 by rule, 40 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1865 of 1865 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16388 of 16389 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 668684 |
| code | 53060 |
| data | 112512 |
| padding | 7536 |
| unused | 59392 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| World Cup 90.adf:setmap | 13 | 627 | 33 |
| World Cup 90.adf:Ed | 3 | 292 | 5 |
| World Cup 90.adf:type | 2 | 117 | 2 |
| slp | 1 | 4 | 1 |
| add21k | 3 | 480 | 33 |
| World Cup 90.adf:WORLDCUP90 | 4 | 14091 | 469 |
| World Cup 90.adf:show | 1 | 440 | 20 |
| World Cup 90.adf:wait | 2 | 117 | 2 |
| World Cup 90.adf:SLIPCHECK | 2 | 209 | 8 |
| World Cup 90.adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- World Cup 90/World Cup 90.adf:S2: high-entropy data file (7.74 bits/byte), classified as packed data
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- World Cup 90/World Cup 90.adf:Ed: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- World Cup 90/World Cup 90.adf:SLIPCHECK: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### World Cup 90.adf:setmap

- `start_and_parse_command_line`: Program entry for the setmap program.
- `close_library_and_reply_message`: Closes the library held in one global handle when it is set, and when a second handle is set it forbids multitasking and replies to a message through the exec library.
- `call_two_library_functions`: Calls two functions from a library base held in a register, at two fixed negative offsets, in turn, and returns.
- `open_dos_library_or_fail`: Opens the dos library and returns its handle when that succeeds.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `invoke_optional_handle_routines`: Calls a routine with each of several global handle values when they are set, passing one global by reference, then always calls it with a fixed handle and the incoming argument.
- `store_arguments_and_call_helpers`: Stores its two arguments into two global variables, then calls one helper with a fixed set of arguments that includes a global address and constants.
- `resolve_keymap_name_and_open`: Determines a keymap name from icon tool types, command-line arguments, or a default value, then converts the string to lowercase.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_head`: Library stub: loads its arguments and the exec library base, calls AddHead and returns its result to the caller.
- `call_find_name`: Library stub: loads its arguments and the exec library base, calls FindName and returns its result to the caller.
- `call_find_task`: Library stub: loads its arguments and the exec library base, calls FindTask and returns its result to the caller.
- `call_alloc_signal`: Library stub: loads its arguments and the exec library base, calls AllocSignal and returns its result to the caller.
- `call_free_signal`: Library stub: loads its arguments and the exec library base, calls FreeSignal and returns its result to the caller.
- `call_put_msg`: Library stub: loads its arguments and the exec library base, calls PutMsg and returns its result to the caller.
- `call_wait_port`: Library stub: loads its arguments and the exec library base, calls WaitPort and returns its result to the caller.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_open_device`: Library stub: loads its arguments and the exec library base, calls OpenDevice and returns its result to the caller.
- `call_close_device`: Library stub: loads its arguments and the exec library base, calls CloseDevice and returns its result to the caller.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `call_open_resource`: Library stub: loads its arguments and the exec library base, calls OpenResource and returns its result to the caller.
- `call_open_library`: Library stub: loads its arguments and the exec library base, calls OpenLibrary and returns its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `pass_params_to_subroutine`: Calls another routine with a global value, its parameter and the address of an eight-byte stack slot.
- `call_global_library_function`: Calls one library function through a library base read from a global variable at a fixed offset, passing no arguments, and returns.
- `format_text_and_write_output`: Formats text into a 140-byte stack buffer through the exec library formatting function, then measures the string length.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.

### World Cup 90.adf:Ed

- `run_ed_entry_memory_ops`: Program entry point for the Ed program.
- `stub_return_only`: The listing is only a return, while the recorded facts list one callee, so it performs no work of its own.
- `forward_call_to_subroutine`: A single-instruction routine whose only recorded behaviour is a call to one other routine.
- `store_values_and_call_vector`: Stores a computed value and a fixed constant into two memory locations, calls another routine, and then calls through a fixed low-memory vector.
- `decompress_bit_stream_backwards`: Decodes a bit-packed stream read from the end of a buffer, using lookup tables to drive back-reference copies that write output backwards.

### World Cup 90.adf:type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: If an output pointer is supplied, copies sixteen 32-bit words from a fixed offset of 536 in the record into it, then calls the code address held in another register.

### slp

- `terminate_execution`: The routine contains no valid instructions and performs no operations.

### add21k

- `open_two_handles_and_configure`: Program entry that obtains two handles through one routine with a global reference, takes a fallback path with zero when either is null, and sets a flag byte in a nested structure.
- `configure_display_surface`: Looks up two handles through a lookup routine, releasing them via library stubs when a lookup fails, then sets a flag byte in the associated display structure and calls a routine with the sizes 640 and 256.
- `clear_block_and_store_pointers`: Zeroes forty-one 32-bit words starting at an offset from a base pointer taken from the stack, then stores the address of a small stack buffer and a global value into two fields of that same block.
- `call_optional_handler`: Calls the function pointer held in a global variable when that variable is non-zero, then calls one further routine and returns.
- `split_string_into_word_array`: Scans a text buffer using a character class table, replaces the separators with null bytes and builds a null-terminated array of pointers to the words it finds.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `initialize_state_then_return`: Calls another routine ten times, then calls an optional handler from a global and conditionally calls routines with values taken from globals.
- `append_string_to_end`: Walks to the end of the destination string and appends bytes from the source string, stopping at the source terminator or after roughly 32 thousand bytes, then null-terminates the result.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination and returns it.
- `copy_cstring_to_buffer`: Copies bytes from the source string into the destination buffer up to and including the zero terminator, then returns the destination pointer.
- `measure_string_length`: Walks a zero-terminated byte string to its terminator and returns the number of bytes before it.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros.
- `release_indexed_slot`: Checks that an index from zero to nineteen refers to a non-empty six-byte slot in a table.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `forward_to_library_function`: Loads a library base pointer from a global variable and jumps indirectly through a vector in that library, passing control to the library function without further work.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `forward_to_library_routine`: Loads the same library base pointer from a global variable and jumps indirectly through a different vector in that library, handing control straight to its function.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_498`: Library stub: loads a library base from a global variable and calls the library function at offset minus 498 of that base, returning its result to the caller.
- `forward_to_library_entry`: Loads a library base pointer from a global variable and jumps through its vector at a fixed offset, returning to the caller through that function.

### World Cup 90.adf:WORLDCUP90

- `start_from_cli_or_workbench`: Clears a global block of nearly four thousand longwords, then reads the process message to decide whether the program was started from the Workbench or the command line.
- `close_libraries_on_exit`: Runs an optional cleanup handler from a global, then calls exec-library functions that close the libraries and exit the interrupt, and when a pair of globals is set it closes the library bases as well.
- `try_library_call_then_cleanup`: Calls a library function and stores its result in a global; if that result is non-zero it returns it.
- `stub_return_void`: Returns immediately without any work, so it is an empty stub.
- `stub_no_operation`: Returns to the caller immediately without doing any work, so it is an empty routine.
- `release_linked_list_nodes`: Walks a singly linked list from a global head pointer, calling an exec-library function on each node, then clears the head and the count fields and returns zero.
- `tokenize_arguments_and_configure`: Splits a command-line string into up to thirty-two tokens, keeping double-quoted words together and storing the token count.
- `setup_graphics_viewport_and_text`: Opens a graphics library and builds a viewport with a copper list, loads a four-bit colour table, sets the drawing pen and drawing mode, and draws text at fixed positions in a raster port, all through graphics.library jump stubs.
- `allocate_and_initialize_game_tables`: Allocates several memory blocks through the library memory functions after checking available memory, then fills sized tables of entries and clears flag and state fields to initial values.
- `redraw_entries_and_restore_state`: Saves a 32-bit state value, redraws several table entries from slices of object data, and clears or fills entries depending on a flag argument.
- `run_state_machine_frames`: Advances a frame counter, clears a table of 120 two-word records, then loops dispatching on a state byte to one of several state routines until the requested number of frames has elapsed.
- `initialize_slot_positions`: Assigns a position and a handler pointer to each of seventeen slots, skipping one slot per group.
- `animate_slots_for_frames`: Initialises seventeen slot records with positions and handler pointers, then steps horizontal and vertical counters within limits for up to 400 iterations.
- `toggle_side_and_populate_slots`: Toggles a side flag byte from a comparison with a stored value, adjusts a counter for that side, and fills the slot records with positions chosen by the side.
- `invert_side_and_populate_slots`: Increments a counter for the inverted side flag, fills the slot records with positions chosen by that side, then loops adjusting two counters until a limit is reached.
- `populate_slots_and_animate`: Fills the slot records with positions and handler pointers across seventeen entries, then loops adjusting two counters and updating active slots until a limit is reached.
- `fill_slot_rows_and_animate`: Fills the slot records for each row with positions and handler pointers, then steps two counters within limits over up to 400 iterations while counting active slots.
- `step_counters_until_limit`: Sets two fixed fields to twelve, then steps two counters within limits over up to 400 iterations while counting active slots.
- `reset_state_and_call_routine`: Clears a state byte, calls a routine with the constants one, twenty, eight thousand and sixty-four along with a stored value, then sets a flag byte.
- `step_counters_toward_targets`: Derives two target values from position fields, clamps them to the ranges zero to 607 and zero to 83, and moves two stored counters one step toward each target.
- `update_animation_frame_and_positions`: Calls two routines with fixed values, then refreshes the display entries for eighteen and four slots, eases one counter halfway toward a derived target and clamps a second counter to a range.
- `refresh_entries_and_frame_counter`: Clears a flag byte, refreshes the display entries for eighteen and four slots, and eases one counter halfway toward a derived target.
- `update_four_entries_and_frame`: Refreshes four display entries, eases one counter halfway toward a derived target, clamps a second counter and advances a frame counter that wraps after 120 steps.
- `update_positions_and_frame_only`: Eases one counter halfway toward a derived target, clamps a second counter to a range, and advances a frame counter that wraps after 120 steps.
- `reset_entries_and_update_frame`: Clears a state byte, calls routines that set up eighteen entries and four entries from a table, then advances a wrapping frame counter.
- `wait_until_status_flag_set`: Sets up a handler pointer and a field, then loops calling a routine until one of two status bytes becomes non-zero.
- `refresh_active_slot_display`: For an active slot, refreshes eighteen and four display entries, eases one counter halfway toward a target derived from a stored value, and clamps a second counter to a range.
- `run_direction_input_loop`: Initialises fields according to a mode value, then loops reading a direction bitmask and mapping it to a step between minus two and plus two.
- `draw_two_text_labels`: Sets the drawing pen and draw mode on a raster port, then draws a byte string at two fixed positions, moving the pen before each call.
- `draw_offset_text_pair`: Draws a text string twice, once at the given coordinates with one pen value and once offset by two units with a second pen value.
- `draw_and_count_down_handler`: Calls a routine to compute a value, draws at a horizontal position derived from four times that value and a fixed vertical position, then decrements a countdown.
- `initialize_data_structure`: Stores several multi-byte values and a function pointer into a table at a specific offset.
- `initialize_data_structure_alt`: Populates a table with different multi-byte values and a function pointer than the previous routine.
- `update_scrolling_state`: This routine manages a state machine that updates scrolling deltas and position offsets based on a counter that decrements toward a threshold.
- `start_scroll_phase_with_deltas`: Stores two values obtained from routines as scroll deltas and sets the phase to one.
- `restart_scroll_phase_deltas`: Stores two values obtained from routines as scroll deltas and sets the phase to one, repeating the start of the scroll phase sequence.
- `clear_status_long_field`: Stores zero into a long-word field two bytes past a base pointer and returns.
- `initialize_entry_table_and_palette`: Initialises twelve table entries, giving each a position, flags and a palette index taken from lookup tables, then sets starting position and scroll values.
- `find_and_remove_named_task`: Looks up a task by a name at a fixed offset and, when one is found, removes it through the exec library.
- `load_view_and_wait_frame`: Waits for the vertical blank, loads a view from a stored pointer, and waits for the vertical blank again, using graphics.library calls.
- `jump_to__remtask`: Jump stub: passes control straight on to _RemTask.
- `draw_and_clear_entry_list`: Loops over a count byte of entries, calls a routine with an offset derived from a width field and a second field for each, then zeroes the two words of each entry.
- `start_audio_channel_dma`: Stores a sample pointer and length for an audio channel, writes the channel period register and sets the volume.
- `run_side_counter_sequence`: Loops five times, adding one to a side counter whenever a routine returns one and calling routines with 50 on each pass.
- `jump_to__findtask`: Jump stub: passes control straight on to _FindTask.
- `unlink_node_from_list`: If the node's flag bit is clear, unlinks it from a doubly linked list by updating its neighbours' pointers.
- `set_node_byte_fields`: Stores each of two arguments into byte fields of a referenced structure when they are non-negative, and returns the last value stored.
- `run_count_loop_or_fail`: Obtains a count from a routine; if the count is at least one it clears a field, calls a routine once per index up to the count, then calls two more routines and returns zero.
- `set_table_flag_entry`: Sets the long-word entry at an index of a flag table to one.
- `jump_to__openlibrary`: Jump stub: passes control straight on to _OpenLibrary.
- `free_bucketed_node_lists`: Walks seventy-one buckets of linked nodes and, when the flag argument is non-zero, releases each node's attached buffers and the node itself, then clears the bucket head.
- `copy_bucket_heads_to_structure`: Copies seventy-one bucket heads and a pointer field from one structure to another, sets a flag byte and a flag bit in the destination, and then calls a routine with the destination.
- `create_record_with_handler`: Allocates a record through a routine, sets its type byte, a second byte, a pointer to a global area and a zero field, then stores a handler pointer.
- `run_round_countdown_loop`: Sets per-side state from a side index, resets several fields, loads a colour table and places entries with fixed values, then runs a countdown from eleven that adjusts animation frames and calls routines with the side value.
- `jump_to__makevport`: Jump stub: passes control straight on to _MakeVPort.
- `jump_to__mrgcop`: Jump stub: passes control straight on to _MrgCop.
- `allocate_block_array`: Calls a routine with the four parameters, then requests up to a count of blocks through a routine and stores each returned pointer in an array.
- `jump_to__loadview`: Jump stub: passes control straight on to _LoadView.
- `relink_node_into_list`: When the node's flag bit is set, takes a record from the head of a free list, clears the flag, links it onto the front of the target list and calls a routine with the node.
- `store_base_pointer_global`: Stores the data base pointer into a global variable and returns.
- `jump_to__availmem`: Jump stub: passes control straight on to _AvailMem.
- `init_record_from_table_entry`: Initialises a record from a table entry, copying a pointer, a width and a height, and zeroes its counters and link fields.
- `copy_string_and_return_length`: Copies a zero-terminated string from source to destination including the terminator, counting the characters copied, and returns that count.
- `format_two_digit_text`: Writes two characters into a buffer: the first is a tens digit obtained from a routine and shown as a blank when zero, the second is a digit made from the difference between the parameter and a second routine result.
- `convert_number_to_text_buffer`: Converts a value into a fixed text buffer through a routine, clears a flag byte, and passes the buffer to another routine with a length of two.
- `build_status_text_line`: Fills a fifty-character buffer with spaces, writes two text strings taken from table entries and two status digits into it, and returns the buffer address.
- `setup_entry_from_parameter`: Copies four 16-bit values from a lookup table into a record, marks a slot as unused and sets its state from a table index.
- `run_main_game_loop`: Sets up the playfield with fixed sizes and loads graphics through library calls, then waits for the beam position.
- `repeat_call_count_times`: Calls a routine once for each unit of the count argument, returning when the count is no longer positive.
- `fade_palette_entries_step`: Steps each entry of a 32-colour palette down by one in each of its three nibble components, loads the result through a library call and calls a routine with the parameter.
- `fade_palette_toward_target`: Clears a 32-entry palette, then for sixteen passes moves each colour's three nibble components one step toward a target palette.
- `blit_slot_bitmap`: Looks up a bitmap record in a slot table and, when one is present, blits it into the raster port at the given coordinates using a graphics.library call.
- `toggle_display_buffer_and_draw`: Toggles a double-buffer flag, switches the view and raster port pointers to the other buffer, and copies 32 colour words from a default table.
- `find_region_code_at_point`: Searches a list of eight-byte rectangle records for one that contains the given point and returns its code byte.
- `run_cursor_movement_loop`: Redraws a cursor image at its current position and loops until a stop flag is set.
- `fill_rectangle_for_matching_record`: Scans a list of eight-byte records terminated by zero for one whose key matches the argument.
- `poll_key_to_character`: Performs a handshake on a hardware byte by writing a fixed value and waiting for the serial status bit, then matches the received code against a table of code pairs.
- `poll_key_code_until_mapped`: Repeats a hardware handshake and reads a received code, scanning a table of code pairs for the code and returning the mapped character once one is found.
- `draw_text_with_fill_padding`: Moves to a position, draws a text string, then draws a fixed fill character repeatedly until the field reaches the given count, using graphics.library text and move calls.
- `draw_indexed_status_entry`: Looks up a status index in a table of fourteen-byte entries.
- `draw_scrolling_table_rows`: Draws a box and twenty table rows from a scroll offset, choosing the pen and text for each row from flags and table values.
- `draw_label_for_state`: Draws a label chosen by a state value: when the value matches one code it copies four words from a table and draws the entry, otherwise it clears three words and draws the default label.
- `wait_for_selection_then_draw`: Loops until a selection routine returns a non-zero code, mapping codes one, two and three to state values twenty-four, eight and two.
- `handle_menu_input_loop`: Loops until a selection flag is set, reading an input code and dispatching it.
- `dispatch_by_table_code`: Searches a table of six-byte entries for the current code byte and jumps to the function pointer stored beside the matching entry.
- `draw_grid_of_table_cells`: Draws eight rows of cells, positioning each cell from a table and repeating within a row while the table count entry is greater than one.
- `run_selection_input_loop`: Loops reading an input code and dispatching it: codes four and five set state bytes for the current item, codes six to eight set a zero state and a character value, and codes eleven and twelve move the item index within bounds.
- `draw_panel_then_wait`: Draws a fixed panel of text fields with a value from a parameter table, then loops calling an input routine until one of two input status bytes becomes non-zero.
- `draw_single_value_and_wait`: Draws a short label, a value taken from a table and a status line, then loops until one of two input status bytes becomes non-zero.
- `draw_six_row_table_panel`: Draws a panel showing a letter from the parameter, a count, and six rows whose values come from table lookups, plus a summary line for the current index.
- `draw_ranked_entry_panel`: Draws up to four ranked rows chosen from a sorted index table, each with a label and a value taken from a table, plus a short encoded text line for the current item.
- `sort_four_indexed_entries`: Initialises four indices and sorts them by comparing two table fields, swapping entries when the first field is smaller or when the first fields tie and the second is smaller.
- `draw_title_then_wait`: Draws a title with a letter from the parameter, four table rows and their values, then loops until one of two input status bytes is set.
- `wait_for_unpaired_status_flag`: Loops calling a routine until one of two status bytes is set without its partner byte being set, then returns.
- `stub_immediate_return_loop`: Sets a flag to true and returns straight away, so the polling loop that follows never runs and the routine does no work.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `initialize_ranking_table`: Marks each entry as free, then fills six records by calling a routine for each and copying its values into four slots, replacing the lowest slot when a better value is found.
- `run_selection_menu`: Shows a selection list of up to fourteen entries and loops reading input.
- `run_round_fixtures`: Loops through rounds while the round counter is below fifteen.
- `set_result_flag_from_counts`: Sets up a display with library calls, sets a state flag and clears a handler pointer, then sets a result word to zero or one depending on whether the first counter is below the second.
- `initialize_four_slot_codes`: Calls routines to set up a display, then fills four slot entries with codes derived from their index and stores the first code as a result.
- `run_pair_table_fixtures`: Fills a six-by-six table of pair codes and clears score fields, then runs thirty-six fixtures in turn, each waiting for input, taking an outcome from a routine, updating the two sides and drawing the result with graphics calls.
- `open_display_window_and_show`: Opens a display resource with two fixed arguments and, when it succeeds, prepares a raster port and draws its contents.
- `clear_four_buffers_with_blitter`: Claims the blitter, clears four buffers each of a size equal to width times height halved, using the record's width and height, then releases the blitter.
- `divide_32bit_software`: Performs a software 32-bit division with sign handling, using shifts to normalise the divisor.
- `jump_to__draw`: Jump stub: passes control straight on to _Draw.
- `jump_to__move`: Jump stub: passes control straight on to _Move.
- `jump_to__setapen`: Jump stub: passes control straight on to _SetAPen.
- `get_string_byte_count`: Walks a zero-terminated string to its terminator and returns the number of bytes before it.
- `process_nonempty_list_head`: Returns the pointer unchanged when it is null or its first long is zero.
- `jump_to__freemem`: Jump stub: passes control straight on to _FreeMem.
- `jump_to__bltbitmaprastport`: Jump stub: passes control straight on to _BltBitMapRastPort.
- `jump_to__setrast`: Jump stub: passes control straight on to _SetRast.
- `jump_to__vbeampos`: Jump stub: passes control straight on to _VBeamPos.
- `unpack_iff_asset_data`: Parses tagged data chunks to decompress run-length encoded pixel data, convert color maps, or initialize bitplanes.
- `sample_input_state_bits`: Copies the four previous input state bytes into history, then sets the current input bytes from a global value by inverting and masking its bits.
- `jump_to__loadrgb4`: Jump stub: passes control straight on to _LoadRGB4.
- `jump_to__waittof`: Jump stub: passes control straight on to _WaitTOF.
- `jump_to__waitbovp`: Jump stub: passes control straight on to _WaitBOVP.
- `allocate_double_buffer_resources`: Allocates a control structure and a second block, then creates one or two sets of resources, each with small buffers and a sub-structure whose links are initialised.
- `read_frame_records_from_file`: Reads a sequence of frame records from a file, allocating a block for each frame and storing its fields, and checks for an end-marker identifier before continuing.
- `jump_to__setdrmd`: Jump stub: passes control straight on to _SetDrMd.
- `set_pointer_if_nonzero`: When the first long at the argument is non-zero, stores the address one long past it into a global pointer, calls a routine and sets a flag.
- `jump_to__text`: Jump stub: passes control straight on to _Text.
- `jump_to__setbpen`: Jump stub: passes control straight on to _SetBPen.
- `free_double_buffer_resources`: Releases the resources created for one or two buffers: for each set it frees the records and buffers in turn, then frees the control structure and the final pair of blocks.
- `allocate_tagged_record`: Calls a routine to obtain a file handle and, when it is positive, allocates a block of the requested size plus four bytes, reads that many bytes from the file into it and stores the size in its first long.
- `multiply_32bit_software`: Multiplies two 32-bit values in software by combining their 16-bit halves, returning the low 32 bits of the product.
- `jump_to__allocmem`: Jump stub: passes control straight on to _AllocMem.
- `dispatch_state_handlers_per_entry`: Loops over a count of records, each 406 bytes long.
- `update_cached_frame_index`: Copies a frame index into a cached field when it has changed or a force flag is set, then calls a routine with the new index and a fixed table.
- `redraw_slot_status_when_forced`: When the force argument is non-zero, calls a routine with a fixed table entry and the status byte stored in the record.
- `redraw_slot_position_when_forced`: When the force argument is non-zero, calls a routine with a second fixed table entry and the position byte stored in the record.
- `update_direction_from_offsets`: Computes a direction code from the horizontal and vertical offsets to a target, stores it, and sets the state to eleven when the code is zero.
- `update_state_byte_on_change`: Stores the incoming byte into a status field when it differs from the stored value or a force flag is set, then calls a routine.
- `handle_state_change_with_offset`: When the force argument is clear, sets the state to eleven if a flag bit is set.
- `set_state_eleven_when_flagged`: Sets the state word to eleven when bit zero of a flag byte is set.
- `set_state_eleven_or_redraw`: When the force argument is non-zero, calls a routine with a fixed table entry and the status byte stored in the record.
- `set_value_eleven`: Checks a bit in a byte at an offset and, if set, stores a word with the value eleven into another offset of a structure.
- `redraw_slot_second_entry`: When the force argument is non-zero, calls a routine with a second fixed table entry and the stored status byte.
- `update_slot_status_flag`: Checks a bit in a structure field and, if set, writes a constant value to a different offset within that same structure.
- `redraw_slot_third_entry`: Calls a routine with a fixed third table entry and the status byte stored in the record.
- `update_direction_clear_on_zero`: Computes a direction code from the horizontal and vertical offsets to a target and stores it.
- `store_status_redraw_on_change`: Stores the incoming byte in the status field when it differs from the stored value or a force flag is set, then calls a routine.
- `clear_state_or_redraw_first`: When the force argument is non-zero, calls a routine with a fixed table entry and the status byte stored in the record.
- `clear_state_when_flagged`: Clears the state word when bit zero of a flag byte is set.
- `clear_state_or_redraw_second`: When the force argument is non-zero, calls a routine with a second fixed table entry and the status byte stored in the record.
- `reset_state_on_flag_bit`: Sets the state word to zero when bit zero of a flag byte is set.
- `reset_state_or_redraw_third`: When the force argument is non-zero, calls a routine with a third fixed table entry and the status byte stored in the record.
- `clear_value_third`: Checks a bit in a byte at an offset and, if set, stores a word with the value zero into another offset of a structure.
- `reset_state_or_redraw_fourth`: When the force argument is non-zero, calls a routine with a fourth fixed table entry and the status byte stored in the record.
- `clear_flagged_word`: Checks if a specific bit is set in a byte at an offset from a pointer, and if so, clears a word at a different offset from that same pointer.
- `clear_timers_or_redraw_entry`: When the force argument is non-zero, calls a routine with a fixed table entry and the status byte stored in the record.
- `clear_timer_fields_when_flagged`: Clears two long-word counters at offsets 20 and 24 when bit zero of a flag byte is set.
- `reset_state_or_redraw_fifth`: When the force argument is non-zero, calls a routine with a fifth fixed table entry and the status byte stored in the record.
- `clear_flagged_word_2`: Checks a specific bit in a byte within a structure and, if set, clears a word at a different offset within that same structure.
- `classify_point_outside_bounds`: Returns a bit mask describing whether the two arguments lie beyond fixed limits of plus or minus 576: bit value one when the first exceeds 576, two when it is below minus 576, four when the second is below minus 576, and eight when it exceeds 576.
- `classify_bounds_into_mask`: Builds a bit mask from two arguments against limits of plus or minus 576: bit two when the first is below minus 576, bit eight when the second exceeds 576, and bit four when the second is below minus 576.
- `set_bit_below_lower_limit`: Sets bit four in the mask when the second argument is below minus 576, and returns the mask.
- `decode_mask_to_direction`: Decodes a bit mask into two direction values: bits one and two give a horizontal step of plus or minus one, and bits eight and four give a vertical step of plus or minus one.
- `load_entry_record_fields`: Looks up an entry record from a table at an index, copies its byte fields into two status bytes and a mode byte, and sets a long field from the record.
- `advance_entry_position_with_velocity`: Adds velocity values to the position fields, updates the vertical position with an overflow check, and calls a routine to test the result.
- `nudge_entry_from_reference_point`: Advances an entry by its velocity, then checks its distances to a fixed reference point and nudges its horizontal and vertical positions by 128 or 256 units when the offsets fall within set limits.
- `nudge_entry_when_near_reference`: Checks an entry's distances to a fixed reference point and nudges its horizontal and vertical positions by 128 or 256 units when they fall within set limits.
- `nudge_entry_horizontal_offset`: Nudges the horizontal position by 128 when the vertical offset is within range, and nudges the vertical position by 256 when the horizontal offset is within range.
- `nudge_entry_unconditional_steps`: Nudges the horizontal position by 128 and, when the target is within range, the vertical position by 256, both toward the reference point.
- `offset_entry_and_nudge_vertical`: Sets the horizontal position to its stored value plus 128 and nudges the vertical position by 256 when the target is far away.
- `nudge_entry_side_by_256`: Moves the vertical position by 256 toward the side of the reference value when the horizontal offset is within range.
- `offset_entry_vertical_by_256`: Sets the vertical position to its stored value plus 256, then updates the status byte and derived fields.
- `update_status_when_flag_set`: When the flag argument is non-zero, sets the status fields chosen by a status byte.
- `set_status_and_derive_fields`: Calls a routine, sets the status fields and the mode field, then computes the derived fields from the position.
- `advance_entry_position_only`: Adds velocity values to the position fields, updates the vertical position with an overflow check, and sets the derived fields from the new position.
- `update_entry_drag_zone`: Applies drag to the velocity, advances the position and tests the new position through a routine.
- `update_entry_position_and_zone`: Advances the position by its velocity and tests the new position through a routine.
- `update_speed_status_and_fields`: Sets a status byte from whether the sum of the absolute velocity components exceeds 256, clears a derived field when a counter is at or above 2689, and sets the remaining derived fields from the position.
- `update_speed_status_from_argument`: Sets a status byte from whether the absolute vertical velocity plus the argument exceeds 256, clears a derived field when a counter is at or above 2689, and sets the remaining derived fields from the position.
- `update_status_from_sum_arguments`: Sets a status byte to one when the sum of its two arguments exceeds 256, clears a derived field when a counter is at or above 2689, and sets the remaining derived fields from the position.
- `clear_status_and_set_fields`: Clears a status byte, clears a derived field when a counter is at or above 2689, and sets the remaining derived fields from the position.
- `derive_position_fields_only`: Sets the derived fields from the position: a negated vertical field, the horizontal position, and a vertical value computed from the current position.
- `update_entry_with_velocity_damping`: Applies drag to the velocity, advances the position and checks a routine result.
- `update_entry_with_offset_damping`: Adds an offset to the position, checks a routine result and may halve the velocity components.
- `clamp_speed_and_update_status`: Clamps a speed counter to 2624, sets state fields chosen by a threshold of 25600 when the clamp applied, then updates the status byte and derived fields.
- `set_flag_and_update_status`: Sets two flag bytes to one and six, sets the status byte when the sum of the absolute velocity components exceeds 320, then sets the derived fields.
- `update_status_from_argument_threshold`: Sets the status byte when the absolute vertical velocity plus the argument exceeds 320, then sets the derived fields from the position.
- `update_status_from_sum_threshold`: Sets the status byte when the sum of its two arguments exceeds 320, then sets the derived fields from the position.
- `clear_status_and_derive_fields`: Clears the status byte and sets the derived fields from the position.
- `initialize_entry_position`: Sets the first and third fields and the vertical value of a record from stored position values, and sets the remaining derived fields.
- `classify_entry_zone`: Classifies an entry's position against one of two sets of fixed boundary values, chosen by a flag byte, and returns a zone code of one, two, four or eight.
- `zone_code_from_vertical_position`: Returns a zone code from the vertical position: four when it is at or above 27777, eight when it is below 7424, and zero otherwise.
- `zone_code_below_lower_limit`: Returns eight when the argument is below 7424, otherwise returns the incoming zone value unchanged.
- `classify_entry_zone_inline`: Sets up fixed boundary values, calls a routine to test them against the entry, then returns a zone code of one, eight or four from the vertical position.
- `classify_vertical_zone_code`: Returns four when the vertical position is at or above 27777 and eight when it is below 7424, leaving the zone value unchanged otherwise.
- `return_zone_constant`: Compares a value against a limit and returns eight if the value is below that limit.
- `add_routine_value_return_change`: Obtains a value from a routine, adds it to a stored value through a pointer, and returns the old value minus the new one.
- `add_offset_return_change`: Obtains a value from a routine, adds the second field of a record to it, stores the sum through a pointer and returns the old value minus the new one.
- `clamp_position_to_bounds_mask`: Clamps a position pair against minimum and maximum bounds held in a record, calling a routine to resolve each bound that is crossed.
- `integrate_motion_with_decay`: Copies the current position into two output values, then repeatedly decays the velocity by a sixteenth and adds it to the position.
- `manhattan_distance_2d`: Returns the sum of the absolute horizontal and vertical differences between two points.
- `sum_abs_and_signed_difference`: Returns the absolute difference between two register values added to the signed difference between two other register values.
- `add_difference_to_argument`: Returns the difference between two register values added to the incoming argument.
- `chebyshev_distance_2d`: Returns the larger of the absolute horizontal and vertical differences between two points.
- `compute_normalized_offset`: Stores the horizontal and vertical differences between two points into two output values.
- `normalize_offset_if_nonzero`: When the absolute second value plus the first is non-zero, replaces the two output values with results from routines.
- `refresh_offset_if_nonzero`: When the sum of the two register values is non-zero, replaces the two output values with results from routines.
- `find_region_hit_in_list`: Walks a linked list of bounding boxes and returns the value attached to the first box that contains the stored position, with its first coordinate offset by 256.
- `find_region_for_entry`: Walks a linked list of bounding boxes, testing the entry's position against each box's ranges, and returns the value attached to the first box that contains it.
- `stub_return_zero`: Returns zero without doing any work, so it is an empty stub.
- `forward_entity_field_to_routine`: Passes its structure pointer to a second routine together with a word-sized value read from a fixed field inside that structure, then returns.
- `call_routine_when_enabled`: Returns zero at once when a status field in the structure is zero or a global flag byte is set.
- `forward_to_nested_routine`: Calls one other routine with no arguments and returns, so it is a bare forwarding stub around that call.
- `fill_handler_tables_for_entries`: Loops over 18 entries and, for each entry whose state byte is below 2, fills a block of 15 handler slots with one routine's address, stores a fixed value and a handler in per-entry fields, and sets the state byte to 4.
- `run_state_handler_loop`: Repeats a count-controlled loop that bumps two global counters, latches the entity state word, and calls the handler from a per-slot table when one is present.
- `update_entity_state_transitions`: Calls a routine and, when its result is negative, updates the entity state byte and state word through a branch tree that tests flags, calls routines and global counters.
- `advance_entity_state_branches`: Runs the same state branch tree as the transition routine, with the entity pointer taken from a register, to update the state byte, state word and global counters.
- `clear_entity_state_word`: Clears the 16-bit state word in the entity structure to zero and returns.
- `handle_routine_result_state`: Calls a routine and branches on its result.
- `set_entity_failure_state`: Sets the entity state word to a fixed failure code, marks a flag byte as full, and sets a counter to sixteen.
- `advance_entity_counter_and_mode`: Increments an entity counter, copies a direction flag into the mode word, and when the global mode byte differs from the entity's saved mode after more than two increments it switches the state to a fixed code with a timer value of nine.
- `set_mode_and_check_switch`: Sets the mode word to one, then if the global mode byte differs from the entity's saved mode and the counter exceeds two, it switches the state to a fixed code and sets a timer value of nine.
- `reset_entity_mode_and_poll`: When a secondary flag is clear, it resets the counter and mode word from a direction flag, calls a routine, and clears a status byte or calls a second routine by its result.
- `reset_mode_and_poll_status`: Clears a counter byte and sets the mode word from a direction flag.
- `set_mode_and_poll_status`: Sets the mode word to one and calls a routine.
- `clear_entity_status_byte`: Writes zero to a single status byte in the entity structure and returns.
- `update_entity_accumulators_and_mode`: With a nonzero second argument, it adds the entity counter into a global total, rescales the counter, and calls a routine for two outputs.
- `rescale_and_accumulate_entity_values`: Rescales an entity counter, calls a routine with two stack-allocated outputs, adds the returned values into two global accumulators, and sets the mode word to two.
- `set_state_when_mode_clear`: If the entity mode word is zero, it sets the entity state word to a fixed code of nine.
- `handle_entity_timer_and_range`: When its second argument is zero, it counts down a timer and restores a saved state when the count reaches zero, or, if the mode word is set, calls a routine and clears a status byte or calls another routine.
- `set_mode_five_and_reset`: Sets the mode word to five, a counter byte to eight and a status byte to zero.
- `count_down_mode_timer`: When the mode word is zero, it counts a timer byte down and restores the saved state when the timer reaches zero.
- `poll_routine_and_clear_status`: Calls a routine and clears a status byte when it returns zero.
- `clear_entity_status_flag`: Writes zero to the status byte in the entity structure and returns.
- `countdown_or_set_mode_nine`: With a zero second argument it counts a timer down while the mode word is zero and restores the saved state at zero.
- `set_entity_timer_value`: Stores a fixed timer value of twenty two in the entity counter byte and returns.
- `countdown_timer_restore_state`: When the mode word is zero, it decrements a timer byte and, when the timer reaches zero, restores the saved state word.
- `advance_position_table_entry`: Steps an index through a table of coordinate pairs held behind a pointer in the entity structure, reads the current pair and stores it scaled up into the entity position fields.
- `update_entity_motion_state`: With a zero second argument it runs the per-frame motion state machine, setting a global speed and speed byte on first entry and, while active, advancing a counter, deriving a direction from a flag and calling a routine when a trigger is set.
- `run_motion_state_branches`: Runs the per-frame motion state branches with the entity pointer taken from a register.
- `begin_motion_state_update`: Initializes the entity with a speed of eight, state ten and a counter of zero, and calls a routine.
- `apply_motion_direction_trigger`: Derives a direction from a flag once the counter is past a threshold and a global frame bit is set, then masks the direction.
- `store_direction_and_trigger`: Stores a direction byte from a register and masks it.
- `dispatch_entity_counter_state`: When its second argument is nonzero it clears a counter and sets the state word to eight.
- `dispatch_counter_state_branches`: When the state word is zero, it dispatches on the counter byte.
- `enter_state_two_with_handler`: Sets the state word to two and the counter byte to two, then calls the handler pointer stored in the entity if one is set.
- `restore_saved_entity_state`: Copies the saved state word into the current state word.
- `update_entity_target_deltas`: When a stored value is below seven hundred, it computes clamped offsets from the entity to a target position, picks a direction state and switches between two state codes depending on whether the direction matches.
- `set_direction_and_target_deltas`: Updates the direction byte, picks a state code from whether the new and old directions match, and stores the target offsets divided by four.
- `set_direction_and_clamp_offsets`: Stores a direction byte from a register and chooses a state code based on whether its direction bits match the previous direction.
- `start_direction_offset_update`: Chooses the state word as four or three depending on whether the new direction bits match the previous direction.
- `reset_direction_and_clamp_offsets`: Sets the state word to four, copies the direction byte, and resets a counter and a timing field.
- `clamp_offset_components`: Zeroes the horizontal offset when a value passed in a register exceeds a fixed limit.
- `clear_vertical_offset_over_limit`: Zeroes the vertical offset when a value passed in a register exceeds a fixed limit.
- `update_offsets_and_notify`: When the state word is zero, it clears a status byte and restores the saved state.
- `clamp_target_offsets_and_notify`: When the status byte is clear, it computes the scaled target offsets and clamps each one to a fixed limit before calling a routine.
- `clamp_offset_and_notify`: Zeroes the horizontal offset when a register value exceeds a fixed limit and zeroes the vertical offset when its magnitude exceeds the same limit.
- `clear_vertical_offset_and_notify`: Zeroes the vertical offset when a register value exceeds a fixed limit.
- `set_entity_error_state`: Sets the entity state word to a fixed error code and returns.
- `run_handler_and_set_mode`: With a zero second argument it restores the saved state when the mode word is zero.
- `restore_state_when_mode_clear`: When the mode word is zero, it copies the saved state word into the current state word.
- `choose_entity_target_response`: Returns a routine result and, when two global flags are clear, sets the entity state from that result: a negative result leads to a target move, a state code with a stored handler or small nudges to two global totals, and other results set a fixed state.
- `choose_target_response_on_negative`: Chooses the target response when a routine returns a negative result, with the entity taken from a register.
- `choose_target_response_on_zero`: Chooses the entity target response when a routine returns zero, with the entity taken from a register.
- `move_to_target_or_nudge`: Moves the entity to a target position and sets its direction when a routine returns zero.
- `apply_direction_or_nudge_totals`: When a routine result is eight or four, it stores that value as the entity direction and sets a state code with a stored handler.
- `nudge_totals_and_reset_timer`: Calls a routine, then adds or subtracts a fixed step from one global total depending on a global flag bit and another fixed step from the second total depending on the entity side flag.
- `subtract_step_from_totals`: Subtracts a fixed step from one global total and adds or subtracts a fixed step from the other depending on the entity side flag.
- `add_step_to_global_total`: Adds a fixed step to one global total, clears the counter byte and sets a timer value of ten.
- `move_to_pair_target_position`: Moves the entity to a target position read from a pair table using its side flag, sets the mode word to eight, and computes the direction byte from the offsets to that position.
- `choose_side_target_response`: Clears a status byte and, below a global threshold, moves the entity to a target position unless it is on the other side and a computed distance is small, in which case it counts attempts before setting a fixed state.
- `move_to_offset_target_position`: Stores a target position made from a global x value minus a fixed offset and the global y value.
- `adjust_side_target_position`: When the entity side differs from the global side and a computed distance is small, it counts attempts and moves toward a target offset to one side, switching to a fixed state after many attempts.
- `set_offset_target_and_timer`: Sets the target position from the global x value minus a fixed offset and the global y value, sets the mode word to eight and sets a timer value of ten.
- `move_to_shifted_target_position`: Sets the target position to a global x value shifted by a fixed smaller offset and the global y value, then sets the mode word to eight and a timer value of ten.
- `advance_timed_entity_sequence`: With a zero second argument it counts a timer down and restores the saved state at zero.
- `store_pending_entity_slot`: When the low bits of a global counter are below eight and a global flag is clear, it saves the entity position and pointer into global slots.
- `count_down_and_restore_state`: Decrements a timer byte and, when it reaches zero, copies the saved state word into the current state word.
- `update_pair_target_approach`: Sets the mode word to twelve.
- `check_pair_range_and_approach`: Sets the mode word to twelve.
- `set_state_by_threshold_value`: Sets the mode word to twelve and the state word to fifteen when a register value is below a fixed threshold, or to fourteen otherwise.
- `enter_state_fourteen_mode`: Sets the state word to fourteen and the mode word to twelve.
- `set_pair_target_and_mode`: Clears a status byte, stores the pair-table target for the entity side as its target position and sets the mode word to twelve.
- `update_mode_thirteen_speed`: With a zero second argument it returns the entity to state twelve when the mode word is eleven and clears a global flag.
- `reverse_speed_when_in_range`: When a register value is below a fixed threshold and a low-bit value from a timing routine passes a test, it negates a global speed total and adds a fixed amount to another total; otherwise it calls a routine.
- `advance_mode_eleven_state`: When the mode word is eleven, it sets the state word to twelve and clears a global flag.
- `run_mode_eleven_transitions`: With a zero second argument it advances a mode eleven sequence: status two resets globals, installs handler slots for all 18 entries and moves to mode fifteen, status one moves to mode fourteen, and otherwise the state returns to twelve.
- `set_handler_and_global_flag`: Stores a handler pointer in the entity, sets a status byte to one, calls a routine with fixed arguments and sets a global flag to one.
- `advance_mode_eleven_transitions`: When the mode word is eleven, it resets globals and installs handler slots for all 18 entries when the status byte is two.
- `reset_globals_enter_mode_fifteen`: Clears a global field, sets another to a fixed negative value and calls a routine.
- `enter_mode_fifteen_with_handler`: Sets the mode word to fifteen and a status byte to three, then stores a handler pointer in a global slot.
- `return_to_state_twelve`: Sets the state word to twelve and clears a global flag.
- `find_nearby_opposite_side_slot`: Scans the 18 entity slots for the first eligible entry on the opposite side from the given entity that lies within a distance threshold and is not the entity itself.
- `find_next_opposite_side_slot`: Scans forward from a starting slot index for an eligible entry on the opposite side that lies within a distance threshold.
- `return_no_slot_index`: Returns minus one, the value that means no entity slot matched.
- `update_speed_from_entity_gap`: Computes the horizontal and vertical gap between two entities and their combined distance.
- `update_speed_from_register_gap`: Uses a horizontal and a vertical gap passed in registers to set a global speed value and a counter field from their combined distance.
- `update_speed_from_gap_sum`: Adds two register values to form a combined distance, then sets a global speed value and a counter field from it.
- `scale_speed_totals_toward_gap`: Scales two global speed totals by fractions of two gap components passed in registers, then calls routines to finish the update.
- `find_nearest_same_side_slot`: Searches the 18 entity slots for the same-side entry nearest to the given entity.
- `return_immediately`: This routine is empty and performs no operations before returning to the caller.
- `check_opposite_side_within_range`: Returns minus one when any entity on the opposite side lies within a distance threshold of the given entity, and zero otherwise.
- `check_opposite_side_range_register`: Returns minus one when an entity on a side that differs from a register value lies within a distance threshold, and zero when no such entity exists.
- `return_zero_result`: Returns zero.
- `set_speed_from_direction_bits`: Calls two routines to start the update.
- `set_fixed_speed_totals`: Sets a horizontal speed total to a fixed value that is reversed for one side and a vertical speed total to another fixed value.
- `set_speed_by_direction_range`: Calls two routines to start the update.
- `set_fixed_speed_when_far`: Compares a routine result with a stored threshold and calls a routine when it is below.
- `set_speed_totals_from_height`: Sets fixed horizontal and vertical speed totals and reverses the horizontal one for one side.
- `set_speed_by_direction_offsets`: Calls two routines to start the update.
- `set_speed_offsets_from_bits`: Sets the horizontal speed to a side-dependent offset and adds or subtracts fixed amounts according to the direction bits.
- `set_fixed_speed_with_direction`: Toggles the direction bits for one side and sets fixed horizontal and vertical speeds, adjusting the horizontal speed when direction bits are set.
- `set_speed_from_masked_result`: Adds a side-dependent offset to a masked routine result to set the horizontal speed.
- `set_speed_from_timer_mask`: Sets the horizontal speed from a masked timing value minus a small fixed amount, where the timing value comes from a routine that reads the video beam position.
- `set_velocity_from_direction_vector`: Calls routines to start the update and looks up a direction vector for the entity direction byte.
- `set_velocity_from_stored_direction`: Gets a direction vector for the entity stored direction through a routine.
- `set_speed_toward_table_target`: Calls routines to start the update, then sets horizontal speed to a table target minus the entity position, scaled down by a shift, and sets vertical speed from a table target and timing and position values.
- `record_side_status_entry`: Stores the entity status byte in a per-side table and sets a flag when that side stored count matches the status plus an offset.
- `record_status_for_side_index`: Stores the entity status byte in a per-side table at an index passed in a register.
- `clear_side_flag_entry`: Clears a flag byte in a per-side table, using an index computed from a register, and records the entity side in a global.
- `ease_speed_toward_direction`: Looks up a direction vector for the side byte and scales it up.
- `ease_speed_toward_stack_vector`: Eases both speed totals halfway toward a vector held in stack outputs.
- `clear_entity_status_flag_2`: Writes zero to the status byte in the entity structure and returns.
- `extend_speed_toward_stack_vector`: Eases the speed totals toward a stack-output vector when the entity direction matches or is zero, and otherwise extends them to one and a half times the vector.
- `update_interpolated_values`: Performs fixed-point arithmetic to update values in a structure and conditionally modifies a byte based on a flag.
- `clear_byte_offset`: Stores a zero byte into a specific offset of a structure.
- `assign_nearest_entity_pairs`: Chooses the two nearest entities from one side's group of nine candidates for a pairing, with distance measured by a routine.
- `reset_handler_tables_for_entries`: Clears a global flag, then loops over the 18 entries.
- `blit_entity_frame`: Selects the frame for the current buffer index, claims the blitter through a jump stub and runs the drawing calls, then releases it through another jump stub.
- `draw_sprite_list_with_blitter`: Walks a linked list of sprite records, skips those outside the visible area and clips each remaining one, then sets the blitter mask, modulo, pointer and size registers and runs row-drawing passes.
- `program_blit_row_registers`: Computes a blit start offset from a record's fields, then writes blitter mask words, modulo values and source and destination pointers for a row pass.
- `initialize_sprite_record`: Fills in a sprite record from a source descriptor, copying its frame pointer, width, height and a size field, then clears its position fields.
- `insert_record_into_list`: When a record flag bit is set, it clears that bit, marks the record as active and links it at the head of a doubly linked list whose head pointer is passed in.
- `remove_record_from_list`: When the record active bit is clear, it unlinks the record from the doubly linked list whose head pointer is passed in, then sets the bit to mark it as removed.
- `update_animated_record`: Updates a record unless its removal bit is set: a custom handler or velocity integrates its position, an animation frame table advances, and the resulting position, size and frame data are copied to an output slot.
- `reposition_record_by_key`: Moves a record within a list ordered by a key field, searching in both directions from its current position and relinking it between the neighbours where it belongs.
- `write_sprite_output_slot`: Writes one entry of the output buffer for a record.
- `position_record_from_frame`: Looks up a frame entry in a per-record list by matching an id byte, falling back to the list head, and copies its dimensions and offsets into the record.
- `jump_to__ownblitter`: Jump stub: passes control straight on to _OwnBlitter.
- `jump_to__disownblitter`: Jump stub: passes control straight on to _DisownBlitter.
- `tally_side_counts_and_start`: Reads two side values from global state and loops five times, incrementing a per-side counter when a routine returns one and installing a callback on each pass.
- `initialize_display_and_run_loop`: Sets per-side colour and position values, initializes nine entity slots and writes the DMA control register to enable DMA channels, then runs a loop that updates a frame counter and uses graphics stubs to set a pen and fill a rectangle.
- `select_frame_for_state`: Chooses an animation state for an entity slot from a routine result and the side it belongs to, unless a status byte is already set.
- `set_default_frame_values`: Sets the state word to twelve and a second value to zero, writes two fixed frame values into the global frame table, and stores the state in two frame fields and a status byte.
- `initialize_frame_data`: Writes several constant values to various offsets in a structure and copies a byte from one location to two others.
- `shrink_packed_nibble_values`: Splits a 16-bit packed value into a high byte and two nibbles, reduces each field by two with a floor at zero, and repacks the results into the same value.
- `run_fifty_step_update_loop`: Sets starting values in a group of global position and frame fields, then runs a fifty step loop.
- `jump_to__rectfill`: Jump stub: passes control straight on to _RectFill.
- `allocate_display_structures`: Allocates view, viewport and raster port structures, colour lists and bitmap or raster buffers for one or two buffers, using jump stubs to initialize each graphics structure.
- `free_display_structures`: Releases the display structures created for one or two buffers, freeing their colour map, viewport copper lists and copper list entries through jump stubs.
- `allocate_raster_buffers`: Initializes a bitmap structure, then allocates the requested number of raster buffers and stores each pointer in the structure.
- `free_raster_buffers`: Frees each raster buffer in the structure using its stored width and height, then clears its pointer slots.
- `switch_active_display_buffer`: Makes the buffer selected by a stored index the active one by pointing the view and colour-list fields at its entries.
- `parse_iff_style_chunks`: Reads tagged chunks from a file in a loop and dispatches on each four-character tag: DELT unpacks run-length delta data, CMAP converts three-byte colour entries to 16-bit words, and BMHD reads a 20-byte header.
- `reset_index_range_and_call`: Sets a start and an end counter to 256 and 257, then calls a routine with a zero argument.
- `read_buffered_bytes`: Copies up to a requested number of bytes from an internal 256-byte buffer into the caller output, refilling the buffer through a routine when it runs out.
- `jump_to__initrastport`: Jump stub: passes control straight on to _InitRastPort.
- `jump_to__freecolormap`: Jump stub: passes control straight on to _FreeColorMap.
- `jump_to__freevportcoplists`: Jump stub: passes control straight on to _FreeVPortCopLists.
- `call_load_rgb4_stub`: Calls the graphics library routine that loads RGB colour values, by jumping through its library vector.
- `call_wait_for_frame_stub`: Calls the graphics library routine that waits for the top of the next display frame, through its library vector.
- `call_make_viewport_stub`: Calls the graphics library routine that builds a viewport, through its library vector.
- `jump_to__freecprlist`: Jump stub: passes control straight on to _FreeCprList.
- `call_merge_copper_lists_stub`: Calls the graphics library routine that merges copper lists, through its library vector.
- `call_load_view_stub`: Calls the graphics library routine that loads a view, through its library vector.
- `jump_to__initvport`: Jump stub: passes control straight on to _InitVPort.
- `jump_to__getcolormap`: Jump stub: passes control straight on to _GetColorMap.
- `jump_to__freeraster`: Jump stub: passes control straight on to _FreeRaster.
- `jump_to__initview`: Jump stub: passes control straight on to _InitView.
- `free_bucket_lists`: Walks 71 bucket lists of linked entries and, when the second argument is nonzero, frees each entry and clears each bucket head.
- `set_record_flag_bytes`: When the record pointer is set, it stores a nonnegative second argument and a nonnegative third argument into two byte fields of the record.
- `allocate_record_if_missing`: When a record pointer is missing, it allocates a new record and, when a flag bit is set, one or two extra blocks whose pointers it stores in the record.
- `free_record_if_present`: When a record pointer is set, it frees the record optional blocks through a routine, releases the record itself, and clears the pointer.
- `build_records_from_count`: Gets a count from a routine and returns one if it is below one.
- `load_records_from_stream`: Reads records from a stream until the FINE tag appears, allocating a record for each and filling its position, size, depth and frame fields.
- `copy_record_buckets`: Copies 71 bucket list heads from one structure to another, copies a head field, and sets a flag byte and bit in the destination.
- `load_bitmap_records_from_stream`: Reads records from a stream until the FINE tag appears, allocating a record for each and initializing a bitmap and raster buffer from the record dimensions through jump stubs.
- `free_pointer_list_entries`: Walks a zero-terminated array of pointers, passing each entry to a routine with a value from the owner structure, freeing it with a fixed size and clearing its slot.
- `jump_to__initbitmap`: Jump stub: passes control straight on to _InitBitMap.
- `jump_to__allocraster`: Jump stub: passes control straight on to _AllocRaster.
- `read_joystick_and_mouse_ports`: Shifts the previous four state bytes along, then decodes the joystick and mouse port data registers and the fire button register into direction and button state for each port.
- `wait_for_blitter_idle`: Polls the DMA control read register until the blitter busy flag clears, then returns.
- `poll_blitter_busy_register`: Polls a hardware status register through an address register until the blitter busy flag clears, then returns.
- `decode_blitter_command_list`: Walks a list of two-byte command pairs, using the first byte to choose one of three modes that store a blitter setting and line count into a per-blit record, and for the data-carrying mode it also records the source address and advances past the data.
- `tick_dma_channel_countdowns`: Calls an optional callback, then ticks four countdown counters.
- `store_base_pointer_global_2`: Stores the current base register value into a global variable slot.
- `build_audio_sample_pointers`: Walks a table of byte-coded entries to find a boundary, then fills 30 pointer slots with offsets derived from a size field.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `update_audio_channel_playback`: Runs once per frame to advance the audio channels: on every sixth tick it selects the next sample position, updates the channel location registers, enables the channels and wraps the sample index, and on other ticks it applies volume or period changes from each channel effect code.
- `apply_audio_channel_effect`: Applies the current effect code to one audio channel.
- `parse_audio_track_command`: Reads one audio track entry, selects its sample pointer and length from a table, and stores the channel state.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `allocate_block_with_header`: Calls a routine to get a size and returns null when it is below one.
- `process_block_if_nonempty`: When the block pointer is set and its first field is nonzero, it passes the block and an offset from that field to a routine.
- `install_interrupt_server`: Allocates an interrupt node, fills in its type, priority, name and handler fields, and registers it with the system interrupt server list through a jump stub.
- `set_audio_request_pointer`: When the first value of its argument is nonzero, it stores a pointer to the following data in a global slot, calls a routine, and sets a flag to one.
- `stop_audio_and_clear_flag`: Clears a global flag, then calls a routine that silences the four audio channels and clears their DMA enable bits.
- `set_audio_channel_word`: Stores a 16-bit value in a per-channel slot of the audio channel table, where the channel index is the first argument.
- `set_audio_channel_period`: Gets a value from a routine and writes it to the period register of the audio channel named by the argument.
- `start_audio_channel_sample`: Starts playback on one audio channel.
- `mark_audio_channel_active`: Sets the per-channel flag for the given channel to one.
- `call_free_mem_wrapper`: Calls a function pointer stored in a global variable to release memory.
- `jump_to__addintserver`: Jump stub: passes control straight on to _AddIntServer.
- `call_alloc_mem_wrapper`: Calls a function pointer stored in a global variable to allocate memory.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `return_without_action`: Returns immediately without doing anything.
- `play_audio_entry_if_found`: Looks up an entry through a routine and returns minus one if it is missing.
- `parse_args_and_open_console`: Splits a command-line string into up to 32 arguments, treating double-quoted text as one argument, and fetches the DOS input and output handles and the current task.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `setup_display_objects`: Creates an object through a routine, then calls setup routines that configure its size, frame, and display position values.
- `release_active_handle_entries`: Walks the handle table from its last index downward and, for each active entry without a flag bit set, calls a routine on the stored value.
- `shutdown_io_and_callbacks`: Runs an optional callback, then calls library routines to shut down the input and output state.
- `append_bounded_string`: Appends up to a limit of characters from the source string to the end of the destination string, writes a terminating zero, and returns the destination.
- `calculate_string_length`: Returns the number of bytes before the zero terminator of a byte string, by scanning forward from the start pointer and taking the distance to the terminator.
- `call_entry_routine_or_fail`: Looks up an entry through a routine and returns minus one when it is missing.
- `open_handle_in_free_slot`: Finds the first free slot in the handle table from index three upward, converts the access bits into a mode value and opens a file for it, retrying with an alternate mode if the first open fails.
- `free_linked_node_list`: Walks a singly linked list of nodes, releasing each node through the system memory free routine.
- `process_audio_lookup`: Calls another routine to retrieve a pointer, then uses that pointer to call a second routine while checking a status flag in a global structure.
- `process_pending_entry_buffers`: Walks a list of entries and, for each one that is active, writable and has unsent data, calls a routine to write the pending bytes from its buffer.
- `write_bytes_and_record_error`: Clears the last error code, runs an optional flush routine, then calls the DOS write routine.
- `seek_file_position`: Clears the error code, optionally runs a flush routine, and seeks in the open file through the DOS seek routine.
- `read_bytes_and_record_error`: Clears the last error code, optionally runs a flush routine, then calls the DOS read routine.
- `open_file_or_record_error`: Clears the last error code, optionally runs a flush routine, then opens a file through the DOS open routine.
- `probe_file_lock_or_open`: Clears the last error code, optionally runs a flush routine, then tries to lock the file: a successful lock is released and the routine returns minus one.
- `delete_existing_file_then_open`: Clears the last error code, optionally runs a flush routine, then locks the file, releasing a successful lock and deleting the file before the open routine is called.
- `close_file_after_flush`: Runs an optional flush routine, closes the open file through the DOS close routine and returns zero.
- `compute_fixed_point_product`: Multiplies two 32-bit fixed-point values from registers and returns the product in the same fixed-point format.
- `divide_fixed_point_values`: Divides one 32-bit fixed-point value by another, handling signs separately, and returns the quotient in the same format.
- `divide_32bit_normalized`: Divides a 32-bit dividend by a 32-bit divisor after shifting both operands so the divisor is normalized, estimating the quotient with a 16-bit division and correcting it.
- `copy_capped_text_and_dispatch`: Copies up to 79 characters from a string held behind a global pointer into a local buffer and terminates it.
- `copy_text_and_dispatch_prompted`: Copies a caller-supplied string into a local buffer and terminates it at the copied length.
- `open_intuition_library_on_demand`: Opens the intuition library the first time it is needed by storing the returned base in a global slot while that slot is empty.
- `return_zero_constant`: Returns zero immediately, with no other operations or memory accesses.
- `clear_table_entry_and_release`: Looks up a table entry through an unnamed routine and returns -1 when none exists.
- `lookup_table_slot_by_index`: Clears a status field, then checks that the index is within the table count and that its slot is in use.
- `check_break_signal_pending`: Reads the task's signal mask and tests the two break-signal bits.
- `invoke_intuition_auto_request`: Calls the intuition AutoRequest routine through the intuition library base stored in a global slot, passing no arguments of its own.
- `_AddIntServer`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `_AllocMem`: Calls a function pointer stored at a fixed offset from a global variable to allocate memory.
- `_FreeMem`: Calls a function pointer stored at a fixed offset from a global variable to release memory.
- `_AvailMem`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `_RemTask`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `_FindTask`: Calls a function pointer stored at a fixed offset from a global variable to locate a task.
- `_OpenLibrary`: Calls a function pointer stored at a fixed offset from a global variable to open a library.
- `_Text`: Calls a function pointer stored at a fixed offset from a global variable to handle text operations.
- `_LoadRGB4`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `_InitRastPort`: Loads an address from a global variable, adds a constant offset, dereferences it as a function pointer and calls the pointed-to routine.
- `_InitVPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `_MrgCop`: Calls a function pointer stored at a fixed offset from a global variable to merge copper lists.
- `_MakeVPort`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `_LoadView`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `_SetRast`: Library stub: loads a library base from a global variable and calls the library function at offset minus 234 of that base, returning its result to the caller.
- `_Move`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `_Draw`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `_WaitTOF`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `_RectFill`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `_SetAPen`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `_SetBPen`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `_SetDrMd`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `_InitView`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `_VBeamPos`: Calls a function pointer stored at a fixed offset from a global variable to retrieve the vertical beam position.
- `_InitBitMap`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `_WaitBOVP`: Library stub: loads a library base from a global variable and calls the library function at offset minus 402 of that base, returning its result to the caller.
- `_OwnBlitter`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `_DisownBlitter`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `_AllocRaster`: Library stub: loads a library base from a global variable and calls the library function at offset minus 492 of that base, returning its result to the caller.
- `_FreeRaster`: Calls a function pointer stored at a fixed offset from a global variable to free a raster port.
- `_FreeVPortCopLists`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `_FreeCprList`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `_GetColorMap`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `_FreeColorMap`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `_BltBitMapRastPort`: Calls a function pointer stored at a fixed offset from a global variable to perform a bitmap bitplane transfer to a raster port.

### World Cup 90.adf:show

- `start_program_open_dos_handles`: Program entry point that finds the current task, opens the DOS library and saves its standard input and output handles, then opens further libraries and runs an unnamed routine.
- `write_nonempty_string_to_output`: When its nonzero argument is set, measures a null-terminated string and writes it through the DOS Write routine to the saved output handle, skipping empty strings.
- `verify_file_read_length`: Locks a file and examines its information block, then reopens the file and reads from it, comparing the bytes read against an expected length taken from a structure.
- `allocate_block_with_size_header`: Allocates a memory block through the system allocator and, if the allocation succeeds, writes the first argument plus four into the block's first longword.
- `free_block_if_nonnull`: Frees the memory block at the pointer argument through the system FreeMem routine, doing nothing when the pointer is null.
- `terminate_string_at_length`: When a task field is set, saves the buffer pointer in a global slot and writes a null terminator at the position given by the length argument.
- `parse_next_argument_token`: Advances a global argument pointer past spaces, then extracts the next token, accepting quoted strings and stopping at a space or comma.
- `call_set_signal_passthrough`: Calls the system SetSignal routine through the library base and returns the register values it was called with.
- `measure_c_string_length`: Counts the characters in a null-terminated string and returns the count, not including the terminator.
- `stub_bad_instruction_data`: Only one instruction decodes at this address, and the pseudo-code shown is the body of a larger picture loader that begins at a different address.
- `parse_resource_file`: Parses a structured data file by searching for specific four-byte magic markers to identify different data blocks.
- `parse_resource_file_2`: Parses a structured data file by searching for specific four-byte magic markers to identify different data blocks.
- `parse_ihdr_resource_file`: Parses an IFF-formatted resource file by locating specific chunks such as IMAG, CMAP, and BODY. It extracts metadata, color palettes, and pixel data to populate global structures for image rendering.
- `parse_picture_file_format`: Parses a custom image file format by searching for specific magic markers and reading header information.
- `parse_resource_data_block`: Parses a structured data block by searching for specific magic markers and extracting associated metadata.
- `parse_resource_chunk`: Parses a structured data block by searching for specific four-byte magic identifiers.
- `parse_resource_chunk_2`: Parses a structured data block by searching for specific four-byte magic markers to identify different resource segments.
- `parse_resource_file_3`: Parses a structured data file by searching for specific four-byte magic markers to identify different data blocks.
- `parse_amiga_disk_image`: Parses a custom disk image format by searching for specific magic markers.
- `load_loadscreen_picture_file`: Takes a picture filename from the command line and shows a usage message when it is missing.

### World Cup 90.adf:wait

- `build_pattern_table_from_list`: Finds the largest value stored in a linked list, then fills a block with an incrementing sequence of words and stores a pointer to it in a stack-relative structure.
- `copy_sixteen_longs_then_call_2`: Copies sixteen 32-bit values from a fixed offset in a structure into a destination buffer when one is provided, then calls an unnamed routine through a stored pointer.

### World Cup 90.adf:SLIPCHECK

- `load_executable_hunks`: Reads an AmigaDOS executable's hunk records, allocating code and data memory through the exec allocator and freeing it on the exit path.
- `stub_empty_return_routine`: This routine is empty and contains only a return instruction.
- `burn_delay_cycles`: Counts a register down to zero in a tight loop with no other effect and then returns, so it only wastes time.
- `tiny_ori_2`: Small routine of 1 instruction that merges 160 into d0.
- `decompress_back_reference_stream`: Decodes a packed byte stream by reading control values that either copy literal bytes or repeat earlier output with a length and offset from the stream.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_cycle_counter`: Decrements a byte register once for each count unit, wrapping to a space character when it reaches zero, and returns.
- `stub_bad_instruction_data_2`: Decodes into bad instruction data that the decompiler truncated immediately, so it has no usable behaviour.

### World Cup 90.adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `store_byte_to_global`: Stores the low byte of its register argument into a global variable and returns.
