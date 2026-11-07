# Game 1614 (realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01614). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

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
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf:bootblock | 1 | 13 | 1 |
| logged disk load from Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf boot-block load | 1 | 159 | 3 |
| logged disk load from Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 70 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 33 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 335 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 137 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 10 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf bytes 35328-102400 (code never loaded by a reference run) | 1 | 0 | 0 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf bytes 147968-151040 (code never loaded by a reference run) | 1 | 0 | 0 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf bytes 151552-152064 (code never loaded by a reference run) | 1 | 122 | 1 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf bytes 153088-155648 (code never loaded by a reference run) | 1 | 577 | 2 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf bytes 156672-157696 (code never loaded by a reference run) | 1 | 331 | 3 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf bytes 158208-162816 (code never loaded by a reference run) | 1 | 283 | 1 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf bytes 480768-484352 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: custom-format (non-DOS) disk, 969 non-empty blocks: matched against track-loaded segments
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: the boot block reads 22528 bytes from disk offset 574464 to an address (found statically); recovered as a unit
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: the boot block reads 1024 bytes from disk offset 563200 to an address (found statically); recovered as a unit
- runtime segment an address: 32 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 104 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 84 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: bytes 35328-102400 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: bytes 147968-151040 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: bytes 151552-152064 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: bytes 153088-155648 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: bytes 156672-157696 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: bytes 158208-162816 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: bytes 480768-484352 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: 9 logged loads cover 105984 bytes of the image
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: 27 track-loaded blocks found in recovered runtime segments and classified through them
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 13-62 (bytes 6656-32256) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.54)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 65-67 (bytes 33280-34816) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 210-211 (bytes 107520-108544) outside the filesystem hold data, not code (opcode density 0.041, entropy 4.99)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 221-287 (bytes 113152-147456) outside the filesystem hold data, not code (opcode density 0.033, entropy 5.00)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 319-321 (bytes 163328-164864) outside the filesystem hold data, not code (opcode density 0.039, entropy 3.55)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 323-323 (bytes 165376-165888) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.61)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 325-325 (bytes 166400-166912) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.68)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 353-353 (bytes 180736-181248) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.70)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 355-599 (bytes 181760-307200) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.90)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 601-677 (bytes 307712-347136) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.03)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 915-929 (bytes 468480-476160) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.04)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 947-953 (bytes 484864-488448) outside the filesystem hold data, not code (opcode density 0.057, entropy 5.21)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)[a]/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf: blocks 955-960 (bytes 488960-492032) outside the filesystem hold data, not code (opcode density 0.059, entropy 5.19)
- 3 never-loaded region(s), 73728 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 6 packed track region(s), 135168 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf:bootblock

- `load_packed_blocks_and_unpack`: Sets up two device read requests and runs them through the exec library's I/O call, then copies a block of data to a fixed address.

### logged disk load from Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf boot-block load

