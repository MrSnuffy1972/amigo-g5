# Game 2196 (faery tale adventure, the (1986)(microillusions)[h movers]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02196). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 19 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 92 of 92 named (0 author symbols), 92 of 92 described (33 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 604 of 604 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1938 of 1970 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 90008 |
| code | 6196 |
| data | 744169 |
| padding | 4939 |
| unused | 55808 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf:mover1 | 3 | 1681 | 82 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf:fmain | 3 | 228 | 7 |
| 9 | 1 | 51 | 2 |
| Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 32-136 (bytes 16384-70144) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.25)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 157-159 (bytes 80384-81920) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.70)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 164-256 (bytes 83968-131584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.96)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 258-311 (bytes 132096-159744) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.47)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 314-340 (bytes 160768-174592) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.17)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 342-342 (bytes 175104-175616) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.08)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 344-393 (bytes 176128-201728) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.98)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 396-415 (bytes 202752-212992) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.67)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 419-422 (bytes 214528-216576) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 424-445 (bytes 217088-228352) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.83)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 447-453 (bytes 228864-232448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.20)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 455-461 (bytes 232960-236544) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.34)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 463-469 (bytes 237056-240640) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.51)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 471-471 (bytes 241152-241664) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.26)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 473-473 (bytes 242176-242688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 477-477 (bytes 244224-244736) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 479-535 (bytes 245248-274432) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.97)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 538-671 (bytes 275456-344064) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.89)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 673-673 (bytes 344576-345088) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.02)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 677-695 (bytes 346624-356352) outside the filesystem hold data, not code (opcode density 0.005, entropy 4.90)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 698-711 (bytes 357376-364544) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.12)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 714-718 (bytes 365568-368128) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.03)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 720-756 (bytes 368640-387584) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.22)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 760-782 (bytes 389120-400896) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.68)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 784-791 (bytes 401408-405504) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.54)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 798-798 (bytes 408576-409088) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 800-868 (bytes 409600-444928) outside the filesystem hold data, not code (opcode density 0.007, entropy 4.58)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 870-879 (bytes 445440-450560) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.35)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 882-917 (bytes 451584-470016) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.28)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 920-1171 (bytes 471040-600064) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.74)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 1210-1210 (bytes 619520-620032) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.89)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 1222-1231 (bytes 625664-630784) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.32)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 1280-1501 (bytes 655360-769024) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.69)
- faery tale adventure, the (1986)(microillusions)[h movers]/Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf: blocks 1653-1653 (bytes 846336-846848) outside the filesystem hold data, not code (opcode density 0.008, entropy 5.73)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf:mover1

