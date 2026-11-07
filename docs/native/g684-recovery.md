# Game 684 (FantasyWorldDizzy v1.1 0120): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00684). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2029108 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 142 raw-word lines |
| Functions named and described | pass, 271 of 271 named (0 author symbols), 271 of 271 described (11 by rule, 257 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2441 of 2441 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9069 of 10418 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 405551 |
| code | 41022 |
| data | 1582468 |
| padding | 67 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 78 | 6 |
| Disk.1:bootblock | 1 | 124 | 3 |
| logged disk load from Disk.1 boot-block load | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 10170 | 256 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 27 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 15 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |

## Cases handled

- FantasyWorldDizzy_v1.1_0120/FantasyWorldDizzy_v1.1_0120_hdf/FantasyWorldDizzy/Disk.1: custom-format (non-DOS) disk, 1710 non-empty blocks: matched against track-loaded segments
- FantasyWorldDizzy_v1.1_0120/FantasyWorldDizzy_v1.1_0120.hdf: hard-disk image (AmigaDOS OFS, 2176 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree FantasyWorldDizzy_v1.1_0120_hdf, which is recovered file by file; no block outside the filesystem holds data
- FantasyWorldDizzy_v1.1_0120/FantasyWorldDizzy_v1.1_0120_hdf/FantasyWorldDizzy/Disk.1: the boot block reads 22528 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- 3 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- FantasyWorldDizzy_v1.1_0120/FantasyWorldDizzy_v1.1_0120_hdf/FantasyWorldDizzy/Disk.1: 3 logged loads cover 443904 bytes of the image
- FantasyWorldDizzy_v1.1_0120/FantasyWorldDizzy_v1.1_0120_hdf/FantasyWorldDizzy/Disk.1: blocks 2-273 (bytes 1024-140288) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.10)
- FantasyWorldDizzy_v1.1_0120/FantasyWorldDizzy_v1.1_0120_hdf/FantasyWorldDizzy/Disk.1: blocks 365-479 (bytes 186880-245760) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.39)
- FantasyWorldDizzy_v1.1_0120/FantasyWorldDizzy_v1.1_0120_hdf/FantasyWorldDizzy/Disk.1: blocks 1212-1715 (bytes 620544-878592) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.59)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `patch_library_vectors`: Calls a function pointer from a structure, then calls another routine and compares its result to a constant; if equal, it writes jump instructions and routine addresses into several fixed memory locations and conditionally sets a global variable, then jumps indirectly.
- `loop_forever_on_vhposr`: Reads the vertical/horizontal position register and then loops forever without returning, so it never exits.
- `wait_for_blitter_done`: Writes a value to the blitter size register and then polls the DMA control register until the blitter busy bit clears, returning once the blit has finished.
- `call_indirect_via_table`: Loads a function pointer from an offset within a table and calls it indirectly, returning the value left in the data register.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.

### Disk.1:bootblock

- `bootblock_load_and_jump`: Sets up an I/O request, calls the exec library, copies a block to low memory, then copies a fixed-size block to a low address and transfers control there.
- `copy_block_to_low_memory`: Copies a fixed-size block of bytes from a source buffer to a destination buffer, decrementing a counter until it wraps to -1.
- `decompress_lz_bitstream`: Copies a 256-entry table and a block of input bytes into a work buffer, then decodes a bit-packed LZ-style stream using two offset tables, writing the expanded bytes through a pointer.

### logged disk load from Disk.1

