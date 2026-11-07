# Game 1890 (future wars - time travellers (1989)(u. s. gold)[cr avenger][m maimex]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01890). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802448 top-level bytes in 33 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 3 raw-word lines |
| Functions named and described | pass, 461 of 461 named (0 author symbols), 461 of 461 described (53 by rule, 21 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4224 of 4224 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16605 of 16605 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1551737 |
| code | 56640 |
| data | 145255 |
| padding | 7344 |
| unused | 41472 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:run | 6 | 29 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:scr1 | 2 | 90 | 3 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:delphine | 3 | 15871 | 427 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:AVENGER | 2 | 83 | 3 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:wait | 2 | 117 | 2 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:NoFastMem | 9 | 307 | 14 |
| Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:bootblock | 1 | 100 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- future wars - time travellers (1989)(u. s. gold)[cr avenger][m maimex](disk 2 of 2)/Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 2 of 2).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:run

- `run_entry_free_memory`: Program entry point that runs a short startup sequence: a four-pass counted loop calls a routine that invokes a system library function, then it clears a stored pointer and returns two register values.
- `decrement_until_minus_one`: Busy-waits by decrementing a 16-bit counter until it wraps to minus one, then returns.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:scr1

- `wait_while_flag_set`: Runs a loop that waits while a flag bit in a global variable stays set, polling a beam-position routine each pass, and sets a status word on exit.
- `open_graphics_and_init_bitmap`: Opens the graphics library, initialises a bitmap and a rendering port, then fills a table of pointers and stores two configuration words.
- `scroll_text_row_buffer`: Decrements a frame counter and, when it reaches zero, resets it and advances a buffer index, then copies rows of a buffer to scroll displayed text.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:delphine

