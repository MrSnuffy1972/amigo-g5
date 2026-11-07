# Game 1613 (realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01613). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (925376 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 70 of 70 named (0 author symbols), 70 of 70 described (5 by rule, 58 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 246 of 246 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 82432 bytes, 7 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1321 of 2070 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 31257 |
| code | 6880 |
| data | 487361 |
| padding | 6 |
| unused | 399872 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf:bootblock | 1 | 13 | 1 |
| logged disk load from Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf boot-block load | 1 | 159 | 3 |
| logged disk load from Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 70 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 33 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 335 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 137 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 10 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf bytes 35328-102400 (code never loaded by a reference run) | 1 | 0 | 0 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf bytes 147968-151040 (code never loaded by a reference run) | 1 | 0 | 0 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf bytes 151552-152064 (code never loaded by a reference run) | 1 | 122 | 1 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf bytes 153088-155648 (code never loaded by a reference run) | 1 | 577 | 2 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf bytes 156672-157696 (code never loaded by a reference run) | 1 | 331 | 3 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf bytes 158208-162816 (code never loaded by a reference run) | 1 | 283 | 1 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf bytes 480768-484352 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: custom-format (non-DOS) disk, 969 non-empty blocks: matched against track-loaded segments
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: the boot block reads 22528 bytes from disk offset 574464 to an address (found statically); recovered as a unit
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: the boot block reads 1024 bytes from disk offset 563200 to an address (found statically); recovered as a unit
- runtime segment an address: 32 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 104 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 84 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: bytes 35328-102400 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: bytes 147968-151040 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: bytes 151552-152064 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: bytes 153088-155648 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: bytes 156672-157696 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: bytes 158208-162816 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: bytes 480768-484352 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: 9 logged loads cover 105984 bytes of the image
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: 27 track-loaded blocks found in recovered runtime segments and classified through them
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 13-62 (bytes 6656-32256) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.54)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 65-67 (bytes 33280-34816) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 210-211 (bytes 107520-108544) outside the filesystem hold data, not code (opcode density 0.041, entropy 4.99)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 221-287 (bytes 113152-147456) outside the filesystem hold data, not code (opcode density 0.033, entropy 5.00)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 319-321 (bytes 163328-164864) outside the filesystem hold data, not code (opcode density 0.039, entropy 3.55)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 323-323 (bytes 165376-165888) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.61)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 325-325 (bytes 166400-166912) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.68)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 353-353 (bytes 180736-181248) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.70)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 355-599 (bytes 181760-307200) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.90)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 601-677 (bytes 307712-347136) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.03)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 915-929 (bytes 468480-476160) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.04)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 947-953 (bytes 484864-488448) outside the filesystem hold data, not code (opcode density 0.057, entropy 5.21)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a2]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf: blocks 955-960 (bytes 488960-492032) outside the filesystem hold data, not code (opcode density 0.059, entropy 5.19)
- 3 never-loaded region(s), 73728 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 6 packed track region(s), 135168 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf:bootblock

- `load_and_unpack_bootblock_payload`: Copies a packed table into memory and then decodes it backwards from its end.

### logged disk load from Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf boot-block load