- `run_initialization_sequence_and_wait`: Calls a long series of setup routines in order, setting a global flag and constants along the way, then loops calling an update routine and incrementing a global counter until the counter reaches a limit or its high bit or a specific character value is set, after which it calls several more routines and sets a global to one.
- `increment_global_and_toggle_word`: Increments a global variable, XORs a 32-bit value at a fixed high address with a pattern, then resets the global variable to zero and returns a byte formed by ORing an input byte with a constant.
- `loop_until_nonzero_result`: Repeatedly calls another routine and clears a global variable after each call, looping while the returned value is nonzero.
- `wait_for_flag_then_clear_audio`: Loops calling another routine and clearing a global variable until the global is nonzero, then calls a routine that clears audio and interrupts and sets the global to one.
- `loop_calling_routine_and_clear_audio`: Repeatedly calls another routine and clears a global variable while the returned value is nonzero, then calls a routine that clears audio and interrupts and clears the global.
- `compute_grid_position_and_loop`: Computes a quotient and remainder of a value by eleven, stores them in the stack frame, calls another routine, then loops while subtracting a clamped count from a frame word, calling routines and adjusting a frame word, and finally computes a result from the frame values.
- `run_disk_loader_sequence`: Loops up to three times, clearing frame words and calling a routine; when a frame word is zero and a global flag bit is set, it writes fixed values through a pointer, calls several routines, and branches on their results, otherwise it calls another routine with a status value.
- `disk_dma_transfer_loop`: Loops calling a routine, checking a global flag, writing to disk DMA and control registers, and retrying up to two times before returning.
- `validate_input_with_retry`: Repeatedly calls helper routines to check a flag and decrement a counter, returning early if the flag is clear or the counter expires; if the loop completes, it validates register values against memory locations, updates a short variable, and returns a status code.
- `scan_object_table_with_checks`: Loops over an object table, calling helper routines to test flags, compare fields, and perform bit tests, returning status codes when conditions fail or match.
- `empty_stub_routine`: The routine consists of five instructions that perform no observable work and then return.
- `set_flag_and_increment_counter`: The routine sets a bit in a 16-bit word located a few bytes before a pointer in A6, using a bit index derived from D3.
- `clear_table_entries`: The routine clears a table by writing zero to successive entries.
- `extract_high_byte_pair`: Calls another routine, takes its 32-bit result, and returns a 32-bit value formed by the low 16 bits of the result in the low half and the original high 8 bits of the result in the high half.
- `interleave_word_bits`: The routine reads two consecutive words from a pointer in A0 and returns a value formed by taking the low bits of the second word and the low bits of the first word shifted left by one.
- `call_and_forward_result`: Calls another routine, stores the returned value in a local variable, then calls a second routine passing that value as an argument.
- `xor_fixed_block_checksum`: The routine XORs together ten consecutive 32-bit words starting at a fixed offset from a base pointer and returns the result masked to retain only alternating bits.
- `compute_xor_checksum`: The routine iterates through a sequence of words pointed to by an input pointer, XORing each word into an accumulator.
- `fill_memory_with_pattern`: The routine writes a repeating 32-bit pattern to a block of memory obtained from a global variable.
- `initialize_object_state`: Calls another routine and uses its return value together with two output values to compute three integers stored at fixed offsets from a pointer, and writes constant words and a derived word to other offsets in the same structure.
- `set_object_transforms`: Calls several routines and writes their return values and constants into fields of a structure pointed to by a register, updating offsets for position, size, and control words across multiple sequences.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `reset_blitter_state`: The routine clears a global variable and writes constants to several custom chip registers, including the blitter control and mask registers and the DMA control register.
- `adjust_byte_flag_bits`: The routine calls two other routines, then reads a byte through a pointer and conditionally sets or clears its high bit based on a flag byte before the pointer and a bit in the byte itself, storing the modified byte back.
- `clear_or_set_flag_bit`: Reads a byte through a pointer and tests the low bit of the preceding byte; depending on that flag it either clears the high bit of the byte or, if the high bit is not already set, sets it, then writes the byte back through the pointer.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `set_disk_dma_control`: Writes control values to custom chip registers, stores an incoming pointer into a disk pointer register, and stores a value derived from the incoming word with its high bit set into a global variable.
- `wait_for_flag_bit`: Calls a routine, tests a bit in a structure, and loops calling another routine until the bit is set, then writes constants to global variables and returns a status.
- `set_global_flags`: Writes the value 2 to a global variable and then overwrites it with a larger constant value.
- `set_adkcon_and_clear_bit`: Writes a constant to a custom chip register and then clears a bit in a global variable selected by a value read from a stack frame slot plus three.
- `set_bit_in_global_flag`: Computes a bit index from a stack frame value plus three, masks it to five bits, and sets the corresponding bit in a global variable while preserving the incoming byte's other bits.
- `set_flag_and_wait`: Writes a constant to a global variable, then calls two routines that set a bit in a global flag and wait for a result.
- `delay_based_on_word_difference`: Reads a word from a global table indexed by a frame variable, optionally calls another routine, computes the difference between two shifted words, and delays by calling another routine repeatedly for the absolute difference before storing a word back into the table.
- `poll_joystick_fire_wait`: Polls a global variable and repeatedly calls another routine while decrementing a counter from 85 down to zero; if the global variable's bit 4 is clear, it clears a 16-bit entry in a table indexed by a value from the caller's stack frame and returns zero, otherwise it continues looping and finally returns a non-zero value.
- `read_port_and_store_masked_byte`: Reads a byte from the CIAB peripheral port register and optionally clears a bit in it based on an extra output from another routine, then stores the masked byte with the low bit forced set into a global variable and calls another routine.
- `update_ciab_prb_bit_2`: Calls a routine that builds a byte from a global variable and clears a selected bit, storing the result back into the global variable.
- `update_ciab_prb_bit`: Builds a byte from a global variable with the low bits set, clears a bit selected by a stack frame value plus three, and additionally clears another bit when a table entry indexed by that value has its low bit set.
- `wait_for_flag_and_result`: Repeatedly calls another routine until it returns 1, spinning while a global variable's least significant bit is set, then returns zero.
- `set_global_values_conditionally`: If a global variable's least significant bit is clear and the data register is not 1, writes three successive byte values to a global variable.
- `set_global_values`: Writes three successive byte values to a global variable and returns.
- `update_object_animation_and_motion`: Iterates over a fixed table of object structures, decrementing timers, applying horizontal and vertical velocity according to direction bits, advancing animation frames through a scripted sequence, and clearing objects whose script or state terminates.
- `update_object_positions_from_table`: Iterates over a fixed number of object structures, and for each whose index field is not minus one, either clears its first word or sets two position words from a table entry indexed by that field.
- `store_byte_and_words_then_call`: Stores a constant byte and two incoming word values into fields of a structure pointed to by an address register, then calls another routine.
- `link_objects_on_overlap`: Checks whether two objects' bounding boxes overlap using their position and size fields; on overlap it links the two objects through pointer fields, copies position and size fields into secondary slots, and zeroes those copies when a flag byte is set.
- `clear_linked_object_and_compare_extents`: If the object's linked-object pointer is non-null, it clears that pointer, then computes signed horizontal and vertical extent differences between the object and the linked object using their size fields and direction bytes, and compares the two resulting values.
- `compare_object_offsets_2`: Computes signed offsets for two objects from their size fields and direction bytes, adds them to incoming values and byte fields, and compares the resulting horizontal and vertical differences.
- `compare_object_offsets`: Computes signed offsets for two objects from their size fields and direction bytes, adds them to incoming values and byte fields, and compares the resulting horizontal and vertical differences.
- `compare_signed_offsets_of_two_objects`: Computes a signed offset for each of two objects using a size field and a direction byte, combines them with an incoming value and a byte field, and returns the incoming argument unchanged.
- `return_argument_unchanged`: Returns its incoming argument unchanged without modifying any data.
- `return_argument_unchanged_2`: Returns its single argument unchanged; the body is effectively empty, passing the input value straight back to the caller.
- `repeat_render_loop`: Repeats a fixed number of times, calling two routines each iteration.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `store_four_byte_constants`: Writes four byte constants in sequence to global variables.
- `store_six_byte_constants`: Writes six byte constants in sequence to global variables.
- `xor_global_with_rotated_register`: XORs a global variable with an inverted value and with a rotated register value minus seven, returning a masked byte.
- `xor_global_with_rotated_value`: Inverts a global variable, then XORs it with a value derived from a register rotated left by three bits and decremented by seven.
- `xor_and_rotate_global`: Inverts a global variable, then XORs it with a value derived by rotating a register left by three bits and subtracting seven.
- `increment_global_four_times`: Adds one, two, three and four successively to a global variable.
- `rotate_global_with_carry`: Repeatedly adds a global variable to a value rotated right with the incoming carry, then XORs the result with the global variable and stores the final rotated value back.
- `initialize_global_state_block`: Stores two incoming words and a series of constant bytes and words into fixed global locations, then clears several global variables.
- `draw_offset_sprite_sequence`: When a global flag is clear, advances two counters, then repeatedly reads bytes through pointers and adds signed offsets to words at fixed positions in a structure, calling another routine between each step.
- `update_actor_movement_and_animation`: Reads a global state byte, applies directional movement offsets to two position words, dispatches through a table of routines, then clamps vertical position, clears animation counters, and triggers animation sequences and buffer clears based on the actor's fields.
- `run_channel_animation_sequence`: Drives a channel structure through a multi-stage animation sequence: it clears and sets fields, selects frame pointers from tables based on a sequence counter, advances the counter with wrap limits, and branches on global flags and structure state to pick the next stage, calling another routine to update channel state.
- `reset_structure_fields_and_sequence`: Sets a global variable to two, clears low bits of a word and several fields in the passed structure, then runs the sequence interpreter and stores a new pointer and zeroed fields in the returned structure.
- `decrement_counter_and_set_animation`: If a global counter is nonzero, calls a routine, and when the low bits of a register are zero decrements the counter, clears bits in a word through a pointer, stores bytes and a word through the same pointer, calls two routines, and selects one of two data pointers based on flag bits.
- `update_channel_state_on_flag`: If a global flag is set, calls another routine and, when the low seven bits of a register are zero, decrements the global, sets fields in a structure pointed to by a register, and conditionally stores a pointer and byte based on flag bits.
- `dispatch_on_global_state`: Compares a global variable against several byte values and, depending on matches, stores data pointers and a byte, toggles a global flag, calls a routine, and conditionally updates a word variable.
- `dispatch_state_byte_to_table`: Reads a state byte and, depending on its value, either stores fixed word values into a table, toggles a flag and calls another routine, or resets several state bytes and table entries.
- `offset_object_and_test_overlap`: Adds incoming deltas to an object's position, temporarily sets global size fields from doubled inputs, calls an overlap routine, restores the fields and clears two link words, returning status flags.
- `clear_links_on_zero_flag`: Calls another routine and, if the zero flag is set, clears a link word in each of two structures.
- `link_and_clear_object`: Calls two routines, and if a byte register is non-zero, clears a pointer field and stores the byte exclusive-ored with three into a global variable.
- `empty_routine`: Contains no instructions and returns immediately.
- `run_scripted_state_sequence`: A state machine driven by a global counter: on the first pass it initializes a block of globals and object fields from a selected script entry, and on later passes it advances the counter, and at a threshold runs a long sequence of other routines, clears a table of object records, and resets the counter.
- `draw_text_glyph_tiles`: Writes character codes into a table of tile/bitmap entries, filling a run of repeated glyph bytes between a start and end marker, then calls the toggle routine and other helpers to update the display.
- `draw_vertical_text_columns`: Reads a byte stream of drawing commands, and for each command writes character bytes into several parallel column buffers, filling runs of repeated characters and terminating each column with a minus-one byte, then advances the stream and repeats.
- `push_byte_into_ring_buffer`: When a nonzero byte is passed in, it shifts a small global byte buffer down by one, stores the new byte at the front, and clears the entry indexed by the new value.
- `push_byte_into_ring_buffer_2`: If a byte argument is nonzero, shifts a four-byte history buffer down and stores the new byte at its head, then clears the indexed slot.
- `push_byte_into_ring_buffer_3`: If the incoming byte is nonzero, it shifts a small ring buffer of bytes forward, stores the new byte at the head, and returns the byte that was displaced.
- `shift_and_scan_byte_buffer`: When given a nonzero byte, shifts a four-byte history buffer and stores the byte, then clears the indexed entry in a byte table.
- `update_direction_and_move`: Reads a global direction/state byte, handles special values by calling other routines and setting flags, then tests a candidate direction against a collision routine and, if clear, shifts a four-byte history buffer and stores the new direction.
- `update_input_and_move_object`: Reads a global input/state byte and, when it is nonzero and flagged, dispatches to helper routines, updates a small history buffer of recent bytes, and conditionally calls routines that test and move an object based on shifted coordinate values.
- `run_sequence_if_global_is_at_sign`: Checks a global byte for a nonzero value equal to the character '@'; if so, it calls a series of routines, sets the global to one, calls more routines, clears the global, and then loops calling a routine and setting the global to one until the global's high bit is set, after which it calls further routines and clears the global.
- `tiny_ori_2`: Small routine of 1 instruction that merges 255 into (a5).
- `copy_tile_data_to_buffers`: Copies a block of tile data from one memory area to two destination buffers, then calls two tile-blitting routines twice each and sets a global flag.
- `format_and_blit_tiles`: Calls a formatting routine and two tile-blitting routines, setting a global variable to a constant between the blits and clearing it afterwards.
- `tiny_or`: Small routine of 1 instruction that merges d0 into d0.
- `periodic_tile_blit_on_counter`: If a global variable is nonzero, increments it modulo four and, when it wraps to zero, calls a formatting routine and two tile-blitting routines with a global toggled by a constant, then restores the original value.
- `blit_tile_columns_repeatedly`: Calls two tile-column blitting routines twice each, then repeats a pair of calls a number of times determined by a global byte minus one, decrementing until it wraps.
- `format_number_and_blit_tiles`: Calls a number-formatting routine and two tile-column blitting routines, sets a global byte to a value, calls the blitting routines again, then clears the global byte.
- `advance_counter_and_call_twice`: Reads a global byte, and if nonzero increments it modulo four; when the result is zero it calls one routine, toggles a bit in the byte, then calls another routine twice, finally restoring the original byte value.
- `advance_counter_and_toggle_blit`: Reads a global byte, and if nonzero increments it modulo four; when the result is zero it calls a routine, toggles a bit in the byte, and calls two blitting routines, then restores the original byte.
- `blit_tiles_and_set_flag`: Calls a formatting/tile-blit routine twice, sets a global variable to a constant, calls two tile-blit routines twice, clears the global variable, and if a byte flag is clear sets it and adds a constant to a global before calling another routine.
- `fill_and_set_bitplane_words`: Starting from a global pointer offset, writes word patterns into four interleaved planes for a run of entries, then sets or clears the high bit and a low bit in words spaced across the planes for a longer run, and finally writes the same word patterns for a trailing run.
- `init_bitplane_border_masks`: Writes fixed word patterns into four parallel tables: a short run of words set to all-ones, zero, and all-ones, then a longer run that sets or clears the high bit of each word and the low bit of a word offset within each row, and finally another short run of all-ones, zero, and all-ones words.
- `copy_block_of_bytes`: Copies a fixed-length block of bytes from one memory area to another, decrementing a counter until it wraps.
- `copy_block_of_bytes_2`: Copies a fixed-size block of bytes from one memory area to another, decrementing a counter until it wraps to minus one.
- `insert_string_entry`: Scans a string table for a terminator, shifts entries to make room, and inserts a new entry with three bytes from registers.
- `find_and_shift_entry`: Scans a table of three-byte entries for one matching a byte and two shifted coordinates, then shifts subsequent bytes down over the found entry until a terminator byte is reached.
- `find_entry_and_store_byte`: Scans a table of three-byte entries for one matching a byte and two shifted coordinates, then stores a byte from a register into the first byte of the matching entry.
- `tiny_move_movea_rts`: Small routine of 3 instructions that copies d7 to (a0), then copies (a7)+ to a0, then returns.
- `scan_terminated_lists`: Sets a pointer to a fixed memory area and repeatedly scans forward through byte lists terminated by a byte value of minus one, for a count derived from a global byte minus one.
- `dispatch_via_jump_table`: Uses a byte value shifted to index a pointer table, then indexes a second table from that pointer and jumps indirectly through the resulting code pointer.
- `allocate_first_free_slot`: Scans a fixed array of slots for one whose first word is zero, then fills that slot's fields from a parameter block and from registers, initializing counters and pointers.
- `allocate_slot_from_table`: Scans a fixed-size table of records for an empty entry and, on finding one, fills it with fields copied from a source structure plus two register-supplied values, marking the entry as active.
- `allocate_slot_with_subentries`: Finds a free slot in a fixed array, fills its fields from a parameter block, then fills up to four following sub-slots from a list of pointers, stopping when a pointer is zero.
- `scroll_playfield_by_direction`: Scans a string table, then based on a global direction value adjusts the vertical or horizontal position of a series of structures by fixed amounts.
- `copy_and_clear_slot_array`: For each of several slots, clears three words and copies a fixed-length block of bytes to another area, then clears the first word of each slot in the array.
- `clear_globals_and_dispatch_objects`: Zeroes four global variables, then if a flag is clear iterates over two tables of object records, calling the function pointer stored in each active record (and, for the second table, only records whose status byte equals a sentinel).
- `advance_script_and_decode_tiles`: Steps a script pointer through a byte stream, handling control bytes that reset or back up the pointer and set a direction flag, then reads a tile byte and applies mirroring based on the direction flag.
- `call_and_clear_links_on_zero`: Calls another routine and, if the zero flag is set, copies a byte and a pointer from a structure into global variables.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `toggle_flag_and_redraw`: Tests the sign bit of a byte in a structure; if set, clears that bit and, when a global flag is clear, sets the flag, calls a formatting routine and two tile-blitting routines with the flag changed between them, then clears the flag and calls the formatting routine again.
- `process_script_state_and_advance`: Checks a global sign and structure fields to update a script pointer, call routines, and set fields, then advances a script pointer and decodes tiles while handling control bytes.
- `handle_negative_flag_simple`: If a byte in a structure is negative, clears its sign bit and, depending on a global, either sets the global and calls two routines or just calls one routine.
- `toggle_flag_and_dispatch`: Tests the sign bit of a byte in a structure; if set, clears that bit and, when a global flag is clear, sets the flag, calls several routines, adds a constant to the global, and recursively invokes itself, clearing the flag and calling another routine if a condition is met.
- `handle_negative_flag_and_update_offset`: If a byte at an offset from a pointer is negative, clears its high bit and, depending on a global byte, calls routines, sets the global to various values, and stores a byte through the pointer; otherwise calls another routine and, if the global is zero, scans a table for an entry with a specific value and stores a computed word at an offset from the pointer.
- `scan_structures_and_write_tilemap`: Iterates over a fixed number of structures, skipping empty ones, and when a structure's pointer field matches a constant and its word field is within a range, writes a constant word to several table entries indexed by that field and calls another routine.
- `advance_object_animation_frame`: For an object whose status byte is non-negative, searches a table for the object's current value, increments a frame counter until it matches the table entry, then resets the counter and sets the status byte to a sentinel before calling another routine.
- `process_command_byte_loop`: Checks a global byte and, if it has the high bit set, clears that bit and loops calling routines and dispatching through a table indexed by the byte, stopping when the byte reaches a limit or a termination value.
- `advance_pointer_with_wrap`: If a global variable equals one, it advances a pointer stored in that variable, and when the pointer reaches a specific value it resets it and sets the global variable to a fixed value.
- `init_bitplane_border_masks_2`: Writes fixed word patterns into four parallel tables: a short run of words set to all-ones, zero, and all-ones, then a longer run that sets alternating bit patterns, and toggles a global byte's high bit while calling another routine repeatedly.
- `copy_tile_data_to_buffers_2`: Copies a block of tile data from one memory area to two destination buffers, then calls two tile-blitting routines twice each and sets a global variable.
- `initialize_and_run_animation_loop`: Initializes global state and object tables, fills parallel word tables with fixed patterns, copies and converts a table of object records into a second table, then loops calling animation and position update routines while advancing a counter until a limit or flag is reached, after which it runs a final sequence of setup routines and returns.
- `expand_tilemap_to_bitmap`: Loops over a grid of tile indices, reads each index byte through a pointer table, and copies four bytes from a tile data table into a destination buffer at two row offsets.
- `expand_tilemap_to_bitmap_2`: Reads bytes from a table of tile indices, uses each as an offset into a lookup table of four-byte entries, and writes the four bytes into a destination buffer as two rows of two bytes per cell, advancing across a fixed-width grid of cells and rows.
- `expand_tile_map_to_bitmap`: Reads a byte from a table indexed by each word of a source array, uses it to index a table of four-byte entries, and copies each entry's two byte pairs into a destination buffer as two rows of a tile block, advancing across an 18-column by 10-row grid.
- `expand_tilemap_to_bitmap_words`: Reads a byte from a table indexed by each entry of a 9-by-18 array of word indices, then uses that byte times four to fetch four consecutive bytes from a second table and store them as two 16-bit words into a 9-by-18 grid of word pairs.
- `expand_tilemap_to_bitmap_3`: Reads a 9x18 grid of byte indices from a table, uses each index to fetch four bytes from a lookup table, and writes those four bytes into a 36-byte-per-row destination buffer, expanding each source cell into a 2x2 block.
- `expand_tile_indices_to_words`: Reads a byte from a table indexed by each entry of a 9-by-18 array of word indices, uses it to fetch four bytes from a byte table, and writes those bytes as two words into a 9-by-18 destination grid, advancing the source pointer and destination pointer per cell.
- `expand_tile_grid_to_bytes`: Loops over a nine-by-eighteen grid of word indices, reads a byte from a table indexed by each index, and copies four bytes from a tile data table into a destination buffer.
- `combine_data_registers`: Returns the incoming data registers combined with a constant high word, producing a long value from the two register inputs.
- `return_data_registers_unchanged`: Returns the incoming data registers combined into a long value without modifying any memory.
- `copy_tile_block_and_fill`: Copies a block of words from a table indexed by a global variable into a destination buffer, calls another routine, then fills a run of words with the constant value two.
- `fill_word_table_with_two`: Writes the value two into a table of one hundred eighty words through an incrementing pointer.
- `dispatch_state_byte_to_table_2`: Reads a state byte and, depending on its value, either stores fixed word values into a table, toggles a flag and calls another routine, or returns.
- `update_tilemap_and_scroll`: Adjusts positions of two sets of structures, compares a grid of tile indices with a stored copy, updates a row from a table, and when a counter wraps, clears a buffer and calls a routine.
- `shift_columns_and_copy_rows`: Adds a constant to a global word, advances a word field in each of two arrays of 14 entries, then copies 9 rows of 18 words from one buffer to another while setting a flag word when a value changes, copies one more row selected by a global index, increments that index, and on wrap resets the index, clears a 14-entry array, sets a global word and calls another routine.
- `return_data_registers`: Returns the incoming data register values unchanged, combining them into a single 64-bit result; it performs no other work and makes no calls.
- `expand_tilemap_to_tiles`: For each of two rows of tile indices, tests a byte in a lookup table indexed by the low bits of each index; if nonzero, copies a 32-byte tile record from a table into an output buffer, otherwise zeroes the record.
- `expand_tilemap_to_tiles_2`: Recursively processes a tilemap structure, then for each of two rows of tile indices tests a byte in a lookup table indexed by the low bits of each index; if nonzero, copies a 32-byte tile from a table into an output buffer, otherwise writes zeros.
- `stamp_tile_pattern_at_position`: Computes a grid offset from two coordinates in a structure, bounds-checks it, and writes a two-by-two or three-by-three block of the value two into a tile table depending on a type field.
- `periodic_tile_remap`: When a global flag is clear, increments a counter and every third call walks a table of word indices, replacing each entry with a value from a lookup table and setting a corresponding status word to two.
- `clear_state_tables`: Writes zero to a series of word-sized global variables, then stores the byte values 63, 64, 65, 66, 63, 64, 65, 66 into eight consecutive words, and finally zeroes further pairs of words in several tables.
- `blit_tile_grid_with_offsets`: Sets up blitter registers and, depending on a global flag, walks a nine-by-eighteen grid of tile indices, decrementing a per-cell counter and issuing a blit for each nonzero cell using a tile data table and a destination offset.
- `clip_and_step_line`: Orders two coordinate pairs, then, if the smaller coordinate is below a limit, copies four consecutive longwords and a word from a table indexed by a shifted register value and walks a line between the two points, stepping one coordinate and clamping it against bounds while looping on a masked counter.
- `decrement_loop_word`: Decrements a word in a data register after ORing its low byte with a constant, looping until the word reaches all ones or a counter reaches two hundred.
- `loop_decrementing_or_value`: Repeatedly ORs the low word of the data register with a constant, decrements it, and exits when it reaches minus one; otherwise it increments a second word and continues while that word stays below two hundred.
- `loop_decrementing_masked_word`: Repeatedly ORs the low word of the data register with a constant, decrements it, and exits when it reaches minus one; otherwise it increments a separate word counter and continues while that counter stays below two hundred.
- `loop_on_byte_condition_2_2`: Loops while the low byte of a register ORed with a constant is negative or while an incremented counter remains below a limit, decrementing the register each iteration.
- `loop_decrementing_or_value_2`: Repeatedly ORs the low word of the data register with a constant, decrements it, and exits when it reaches all-ones, while also incrementing a separate word counter until it reaches two hundred.
- `loop_or_and_decrement`: Repeatedly ORs the low word of the data register with a constant, decrements it, and returns when it wraps to all ones; otherwise it increments a separate word counter and loops while that counter stays below two hundred.
- `decrement_or_loop_counter`: Loops while a byte formed from the low word of the data register ORed with a constant is negative, or while an unrelated word counter stays below a limit; each iteration decrements that ORed value and returns when it wraps to all ones.
- `loop_on_byte_condition_2`: Loops while the low byte of a register ORed with a constant is negative or while an incremented counter remains below a limit, decrementing the ORed value each iteration and returning when it reaches all ones.
- `decrement_loop_word_third`: Decrements a word in a data register after ORing its low byte with a constant, looping while the sign bit is clear or a second register counts down past fifteen.
- `decrement_counter_until_limit`: Loops decrementing a word counter while its low byte ORed with a constant stays negative or the counter remains above a small limit, returning early when the counter wraps to all ones.
- `loop_decrementing_or_masked_counter`: Repeatedly ORs the low word of the data register with a constant, decrements it, and returns when it wraps to all ones; otherwise it decrements a second word register and loops while that value stays above a threshold.
- `loop_on_byte_condition`: Loops while a byte formed from the low word of a register ORed with a constant is negative, or while a decremented counter register remains above a limit, decrementing the word each iteration and returning when it wraps to minus one.
- `loop_until_counter_reaches_limit`: Repeatedly decrements a word value after ORing it with a constant and returns when the value becomes all ones, or exits when a signed byte test fails and an incremented counter reaches a limit.
- `loop_or_and_decrement_word`: Repeatedly ORs a word value with a constant, decrements it, and returns when it wraps to all ones; otherwise increments a second word until it reaches a limit.
- `decrement_or_value_loop`: Loops while the low word of the data register ORed with a constant stays negative as a signed byte, decrementing that word each pass and returning when it wraps to all ones; otherwise increments a second register's word until it reaches a limit.
- `loop_until_counter_expires`: Loops decrementing a data register while a byte condition holds, incrementing another register until it reaches a limit, and returns when the decremented value wraps.
- `store_line_endpoints_and_deltas`: Stores five incoming word values into fields of a structure pointed to by an address register, then computes and stores the absolute differences and sign directions for two coordinate deltas.
- `step_along_line_to_target`: Iteratively advances two coordinates using stored increments and an error accumulator until they match stored target coordinates, then stores the final coordinates and error.
- `format_number_as_decimal_digits`: Writes a terminator byte and then decimal digit characters derived from a value by repeated division by ten into a buffer at a fixed offset plus an index.
- `tiny_ori_ori_ori`: Small routine of 3 instructions that merges 0 into d0, then merges 0 into d0, then merges 0 into d0.
- `format_number_as_decimal_digits_2`: Writes a byte value of 255 at a fixed offset plus the low word of the first argument, then converts the first argument into decimal digits and stores them as ASCII characters into a buffer at that offset, using a digit count from the second argument (with a five-digit split when it exceeds four).
- `format_number_into_buffer`: Writes a fixed-width decimal string of a value into a byte buffer, filling the leading positions with '0' digits and terminating the buffer with a 255 byte.
- `decrement_word_until_odd_zero`: Takes a word from a register, subtracts one, then repeatedly decrements it while skipping odd values until it reaches minus one, at which point it returns.
- `fill_bytes_and_call`: Calls a small routine four times, fills eight consecutive bytes with a constant value, sets two global bytes, then calls another routine and a small routine.
- `set_bit_and_loop_eight_times`: Saves a global word, sets its high bit, calls another routine eight times in a loop, then restores the saved word.
- `copy_blocks_and_update_map`: Waits on a global byte, copies a large block of longwords into a computed destination with four interleaved offsets, adjusts a global byte by bit tests, calls another routine, and updates a byte in a table based on sign checks.
- `scan_table_and_copy_digits`: Copies six characters from a table row into a fixed buffer, substituting a zero for spaces, calls another routine, and advances through rows until a comparison against a global succeeds or six rows are processed.
- `copy_blocks_and_call_conversion`: Copies a computed number of sixteen-byte blocks backwards, copies eight bytes from one global area to another, calls a conversion routine, then copies six bytes between two buffers.
- `test_byte_and_halt`: Reads a byte from the data register, ORs it with a constant, and branches on whether the result is zero; the disassembly at this point is invalid, so the routine does not execute a valid instruction sequence.
- `clear_custom_registers_and_enable`: Writes enable values to the interrupt and DMA control registers and then clears eight longwords starting at a custom chip register.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `set_display_and_interrupt_control`: Writes three fixed values to the DMA control, interrupt request, and interrupt enable registers.
- `store_constant_in_global`: Stores a fixed constant value into a global variable and returns.
- `increment_byte_and_call_chain`: Increments a global byte, then calls a series of other routines in sequence, and finally stores a constant into the global byte.
- `clear_byte_buffer`: Loops over a fixed count, storing zero bytes through a pointer to clear a block of memory.
- `clear_memory_block`: Repeatedly writes zero bytes through an incrementing pointer until a fixed counter expires, zeroing a memory region.
- `zero_fixed_region`: Clears a fixed-size area of memory by writing zero bytes through a pointer in a counted loop.
- `wipe_counted_bytes`: Stores zero bytes sequentially through a pointer for a fixed number of iterations, clearing a buffer.
- `reset_large_buffer`: Fills a fixed-length memory range with zero bytes by looping a store through an advancing pointer.
- `clear_small_buffer`: Zeroes a fixed number of bytes by writing through a pointer in a decrementing counted loop.
- `blank_fixed_area`: Writes zero bytes through a pointer over a fixed count, blanking a memory area.
- `clear_bytes_loop`: Clears a fixed-length block by storing zero bytes through an incrementing pointer until the counter reaches minus one.
- `clear_and_set_global_bits`: Clears the low bit and high bit of a global byte, then sets the high bit four times in succession.
- `spin_infinite_loop`: Contains an empty infinite loop with no data operations.
- `toggle_flag_bits`: Waits while a global variable compares less than itself, then toggles bits of that variable with several exclusive-or operations.
- `toggle_global_bits_repeatedly`: Toggles the low bit of a global byte, then toggles its high bit five times in succession.
- `check_global_value_for_halt`: Checks if a global variable equals a specific character value and enters an infinite loop if it does.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `initialize_display_registers`: Writes constant values to the bitplane modulo, control, data-fetch and display-window hardware registers to configure the display.
- `initialize_blitter_registers`: Writes constant values to the blitter control and first/last word mask registers and enables DMA channels via the DMA control register.
- `loop_thirty_calls_to_two_routines`: Calls two other routines alternately thirty times in a loop.
- `blit_masked_sprite_rows`: Clips a sprite descriptor's vertical extent against the screen, selects a mask/merge mode from its horizontal field, and copies masked rows of pixel data into a bitmap, shifting the mask and data when a horizontal offset is present.
- `blit_masked_sprite_rows_2`: Reads a sprite descriptor from a pointer, validates its type, vertical position and horizontal offset, then blits masked rows of 32-bit words into a planar bitmap, clearing bits with the inverted mask and OR-ing in the source data across four bitplanes.
- `render_clipped_sprite_rows`: Validates a sprite descriptor's type and vertical extent against the screen, clips the top and bottom, computes a screen buffer offset from its horizontal field, selects a mask/merge mode, and copies masked rows of pixel data into the buffer while advancing by a fixed row stride.
- `render_scrolling_sprite_rows`: Validates a sprite descriptor's type, vertical position and horizontal offset, computes a screen buffer offset and a source pointer from a table, then blits masked 32-bit rows into the display buffer while advancing by a fixed row stride, optionally flipping vertically.
- `blit_masked_tile_column`: For each byte in a list terminated by a byte value of minus one, it indexes a tile in a table using a global offset, then for eight rows copies four bitplane bytes into the destination at the given column and row offset, combining them with the existing pixels using the tile's mask (inverted OR of the four bytes) before ORing in the tile bits.
- `blit_unmasked_tile_column`: For each byte in a list terminated by a byte value of minus one, it indexes a tile in a table using a global offset, then for eight rows copies the four bitplane bytes directly into the destination at the given column and row offset without any masking.
- `blit_masked_tiles_to_bitplanes`: For each entry in a list of tile indices terminated by a sentinel, it reads a 128-byte tile pattern and ORs its four bitplanes into a destination bitmap at a computed offset, using the low bit of a counter to choose between word-wide and byte-wide masked writes.
- `blit_tile_rows_from_list`: For each word in a list terminated by minus one, indexes a table by that word times one hundred twenty-eight and copies sixteen rows of bytes into a screen buffer, using word or byte copies depending on a parity bit.
- `copy_tile_rows_from_indexed_table`: For each word in a list terminated by minus one, indexes a table by that word times one hundred twenty-eight and copies sixteen rows of bytes into a destination with fixed row offsets, alternating between two copy patterns based on a counter.
- `return_stack_value`: Returns the value passed on the stack unchanged.
- `clear_8k_block`: Writes zero to a block of memory starting at the address in A0, repeating 512 times over 16 longwords each.
- `copy_8k_block`: Copies 512 blocks of 16 longwords from the address in A0 to the address in A1.
- `interleave_copy_blocks`: Copies 800 groups of 20 words from A0 into four interleaved destination areas based at A1, writing every fifth word to each area.
- `copy_sixteen_words_to_global`: Copies sixteen words from the address in a register to a fixed global area, then calls another routine.
- `store_word_indexed_by_register`: Stores a word from a data register into a global table at an offset computed from another data register, then calls another routine.
- `increment_nibbles_conditionally`: Waits for a global flag to equal two, then for each of sixteen words in a table increments the high, middle, or low nibble when the corresponding masked bits of a source word exceed decreasing thresholds, repeating for eight passes and calling another routine each pass.
- `decrement_nibbles_conditionally`: Waits for a global flag to equal two, then for each of sixteen words in a table decrements the high, middle, or low nibble when the corresponding masked bits are nonzero, repeating eight times and calling another routine each pass.
- `clear_sixteen_word_table`: Clears sixteen consecutive words in a global table to zero, then calls another routine.
- `copy_sixteen_words_doubled`: Copies sixteen words from a fixed source table to a global area, doubling each word by shifting left one bit.
- `set_global_and_call`: Stores a constant word into a global variable, stores a byte into another global variable, and calls another routine.
- `call_and_set_global`: Calls another routine, stores a constant byte into a global variable, and returns.
- `handle_cia_control_bits`: If bit three of a global byte is set, rotates that byte and writes a sequence of values to a global variable; otherwise if bit one is set, writes zero then eight to the same variable.
- `reset_cia_if_bit_set`: If bit 1 of D0 is set, writes zero then eight to a global variable, touching the CIA control registers.
- `read_joystick_and_update_flags`: Reads joystick and mouse port data and the CIA port, derives direction bits from those values, merges them into a global word, and if the result is nonzero clears and masks a global byte and calls another routine.
- `store_constant_word`: Stores a constant word value into a global variable.
- `read_mouse_port`: Reads the mouse port movement and button registers and stores a constant word into a global variable.
- `adjust_global_bytes_and_set_flags`: Adjusts two bytes of a global word by subtracting their own values, tests bits of the result, and conditionally sets bits in the global word.
- `load_word_from_table_if_flag`: If a global flag is nonzero, loads a word from a table indexed by a byte in a data register into a global variable, clears the flag, calls another routine, and sets the flag to one.
- `wait_on_register_bit`: Calls another routine, invokes a routine through a computed pointer, polls a hardware register bit until it clears, invokes another routine through a pointer, and calls another routine.
- `clear_audio_dma`: The routine writes to the four audio volume registers and to the DMA control register, setting the audio channels off and clearing DMA enables.
- `compute_and_store_channel_offsets`: Reads a version byte from a fixed header, scans a table of per-entry values to find the maximum, and combines that maximum with another header byte and a base offset to form a starting offset.
- `compact_pointer_table`: Scans four entries of a table; for each non-null pointer, clears the entry and copies eight words from the pointed-to data into the table slot.
- `update_channels_and_audio`: Decrements a global counter, updates three channel structures by adding a byte to a word and writing position and control words to audio registers, calls routines conditionally, and delays before processing flagged channel bytes.
- `update_object_records_from_table`: Iterates over a small set of object records, reading fields from a table and updating per-object bytes, words, and a global flag based on the record's type and value codes; also writes a word to a hardware register and adjusts a stored word by adding or subtracting a value.
- `spawn_object_from_table`: Looks up a per-type entry in a table indexed by a byte from a structure, and if the entry is active, copies its fields into the object's state, sets position and size, and optionally negates the first bytes of the object's data; otherwise clears the object's active flags and copies a fallback entry.
- `set_player_sprite_offset`: The routine writes a 16-bit value from a table indexed by a byte in D4 to the word at offset six from the pointer in A1.
- `iterate_three_channels`: Loops three times over a structure spaced by a fixed stride, and for each entry whose flag byte is nonzero calls another routine.
- `update_three_channel_animations`: Advances three independent animation channels held in a structure: each channel counts down a delay, then steps an index through a table, computes a scaled value from a global table and writes it to an output field, wrapping the index and optionally clearing the channel's enable flag.
- `initialize_three_channel_structures`: Writes a fixed set of values into three parallel data structures, setting a count and a base value and clearing several fields in each.
- `clear_audio_and_interrupts`: Writes zero to several audio volume and control registers and clears interrupt and DMA enable bits, then zeroes a series of global variables.
- `select_channel_and_update_state`: Uses the low byte of the data register to index a table of channel structures, and depending on a state word either stores the data and direction bytes into globals and calls a routine, or updates a bit in a global byte based on the direction byte and calls a routine.
- `chain_guarded_subroutine_calls`: Checks a global flag; if clear, it calls other routines in a nested chain and returns early once the flag becomes set.
- `store_constant_and_return_byte`: Stores a constant value into a global variable and returns the low byte of the data register unchanged.
- `store_constant_global`: Stores a constant value into a global variable and returns.
- `process_interrupt_requests_2`: Reads pending interrupt bits, walks a list of structures, and for each flagged entry sets a bit, optionally stores constants into fields, and updates a global variable.
- `process_interrupt_requests`: Reads pending interrupt bits, walks a list of structures, and for each flagged entry sets a bit, optionally stores constants into fields, and loops until all bits are handled.
- `initialize_channel_tables_from_index`: Uses the low byte of the data register to index a table of offsets, sets up three channel structures with pointers and flags, fills a buffer with an incrementing byte pattern, and writes initial values including a final byte.
- `run_script_bytecode_interpreter`: Interprets a byte-coded script stream: reads opcode bytes, handles extended opcodes that set a counter or terminate, loads register values from a table, and dispatches to another routine for each command while managing delay counters.
- `store_one_through_pointer`: Writes the byte value one to the address held in the address register passed in.
- `run_sequence_interpreter`: Interprets a byte-coded sequence through a state structure: advances a delay counter, decodes command bytes (including table lookups, immediate byte stores, and repeat counts), and dispatches entries by writing command words, counts, and handler pointers into a hardware/context frame.
- `decode_command_byte`: Examines a byte through a pointer and, depending on its value, stores a table-derived pointer, sets a flag byte, or stores a masked nibble into fields of a structure.
- `advance_sequence_state`: Updates a state byte and counters in a structure, fetches bytes from a pointer, and writes a table word to a hardware register based on the resulting index.
- `init_interrupt_handler_frame`: Saves registers and context into a frame, clears a word through a pointer, stores a handler address, and loads a word and pointer from a source structure, branching on a flag bit.
- `dispatch_audio_interrupt`: Calls three routines, copies fields from a structure into audio hardware registers, conditionally clears a flag and updates globals, decrements a counter, and jumps through a stored pointer.
- `process_interrupt_structure`: Checks a flag in a structure, copies fields into hardware registers, conditionally clears a bit and updates two globals, decrements a counter, then calls a routine through a pointer from the structure.
- `advance_byte_with_limit_2`: Adds a byte from the caller's frame to another byte in the caller's frame, and if the result is negative or exceeds a limit byte, clamps it and stores a callback pointer.
- `advance_byte_with_limit`: Adds a byte from the caller's frame to another byte in the caller's frame, and if the result is negative or exceeds a limit byte, clamps it to the limit and stores a routine address into a stack slot.
- `set_callback_if_zero`: Checks a short value in the caller's stack frame; if zero, stores the address of another routine into a stack slot and returns.
- `update_position_and_flag`: Adds a byte from one structure to a byte in another, and if the result is negative and a flag bit is clear, clears the flag and stores constants into fields of a pointed-to structure.
- `clear_memory_block_2`: Clears a block of memory by writing zero bytes in a loop until a counter reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_2`: Contains a single instruction and no observable effect; it is empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `blit_loop_over_table_entries`: Loops over a table of entries, advancing a pointer by six bytes per iteration and calling another routine for each entry while setting up blitter path and data registers.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `scan_structure_flags`: Calls a routine, then iterates over four entries spaced apart, checking a flag byte and calling another routine when the flag is set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately`: This routine is empty: it performs no operations and simply returns to its caller.
- `return_immediately_2`: The routine contains no instructions and returns immediately without touching any data or registers.
- `return_immediately_3`: The routine is empty and simply returns to its caller.