- `unpack_marker_run_stream`: Copies a block of data and then decodes a packed bit stream with a bit reader.
- `refill_bit_reader_longword`: Fetches the previous longword of the packed stream, writes its low byte to the caller's buffer, and returns the value shifted right by one bit with its top bit set, as a bit-reader refill step.
- `discard_bits_from_buffer`: Shifts a bit buffer right by the number of bits held in a register, refilling it from the packed stream through a routine whenever it runs empty.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_and_call_nested_pointers`: Checks certain bits in a hardware register and, if certain conditions are met, calls a routine pointed to by a global variable, repeating this check up to three times.
- `execute_repeated_function`: Sets a global variable and then calls a global function multiple times if certain bitwise or arithmetic conditions are met.
- `execute_repeated_calls`: Calls a global function multiple times, doubling a value and checking if it is negative before deciding whether to call it again.
- `execute_limited_calls`: Calls a global function and checks if a doubled value is negative to determine if another call should occur.
- `test_global_value_bounds`: calls a global function and checks if the returned value or its doubled value is negative.
- `check_and_execute_function`: calls a global function and, if the returned value or its doubled value is negative, calls the same function a second time.
- `execute_global_function`: Executes a routine stored in a global variable.
- `increment_global_variable`: Increments a global variable and modifies a hardware register.
- `decrement_and_set_interrupt_bit`: Decrements a global counter and, if the result is zero, sets the top bit of that global, which is used to enable interrupts via a hardware register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `execute_offset_call`: Executes a routine at an address calculated by adding a short offset to a value stored in a memory location.
- `call_indirect_from_global`: Executes a routine at an address calculated by adding an offset found in a table to a pointer stored in a global variable.
- `execute_indirect_calls`: Executes a routine and, if the result is false, attempts to execute another routine using an offset from a table and a pointer in a global variable.
- `execute_indirect_loop`: Sets a bit in a global variable and then enters a loop that executes a routine using a pointer and an offset from a table.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `traverse_and_update_list`: Traverses a linked list, updating counters and pointers, and potentially updates a global pointer or modifies a list head.
- `apply_relocation_data`: Executes a complex relocation routine and then copies a block of words from a table to a destination pointer.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 12 into d2, then returns.
- `process_linked_list_elements`: Decrements a value in a structure or, if it is zero, traverses a linked list of structures, updating a pointer and a counter or value in a structure if certain conditions are met.
- `tiny_moveq_moveq_rts`: Small routine of 3 instructions that loads 0 into d0, then loads 8 into d2, then returns.
- `iterate_list_and_call_functions`: Iterates through a list of pointers, and for each non-zero pointer, checks a value in a structure and calls a routine if that value is non-negative.
- `execute_complex_relocation`: Executes a routine that copies or adds values from a source table to a destination array using complex conditional checks on index values.
- `apply_relocation_logic`: Performs a complex sequence of copying and adding values from a source table to a destination array based on certain index comparisons.
- `apply_offset_to_array_2`: Iterates through a list of offsets and adds a value from a source array to a destination array.
- `apply_offset_to_array`: Iterates through a list of offsets and applies them to a destination array, updating the destination array with a value from a source array.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_structure_field_conditionally`: Decrements a value in a structure and, if the result is non-zero or a specific bit is set, copies a 4-byte value from one part of the structure to another.
- `update_linked_list_node`: Decrements a counter in a structure and, if it reaches zero, removes the node from a linked list and updates a global counter.
- `execute_empty_routine`: the routine is empty and performs no operations before returning.
- `add_to_global_offset`: Adds two values together and stores the result into a memory location if a global variable is zero.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `return_empty_routine`: The routine is empty and performs no operations before returning.
- `copy_padded_memory_blocks`: copies a series of memory blocks from a source to a destination while applying bitmasks to clear or preserve specific bits in the padding or certain memory offsets.
- `copy_memory_block_with_padding`: copies a block of memory or a series of memory blocks from a source to a destination, applying bitmask-based padding or clearing to the start and end of certain memory segments or or certain offsets within the block.
- `copy_memory_block`: copies a fixed-size block of memory using a loop that decrements a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_global_variable`: Sets a global variable to zero.
- `execute_indirect_function_and_set_global`: Calls a function pointer located at an offset from a global variable and then sets a global variable to a constant value.
- `empty_stub_routine`: The routine contains four instructions and performs no observable work; it ends immediately.
- `compute_bitmask_from_table`: Iterates through a table of signed 16-bit values.
- `add_to_global_values`: Adds values from a source pointer to a global state block and returns a constant.
- `clear_three_word_fields`: Clears three consecutive 16-bit fields within an object record to zero and then calls another routine.
- `clear_and_return_values`: Clears two words in a global state block and returns the original values.
- `set_conditional_hunk_codes`: Checks a byte and stores one of two possible pairs of 32-bit values into a record before calling another routine.
- `set_hunk_codes`: Stores two specific 32-bit values into a record and then calls another routine.
- `set_object_flag`: Stores a zero byte and an address into an object field and calls a method.
- `update_state_with_trap_calls`: Uses a trap to read a value, adds it to a global variable, stores the result in a global state block, and uses further traps.
- `set_flag_and_branch_on_trap`: Uses a trap to clear a memory location, sets a bit in a flag byte, and uses traps to check a global value and a flag before returning a value.
- `store_argument_and_call_method`: Stores a flag byte and an address argument in the record, then calls a method on the object in another register.
- `call_first_node_entry_if_linked`: Follows the first link from the record and, when that node has a successor, calls the entry routine stored at offset 16 of the node.
- `store_node_fields_with_flag`: Stores two 32-bit values and a 16-bit value into the record at offsets 10, 14 and 18, then clears the flag byte at offset 20.
- `insert_node_by_key_byte`: Increments the counter of the node passed in an address register, walks the list in the record until it finds a node whose key byte at offset 14 is lower than the new node's key, and splices the new node after it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `test_bitset_and_accumulate`: Iterates through a list of words, checking a bitset in a global buffer and setting a bit in a return register if a bit is set.
- `get_byte_or_zero`: Returns a byte from a pointer if it is non-negative, otherwise returns zero.
- `clear_two_words`: Reads two words from a global buffer and then clears those same words in the buffer.
- `set_hunk_addresses`: Sets two 32-bit values at specific offsets in a global buffer based on the value of a byte, then calls another routine.
- `update_pointer_with_trap`: Performs a system trap, adds a value from a register to a global variable, and stores the result in a buffer before performing further traps.
- `execute_conditional_traps`: Performs a trap, clears a 32-bit value in a buffer, sets a bit in a flag byte, and executes further traps depending on the state of certain buffer values.
- `traverse_and_execute_list`: Walks a linked list of pointers and executes the function pointer located at an offset within each node.
- `update_status_flag`: stores a register or a value or a register or a value into a structure or a table and sets a bit in a status byte if a certain condition is met.
- `insert_into_list`: Increments a value in a structure and traverses a linked list to insert a new node, updating pointers to maintain the list order.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf bytes 151552-152064 (code never loaded by a reference run)

- `or_byte_and_return_field`: Sets bits in a byte just before the address register and returns a longword field of a record.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf bytes 153088-155648 (code never loaded by a reference run)

- `run_object_lifecycle_methods`: Calls methods through an object's function table, with some calls conditional on flag bits and a counter field decremented on each pass, and returns when the flags or call results show completion.
- `decrement_counter_and_call`: Decrements a word counter at a field of a record, traps on signed overflow, and calls one unnamed routine.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf bytes 156672-157696 (code never loaded by a reference run)

- `decrement_and_dispatch_routine`: Decrements a value stored at a memory location pointed to by a register and executes a routine at an offset, then executes another routine using a pointer-based offset lookup.
- `execute_offset_calls`: Iterates through a buffer and executes a routine at an address calculated by adding a short offset to a value stored in a memory location.
- `copy_string_until_delimiter`: Copies characters from a source buffer to a destination buffer until a null, backslash, or dot is encountered, or a counter reaches zero.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2)[a].adf bytes 158208-162816 (code never loaded by a reference run)

- `execute_indirect_call`: Copies a value from a memory location and executes a routine at an address calculated by adding an offset to that value.
