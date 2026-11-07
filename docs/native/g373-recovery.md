# Game 373 (Elfmania v1.2b 0114): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00373). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4276849 top-level bytes in 21 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 32 of 32 units identical (bytes and relocations), 35 raw-word lines |
| Functions named and described | pass, 222 of 222 named (0 author symbols), 222 of 222 described (9 by rule, 177 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 3401 of 3401 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 4864 bytes, 4 functions |
| Independent decoder cross-check (Ghidra against capstone) | 5193 of 5205 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 29625 |
| code | 21166 |
| data | 4168769 |
| padding | 59 |
| unused | 57230 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 584 | 30 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 337 | 16 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 461 | 10 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 20 | 4 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1480 | 53 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 106 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 962 | 58 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 37 | 8 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 819 | 21 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 260 | 9 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 5 | 5 |
| Disk.1 bytes 0-256 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 2560-3072 (code never loaded by a reference run) | 1 | 54 | 1 |
| Disk.1 bytes 3328-4864 (code never loaded by a reference run) | 1 | 9 | 1 |
| Disk.1 bytes 703744-704000 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 0-256 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.2 bytes 2560-3072 (code never loaded by a reference run) | 1 | 54 | 1 |
| Disk.2 bytes 3328-4864 (code never loaded by a reference run) | 1 | 9 | 1 |

## Cases handled

- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.1: raw disk image (941568 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.2: raw disk image (941568 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114.hdf: hard-disk image (AmigaDOS OFS, 4544 blocks): its filesystem parses and all 9 files in it are byte-identical to the extracted tree Elfmania_v1.2b_0114_hdf, which is recovered file by file; no block outside the filesystem holds data
- 3 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- 14 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address (f150): 402 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (ext): 828 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 98 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 687 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 25 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 561 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (f150): 236 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 5 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.1: bytes 0-256 look like code no run loaded: recovered position-independently (load address unknown)
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.1: bytes 2560-3072 look like code no run loaded: recovered position-independently (load address unknown)
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.1: bytes 3328-4864 look like code no run loaded: recovered position-independently (load address unknown)
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.1: bytes 703744-704000 look like code no run loaded: recovered position-independently (load address unknown)
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.2: bytes 0-256 look like code no run loaded: recovered position-independently (load address unknown)
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.2: bytes 2560-3072 look like code no run loaded: recovered position-independently (load address unknown)
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.2: bytes 3328-4864 look like code no run loaded: recovered position-independently (load address unknown)
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.1: 18 logged loads cover 516018 bytes of the image
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.1: 12 track-loaded chunks found in recovered runtime segments and classified through them
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.1: chunks 3-4 (bytes 768-1280) outside the filesystem hold data, not code (opcode density 0.059, entropy 5.62)
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.1: chunks 23-43 (bytes 5888-11264) outside the filesystem hold data, not code (opcode density 0.013, entropy 6.85)
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.2: 3 logged loads cover 2304 bytes of the image
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.2: 12 track-loaded chunks found in recovered runtime segments and classified through them
- Elfmania_v1.2b_0114/Elfmania_v1.2b_0114_hdf/Elfmania/Disk.2: chunks 3-4 (bytes 768-1280) outside the filesystem hold data, not code (opcode density 0.059, entropy 5.62)
- 5 never-loaded region(s), 3840 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)
- 6 packed track region(s), 1294592 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `drive_disk_read_retry_loop` (Disk.1 +an address loaded at an address (1546 bytes)): Runs a disk read loop that sets and clears status bits and calls an unnamed disk routine, retrying on failure. Kind: disk track/read error check with conditional jump table. Pass path: the loop continues or returns via the indirect jump table branch when sVar3 >= 0.
- `setup_disk_transfer_copy` (Disk.1 +an address loaded at an address (1546 bytes)): Sets a status bit, writes transfer and interrupt values, copies 3750 longwords into low memory, calls two unnamed routines, then jumps to a fixed entry address. Kind: checksum/data integrity check. Pass path: the loop completes and execution continues to u2__sub_059A.
- `initialize_hardware_and_sync_memory` (runtime segment an address (ext)): Initializes hardware registers and copies a block of data to a destination table. Kind: disk hardware/track check. Pass path: return 1.
- `initialize_dma_parameters` (runtime segment an address): Sets several global variables and hardware registers to specific values before calling another routine and updating a global pointer. Kind: disk hardware/sync check. Pass path: the branch where the code continues normal execution (the 'bne.w an address' or 'bne.b an address' paths depending on the specific bit check).

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `call_two_unnamed_routines`: Calls two unnamed routines and nothing else is visible.
- `call_library_patch_on_failure`: Calls a library function through a table entry, storing the result pointer and reference in RAM cells.
- `patch_code_locations_with_opcodes`: Overwrites many code locations with no-op, branch and return opcodes, calls three unnamed routines in the middle of the sequence, then writes two values including one to the interrupt enable register.
- `copy_global_into_cell`: Copies a shared global value into another RAM cell and returns.
- `return_empty_routine`: This routine is empty and performs no operations before returning.
- `adjust_byte_unless_letter_f`: Subtracts two from the byte addressed by A0 unless that byte is the letter f, then adds one to it.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `update_state_and_dispatch_interrupt`: Updates a state counter and checks for changes in a global state variable.
- `update_state_and_dispatch_interrupt_2`: Increments a state counter and checks for changes in a global state variable.
- `advance_state_and_dispatch_irq`: Runs the same state advance and handler dispatch as its neighbours, with 13 further unnamed calls.
- `execute_conditional_logic`: Checks a global byte for a specific character and a flag, then executes another routine and updates the flag.
- `run_once_when_marker_matches`: When a marker byte equals the letter M and a flag is still zero, calls an unnamed routine and sets the flag.
- `write_padded_text_message`: Fills a 22-longword buffer with a text string by storing four-character groups at fixed offsets, padding the gaps with spaces.
- `store_cells_and_patch_opcodes`: Stores its argument and two pointer values into RAM cells, calls an unnamed routine, then compares a cell against a fixed address and overwrites words with no-op opcodes on a match.
- `wait_for_raster_line`: Busy-waits by reading the beam position registers (VPOSR/VHPOSR) until the display beam reaches a given raster line, then returns.
- `wait_until_raster_result_zero`: Repeatedly calls the raster-line wait routine until the word returned in D1 is zero, which waits for the display beam to reach the target line.
- `return_masked_cell_value`: Returns the value of a RAM cell masked to its low 48 bits and does not write anything.
- `execute_indirect_function_pointer`: Loads a function pointer from a table at a specific offset and executes it.
- `execute_indirect_function_call`: Loads a function pointer from a table at a specific offset and executes it.
- `execute_indirect_function_pointer_2`: Loads a pointer from a table and executes the routine located at the offset specified by the pointer.
- `call_library_slot_save_text`: Calls a function through a library table slot and passes its results back.
- `call_alt_library_slot_save`: Calls a function through a different library table slot and passes its results back.
- `disable_all_dma_and_interrupts`: Writes the all-bits-clear value, with bit 15 clear, to the interrupt, DMA and interrupt request registers, then calls one unnamed routine.
- `set_background_color_value`: Writes the value 240 to the background colour register and returns.
- `decode_bitstream_into_buffer`: Reads bits from a packed input stream, refilling it in 16-bit words, and decodes codes through lookup tables for values and lengths.
- `write_branch_and_jump_patches`: Writes branch, no-op followed by absolute jump, and subroutine-call opcodes at fixed offsets from the base pointer, then returns zero.
- `store_constant_block_and_flag`: Sets a flag word to 1 and writes four fixed longwords into a buffer, the first of which is zero.
- `patch_jsr_and_call_target`: Writes a subroutine-call opcode and an absolute address into a code location, then calls through an indirect jump that the decompiler could not resolve.
- `modify_and_call_jump_target`: Loads a function pointer from a table, writes a constant value to an offset within that target, and calls it.

### logged disk load from Disk.1

- `drive_disk_read_retry_loop`: Runs a disk read loop that sets and clears status bits and calls an unnamed disk routine, retrying on failure.
- `poll_disk_status_and_retry`: Clears a status bit, sets two others and repeatedly calls an unnamed routine until a status word reports completion, retrying when the result is negative.
- `select_pattern_and_wait_status`: Selects one of three constant patterns from status bits, then waits for a status bit to clear and steps a counter down from 20 until it reaches zero.
- `set_flag_pattern_when_clear`: When the high bits of a status byte are clear and bits 0 and 3 are clear, sets bit 3 and writes a fixed pattern into a shared cell.
- `dispatch_status_change_handler`: Reads a cell and updates a rolling byte counter.
- `swap_buffers_by_status_bit`: Swaps 16500 longwords between a buffer and a second buffer offset by 1000 bytes.
- `wait_and_clear_pending_word`: Waits for a flag to clear, then if a counter word is non-negative it zeroes a word and waits until a second word reads zero before setting the counter to minus one.
- `install_interrupt_vector_and_enable`: Stores a routine address in a vector cell, writes a disabling value and then an enabling value to the interrupt control registers, and clears a state cell.
- `call_pointer_then_clear_request`: Calls a stored routine pointer, calls it again when a second pointer is non-null, then writes a bit 5 value to the interrupt request register and calls one unnamed routine.
- `copy_block_and_relocate_pointers`: Copies a block of longwords into the start of a buffer, with the count taken from the block header, then adds the buffer base address to each entry of a second table as a relocation pass.
- `copy_longwords_then_spin`: Copies 120 longwords from one address to another, calls one unnamed routine, then loops forever.
- `call_stack_address_then_spin`: Computes two address values from a stack argument by fixed offsets, stores them in global cells, calls the routine at the argument address, then loops forever.
- `load_segments_then_jump`: Copies a block of longwords from a source and relocates its pointer entries, then writes transfer parameter values for each entry of a table.
- `execute_infinite_loop`: An infinite loop that performs no actions.
- `setup_disk_transfer_copy`: Sets a status bit, writes transfer and interrupt values, copies 3750 longwords into low memory, calls two unnamed routines, then jumps to a fixed entry address.
- `add_value_to_offset_entries`: Adds a value to each longword addressed by a table of offsets relative to a base, stepping through the table until its counter wraps.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_interrupt_and_dma`: This routine configures DMA and interrupt enable bits while managing bitstream decompression.
- `set_callback_structure`: Stores a parameter and a routine address into a structure located at an offset from a global variable.
- `manage_hardware_interrupt_and_bitstream`: This routine manipulates CIA timer control registers, color registers, and interrupt enable bits while performing bitstream decompression.
- `call_pointer_and_assign_constant`: Executes a routine via a pointer stored in a global variable and then overwrites that global variable with a constant value.
- `loop_and_write_data`: Enters an infinite loop that writes a value to a table offset and performs a bitwise check and write operation ten times.
- `toggle_and_update_ram`: Toggles a global bit and writes several values to specific memory locations.
- `wait_for_bit_clear`: Polls a byte at an offset from a pointer until a specific bit is cleared.
- `decompress_bitstream_data`: Decodes a compressed bitstream using a variable-length Huffman-style encoding scheme.
- `execute_repeated_calls`: Calls another routine multiple times within several loops.
- `update_parameter_structure`: Performs signed arithmetic and bounds checking on two input values to calculate several derived constants.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `trigger_dma_and_wait`: Writes a value to a DMA control register and waits for a specific bit in a global variable to clear.
- `trigger_dma_and_wait_return`: Writes a value to a DMA control register and waits for a specific bit in a global variable to clear before returning a byte.
- `update_display_control_and_wait`: Writes a value to a memory location offset from a register and enters a busy-wait loop until a specific bit in a global variable is cleared.
- `enable_dma_copper_bitplane`: Sets a specific bit in a DMA control register and waits for a bit in a global variable to clear.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_hardware_and_sync_memory`: Initializes hardware registers and copies a block of data to a destination table.
- `copy_and_add_to_table`: Copies a block of data from one location to another and then adds a global value to several memory locations using a table of offsets.
- `add_values_to_table`: Iterates through a list of offsets and adds a constant value to the memory locations they point to.
- `process_hardware_state`: Evaluates bits in a global variable and hardware registers to perform an indirect jump and update status flags.
- `tiny_ori_ori`: Small routine of 2 instructions that merges 134 into d0, then merges 86 into d0.
- `call_indirect_via_table`: Loads an address from a table using an offset and executes it.
- `execute_indirect_call_4`: Performs an indirect jump to an address stored in a memory location relative to the current instruction pointer.
- `execute_indirect_function`: Loads an address from a global variable and jumps to that location.
- `execute_indirect_call_3`: Performs an indirect call to a routine stored at a location pointed to by a value in memory.
- `update_state_flags_and_pointers`: Updates various status flags and bitmasks within a control structure and modifies global pointers based on conditional checks.
- `update_entity_state_flags`: Updates a series of bitwise flags within a structure and modifies global state variables based on conditional checks.
- `check_and_update_flags`: Tests a bit in a global variable and performs a series of conditional checks, recursive calls, and memory updates based on the result.
- `execute_conditional_logic_3`: Calls another routine and, if a zero flag is not set, performs a sequence of memory writes and conditional recursive calls.
- `perform_state_reset`: Writes a global variable to a memory location through a pointer and then executes a sequence of updates to various memory locations and registers.
- `update_memory_state`: Calls another routine and, if a zero flag is set, performs specific bitwise operations and memory updates using a pointer.
- `execute_conditional_logic_2`: Updates several global variables and memory locations through a series of conditional checks and indirect calls.
- `modify_status_byte`: Manipulates a byte at a specific offset within a structure and conditionally updates a global variable and a memory location.
- `execute_indirect_call_2`: Checks if a global variable contains a non-zero address and, if so, executes the routine pointed to by that variable.
- `call_function_and_modify_string`: Copies data from a global variable to a buffer, calls a function pointer, copies the result back, and replaces '@' characters with spaces in the buffer.
- `count_conditional_increments`: Iterates through a table of bytes and increments a counter based on specific bit flags found in each byte.
- `find_maximum_values`: Iterates through multiple calls to another routine to find and track the maximum values of two separate byte results.
- `search_and_shift_table`: Searches a table for a matching value and, if found, performs a block move of data elements within that table.
- `reset_hardware_and_initialize_scores`: Clears several DMA registers and copies a data block to a specific memory location.
- `initialize_high_score_data`: Sets up memory parameters and data tables related to player high scores.
- `update_system_state`: Checks a bitwise condition and, if met, writes a value to a specific memory location and updates several global variables.
- `decrement_counter_and_branch`: Decrements a value in a structure and executes a recursive call if the value reaches zero.
- `process_data_and_update_structure`: Iterates through a data stream, performs conditional logic based on byte values, and writes color and coordinate data to a structure.
- `set_blitter_parameters`: Waits for a flag to clear and then writes three sets of coordinate and color data to a structure in memory.
- `setup_blitter_transfer`: Calculates offsets based on input values and populates a series of memory locations used for blitter operations.
- `update_table_values`: Performs arithmetic on a pointer and conditionally executes a routine, then iterates through a table to add values from a global table into a destination buffer.
- `tiny_ori_ori_2`: Small routine of 2 instructions that merges 0 into d0, then merges 142 into d7.
- `initialize_blitter_sequence`: Calls three different routines to configure and write sequences of data to memory.
- `execute_blitter_setup_sequence`: Calls three consecutive routines to configure a sequence, write a command list, and initialize structure fields.
- `perform_indirect_jump`: Executes a routine at an address calculated by adding an offset to the current instruction pointer.
- `nop_padding_byte_2`: This routine is an empty instruction acting as a single-byte filler in the executable segment.
- `copy_memory_block`: Copies a fixed number of words from a global variable to a destination address.
- `initialize_memory_parameters`: Counts bytes in a string and then writes several specific values to a set of global memory locations after polling a bit.
- `initialize_data_tables_2`: Iterates through a loop to populate a table with values derived from a data stream and then clears several global memory locations.
- `calculate_mathematical_loop`: Performs a loop involving division, modulo, and bitwise operations to transform a value.
- `perform_mathematical_loop`: Executes a loop involving multiplication, division, and modulo operations to transform values.
- `process_blitter_parameters`: Updates various memory locations and hardware registers based on values found in a data stream, including conditional color and coordinate assignments.
- `update_blt_registers`: Updates several Blitter hardware registers and writes values to a structure using a pointer.
- `parse_data_stream`: Iterates through a data stream to convert nibbles into characters and writes color and coordinate data to a structure based on conditional logic.
- `sync_dual_memory_regions`: If a specific bit is not set, it clears that bit and copies data from a global variable to two different memory locations.
- `configure_blitter_sequence`: Waits for a flag to clear and then populates multiple memory locations with values used for a sequence of blitter transfers.
- `write_blitter_command_list`: Writes a series of values, including coordinates and colors, into a structured memory block to define blitter operations.
- `update_display_control_parameters`: Repeatedly waits for a bit in a global variable to clear before writing a sequence of values into a memory structure pointed to by a register.
- `initialize_data_table`: Writes a series of 16-bit values into multiple offsets within a table using a loop that repeats four times.
- `initialize_structure_fields_2`: Performs a series of writes to a multi-field structure at an offset from a pointer, while checking a global variable bit in a loop.
- `calculate_and_store_struct_values`: Performs complex arithmetic and conditional checks on two input values to populate a multi-field structure in memory.
- `update_table_via_masking`: Iterates through a list of five-byte entries to add values from a global table into a destination buffer based on indexed offsets.
- `apply_bitmask_to_table`: Fills a small buffer with incrementing nibbles and then updates a set of four 32-bit words by masking and merging values from a global table.
- `decompress_bitstream_data_2`: This routine performs bitstream decompression by reading a source buffer and writing expanded data to a destination buffer.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_stack_pointers`: Sets two global variables to specific offsets relative to the stack pointer and then executes a routine at the stack pointer.
- `add_table_values`: Iterates through a table and adds a value to each longword entry at a specified offset.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `dispatch_indirect_call`: Checks a global variable and either executes a routine at an offset from that variable or performs a series of assignments to global variables and calls another routine.
- `process_status_flags`: Updates several global variables and memory locations based on bitwise checks of a value, eventually returning a status code.
- `update_state_variable`: Checks specific bits in a global byte and updates a global variable with predefined constants or zero based on the bitwise state and a secondary global value.
- `modify_and_check_flags`: Sets a global variable with a specific bitmask and performs a series of conditional checks to return a status value.
- `reset_system_state`: Clears several global variables and pointers, modifies a global bitmask, and performs conditional logic based on the state of a global byte.
- `update_global_data`: Sets a bit in a global variable and copies four consecutive longwords from a source to a calculated destination in a table.
- `search_and_execute_table_entries`: Iterates through a table to find a specific entry based on several conditions and then executes routines via indirect calls.
- `set_global_constant`: Sets a global variable to a constant value and returns.
- `update_global_flags`: Sets a specific bit in a global variable and stores a value into a global memory location.
- `initialize_dma_parameters`: Sets several global variables and hardware registers to specific values before calling another routine and updating a global pointer.
- `decrement_and_update_state`: Decrements a value in a structure, updates a counter, and performs bitwise operations on a global variable and a structure field.
- `decrement_counter_and_update_bitmask`: Decrements a short integer within a structure and resets a field to zero if the value reaches zero.
- `fill_memory_pattern`: Repeatedly writes a byte and a specific pattern to a memory location using a loop, then updates global bitmasks.
- `check_and_set_bit`: Checks a value in a table at an offset determined by a scaled index and sets a bit in a byte within a structure if the table entry is zero.
- `set_status_bitmask`: Sets a specific bit in a byte within a structure based on a bitmask and performs several memory writes if the bit was already set.
- `store_register_via_pointer`: Stores a value from a register into a memory location pointed to by another register.
- `store_value_via_pointer`: Stores a value from a register into a memory location pointed to by another register.
- `clear_and_update_state`: A short routine that clears a block of data and updates state counters through several subroutines.
- `call_subroutines`: This routine calls two other routines and then returns.
- `execute_nested_routines`: This routine executes a sequence of sixteen instructions that calls two other routines.
- `execute_unreached_routine`: An unreached routine consisting of sixteen instructions that calls two other routines.
- `initialize_structure_fields`: Writes several constant word values to specific offsets within a structure pointed to by a register.
- `clear_data_block`: Clears a series of word and longword entries at an offset determined by a scaled index.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `store_registers_to_memory`: Copies a series of registers and stack values into a sequence of memory locations.
- `null_routine`: This routine contains three instructions and performs no operations on registers, memory, or hardware.
- `save_register_state`: A three-instruction routine that calls a routine to copy registers and stack values to memory.
- `copy_sprite_attribute_data`: Copies a block of data from a calculated offset in a large table into a structure pointed to by a register.
- `offset_array_elements`: Adds a base address value to each element in a list of longwords until a counter reaches zero.
- `initialize_object_parameter_tables`: Populates two separate data structures with specific constant values, pointers, and calculated offsets.
- `clear_memory_block`: Fills a specific range of memory with zeros using a fixed count.
- `update_object_data`: Writes several values into a structure at an offset from a pointer and performs conditional calls to other routines based on a loop and a global variable.
- `update_data_structure`: Writes several values into a structure at an offset from a pointer and waits for a specific bit in a global variable to clear.
- `update_data_structure_variant`: Writes a set of values into a structure at an offset from a pointer and waits for a specific bit in a global variable to clear.
- `update_object_parameters`: Writes several values, including two pointers and constant integers, into a data structure at an offset from a pointer held in a register.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `update_object_properties`: Writes several values to a data structure pointed to by a register, including two identical values from another register and a constant.
- `return_immediately`: This routine is empty and performs no operations before returning to its caller.
- `nop_padding_byte`: The routine is empty and consists of a single instruction that performs no operation.
- `set_function_pointer`: Stores the current routine address into a memory location and writes a constant value to another.
- `reset_global_state_2`: Checks a global variable and, if it is zero, updates that variable and several other memory locations.
- `reset_global_variable`: Sets a global variable to a specific value and updates several other memory locations if the variable is not a specific constant.
- `check_and_update_state`: Returns a value from a global variable and, if certain conditions are met, updates a global variable and several memory locations.
- `modify_global_state`: Checks a global variable against two specific values and, if they do not match, writes values to memory and increments a global pointer.
- `reset_global_state`: Sets a global variable to a specific value and updates several other memory locations if the value is not a specific constant.
- `initialize_data_tables`: Populates two consecutive sequences of memory locations with incrementing byte values.
- `reset_and_increment_values`: Sets three consecutive memory locations to all ones and increments a global variable by a fixed amount.
- `call_empty_routines`: Calls two other routines sequentially.
- `nop_placeholder`: The routine is empty and performs no operations.
- `copy_data_to_structure`: Copies a sequence of words from a source pointer into a destination structure at fixed intervals.
- `build_color_attribute_table`: Constructs a table in memory by writing sequences of color indices, brightness/attribute bytes, and coordinate pairs.
- `update_configuration_table`: Checks a global byte and writes a sequence of values to a global table based on its value.
- `tiny_ori_ori_3`: Small routine of 2 instructions that merges 0 into d0, then merges 0 into d0.
- `update_configuration_table_2`: Writes a specific set of values to a global variable based on the state of a global byte.
- `update_configuration_table_3`: Selects a sequence of values to store in a global variable by branching on a global byte.
- `update_configuration_table_4`: Populates a global table with different data depending on the value of a global byte.
- `load_configuration_values`: Writes a sequence of values into a global variable based on the value of a specific global byte.
- `populate_data_table`: Fills a table in memory with a series of multi-byte values based on a loop count determined by a global byte.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `nop_padding_sequence`: A sequence of six no-operation instructions used as padding within the executable segment.
- `execute_indirect_call`: Performs an indirect call using a value from a table indexed by a register.
- `scale_value_seven`: Multiplies a short value by seven, shifts it right by three, and ensures the result is at least one.
- `scale_value_six`: Multiplies a short value by six, shifts it right by three, and ensures the result is at least one.
- `scale_value_five`: Multiplies a short value by five, shifts it right by three, and ensures the result is at least one.
- `scale_value_four`: Shifts a short value left by two, then right by three, and ensures the result is at least one.
- `scale_and_clamp_value`: Multiplies a value by three, performs a right shift by three, and ensures the result is at least one.
- `calculate_scaled_value`: Multiplies a short value by two, shifts it right by three, and ensures the result is at least one.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_2`: An empty routine that returns immediately.
- `compare_short_value`: Compares a short value against a constant and returns based on the result.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `configure_audio_dma`: Configures audio hardware registers and DMA settings.
- `initialize_dma_and_data_structures`: Initializes several global variables and tables by copying data from a source structure through a loop.
- `populate_sprite_attribute_tables`: Iterates through a compressed data stream to initialize a series of structures.
- `process_compressed_sprite_data`: Iterates through a compressed data stream to decode and expand sprite attribute structures into a global table.
- `populate_linked_data_structures`: Iterates through a series of linked lists or indexed tables to extract and transform data into a structured format.
- `unpack_and_initialize_data_structures`: Extracts data from a source buffer using bitwise shifts and offsets to populate a structured table.
- `process_linked_data_stream`: Iterates through a data structure by incrementing a pointer and performing bitwise shifts on a counter.
- `process_linked_data_list`: Iterates through a series of linked data structures, performing bitwise shifts and conditional updates to internal counters and pointers.
- `parse_linked_data_structures`: Iterates through a series of linked lists or indexed tables to extract and transform data into a structured format.
- `process_linked_data_stream_2`: Iterates through a linked list or indexed table to transform data into a structured format.
- `process_linked_data_list_2`: Iterates through a linked list of data structures, performing arithmetic on specific byte fields and updating pointers.
- `process_linked_data_structures`: Iterates through a linked list or indexed table to transform and store data into a structured format.
- `update_state_flags`: Checks specific bits in a byte and, if set, copies data from three different tables into a structure.
- `restore_state_data`: Tests bits in a byte and, if set, populates a structure with values retrieved from various global tables.
- `calculate_complex_value`: Updates three bytes in a structure based on a value from a global variable and a series of bitwise operations and table lookups.
- `update_parameter_block`: Checks a pointer for a specific signature and, if valid, updates a global variable and several memory locations using a calculated offset.
- `store_parameter_block`: Stores four consecutive words into a global variable and two subsequent memory locations.
- `decode_bitstream_delta_pcm`: Decodes a compressed bitstream using a delta-encoding scheme and a lookup table.
- `negate_and_move_data`: Iterates through a range of memory defined by two pointers, moving values from one location to another while negating them.
- `traverse_linked_data_structures`: Iterates through two parallel sets of linked lists or indexed tables by decrementing counters and updating pointers.
- `store_constant_value`: Stores a constant integer into a memory location offset from a register.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `initialize_function_pointer_table`: Sets a global variable and populates a table with several function pointers before executing a routine multiple times.
- `write_memory_word_long`: Writes a word and a long value to a memory location pointed to by a register.
- `copy_data_block_2`: Moves a 16-byte block of data from one location to another while temporarily storing values in a register.
- `store_half_block`: Writes a 4-byte value to a memory location using values from registers.
- `swap_pointer_and_data`: Stores two 32-bit words from a source location into a destination location, then calls another routine before restoring the original values to the source location.
- `copy_data_block`: Copies a 16-byte block of data from one memory location to another if a specific condition is met, otherwise stores a constant value in the destination.
- `test_bit_in_register`: Tests the status of a specific bit within a longword in data register zero.
- `wait_while_flag_set`: Busy-waits, polling a global variable and returning only once bit 6 (mask 64) of that variable is clear.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_immediately_3`: This routine is empty and performs no operations before returning to the caller.
- `return_immediately_4`: This routine is empty and returns to its caller without performing any operations.
- `nop_instruction_placeholder`: This routine is empty and performs no operations.
- `execute_nothing`: The routine performs no operations and returns immediately to the caller.
- `return_nothing`: The routine is empty and performs no operations before returning.

### Disk.1 bytes 2560-3072 (code never loaded by a reference run)

- `calculate_parameter_structure`: Performs signed arithmetic on two input values to calculate several derived constants and stores them in a structure.

### Disk.1 bytes 3328-4864 (code never loaded by a reference run)

- `store_constant_into_offset_cell`: Stores a shared longword into a fixed offset of the block addressed by A1, then returns.

### Disk.2 bytes 2560-3072 (code never loaded by a reference run)

- `update_data_structure_fields`: Performs signed arithmetic and overflow checks on two input values to calculate a set of derived parameters.

### Disk.2 bytes 3328-4864 (code never loaded by a reference run)

- `store_long_value`: Stores a long value into a memory location at an offset from a pointer.
