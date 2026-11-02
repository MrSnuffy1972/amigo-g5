# Game 1868 (flashback (1992)(u. s. gold)[cr interpol]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01868). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3604480 top-level bytes in 178 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 10 of 10 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 672 of 672 named (0 author symbols), 672 of 672 described (6 by rule, 639 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 15148 of 15148 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 30016 of 30016 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3001688 |
| code | 3540 |
| data | 406498 |
| padding | 46322 |
| unused | 146432 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad | 1 | 222 | 21 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL | 2 | 267 | 8 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL#unpacked | 1 | 1200 | 28 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback | 7 | 63 | 1 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback#unpacked | 5 | 27535 | 576 |
| explode.library | 1 | 475 | 30 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:Failat | 2 | 117 | 2 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:endrun | 1 | 117 | 4 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:bootblock | 1 | 10 | 1 |
| Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4).adf:bootblock | 1 | 10 | 1 |

## Cases handled

- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL: cracktro self-loader: PowerPacker-style backward bit stream (offset widths 9,10,11,11) and compact relocations, unpacked by our port of the game's own depacker (14180 -> 33324 bytes): unpacked by tools/ppunpack.py, a line-by-line port of INTERPOL's own decruncher (+1CC shift_byte_buffer) and loader (+2C..+1A0, compact 03EC relocation groups): the 13,496-byte stream is consumed exactly, the 33,076-byte image parses to its last byte (one 32,940-byte code hunk, 83 relocations), and relocated at an address it equals the reference run's chip RAM in 32,543 of 32,940 bytes (the rest are the cracktro's own variables changed while it ran)
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback: Imploder V6 library mode (needs libs/explode.library), unpacked by our port of the game's own depacker (83312 -> 190628 bytes): exploded by tools/explode.py, a line-by-line port of explode.library's own routines (+2FC descriptor, +3CA hunk filler, +44E decruncher): the 82,920-byte packed hunk is consumed to its first byte, the 190,564-byte output is consumed exactly by the hunk filler (5 hunks, 5,288 relocations), and the reference harness runs the result through the protection, intro, title and level 1
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +9C..A8: data by evidence ("dos.library", taken by the start-up code with pea); 0 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +B8..CA: data by evidence ("intuition.library": inline argument after a call to the open-the-library-named-after-the-call helper); 8 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +FA..10E: data by evidence (" already resident!": inline text after a call to the print-the-text-after-the-call helper); 6 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +116..126: data by evidence ("explode.library": inline argument after a call to the open-library helper); 7 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +132..142: data by evidence (" initialized!": inline text after a call to the print helper); 7 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +14C..15E: data by evidence (" not available!": inline text after a call to the print helper); 7 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad +298..2A2: data by evidence ("*** BREAK": text printed on Ctrl-C); 0 instructions decoded there were removed
- flashback (1992)(u. s. gold)[cr interpol](disk 3 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 3 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr interpol](disk 4 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 4 of 4).adf: boot area holds bytes but the checksum is invalid, so Kickstart never runs it
- flashback (1992)(u. s. gold)[cr interpol](disk 1 of 4)/Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL
- Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL#unpacked

## Copy protection found

- none found

## Functions


### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:ExpLoad

- `initialize_expansion_loader`: The routine sets up a small stack frame and calls a loader function from a table pointed to by a global variable.
- `set_expansion_loader_state`: The routine stores a pointer to a global variable in a structure pointed to by A5 and writes the constant 100 into another field of that structure.
- `store_explode_library_pointer`: The routine stores an incoming value into a global variable and then calls another routine with a constant.
- `find_explode_library`: The routine searches the Amiga OS library list for a library named explode.library using exec.FindName and stores the resulting library base pointer in a global variable.
- `report_explode_library_open`: Explode loader: stores the explode.library base returned by OpenLibrary and prints "explode.library initialized!" when it opened, or "not available!" when it did not.
- `call_hook_and_clear_flags`: Checks a pointer stored in a global variable; if non-zero, it invokes a function pointer located at an offset from that address.
- `clear_flag_and_call_hook`: The routine checks a flag stored at an offset from a global variable using a short index in D0.
- `self_recursive_call_with_callback`: The routine calls itself and then invokes a function pointer stored in a global variable table.
- `call_other_routine_sequence`: The routine calls another routine five times in a row with different arguments, passing a stack slot, a fixed address, an offset from a register and a constant, and returns.
- `call_indirect_via_table`: The routine calls another routine through an indirect pointer obtained from a global variable and returns a 64-bit value formed by concatenating the two input registers.
- `store_byte_and_advance_pointer`: Stores a byte from a register into the memory location pointed to by a global pointer.
- `store_return_values`: The routine calls another routine, captures its return value and an extra output pointer, and stores both into two consecutive words of a caller-provided structure via a pointer in A4, then returns the original input value in D0.
- `skip_string_table_entry`: The routine walks a short string table, finds the length of the first string by counting words until a zero short, then updates a pointer to the next entry and returns an offset for the next entry.
- `parse_string_table_entry`: The routine reads a zero-terminated string from a pointer table, advances the pointer past the string and returns the byte offset of the next entry.
- `set_signal_flag`: The routine calls another routine and tests a returned value for a specific bit.
- `clear_flag_in_structure`: The routine calls another routine and then writes a constant to a field of a structure pointed to by a register.
- `check_and_invoke_callbacks`: The routine calls another routine and then checks a zero flag.
- `store_param_in_offset`: The routine calls another routine and then stores the input parameter into a global variable at a fixed offset from a pointer returned in a register.
- `call_indirect_via_global_table`: The routine calls another routine through a pointer stored in a global variable.
- `count_string_bytes`: The routine receives a pointer in a0 and walks forward byte by byte until it finds a zero byte, then returns the number of bytes from the start pointer to the terminating zero.
- `do_nothing`: The routine is empty.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL

- `cracktro_self_loader`: Interpol cracktro start: decrunches the data hunk into a hunk image (cracktro_decrunch), lays its hunks out with AllocMem, applies its compact relocations and runs it.
- `decrement_until_zero`: The routine receives a 16-bit value in D1 and repeatedly subtracts one from it until it reaches zero, effectively consuming the counter.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `cracktro_decrunch`: Decrunches the cracktro's data hunk backwards in place: a PowerPacker-style bit stream with literal runs and back references whose offset widths come from the table 9, 10, 11, 11.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `extract_bits_to_word`: The routine extracts a series of bits from a byte stream pointed to by a pointer, using a bit buffer in a register, and assembles them into a 32-bit word in a register.
- `negate_byte_via_pointer`: The routine negates a byte pointed to by a parameter using a register value, ORs a byte pointed to by a pointer with a constant, copies an int from a global variable to a pointer target, and writes a byte derived from a comparison to memory.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:INTERPOL#unpacked

- `initialize_blitter_and_wait`: The routine programs the Amiga blitter registers BLTCON0, BLTCON1, BLTBMOD, BLTBPTH, BLTDMOD, BLTDPTH and BLTSIZE and then repeatedly polls CIAA_PRA while performing a small blitter copy loop that moves bytes from a table into a set of memory locations.
- `increment_timer_counter`: The routine reads a 16-bit counter from a global variable and, if the counter is zero, it resets the counter to four and increments two 32-bit values built from word pairs in the same global structure, returning the high word of the second value.
- `other_routine`: The routine contains no instructions and returns immediately.
- `decrement_two_word_counters`: The routine checks a global variable and, when it equals four, clears it and decrements two 32-bit counters stored as paired 16-bit words in a global structure, updating both halves.
- `empty_routine_2`: The routine contains no instructions and returns immediately.
- `decode_script_bytes`: The routine reads bytes from a source pointer, skipping newline characters by advancing the destination pointer by a fixed stride, and stops when it encounters an escape byte.
- `decode_text_to_tile_data`: Iterates through a text buffer, skipping newline characters by advancing a destination pointer by a fixed offset.
- `build_copper_list_and_start_dma`: The routine builds a copper list in chip memory by writing word pairs that form copper move and wait instructions, using three pointers obtained from another routine and offsets added to two of them.
- `initialize_dma_and_coprocessor`: Opens the graphics library and allocates memory.
- `wait_for_global_flag`: The routine spins until a global variable no longer has a specific bit set, then returns.
- `wait_for_global_flag_clear`: The routine spins in a loop until a global variable has its bit cleared, then returns.
- `wait_for_vblank`: The routine reads the vertical and horizontal position status register and repeatedly copies a global variable to itself until the variable is zero, effectively spinning until a condition on that global variable is met.
- `reset_input_state`: The routine walks a table of bytes starting at a fixed address, comparing each byte with a running maximum and using a counter to decide when to stop.
- `disable_dma_channels`: The routine writes to a global variable five times, the last write storing a value that sets bit 15.
- `reset_animation_frame_counter`: The routine increments a global counter and, when the counter wraps, it resets several globals and calls another routine four times.
- `update_display_window_stop`: The routine updates a hardware display window stop register and related control words using values from a global variable and several RAM locations.
- `update_sprite_animation_state`: The routine updates animation state for an entity using a table lookup and writes values into a structure.
- `initialize_interrupt_table`: This routine writes a value from a register to a global variable, stores a constant to a global variable, and stores a pointer to itself in a global variable.
- `reset_audio_dma_pointers`: The routine copies eight words from a table in memory to a block of global variables and then writes four values to a global variable.
- `process_timer_flags`: The routine reads flag bytes from a structure pointed to by an argument register and updates a timer word in the same structure using a global variable and a small table.
- `copy_word_to_a5_offset`: The routine copies a 16-bit word from an address derived from a register plus a constant offset to an address derived from another register plus a constant offset.
- `set_collision_flag`: The routine looks up a value from a global table using an index derived from a mask of a pointer, then writes that value into a field of a structure pointed to by a register and clears a byte in the same structure.
- `update_timer_and_lookup_value`: The routine updates a short timer field in a structure pointed to by a register using a byte delta, clamps it against a limit field and clears the limit when reached, then builds an index from the limit and a byte field, masks it, and looks up a value in a global table using the timer as a threshold.
- `update_animation_counter`: The routine reads a byte from a structure, indexes a global table or computes a signed offset, scales it by a nibble, adds a base offset, writes the result to another structure, and updates a counter byte in the structure.
- `decode_input_byte`: The routine decodes a byte from a table using a value derived from another routine and the CIAA_PRA register, updates several fields in a structure pointed to by a register, and returns a pointer or value derived from the decoded byte.
- `mask_byte_to_offset`: The routine masks a byte from a global variable with a constant and stores the result into another byte of the same global variable.
- `negate_and_advance_string`: The routine updates a signed byte field using a nibble from a byte at offset 1F and a table of values, then clears the field if its high bit is set.
- `other_routine_2`: The routine contains an infinite loop with no operations.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback

- `initialize_game_state`: The routine checks a global variable and a flag, then calls another routine from a table and handles an error path.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:flashback#unpacked

