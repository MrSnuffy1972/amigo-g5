# Game 1486 (deuteros - the next millennium (1991)(activision)(m3)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01486). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1818000 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 21 raw-word lines |
| Functions named and described | pass, 127 of 127 named (0 author symbols), 127 of 127 described (14 by rule, 107 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 733 of 733 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 108032 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 2522 of 2522 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 4513 |
| code | 15856 |
| data | 1669945 |
| padding | 198 |
| unused | 127488 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf:bootblock | 1 | 29 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 295 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2198 | 114 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf bytes 13824-14336 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf bytes 23552-25088 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf bytes 28160-29184 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf bytes 497664-526336 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf bytes 566272-636928 (code never loaded by a reference run) | 1 | 0 | 0 |
| Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf bytes 23552-29184 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: custom-format (non-DOS) disk, 1572 non-empty blocks: matched against track-loaded segments
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: custom-format (non-DOS) disk, 1695 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 241 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 466 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: bytes 13824-14336 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: bytes 23552-25088 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: bytes 28160-29184 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: bytes 497664-526336 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: bytes 566272-636928 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: bytes 23552-29184 look like code no run loaded: recovered position-independently (load address unknown)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: 5 logged loads cover 102400 bytes of the image
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: 14 track-loaded blocks found in recovered runtime segments and classified through them
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 2-22 (bytes 1024-11776) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.85)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 33-44 (bytes 16896-23040) outside the filesystem hold data, not code (opcode density 0.058, entropy 5.57)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 50-50 (bytes 25600-26112) outside the filesystem hold data, not code (opcode density 0.047, entropy 5.63)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 52-53 (bytes 26624-27648) outside the filesystem hold data, not code (opcode density 0.047, entropy 4.34)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 61-61 (bytes 31232-31744) outside the filesystem hold data, not code (opcode density 0.039, entropy 4.12)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 65-65 (bytes 33280-33792) outside the filesystem hold data, not code (opcode density 0.023, entropy 4.30)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 68-68 (bytes 34816-35328) outside the filesystem hold data, not code (opcode density 0.004, entropy 0.57)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 154-170 (bytes 78848-87552) outside the filesystem hold data, not code (opcode density 0.043, entropy 5.84)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 172-194 (bytes 88064-99840) outside the filesystem hold data, not code (opcode density 0.044, entropy 6.24)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 198-360 (bytes 101376-184832) outside the filesystem hold data, not code (opcode density 0.002, entropy 7.09)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 363-873 (bytes 185856-447488) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.98)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 875-880 (bytes 448000-451072) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.23)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 883-884 (bytes 452096-453120) outside the filesystem hold data, not code (opcode density 0.002, entropy 0.41)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 931-931 (bytes 476672-477184) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.59)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 934-937 (bytes 478208-480256) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.47)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 951-970 (bytes 486912-497152) outside the filesystem hold data, not code (opcode density 0.035, entropy 5.48)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 1089-1104 (bytes 557568-565760) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.32)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 1245-1745 (bytes 637440-893952) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.61)
- deuteros - the next millennium (1991)(activision)(m3)(disk 1 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf: blocks 1749-1759 (bytes 895488-901120) outside the filesystem hold data, not code (opcode density 0.055, entropy 5.54)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: 1 logged loads cover 5632 bytes of the image
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: 4 track-loaded blocks found in recovered runtime segments and classified through them
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 2-22 (bytes 1024-11776) outside the filesystem hold data, not code (opcode density 0.033, entropy 5.00)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 27-27 (bytes 13824-14336) outside the filesystem hold data, not code (opcode density 0.047, entropy 4.61)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 33-44 (bytes 16896-23040) outside the filesystem hold data, not code (opcode density 0.056, entropy 5.57)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 60-76 (bytes 30720-39424) outside the filesystem hold data, not code (opcode density 0.041, entropy 4.80)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 88-111 (bytes 45056-57344) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.74)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 132-148 (bytes 67584-76288) outside the filesystem hold data, not code (opcode density 0.049, entropy 4.91)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 150-182 (bytes 76800-93696) outside the filesystem hold data, not code (opcode density 0.055, entropy 4.97)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 186-262 (bytes 95232-134656) outside the filesystem hold data, not code (opcode density 0.017, entropy 5.34)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 277-306 (bytes 141824-157184) outside the filesystem hold data, not code (opcode density 0.043, entropy 6.12)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 308-441 (bytes 157696-226304) outside the filesystem hold data, not code (opcode density 0.023, entropy 5.95)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 451-1488 (bytes 230912-762368) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.47)
- deuteros - the next millennium (1991)(activision)(m3)(disk 2 of 2)/Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 2 of 2).adf: blocks 1496-1759 (bytes 765952-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.01)
- 6 never-loaded region(s), 108032 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 2 packed track region(s), 28672 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `issue_doio_with_command_nine` (runtime segment an address): Sets fields in a global structure to request command nine with a length of one and a zero flag, then calls the I/O routine. Kind: track/sector command check. Pass path: the function returns normally, allowing execution to continue to the next instruction after the call.
- `issue_doio_with_command_8004` (runtime segment an address): Sets the command field of a global structure to 32772 and a flag byte to zero, then calls the I/O routine. Kind: trackdisk.device I/O command (likely a custom/direct track read). Pass path: The function returns to the caller; the program continues if the disk read succeeds (no error returned in the status field)..
- `issue_doio_with_command_five` (runtime segment an address): Sets the command field of a global structure to five and a flag byte to zero, then calls the I/O routine. Kind: track/disk integrity check. Pass path: the branch where the comparison `cmp.w #an address, d0` results in a non-zero/not-equal state or where the subsequent logic proceeds to data loading rather than an error/crash..
- `issue_doio_and_copy_words` (runtime segment an address): Fills fields of a global structure for an I/O request, calls the I/O routine, and if a status byte is zero copies a number of words determined by the input word into a destination pointer. Kind: disk/track integrity check via I/O. Pass path: the branch where the loop is skipped (if the check fails/returns non-zero) or the branch where the loop executes (if the check succeeds and data is copied).

