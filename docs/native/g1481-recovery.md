# Game 1481 (combat air patrol (1992)(psygnosis)[cr xpress]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01481). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901184 top-level bytes in 55 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 11 of 11 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 165 of 165 named (0 author symbols), 165 of 165 described (12 by rule, 9 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2789 of 2789 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 42784 |
| code | 7686 |
| data | 820384 |
| padding | 10874 |
| unused | 19456 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Run | 2 | 117 | 2 |
| Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:type | 4 | 27 | 2 |
| Ram-Handler | 2 | 10 | 1 |
| Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:pp | 4 | 8 | 1 |
| ramdrive.device | 3 | 345 | 17 |
| powerpacker.library | 2 | 1594 | 97 |
| CrM.library | 1 | 545 | 41 |
| Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:cap.amg | 5 | 8 | 1 |
| Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Wait | 2 | 117 | 2 |
| Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:bootblock | 1 | 18 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 0 | 0 |

## Cases handled

- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/f18_n8p4.pcc: high-entropy data file (7.80 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/STARTUP.PCC: high-entropy data file (7.82 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/letter2.lbm: high-entropy data file (7.74 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/F18_RR4P.PCC: high-entropy data file (7.80 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/mavrick2.lbm: high-entropy data file (7.61 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/mapnew.lbm: high-entropy data file (7.77 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/flag2.lbm: high-entropy data file (7.61 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/PRINTER.PCC: high-entropy data file (7.76 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/MONITOR.LBM: high-entropy data file (7.70 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/F14_L_4P.PCC: high-entropy data file (7.75 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/caught.lbm: high-entropy data file (7.70 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/letter3.lbm: high-entropy data file (7.76 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/F14_RL4P.PCC: high-entropy data file (7.81 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/f18hang.pcc: high-entropy data file (7.83 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/F14_R_4P.PCC: high-entropy data file (7.75 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/letter1.lbm: high-entropy data file (7.68 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/capcredl.lbm: high-entropy data file (7.72 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/f14_n8p4.pcc: high-entropy data file (7.82 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/debrief.lbm: high-entropy data file (7.69 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/F18ZOOM.PCC: high-entropy data file (7.83 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/f14hang.pcc: high-entropy data file (7.83 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/caplogo.lbm: high-entropy data file (7.71 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/SICKCALL.LBM: high-entropy data file (7.75 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/newops2.lbm: high-entropy data file (7.76 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/PSYLOG16.PCC: high-entropy data file (7.71 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/F18_L_4P.PCC: high-entropy data file (7.74 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/f14zoom1.lbm: high-entropy data file (7.74 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/gfx/F18_R_4P.PCC: high-entropy data file (7.74 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/anims/SHAKE.ANM: high-entropy data file (7.71 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/anims/newnews.anm: high-entropy data file (7.73 bits/byte), classified as packed data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Cap/anims/COPT.ANM: high-entropy data file (7.69 bits/byte), classified as packed data
- reference run failed (Karl 4 Nov 2026): run play-rs-x25k slid through memory (an address: 163 64 KB pages each with >= 29491 distinct PCs executed, 2736097 of its 3347761 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- reference run failed (Karl 4 Nov 2026): run play-rs-x35k slid through memory (an address: 163 64 KB pages each with >= 29491 distinct PCs executed, 2736097 of its 3347761 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:pp: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data
- combat air patrol (1992)(psygnosis)[cr xpress]/Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:cap.amg: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Run

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_sixteen_longs_then_call_2`: Copies 16 longwords from a fixed offset in the data block into the destination named by the stack parameter when one is given, then calls through a stored routine pointer.

### Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:type

- `run_program_entry_and_free`: Program entry point for the type file: it runs its setup routines, calls through a stored function pointer, releases memory through an Exec library call, clears a field and returns.
- `decrement_to_minus_one`: The routine loads a short value from a register, repeatedly decrements it until it reaches -1, and then returns.

### Ram-Handler

- `copy_memory_then_jump_unrecovered`: Program entry for the handler file: it makes an Exec memory copy library call through the library base, then calls through a jump target that could not be recovered.

### Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:pp

- `open_crm_library_then_call`: Retries opening the CrM.library shared library through an Exec call until the open succeeds, then calls a function at a fixed offset in the opened library.

### ramdrive.device

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `open_ramdrive_device_unit`: Opens dos.library, searches the unit list for an entry whose name matches the requested name, and when one is found allocates and initialises that unit's buffer and fields from a template.
- `initialise_ramdrive_unit_buffers`: Takes a unit descriptor, copies its name into a global buffer and up to 16 parameter longwords, computes the unit size, then allocates and initialises the unit's memory and table fields.
- `copy_block_and_initialise_unit`: Copies a block of longwords from the source address to the destination address, then computes the unit size and allocates and initialises the unit's memory and fields, returning zero on completion.
- `initialise_ramdrive_device`: Links a new device structure into the global device list and stores pointers to its internal buffers.
- `release_memory_twice`: Makes two Exec memory-release library calls through the library base held in the global slot, then returns zero.
- `find_masked_entry_and_call`: Searches a linked list for an entry whose flag bits include the requested mask and calls a library routine with the best matching value found, returning that routine's result, or zero when nothing matches.
- `call_or_search_masked_entry`: Uses a count it is given: when it is non-zero it calls a library routine with that value.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `release_interrupt_disable_count`: Decrements a nesting count of interrupt-disable requests and, when it reaches zero, writes the interrupt enable value that restores the master interrupt switch, then returns zero.
- `store_unit_field_return_zero`: Stores the data-register value into a fixed field of the structure at the address register and returns zero.
- `process_device_message`: Forbids multitasking, dispatches on a message's command field through a table of up to 16 routines, stores the result byte back in the message, replies to the message unless its flag says otherwise, then permits multitasking again.
- `return_minus_three_stub`: Returns minus three without doing other work.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 253 into d0, then returns.
- `expire_and_free_unit_node`: Counts down a field in a unit record.
- `traverse_linked_list`: Traverses a linked list to find a matching node, updates a pointer in a structure, and appends a colon to a string.
- `terminate_string_with_colon`: Copies a stored pointer into an output slot, calls a library routine, then walks the string at the address register to its terminating zero, replaces that zero with a colon, and calls Permit.

### powerpacker.library

- `return_minus_one_entry`: Entry point of a shared library file that does nothing except return minus one.
- `open_three_system_libraries`: Stores a pointer in a structure, then opens the dos, graphics and intuition libraries through Exec calls, keeping each returned base in a global slot.
- `increment_counter_and_clear_flag`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `release_structure_when_count_expires`: Counts down a field in a structure.
- `release_resources_or_set_flag`: When the structure's count is zero it releases its library bases and memory through library calls and returns the stored pointer.
- `release_structure_resources`: Releases a structure's resources through library calls, closing three libraries and freeing memory, then returns the pointer stored in the structure.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `call_one_routine_and_return`: Calls one other routine and returns.
- `unpack_powerpacker_stream`: Decodes a PowerPacker-compressed stream by reading bit-coded lengths and offsets and copying literal bytes or earlier output bytes into the destination until the output start is reached.
- `return_after_bit_test`: Tests whether the upper bits of a register are set and returns in either case, doing nothing else.
- `return_immediately`: This library routine is empty.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_bit_buffer_backwards`: Shifts a counted number of bits out of a bit buffer that refills backwards from memory, taking a new longword whenever the buffer empties.
- `refill_backwards_bit_buffer`: Refills a backward-reading bit buffer from memory and shifts out a counted number of bits, carrying the last bit shifted out into the extend flag.
- `copy_back_references_to_output`: Continues the PowerPacker decode loop by decoding match lengths and offsets, copying earlier output bytes or literal bytes into decreasing output addresses, and stopping when the output reaches its start.
- `decode_backward_match_loop`: Runs a backward match loop over PowerPacker data, copying bytes into decreasing output addresses for the stored length, reading further literal or match codes, and returning when the output start is reached.
- `hash_string_with_rotation`: Folds a zero-terminated byte string into a 16-bit value by rotating the running value by an amount taken from the previous byte and adding that byte.
- `compute_rotating_checksum`: Computes a 32-bit checksum over a zero-terminated byte string, rotating the accumulator by one bit and then by sixteen bits for each byte, and returns the accumulator.
- `xor_block_with_key`: XORs each 32-bit word of a block with a key, counting the block in longwords rounded up from its byte length.
- `load_and_relocate_hunk_file`: Loads an AmigaDOS executable's hunk sections into memory by allocating each hunk, copying its contents and applying its 32-bit relocation records, using Exec memory and library calls.
- `process_hunk_relocations`: Iterates through AmigaDOS hunk records to identify and apply relocation data.
- `load_hunk_sections_alternate`: Reads AmigaDOS hunk headers into stack variables, loads each hunk's contents, applies its relocation records, and finishes by releasing temporary memory.
- `load_hunk_sections_and_close`: Loads AmigaDOS hunk sections like the other hunk loaders in this file, then finishes by releasing temporary memory and closing its library.
- `load_hunk_sections_from_header`: Loads AmigaDOS hunk sections using a header block passed in the address register and a size argument, copies the header's trailing fields into the output, and finishes by releasing temporary memory and closing its library.
- `dispatch_hunk_file_records`: Walks the records of an AmigaDOS hunk file and dispatches each one.
- `allocate_and_load_hunk_records`: Walks AmigaDOS hunk records, allocating memory for each code, data or BSS hunk through an Exec allocation call, loading its contents and applying its relocations, then releasing temporary memory.
- `copy_hunk_contents_into_memory`: Copies hunk data from the file buffer into freshly allocated memory through an Exec copy call, then processes the following hunk records and relocations.
- `copy_hunk_block_then_free`: Copies a block of longwords from the source buffer to the destination, releases a temporary buffer through an Exec call, and continues processing the hunk records.
- `process_hunk_records_and_close`: Processes the next AmigaDOS hunk records, allocating and loading each hunk's contents, then releases temporary memory and closes its library when the records end.
- `apply_hunk_relocations`: Applies 32-bit relocation records to the loaded hunks by adding the hunk base address to each longword named in the list.
- `count_down_word_counter`: Steps a 16-bit counter from its current value down past zero, doing no other work, and returns after the counter wraps.
- `tiny_addq_rts`: Small routine of 2 instructions that adds 4 to a1, then returns.
- `finish_hunk_records_and_close`: Handles the end of AmigaDOS hunk records: it copies trailing header fields when a flag is set, releases temporary memory through an Exec call, runs a further library call, and closes its library.
- `write_powerpacker_header_tag`: Writes a four-byte PowerPacker signature, a variant chosen by a flag, through a library write call.
- `initialize_header_data`: This library routine populates a local structure with a signature and a value from a table, then performs a comparison.
- `set_powerpacker_header`: Writes the PowerPacker signature into a header structure, stores a value from a table indexed by the data register in the header, and calls a stored routine.
- `open_library_or_call_fallback`: Calls a library routine that opens a library.
- `call_library_function`: This library routine calls a function via a pointer and returns the result, or calls another function if the first returns zero.
- `call_library_function_2`: This library routine executes a function through a pointer and returns the result, or executes an alternative function if the result is zero.
- `open_reqtools_or_close`: Opens reqtools.library through an Exec call.
- `call_close_library`: Library stub: loads its arguments and the exec library base, calls CloseLibrary and returns its result to the caller.
- `call_stored_hook_return_argument`: Calls a function pointer taken from a global variable with no arguments, then returns its own argument unchanged.
- `get_powerpacker_table_entry`: Inverts the index in the data register, clamps it to at most 7, and returns the longword at that index in a global table.
- `load_powerpacker_file_header`: Copies a file name into a buffer, opens the file through library calls, reads its signature to identify PowerPacker data, handles the encrypted and password variants, and returns zero on success or a negative error code.
- `open_powerpacker_file_with_retry`: Copies a file name into a buffer and retries an open call through a library routine until it succeeds or a flag makes it stop.
- `read_powerpacker_file_header`: Reads a PowerPacker file's signature and header fields from an already opened file, handling the plain, encrypted and password variants, and returns zero on success or a negative error code.
- `verify_powerpacker_header_transfer`: Reads back the file header through a library call and checks the transfer count against the expected length.
- `transfer_powerpacker_header_fields`: Checks a transfer count and, for the four-byte header variant, reads the remaining fields and records the output size when the stored length matches, returning zero; otherwise it returns a negative error code.
- `finish_powerpacker_stream_check`: Calls a stored routine on the buffer, reads four more bytes, and checks the transfer count.
- `check_powerpacker_stream_length`: Calls a stored routine on the signature, reads the next longword, and when the stored length matches it records the unpacked size and returns zero.
- `read_powerpacker_length_fields`: Reads a packed-length longword from the file, checks the transfer count against the expected length, records the output size on success, and returns zero or a negative error code.
- `write_powerpacker_chunk_checked`: Shifts the stored size field, writes a chunk through a library call and, when the transfer count matches the expected length, records the output size and returns zero.
- `adjust_powerpacker_expected_length`: When its argument is minus one it reads a header size longword and steps the expected length back by twelve bytes.
- `check_powerpacker_transfer_count`: Checks the stored signature for the packed-header variant, records the expected length, and writes through a library call.
- `record_powerpacker_expected_length`: Records the expected length in the file state, writes through a library call, and returns zero when the transfer count matches it, or a negative error code otherwise.
- `finish_powerpacker_file_state`: Checks a transfer count against the expected length and, for the variant selected by a flag, records the output size and releases temporary buffers through library calls.
- `release_buffer_return_error`: Releases a temporary buffer through an Exec call and returns the stored error code.
- `release_three_routines_if_set`: When its argument is non-zero, calls three other routines in turn.
- `allocate_powerpacker_decoder_state`: Allocates a decoder state structure and fills it with the packing parameters chosen by a mode value, including block sizes, bit counts and lookup table sizes.
- `initialise_powerpacker_decoder_state`: Fills an existing decoder state structure with the packing parameters chosen by a mode value, then allocates its two working tables, calling a release routine if either allocation fails.
- `initialise_fixed_powerpacker_state`: Fills an existing decoder state structure with fixed packing parameters and allocates its two working tables, calling a release routine when an allocation fails.
- `initialise_powerpacker_state_bits`: Fills an existing decoder state with a bit width taken from the argument and fixed table sizes, then allocates its two working tables, releasing them when an allocation fails.
- `initialise_powerpacker_state_wide`: Fills an existing decoder state with a bit width taken from the argument and larger fixed table sizes, then allocates its two working tables, releasing them if an allocation fails.
- `initialise_powerpacker_state_large`: Fills an existing decoder state with a bit width taken from the argument and the largest fixed table sizes, then allocates its two working tables, releasing them if an allocation fails.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `store_powerpacker_buffer_bounds`: Stores a start pointer and length in a structure, records the end address as their sum, then calls another routine and returns.
- `find_powerpacker_matches`: Clears two work tables, then scans the input for repeated byte sequences, storing the best match length and offset for each position and calling a progress callback after every 512 bytes.
- `find_powerpacker_matches_from_stack`: Records the start position from the stack argument, then runs the repeated-sequence search, storing the best match length and offset for each position and calling a progress callback after every 512 bytes.
- `find_powerpacker_matches_at_register`: Runs the repeated-sequence search from the position in the address register, storing the best match length and offset for each position and calling a progress callback after every 512 bytes.
- `resume_powerpacker_match_search`: Resumes the repeated-sequence search from a saved position, storing the best match length and offset for each position and reporting progress through a callback.
- `emit_powerpacker_token`: Writes one packed token to the output: a literal is written through a bit routine, and a match is written as length and offset codes with its table entries cleared, then the output position advances.
- `emit_powerpacker_token_mode_one`: Writes one packed token for the mode-one format, clearing the match table entries it uses and returning the output position.
- `finish_powerpacker_token_stream`: Writes the remaining packed tokens of the block, clearing the match tables for the rest of the block, and returns the final output status.
- `emit_powerpacker_tokens_from_stack`: Writes packed tokens for the block using the stack argument as the start position, clearing the table entries of each emitted match until the block is complete.
- `emit_powerpacker_tokens_to_pointer`: Writes packed tokens for the block into the output pointer given as its first argument, clearing the table entries of each emitted match, and returns the status with the final output position.
- `emit_powerpacker_literal_run`: Writes a run of literal tokens from the current position, clearing their table entries, and stops when the output reaches the block end.
- `finish_powerpacker_token_block`: Writes the remaining tokens of the block and then the final control value through a library routine, returning the output position and status.
- `emit_powerpacker_match_run`: Writes a run of match tokens into the output, clearing table entries and reporting progress through a callback, then returns the status.
- `emit_powerpacker_literal_or_match`: Writes the next packed token for the block, choosing a literal or a match, clearing the matched table entries, and returns the output status.
- `emit_powerpacker_tokens_with_search`: Writes packed tokens while searching for the next match in the block, storing lengths and offsets as it goes, and returns when the block is complete.
- `emit_powerpacker_literal_token`: Writes one literal token and advances the output, then finishes the block when no input remains, returning the output status.
- `emit_powerpacker_match_token`: Writes a match token from the search results, advancing the output, and finishes the block when the output reaches the block end.
- `emit_powerpacker_zero_byte_token`: Clears one table entry and advances the output, then finishes the block when the output reaches its end, returning the status.
- `emit_powerpacker_packed_run`: Writes a run of packed tokens from the current position, choosing literal or match coding by the packing mode, and returns the output status once the block is complete.
- `emit_powerpacker_tail_token`: Writes a final packed token and returns the output status when the block is finished.
- `emit_powerpacker_zero_run`: Clears a run of table entries from the current position, writes the zero-filled remainder of the output, then returns the status.
- `decode_packed_bit_value`: Decodes one value from the packed bit stream held in the decompressor state.
- `fill_zeros_wrapping_buffer`: Writes zero words into a buffer while counting down a length, and wraps back to the stored buffer start when a width counter runs out.
- `fill_zeros_and_copy_block`: Clears a run of words in a wrapping buffer, then decodes a bit value and copies a block with the exec CopyMemQuick call when the output position passes the stored limit.
- `record_word_offsets_in_tables`: Walks a range of 16-bit words and looks up each word's stored previous position in a table.
- `record_byte_offsets_in_tables`: Walks a range of bytes, uses each byte as an index into a table of stored pointers, and computes the distance from the stored position.
- `record_byte_offset_one_table`: Walks a range of bytes, uses each byte as an index into a pointer table, and computes the distance to the previous occurrence.
- `dispatch_calls_by_run_length`: Compares a length counter against four.
- `chain_two_library_calls`: Calls two other library routines in sequence and returns.
- `pack_bits_into_words`: Shifts bits one at a time from a source word into an accumulator, and stores the accumulator as a word whenever its top bit is set.
- `allocate_sized_block`: Allocates memory through the library's allocator vector with four extra bytes, stores the requested size in the first longword, and returns a pointer just past that header.
- `free_memory_if_nonnull`: If the pointer in the A1 register is not zero, it calls the library's free routine through a vector stored in the library base.

### CrM.library

- `tiny_moveq_rts_3`: Small routine of 2 instructions that loads 0 into d0, then returns.
- `open_library_save_entry`: Opens a library by name through the exec vector and keeps the returned base.
- `save_patched_entry_target`: Calls a library routine, then reads the saved entry bytes from the record.
- `decode_marked_branch_target`: Checks whether a saved entry stub begins with a particular marker byte and a branch-always opcode.
- `store_branch_stub_target`: Stores the second byte of a stub into the record and, when the stub is a branch-always opcode, computes and stores the branch target.
- `increment_counter_and_clear_flag_2`: The routine increments a 16-bit counter stored at an offset from a pointer in A6 and clears a single bit in a byte at another offset from the same pointer.
- `restore_stub_after_countdown`: Decrements a countdown in the record and, once it reaches zero with a flag bit set, checks whether the stub still jumps to one of two known routines.
- `return_zero_handler`: Returns zero in the result register and does nothing else.
- `restore_stub_release_library`: If the saved stub still jumps to one of two known routines, it writes back the saved bytes, then clears the cache, closes the library, frees the memory block and removes the library entry.
- `set_countdown_flag_bit`: Sets bit three of a flag byte stored in the library record.
- `find_crm_library_base`: Looks up a library base through the system and checks that memory at its entry contains the text CrM.library together with a register-save opcode.
- `tiny_moveq`: Small routine of 1 instruction that loads 0 into d0.
- `match_crm_library_signature`: Looks up a library base through the system and checks the memory at the returned entry for the text CrM.library and a register-save opcode.
- `decrement_countdown_clear_cache`: Decrements a 16-bit countdown in the library record and calls a library routine to clear the processor cache.
- `advance_countdown_then_callback`: Decrements a byte countdown in the record and returns early while it is not yet zero.
- `decrement_value_loop`: This library routine decrements a value in a loop until it reaches negative one.
- `copy_nonzero_count_blocks`: Copies a table of 16-bit counts from the record into a reversed working area.
- `tiny_move_rts`: Small routine of 2 instructions that copies d3 to d6, then returns.
- `decode_lz_copy_or_literals`: Decodes one token from the bit stream: either a run of literal bytes produced by a helper, or a match whose length comes from a set of bit tests and whose distance comes from a masked table lookup.
- `decode_match_block_copy`: Reads a length from bit-tested selectors and a distance from a masked table, then copies a block of earlier output forward to the current position.
- `decode_selected_match_copy`: Chooses a length from a sequence of bit tests and adds a value from a lookup, then copies that many bytes backward within the output buffer.
- `decode_literal_then_match`: Writes literal bytes obtained from a helper when the bit test is clear, otherwise computes a length and copies backward from earlier output.
- `decode_match_small_selector`: Selects a small base length from a short chain of bit tests, adds a decoded value, and copies that many bytes backward from earlier output.
- `decode_bitstream_match_table`: Decodes a match from the bit stream by reading a masked table value, computes the source position from the output pointer, and copies the block backward.
- `copy_long_run_from_stream`: Decodes a run length from a bit-tested selector and writes that many bytes from a helper into the output buffer, moving backward.
- `copy_short_run_backwards`: Uses a fixed base length when a flag bit is set, adds a decoded value, and copies that many bytes backward from the output pointer.
- `select_halved_or_input_word`: Returns the word in the D6 register shifted right by one when the counter in D7 equals one, otherwise returns the word passed in D0 unchanged.
- `tiny_lsr_rts`: Small routine of 2 instructions that shifts d6 right by 1, then returns.
- `lookup_table_masked_value`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `return_from_subroutine_2`: This routine is an empty function that performs no operations before returning.
- `allocate_use_free_block`: Allocates a memory block through the library's allocator vector.
- `build_code_lookup_table`: Clears a 16-entry counter table, reads code lengths from the bit stream and fills a 16-bit lookup table for each code.
- `decode_symbol_bit_by_bit`: Reads bits one at a time from a bit buffer into an accumulated code, and stops as soon as the code is no larger than a limit from the decode table.
- `decode_symbol_by_limits`: Reads bits one at a time into an accumulator, testing it against the code limit for each length from 1 to 16.
- `lookup_table_masked_value_2`: The routine reads a 16-bit value from a table using an index derived from the input word, and returns that value masked with an input mask.
- `fill_two_tables`: The routine calls another routine to obtain a count, then fills a short table with that many values returned by the same routine, accumulating the values.
- `fill_accumulated_code_tables`: Sets the first table entry to zero, then runs fifteen steps that accumulate counts from a source array into running sums.
- `return_empty_routine`: Contains nothing but a return, so it changes no memory or registers.
- `classify_crm_file_signature`: Checks that the input pointer is word aligned, reads the first four bytes, and maps the tags CrM!, CrM2, Crm! and Crm2 to codes 1, 2, 16 and 17.
- `apply_delta_sum_when_flagged`: If the input is word aligned and non-zero, it reads a count and a start value from the record.
- `accumulate_bytes_running_sum`: Calls a stored routine, then adds each following byte onto the previous byte in place, repeating for a count held in D1.

### Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:cap.amg

- `open_crm_library_retry`: Program entry that repeatedly tries to open the CrM.library by name through the old open-library call until it gets a base.

### Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:Wait

- `find_max_then_fill_buffer`: Walks a linked list of entries to find the largest value in a field, then allocates a buffer if the allocation succeeds.
- `copy_sixteen_longs_then_call`: If the destination pointer is not zero, it copies sixteen longwords from a fixed area into it.

### Combat Air Patrol (1992)(Psygnosis)[cr Xpress].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
