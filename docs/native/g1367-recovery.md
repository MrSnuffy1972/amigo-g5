# Game 1367 (realms (1991)(virgin)(m3)[cr flt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01367). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1826496 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 16 of 16 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 72 of 72 named (0 author symbols), 72 of 72 described (15 by rule, 57 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 246 of 246 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 82432 bytes, 7 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1340 of 2089 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 31257 |
| code | 6950 |
| data | 899451 |
| padding | 6 |
| unused | 888832 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf:bootblock | 1 | 13 | 1 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf:bootblock | 1 | 19 | 2 |
| logged disk load from Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf boot-block load | 1 | 159 | 3 |
| logged disk load from Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 70 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 33 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 335 | 20 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 137 | 16 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 10 | 10 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf bytes 35328-102400 (code never loaded by a reference run) | 1 | 0 | 0 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf bytes 147968-151040 (code never loaded by a reference run) | 1 | 0 | 0 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf bytes 151552-152064 (code never loaded by a reference run) | 1 | 122 | 1 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf bytes 153088-155648 (code never loaded by a reference run) | 1 | 577 | 2 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf bytes 156672-157696 (code never loaded by a reference run) | 1 | 331 | 3 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf bytes 158208-162816 (code never loaded by a reference run) | 1 | 283 | 1 |
| Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf bytes 480768-484352 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: custom-format (non-DOS) disk, 969 non-empty blocks: matched against track-loaded segments
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: custom-format (non-DOS) disk, 803 non-empty blocks: matched against track-loaded segments
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: the boot block reads 22528 bytes from disk offset 574464 to an address (found statically); recovered as a unit
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: the boot block reads 1024 bytes from disk offset 563200 to an address (found statically); recovered as a unit
- runtime segment an address: 32 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 20 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 104 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 84 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 6 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: bytes 35328-102400 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: bytes 147968-151040 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: bytes 151552-152064 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: bytes 153088-155648 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: bytes 156672-157696 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: bytes 158208-162816 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: bytes 480768-484352 look like code no run loaded: recovered position-independently (load address unknown)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: 9 logged loads cover 105984 bytes of the image
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: 27 track-loaded blocks found in recovered runtime segments and classified through them
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 13-62 (bytes 6656-32256) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.54)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 65-67 (bytes 33280-34816) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.60)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 210-211 (bytes 107520-108544) outside the filesystem hold data, not code (opcode density 0.041, entropy 4.99)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 221-287 (bytes 113152-147456) outside the filesystem hold data, not code (opcode density 0.033, entropy 5.00)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 319-321 (bytes 163328-164864) outside the filesystem hold data, not code (opcode density 0.039, entropy 3.55)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 323-323 (bytes 165376-165888) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.61)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 325-325 (bytes 166400-166912) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.68)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 353-353 (bytes 180736-181248) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.70)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 355-599 (bytes 181760-307200) outside the filesystem hold data, not code (opcode density 0.000, entropy 6.91)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 601-677 (bytes 307712-347136) outside the filesystem hold data, not code (opcode density 0.006, entropy 7.03)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 915-929 (bytes 468480-476160) outside the filesystem hold data, not code (opcode density 0.005, entropy 7.04)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 947-953 (bytes 484864-488448) outside the filesystem hold data, not code (opcode density 0.057, entropy 5.21)
- realms (1991)(virgin)(m3)[cr flt](disk 1 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf: blocks 955-960 (bytes 488960-492032) outside the filesystem hold data, not code (opcode density 0.059, entropy 5.19)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 2-3 (bytes 1024-2048) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.63)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 6-6 (bytes 3072-3584) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.85)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 8-8 (bytes 4096-4608) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.40)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 10-10 (bytes 5120-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.00)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 12-232 (bytes 6144-119296) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.02)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 576-576 (bytes 294912-295424) hold ASCII text
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 578-578 (bytes 295936-296448) hold ASCII text
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 580-596 (bytes 296960-305664) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.83)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 598-614 (bytes 306176-314880) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.93)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 616-632 (bytes 315392-324096) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.96)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 634-650 (bytes 324608-333312) outside the filesystem hold data, not code (opcode density 0.004, entropy 1.96)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 652-668 (bytes 333824-342528) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.98)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 670-686 (bytes 343040-351744) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.00)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 688-704 (bytes 352256-360960) outside the filesystem hold data, not code (opcode density 0.003, entropy 2.03)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 706-722 (bytes 361472-370176) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.00)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 724-740 (bytes 370688-379392) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.88)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 742-758 (bytes 379904-388608) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.83)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 760-776 (bytes 389120-397824) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.93)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 778-794 (bytes 398336-407040) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.96)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 796-812 (bytes 407552-416256) outside the filesystem hold data, not code (opcode density 0.004, entropy 1.96)
- realms (1991)(virgin)(m3)[cr flt](disk 2 of 2)/Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf: blocks 814-829 (bytes 416768-424960) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.89)
- 3 never-loaded region(s), 73728 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 10 packed track region(s), 308224 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf:bootblock

