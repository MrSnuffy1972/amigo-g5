# Game 678 (spot - 7-up game (1990)(virgin mastertronic)[cr qtx]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00678). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 132 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 43 raw-word lines |
| Functions named and described | pass, 402 of 402 named (0 author symbols), 402 of 402 described (2 by rule, 400 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1518 of 1518 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 35608 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9051 of 9051 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 583513 |
| code | 29902 |
| data | 187188 |
| padding | 39077 |
| unused | 61440 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Run | 2 | 117 | 2 |
| type | 2 | 117 | 2 |
| Runback | 3 | 709 | 39 |
| EndCLI | 2 | 117 | 2 |
| FastMemFirst | 7 | 207 | 6 |
| wait | 2 | 117 | 2 |
| Spot - 7-Up Game (1990)(Virgin Mastertronic)[cr QTX].adf:Spot | 3 | 7630 | 348 |
| Spot - 7-Up Game (1990)(Virgin Mastertronic)[cr QTX].adf:bootblock | 1 | 37 | 1 |
| option.ani (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- spot - 7-up game (1990)(virgin mastertronic)[cr qtx]/Spot - 7-Up Game (1990)(Virgin Mastertronic)[cr QTX].adf:inst2/Flute.ss: high-entropy data file (7.68 bits/byte), classified as packed data
- spot - 7-up game (1990)(virgin mastertronic)[cr qtx]/Spot - 7-Up Game (1990)(Virgin Mastertronic)[cr QTX].adf:ani/option.ani: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 35608 bytes, recorded as UNVERIFIED DATA (0 bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Run

- `initialize_game_state`: The routine walks a linked list pointed to by a global variable, finds the maximum value stored in the list, and then builds a small table in a stack buffer with that maximum.
- `copy_16_words_from_table`: The routine copies sixteen words from a table located at an offset from a pointer in a2 into a caller-provided buffer, then calls another routine.

### type

- `find_max_and_build_table`: The routine walks a linked list of structures in RAM to find the largest value stored at an offset from each node, then builds a small table in a stack buffer and copies a 16-word block between a global buffer and the stack.
- `copy_table_to_stack`: The routine copies sixteen 32-bit values from a table located at an offset from a pointer in A2 into a memory area pointed to by the first argument on the stack, then calls another routine.

### Runback

- `initialize_runback`: The routine is empty and contains no instructions.
- `run_background_task`: The routine parses a command line for an optional load delay and a program name with parameters, then starts the named program as a background task, handling errors such as missing command, missing file, directory instead of file, out of memory and usage errors, and prints status messages.
- `clear_table_and_recurse`: The routine clears a block of 40 words starting at an offset from a base pointer passed on the stack, writes a pointer to a local stack slot and a global variable into two fields following the block, then calls another routine.
- `call_indirect_if_flag_then_recurse`: The routine checks a global variable and, if it is non-zero, calls the function pointer stored there.
- `process_dialogue_text`: The routine processes a text block referenced by a global table, splitting it into words by scanning characters and using a flag table to skip delimiters, storing pointers to each word in a newly allocated array.
- `runback_loop_and_callbacks`: The routine runs a ten-iteration loop calling another routine, then checks three global variables for non-zero values and conditionally calls another routine with values taken from those globals, and if a fourth global is zero it performs two more calls with values from globals and returns the result of a call with a field from its parameter; otherwise it makes two further calls and returns its parameter unchanged.
- `run_back_loop_handler`: The routine performs a short loop calling another routine ten times and then inspects several global variables.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination buffer, stopping at a null terminator or after a bounded number of iterations, and ensures the destination is null-terminated.
- `copy_string_with_limit_2`: The routine copies bytes from a source pointer to a destination buffer, stopping at the first null byte or when a counter in the third argument reaches -1.
- `copy_null_terminated_string`: The routine copies bytes from a source pointer to a destination pointer one at a time until a zero byte is encountered, then returns the original destination pointer.
- `calculate_string_length`: The routine walks forward through a null-terminated byte string starting at the input pointer until it finds the terminating zero byte.
- `copy_bytes_with_padding`: The routine copies bytes from a source pointer to a destination pointer for a limited count derived from a 32-bit parameter, stopping early if a zero byte is encountered, then writes zero bytes to the remainder of the destination area for the same count.
- `clear_node_and_check_bounds`: The routine checks the index from the parameter against bounds and a table entry, sets a global flag on error, otherwise clears the table entry and recursively processes a linked node if a flag bit is set.
- `call_indirect_via_table`: The routine calls another routine indirectly through a pointer obtained from a global variable table.
- `call_indirect_via_table_2`: The routine calls another routine indirectly via a pointer obtained from a global variable table.
- `call_indirect_via_table_3`: The routine calls another routine indirectly through a pointer obtained from a global variable.
- `call_indirect_through_table`: The routine calls another routine indirectly through a table.
- `call_indirect_via_table_4`: The routine loads a pointer from a global variable table and calls another routine through an indirect jump.
- `call_indirect_through_table_2`: The routine loads a pointer from a global variable, adds a constant offset to it, and calls the routine found there indirectly.
- `call_indirect_via_table_5`: The routine calls another routine indirectly through a table referenced by a global variable.
- `call_indirect_via_table_6`: The routine calls another routine indirectly through a table.
- `call_indirect_via_table_7`: The routine calls another routine indirectly through a pointer obtained by adding an offset to a global variable and then adding a second offset.
- `call_indirect_via_table_8`: The routine calls another routine indirectly through a table.
- `call_indirect_through_table_3`: The routine loads a pointer from a global variable, adds an offset to it, and calls the routine found at that address.
- `call_indirect_through_table_4`: The routine calls another routine indirectly through a table referenced by a global variable.
- `call_indirect_via_table_9`: The routine loads a value from a global variable, adds a constant offset to it, and calls the resulting address as a function pointer.
- `call_indirect_via_table_10`: The routine loads a pointer from a global variable, adds an offset, and calls the routine found there indirectly.
- `call_indirect_through_table_5`: The routine loads a pointer from a global variable, adds an offset to it, and calls the resulting address as a function.
- `call_indirect_through_table_6`: The routine performs an indirect call via a pointer taken from a table referenced through a global variable.
- `call_indirect_via_table_11`: The routine fetches a pointer from a table stored in a global variable and calls the routine it points to.
- `call_indirect_table_entry`: The routine reads a pointer from a global variable, adds a fixed offset to it, then calls the routine found at that address.
- `call_indirect_via_global_table`: The routine loads a function pointer from a global variable table via an offset, adjusts it with a constant, and calls the resulting routine indirectly.
- `call_indirect_through_table_7`: The routine reads a pointer from a table pointed to by a global variable and then calls another routine through that pointer.
- `call_indirect_via_table_12`: The routine loads a pointer from a global variable, adds an offset, and calls the routine found there via an indirect jump.
- `call_indirect_table_entry_2`: The routine loads a function pointer from a global variable table and calls the referenced routine indirectly.
- `call_indirect_through_table_8`: The routine reads a pointer from a table located at an offset from a global variable and calls another routine through an indirect jump.
- `call_indirect_via_global`: The routine loads a value from a global variable, adds a constant to it, and calls the resulting address as a function pointer.
- `call_indirect_table_entry_3`: The routine reads a function pointer from a table referenced via a global variable, adjusts the pointer and calls the retrieved routine.
- `call_indirect_via_table_13`: The routine calls another routine indirectly through a pointer obtained from a global variable.

### EndCLI

- `initialize_cli_table`: The routine walks a linked list of entries to find the maximum value stored in a global variable, then builds a small table in a stack buffer and fills it with sequential words.
- `copy_table_to_stack_2`: The routine copies sixteen 32-bit words from a fixed table located at an offset from a2 into the memory area pointed to by the first argument on the stack.

### FastMemFirst

- `parse_command_line`: The routine obtains a pointer to a command-line buffer from the Amiga DOS library and parses it into a null-terminated argument vector.
- `find_and_copy_fastmem_block`: The routine walks a linked list of memory blocks starting from a global variable, selects the first fast-memory block with a specific type tag and the first chip-memory block with another type tag, and if both are found and different it calls another routine on the fast block and then calls another routine with the list head, the fast block and the chip block.
- `call_indirect_table_entry_4`: The routine calls another routine through a pointer stored in a global variable table.
- `call_indirect_table_entry_5`: The routine calls another routine through a pointer obtained by indexing a global table with an offset.
- `call_indirect_table_entry_6`: The routine calls another routine through a pointer obtained by indexing a global table with a fixed offset.
- `call_indirect_table_entry_7`: The routine calls another routine through a pointer obtained by indexing a global variable table with a fixed negative offset.

### wait

- `populate_object_pool`: Allocates a temporary table from the stack and fills it with a range of integers, then walks a linked list in a global variable to find a maximum length and uses that length to size the table.
- `copy_16_words_from_table_2`: The routine copies sixteen words from a table located at an offset from a pointer in a2 into a caller-supplied buffer, then calls another routine.

### Spot - 7-Up Game (1990)(Virgin Mastertronic)[cr QTX].adf:Spot

- `jump_to_draw_spot_grid`: Jump stub: passes control straight on to draw_spot_grid.
- `set_flag_byte`: The routine stores the value 1 into a byte of a global variable.
- `decrement_and_update_table`: The routine walks a null-terminated list of signed short indices stored in a table pointed to by a global variable.
- `empty_routine`: The routine contains three 68000 instructions and performs no observable work; it ends immediately.
- `count_bytes_in_buffer`: The routine clears four bytes at fixed offsets in a buffer and then scans the first 49 bytes of the buffer, incrementing the corresponding counter byte for each occurrence of a marker value.
- `increment_counter_bytes`: The routine increments specific bytes in a structure pointed to by a pointer stored in a register.
- `increment_counter_bytes_2`: The routine walks through a table of bytes using an index stored in a local variable and increments specific counter bytes in a structure pointed to by a pointer on the stack.
- `increment_table_counters`: The routine walks an index stored in a global variable through a table of bytes, incrementing a separate counter byte for each table entry value it encounters.
- `empty_routine_2`: The routine contains three instructions and performs no observable work.
- `clear_and_copy_bytes`: The routine clears three bytes in a global variable and then copies 49 bytes from one global buffer to another global buffer.
- `copy_bytes_from_table`: The routine copies a sequence of bytes from one memory area to another using a loop counter stored in a global variable.
- `empty_routine_3`: The routine contains three instructions and performs no observable work; it is effectively empty.
- `clear_three_bytes`: The routine clears three consecutive bytes in a global variable by storing zero to each of them.
- `check_spot_in_table`: The routine checks a byte parameter against a table of bytes using an index stored in a global variable.
- `consume_string_table_entry`: The routine uses two global areas as a table and state.
- `find_next_character_match`: The routine advances a counter and attempts to locate a matching byte from a source buffer within a lookup table, updating an output pointer and a global state byte.
- `decode_input_character`: The routine walks a character index and compares a byte from a table with a byte from a caller-supplied buffer.
- `handle_input_buffer`: The routine increments a state byte and processes a pointer-based input buffer stored in a global variable, using a lookup table to compare characters against expected values.
- `copy_string_with_lookup`: The routine copies bytes from a source pointer into a destination pointer using an index stored in a global variable, advancing the index with a lookup table and a secondary table to compute the destination offset.
- `copy_string_with_table_lookup`: The routine walks two index bytes using a table lookup to decide when to copy characters from a source buffer pointed to by a global pointer into a destination buffer, advancing the destination pointer each time.
- `search_and_update_table_entry`: The routine checks a flag byte in a global variable and, if set, iterates over an index byte to compare a byte from a table with a byte pointed to by a pointer argument.
- `increment_and_compare_byte`: The routine increments a counter byte in a global variable and uses it to index a table of bytes, comparing each entry with a byte from another global structure.
- `increment_and_compare_table`: The routine increments a counter byte in a global variable and uses it to index a table of bytes, comparing each entry against a byte from a pointer stored in another global variable.
- `find_next_valid_entry`: The routine scans a table of bytes using an index stored in a global variable, compares each entry to a byte from another structure, and on a match loads a pointer from a table and advances it while checking a lookup table for termination.
- `increment_and_compare_table_2`: The routine increments a counter byte in a global variable and uses it to index a table of bytes.
- `find_next_matching_byte`: The routine scans a byte table using an index stored in a global variable, compares each entry to a byte from another global structure, and on a match loads a pointer from a second table into a global variable and advances the pointer while checking a termination table.
- `parse_input_stream`: The routine walks a byte index through a table and compares a byte from a table with a byte from a structure.
- `return_zero_stub`: The routine is empty and returns zero.
- `process_player_item_queue`: The routine processes an item queue indexed by a state variable in a global variable, matching a player id against a lookup table and advancing a cursor through a per-player string.
- `reset_and_copy_table`: The routine resets a set of global counters and copies a block of bytes from a source table to a destination table.
- `process_spot_table`: The routine updates a small set of global variables and walks through a table indexed by a counter, copying bytes from a source table into a destination table.
- `scan_input_for_match`: Iterates over a small index range, compares bytes from a lookup table with a target byte from the caller's context and uses that match to drive a small state machine that consumes bytes from a caller-supplied buffer, updates counters in a global variable area and a per-element table, and returns when the buffer ends, a sentinel is hit or a match condition is met.
- `reset_and_copy_table_2`: The routine resets a counter and copies a table of bytes from one global area to another using an index stored on the stack.
- `find_next_matching_symbol`: The routine scans a small index range using a global table and a byte from a second structure, and on a match jumps to a block that loads a pointer from a table into a local variable and walks the pointed-to bytes, updating counters and marking a table entry.
- `copy_palette_bytes`: The routine copies a block of bytes from one global table to another and then, when a flag is set, scans a byte table using an index to locate a matching byte, updates a counter, and walks through a string pointed to by a global variable, writing characters into a buffer and marking entries in a table.
- `process_input_buffer`: The routine processes a byte buffer pointed to by a pointer stored in memory.
- `mark_word_in_7_up_list`: The routine walks a small word list, compares its first character with a byte passed in a caller structure, and updates a position index in that structure while advancing through the list.
- `process_input_buffer_2`: The routine scans a byte table using an index stored in a global variable and compares each entry to a byte from a caller-supplied structure.
- `increment_and_compare_table_entry`: The routine increments a counter byte in a global variable and uses it to index a table of bytes, comparing each entry against a byte from another global variable.
- `increment_and_match_byte`: The routine increments a counter byte in a global variable and uses it to index a table to compare a byte from a global structure with a byte from a pointer.
- `decode_password_check`: The routine attempts to validate a short password by comparing characters against a table stored in a global variable and a reference byte.
- `increment_and_compare_byte_2`: The routine increments a counter byte in a global variable and uses it to index a table of bytes.
- `decode_input_byte`: The routine increments a counter byte and compares a table byte indexed by that counter with a byte from another routine's argument block.
- `check_tile_and_advance_cursor`: The routine scans a table of bytes using an index stored in a global variable, comparing each entry to a byte from another global structure.
- `return_zero_stub_2`: The routine is empty and returns zero.
- `search_character_in_tables`: The routine walks two indexed tables using bytes stored in a global variable area, copying characters from a table entry into a working byte and comparing the character’s table index with a byte from the parameter.
- `search_string_in_table`: The routine walks two tables indexed by a byte counter, loads a pointer from a table into a temporary location and scans the pointed-to string byte by byte, comparing each byte via a lookup table against a target byte in memory.
- `match_string_in_table`: The routine walks a table of string pointers using a state byte and an index byte, fetching a pointer from a table, advancing through the string character by character, and comparing each character via a lookup table against a target byte.
- `find_string_match_in_table`: The routine scans a null-terminated string via a pointer stored in a global variable, compares each character against a table of characters indexed by a byte read from a global table, and returns when a match is found.
- `compare_string_with_table`: The routine walks through a table of pointers and compares a byte from a string pointed to by a global variable with a byte from a global table, advancing the string pointer and a counter until a match or end of string is found.
- `match_string_in_table_2`: The routine increments a counter and walks through a table of pointers, loading a character from the pointed-to string into a global byte and testing it against a target character.
- `increment_and_compare_string`: The routine increments a counter byte in a global variable and repeatedly reads bytes from a pointer stored in a global variable, writing each byte to a global byte.
- `search_string_in_table_2`: The routine walks a byte index through a table of strings and compares each character against a target byte using a translation table.
- `match_input_against_string_table`: The routine walks a table of pointers and a string table to compare a byte from a caller-supplied buffer against characters from a selected string, advancing indices and returning a match flag.
- `find_matching_character`: The routine scans a table of pointers using an index stored in a global byte, skipping empty entries, and compares each character from the pointed-to strings against a target character supplied via a register.
- `scan_string_table_for_byte`: The routine walks an index byte and uses it to select a pointer from a table of strings, then scans the chosen string byte by byte comparing each character’s table-mapped value with a target byte supplied in a register.
- `copy_sprite_table`: The routine copies a block of bytes from a global source area to a global destination area, then updates a byte in the destination area using an index from a global variable and increments a counter byte.
- `copy_table_and_update_counters`: The routine copies a block of bytes from one area of memory to another using an index stored in a global variable, then updates a byte pointed to by an offset stored in a global variable and adjusts a counter byte.
- `increment_counter_and_recursion`: The routine reads a byte from a global variable, uses it as an index into a table to fetch a byte from another global variable, and increments that byte.
- `decrement_linked_counters`: The routine updates four bytes of a global variable from another global variable and conditionally increments a counter byte for a given index.
- `compute_adjusted_offset`: The routine reads several bytes from global variables and from a table indexed by a byte from the pointer argument, combines them with arithmetic and shifts, adds a base byte from a global variable, and returns the low byte of the result as a signed short.
- `increment_counter_table`: The routine clears a 49-byte counter table in a global variable, then for each of 49 entries whose selector byte matches the input byte it walks a null-terminated short offset list from a global pointer table and increments the corresponding counter byte when the referenced selector byte is zero.
- `clear_counters_and_accumulate`: The routine clears a 49-byte counter table, then for each entry whose key matches a byte from a global table it loads a pointer from another table and walks the pointed-to string, incrementing the corresponding counter for each character encountered.
- `count_character_occurrences`: The routine scans a 49-byte table for matches to a byte in a global variable and, for each match, walks a zero-terminated string from a table of pointers, incrementing a counter in a byte table for each character encountered.
- `count_nonzero_table_entries`: The routine iterates over a table of 49 bytes and, for each entry that matches a reference byte, it loads a pointer from a second table and walks the pointed-to string.
- `decode_string_table`: The routine iterates over a 49-byte table, finds entries matching a byte, loads a pointer from a table, then walks the pointed-to string byte-by-byte, converting each character to a short and accumulating a checksum while incrementing a counter for each null terminator encountered in a second table.
- `sum_table_bytes`: The routine clears a local accumulator, then loops 49 times adding a signed byte from a table pointed to by an address register offset by the loop index to the accumulator, and returns the accumulated sum.
- `accumulate_table_sum`: The routine loops a fixed number of times, adding successive signed bytes from a table pointed to by a register offset to an accumulator stored on the stack, and returns the accumulated sum.
- `return_negative_offset_from_a5`: The routine returns the 32-bit value stored at an address computed as the contents of A5 minus a constant offset.
- `copy_bytes_to_global`: The routine copies three bytes from a global variable to three other global variables and then writes the constant 1 to a fourth global variable.
- `copy_three_bytes_from_buffer`: The routine copies three bytes from a fixed source area in memory to consecutive locations in a destination area whose base is held in a global variable.
- `reorder_score_table_entries`: The routine reorders a list of three-byte entries using another routine for comparison and stores the best entry's fields into globals.
- `reorder_score_table_entries_2`: The routine iterates over a table of entries, copies three attribute bytes from each entry into temporary storage, calls another routine to process the entry and obtains a value, and then reorders the entries in-place by score using a temporary index and a counter.
- `process_entity_attributes`: The routine loads an index, reads three signed bytes from tables using that index and stores them to globals.
- `process_string_list`: The routine receives a count from a global variable, calls another routine with that count and stores the result.
- `empty_routine_4`: The routine contains three 68000 instructions and performs no observable work; it ends immediately after being entered.
- `restore_buffer_and_compute_checksum`: The routine copies bytes from a global buffer into a working area and back, updates a global offset, and then iterates over a small set of indices, calling another routine with different parameters and handling its return value.
- `copy_and_decompress_animated_sprites`: A routine that moves a block of bytes from a global table into an animation buffer using an offset stored in a global variable, then restores the buffer from another global table, and then drives a short loop that steps a position index through a lookup table of byte values, calls another routine for each entry and processes the returned result to optionally copy sprite bytes back and return the resulting byte value.
- `copy_sprite_tiles_to_buffer`: The routine copies a block of bytes from a global table into a work buffer, then repeatedly selects a byte from a table indexed by a counter and uses it to compute a value that is stored in a global variable.
- `copy_sprite_tiles_and_draw`: The routine copies a small block of bytes from a table to a work buffer, then repeatedly selects a byte from a table using an index stored in a global variable, calls another routine to obtain a value, and updates a global sprite buffer with tiles from a larger table.
- `select_sprite_tile_and_draw`: The routine copies a byte from a table into a byte in a structure, clears a counter and then loops.
- `draw_digit_tile`: The routine repeatedly fetches a byte from a table via an index stored in a stack variable and uses it to select a code path that calls another routine through a function pointer on the stack.
- `process_input_buffer_3`: The routine repeatedly calls a function pointer stored in a global variable and processes a byte buffer using a global variable as a base.
- `copy_byte_table_loop`: The routine increments a counter stored in a global variable and loops while the counter is less than a fixed limit.
- `read_byte_from_a5_offset`: The routine reads a byte from an address formed by adding a fixed offset to an address in A5, increments the byte value by one, and returns the low byte of the result as a signed value.
- `restore_sprites_to_buffer`: The routine manages a small counter stored in a global variable and calls another routine to obtain a byte result.
- `copy_palette_buffer`: The routine copies a 57-byte block between a global buffer and a global palette table using an index stored on the stack, then calls another routine and stores its byte result.
- `copy_sprite_pattern`: The routine copies a 57-byte pattern between two tables using an offset stored in a global variable, calls another routine to obtain a byte, and then restores the original tables.
- `check_input_and_refresh_buffer`: The routine saves and restores a global offset, calls another routine and stores a byte result, copies 57 bytes from a source table using that offset into two different destinations depending on a condition, and then performs a timed polling loop that reads and compares bytes from a table and a callback while updating buffer pointers and flags before returning a byte value.
- `rotate_palette_buffer`: The routine copies a small block of bytes from a source table into two destination buffers using an index stored in a global variable, adjusting the index when a condition on flags and a global value is met.
- `copy_sprite_pattern_2`: The routine copies a 57-byte pattern from a source table into a destination buffer using an offset stored in a global variable, and updates that offset.
- `copy_sprite_tile_block`: The routine copies a 57-byte block from a source area to a destination area using an offset stored in a global variable, and returns a byte from a structure.
- `copy_bytes_from_table_2`: The routine increments a counter stored in a global variable and, while the counter is less than a limit, copies a byte from a table addressed by a global variable and an offset to a destination buffer addressed by another global variable and the counter.
- `read_byte_from_stack_offset`: The routine returns the low byte of a word located nine bytes before the value in register A5, interpreted as a signed 16-bit value.
- `update_player_sprite`: The routine updates a byte in a global variable using values from a table and a global variable, then repeatedly calls another routine until a condition is met.
- `copy_byte_table`: The routine copies a block of bytes from a source table to a destination table, advancing a counter stored in a global variable until the counter reaches a limit.
- `increment_byte_field`: The routine reads a byte from an offset within a structure pointed to by A5, adds one to it, and returns the low byte as a signed value.
- `copy_sprite_tiles`: The routine copies 57 bytes between a global tile buffer and a global sprite buffer using an offset stored in a global variable, updates a byte in a structure pointed to by A5, and returns a byte from that structure.
- `compare_input_bytes`: The routine copies a 57-byte block from a table to a work buffer, reads a byte from another routine, then copies the block back.
- `copy_sprite_row_data`: The routine copies a row of bytes from a source table into a destination buffer using an offset stored in a global variable, updates a counter, and returns a byte from the buffer.
- `copy_sprite_pattern_3`: The routine copies a 57-byte pattern from a source table to two different destination buffers using an offset stored in a global variable, then repeatedly polls another routine and updates a byte in a structure while checking several flags and a function pointer.
- `reset_display_buffer`: The routine repeatedly increments a counter in a structure and, when the counter exceeds a limit, it copies a block of bytes from a source table to a destination buffer and returns a byte from the structure.
- `select_min_value_and_copy_bytes`: The routine compares two bytes in a structure, keeps the smaller one, and returns the chosen byte.
- `copy_tile_buffer_and_return_value`: The routine repeatedly queries another routine for input and updates a byte in a structure.
- `copy_byte_table_loop_2`: The routine copies a block of bytes from a source table to a destination table, incrementing a counter stored in a global variable on each iteration until the counter reaches a fixed limit, then returns the low byte of another global variable.
- `read_byte_from_pointer`: The routine reads a byte from memory using an address derived from an uninitialized pointer and returns the low byte as a signed short.
- `swap_and_compare_bytes`: The routine repeatedly calls another routine via a function pointer stored in a global variable and uses the returned byte to update a small byte buffer.
- `copy_bytes_from_table_3`: The routine decrements a global counter and clears a loop index in a register variable, then copies a sequence of bytes from a table addressed by a global variable to a buffer addressed by another global variable, incrementing the index each iteration.
- `copy_byte_table_loop_3`: The routine increments a counter stored in a global variable and, while the counter is less than a fixed limit, copies a byte from a source table to a destination table using the counter as an index, with the source offset also influenced by another global variable.
- `read_byte_from_stack`: The routine reads a byte from an address computed as the address in A5 minus nine and returns it as a signed short.
- `draw_digit_and_advance`: The routine drives a digit display loop using a global counter and a table of digit patterns.
- `copy_bytes_from_table_4`: The routine copies a block of 57 bytes from a table in memory to another table, using an offset stored in a global variable that it decrements.
- `copy_byte_table_loop_4`: The routine increments a counter stored in a global variable and, while the counter is below a limit, copies a byte from a source table to a destination table using the counter as an index, with the source offset adjusted by another global variable.
- `read_byte_from_stack_offset_2`: The routine reads a byte from a memory location computed as the address in A5 minus nine and returns it as a signed short.
- `spot_7up_game_loop`: The routine is a small loop that repeatedly calls another routine, then checks a global variable and branches back to the start of the loop.
- `process_game_state`: The routine updates a set of global flags and a small byte buffer based on values read from a global structure, calls another routine repeatedly until a condition on a flag and a function pointer changes, and copies 57 bytes between two global buffers while toggling a flag.
- `swap_tile_buffers`: The routine copies a 57-byte block between two memory areas using a temporary offset, then restores the original area, and updates a flag byte based on a computed value from a set of signed bytes.
- `copy_and_swap_buffer_bytes`: The routine copies 57 bytes from one table to another and then swaps them back, using a global offset.
- `swap_pattern_buffers`: The routine copies 57 bytes between two pattern buffers using an offset, invokes another routine and stores its byte result, then restores the buffers.
- `copy_sprite_buffer`: The routine swaps a 57-byte block between two buffers using a temporary offset, then computes a byte from a table using an index from the stack and stores the result.
- `copy_buffer_table`: The routine copies 57 bytes from a source table to a destination table using an offset stored in a global variable, then computes a byte from a small constant table and a byte in a structure and stores the result.
- `update_game_state`: The routine computes a byte from a table using an index, then loops while a callback returns non-zero and a flag is clear.
- `compare_input_to_expected`: The routine reads a byte from a global variable and a byte from a caller-provided buffer, compares them and updates a status byte in a global variable.
- `swap_and_poll_input`: The routine repeatedly copies 57 bytes between two memory areas using an offset stored in a global variable, then calls another routine and stores its returned byte.
- `copy_tile_row_to_buffer`: The routine copies bytes from a source table to a destination buffer using an index and an offset stored in a global variable, repeating the copy while the index is below a threshold.
- `copy_sprite_buffer_2`: The routine repeatedly copies a block of bytes from one global buffer to another and back again using an offset stored in a global variable, then polls a byte from another routine and updates a flag byte.
- `copy_tile_row`: The routine copies bytes from a source table to a destination table in a loop, using a counter stored in a global variable.
- `synchronize_game_state`: The routine repeatedly compares two bytes from a structure, optionally calls another routine and copies a byte between them, then invokes a function pointer in a loop while checking a global flag and a global byte.
- `unknown_152c`: A looped routine that reads a byte from another routine, compares two bytes stored on the stack, optionally invokes another routine, and periodically copies a block of 57 bytes between two areas in memory via a global pointer.
- `call_other_and_return_one`: The routine calls another routine and then returns the constant 1.
- `clear_game_state`: The routine clears a set of fields in a global variable by storing zero and all-ones values to consecutive offsets.
- `insert_sorted_linked_node`: The routine inserts a node into a sorted singly linked list stored in a global table.
- `remove_node_from_sorted_list`: The routine walks a sorted linked list using a global table and a counter, following next pointers until it finds a matching entry or reaches the end.
- `clear_table_entry_and_push_index`: The routine clears a 32-bit entry in a table indexed by a global counter, increments that counter, and pushes the previous counter value onto a stack slot via a pointer in a register.
- `process_spot_state`: The routine checks a byte from a table indexed by the parameter against a sum of several global bytes and selects one of two code paths.
- `reset_input_buffer_and_update_state`: The routine resets a flag byte in a global variable and updates a game state byte in a global variable.
- `swap_tile_buffer_and_draw`: The routine copies a 57-byte tile buffer between two global areas and updates a global flag based on a comparison of five signed bytes indexed by a byte from a structure.
- `swap_buffer_copy_and_poll`: Copies a 57-byte block between two buffers using a global offset, swaps the buffers back, and calls another routine to update state.
- `swap_sprite_buffer_and_poll`: The routine copies 57 bytes from a source buffer to a destination buffer using an offset stored in a global variable, then calls another routine and stores its byte result in a stack slot.
- `swap_screen_buffers`: The routine swaps the contents of two 57-byte screen buffers via a global offset, updates a byte derived from a table lookup, then repeatedly polls a callback until it returns zero or a flag is set.
- `copy_tile_rows`: The routine copies 57 bytes from a source table to a destination table using an offset, calls another routine, computes a byte value from a small table and stores it, then repeatedly polls a function pointer while swapping bytes between two tables and updating a minimum byte value.
- `update_display_buffer`: The routine computes a byte from a table using an index from a global variable and stores it, then loops while a callback returns non-zero and a flag is clear.
- `decode_sprite_color_index`: The routine computes a byte value from a small table and stores it, then repeatedly calls another routine and compares returned bytes with a threshold from the caller’s stack frame.
- `swap_and_poll_input_2`: The routine repeatedly swaps 57 bytes between two buffers whose base is offset by a global variable, then calls another routine and stores its returned byte.
- `copy_sprite_row_loop`: The routine repeatedly increments a counter stored in a global variable and copies a byte from a source table to a destination table using an offset derived from another global variable.
- `copy_sprite_buffer_3`: The routine repeatedly updates a global offset, calls another routine to obtain a byte and stores it, then copies 57 bytes between two tables using the offset, and runs a nested loop that polls another routine, compares bytes, calls a function pointer, and conditionally returns.
- `copy_tile_row_to_buffer_2`: The routine repeatedly copies a fixed number of bytes from a source table to a destination buffer using an offset that is advanced each iteration, and it updates a byte in a global variable with the result of another routine.
- `synchronize_game_state_2`: The routine repeatedly calls another routine and updates a small set of bytes in a global structure, copying 57 bytes between two global buffers via an offset and then restoring them.
- `copy_sprite_buffer_while_looping`: The routine loops processing input bytes, updating two byte-sized flags and invoking callbacks through function pointers stored in memory.
- `return_constant_one`: The routine returns the constant value one.
- `set_table_entry`: The routine stores a byte from a register through a pointer.
- `process_tile_attribute_loop`: The routine checks a condition involving several bytes from a global variable and clears a word in a structure if true.
- `copy_tile_attributes`: The routine repeatedly copies a set of bytes from a table indexed by a global variable into a small set of global variables and updates a flag byte based on one of the copied bytes.
- `call_and_return_one`: The routine calls another routine and then returns the constant value one.
- `update_flag_and_dispatch`: The routine clears a global flag byte and increments a global counter.
- `set_flag_and_load_byte`: The routine checks a flag in a structure and, if set, writes a flag to a global variable and calls another routine.
- `read_byte_from_table`: The routine calls another routine to obtain an index, then reads a byte from a table addressed via a global variable and stores it into a global variable.
- `dispatch_state_handler`: The routine calls another routine to obtain an index, then either returns zero or calls another routine again to obtain a result.
- `empty_stub_routine`: The routine contains no instructions and performs no operations.
- `copy_three_bytes_to_table`: The routine copies three bytes from a byte table into a short table, advancing the source by one byte and the destination by a fixed stride each iteration.
- `copy_tile_rows_with_flags`: The routine copies bytes from a source area to a destination area row by row, updating a set of global variables that hold counters and flags.
- `initialize_spot_sprite_and_wait_input`: The routine copies a 48-byte tile pattern into a work buffer, masks the buffer to the high bit, sets four control bytes, then enters a loop that waits for a global flag and polls the CIAA_PRA register via another routine.
- `toggle_flag_in_table`: The routine computes an index from two input values and checks a flag byte in a global table.
- `mask_spot_table`: The routine copies 48 bytes from a source table to a destination table when a flag is clear, then masks each byte in the destination table to keep only the high bit.
- `set_sprite_bitmask_and_copy`: The routine obtains a value from another routine, masks it and then writes a series of bytes into a 3x3 block of memory referenced through a global variable, setting the high bit for each bit set in the value and clearing it otherwise.
- `rotate_word_by_global`: The routine reads a 16-bit word from a table pointed to by a global variable, multiplies it by a constant, then rotates the resulting 32-bit value left by the low three bits of another global variable and writes the rotated value back to the same table entry.
- `scale_value_by_table_entry`: The routine reads a 16-bit value from a global table using an offset derived from an unused register, multiplies it by the high word of the parameter, and returns the product shifted right by 16 bits.
- `count_and_check_spot_tiles`: The routine clears a small counter table and a flag, then scans a block of bytes, decrementing a counter and tallying nibble values into the table.
- `store_and_update_score_entry`: The routine writes two 16-bit values from registers into a structure pointed to by a register, computes a short field from a register and a temporary value, calls another routine, then recomputes that short field using the register value and the previous field, adds a constant to another short field in the structure, and calls another routine.
- `scan_table_for_matching_byte`: The routine reads a byte from a table and checks its low three bits against a value loaded from a global variable.
- `check_grid_for_empty_or_at`: The routine receives two 16-bit coordinates in registers and a pointer in A0.
- `propagate_flag_to_neighbors`: The routine clears a 13-byte work area, then walks a table of short pointers using a running index until a byte mask matches.
- `mark_mines_in_board`: The routine updates a board byte with the high word of a combined value and walks a short offset table to propagate marks to neighboring cells when the cell is marked.
- `propagate_tile_flags`: The routine copies a block of bytes from a source table into a destination table while iterating over a string and a set of short pointers.
- `decode_sprite_pattern`: The routine decodes a sprite pattern by walking a byte table using an index derived from VPOSR and a mask, copying bytes into a buffer and setting flag bytes in a second table.
- `return_stack_value`: The routine returns the value stored on the stack at the caller's frame.
- `count_matching_bytes`: The routine clears a global counter and then scans 48 bytes from an input pointer, incrementing the counter each time a byte matches the value in D0.
- `restore_state_and_write_byte`: The routine saves five global variables, adjusts a counter, then repeatedly calls another routine until a value matches.
- `spot_7up_game`: The routine is four instructions long and appears to be unreachable code.
- `decode_and_store_bytes`: The routine reads a byte from a global variable, uses it as an index into a table, and copies a fixed number of bytes from the indexed location to a destination pointed to by another global variable, advancing the destination pointer.
- `decode_unreached_gap`: The routine is a single instruction that jumps to another routine.
- `store_byte_from_d0_via_pointer`: The routine loads a pointer from a global variable, reads a byte from the address held in register d0, and stores that byte through the pointer.
- `copy_bytes_with_run_length`: The routine copies bytes from a source pointer to a destination pointer using a run-length scheme.
- `decode_bitmask_copy`: The routine reads a two-byte bitmask from the source pointer and then copies bytes from a source buffer to a destination buffer, skipping source bytes whose corresponding bit in the mask is set.
- `configure_blitter_copy`: The routine reads a sprite descriptor from a table and computes blitter source and destination parameters for a copy operation.
- `set_blitter_copy_parameters`: The routine waits for a global flag to clear, then computes a blitter modulo and a blitter size from input values and a table entry, writes those values to blitter registers and to a small structure pointed to by a global pointer, and sets up source and destination pointers for a blitter copy operation.
- `wait_for_flag_and_load_blitter_setup`: The routine spins until a flag in a global variable is cleared, then loads a pointer from a table into a variable and copies several words from that table into custom chip registers used for blitter control and DMA. It also writes constant values to a few of those registers and returns.
- `wait_and_clear_blitter_state`: The routine spins until a global variable clears a flag, then copies a word from a table into a RAM location, computes a 32-bit value from two table entries and stores it, and clears several RAM words.
- `set_blitter_copy_parameters_2`: The routine waits for a global flag to clear, then computes two values from its inputs and a table entry, writes them to chip memory locations and to a RAM variable, and loads blitter registers with a fixed pattern and the computed values to set up a blitter copy operation.
- `copy_tile_row_data`: The routine waits for a flag in a global variable, then walks a byte stream accessed through a pointer in a table, using another routine to pace execution.
- `wait_for_vblank_and_decode_sprite_command`: The routine waits for a global flag to clear and then repeatedly polls a counter until it reaches a threshold, resetting it.
- `decode_script_pattern`: This routine decodes a byte pattern from a table using a running counter modulo eight and copies a 3-element 42-byte block from one buffer to another when a condition on two counter variables holds.
- `decode_script_bytecode`: The routine reads a byte stream from a table pointed to by a register and drives a small interpreter loop.
- `decode_pattern_into_table`: The routine decodes a bit pattern from a word into a table of bytes and then interprets a command byte from a table, performing a small loop that calls another routine and updates a modulo-eight counter.
- `jump_table_dispatcher`: The routine checks a word value and, if the check passes, it uses another word value as an index into a table of routine pointers stored in a global variable.
- `clear_bit_from_four_bytes`: The routine clears a single bit from four bytes in a structure pointed to by a pointer.
- `clear_bitfield_flags`: The routine sets a bit in a byte pointed to by a pointer and clears the same bit in three other bytes located at fixed offsets from that pointer.
- `clear_bit_and_set_flags`: The routine clears a single bit in a byte pointed to by a pointer, then sets the same bit in a byte 42 bytes later in the same structure, and clears the same bit in bytes 84 and 126 bytes later in the same structure.
- `set_bitfield_flags`: The routine modifies four bytes pointed to by a pointer in memory.
- `clear_bit_from_bytes`: The routine clears a bit in a byte pointed to by a pointer and in a byte 42 bytes later, sets the same bit in a byte 84 bytes later, and clears the same bit in a byte 126 bytes later.
- `set_bitfield_flags_2`: The routine sets and clears individual bits in four bytes of a byte array pointed to by a pointer.
- `clear_bit_and_set_flags_2`: The routine clears a single bit in a byte pointed to by a pointer and sets the same bit in two other bytes located at fixed offsets from that pointer, while also clearing the same bit in a fourth byte at a fixed offset.
- `set_bit_flags_in_table`: The routine sets a bit in three bytes of a byte table pointed to by a pointer and clears the same bit in a fourth byte of the same table.
- `clear_bit_flags`: The routine clears a bit in a byte pointed to by a pointer and in two other bytes 42 and 84 bytes later in the same block, and sets the same bit in a byte 126 bytes later in the same block.
- `toggle_bitfield_flags`: The routine uses a byte pointer from a global variable and an index from a register to set or clear a single bit in four byte locations at fixed offsets from the pointer.
- `clear_and_set_bits_in_table`: The routine clears a bit in one byte of a table pointed to by a pointer and sets the same bit in three other bytes of the same table.
- `set_bits_in_byte_table`: The routine sets and clears a single bit in four bytes of a byte table pointed to by a pointer.
- `clear_bit_and_set_flags_3`: The routine clears a single bit in a byte pointed to by a pointer and in the byte 42 bytes later, then sets the same bit in the bytes 84 bytes later and 126 bytes later in the same memory block.
- `set_bitfield_flags_3`: The routine sets a bit in a byte pointed to by a pointer, clears the same bit in another byte 42 bytes later, and sets the same bit in bytes 84 and 126 bytes later in the same memory block.
- `clear_bit_set_flags`: The routine clears a single bit in a byte pointed to by a pointer and sets the same bit in three other bytes located at fixed offsets from that pointer.
- `set_bit_in_four_bytes`: The routine sets a single bit in four bytes pointed to by a pointer.
- `propagate_bit_flags`: The routine updates a set of four bit-mapped byte tables that are indexed from a base pointer using an offset derived from a global variable and an input index.
- `set_bit_in_byte_array`: The routine checks a bit in a byte taken from a computed address using an index and a bit mask derived from a register, and if the bit is set it ORs the same bit mask into another byte at a related address.
- `wait_for_vblank_and_clear_blitter`: The routine waits for a global variable to clear and for a vertical blank condition, then writes values to custom chip registers including blitter control and mask registers and the blitter source and destination pointers and modulo values, and sets the blitter size.
- `clear_blitter_and_wait`: The routine clears several custom chip registers, waits for a global variable flag to clear, programs the blitter control and mask registers from the first argument and a second argument, then waits again for the flag and returns.
- `wait_for_blitter_and_clear_regs`: The routine spins until a global flag bit is clear, then writes constants to several custom chip registers, spins again for the flag, clears another register, spins again, reads a pointer from a table via an uninitialized pointer, adds a constant to it and stores the result, then writes a constant to a register before returning.
- `copy_two_words_from_struct`: The routine checks a flag byte in a structure pointed to by a pointer.
- `decrement_timers_and_update_potgo`: The routine writes two bytes from a hardware register into a global variable and sets a global variable to a constant, then increments a counter and decrements three short counters stored via a pointer, clearing each to zero when it reaches zero, and decrements another short counter via the pointer.
- `clear_memory_block`: The routine clears a block of memory by writing zero bytes to consecutive locations, then calls a number of other routines and invokes two library functions via a table of function pointers.
- `self_recursive_stub`: The routine contains no instructions and therefore performs no operation.
- `initialize_joystick_device`: The routine builds a custom device request structure in memory, sets up fields for a joystick device, and then calls a library routine to open the device.
- `check_and_trigger_callback`: The routine checks a global flag and, if set, writes a constant to a global word, stores a pointer to a global byte in another global word, then calls two routines via function pointers taken from a global variable.
- `register_interrupt_server`: The routine writes a small constant block into a global structure, stores a pointer to itself in that structure, and then invokes a function through a global variable.
- `spot_interrupt_disable`: The routine checks a global flag and, if set, calls another routine via a function pointer stored in a global variable and then clears the flag.
- `check_and_set_flags`: The routine calls another routine repeatedly, storing each non-zero return value into several distinct locations referenced through a base pointer, and aborts early if any call returns non-zero.
- `clear_pending_flags`: The routine checks a set of global flags, calls another routine for each flag that is non-zero and then clears the flag.
- `fill_input_table`: The routine repeatedly calls another routine, stores the returned value into a table and, when the value is non-zero, stores two additional derived values alongside it.
- `clear_flags_and_poll`: The routine iterates over a fixed number of entries in a table of integers located via a global variable.
- `allocate_and_prepare_buffer`: The routine checks a condition via another routine and, if true, stores a pointer derived from a global variable into a global slot, then obtains a buffer from a library allocator.
- `call_indirect_table_entry_8`: The routine calls a function pointer taken from an entry in a global table.
- `initialize_game_window`: The routine sets up a set of window and view structures in memory, allocates a graphics view with Exec AllocMem, and then calls another routine to open the window.
- `clear_pending_callbacks`: The routine clears pending callback flags stored in a global structure by invoking stored function pointers and resetting the flags, then optionally invokes another callback and returns a combined value built from a high word and a short field, or the result of a further function call.
- `call_indirect_table_entry_9`: The routine loads a function pointer from a global variable table and calls the function it points to.
- `open_library_and_chain_calls`: The routine reads a function pointer from a global variable table, calls it, stores the result in a stack-based area and repeats the lookup with the same table.
- `initialize_game_state_2`: The routine initializes a block of memory using values derived from a global variable and copies a set of null-terminated strings into a table.
- `store_abs_value_to_global`: The routine takes a value from a register, replaces it with its absolute value if it is negative, stores the result as a short into a global variable, then calls another routine and returns the original register value combined with another register.
- `conditionally_call_and_clear_flag`: The routine checks a global variable and, if it is non-zero, calls another routine and stores its return value.
- `initialize_pattern_table`: The routine calls another routine via a table and a library function, then either clears a pointer or fills a small table with a repeated value derived from a register and an input value.
- `spot_7up_cleanup`: The routine checks a pointer in a0 and a flag at offset 8.
- `call_indirect_through_table_9`: The routine checks a pointer in a0 and, if non-zero, calls another routine and then calls a function pointer taken from a table addressed by a6 with a fixed negative offset.
- `load_pattern_table`: The routine copies a zero-terminated string from a source pointer into a buffer in a global structure and then attempts to allocate a linked list of pattern entries using Exec memory allocation and DOS library calls.
- `free_linked_list`: The routine walks a linked list of pointers stored in memory, following each non-zero entry and recursing on entries that are not -1, then calls a function pointer taken from a global variable.
- `update_object_state`: The routine updates a set of global variables that track an object’s position and state, copies position words, checks for unchanged position, and handles several state codes by invoking other routines and clearing flags.
- `reset_game_state`: The routine clears a flag and calls two other routines when a global flag is set.
- `clear_global_variable`: The routine stores zero into a global variable via an address derived from a register.
- `clear_flag_and_run_pair_loops`: The routine clears a global variable and then runs a nested pair of loops that call another routine twice per inner iteration, conditionally calling it a third time based on a counter and a stored value.
- `reset_spot_table`: The routine clears a couple of global flags and loads a value from a table indexed by a global index.
- `set_global_flag`: The routine calls another routine and then stores the constant 2 into a global variable.
- `update_animation_timers`: The routine updates two animation timers stored in a global variable.
- `read_joystick_and_disk_status`: The routine initializes a one-time setup via two indirect calls and then reads joystick/mouse fire button and disk status from a hardware register.
- `update_spot_cell_state`: The routine updates a cell entry in a table using the two input indices and two saved indices, computes the absolute differences between the current and saved indices, and decides whether to skip the main update.
- `advance_card_index`: The routine increments a counter and loops over entries in a table pointed to by a global variable, advancing an index and copying two words from the current entry to global variables.
- `store_two_values_and_branch`: The routine stores two 16-bit inputs from registers into two global variables and sets a third global variable to a constant.
- `fetch_tile_byte`: The routine checks that two 16-bit inputs are in the range 1 to 7 inclusive.
- `store_byte_into_table`: The routine stores a byte from a register into a table referenced by a global variable using an index derived from two input values.
- `apply_delta_to_table_entry`: The routine updates two short fields of a table entry indexed by a signed offset in D0, using values from D1 and D2 and a base table.
- `clear_action_flags_and_reinit`: The routine clears a flag, checks a function pointer against itself and, if different, loads a value from a table into a global variable, selects a table of word pointers, writes the selection to two global pointers and a global word, recurses into itself, copies two words from a global pointer into globals, recurses again and sets a global word to one.
- `clear_flag_and_recurse`: The routine clears a 16-bit global variable and, if another 16-bit global variable is non-zero, it calls another routine and then clears that flag variable.
- `reset_global_flags`: The routine checks a global variable and, if non-zero, calls another routine twice, then clears two global word flags.
- `reset_game_state_flags`: The routine checks a global table entry and, if a condition is met, increments a counter in that table, clears a flag, writes constant values to several global variables, calls another routine twice, and sets a few more global flags.
- `update_game_timer_and_score`: The routine updates a game timer and a score display when a flag is set and a counter is negative.
- `clear_global_word`: The routine stores zero into a global variable.
- `skip_frame_wait`: The routine checks a global flag and a counter and may return early.
- `initialize_spot_state`: The routine initializes a set of global variables when a flag is clear, storing two input values into several locations and computing two offsets from them.
- `clear_global_variable_2`: The routine calls another routine and then stores zero into a global variable via an address derived from a register.
- `toggle_spot_state`: The routine toggles a flag in a global variable and, when a set of conditions on other global variables holds, copies values from a group of globals into another group, adjusts counters, flips a bit, and recursively calls another routine twice before restoring a saved value and setting a timer.
- `reset_animation_state`: The routine copies a short value from a global variable to another global location and adds four to it, copies two word values from global variables to other global locations, increments two short values by three and two, sets a flag bit in a global word, calls another routine twice with the flag toggled between calls, restores a saved word value, and writes the constant ten to a global word.
- `initialize_game_state_3`: The routine sets up a small fixed-size table in a global variable, copying a short value from a table indexed by a global variable and marking entries based on whether the value is zero, then clears a 48-byte buffer in another global variable, writing a flag byte when a zero byte is encountered in a short array.
- `draw_spot_grid`: The routine prepares a grid of spot positions by clearing a table, then fills it by calling another routine for each cell and storing coordinates and a type flag.
- `reset_game_state_2`: The routine clears two short fields in a global structure, calls another routine three times, then calls a routine that returns a value used to compute a short offset stored in the structure.
- `process_input_byte`: The routine processes a pending input byte stored in a global variable.
- `update_display_coordinates`: The routine updates two coordinate values stored in global variables.
- `decode_tile_buffer`: The routine updates a 32-word tile buffer pointed to by a global variable based on a mode stored in a global variable.
- `copy_and_adjust_palette_entries`: The routine copies 32 palette entries from a table referenced through a global pointer into a buffer pointed to by a0, adjusting each entry toward a target value by moving the low, high and middle nibbles one step at a time.
- `copy_31_word_table`: When a short value in D0 equals five, the routine copies thirty-one 16-bit words from a table pointed to by a global variable into a buffer pointed to by A0, starting two bytes after the buffer base, and then clears a global variable.
- `read_joystick_and_mouse_axes`: The routine reads the joystick and mouse fire buttons from CIAA_PRA and mouse movement from JOY0DAT, updates four local counters based on button and axis bits, and returns the vertical axis counter.
- `wait_for_vbl_and_poll_input`: The routine waits for a vertical blank condition, clears and polls a synchronization flag, and then repeatedly checks a global variable bit while reading a pair of 16-bit values from a table pointed to by a global variable.
- `set_sprite_frame_and_offset`: The routine selects a sprite frame and offset from a table indexed by the input value and writes the results to a global variable and a global offset.
- `wait_for_input_and_update_sprite`: The routine waits for a global flag, clears a timer, waits for the timer to become non-zero, calls another routine three times, writes sprite control words and toggles a flag, then waits for a countdown to reach zero while polling the flag.
- `reset_state_flags`: The routine clears several word-sized global variables and then repeatedly calls other routines while polling flag words in global memory.
- `append_path_components`: The routine builds a path string in a buffer located via a global variable.
- `call_indirect_via_table_14`: The routine calls another routine through a pointer obtained by indexing a table with an offset.
- `call_indirect_through_table_10`: The routine loads an address from a global variable and calls the routine stored at an offset from that base.
- `call_indirect_through_table_11`: The routine calls another routine indirectly via a pointer stored in a table.
- `call_indirect_via_table_15`: The routine loads a function pointer from a global table and calls it indirectly.
- `process_state_table`: The routine clears a flag in a byte at a fixed offset from a register, scans a small table of entries for a zero value, remembers the first matching entry, then calls another routine for each non-zero entry.
- `reset_table_entries`: The routine clears a two-byte field in a table repeatedly and calls another routine each time, using a counter that runs from a positive start value down to zero.
- `decrement_and_clear_counter`: The routine decrements a short counter stored at an offset from a pointer and, if the counter reaches zero or was already invalid, clears the counter and, when a flag at the base of the structure is non-zero, calls another routine, invokes a function pointer stored in a table, and clears the flag.
- `search_table_for_match`: The routine walks a table of entries using a fixed stride and compares an input value with a field in each entry.
- `parse_resource_record`: The routine prepares a state block and dispatches several calls to another routine to obtain a record header and body.
- `call_other_routine_and_return_zero`: The routine calls another routine and then returns zero.
- `call_and_return_zero`: The routine calls another routine and then returns zero.
- `mask_lower_bytes`: The routine calls another routine and masks the returned value with the lower bytes cleared, returning the result.
- `recursive_call`: The routine calls another routine and then returns.
- `check_and_update_pointer`: The routine calls another routine, checks its result and, if the high word is zero, returns a combined value from two inputs.
- `wait_for_flag_clear`: The routine loops, calling another routine, until a global variable is cleared to zero, then returns the combined value of the two input registers.
- `remove_object_from_table`: If the object pointer is non-zero, the routine clears a status flag in a global structure and iterates over a 64-byte name field, calling another routine for each non-empty byte, then calls it three more times.
- `reset_game_state_flag`: The routine clears a 16-bit value in a global variable and sets a flag byte in the same structure by OR-ing in a constant.
- `copy_struct_to_message`: The routine calls another routine and, if a condition on the returned values and a pointer is met, copies a small fixed-size block from memory pointed to by the pointer into a structure pointed to by A6, stores two 32-bit values and several 16-bit fields from the pointer into the structure, clears a byte in the structure, and calls another routine twice.
- `reset_game_state_and_clear_flags`: The routine clears a global variable and clears a flag bit in a byte at a fixed offset from a base pointer.
- `clear_flag_and_iterate_table`: The routine checks a flag stored in a global variable.
- `clear_and_process_four_tables`: The routine clears four entries of a global table and then, for each entry, copies a short from a pointer table into the table and walks a short list subtracting values from a counter, storing overflow and flag bytes.
- `copy_struct_fields_and_update_table`: The routine copies seven words from one area of a structure to another area of the same structure and then copies a word from one offset to another.
- `execute_callback_and_helpers`: The routine checks a function pointer stored in a structure and calls it if present, then calls three other routines and returns a 64-bit value formed from two input registers.
- `update_displayed_value`: The routine updates a stored index derived from an input word and, when it changes, reads a 16-bit entry from a global table.
- `process_sprite_animation`: The routine updates a set of four sprite records stored in a global structure.
- `process_entity_state`: The routine walks a linked list of entity structures, updates a counter and a pointer in each entity, and writes a byte-derived value into a field.
- `process_message_queue`: The routine walks a linked list of 16-bit words in memory, skipping zero entries and stopping on a sentinel, and writes fields into a per-message block of a global structure.
- `process_object_list_2`: The routine walks a linked list of objects stored in a global variable and updates per-object timers and flags.
- `process_object_list`: The routine walks a small fixed-size list of objects referenced through a global structure, decrementing per-object counters and advancing linked list pointers stored in each object.
- `clear_flag_after_callback`: The routine checks a short flag at an offset from a pointer and a global variable at another offset.
- `signal_on_flag`: The routine checks a global flag and, if set, calls another routine via a function pointer stored in a global variable.
- `process_sound_table`: Iterates a table of entries and invokes callbacks stored in each entry, setting bits in a mask based on their return values.
- `spot_sound_channel_setup`: The routine writes a value derived from an input register to a global variable and then writes the constant 2 to a hardware register at an offset from the custom chip base, which corresponds to the left channel of audio channel 0.
- `find_next_free_slot`: The routine searches a table of 22-byte entries for a free slot matching a byte flag and a zeroed pointer, updates a global index, and writes a pointer, a byte from a register, and a word mask into the chosen entry before returning a combined value.
- `select_next_entry`: The routine walks a table of entries using an index stored in a global variable and skips entries whose flag is zero or whose pointer is null, advancing the index with wrap-around until a matching entry is found.
- `return_stack_value_2`: The routine returns a 32-bit value formed by combining all ones in the high 32 bits with a 32-bit value taken from the stack at the call site.
- `clear_state_and_wait`: The routine clears a global variable and a global word, then repeatedly calls another routine until it returns zero, and returns the value in D0.
- `clear_flag_byte`: The routine computes an address from a global variable and a value in a register, writes zero to a byte at that address, and if the following byte equals one it writes two to the first byte.
- `clear_globals_and_wait`: The routine clears a global variable and a word-sized field in a structure pointed to by a register, then repeatedly calls another routine until it returns zero, and returns the value in D0.
- `clear_slot_and_notify`: The routine computes a byte offset from the input value and a base pointer, clears a byte at that slot, and if the following byte is non-zero it marks the slot, calls a callback stored four bytes after the slot via an indirection through a pointer, clears the next byte and the slot byte, then returns the input value.
- `set_table_entry_2`: The routine writes to a global variable and, when a divisor is non-zero, computes a scaled difference between a stored value and an input, divides it, and stores the result together with the scaled input as a 32-bit entry.
- `update_sprite_x_position`: The routine updates a word at the start of a structure using a value derived from the structure's fields, handling sign based on field comparison and clearing a flag.
- `clear_flag_and_propagate_value`: The routine reads a word from a table entry, clears a flag byte in the same entry and, if the flag was set, calls another routine and uses the returned word.
- `check_global_flag_and_return`: The routine calls another routine and, if a returned pointer is non-zero, calls another routine and tests the high word of its result.
- `decrement_and_test_counter`: The routine calls another routine and, if the result is non-zero, treats it as a pointer to a structure.
- `decrement_counter_and_branch`: The routine calls another routine, checks the returned pointer, reads a short from an offset in the pointed-to block, decrements it, writes it back, and if the result becomes zero it calls another routine.
- `copy_struct_fields_and_clear_flag`: The routine copies several fields from a structure pointed to by a1 into a destination pointed to by a2 when a condition on d7 is met, then returns the original value of one of the source fields after clearing that field in the source structure.
- `copy_sprite_data`: The routine walks a table of pointers and compares a 31-word block from each entry with a reference block, then updates reference counters in a structure pointed to by a field at offset 470 and calls another routine to obtain a new reference.
- `increment_table_entry`: A routine that initializes a table entry to zero, then walks a list of structures via a pointer table on the stack, calling another routine for each entry that matches a global variable pointer, and increments a short counter at an offset from a linked value.
- `decrement_counter_and_branch_2`: The routine reads a pointer from an object field, checks it is non-zero, then loads a short from an offset in the pointed-to structure, decrements it and writes it back.
- `decrement_counter_and_call`: The routine reads a pointer from an object field, checks it is non-zero, then loads a short from an offset in the pointed-to structure, decrements it and writes it back.
- `initialize_audio_device`: The routine obtains a pointer from another routine and, if present, writes several fields in a structure that matches the audio.device open request and then opens the audio.device.
- `check_a6_and_call_routines`: The routine checks a register value and, if non-zero, calls another routine four times in a row.
- `reset_library_device`: The routine clears a device handle and closes the associated library device, then frees a signal and invokes other cleanup.
- `spot_signal_wait`: The routine calls another routine, then invokes a function from a table in a global variable, calls another routine, invokes a second table function with an argument from the stack, calls another routine, and returns a value formed by combining the input register with the result of the table call.
- `store_task_and_flags`: The routine clears a word in a structure pointed to by A6 and, if a flag in D1 is non-zero, stores a value from D0 into the structure, calls another routine via a function pointer taken from a global variable, stores the returned value into the structure, copies the flag from D1 into the structure, and returns the original D0 and D1 values concatenated.
- `compare_strings_case_insensitive`: The routine compares two byte strings character by character, converting lowercase letters to uppercase before comparison, and advances both pointers until a mismatch or a zero terminator is found.
- `spot_7up_game_2`: The routine calls another routine and returns the value left in address register A0 by that call.
- `allocate_memory_block`: The routine obtains a memory block pointer from a function pointer stored in a global variable or from a table entry, then writes a size value derived from an input register into the block and returns a combined value from two input registers.
- `call_cleanup_routine`: The routine checks a pointer in a global variable and calls either a stored cleanup routine or a fallback routine from a table, then returns the combined values from two registers.
- `call_and_return_a0`: The routine calls another routine and then returns the value that the call left in address register A0.
- `check_and_branch_on_result`: The routine calls another routine and tests its return value.
- `empty_stub_routine_2`: The routine consists of six instructions that perform no observable work and end with a return.
- `negate_if_negative`: The routine calls another routine and receives a value in D1.
- `call_and_return_concatenated_value`: The routine calls another routine and then returns a 64-bit value formed by concatenating the routine's D1 result with a local copy of D1.
- `multiply_16bit_divide_16bit`: The routine takes two 32-bit values in registers and treats them as 16-bit numerator and denominator.
- `swap_and_rotate_words`: The routine swaps the high and low words of a value in a register, then repeatedly doubles a pair of words while propagating carry, subtracting a constant if the intermediate value exceeds a threshold, and decrements a counter fifteen times.

### Spot - 7-Up Game (1990)(Virgin Mastertronic)[cr QTX].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
