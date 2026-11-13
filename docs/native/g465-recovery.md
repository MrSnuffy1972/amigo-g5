# Game 465 (Batman - the Movie): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00465). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1813792 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 36 raw-word lines |
| Functions named and described | pass, 76 of 76 named (0 author symbols), 76 of 76 described (3 by rule, 32 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 832 of 832 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 1906 of 1907 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 3457 |
| code | 6426 |
| data | 1706627 |
| padding | 2 |
| unused | 97280 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Batman - The Movie (1989)(Ocean)[cr QTX](Disk 1 of 2).adf:bootblock | 1 | 42 | 3 |
| Batman - The Movie (1989)(Ocean)[cr QTX](Disk 2 of 2).adf:bootblock | 1 | 44 | 3 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1142 | 34 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 592 | 29 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 83 | 6 |

## Cases handled

- batman - the movie (1989)(ocean)[cr qtx](disk 1 of 2)/Batman - The Movie (1989)(Ocean)[cr QTX](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1738 non-empty blocks: matched against track-loaded segments
- batman - the movie (1989)(ocean)[cr qtx](disk 2 of 2)/Batman - The Movie (1989)(Ocean)[cr QTX](Disk 2 of 2).adf: custom-format (non-DOS) disk, 1588 non-empty blocks: matched against track-loaded segments
- runtime segment an address: 398 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 415 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 4 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 15 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 operand
- batman - the movie (1989)(ocean)[cr qtx](disk 1 of 2)/Batman - The Movie (1989)(Ocean)[cr QTX](Disk 1 of 2).adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.77)
- batman - the movie (1989)(ocean)[cr qtx](disk 2 of 2)/Batman - The Movie (1989)(Ocean)[cr QTX](Disk 2 of 2).adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.61)
- 5 packed track region(s), 1701888 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `configure_hardware_and_copy_loader` (bootblock): Initializes custom hardware registers including DMA, blitter, interrupt, and color registers, then copies a block of instructions or data from a source to a destination using a loop with a clamped step value. Kind: track/size-dependent loader/checksum trap. Pass path: uVar1 != 0 (valid size/diff check).

## Functions


### Batman - The Movie (1989)(Ocean)[cr QTX](Disk 1 of 2).adf:bootblock

- `initialise_hardware_and_copy_loader`: Writes fixed values to the interrupt, DMA, colour, CIA and disk registers and copies four words to a fixed low address.
- `copy_words_then_step_value`: Copies a block of words from the source to the destination until the count runs out, then clamps a shared step value and calls a routine when the result is nonzero.
- `initialise_blitter_and_disk_registers`: Writes fixed values to the interrupt, DMA, colour, blitter control and size, disk length and CIA port registers, then clamps a shared step value and calls two routines.

### Batman - The Movie (1989)(Ocean)[cr QTX](Disk 2 of 2).adf:bootblock

