# Game 2417 (conqueror (1989)(rainbow arts)(de-en)[cr qtx][h tfs]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02417). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902608 top-level bytes in 12 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 4 raw-word lines |
| Functions named and described | pass, 33 of 33 named (0 author symbols), 33 of 33 described (3 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 462 of 462 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 904 of 904 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 32032 |
| code | 3130 |
| data | 348387 |
| padding | 1939 |
| unused | 517120 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX][h TFS].adf:CONQUEROR | 2 | 137 | 9 |
| tfs | 6 | 231 | 6 |
| type | 2 | 117 | 2 |
| Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX][h TFS].adf:intro | 2 | 83 | 3 |
| Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX][h TFS].adf:game | 2 | 83 | 3 |
| Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX][h TFS].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 231 | 7 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 1 |

## Cases handled

- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address: 180 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 10 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- conqueror (1989)(rainbow arts)(de-en)[cr qtx][h tfs]/Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX][h TFS].adf:intro: a hunk has very high entropy (packed payload); its depacker is recovered as code, the payload as data

## Cracktros (kept, not protection)

- Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX][h TFS].adf:intro

## Copy protection found

- none found

## Functions


### Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX][h TFS].adf:CONQUEROR

- `start_program_and_open_dos`: Program entry that opens the dos library through the exec base, switches to supervisor state, and writes through dos.Write.
- `describe_undecompiled_handler`: Only four instructions were decoded for this routine, and its pseudo-code is unavailable.
- `decode_packed_stream_step`: No pseudo-code is available for this 43-instruction routine.
- `describe_undecompiled_routine`: No pseudo-code is available for this three-instruction routine.
- `read_packed_bits_then_call`: No pseudo-code is available for this 11-instruction routine.
- `read_bits_and_set_color`: No pseudo-code is available for this 14-instruction routine.
- `sweep_all_word_values`: Writes each 16-bit value to a single global location, counting down from minus one through all 65536 values until it wraps back to minus one, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### tfs

- `free_memory_blocks_in_loop`: Program entry that calls unnamed routines and, in a loop of three passes, calls the exec free-memory routine after each one.
- `decrement_until_minus_one`: Decrements a 16-bit counter held in a register until it reaches minus one, then returns.
- `accumulate_decoded_table_values`: Runs an outer loop of two passes, and in each pass a nested loop reads values from an unnamed routine and adds a value into a table entry at an offset built from those reads.
- `unpack_backward_bitstream_runs`: The listed facts give only two instructions and one call to a tiny OR routine, which does not match the long pseudo-code.
- `unpack_backward_bitstream_block`: The pseudo-code shows a bit-driven loop that refills a bit buffer from the packed stream and copies runs of bytes backward into a destination.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_longwords_then_call`: If the destination pointer passed on the stack is non-null, copies sixteen longwords from a fixed offset of a base register into it.

### Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX][h TFS].adf:intro

- `intro_bytekiller_depack`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `bk_refill_bit_buffer_2`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits_2`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX][h TFS].adf:game

- `intro_bytekiller_depack_2`: Cracktro start: ByteKiller depacker that decodes the packed stream after it backwards into 50000 hex onwards, showing progress on COLOR00, checks the checksum and jumps to the unpacked cracktro at 50000 hex.
- `read_previous_longword_shifted`: Reads the longword just before an address register's position, shifts it right by one bit, sets its top bit, and returns the result.
- `advance_bit_buffer_by_count`: Shifts a bit buffer right by a count taken from a register, refilling it from the previous longword with its top bit set whenever it empties.

### Conqueror (1989)(Rainbow Arts)(De-En)[cr QTX][h TFS].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `poll_flag_and_set_registers`: Enters a loop checking a flag bit, and upon meeting a condition, writes several values to a global variable and returns.
- `clear_memory_and_build_copper`: Clears a flag bit, zeroes a block of longwords and copies a template into a list until it reaches a sentinel, then fills the rest of the list with generated entries.
- `fill_copper_list_from_table`: Returns early unless a flag is zero, then walks a table, computing a word from each entry's low four bits and a running offset and writing three words at fixed offsets in a list.
- `advance_sequence_and_copy_entries`: Advances a sequence index held in a global, wrapping at a sentinel of minus one, then copies values into a list at a stride of twelve words with the copy count taken from the index.
- `search_and_configure_blt`: Checks a flag, searches a list for a matching byte, and if found, calculates an address and writes several values to hardware registers including BLT control and size registers.
- `calculate_blt_parameters`: Calculates a value based on the contents of a register, modifies a global variable, and performs several writes to memory and registers.
- `copy_table_pairs_into_buffers`: Copies sixteen table values into two buffers at once, one walking backward and one forward, and advances the table pointer with a wrap at a sentinel when a counter reaches zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `copy_bytes_and_reset_pointers`: Copies bytes one at a time from a source region into a fixed buffer address until the source reaches a fixed end address, then stores that buffer address in two pointer slots and calls another routine.
