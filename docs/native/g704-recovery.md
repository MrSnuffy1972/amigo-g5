# Game 704 (graham taylor soccer challenge (1992)(krisalis software)(m5)[cr csl]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00704). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802304 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 26 of 26 named (0 author symbols), 26 of 26 described (0 by rule, 13 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2 of 2 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 341 of 341 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 1998 |
| code | 1234 |
| data | 3878 |
| padding | 634 |
| unused | 1794560 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Graham Taylor Soccer Challenge (1992)(Krisalis Software)(M5)[cr CSL](Disk 1 of 2).adf:bootblock | 1 | 176 | 13 |
| Graham Taylor Soccer Challenge (1992)(Krisalis Software)(M5)[cr CSL](Disk 2 of 2).adf:bootblock | 1 | 163 | 12 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Graham Taylor Soccer Challenge (1992)(Krisalis Software)(M5)[cr CSL](Disk 1 of 2).adf:bootblock

- `load_disk_block_and_relocate`: Boot-block entry that issues a disk read request with a fixed length and offset, into a buffer placed 256 bytes past a computed address.
- `store_slots_and_copy_block`: Stores three addresses into a fixed memory slot in turn, calling one unnamed routine after each store.
- `call_and_return_second_result`: Calls one unnamed routine, then calls another and returns its 16-bit result.
- `copy_words_by_count`: Copies a run of 16-bit words from a source pointer to a destination pointer, with the count taken from a register.
- `write_text_marker_by_flag`: Writes one of two four-character text markers, FAT0 or FAT1, chosen by one bit of a global value.
- `detect_refresh_rate_hz`: Resets the CIA port and timer registers, then reads a counter from two CIA bytes and returns fifty or sixty depending on whether it is below a threshold.
- `wait_until_status_byte_clear`: Repeatedly checks a status byte at a fixed offset from a base register and returns minus one once its top bit is clear.
- `build_ram_marker_records`: Clears a block of about 188 bytes, writes the text markers RAMC and RAMF into it, and loops over values from an unnamed routine.
- `find_list_node_with_bit`: Walks a linked list from a pointer at a fixed offset of a base register and stops at the first node whose stored word has the bit selected by a register set, or at the end of the list.
- `walk_list_until_bit_set`: Walks a linked list from the pointer passed in an address register and stops at the first node whose stored word has the bit selected by a register set, or at the end of the list.
- `store_tagged_nibble_value`: When a flag byte is zero, stores a longword combining a nibble-derived value with the two-character tag CP and returns.
- `store_tagged_word_value`: Copies a longword from one pointer to another, and stores a longword combining the input word with the two-character tag CP into a structure field.
- `load_block_then_call_code`: Sets up an I/O request that reads a 4096-byte block into a fixed address and sends it through the library's do-I/O entry.

### Graham Taylor Soccer Challenge (1992)(Krisalis Software)(M5)[cr CSL](Disk 2 of 2).adf:bootblock

- `initialize_boot_sequence`: Sets up hardware registers, performs a memory copy of a code block, and manages several memory markers.
- `relocate_code_block`: Copies a sequence of words from one memory location to another.
- `fetch_word_from_memory`: Returns a 16-bit word from a memory location.
- `copy_memory_block`: Copies a specified number of 16-bit words from a source pointer to a destination pointer.
- `write_text_marker`: Writes a four-character text marker to a memory location based on a bit in a global variable.
- `check_hardware_status`: Clears several memory locations and checks a status byte to determine a return value.
- `verify_memory_consistency`: Loops until a byte at a specific offset changes or a value in another location becomes negative.
- `populate_marker_table`: Clears a block of memory, writes text markers, and populates a table with values from a linked list.
- `search_linked_list_bit`: Walks a linked list until a specific bit is found set within a word in each node.
- `traverse_linked_list`: Iterates through a linked list as long as the current node pointer is not null and a bitwise condition is met.
- `store_tagged_word`: Checks a global variable and, if zero, constructs a tagged longword using a specific two-character tag and a value derived from a register before storing it into a memory location.
- `store_tagged_nibble`: Copies a longword from one memory location to another and then stores a tagged longword combining a register value with a specific two-character tag into a memory location.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_color_register`: Writes a value to the first color register.