- `setup_hardware_and_copy_boot_stub`: initializes hardware registers for the blitter, disk controller, and certain CIA or DMA or color or interrupt or size or length or step registers, then copies a short sequence of instructions from a certain location to a destination buffer.
- `copy_memory_block`: copies a sequence of bytes from a source pointer to a destination pointer, decrementing a counter or length register until it reaches zero.
- `configure_hardware_and_copy_loader`: Initializes custom hardware registers including DMA, blitter, interrupt, and color registers, then copies a block of instructions or data from a source to a destination using a loop with a clamped step value.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `clamp_and_step_shared_value`: Computes a step from a shared value and clamps it to fixed limits, then combines it with a second value and clamps the sum to an upper limit of 1344.
- `copy_tile_rows_into_buffer`: Copies rows of 21 tile entries from a tile table into a destination buffer, looking up each tile index from a map at a position derived from the input row and column.
- `copy_tile_map_with_wrap`: Copies tile columns from a tile table into a buffer, selecting tile indices from a map that starts at a position given by the input and global values.
- `toggle_dma_around_helper`: Writes a DMA control value with the enable bit set, calls a helper and a second helper when its returned value is nonzero, then writes a DMA control value with the enable bit clear.
- `start_blitter_copy_pass`: Waits until the blitter is idle, sets its mask, modulo and pointer registers from the inputs, then runs up to four blitter passes.
- `dispatch_jump_table_entry`: Reads a byte from a global variable, shifts it left by two bits, and uses the result as an index into a table to load a target address into a0 before jumping to it.
- `update_global_state`: Increments a 32-bit value at a pointer in a6, increments a 16-bit value at an offset of negative 2 from a2, and performs a series of comparisons and conditional updates to a global variable based on its current value, including calling another routine and performing arithmetic operations.
- `process_global_value`: checks a byte at a global address, calls another routine, compares a 16-bit global value against several constants, calls sum_three_bytes_into_global, updates a global 16-bit value, and performs arithmetic on a second global 16-bit value, capping it at 12.
- `add_triple_offset`: Loads a 16-bit value from a global variable, adds the value in d6 to it three times, and stores the result back into the same global variable.
- `decrement_and_branch`: Decrements a 16-bit global variable and branches if the result is not zero, otherwise it writes a 16-bit constant to a global variable and branches to another routine.
- `update_global_parameters`: Tests a byte at a global address and branches if zero, calls a routine, clears a 16-bit value in a global variable, calls another routine, and writes two 16-bit constants to global variables before returning.
- `inspect_unreachable_entry`: Most of the decompiled body is removed as unreachable, so the listed facts show ten instructions, one caller, a call to an empty stub and two calls to unnamed routines.
- `call_and_branch`: Calls another routine and then performs a branch to another location.
- `update_counter_and_modify_globals`: Stores a word into a global variable, calculates a value based on a global word and a register, and if a specific bitmask condition is met, calls another routine and modifies several global values including a word at a specific address and a byte at another.
- `increment_counter_and_update_table`: Increments a global variable if the value in register d2 is greater than 22, then calls another routine to perform arithmetic and update a table entry based on the modified value in d2.
- `update_counter_and_write_table_entry`: Increments a global variable if a byte in d2 is greater than 22, then performs arithmetic on d2 and uses the result to calculate a value that is stored into a table starting at a memory location pointed to by a0.
- `step_position_with_threshold`: Calls two routines, then returns unless a shared input value reaches 23 or more.
- `check_threshold_and_advance_counter`: Calls a routine, then returns unless a shared input value reaches 23 or more.
- `update_pointer_and_test_bit`: Loads a pointer from a global variable into a0 and calls another routine, then tests bit 2 of a byte at a global location to either branch to another routine or write the value 21574 to a global variable and call the same routine again with a different pointer.
- `test_bit_and_branch`: Tests bit 4 of a word at a specific memory location and branches to another routine if the bit is set, otherwise it returns.
- `load_address_and_jump`: Loads a 16-bit address into a register and performs a jump to another routine.
- `sum_three_bytes_into_global`: Reads three consecutive bytes from the input address and writes the first, the sum of the first two, and the sum of all three into a shared word, each masked to a fixed bit pattern.
- `decrement_and_dispatch_logic`: Decrements a 16-bit global variable and returns if it is not zero; otherwise, it checks a byte at a global address, writes a long value to a global address, loads an address into a0, compares a 16-bit global variable against 80, calls another routine with d6 set to 90, writes 4 to a global address, and either branches or jumps to an address from a table based on a bit test of a global byte.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `no_op_routine`: The routine is empty and performs no operations before returning.
- `start_blitter_shifted_copy`: Computes a shift and a blitter control value from shared values, waits until the blitter is idle, then writes the blitter pointer, modulo, mask and size registers to run a shifted copy.
- `clip_and_blit_sprite_entry`: Reads a sprite entry from a table, clips its position against fixed bounds of 173, and returns early when it lies outside them.
- `decode_joystick_direction_bits`: Combines bits of the joystick movement and fire-button readings into a shared direction byte.
- `call_helper_for_21_rows`: Computes a table address from a shared index, then calls a helper 21 times, advancing the position by eight and the table address by two each time.
- `expand_and_bit_manipulate_data_2`: Performs a series of bitwise operations on a data structure, expands it into a larger buffer using nested loops, and calculates a value based on the product of dimensions to store in a global variable.
- `expand_and_bit_manipulate_data`: Iterates through a nested structure to expand data by multiplying dimensions, performs bitwise inversion and bit-shifting operations on the resulting values, and writes the transformed data to a destination buffer.
- `wait_until_sign_bit_set`: Spins while its input word stays non-negative and returns once the sign bit is set.
- `compare_and_branch_values`: Subtracts a 16-bit value in d1 from d3, branches if the result is less than 1, subtracts d0 from d2, branches if d2 is less than 32, loads a byte from a global variable into d2, compares it to 36, and returns.
- `compare_and_branch_data`: Compares a byte from a memory location pointed to by a register against a constant value and branches to another routine if they are equal; otherwise, it performs a series of arithmetic operations on a 16-bit value in a register and returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `init_sound_channels_from_table`: Stores the input value as a shared index and checks it against a range of 0 to 12.
- `init_sound_channels_when_allowed`: Returns its input unchanged when a shared enable value is set and the input is below a stored limit.
- `update_channel_flags_and_helpers`: Updates a shared set of channel flags, calling a routine for each active group and combining bits into a shared mask that is cleared at the end.
- `reset_audio_channel_state`: Clears two shared state words, calls four routines in sequence, then clears a shared flag word.
- `reset_channel_record_fields`: Clears one word of a record, sets a second word to one and a third to zero, then clears a word at a second address.
- `run_helper_for_active_entries`: Checks the input index against a range of 0 to 12.
- `call_helper_for_nonzero_entries`: Walks four table entries from an index derived from the input, calls a helper for each nonzero entry and returns the input value.
- `scan_table_for_nonzero_entries`: Repeatedly calls a helper, then scans a table forward for the next nonzero entry until the count runs out.
- `update_audio_channel_parameters`: This routine checks a global variable against a register value, updates a global variable, and then traverses a table to update several fields in a structure, including setting a pointer or a value from a table-based offset.
- `reset_audio_state`: This routine resets shared audio state, clears certain shared flag words, and resets audio channel volume and pointer fields based on a shared channel-flag value.
- `inspect_undecompiled_routine`: No decompiled body is available, so only the listed facts are known: 17 instructions, ten callers and one call to an unnamed routine.
- `pop_and_update_linked_list`: Decrements a byte at offset 24 of a structure and, if it reaches zero, updates the structure's pointer at offset 6 with a value from a memory buffer.
- `store_pointer_and_branch`: stores the value in a3 into a memory location at an offset from a4 and then branches to another routine.
- `jump_to_inspect_undecompiled_routine`: Jump stub: passes control straight on to inspect_undecompiled_routine.
- `jump_to_inspect_undecompiled_routine_2`: Jump stub: passes control straight on to inspect_undecompiled_routine.
- `inspect_undecompiled_routine_2`: No decompiled body is available, so only the listed facts are known: 3 instructions and one call to an unnamed routine.
- `inspect_undecompiled_routine_3`: No decompiled body is available, so only the listed facts are known: 2 instructions and one call to an unnamed routine.
- `inspect_undecompiled_routine_4`: No decompiled body is available, so only the listed facts are known: 2 instructions, an unreached entry and one call to an unnamed routine.
- `modify_status_and_branch`: clears bit 4 and sets bit 7 of d7, clears a 16-bit value at offset 76 of a4, and then branches to another routine.
- `set_bits_and_copy_bytes`: sets bit 3 of d7, copies a byte from a3 to offset 36 of a4, copies another byte from a3 to offset 37 of a4, and then branches to another routine.
- `mask_bits_and_copy_bytes`: masks d7 with a constant, sets bit 0 of d7, copies a byte from a3 to offset 33 of a4, copies another byte from a3 to offset 34 of a4, and then branches to another routine.
- `clear_bit_and_branch`: clears bit 0 of d7 and then branches to another routine.
- `inspect_undecompiled_routine_5`: No decompiled body is available, so only the listed facts are known: 12 instructions and one call to an unnamed routine.
- `execute_unnamed_routine`: This routine consists of two instructions and calls another routine.
- `inspect_undecompiled_routine_6`: No decompiled body is available, so only the listed facts are known: 6 instructions and one call to an unnamed routine.
- `invoke_unnamed_subroutine`: This routine contains seventeen instructions and calls another unnamed routine.
- `inspect_undecompiled_routine_7`: No decompiled body is available, so only the listed facts are known: 8 instructions and one call to an unnamed routine.
- `inspect_undecompiled_routine_8`: No decompiled body is available, so only the listed facts are known: 231 instructions, one caller and one call to an unnamed routine.
- `start_audio_channels_from_flags`: Sets up audio channel records from a shared channel-flag value: for each set flag bit it resets that channel's volume and pointer fields.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_helper_return_inputs`: Calls one routine and returns the two input register values unchanged.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_no_op_stub`: Its body only returns, so it does nothing.
- `accumulate_or_clear_value`: Adds the input word to a shared value unless that value is zero, in which case it leaves the value at zero.
- `add_input_to_shared_value`: Adds its input word to a shared value and returns.
- `step_counter_and_copy_entries`: Clamps a shared counter at zero and, while it is positive, copies eight words between table locations and advances the counter and a second counter.
- `decrement_bcd_counter_and_call`: Calls a routine, then unless a shared value equals a fixed sentinel, decrements a shared counter.
- `inspect_undecompiled_routine_9`: No decompiled body is available, so only the listed facts are known: 9 instructions, one caller and one call to an unnamed routine.
