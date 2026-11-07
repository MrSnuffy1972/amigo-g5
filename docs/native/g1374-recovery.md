# Game 1374 (manchester united - europe (1991)(krisalis software)(m5)[cr cls]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01374). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (903952 top-level bytes in 2 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 28 of 28 named (0 author symbols), 28 of 28 described (0 by rule, 28 by the local model) |
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
| Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS].adf:bootblock | 1 | 165 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 242 | 16 |

## Cases handled

- manchester united - europe (1991)(krisalis software)(m5)[cr cls]/Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS].adf: custom-format (non-DOS) disk, 1616 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 186 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- manchester united - europe (1991)(krisalis software)(m5)[cr cls]/Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS].adf: 2 track-loaded blocks found in recovered runtime segments and classified through them
- manchester united - europe (1991)(krisalis software)(m5)[cr cls]/Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS].adf: blocks 2-11 (bytes 1024-6144) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.28)
- manchester united - europe (1991)(krisalis software)(m5)[cr cls]/Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS].adf: blocks 1540-1759 (bytes 788480-901120) outside the filesystem hold data, not code (opcode density 0.002, entropy 5.09)
- 2 packed track region(s), 708608 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Manchester United - Europe (1991)(Krisalis Software)(M5)[cr CLS].adf:bootblock

- `initialize_system_with_io`: Calls a routine via a pointer, populates a structure with certain values, and copies a block of code to a high memory address.
- `copy_boot_code_block`: Writes a specific value to a memory address and copies a fixed number of words from a routine to a destination address.
- `copy_memory_block`: copies a block of memory from a source address to a destination address for a fixed number of words.
- `write_text_marker_by_flag`: writes one of two four-character text markers, FAT0 or FAT1, chosen by one bit of a global value.
- `detect_refresh_rate_hz`: Resets certain hardware or memory-mapped registers, checks a condition, and writes a text marker into a structure based on a threshold check.
- `wait_until_status_byte_clear`: repeatedly checks a status byte at a fixed offset from a base register and returns minus one once its top bit is clear.
- `initialize_linked_list_data`: Clears a range of memory, writes a signature, and then traverses a linked list to populate or modify data structures with specific signatures.
- `traverse_linked_list_bit_test`: traverses a linked list of pointers and stops if a specific bit or a certain bit-pattern is detected in the next word.
- `traverse_linked_list_with_data`: traverses a linked list of pointers and stops if a bit-pattern or certain bit is detected in a word located at a fixed offset from the current pointer.
- `update_memory_with_word`: Checks a global variable and, if zero, constructs a value from a register and writes it to a global memory offset.
- `copy_word_and_update_memory`: copies a word from one memory location to another and writes a 32-bit value to a global memory offset, where the upper half of the value contains a constant pattern and the lower half contains a word from a register.
- `set_register_values`: writes constant values to a table or structure and then calls a routine via a pointer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_loop_routine`: The routine is empty and consists only of an infinite loop.
- `update_table_values`: Writes a value to a global variable and enters a loop that processes a table of values if certain conditions are unable to be met.
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