- `setup_display_frame_loop`: Program entry for the mover program: opens libraries and a display, loads a sixteen-entry colour palette, draws border lines and text, then loops over frames until a completion value is seen, after which it closes the resources.
- `draw_credit_lines_and_loop`: Draws a set of fixed text lines that include a city and postal address, a product code, the words THE MOVERS and a country name, sets the palette and border lines, then runs a frame loop that reports a graphics error when a call fails.
- `clear_slot_block_save_return`: Clears a block of sixty longwords in a per-slot record, stores the caller's return address and a second value there, then calls another routine.
- `format_into_buffer_and_emit`: Points a buffer pointer at a per-stream record, runs a formatting routine over the arguments, then passes the buffer and its length to an output routine, choosing a variant from a mode field.
- `buffer_char_and_flush`: Stores one byte in a 40-byte line buffer and, once the buffer is full, passes it to an output routine and resets the buffer pointer, returning the byte.
- `flush_line_buffer`: Writes the buffered bytes out through the output routine, resets the buffer pointer to the start of the buffer, and returns the low byte of a stored field.
- `dispatch_two_stream_helpers`: Decompilation timed out; the recovered facts show 49 instructions and two calls into other routines, with no further behaviour recovered.
- `parse_printf_format_directives`: Walks a printf-style format string, copying literal characters through a callback, and handles width, precision and length modifiers for character, string, decimal, octal, unsigned and hexadecimal conversions, returning the number of characters produced.
- `perform_signed_division`: Determines the sign of the result from the signs of the two operands, calls the unsigned division routine, and negates the quotient when the sign is negative.
- `return_divide_remainder_value`: Calls the unsigned division routine and returns the value it leaves in the second data register.
- `perform_32_bit_division`: Computes the quotient and remainder of a 32-bit unsigned division, using a native divide when the high word of the divisor is zero and a shift-and-subtract loop otherwise, and returns both results together.
- `call_helper_until_count_reached`: Calls another routine repeatedly, counting successes up to the product of the two 16-bit halves of its second argument, stops early on a minus-one result, and returns the second argument's high half, or zero on failure.
- `forward_to_single_routine`: Decompilation timed out; the recovered facts show 22 instructions and a single call into another routine, with no further behaviour recovered.
- `iterate_records_calling_routine`: Walks a region of memory in 22-byte steps from a fixed start address to a fixed end address, calling another routine on each record.
- `release_record_fields_and_clear`: Returns minus one for a null record; otherwise, when the record's flag byte is set, it calls release routines selected by the flag bits, then clears the record's fields and flag byte.
- `release_flagged_record_fields`: When the record's flag byte is set, calls release routines selected by its bits, then zeroes the record's fields and returns the combined status word.
- `write_byte_to_buffered_stream`: Writes one byte to a buffered output stream, flushing through a helper when the buffer is full.
- `allocate_free_record_slot`: Scans a 22-byte record table for an unused entry, clears its three leading longwords, and returns it, or null when every entry is in use.
- `find_free_record`: Scans a table of 22-byte records for an entry where a specific byte is zero, clears the first three longwords of that entry, and returns its pointer.
- `clear_record_header_longs`: Zeroes the first three longwords of a record.
- `initialize_stream_buffer`: Sets up a record's buffer pointer and size: when one check succeeds and a buffer is obtained, it selects a 1024-byte buffer with a buffered flag, otherwise it points the buffer at the record's inline area with a size of one.
- `free_all_list_nodes`: Walks a singly linked list from a global head, releasing each node through a helper with its stored size, then clears the global head.
- `free_list_nodes_from_pointer`: Releases each node of a singly linked list, starting from the given pointer, through a helper, then clears the global list head.
- `allocate_node_and_link`: Allocates a node through the library allocation call, stores the node's size and payload pointer, pushes it onto a global singly linked list, and returns the payload address or null.
- `forward_low_word_argument`: Passes the low word of its argument on to another routine.
- `remove_node_and_release`: Searches a global linked list for the node whose payload matches the argument; if found, unlinks it, releases it through a helper and returns zero, otherwise returns minus one.
- `unlink_matching_node_and_release`: Searches a linked list from a given start for the node matching the argument, releases it through a helper and returns zero, or returns minus one when no node matches.
- `replace_link_and_release_node`: Replaces the link that points to a node with that node's successor, passes the node to the release routine, and returns zero.
- `pop_list_head_and_free`: Sets the global list head to the successor of the first node, makes one library call, and returns zero.
- `test_table_entry_nonzero`: Looks up an entry in a table of six-byte records by the low word of its argument, calls a library routine with it, and returns whether the result was non-zero.
- `store_library_result_on_failure`: Calls one routine; when it returns zero, makes two library calls and stores the low word of the second result in a global, returning minus one, otherwise returns zero.
- `return_zero_stub`: Returns zero with no other effect.
- `invoke_two_library_vectors`: Decompilation timed out; the listed facts show two library calls at fixed offsets and a call into another routine, with no further behaviour recovered.
- `check_flag_and_call_library`: Makes a library call and returns its status; when the bit 4096 is set and a stored flag is non-zero, it makes further library calls before returning.
- `call_optional_hook_then_route`: Calls a stored hook routine if one is installed, then calls another routine.
- `open_libraries_and_parse_names`: Opens the DOS and mathffp libraries by name, then parses a delimited list of names from a buffer into a pointer table and calls library routines to finish setup, reporting a failure when a library cannot be opened.
- `tiny_move`: Small routine of 1 instruction that copies d0 to d0.
- `shutdown_resources_and_return`: Calls a helper ten times, then conditionally calls library routines that release the stored handles, and returns the caller's value.
- `append_string_to_destination`: Appends a zero-terminated source string to the end of the destination buffer, copying at most about 32,767 bytes, terminates the result and returns the destination pointer.
- `append_string_with_limit`: Finds the end of the destination string, then copies the source string onto it byte by byte, stopping at the source terminator or when the count in the third argument runs out, and terminates the result.
- `copy_string_to_buffer`: Copies bytes from a source pointer to a destination pointer until a zero byte is copied, including the terminator, and returns the original destination pointer.
- `compute_string_length`: Walks a zero-terminated byte string to its terminator and returns the number of bytes before it, without modifying memory.
- `copy_bytes_with_padding`: Copies bytes from a source to a destination for a counted length, stopping early at a zero byte, then writes zero bytes to fill the rest of the counted area, and returns the destination pointer.
- `release_indexed_table_entry`: Validates a table index from zero to nineteen; when the entry is set, releases its memory through a library call unless a flag bit is set, then clears the entry.
- `call_library_vector_36`: Library stub: loads a library base from a global variable and calls the library function at offset minus 36 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_144`: Library stub: loads a library base from a global variable and calls the library function at offset minus 144 of that base, returning its result to the caller.
- `call_library_vector_54`: Library stub: loads a library base from a global variable and calls the library function at offset minus 54 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_library_vector_216`: Library stub: loads a library base from a global variable and calls the library function at offset minus 216 of that base, returning its result to the caller.
- `call_library_vector_30`: Library stub: loads a library base from a global variable and calls the library function at offset minus 30 of that base, returning its result to the caller.
- `call_library_vector_60`: Library stub: loads a library base from a global variable and calls the library function at offset minus 60 of that base, returning its result to the caller.
- `call_library_vector_48`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `call_library_vector_108`: Library stub: loads a library base from a global variable and calls the library function at offset minus 108 of that base, returning its result to the caller.
- `close_library_base`: Closes a library by calling its close function through the stored library base.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_294`: Library stub: loads a library base from a global variable and calls the library function at offset minus 294 of that base, returning its result to the caller.
- `invoke_library_routine_132`: Calls the library function at a fixed offset through the stored library base and returns its result.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_function`: Loads a library base from a global variable and executes a function at a specific offset within that library.
- `call_library_vector_372`: Library stub: loads a library base from a global variable and calls the library function at offset minus 372 of that base, returning its result to the caller.
- `open_library_by_name`: Opens a library through the stored library base using the open-library function, returning the library base or null.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_378`: Library stub: loads a library base from a global variable and calls the library function at offset minus 378 of that base, returning its result to the caller.
- `call_library_vector_306`: Library stub: loads a library base from a global variable and calls the library function at offset minus 306 of that base, returning its result to the caller.
- `call_library_vector_318`: Library stub: loads a library base from a global variable and calls the library function at offset minus 318 of that base, returning its result to the caller.
- `call_library_vector_384`: Library stub: loads a library base from a global variable and calls the library function at offset minus 384 of that base, returning its result to the caller.
- `call_library_vector_246`: Library stub: loads a library base from a global variable and calls the library function at offset minus 246 of that base, returning its result to the caller.
- `call_library_vector_240`: Library stub: loads a library base from a global variable and calls the library function at offset minus 240 of that base, returning its result to the caller.
- `call_library_vector_396`: Library stub: loads a library base from a global variable and calls the library function at offset minus 396 of that base, returning its result to the caller.
- `call_library_vector_342`: Library stub: loads a library base from a global variable and calls the library function at offset minus 342 of that base, returning its result to the caller.
- `call_library_vector_288`: Library stub: loads a library base from a global variable and calls the library function at offset minus 288 of that base, returning its result to the caller.
- `call_library_function_2`: Executes a function from a library retrieved via a pointer stored in a global variable.
- `call_library_vector_66`: Library stub: loads a library base from a global variable and calls the library function at offset minus 66 of that base, returning its result to the caller.
- `call_library_function_3`: Calls a library function using an offset relative to a pointer found in a global variable.
- `allocate_memory_block`: Allocates a memory block through the library's allocation call using the stored base, and returns the block pointer.
- `call_library_vector_204`: Library stub: loads a library base from a global variable and calls the library function at offset minus 204 of that base, returning its result to the caller.
- `call_library_vector_282`: Library stub: loads a library base from a global variable and calls the library function at offset minus 282 of that base, returning its result to the caller.
- `call_library_vector_300`: Library stub: loads a library base from a global variable and calls the library function at offset minus 300 of that base, returning its result to the caller.
- `call_library_function_4`: Performs an indirect call to a library function based on a pointer stored in a global variable.

### Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf:fmain

- `forward_program_entry`: Program entry for the file; decompilation timed out, so the only recovered behaviour is a single call into another routine.
- `load_amiga_hunk_file`: Parses AmigaDOS hunk records for code, data, uninitialised data and 32-bit relocation entries, allocating memory for each section, copying its contents and freeing the scratch buffer through exec.
- `return_no_op_stub`: Returns without doing anything, so it is a no-op stub.
- `decrunch_backward_bitstream`: Unpacks a compressed block by reading bits from a stream, copying literal bytes and back-references backwards from the end of the output buffer until the output pointer reaches the start.
- `decompress_bitstream_data`: Reads a bitstream to determine how many bytes to copy from a source table into a destination buffer using bit-shifting and pointer updates.
- `shift_pointer_value`: The routine reads a 32-bit value from four bytes before the address in A0, shifts it right by one bit and sets the most significant bit, then returns the result.
- `shift_bit_buffer_with_refill`: Shifts a 32-bit bit buffer right by a count, refilling it from the preceding longword with its top bit set whenever it empties, and stores the current read pointer in a global variable.

### 9

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `empty_routine`: This routine is empty and contains no executable instructions.

### Faery Tale Adventure, The (1986)(MicroIllusions)[h Movers].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
