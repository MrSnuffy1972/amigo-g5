# Game 2514 (escape from colditz (1990)(digital magic)(m4)[cr agl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02514). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 49 raw-word lines |
| Functions named and described | pass, 171 of 171 named (0 author symbols), 171 of 171 described (5 by rule, 166 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1086 of 1086 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5057 of 6574 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 34097 |
| code | 24530 |
| data | 819507 |
| padding | 10 |
| unused | 23040 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf:bootblock | 1 | 165 | 6 |
| logged disk load from Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf boot-block load | 1 | 155 | 2 |
| logged disk load from Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf boot-block load | 1 | 6253 | 162 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- escape from colditz (1990)(digital magic)(m4)[cr agl]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf: custom-format (non-DOS) disk, 1696 non-empty blocks: matched against track-loaded segments
- escape from colditz (1990)(digital magic)(m4)[cr agl]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf: the boot block reads 8704 bytes from disk offset 878592 to an address (found statically); recovered as a unit
- escape from colditz (1990)(digital magic)(m4)[cr agl]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf: the boot block reads 56320 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- escape from colditz (1990)(digital magic)(m4)[cr agl]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf: 2 logged loads cover 65024 bytes of the image
- escape from colditz (1990)(digital magic)(m4)[cr agl]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf: blocks 121-491 (bytes 61952-251904) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.09)
- escape from colditz (1990)(digital magic)(m4)[cr agl]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf: blocks 494-1302 (bytes 252928-667136) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.59)
- escape from colditz (1990)(digital magic)(m4)[cr agl]/Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf: blocks 1304-1709 (bytes 667648-875520) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.61)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `decode_message_block` (bootblock): The routine searches a table at a fixed address for an entry whose bytes match values derived from an input word and a global state, then verifies a checksum built from the entry and a global mask. Kind: disk/track/checksum check. Pass path: The loop that performs bitwise manipulation/decryption of data into the destination buffer (pbVar9) after the checksum/equality validation passes..

## Functions


### Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf:bootblock

- `initialize_hardware_registers`: The routine is the boot block entry called by the Kickstart strap and contains 30 instructions.
- `mask_and_wait_for_equality`: The routine masks a global variable, writes a constant to a global variable, computes a value from an input register and stores it in a global variable, then repeatedly calls another routine until the sum of two input registers equals the routine's output register, after which it masks the first global variable again, sets a bit in it, and returns a combined value from two input registers.
- `sync_memory_to_pointer`: The routine repeatedly clears a byte pointed to by a pointer until a zero terminator is reached, then adjusts the pointed byte toward a saved byte value by setting or clearing a flag in another byte and calling another routine each step, and finally waits until a flag bit in a global variable is cleared.
- `clear_flags_and_wait`: The routine clears a flag in a byte pointed to by an address register, sets a different flag in the same byte, writes constant values to three offsets from that base, then spins until a bit in one offset clears and spins again until a bit in another offset clears.
- `decode_message_block`: The routine searches a table at a fixed address for an entry whose bytes match values derived from an input word and a global state, then verifies a checksum built from the entry and a global mask.
- `empty_routine`: The routine contains five instructions and performs no observable work; it ends with a return.

### logged disk load from Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf boot-block load

- `extract_word_and_pack_byte`: The routine reads a 32-bit value from four bytes before a pointer in A0, stores the high byte of D7 together with the low byte of that value through a pointer in A5, and returns the value shifted right by one bit with the sign bit set.
- `process_bitfield_loop`: The routine runs a loop over a field-sized counter derived from an input short, shifting an input integer right each iteration and, if the value becomes zero, replaces it with the return value of another routine and updates the counter from a short returned in the loop.

### logged disk load from Escape from Colditz (1990)(Digital Magic)(M4)[cr AGL].adf boot-block load