- `program_entry_jump`: Program entry of the exploded game: jumps straight on to game_main.
- `clear_memory_and_print_strings`: The routine clears a block of RAM to zero and then repeatedly writes ASCII bytes into a small buffer and invokes another routine, effectively printing a sequence of status lines.
- `clear_ram_and_print_text`: The routine clears a large block of RAM to zero and then repeatedly writes ASCII bytes into a set of global variables and calls another routine, effectively printing a series of text lines to the screen.
- `concatenate_two_globals`: The routine returns the 64-bit value formed by concatenating the contents of two global variables.
- `decode_hex_digit_string`: This routine decodes a length-prefixed nibble stream in D0, turning each nibble into an ASCII character via a simple offset table and storing the result through A0.
- `game_main`: The game's main program: clears the game state, takes over the hardware, loads the font and sprites, plays the logos cutscene, runs the protection screen and the intro, then the title menu and the game.
- `initialize_hardware_and_memory`: Resets global variables and clears memory buffers while configuring audio volumes, copper lists, and DMA control registers.
- `reset_game_state_2`: The routine clears a set of global variables and then, under a specific condition, performs a loop that XORs a block of memory with a constant and zeroes a large table, effectively resetting the game state.
- `initialize_game_state_3`: The routine clears and initializes a set of global variables and then enters a loop that repeatedly calls another routine, updates a pointer into a table, checks flags in a global variable and decrements a counter, and returns a value from a called routine when a condition is met.
- `iterate_entity_table`: The routine walks a table of pointers, and for each non-zero entry it reads two short fields from the pointed structure, scales and masks them, stores the results in two globals, and calls another routine with the pointer.
- `update_sound_channels`: The routine iterates over a global table of entries and calls another routine for each non-zero entry, then walks a linked list of structures updating a global variable and writing a flag byte into each structure.
- `reset_global_state`: The routine clears a global variable and a second global variable to zero and then returns.
- `describe_routine_0_1_a46`: The routine contains 83 instructions.
- `update_animation_flags`: The routine updates a byte flag field and a word field in a structure pointed to by its parameter.
- `apply_movement_delta`: The routine updates a short pair in the parameter structure using a delta taken from a table indexed by the first short of the parameter.
- `apply_object_transform`: The routine updates the two short fields at offsets 1 and 2 in the parameter structure by adding signed delta values taken from a table indexed by a byte at offset 3, with the sign optionally inverted by a flag.
- `play_cutscene`: Plays the cutscene whose number is in the pending-cutscene word (if any and not already played): loads its .cmp file, depacks it and runs its command and polygon streams.
- `object_crossing_room_edge`: Unreached object test: for an object in another room than the current one, checks its position against the room-link table to decide whether it crosses into the current room.
- `call_subroutine_once`: This routine calls another routine and returns.
- `process_unreached_code_gap`: This routine resides in an unreached code gap and is called by two other unnamed routines.
- `check_dialogue_condition`: The routine walks a table of dialogue entries indexed by a global variable and a short from the first parameter.
- `check_dialogue_tag_match`: The routine walks a table of dialogue entries using a pointer in a register and counters in registers, comparing a byte at offset seven and a short at offset six against a target value in a register and returning success flags for specific tag values.
- `match_keyed_string_table`: Scans a table of string entries for a keyed match using flags passed in registers and a pointer.
- `return_constant_one`: The routine returns the constant value 1.
- `return_zero`: The routine is not empty.
- `find_matching_dialogue_entry`: The routine looks up a table of dialogue entries using an index derived from a parameter block and compares a short value from the block against an entry in the table.
- `resolve_entity_by_index_chain`: The routine takes a pointer and an entity pointer, reads a byte field from the first pointer, then follows a chain of indices stored in a global table.
- `swap_byte_fields`: The routine copies a byte from a third argument to an offset in the second argument and then swaps a byte between the first and second arguments, using an offset in the first argument when the third argument equals the first argument, otherwise using a fixed offset in the first argument.
- `swap_byte_flags`: The routine writes a constant byte to an offset in a structure pointed to by a pointer, then conditionally copies a byte between two structures and marks the source byte as unused.
- `compare_and_sync_byte`: The routine reads a byte from an object at a fixed offset, obtains a pointer from another routine using a global variable and the object, then reads a byte from that pointer at a fixed offset, defaulting to a global byte if the pointer equals the global variable.
- `propagate_collision_flags`: The routine checks a signed byte at an offset from a pointer and, if it is non-negative, calls another routine with that pointer.
- `compare_and_propagate_entry`: The routine checks a byte at an offset in the input structure; if it is not a sentinel value it indexes into a global table using that byte and calls another routine with the table entry and the input structure.
- `set_flag_and_forward_call`: The routine forwards its first argument to another routine using a table lookup based on the high word of its second argument, then stores a constant byte at an offset from the first argument and returns a constant.
- `check_collision_and_propagate_flags`: Looks up an index for a linked item using a parameter.
- `search_linked_list_for_id`: The routine traverses a linked list of structures using a global variable as a base pointer.
- `self_linking_table_search`: The routine walks a self-linking table of pointers starting from a pointer in a register.
- `search_link_and_propagate_flags`: Traverses a linked list by following pointers until a null terminator is found or a specific short value matches a register.
- `copy_entity_state_from_table`: The routine copies a set of fields from a table entry indexed by the high word of the second argument into a structure pointed to by the first argument, merges a flag from the table into a byte field of the structure, copies one byte from a location derived from a pointer inside the structure, then calls another routine and returns a constant.
- `find_entry_in_linked_table`: The routine uses a byte from the first argument to index into a global variable table and follows a linked list of entries until it finds one whose second field matches the short field of the second argument.
- `follow_linked_list_until_match`: The routine follows a linked list via a pointer in A2, dereferencing it repeatedly until the pointer becomes null or the word at offset six matches the value in D2.
- `return_zero_2`: The routine is empty and simply returns a constant zero value.
- `find_entry_in_linked_list`: The routine walks a linked list whose head is taken from a global variable indexed by a byte from the argument.
- `follow_linked_list_until_match_2`: The routine follows a linked list via a global pointer, dereferencing it repeatedly until the pointer becomes null or the word at offset six matches a register value.
- `return_constant_zero`: The routine contains four instructions and returns a constant zero.
- `check_table_byte_match`: The routine checks a global variable against a fixed value and, if it is not that value, compares a byte from a table indexed by the variable with a byte from the second argument.
- `clear_global_variable`: The routine stores the address of a global variable into a four-byte global variable and then writes zero to the global variable.
- `validate_palette_index`: The routine adjusts a 16-bit offset in a structure using a signed byte from a table entry, then checks a signed byte from a global table for a non-zero sentinel and returns -1 if found.
- `find_index_in_table`: The routine searches a table of short values pointed to by a global variable for a match with a short field from the argument.
- `update_sprite_palette_entry`: Iterates through a global table to validate and update palette indices for a sprite object, linking the object to a specific palette entry and handling collision or state flags.
- `build_palette_entry_table`: This routine iterates through data to validate palette indices and populate a table with pointers and associated values.
- `update_entity_link`: The routine updates a linked list of entity structures and sets a few global flags based on a byte field in the entity.
- `empty_loop_routine`: The routine contains no reachable instructions and ends in an infinite loop, so it performs no observable work.
- `parse_input_stream`: The routine reads a byte from a register and stores it into a global variable table indexed by a high byte from another register and a counter, handling tab and newline characters specially and resetting counters.
- `decode_input_buffer`: The routine processes a byte from a register using a table index built from the high byte of another register and a counter, storing the byte into a global table and maintaining a small state counter.
- `process_input_buffer`: The routine processes a byte from a register through a pointer and updates a global variable and a table entry, then calls another routine and copies a string from a global variable to a fixed buffer with a terminator, and performs further buffer and table manipulations using another routine.
- `copy_sprite_state`: The routine copies a 16-bit pair and a byte from one sprite slot to another and then updates several fields in the destination slot, including a flag byte and a short value taken from a global table indexed by the destination slot.
- `remove_entry_from_linked_list`: The routine removes an entry from a singly linked list stored in a global table.
- `clear_and_link_table`: The routine clears a block of memory that holds a global variable and then builds a singly linked list of entries in that block, linking each entry to the next and terminating the list with a null pointer.
- `random_number`: Random number generator: doubles the 32-bit seed and XORs an address into it when a bit is carried out or the result is zero; returns the low 16 bits.
- `decode_byte_runlength`: A routine that decodes a byte stream using run-length encoding where a marker byte indicates a repeat count.
- `reset_game_state_3`: The routine repeatedly calls another routine until a comparison of two returned values is satisfied, then calls that routine twice more, clears a global variable and stores a computed difference into the next word of that variable, and returns a concatenated value built from a global pointer and a RAM address.
- `encode_longest_match`: Scans a data buffer to find the longest repeated substring and calculates the corresponding length and distance values.
- `compare_and_update_minimum_match`: The routine compares a fixed source buffer against successive positions in a destination buffer, tracking the length of the longest common prefix for each alignment.
- `find_longest_common_substring`: The routine scans a byte buffer for the longest repeated substring and updates a global variable with the best length and offset found so far.
- `call_other_routine_and_return_one`: The routine calls another routine and then returns the constant value one.
- `shift_bits_to_memory`: The routine shifts bits from a value built from an input word and a global variable into a destination buffer pointed to by a pointer in a register, writing one word per iteration and stopping early on a carry flag.
- `empty_routine_3`: The routine is empty.
- `shift_bits_to_word_table`: The routine shifts bits from a packed source into a destination word table.
- `shift_bits_into_table`: The routine shifts bits from a 32-bit value in a register into a destination table via a pointer, writing one 32-bit word per iteration and repeating for a count derived from an input word.
- `shift_merge_words`: The routine merges two 32-bit values by repeatedly taking the least significant bit of one word and the most significant bit of another, shifting both words, and writing the intermediate result through a pointer that is advanced each iteration.
- `swap_linked_list_entry`: The routine checks a byte from a structure against a value in D0 and, if they differ, walks a singly linked list rooted in a global table entry.
- `replace_linked_entry`: The routine checks a byte stored at an offset in a structure against a value in D0.
- `swap_linked_list_entry_2`: The routine checks a byte from a pointer against a short input and, if they differ, walks a singly linked list rooted in a global table entry to find the node pointed to by the pointer.
- `fetch_palette_index`: The routine takes a pointer and a small structure, adjusts two fields of the structure using two global values and optionally negates one field based on a global flag, then indexes into a global table using a byte from the pointer and the adjusted fields to return a signed byte as a short.
- `fetch_palette_indexed_byte`: The routine computes two signed offsets from its inputs and a global variable, then uses a byte from memory pointed to by its pointer argument as an index into a global table to obtain a base offset.
- `mark_tile_animation`: The routine computes a pointer into a global table from fields of the first argument and a second argument, then searches a linked list of animation descriptors for a matching entry.
- `allocate_tile_buffer_entry`: Checks a character code from the input structure and searches a linked list for a matching buffer pointer.
- `clear_sprite_cells`: The routine computes a pointer into a global table from fields of a structure and a byte value, then searches a linked list of entries for a matching pointer.
- `copy_bytes_from_table_entry`: The routine walks a linked list of table entries stored in a global variable until it finds an entry whose second field matches a computed address derived from a global variable and fields from the first parameter and the second parameter.
- `copy_bytes_from_stack`: The routine copies bytes from a source address on the stack to a destination pointer taken from the stack, using a byte count taken from the stack.
- `copy_animation_state`: The routine copies a small block of fields from a source animation entry to a destination entry selected by an index, then calls another routine with the source pointer.
- `store_word_and_maybe_trigger`: The routine stores a word from a register into a table entry via a pointer and then calls another routine unconditionally.
- `check_object_relationship`: The routine reads a 16-bit index from an object structure and walks a global table of entries, following linked lists via pointers stored in each entry.
- `find_entry_by_second_byte`: The routine walks a linked list stored in a global table using a byte index.
- `decrement_global_index`: The routine receives two parameters and stores the second parameter minus one into a global variable.
- `call_three_routines`: The routine invokes three other routines sequentially.
- `check_linked_list_entry`: The routine uses a byte from the first argument as an index into a global table to obtain a pointer to a linked list.
- `lookup_table_entry_by_index`: The routine uses a byte from the input structure as an index into a global table and walks a linked list of entries until it finds one whose short field matches a value from the second parameter.
- `check_linked_list_for_flag`: The routine indexes a global table with a byte from the argument structure and follows a linked list of entries.
- `check_table_entry_match`: The routine walks a linked list of entries indexed by a byte from the first parameter, using a global variable as a base.
- `check_linked_list_entry_2`: The routine walks a linked list via a pointer in A1, dereferencing it until it reaches a null pointer or finds a node whose two short fields at offset six match the values in D0 and D1.
- `return_zero_constant`: The routine is empty and returns the constant zero.
- `call_other_routine_with_one`: The routine calls another routine passing the incoming argument and the constant 1.
- `check_table_entry_match_2`: Passes the first argument and the constant value 2 to a table matching routine.
- `call_other_routine_with_constant`: The routine calls another routine with the original argument and the constant 3.
- `check_table_entry_match_3`: Passes the first argument and the constant value 4 to a table matching routine.
- `invoke_callback_and_return`: The routine calls another routine with an argument derived from a pointer stored in a global variable and then returns a constant value.
- `flashback`: The routine receives a pointer, reads a 16-bit value from an offset within the pointed-to structure, adds a constant offset to obtain a second pointer, and calls another routine with that pointer.
- `call_other_routine_and_return_minus_one`: The routine calls another routine with an argument derived from a pointer stored in a global variable, then returns a constant value of minus one.
- `invoke_callback_and_return_minus_one`: The routine reads a pointer from a global variable via an offset from its parameter, then calls another routine with a value derived from that pointer and returns a constant negative one.
- `check_and_call_other_routine`: The routine checks a byte at an offset from the input pointer.
- `unrolled_chain_handler`: The routine reads a byte from a structure field and loops while that byte is not -1.
- `iterate_table_until_sentinel`: The routine repeatedly calls another routine with a byte taken from an offset in the current pointer, then updates the pointer from the callee’s output and continues while a sentinel byte at an offset in the new pointer is not -1.
- `propagate_collision_flag`: The routine calls another routine with its parameter and, if the call returns non-zero, it reads a byte from an offset in the parameter and a byte from an offset in the returned value, combines them into a 16-bit value, and passes that value to the same other routine.
- `store_word_to_struct_field`: Writes a 16-bit value from the second argument into a structure field located at offset 10 from the pointer in the first argument.
- `decrement_counter_and_compare`: The routine decrements a 16-bit counter stored at an offset from a pointer and compares it with a 16-bit field from a second argument.
- `return_zero_constant_2`: The routine is empty and returns the constant zero.
- `copy_word_from_table`: The routine reads a 16-bit word from a table pointed to by a global variable and stores it into a structure field.
- `set_flag_and_table_pointer`: The routine reads a 16-bit index from a table pointed to by the first argument and sets a flag byte in a global byte array.
- `clear_collision_flag`: The routine looks up a 16-bit index from a table pointed to by the first argument and clears a 32-bit entry in a global variable array at that index, then clears a bit in a byte table at the computed offset.
- `clear_entity_flags`: The routine clears a flag byte in the structure and writes a constant to another byte field, then clears an entry in a table indexed by a byte field from the structure, and finally resets a global variable to zero before returning a constant.
- `copy_byte_from_table_and_flag`: The routine reads a byte from a table pointed to by a pointer stored in the argument structure and writes it into a byte field of the structure.
- `check_divisibility_flag`: The routine checks whether a second parameter field is non-zero and, if so, calls another routine to obtain a value.
- `detach_object_from_room`: Object handler: marks the object as in no room, clears its active flag and its slot in the per-object table, then finds the linked object in the object chain and swaps their link flags.
- `compare_byte_and_call`: The routine compares a byte from a table indexed by a global variable with a byte from an input buffer and calls another routine if they match, then returns a constant.
- `compare_byte_to_global`: The routine checks whether the byte at offset seven from the supplied pointer equals a global variable.
- `return_zero_stub`: The routine is empty and returns zero.
- `compare_byte_to_global_2`: The routine checks a byte at an offset from a pointer against a global variable and returns a non-zero value if they differ, otherwise zero.
- `set_global_if_unset`: The routine checks a global variable and, if it is zero, stores a field from the second parameter into that global variable.
- `initialize_global_state`: The routine checks a global variable and, if it is zero, reads a short from a structure pointed to by the first argument, increments it and stores the result in the global variable.
- `set_global_if_flag_clear`: The routine checks a global variable and, if it is zero, copies a field from the second parameter into another global variable.
- `compare_global_to_argument`: The routine receives two arguments and compares a global variable with a field of the second argument.
- `return_zero_3`: The routine contains no instructions and immediately returns zero.
- `adjust_tile_column`: The routine adjusts a 16-bit column value stored in the structure pointed to by the first argument, using a signed step derived from a short field and a direction flag in a global variable.
- `add_offset_to_short_table`: The routine adds a 16-bit offset from the second argument to a short field at a fixed offset in the first argument’s structure and then iterates over a byte-indexed table.
- `accumulate_table_entries`: The routine repeatedly reads a byte from a global table using an index derived from a register, adds a register value to a short at a related offset in the same table, and updates the index with the byte read, looping until a sentinel byte is encountered.
- `clear_entity_flags_2`: The routine writes a constant byte to an offset in the structure pointed to by the argument, clears a specific bit in another byte of the same structure, and stores a zero word into an entry of a global table indexed by a byte from the structure.
- `retry_until_success_or_fail`: The routine repeatedly calls another routine with the same argument until it returns a non-zero value or four consecutive zero results are obtained.
- `apply_table_value_to_struct`: The routine checks a flag byte in the structure and, if it is not set to the sentinel value, loads a 16-bit value from a global table indexed by that flag byte and stores it into a field of the structure.
- `update_object_timer`: The routine reads a byte from an object structure and uses it as an index into a global table of short values.
- `check_and_return_flag`: The routine calls another routine twice and checks the returned byte.
- `return_zero_stub_2`: The routine contains no instructions and therefore performs no operation.
- `check_other_routine_result`: The routine calls another routine and stores the returned byte in a local variable.
- `return_constant_3`: The routine is empty and returns a constant value.
- `check_input_byte`: The routine calls another routine and tests the returned byte.
- `empty_routine_4`: The routine contains no instructions and therefore performs no operations; it returns immediately.
- `clear_flag_on_mismatch`: The routine checks whether two integer parameters are equal.
- `reset_and_forward_flag`: The routine clears a global variable to zero, calls another routine with the incoming argument, and returns the cleared global variable.
- `compare_second_byte`: The routine calls another routine with one argument and compares the returned short to the second byte of the second parameter.
- `lookup_item_by_type`: The routine checks a byte field in a structure to index into a global table of linked entries.
- `find_linked_item_index`: The routine reads a byte from the input structure and uses it as an index into a global table of linked items.
- `check_ram_flag_and_table_entry`: The routine reads a byte from an offset in the argument structure and compares it with a constant stored in RAM. If the byte matches, it compares a short from the structure with a RAM value scaled by a constant; otherwise it indexes a global table with the byte and checks the entry against the same constant.
- `compare_ram_byte_to_table`: Reads a byte from an offset in a register, checks if it is a printable character, and compares it against a value in a global table indexed by that character.
- `check_ram_byte_and_table`: The routine receives a pointer and examines a byte at offset seven.
- `check_char_table_match`: Reads a byte from an offset in the first argument pointer and checks if it is a printable character.
- `check_collision_range`: The routine compares a short field from the first argument with a global short, using a flag to choose the comparison direction, and returns a failure code if the value is outside a range defined by a second argument and a global short.
- `compare_short_fields`: The routine compares two short values located at offset two in two memory blocks pointed to by two registers.
- `compare_word_offsets`: The routine compares two 16-bit values located at offset 2 in two input structures, optionally swapping the order based on a global variable, and computes their signed difference.
- `compare_word_range`: The routine compares two 16-bit values located at offset 2 from two pointers and checks whether the difference is less than a 16-bit limit in D0.
- `check_byte_range_and_table_match`: The routine checks that a second parameter field is zero and that a byte at offset seven from a pointer is in the range 0 to 63.
- `compare_byte_at_offset`: The routine compares a byte from a global variable at a fixed offset with a byte from a table pointed to by a pointer.
- `check_collision_range_2`: The routine compares a short field from the first argument with a global short value, using a flag to decide the comparison direction, and returns a sentinel value if the difference is within a scaled threshold derived from the second argument.
- `compare_short_fields_2`: The routine compares two short values located at offset two in two structures pointed to by the first two arguments.
- `compare_word_offsets_2`: The routine compares the signed 16-bit values located two bytes after two pointers, choosing the subtraction order based on a global variable.
- `compare_word_range_2`: The routine receives two pointers and a word in D0.
- `check_byte_range_and_table_match_2`: The routine checks a flag in the second argument and a byte at offset seven from the first argument.
- `compare_byte_at_offset_2`: The routine compares a byte from a global variable at a fixed offset with a byte from a pointer in A0 at a fixed offset.
- `store_param_into_global`: The routine copies the second argument into a global variable and returns a constant.
- `call_other_and_return_minus_one`: The routine calls another routine and then returns the value -1.
- `invoke_other_routine_and_return_minus_one`: The routine calls another routine and then returns a constant value of -1.
- `return_constant_negative_one`: The routine is effectively empty except for returning a constant negative one.
- `check_global_flag`: The routine loads a global variable and returns all bits set if the variable is non-zero, otherwise it returns zero.
- `compare_flag_to_global`: The routine tests a byte from a register against a global variable.
- `return_flag_from_global`: The routine reads a global variable and returns zero unless the variable equals a specific byte value, in which case it returns all bits set.
- `return_flag_based_on_global`: The routine reads a global variable and selects a byte value based on whether another global variable is non-zero.
- `return_flag_from_global_2`: The routine loads a global variable and returns zero unless the variable equals a specific value, in which case it returns all bits set.
- `set_flag_based_on_global`: The routine reads a global variable and selects a constant byte, then compares that byte with another global variable.
- `check_global_array_entry`: The routine receives two parameters and indexes a global variable array with the second parameter.
- `check_global_masked_flag`: The routine returns a constant zero unless a masked comparison of a global variable with a byte taken from a table indexed by the second parameter matches the global variable, in which case it returns all bits set.
- `compare_global_indexed`: The routine receives two parameters and uses the second as an index into a global variable to fetch a value.
- `check_table_entry_masked`: The routine selects a mask of 8 or 4 depending on a global flag, then tests whether the bitwise OR of a byte from a table indexed by the second argument with that mask equals the table base value.
- `compare_global_with_indexed_value`: The routine receives two parameters and uses the second as an index into a global variable to form a value.
- `check_flag_and_mask`: The routine selects a mask of 4 or 8 depending on a global flag, then tests whether the bitwise OR of a byte from a table indexed by the second argument with that mask equals a global variable.
- `check_and_return_error`: The routine calls another routine, checks its short result and, if non-zero, calls another routine and checks the returned byte.
- `retrieve_palette_entry`: The routine computes an index into a global table using a byte from the first argument and a short from the second argument, with sign handling controlled by a global flag, then returns a signed byte from that table or a default value.
- `retrieve_palette_entry_2`: The routine indexes a global table using a byte from the argument and two global indices, applying different offsets depending on the index ranges, and returns a short value from the table or 1 if the conditions are not met.
- `fetch_palette_index_2`: The routine reads a signed offset from a structure, optionally negates it using a global flag and adds a global bias, then indexes into a global table using a byte from the structure and a global multiplier to return a palette entry, with several range checks and fallback returns.
- `retrieve_sprite_pixel`: The routine checks a signed byte at offset seven of the input structure and, if it is non-negative, computes an index into a global byte table using several global variables and the second parameter.
- `retrieve_palette_entry_3`: The routine indexes a global byte table using a byte from the argument and two global offsets, with several conditional adjustments to the row and column components of the index, and returns the signed byte found at that location or a default value when the guard byte is not positive.
- `retrieve_palette_entry_4`: The routine reads a signed byte from the object at offset seven to select a row in a global table and uses a global index and a signed offset in a second parameter to compute an index into that table, returning a signed byte from the table or one if the selection byte is negative.
- `fetch_palette_indexed_byte_2`: The routine reads a signed byte from the input structure at offset seven to select a table bank, computes an index from the input parameters and two global variables, applies sign handling and range checks, and returns a byte from a global table addressed by the bank and the computed index, or returns one if the checks fail.
- `retrieve_table_byte`: The routine checks a signed byte from the argument and, if it is non-negative, uses two global variables and a global table to compute an index and returns a byte from the table, otherwise it returns one.
- `fetch_sprite_palette_index`: The routine checks a flag byte at offset seven of the first argument and, if set, conditionally negates a field in the second argument using a global flag.
- `check_input_byte_2`: The routine calls another routine and tests the returned byte.
- `return_zero_stub_3`: The routine is empty and returns zero.
- `check_other_routine_result_2`: Calls another routine and returns all ones if the returned byte is zero, otherwise returns zero.
- `empty_routine_5`: This routine contains no instructions and therefore does nothing.
- `check_char_return_flag`: A routine calls another routine and tests the returned byte.
- `return_zero_constant_4`: The routine is empty of logic and immediately returns a zero value.
- `check_input_byte_zero`: The routine calls another routine and tests the returned byte.
- `check_input_byte_3`: The routine calls another routine and checks the returned byte.
- `check_other_routine_return_zero`: The routine calls another routine and stores its returned byte.
- `return_zero_stub_4`: The routine is empty and immediately returns zero.
- `check_zero_terminator`: The routine calls another routine and checks whether the returned byte is zero.
- `check_null_terminator`: The routine calls another routine and tests the returned byte.
- `check_other_routine_result_3`: The routine calls another routine and stores its returned byte in a local variable.
- `fetch_palette_bytes_sequence`: Calls a routine to fetch palette-indexed bytes three times.
- `check_other_routine_result_4`: The routine calls another routine three times in sequence, testing the returned short each time.
- `retry_three_calls`: The routine calls another routine three times in sequence, returning a non-zero value only if the first call returns non-zero and the second and third calls both return zero; otherwise it returns zero.
- `retry_three_times`: The routine calls another routine up to three times in succession.
- `check_palette_byte_sequence`: Reads three bytes from a palette-indexed source.
- `check_secondary_condition`: The routine calls another routine twice.
- `empty_routine_6`: The routine contains no instructions and performs no operations.
- `check_other_routine_result_5`: The routine calls another routine twice and returns a constant on a specific result combination, otherwise returns zero.
- `check_two_conditions`: The routine calls another routine twice and checks the returned short.
- `check_flag_and_return`: The routine calls another routine, tests bit 1 of the returned 16-bit value, and returns all ones if the bit is set, otherwise returns zero.
- `check_message_flag`: The routine reads a short value from an offset in the parameter structure and, if it is less than one, it checks a byte obtained by dereferencing a pointer stored in the structure and compares it to a newline character.
- `routine_stores_16_bit_value`: The routine stores the low word of the second argument into the two bytes at an offset into the memory referenced by the first argument, and returns a constant.
- `add_short_to_struct_field`: The routine adds a 16-bit value taken from the second argument to the 16-bit field at offset 8 of the structure pointed to by the first argument, then returns a constant.
- `copy_word_from_struct_to_table`: The routine copies a 16-bit word from an offset inside a structure pointed to by the first argument into a table indexed by the high word of the second argument.
- `set_short_from_table`: The routine writes a short value into memory at an offset from a pointer, using an index derived from the high word of a second argument to look up a short in a global table and subtracting one.
- `add_offset_to_table_entry`: The routine reads a pointer from a structure, indexes a table with a scaled short parameter, loads a short from that table, uses it as an index into a second table, and adds a second table value to the first table entry, storing the result back.
- `decrement_table_entry`: The routine reads a pointer from a structure, indexes a table with a scaled short parameter, loads a pointer from a global table using the table entry, and subtracts the next word from the pointed-to short value.
- `check_collision_chain`: The routine walks a linked list of records indexed from a global table using a 16-bit index taken from an object field.
- `compare_byte_flags_and_return`: The routine reads a byte from an offset of a global variable, compares a byte from a pointer in the first argument with a byte from the third argument, and checks the low bit of two bytes.
- `return_constant_zero_2`: The routine is empty and contains no executable logic.
- `check_collision_chain_2`: Passes the first argument to a routine that checks a collision chain.
- `compare_flag_bytes`: The routine reads a byte from an object field and a flag from another object, compares a byte from a pointer table with a parameter, and returns a 64-bit value built from a constant and the flag when the two flags match, otherwise returns the flag value.
- `empty_routine_7`: The routine contains no instructions and returns immediately.
- `recursive_call`: The routine calls another routine with the same argument it received and then returns.
- `subtract_two_values`: The routine receives two integer parameters and returns their difference by subtracting the second from the first.
- `recursive_call_wrapper`: The routine receives a 32-bit value and immediately calls another routine with the same value, then returns.
- `check_and_update_flag`: The routine compares two structures, and if they differ in a flag byte and a field is set, it sets a global variable and calls another routine with a value from the first structure.
- `return_zero_constant_3`: The routine is empty and returns the constant zero.
- `reset_global_and_call`: The routine clears a global variable to zero, calls another routine with the incoming argument, and returns the cleared global variable.
- `check_flag_and_call_handler`: The routine compares two structures by a flag byte and a size field, sets a global variable, and calls another routine with a byte from the first structure.
- `empty_routine_8`: The routine contains no instructions and returns immediately.
- `reset_global_and_call_2`: The routine clears a global variable to zero, calls another routine with the supplied argument, and returns the global variable.
- `check_param_and_return`: The routine reads a byte from the pointed structure and compares a character extracted from the third argument with two values obtained from a global table and the structure.
- `empty_routine_9`: The routine contains no instructions and performs no operation.
- `call_other_routine`: The routine receives a 32-bit value and immediately passes it unchanged to another routine, then returns.
- `compare_byte_from_table`: The routine checks whether two short pointers are different and whether the low byte of a parameter matches a byte taken from a table indexed by the first pointer.
- `self_recursive_call`: The routine receives a 32-bit value and immediately calls another routine with the same value, then returns.
- `compare_input_to_expected`: The routine checks a byte from a table pointed to by a field in the first argument against a value derived from the third argument.
- `self_recursive_call_2`: The routine receives a 32-bit argument and immediately calls another routine with the same argument, then returns.
- `toggle_return_value`: The routine calls another routine with its argument and returns the result with the least significant bit inverted.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `compare_and_copy_flag`: The routine calls another routine with a global variable and with an argument.
- `empty_routine_10`: The routine contains no instructions and returns immediately with a constant zero value.
- `compare_and_trigger_action`: The routine calls another routine with a global variable and checks its result.
- `empty_stub_routine`: The routine contains no instructions and performs no operations.
- `check_other_routine_result_6`: The routine calls another routine with a pointer to a global variable and returns true if the call returns non-zero, otherwise false.
- `compare_byte_and_call_2`: The routine compares a byte argument with a byte stored at an offset from a pointer argument.
- `empty_stub_routine_2`: The routine contains no instructions and returns immediately.
- `call_other_routine_with_global`: The routine calls another routine, passing the address of a global variable as its argument, and then returns.
- `decode_and_iterate_table`: The routine decodes a byte from a structure using a global variable table and then iterates a bounded loop, calling two other routines with derived indices and returning early on non-zero results.
- `adjust_sprite_row_offset`: The routine computes a row offset from a pointer and a stack flag, then iterates over a range derived from a register, adjusting the offset up or down by a fixed step while skipping entries in a global table, and for each entry it calls another routine and walks a linked list of structures calling a function pointer from a register, returning early on certain conditions.
- `check_table_entries`: The routine iterates over a table of indices using another routine, calling a function pointer from a structure to process each entry and returning on non-zero result.
- `return_zero_4`: The routine contains no instructions and immediately returns zero.
- `check_entity_collision`: The routine examines a byte from the first argument and uses it with offsets into a global table to obtain two pointers.
- `check_object_in_range`: The routine checks whether a particular object is present in a limited range of a two-dimensional table of object pointers.
- `check_collision_list`: The routine checks a collision list for a matching object by scanning a table of pointers and comparing a byte field from each linked entry against a byte from another structure.
- `check_object_property_list`: The routine walks a two-dimensional table stored in a global variable, using two nested counters to index rows and columns.
- `return_constant_one_2`: The routine unconditionally returns the constant value one.
- `search_dialogue_lines`: The routine checks a byte field in a structure against a global variable and walks two tables indexed by that byte and by fields from the structure.
- `iterate_sprite_tiles`: The routine processes a sprite descriptor pointed to by param_1 and iterates over a range of tiles defined by a signed count field in the descriptor.
- `accumulate_table_entries_2`: The routine processes a table referenced by an argument and iterates over its entries, using another routine to obtain an index and then walking a linked list of entries to accumulate values returned by a callback.
- `accumulate_callback_results`: The routine repeatedly calls a callback through a pointer stored in a structure and accumulates results from another routine via a table lookup, adjusting an offset with a global variable and breaking on certain conditions.
- `read_word_from_a5_offset`: The routine reads a 16-bit word from an address computed as the contents of A5 minus two and returns it.
- `check_collision_response`: The routine checks whether two objects differ in a flag, have a specific attribute set in the first object, and the third argument matches a byte from a structure pointed to by the first object.
- `return_immediate_zero`: The routine is empty and returns the immediate value zero.
- `call_other_routine_and_return_zero`: The routine calls another routine with its argument and then returns zero.
- `self_recursive_call_3`: The routine calls another routine, passing its own address twice along with the parameters it received.
- `check_entity_match`: The routine compares two entity structures for a flag in a status byte, a matching byte through a pointer, and a common bit in a word field.
- `empty_routine_11`: The routine contains no instructions and performs no operation.
- `forward_self_reference_call`: The routine receives a 32-bit value and a 16-bit value and immediately calls another routine, passing the two parameters together with two copies of its own address as the third and fourth arguments.
- `process_tile_column`: The routine processes a column of tiles defined by a byte field and a length field in a structure pointed to by param_1.
- `check_objectives`: The routine checks a table of pointers referenced via a global variable and, depending on a flag stored on the stack, walks a signed loop moving an index up or down to call another routine and then a callback function for each entry, accumulating a short result.
- `process_input_stream`: The routine repeatedly calls a callback stored in a structure and accumulates results from another routine using a table of function pointers.
- `empty_stub_routine_3`: The routine contains four instructions and performs no observable work.
- `check_collision_flag`: The routine compares two objects by checking a flag in one object, matching a type byte from a pointer stored in the first object, and comparing a flag bit in both objects.
- `return_immediate_zero_2`: The routine returns immediately with the constant value zero.
- `call_other_routine_2`: The routine calls another routine, passing the first two parameters unchanged and passing the same routine address twice as the third and fourth arguments.
- `check_collision_flag_2`: The routine checks a flag in a byte field of the first object and a type byte from a pointer stored in the first object, compares a bit in a word field of both objects, and calls another routine with the first object.
- `return_zero_stub_5`: The routine is empty and immediately returns zero.
- `self_recursive_handler`: The routine receives two parameters and immediately calls another routine, passing the first two parameters together with two copies of its own address as the third and fourth arguments.
- `other_routine_3`: The routine returns a constant value with no side effects.
- `process_tile_column_2`: Passes two arguments along with pointers to a collision flag check routine and another helper routine to a tile column processing function.
- `call_other_routine_with_self`: The routine calls another routine, passing the first argument, the second argument, and two copies of its own address as the third and fourth arguments.
- `check_tile_collision`: The routine walks a small set of tiles around an object using a direction flag and a step count taken from the object structure.
- `check_collision_flag_3`: The routine checks a flag in the first object’s status byte and a flag in a sub-structure, then compares a flag in the second object’s status byte with the first.
- `check_and_set_flag`: The routine calls another routine and tests its short result.
- `return_zero_5`: The routine contains no instructions and therefore performs no operation.
- `check_and_clear_flag`: The routine calls another routine with one argument and tests the returned value.
- `self_calling_stub`: The routine receives two parameters and immediately calls another routine, passing the first two parameters together with two copies of its own address as the third and fourth arguments.
- `check_collision_events`: The routine processes a single byte from a structure to index a global table and, if the index is in range, iterates a signed count from a field in another structure, calling a callback for each step and invoking another routine to test collisions.
- `adjust_tile_column_cursor`: Iterates over a range of tile columns and checks a tile map for a non-empty tile, then calls another routine for each column and processes a linked list of callbacks via a table, returning early if a callback signals a result.
- `poll_event_table`: The routine repeatedly calls a callback through a pointer stored in a structure and updates a counter and an offset while walking a byte stream from a global variable using a 16-bit index.
- `empty_stub_routine_4`: The routine contains four instructions and performs no observable work; it returns a constant zero without reading or writing memory.
- `check_and_trigger_event`: The routine checks two objects for a flag in a byte field and a condition in a referenced byte, then compares a flag bit between them.
- `check_flag_and_mark`: The routine calls another routine and tests its short result.
- `return_constant_zero_3`: The routine is empty and returns a constant zero.
- `check_collision_events_2`: Passes two arguments along with pointers to an event trigger routine and a flag clearing routine to a collision event checking function.
- `clear_and_copy_palette_entries`: The routine clears a block of memory and then copies bytes from a table indexed by two bytes read from a global variable into two separate tables, using a loop that processes entries from a global list.
- `set_global_from_other_routine`: The routine calls another routine, masks the returned 16-bit value with 7, subtracts 3, and stores the result in a global variable.
- `normalize_object_flags`: The routine clears the low nibble of a word at an offset from a pointer and, unless the word at another offset matches one of three specific values, it replaces that word with a value derived from an input register and a constant divisor, then returns a constant.
- `store_param_and_return`: The routine copies the second argument into a global variable and returns a constant value.
- `check_param2_flag`: The routine receives two parameters and checks whether the second parameter equals a specific global variable.
- `initialize_audio_channels`: The routine clears and sets up a set of global variables and tables used for audio, then initializes the four Amiga audio channel volume registers AUD0VOL through AUD3VOL to zero, and prepares channel pointers and state flags.
- `process_object_list_twice`: The routine iterates backwards over a list of objects stored in a global table, using a global variable as a count and index, and calls another routine for each entry, with a few conditional branches that read flags from the objects.
- `amiga_display_dma_setup`: The routine prepares a DMA control pattern in memory and manipulates a few global flags.
- `clear_status_flags`: The routine clears a global variable four times and then tests four individual bits of that variable.
- `empty_routine_12`: The routine contains no reachable instructions and performs no observable work.
- `disable_dma_and_loop`: The routine writes to the DMACON register with bit 15 set to enable DMA channels, then repeatedly calls another routine in an infinite loop.
- `render_separator_string_loop`: Copies a string into a buffer, appends pipe characters, and draws the text to the screen using the blitter.
- `copy_string_with_delimiters`: The routine copies a null-terminated string from a global variable to another location, inserts pipe delimiters, writes a repeated pipe pattern through a pointer, and then copies the string again with a space terminator.
- `set_byte_and_loop`: The routine writes a constant byte value to a global variable, then tests the variable with a bitwise and and an equality check, and if the condition is true it calls another routine and returns.
- `empty_routine_13`: The routine contains no reachable instructions and is effectively empty.
- `wait_frame_and_show_message`: Waits for the next vertical-blank tick, draws the current message string at the top of the screen and swaps the display buffers.
- `initialize_audio_channels_2`: The routine clears and initializes a large block of global state and several pointers, then configures the four Amiga audio channel volume registers.
- `copy_word_buffer_and_zero`: The routine copies a word buffer pointed to by a global variable into another buffer pointed to by a global variable, using a count from a global variable, then copies a fixed number of bytes from a global variable to itself and calls another routine before looping forever.
- `copy_sprite_table_and_loop`: The routine copies a block of words from a source table pointed to by a global variable into a destination table pointed to by another global variable, using a count stored in a global variable, then copies a fixed number of bytes from one global area to another and calls another routine before entering an infinite loop.
- `copy_blocks_and_zero_loop`: The routine copies a block of 16-bit words from a source pointer to a destination pointer, using a count taken from an argument, then copies 22 bytes from a global variable to itself and calls another routine before entering an infinite loop.
- `empty_routine_14`: This routine is empty.
- `mask_and_combine_global_variable`: The routine checks a global variable for two bit conditions.
- `update_sprite_position`: The routine updates a short pair in a parameter block using offsets from a table entry and a flag byte, then stores the computed coordinates and a pointer to the table entry into a linked structure pointed to by a global variable, and increments a counter byte.
- `choose_handler_by_index`: The routine selects an entry from a table using an index, checks a signed flag at an offset, and if the flag is negative it loads a pointer from the table into a global variable; otherwise it calls another routine and updates a global pointer with an offset derived from an input word.
- `infinite_loop_stub`: The routine contains no reachable instructions and consists of an infinite loop with no operations.
- `set_display_dma_control`: The routine writes to the Amiga custom chip registers COP1LCH, COPJMP1, DMACON and VHPOSR to configure copper lists and DMA. It enables DMA channels by writing to DMACON with bit 15 set.
- `setup_copper_interrupt_handler`: The routine installs a Copper interrupt hook by writing a jump address into the COPJMP1 register and toggles COP1LCH. It reads VHPOSR to check raster position, writes to DMACON to enable DMA channels, and then calls another routine.
- `clear_global_variable_2`: The routine clears a global variable and then runs a loop that rotates a value left by one bit, calling another routine whenever the value is negative, until a counter reaches zero.
- `add_bcd_digits`: The routine adds four packed BCD digits from one byte array to another byte array using an accumulator pointer and an input pointer, adjusting each digit with a BCD correction after each addition.
- `copy_entity_state`: The routine copies a fixed set of words from a source table to two destination slots in a RAM table, once for each of two indices passed in registers.
- `write_dma_control_and_loop`: The routine writes a byte to an offset in a table pointed to by a global variable, writes another byte to a different offset in the same table, stores a constant to a global variable, sets a flag in another global variable, calls another routine four times, and then enters an infinite loop.
- `restore_game_state`: The routine saves a global variable, temporarily changes it while calling another routine, then restores it.
- `clear_global_flags`: The routine clears a global variable to zero and then tests a series of individual bits in that variable.
- `null_terminated_string_length_scan`: The routine reads a byte from an offset of the input pointer, then walks forward through memory byte by byte until a zero byte is found, effectively scanning the length of a null-terminated string.
- `null_terminated_string_length`: The routine walks a pointer through a null-terminated byte string until the terminator is found, then sets a global variable to a non-zero value, calls another routine with the address just past the first byte and a word argument shifted left by three, and finally clears the global variable.
- `decode_nibble_loop`: The routine repeatedly extracts a nibble from a 32-bit value in D3 by rotating it left four bits, then indexes a global byte table with that nibble and writes a byte to a global variable.
- `reset_global_flags`: The routine checks a global variable for two specific bits.
- `reset_game_state_4`: The routine checks a global variable for two specific bits.
- `set_state_flags_and_call`: The routine checks a global variable for two bit flags.
- `empty_unreachable_stub`: The routine is empty.
- `reset_global_state_and_swap_buffer`: The routine clears a global variable, calls another routine, sets a flag, then performs ten iterations of swapping two global word values via a temporary and calling another routine between and after the swap, finally clearing the flag.
- `check_memory_signature`: The routine checks a global flag and then scans a table of six-byte entries for a specific XOR signature.
- `verify_table_entry`: The routine walks a table of entries six bytes apart, decrementing a counter and comparing three decoded bytes of each entry against three global bytes.
- `empty_routine_15`: The routine contains no instructions and returns immediately.
- `process_input_buffer_2`: The routine reads a byte from a global input buffer until a sentinel is found and compares it with a value from another routine, then updates the buffer based on the character: it clears the buffer on escape, backspaces on tab, appends printable characters up to a limit, and otherwise shifts the buffer contents and pads the display with a space or underscore before invoking another routine to render the text.
- `process_input_buffer_3`: The routine updates a global character buffer using a byte from a register and a high-byte index, handling tab and newline cases, then shifts the buffer contents forward and terminates it with a space or underscore depending on the buffer length before calling another routine with the buffer and a constant.
- `format_and_draw_text`: The routine updates a global counter, copies a string from a global variable into a buffer, and appends a space or underscore character based on the counter value.
- `set_global_state_flags`: The routine calls another routine three times and then sets a global variable to a constant.
- `decrement_increment_counter`: The routine checks a global variable's bits and a global counter.
- `decrement_and_increment_flag`: The routine checks a global variable for two flag bits.
- `decrement_and_increment_global`: The routine conditionally decrements a global variable when a specific bit is set and the variable is non-zero, and conditionally increments the same global variable when another bit is set and the variable is below a threshold.
- `set_audio_dma_channel`: The routine maps an input value through a lookup table to an index, then writes a value to a global table and programs the Amiga audio DMA channel registers AUD0LCH and DMACON to start DMA for that channel.
- `set_audio_dma_channel_2`: The routine configures an Amiga audio DMA channel by writing to AUD0LCH and DMACON. It loads four words from a table pointed to by a register, stores the first two words into hardware registers and a global variable, enables the channel via DMACON, waits, then updates the channel pointer and period registers with the remaining table words.
- `store_four_words_via_pointer`: The routine stores four values from registers into consecutive memory locations pointed to by a pointer in a1.
- `protection_code_wheel_screen`: Copy protection: shows PROTECTION, a random code-wheel shape (0-29, the player may step it) and one of five codes from a table XORed with an address, reads up to six typed characters and compares them with the expected answers from the same table; the Interpol crack replaced the branch on a mismatch with two no-operations, so any six characters and Return pass.
- `protection_failed_retry`: Copy protection failure path: counts failed attempts and re-shows the screen; after the third it clears the ExecBase pointer at address 4, which crashes the machine.
- `reset_palette_and_pointers`: The routine clears several global variables to fixed values and then writes a series of values into a global variable, using results returned from other routines as bases for pointer arithmetic.
- `protection_draw_shape`: Copy protection: clears both screen buffers and draws the selected code-wheel shape in the protection screen's colours.
- `decode_message_table`: The routine walks a table of messages indexed by an input value and recursively decodes entries, using flags in each entry to select a character and to set two global variables, then stores the resulting character into a global variable and updates a pointer.
- `clear_and_iterate_table_entries`: The routine clears several global variables and then walks a table pointed to by A6.
- `decode_digit_table`: The routine decodes a value in D0 using a table of byte-sized codes pointed to by a global pointer, repeatedly subtracting table entries until the remainder is smaller than the current entry, then writes the corresponding ASCII digit character to a buffer pointed to by A0.
- `write_digit_count`: The routine writes a sequence of ASCII digit characters to a buffer pointed to by a pointer, using a table of values as divisors.
- `fill_counting_table`: The routine writes a sequence of bytes into a buffer pointed to by a pointer, using a table of counts read from memory.
- `store_byte_from_register_to_table`: The routine calls another routine twice and then stores a byte from D0 into a global variable table via a pointer in A0, using the low nibble of the byte from D0 as an index.
- `store_byte_from_register`: The routine calls another routine and then stores a byte from a register into a global variable selected by the low nibble of another register.
- `store_nibble_indexed_byte`: The routine stores a byte from a register into a global variable table indexed by the low nibble of another register value.
- `reset_game_state_5`: The routine clears several global variables, writes constant values and a pointer to a table into them, then calls another routine with a constructed argument and a pointer offset.
- `reset_global_state_2`: The routine clears several global variables, writes constant values and pointers into them, calls another routine, stores the returned registers offset by a constant into two globals, writes another constant, calls the other routine again and returns a value derived from an input register.
- `set_blitter_parameters`: The routine writes a series of constants and pointers to a set of global variables, calls another routine, then adds a fixed offset to two values returned in registers and stores the results, writes another constant, and calls the same routine again.
- `configure_blitter_copy_2`: The routine programs the Amiga blitter registers to set up a block copy operation and then invokes another routine to perform the copy.
- `initialize_blitter_parameters`: The routine writes a fixed set of values to a global variable and then calls another routine three times, each time storing the returned pointers offset by constants into the same global variable.
- `decode_input_byte_2`: The routine reads a byte from a pointer argument, masks it, and stores a derived value into a global byte table.
- `load_room_graphics`: Room change: waits for vertical blanks, blacks the 32-colour palette, unpacks the room and sprite graphics (ByteKiller), copies the tile and animation data into place, decodes the sprites and fades the new palette in.
- `copy_animation_frames`: The routine copies a fixed set of bytes from a source block selected by an index table entry into a destination block, with conditional calls to another routine that may update the source and destination pointers and a loop counter.
- `copy_bytes_to_global`: The routine copies a block of bytes from a source pointer passed in A2 into a sequence of global variables, storing the bytes in reverse order within the destination area.
- `load_table_into_globals`: The routine copies a sequence of 32 word indices taken from a byte table pointed to by A2 into a consecutive block of global variables, starting with a global variable and continuing through the next 31 globals.
- `render_copper_list`: Calculates display list offsets and iterates through a memory buffer to fetch color values from a lookup table.
- `decode_palette_from_ram`: The routine reads a 32-bit block from RAM and expands each 32-bit word into sixteen 16-bit palette entries by indexing a global 256-entry color table.
- `fill_bit_reversal_table`: The routine writes 256 bytes to a global variable, treating it as a table.
- `expand_palette_table`: The routine fills a 256-entry table in memory with 16-bit values derived from an 8-bit index by mapping each bit to a 2-bit nibble in the high and low bytes.
- `generate_palette_from_index_table`: The routine builds a 256-entry table in a global variable by iterating over all 256 byte indices and expanding each index into a 32-bit value whose four bytes are derived from the four high and low nibbles of the index.
- `decode_sprite_data`: Iterates over a short array using a global counter and builds a composite value from consecutive elements, then for each non-terminator entry looks up a signed offset in a table, copies or loads bytes into a buffer and calls another routine with the gathered parameters.
- `copy_word_block`: The routine copies a block of 16-bit words from a source pointer to a destination pointer.
- `set_global_pointers`: The routine calls another routine and, if the result is non-zero, stores that result into several global variables and then computes a series of other global variable values as offsets from the result of a second routine call, storing those offsets.
- `clear_global_flag_on_zero`: The routine calls another routine and tests its return value.
- `clear_global_variable_on_zero`: The routine calls another routine and tests the returned value.
- `reset_global_flag_and_call_hook`: The routine checks a global variable and, if it is non-zero, calls another routine and then clears the global variable to zero.
- `clear_pending_flags`: The routine calls another routine unconditionally and then checks ten global variables in order.
- `clear_flags_and_return_value`: The routine checks two global variables and, if both are non-zero, calls another routine and clears the first flag.
- `initialize_game_state_4`: The routine clears a global variable and a set of globals, zeroes a 1536-byte table, then repeatedly calls another routine while copying values from an output structure into several global variables and updating a working pointer.
- `initialize_game_state_5`: The routine clears a large block of memory, resets several global variables and then repeatedly calls other routines while copying values from a multi-word structure into a set of global variables.
- `clear_and_build_tables`: The routine clears a block of global storage and then, depending on a global variable, builds two parallel tables from a short script and a string buffer, advancing pointers through the script while filling the tables with offsets and lengths.
- `copy_tile_data`: The routine conditionally invokes another routine based on a global flag and a table index.
- `copy_string_from_table`: The routine reads a length and a source pointer from a table entry pointed to by a register, computes a destination address from a global variable and a register, and copies a variable number of bytes from the table into memory at that destination.
- `self_recursive_handler_2`: The routine calls another routine repeatedly, including a conditional second call when a global variable is non-zero.
- `set_global_handler_and_offset`: The routine clears a global variable, calls another routine, then calls another routine and stores its return value.
- `restore_interrupt_handlers`: The routine checks a flag and, if set, temporarily installs a set of interrupt vectors and a RAM pointer into a global table, calls another routine, then restores the table entries to default values and clears two control flags before returning the called routine's result.
- `handle_input_byte`: The routine reads a byte from a structure pointed to by a register, combines it with the high word of an input value, and searches a global byte table for a matching entry.
- `copy_bytes_with_length_check`: The routine checks a short value in a global variable against an input value, updates the global if different, and calls another routine.
- `self_modifying_dispatch`: The routine checks a global byte flag and, if set, reads a byte from a table indexed by the flag value.
- `increment_two_globals_and_call_handler`: The routine adds the value in D0 to two global variables and then calls a handler routine through a pointer stored in a table.
- `decrement_counter_and_call_handler`: The routine subtracts the value in D0 from a global variable and then calls a handler routine obtained from a table in RAM. It then returns.
- `write_debug_strings_to_log`: The routine writes a sequence of ASCII strings into memory via an indirect call, using a global variable and several RAM locations as a buffer, and then repeatedly calls another routine until a zero terminator is cleared.
- `remove_unreachable_blocks`: A generated decompiler stub for a non-existent routine.
- `reset_handler_table`: The routine clears a circular byte buffer pointed to by a global variable, sets up a few globals, calls other routines via pointers derived from globals, and repopulates a small table with routine pointers and constants before entering an infinite loop.
- `initialize_loading_state`: The routine clears a small byte table, sets a global flag, calls another routine twice, invokes two handler pointers from a table, and configures a disk-loading context by writing path bytes and installing three handler pointers.
- `store_extra_output`: The routine calls another routine twice, reads a value from a pointer derived from a register and stores it into a global variable, copies an uninitialized register value into a global variable, then computes a masked sum of those two values plus one and stores the result into a global variable, and finally copies one global variable to another.
- `save_cpu_state`: The routine stores registers D0-D7 and A0-A6 into a block of RAM, saves a byte from the stack into a global variable, and sets a pointer in RAM to the address in register A0.
- `set_global_and_dispatch`: The routine stores a constant byte into a global variable, calls another routine, copies a byte from a table into a RAM location, and then calls a routine through an indirect pointer loaded from RAM.
- `dispatch_handler_pointer`: This routine acts as an indirect jump target, likely part of a function pointer table or interrupt vector.
- `set_flag_and_indirect_call`: The routine stores a constant byte value into a global variable, calls another routine, copies a pointer from a memory location into a RAM location, and then performs an indirect call through the copied pointer.
- `call_three_subroutines`: The routine calls another routine three times in succession and then returns.
- `store_message_and_update_state`: The routine reads a flag from a structure and a global variable, selects a byte from a table using an index derived from the structure, and walks a linked list of entries until a match is found.
- `update_object_collision_state`: The routine reads fields from a structure in memory, compares identifiers and flags against a small lookup table, updates several global variables, and conditionally calls another routine.
- `reset_global_state_3`: The routine clears a global variable to zero after setting it to a non-zero value, then calls another routine and clears the same global variable again multiple times before returning the value from the stack.
- `process_input_byte_and_update_tables`: The routine reads a byte from an input structure using an offset and selects a pointer to a global table entry based on the byte value.
- `do_nothing_stub`: The routine is empty and returns immediately without performing any operations.
- `recursive_other_routine_call`: The routine calls another routine twice in succession and then returns.
- `flashback_interrupt_setup`: The routine clears two global variables, calls another routine, then writes several values into a global variable, including a pointer to another routine and a pointer to RAM. It touches CIA and interrupt registers, indicating it configures hardware interrupts.
- `recursive_self_call`: The routine consists of two consecutive calls to itself and then returns.
- `recursive_self_call_2`: The routine calls another routine twice in succession and then returns.
- `clear_screen_buffer`: Clears one 320x224 four-plane screen buffer (a0) with the blitter.
- `copper_level1_interrupt`: Level 1 interrupt handler: counts down a cutscene delay and rewrites palette and sprite words in the copper list from the current scroll state.
- `empty_routine_16`: The routine is empty.
- `set_up_sprite_table`: The routine writes a set of 32-bit values split into 16-bit halves into a structure pointed to by a global variable, using a base offset and an index derived from a global flag.
- `keyboard_interrupt_level2`: Level 2 interrupt: reads the CIA-A serial byte when the SP flag is set, marks it in the key table, keeps the last key code for the menus and the Ctrl state, handshakes the keyboard and clears PORTS.
- `initialize_game_state_6`: The routine writes a series of constants and copies from a global variable into several global variables and RAM locations, then calls another routine twice with different global values, copies a block of 16 words from one global area to another, and then enters an infinite loop.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `take_over_hardware`: Saves the system interrupt vectors and INTENA, sets up the double screen buffers, installs the game's level 1 and level 2 handlers and enables its interrupts.
- `copy_word_table_until_marker`: The routine copies 16-bit words from a source pointer to a destination pointer until a sentinel word is encountered, then clears a global variable, stores the destination pointer into a global variable and another global variable, calls another routine, and sets a global variable to a constant value.
- `infinite_recursive_loop`: The routine calls another routine and then enters an infinite loop, never returning.
- `self_recursive_loop`: The routine calls another routine and then enters an infinite loop that never exits.
- `self_recursive_loop_2`: The routine calls another routine and then enters an infinite loop.
- `self_recursive_loop_3`: The routine calls another routine and then enters an infinite loop.
- `self_recursive_infinite_loop`: The routine calls itself unconditionally and then enters an infinite loop, never returning.
- `self_calling_loop`: The routine immediately calls another routine and then enters an infinite loop.
- `swap_buffers_and_wait_frames`: Ends a frame: swaps the front and back screen buffers and waits until the vertical-blank counter reaches the frame length (the game's frame pacing), then updates the copper list.
- `call_indirect_via_table_2`: The routine loads a function pointer from a global variable table and calls the pointed-to routine, returning its result.
- `call_indirect_via_table_3`: The routine loads a function pointer from a global variable table and calls the pointed-to routine, returning its result.
- `invoke_callback_via_table`: The routine fetches a function pointer from a global variable table and calls the pointed-to routine, returning its result.
- `call_indirect_through_table`: The routine calls another routine indirectly via a global variable table and then returns.
- `clear_byte_table`: The routine clears a 256-byte block starting at a global variable by writing zero to each byte.
- `update_input_state_flags`: The routine reads a byte from another routine, updates a global state variable through a small state machine, and sets bits in the byte based on mismatches between pairs of RAM bytes.
- `read_input_device_state`: The routine reads the current input device mode from a global variable and combines button and mouse information from CIAA_PRA, JOY0DAT and POTGOR into a single status word.
- `wait_for_flag_and_return`: The routine spins until a specific bit in a global variable is set, then returns the value of that bit.
- `wait_for_flag_then_return`: The routine spins until a specific bit in a global variable is set, then returns the value of that bit from the global variable.
- `wait_for_interrupt_flag`: The routine spins until a specific bit in a global variable is set, then returns the value of that bit.
- `configure_bitplane_dma`: The routine writes a series of values to a set of global variables and then calls another routine multiple times, passing adjusted pointers derived from the outputs of those calls.
- `initialize_interrupt_handlers`: The routine writes constants and pointers into a global variable area, setting up a table of handler addresses and a RAM reference.
- `swap_palette_entries`: The routine copies sixteen 16-bit palette entries from a source table, reorders the bits of each entry, writes the transformed values to a destination buffer in memory and mirrors them to a hardware color register area.
- `swap_palette_entries_2`: The routine copies sixteen 16-bit palette entries from a source table into a destination table and writes them to the Amiga custom chip color register.
- `rotate_color_table`: The routine copies sixteen 16-bit color entries from a global table to a hardware color register and to another memory location, rotating the byte order of each entry.
- `swap_nibble_bytes`: The routine copies sixteen 16-bit words from a source pointer to a destination pointer, reordering the bytes within each word.
- `swap_nibble_bytes_2`: The routine copies sixteen 16-bit words from a source pointer to a global table, reordering each word’s bytes and nibbles by masking and shifting the bits.
- `copy_palette_to_hardware`: The routine copies sixteen 16-bit palette entries from a table pointed to by a register to the Amiga custom chip color registers, doubling each entry.
- `read_joystick_buttons`: The routine reads the joystick/mouse fire buttons from CIAA_PRA, the mouse movement from JOY0DAT and the right/middle buttons from POTGOR, combines the values with a global variable and stores the results into a few global variables.
- `check_byte_range_condition`: The routine compares two byte values and a signed byte value, returning early if they are equal or if certain arithmetic relations between them hold.
- `return_register_value`: The routine loads a byte from a register and returns it without any further processing.
- `return_global_variable`: The routine loads a value from a global variable and returns it in D0.
- `empty_routine_17`: The routine contains no instructions.
- `call_other_routine_with_global_2`: The routine calls another routine, passing the address of a global variable twice, and then returns.
- `draw_text`: Draws a zero-terminated string with the 8x8 game font at x, y in a colour into a screen buffer; '/' starts a new line.
- `compute_max_line_width`: The routine scans a zero-terminated byte string, using a table indexed by the low seven bits of each byte to add a per-character width, accumulating a running width for the current line.
- `compute_text_width`: The routine walks a byte string containing control codes and printable characters, using a global table to map each character to a width and calling another routine for certain characters.
- `compute_text_width_and_render`: The routine measures the visual width of a null-terminated text string using a per-character width table, tracking the maximum line width across line breaks, then iterates over the string to emit rendering information while updating a horizontal position accumulator and invoking another routine for each printable character.
- `compute_text_width_and_draw`: The routine measures the pixel width of a text string using a per-character width table stored in a global variable, handling control codes and line breaks, and then calls another routine to render the text at a position derived from the measured width.
- `measure_text_width`: The routine walks a byte string and accumulates a width value using a per-character table stored in a global variable, handling control codes and line breaks, and returns a combined 32-bit value with the measured width in the low word.
- `render_text_width`: The routine measures the display width of a null-terminated text string using a per-character width table stored in a global variable, accumulating widths and control codes while handling line breaks and a special delimiter.
- `compute_text_width_and_advance`: The routine walks a byte string until a terminator and accumulates a width value using a per-character table stored in a global variable, then returns a packed result containing the computed width and a register value.
- `call_handler_routine`: This routine serves as a handler pointer target, being called by one unnamed routine.
- `reset_interrupt_state`: The routine clears a global variable and then writes a series of values into a global variable and a small table of four words, followed by copying a value from another global variable into two locations and finally zeroing the global variable.
- `set_display_dma_control_2`: The routine writes to Amiga custom chip registers that control display DMA and copper list pointers, and it sets a global variable to a constant value before copying a value from another global variable into a third global variable and then clearing the first global variable three times.
- `load_and_parse_file_table`: This routine loads a file and iterates through a table of structures, calculating offsets based on size fields stored in the data.
- `populate_pointer_table`: The routine walks a byte table pointed to by a global variable and fills an array of pointers with successive entries, advancing the source pointer by a size derived from the entry bytes.
- `initialize_display_buffer`: The routine sets up a display buffer by writing to global variables and calculating offsets based on input values and table entries.
- `initialize_display_buffer_2`: The routine initializes a set of global variables used for a display buffer and computes a starting offset based on input values and a table pointed to by a register.
- `decode_sprite_tile_indices`: The routine copies bytes from a source buffer referenced by a pointer into a destination buffer at a fixed offset, using index values taken from a table and from registers.
- `setup_blitter_sprite`: Sets blitter registers for a sprite transfer and copies palette bytes into a buffer derived from a global base pointer.
- `configure_blitter_copy_3`: The routine writes a set of values to Amiga blitter registers and then calls another routine twice, with an early exit if a global flag is non-zero.
- `set_blitter_copy_parameters_2`: The routine programs the Amiga blitter registers to set up a block copy operation, writing values to the blitter source and destination pointers, modulo values, control words and size registers.
- `clear_blitter_state`: The routine writes a series of constant and register values to a set of global variables and then calls another routine.
- `clear_sprite_table_entries`: The routine iterates over a table of sprite descriptors and clears their entries.
- `set_blitter_copy_parameters_3`: The routine computes source and destination offsets and dimensions for a blitter operation, loads a mask and modulo values into blitter registers, and invokes another routine to perform the copy.
- `initialize_blitter_parameters_2`: The routine writes a series of values to a set of global variables and then calls another routine twice.
- `wait_for_cpu_clear_flag`: A routine that spins until a global variable’s bit is cleared and then writes a constant to the same global variable.
- `load_file`: Loads a whole data file: gives the machine back to the system, waits for the disk if a disk change is pending, reads the file through dos.library and takes the hardware over again.
- `load_and_run_config_file_2`: The routine opens a file whose name is copied from a pointer argument into a fixed buffer, then seeks and reads it using DOS calls.
- `open_input_device_file`: The routine prepares a filename buffer from a pointer, then calls a helper to open a DOS file and returns the result.
- `invoke_callback_and_return_value`: The routine calls another routine through a pointer stored in a global variable and returns a 64-bit value formed by concatenating the contents of two registers.
- `invoke_callback_and_return_concat`: The routine calls another routine via an indirect pointer derived from a global variable and then returns the concatenation of the two input values.
- `disable_interrupts_and_close_file`: The routine disables interrupts by writing to INTENA, triggers a CIAB_PRB access and INTREQ, then closes an open file via DOS Close, and finally returns the concatenated values from D0 and D1.
- `reset_interrupt_state_2`: The routine clears a set of global variables, calls another routine through a table, and conditionally invokes further routines via function pointers.
- `decrement_until_zero_2`: The routine receives a value in D0 and repeatedly subtracts one from it until it reaches zero, then returns.
- `call_indirect_through_table_2`: The routine fetches a function pointer from a global variable table and calls the pointed-to routine, returning its result.
- `initialize_collision_tables`: The routine clears a block of 64 entries in a table pointed to by a global variable, writing a fixed pattern of constants to each entry at several offsets, then writes the same pattern to a set of fixed locations in the same global structure.
- `initialize_collision_tables_2`: The routine clears and seeds a set of collision-related tables referenced through a global variable.
- `draw_framed_panel`: Draws a framed rectangular panel straight into the five 40-byte-wide bitplanes (8,960 bytes each): rows of 0/an address/0/an address..an address/0 per plane give the border and fill, used for message and inventory boxes.
- `initialize_memory_table_2`: The routine writes a series of constant 32-bit values into a large block of memory accessed through an uninitialized pointer, initializing several spaced-out fields with sentinel and zero values.
- `copy_sprite_blocks`: The routine copies a fixed number of 40-byte blocks from a source area computed from two input values and a base pointer in RAM to a destination starting at a global variable.
- `copy_tilemap_blocks`: The routine copies a fixed 19-byte block from a global variable to a destination computed from two inputs and a pointer, repeating the copy 36 times per row and 4 times total, advancing the destination by a large stride each outer iteration.
- `decode_and_dispatch_table`: The routine reads a length from a pointer in a4 and then walks a table of signed entries.
- `copy_struct_96_bytes`: The routine copies 96 bytes from a source buffer pointed to by a register to a destination buffer pointed to by another register, byte by byte.
- `bytekiller_unpack`: Depacks a data file packed with the ByteKiller-style coder: reads 32-bit words backwards from the end (unpacked size, checksum, bit stream), copies literal runs and back references to the output backwards and keeps an XOR checksum.
- `decode_and_write_table`: The routine decodes a byte stream from a pointer, accumulating offsets from a base value and writing the resulting short pairs into a global variable table.
- `update_sprite_position_table`: The routine updates a global position table for a sprite by accumulating delta values scaled with a speed factor and writing the resulting fixed-point results back into the table.
- `update_sine_table_entries`: The routine updates a global sine table using fixed-point arithmetic and a short table pointed to by a global variable.
- `multiply_32bit_fixed_point`: The routine takes two 32-bit fixed-point values from registers and returns a 32-bit fixed-point product.
- `multiply_fixed_point`: The routine multiplies a 16.16 fixed-point value in a register with a 16.16 fixed-point value from a global variable and returns a 32-bit result in the high part of the return value while preserving a separate input value in the low part.
- `draw_line_masked`: The routine computes a Bresenham line between two 16-bit coordinates and writes a mask bit into a planar bitmap and a secondary buffer, using a global flag to decide clearing or setting.
- `draw_scanline_segment`: The routine computes a set of integer values from its parameters and then repeatedly calls another routine while updating counters and two accumulators.
- `call_callback_with_threshold_check`: The routine calls another routine via a function pointer and then compares a short value with a short stored at an offset from a pointer.
- `empty_stub_routine_5`: The routine is 13 instructions long but performs no observable work; it does not read or write memory, does not access hardware registers, and does not call another routine.
- `store_indexed_pair`: The routine checks a signed index against a global limit and a base value, computes a pointer into a table using the index and a base pointer, clamps two input values to non-negative and an upper bound from a global variable, then stores the two clamped values as a short pair at the computed table entry.
- `update_rotation_matrix`: The routine reads two short values from a global table using an index derived from an input value, then computes four short-based products and shifts, storing the results into three global variables and a global workspace location.
- `line_draw_noop`: Empty draw routine (rts) chosen by the 3D line setup when a segment needs no drawing; its address is loaded into the draw-routine pointer.
- `find_min_max_bounds`: The routine scans a table of 16-bit pairs to compute the minimum and maximum values for the low and high words, then checks whether two global 16-bit values lie inside those bounds.
- `check_range_and_return_flag`: The routine calls another routine, then reads a 32-bit value from a global variable table using an index derived from two globals, extracts its high 16 bits as a signed short, and compares that value with another global.
- `find_min_max_and_sort`: The routine reads a series of 32-bit values from a table indexed by bytes in a buffer, keeps running minimum and maximum values for both the low and high 16-bit halves, writes the values into two temporary buffers in reverse order, and then performs additional min/max checks and stores results into global variables.
- `find_min_max_and_copy_table`: The routine copies a block of words from a source table into a destination table and a global array, tracking the minimum and maximum values of the block and of their high and low bytes.
- `compare_word_pairs`: The routine walks backwards through a table of 16-bit words pointed to by a register, comparing each word with the previous one using signed comparison.
- `check_sign_extension`: The routine loads a 16-bit value into a register, then checks whether the sign of the value is consistent with the sign of the same value after it is shifted left by eight bits.
- `find_next_table_entry`: The routine walks a table of 32-bit entries using a pointer in A2, comparing a short field from the current entry with a short field six bytes ahead, and builds a 32-bit value from consecutive shorts.
- `verify_sign_extension`: The routine loads a 16-bit value from a register and tests whether the sign bit is preserved when the value is shifted left by eight bits.
- `draw_scanline_pair`: The routine builds a 32-bit value from two input words and a pair of offsets, swaps the inputs if needed, computes a delta and a sign flag, then writes a series of words from a table to a custom hardware register while polling a busy flag in a global variable.
- `set_sprite_animation_offsets`: The routine writes a set of fields into a structure pointed to by a register, using values derived from two input words and a table of words.
- `wait_and_write_sprite_setup`: The routine waits for a hardware flag to clear, then writes several fixed words and computed short values into a structure pointed to by a register, using a small table of words and an input value that is incremented each iteration.
- `set_sprite_animation_offsets_2`: The routine writes several fixed words into a structure pointed to by a6, including a flag word built from a constant and a sign bit derived from two short inputs, and then loops four times to fill animation-related fields using a table of words and an accumulating offset.
- `copy_table_to_struct`: The routine copies a sequence of entries from a table pointed to by a pointer into fixed offsets of a structure pointed to by a pointer.
- `set_table_pointer_by_index`: The routine receives a value in D0 and uses the low 16 bits as an index.
- `process_tilemap_data`: Iterates through a table of word-sized values, performing bitwise manipulations and pointer arithmetic to update global state variables based on conditional checks.
- `set_blitter_copy_parameters_4`: The routine prepares a blitter copy by waiting for the blitter to be idle, then writes a mask and size to the blitter registers and loads source, mask and destination pointers and modulo values from tables indexed by a global variable.
- `clear_tile_bitmask`: The routine clears a bit in a tile bitmask and optionally sets it, using two input coordinates and a control flag.
- `store_byte_via_a6_2`: Stores a byte from a register into a global variable at an offset from a base pointer.
- `store_byte_via_pointer`: The routine stores a byte from a register into memory addressed by a pointer held in a register, using the pointer as a base with an offset.
- `set_display_state`: The routine clears a global flag, calls another routine, computes an address from a table entry and stores it and several constants into globals, then calls another routine and conditionally loops while calling other routines until a sentinel value is reached.
- `decrement_counter_dispatch_table`: The routine repeatedly decrements a global counter and, when it reaches zero, copies a global byte value and optionally calls another routine.
- `parse_script_byte_stream`: The routine processes a byte stream stored in a global buffer, interpreting control bytes to update a global variable and a pointer into the buffer.
- `store_input_and_update_state`: The routine stores the incoming value into a global variable, calls another routine, sets a flag to one and possibly changes it to four depending on a global variable, calls another routine again, and then conditionally calls another routine with either two arguments or one argument based on a global flag.
- `store_flag_and_dispatch`: The routine stores its input argument into a global variable, calls another routine, sets a flag global to one and possibly changes it to four depending on a global variable, calls another routine again, and then conditionally calls another routine with either two arguments or one argument based on the value of a global variable.
- `read_handler_entry`: A small handler entry reads a 16-bit word from a pointer, and if it is non-zero, it invokes another routine with a table argument and an optional second argument; otherwise it returns without action.
- `process_message_table`: The routine reads a message table pointed to by a global variable and walks its entries, extracting flags and text offsets.
- `decode_message_table_entry`: Routine initializes several global variables and decodes a message entry pointed to by A0 using a table.
- `decode_sprite_command_block`: The routine decodes a command block pointed to by a0 and uses a global table to resolve sub-commands, then iterates over a list of sprite descriptors, calling another routine for each entry and updating several global variables with fields from the descriptors.
- `check_string_terminator_and_reset_flag`: The routine clears a global variable, then calls another routine twice depending on whether the byte at offset one from the input pointer is zero, and if the global variable is non-zero it writes a constant to a global word.
- `process_extra_output_buffer`: The routine processes a byte buffer pointed to by a register, using the first byte as a loop count and repeatedly calling other routines while updating global variables.
- `increment_counter_and_recurse`: The routine checks whether a global variable matches a specific address and, if so, increments another global counter.
- `decode_input_byte_mask`: The routine reads a byte stream from a pointer and checks each entry against a mask obtained from another routine via a global function pointer.
- `configure_blitter_copy_4`: The routine programs Amiga blitter registers BLTAFWM, BLTAMOD, BLTAPTH, BLTCON0, BLTCON1, BLTDMOD, BLTDPTH and BLTSIZE to set up a blitter copy operation, then triggers the blitter by calling another routine.
- `configure_blitter_copymode`: The routine sets up Amiga blitter registers for a copy operation and stores source and destination pointers derived from two calls to another routine, with offsets added to the returned pointers.
- `wait_for_bit_clear`: The routine spins until a specific bit in a global variable is cleared, then returns.
- `configure_blitter_copy_5`: The routine writes a series of values to a global variable and then calls another routine four times, each time adding a constant offset to a pointer returned by the previous call.
- `configure_interrupt_controller`: The routine writes to CIAB registers CRA, CRB, ICR and PRA, modifies INTENA and INTREQ, and sets a global variable used as a handler pointer.
- `check_global_flag_and_call`: The routine writes a constant to a global variable, tests a bit in that variable and conditionally calls another routine, then sets another bit in the variable and returns the concatenation of the two input registers.
- `clamp_and_store_scaled_value`: The routine clamps a word input to a minimum, divides a global variable by that clamped value, stores the low and high bytes of the result into two byte locations derived from another global variable, writes the clamped value to a global variable, and returns a third global variable.
- `clear_input_buffer_and_reset_state`: The routine clears several global variables and then walks a byte table pointed to by a global pointer, comparing bytes to find a match and decrementing a counter.
- `clear_audio_dma_channels`: The routine writes zero to four audio volume registers and to a global variable, then writes a non-zero value to the same global variable and writes to DMACON and a custom hardware register.
- `reset_input_buffer_state`: The routine clears a global flag and a counter, calls another routine four times, waits with two short delay loops, then copies eight words from a fixed source area into a fixed destination area in RAM. It then updates a global index variable and a byte flag, and loops clearing the flag while adjusting the index until the flag is cleared.
- `audio_channel_select`: The routine updates a global audio control word and returns a channel index.
- `process_audio_channel_state`: The routine updates a per-channel state block pointed to by a6 using a table in a3 and a global table, handling a nibble-driven command byte, loading sample pointers and lengths into the state, and writing DMA control and audio channel registers AUD0LCH/AUD0LEN through AUD3LCH/AUD3LEN via DMACON and custom.
- `update_sprite_timer`: Updates a sprite timer field in a structure pointed to by a register and returns a masked timer value.
- `copy_word_from_table_2`: The routine copies a 16-bit word from a table pointed to by a register to a location pointed to by another register.
- `select_animation_frame`: The routine walks a table of 24 16-bit entries indexed by a byte from a structure, finds the first entry greater than a masked field of that structure, then adjusts the index and writes the selected entry into a field of the structure, clearing a flag byte and optionally setting it to one when the selected value is less than another field of the structure.
- `evaluate_animation_timer`: The routine updates a short timer field in a structure pointed to by A6 using a byte rate, and clears a limit field when the timer reaches it.
- `a_global_variable`: This routine computes a signed value and adds it to a short at offset 6 of a destination structure.
- `process_input_byte`: The routine reads a byte from a table pointed to by a register, masks it to obtain a command nibble and then performs a series of command-specific updates to fields in the same table and to a few global variables.
- `read_low_byte_from_global`: A routine that masks the value stored at an offset within a global variable and stores the masked result at another offset of the same variable.
- `clear_sprite_entry`: The routine reads a nibble from a byte in a structure, uses it as an index into a global table and adds the result to another byte in the same structure.
- `apply_masked_tile_to_planes`: The routine reads a mask byte from a table and uses it to mask and merge four source bytes into four bit planes of a destination tile buffer, writing the results to several plane offsets and a palette entry.
- `apply_sprite_mask_to_framebuffer`: The routine reads a mask byte from a table and a mask byte from a source buffer, combines them with the current framebuffer bytes using AND and OR operations, and writes the results to multiple framebuffer locations with fixed offsets.
- `set_sprite_clip_and_mask`: The routine computes clipping and mask parameters from input registers and global tables, writes a small descriptor into a caller-provided structure, and then iterates over a set of source bytes, combining them with a mask byte to update destination bytes and several fixed offsets within a block.
- `apply_sprite_mask_to_bitmap`: The routine reads mask bytes from a table via an index pointer and combines them with a mask byte from a source buffer, then applies the resulting mask to four destination bitmap planes using AND and OR operations.
- `return_concatenated_registers`: The routine calls another routine and then returns a 64-bit value formed by concatenating the contents of two registers.
- `return_concatenated_registers_2`: The routine calls another routine and then returns a 64-bit value formed by concatenating the contents of two registers.
- `copy_sprite_tables`: The routine copies 32-byte blocks from a source table to a destination table using two nested loops, then copies a second set of 32-byte blocks from another source to the same destination area.
- `copy_palette_block`: The routine copies a block of palette words from a table indexed by the inputs into a destination palette area pointed to by a pointer stored in a global variable, using two nested loops to copy four sub-blocks of 32 entries each, then copies a fixed six-byte offset block of 32 entries.
- `apply_sprite_mask`: The routine builds a 64-word mask by OR-ing four 64-word blocks from a source table into a destination buffer, then computes an index from two input values and uses the first 64 words of the built mask to AND-NOT selected words of a target table entry, clearing bits in the target.
- `copy_palette_blocks`: The routine copies 32 long-word pairs from a source pointer in the argument structure to four destination blocks in a global table, each block offset by a fixed stride, then copies another 32 pairs to a second global table.
- `copy_tilemap_to_buffer`: The routine copies a 64-word block from a global table indexed by the inputs to a destination buffer pointed to by a global pointer, repeating the copy four times with a large stride between source blocks.
- `clear_sprite_mask`: The routine builds a 32-word mask by OR-ing four words from a source table for each of 32 entries into a temporary buffer, then computes a destination pointer from two input values using a base table and an offset, and clears bits in 512 words of a destination mask by AND-ing each word with the bitwise NOT of the corresponding word from the temporary buffer.
- `copy_sprite_pattern_rows`: The routine copies a block of sprite pattern words from a source pointer in the argument structure to a destination area in a global table indexed by the argument fields, using a fixed row count and stride.
- `copy_palette_blocks_to_video_ram`: The routine copies 16-byte source blocks, with 7 blocks per set, repeated for 4 sets, from a global palette/table area to four distinct destination areas in video RAM, then copies a further set of 7 blocks to another destination area.
- `unknown_function_behavior`: The routine is not identifiable from the visible pseudo code.
- `reset_blitter_state`: The routine clears several global variables and repeatedly calls another routine while iterating over a table of words, copying values and updating pointers.

### explode.library

- `return_constant_value`: The routine returns a constant value without performing any other operations.
- `handle_message_request`: Reads a request structure from the caller, resolves a dos.library base via an indirect call and stores a handle, then extracts a short flag and a pointer from the request, and dispatches based on the flag value using table lookups, storing a result pointer and invoking further library routines before returning.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `decrement_and_poke_handlers`: The routine decrements a counter at an offset from a passed pointer and, if the counter reaches zero while a flag bit is set, it validates a nearby jump table and patches two entries in that table with values from the passed structure.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `validate_and_update_library`: Library code that checks magic values and function pointers within a structure to verify its integrity.
- `set_flag_in_structure`: The routine receives an address in a register and sets a flag bit in a byte field of the pointed-to structure.
- `validate_library_name`: The routine obtains a handle from another routine and checks a header field for a specific value.
- `check_library_name`: The routine obtains a library base pointer from a global variable, validates the library header magic and then checks the library name field for the string 'explode.library' in either forward or reversed byte order.
- `validate_library_name_2`: The routine obtains a library base pointer from a global variable, checks a version field and then validates the library name stored in the structure, accepting both forward and reversed byte order forms.
- `check_library_name_2`: The routine receives a pointer in a0 and checks whether the four words pointed to by that pointer spell a library name string.
- `explode_entry`: explode.library function -30 (called by an imploded program's stub): explodes the calling program's own segment list and returns into its real first hunk.
- `explode_program`: Explodes an Imploder V6 library-mode program in place: reads the descriptor after the stub's call, decrunches the packed hunk through the buffer hunk into the real hunks, applies the relocations and frees the two packing hunks.
- `copy_memory_blocks`: The routine copies a block of memory from a source area to a destination area, handling full 8-word chunks and a remaining byte tail, and updates a running sum into a table while looping until a counter in a global variable reaches a limit, then clears a flag and returns.
- `check_pointer_and_decrunch`: Library code that compares two pointers; if they are equal, it calls a routine to decrunch a block of data.
- `copy_block_words`: The routine copies a variable number of 32-bit words from a source area to a destination area, handling full 8-word blocks first and then any remaining words one by one.
- `copy_memory_blocks_2`: The routine copies 32-bit words from a source area to a destination area in a loop, handling a signed count and a termination condition based on a pointer reaching the end of a buffer.
- `copy_struct_array_loop`: The routine copies blocks of eight words from a source area into a destination area, then copies any remaining words one by one, using a global variable as a loop counter and a table of function pointers for repeated callbacks.
- `explode_decrunch_block`: Imploder decruncher: fills the buffer hunk with the next block of output, keeping its bit, literal-run and match state between calls.
- `decode_explode_byte`: The routine decodes a byte stream from a pointer in A0 using a shift-and-carry loop driven by D0 and D1, producing a small index in D0 and a length value in D1 for each iteration.
- `decode_bitstream_from_table`: The routine decodes a bitstream pointed to by a pointer in D1, using a table pointed to by A4, and writes the decoded bits into a destination buffer via A0.
- `extract_bits_from_source_buffer`: The routine walks a source byte buffer backwards, extracting bits from each byte with carry handling and using a table indexed by a counter to select offsets and lengths.
- `decode_bitstream_to_buffer`: The routine decodes a bit-packed stream pointed to by a pointer in D1, using a table pointed to by A4, and writes the resulting bytes into a buffer pointed to by A3 with bounds checked against A2 and D4.
- `decode_and_copy_bytes`: The routine decodes a byte stream by repeatedly extracting bits from a source buffer using carry-based bit extraction and uses the decoded values as indices into a table to compute offsets, then copies a block of bytes from a source area to a destination area using those offsets.
- `decode_bitstream_from_table_2`: The routine decodes a variable-length bitstream that is read from a byte pointer, using carry flags to extract bits and a table pointed to by a pointer to perform table lookups.
- `decode_and_copy_bytes_2`: The routine decodes a byte stream using a bitstream extracted from a source buffer and a table in memory, then copies the decoded bytes into a destination buffer while updating a counter in the table.
- `decode_and_copy_bytes_3`: The routine decodes a byte stream using a carry-propagating shift and a table indexed by a global variable, then copies the decoded bytes into a destination buffer while updating counters in a control block.
- `decode_table_shift_blocks`: A routine in the explode library that decodes a compressed block from a source buffer into a destination buffer using a byte stream interpreted as variable-length bit codes.
- `execute_callback_loop`: The routine reads a counter from a structure, increments it, and repeatedly calls another routine and a callback taken from a table indexed by a global variable until the counter condition is met, then calls the routine once more and clears a word in memory.
- `decrement_loop_counter`: The routine receives an unsigned value in an input register, converts it to signed, subtracts one, and if the result is positive, repeatedly decrements it until it reaches -1.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:Failat

- `initialize_memory_table`: The routine walks a linked list of entries to find the maximum of a field and then builds a small table in memory with a repeating pattern.
- `copy_table_to_stack`: The routine copies sixteen 32-bit values from a table located at an offset from a pointer in a2 into the memory area pointed to by a pointer on the stack, then calls another routine.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:endrun

- `process_command_line_input`: The routine checks two function pointers from a global variable and, if present, clears fields in a structure and updates several fields with a value returned by another routine, then copies a command-line string from the input buffer into a global buffer, skipping leading spaces and truncating at space, newline or null, and stores the length and a result from a library call into the structure.
- `empty_routine`: The routine contains no instructions and returns immediately.
- `infinite_loop_routine`: The routine contains an empty infinite loop and performs no operations.
- `clear_dma_control_block`: The routine clears two fields in a DMA control block pointed to by a pointer derived from an input value, writes all-ones to two other fields, then calls another routine via an indirect pointer.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 1 of 4).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Flashback (1992)(U. S. Gold)[cr Interpol](Disk 2 of 4).adf:bootblock

- `check_boot_block_flag`: The routine calls another routine through a pointer stored in a global variable and returns a flag value based on its result.
