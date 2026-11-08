# Game 1282 (Ruffian v1.1 2411): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01282). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (6061554 top-level bytes in 27 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 7 of 7 units identical (bytes and relocations), 13 raw-word lines |
| Functions named and described | pass, 198 of 198 named (0 author symbols), 198 of 198 described (6 by rule, 149 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 2424 of 2424 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 4496 of 4496 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 290599 |
| code | 16016 |
| data | 5603896 |
| padding | 3 |
| unused | 151040 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 251 | 15 |
| logged disk load from Disk.1 | 1 | 793 | 28 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 116 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 616 | 39 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2717 | 111 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 3 |

## Cases handled

- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: raw disk image (895488 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.2: custom-format (non-DOS) disk, 1673 non-empty blocks: matched against track-loaded segments
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.3: custom-format (non-DOS) disk, 1633 non-empty blocks: matched against track-loaded segments
- Ruffian_v1.1_2411/Ruffian_v1.1_2411.hdf: hard-disk image (AmigaDOS OFS, 6496 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree Ruffian_v1.1_2411_hdf, which is recovered file by file; no block outside the filesystem holds data
- 2 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 61 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 262 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 1223 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 3 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: 2 logged loads cover 283648 bytes of the image
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 1148-1359 (bytes 293888-348160) hold a S404: StoneCracker v4.10 stream (54272 -> 106036 bytes)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 1366-1744 (bytes 349696-446720) hold a S404: StoneCracker v4.10 stream (97024 -> 206108 bytes)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 1798-3044 (bytes 460288-779520) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.69)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3050-3056 (bytes 780800-782592) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.61)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3058-3061 (bytes 782848-783872) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.73)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3076-3173 (bytes 787456-812544) hold a S404: StoneCracker v4.10 stream (25088 -> 65536 bytes)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3180-3182 (bytes 814080-814848) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.81)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3184-3192 (bytes 815104-817408) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.09)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3196-3283 (bytes 818176-840704) hold a S404: StoneCracker v4.10 stream (22528 -> 65536 bytes)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3312-3314 (bytes 847872-848640) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.27)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3316-3328 (bytes 848896-852224) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.67)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3338-3432 (bytes 854528-878848) hold a S404: StoneCracker v4.10 stream (24320 -> 65536 bytes)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3442-3460 (bytes 881152-886016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.81)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3462-3490 (bytes 886272-893696) hold a S404: StoneCracker v4.10 stream (7424 -> 32768 bytes)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.1: chunks 3494-3497 (bytes 894464-895488) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.28)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.2: blocks 2-652 (bytes 1024-334336) outside the filesystem hold data, not code (opcode density 0.004, entropy 5.93)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.2: blocks 660-671 (bytes 337920-344064) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.88)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.2: blocks 673-726 (bytes 344576-372224) hold a S404: StoneCracker v4.10 stream (27648 -> 65536 bytes)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.2: blocks 730-794 (bytes 373760-407040) hold 1 packed streams (S404: StoneCracker v4.10) found by scanning, 84% of the bytes
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.2: blocks 798-809 (bytes 408576-414720) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.97)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.2: blocks 811-818 (bytes 415232-419328) hold a S404: StoneCracker v4.10 stream (4096 -> 32768 bytes)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.2: blocks 824-1034 (bytes 421888-529920) hold 1 packed streams (S404: StoneCracker v4.10) found by scanning, 98% of the bytes
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.2: blocks 1086-1737 (bytes 556032-889856) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.27)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.2: blocks 1752-1759 (bytes 897024-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.45)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.3: blocks 61-113 (bytes 31232-58368) hold 1 packed streams (S404: StoneCracker v4.10) found by scanning, 86% of the bytes
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.3: blocks 127-783 (bytes 65024-401408) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.44)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.3: blocks 946-948 (bytes 484352-485888) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.85)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.3: blocks 950-958 (bytes 486400-491008) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.79)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.3: blocks 960-1035 (bytes 491520-530432) hold a S404: StoneCracker v4.10 stream (38912 -> 71976 bytes)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.3: blocks 1098-1748 (bytes 562176-895488) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.76)
- Ruffian_v1.1_2411/Ruffian_v1.1_2411_hdf/Ruffian/Disk.3: blocks 1757-1759 (bytes 899584-901120) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.92)
- 4 packed track region(s), 95232 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `modify_global_bitmask` (runtime segment an address): Sets a global variable to specific values and performs bitwise operations involving a bit shift. Kind: checksum/bitmask validation trap. Pass path: the branch not taken by the bne.b instruction (the fall-through path).
- `poll_and_clear_memory` (runtime segment an address): performs a loop that checks bits in a global variable and clears a word in a table based on a calculated index. Kind: disk hardware/status polling loop. Pass path: the loop terminates when (DAT_00bfe001 & 4) == 0 or sVar1 reaches -1, proceeding to the instruction after the call site.
- `update_global_state` (runtime segment an address): modifies bits in a global variable based on a register value and updates a table entry if the value differs. Kind: disk hardware/timing check. Pass path: the branch where sVar3 == 0 (no update needed) or the loop completes without triggering a failure state.
- `update_value_with_loop` (runtime segment an address): Compares a value against a table entry, updates the entry, and executes a loop that toggles bits in a global variable. Kind: disk hardware timing/sync check. Pass path: sVar3 == 0 (no loop executed).
- `toggle_bit_in_loop` (runtime segment an address): Clears and sets a specific bit in a global variable within a loop that executes a set number of times. Kind: bit-toggling loop/timing check. Pass path: loop completes normally without triggering a crash or jump to error handler.
- `manipulate_bitmask` (runtime segment an address): writes a value to memory and performs bitwise operations on a global variable using a loop. Kind: disk hardware/timing check. Pass path: the code continues execution normally (the branch after the call is not taken or the routine returns without triggering a crash/reset).

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `init_then_jump_to_start`: Sets a few globals, stores a library base and makes five calls, then writes a jump instruction, calls through a library pointer at offset 100 and transfers control to the start label.
- `replace_marker_values`: Unpacks a compressed stream and then iterates through a table, replacing specific short values with a different constant until a terminator is reached.
- `wait_flag_edge_then_call`: Waits for a byte flag at offset 5 to become set and then clear, and then calls through a pointer slot at offset 32.
- `apply_hunk_relocations`: Scans an AmigaDOS-style executable for the code and relocation hunk markers by comparing values with their hunk type codes, then adds the load base to each longword in the relocation table.
- `unpack_backward_compressed_stream`: Walks a packed input backward from a table header, reading 16-bit words and processing each in inner loops of up to sixteen steps, and returns one when the input is finished.
- `dispatch_by_threshold_call`: Chooses one of two targets by comparing a 16-bit argument with fifteen, then calls the chosen routine through an indirect jump.
- `combine_conditions_and_store`: Combines a couple of conditions into a single boolean result and stores it in a global variable, then returns without calling anything else.
- `copy_back_reference_bytes`: Copies a run of bytes backward from a source offset into the output, stepping a count down to minus one, then checks an end bound.
- `copy_back_reference_bytes_2`: Copies a run of bytes backward from a source offset into an output buffer, stepping a count down to minus one and stopping when the destination pointer reaches an end bound.
- `scale_value_by_512`: Multiplies a 16-bit value by 512 after subtracting eleven when a global equals one, and returns the result of a table-slot call at offset 40 instead when a second argument is non-zero.
- `xor_block_with_descending_key`: XORs a block of bytes with a descending key from 223 to zero after calling through two table slots, the second only when the first result is non-zero.
- `call_pointer_then_optional_slot`: Calls through a computed pointer and returns its result, also calling a table slot at offset 12 and one unnamed routine when a flag global is zero.
- `xor_block_with_descending_key_2`: XORs a block of bytes with a descending key from 223 to zero.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.