- `clear_memory_and_init`: Clears a block of memory by writing zero longwords in a counted loop, stores the system library base and an opened-library value into globals, and calls a library function only when a flag bit is set.
- `test_flags_and_store`: When the overflow and negative flags differ and the carry flag is set, returns the incoming byte ORed with a constant.
- `set_flag_bit_two`: Sets bit two in a global variable and returns, leaving the other bits unchanged.
- `read_input_flags`: Returns a byte built from two bits of a global variable: bit six clear gives one, and bit four clear adds two.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `disable_several_interrupts`: Writes a constant to the interrupt control register with the top bit clear, which disables several interrupt sources at once.
- `enable_several_interrupts`: Writes a constant to the interrupt control register with the top bit set, which enables several interrupt sources at once.
- `disable_ports_interrupt`: Writes a constant to the interrupt control register with the top bit clear, which disables the ports interrupt source only.
- `update_pseudo_random_state`: Advances a 32-bit state stored behind a pointer by doubling it and XORing in a fixed constant when the shifted-out bit is set or the result is zero.
- `read_keyboard_serial_byte`: When a flag bit in a global variable is set, returns the bitwise complement of that variable rotated by one bit, waits in a short countdown loop, then clears the flag.
- `lookup_code_in_table`: Calls a helper that returns a 16-bit code; if the code is nonzero and lacks bit seven, it scans a table of 49 entries for a matching code and returns the paired value, otherwise it returns zero.
- `initialize_table_entries`: Copies a 180-word template into the buffer addressed by a global pointer, then writes the masked high and low halves of the two parameters and of several offset values at fixed positions, with a loop filling seven more shifted entries.
- `write_structure_constants`: Writes thirteen fixed 16-bit constants into a structure pointed to by a global variable, each at its own fixed offset.
- `store_parameter_and_control_words`: Writes a fixed control word, stores the 32-bit parameter and a zero word at fixed offsets in a structure pointed to by a global, then writes a second control word.
- `read_global_pointer`: Returns the 32-bit value stored at a fixed offset from the global data base, exposing a global pointer to the caller.
- `update_display_plane_pointers`: Stores the first parameter as a global pointer unless it is minus one.
- `copy_sixteen_words_into_structure`: Copies sixteen 16-bit words from a source buffer passed as a parameter into alternate word slots of a structure referenced by a global pointer.
- `configure_blitter_parameters`: Stores eleven parameters at fixed offsets of a structure referenced by a global, combining two of the values into one word, then calls a helper that waits for the blitter.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `empty_routine`: The routine contains no instructions and performs no operations, so it is described as empty without guessing its use.
- `invoke_helper_routines`: Calls three helper routines in sequence: the first with a computed address parameter, the second with two all-ones arguments, and the third with no arguments.
- `query_and_process_system_value`: It queries the system for a value and, when that value is non-zero, passes it to another routine along with a stored pointer, then invokes that routine again with the same value.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d5.
- `read_volume_info_word`: Locks the volume named by a text string, reads the field at offset four of its information record, unlocks it and returns the low word of that field.
- `negate_and_call_routines`: Negates one register and calls four helper routines.
- `tiny_neg_move`: Small routine of 2 instructions that negates d6, then copies d0 to d0.
- `wrap_single_helper_call`: Calls one other helper routine.
- `initialize_game_tables_and_state`: Allocates memory blocks, sets raster timing values, and clears several global tables in counted loops, releasing any entries still allocated and then calling raster and library helpers.
- `empty_return_routine`: This routine contains no instructions and performs no operations, returning immediately to its caller.
- `swap_and_raster_wait`: Swaps two 32-bit globals, calls raster timing helpers and a helper that copies word tables, then writes a word to a structure, choosing one of two constants based on the parameter, and conditionally calls a further helper.
- `set_raster_constant`: Calls a raster wait helper and then stores a fixed constant word at a fixed offset of a structure referenced by a global.
- `clear_table_entries_loop`: Walks 255 entries of a table; for each non-empty entry it calls a helper with the product of two stored sizes, then clears the entry and sets two of its words to all ones.
- `allocate_table_entry`: Finds the first empty slot among 255 entries and allocates a block sized from the parameter, stores the size fields and marks the slot unused, then increments a count.
- `free_table_entry`: If the given slot is in use, frees its block through a helper, clears its size fields, marks it unused, and decrements a positive count.
- `repeat_helper_call_count`: Calls a helper once per iteration until the count parameter is reached, performing no other work.
- `clear_two_global_longs`: Stores zero into two 32-bit locations referenced through a global base pointer and returns.
- `free_linked_list_nodes`: Walks a singly linked list held in a global variable, calling a deallocation helper with a 20-byte size for each node, then calls a final cleanup helper.
- `insert_sorted_list_node`: Allocates a 20-byte node and links it into a global list in key order, comparing keys looked up through a table, then stores two fields from the parameter in the node.
- `remove_matching_list_node`: Searches the global list for a node whose two stored fields match the parameter.
- `clear_two_global_longs_2`: Writes zero to two adjacent 32-bit global variables and returns.
- `free_linked_list_nodes_2`: Walks a linked list held in a global variable, calling a deallocation helper with a 20-byte size on each node, then calls a final cleanup helper.
- `append_list_node_from_table`: Allocates a node, copies four words from a table entry indexed by the parameter into the node, and links the node at the end of a global list, recording the parameter in it.
- `prepend_node_to_list`: Allocates a 20-byte node, links it at the front of a list held in a global variable, copies two parameter fields and four words from a table entry indexed by the parameter.
- `clear_global_word_and_flag`: Writes zero to a 32-bit global variable and an all-ones value to a 16-bit global variable, then returns.
- `walk_list_and_clear_flag`: Walks a linked list held in a global variable, calling a deallocation helper with a 214-byte size on each node, then calls a final cleanup helper.
- `append_node_to_tail`: Finds the tail of a global list, allocates a 214-byte node and links it at the end.
- `mark_matching_node_removed`: Searches a global list for a node whose 16-bit field equals the parameter and sets that field to minus one.
- `remove_dead_nodes_from_list`: Walks a global list and unlinks each node whose 16-bit field is minus one, deallocating each removed node through a helper with a 214-byte size.
- `clear_two_globals`: The routine writes zero to a global variable and writes a word with all bits set to another global variable, then returns.
- `walk_list_freeing_nodes`: Walks a linked list held in a global variable, calling a deallocation helper with a 214-byte size on each node, then calls a final helper.
- `append_cleared_node_to_tail`: Walks to the tail of a global list, allocates a 214-byte node and links it at the end, zeroes two 50-word arrays inside it, stores the parameter and a table pointer, then calls a helper with that table pointer and the node's index field.
- `find_and_mark_list_entry`: The routine walks a linked list comparing a stored word at each node to a passed value, and when it finds a match it overwrites that word with all bits set before returning zero, otherwise it returns a non-zero value.
- `self_referential_table_chase`: Walks a singly linked list whose head pointer comes from a global variable offset, and for each node whose short field at a fixed offset equals negative one it rewires the previous link, calls another routine with the node and a constant argument, and follows the node's own link, otherwise advancing to the next node.
- `dispatch_node_with_table_args`: Reads fields from a table entry and from a node slot.
- `dispatch_with_computed_args`: Calls a helper with arguments taken from stack-frame values and a global pointer, masked and combined, then calls a helper with zero.
- `store_parameters_to_globals`: Copies the low and high words of two parameters into four globals, then stores two derived values: a scaled sum of the first parameter's words, and 160 minus eight times the second parameter's low word.
- `call_helper_with_two_pointers`: Loads two 32-bit pointers from globals, calls a helper once with them, then returns.
- `call_pair_with_region_check`: Compares three stored 16-bit values with zero, 160 and 200.
- `call_helper_twice_with_args`: Calls one helper twice with the same global pointer and word arguments, changing only the second pointer loaded from a global, then returns.
- `return_immediately`: The routine executes three instructions and then returns without modifying any data or registers.
- `store_table_field_by_selector`: Stores the low word of the second parameter into one of six fields of an entry in a 32-byte-stride table, with the field and entry chosen by the parameters.
- `store_table_field_from_frame`: Stores a word taken from the caller's frame into the first field of a table entry whose index also comes from the caller's frame.
- `copy_word_via_index`: Reads a 16-bit index from the frame, scales it into a table offset, and copies a 16-bit value from the caller's frame into the sixth field of that table entry.
- `store_table_entries_and_cleanup`: Stores four 16-bit values from three parameters into a table entry indexed by the first parameter's low word, then calls a helper with zero and calls it again when that result is zero.
- `compute_bitmask_from_table`: Computes an address from its parameters, reads four consecutive 16-bit words there, tests one bit position in each, and returns a four-bit mask built from the results.
- `test_bit_in_table`: Reads a 16-bit word from a table address computed from the parameters and returns whether the bit at a position given by the low nibble of the first parameter is set.
- `initialize_and_run_main_loop`: Clears several global variables and tables, sets raster timing values, then loops while waiting for raster lines, polling input and calling helpers until a termination flag is set.
- `process_input_flags_and_dialog`: Reads input flags over several passes and accumulates the button bits.
- `handle_dialog_choice_cases`: Skips its work when a state flag is set.
- `find_entry_in_table`: Scans up to 255 twelve-byte table entries for the first active one whose stored fields match the first parameter's low word and the second parameter's first word.
- `find_hit_entry_in_list`: Walks a linked list of entries, skipping inactive ones, and returns the index of the first active entry whose rectangle contains the point given by the parameter.
- `process_display_list`: Walks a linked list of display entries and dispatches on each entry's type.
- `draw_type_five_entries`: Walks a linked list and, for each entry of type five, draws its table-selected image through a helper at a position offset from the parameter.
- `draw_type_five_entries_masked`: Walks a linked list and, for each entry of type five, draws its table-selected image through a helper at an offset position, passing masked control words.
- `visit_each_node_in_list`: Walks the linked list held in one global variable and calls a helper with each node pointer in turn.
- `visit_each_node_in_list_2`: Walks the linked list held in a second global variable and calls a helper with each node pointer in turn.
- `interpret_entry_bytecode`: Runs a byte-coded script attached to an entry structure, reading opcodes until a stop flag is set.
- `run_script_opcode_stream`: Reads a byte-coded opcode stream through a jump table until a stop flag is set.
- `run_script_opcode_stream_short`: Reads a byte-coded opcode stream through a jump table until a stop flag is set, with the same opcode set as its sibling but a different entry offset.
- `run_script_pair_call`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_pair_call_alt`: Interprets a byte-coded opcode stream with a nine-byte entry instruction that reads a byte index and four 16-bit operands and calls a helper with two of them.
- `run_script_store_fields`: Interprets a byte-coded opcode stream.
- `run_script_accumulate_field`: Interprets a byte-coded opcode stream.
- `run_script_add_word`: Interprets a byte-coded opcode stream.
- `run_script_copy_to_buffer`: Interprets a byte-coded opcode stream.
- `run_script_call_into_slot`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_call_zero`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_set_variable`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_call_zero_alt`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_three_calls`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_call_one`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_record_position`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_clear_position`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_conditional_jump`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_jump_unless_flag`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_call_with_operand`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_call_no_args`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_call_plain_alt`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_read_string_call`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_load_animation`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_store_named_entry`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_load_background`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_load_asset_name`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_load_named_resource`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_call_three_helpers`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_call_once`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_set_flag_one`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_save_position_stop`: Interprets a byte-coded opcode stream.
- `run_script_set_other_flag`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_clear_other_flag`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_choose_call_stop`: Interprets a byte-coded opcode stream.
- `scan_script_labels`: Fills fifty entries of a label table with minus one, then walks the byte-coded script opcode by opcode, advancing by each opcode's operand length and recording the stream position for label opcodes.
- `run_script_read_word_operand`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_read_byte_operands`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_read_word_alt`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_read_four_words`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_store_word_byte`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_read_index_byte`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_record_label_position`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_read_index_alt`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_read_byte_pair`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_read_single_byte`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_read_index_word`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_read_string_operand`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_no_op_entry`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_stop_immediately`: Interprets a byte-coded opcode stream.
- `run_script_no_op_alt`: Interprets a byte-coded opcode stream until a stop flag is set.
- `run_script_read_eight_bytes`: Interprets a byte-coded opcode stream until a stop flag is set.
- `compare_low_high_words`: Compares the two 16-bit halves of its 32-bit parameter and returns a flag byte: bit zero is set when the halves are equal, bit one when the second half is less than the first, and bit two when the first half is less than the second.
- `clip_blit_rectangle`: Clips a horizontal span against a 160-unit-wide area, moving the source pointer and reducing the column count for any left overflow, then clips the vertical range and passes the adjusted values to helper routines.
- `return_argument_unchanged`: Returns its single argument unchanged and calls nothing.
- `clip_blit_rectangle_copy`: Clips a horizontal span against a 160-unit-wide area in the same way as its sibling, but through a different entry point, then passes the adjusted values to helper routines.
- `return_parameter_unchanged`: The routine receives a single value in a register and returns it unchanged, performing no operations on the value and containing no calls to other routines.
- `clip_blit_rectangle_wide`: Clips a horizontal span against a 160-unit-wide area, moving the source pointer by a quarter of the overflow rather than an eighth, then clips the vertical range and passes the adjusted values to helper routines.
- `return_parameter_unchanged_2`: This routine takes a single value passed in a register and returns it unchanged, performing no operations on it and making no calls to other routines.
- `configure_blit_single`: Calls a helper once with the first parameter as one pointer, the second as another pointer, zero offsets, an all-ones mask word and a constant of 800.
- `configure_four_plane_offsets`: Calls a helper four times with the same fixed arguments, passing the parameter as the pointer for the first call and the parameter plus 8000, 16000 and 24000 for the other three.
- `noop_three_instr`: It contains three instructions that decode as code but perform no observable action on memory, registers, or hardware.
- `call_four_planes_with_flags`: Gets a global pointer, derives four flag words from bits 16 to 19 of the third parameter, and calls a helper four times with that pointer offset by 0, 8000, 16000 and 24000 bytes, passing one flag word each time.
- `build_plane_blit_parameters`: Computes a column count, a shift mask and a row count from two packed corner parameters, sets four plane-select words to one of two constants depending on flag bits 16 to 19, and passes them with a base address to helper routines.
- `configure_blit_from_rect`: Computes a row count and a base address from a global pointer and two packed corner parameters, then calls a helper once with the base, the row count, a bit mask and a width.
- `call_one_helper_undecoded`: The decompiler timed out, so only the call structure is known: it calls one other routine and has 76 instructions.
- `shift_eight_word_rows`: Repeats eight times over two pointers computed from the parameters: it reads a 16-bit word, shifts it by an amount taken from a nibble of the parameter, and writes an inverted combined 32-bit value to the destination row, which is 40 bytes further on each pass.
- `process_undecoded_routine`: The decompiler timed out, so only its size of 81 instructions and its four callers are known; its purpose is not stated.
- `draw_text_line_wrapped`: Draws one text line taken from a string table entry.
- `append_node_to_list_tail`: Walks a global list to its last node, allocates a 20-byte node through a helper and links it at the end of the list.
- `prepend_typed_list_node`: Allocates a 20-byte node and links it at the front of a list held through a stack pointer, setting its type field to two and copying an operand from the parameter into it.
- `append_node_with_fields`: Walks a list held in a global variable to its last node, allocates a new node through a helper, links it in and copies fields into it.
- `draw_text_string_glyphs`: Calls several helper routines with coordinates offset by one and eleven units from the parameter, then draws each non-space character of a string through a glyph table entry held in a global, using a helper call per glyph.
- `render_text_line`: Computes a line width from a string length, centers the line vertically, and walks its characters, skipping leading spaces.
- `render_text_block_synced`: Computes the size and position of a text block from its parameters, clamps the block to the 320-by-200 screen, and draws its characters through helper calls, with raster waits between steps.
- `render_character_cells`: Clips a character cell's position against screen bounds, then walks each cell's bytes, skipping spaces and drawing non-space characters using a lookup table to place them on the display.
- `prepare_text_block_blits`: Clamps the height of a text block to the screen, then issues blitter configuration calls that set up copies and masks for each character.
- `build_string_index_table`: Clears a fixed-size area in a global structure, scans a table of strings for entries with a specific marker, copies the matching strings into the cleared area, and stores their indices in another global area.
- `allocate_text_slot_or_fail`: Returns minus two when a helper reports no room.
- `poll_text_slot_until_clear`: Returns minus two when a helper reports no slot.
- `scale_and_clamp_fields`: Doubles several fields of the two 32-bit parameters, then computes three derived values from the first parameter using nibble extraction and clamping with the second parameter, updating intermediate variables as it goes.
- `process_word_table_loop`: Loops sixteen times over sixteen words of a global area, passing each through a scaling and clamping helper and storing the result back.
- `process_word_table_reverse`: Loops sixteen times in reverse order over sixteen words of a global area, passing each through a scaling and clamping helper with negated arguments and storing the result back.
- `fill_word_range_from_helper`: Loops from the first to the second word of the first parameter, calling a helper with two words on each pass and storing each result in a 16-bit table entry indexed by the loop value.
- `fill_word_table_from_frame`: Loops over an index range taken from the stack frame, calling a helper with two frame words on each pass and storing each result in a 16-bit table entry.
- `load_database_records`: Clears a 255-entry table of 30-byte records, then opens and reads the named database file through the file read wrapper and helper calls.
- `clear_record_table_fields`: Clears several fields in each of 255 thirty-byte records of a table held through a global variable, then calls two helpers.
- `copy_block_to_four_destinations`: Copies a block of words from a source pointer to four destination pointers, one copy for each destination, with the destinations spaced a fixed distance apart.
- `fill_byte_table_from_helper`: Returns minus one when a global value is negative.
- `seek_and_read_record`: Reads a record from an open file at an offset stored in a 30-byte database record: it seeks to that offset and reads the bytes through the read and seek wrappers.
- `copy_bytes_advancing_source`: Copies a given number of bytes from the buffer whose current pointer is stored behind the first parameter into the destination, then stores the advanced pointer back.
- `allocate_frame_buffer`: Allocates a frame buffer for a width and height pair, waits for a raster line and calls helpers.
- `expand_nibbles_to_bitflags`: Returns zero for a zero input.
- `process_indexed_entry_with_raster`: Looks up an index for its parameter through a helper and, when the index is valid, copies the parameter into a table entry.
- `lookup_index_and_call`: Looks up an index for its parameter through a helper and, when the index is valid, calls a second helper with a 32-byte stack buffer.
- `update_display_on_raster_match`: Compares two stored pointers and, when they are equal, calls helpers, waits for a raster line and calls further helpers with values from a global pointer.
- `find_period_in_name`: Scans a string up to its stored length for a period character, then continues with helper calls on the string's position.
- `advance_on_period_n_marker`: Checks whether a period in a string is followed by the letter N within the string length.
- `write_header_chunks_then_succeed`: Calls two helpers, one with a count of two and one with a count of 32, then stores a global value plus 34 into another global and returns one.
- `return_zero_status`: Returns zero and does nothing else.
- `read_dimensions_and_pixels`: Opens a file through a helper with a constant mode, reads two 16-bit values, then reads a block of bytes whose length is their product into the caller's buffer, and closes the file.
- `decode_packed_block`: Decodes a packed stream with a bit reader that refills from packed longwords and folds them into a checksum, writing the output through a pointer.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `shift_bit_reader_with_refill`: Shifts a 32-bit bit buffer right by a count taken from its second parameter, refilling the top bit from the next longword of the packed stream whenever the buffer empties.
- `update_frame_state_with_raster`: Updates a frame state through helper calls and raster waits.
- `process_indexed_resource_with_raster`: Looks up an index for its parameter through a helper and, when the index is valid, processes the resource through further helpers, with raster waits.
- `scan_records_with_raster_waits`: Loops over stored records, comparing values and calling helpers for each one, with raster waits between passes.
- `scan_records_with_table_updates`: Loops over a stored record list, updating a table counter and calling helpers for matching records.
- `check_four_locations_and_call`: Checks four byte locations relative to a global base and, for each non-zero one, calls helpers with the matching string offsets.
- `clean_up_global_data_tables`: Performs a series of clean-up helper calls on data referenced through a global base pointer, then calls a library function through its library vector.
- `read_directory_file_entry`: Fills two local buffers of 120 and 80 bytes through helper calls, then calls five helpers.
- `read_block_into_slot`: Reads a block of 200 bytes from a file through the read wrapper into a slot of a global area, closes the file and returns one.
- `restore_backup_or_report`: Copies an entry into a local buffer through helper calls.
- `read_table_entries_from_file`: Reads table entries through the read wrapper and processes them in a loop over a table held in a global area.
- `read_file_records_loop`: Loops from a counter up to a limit, calling a helper with a stored file handle on each pass.
- `read_and_cleanup_file_records`: The routine loops through file records, walking linked lists and calling a read wrapper to load a block, then checks a series of global variables and calls a cleanup routine for each one that is non-zero.
- `read_file_records_block`: Loops from a counter up to a limit, calling a helper with a stored file handle on each pass, then reads a block through the read wrapper and calls two further helpers.
- `call_handle_helpers_loop`: Loops from a counter up to a limit, calling a helper with a stored file handle on each pass.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `visit_each_node_in_list_3`: Walks the linked list held in a global variable node by node.
- `call_helper_per_list_node`: Walks a linked list held in a global variable and calls one helper for each node.
- `read_node_from_file`: Allocates a 214-byte node, links it into a buffer chain, reads data into it through the read wrapper and calls one further helper.
- `copy_table_fields_into_nodes`: Walks the list held in a global variable and copies a field from a table entry, selected by each node's index, into the node.
- `tiny_unlk_rts`: Small routine of 2 instructions that removes the stack frame held in a5, then returns.
- `call_helper_per_second_node`: Walks a second linked list held in a global variable and calls one helper for each node.
- `read_node_from_file_alt`: Allocates a 214-byte node, links it into a buffer chain, reads data into it through the read wrapper and calls one further helper, as its sibling does.
- `refresh_list_entries_from_table`: Walks a singly linked list headed by a global pointer.
- `return_without_action`: Contains only a return, so calling it does no work.
- `append_copied_record_to_list`: Allocates a twenty-byte record and copies the caller's buffer into it.
- `read_record_and_link`: Allocates a twenty-byte record and fills it by calling the file read wrapper with the caller's handle.
- `append_copy_to_second_list`: Allocates a twenty-byte record and copies the caller's buffer into it.
- `read_record_then_link_second`: Allocates a twenty-byte record, reads twenty bytes into it through the file read wrapper, and links it into a global list.
- `draw_slots_buffer_swap`: Loops over a table of up to 255 slot records and skips empty or inactive ones.
- `draw_slots_with_raster_wait`: Walks a table of slot records like the slot drawing loop, calling the raster wait routine during the pass.
- `draw_remaining_slot_records`: Continues the slot table drawing loop with a structure that reads one slot per pass.
- `draw_slot_records_by_index`: Draws active slot records from a table, advancing a slot index and a record index separately and placing each record's data at its slot position.
- `draw_slot_records_restart_scan`: Scans the slot table after each drawn record, drawing active entries and swapping the two screen buffer pointers at the threshold.
- `draw_slot_records_sequential`: Advances through the slot table one entry at a time, drawing active records at their positions and making the list-walk and blit calls for each.
- `draw_linked_sprite_list`: Walks a global linked list of draw nodes.
- `render_linked_slot_entries`: Walks a linked list whose entries each name a slot, reading the slot's buffer and position and passing them to the graphics routine.
- `render_linked_slot_entries_alt`: A second form of the entry list renderer that visits each node in a do-while pattern.
- `return_immediately_empty`: Contains only a return and performs no work.
- `prompt_insert_disk_message`: Fills two text buffers with the 'Please insert DISK , VOL., ' prompt and the disk and volume values read from a global table, draws them, then loops while a status query stays nonzero before returning.
- `wait_then_spin_forever`: Calls a routine repeatedly until a local word reads as zero, then enters an endless loop calling routines, so it never returns.
- `setup_blitter_and_wait`: Waits for a raster line, then loads blitter masks and values from a global structure, sets the modulo and size, and runs one blit of 200 rows at a 32000-byte offset.
- `draw_centered_text_box`: Splits its text at vertical bar separators to find the longest line, sizes a box around it, and centres the box on a 320-pixel line.
- `draw_centered_text_box_rows`: Measures the longest bar-separated line of its text and sizes and centres a box on a 320-pixel line.
- `draw_text_box_with_rules`: Sizes and centres a text box from the longest bar-separated line, draws it, and renders the characters row by row from a font table.
- `draw_text_box_after_raster`: Sets the box width from a row counter rather than from the text length, then waits for a raster line before drawing the box.
- `draw_text_box_from_string`: Draws a centred text box whose rows come from a second string argument, waiting on the blitter busy flag after each box operation.
- `draw_box_then_read_input`: Draws a text box sized from its string, renders the string with per-character glyphs, then draws a frame and an input line.
- `read_keyboard_text_input_line`: Reads keyboard codes in a loop and appends letters, digits and spaces to an input line, redrawing it with per-character glyphs.
- `read_text_input_with_cursor`: Reads keyboard codes into a text field and redraws the typed characters with glyphs from a font table, moving a cursor as the text changes.
- `read_input_after_raster_wait`: Waits for a raster line, clears two counters, then loops reading key codes into a text field.
- `handle_text_input_keystroke`: Handles one keystroke for a text field.
- `handle_input_with_raster`: Handles a keystroke for a text field while also waiting on the raster.
- `call_library_stub_and_return`: Calls a library stub through its base vector, then calls two further routines before returning.
- `update_offsets_from_slot_table`: Compares a stored slot position with the current position and adds the difference to two global offsets when they differ.
- `initialize_pair_table_structure`: Copies two globals into fixed words, writes two constant words at the start of a structure, and fills sixteen word pairs of the structure from a global table.
- `save_frame_write_interrupt_enable`: Stores the caller's frame value into a global, writes constants to other globals including an interrupt enable setting, and returns its two input registers unchanged.
- `enable_dma_and_interrupt_channels`: Writes a saved constant, an interrupt enable value and a DMA control value to hardware register globals.
- `loop_while_flag_set`: Checks a bit in a global variable and, while that bit is set, calls itself repeatedly, then enters an endless loop that never returns.
- `wait_vertical_retrace`: Waits for a bit in a global variable to clear, then enters an endless loop that never returns.
- `read_mouse_movement_deltas`: Reads the mouse port movement counters and computes the horizontal and vertical change since the previous reading.
- `accumulate_mouse_deltas_wrapped`: Applies the wrapped horizontal and vertical movement differences to the stored pointer coordinates and records the latest counter values.
- `clamp_pointer_set_copper`: Applies the mouse movement to the stored position, clamps it to a fixed screen limit, and writes the copper list pointer so the display uses the updated values.
- `setup_blitter_line_from_points`: Derives blitter line parameters from two endpoints, including the larger delta, the octant lookup and the error terms, and writes them to the blitter registers.
- `copy_length_prefixed_string`: Reads the length from the first byte of a source record, copies that many following bytes to the destination, and terminates the destination with a zero byte.
- `find_matching_name_in_list`: Walks a list of records from a global pointer, copies each candidate string into a local buffer and compares it with the input string.
- `find_trackdisk_device_entry`: Finds the trackdisk.device by name through the system library, then reads a flag byte from the structure at the entry indexed by the argument.
- `call_then_report_success`: Calls a routine and returns minus one if it gives zero.
- `poll_until_status_clears`: Repeatedly polls a status routine and checks bit one of its result.
- `poll_status_bit_with_retry`: Loops polling a status routine and tests bit one of the result.
- `wait_for_formatted_diskette_prompt`: Polls a status routine in a loop, waiting on the raster line between polls, and returns once the status clears.
- `wait_status_low_bits_clear`: Waits in three polling loops until the low two bits of a status routine's result are clear, then returns the high word of the final status.
- `poll_status_bits_in_sequence`: Performs several polling loops in sequence, each waiting until a status routine returns a result whose low two bits are clear.
- `poll_status_word_until_ready`: Polls a status routine until its low two bits are clear, then reads the status word and returns the masked result.
- `poll_status_return_high_word`: Polls a status routine until its low two bits are clear and returns the high word of the status.
- `poll_status_high_word`: Reads a status value and returns its high word once the low two bits are clear, polling again otherwise.
- `open_dos_and_read_file`: Waits on global flags, then opens dos.library, allocates memory, and opens and reads a file through dos library calls.
- `examine_and_lock_file`: Locks a file through the dos library, examines it, reads one field from its info block, then unlocks the file and frees the memory it allocated.
- `set_global_flag`: Sets one global flag variable to one and returns.
- `wait_until_flag_reaches_limit`: Sets a global flag to one if it is zero, then busy-waits until the flag reaches sixty-four.
- `initialize_ciaa_resource_tables`: Opens ciaa.resource through the system library, copies a few values from it into globals, and fills a table of fourteen computed entries.
- `shutdown_audio_if_active`: When a global flag is set, calls a library vector through a stored base and a routine that handles the audio channels, then clears the flag.
- `free_memory_if_allocated`: When two global pointers are both set, frees the memory block through the system library and clears one of the globals.
- `compute_max_table_value_scaled`: Scans a byte table located through a global variable, keeping the largest byte seen.
- `write_cia_timer_control`: Writes a fixed control value to the CIA A control register, then copies a stored value into the timer high and low registers through globals.
- `clear_pointer_table_entries`: Walks fourteen pointers stored in a table and writes zero to the memory each one refers to.
- `reset_audio_volumes_and_state`: Writes zero to the four audio volume registers and clears several global audio state variables, setting one global to all ones.
- `initialize_four_audio_channels`: Clears the audio volume registers, points each of the four audio channels at a shared buffer with a length of one, and stores a channel mask in a global.
- `call_and_return_zero`: Calls one routine and returns zero.
- `service_audio_channels_per_tick`: Counts ticks, and on most of them calls a routine for each of the four audio channel register blocks, spaced sixteen bytes apart.
- `call_and_pass_argument_through`: Calls one routine and returns the argument it was passed, unchanged.
- `write_four_audio_channel_registers`: Loops over the four audio channel register blocks, calling a routine with a decreasing channel index and stepping the register address sixteen bytes each pass.
- `apply_channel_slide_with_clamp`: Advances a channel's slide value by its per-tick step and clamps it to a limit, stopping at the upper or lower bound depending on the step's sign.
- `advance_value_clamped_to_limit`: Adds a step to a stored value and, when the result reaches a limit field, replaces it with that limit before storing it in the output record.
- `dispatch_channel_effect_command`: Reads the effect command nibble from a note record and dispatches it through a table of handler routines, applying a volume change when the nibble is two.
- `set_cia_port_bit_one`: Sets bit one of the stored CIA A port value, a port that holds the joystick fire buttons and disk status.
- `clear_cia_port_bit_one`: Clears bit one of the stored CIA A port value, a port that holds the joystick fire buttons and disk status.
- `load_channel_note_record`: Fills a channel record from a note entry, storing the effect nibble and the note value.
- `copy_word_with_signed_offset`: Copies one word from a table at an offset equal to the input value, doubled and negated when the flag is zero, into the output record.
- `store_word_into_record`: Stores the word passed in the data register into the output record at offset four and returns.
- `load_channel_note_with_sign`: Fills a channel record from a note entry, negating the effect nibble when the sign byte is set.
- `dispatch_effect_handler_by_counter`: Calls one of several effect handlers through a table, indexed by a tick counter that is decremented by one first.
- `lookup_table_column_by_index`: Searches a sentinel-terminated table of words for the matching key, then copies the entry at the given index of that row into the output record.
- `lookup_table_column_high_nibble`: Searches a word table for a key and copies the entry selected by the high nibble of a byte in the input record into the output record.
- `lookup_table_column_low_nibble`: Searches a word table for a key and copies the entry selected by the low nibble of a byte in the input record into the output record.
- `copy_word_from_record_offset`: Copies one word from offset sixteen of the source record into the output record at offset six.
- `adjust_word_by_nibble_step`: Adds the high nibble of a record byte to a stored word, or subtracts the low nibble when the high nibble is zero, and stores the result in the output record.
- `decrement_short_by_nibble`: Subtracts the low nibble of a record byte from a stored word when that nibble is nonzero, and stores the result in the output record.
- `start_audio_channels_with_delay`: Clears a global, calls a routine four times, and writes a constant to a DMA control global.
- `fetch_instrument_for_channel`: Reads a command word from a sound sequence and, unless it is an end marker, fetches the sample pointer and parameters for the instrument selected from a table into the channel record.
- `apply_volume_slide_effect`: Updates a channel's volume from its effect nibble: a five adds the parameter and a six subtracts it, both clamped to the range zero to sixty-four.
- `update_channel_volume_and_markers`: Clamps a channel's volume to sixty-four and stores it.
- `start_channel_from_sequence_step`: Starts an audio channel from a nonzero sequence step by storing the step's data in the channel record and setting the channel's bit in the DMA status flags after a short delay.
- `store_fields_and_or_flag`: Copies a long word and two short words from a table into a structure, then ORs one more table word into a global flag.
- `call_two_routines_undecoded`: The decompiler could not produce pseudo-code for this routine.
- `clear_entry_and_call_twice`: Clears the first word of a structure and one word of a table entry, then makes two calls with fixed parameters, one using a mask of all ones and one using the values one and zero.
- `decrement_and_clamp_volume_entry`: Decrements a word by ten, clears the entry's first word and one table word, and clamps another word at sixty-three before calling a routine with the entry's values.
- `call_one_routine_undecoded`: The decompiler could not produce pseudo-code for this routine.
- `clear_and_initialize_sound_twice`: Clears a word in the sound structure and another word computed from it, then calls a routine twice with parameters taken from the structure and fixed constant values.
- `open_cia_resource_once`: Opens ciaa.resource through the system library the first time it is called and records the handle.
- `reset_audio_library_if_set`: If a global word is nonzero, calls a library vector through a stored base, calls one more routine, and clears the word.
- `write_cia_timer_control_alt`: Writes a fixed control value to the CIA A control register, then copies a stored RAM value into the timer high and low registers through globals.
- `clear_audio_pointers_and_counters`: Clears the volume registers of the four audio channels, points each channel at a shared silent buffer, and zeroes four RAM counters.
- `reset_audio_channels_enable_dma`: Clears the four audio channel volume registers, points each channel at a shared buffer, zeroes four RAM counters and writes a channel mask into a global.
- `call_then_return_second_argument`: Calls one routine and returns its second register argument unchanged.
- `rearm_audio_every_sixth_tick`: Calls a routine for each of the four audio channels on every tick.
- `restart_audio_channels_after_delay`: Clears two global words, makes three calls to a routine, sets a flag bit and waits through a delay loop.
- `initialize_sound_channel_state`: Computes the record offset for a sound channel from its index, clears a long word there, and stores several words from the parameters into the channel record.
- `tick_channel_volume_envelope`: Counts down a channel's delay counter and, when it reaches zero, reloads it and adds a step to the channel's current value.
- `clamp_and_copy_short`: Copies a word within the record, adds two words and clamps the sum to the range zero to sixty-three.
- `copy_channel_fields_then_clear`: When a channel field is nonzero, copies its stored value and parameters into the output record, then clears the field when its flag word is zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `copy_and_update_scroll_state`: Copies a block of long words from a source into a destination word buffer while a flag is clear, then updates the scroll state globals, including a base offset, a word table pointer, a buffer pointer and a signed index.
- `setup_dos_file_streams`: Allocates a file context record, stores its fields, and creates an entry tagged with the text XNAM. It uses dos wrapper calls for the current directory, input, open and output to set up its streams.
- `read_cli_argument_when_present`: Finds the current process and, when it has a command-line interface record, reads the argument from that record through the dos library and stores it in a global.
- `launch_program_with_dos_lock`: Checks the current process for a command-line record, duplicates a lock on the directory and opens a file through the dos library.
- `run_library_vector_sequence`: Calls two library vectors through the stored library base and one further routine, with several conditional paths.
- `copy_c_string_return_destination`: Copies a zero-terminated string into the destination buffer, including the terminator, and returns the destination pointer.
- `copy_c_string_to_buffer`: Copies a zero-terminated string into a buffer byte by byte, including the terminator, and returns the original destination pointer.
- `measure_c_string_length`: Walks a zero-terminated string to its terminator and returns the number of bytes before it.
- `convert_to_uppercase`: Converts a lowercase ASCII letter to uppercase by subtracting the case offset, and returns any other byte unchanged.
- `convert_to_lowercase`: Converts an uppercase ASCII letter to lowercase by adding the case offset, and returns any other byte unchanged.
- `compare_two_byte_strings`: Compares two zero-terminated byte strings with a length limit.
- `safe_memory_copy`: Copies a given number of bytes from source to destination, copying backwards when the destination lies above the source so that overlapping regions are preserved.
- `initialize_dos_library_and_globals`: Clears a large block of globals to zero, reads the processor flags, and opens dos.library through the system library.
- `return_only_stub`: Contains only a return and performs no work.
- `read_command_input_block`: Chooses an input buffer from the command-line record or a stored default pointer, then reads and copies bytes through a library vector using a fixed-size loop that stops at a terminator.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d0.
- `append_c_string_to_buffer`: Appends a zero-terminated string to the end of the destination, copying at most a fixed count of bytes and terminating the result with a zero byte.
- `append_bounded_string_terminated`: Appends a zero-terminated string to the destination, copying at most a counted number of bytes.
- `copy_string_pad_zeros`: Copies bytes from a source string up to a count or until a zero byte, then fills the rest of the count with zeros.
- `open_icon_library_and_window`: Opens icon.library through the system library, looks up a window-related file name, and stores the resulting handles into the program's structure.
- `format_string_into_buffer`: Stores the destination buffer pointer in a global, runs a formatting routine with a character output callback and the caller's arguments, then terminates the buffer with a zero byte.
- `format_into_buffer_return_status`: Stores the destination pointer in a global, runs a formatting routine that writes through a character callback, then terminates the buffer and returns the formatter's result.
- `store_byte_and_advance_pointer`: Stores the low byte of its argument at the current position of a global output pointer, advances that pointer, and returns the byte.
- `forward_format_call_to_formatter`: Forwards its format string and stack arguments to a formatting routine, using the character output routine as its callback, and returns.
- `call_two_routines_undecodable`: The decompiler could not produce pseudo-code for this routine.
- `parse_printf_format_directives`: Walks a format string, copying literal characters through a callback and parsing each percent directive with its width and precision fields.
- `perform_signed_division`: Divides two signed values by calling the unsigned division routine on their magnitudes, tracks the signs, and negates the quotient when the signs differ.
- `call_and_return_concatenated`: Calls one routine and returns the value it leaves in the second data register, combined with the original second register.
- `call_routine_and_return`: Calls another routine and returns, performing no other work.
- `divide_unsigned_32bit`: Computes the quotient and remainder of an unsigned 32-bit division.
- `call_single_routine_wrapper`: Calls one other routine and returns.
- `call_three_routines_undecodable`: The decompiler could not produce pseudo-code for this routine.
- `return_immediately_2`: Performs no operations and returns to its caller immediately.
- `call_two_routines_undecodable_b`: The decompiler could not produce pseudo-code for this routine.
- `write_byte_to_buffer_cursor`: Writes one byte from a field of its frame to the current position of a buffer pointer, advances that pointer and returns the byte.
- `call_routine_for_each_record`: Walks an array of twenty-two-byte records from a fixed global start up to a fixed end, calling a routine on each record.
- `close_stream_record`: Closes a stream record: when its flags show it is open, it flushes the buffer, frees its storage and clears its fields.
- `close_stream_record_direct`: Closes the stream record at the current address using the same flush and release steps, clears its fields and returns a status word.
- `flush_stream_buffer`: Writes out buffered bytes of a stream record.
- `prepare_stream_buffer_pointers`: Resets a stream record on a close request, otherwise sets its current and end pointers from the buffer base and size and marks the record as active.
- `read_byte_from_stream_record`: Returns the next byte from a stream record's buffer, refilling it through a read routine when it is empty.
- `refill_stream_buffer_and_read`: When a stream record's buffer is empty, refills it through a read routine and resets its current and end pointers.
- `write_byte_to_stream_record`: Stores one byte at the current position of a stream record's buffer, sets the buffer end from the buffer size, and marks the record as active.
- `allocate_free_stream_record`: Scans the fixed table of twenty-two-byte stream records for one whose flag byte is clear.
- `allocate_free_stream_record_alt`: Searches the stream record table from a given position for a free record, clearing its fields and returning it, or returning null when none is free.
- `clear_record_header_words`: Zeroes the first three words of the record pointed to by its argument.
- `init_stream_record_buffer`: Sets up a stream record's buffer: when the allocation fails it uses a one-byte inline buffer, otherwise it uses an allocated buffer of 1024 bytes and marks the record as buffered.
- `init_stream_record_fixed_buffer`: Sets a stream record's buffer size to 1024 bytes and marks it as buffered, using the buffer supplied by the caller.
- `free_all_list_nodes`: Walks a linked list of allocated nodes from a global head, releasing each node with its stored size plus a fixed header.
- `release_node_list_via_library`: Walks a linked list of nodes, passing each node and its size plus a header to a library free routine.
- `allocate_and_link_list_node`: Allocates a block of the requested size plus a small header through the library, links it at the front of a global list, and returns a pointer just past the header, or null on failure.
- `push_node_onto_list_front`: Links a node at the front of a global list, setting its link to the previous head and its size from the caller's argument.
- `forward_parameter_word`: Forwards the first word of its parameter structure to another routine and then returns without other work.
- `unlink_and_free_list_node`: Searches the global node list for the node starting at the given address.
- `unlink_matching_list_node`: Searches the node list for the entry matching the caller's stored address, unlinks and frees it, and returns zero, or minus one when no match exists.
- `unlink_node_after_predecessor`: Unlinks a node from a global list given its predecessor.
- `pop_list_head_node`: Removes the first node from the global list by making its next link the new head, then calls one routine and returns zero.
- `check_slot_entry_open`: Checks that a slot index lies within the table and that its entry is in use.
- `check_slot_is_interactive`: Looks up a file slot's entry and asks the dos library whether its handle is interactive, returning true when it is.
- `return_zero_stub`: Returns zero without doing any other work.
- `delete_file_record_error`: Deletes a file through the dos library.
- `return_negative_one`: Returns the value negative one to its caller without modifying any data.
- `call_two_routines_undecodable_c`: The decompiler could not produce pseudo-code for this routine.
- `write_slot_checked_with_error`: Checks a slot's mode bits for writable access, then writes through the slot's handle with the dos library.
- `write_to_slot_record_error`: Writes data from a slot through the dos library write wrapper.
- `empty_routine_2`: This routine contains no instructions and performs no operations.
- `poll_console_flag_and_return`: Calls a library vector to test a status bit.
- `call_two_routines_in_sequence`: Calls two routines in sequence and returns zero.
- `write_global_buffer_to_output`: Gets the output handle from the dos library, writes a four-byte value from a global through the write wrapper, and returns.
- `call_hook_if_set`: If a global function pointer is set, calls through it, then calls another routine and returns.
- `close_all_open_file_slots`: Walks the file slot table and closes each open handle through the dos library, then releases the slot table.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_file_slot`: Closes the file in the given slot through the dos library close wrapper unless the slot is flagged as protected, then clears the slot entry.
- `dispatch_device_handler`: Calls a handler from the active device's function table through an indirect jump, then returns once that handler finishes.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_create_proc`: Library stub: loads its arguments and the dos library base, calls CreateProc and returns its result to the caller.
- `dispatch_current_dir_handler`: Calls the current-directory handler from the active device's function table through an indirect jump.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_dup_lock`: Wrapper that calls the dos library function DupLock and returns its result to the caller.
- `call_info`: Wrapper that calls the dos library function Info and returns its result to the caller.
- `call_input`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `dispatch_open_handler`: Calls the open handler from the active device's function table through an indirect jump.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `dispatch_write_handler`: Calls the write handler from the active device's function table through an indirect jump.
- `call_write`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `dispatch_library_entry_point`: Calls a library entry point through a function table reached from a stored base.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `dispatch_shared_library_entry`: Calls a library function through a table entry reached from a stored base, a path shared by many callers.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `dispatch_library_call_alt`: Calls a library function through a table entry reached from a stored base.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector`: Loads a library base from a global variable, indexes into its vector table, and jumps to the resulting function pointer before returning.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:AVENGER

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:wait

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_words_then_call`: Copies sixteen 32-bit words from a fixed offset of a base block into a caller-supplied buffer, but only when the buffer pointer is non-null.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:NoFastMem