## Functions


### Deuteros - The Next Millennium (1991)(Activision)(M3)(Disk 1 of 2).adf:bootblock

- `issue_timed_io_request_sequence`: Issues three Exec library requests, filling in request fields between them and running a fixed countdown loop of fifty thousand steps before the second request.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `open_trackdisk_and_create_port`: Calls exec to open trackdisk.device, fills in a message port structure with a name and flags, adds the port, finds a task, and if the lookup fails it loops forever; otherwise it sets a field to all ones and clears a byte.
- `call_do_i_o`: Library stub: loads its arguments and the exec library base, calls DoIO and returns its result to the caller.
- `reset_request_and_do_io`: Clears the length field of the global IO request, sets its command word to nine and its flag byte to zero, then invokes exec DoIO on that request.
- `initialize_system_and_buffers`: Performs a sequence of initialization tasks including setting global function pointers, calling several setup routines, and managing memory through system library calls.
- `return_constant_pair`: Returns a fixed pair of constant values in the data registers without touching memory or calling anything.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `tiny_ori_2`: Small routine of 1 instruction that merges 0 into d1.
- `initialize_memory_buffers`: Sets up two distinct memory buffer structures by initializing control blocks, clearing associated data areas with zeros, and defining a series of memory address offsets.
- `initialize_memory_tables`: Initializes two distinct blocks of memory by zeroing out a series of words and populating them with specific offset values and pointers.
- `initialize_graphics_buffers`: Sets up and clears two distinct memory regions by populating them with specific offset values and zeroing out data blocks.
- `setup_bitplanes_and_load_libraries`: Opens the Amiga system libraries, checks available memory, and clears a small table.
- `initialize_and_copy_buffers`: Initializes two memory buffer structures and then copies data from a global variable into four separate destination memory areas using a loop.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `invoke_bitstream_decompression`: Calls a routine to decompress a bit-packed data stream into a destination buffer.
- `decompress_bitstream_data`: Decompresses a bit-packed stream into a destination buffer using a variable-length coding scheme.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
- `update_status_and_branch`: Sets a series of tracking values and calls several subroutines to update state.
- `open_graphics_library`: Attempts to open a graphics library and increments a global counter if successful, otherwise enters an infinite loop.
- `call_library_vector_222`: Library stub: loads a library base from a global variable and calls the library function at offset minus 222 of that base, returning its result to the caller.
- `call_library_function`: Calls a function from a library using a pointer retrieved from a global variable.
- `clear_global_word`: Calls a library function and then sets a global word to zero.
- `advance_index_from_table`: Adds a table entry indexed by the current global value to that value plus a constant, storing the result back into the global variable.
- `tiny_ori_3`: Small routine of 1 instruction that merges 136 into d2.
- `copy_word_to_global`: Stores a word from a global variable into another global variable.
- `update_global_register_value`: Stores the contents of a data register into a global variable.
- `update_function_pointer`: Copies a value from one global variable into another global variable.
- `store_register_to_global`: Stores the contents of a data register into a global variable.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `return_from_subroutine`: This routine is empty and performs no operations before returning.
- `tiny_ori_4`: Small routine of 1 instruction that merges 176 into d2.
- `call_other_routine`: Calls another routine and then returns.
- `store_pointer_from_low_nibble`: Computes a pointer from the low nibble of the input word shifted left three bits and stores it into a global variable.
- `store_pointer_from_low_nibble_2`: Computes a pointer by shifting the low nibble of an input word left by three bits and adding it to a global variable, then stores the result in another global variable.
- `clear_tiny_ori_byte`: Clears a byte variable to zero, after which execution falls through into the next block.
- `parse_compressed_stream`: Iterates through a data stream, interpreting bytes as control codes to perform pointer arithmetic, update global variables, or call other routines.
- `parse_compressed_data_stream`: Iterates through a data stream, interpreting bytes as control codes to perform pointer arithmetic or update global variables.
- `set_global_and_call`: Stores the input word ORed with a constant into a global location, then calls another routine.
- `compute_offset_from_bytes`: Reads two bytes from a structure, clamps the second to a maximum of 48, and computes a pointer by adding the first byte plus the clamped value times 160 to a base address.
- `call_byte_returning_routine`: Calls another routine and returns the value it produces in the data register.
- `blit_masked_bytes_into_bitmap`: Uses a byte from a table to combine two source bitmaps with a mask and writes the result into a destination bitmap across a small block, then doubles a global variable.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `initialize_bitmap_structures_at_offset`: Calls another routine with a pointer derived from an address register plus two.
- `repeat_blit_until_counter_zero`: Repeatedly calls a bitmap-blitting routine and loops until a returned word value becomes zero.
- `execute_self_and_return`: Calls itself once and then returns the value currently held in a register.
- `copy_table_data_2`: Increments a global pointer, checks a status byte, and performs two block copies of data from a table into different memory locations.
- `update_function_pointers`: Stores two constant values into consecutive global memory locations and then copies the address of the current routine into two subsequent memory locations before executing the function pointed to by a third global variable.
- `store_interrupt_server_table`: Writes fixed values into consecutive global memory locations and then calls another routine through a pointer derived from a global variable.
- `call_rem_int_server`: Library stub: loads its arguments and the exec library base, calls RemIntServer and returns its result to the caller.
- `initialize_trackdisk_port`: Opens trackdisk.device and initializes a message port structure with specific flags and a name.
- `issue_doio_with_command_nine`: Sets fields in a global structure to request command nine with a length of one and a zero flag, then calls the I/O routine.
- `issue_doio_with_command_8004`: Sets the command field of a global structure to 32772 and a flag byte to zero, then calls the I/O routine.
- `issue_doio_with_command_five`: Sets the command field of a global structure to five and a flag byte to zero, then calls the I/O routine.
- `process_io_requests_in_chunks`: Iterates through a series of data chunks, performing I/O operations for each non-zero segment by setting command and length fields in a global structure.
- `issue_doio_and_copy_words`: Fills fields of a global structure for an I/O request, calls the I/O routine, and if a status byte is zero copies a number of words determined by the input word into a destination pointer.
- `issue_io_and_load_pointer`: Calls two routines that issue I/O requests, then loads a longword from a fixed offset in a global structure into a global variable.
- `expand_compressed_graphics_data`: Decompresses and expands bitstream data into a planar bitplane buffer using nested loops for block copying and run-length expansion.
- `blit_compressed_sprite_data`: Decompresses a custom bitstream format to write pixel data into a video buffer.
- `blit_run_length_encoded_tile`: Decodes a run-length encoded byte stream into a planar bitplane buffer, expanding literal, repeated-byte, and word-copy runs while advancing through rows of a fixed stride and wrapping at the buffer end.
- `copy_slot_table_to_active_objects`: Initializes a block of per-object state from a source table: it clears a counter, copies a fixed set of parameter words and pointers from the preset data into the working slots, then runs a per-slot update pass that dispatches on a type byte and tallies active entries.
- `tiny_ori_5`: Small routine of 1 instruction that merges 188 into d3.
- `dispatch_stored_handler`: The routine is a single 68000 instruction that is entered as a target of absolute move and movea references from several callers, and its body is a one-instruction stub.
- `tiny_ori_6`: Small routine of 1 instruction that merges 2 into d4.
- `tiny_ori_7`: Small routine of 1 instruction that merges 130 into d4.
- `process_event_queue`: Iterates through a table of event structures to update state or trigger callbacks based on specific command codes.
- `update_active_object_slots`: Iterates through a table of object slots, decrementing timers and invoking specific handlers based on a type byte.
- `parse_bytecode_stream`: Iterates through a stream of data, interpreting specific byte values as instructions to manipulate a destination structure, perform arithmetic on registers, or jump to new memory locations.
- `clear_buffer_alternating`: Increments a global counter, selects one of two buffer pointers based on the counter's low bit, stores that pointer in a global variable, then writes zero to four consecutive longwords per iteration for two thousand iterations, clearing the selected buffer.
- `call_library_and_wait_vblank`: Calls a routine through a pointer offset from a global pointer, then polls a global variable until its bit 32 becomes set, waiting on a hardware status flag.
- `enter_infinite_loop`: Executes an unconditional loop that prevents further code execution.
- `execute_infinite_loop`: Checks a global variable and enters an infinite loop.
- `wait_for_potgor_bit`: Stores a word from a register into two global variables, calls a routine through a table indexed by that word, and then loops until bit 2 of a global variable is set, returning the value of another global variable.
- `tiny_ori_8`: Small routine of 1 instruction that merges 4 into d0.
- `monitor_state_variable`: Checks a global byte variable and enters an infinite loop if the value is two or if certain conditions are met.
- `close_device_and_remport`: Sets a global variable to five, then loops calling a routine and a library vector until a returned value matches a constant; afterwards it calls two library vectors through a base pointer, stores that base into two globals, and polls a global bit while waiting.
- `clear_2000_longwords`: Clears a block of memory by writing zero to 2000 consecutive longwords starting at a global pointer.
- `zero_bitwise_or_stub`: A lone bitwise OR with zero leaves the data register unchanged and is reached through an address stored by entry code.
- `decrement_loop_counter`: Acts as a global counter that is decremented by various callers in the system.
- `decrement_buffer_values`: Decrements a series of values in a table by a specific amount through a loop and then calls two other routines.
- `set_flag_and_call_helpers`: Sets bit 1 of a global variable, writes a constant to another global variable, and calls a routine that initializes audio and scans a table.
- `initialize_audio_and_scan_table`: Clears several global variables, scans a table to find a maximum value, and initializes audio-related pointers and hardware registers.
- `process_table_entries`: Iterates over a table of entries, skipping zero entries, and for each nonzero entry updates fields based on a running value and a constant, possibly resetting a field to a fixed address.
- `silence_audio_channels`: Writes zero to the volume registers of all four audio channels and enables DMA for those channels.
- `execute_conditional_callback`: Checks a bit in a global variable and, if a register is non-zero, executes a series of function calls through pointers stored in a table.
- `update_runtime_state`: Increments a global variable and, if a threshold is met, performs a table search to update several global pointers and address registers.
- `initialize_self_modifying_logic`: Performs arithmetic on a global pointer, enters a loop to find a specific value in a table, and updates self-modifying code and global pointers.
- `initialize_global_pointers`: Sets a global variable to one, waits for a value at a specific offset in a table to be non-zero, and then calculates and stores several pointer values into global variables.
- `increment_global_pointer`: Repeatedly adds a constant value to a global variable and modifies address registers.
- `update_audio_parameters_2`: Calculates a pointer from an input value and loads data from that pointer into various global variables and audio hardware registers based on a selector.
- `update_audio_parameters`: Loads values from a calculated pointer into various global variables and audio hardware registers based on a channel index.
- `store_audio_parameters`: Depending on a register value, it stores values from a pointer into various global variables and audio hardware registers for different channels.
- `store_audio_channel_three`: If a register equals three, it stores values from a pointer into several global variables and audio hardware registers.
- `store_audio_registers`: Stores values from a structure into global variables and writes to audio hardware registers for channel 3 (length, period, volume, and high bits of pointer).
- `dispatch_parameter_updates`: Checks a value in a register and updates various global variables, pointers, or return values based on specific bit patterns and a selector.
- `update_joystick_flag_2`: Checks a register value and sets or clears specific bits in a global variable based on whether the value matches certain constants.
- `update_joystick_flag`: Checks a register value and sets or clears a specific bit in a global variable based on whether the value matches certain constants.
- `set_flag_if_register_matches`: If a register equals a specific value, sets bit 1 of a global variable.
- `update_self_modifying_pointers`: Calculates and stores a new value into a global variable and updates a pointer and a self-modifying instruction address.
- `store_negated_word`: Stores the negation of a 16-bit register into one of four different global variables based on a selector.
- `store_word_to_various_globals`: Stores a word from a register into one of three different global variables based on the value of a register.
- `store_word_to_sub_2bf0`: If a register is not three, it stores a word from a register into a global variable.
- `store_word_to_global`: Stores a word from a register into a global variable.
- `update_function_pointer_3`: Stores a 16-bit value into one of four different global variables based on a selector.
- `update_conditional_function_pointers`: Compares a 16-bit register against specific values to select which global function pointer to update.
- `conditionally_update_global_word`: Checks if a 16-bit register value equals three.
- `store_word_to_global_2`: Stores a 16-bit value from a register into a global variable.
- `update_audio_channels`: Updates audio channel registers (pointer, length, period, volume) for channels 0-3 based on flags, enables DMA channels, and computes scaled values from global variables.
- `calculate_address_offset`: Performs arithmetic operations using a global variable and an input value to update a register.
- `update_pointer_value`: Calculates a new value based on a global variable and an input, then stores the result.
- `modify_register_content`: Uses a global variable and an input to perform a calculation and update a register.
- `update_dynamic_pointer`: Calculates a new address by adding a value from a global variable and a bitwise manipulated constant to an existing pointer, then stores the result back into that pointer.
- `update_pointer_offset`: Calculates a new value by adding a global variable and a bitwise manipulated value to an existing address, then stores the result back into that same address.
- `update_function_pointer_2`: Calculates a new address by adding a value from a global variable and a bitwise manipulated value derived from the current routine's address to the existing pointer.
- `update_relative_pointer`: Calculates a new address by adding a value derived from a global variable and a bitwise manipulation of the current instruction pointer to another global variable.
- `tiny_ori_9`: Small routine of 1 instruction that merges 0 into d5.
- `modify_address_register`: The routine performs arithmetic operations on an address register.
- `update_self_modifying_pointer`: Calculates a new value based on a global variable and the current instruction address, then stores the result back into the routine's own memory location.
- `modify_register`: The routine performs arithmetic operations on a register.
- `move_register`: The routine performs two move operations.
- `update_global_pointer`: Calculates a new value by adding a constant and a bitwise manipulated value to a global variable, then stores the result back into that same global variable.
- `update_global_pointer_2`: The routine updates a global variable using a value derived from a calculation involving another global variable.
- `add_indexed_byte_to_global`: Reads a byte from a table indexed by the low 15 bits of a global variable, masks off one bit, adds it to that global, and stores the result back into the global while returning the low byte of the sum.
- `copy_table_data`: Copies several sets of three words from a source table into multiple destination locations in memory.
- `copy_selected_entries`: Selects a table entry based on a word argument, then for each set bit in a byte argument copies four words from that entry into fixed memory locations and sets corresponding bits in a global variable.
- `fetch_table_entry_and_distribute_fields`: Calculates an offset into a table using a word argument and retrieves a pointer, following a secondary pointer if the initial value is negative.
- `select_entry_and_store_fields`: Uses a word argument times fourteen as an offset into a table to fetch an entry; if the first longword is negative it follows the second longword as a pointer, then conditionally copies a longword from offset ten of that entry into four separate global locations according to bits of a byte argument, returning the first longword.
- `update_dma_control_state_machine`: Processes a bit-flagged control word to update counters, pointers, and a state block, while writing to the DMA control register to enable specific channels.
