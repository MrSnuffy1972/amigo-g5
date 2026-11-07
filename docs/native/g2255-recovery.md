# Game 2255 (railroad tycoon (1990)(microprose)[cr image][h reaktor]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02255). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 86 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 19 of 19 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 222 of 222 named (5 author symbols), 222 of 222 described (53 by rule, 33 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5540 of 5569 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 28604 |
| code | 17190 |
| data | 810208 |
| padding | 21054 |
| unused | 24064 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| binddrivers | 11 | 652 | 35 |
| type | 2 | 117 | 2 |
| path | 2 | 117 | 2 |
| assign | 2 | 117 | 2 |
| border | 2 | 207 | 8 |
| Disk-Validator | 1 | 386 | 1 |
| ram-handler | 2 | 10 | 1 |
| 16 | 1 | 2 | 1 |
| 19 | 1 | 48 | 2 |
| 1 | 1 | 2 | 1 |
| 4 | 1 | 2 | 1 |
| 7 | 1 | 2 | 1 |
| 9 | 1 | 2 | 1 |
| diskfont.library | 13 | 830 | 67 |
| Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:nick.a | 5 | 121 | 7 |
| Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:game | 2 | 233 | 8 |
| Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:Crystal | 2 | 85 | 3 |
| Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:rinstall | 5 | 2626 | 78 |
| Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:bootblock | 1 | 10 | 1 |

## Cases handled

- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:diffs.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:clocos.pix: high-entropy data file (7.86 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:page7.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:locos2.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:page5.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:diffsp.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:locos0.pix: high-entropy data file (7.87 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:council.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:credits2.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:page3.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:elocos2.pix: high-entropy data file (7.89 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:tracks.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:title.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:elocos0.pix: high-entropy data file (7.86 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:page2.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:locosm.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:page0.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:advert.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:elocosm.pix: high-entropy data file (7.88 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:page8.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:britain.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:csprites.pix: high-entropy data file (7.91 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:europe.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:page6.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:locos1.pix: high-entropy data file (7.88 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:locos.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:page4.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:elocos3.pix: high-entropy data file (7.87 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:westus.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:estation.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:elocos1.pix: high-entropy data file (7.83 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:station.pix: high-entropy data file (7.94 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:clocosm.pix: high-entropy data file (7.89 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:labs.pix: high-entropy data file (7.87 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:page1.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:esprites.pix: high-entropy data file (7.90 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:faces.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:sprites.pix: high-entropy data file (7.92 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:eastus.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:elocos.pix: high-entropy data file (7.88 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:page9.pix: high-entropy data file (7.93 bits/byte), classified as packed data
- railroad tycoon (1990)(microprose)[cr image][h reaktor]/Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:game: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### binddrivers

- `parse_startup_arguments_and_run`: Allocates a startup record, takes the arguments from the command line or from the Workbench startup message, parses quoted and escaped words into a list, runs the main routine, then frees the record and closes the libraries, returning 20 when the allocation fails.
- `bind_expansion_drivers`: Opens the expansion and icon libraries and allocates a 260-byte buffer, then walks a list of expansion entries, matching each product name against a stored table and running a per-entry routine with the matched entry temporarily guarded, before releasing the buffers and libraries.
- `find_resident_module_tag`: Scans a block of memory from the start address for a header that carries a fixed marker word and a pointer back to itself, returning that header or null when none is found within the range.
- `combine_pipe_separated_segments`: Walks a string split at vertical-bar characters, passing each segment to a combining routine and accumulating the results, and returns the total.
- `parse_slash_separated_path_part`: Finds the next slash-separated part of a path, performs a lookup on the part before it, and combines the result with the lookup of the remaining path, returning zero when a lookup fails.
- `prepend_flagged_nodes_to_list`: Walks the nodes returned by a search routine and, for each node whose flag bit one is set and that is not already in the list, links it at the head by storing the previous head in its link field, then returns the new head.
- `search_link_chain_for_node`: Follows the link stored at offset 48 from each node in turn, returning the pointer with a flag set to one when the target node is reached and zero when the chain ends.
- `find_non_digit_within_length`: Scans up to the given number of characters and returns one as soon as a non-digit is found, and zero when every checked character is a digit; a zero length returns one.
- `parse_decimal_integer`: Reads decimal digit characters from a string until a non-digit or the terminator, accumulating them into an integer that it returns.
- `return_result_if_within_bound`: Calls a lookup with its first two arguments and returns the result, or zero when that result exceeds the first argument plus the third argument.
- `call_library_vector_102`: Library stub: loads a library base from a global variable and calls the library function at offset minus 102 of that base, returning its result to the caller.
- `call_library_vector_198`: Library stub: loads a library base from a global variable and calls the library function at offset minus 198 of that base, returning its result to the caller.
- `call_library_vector_210`: Library stub: loads a library base from a global variable and calls the library function at offset minus 210 of that base, returning its result to the caller.
- `call_library_vector_414`: Library stub: loads a library base from a global variable and calls the library function at offset minus 414 of that base, returning its result to the caller.
- `call_library_vector_552`: Library stub: loads a library base from a global variable and calls the library function at offset minus 552 of that base, returning its result to the caller.
- `call_library_vector_72`: Library stub: loads a library base from a global variable and calls the library function at offset minus 72 of that base, returning its result to the caller.
- `call_library_vector_120`: Library stub: loads a library base from a global variable and calls the library function at offset minus 120 of that base, returning its result to the caller.
- `call_library_vector_126`: Library stub: loads a library base from a global variable and calls the library function at offset minus 126 of that base, returning its result to the caller.
- `call_library_vector_132`: Library stub: loads a library base from a global variable and calls the library function at offset minus 132 of that base, returning its result to the caller.
- `call_make_library`: Library stub: loads its arguments and the exec library base, calls MakeLibrary and returns its result to the caller.
- `call_make_functions`: Library stub: loads its arguments and the exec library base, calls MakeFunctions and returns its result to the caller.
- `call_init_resident`: Library stub: loads its arguments and the exec library base, calls InitResident and returns its result to the caller.
- `call_alert`: Library stub: loads its arguments and the exec library base, calls Alert and returns its result to the caller.
- `call_enable`: Library stub: loads its arguments and the exec library base, calls Enable and returns its result to the caller.
- `call_super_state`: Library stub: loads its arguments and the exec library base, calls SuperState and returns its result to the caller.
- `call_user_state`: Library stub: loads its arguments and the exec library base, calls UserState and returns its result to the caller.
- `compare_two_strings`: Compares two zero-terminated byte strings character by character, returning minus one at the first difference where the first character is smaller and one where it is larger, and zero when both strings end together.
- `measure_c_string_length`: Counts the characters of a zero-terminated string and returns that length.
- `compute_checksum`: Initializes a 32-bit accumulator to all ones and decrements it by one for each byte in a null-terminated string.
- `find_character_in_string`: Returns a pointer to the first occurrence of the given character in a zero-terminated string, or null when the character is absent.
- `copy_c_string`: Copies the source string byte by byte into the destination, including the terminating zero byte.
- `compute_bounded_offset`: Computes an offset from the argument and two values returned by lookups, returning that offset when the second lookup value does not exceed the first and a further check passes, and zero otherwise.
- `call_library_vector_78`: Library stub: loads a library base from a global variable and calls the library function at offset minus 78 of that base, returning its result to the caller.
- `call_library_vector_90`: Library stub: loads a library base from a global variable and calls the library function at offset minus 90 of that base, returning its result to the caller.
- `call_library_vector_96`: Library stub: loads a library base from a global variable and calls the library function at offset minus 96 of that base, returning its result to the caller.

### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call`: When the destination pointer is non-null, copies sixteen 32-bit words from a fixed block in the base record into it, then calls a jump-vector routine.

### path

- `build_handler_block_from_template`: Builds a block sized from the largest of several table entries, fills its header words with a tagged sequence, records the handler address and copies a sixteen-word template into it, returning minus one when the allocation fails.
- `copy_and_call_routine`: Copies a block of sixteen bytes from a table to a stack location and then calls another routine.

### assign

- `initialize_engine_state`: Iterates through a linked list to find a maximum value, then populates a data structure with a sequence of incrementing values.
- `copy_startup_data`: Copies a block of sixteen bytes from a memory location to a destination pointed to by a stack variable before calling another routine.

### border

- `load_hunk_format_executable`: Reads a hunk-format executable by allocating memory for each hunk, then handles code, data and uninitialised hunks and 32-bit relocation records, calling exec library routines along the way.
- `stub_empty_return_routine`: This routine is an empty stub in the program.
- `delay_by_counting_down`: Decrements the count register in a loop until it passes zero, consuming time, and returns with no result.
- `tiny_ori`: Small routine of 1 instruction that merges 208 into d0.
- `unpack_backward_compressed_stream`: Expands a packed byte stream into the output buffer working backward from its end, copying literal bytes and runs that refer back to earlier output until the output reaches the start address.
- `tiny_moveq`: Small routine of 1 instruction that loads 1 into d0.
- `decrement_byte_wrapping_to_space`: Decrements a byte once per count unit, replacing a zero result with a space character, for the given number of iterations.
- `stub_bad_instruction_data_2`: Contains only undecodable data and no real instructions, so it has no behaviour to describe.

### Disk-Validator

- `compare_bitmap_cells_and_count`: Writes a two-digit value into a text buffer with a leading space for values below ten, then compares the bits of two bitmaps cell by cell, calling a routine and incrementing a counter wherever they differ.

### ram-handler

- `copy_memory_then_dispatch`: Copies memory through the exec library's copy routine, then calls a handler through an indirect pointer.

### 16

- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.

### 19

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `return_zero_status`: Returns zero with no other work.

### 1

- `return_zero_value`: Returns a zero value to the caller.

### 4

- `return_zero`: This routine is empty and simply returns a value of zero to the caller.

### 7

- `return_zero_2`: This routine performs no operations and returns a value of zero to the caller.

### 9

- `return_zero_status_2`: Returns zero with no other work and reports no result.

### diskfont.library

- `store_argument_into_font_handle`: Calls a diskfont library routine and, when it returns a non-null handle, stores the argument at offset 34 of that handle and calls a second library routine, returning whether a handle was obtained.
- `check_and_set_font_handle`: This library code calls a routine via a table entry using the library base.
- `open_libraries_init_list`: Opens two system libraries and, when both succeed, initialises an empty list header inside the supplied block and returns that block; otherwise it closes any library that opened, frees the block and returns zero.
- `close_library_and_free_block`: Closes a library and frees the memory block it was allocated in, then returns zero.
- `unload_idle_entries_then_free`: Diskfont library code: walks a list of loaded entries, unloading the segment of each entry whose count has reached zero and unlinking it, then closes two libraries and frees the block once nothing is pending, returning the stored result.
- `free_block_when_idle`: Diskfont library code: when both the pending count and the entry count are zero, closes two libraries, frees the block and returns its stored result; otherwise it sets the pending bit and returns zero.
- `mark_block_pending_flag`: Diskfont library code: sets the high bit of a flag byte in the block to mark it as pending and returns zero.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_pending_flag_and_count`: Diskfont library code: clears the pending high bit in a flag byte of the block and increments a counter word.
- `decrement_count_dispatch_if_pending`: Decrements a counter word in the block and, when it remains nonzero and the pending bit is set, calls a library routine through its jump table and returns the result; otherwise it returns zero.
- `return_zero_status_3`: Diskfont library code: returns zero with no other work and reports no result.
- `collect_font_entries_into_buffer`: Diskfont library code: fills a buffer with font entries: when the first flag is set it copies names from an in-memory font list, and when the second flag is set it scans the fonts directory, then returns the amount by which the buffer overflowed, or zero when everything fitted.
- `copy_list_names_into_buffer`: Diskfont library code: walks a linked list of font entries, copying each entry's name into the buffer, and when the second flag is set also scans the fonts directory; returns the overflow amount, or zero when everything fitted.
- `copy_name_bytes_into_buffer`: Diskfont library code: copies the bytes of a name into the buffer, writing a record each time the buffer limit is reached, and when the second flag is set also scans the fonts directory; returns the overflow amount or zero.
- `scan_fonts_directory_entries`: Diskfont library code: locks the fonts directory, examines its entries, and for each matching name copies it into the buffer with its size, freeing the scan buffers at the end and returning the overflow amount or zero.
- `emit_font_records_from_scan`: Diskfont library code: copies a name into the buffer and writes a record for it, repeating over the directory scan results until none remain, then returns the overflow amount or zero.
- `read_font_file_records`: Diskfont library code: reads a font file block by block through the dos read routine, copying names and records from the parsed entries into the buffer until the file is exhausted, and returns the overflow amount or zero.
- `read_file_records_then_close`: Diskfont library code: copies font records from the parsed file data into the buffer, closes the file through the dos close routine when finished, and returns the overflow amount or zero.
- `release_scan_buffers_return_overflow`: Diskfont library code: restores the current directory, unlocks the scan lock and frees the scan buffer, then returns the overflow amount, or zero when the counter is not negative.
- `return_zero_constant_2`: This library code immediately returns a zero value without performing any other operations or accessing memory.
- `score_font_match_candidate`: Diskfont library code: scores how closely a candidate font matches a request by weighting the vertical size difference, subtracting a table weight for each of seven style bits that differ, and returning minus 32767 when the candidate has no size.
- `score_font_style_bits`: Diskfont library code: starts from a score derived from the size argument and subtracts a table weight for each of seven style bits that differ between the request and the candidate.
- `score_style_bits_fallback`: Diskfont library code: walks seven style bits and subtracts a table weight for each bit that differs between the request and the candidate, returning the accumulated score.
- `score_style_bits_alternate`: Diskfont library code: walks seven style bits using a second set of masks and subtracts a table weight for each differing bit, returning the accumulated score.
- `return_minimum_score`: Diskfont library code: returns minus 32767, the score given to a candidate that cannot match.
- `find_best_matching_font`: Diskfont library code: copies a seven-character font name, looks up the matching entry in the font list, scans candidate font files in the fonts directory, scores each against the request and keeps the best, then returns the chosen entry.
- `scan_fonts_for_best_match`: Diskfont library code: scans the fonts directory for candidate entries, scores each against the request, links the best candidate into the font list, frees its scan buffers and returns the chosen entry.
- `select_best_candidate_entry`: Diskfont library code: repeatedly scores the candidate block against the request, takes the highest-scoring entry, links it into the font list, and returns the final entry once no candidate remains.
- `load_best_font_segment`: Diskfont library code: repeatedly picks the highest-scoring candidate, loads its font segment, and under a forbid and permit bracket links the entry into the font list with the graphics add-font routine, returning the last entry handled.
- `replace_best_candidates_and_unload`: Diskfont library code: picks the best-scoring candidates in turn, links each into the font list and unloads the segment of any entry that no longer matches, returning the last entry.
- `release_font_scan_resources`: Diskfont library code: repeatedly picks candidates and links them into the font list, then restores the current directory, unlocks the lock, frees the buffers and closes the font handles, returning the last entry.
- `call_forbid`: Library stub: loads its arguments and the exec library base, calls Forbid and returns its result to the caller.
- `call_permit`: Library stub: loads its arguments and the exec library base, calls Permit and returns its result to the caller.
- `call_alloc_mem`: Library stub: loads its arguments and the exec library base, calls AllocMem and returns its result to the caller.
- `call_free_mem`: Library stub: loads its arguments and the exec library base, calls FreeMem and returns its result to the caller.
- `call_add_tail`: Library stub: loads its arguments and the exec library base, calls AddTail and returns its result to the caller.
- `call_open`: Library stub: loads its arguments and the dos library base, calls Open and returns its result to the caller.
- `call_close`: Library stub: loads its arguments and the dos library base, calls Close and returns its result to the caller.
- `call_read`: Library stub: loads its arguments and the dos library base, calls Read and returns its result to the caller.
- `call_lock`: Library stub: loads its arguments and the dos library base, calls Lock and returns its result to the caller.
- `call_un_lock`: Library stub: loads its arguments and the dos library base, calls UnLock and returns its result to the caller.
- `call_examine`: Library stub: loads its arguments and the dos library base, calls Examine and returns its result to the caller.
- `call_ex_next`: Library stub: loads its arguments and the dos library base, calls ExNext and returns its result to the caller.
- `call_current_dir`: Library stub: loads its arguments and the dos library base, calls CurrentDir and returns its result to the caller.
- `call_load_seg`: Library stub: loads its arguments and the dos library base, calls LoadSeg and returns its result to the caller.
- `call_un_load_seg`: Library stub: loads its arguments and the dos library base, calls UnLoadSeg and returns its result to the caller.
- `copy_c_string_2`: Diskfont library code: copies bytes from the source string into the destination byte by byte until the terminating zero byte has been copied.
- `seek_string_end_then_forward`: Diskfont library code: advances to the terminating zero of the string and passes that end position and the argument on to a follow-on routine.
- `forward_to_library_routine`: Calls one library routine through its jump vector and returns, adding no work of its own.
- `measure_c_string_length_2`: Diskfont library code: counts the characters of a zero-terminated string and returns that length.
- `return_without_effect`: Diskfont library code: returns immediately with no body, so the routine has no effect.
- `compare_two_strings_2`: Diskfont library code: compares two zero-terminated strings character by character, returning minus one when the first differing character is smaller, one when it is larger, and zero when the strings match.
- `compare_byte_with_previous_entry`: Diskfont library code: compares the byte at the first pointer with the byte just before the second pointer and returns minus one when it is smaller, otherwise one.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `find_character_in_string_2`: Diskfont library code: returns a pointer to the first occurrence of the given character in a zero-terminated string, or null when the character is absent.
- `find_next_character_occurrence`: Diskfont library code: moves past the current position in a string and returns a pointer to the next occurrence of the search byte, or null when the string ends first.
- `return_zero_status_4`: Diskfont library code: returns zero with no other work.
- `find_last_character_in_string`: Diskfont library code: returns a pointer to the last occurrence of the given character in a string by scanning backward from its end, or null when the character is absent.
- `find_previous_character_before_bound`: Diskfont library code: scans backward from the current position for the given byte, stopping at the lower bound, and returns the pointer found or null.
- `return_zero_3`: This library code returns zero.
- `find_path_separator_or_start`: Diskfont library code: returns the position of the first slash or colon in the string, or the start of the string when neither character is present.
- `return_void`: This library code returns without performing any operations.
- `call_open_font`: Library stub: loads its arguments and the graphics library base, calls OpenFont and returns its result to the caller.
- `call_close_font`: Library stub: loads its arguments and the graphics library base, calls CloseFont and returns its result to the caller.
- `call_add_font`: Library stub: loads its arguments and the graphics library base, calls AddFont and returns its result to the caller.
- `invoke_unnamed_routine_and_return`: Diskfont.library entry that makes a single call to an unnamed routine and then returns, with no other work visible.
- `dispatch_unnamed_routine_call`: Small diskfont.library entry that calls one unnamed routine through a pointer and returns.

### Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:nick.a

- `launch_program_via_two_calls`: Program entry point in the nick.a file, 33 instructions long, reached through an address-taken reference and the program entry.
- `save_and_install_self_handler`: Saves a previous value, writes a pointer to itself into a library slot, and fetches a structure through a library call whose field is then pointed at a global variable.
- `call_library_then_install_handler`: Calls a library entry first, then stores a pointer to itself into a library slot, points a structure field at a global variable, and makes an indirect call before returning.
- `detach_slot_into_global`: Reads a pointer from a slot in the data area.
- `setup_screen_and_wait`: Opens dos.library and intuition.library, sets fixed values in a display structure, and waits until two screen coordinate fields reach 640 and 256.
- `write_byte_then_wait_flag`: Writes one byte taken from the preceding byte of an input pointer to a destination pointer, then busy-waits while bit 6 of a global variable stays set.
- `wait_until_ciaa_bit_clear`: Spins while bit 6 of a global variable is set, which the listing associates with the CIAA port that carries the fire buttons and disk status, and then returns.

### Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:game

- `load_hunks_and_relocate`: Program-entry loader that walks the executable's hunk records, allocates memory for code and data hunks with AllocMem, and applies 32-bit relocation entries until it reaches the end marker.
- `busy_wait_on_counter`: Loops decrementing the 16-bit counter in register D1 until it wraps around, burning a number of iterations set by the caller.
- `return_from_subroutine`: This routine is empty and performs no operations before returning to the caller.
- `decode_backward_run_copy`: Decodes packed data backwards into a buffer from its end.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_bits_refilling_stream`: Reads bits from a stream into an accumulator one count at a time, loading the next longword from memory whenever the bit buffer empties.
- `stub_bad_instruction_data`: Contains a trap instruction followed by undecodable bytes, so the decompiler truncates it.

### Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:Crystal

- `read_packed_bits_set_color`: Program entry in the Crystal file that calls the bit-buffer refill and multi-bit read routines to decode a packed stream, and writes the COLOR00 background colour register.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:rinstall

- `check_signature_then_install`: Program entry in the rinstall file that first checks a magic word at a fixed address.
- `return_fixed_constant_value`: Returns the fixed value 1192 in the result register and does no other work.
- `clear_state_and_open_font`: Clears a block of global state, then tries to open the font file RAILA:fonts/railf0.font through the DOS library, either using a preset file or building a quoted path from the argument text.
- `cleanup_close_handles_and_reply`: Stores its result argument and runs an optional callback.
- `call_library_vector_entry`: Calls a library entry through a pointer read from a value on the stack at a fixed negative offset, then returns.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `nop_placeholder`: This routine is empty and performs no operations.
- `lookup_stream_then_forward`: Looks up a stream record through a routine and returns minus one if it is missing.
- `emit_string_into_buffer`: Walks a zero-terminated string and writes each character into a buffer while a remaining-space counter stays positive.
- `parse_args_and_open_console`: Splits a command line into words, honouring quoted strings and star escapes for a newline and a control code, and stores the word pointers in an argument table.
- `write_fixed_fields_to_file`: Opens a file through the DOS library and writes a long fixed sequence of text and byte fields to it through the file handle.
- `initialise_stream_record_fields`: When a record's size field is zero or its flag bit is set, it fetches a value through a routine and stores it into the record's pointer fields, clearing the flag and counters.
- `write_char_buffered_with_crlf`: Writes one character into a stream buffer, inserting a carriage return before a line feed, and tracks the remaining space in the stream record.
- `close_open_records_in_reverse`: Walks the table of open records from last to first and passes each record whose first byte is set and whose bit 4 is clear to a routine.
- `append_bounded_text_checked`: Appends a source string to a destination string under a length limit, finding both ends first and then null-terminating the result.
- `concatenate_limited_string`: Appends up to a given number of bytes from a source string to the end of a destination string, then writes a zero terminator and returns the destination.
- `calculate_string_length`: Walks a zero-terminated string to its terminator and returns the number of bytes before it.
- `compare_c_strings`: Compares two zero-terminated strings byte by byte.
- `return_block_to_free_list`: Returns a block of memory to a free list kept in address order, rounding its size up to whole longwords with a minimum of eight bytes.
- `lookup_record_then_forward`: Looks up a record through a routine and returns minus one if none is found.
- `open_file_into_free_slot`: Finds the first unused entry in a table of open records from index three upward.
- `create_directory_save_error`: Creates a directory through the DOS library.
- `free_linked_allocations_list`: Walks a linked list of allocations, freeing each node with the exec FreeMem call, then clears the list head and count fields and returns zero.
- `allocate_block_with_header`: Releases any block pending in a stored pointer, then for a non-zero size allocates the size plus a four-byte header through a routine.
- `execute_recursive_search`: Recursively calls a routine with a pointer-based argument and checks a global variable for a stop condition.
- `allocate_tracked_node`: Allocates a node of the requested size plus a header through the exec AllocMem call, records the size and links the node into a list of tracked allocations.
- `allocate_from_free_list`: Allocates a block of at least the requested size, rounded up to whole longwords with a minimum of eight bytes.
- `store_pending_block_pointer`: Stores a pending block pointer for later release.
- `flush_pending_stream_writes`: Walks a table of stream records and, for each one with flag value 2 set and flag value 4 clear that still has unwritten bytes, passes the pending bytes to a write routine.
- `write_bytes_with_error_capture`: Calls the DOS Write through a stored vector and returns its result.
- `seek_stream_position_checked`: Wraps the DOS Seek call and saves the DOS error code with error code 22 when it fails.
- `read_bytes_with_error_capture`: Calls the DOS Read through a stored vector and returns the byte count.
- `open_file_record_error_code`: Calls the DOS Open through a stored vector.
- `probe_file_lock_or_open`: Tries a DOS Lock on the named file and, if that fails, tries DOS Open and returns the handle.
- `remove_locked_file_then_open`: Locks the named file and, when the lock succeeds, unlocks it and deletes the file.
- `close_file_handle`: Runs a stored callback first when its pointer is set, then closes the DOS file handle through the Close vector and returns zero.
- `open_intuition_then_copy_name`: Opens intuition.library through exec.
- `compute_fixed_point_product`: Multiplies two 32-bit fixed-point values held in registers by combining the cross products of their 16-bit halves.
- `divide_signed_fixed_point`: Divides two 32-bit fixed-point values, negating the result when exactly one operand is negative.
- `normalize_fixed_point_divide`: Divides a 32-bit dividend by a 32-bit divisor using normalised shifts and 16-bit divides.
- `report_message_cli_or_window`: Copies a length-prefixed message into a local buffer, capped at 79 bytes.
- `report_break_notice`: Copies a counted run of bytes into a local buffer and terminates it.
- `open_intuition_with_dimensions`: Opens a library through exec and, when that succeeds, passes a window argument block with the dimensions 250 and 60 to a routine.
- `store_buffer_and_open_window`: Stores the address of a local buffer in the state block, then passes the state fields together with the dimensions 250 and 60 to a routine.
- `return_zero_constant`: Returns zero with no other work and no memory access.
- `close_record_handle_checked`: Looks up a record through a routine and returns minus one if it is missing.
- `lookup_table_entry_by_index`: Clears the error code and checks a table index against the table count and whether the entry is in use.
- `check_break_signal_callback`: Reads the task signal state through SetSignal and, when the break-signal bits are set and a callback is stored, runs the callback.
- `allocate_zeroed_block`: Obtains a size from a routine and allocates that many bytes through another routine.
- `format_and_write_file_record`: Constructs a formatted string by concatenating multiple data fields and writes the resulting buffer to a file record.
- `release_handle_return_pointer`: Calls a routine with the stored file handle and then calls another routine with zero, returning the pointer that second call produces.
- `release_state_values_pointer`: Passes two stored state values to routines and returns the pointer from a third call made with one.
- `copy_string_to_buffer_2`: Copies a null-terminated string into a destination buffer and ensures it is null-terminated, or returns a pointer from another routine if a condition is not met.
- `copy_string_with_parameter`: Copies a null-terminated string into a destination buffer using a length derived from a parameter, or calls another routine to process data if a condition is not met.
- `retry_until_nonzero_result`: Calls a routine repeatedly until it returns a non-zero pointer and returns that pointer.
- `zero_allocated_block`: Allocates a block of memory and fills it with zeros using a loop.
- `open_file_record`: Attempts to open a file and stores an error code in a global variable if the operation fails.
- `close_record_handle`: Closes a record handle and clears its pointer while checking a status bit and a global error variable.
- `execute_callback_and_close`: Executes a stored callback function through a pointer and returns zero.
- `compare_c_strings_2`: Compares two null-terminated strings and returns a result indicating their relative order.
- `calculate_string_length_2`: Calculates the number of bytes in a null-terminated string.
- `traverse_linked_list`: Recursively traverses a list using a pointer from a record and checks a global variable for a stop condition.
- `store_pending_pointer`: Stores a pointer offset by four bytes into a global variable.
- `create_directory_record`: Attempts to create a directory and updates global error and status variables based on the result.
- `jump_to__unlock`: Jump stub: passes control straight on to _UnLock.
- `jump_to__delay`: Jump stub: passes control straight on to _Delay.
- `copy_string_to_buffer`: Copies a null-terminated string into a destination buffer while decrementing a remaining space counter.
- `jump_to__write`: Jump stub: passes control straight on to _Write.
- `flush_stream_records`: Iterates through a linked list of stream records and calls another routine to write data for records meeting specific flag and size criteria.
- `jump_to__lock`: Jump stub: passes control straight on to _Lock.
- `open_file_record_2`: Searches a table for an available file record slot and attempts to initialize it using various bitwise flags and parameters.
- `forward_stream_request`: Retrieves a stream record and forwards a request to the associated data source, returning a failure value if the record is missing or an error occurs.
- `jump_to__read`: Jump stub: passes control straight on to _Read.
- `_Read`: Library stub: loads a library base from a global variable and calls the library function at offset minus 42 of that base, returning its result to the caller.
- `_Write`: Library stub: loads a library base from a global variable and calls the library function at offset minus 48 of that base, returning its result to the caller.
- `_Lock`: Library stub: loads a library base from a global variable and calls the library function at offset minus 84 of that base, returning its result to the caller.
- `_UnLock`: Calls a function pointer stored at an offset from a global variable.
- `_Delay`: Author-named _Delay wrapper that calls the library vector at offset minus 198 with no other work and returns.

### Railroad Tycoon (1990)(MicroProse)[cr Image][h Reaktor].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