- `start_program_and_messages`: Program entry point.
- `install_alloc_vector_or_error`: Installs a replacement memory allocation vector through the system library.
- `call_library_entry_and_return`: Calls a library entry point at a fixed offset through a stored base and returns, with no other logic.
- `call_library_entry_second`: Calls a library entry point at a different fixed offset through a stored base and returns, with no other logic.
- `call_library_entry_third`: Calls a library entry point at another fixed offset through a stored base and returns, with no other logic.
- `call_library_vector_420`: Library stub: loads a library base from a global variable and calls the library function at offset minus 420 of that base, returning its result to the caller.
- `call_library_vector_324`: Library stub: loads a library base from a global variable and calls the library function at offset minus 324 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_stored_function_pointer`: Calls the function whose address is stored in a global variable, then returns.
- `call_second_stored_function_pointer`: Calls the function whose address is stored in a second global variable, then returns.
- `call_exit_intr`: Library stub: loads its arguments and the exec library base, calls ExitIntr and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_supervisor`: Library stub: loads its arguments and the exec library base, calls Supervisor and returns its result to the caller.
- `call_reschedule`: Library stub: loads its arguments and the exec library base, calls Reschedule and returns its result to the caller.

### Future Wars - Time Travellers (1989)(U. S. Gold)[cr Avenger][m Maimex](Disk 1 of 2).adf:bootblock

- `relocate_boot_code_and_call`: Boot block entry point.
- `open_library_and_init_display`: Boot-time setup.
- `call_library_pair_and_return`: Calls two library entry points through a stored base in sequence and returns.
- `init_color_clear_interrupts`: Writes a bright colour value, runs a short delay loop that reads the raster position, and clears several words of interrupt state.
- `find_dos_resident`: Looks for the dos.library resident module through the system library.
- `return_empty_result`: It is an empty routine that returns immediately without performing any operation.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_infinite_loop`: Is empty, containing only an endless loop that never returns.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_3`: The routine contains no instructions and performs no operations on any data or registers; it is an empty routine that returns immediately.
