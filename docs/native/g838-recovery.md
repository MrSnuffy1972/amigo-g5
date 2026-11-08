# Game 838 (ThunderStrike v1.0 0406): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00838). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2056487 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 67 of 67 named (0 author symbols), 67 of 67 described (6 by rule, 61 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 619 of 619 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 8704 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 837 of 837 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 272949 |
| code | 2882 |
| data | 1775011 |
| padding | 13 |
| unused | 5632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 137 | 13 |
| Disk.1:bootblock | 1 | 8 | 2 |
| logged disk load from Disk.1 | 1 | 690 | 50 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 2 |
| Disk.1 bytes 281600-290304 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- ThunderStrike_v1.0_0406/ThunderStrike_v1.0_0406_hdf/ThunderStrike/Disk.1: custom-format (non-DOS) disk, 1733 non-empty blocks: matched against track-loaded segments
- ThunderStrike_v1.0_0406/ThunderStrike_v1.0_0406.hdf: hard-disk image (AmigaDOS OFS, 2208 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree ThunderStrike_v1.0_0406_hdf, which is recovered file by file; no block outside the filesystem holds data
- 1 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 2 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 2 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- ThunderStrike_v1.0_0406/ThunderStrike_v1.0_0406_hdf/ThunderStrike/Disk.1: bytes 281600-290304 look like code no run loaded: recovered position-independently (load address unknown)
- ThunderStrike_v1.0_0406/ThunderStrike_v1.0_0406_hdf/ThunderStrike/Disk.1: 3 logged loads cover 289280 bytes of the image
- ThunderStrike_v1.0_0406/ThunderStrike_v1.0_0406_hdf/ThunderStrike/Disk.1: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- ThunderStrike_v1.0_0406/ThunderStrike_v1.0_0406_hdf/ThunderStrike/Disk.1: blocks 330-549 (bytes 168960-281600) outside the filesystem hold data, not code (opcode density 0.016, entropy 6.77)
- ThunderStrike_v1.0_0406/ThunderStrike_v1.0_0406_hdf/ThunderStrike/Disk.1: blocks 567-945 (bytes 290304-484352) outside the filesystem hold data, not code (opcode density 0.007, entropy 6.55)
- ThunderStrike_v1.0_0406/ThunderStrike_v1.0_0406_hdf/ThunderStrike/Disk.1: blocks 1186-1759 (bytes 607232-901120) outside the filesystem hold data, not code (opcode density 0.001, entropy 5.50)
- 1 never-loaded region(s), 8704 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `update_adkcon` (Disk.1 +an address loaded at an address (157696 bytes)): Moves a byte from the address in a1 to d1, extends it to a word, clears bit 15 of d1, and writes the result to the ADKCON register before branching to another routine. Kind: disk hardware/ADKCON check. Pass path: The routine appears to be a custom disk-access/command parser that validates specific byte sequences (likely hardware-dependent timing or sector data) against a table. A successful check allows the execution to proceed to the next instructi.

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `patch_memory_and_jump`: Calls two routines, then iterates through memory starting at a specific address to find a pattern, replaces it with a new value and a pointer, and finally performs a jump to a location stored in a6.
- `wait_for_bit_and_store`: Polls a bit in a global variable until it matches a specific value, then stores a register value into two consecutive memory locations.
- `check_hardware_status`: Calls another routine with a specific value, checks bits 6 and 7 of a hardware register, and if those bits are set and a comparison with a value from a hardware register fails, it enters a loop; otherwise, it calls the same routine again with a different value.
- `enter_infinite_loop`: Performs an infinite loop that never returns.
- `store_word_and_branch`: Stores a 16-bit word from a register into an offset of 88 of a structure in a6 and branches to another routine.
- `update_offset_value_and_branch`: Stores a 16-bit word from register d2 into a global variable at an offset of 88 from the base address in a6, then performs a relative branch into another routine.
- `store_word_and_branch_2`: Stores a 16-bit word from a register into an offset of 88 of a structure in a0 and branches to another routine.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `update_obfuscated_value`: Performs arithmetic and bitwise operations on a global variable based on a carry flag.
- `bitwise_or_to_memory`: Performs a bitwise OR operation between a register and a value at a memory location.
- `call_function_via_pointer`: Multiplies a 16-bit register by 512, then calls a function through a pointer stored in a table.
- `return_without_action`: The routine takes its entry, makes no calls, reads or writes no data, and returns to its caller.

### Disk.1:bootblock

- `branch_forward`: Branches to a different location.
- `copy_data_with_loop`: Copies data from a source to a destination using a loop and then jumps to a new location.

### logged disk load from Disk.1

- `calculate_scaled_values`: Stores two values into global variables and performs division on shifted versions of input registers.
- `calculate_scaled_values_alt`: Stores two values into global variables and performs division using values from a table.
- `store_two_words`: Stores two words from registers d1 and d2 into global variables.
- `store_three_values`: Stores three specific values into three different global variables.
- `update_conditional_flags`: Calls another routine, then checks bits in a hardware register and a byte from a table to conditionally store values into a structure in a6.
- `update_conditional_flag`: Checks bit 3 of a register and a byte value to conditionally set a byte at offset 19 of a structure in a6 to one.
- `process_mouse_movement`: Reads a 16-bit value from the mouse port, calculates differences against a global value, and updates two scaled global variables using multiplication and division with clamping.
- `store_word_to_global`: Stores a 16-bit word from register d3 into a global variable at offset 26 of a6.
- `tiny_ori`: Small routine of 1 instruction that merges 78 into d0.
- `branch_if_equal`: Branches to another routine if a condition is met; otherwise it returns.
- `compare_byte_with_global`: Compares a byte in register d0 with a byte in a global variable.
- `branch_if_equal_2`: Branches to another routine if the zero flag is set.
- `compare_byte_and_branch`: Compares a byte in a register with a byte at a global variable and branches to another routine if they are equal; otherwise it returns.
- `set_byte_on_match`: Compares a byte against several global values and sets a byte at a calculated offset to one if a match is found.
- `calculate_bitmask`: Reads joystick and fire button status from hardware registers to generate a bitmask in register d0 through bitwise operations and conditional testing.
- `update_game_state`: Checks the status of a hardware bit, increments three global byte counters, and calls another routine.
- `store_scaled_values`: Loads two words from memory, shifts both right by 2 bits, increments them by 1, and calls another routine.
- `mask_and_store_word`: Applies a bitmask to a word and stores the result in two separate global word locations.
- `load_and_branch`: Loads a word from a memory location into a register and then branches to another location.
- `update_pointer_tables`: Calls another routine, loads a long value into a global variable from a specific address, calls another routine to update two global words, and then loads a different long value into a global variable.
- `process_byte_table`: Iterates through a table of bytes, performing bitwise operations and updating global values based on the content of the table.
- `process_and_distribute_bytes`: Reads bytes from a table using an offset, performs bitwise operations, and writes the results to multiple global locations based on a bitmask.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `modify_memory_blocks`: Performs bitwise AND and OR operations on four different locations in a memory table using values from registers.
- `update_audio_parameters`: Sets a bit in a hardware register, writes to a hardware register, and then iterates through a loop to update audio channel parameters, sample pointers, and hardware registers based on a structure in a0.
- `step_channel_countdown_table`: Decrements a per-channel countdown byte; when it reaches zero it reads the next duration from a table, ending the step if that value is zero.
- `update_audio_channel_parameters`: Calculates and updates audio hardware registers and sample playback parameters based on a structure in a0, including sample pointer increments, volume adjustments, and period values.
- `reset_audio_channel_state`: Clears five per-channel state bytes, then writes two values to a shared control word, first all bits set and then a low nibble, which the facts show as audio DMA and ADKCON register writes.
- `increment_and_jump`: Increments a byte from a structure in a0, compares it to another byte in the structure, and if they match, adds the value to a word in a structure and uses a table to update a pointer before branching.
- `move_and_branch_2`: Moves a byte from the address pointed to by a1 to an offset of 187 from a3 and then branches.
- `set_channel_bits`: Sets bit 5 and bit 1 of a byte at the address in a0, then branches to another routine.
- `set_single_bit`: Sets bit 5 of a byte at the address in a0 and then branches to another routine.
- `move_and_branch_offset`: Moves a byte from the address pointed to by a1 plus an offset of 26 from a0 and then branches.
- `clear_and_set_bits`: Moves three bytes from the address pointed to by a1 into a structure in a0, clears a word at an offset in a0, sets bit 1 of a byte at an offset in a0, and then branches.
- `update_status_byte`: Moves a byte from the address pointed to by a1 plus an offset of 24 from a0 into a0, moves another byte into a0, modifies it, and sets bit 6 of a byte at offset 7 of the structure in a0 before branching.
- `store_byte_at_offset_18`: Stores a byte from the address pointed to by a1 into offset 18 of the structure in a0, then branches to another routine.
- `set_multiple_bits`: Sets bit 7 and bit 3 of a byte at the address in a0, then branches to another routine.
- `clear_byte_and_branch`: Clears the byte at the address in a0 and then branches to another routine.
- `update_adkcon`: Moves a byte from the address in a1 to d1, extends it to a word, clears bit 15 of d1, and writes the result to the ADKCON register before branching to another routine.
- `process_channel_data`: Decrements a byte at offset 19 of a structure in a0, and if it is not zero, performs bitwise operations and uses a table to jump to another location; otherwise, it modifies bytes at offset 15 or 20 of the structure and jumps.
- `store_byte_at_offset_28`: Stores a byte from the address pointed to by a1 into offset 28 of the structure in a0, then branches to another routine.
- `store_byte_at_offset_188`: Stores a byte from the address pointed to by a1 into offset 188 of the structure in a3, then branches to another routine.
- `decrement_and_branch_on_zero`: Decrements a byte at offset 32 of the structure in a0 and branches to another routine if the result is zero; otherwise, it updates a1 by adding a 16-bit word from offset 12 of the structure in a0 to the value in a3 and branches to another routine.
- `modify_structure_data`: Moves a byte from the address in a1 to offset 32 of the structure in a0, subtracts the value in a3 from a1, writes the new a1 to offset 12 of the structure in a0, adds the value in a3 back to a1, and branches to another routine.
- `write_constant_byte`: Writes the value 2 to offset 32 of the structure in a0 and then branches to another routine.
- `move_and_branch`: Moves a byte from a pointer in a1 to an offset of 181 of a structure in a3, then branches to another routine.
- `update_channel_parameters`: Updates various fields in a structure in a0, including a byte at offset 22, a word at offset 2, and potentially a word at offset 30 or a DMA register based on bit tests.
- `load_table_entry_into_channel`: Looks up a table entry at the given index, saves the index in the channel's table position field, stores the looked-up value as the channel's current level, and sets its counter to one.
- `update_channel_sample_pointer_length`: For a channel whose start delay is running, it counts the delay down, clamps a negative offset to zero on expiry, and writes the sample pointer and word length into the channel's audio hardware slot, then writes a DMA control value.
- `reset_and_load_channel_table`: Clears the channel state table of sixteen records, marks two global bytes with a full-set sentinel value, then loads four channel start entries from a table selected by the argument, setting each channel's pointer, length and sentinel fields.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
