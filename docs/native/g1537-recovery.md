# Game 1537 (ghouls 'n ghosts (1989)(u. s. gold)[cr family]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01537). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (944848 top-level bytes in 70 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 19 of 19 units identical (bytes and relocations), 304 raw-word lines |
| Functions named and described | pass, 318 of 318 named (14 author symbols), 318 of 318 described (17 by rule, 51 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1200 of 1206 (99.50%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5658 of 5674 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 130501 |
| code | 23418 |
| data | 734102 |
| padding | 14331 |
| unused | 42496 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| picend | 1 | 329 | 21 |
| picshow | 1 | 440 | 20 |
| INTRO | 1 | 26 | 1 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:cls | 2 | 20 | 1 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:1 | 1 | 183 | 15 |
| Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 25 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 394 | 6 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 26 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 6 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 89 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 12 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2417 | 141 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1253 | 84 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 19 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 409 | 8 |

## Cases handled

- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:PANEL.SPR: high-entropy data file (7.66 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:HIGH.T1: high-entropy data file (7.77 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:DEVIL.SPR: high-entropy data file (7.61 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LEVEL4.IFF: high-entropy data file (7.83 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LEVEL4.SPR: high-entropy data file (7.65 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:TITLE.T1: high-entropy data file (7.72 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:CERBERUS.SPR: high-entropy data file (7.74 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LEVEL202.IFF: high-entropy data file (7.81 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LEVEL101.IFF: high-entropy data file (7.82 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:COUNT.T1: high-entropy data file (7.63 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LUCIFER.IFF: high-entropy data file (7.80 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:ARTHUR1.SPR: high-entropy data file (7.83 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LEVEL5.IFF: high-entropy data file (7.81 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LEVEL5.SPR: high-entropy data file (7.68 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LEVEL2.T1: high-entropy data file (7.75 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:level3.t1: high-entropy data file (7.70 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LEVEL3.IFF: high-entropy data file (7.76 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:CLOUD.SPR: high-entropy data file (7.63 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:SKELETON.SPR: high-entropy data file (7.63 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:level4.t1: high-entropy data file (7.68 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LEVEL5.T1: high-entropy data file (7.78 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LEVEL102.IFF: high-entropy data file (7.82 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:LEVEL201.IFF: high-entropy data file (7.83 bits/byte), classified as packed data
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:CHEST.SPR: high-entropy data file (7.62 bits/byte), classified as packed data
- runtime segment an address (ext): 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 60 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 7 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 51 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f100): 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f100): 684 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 69 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f100): 19 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 211 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying code at an address (51 bytes change between runtime segment an address (ext) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (39 bytes change between runtime segment an address and runtime segment an address (ext)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (56 bytes change between runtime segment an address and runtime segment an address (ext)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address and runtime segment an address (ext)): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (44 bytes change between runtime segment an address (f100) and runtime segment an address): both forms recovered; writer: not identified statically (indirect store)
- self-modifying code at an address (58 bytes change between runtime segment an address and runtime segment an address (f100)): both forms recovered; writer: not identified statically (indirect store)
- ghouls 'n ghosts (1989)(u. s. gold)[cr family]/Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:1: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- INTRO

## Copy protection found

- `program_disk_read_registers` (runtime segment an address (ext)): Programs the disk hardware for a read: writes a disk pointer, a sync word and a control value into the disk registers, sets a global to a fixed value, and enables the disk DMA with a transfer length of two. Kind: disk hardware/sync check. Pass path: the code following the call (specifically the logic involving the manipulation of d0 after the call).
- `poll_disk_transfer_completion` (runtime segment an address (ext)): Repeats a disk transfer step while two stored positions differ. Kind: disk hardware status/sync check. Pass path: the branch where d0 is not zero (the instruction after the call is 'tst.l d0' followed by 'beq.w an address', implying success is when d0 != 0).
- `clear_selected_control_bit` (runtime segment an address (ext)): Ors a byte with the constant 125, clears one bit whose index comes from a stored value plus three, and writes the result to a global that drives a control port. Kind: bitmask manipulation for hardware/status flag. Pass path: the branch where the bit test (btst) succeeds or the code continues linearly if the bit is expected to be set.
- `write_drive_control_until_ready` (runtime segment an address (ext)): Builds a control-port byte by clearing one bit chosen by a stored index, then sets or clears bit two depending on a flag, and writes it to a global. Kind: disk hardware/timing synchronization check. Pass path: beq.w an address.
- `clear_control_bit_then_delay` (runtime segment an address (ext)): Clears one control bit chosen by a stored index, waits while bit five stays set, then runs a fixed countdown delay. Kind: disk hardware/timing check. Pass path: the code continues to the next instruction after the bsr.w calls.
- `fill_entries_with_stepped_values` (runtime segment an address (ext)): Fills 128 three-word entries: the first two words are set to minus 128, and the third counts upward in steps of seven. Kind: checksum/data integrity trap. Pass path: the loop completes normally and execution continues to the next instruction (rts).

## Functions


### picend

- `start_entry_open_libraries`: Program entry that saves register state, opens the system libraries it needs, and stores their base addresses in shared globals.
- `write_null_terminated_text`: Measures a null-terminated text string and, when it is not empty, calls a system library routine to write it out.
- `read_file_via_dos_library`: Locks and examines a named file, opens it, and reads its contents with the system file routines, then closes and unlocks it.
- `close_file_and_finish`: Closes a file through the system file routines, then calls one routine and returns.
- `allocate_tracked_memory_block`: Allocates a memory block through the system memory routine and, if the allocation succeeds, stores the requested size plus four in its first longword.
- `free_memory_block_if_set`: Frees a memory block through the system memory routine when the pointer passed in is not null.
- `terminate_text_buffer`: When a flag in a shared structure is set, records the buffer pointer and writes a zero byte at the end of the buffer, terminating the string.
- `read_next_text_token`: Advances through a shared text pointer, skipping spaces, and returns the next token.
- `call_set_signal_via_exec`: Forwards control to the exec SetSignal library function through the exec base pointer and returns the two result registers unchanged.
- `open_dos_graphics_intuition_libraries`: Opens the dos, graphics and intuition libraries through exec, fetches the dos input and output handles, finds the current task, and closes libraries already opened when a later step fails.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `uppercase_filename_and_lookup`: Checks the command argument, upper-cases the filename after the last path separator, and runs a library lookup on it.
- `process_file_path`: Checks a string for a null or question mark, and if not, iterates backward through the string to convert uppercase letters to lowercase until a directory separator or colon is found.
- `process_file_path_2`: Validates a string and performs a backward scan to convert specific characters to lowercase.
- `process_file_path_3`: Scans a string backward to modify characters and calls a routine through a global pointer.
- `process_file_path_4`: Iterates through a string to perform lowercase conversion on characters between backtick and curly brace.
- `process_file_path_5`: Performs a backward search on a string to find separators and modifies characters to lowercase.
- `process_file_path_6`: Checks a string for specific termination characters and converts characters to lowercase.
- `process_file_path_7`: Traverses a string backward to convert letters to lowercase and calls a function via a pointer in a global variable.
- `process_file_path_8`: Processes a string by converting characters to lowercase and searching for path separators.
- `process_file_path_logic`: Scans a string for path separators, converts characters to lowercase, and manages system ports and screens based on the results.

### picshow

- `entry_open_required_libraries`: Program entry point that opens the exec library by name and then, through the exec base, obtains further library and screen pointers.
- `write_string_if_nonempty`: Measures the null-terminated string in the buffer and, when it is longer than one character and the length flag is set, writes it through dos.Write.
- `lock_examine_and_read_file`: Locks and examines a named file, opens and reads it through dos when the examined entry qualifies, then closes it and releases the lock.
- `allocate_memory_with_size_header`: Allocates a memory block through exec.AllocMem and, on success, stores the requested size plus four bytes as a header in the block's first longword.
- `free_memory_block_if_set_2`: Frees a memory block through exec.FreeMem when the pointer is non-null and leaves the registers unchanged otherwise.
- `terminate_string_at_offset`: When a stored structure field is nonzero, records the current pointer and writes a zero byte at the position given by the offset register, ending a string there.
- `split_next_argument_token`: Scans the saved command line from its current position, skipping spaces, and returns the next argument.
- `call_set_signal`: Calls a function through a pointer located at an offset from a global variable and returns the concatenated result.
- `calculate_string_length`: Iterates through a string to calculate its length by counting characters until a null terminator is reached.
- `load_ilbm_picture_bitplanes`: Parses an IFF picture's header, palette and body chunks, applies the palette colours, and unpacks the run-length compressed body into the bitplanes of a new bitmap.
- `parse_iff_resource`: Parses an IFF-formatted data structure from a memory buffer to extract specific chunks.
- `parse_resource_file`: Parses a custom file format by searching for specific four-byte magic markers to locate data chunks.
- `parse_resource_file_2`: Parses a custom data file format by searching for specific four-byte magic markers.
- `parse_resource_file_3`: Parses a custom data file format by searching for specific magic markers to locate resource chunks.
- `parse_text_resource_block`: Parses a structured text data block by searching for specific magic markers and metadata tags.
- `parse_resource_file_4`: Parses a custom resource file format by searching for specific four-character magic identifiers.
- `parse_resource_file_5`: Parses a custom resource file format by searching for specific magic markers.
- `parse_amiga_iff_resource`: Parses an IFF (Interchange File Format) data structure to extract specific chunks such as BODY and PICA. It iterates through the file chunks to locate metadata and then processes the contained data buffers into memory.
- `parse_amiga_iff_resource_2`: Parses an IFF file by searching for specific chunk identifiers.
- `load_custom_bitmap_file`: Parses a custom file format containing bitmap data by searching for specific magic markers.

### INTRO

- `decode_packed_intro_and_start`: Waits for the blitter to go idle, then unpacks a run-length packed block backward into memory, with negative counts repeating a byte and positive counts copying literal bytes.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:cls

- `write_output_via_dos_library`: Opens the dos library, takes its output handle, and writes through it; when the write reports an error it calls a further dos function and returns that result, otherwise it returns zero.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:1

- `Relocate`: Three-instruction program entry that calls one other routine.
- `init_dma_colour_sprite_regs`: Writes the background colour, the sprite-zero pointer and the DMA and interrupt enable registers, then calls the start routine.
- `start`: Copies the depacker routine to low memory and a second code block into place, clears a run of sprite-pointer words, sets the blitter masks and bitplane modulo registers, then calls the relocated depacker.
- `depack`: Checks the first longword for the ICE packer signature and, if it matches, runs the ICE decoding routines and copies the unpacked bytes to their destination.
- `exit`: Returns the value found at the top of the stack without other work.
- `ice2_08`: Main ICE decoding loop: while output remains, it copies a literal byte run or, when the flag bit says so, decodes a length and distance pair and copies that many earlier bytes backward into the output.
- `ice2_0c`: Counts a short register down from three to zero as a timing delay, then returns.
- `ice2_0e`: Shifts bits from the compressed stream into the carry bit for a set count, refilling from the next stream word when the buffer runs out.
- `ice2_0f`: Refills the bit buffer from the compressed stream one word at a time, shifting out bits until the requested count is reached.
- `ice2_10`: Tests whether the bit buffer still holds bits and whether the stream pointer is aligned, then returns.
- `ice2_11`: Tests the low bit of the stream pointer and returns without changing state.
- `ice2_12`: This routine is empty and performs no operations before returning to its caller.
- `ice2_13`: Shifts the next bit out of the bit buffer for a loop count, loading a new stream word when the buffer is empty.
- `ice2_16`: Decodes the next compressed token by reading flag and bit-count codes from the bit stream, then either sets a copy length from a lookup table or hands over to the byte-copy routine.
- `ice2_1a`: Copies a run of previously decoded bytes backward to the output, using a distance read from the bit stream, and loops until the count is used up.

### Ghouls 'N Ghosts (1989)(U. S. Gold)[cr Family].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialise_runtime_state_and_pause`: Runs a sequence of setup calls that store addresses into the runtime segment, touches the CIA A port register that carries the fire-button and disk status bits, then spins in a countdown delay before returning.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_table_with_stride_addresses`: Fills a table of longwords with addresses starting from a fixed base and stepping by a fixed size per entry, with the entry count taken from a stored value.
- `build_sign_bit_lookup_table`: Fills a 256-byte lookup table where each entry is built from the index bits, with the top bit set by threshold tests on the index and a parity bit that alternates from entry to entry.
- `offset_coordinate_list_into_records`: Walks a coordinate list ending with the value minus-one, adds a base x and y to each pair, and stores the results at the start of each record.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 83 into d0, then merges 0 into d0.
- `offset_coordinate_list_second_pair`: Walks the same coordinate list as its neighbour, adding base x and y to each pair and storing the results at a later offset inside each record.
- `dispatch_record_handlers_table`: Iterates over fixed-size records and calls the handler whose table index is stored in each record when that index is nonzero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_bad_instruction_data`: Writes a mask that clears all DMA channel bits and stores a few addresses, then runs into undecodable bytes, so the rest of the decompiled body is unreliable.
- `stub_bad_instruction_data_2`: Stores a few address words and then runs into undecodable bytes, so only the first stores are meaningful.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `set_shared_byte_to_ones`: Writes an all-ones value to one shared byte and returns.
- `handle_interrupt_request_flags`: Checks two flag bits in a shared byte: when the first is set it records that and returns; when the second is set it records it and calls a series of routines.
- `clear_interrupt_request_flag`: Clears the top bit of a shared interrupt value, leaves the other bits unchanged, and returns its input value unchanged.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enable_interrupts_and_dma_masks`: Writes several interrupt-enable and DMA-enable masks to the custom chip registers and sets a CIA A interrupt control value, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tick_shared_counter`: Calls a series of routines, most of them empty, then increments a shared counter before returning.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `reset_input_state_and_setup`: When a shared flag bit is clear, toggles a status bit, zeroes the shared byte, stores addresses into the runtime segment and calls setup routines.
- `poll_joystick_port_flags`: Calls two routines, then sets a shared status bit when the flag at a fixed bit is clear.
- `decode_joystick_direction_bits`: Converts a joystick-port register value into a four-bit direction mask, deriving each bit from XOR tests on adjacent bit pairs.
- `decode_keyboard_key_code`: Stores a key-code byte from keyboard input, inverts and rotates it into the key code, then waits on the serial interrupt flag before resetting the CIA A interrupt control value.
- `handshake_keyboard_serial_line`: Stores the keyboard serial data, inverts and rotates it into a key code, then waits on the serial timer interrupt flag before clearing the serial control registers.
- `decrement_shared_counter_if_set`: Decrements a shared byte counter when it is nonzero and returns; otherwise it returns without change.
- `copy_words_into_strided_table`: Copies thirty-two words from a source array into a destination at a stride of four bytes, storing each one in the second word of its slot.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_and_wait_for_blitter`: Waits for the blitter to be idle, writes the interrupt enable mask that clears all channels, calls a fixed routine and then a run of other routines.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_bad_instruction_data_3`: Checks a few pointers and writes some words into the runtime segment, then runs into undecodable bytes, so the rest of the body is unreliable.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_interrupts_and_run_setup`: Clears the interrupt enables, waits for the blitter to go idle, then runs seven setup routines, including the copper-list start, and returns its first argument unchanged.
- `forward_to_fixed_routine`: Calls one fixed routine and returns.
- `forward_to_one_routine`: Calls one other routine and returns, with nothing else done.
- `clear_interrupts_start_copper_list`: Waits for the blitter to be idle, clears the interrupt enables, stores its input word in shared state, then calls the copper-list start routine and four other routines.
- `scan_record_table_for_letter`: Walks a table of records fifty bytes apart; for records of type six it scans a text buffer for the letter o and advances past it.
- `scan_record_table_for_letter_2`: Walks the same record table as its neighbour, scanning text for the letter o on type-six records until it reaches a terminator record.
- `scan_record_table_for_letter_3`: Walks the same record table as its neighbours and scans the text buffer for the letter o on type-six records, stopping at a terminator record.
- `dispatch_record_table_entries`: Walks a table of fifty-byte records, storing each non-negative code in shared state and calling a handler from a dispatch table, stopping at the first negative record.
- `run_record_script_dispatch`: Runs through a record table, calling a handler for each entry from a dispatch table until a terminator stops it.
- `check_fire_button_flag`: Checks a shared flag bit that reflects the fire-button and disk status and, when the bit is clear, spins while it is set.
- `initialise_sprite_record_table`: Writes a fixed value into one field of a hundred and fifty-seven twenty-four-byte records and clears or sets several other fields in each one.
- `run_blitter_operation`: Waits for the blitter to be idle, programs its control, modulo, destination pointer and size registers, then waits for it to finish.
- `update_display_records_at_raster`: Sets a status bit from a record field, then loops over a table of records writing values into them.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `copy_table_words_into_records`: Copies a column of words from a source table, chosen by the caller's index and a fixed row size, into one field of consecutive twenty-four-byte records.
- `copy_table_words_to_slot`: Copies a column of words from a source table, chosen by the caller's index and a fixed row size, into a second field of consecutive twenty-four-byte records.
- `set_bitplane_pointers_and_modulo`: Writes two bitplane address pointers stepping by forty bytes per row into a list, sets a fixed width value in each of a hundred and fifty-seven records, and stores three addresses for the display.
- `compute_aligned_rectangle_masks`: Aligns a rectangle's coordinates to word boundaries by setting their low bits, then calls nine routines that act on the aligned region.
- `set_display_dimensions_then_draw`: Stores a row stride of forty, a width of three hundred and twenty, a height of a hundred and fifty-eight and a source address into shared state, then calls three routines.
- `set_region_dimensions_then_copy`: Stores a region height, width, source and destination addresses into shared state, then calls three routines.
- `select_table_entry_and_draw`: Indexes a table of eight-byte entries by the caller's index, loads the entry's word fields into shared state, and calls seven routines.
- `advance_shared_position_then_draw`: Advances a shared position by two hundred and forty-five, clears a counter, sets a fixed value and calls two routines.
- `dispatch_table_entry_actions`: Reads a value from a table to determine a control type, then performs a specific action such as jumping to a new address, calculating a pointer based on table offsets, or executing a nested loop to copy memory blocks.
- `advance_row_offset_then_draw`: Adds forty to a shared row offset, sets two fixed values and calls two routines.
- `advance_column_offset_then_draw`: Advances a shared offset by one hundred and fifty-eight, sets a value of one hundred and fifty-nine and calls two routines.
- `advance_offset_and_select_entry`: Advances a shared offset by seventy-eight, looks up an eight-byte table entry by the caller's index and calls six routines to act on it.
- `install_handler_and_run_steps`: Stores the address of a handler routine in shared state, calls another routine, sets three shared values and calls two more routines.
- `load_record_fields_into_state`: Copies two fields from a record into shared state, computes an address from a third field, and calls two routines.
- `compute_record_addresses_and_fill`: Computes two addresses from record fields and a fixed table base, stores them in shared state, sets a bit in a memory cell and calls two routines.
- `configure_record_table_and_draw`: Sets several shared addresses and fields, then fills a record table through nine routines, writing single-bit flags into memory cells as it goes.
- `check_table_entry_then_update`: Looks up a longword through a table pointer and, when it is nonzero, calls two routines to update the record.
- `run_threshold_scan_loop`: Repeatedly calls three routines per pass, comparing a returned value against a shared limit to choose between two follow-up routines, and loops until a returned word reaches zero.
- `draw_nine_glyph_rows`: Looks up character data in a font table and copies nine rows of glyph data into a destination.
- `draw_nineteen_glyph_rows`: Looks up character data in a font table and copies nineteen rows of glyph data into a destination.
- `draw_thirtynine_glyph_rows`: Looks up character data in a font table and copies thirty-nine rows of glyph data into a destination.
- `copy_wide_tile_rows_2`: Copies a block of rows from a source table entry into a destination using a wider row width.
- `copy_tall_tile_rows_2`: Copies a taller block of rows from a source table entry into a destination.
- `iterate_table_entries_with_limit_check`: Iterates through a sequence of routine calls, comparing a returned value against a global limit.
- `scan_until_limit_then_set`: Loops through a sequence of routine calls and compares a returned value against a shared limit.
- `return_empty_stub`: Empty routine that returns at once.
- `tiny_ori_2`: Small routine of 1 instruction that merges 180 into d2.
- `tiny_ori_3`: Small routine of 1 instruction that merges 8 into (a0).
- `copy_words_into_records`: Copies words from a source buffer into one field of consecutive records, wrapping back to the start of the buffer and stopping at a fixed end address.
- `advance_word_pointer_by_two`: Advances a shared word pointer by two bytes and returns; the comparison that follows has no effect.
- `copy_words_into_mirrored_records`: Copies words from a source buffer into one field of records stepping forward and into another field stepping backward, wrapping the source at its end.
- `copy_words_then_mask_records`: Copies a hundred and fifty-six words into one field of consecutive records, then computes a masked value from shared state and writes it into a second field of the records.
- `step_pointers_by_table_offsets`: Advances two shared pointers by the offsets stored at them, unless they have reached their end values, then calls two routines and returns their result.
- `advance_pointer_by_table_offsets`: Advances a shared pointer by two offsets read from the pointer's own table, calling two routines around the update.
- `write_record_pair_and_advance`: If the shared pointer has not reached the end, writes the same value into two fields of the record it points to and moves on to the next record.
- `write_record_pair_and_retreat`: If the shared pointer has not reached the start, writes a negative value into two fields of the record it points to and moves back one record.
- `increment_counter_until_limit`: Increments a shared counter and calls one routine until the counter reaches its limit, then returns.
- `step_counter_down_by_two`: Subtracts two from a shared counter while it is not two, and calls one routine after each step.
- `reset_record_fields_and_compute`: Sets two fields of each of a hundred and fifty-seven records to a fixed negative value, then computes a division-based value from shared state.
- `reset_or_scale_shared_value`: When a shared value is zero it resets it to a hundred and fifty-eight and calls a routine.
- `clamp_and_double_shared_value`: Calls a routine, clamps a shared value above a limit, halves it with sign, then doubles it twice and returns.
- `increment_counter_and_call_two`: Increments a shared counter and then calls two other routines.
- `increment_global_variable`: increments a global variable by one.
- `set_global_variable`: stores a value into a global variable.
- `execute_blitter_sequence`: increments a global variable and calls another routine.
- `run_blitter_sequence`: Calls a routine that increments a global variable and then calls a routine to set blitter masks and modulo values.
- `increment_counter_and_call_four`: Increments a shared counter and then calls four other routines in turn.
- `set_blitter_masks_then_run`: Waits for the blitter to be idle, sets its first-word and last-word masks and modulo values, then calls a routine three times.
- `compute_nibble_offset_address`: Builds a byte from the inverted low nibble of a shared value, derives an address from it using row stride values, and calls one routine.
- `fill_record_fields_by_range`: Sets two fields of a range of records to a fixed negative value, then sets the remaining records' fields to a fixed positive value, starting from the caller's index.
- `advance_pointer_by_table_offsets_2`: Advances a shared pointer by two offsets read from the pointer's own table, calling one routine as it goes.
- `step_pointers_by_table_offsets_2`: Advances a shared pointer by the offsets stored at it, unless it has reached one of two end values.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `reset_shared_counter_and_constants`: Stores two fixed values into fixed memory addresses and clears a shared counter.
- `call_eight_routines`: Calls eight other routines in turn and returns.
- `clear_byte_range`: Zeroes a run of bytes starting at the address in the address register, for the count given in the data register plus one.
- `initialise_state_and_draw_records`: Initialises a large set of shared fields and table values, then calls sixteen routines that fill and update the display records.
- `tiny_move_move`: Small routine of 2 instructions that copies -(a0) to d0, then copies d0 to d0.
- `load_record_values_from_table`: Reads four words from a source record, stores three of them doubled into shared state, and uses the fourth as an index into a table to look up a further value.
- `return_empty_stub_2`: Empty routine that returns at once.
- `set_shared_address_and_mode`: Stores a fixed address into shared state, sets a mode value of two, and returns.
- `wrap_angle_pair_modulo`: Adds two input deltas to a pair of stored 16-bit values and wraps each one into the range of a fixed period of seven hundred and twenty, adding the period back when the sign bit is set.
- `copy_tile_rows_from_table`: Copies a block of rows from a source table entry, selected by the caller's index, into the destination, skipping the copy when the entry or the shared buffer is null.
- `clear_word_buffer`: Zeroes a buffer of twenty-five thousand two hundred and eighty words, one word at a time, starting at the address in the address register.
- `copy_wide_tile_rows`: Copies a block of rows from a source table entry into the destination with a wider row width, skipping the copy when the entry or the shared buffer is null.
- `copy_tall_tile_rows`: Copies a taller block of rows from a source table entry into the destination, skipping the copy when the entry or the shared buffer is null.
- `draw_text_glyph_rows`: Looks up each character of a text string in a font table by subtracting the first printable code, then copies nine rows of glyph data into the output.
- `draw_text_glyph_rows_2`: Looks up each character in a font table by subtracting the first printable code and copies nine rows of glyph data into the output, without the setup step of its neighbour.
- `draw_text_glyph_rows_3`: Looks up each character of a text string in a font table by subtracting the first printable code, then copies nineteen rows of glyph data into the output.
- `draw_text_glyph_rows_4`: Looks up each character in a font table by subtracting the first printable code and copies nineteen rows of glyph data into the output.
- `draw_text_glyph_rows_5`: Looks up each character of a text string in a font table by subtracting the first printable code, then copies thirty-nine rows of glyph data into the output.
- `draw_text_glyph_rows_6`: Looks up each character in a font table by subtracting the first printable code and copies thirty-nine rows of glyph data into the output.
- `copy_data_table`: copies data from a source table to a destination buffer using nested loops, or zeroes a large block of memory if conditions are not met.
- `copy_display_data`: Copies data from a source table to a destination buffer using nested loops, or zeroes a large block of memory if a condition is not met.
- `copy_sprite_data_blocks`: If a specific global pointer and flag are non-zero, the routine performs a triple-nested loop to copy multiple blocks of data from a table into a destination buffer.
- `clear_word_run`: Zeroes a run of three thousand one hundred and sixty words starting at the address in the address register.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `open_library_and_start_copper`: Opens a library through the exec OldOpenLibrary function, stores one of its fields, then sets the copper list address, starts the copper and writes the DMA control mask.
- `enable_vertical_blank_interrupt`: Stores an interrupt-related address in shared state and writes the interrupt enable mask for the vertical blank interrupt.
- `disable_all_interrupt_sources`: Sets the top bit of a shared interrupt word, then writes a mask that clears every interrupt-enable bit to the interrupt registers, and returns.
- `dispatch_interrupt_request_state`: Checks a shared value through several branches and calls up to nine routines according to the result, touching the interrupt request register on the way.
- `subtract_graded_byte_columns`: Subtracts one from the first column of twelve bytes spaced forty bytes apart, then two from the next column, stepping through the buffer.
- `decrement_memory_table`: performs a nested loop to subtract incrementing values from bytes in a memory table.
- `tiny_ori_4`: Small routine of 1 instruction that merges 196 into d0.
- `decrement_memory_table_values`: Iterates through a structured table of data, decrementing bytes at specific offsets within each entry by an increasing value across multiple passes.
- `decrement_memory_table_values_2`: Iterates through a structured table of data, decrementing bytes at specific offsets within each entry.
- `subtract_graded_byte_columns_counted`: Subtracts an increasing value from each of a counted number of byte columns, each column twelve bytes spaced forty bytes apart, and stops when the count runs out.
- `write_sprite_control_pairs`: Clears a shared word and writes a pair of control words into each of two table entries spaced eight bytes apart.
- `select_display_mode_by_index`: Sets shared display-mode values, chooses between two values by the low bit of its input, looks up a table entry from its shifted index, and calls four routines.
- `copy_strided_word_block`: Copies a block of twelve rows of seventeen words, reading the source at a stride of three words and writing them consecutively into the destination.
- `add_packed_decimal_digits`: Adds packed decimal digits from two bytes with carry, decimal-adjusting each result, and steps a counter on each digit.
- `pop_and_reorganize_list_element`: Decrements a counter in a structure and, if zero, pops an element from a linked list or buffer.
- `pop_and_process_queue_element`: Decrements a counter in a structure and, if zero, pops an element from a linked list or buffer.
- `advance_linked_digit_list`: Decrements a counter in a list header and, when it runs out, reloads it and follows a link to the next list entry, returning the entry's first word.
- `advance_linked_digit_list_2`: Steps a counter in a list header and, on underflow, reloads it and follows the stored link to the next entry, copying words from the list.
- `call_two_routines_pass_through`: Calls two other routines and returns with its register arguments unchanged.
- `call_routine_pass_through`: Calls one other routine and returns its register arguments unchanged.
- `scale_packed_nibbles`: Calls a routine that multiplies four 4-bit fields in a word by a factor and stores the result in a destination array.
- `scale_nibble_data`: Calls a routine that scales four 4-bit fields within a word by a specified factor.
- `scale_packed_nibble_data`: Iterates through a source array and applies a scaling factor to each 4-bit field within a word.
- `scale_packed_nibble_array`: Iterates through a source array of words containing packed nibbles and scales each nibble by a specified factor.
- `scale_packed_nibble_data_2`: Passes data to a routine that scales four 4-bit fields within a word by a specified factor and stores the reassembled results in a destination array.
- `scale_packed_nibbles_by_factor`: Multiplies each of the four 4-bit fields of a word by a factor from a register, reassembles the words and stores them in a destination array.
- `scale_packed_nibbles_clamped`: Scales the four 4-bit fields of each word by a factor clamped to sixteen, storing the results in a destination array.
- `advance_record_state_machine`: Advances a per-object state machine held in a record: it selects a branch from a state field, updates counters and positions in the record, and calls four other routines.
- `fill_record_block_when_idle`: When a record flag is zero, calls a routine and fills a block of records with a fixed negative value, then continues with the rest of the record update.
- `read_next_stream_byte_pair`: Reads the next byte from a stream pointer held in a record.
- `read_next_stream_word_pair`: Reads the next word from a stream pointer held in a record.
- `parse_stream_command_byte`: Reads bytes from a stream, stopping on a byte in the range thirty-two to ninety.
- `copy_record_field_and_return`: Copies one longword field of a record into another field and returns the constant thirty-two.
- `tiny_ori_5`: Small routine of 1 instruction that merges 90 into d2.
- `build_wave_offset_blit`: Builds a table of three hundred and twenty offsets by stepping two phase counters through a 1024-entry lookup table, then programs the blitter and writes a row of words per line.
- `perform_blitter_transfer`: Updates several global variables and uses a blitter routine to transfer data between memory locations using specific control and size parameters.
- `decrement_then_start_blit`: Decrements a shared counter and returns while it is nonzero; otherwise it waits for the blitter, programs its control and size registers and calls one routine.
- `start_blit_with_countdown`: Waits for the blitter to be idle, programs its control, mask, modulo and pointer registers from fixed values and shared state, decrements a counter and calls one routine.
- `parse_stream_command_byte_2`: Reads bytes from a stream, stopping on a byte in the range thirty-two to ninety.
- `copy_stream_bytes_into_buffer`: Copies bytes from a stream into a buffer, decoding each byte from a shared pointer; bytes at or above a threshold trigger a repeat operation.
- `blit_text_glyph`: Waits for the blitter to be idle, then programs its registers to copy one glyph from a font table, indexed by the character code minus the first printable code.
- `wait_raster_then_run_steps`: Calls a fixed routine and four other routines in turn, one of them a wait for a raster line, and returns its register arguments unchanged.
- `clear_sprite_table_entries`: Writes a fixed control value and zero into two blocks of nineteen entries spaced six bytes apart, clearing a sprite table.
- `advance_angle_table_entries`: Adds two to each of forty stored values spaced six bytes apart, wrapping each one back by the period of seven hundred and twenty when it exceeds it.
- `decrement_counter_read_beam_position`: Decrements a shared counter and, when it reaches zero, reads the beam position register and resets its state, then calls one routine.
- `clear_bit_in_row_pairs`: Walks a table of 40 row pointers and clears bit six in each addressed byte and in the byte 40 bytes further on, updating both bitplanes of each row pair.
- `set_pixel_bit_in_planes`: Computes the byte and bit position of a pixel from its x and y coordinates on an 80-byte-stride bitplane, stores both in an output record, then sets that bit in one plane, both planes, or the second plane only, depending on the colour value.
- `tiny_ori_6`: Small routine of 1 instruction that merges 196 into d2.
- `tiny_ori_7`: Small routine of 1 instruction that merges 212 into d2.
- `store_row_address_pairs`: Writes a starting address into two table slots, splitting each 32-bit value into high and low words, and advances the address by 40 bytes between the slots.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `repeat_call_routine_eleven_times`: Invokes one other routine eleven times in a row with no arguments and returns, with no other work in the body.
- `clear_state_and_run_steps`: Clears one global flag, stores a fixed value into a second global, and interleaves those writes with calls to other routines.
- `set_state_and_run_steps`: Sets one global to one and another global to a fixed value, with a further constant stored into a third global, interleaved with calls to other routines.
- `unpack_multiplane_graphics_into_buffers`: Copies word-sized tiles from a graphics table into interleaved plane buffers, then copies bytes and longwords from the same table into other buffers.
- `select_indexed_entry_then_load`: Reads a 32-bit entry from a table indexed by a word value and passes it to a loader routine; when the entry is not minus one, it repeats the load with a second value and calls further routines.
- `select_indexed_entry_then_call`: Looks up a 32-bit entry in a table using a word index, stores it in a global, then calls one routine with a fixed reference value.
- `request_arthur_sprite_load`: Zeroes one global, points two others at a shared buffer and a loader routine, calls the loader, adds a value read from a fixed memory location, and calls it again.
- `request_weapons_sprite_files`: Sets a fixed value and a shared buffer pointer, calls the loader, then resets the buffer pointer and calls the loader again.
- `request_chest_sprite_load`: Points a global at a shared buffer and calls the loader routine.
- `forward_indexed_entry_to_loader`: Reads a 32-bit entry from a table indexed by a word value, stores it in a global, and calls the loader routine.
- `request_skeleton_sprite_load`: Points a global at a shared buffer and calls the loader routine.
- `request_bigpig_sprite_load`: Points a global at a shared buffer and calls the loader routine.
- `request_shield_sprite_load`: Points a global at a shared buffer and calls the loader routine.
- `request_devil_sprite_load`: Points a global at a shared buffer and calls the loader routine.
- `request_cerberus_sprite_load`: Points a global at a shared buffer and calls the loader routine.
- `compute_request_offset_from_memory`: Calls the loader routine twice around a step that scales a value read from a fixed memory location and adds it to a request value.
- `request_spic_image_load`: Calls two routines, stores three fixed words into a global, then points a global at a shared buffer and calls the loader several times.
- `load_entry_with_mode_switch`: Saves a counter, calls several routines, then looks up a table entry using the counter minus one and passes it to the loader with a fixed value.
- `gather_plane_longwords_into_buffer`: Copies longwords from a source block into a destination buffer, reading four rows spaced 2100 bytes apart for each column and advancing through the source in 168-byte steps.
- `repeat_routine_rows_until_zero`: Calls a routine repeatedly across four entries per row, advancing the pointer 42 bytes after each row, and stops when the routine returns zero.
- `scan_forward_for_body_tag`: Advances a pointer two bytes at a time until the longword it reads equals the four-character tag BODY, then returns with the pointer left at that position.
- `unpack_run_length_row`: Decodes a PackBits-style stream into a 40-byte row: a positive count copies that many following literal bytes, a negative count repeats the next byte, and the value 128 is skipped.
- `replicate_pattern_across_bitplanes`: Writes a ten-longword pattern into four planes spaced 2100 bytes apart for each of 200 rows, advancing 42 bytes per row.
- `replicate_pattern_with_counted_rows`: Writes the same ten-longword pattern into four planes, but the number of rows is taken from the caller's counter register instead of being fixed.
- `forward_call_to_inner_routine`: Calls one other routine and returns, with no other work in the body.
- `toggle_two_buffer_addresses`: Sets a pair of globals to two fixed addresses, choosing which address goes into which global based on whether the first global already holds one of them.
- `set_default_buffer_addresses`: Stores two fixed buffer addresses into two globals.
- `wait_for_beam_position`: Spins until a global holds minus 56, then enters an endless loop.
- `store_split_buffer_address_table`: Stores five buffer addresses, each computed as a base plus a fixed offset, into a pointer table as split high and low words.
- `shift_two_word_register_left`: Shifts a 32-bit value held in two words left by one bit, taking the new low bit from a bit of a state byte, and stores the high byte of the result in a global.
- `repeat_steps_until_result_clears`: Calls three routines, then repeatedly calls another routine with a fixed argument until a register reaches zero, and repeats the whole sequence while its return value is nonzero.
- `decrement_nonzero_digit_fields`: Subtracts one unit from each of the three low four-bit fields of a 16-bit value that is pointed to by the caller, but only where that field is nonzero.
- `repeat_steps_with_two_pointers`: Calls three routines, then repeatedly calls another routine with two pointer arguments until a register reaches zero, and repeats while its return value is nonzero.
- `step_digit_fields_toward_target`: Compares the three low four-bit fields of one word with the matching fields of another and adds one unit to the second word's field wherever they differ.
- `set_blitter_control_defaults`: Writes six fixed values to the blitter control and modulo registers, zeroing the modulo registers and loading the control registers with constant settings.
- `tiny_ori_8`: Small routine of 1 instruction that merges 48 into d7.
- `tiny_ori_9`: Small routine of 1 instruction that merges 96 into d6.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `deinterleave_four_word_planes`: Splits a stream of four-word groups into four separate planes, writing one word of each group to each plane, for 4000 groups.
- `interleave_planes_with_zero_padding`: Writes groups of planar words from a source table into a destination grid with 48-word row strides, storing zero in the third word of each group.
- `init_display_state_if_armed`: If a dispatch slot is set, clears a 256-byte scratch block, copies three words from a table chosen by a flag word, stores pointers to the scratch block in state variables, and returns one.
- `reset_state_buffer_and_pointers`: If a dispatch slot is set, clears a 256-byte scratch block, copies table values chosen by a flag word into state variables, and stores pointers to the scratch block.
- `disable_interrupts_and_set_flag`: Sets a global variable to a specific value and modifies a register before calling another routine.
- `stub_empty_return`: Empty routine: it only returns, with no reads or writes.
- `countdown_and_dispatch_steps`: If a dispatch slot is set, decrements a state byte.
- `call_routine_four_times`: If a dispatch slot is set, calls one routine four times in a row.
- `relink_object_frame_pointer`: When a per-object flag is set, clears it, looks up a pointer from a table chosen by a state index, stores that pointer and its first word into two per-object slots, and copies a word into a global.
- `store_object_pointer_and_flag`: Loads a per-object pointer and stores it, with its first word, into two per-object slots.
- `reset_idle_object_fields`: When a per-object counter is below four, copies one state byte to another field, clears a third, sets a fourth to seven, and fills eight longword fields of a pointed-to record with blank values: four-byte runs of 128 followed by zeros.
- `store_word_through_indirect_slot`: Writes the register word into the location that a pointer slot points to, at an offset given by the caller.
- `clamp_value_to_global_quarter`: Limits a byte to a quarter of a global when that global is nonzero and the byte is at least that large.
- `advance_per_object_sequencer`: Runs one step of a per-object sequencer: when a countdown expires it reloads it and writes alternating marker values into a table, ramps a byte gradually, toggles a phase bit between two values, and updates a 16-bit value from a lookup table.
- `reset_object_motion_fields`: Copies one state byte to another field, clears a second, sets a third to seven, and fills eight longword fields of a pointed-to record with four-byte runs of 128 followed by zeros.
- `initialise_pointer_table_sequence`: Saves two globals into fixed slots, toggles a flag bit, resets a routine slot and two globals, then writes a sequence of constant pointers into one table slot, calling routines between writes.
- `increase_global_by_1024`: Adds 1024 to a global variable and returns.
- `store_default_parameter_block`: Stores nine fixed default values into consecutive globals: a zero, the values 980, 79 and 40, and several minus-one and one sentinels.
- `loop_steps_until_countdown_ends`: Sets a loop counter to three and clears a flag, then repeatedly calls several routines, exiting when a routine returns a negative value or when a flag and a countdown both reach their end conditions.
- `tiny_ori_10`: Small routine of 1 instruction that merges 0 into d3.
- `copy_list_entries_in_passes`: Walks a list of signed words ending in minus one.
- `verify_table_checksum`: Negates and sums 128 longwords from a fixed table, then returns zero when the total equals a stored reference value and minus one otherwise.
- `copy_nonzero_entries_until_end`: Repeatedly calls a routine, then copies nonzero longwords from a record's table into an output list in reverse order while a chained value stays nonzero, and finally writes a minus-one terminator.
- `find_matching_table_record`: Scans a table of fixed-size records, comparing each record's stored name bytes with a reference buffer.
- `build_record_list_returning_status`: Once, walks a table of words ending in minus one, calls a routine for each entry, and stores the result, a longword, and 30 bytes copied from a fixed source area into a record ending in minus one.
- `populate_record_list_from_table`: Once, walks a table of words ending in minus one, calls a routine for each entry, and stores the result, a longword, and 30 bytes copied from a fixed source area into a record ending in minus one.
- `copy_nonzero_words_into_list`: Once, copies the nonzero words from a source table into an output list, skipping zeros, and ends the list with minus one.
- `copy_longword_block_when_ready`: When not already done and a status check after a routine call passes, copies 128 longwords from one pointer to another and returns the remaining counter.
- `copy_longword_block_after_check`: When not already done and a status check after a routine call passes, copies 128 longwords from one pointer to another.
- `update_global_status`: Calls a routine and stores its result in a status global.
- `divide_value_by_twenty_two`: Stores a word argument in a global, then divides the byte value by 22, storing the quotient and remainder in globals.
- `divide_value_by_twenty_two_2`: Identical to its neighbour: stores a word argument in a global, divides the byte value by 22 into a quotient and remainder global, reduces the remainder by 11 when it exceeds 10, and records that case in a flag.
- `program_disk_read_registers`: Programs the disk hardware for a read: writes a disk pointer, a sync word and a control value into the disk registers, sets a global to a fixed value, and enables the disk DMA with a transfer length of two.
- `poll_disk_transfer_completion`: Repeats a disk transfer step while two stored positions differ.
- `call_two_routines_in_order`: Calls two other routines in sequence and returns.
- `scan_table_until_routine_succeeds`: Repeatedly finds the next record in a fixed table whose first word matches a counter, calls a routine with it, and increments the counter.
- `interleave_clock_and_data_bits`: Combines 128 longwords pairwise: each output longword takes its even bits from one source longword and places the even bits of a second source, 512 bytes further on, into the odd bit positions.
- `pack_bit_interleave_table_entries`: Calls a routine, then for ten table entries combines two longword fields by interleaving their bits.
- `locate_sync_words_in_buffer`: Scans a data buffer for the sync word and records up to ten positions in a table, backing up one word when the match is not exact.
- `step_drive_position_toward_target`: Moves a stored position one step at a time toward a target, writing a direction pattern to a drive control port on each step and calling a routine.
- `poll_status_bit_then_reset`: If a stored position differs from a second value, repeatedly calls two routines while bit four of a status byte stays set and a counter is nonzero, then resets the stored value to zero.
- `busy_wait_count_down`: Spins by decrementing the register value until it reaches zero, then returns.
- `clear_selected_control_bit`: Ors a byte with the constant 125, clears one bit whose index comes from a stored value plus three, and writes the result to a global that drives a control port.
- `write_drive_control_until_ready`: Builds a control-port byte by clearing one bit chosen by a stored index, then sets or clears bit two depending on a flag, and writes it to a global.
- `clear_control_bit_then_delay`: Clears one control bit chosen by a stored index, waits while bit five stays set, then runs a fixed countdown delay.
- `fill_entries_with_stepped_values`: Fills 128 three-word entries: the first two words are set to minus 128, and the third counts upward in steps of seven.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `stub_no_decoded_body`: The decompiler produced no body, so only the facts are known: the routine touches the raster position register and one other custom chip register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_operation`: This routine is empty and performs no operations.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `fill_pointer_table_from_scan`: Scans a 128-entry byte table for the next value above the previous one.
- `advance_audio_playback_tick`: Advances a tick counter.
- `fill_pointer_table_from_scan_2`: Same scan-and-fill logic as its neighbour: finds the next larger byte value in a 128-entry table and fills a 31-entry pointer table with a stride from the structure, then sets a flag and clears counters.
- `reset_control_fields_to_defaults`: Writes zero to four global control bytes and fifteen to a fifth.
- `program_audio_channels_and_dma`: Pseudo-code is unavailable because the decompiler timed out.
- `lookup_twelve_bit_table_indices`: For four 14-word entries of a source table, copies a word and a nibble-based byte into a record.
- `mask_and_compare_word`: Masks a word to its low twelve bits and stores it in a field, then clears a flag byte.
- `dispatch_command_by_nibble`: Interprets one command from a packed control word by its low nibble: it updates a stored value by a byte amount with clamping, steps a value, adds a table-driven scaled offset, or adjusts a counter with a ceiling of 64, then returns the updated word.