### logged disk load from Disk.1

- `copy_bytes_to_four_offsets`: Increments a global counter, then copies bytes from a source into four destination offsets spaced 40 bytes apart, two columns at a time over eleven rows.
- `distribute_byte_groups_across_rows`: Copies five-byte groups from a source into destination positions spaced 40 bytes apart over a fixed number of groups.
- `fill_region_with_ones`: Calls one unnamed routine, then writes all-ones longwords backward through a region in blocks of ten.
- `spin_in_infinite_loop`: Its decompiled body is an empty infinite loop.
- `distribute_three_byte_groups`: Copies three-byte groups from a source into three destination rows spaced 40 bytes apart, filling a block of columns on each pass.
- `repeat_pair_of_calls`: Repeats a pair of unnamed calls over a 16-bit counter until it reaches minus one.
- `find_highest_set_bit`: Searches a 32-bit value from bit 31 downward for the highest set bit and returns as soon as it finds one, doing nothing for zero.
- `draw_bit_mask_run_pair`: Computes two destination addresses from a row and column index using a stride of 200 bytes, clears a byte at each, and writes a single-bit mask pattern into them and along a run of following bytes.
- `clear_byte_pairs_above_below`: Zeroes two bytes one row above and two bytes one row below a position, using a row stride of 200 bytes.
- `copy_clipped_three_plane_columns`: Copies three bytes per column from a source into three rows spaced 40 bytes apart, skipping columns outside a clipping window and stepping the row base by 200 bytes.
- `run_four_calls_every_third`: Counts passes in a global and, on every third pass, resets the counter and makes four calls to unnamed routines.
- `copy_triple_longs_into_rows`: When a flag is set, copies five groups of three longwords into rows, with values 40 bytes apart within a row and the destination advancing 200 bytes per group.
- `step_table_driven_position`: Advances a position by a table-driven step, reversing direction past five and reading the next table entry unless it is the terminator 128.
- `shift_merge_word_groups`: Reads five groups of three words spaced 36 bytes apart, shifts each word right by an amount taken from a global, and merges the results into output longwords.
- `load_five_triple_groups_table`: Sets a flag to one and copies five groups of three longwords from a source pointer into a fixed table, taking each group 200 bytes after the previous one.
- `clear_pixel_bits_from_list`: Walks a list of 16-bit entries until it reads a negative one, clearing a single pixel bit at each listed position with a column byte index and a row stride of 200 bytes.
- `init_record_with_constants`: Stores a pointer to a global and three constant words into fixed offsets of a record passed in a register.
- `init_four_record_slots`: Writes a pointer to a global and the value one into four fixed slots of a record passed in a register.
- `clear_rows_of_longwords`: Zeroes blocks of ten longwords in rows 200 bytes apart, over a fixed number of rows and a second block.
- `advance_counter_and_write_values`: Steps a counter and a value held in globals, resetting the counter past a limit and making several calls to a routine at a fixed address.
- `search_table_set_cia_timer`: Searches a byte table for the first entry greater than a running value, then programs the CIA-A timer high and low registers and control register from the matched position.
- `reset_record_fields`: Zeroes four word fields in a record and sets another field to fifteen.
- `wrap_state_and_wait_flag`: Steps a state counter and, when it wraps, calls routines, waits on a byte flag, and writes a word with its top bit set into a record field.
- `decode_tracker_note_event`: Reads a four-byte note event from a pattern, takes the sample number from its high nibbles, and when that number is non-zero copies the matching 30-byte sample header fields into the channel record.
- `process_tracker_effect_command`: Handles one effect command of a tracker channel, returning a masked period value; when the period is zero it copies a stored value, otherwise it dispatches on the effect nibble with a modulo-three counter.
- `store_masked_period_value`: Stores the low twelve bits of its argument into the field at offset six of a record.
- `store_word_into_record_field`: Writes its word argument into the field at offset six of a record.
- `multiply_then_shift_six`: Multiplies a 16-bit argument by a global value and returns the product shifted right by six bits.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `manage_hardware_and_state`: modifies various hardware registers including CIA and DMA controllers, updates several global variables, and calls another routine before entering an infinite loop.
- `update_hardware_state`: Checks a global variable and a memory location to conditionally call another routine, modifies a byte in a table, and writes a value to a memory location while interacting with the CIAA port.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_status_values`: Performs several subroutines and updates two memory locations with specific values, including conditional updates based on comparisons.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `calculate_loop_iterations`: Calculates a loop count based on input values and executes a nested loop structure that modifies a global variable.
- `modify_global_bitmask`: Sets a global variable to specific values and performs bitwise operations involving a bit shift.
- `poll_and_clear_memory`: performs a loop that checks bits in a global variable and clears a word in a table based on a calculated index.
- `update_global_state`: modifies bits in a global variable based on a register value and updates a table entry if the value differs.
- `update_value_with_loop`: Compares a value against a table entry, updates the entry, and executes a loop that toggles bits in a global variable.
- `toggle_bit_in_loop`: Clears and sets a specific bit in a global variable within a loop that executes a set number of times.
- `initialize_hardware_registers`: Writes several constant values to memory locations and waits in a loop for a specific bit in a memory location to be set.
- `interleave_bit_patterns`: Searches for specific magic numbers in memory and, if found, performs bitwise interleaving of data into a destination buffer.
- `process_bit_patterns`: checks for specific magic numbers in memory to either perform bitwise interleaving into a destination or enter an infinite loop.
- `manipulate_bitmask`: writes a value to memory and performs bitwise operations on a global variable using a loop.
- `toggle_global_and_loop`: Sets a global variable to zero, then to one, and enters an infinite loop.
- `execute_empty_loop`: Performs a nested loop that repeatedly compares a byte in memory to itself and decrements a counter.
- `run_decrement_loop`: Performs a nested loop that decrements a counter while checking a bit in a global variable.
- `execute_single_instruction`: A routine consisting of a single instruction.
- `set_interrupt_and_dma`: Sets a global variable to zero and one while performing bitwise operations and then enters an infinite loop.
- `store_memory_byte`: Loads a byte from a memory location calculated from two registers and stores it in a global variable before clearing that variable.
- `poll_input_state`: Repeatedly checks a global variable and performs actions based on the state of a value, potentially interacting with hardware registers.
- `check_global_state`: Performs a series of calls to other routines and compares a value from a table against a value stored in a structure.
- `nop_sequence`: This routine is empty and performs no operations.
- `cast_global_to_char`: Stores a global variable as a single byte.
- `truncate_global_byte`: Reads a byte from a global variable and writes it back to the same location, effectively masking out the upper bits of the original value.
- `clear_memory_blocks`: Uses nested loops to clear specific bytes within a large memory range by writing zeros to them.
- `call_other_routines`: Calls another routine three times in succession.
- `format_numeric_string`: Calculates the digits of a value from a memory location and stores the resulting characters into two consecutive RAM locations.
- `load_data_from_table`: Calculates an offset based on a value in memory and copies a sequence of bytes from a table into several RAM locations.
- `convert_value_to_ascii`: Calculates a value from memory, performs division and modulo operations, and stores the resulting digits as ASCII characters in two global variables.
- `update_ascii_character`: Adds a constant value to a byte and stores the result in a global variable.
- `initialize_parameter_table`: Populates a large block of memory with various constant values and zeroed fields.
- `update_state_values`: Compares a value from a table against several thresholds and updates two different memory locations based on the result.
- `patch_data_table`: Iterates through a table and, upon finding specific characters, updates two memory locations with values derived from other memory locations.
- `copy_modified_data`: Iterates through a memory range, performing subtractions on values from a table and storing them in a destination buffer until specific characters are encountered.
- `clear_memory_block`: Sets a byte to a specific value and then fills a block of memory with zeros using a loop.
- `copy_memory_block`: Copies a block of data from a calculated source address to a fixed destination address.
- `initialize_global_parameters`: Stores several constant values into various global variables and memory locations.
- `zero_multiple_tables`: Clears two separate tables of memory using a loop and then sets several global variables to zero.
- `clear_two_buffers`: Fills a fixed memory area with zeros and then fills a second area with zeros using a pointer derived from a table lookup.
- `execute_jump_routine`: This routine consists of eight instructions and performs an absolute jump to another address.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `reset_entity_attributes_and_sync_state`: Clears a series of word-sized fields within a data structure and performs a sequence of state updates involving bitwise inversions and memory block synchronization.
- `execute_nested_loop_calls`: Executes a routine multiple times within a nested loop structure.
- `execute_nested_loops`: Executes a nested loop structure that calls another routine multiple times.
- `copy_data_with_stride`: Copies data from a source to two different destinations using a loop with a variable stride length.
- `expand_data_pattern`: Performs a nested loop to distribute bytes from a source table into two large destination buffers at specific offsets.
- `update_threshold_flags`: Updates global status flags by passing a value to a routine that evaluates it against specific numeric conditions.
- `invoke_threshold_check`: Calls a routine that evaluates a value against specific numeric conditions and updates two global variables.
- `set_threshold_flags`: Stores a value into a memory location and then updates two global variables based on whether that value meets specific numeric conditions.
- `bulk_copy_data_tables`: Performs multiple sequential block memory transfers from various source tables to specific destination offsets within a memory buffer.
- `update_status_byte`: Checks the return value of another routine and sets a byte at a specific offset to either zero or all ones.
- `prep_shared_bitmask_two_status`: Called by status and counter updates, this loads a program counter relative pointer into address register four, sets data register two to two, calls the shared bitmask routine, and returns.
- `prep_shared_bitmask_two_single`: Called exclusively by the status updater, this sets up a relative pointer in address register four, moves two into data register two, calls the shared bitmask routine, and returns.
- `prep_shared_bitmask_zero_counters`: Invoked during status and counter state updates, this loads a relative pointer into address register four, clears data register two to zero, calls the shared bitmask routine, and returns.
- `update_state_counters`: Manages two counters at specific offsets from a pointer by incrementing, decrementing, or resetting them based on the results of a subroutine call.
- `manage_state_counters`: updates status bytes and a counter in a structure based on the return value of another routine and executes an indirect call.
- `manage_entity_state_cycles`: Updates two state counters and performs conditional resets based on their values.
- `dual_shared_bitmask_counter_call`: Called by counter managers, this calculates base pointers in address register one, prepares address register four and data register two, and calls the shared bitmask routine twice before returning.
- `single_shared_bitmask_state_prep`: Called by the state counters updater, this calculates a pointer in address register one, prepares address register four and data register two, calls the shared bitmask routine, and returns.
- `execute_bitmask_loop`: Repeatedly executes a specific routine until a bitmask condition is met in a register.
- `execute_bitmask_loop_2`: Repeatedly executes a specific routine until a bitmask condition is met in a register.
- `execute_bitmask_loop_3`: Repeatedly executes a specific routine until a bitmask condition is met within a register.
- `test_bitmask_condition`: Repeatedly executes a specific routine until a bitmask operation returns a required value in a register.
- `update_state_values_2`: checks a short value at an offset from a pointer and stores two different word values into subsequent memory locations based on that value.
- `execute_recursive_call`: The routine calls another routine and then returns.
- `update_entity_attribute_offsets`: Calculates and stores a series of memory offsets into a global table based on two base addresses.
- `compare_memory_blocks`: Compares two sequences of bytes and stores the count of matching blocks into a memory location.
- `write_status_bytes_2`: Writes specific byte or word values to various offsets within a structure based on the value of a register.
- `write_status_bytes`: writes specific byte or word values to various offsets from a pointer based on the value of a register.
- `write_conditional_bytes`: Writes specific byte or word values to various offsets from a pointer depending on the value held in a register.
- `update_conditional_memory`: Depending on the value in a register, the routine writes a single byte or a sequence of bytes to various offsets from a pointer.
- `update_conditional_values`: Checks a word value and either writes a sequence of bytes to a memory offset or sets a single byte to its maximum value.
- `set_byte_if_one`: Sets a byte at a specific offset to a maximum value if a register contains one.
- `manipulate_word_bits`: Performs bitwise operations and shifts on a word retrieved from a memory offset.
- `copy_data_block`: Copies a block of words from one memory location to another.
- `increment_global_value`: Adds a constant value to a global variable unless it equals a specific threshold.
- `decrement_global_until_threshold`: Repeatedly calls another routine and subtracts a constant value from a global variable until the variable is no longer greater than a specific threshold.
- `clear_and_copy_block`: Clears a global variable and then copies a block of words to a destination address.
- `wait_for_zero_write`: Writes zero to a memory location and enters a loop until that location is no longer zero.
- `reset_data_structure`: Clears a series of word-sized memory locations at specific offsets from a pointer and updates several other word-sized locations with specific values.
- `populate_address_table`: Extracts various offsets from two pointers and stores them into a global table.
- `copy_data_with_constant`: Sets a global variable to a constant and copies a block of words to a destination address.
- `update_object_properties`: Increments a global counter and copies several blocks of data from memory to a destination structure.
- `process_sorted_data`: Iterates through a memory buffer to find specific byte values and, upon finding them, initializes a table and updates a structure.
- `clear_object_data`: Clears several word-sized memory locations at specific offsets from a pointer and clears the lowest bit of a global variable.
- `process_object_command_data`: Extracts a command byte from an object structure to perform table lookups and updates internal state.
- `update_object_state`: Updates a data structure based on a command byte, performing operations such as addition, subtraction, or table lookups.
- `multiply_and_shift`: Multiplies a value by a constant and returns the result after bitwise shifting.
- `manage_game_state_and_io`: Updates various internal state counters and pointers while performing bitwise inversions on memory contents during disk I/O retry loops.
- `manage_disk_io_retry_loop`: Decrements a retry counter and manages a state machine that performs bitwise inversions on memory contents while waiting for specific byte values.
- `initialize_state_variables`: Sets several values in a memory block, including zeroing specific offsets and assigning constants like ten and one hundred, based on a bitwise AND of two 16-bit values.
- `invert_and_swap_tables`: Inverts the bits of a 16-bit value in a global variable and performs a block swap between two memory tables.
- `execute_disk_io_sequence`: Monitors a memory location for changes and performs word inversions while waiting for specific byte values.
- `invert_memory_word_2`: Repeatedly inverts a word at a memory location after waiting for a specific byte to become non-zero.
- `invert_memory_word`: Performs a bitwise NOT operation on a 16-bit value stored at a specific memory location.
- `wait_for_space`: Polls a memory location for a space character while checking a bit in a global variable and decrementing a counter.
- `update_coordinates`: Calculates and stores two 16-bit values into global variables based on offsets from a pointer, with conditional logic for the second value.
- `calculate_offsets`: Adds values from two different offsets of a pointer and subtracts constants to store the results in global variables.
- `initialize_and_calculate_time_display`: Clears a series of memory fields and calculates a total time value in seconds from several digit components.
- `execute_recursive_calls`: The routine performs three consecutive recursive calls to itself before returning.
- `decrement_nibbles_in_loop`: Decrements nibbles within a series of words and updates a global value based on the results.
- `execute_repeated_calls`: Executes a specific routine multiple times within a loop that decrements a counter.
- `decrement_nibbles`: Extracts three nibbles from a 16-bit value, decrements each non-zero nibble by one, and reassembles them into the original location.
- `call_unnamed_routine`: Executes a series of calls to another routine.
- `clear_memory_block_2`: Sets a series of specific 16-bit offsets within a memory block to zero.
- `copy_structured_data`: Performs a nested loop to copy 5-byte chunks from a source table into two separate destination tables using specific offsets.
- `unpack_compressed_data_blocks`: Decompresses multiple data blocks by reading triplets of bytes and distributing them into specific offsets within a destination buffer.
- `copy_data_blocks`: Performs a nested loop to copy multiple blocks of data from a source table to a destination buffer using calculated offsets.
- `update_digit_graphics`: Decomposes a value into decimal digits and copies corresponding graphic data from a table to a destination buffer.
- `format_time_display`: Calculates minutes and seconds from a total value and copies the associated graphic data for each digit to a buffer.
- `copy_digit_graphics`: Decomposes a value into individual digits and copies the corresponding graphic data from a table to a destination buffer.
- `process_data_blocks`: Iterates through nested loops to copy bytes from one location to another and perform bitwise OR operations on them, followed by a second pass to zero out the destination memory.
- `wait_for_bit`: Enters a loop that spins until a specific bit in a global variable is set.
- `decrement_and_adjust_table`: Decrements multiple counters within a structure and performs a loop to adjust values in a table by subtracting nine if they exceed ten.
- `update_table_values`: decrements two counters in a structure and adjusts values in a table by subtracting nine if they exceed ten.
- `decrement_table_values`: Decrements a counter in a structure and performs a loop to adjust values in a table by subtracting nine if they exceed ten.
- `clear_memory_offsets`: Sets two specific word-sized locations in a structure to zero.
- `initialize_structure_values`: Populates a series of memory locations within a structure with specific constant values.
- `fill_memory_with_results`: Repeatedly calls another routine and stores the returned 16-bit value into a memory table until a condition is met.
- `decrement_nibbles_in_table`: Iterates through a table and compares nibbles of words against another table, decrementing the nibbles in the destination table if they are greater than the source nibbles.
- `wait_for_character_transitions`: Runs a counted loop in which it spins while a character byte at a fixed offset is a space, then spins while it is not a space, repeating for the count.
- `wait_for_character_with_interrupt`: Spins while waiting for a character at a fixed offset to transition between space and non-space, while checking a global variable to allow an early exit.
- `tiny_moveq_rts_2`: Small routine of 2 instructions that loads 255 into d4, then returns.
- `wait_for_register_bits`: Spins in a loop until specific bits in a memory location are set and then non-zero, returning the masked value.
- `poll_character_byte`: Polls a byte at a specific offset, returning if the byte is a space or matches one of two specific signed character values.
- `nop_padding_byte`: This routine is empty and consists of a single instruction that performs no operation.
- `initialize_data_structures`: waits for a bitmask condition to be met and then populates several memory locations and a table with specific values.
- `clear_memory_with_bounds`: Sets a byte to a specific value and then clears a series of memory blocks using nested loops before resetting the initial byte and a 16-bit value.
- `clear_memory_blocks_2`: iterates through a series of memory blocks and fills them with zero values using a fixed stride.
- `reconstruct_nibble_table_2`: Compares values from a source table against a reference table and updates the source table by adjusting its nibbles based on the differences found.
- `reconstruct_nibble_table`: Compares values from two tables and updates a third table by packing nibbles extracted from the source values.
- `reconstruct_nibbles`: compares values from two tables and updates a third table by packing nibbles extracted from the source values.
- `pack_nibbles_from_table`: Reads words from a table, extracts nibbles, and packs them into new 16-bit values while incrementing counters.
- `reconstruct_nibbles_from_table`: Reads words from a table and uses their nibbles to increment various counters, which are then used to pack new 16-bit values into a destination memory area.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `skip_space_characters`: loops through a memory area to skip over sequences of space characters.
- `copy_characters_from_table`: Polls a memory location for a non-zero value, performs bitwise operations on it, and copies characters from a table into a buffer.
- `assign_byte_to_offsets`: stores a single byte from a register into multiple specific offsets relative to a base address.
- `store_byte_to_offsets`: Stores a single byte from a register into multiple specific offsets relative to a base address.
- `loop_bitmask_operations`: Repeatedly executes a routine until a specific value is returned in a register.
- `update_bitmask_buffers`: Iterates through a source list of three-byte triplets to perform bitwise OR, AND, or NAND operations on two identical destination buffers.
- `initialize_object_fields`: Clears multiple word-sized memory locations at fixed offsets from a pointer and performs conditional logic based on a register value.
- `nop_padding_sequence`: This routine is empty and consists only of two instructions that perform no operation.
- `calculate_scaled_offset`: Calculates a value using a multiplication and shift, then increments a counter at a memory location.
- `increment_word_at_pointer`: adds two to a word stored at a location pointed to by a register.
- `set_ram_character`: Performs a loop that waits for a memory location to be non-zero and then stores a character derived from a register into a global variable.
- `initialize_data_table`: stores a series of constant values into a sequence of memory locations.
- `transform_string_data`: Compares values from a table against a memory range, moves data blocks, and populates a range with a constant character and transformed values.
- `fill_and_transform_strings`: fills a memory range with a specific character and then populates another range by adding a constant to values from a source table.
- `copy_memory_block_2`: copies a block of words from a source address to a destination address.
- `swap_memory_blocks`: swaps the contents of two memory blocks of a specific size.
- `execute_twelve_instructions`: a routine consisting of twelve instructions.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `do_nothing`: This routine contains no instructions; it does nothing and simply returns.
- `do_nothing_2`: The routine contains no instructions and returns immediately without reading or writing any data, registers, memory, or hardware.