- `decode_packed_bootblock_stream`: Decodes a packed boot-block payload backwards into memory using the same bit reader scheme of literal bytes and back-references.
- `shift_bit_reader_word`: Writes the low byte of a stored value into the output word, then returns the source long shifted right by one bit with its top bit set.
- `shift_bit_buffer_by_count`: Shifts a bit buffer right by a count taken from a register, refilling it through a helper call whenever it empties.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_interrupt_status`: Reads bits from a hardware register and repeatedly calls a subroutine stored in a global variable based on the resulting value.
- `call_global_repeatedly_2`: Calls a function stored in a global variable multiple times, doubling the return value and checking if it is negative before subsequent calls.
- `call_global_repeatedly`: Calls a global function multiple times, multiplying its return value by two and checking if it is negative before subsequent calls.
- `execute_conditional_function_calls`: Repeatedly calls a function stored in a global variable and performs arithmetic on the return value.
- `call_global_conditionally`: Calls a global function and multiplies the result by two, performing a conditional second call if the value is negative.
- `call_global_once`: Calls a global function and performs a conditional second call if the result multiplied by two is negative.
- `call_global_single`: Calls a global function and returns.
- `increment_global_variable`: Increments a global variable by one.
- `decrement_global_counter`: Decrements a global counter by one and, when the result is zero, sets the top bit of that global.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_offset_subroutine`: Calls a subroutine at an address calculated by adding an offset to a pointer.
- `call_pointer_offset_subroutine`: Calls a subroutine using an offset relative to the address stored in a global variable.
- `execute_conditional_subroutine`: Calls a subroutine via a pointer and an offset, then conditionally calls another subroutine based on the result.
- `execute_conditional_loop`: Sets a bit in a global variable and repeatedly calls a subroutine via a pointer and an offset while the bit remains set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_complex_list`: Iterates through a data structure, updating pointers and counters, and executing subroutines retrieved from a table based on bitwise operations on list elements.
- `merge_and_update_list`: Copies a block of data from a source table to a destination and then updates a linked list structure.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 12 into d2, then returns.
- `fetch_next_element`: Iterates through a list of data to find a value with a specific bit pattern, updating a pointer and a counter as it progresses.
- `tiny_moveq_moveq_rts`: Small routine of 3 instructions that loads 0 into d0, then loads 8 into d2, then returns.
- `traverse_linked_lists`: Iterates through sixteen linked lists and executes a subroutine for each element that contains a valid pointer.
- `process_list_merge`: Iterates through a list to merge or copy values into a destination based on comparisons with a target value.
- `merge_and_copy_lists`: Performs a complex merge of two lists by copying values or summing them based on comparisons of signed short values.
- `merge_sorted_lists_2`: Merges two lists by summing values from corresponding indices using a loop that continues as long as elements are treated as signed shorts.
- `merge_sorted_lists`: Merges two lists by summing values from corresponding indices, using a loop that continues as long as list elements are treated as signed shorts.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_buffer_pointer`: Decrements a value in a structure and, if the result is non-zero or a specific bit is set, copies a word from one location to another within that structure.
- `remove_linked_list_node`: Decrements a counter in a structure and removes the node from a linked list if the counter and a flag reach zero.
- `call_other_routine`: Calls another routine and returns.
- `store_sum_on_zero`: Checks if a global variable is zero and, if so, stores the sum of two values into a memory location offset from a pointer.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `nop_placeholder`: This routine is empty and performs no operations.
- `blit_masked_tile_block_2`: Copies a rectangular block of pixel data from a source to a destination while applying bitwise masks to the edges.
- `blit_masked_tile_block`: Copies a block of data from a source to a destination using bitwise masking to handle partial transparency or edge clipping.
- `copy_memory_block`: Copies a large block of data from a source address to a destination address using a loop that moves data in fixed-size chunks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_variable`: Sets a global variable to zero.
- `execute_indirect_function`: Calls a function via a pointer stored in a global variable and updates that global variable based on the result.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it ends immediately.
- `compute_bitmask_from_table`: Iterates through a table of signed 16-bit values.
- `update_accumulated_values`: Adds values from a source table to a destination block and stores a third value into that block.
- `reset_three_word_fields`: Zeroes three consecutive 16-bit fields of a record at offset 30 and then calls another routine.
- `get_and_clear_values`: Reads two 16-bit values from a block, clears those values in the block, and returns them combined.
- `set_conditional_hunk_codes`: Sets two different pairs of 32-bit values into a record based on the value of a byte and then calls another routine.
- `set_hunk_codes`: Stores two specific 32-bit values into a record and then calls another routine.
- `store_flag_and_call`: Stores a byte and an address into a record and then calls a method on an object.
- `perform_io_operations`: Executes a series of system traps and stores a calculated value into a memory location.
- `reset_and_check_status`: Performs system traps to clear a byte in a block and returns a value from a specific offset.
- `store_argument_and_call_method`: Stores a flag byte and an address argument in the record, then calls a method on the object in another register.
- `call_first_node_entry_if_linked`: Follows the first link from the record and, when that node has a successor, calls the entry routine stored at offset 16 of the node.
- `store_node_fields_with_flag`: Stores two 32-bit values and a 16-bit value into the record at offsets 10, 14 and 18, then clears the flag byte at offset 20.
- `insert_node_by_key_byte`: Increments the counter of the node passed in an address register, walks the list in the record until it finds a node whose key byte at offset 14 is lower than the new node's key, and splices the new node after it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `build_bitmask_from_table`: Iterates through a list of values and builds a bitmask by checking specific bits within a table based on the value of each element.
- `get_signed_byte`: Returns the byte pointed to by an address register if it is non-negative, otherwise returns zero.
- `clear_and_return_words`: Reads two 16-bit values from a record, clears those locations in the record, and returns the combined 32-bit result.
- `set_hunk_type_values`: Sets two 32-bit values in a record based on the value of a byte, using constants associated with AmigaDOS hunk types.
- `execute_system_traps`: Performs a system trap, stores a calculated address into a record, and then performs two additional system traps.
- `perform_io_operations_2`: Executes a system trap, modifies a byte in a record, and conditionally executes further system traps based on the contents of a record and a flag byte.
- `traverse_and_call_list`: Walks a linked list and executes a function pointer stored within each node until the end of the list is reached.
- `update_status_flags`: Stores multiple values into a memory structure and sets a specific bit in a byte if a register is non-zero.
- `insert_into_linked_list`: Increments a value in a structure and performs a sorted insertion of a node into a linked list based on a comparison of byte values.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf bytes 151552-152064 (code never loaded by a reference run)

- `or_flag_and_return_field`: Sets high-bit flags in a byte just before the address register and returns a long field of the record.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf bytes 153088-155648 (code never loaded by a reference run)

- `release_object_with_refcount`: Calls virtual methods on up to three objects, choosing the path from the sign of the first method's result.
- `decrement_counter_and_call_hook`: Decrements a 16-bit counter reached through a pointer in the record, trapping on signed overflow, then calls a fixed routine.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf bytes 156672-157696 (code never loaded by a reference run)

- `execute_offset_commands_2`: Iterates through a string and executes subroutines at addresses calculated by adding an offset to a pointer.
- `execute_offset_commands`: Iterates through a string until a space is found, executing subroutines at addresses calculated by adding an offset to a pointer for each character.
- `copy_string_with_terminator`: Copies characters from a source to a destination until a specific delimiter or null terminator is encountered, then fills the remainder of the destination with a question mark character.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a2].adf bytes 158208-162816 (code never loaded by a reference run)

- `execute_indirect_call`: Copies a value from one memory location to another and then calls a subroutine using an offset stored at that location.
