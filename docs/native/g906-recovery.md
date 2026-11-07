# Game 906 (test drive (1988)(accolade)[cr nightsoft]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00906). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (936416 top-level bytes in 95 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 12 of 12 units identical (bytes and relocations), 9 raw-word lines |
| Functions named and described | pass, 699 of 699 named (0 author symbols), 699 of 699 described (91 by rule, 607 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 7421 of 7428 (99.91%) |
| Recovered without load address (counted in proof 1, flagged) | 110144 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 16658 of 16793 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 237268 |
| code | 56284 |
| data | 588135 |
| padding | 23497 |
| unused | 31232 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| AddBuffers | 2 | 117 | 2 |
| KickRAMTest | 3 | 457 | 30 |
| AddMem | 13 | 320 | 18 |
| EndIF | 1 | 2 | 1 |
| IF | 2 | 117 | 2 |
| Test Drive (1988)(Accolade)[cr Nightsoft].adf:TD | 3 | 8629 | 424 |
| Test Drive (1988)(Accolade)[cr Nightsoft].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 86 | 86 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4661 | 117 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2393 | 17 |
| Road.Shp (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/Vette.ST.Shp: high-entropy data file (7.72 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/VetteDash.Shp: high-entropy data file (7.83 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/VetteLogo: high-entropy data file (7.75 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/P911TLogo: high-entropy data file (7.67 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/LotusGas: high-entropy data file (7.91 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/P911TGas: high-entropy data file (7.92 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/Countach.ST.Shp: high-entropy data file (7.91 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/Lotus.SB: high-entropy data file (7.70 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/P911T.SB: high-entropy data file (7.70 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/P911TDash.Shp: high-entropy data file (7.86 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/RossaLogo: high-entropy data file (7.61 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/RossaGas: high-entropy data file (7.91 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/VetteGas: high-entropy data file (7.91 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/CountachGas: high-entropy data file (7.91 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/Rossa.ST.Shp: high-entropy data file (7.71 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/CountachDash.Shp: high-entropy data file (7.86 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/LotusLogo: high-entropy data file (7.64 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/Lotus.ST.Shp: high-entropy data file (7.80 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/RossaDash.Shp: high-entropy data file (7.84 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/Rossa.SB: high-entropy data file (7.70 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/P911T.ST.Shp: high-entropy data file (7.78 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/Countach.SB: high-entropy data file (7.70 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/Vette.SB: high-entropy data file (7.70 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/CountachLogo: high-entropy data file (7.62 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Cars/LotusDash.Shp: high-entropy data file (7.83 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Pics/Gas.Shp: high-entropy data file (7.69 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Pics/EndGame: high-entropy data file (7.61 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Pics/Endgame.Shp: high-entropy data file (7.75 bits/byte), classified as packed data
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Pics/TitleCar.Shp: high-entropy data file (7.66 bits/byte), classified as packed data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 63 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2055 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2236 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- test drive (1988)(accolade)[cr nightsoft]/Test Drive (1988)(Accolade)[cr Nightsoft].adf:Pics/Road.Shp: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 110144 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### AddBuffers

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_buffer_entries`: Copies a fixed count of four-byte words from a source table into a caller-provided destination buffer, then invokes a routine stored in a global pointer.

### KickRAMTest

- `write_test_character`: Writes the character Z into the memory location passed as an argument, then returns whether that write succeeded.
- `write_and_verify_z`: Writes the letter Z through a pointer and then reads it back to confirm the value was stored.
- `run_memory_self_test`: Performs a self-test by writing a value through a pointer and reading it back to confirm the memory was stored, then invokes a function pointer stored in a global variable when that variable is non-zero.
- `clear_table_and_recurse`: Clears a block of forty words starting at an offset from a base pointer, stores a local stack address and a global variable into subsequent fields, and calls another routine.
- `call_optional_handler`: Invokes a function pointer stored in a global variable when that variable is non-zero, then calls one further routine and returns.
- `run_memory_and_split_text`: Allocates two memory blocks and, if either allocation fails, reports the error.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `run_back_loop_handler`: Runs a short loop calling another routine ten times, then inspects several global variables and, depending on their values, calls other routines with arguments taken from those globals before returning a value.
- `append_string_with_count_2`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when a counter reaches zero, then writes a terminator.
- `append_string_with_count`: Walks to the end of the destination string and appends bytes from the source, stopping at the source terminator or when the count argument runs out, then null-terminates the destination and returns it.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator, and returns the original destination pointer.
- `compute_string_length`: Walks forward byte by byte through a zero-terminated string until it finds the terminator and returns the number of bytes from the start pointer up to but not including that terminator.
- `copy_bytes_zero_padded`: Copies up to a counted number of bytes from the source, stopping early after a zero byte, then fills the rest of the destination up to that count with zeros, returning the destination pointer.
- `invoke_table_entry`: Indexes into a table of entries using a passed value and returns an error indicator when the index is out of range or the entry is empty, otherwise clears the entry, calls the stored routine, and returns success.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.

### AddMem

- `initialize_and_tokenize_input`: Initializes a small data structure with several zero and pointer fields, then calls a routine through a function pointer.
- `save_and_restore_with_flags`: Saves a value into a global variable, sets a field to a constant, calls a routine, and then, if a flag is set, invokes a stored function pointer and another routine before restoring the saved value.
- `call_two_library_functions`: Calls two routines through function pointers held at fixed negative offsets from a register, in turn, and returns.
- `call_and_prepare_buffer`: Calls a function pointer stored in a global variable and stores the returned value into a global variable, returning early if that value is nonzero.
- `call_input`: Library stub: loads its arguments and the dos library base, calls Input and returns its result to the caller.
- `call_output`: Library stub: loads its arguments and the dos library base, calls Output and returns its result to the caller.
- `invoke_memory_checks`: Invokes another routine passing a global variable address and an eight-byte stack slot, then invokes a second routine passing the value ten.
- `init_struct_and_call_routines`: Initializes fields of a structure pointed to by an argument, then calls a routine with no arguments, calls a routine with a computed address and the argument, and calls the no-argument routine once more.
- `accumulate_hex_value`: Walks a null-terminated string one character at a time and accumulates a base-16 value from the digit characters it encounters.
- `parse_hex_string`: ORs a fixed bit pattern into three consecutive bytes of a buffer, then scans that buffer character by character, accumulating a base-16 value from each digit and invoking a routine for every non-digit byte.
- `store_value_and_combine`: Stores a value from a global variable into a fixed global slot, then branches when a parameter equals three.
- `pass_params_to_subroutine`: Invokes another routine passing a value from a global variable, the routine's parameter, and the address of an eight-byte stack slot.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_add_head`: Library stub: loads its arguments and the exec library base, calls AddHead and returns its result to the caller.
- `call_global_and_checksum_buffer`: Invokes a function pointer stored in a global variable, then computes a rolling checksum over a stack buffer until a null terminator is found, and passes the buffer and its inverted checksum to another routine when the result is non-zero.
- `tiny_move_rts`: Small routine of 2 instructions that copies d0 to (a3)+, then returns.
- `call_write`: Library stub: loads its arguments and the dos library base, calls Write and returns its result to the caller.

### EndIF

- `tiny_eor_rts`: Small routine of 2 instructions that exclusive-ors d0 into d0, then returns.

### IF

- `find_max_and_prepare_buffer_2`: Walks a linked list of entries to find the maximum value in a field, then writes that value and a small header block into a buffer.
- `copy_words_then_call`: Copies a fixed number of words from a source address into a destination pointer when that pointer is non-null, then calls a routine through a stored function pointer and returns.

### Test Drive (1988)(Accolade)[cr Nightsoft].adf:TD

- `initialize_globals_and_link_process`: Clears a block of four-word global entries to zero, stores a value from a hardware register and a global variable into fixed global slots, then calls stored routines based on a flag byte and the result of another routine.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `empty_routine_2`: An empty routine that performs no operations and returns immediately.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `dispatch_and_init_system`: Dispatches work through a jump table while initializing timers, audio channels, and input structures, writing results into a global structure.
- `dispatch_via_jump_table`: Reads an offset from a global variable and indexes a jump table in data, then branches to the routine stored at that slot.
- `store_loaded_string`: Loads a string from a file, validates its length, and stores it into a table of four-byte entries while incrementing a counter.
- `skip_string_until_newline`: Advances a character pointer past a string until it reaches a null terminator or a newline, skips the newline if present, and returns the updated pointer.
- `store_bit_and_reset_global`: Stores a masked bit of a global variable into a global word, then overwrites that global variable with a fixed constant before returning.
- `reset_global_counter_if_zero`: Reads a signed word from a fixed global location and, when that value equals zero, overwrites it with the constant 1024.
- `handle_toggle_settings`: Reads a pressed key and toggles various on/off settings depending on which key was pressed, running a repeating action while a particular flag is cleared and restoring that flag when it is set.
- `copy_and_set_flag`: Invokes another routine, then copies a word from a location below the frame pointer into a fixed global word.
- `toggle_flag_by_key`: Reads a character from a frame-pointer offset and toggles a global word between one and zero based on which key was pressed.
- `clear_flag_and_call`: Clears a global word, and when that word becomes zero it invokes another routine pointed to by a fixed global.
- `toggle_flag_by_char_3`: Reads a character from a frame-pointer offset and toggles a global word between one and zero based on which key was pressed.
- `clear_and_invoke`: Clears a fixed global word, then invokes another routine when that word still holds zero.
- `toggle_flag_by_char_2`: Reads a character from a frame-pointer offset and toggles a global word between one and zero, mirroring its negated value into two adjacent bytes when the character is a specific letter.
- `negate_and_store_byte`: Clears a global word, negates its value, and stores the result into two consecutive byte locations.
- `toggle_flag_by_char`: Toggles a fixed global word between one and zero when a byte at a frame-pointer offset equals a specific character and another global word is non-zero, otherwise sets a different global word when that byte equals a second character.
- `clear_single_word`: Clears a single global word and returns without performing any other operation.
- `set_flag_when_byte_matches`: Reads a byte from a fixed global location and, when that byte equals 18, stores the value one at a fixed global word.
- `chain_conditional_calls_2`: Invokes a stored routine with a code pointer, then chains a series of stored code pointers, storing each returned value and continuing only while the previous result is zero.
- `decrement_word_via_pointer`: Reads a signed word through a pointer, decrements it by one, and writes the result back through the same pointer.
- `parse_stream_markers_2`: Invokes a routine through a global pointer passing a parameter, then invokes another routine when two consecutive four-byte values at a source pointer match two specific magic markers.
- `advance_index_using_table`: Reads a word from a table indexed by a value held at a pointer, adds that word and nine to the value, and stores the even result back through the pointer.
- `parse_stream_markers`: Scans a sequence of four-byte markers, matching specific magic values and invoking routines based on which one is found.
- `copy_table_and_unpack_stream`: Copies eighteen words from a fixed source table into a destination buffer, then decodes a packed byte stream into nibble-swapped words up to a capped count.
- `invoke_and_unpack_stream`: Invokes a routine pointed to by a fixed global, saves its return value, then decodes a packed byte stream into words up to a capped count.
- `load_entries_and_invoke_routines`: Loads a number of four-byte entries from a table into stack slots, then repeatedly invokes a routine pointed to by a fixed global for each slot, and finally calls two more routines pointed to by fixed globals.
- `copy_and_loop_entries`: Copies a counted block of four-byte entries from two source lists into stack slots, then invokes another routine once per copied entry.
- `copy_and_loop_routines`: Copies a block of four-byte entries from a source list into a destination buffer while decrementing a counter, then invokes another routine a fixed number of times.
- `loop_routine_calls`: Runs a fixed number of iterations, invoking another routine on each pass while decrementing a counter.
- `blend_buffers_with_masked_value`: Runs a loop that masks and combines words from two buffers using a per-iteration value, advancing the source index and wrapping the loop counter each pass.
- `route_stream_values_3`: Reads four-byte values from a stream and routes each to one of two destinations depending on a control value.
- `decode_stream_values`: Reads four-byte values from a stream, invokes a routine on each, stores the results into a destination buffer, and advances through the stream until it encounters a value outside a valid range.
- `route_stream_values_2`: Reads four-byte values from a source stream, filters them by range, and stores decoded results into a destination buffer, returning a value held in a global register.
- `route_stream_values`: Reads four-byte values from a source, filters them by range, and routes them to one of two destinations based on a control value.
- `invoke_and_initialize_pointer`: Invokes a routine pointed to by a fixed global with an argument, and when the returned pointer is non-null, stores the argument and the value eight at the pointed-to location.
- `clamp_value_and_clear_flag`: When the first argument is non-null and the second argument does not exceed the difference of the pointed-to word and the following word, updates that following word and clears a fixed global word.
- `subtract_two_pointed_words`: subtracts the second word pointed to by the argument from the first word it points to and returns the difference.
- `multiply_words_add_offset`: multiplies the two consecutive words given by the pointer together and adds a fixed offset to the product before returning it.
- `allocate_and_unpack`: Allocates a buffer, copies data into it, and decodes a packed bit stream into consecutive words.
- `empty_c_function`: Compiled C function with no body: sets up a stack frame with link, removes it with unlk and returns without doing any work.
- `return_immediately`: The routine executes five instructions and then returns without modifying any registers, memory, or hardware state.
- `init_blitter_registers`: Initializes blitter control registers to default values and waits for a status bit to be set.
- `store_fixed_word_offset`: writes a fixed word value into a location reached through a base register plus offset.
- `wait_for_status_bit`: increments a counter reached through a base register, then polls a status byte until a bit is set or a small counter overflows, returning the value passed in on timeout and otherwise incrementing a second counter.
- `read_input_register`: reads and returns the current value of the custom input register.
- `store_structure_defaults`: Invokes a referenced routine, then stores fixed and derived word values into several slots of a structure reached through a base register, including a computed value built from shifted register contents.
- `store_structure_limits`: Invokes a referenced routine, then stores fixed and derived word values into several slots of a structure reached through a base register, using a slightly larger constant for one slot than the sibling routine.
- `load_word_from_base`: returns the word stored at an offset from a base register held in an unaffected register.
- `subtract_pointed_from_register`: subtracts a word reached through a base register from a value passed in a data register and returns the difference.
- `copy_words_to_base`: copies a fixed number of words from the source pointer into consecutive locations reached through a base register, advancing both destinations.
- `store_pointer_value`: It writes a value into a memory location pointed to by a pointer in a data structure, then returns.
- `write_custom_register_byte`: It writes a byte from a register through a pointer to a custom hardware register, then performs a couple of additional register-to-register moves before returning.
- `wait_for_status_bit_2`: Waits for a hardware status bit by polling a status byte through a custom register until the bit is set or a counter overflows.
- `setup_blit_descriptor`: Initializes a sprite or blit descriptor by copying source coordinates and dimensions into a destination structure, clamping the rectangle bounds and selecting a transfer mode based on a size check.
- `load_value_from_base`: returns the value found at a location reached by subtracting a fixed offset from a base register.
- `invoke_and_forward`: Invokes a referenced routine and returns, leaving its incoming register values unchanged.
- `store_and_mask_words`: It saves four incoming word values into four consecutive storage slots and then clears the low nibble of the last two values.
- `clamp_and_store_rectangle`: Adjusts the coordinates of a rectangle described by a pointer when its values fall outside a range, clamping them and recomputing the width, height, and position.
- `return_stack_word`: It simply returns the single word value passed to it on the stack without modifying anything.
- `initialize_object_state`: Initializes a block of per-object state by copying position, rotation, and offset fields from a source structure, then computes and stores four coordinate pairs derived from a base value plus a shared offset.
- `adjust_and_align_coordinates`: Adjusts a pair of coordinates stored through a pointer by subtracting a base offset, then aligns them to a grid and indexes a lookup table using the aligned values.
- `return_stack_value`: It returns the value already present in the return slot without modifying anything.
- `update_and_store_rectangle`: Updates a running counter, masks off its low bits, and then uses the masked value to index into a table.
- `store_dispatched_values`: Calls a routine through a value loaded from a fixed offset in a global structure and stores the returned value at two different offsets in that structure.
- `pass_stored_values`: Loads two values from fixed offsets in a global structure and passes each one as an argument to another routine.
- `setup_vehicle_structures`: Initializes two vehicle-like structures by copying shared data, setting their position and size fields, and filling a five-entry array with values returned by a routine called with 8000.
- `invoke_routines_and_set_flag`: It invokes a series of stored routines to process several data blocks and then writes a value into a global variable.
- `chain_dispatched_calls`: Calls a routine with a zero argument and then calls another routine with the returned value and part of an incoming argument.
- `initialize_timer`: Initializes a timer structure with fixed values and pointers, then calls a routine to start it.
- `call_routine_with_args`: It passes a fixed numeric argument and a storage address to another routine and returns.
- `read_and_invert_flag`: It reads a bit from a global variable, inverts it, and returns the resulting single byte value.
- `read_indexed_byte`: It computes a table index from a global variable, reads a byte at that offset, and combines it with an incoming value before returning.
- `increment_and_update_flags`: It increments a counter stored through a pointer and then adjusts several flags based on the new counter value and other stored values.
- `increment_and_branch_counter`: Increments a counter stored through a pointer, then branches on the helper's result to clear a field or, when a stored flag equals one, swaps two stored words and invokes two stored code pointers when the compared values match.
- `chain_conditional_calls`: Chains a series of stored code pointers, storing each returned value into a record and continuing only while the previous result is zero, otherwise returning a failure flag.
- `return_immediately_2`: It performs no operations and returns to its caller immediately.
- `clamp_negated_value_to_limit`: Negates a stored word and returns it when it lies from zero up to twenty thousand four hundred and seventy nine.
- `return_code_pointer`: Performs no operations and returns a stored code pointer to its caller immediately.
- `call_sequential_routines`: Invokes a fixed sequence of stored routines in order and then returns to its caller.
- `store_fixed_offset_global`: Stores a fixed negative constant into a global word and returns the incoming register value.
- `loop_and_dispatch_routines_2`: Runs an initialization sequence, then loops calling helper routines until one returns nonzero.
- `call_stored_code_pointer`: Invokes a stored code pointer located at a fixed offset from a base pointer held in an unaffiliated register, then returns.
- `call_library_vector_228`: Library stub: loads a library base from a global variable and calls the library function at offset minus 228 of that base, returning its result to the caller.
- `store_and_call_routines`: It stores a fixed value into a location, calls a chain of stored routines under certain conditions, and returns the stored value.
- `open_library_and_release`: Stores minus one in a field of a record, then calls a routine through a base pointer and, when it succeeds, invokes two further routines on that base before calling another routine, finally returning the stored value.
- `setup_trackdisk_request`: Writes a request pointer, a fixed length word, and a copied value into a device request record, then calls a stored routine on a base pointer to open the trackdisk.device.
- `store_two_library_results`: Calls two library routines and stores their results in a record and in a second record, one as a long and one as a byte.
- `call_library_vector_336`: Library stub: loads a library base from a global variable and calls the library function at offset minus 336 of that base, returning its result to the caller.
- `call_base_then_other`: Calls a routine through a base pointer held in a register and then calls another routine, returning afterwards.
- `init_and_dispatch_record_2`: Initializes a record with several values copied from other locations and then calls a stored routine through a base pointer.
- `init_and_dispatch_record`: Initializes a record with a flag, a marker value, two copied words, and two computed values derived from incoming words, then calls a stored routine through a base pointer.
- `init_and_dispatch_record_variant`: Initializes a record with a flag, a distinct marker value, two copied words, and two computed values derived from incoming words, then calls a stored routine through a base pointer.
- `populate_record_and_call`: Writes a flag, a marker value, and two copied words into a record, then stores two computed values derived from register inputs before invoking a routine through a pointer.
- `init_and_dispatch_record_direct`: Initializes a record with a flag, a numeric marker value, two copied words, and two directly passed values, then calls a stored routine through a base pointer.
- `init_and_dispatch_record_values`: Initializes a record with a flag, a marker value, two copied words, and two directly passed values, then calls a stored routine through a base pointer.
- `init_and_dispatch_record_fixed`: Initializes a record with a flag, a marker value, two copied words, a passed value, and a fixed one, then calls a stored routine through a base pointer.
- `init_record_and_call`: Initializes a record by writing a flag, a marker value, copied words from another record, and a passed value, then calls a routine through a base pointer.
- `register_disk_resource_handler`: Obtains a value through a library call and stores it in a record, then initializes a second record with a flag, a fixed value, a copied value, a base pointer, and a stored code pointer before returning.
- `loop_helper_routines`: Loops calling two helper routines through pointers stored in a register, repeating until one returns nonzero, then sets two global variables before returning.
- `invoke_disk_resource_routine`: Invokes a routine through a base pointer held in a register and returns.
- `configure_disk_registers`: Writes a fixed value to a custom chip register, sets a global variable, then calls a helper routine through a pointer held in a register.
- `return_immediately_4`: Performs no operations and returns to its caller immediately.
- `update_flag_byte_bits`: Clears the bit selected by the first register value in a state byte, then sets bit two when the second register value is zero.
- `copy_state_byte_to_register`: Copies a state byte into a control word and writes a fixed flag value to a second control word.
- `start_disk_sync_transfer`: Calls a helper routine, waits for a countdown to run out, then writes the DMA control, disk pointer and sync words to the custom chip registers.
- `set_top_bit_field`: Stores the incoming value with its top bit set into a sixteen-bit field of the record addressed by the second register, twice.
- `reset_record_and_call_library`: Sets a state word to a fixed value, sets a second word to two, and calls a library routine on the base pointer, then returns.
- `find_disk_sync_marker_offset`: Decodes a track number from two register-held longs and returns minus one when it does not match the expected value.
- `forward_to_helper_routine`: Calls one helper routine and returns with no other work.
- `compute_bitwise_checksum`: Computes a 16-bit bitwise checksum over a run of words by shifting each bit through an accumulator and exclusive-ORing with a feedback value when the top bit is set.
- `multiply_signed_32_adjust_2`: Multiplies two signed 32-bit values by combining their 16-bit partial products and adjusting the result for the signs of the operands, then stores the 32-bit product through a pointer.
- `multiply_signed_32`: Multiplies two signed 32-bit values using 16-bit partial products and returns the full 64-bit result, correcting the sign based on the input signs.
- `multiply_signed_32_adjust`: Multiplies two signed 32-bit values by combining 16-bit partial products and adjusting the result for the signs of the operands.
- `compute_signed_division`: Divides a signed 32-bit value by a signed 16-bit value, handling negative operands, and packs the quotient and remainder into a 64-bit result before returning it.
- `divide_signed_32_pack`: Divides a signed 32-bit value by a signed 16-bit value and packs the 16-bit quotient and remainder into a 64-bit result, correcting the sign.
- `compute_signed_sqrt`: Takes the absolute value of a 32-bit operand, computes an integer square root via successive averaging, clamps the result, and returns it as a 64-bit value.
- `combine_32_bit_halves`: Reinterprets a 32-bit value as a 64-bit value by combining two 32-bit halves and returning the combined result.
- `empty_routine_3`: The routine contains four 68000 instructions and performs no observable work; it ends immediately.
- `empty_routine_205e`: It contains four 68000 instructions and performs no observable work, ending immediately.
- `convert_int_to_decimal_string`: Converts a signed 32-bit number into a decimal ASCII string by accumulating BCD digits, formatting them, and stripping leading zeros before adding a sign.
- `lookup_character_table`: Transforms a character code, shifting uppercase letters, and indexes into a table to look up a corresponding value.
- `lookup_bounded_table_entry_2`: Maps a signed 16-bit value into a bounded table index using modular arithmetic and returns the 16-bit entry stored at that index.
- `lookup_bounded_table_entry`: Maps a value into a bounded table index using modular arithmetic and returns the 16-bit entry stored at that index.
- `read_indexed_table_word_2`: Reads a 32-bit value from a table indexed by a signed 16-bit value, negating the result when the index is negative.
- `read_indexed_table_word`: Computes a byte offset from the low bits of a data register and reads a 32-bit value from a table indexed by that offset through a pointer.
- `start_audio_timer`: Initializes audio channels and starts a timer when they are not already active.
- `stop_audio_channels`: Stops audio channels and clears the active flag when they are running.
- `update_interrupt_enable`: Writes a masked bit pattern into an interrupt-enable register and, when a flag is set, toggles the interrupt-enable bits to enable or disable interrupts.
- `start_channel_if_valid`: Calls a routine with a pointer and starts a channel when the pointer is not null.
- `init_channel_block`: Initializes a channel control block when given a nonzero identifier, filling its fields with a timer value, a pointer, and several fixed markers.
- `reset_audio_channels`: Repeatedly resets a fixed number of audio channels in a loop, clearing each channel entry in turn.
- `reset_audio_channel`: Resets a channel entry, clears its audio output register, and stores a one-off timer value and a pointer into its control block.
- `check_channel_active`: Checks whether a channel flag is set and returns a fixed value when it is, otherwise returning zero.
- `set_audio_output_level`: Sets the audio output level when a value is valid and, when a second value is valid, records it as the channel's timer and output.
- `store_channel_slot_values`: Stores a 32-bit value into a channel slot and places a shifted value into a second field of the same slot.
- `update_active_audio_channels`: Scans a set of channel slots, and for each active slot with a remaining countdown decrements it, clears its output, and assigns a new value and timer.
- `update_audio_channels`: Scans a set of audio channel slots, and for each active one with a remaining countdown it resets the channel and reprograms the audio output register.
- `advance_channel_timers`: Increments a shared counter, then shifts through a set of channel slots copying and advancing timers for any active entries.
- `init_vbl_record`: Initializes a record by filling its fields with a base pointer, a code pointer, and control bytes, then invokes a routine through the stored code pointer and sets a completion flag.
- `process_pending_channels`: Checks whether a global flag is nonzero and, if so, runs a cleanup routine for each pending entry, invokes a library function, and repeats another routine in a loop.
- `resolve_channel_names`: Resolves a list of entries, copying table values for small indices and calling a name-resolution routine for larger ones, then marks the operation complete.
- `cleanup_pending_channels`: Calls a cleanup routine for each entry that has a pending value, then clears the pending flag.
- `store_scaled_values`: Stores the incoming register values into a block of global memory, then derives and stores scaled multiples of those values into a following table.
- `return_flagged_value`: Reads a flag in global memory and, when it is set, returns a stored word from the same block, otherwise returning zero.
- `run_guarded_subroutine`: Checks two adjacent global flags and, when both are nonzero, calls another routine twice in succession.
- `set_flag_and_loop`: Sets a global flag to a fixed value when it is nonzero, then repeatedly calls another routine until it returns zero.
- `store_comparison_result`: Checks a global flag and, when set, records a comparison result into memory, otherwise leaving the stored value unchanged.
- `check_flag_not_three`: Returns a nonzero marker when a global flag holds a value other than three, and zero otherwise.
- `manage_audio_channels`: Manages a set of audio channels by resetting them when a control flag is cleared, starting or stopping playback based on that flag, and decoding a compressed data stream by walking through it byte by byte while adjusting channel volume and timing.
- `clamp_global_counter`: Clamps a global counter between two bounds, storing the adjusted value back when it falls outside the allowed range.
- `scan_table_for_match`: Walks a table of words using an incrementing index and returns as soon as the current entry matches a compared value.
- `resolve_channel_names_2`: Resolves channel names from a table, formats strings through a stored routine, and invokes a sequence of stored code pointers based on adjacent global flags.
- `process_pending_channels_2`: Waits for a global counter to reach zero, repeatedly invoking a cleanup routine for each pending entry and then a flagged value routine before clearing the counter.
- `clear_table_and_halt`: Clears a word in a table indexed by a running counter, decrements a global counter, and then stops execution.
- `contains_non_code_data`: This routine contains no executable instructions, only data that the decompiler could not interpret as code.
- `halt_baddata`: This routine contains no executable instructions; it holds only data that the decompiler cannot interpret as code.
- `invoke_channel_resolver`: Invokes a stored routine with a global variable and two frame-pointer offsets plus a fixed size value.
- `clear_table_entry_and_decrement`: Clears the two-byte value at the indexed position within a table and then decrements a global variable.
- `describe_empty_routine`: The routine contains no executable instructions, only data that the disassembler misinterprets as code.
- `invoke_channel_setup`: Passes four arguments to another routine: a pointer to a global variable, two computed addresses derived from a base register, and an immediate value of 22.
- `invoke_channel_setup_2`: Passes four arguments to another routine: a pointer to a global variable, two computed offsets derived from a base register, and an immediate value of 22.
- `invoke_channel_resolver_2`: Passes a global variable, two negative offsets, and a length of 22 to another routine that resolves channel names, formats strings, and invokes a stored sequence of code pointers.
- `read_input_and_show_scores`: Reads keyboard input in a loop, repeatedly setting up font rendering and displaying the top score entries until a condition is met.
- `copy_string_until_control`: It copies characters from the source string into the destination until it reaches a byte at or below 31, then skips one trailing character.
- `store_high_score_records`: Copies up to eight high score records from a source block into a destination block, comparing and ordering them while copying the associated name string.
- `process_high_score_records`: Processes a block of eight high score records, shifting their values, searching a string table, and formatting the entries for display.
- `setup_font_render`: Configures a font structure through library calls, then invokes a sequence of routines to set up and render text.
- `call_library_from_pointer`: It loads a pointer from a global variable and passes it to a library stub.
- `chain_two_vector_calls`: It calls a routine through a vector with a single argument and stores the returned value.
- `call_vector_split_arg`: It calls a routine through a vector using a stored pointer and the two halves of a 32-bit argument split into separate parameters.
- `call_vector_low_half`: It calls a routine through a vector using a stored pointer and the low half of a 32-bit argument.
- `restore_pointer_and_call`: It reads a byte from a fixed offset of a stored pointer, then calls several routines through vectors with the values 1 and 0.
- `save_state_and_call`: It reads several bytes and shorts from fixed offsets of a stored pointer, then calls routines through vectors using those saved values.
- `save_state_and_call_offset`: It reads several bytes and shorts from fixed offsets of a stored pointer, then calls routines through vectors using those saved values.
- `read_string_input`: Reads characters from a stream, appends printable ones to a buffer, deletes the last character on a backspace, and terminates on a carriage return.
- `read_and_confirm_driver_name`: Reads a driver's name from the keyboard into a buffer, trimming the last character on backspace and stopping on carriage return.
- `initialize_palette_and_run`: Initializes a set of palette entries and then repeatedly runs a loop of setup routines until a global timer reaches a threshold.
- `display_top_scores`: Formats a list of up to four entries and a final entry into a temporary buffer, then displays the resulting text on screen.
- `format_and_process_string`: It calls a routine to format a string into a stack buffer and then processes it.
- `load_palette_string`: Copies characters from a source string into a destination palette until it reaches a control byte, then skips one trailing character.
- `call_shift_table`: Invokes a single routine that shifts each 16-bit value from a table through a bit-shifting routine controlled by a passed value.
- `shift_table_values`: Shifts each 16-bit value from a table pointed to by a register through a bit-shifting routine controlled by a passed value, applying a conditional XOR on the accumulator.
- `run_sequential_steps`: Repeatedly invokes a sequence of routines, incrementing a counter until it reaches a threshold or a called routine returns nonzero.
- `run_sequential_routines`: Repeatedly invokes a sequence of routines and increments a counter, exiting when the counter reaches a threshold or a called routine returns a nonzero value.
- `store_routine_result`: Invokes a routine and, when it returns a nonzero value, stores one at a global location before returning that stored value.
- `run_sequential_step_loop`: Checks whether a byte pointed to by its argument is nonzero, then performs a series of library calls before running a fixed eight-iteration loop that repeatedly invokes helper routines and clears a global location when a helper reports a nonzero value.
- `build_speed_time_report_2`: Picks a pair of values from one of several tables based on a counter and a range comparison, stores them into a work area, then drives a second routine to build a formatted report.
- `build_speed_time_report`: Selects a pair of words from one of several tables based on a counter and a range comparison, stores them into a work area, then drives a sequence of formatting and rendering routines to build a speed and time report.
- `copy_speed_time_report`: Copies two consecutive words from a table selected by a counter into a work area, choosing between two table offsets based on a comparison, then formats a speed-time report and updates a stored value.
- `store_formatted_speed_time`: Copies two consecutive words from a table selected by a counter into a work area, choosing between two table offsets based on a comparison, then repeatedly invokes a formatting routine to build a report buffer.
- `format_speed_time_report`: Stores two consecutive words from a table indexed by a counter into a work area, then sets up a formatted buffer and repeatedly runs a sequence of formatting routines until a counter reaches a threshold.
- `print_formatted_time_message`: Prints a timed message that reports a value read from a global variable, splitting it into minutes and seconds and formatting each part with its own format string.
- `print_point_messages`: Sends three formatted messages to the console, each built from a value read from a global variable combined with a fixed format string, then reads a system time value and selects one of three stored entries from a table using its remainder modulo three.
- `invoke_channel_setup_3`: Increments a global counter, zeroes a flag, and invokes a sequence of setup routines to configure a channel and copy data between global locations.
- `copy_byte_clear_array`: Copies a byte between two global locations, then clears a 32-entry array of 16-bit values when a global value is nonzero.
- `clear_array_conditionally`: Copies a byte between two global locations and, when a global value is nonzero, clears a 32-entry array of 16-bit values.
- `invoke_setup_routines`: Invokes several routines, passing a dereferenced field from a structure and a computed value to one of them.
- `call_helper_with_pair_three_2`: Calls one helper routine with the incoming argument and a fixed packed pair of values, the second value being three.
- `load_record_with_three`: Invokes a record-loading routine with an incoming pointer and a fixed second argument of three.
- `load_and_link_record`: Reads a record from disk and stores its fields into a freshly allocated node.
- `read_record_field_eight`: Reads and returns the 32-bit value stored at offset eight of the record passed as an argument.
- `init_and_dispatch_record_3`: Initializes several fields of a structure to zero, copies a value from the argument structure, calls a helper routine, and stores the result before dispatching on a flag value to run one of several byte-sequence handling loops.
- `drain_buffer_in_loop`: Reads one byte at a time from a buffer in a loop, discarding each returned byte, and stops once the buffer reports it has no more data.
- `empty_return_routine_8`: An empty routine that immediately returns to its caller without touching any data or registers.
- `loop_two_byte_sequence`: Runs a helper routine once and then repeatedly calls two other routines in a loop until the helper reports the sentinel value, processing a two-byte sequence each iteration.
- `call_and_return`: Invokes another routine and then returns to its caller without performing any other operation.
- `load_paired_value_arrays`: Stores the constant 99 into a global location, reads a value from another routine, and then either clamps it to a sentinel or copies that many paired values from a second routine into two parallel global arrays.
- `read_little_endian_word`: Reads two values from the input stream and combines them into one 16-bit word, with the second value in the high byte.
- `increment_and_shift_buffer`: Increments a counter stored in a global variable and, while it stays below eight, shifts a second global value right by one bit and reloads a lookup entry from a table.
- `handle_two_byte_sequence`: Switches on the value of a global state flag to handle a two-byte sequence, storing the second byte when needed and looping while the first byte equals zero.
- `pop_next_buffer_byte`: Decrements a counter and, while it is nonzero, returns the next byte from a buffer, otherwise returns the all-ones value.
- `push_byte_into_buffer`: Increments a counter and, while it stays below a limit, stores a byte into a buffer, advancing the buffer pointer.
- `expand_packed_buffer_2`: Initializes a sliding-window dictionary by filling a symbol table with byte values and zeroing the associated length table, then repeatedly reads the next byte from a buffer and emits either a literal byte or a back-reference followed by a run length, extending the dictionary as it goes.
- `expand_packed_buffer`: Initializes a sliding-window dictionary by filling a symbol table with byte values and zeroing the associated length table, then repeatedly reads packed characters and expands them through the dictionary into a growing output buffer.
- `expand_byte_sequence_table`: Reads bytes from a source buffer and writes them to a destination, expanding two-byte sequences through a symbol table, and resets the table when a sentinel byte is encountered.
- `process_byte_sequence_table`: Continuously reads bytes from a source buffer, writing them to a destination while advancing pointers, and maintains a symbol table that records byte positions and resets it when a special terminator byte is encountered.
- `dispatch_and_clear_global_2`: Invokes a routine stored in a global variable when that value is nonzero and then clears the global.
- `decompress_packed_buffer`: Initializes bit-decompression counters and a mask, then repeatedly extracts packed characters into a growing buffer until the requested count is filled, and finally adjusts the bit position to the start of the next character.
- `extract_packed_bits`: Initializes a couple of global counters and a bit mask, then repeatedly pulls characters from a bit-packed buffer into a growing byte array until the requested number of bits is collected.
- `extract_bits_from_buffer_2`: First copies a run of bytes from a buffer into a global array until a limit is reached, then extracts a multi-bit value by combining bits spanning adjacent bytes at a tracked bit offset.
- `extract_bits_from_buffer`: Computes a bit offset from a global counter, reads bytes from a buffer at that bit position, and combines bits from two adjacent bytes to produce a shifted result.
- `initialize_global_offsets`: Loads a value from a routine when a global is zero, then stores derived values into three global locations when that value is nonzero.
- `dispatch_and_clear_global`: Invokes a routine with a stored global value when it is nonzero, then clears that global.
- `wait_vertical_blank`: Waits for the vertical blank by polling the video position register until it reads zero, then stores a global value into a nearby short and sets an adjacent short to a flag.
- `decode_compressed_stream`: Decodes a compressed byte stream, copying literal runs verbatim and expanding repeat markers that fill a target buffer, then stores the updated source and destination pointers.
- `clear_high_bit_global`: Clears the top bit of a global byte mapped to the CIA chip, masking off its high signal bit.
- `store_shifted_flag`: Calls a routine that shifts a global byte right by seven bits and inverts the result, then stores the single boolean value into a global variable.
- `store_joystick_handler`: Calls a routine that reads the joystick port and selects a handler from a table using a four-bit index, then stores the returned handler address into a global variable.
- `store_joystick_directions`: Reads the joystick port byte twice, converts each into a four-bit direction index, and stores the resulting values into two global variables.
- `return_shifted_flag`: Shifts a global byte right by seven bits, inverts the result, and returns the single boolean value.
- `select_joystick_direction_entry`: Reads the joystick port byte, builds a four-bit index from bits at each end of the value, and returns the handler address selected from a dispatch table.
- `check_result_set_flag`: Clears a flag, then calls a callback and inspects its result to decide whether to leave the flag set or overwrite it with a nonzero value before returning it.
- `return_threshold_pointer`: Calls a routine that searches a sorted table of threshold values and returns an index-based pointer when a match is found, then passes that pointer straight back as the return value.
- `lookup_threshold_index`: Searches a sorted table of threshold values for a matching entry and returns an index-based pointer when found, otherwise returns zero.
- `build_input_structures`: Builds two input structures by calling a table of routines that allocate and initialize each entry from incoming parameters, then records input values into their growing lists while checking flags and mask bits.
- `release_object_fields_2`: Invokes a routine on several values loaded from fixed offsets in a global structure, then writes the constant ten into a field at a fixed offset and stores a computed pointer into another field.
- `chain_helper_results`: Invokes one helper routine and passes its returned value directly as the argument to a second helper routine.
- `open_library_window`: Initializes a library request structure, calls the graphics library to open a window, and returns the low byte of the resulting window pointer.
- `store_input_value`: Checks a flag in the input structure and, when set, records a value into a growing list while incrementing its entry count.
- `record_input_value`: Checks a flag in an input structure and, when set, verifies a mask bit before recording a value into a growing list and incrementing its entry count.
- `check_global_counter`: Returns one when a global counter holds a nonzero value and zero otherwise.
- `process_list_entries`: Loops while a global counter is nonzero, invoking handlers from a table at each step.
- `process_node_chain_indirect`: Clears a 32-byte stack buffer to zero, then walks a chain of linked nodes following their forward pointers.
- `call_two_node_chain_twice`: Invokes a two-argument helper routine twice in succession, passing the same two arguments each time.
- `process_two_node_chains`: Walks two parallel chains of linked nodes simultaneously, following their forward pointers until either chain ends.
- `do_nothing_return`: It performs no actions, just returns to the caller without reading or writing any data or registers.
- `process_two_node_chains_shift`: Walks two parallel chains of linked nodes simultaneously, following their forward pointers until either chain ends.
- `empty_routine_4`: This routine contains no instructions and simply returns to the caller without performing any work.
- `call_helper_with_pair_one`: Invokes a helper routine with the incoming argument and a fixed packed pair of values, the second value being one.
- `call_helper_with_pair_three`: Invokes a helper routine with the incoming argument and a fixed packed pair of values, the second value being three.
- `allocate_and_link_node`: It allocates a node from a pool and links it into a doubly linked list, storing an index and marker values within the node.
- `manage_dispatched_list_removal`: Handles a sentinel argument by repeatedly recursing through a linked list until a nonzero result appears, otherwise validates a tagged structure and rewires its links before decrementing a global counter.
- `call_raw_put_char`: Library stub: loads its arguments and the exec library base, calls RawPutChar and returns its result to the caller.
- `write_string_characters`: It walks through a zero-terminated string, invoking a character output helper for each byte until the terminator is reached.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `forward_to_formatted_output`: It forwards control to a formatted-output helper routine and returns with no other work.
- `call_raw_do_fmt`: Library stub: loads its arguments and the exec library base, calls RawDoFmt and returns its result to the caller.
- `call_formatted_output_helper`: It calls the formatted-output helper routine and returns with no other work.
- `convert_to_uppercase`: It checks whether an input byte represents a lowercase ASCII letter and, if so, subtracts a constant to convert it to uppercase, otherwise it returns the byte unchanged.
- `convert_to_lowercase`: It checks whether an input byte represents an uppercase ASCII letter and, if so, adds a constant to convert it to lowercase, otherwise it returns the byte unchanged.
- `format_string_output`: Stores the first argument into a global location, clears a nearby byte, then invokes a formatting routine passing a second argument and a stack address to produce output.
- `advance_string_pointer`: It advances or retreats a pointer into a global string buffer by one character, returning the fetched character or a failure indicator when the buffer is empty.
- `decode_format_string`: Parses a text string containing format tags, decoding escape sequences for special characters, numeric values, and bracketed sections while skipping entries flagged in a lookup table.
- `fetch_next_valid_entry`: It repeatedly calls a helper routine, skipping entries flagged in a lookup table, and reports whether a valid entry was found.
- `find_byte_in_string_2`: Scans a zero-terminated string from its start for the first occurrence of a given byte and returns its position, or zero when the termi.
- `search_table_and_call`: Searches a table of string entries for a match, and if found calls another routine with the argument values and the located index, otherwise returns zero.
- `process_string_table`: Walks a table of six-byte entries, comparing each string and calling another routine for each one until a match or end is found.
- `store_param_and_parse_format`: Stores an incoming value into a fixed location in a global structure, then passes that structure and another value to a format-parsing routine.
- `invoke_byte_transfer`: Invokes a byte-oriented transfer routine and then returns to its caller without modifying any further data.
- `traverse_and_modify_linked_structures`: Checks whether the incoming value matches a specific marker, then walks a reversed list of short values looking for a matching entry.
- `find_byte_in_string`: It scans a zero-terminated string from its start for the first occurrence of the given byte and returns its position, or zero when the terminator comes first.
- `clear_globals_and_link_process`: Clears a block of four-word global entries, links a process node, and conditionally invokes stored routines based on a flag byte.
- `empty_routine_5`: The routine contains no instructions and performs no operation.
- `allocate_and_dispatch_buffer`: Allocates a buffer, writes a magic marker into it, and dispatches to routines via several indirect tables based on an incoming index.
- `tiny_move_2`: Small routine of 1 instruction that copies d0 to d5.
- `parse_string_into_words`: Splits a text string into individual words, stripping surrounding whitespace and unescaping doubled quote characters inside quoted sections.
- `copy_string_pad_zeros`: The routine copies characters from a source string to a destination string up to a byte count, stopping early if a null terminator is reached, then pads the remaining bytes of the count with zeros.
- `copy_string_pad_zeros_2`: The routine copies bytes from a source to a destination up to a count or until a null terminator is found, then fills any remaining bytes in the count with zeros.
- `pass_arg_and_call`: Stores its own address into a global variable and then calls another routine with the same argument it received, returning that routine's result unchanged.
- `process_file_table_entries`: Walks a table of six-byte entries, opening and locking files, reading and writing them, and cleaning up locks, returning an index or a negative value on failure.
- `shift_and_set_byte`: The routine shifts a global variable right by one bit, then sets specific bits in a byte pointed to by an address register.
- `process_table_entries`: Reads a value from a table indexed by a computed offset, then walks a list of entries while writing jump instructions into a buffer and decrementing a counter for each non-zero pair.
- `manage_library_resources`: The routine interacts with library structures to check status, execute functions, and update a pointer in a table.
- `copy_string_with_limit`: The routine copies bytes from a source pointer to a destination pointer, stopping at a null terminator or when a counter in the third argument reaches -1.
- `compare_two_byte_strings`: The routine compares two byte strings byte by byte.
- `copy_string_to_buffer_2`: The routine copies bytes from a source pointer to a destination pointer until a zero byte is encountered, including the terminator.
- `multiply_32_bit_values`: The routine takes two 32-bit values from registers and returns a 64-bit product formed from the high and low 16-bit halves of each operand.
- `store_and_dispatch_result`: It calls a routine with a global variable and stores the result at a fixed offset from a base pointer.
- `store_value_then_call`: The routine stores a 32-bit value at a fixed offset from an address register, calls another routine, then writes a zero byte at that offset before returning.
- `store_and_parse_format`: Stores a parameter into a global variable, calls a routine that parses a format string and writes its result to a stack location, then clears a byte at a fixed offset in that global variable.
- `increment_and_store_byte`: It stores a byte from a register through a pointer, then increments the 32-bit value located at that same pointer location by one.
- `parse_format_string_and_copy`: Parses a format string and copies a byte from a table into a destination pointed to by a global pointer.
- `return_concatenated_division_2`: It invokes a division routine and returns a value formed by concatenating the original contents of a register with the quotient left behind by the division.
- `parse_format_directives`: Parses a format string containing percent-sign directives, extracting integer fields with optional precision and flags, and invokes another routine to convert each field into a formatted string.
- `compute_signed_division_2`: Reads two signed inputs, determines the resulting sign from their individual signs, calls a division routine, and negates the result when needed before returning the combined value.
- `compute_signed_division_3`: It computes a signed division of the incoming register values and negates the quotient when the dividend is negative before returning the sign-adjusted result.
- `return_concatenated_division`: Invokes a division routine and returns a value formed by concatenating the original contents of a register with the result left behind by the call.
- `call_recursively`: Calls itself repeatedly without performing any other operation, creating an infinite recursion.
- `compute_32bit_division`: It divides a 32-bit integer by another using a bit-shifting loop for large divisors and direct arithmetic for small ones, returning the quotient and remainder packed together.
- `compute_string_length_2`: It walks forward through a zero-terminated byte string one byte at a time until it reaches the terminator and returns the count of bytes before it.
- `index_and_store_byte`: Loads a byte from a table indexed by a value in a register and stores it to a memory location pointed to by another register.
- `process_byte_transfer`: Performs a byte-oriented data operation that branches on a computed comparison and writes a byte through a pointer, using a helper routine that reads and increments a byte pointer and another that sets a flag bit before comparing.
- `empty_routine_22`: An empty routine that returns immediately without performing any operation.
- `store_incremented_byte_2`: Reads a byte from a location pointed to by a register, increments that pointer, then stores the byte at the destination pointer.
- `store_incremented_byte`: Reads a byte from a location pointed to by a register, increments that pointer, stores the byte at the destination pointer, and returns the stored value.
- `iterate_range_values`: Loops over a range of values, passing each value to a routine before advancing to the next one until the upper bound is reached.
- `process_struct_flags`: Checks a flag byte in a structure and, depending on which bits are set, calls another routine with various computed arguments.
- `compare_and_store_byte`: Sets a flag bit in a structure, then compares two computed values and branches on their equality.
- `find_and_clear_table_entry`: It scans a fixed-size table of four-byte entries, advancing by twenty-six bytes each time, and zeroes the first three fields of the first entry whose fourth byte is a terminator.
- `initialize_structure_fields`: Initializes a structure's fields, setting different values depending on whether a lookup returns zero, and setting extra bits when a second lookup succeeds.
- `clear_dispatch_list_2`: It walks a linked list of pointers, invoking a routine for each entry until reaching a null pointer, then clears the list head and returns zero.
- `clear_dispatch_list`: Walks a linked list of pointers, calling a routine for each entry until reaching a null pointer, then clears a table slot and returns zero.
- `store_self_and_dispatch`: Stores its own address into a global variable at a fixed offset, then calls another routine with a value taken from the caller's stack.
- `forward_parameter_field`: Extracts the first half of a 32-bit parameter and forwards it to another routine before returning.
- `remove_and_call_entry`: Searches a linked list for a matching entry, removes it from the chain, calls a routine with that entry, and returns zero.
- `check_and_query_table_slot`: It validates an index against bounds and a table slot, then calls a routine on the pointed-to value and returns whether that value was non-zero.
- `handle_dos_failure`: It calls a routine on the given argument and, when that call returns zero, reads an error code from the dos library and stores it before returning a failure indicator.
- `write_and_check_dos_result`: It writes a two-byte value to a file using the DOS library, then reads the DOS error code and passes it along with the file handle to a helper that compares the result against a flag and either returns that value or invokes another routine.
- `check_result_then_callback`: Calls a routine with two fixed arguments and inspects a returned bit, then either returns that value directly or invokes another routine when a global byte is nonzero.
- `write_text_then_callback`: Calls a routine to obtain a handle from a fixed address, passes that handle to a routine that outputs text through the dos library, then calls a routine that writes text through the dos library, and finally returns.
- `execute_flagged_routine`: It checks a flag stored at a fixed offset and, when set, invokes the routine pointed to by that flag, then calls another routine before returning.
- `execute_flagged_routines`: Checks several flags and loops through a count to invoke a routine, then calls routines stored in various locations and returns a value from the stack area.
- `tiny_move_unlk_rts`: Small routine of 3 instructions that copies (a7)+ to d4, then removes the stack frame held in a5, then returns.
- `close_and_clear_table_slot`: It validates an index and a table slot, clears a flag bit before closing the referenced resource, zeroes the slot, and returns success.
- `dispatch_indirect_call`: It is an empty routine that only dispatches through an indirect jump table and returns.
- `call_close`: Wrapper that calls the dos library function Close and returns its result to the caller.
- `call_current_dir`: Wrapper that calls the dos library function CurrentDir and returns its result to the caller.
- `call_delay`: Wrapper that calls the dos library function Delay and returns its result to the caller.
- `call_delete_file`: Wrapper that calls the dos library function DeleteFile and returns its result to the caller.
- `call_examine`: Wrapper that calls the dos library function Examine and returns its result to the caller.
- `call_input_2`: Wrapper that calls the dos library function Input and returns its result to the caller.
- `call_io_err`: Wrapper that calls the dos library function IoErr and returns its result to the caller.
- `call_is_interactive`: Wrapper that calls the dos library function IsInteractive and returns its result to the caller.
- `invoke_indexed_routine`: It reads an index from a global variable, uses it to look up a routine pointer in a jump table, and calls that routine.
- `call_lock`: Wrapper that calls the dos library function Lock and returns its result to the caller.
- `invoke_table_routine`: Looks up an entry in a table using an offset stored at a negative displacement from a pointer, then invokes the routine found there and returns its result.
- `call_open`: Wrapper that calls the dos library function Open and returns its result to the caller.
- `call_output_2`: Wrapper that calls the dos library function Output and returns its result to the caller.
- `call_read`: Wrapper that calls the dos library function Read and returns its result to the caller.
- `call_seek`: Wrapper that calls the dos library function Seek and returns its result to the caller.
- `dispatch_indirect_call_2`: Loads a pointer from a table indexed by an offset and jumps to the routine stored there, then returns.
- `call_un_lock`: Wrapper that calls the dos library function UnLock and returns its result to the caller.
- `call_write_2`: Wrapper that calls the dos library function Write and returns its result to the caller.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `jump_indirect`: Reads a pointer stored at an offset from a base address and jumps to the routine located there.
- `call_library_vector_450`: Library stub: loads a library base from a global variable and calls the library function at offset minus 450 of that base, returning its result to the caller.
- `dispatch_via_jumptable_4`: It reads a value from a fixed offset in a global structure and uses it to index into a jump table, then jumps to the routine stored at that entry.
- `call_indirect_table`: Looks up a value in a table using an index derived from a global variable and then calls the routine found at that table entry.
- `call_library_vector_630`: Library stub: loads a library base from a global variable and calls the library function at offset minus 630 of that base, returning its result to the caller.
- `allocate_and_init_entry`: It looks up a pointer in a table and, when available, initializes a structure's fields with the incoming parameters before storing a result and invoking a routine on the created entry.
- `free_display_object`: It invokes a routine on the referenced object when a field is nonzero, then clears specific fields by setting a byte to 255 and words to a fixed address before releasing associated resources.
- `call_library_vector_354`: Library stub: loads a library base from a global variable and calls the library function at offset minus 354 of that base, returning its result to the caller.
- `call_library_vector_330`: Library stub: loads a library base from a global variable and calls the library function at offset minus 330 of that base, returning its result to the caller.
- `create_display_entry`: It calls another routine passing the input parameter and a fixed constant value of 48.
- `forward_pointer`: Passes its input pointer directly to another routine and returns.
- `init_struct_from_call`: Returns zero when the first argument is zero, otherwise calls a routine from a table and, if it returns nonzero, stores a byte, a word and a long at fixed offsets within the returned structure, then returns that pointer.
- `release_object_fields`: When given a nonzero pointer it clears specific structure fields and invokes a dispatched routine with the object and a stored value, otherwise it returns zero.
- `invoke_indirect_table_5`: It reads a pointer from a table indexed by a value stored in a global variable, then jumps to the routine located at that pointer.
- `invoke_indirect_table`: Reads a pointer from a table indexed by a value stored in a global variable, then jumps to the routine located at that pointer.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_456`: Library stub: loads a library base from a global variable and calls the library function at offset minus 456 of that base, returning its result to the caller.
- `dispatch_via_jumptable_5`: It looks up a pointer in a table indexed by a value and jumps to the routine stored there.
- `dispatch_via_jumptable`: Looks up a pointer in a table indexed by a value and jumps to the routine stored there.
- `dispatch_table_entry_2`: It looks up an entry in a table indexed by a value stored at a fixed global variable and jumps to the routine found there.
- `dispatch_table_entry`: Looks up an entry in a table indexed by a value stored at a fixed global variable and jumps to the routine found there.
- `invoke_dispatched_routine`: Looks up an entry in a dispatch table using an index stored in a global variable, then jumps to the routine located at that table slot.
- `dispatch_table_routine`: It looks up an entry in a table indexed by a value stored at a fixed offset from a base pointer, then jumps to the routine found there.
- `dispatch_via_jumptable_2`: Looks up an entry in a jump table using a value stored at a fixed offset from a base pointer, then jumps to the routine at that table slot.
- `invoke_jumptable_slot`: Looks up an entry in a jump table using a value stored at a fixed offset from a base pointer, then calls the routine at that table slot.
- `setup_pointer_chain`: Writes its own address into the location it points to, increments that stored value by four, zeroes the following word, and stores the original pointer address into the word after that.
- `call_library_vector_444`: Library stub: loads a library base from a global variable and calls the library function at offset minus 444 of that base, returning its result to the caller.
- `dispatch_library_call_2`: It loads a value from a fixed offset in a global structure and uses it to index into a jump table, then performs an indirect call to the selected routine.
- `dispatch_library_call`: Loads a value from a fixed offset in a global structure and uses it to index into a jump table, then performs an indirect call to the selected routine entry.
- `call_library_vector_138`: Library stub: loads a library base from a global variable and calls the library function at offset minus 138 of that base, returning its result to the caller.
- `call_library_vector_174`: Library stub: loads a library base from a global variable and calls the library function at offset minus 174 of that base, returning its result to the caller.
- `call_library_vector_360`: Library stub: loads a library base from a global variable and calls the library function at offset minus 360 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `invoke_indirect_table_2`: It reads a pointer from a table indexed by a value stored in a global variable, then jumps to the routine located at that address.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `invoke_indirect_table_3`: Looks up a pointer in a table indexed by a value stored at a fixed offset from a base register, then jumps to the routine found there.
- `invoke_indirect_routine`: Reads a pointer from a table indexed by an offset stored at a fixed distance from a register, then jumps to the routine located there.
- `invoke_indirect_table_4`: Looks up a pointer in a table indexed by a fixed offset and invokes the routine stored there.
- `call_library_vector_366`: Library stub: loads a library base from a global variable and calls the library function at offset minus 366 of that base, returning its result to the caller.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_routine_from_table`: Looks up a routine pointer in a jump table and calls it.
- `dispatch_library_request`: Looks up a pointer in a table indexed by a value and jumps to the routine stored there.
- `call_library_vector_462`: Library stub: loads a library base from a global variable and calls the library function at offset minus 462 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_576`: Library stub: loads a library base from a global variable and calls the library function at offset minus 576 of that base, returning its result to the caller.
- `call_library_vector_564`: Library stub: loads a library base from a global variable and calls the library function at offset minus 564 of that base, returning its result to the caller.
- `call_library_vector_540`: Library stub: loads a library base from a global variable and calls the library function at offset minus 540 of that base, returning its result to the caller.
- `call_library_vector_570`: Library stub: loads a library base from a global variable and calls the library function at offset minus 570 of that base, returning its result to the caller.
- `call_library_vector_390`: Library stub: loads a library base from a global variable and calls the library function at offset minus 390 of that base, returning its result to the caller.
- `dispatch_table_caller`: Looks up a pointer in a table indexed by a value and jumps to the routine stored there.
- `invoke_jumptable_routine`: Looks up a routine address in a jump table indexed by a value stored at a global variable and jumps to it.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_192`: Library stub: loads a library base from a global variable and calls the library function at offset minus 192 of that base, returning its result to the caller.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `dispatch_via_jumptable_3`: It loads an offset from a global variable and uses it to index into a jump table stored in data, then jumps to the routine found there.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `jump_from_table_index`: It reads an offset from a global variable and uses it to index into a jump table in data, then branches to the routine located at that entry.
- `call_table_entry`: It fetches an offset from a global variable and applies it to index a jump table in data, then calls the routine stored at that slot.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_348`: Library stub: loads a library base from a global variable and calls the library function at offset minus 348 of that base, returning its result to the caller.
- `route_to_routine`: It takes an offset stored in a global variable and uses it to locate an entry in a jump table, then transfers control to the routine at that address.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `indirect_dispatch`: It reads an offset from a global variable and indexes a jump table in data, then jumps to the routine found at that entry.
- `select_routine_index`: It loads an offset from a global variable and uses it to index a jump table in data, then branches to the routine stored there.
- `lookup_and_call`: It fetches an offset from a global variable and applies it to index a jump table in data, then calls the routine located at that slot.
- `call_library_vector_270`: Library stub: loads a library base from a global variable and calls the library function at offset minus 270 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `dispatch_offset_index`: It reads an offset from a different global variable and uses it to index a jump table in data, then jumps to the routine found at that entry.
- `call_indexed_routine`: It loads an offset from a global variable and indexes a jump table in data, then branches to the routine stored at that location.
- `route_via_offset`: It takes an offset from a global variable and uses it to index a jump table in data, then transfers control to the routine at that slot.
- `call_library_vector_114`: Library stub: loads a library base from a global variable and calls the library function at offset minus 114 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.

### Test Drive (1988)(Accolade)[cr Nightsoft].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_routine_6`: The routine contains no instructions and performs no operations; it is an empty routine that returns immediately.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `return_zero_value`: This routine returns a zero value in its result register without performing any operations.
- `return_zero_empty`: This routine is empty and returns a zero value without performing any operations.
- `combine_modulo_parts`: This routine takes a value and a modulus, then combines the remainder and quotient of the division into a single result by shifting the remainder and OR-ing it with the quotient.
- `noop_routine`: This routine is empty; it performs no operations and simply returns.
- `pack_divmod_result`: Splits a 32-bit value into its high and low 16-bit halves and packs them back together.
- `swap_word_halves`: Splits a 32-bit value into its high and low 16-bit halves and packs them back together, effectively swapping the two halves.
- `split_16bit_quotient_remainder`: Splits the 32-bit value in D1 into two 16-bit halves: the low word becomes the remainder of D1 divided by the 16-bit value in D6, and the high word becomes the quotient.
- `split_pack_16bit`: Splits a 32-bit value into its high and low 16-bit halves using a 16-bit divisor and packs them back together.
- `empty_routine_7`: This routine is empty; it contains no instructions and simply returns without performing any operation.
- `return_empty_routine`: This routine contains no instructions and simply returns without performing any operation.
- `empty_return_routine`: It is an empty routine that returns without doing anything.
- `empty_return_routine_2`: An empty routine that returns immediately without touching any data, registers, or hardware.
- `swap_remainder_quotient`: It takes a 32-bit input and a modulus value, then combines the remainder and quotient of their division into a single 32-bit result by placing the remainder in the upper half and the quotient in the lower half.
- `empty_returner`: An empty routine that returns immediately without performing any operations.
- `empty_noop_routine`: It is an empty routine that returns without touching any data or registers.
- `split_32bit_into_words`: Splits a 32-bit value into its two 16-bit halves, placing the low word and the high word into the result.
- `swap_16bit_halves`: Splits a 32-bit value into its high and low 16-bit halves and recombines them in reversed order.
- `swap_half_words`: Splits a 32-bit value into its high and low 16-bit halves and packs them back together, effectively swapping the two halves of the value.
- `swap_half_words_2`: Splits a 32-bit value into its high and low 16-bit halves and recombines them in reversed order, returning the swapped result.
- `swap_16bit_halves_2`: It takes a value from a register and a value from another register, then swaps their high and low 16-bit halves and returns the result.
- `return_zero_routine`: It is a one-instruction routine that does nothing and returns zero.
- `empty_return_routine_3`: It is an empty routine that returns without performing any operation.
- `split_value_into_quotient_and_remainder`: Takes a 32-bit value and two operands, then splits the dividend by the divisor into its remainder and quotient.
- `empty_routine_8`: It is an empty routine that decodes as a single instruction and does nothing.
- `empty_routine_9`: The routine contains no executable instructions; it is an empty routine with no code to analyze.
- `empty_data_gap`: This routine contains no instructions; it is an empty gap that decodes as code.
- `return_nothing`: It returns without performing any operation.
- `mark_empty_gap`: Contains no instructions; it is an empty routine occupying a gap that the disassembler misinterprets as code.
- `mark_empty_routine`: The routine contains no executable instructions; it is an empty routine with no code to describe.
- `return_zero_value_2`: This routine contains a single instruction that returns a constant zero value to its caller.
- `describe_bad_data`: This routine consists of a single instruction that decodes as invalid 68000 code, so the surrounding data is not genuine executable code.
- `empty_noop_routine_2`: This routine is empty; it performs no operations and returns without touching any data or registers.
- `empty_noop_routine_3`: An empty routine that returns without doing anything.
- `empty_routine_10`: The routine contains no executable instructions; it is a completely empty routine that does nothing.
- `empty_routine_11`: The routine contains no executable instructions; it is an empty routine that performs no operations.
- `empty_routine_12`: An empty routine that returns without performing any operations.
- `mark_empty_routine_2`: The routine contains no executable instructions; it is empty.
- `empty_code_gap`: This routine contains no instructions; it is an empty gap that Ghidra decodes as code and returns a constant zero.
- `empty_return_routine_4`: An empty routine that returns immediately without reading or writing any data, calling anything, or touching hardware.
- `empty_routine_13`: This routine is empty and contains no instructions to analyze.
- `empty_routine_14`: The routine contains no executable instructions; it is empty.
- `return_zero_value_3`: This routine returns a constant zero value and then exits.
- `empty_routine_15`: This routine contains no instructions and performs no operations on any data or registers.
- `empty_return_routine_5`: An empty routine that immediately returns without executing any instructions or touching any data.
- `empty_routine_16`: This routine contains no instructions and performs no operations.
- `noop_routine_2`: It contains no instructions and performs no operations on any data.
- `noop_empty_routine`: It contains no instructions and returns immediately without touching any data or registers.
- `empty_routine_17`: This routine contains no instructions and performs no operations.
- `do_nothing`: It contains no instructions and returns without doing anything.
- `halt_baddata_2`: The routine contains no valid instructions; the bytes at this location are data rather than executable code, so nothing is executed or modified.
- `return_zero`: Returns a fixed value of zero to its caller.
- `empty_return_routine_6`: It performs no operations and returns immediately without touching memory, registers, or hardware.
- `report_empty_routine`: The routine contains no executable instructions; the referenced bytes are non-code data rather than a callable body.
- `noop_routine_3`: This routine performs no operations and returns immediately without touching any data or registers.
- `empty_routine_18`: This routine contains no instructions and performs no operations.
- `misread_gap_data`: This routine is not real code; it is uninitialized data in the runtime segment that the disassembler misread as instructions.
- `noop_routine_4`: This routine is empty; it performs no operations and returns immediately.
- `do_nothing_2`: It contains no instructions and returns without performing any operation.
- `empty_routine_19`: The routine contains no executable instructions; the referenced segment holds only non-code data rather than any operations.
- `do_nothing_routine`: This routine is empty and contains no instructions.
- `noop_routine_5`: It performs no operations and returns immediately without touching any data or registers.
- `mark_empty_gap_2`: The routine contains no instructions; it is an empty gap that decodes as code.
- `store_table_byte`: Loads a byte from a table indexed by a value in a register and stores the result into a global variable.
- `empty_routine_20`: The routine contains no executable instructions; the referenced memory holds non-code data.
- `load_byte_and_return`: A single instruction that loads a byte from a global variable into a register and then returns.
- `do_nothing_3`: It performs no operations and returns without doing anything.
- `empty_return_routine_7`: An empty routine that returns immediately without executing any instructions or touching any data.
- `empty_return`: An empty routine that returns without executing any instructions or touching any data.
- `execute_bad_instruction`: Executes a single instruction that decodes to invalid 68000 opcode data, then returns.
- `decode_bad_instruction`: Consists of a single instruction that the decompiler cannot decode as valid code, so it stops analysis at that point.
- `return_zero_value_4`: Contains one instruction that returns a fixed value of zero to its caller.
- `halt_baddata_3`: The routine contains no valid instructions; it is entirely made up of bad instruction data.
- `empty_routine_21`: The routine contains no executable instructions; it is a completely empty routine that performs no operations.
- `misaligned_data_gap`: This routine contains no real instructions; the bytes here are data that Ghidra misinterprets as code.
- `noop_routine_6`: The routine contains no instructions and returns immediately without touching any data or registers.
- `empty_noop_routine_4`: It performs no operations and returns without touching any data or registers.
- `empty_return_stub`: This routine contains no instructions and performs no operations.
- `decode_gap_data`: This routine is not real code; it is uninitialized data in the runtime segment that the disassembler mistakenly decodes as instructions.
- `empty_data_word`: The routine contains no executable instructions, only a single data word, so it performs no operations on any data or registers.
- `call_routines_via_table`: It invokes a routine pointed to by a global table, passing a fixed value, a decremented value from another global, and a short read from a global, then invokes a second routine pointed to by the same table.
- `return_immediately_3`: It performs no operations and returns immediately to its caller.
- `reset_and_init_object`: Clears a block of fields in a per-object structure and, when a flag is clear, loads a data record by index, copies a buffer, and runs a series of setup and cleanup routines that populate and reset several fields.
- `resolve_and_copy_identifiers`: Looks up four-character identifiers in a string table and stores the matching pointers.
- `store_routine_results`: Calls a routine pointed to by an indirect table, passing the address of a global variable, and stores each returned value into four consecutive four-byte global locations.
- `do_nothing_4`: The routine contains no instructions and performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_and_run_state_machine`: Resets a large block of global state to zero, stores initial values into several fields, then repeatedly runs a table of setup and update routines while tracking a counter and a completion flag.
- `clear_memory_blocks`: It zeroes out a large contiguous block of memory by writing zero to every word in a 512-word table, then clears five four-byte slots, four two-byte slots, and one more four-byte slot in a second data structure.
- `update_table_bounds`: Clamps two stored coordinate values toward each other within small bounds, then either resets a flag when they match a stored target or writes four neighbor entries into a table and records the new position.
- `process_endgame_sequence`: Computes a value from a routine call, stores it in a global variable, and then, when that variable is non-zero, drives a sequence of sound-table updates and pointer-based calculations that scale table entries and store results into related slots.
- `set_flag_bit`: Validates two index parameters against table bounds, then sets one specific bit within a byte selected from a computed table entry.
- `clear_flag_bit`: Validates two index parameters against table bounds, then clears one specific bit within a byte selected from a computed table entry.
- `multiply_signed_value`: Multiplies a signed value by a table entry, returning the negated product when the value is negative.
- `clamp_value_range`: Clamps a value into a range by storing the smaller of two inputs when below a stored value and the larger when above it.
- `adjust_stored_value`: Adjusts a stored value downward by a fixed offset when it exceeds the stored value or a shifted comparison, provided a related byte holds a sentinel.
- `initialize_and_invoke`: Writes a fixed constant into a global location, invokes a routine pointed to by an indirect table, and copies one value from another global location.
- `clamp_and_indirect_call`: Clamps a value from a structure to the range -10 to 10, then invokes a routine through a pointer stored in a global variable.
- `loop_dealership_prompts`: Waits for a structure field to become zero, then invokes a routine through a pointer and performs a sequence of calls that reference on-screen text such as pulling into the dealership and pulling into the gas station.
- `compute_capped_result`: Subtracts two parameters, caps the result at a fixed maximum, calls two routines, and applies a correction to the second result when the capped value differs from the original.
- `invoke_when_below`: Compares two stored values and, when the first is smaller, invokes a routine pointed to by an indirect table several times in sequence.
- `clear_global_word`: Writes a zero word to a fixed offset of a global variable pointed to by register A4.
- `call_indirect_routines_4`: Performs a series of indirect calls through a routine pointer stored in a table, then returns the two incoming data registers unchanged.
- `init_globals_and_call_indirect`: Stores the value 20000 into three adjacent global words, zeroes two other globals, adds a global word to a fourth global, subtracts a routine's return value from a constant, invokes a routine pointed to by an indirect table, and writes several fixed values into more globals.
- `scale_and_clamp_globals`: Sets two adjacent global words to their maximum value, derives a divisor from another global word, clamps it to at least one, stores it, and decrements one of the words by a scaled amount.
- `setup_globals_with_shifted_value`: Stores the value 20000 into three adjacent global words, zeroes two globals, sets a third global, computes a shifted scaled difference of a global word into a fourth global, invokes a couple of routines, and writes a fixed value into another global.
- `init_tables_and_call`: Initializes two tables with repeated word values, invokes a routine stored in a global pointer, and copies one global word to another before returning.
- `check_globals_and_set_flag`: Compares signed and unsigned global values against fixed thresholds and sets a global flag word to one when the comparisons fall within the checked ranges.
- `dispatch_by_status_byte`: Reads a small byte from a global table and, based on its value, performs one of several actions: it may compare two status bytes and set a flag when one is not greater, it may copy a value into a run of table slots and update an adjacent counter, it may set timer fields for two separate groups, or it may compute a clamped absolute difference and store a byte when it falls in a range.
- `adjust_global_byte`: Reads a byte from a pointer, adds a scaled version of it to a global word, then repeatedly adjusts that word until it falls within a fixed range before returning a shifted value.
- `add_byte_to_word`: It adds a sign-extended byte to a word stored in a global structure, then calls another routine and returns the original input values.
- `scale_globals_and_clear`: Scales two adjacent global words by a fixed multiplier and clears a nearby global word to zero.
- `clamp_and_multiply`: It keeps the larger of two stored words, multiplies a value by a table entry, then subtracts the resulting product from the stored value.
- `scale_and_accumulate`: It optionally scales a value by a stored factor and shifts it, then multiplies a sign-extended byte by a computed term and adds the result to a stored word.
- `update_race_position_counters`: Walks a fixed-size table of entries, comparing each entry's value against a running offset and a small range, and adjusts the offset and a shared position counter when the values are close enough.
- `decrement_and_clamp_counter`: Compares a stored counter against a reference value held in a register and returns early when they differ.
- `increment_and_clamp_counter`: Compares a stored counter against a reference value held in a register and returns early when they differ.
- `clamp_and_adjust_counter`: Compares a stored counter against a reference value and returns early if they differ.
- `clamp_and_set_global`: Returns early when a stored counter differs from a reference value.
- `compute_clamped_delta_2`: Compares a stored counter against a reference value and returns early if they differ.
- `compute_clamped_delta`: Compares a stored counter against a reference value and returns early if they differ.
- `scan_and_patch_table`: Saves a stored word, then scans a table comparing its entries against an incoming value.
- `conditionally_invoke_routine`: Checks whether the low nibble of an incoming value is zero and a stored counter exceeds a threshold, and if so calls another routine before returning.
- `conditionally_invoke_routine_2`: Tests whether a byte in a table indexed by an incoming value is non-zero and a second incoming value is below a limit, and if so calls another routine before returning.
- `conditionally_invoke_routine_3`: It checks whether a byte in a global table is non-zero and whether a value in a register is below 122, and if both hold, it calls another routine.
- `conditionally_invoke_nested_routine`: It checks whether a byte stored at a slot of a 64-entry table is non-zero and whether a value in D1 is below 122; if both hold, it calls another routine.
- `forward_routine_result`: Invokes another routine and returns its result unchanged.
- `initialize_and_loop_globals`: Initializes two global words to zero, then repeats a fixed number of times depending on a flag, each iteration invoking several helper routines, copying values between global words, computing scaled products, and tracking bounds.
- `compare_globals_and_increment_loop`: Sets a global word to a filled pattern, zeroes a global, then branches on the sign and equality of two globals to store a computed difference into a global and invoke routines, otherwise repeatedly increments a global while adjusting a stored value and invoking routines, and finally copies one global onto another.
- `copy_table_coordinates`: Copies two coordinate pairs from a source table into a working area, choosing the pair based on a byte index and preserving the previous values when a flag is zero.
- `conditionally_invoke_routines`: Reads a signed 16-bit value from a global variable and, when it is greater than 29, invokes two other routines in sequence through pointers stored in that same global variable.
- `toggle_flag_bits`: Clears a stored word, then when a stored value exceeds a threshold and a specific bit of an incoming value is set it writes a constant, calls two bit-modifying routines, and clears the constant again.
- `check_flag_and_set_state`: Returns early when a stored counter is below a fixed threshold, then branches on a flag bit in a register.
- `decrement_and_maybe_invoke`: Decrements a stored doubleword by a fixed amount and zeroes it when the original value was below that amount.
- `set_flag_when_value_in_range`: Takes the absolute value of the difference between two registers and sets a flag in a global variable when that value falls inside a fixed range.
- `save_and_call_routines`: Copies one stored value into another, saves a stored word, invokes two stored routines, stores the saved word, and returns two incoming values unchanged.
- `call_stored_routines_four_times`: Calls two stored callables in a loop four times, discarding their results.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 20 into (a1), then merges 38 into -(a3).
- `write_buffer_and_loop`: Stores a fixed value into a global word, and when several conditions hold it writes several computed values into consecutive slots of a buffer.
- `subtract_and_clamp_values`: Subtracts a scaled version of an incoming value from three stored words and three stored doublewords, then calls another routine three times to clamp each value into range before returning the original value.
- `scale_and_update_four_ints`: Adds a scaled version of the passed integer to four consecutive global integers, then subtracts half and a quarter of that scaled value from the same four integers, adds fixed constants to each, and finally truncates each of the four integers to their low 16 bits.
- `clamp_value_between_bounds`: Clamps a passed integer into the range between negative four hundred and four hundred by repeatedly adding or subtracting eight hundred.
- `initialize_word_block_constants`: Initialises a block of six consecutive word-sized fields in a global variable with a fixed set of constants.
- `clamp_difference_between_bounds`: Clamps the difference between two passed integers into the range between negative one hundred eighty and one hundred eighty by repeatedly adding or subtracting two hundred sixty-four.
- `invoke_stored_routines`: Invokes four stored routines in an alternating sequence, then calls two more stored routines in turn, and finally returns two incoming values unchanged.
- `update_counter_and_setup_timer`: Decrements a counter in a global variable by a scaled value, then sets up a timer field and copies a parameter into a target slot when several conditions hold.
- `compare_values_and_update_counter`: Compares two stored values, calls two stored callables, and updates a counter and its source depending on which range the difference falls into.
- `adjust_and_swap_counter`: Computes a shifted and sign-adjusted value from a stored callable, compares it against a stored counter, invokes another stored callable when that counter holds a small non-two value, records the result, and then swaps two adjacent global integers.
- `call_routines_when_not_equal_two`: Calls two stored callables whenever a passed integer is not equal to two, otherwise skips them.
- `store_or_copy_short`: It writes the constant 65036 to two adjacent short values in a global structure when a passed short equals zero, otherwise it copies one short onto the next.
- `return_data_registers`: It returns the two data registers unchanged, packed into a single 64-bit value, without touching any memory.
- `update_and_clamp_short`: It increments a global short while clamping negatives to zero and upper-bounding it by another short, then derives two more shorts from a table and stores them.
- `index_and_store_shorts`: It loads a byte from a table using a passed index, computes two shorts from another table, and stores a masked index plus a shifted value into global shorts.
- `call_stored_functions`: It calls two different stored function pointers in a fixed interleaved order, passing a split parameter and a constant offset each time.
- `combine_shifted_shorts`: It combines a shifted low short with a high short multiplied by 256 into a single 32-bit result.
- `pack_bits_into_short`: It packs several bits from two passed values into the low byte and stores one passed short into the high byte of the result.
- `pack_and_deliver_results`: Combines two passed shorts into a 32-bit value, packs bits from the parameters into a result, and delivers each result to a stored callable through two consecutive hardware data registers.
- `store_table_results`: Subtracts a stored value from a passed integer, then stores two computed 16-bit results into consecutive slots of a table indexed by a second parameter.
- `update_table_slot`: Invokes a stored routine, then computes a value from a passed integer and a table slot and stores the sum of that value and the existing slot contents into the table.
- `copy_selected_shorts`: It copies shorts from a source table into a destination table, skipping entries selected by a bitmask and masking values when a second table is present.
- `accumulate_selected_shorts`: Invokes itself with an incremented source offset and a computed destination index, then adds a scaled source value to the short currently stored at that destination slot.
- `store_and_copy_table_results`: Clears a forty-nine-entry table of longs, then, when the first argument indexes a valid slot, subtracts stored byte and short values from the arguments and passes the results to two other routines.
- `offset_and_forward_pair`: Adds fixed offsets to two halves of a passed 32-bit value and forwards the result along with a constant to another routine.
- `initialize_sound_tables`: Initializes a set of global variables to zero, fills a table with a linearly increasing sequence, and copies values between related tables.
- `flush_pending_calls`: It invokes a stored function, sets a global to a constant, then calls another stored function for each non-zero global pointer before clearing those pointers.
- `combine_flags_and_forward`: Combines several stored words with fixed offsets and forwards the results to another routine, then branches on a set of stored flags to forward additional computed values.
- `invoke_pointers_and_loop`: Invokes two stored function pointers with fixed arguments, then loops eight times calling another routine with a growing base address and a per-iteration stored value.
- `loop_and_call_stored`: It calls a stored function with a decremented parameter, then loops calling another stored function with successive addresses from a table, and finally calls the first function with fixed constants.
- `initialize_and_dispatch_routines`: Initializes a routine table by storing two swapped bytes and invoking several stored routines, then repeatedly loads paired values from a table and dispatches them to two stored routines for each of several iterations.
- `invoke_stored_routines_2`: Invokes four stored routines in an alternating sequence, then calls two more stored routines in turn, and finally returns two incoming values.
- `clamp_value_to_maximum`: Clamps one passed value to a maximum of another passed value and returns the resulting value.
- `load_word_from_global_pointer`: Reads a 16-bit value from a memory location obtained by adding an offset to a pointer stored in a global variable, then returns that value to the caller.
- `clamp_value_between_bounds_2`: Clamps a passed value between two stored bounds, storing the clamped result back into a global variable and then filling several fields of a structure with adjusted values and looked-up table entries.
- `track_value_bounds_and_step`: Tracks the lowest and highest values seen in a running sequence, updating stored minimums and maximums.
- `find_min_and_shift_table`: Walks a table of words, tracking the smallest value and its position, then shifts that value down and calls another routine for each entry that falls below the shifted amount.
- `set_flag_and_call_routines`: Checks a stored word and a flag bit, and when either is set, writes a constant to a global variable, invokes two other routines, and calls a fourth routine.
- `align_and_reduce_stored_value`: Aligns a stored value up to a multiple of sixteen, reduces another stored value toward it, and if a positive gap remains invokes several other routines before swapping two stored values.
- `toggle_byte_when_mismatch`: Compares two adjacent bytes addressed through a pointer derived from a stored value, inverts the first byte when they differ, and then calls two other routines.
- `push_values_into_tables`: Pushes two incoming values into indexed slots of three separate tables, using a stored counter as an index that it then increments, provided the counter stays below a limit.
- `loop_and_dispatch_routines`: Loops a decremented counter, indexing into a table with a shifted value and invoking stored routines based on the compared and masked values, then restores the original values.
- `copy_value_and_call_routines`: Copies a value from one global variable into another, then invokes two other routines before returning.
- `call_indirect_routines`: It calls two routines located at computed addresses derived from a base register and then returns.
- `call_indirect_routines_2`: It calls two routines through addresses computed from a base register and then returns.
- `call_indirect_routines_3`: It calls two routines indirectly through addresses computed from a base register, using offsets -32658 and -32610.
- `copy_global_value`: It loads a value from a global variable, stores it into another global variable, and returns without changing anything else.
- `copy_global_value_2`: The routine loads a value from a global variable into a register, stores that value into another global variable, and then returns.
- `adjust_and_store_value`: Copies a stored word into a global variable, adjusts it with a multiply and divide, stores the result back, invokes two other routines, and increments a byte when a certain offset is reached.
- `dispatch_pointers_conditional`: Checks a stored word, calls another routine, then dispatches to two stored function pointers in alternating order with and without a passed argument.
- `copy_global_value_3`: Loads a value from one global variable and stores it into another global variable, then returns without modifying anything else.
- `multiply_shifted_high_byte`: Reads a doubleword, shifts its high byte, multiplies it by a stored word, and stores the product back into that doubleword.
- `divide_shifted_doubleword`: Treats a stored word as a divisor, defaulting it to one when zero, then stores the shifted-and-divided result of a doubleword back into that doubleword.
- `update_values_and_copy`: Tests a stored word and, depending on several other values, halves a quantity, swaps values, and stores a constant, then copies two words between global variables.
- `adjust_and_clamp_counter`: Adjusts a stored counter by a scaled amount derived from a table lookup, clamps it to non-negative values, and divides a related quantity.
- `compare_bytes_and_store`: Invokes another routine, clears a stored word, computes the absolute difference between two bytes, sets a flag when the difference is large, and copies a byte into a global variable.
- `reset_flag_and_call_2`: Compares two adjacent global words and, depending on which is zero, resets one of them to zero or to a negative value while invoking another routine.
- `reset_flag_and_call`: Checks whether a global variable is nonzero, and if so resets it to zero and calls another routine before returning.
- `store_one_and_call`: Stores a one into a global variable, invokes another routine, and returns.
- `store_negative_and_call`: Stores a negative value into a global variable, invokes another routine, and returns.
- `invoke_scaled_updates`: Invokes five function pointers held in a global variable in sequence, then computes two scaled differences from a pair of global words and stores the results at two fixed global locations.
- `invoke_routine_by_threshold`: Compares a passed value against two thresholds and invokes another routine once for each threshold it exceeds, then always invokes that routine a final time.
- `update_and_conditionally_call`: Updates a global variable when it changes, indexing into tables with a scaled value, and conditionally invokes routines based on comparisons, then returns.
- `manage_state_counter`: Waits for a counter to reach a threshold, then drives a state machine that calls two routines, stores their results into a table and registers, and adjusts a running total.
- `copy_table_data_block`: Loads a block of data from a table pointed to by a global variable and copies its contents into a second global structure.
- `store_pointer_and_string_length`: Stores a pointer to a global variable and a computed address into several global variables, then records the length of a null-terminated string found by scanning for a terminator.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `accumulate_scaled_byte`: Computes a scaled value from a byte using either a shift or a multiply by a stored factor, adds it to a running total, and then calls another routine.
- `process_timer_tick`: Manages a countdown timer and, while it is active, calls a stored routine and adjusts a running total using several bit-shift amounts, returning when the timer expires.
- `empty_routine`: An empty routine that returns immediately without doing anything.
- `initialize_game_data`: Clears a block of global state, then, when a flag is clear, loads several game data files by name, copies a block of words into a stack area, and reads a series of values from those files into global variables.
- `call_routines_from_table`: Invokes two other routines through pointers stored in a table, passing the same pointer value to each of them.
- `invoke_routines_from_table`: Copies a block of words from a source into a stack area when a flag is clear, then invokes two other routines through pointers stored in a table and stores a returned value into a global variable.
- `invoke_table_routines`: Looks up two routines in a jump table and invokes them, passing a 16-bit value and a fixed argument to each, then calls two more routines from the table with adjacent pointer values.
- `init_road_drawer`: Stores the current frame pointer in a global variable, calls another routine, then retrieves two values from a routine through a global pointer and writes them into a structure along with several fixed fields.
- `store_flag_via_indirect_pointer`: Reads a global variable, calls another routine through it, then stores the value 65535 through a pointer derived from that value.
- `flag_indirect_caller`: Reads a global word, invokes the routine stored at that address, then writes the value 65535 through a pointer offset from the retrieved value.
- `populate_symbol_tables`: Looks up a series of fixed four-character identifiers in a symbol table and stores the results, then combines some of those results with values from another table.
- `populate_symbol_tables_2`: Looks up a series of four-character identifiers in a symbol table, stores the results, and copies each result into a table via another routine.
- `load_and_copy_identifiers`: Looks up several four-character identifiers and copies the returned values into a table, then copies the contents of that table into another table.
- `resolve_string_arrays`: Looks up five strings from a symbol table by name and copies each result into a contiguous array, then repeats the same lookup-and-copy process for two additional groups of strings.
- `resolve_string_pointers`: Looks up five-character ASCII identifiers ("grid", "trk", "sed", "sdr", ...) in a string table and stores the resulting pointers.
- `call_and_store_global_values`: Invokes a routine via a function pointer in a library base, passing the address of a global variable, and stores each returned value into consecutive global variables.
- `store_argument_and_offsets`: Calls a routine via a function pointer using a passed argument, then stores that argument and several computed addresses into global variables, adjusting some of them when the argument matches one of the stored addresses.