- `copy_code_to_memory`: The routine clears a global variable and a RAM location, then copies a block of code from another routine into a table of words in memory.
- `copy_code_to_global`: The routine copies a block of code from its own image into a global variable table, word by word, until the counter wraps, then clears the global variable twice.
- `clear_flag_and_initialize_state`: The routine clears a flag bit in a byte pointed to by a pointer, and if the flag was set it writes constants and zero to fields of a structure pointed to by a register, calls another routine, reads a value from a location pointed to by another pointer, adds it to a global variable and stores the sum and further constants into the structure.
- `store_offset_and_sum`: The routine writes a constant to a field in a structure pointed to by a register, adds an input value to a second field from another register and stores the result, copies a word from a register into a third field, and returns the sum of the input value with a register value.
- `clear_object_flags`: The routine clears a flag in a structure and writes constant bytes to fixed offsets of the structure.
- `clear_object_flags_and_return_delta`: The routine checks a flag byte via a pointer and, if the flag is clear, writes four bytes to fixed offsets in the pointed-to structure and clears two bytes, then returns the difference between an input word and a word stored in the structure.
- `copy_sprite_attribute_table`: The routine sets a byte in a structure pointed to by a pointer and then repeatedly copies a fixed set of word fields from a source table to a destination table, advancing both pointers each iteration until a counter reaches zero.
- `set_flag_and_copy_table`: The routine writes a constant to a field of a structure pointed to by a1, calls another routine, then copies four word values from one table to another table using a pointer pair.
- `initialize_object_fields`: The routine reads two values from an input structure using offsets, adds a global offset to each, and writes the results into a destination structure at fixed offsets.
- `compute_sprite_offset`: The routine computes an offset value from a set of input pointers and flags and returns the sum of an input base with that offset.
- `store_three_fields`: The routine calls another routine and then stores three values from registers into consecutive fields of a structure pointed to by a register.
- `decode_byte_stream`: The routine processes a byte stream pointed to by a global variable and a pointer, using two calls to another routine before the work.
- `search_for_value_in_table`: The routine receives a value and a pointer to a table of 16-bit entries.
- `decode_bitstream_to_word_table`: The routine decodes a bitstream from a byte buffer into a table of 16-bit words.
- `initialize_global_table`: The routine clears a global variable and then walks a byte table at a fixed address, comparing each byte with the previous one while decrementing a counter.
- `clear_audio_channels`: The routine clears a set of audio channel volume registers and writes zero to a table of pointers, then sets a global variable to zero.
- `mute_audio_channels`: The routine writes to the four audio volume registers and to DMACON, disabling audio DMA. It then writes a constant to a global variable six times, the last write storing a non-zero value.
- `reset_audio_channels`: The routine resets the Amiga audio hardware by writing to the four audio channel registers and copies a set of values from a table in memory to the audio control registers.
- `decode_sound_effect`: The routine decodes a packed sound-effect descriptor from a buffer, looks up a table entry and writes channel length and pointer values into a structure and an output area.
- `mask_and_compare_word`: The routine reads a word from a pointer, masks it to the low twelve bits, stores the result into an offset of the same structure and clears a byte at another offset.
- `handle_game_command`: The routine processes a command structure passed via registers.
- `call_and_return`: The routine calls another routine, then calls itself recursively, and then returns.
- `process_game_command_loop`: The routine has 24 instructions, is called by handle_game_command and touches custom+0.
- `handle_message_state`: The routine updates a message structure and a timer, copies state words between a global block and a message block, and invokes a handler selected by a table index.
- `update_player_state`: The routine reads a timer value from a table indexed by a short in the input structure, clamps it, and writes derived bytes to a global variable and to several fields of a player structure.
- `decode_item_attributes`: The routine reads a global flag and a table of function pointers, calls one of them to obtain a value, clamps it and stores it together with several derived offsets into a structure pointed to by A1.
- `compute_adjusted_difference`: The routine has 9 instructions, is unreached as gap code and calls empty_routine_2.
- `empty_routine_2`: The routine contains no instructions and performs no operations; it returns immediately.
- `decode_item_attributes_flags`: The routine has 35 instructions, is called by decode_item_attributes and calls set_or_clear_bit_and_update_table.
- `set_or_clear_bit_and_update_table`: The routine sets or clears a single bit in a byte pointed to by a pointer, using a bit index derived from a register value.
- `set_entity_position_and_flags`: The routine prepares a structure pointed to by a pointer in memory and writes position, size and flag fields into it, including clamped coordinates and a small generated table in RAM. It also updates a global variable and a table entry based on a signed value read from a pointed structure.
- `process_entity_state`: The routine updates fields in a structure pointed to by a1, copies a short from a global pointer, clears and sets flag bits in a byte field and in a global variable, and calls another routine conditionally.
- `initialize_sprite_structure`: The routine sets up a sprite descriptor in memory, writing size, position and derived values computed from a global state and a table, and updates flags in a global variable.
- `compare_two_pairs_of_values`: The routine loads two short values from a table indexed by a word in D0 and two short values from a structure pointed to by A5.
- `set_object_flags_by_table`: The routine computes an index from two input words and a global flag, looks up a table entry, and tests a byte at an offset from that entry.
- `check_position_bounds`: The routine checks whether a global variable matches an input word and, if so, whether two input words lie within ranges defined by two global variables.
- `handle_collision_response`: The routine reads short values from a global structure, compares them against a computed range derived from the input value, and updates several flag bytes in the same structure and in a table entry pointed to by a pointer.
- `process_object_flags`: The routine walks an array of objects and updates per-object flags based on position, a character code table and the object timer fields, and it disables matching object timers.
- `reset_object_flags`: The routine clears a global byte and then iterates three times, testing a set of flag bits in a global object structure and clearing or setting bits accordingly, invoking another routine repeatedly and updating a counter in a global variable.
- `process_input_buffers`: The routine repeatedly reads flags from two byte buffers and updates a global variable and a small set of memory locations, calling other routines in a fixed sequence.
- `process_object_collision_flags`: A small routine that reads and modifies bytes in two object structures, clearing and setting flag bits in the second structure and masking bytes in the first structure based on flag tests.
- `clear_object_flags_2`: The routine clears two flag bits in a byte pointed to by a pointer and then zeroes a byte at an offset from a pointer plus an index.
- `configure_music_player_state`: The routine configures a global variable used by the music player, sets a flag in a control byte based on a flag in a parameter block, and repeatedly clears and sets bits in two control bytes while updating a counter.
- `decrement_and_compare_byte`: The routine computes an address from an input pointer and a word offset, then decrements a byte at that address and checks if it became zero.
- `return_combined_registers`: The routine calls another routine and then returns the concatenation of the two input values it received in registers D0 and D1.
- `set_copper_list_entry`: Checks a flag in a global variable and updates a table entry if the flag is clear.
- `inspect_empty_routine`: The routine contains five instructions and performs no observable work; it ends immediately without reading or writing memory or calling other routines.
- `search_table_for_value`: The routine walks a table of entries and compares a global short with the entry value.
- `initialize_message_struct`: The routine writes constant values and copies fields from a source structure pointed to by a0 into a destination structure pointed to by a1, and clears two words in a third structure referenced by an uninitialized register.
- `decrement_global_and_dispatch`: The routine decrements a global counter when it is non-zero and returns the incoming word.
- `decrement_global_and_dispatch_2`: The routine decrements a global counter when it is non-zero and returns.
- `decrement_counter_and_check_flags`: The routine checks a global flag and a byte indexed by a short field in a structure.
- `decrement_counter_and_check_zero`: The routine checks a global flag and a byte in a table indexed by a short value taken from an input structure.
- `set_global_values_and_loop`: The routine writes several constant values to a global variable, calls another routine, and then runs a small decrementing loop that exits when the variable reaches zero or a specific condition is met.
- `poll_joystick_buttons`: The routine reads the joystick and mouse fire buttons via the CIAA port register and stores the value in a global variable.
- `set_global_and_return_concat`: The routine calls another routine, stores a constant value into a global variable, calls another routine, and returns a 64-bit value formed by concatenating the two input arguments.
- `copy_word_table`: The routine copies sixteen 16-bit words from a source table derived from a global variable to a destination table.
- `copy_word_table_2`: The routine copies a block of 16-bit words from a source pointer in A0 to a fixed destination table in memory, decrementing a counter in D7 until it reaches zero.
- `set_flag_and_store_word`: The routine stores a 16-bit value from a register into a word field at a fixed offset from a pointer, then sets the least-significant bit of a byte pointed to by another pointer.
- `apply_room_masks_and_wait`: The routine initializes global state and a room descriptor, then applies a set of mask/or word pairs from a table to several fields of a block of room structures, iterating over the entries.
- `copy_word_blocks`: The routine copies a small fixed-size block of memory repeatedly.
- `initialize_room_descriptor`: The routine clears a set of globals, computes an index from inputs and a table, and then writes constants and computed values into a structure pointed to by a register, using another routine to fill parts of it.
- `copy_word_and_add_offset`: The routine reads a word from an offset in a structure pointed to by a register, calls another routine, then writes that word back to the same offset in another structure and adds a short value from a global variable to a short field in the second structure.
- `fill_object_table`: The routine calls another routine ten times and writes the values returned in registers to a global structure ten times, updating a field with an added constant on each iteration when a flag in a byte pointed to by a pointer is clear.
- `populate_object_table_entries`: The routine loops ten times, calling another routine each iteration and writing the returned values into consecutive entries of a table pointed to by an address in a register.
- `initialize_custom_chips`: The routine writes to custom chip registers to set up the copper list and DMA control, then copies a small table from a fixed location into another area, zeroing the source entries.
- `empty_infinite_loop`: The routine contains no instructions and immediately enters an infinite loop.
- `escape_from_colditz_routine`: This routine stores a byte from a register through a pointer.
- `unknown_routine`: The routine reads a byte from a pointer, updates two global short values using a table lookup, and then conditionally calls another routine and manipulates several global variables and a table lookup before returning a concatenated 32-bit value.
- `decode_tile_pair`: The routine decodes a pair of tiles by looking up two tables through indices derived from input registers and a structure pointer, then compares four words from those tables with a mask derived from a flag in the input structure and a shift value.
- `compare_table_to_global`: The routine compares a short from a table with a short from a global variable, advancing through the table and the global variable in lockstep.
- `compare_two_word_tables`: The routine loads two word tables using indices derived from input registers and a pointer argument, then compares four 16-bit words from each table with a mask derived from a global variable and an input register.
- `decrement_byte_and_clear_flag`: The routine reads a 16-bit index from a structure, computes an address in a byte table using that index and an input word, then decrements the byte at that address.
- `update_object_state`: The routine writes constants to a global variable and to memory pointed to by a register, then calls another routine.
- `process_input_device_state`: The routine reads the joystick/mouse fire buttons and disk status from CIAA_PRA, updates a global variable and a set of memory-mapped flags, and then walks a small table of pointers to copy and swap word fields between two structures while toggling a few bytes.
- `initialize_object_table`: The routine clears and sets fields in a global structure array, then copies a short value from a register into a table entry and updates a global counter.
- `decode_object_table`: The routine walks a two-dimensional table pointed to by a0 and compares each entry against a global table of 16-bit patterns.
- `decrement_global_counter`: The routine reads a 16-bit value from a global variable, stores it back unchanged if it is zero, otherwise decrements it by one and writes the result back to the same global variable.
- `search_tile_in_table`: The routine looks up a pointer from a table using a value derived from an input register and then scans a two-dimensional table of bytes.
- `fetch_table_entry`: The routine computes an index from a short value taken from an address derived from a register and a signed offset, multiplies it by a constant, adds an offset derived from another signed value, and returns a 16-bit value from a global variable table at the computed offset.
- `clear_object_flags_and_copy_fields`: The routine clears a global variable and sets a flag in a byte pointed to by a pointer stored in the caller's structure.
- `skip_if_flag_set`: The routine checks a flag in a byte pointed to by a pointer in A1.
- `sort_short_table`: The routine fills a small code buffer with a sequence of short values and then repeatedly performs a bubble sort pass over a short table stored in memory, swapping adjacent entries when they are out of order and toggling a flag in the code buffer to indicate a swap occurred.
- `sort_table_by_swapping`: The routine initializes a small table of short values with a sequence and then repeatedly performs a bubble sort pass over a global table of shorts, swapping adjacent entries when they are out of order and swapping two words in the routine itself as a side effect.
- `set_copper_registers`: The routine reads a short from a pointer, checks a flag in a byte and conditionally toggles a bit in a byte and loops while a bit in a table entry is set, then writes a value derived from the short to a global variable and programs several custom chip registers with constant values.
- `decode_message_payload`: The routine reads a byte stream using a bitmask check and loads fields into a global structure, updating a counter and linked offsets while looping over fixed steps.
- `decode_message_lengths`: The routine processes a block of four-byte entries using a bitmask in a register to decide which entries to handle.
- `set_objective_flag_and_trigger_hardware`: The routine checks a set of flags in an object structure and a global variable, then sets a flag in a global table and increments a counter in a per-object table.
- `set_copper_list_registers`: The routine writes to a global variable and a table entry, then sets a byte flag via a pointer.
- `search_object_in_table`: The routine scans a zero-terminated table of structures and checks whether the 16-bit values at offsets two and four lie within eight of two 16-bit values from a global context.
- `decrement_loop_call`: The routine loads a short value from a global variable, then repeatedly calls another routine while decrementing the value until it reaches -1.
- `decrement_global_loop`: The routine loads a short value from a global variable, then repeatedly calls another routine while decrementing the loaded value until it reaches -1.
- `decrement_global_loop_2`: The routine loads a short value from a global variable, then repeatedly calls another routine while decrementing the loaded value until it reaches -1.
- `decrement_loop`: The routine loads a short value from a global variable and then repeatedly calls another routine while decrementing the value until it reaches -1.
- `process_message_queue`: The routine processes a message queue stored in a global variable, dequeuing entries and updating flags in the global variable and a per-message state structure.
- `initialize_global_and_loop`: The routine sets a global variable to a pointer to another routine and then runs a small loop that repeatedly reads a byte from a code pointer, calls another routine when the byte is positive, and decrements a counter until it reaches -1.
- `self_modifying_loop`: The routine sets a global variable to point to another routine and then executes a small self-modifying loop that decrements a counter from three while advancing a code pointer, invoking a function when a byte is positive and otherwise using an extra output pointer.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `process_object_state`: The routine walks a table of object records in memory and updates fields of each record based on flags in the first byte, copying data from a referenced block and advancing an index through a byte table.
- `clear_callback_table`: The routine checks a global short counter.
- `execute_dispatch_table`: The routine reads a signed count from a global variable and, while the count is non-negative, repeatedly calls a routine obtained by indexing a table of function pointers with an offset derived from another global variable and the current loop index.
- `copy_object_fields_to_state`: The routine copies several fields from a source structure pointed to by a0 into a state structure pointed to by a6, including a 32-bit value, two 32-bit values, and several 16-bit values.
- `initialize_structure_from_table`: The routine writes a constant to the first word of a table pointed to by a0, reads a word from that table and uses it to compute a short value stored in a global structure, copies several words and a long from the table into fixed offsets of a global structure, and clears a long field in that structure.
- `initialize_object_from_template`: The routine copies fields from a template structure pointed to by a0 into a working object pointed to by a6.
- `init_game_objects`: The routine clears and initializes several global variables and a RAM area, copies a small block from one memory area to another, updates a player-state structure with a few flags and values, and sets up several object descriptors by writing constant fields and computing derived values from input words, then fills a table with accumulated sums.
- `clear_byte_buffer`: The routine clears a block of memory by writing zero bytes through a pointer.
- `fill_memory_area`: The routine fills a memory area pointed to by a pointer with a constant byte value, decrementing a counter until it reaches zero.
- `set_object_flags_and_copy_data`: The routine sets a flag bit in a byte pointed to by a pointer, clears several bytes at fixed offsets from that pointer, writes a constant into one of those offsets, copies a 32-bit value from a global variable indexed by a word-sized value into the object, and copies a 16-bit value from a table indexed by the same word-sized value into the object.
- `set_memory_flags`: The routine sets a flag byte by ORing in a bit, clears several bytes in a structure, writes a constant into one byte, and copies two values from a pointer into fields of the structure.
- `copy_word_triple_step`: The routine copies 16-bit words from a source pointer in A0 to a destination pointer in A1, advancing the destination by three words each iteration while advancing the source by one word, repeating until a counter in D7 reaches -1.
- `clear_tables_and_recurse`: The routine clears a small table of words, then adds a constant to a series of integers and updates a global variable, then recursively calls itself several times.
- `set_custom_copper_list`: The routine stores a short from a register into a global variable and then into a table indexed by that short, writes a constant to a global variable, and if the low byte of the input differs from that constant it updates a global variable and programs a set of custom copper registers with values derived from the input and constants, otherwise it returns without further effect.
- `reset_global_and_call`: The routine clears a global variable to zero, calls another routine and returns a 32-bit value from register D0.
- `store_byte_and_set_hardware_registers`: The routine compares a byte from a register with a byte from a global variable table indexed by another register.
- `update_display_parameters`: The routine checks a byte value in D0 against a global variable and, if they differ, stores the byte into the global variable and writes several constant values into a set of global variables, then returns.
- `set_palette_entry`: The routine stores a byte from a register into a global variable and then writes several constant values to a set of memory locations.
- `set_blitter_copy_parameters`: The routine stores a byte from a register into a global variable and, when the value changes, writes a set of blitter control words to custom chip registers.
- `process_script_table`: The routine walks a short script table pointed to by a global variable, compares entries and dispatches based on a signed index, updating a global pointer and invoking another routine.
- `copy_table_bytes`: The routine copies a block of bytes from a table indexed by a value in a register to a destination pointer, advancing the destination by a fixed stride each iteration.
- `copy_bytes_from_table`: The routine copies 64 bytes from a table pointed to by a global variable offset by the low word of the first argument shifted left by six, into a buffer pointed to by the second argument, writing one byte at a time with a stride of 40 bytes between successive writes.
- `poll_until_global_clears`: The routine calls another routine, then repeatedly calls it again while a global variable is non-zero, calling it once more each iteration, and finally returns a 64-bit value formed from the two input registers.
- `poll_until_global_clears_2`: The routine calls another routine, then repeatedly calls another routine while checking a global variable, breaking when the variable is zero, and finally returns a 64-bit value formed from two input registers.
- `propagate_table_value`: The routine loads a short from a global variable and uses it as a loop counter.
- `decrement_global_loop_3`: The routine reads a short value from a global variable, then repeatedly calls another routine while decrementing the value until it reaches -1.
- `decompress_tile_block`: The routine processes a block descriptor in a register argument.
- `fill_sprite_palette_entries`: The routine updates a block of sprite palette entries from a table using a length byte and a count variable, then fills any remaining entries with a constant value.
- `fill_memory_table_from_script`: The routine reads a script byte table indexed by a global variable and copies a set of six bytes from a fixed data table into a destination memory table for each non-zero script entry, then fills the remaining entries of the table with 255.
- `clear_memory_table`: The routine clears a table pointed to by a pointer by writing a byte value to a series of fixed offsets and then stepping the pointer while decrementing a counter.
- `copy_word_table_3`: The routine copies sixteen 16-bit words from a table derived from a global variable to two destinations: a fixed memory area and another global table.
- `set_flag_and_dispatch`: The routine reads a byte from a global variable, sets its high bit, and if the high bit was previously clear and the entry indexed by the original byte value in a table of routine pointers is non-null, it calls the indexed routine.
- `infinite_loop_stub`: The routine contains four instructions that form an empty infinite loop with no observable side effects.
- `fill_table_from_register`: The routine writes a global variable and then fills three entries of a table in memory with a base value taken from a register and a global variable, adding a constant offset to each successive entry.
- `store_word_table`: The routine writes a 32-bit value from a register to a table of four 16-bit entries in a global variable, incrementing the source value by a constant each iteration and advancing the destination pointer by four bytes, repeating three times.
- `fill_table_with_cumulative_sums`: The routine writes a 32-bit value from a register into a global table as two 16-bit words, advances the pointer, adds the input registers together for the next iteration, and repeats while a counter register decrements.
- `store_word_pair_loop`: The routine writes a 32-bit value from a register to memory via a pointer, storing the low word then the high word, advances the pointer by four bytes, adds the original value to another register value, and repeats while decrementing an unsigned word counter until it wraps to zero.
- `clear_keyboard_interrupt_state`: The routine checks a condition involving a custom chip register and a global variable.
- `check_interrupt_status_and_acknowledge`: The routine reads the Amiga interrupt enable and request registers, masks the request bits, and if a specific request bit is set it calls another routine, increments a global variable, and writes an acknowledge value to a custom chip register.
- `execute_unnamed_routine`: An executed routine consisting of twenty instructions that calls another routine.
- `update_global_state_and_loop`: Stores values from several registers into global variables, performs arithmetic on a pointer to update a global integer, and enters a loop that repeatedly calls itself until a global value is no longer negative.
- `initialize_global_state`: Stores several values into global variables and enters a loop that calls another routine until a condition is met.
- `initialize_hardware_and_check`: Loads several registers with constants, calls two other routines, and writes three bytes to a hardware register offset.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d1, then returns.
- `tiny_tst_rts`: Small routine of 2 instructions that tests d4, then returns.
- `branch_or_jump`: Decrements a register and branches to another routine or jumps to a long address based on the result.
- `execute_nested_logic_loop`: Performs a series of operations including updating a global variable, calling other routines, and toggling a bit in a global variable within nested loops.
- `branch_and_decrement`: Loads a pointer into a register, restores registers from the stack, decrements a global longword, and performs conditional branching.
- `update_masked_byte`: Determines a value based on two global variables and stores the bitwise complement of that value into a byte at an offset from a pointer.
- `check_button_status`: Calls another routine, performs two multiplications, checks a bit in the CIAA port, and executes a loop based on a register value.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d4, then returns.
- `wait_for_ciaa_bit_clear`: Polls a bit in the CIAA Port A register in a loop and clears a global variable when the loop terminates.
- `update_longword_with_offset`: Tests a longword, calls another routine, and then adds or subtracts a value from the longword based on the result.
- `set_byte_and_branch`: Loads a byte into a register, sets its highest bit, and branches to another routine.
- `write_inverted_bytes`: Writes the bitwise complement of two different values into a table at an offset from a pointer within a loop.
- `store_alternating_bytes`: Stores two different bytes into a table at an offset from a pointer while executing nested loops.
- `loop_and_store_byte`: Stores a byte from a register into a table and executes two nested loops using multiplication and decrementing counters.
- `write_memory_and_wait`: Writes several values to memory offsets from a pointer and enters a loop that waits for a specific bit in a memory location to be set.
- `nop_padding_sequence`: This routine is empty and performs no operations.
- `nop_padding_sequence_2`: This routine is empty and consists of only two instructions that perform no operation.
- `interleave_masked_longword_pairs`: Iterates through two source tables, combining longwords from each by applying a bitwise mask and interleaving them through a bit shift, then stores the resulting longwords into a destination table.
- `combine_masked_longword_pair`: Returns a value built from the two longwords addressed by the pointer, each masked by a register value, with the first shifted left by one bit.
- `xor_sum_table`: Iterates through a table of longwords, performing a bitwise XOR on all values, and returns the result masked by a register.
- `update_hardware_registers`: Calculates values based on registers and a table to update several memory locations, including hardware-related offsets and a bitmask for a port.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