- `initialize_and_copy_boot_data`: Configures hardware registers and calls a function to initialize the system before copying a block of data from a source to a destination in memory.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 2 of 2).adf:bootblock

- `initialize_hardware_and_copy_data`: Clears a block of hardware registers and then copies 256 longwords from a global source to a destination starting at a fixed memory address.
- `copy_data_block`: Copies a block of 256 longwords from a global source to a destination and enters an infinite loop.

### logged disk load from Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf boot-block load

- `process_bit_stream_data`: Iteratively examines the bits of a 32-bit value to determine a sequence of offsets and lengths.
- `store_concatenated_value`: Reads a 32-bit value from memory, stores it as two bytes at a destination, and returns a modified version of the original value.
- `loop_with_bit_shifts`: Repeatedly shifts a value right until it reaches zero, calling another routine during each iteration of a loop controlled by a counter.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_interrupt_requests`: Reads the interrupt status from a hardware register, performs bitwise operations, and calls one of three subroutines based on the resulting value.
- `process_interrupt_logic`: Reads a byte from a global variable, modifies the CIAA interrupt mask, and performs a series of conditional shifts and additions on the value before returning via an exception return.
- `call_pointer_at_offset_2156`: Loads a pointer from a memory location at offset 2156 and calls it, then branches to another location.
- `call_pointer_at_offset_215a`: Loads a pointer from a memory location at offset 215a and calls it, then branches to another location.
- `call_pointer_at_offset_215e`: Loads a pointer from a memory location at offset 215e and calls it, then branches to another location.
- `call_pointer_at_offset_2162`: Loads a pointer from a memory location at offset 2162 and calls it, then branches to another location.
- `call_pointer_at_offset_2166`: Loads a pointer from a memory location at offset 2166 and calls it, then branches to another location.
- `increment_interrupt_enable`: Writes a value from a global variable to the interrupt enable register and increments that global variable.
- `decrement_global_counter`: Decrements a global counter by one and, when the result is zero, sets the top bit of that global.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_offset_subroutine`: Calls a subroutine at an address calculated by adding an offset to a pointer.
- `call_pointer_offset_subroutine`: Loads a pointer from a global variable, reads a 16-bit offset at offset 118 of the pointed-to structure, calls the routine addressed by that offset relative to the pointer, then restores the registers and returns.
- `execute_conditional_subroutine`: Sets d0 to 82 and calls the routine at offset 82 of the structure in a6, returning if it reports success; otherwise sets d0 to 86 and calls the same routine, and only if that also reports success it loads a pointer from a global variable and calls the routine at address 76234.
- `execute_conditional_loop`: Sets a bit in a global variable and repeatedly calls a subroutine via a pointer and an offset while the bit remains set.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `process_complex_list`: Iterates through a data structure, updating pointers and counters, and executing subroutines retrieved from a table based on bitwise operations on list elements.
- `update_linked_list_and_copy_data`: Stores a pointer from a0 into a global variable, calls another routine, and if the result is not zero, increments a global 16-bit word, copies 62 words from a source table to the memory pointed to by a0 using a loop, updates a linked list by swapping pointers, calls another routine, and then modifies fields at offsets 12, 16, 40, and 10 of a structure in a2.
- `tiny_moveq_rts`: Small routine of 2 instructions that loads 12 into d2, then returns.
- `update_list_index`: Decrements a 16-bit value at offset 88 of a structure, or if the value is zero, iterates through a table to find a new index and updates offsets 80, 86, and 88.
- `tiny_moveq_moveq_rts`: Small routine of 3 instructions that loads 0 into d0, then loads 8 into d2, then returns.
- `traverse_linked_lists`: Iterates through sixteen entries of a table of pointers, and for each non-zero element reads a routine address from offset 18 and calls it when the signed value at offset 18 is non-negative.
- `branch_to_offset_routine`: Performs a short branch to an instruction located at a fixed offset within another routine.
- `apply_indexed_offset_additions_2`: Uses a loop to copy data from a source buffer to a destination buffer using offsets stored within the source data.
- `branch_on_register`: Copies the value from a2 to d1, then branches to a different location based on whether a2 is zero.
- `apply_indexed_offset_additions`: Iterates through a data stream in a0, using values from a table indexed by d1 to add values from a0 to a destination buffer in a1.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_buffer_pointer`: Checks bit 14 of the value at offset 64, decrements that value, and if the bit was set or the result is non-zero copies the longword at offset 68 to offset 40 of the structure in a2.
- `remove_node_if_zeroed`: Stores a word at offset 8 of a structure in a2, decrements a 16-bit counter at offset 16 of a pointed-to structure, and if that counter and a 16-bit flag at offset 8 both reach zero, removes the node from a linked list, calls another routine, and decrements a global variable, otherwise it returns.
- `call_routine_with_address`: Loads the sum of registers a3 and d4 into a0 and calls another routine.
- `store_sum_on_zero`: Tests a global word at offset 4134, and if it is zero, adds d4 to a3 and stores the result at offset 40 of a structure in a2, otherwise it returns.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `blit_masked_tile_block_2`: Copies a block of data from a source to a destination using bitwise masking to handle partial transparency or edge clipping.
- `blit_masked_tile_block`: Copies a block of data from a source to a destination using bitwise masking to handle partial transparency or edge clipping.
- `copy_memory_block`: Copies a large block of data from a source address to a destination address using a loop that moves data in fixed-size chunks.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clear_global_long`: Clears a long value in a global variable and returns.
- `execute_indirect_function`: Clears a word in a structure, then calls a routine via a pointer indexed by a value read from that structure.
- `test_bit_from_scaled_index`: Copies a 16-bit value from one register to another, shifts the second value right by three bits, and tests a bit in a table located at an offset of forty-eight plus the shifted value from a pointer in a6 before returning.
- `compute_bitmask_from_table`: Iterates through a table of signed 16-bit values.
- `update_accumulated_values`: Adds the two source values to the 16-bit accumulators at offsets 30 and 32 of the structure in a6, then stores the third source value into the 16-bit field at offset 34 of that structure.
- `reset_three_word_fields`: Zeroes three consecutive 16-bit fields of a record at offset 30 and then calls another routine.
- `get_and_clear_values`: Reads the 16-bit values at offsets 30 and 32 of the structure in a6, clears both of them, and returns them combined with the second value in the high word and the first value in the low word.
- `set_alternative_hunk_codes`: If a byte in d1 is non-zero, it stores two 32-bit values into a record at offsets 20 and 24 and branches to another location; otherwise it branches to another location.
- `set_hunk_codes`: Stores two specific 32-bit values into a record at offsets 20 and 24 and then calls another routine.
- `call_method_and_clear_word`: Calls a routine via a pointer and then clears a 16-bit word at offset 28 of the structure in a6.
- `perform_io_operations`: Executes a series of system traps and stores a calculated value into a memory location.
- `reset_and_check_status`: Performs system traps to clear a byte in a block and returns a value from a specific offset.
- `store_argument_and_call_method`: Stores a flag byte and an address argument in the record, then calls a method on the object in another register.
- `call_first_node_entry_if_linked`: Follows the first link from the record and, when that node has a successor, calls the entry routine stored at offset 16 of the node.
- `store_node_fields_with_flag`: Stores two 32-bit values and a 16-bit value into the record at offsets 10, 14 and 18, then clears the flag byte at offset 20.
- `insert_node_by_key_byte`: Increments the counter of the node passed in an address register, walks the list in the record until it finds a node whose key byte at offset 14 is lower than the new node's key, and splices the new node after it.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_3`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_4`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_5`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_6`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_7`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_8`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_9`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `jump_vector_entry_10`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf bytes 151552-152064 (code never loaded by a reference run)

- `modify_byte_and_return`: Performs a bitwise OR operation on a byte at a specific offset and returns a value from another memory location.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf bytes 153088-155648 (code never loaded by a reference run)

- `process_nested_structures`: Executes a series of function calls through pointers stored in various structures and decrements two separate 16-bit counters.
- `decrement_and_call`: Decrements a 16-bit value pointed to by a pointer derived from a global variable and then calls another routine.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf bytes 156672-157696 (code never loaded by a reference run)

- `execute_offset_commands_2`: Iterates through a string and executes subroutines at addresses calculated by adding an offset to a pointer for each character.
- `execute_offset_commands`: Iterates through a string until a space is found, executing subroutines at addresses calculated by adding an offset to a pointer for each character.
- `copy_string_with_terminator`: Copies characters from a source to a destination until a specific delimiter or null terminator is encountered, then fills the remainder of the destination with a question mark character.

### Realms (1991)(Virgin)(M3)[cr FLT](Disk 1 of 2).adf bytes 158208-162816 (code never loaded by a reference run)

- `execute_indirect_call`: Copies a value from one memory location to another and then calls a subroutine using an offset stored at that location.
