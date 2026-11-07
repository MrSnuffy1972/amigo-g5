# Game 1556 (manchester united - europe (1991)(krisalis software)(m5)[cr cls][a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01556). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903952 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 28 of 28 named (0 author symbols), 28 of 28 described (0 by rule, 16 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 186 of 186 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 407 of 407 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 2526 |
| code | 1406 |
| data | 827316 |
| unused | 72704 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS][a].adf:bootblock | 1 | 165 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 242 | 16 |

## Cases handled

- manchester united - europe (1991)(krisalis software)(m5)[cr cls][a]/Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS][a].adf: custom-format (non-DOS) disk, 1616 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 186 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- manchester united - europe (1991)(krisalis software)(m5)[cr cls][a]/Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS][a].adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- manchester united - europe (1991)(krisalis software)(m5)[cr cls][a]/Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS][a].adf: blocks 2-11 (bytes 1024-6144) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.28)
- manchester united - europe (1991)(krisalis software)(m5)[cr cls][a]/Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS][a].adf: blocks 1540-1759 (bytes 788480-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.09)
- 2 packed track region(s), 708608 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS][a].adf:bootblock

- `boot_read_blocks_with_io`: Boot block entry that calls the exec library, fills a disk I/O request with a transfer length and offset, and issues the read, then writes the interrupt and DMA control registers.
- `store_vectors_and_copy_table`: Stores three pointer values into a fixed low-memory slot, calling a routine after each, then copies a table of words to a fixed buffer.
- `copy_block_and_run_forever`: Copies a block of longwords from a source pointer to a fixed low-memory address, then calls the code at another fixed address and spins forever.
- `write_text_marker_by_flag`: Writes the four-character text marker FAT0 or FAT1 into a structure, choosing FAT1 when one bit of a global value is set.
- `detect_frame_rate_from_cia`: Resets CIA port and timer registers, reads a counter from two CIA bytes and compares it with a threshold to choose between two frame rates, then writes a matching text marker.
- `wait_until_status_byte_clear`: Polls a status byte at a fixed offset from the base register until its top bit is clear.
- `initialize_tagged_structure`: Zeroes a 172-byte structure, writes the four-character tag RAMC into it, then stores two values and calls two routines to fill in the rest.
- `find_bit_node_in_list`: Loads the head of a linked list from a structure and walks it, returning the first node whose 16-bit field has the requested bit set.
- `scan_list_for_bit`: Walks a linked list from the argument pointer, testing one bit of each node's 16-bit field against a bit index, and stops at the first node with that bit set or at the end of the list.
- `store_tag_and_vector`: When a flag byte is clear, writes a status value and the two-character tag CP into a structure and stores a vector address.
- `store_tag_and_value`: Stores a value through one pointer and writes the two-character tag CP together with a status word into a structure.
- `issue_io_then_call_code`: Sets up an I/O request with a transfer address, length and offset, issues it through the exec library, and then calls the code loaded at a fixed address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_loop_routine`: The routine is empty and consists only of an infinite loop.
- `update_table_values`: Writes a value to a global variable and enters a loop that processes a table of values if certain conditions are met.
- `xor_memory_with_register`: Xors a value from a register with a value stored at a pointer and saves the result back to that pointer.
- `copy_memory_range`: Copies a block of memory from a source range to a destination range if certain conditions are met.
- `clear_memory_blocks`: Iteratively clears several large, fixed-offset memory locations and a pointer-referenced word to zero.
- `return_data_registers_unchanged`: Performs no work and returns the incoming values of the two data registers unchanged as a certain pair.
- `convert_field_to_word_count`: Reads a 16-bit field from the record addressed by the address register, adds fifteen and shifts right by four, returning the number of sixteen-unit columns needed to cover that value.
- `scale_and_store_values`: Multiplies a sequence of short values from a source pointer by two and stores the results in a destination table.
- `set_hardware_registers_2`: Writes constant values to several hardware registers and updates a global variable.
- `set_hardware_registers`: Writes specific constant values to certain memory-mapped hardware registers.
- `swap_global_values`: Swaps the contents of two adjacent global variables.
- `split_and_store_integer`: Takes a 32-bit value from a pointer and stores its lower and upper 16-bit halves into several global variables, while also storing incremented versions of that value into other global variables.
- `increment_global_variable`: Sets a global variable to a constant and increments another global variable.
- `loop_until_condition_met`: Executes a loop that checks a condition involving a pointer-based lookup and a multiplier, then sets a pointer-referenced value to zero.
- `wait_for_value_match`: Sets a pointer-referenced value to zero and loops until a second pointer-referenced value matches that zeroed value.
- `reset_global_variable`: Sets a global variable to zero and loops until that variable equals zero.
