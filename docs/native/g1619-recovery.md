# Game 1619 (rick dangerous ii (1990)(micro style)[cr fas][t fas]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01619). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902546 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 15 of 15 units identical (bytes and relocations), 33 raw-word lines |
| Functions named and described | pass, 331 of 331 named (0 author symbols), 331 of 331 described (47 by rule, 280 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4100 of 4100 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 9254 of 10137 instructions identical in start and length, 1 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 33513 |
| code | 38932 |
| data | 415816 |
| padding | 77 |
| unused | 414208 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf:bootblock | 1 | 39 | 1 |
| logged disk load from Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load | 1 | 865 | 8 |
| logged disk load from Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load | 1 | 88 | 3 |
| logged disk load from Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load | 1 | 617 | 39 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 92 | 6 |
| logged disk load from Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf | 1 | 8418 | 264 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4 | 2 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 3 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 2 | 1 |

## Cases handled

- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: custom-format (non-DOS) disk, 926 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 2003 of the 4078 PCs the canonical run logged (121382 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 98304 of its 121382 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: the boot block reads 11264 bytes from disk offset 563200 to an address (found statically); recovered as a unit
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: the boot block reads 11264 bytes from disk offset 574464 to an address (found statically); recovered as a unit
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: the boot block reads 2560 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- 2 executed PCs run code whose absolute operand the game patches at run time (first an address): mapped to the file bytes they came from
- runtime segment an address (play-rs-m1500): 89 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: 2874 executed PCs whose RAM was overwritten before any snapshot match its bytes 207360-255776: recovered as a unit run at an address
- 18 executed PCs ran code that was later overwritten: recovered from their logged first-execution bytes as 9 first-run image unit(s) in 1 version group(s); 180 unexecuted gap bytes taken from the RAM snapshot; 0 not decodable
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: 4 logged loads cover 73504 bytes of the image
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: blocks 27-27 (bytes 13824-14336) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.09)
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: blocks 29-29 (bytes 14848-15360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.89)
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: blocks 31-31 (bytes 15872-16384) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.76)
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: blocks 33-33 (bytes 16896-17408) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.72)
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: blocks 35-35 (bytes 17920-18432) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.99)
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: blocks 500-883 (bytes 256000-452608) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.77)
- rick dangerous ii (1990)(micro style)[cr fas][t fas]/Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf: blocks 1144-1157 (bytes 585728-592896) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.99)
- 3 packed track region(s), 205824 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `read_disk_track_with_retries` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load +an address to an address (2560 bytes)): Performs up to five attempts at a disk read, setting the disk pointer, DMA and length registers on each attempt and calling another routine to wait for completion. Kind: track/sector check with retry-based sabotage path. Pass path: The loop continues if sVar2 (retry counter) has not reached -1 and the check (u3__sub_06DA) does not return true (bVar3), allowing the routine to proceed or retry a valid track/sector read..
- `set_audio_dma_control` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load +an address to an address (2560 bytes)): Writes to a custom chip register, another custom chip register, a custom chip register, a custom chip register, a global variable, and a global variable. Kind: disk hardware/sync check. Pass path: the branch or state where the hardware-specific parameters (ADKCON, DSKLEN, etc.) or sync-checks validate the disk/track or bypass the sabotage path.
- `set_adkcon_register` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load +an address to an address (2560 bytes)): Writes a constant word to a hardware register, then checks a global variable and otherwise sets a register or modifies d1. Kind: disk/track check (bit-pattern/sync check). Pass path: the branch where the condition is false (skipping the modification or specific bit-set/clear logic).
- `set_flag_and_call_twice` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load +an address to an address (2560 bytes)): It stores a constant byte into a global variable, then calls another routine twice in sequence. Kind: loader-based protection (track/disk check). Pass path: execution continues to subsequent instructions (no branch or trap triggered).
- `read_port_into_status_byte` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load +an address to an address (2560 bytes)): It calls another routine to obtain a byte, clears bit one of that byte when a secondary value from the same call is non-negative, and then forces bit zero high before storing the result into a global variable. Kind: disk hardware/track check. Pass path: the branch where bVar1 (the status byte) permits the masking/logic to proceed, specifically avoiding the failure state where the check fails or returns an invalid status (extraout_D1b < 0).
- `update_ciab_mask` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load +an address to an address (2560 bytes)): Calls a routine and writes its return value to a hardware register. Kind: disk hardware check (CIAB_PRB). Pass path: the routine checks for a specific hardware-dependent pattern or disk signature (CIAB_PRB) and if present, allows execution to proceed to the main game code; if absent, it would fail or deviate..
- `compute_ciab_mask_from_table` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load +an address to an address (2560 bytes)): It derives a bit mask from a global variable by clearing one bit chosen by a frame-relative index, and also clears a second bit when a related table entry has its or-ed bitset. Kind: manual/code-wheel/colour-card lookup. Pass path: the branch where the bitmask check does not trigger a modification or failure (the default path where the bitmask remains unchanged or follows the standard logic without the conditional bit-clear or branch failure).
- `poll_input_and_update_globals` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf +an address run at an address (48416 bytes, matched to executed PCs)): Repeatedly calls another routine and updates a global variable based on a counter and the value held in a stack slot. Kind: disk hardware/track/sector check. Pass path: the loop continues if (DAT_00bfe001 & 8) != 0 and sVar2 != -1.
- `set_audio_dma_control_2` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf +an address run at an address (48416 bytes, matched to executed PCs)): The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable. Kind: disk hardware/sync check. Pass path: the branch where the routine completes without triggering a failure or a jump to a failure-state (e.g., bne.b an address or similar valid execution flow or simply returning to the caller if the hardware-specific registers match expected values o.
- `set_global_bit_conditional` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf +an address run at an address (48416 bytes, matched to executed PCs)): Writes a constant word to a hardware register, checks a bit in a global variable, and if the bit is not set, sets a new value in a register. Kind: disk hardware/track check. Pass path: the branch where the condition (bit check) does not trigger a change to the global state, or specifically the path where the conditional logic bypasses the modification of the target bit..
- `set_bit_in_global_flag_2` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf +an address run at an address (48416 bytes, matched to executed PCs)): Reads a 16-bit value from a table, adds a constant, and uses the result as a bit index to set a bit in a global variable. Kind: disk hardware/track check (CIAB_PRB). Pass path: the routine modifies a global flag (DAT_00bfd100) based on a bit-set operation, which is a characteristic side-effect of a successful hardware-dependent check or a specific track/sector-dependent bit-set used to validate a disk signature or.
- `set_flag_and_call_twice_2` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf +an address run at an address (48416 bytes, matched to executed PCs)): Stores a constant byte into a global variable, then calls another routine twice in sequence and returns. Kind: track/disk check. Pass path: execution continues to next instruction after calls.
- `read_port_into_status_byte_2` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf +an address run at an address (48416 bytes, matched to executed PCs)): It calls another routine to obtain a byte, clears bit one of that byte when a secondary value from the same call is non-negative, and then forces bit zero high before storing the result into a global variable. Kind: disk hardware/track check (CIAB_PRB). Pass path: the branch where the check succeeds (avoiding the error/sabotage path or invaliding the status byte).
- `update_ciab_mask_2` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf +an address run at an address (48416 bytes, matched to executed PCs)): Calls a routine and then stores the result of that routine into a hardware register. Kind: disk hardware check (CIAB_PRB). Pass path: the code following the routine executes normally, implying the check passes or a valid state is returned to the caller.
- `compute_ciab_mask_from_table_2` (Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf +an address run at an address (48416 bytes, matched to executed PCs)): Derives a bit mask from a global variable by clearing one bit chosen by a frame-relative index, and also clears a second bit when a related table entry has its low bit set. Kind: disk hardware/CIAB_PRB check. Pass path: the branch where the mask bit remains unchanged or follows the logic without triggering a failure-state modification.

## Functions


### Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf:bootblock

- `initialize_boot_environment`: Configures a structure in a1 with certain values, calls a library function, and then enters a loop that updates the structure's members and calls the library function again until a certain byte in the structure is not zero, finally jumping to a routine or returning a value.

### logged disk load from Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load

- `initialize_boot_sequence`: Calls two routines, sets a hardware register, clears a hardware register, and calls a third routine; then sets several hardware registers, checks a bit, and if certain conditions are met, sets a pointer, calls a trap, and writes several hardware registers or table values.
- `initialize_boot_tables`: Calls two library functions, updates a table with a pointer and a calculated value, and writes constant values to several hardware registers and a global variable.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_2`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_3`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `copy_boot_data_blocks`: Calculates a destination address based on values read from a buffer, and if a certain byte is not zero or certain, copies a fixed number of bytes from a source table to that destination address using a loop.
- `clear_memory_block`: Clears an 8192-byte block of memory starting at a global variable address using a loop, then returns.
- `contains_data_table`: This routine contains a sequence of instructions that do not change any data registers and does not contain any loops or calls.

### logged disk load from Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load

- `decompress_bitstream_to_ram`: Reads a bit-packed stream, using bit-refilling and bit-reading routines to determine a number of bytes to copy, then writes those bytes to a destination buffer in RAM. The routine uses a loop to check for end-of-stream or specific bit patterns to decide whether to copy a fixed number of bytes or a variable number of bytes calculated from the bitstream.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### logged disk load from Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf boot-block load

- `initialize_boot_address`: Loads the address of a nearby instruction into a global variable, executes a trap, and then branches to a nearby instruction.
- `write_stack_pointer_value`: Writes a constant word value to the location pointed to by the stack pointer and then returns from a subroutines.
- `copy_boot_code`: Disables interrupts, sets hardware registers, sets the stack pointer, and loops to copy a certain number of bytes from a source address to a destination address before jumping to a new address.
- `initialize_hardware_and_jump`: Sets a hardware register, loads several constants into registers, calls another routine, and if the return value is non-zero, enters an infinite loop or jumps to a new address.
- `advance_blitter_cell_copy`: Divides a position by eleven to find a cell and offset, then copies a run of cells with the blitter, advancing until the count is used up.
- `write_disk_sector_sync_marks`: Clears three 16-bit fields in a stack frame and enters a loop that runs five times.
- `read_disk_track_with_retries`: Performs up to five attempts at a disk read, setting the disk pointer, DMA and length registers on each attempt and calling another routine to wait for completion.
- `verify_sector_header_with_retries`: Tries up to ten times to read a sector header, comparing the track and sector values with the unable expected ones.
- `process_object_state_and_transforms`: Calculates a pointer or offset, calls several routines to verify or update data, and loops to update a certain number of items in a structure or checks hardware or status bits before returning a zero or negative value.
- `scale_register_value`: multiplies a value derived from subtracting a constant from a register by five hundred and returns.
- `set_flag_and_increment_counter`: It sets a bit in a 16-bit word located a few bytes before a pointer in a6, using a bit index derived from d3.
- `clear_ten_strided_longwords`: Writes zero to ten longwords spaced 272 bytes apart, starting at a pointer held in a register.
- `repack_result_word_and_byte`: Calls a routine and repacks its 32-bit result, placing the low word first and the top byte of the result in the second word.
- `interleave_word_bits`: It reads two consecutive words from a pointer and returns a value formed from their low bits.
- `chain_call_result`: Calls an unnamed routine and passes its 32-bit result as the argument to a second unnamed routine.
- `fall_through_entry_4`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `compute_xor_checksum_by_length`: XORs a run of longwords whose count is derived from a length argument, and returns the result masked to the alternate bits.
- `fill_memory_with_pattern`: Fills a block whose pointer is stored in the frame with a repeating alternating-bit longword, writing eight longwords per loop pass.
- `initialize_object_state`: Calls another routine and uses its result to compute three integers stored at fixed offsets in a structure, then writes two constant words and a derived word into it.
- `set_object_transforms`: Writes several fields of a structure from the results of unnamed routines, storing constant words and copying values between the results, and calls a routine that waits for the blitter to finish.
- `wait_for_blitter`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `reset_blitter_state`: Waits for the blitter, then writes constant masks and a control value into the blitter and DMA registers and clears a global variable.
- `transform_and_call_twice`: Pushes d0 onto the stack, shifts it right by one, calls another routine, restores d0, and calls that routine again.
- `update_sign_bit_from_neighbour`: Adjusts bit seven of a byte using bit zero of the byte before it: when that bit is clear, it sets bit seven unless bit six is already set, and when that bit is set, it clears bit seven.
- `bitwise_mix_store_word`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `set_audio_dma_control`: Writes to a custom chip register, another custom chip register, a custom chip register, a custom chip register, a global variable, and a global variable.
- `set_hardware_and_branch`: Loads a pointer to a hardware register, sets a value in a register, calls a routine, checks a bit in a hardware register, and calls another routine or sets a register to a constant and branches.
- `set_global_flags`: It writes two constants to the same global variable in sequence, leaving the second value in place.
- `set_adkcon_register`: Writes a constant word to a hardware register, then checks a global variable and otherwise sets a register or modifies d1.
- `set_bit_in_global_flag`: It reads a 16-bit value from a table, adds a constant, and uses the result as a bit index to set a bit in a global variable.
- `set_flag_and_call_twice`: It stores a constant byte into a global variable, then calls another routine twice in sequence.
- `update_table_entry`: Reads a 16-bit value from a table via an index in a structure, calls another routine when the value is negative, then spins a number of times derived from the difference between two 16-bit fields before writing a value from a register back into the table and calling another routine.
- `wait_then_clear_table_entry`: Waits, calling another routine up to 56 times, for bit four of a shared variable to clear.
- `read_port_into_status_byte`: It calls another routine to obtain a byte, clears bit one of that byte when a secondary value from the same call is non-negative, and then forces bit zero high before storing the result into a global variable.
- `update_ciab_mask`: Calls a routine and writes its return value to a hardware register.
- `compute_ciab_mask_from_table`: It derives a bit mask from a global variable by clearing one bit chosen by a frame-relative index, and also clears a second bit when a related table entry has its or-ed bitset.
- `wait_until_routine_signals_one`: Calls a routine repeatedly, and between calls busy-waits while the low bit of a certain hardware register remains set, returning zero once the called routine returns one.
- `check_hardware_bit`: Checks a hardware register bit and otherwise decrements a register or branches.
- `program_cia_timer_registers`: Writes three fixed values to the CIAA control and timer registers, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `setup_graphics_and_tables`: Calls several routines, writes to hardware registers, and populates a table with values from a pointer, otherwise it clears a register and returns.
- `initialize_graphics_structures`: Calls two library functions, updates a global pointer, and populates a table with or-ed or masked values while setting certain hardware registers and a loop counter.
- `fall_through_entry_5`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `expand_table_data`: Iterates through a source buffer, calculating target addresses based on a table, and copies blocks of 8 bytes from a source table to a destination table.
- `populate_pointer_table`: A loop that writes the current pointer address and its high word into a table, incrementing the pointer by a fixed amount each iteration.
- `clear_memory_range`: Clears a range of memory starting at a fixed address using a loop.

### logged disk load from Rick Dangerous II (1990)(Micro Style)[cr FAS][t FAS].adf

- `update_state_and_branch_on_variable`: Updates a global variable with a sequence of addresses and calls another routine, then enters a loop that checks a global flag and certain character values, potentially calling other routines or branching back to a global variable or a loop header based on the state of a global variable and other flags.
- `set_zero_and_load_address`: Clears register d0 and loads the address of a global variable into a0, then jumps to another routine.
- `call_other_routine`: Saves registers, loads two words from a pointer-based table into d1 and d2, sets d3 to a constant, loads a pointer into a1, calls another routine, and returns the resulting registers.
- `update_tile_index_mapping`: Saves registers, loads two words from a table into d1 and d2, sets d3 to a constant, loads a pointer into a1, and calls another routine via a branch with link.
- `advance_blitter_cell_copy_2`: Divides a position by eleven to find a cell and offset, then copies a run of cells with the blitter, advancing until the count or a certain condition is met, and returns the updated position.
- `initialize_input_state`: Clears three 16-bit fields in a stack frame and enters a loop that runs five times.
- `poll_input_and_update_globals`: Repeatedly calls another routine and updates a global variable based on a counter and the value held in a stack slot.
- `validate_input_with_retry`: Repeatedly calls helper functions to check a flag and decrement a counter, returning early if the flag is clear or the counter expires.
- `iteratively_validate_object_state`: Iterates up to 11 times, checking a flag in a hardware register and validating a pointer-based structure or table, while updating a counter at a negative offset from a6 and performing conditional checks on bit-flags and pointer-relative values.
- `multiply_register_by_constant`: multiplies the value in d1 by 512 after subtracting a 16-bit value from the stack-relative offset or pointer in a6 and storing the result in d1.
- `set_flag_and_increment_counter_2`: It sets a bit in a 16-bit word located a few bytes before a pointer in A6, using a bit index derived from D3.
- `clear_table_entries`: It clears a table by writing zero to successive entries.
- `repack_result_word_and_byte_2`: Calls a routine and repacks its 32-bit result, placing the low word first and the top byte of the result in the second word.
- `interleave_word_bits_2`: It reads two consecutive words from a pointer and returns a value formed from their low bits.
- `chain_call_result_2`: Calls an unnamed routine and passes its 32-bit result as the argument to a second unnamed routine.
- `fall_through_entry_6`: Entry point part-way through straight-line code: runs two instructions (lea.l, moveq) and then falls through into the code that follows, with no return of its own.
- `compute_xor_checksum`: It iterates through a sequence of words pointed to by an input pointer, XORing each into an accumulator.
- `fill_memory_with_pattern_2`: Fills a block of memory starting at a pointer stored in the stack frame with the repeating pattern a fixed address, writing eight longwords per loop pass, and returns.
- `initialize_object_state_2`: Calls another routine and uses its return value to compute and store three integers at offsets 76, 80, and 84 of a structure pointed to by a2, then writes two constant words and a derived word into offsets 64, 66, and 88 of that structure.
- `set_object_transforms_2`: Updates several fields of a structure, including source/destination pointers and control words, using results from other routines and performing arithmetic on coordinate or size values, then calls a routine that waits for the blitter to finish.
- `wait_for_blitter_2`: Waits until the blitter has finished its current operation by polling the blitter-busy bit of DMACONR in a loop, then returns.
- `reset_blitter_state_2`: Waits for the blitter, then writes constant masks and a control value into the blitter and DMA registers and clears a global variable.
- `mix_and_call_routine`: It pushes a longword onto the stack, shifts it right by one bit, calls another routine, restores the longword from the stack, and calls another routine.
- `update_sign_bit_from_neighbour_2`: Adjusts bit seven of a byte using bit zero of the byte before it: when that bit is clear, it sets bit seven unless bit six is already set, and when that bit is set, it clears bit seven.
- `bitwise_mix_store_word_2`: The routine mixes the input word with a transformed mask derived from the low bits of the input, optionally clears the sign bit based on a flag byte before the output pointer, and stores the resulting word through the pointer.
- `set_audio_dma_control_2`: The routine writes to a global variable, another global variable, a custom chip register, a custom chip register, a global variable, and a global variable.
- `configure_timer_and_branch`: Loads a pointer to a hardware register, sets a value in a register, calls a routine, checks a bit in a hardware register, and calls another routine or sets a register to a constant and branches.
- `set_global_flags_2`: Writes two constants to the same global variable in sequence, leaving the second value in place.
- `set_global_bit_conditional`: Writes a constant word to a hardware register, checks a bit in a global variable, and if the bit is not set, sets a new value in a register.
- `set_bit_in_global_flag_2`: Reads a 16-bit value from a table, adds a constant, and uses the result as a bit index to set a bit in a global variable.
- `set_flag_and_call_twice_2`: Stores a constant byte into a global variable, then calls another routine twice in sequence and returns.
- `update_table_entry_v2`: Reads a 16-bit value from a table via an index in a structure, calls another routine when thevalue is negative, then spins a number of times derived from the difference between two 16-bit fields before writing a value from a register back into the table and calling another routine.
- `poll_status_with_timeout`: Repeatedly polls a global status byte up to eighty-five times, clearing a word in a table indexed by a value from the caller's stack frame if a specific bit is set, and returns a fixed non-zero value if the loop completes without success.
- `read_port_into_status_byte_2`: It calls another routine to obtain a byte, clears bit one of that byte when a secondary value from the same call is non-negative, and then forces bit zero high before storing the result into a global variable.
- `update_ciab_mask_2`: Calls a routine and then stores the result of that routine into a hardware register.
- `compute_ciab_mask_from_table_2`: Derives a bit mask from a global variable by clearing one bit chosen by a frame-relative index, and also clears a second bit when a related table entry has its low bit set.
- `wait_until_routine_signals_one_2`: Calls a routine repeatedly, and between calls busy-waits while the low bit of a global flag remains set, returning zero once the called routine returns one.
- `check_timer_bit`: Checks a bit in a hardware register and, if set, decrements a register and potentially returns or continues.
- `program_cia_timer_registers_2`: Writes three fixed values to the CIAA control and timer registers, then returns.
- `fall_through_entry_7`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `copy_and_reset_sound_state`: Copies a word from a global address to another, then calls a routine and returns.
- `compare_and_branch_table_index`: Compares two words from a table, and if they are not equal, calls another routine before potentially calling a third routine and returning.
- `reset_sound_channels`: Calls several subroutines, stores the value from a global variable into another global variable, and returns.
- `fall_through_entry_8`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_9`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_10`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_11`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_12`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `reset_game_state_variables`: Clears many word variables in memory to zero, sets a few flags to one and one to 256, then zeroes four entries spaced 88 bytes apart in a table pointed to by a6.
- `copy_state_words_to_buffer`: Copies eight word values from game state variables into consecutive words starting at the buffer at 2FD0, then returns.
- `call_refresh_and_load_state_registers`: Calls a subroutine, then loads eight words from the 2FD0 buffer into d0 through d7 and returns.
- `update_scrolling_and_tile_buffer`: Updates a scrolling-related buffer by calculating new pointer or offset values based on a counter, and if certain flags or conditions are met, it executes a loop that writes a sequence of calculated values into a table or structure in memory, or calls another routine to handle a specific state or certain bit-flags.
- `spawn_projectile_logic`: Checks a bit in a register and otherwise writes a zero to a global word and returns; if the bit is set, it writes a one to a global word, checks a global word, checks another global word, and if certain conditions are met, calls a routine and writes a one to a global word before returning.
- `spawn_projectile_with_direction`: Plays a sound, marks the projectile active, decrements ammo, sets projectile position from player coordinates and chooses horizontal velocity and sprite based on facing.
- `update_projectile_position_and_collision`: Checks the active flag, computes a screen position from the object's x/y and velocity, calls the collision routine, and updates the stored position or clears the active flag.
- `update_player_movement_and_collision`: Updates the player's horizontal position, velocity, and animation state based on certain flags, checks for screen-edge collisions, and updates a global pointer or counter, then calls a routine to handle level or death transitions.
- `read_next_scripted_byte`: Returns the next byte from a scripted byte stream, refilling from the pointer and count, or returns a fixed byte when the stream is disabled.
- `clear_sound_state_if_active`: Tests the word at a fixed address; if zero returns immediately, otherwise clears that word and resets the sound state words and pointer at a fixed address before returning.
- `set_global_zero`: Sets a global variable to zero and then calls itself recursively.
- `start_sound_from_channel_table`: Clears the sound flag, checks the active word, calls the sound setup and stop routines, then loads channel parameters from the table into d0-d7 and runs the sound start and clear routines.
- `call_refresh_and_load_state_registers_2`: Calls a routine that loads eight words from a buffer into registers d0 through d7 and returns.
- `save_registers_and_run_frame_setup`: Loads a base pointer into a6, calls several setup routines, stores d2 through d7 into fixed memory words, runs more routines, sets a flag word to all ones, calls another routine, clears the flag and returns.
- `scan_object_table_and_apply_flags`: Walks a four-byte-entry table, matches type and position bytes against current values, sets flag words and calls a routine with computed offsets, then restores registers and returns.
- `toggle_sound_flag_and_call`: Tests the word at absolute address 16afa; if zero stores 232 there and calls the routine at 8cc8, otherwise stores zero and calls the routine at 8d90, then returns.
- `load_level_pointer_table_entry`: Saves d0-d1/a0, indexes the table at 54c00 by d0 times eight, stores derived pointers and d1 into globals 6600-6508, 44fa and 4662, restores registers, then calls two routines and returns.
- `count_down_scan_table_words`: Saves d0/a0, reads the word at 125a6, subtracts one, doubles it to index the table at 1466a, then repeatedly calls the routine at 469e while decrementing d0 until it reaches zero, restores registers and returns.
- `clear_high_bit_walk_entity_list`: Saves d0/a0, indexes a table at a fixed base by d0 times eight, then walks four-byte records clearing bit seven of each first byte and skipping extra records until a zero byte is found.
- `advance_entity_pointer_past_visible`: Saves d0/a0, loads a pointer from a global, and advances it through four-byte records while the first byte is nonzero and the second byte times eight stays below a global limit, storing the result.
- `scan_entities_and_dispatch_collision`: Checks two global flags, then walks four-byte entity records up to a limit, testing flag bits and calling collision or spawn routines before restoring registers and returning.
- `dispatch_object_byte_to_three_handlers`: Saves d0, reads the byte at a0, and calls one of three routines depending on whether it is 120, 124, or between 1 and 116, then restores d0 and returns.
- `parse_script_command_and_update_state`: Parses a command byte from a script stream, updating a state structure in a6 with new instruction pointers, delays, and jump targets, or resets certain state offsets if specific command types or flags are detected.
- `init_projectile_entity_from_type_byte`: Calls a collision check, then uses the low bits of the byte at a0 to select a type table entry, storing animation, position, velocity and flags into a6 and marking a0.
- `scan_four_objects_for_zero_state_2`: Pushes d0 to the stack, loads a pointer to a table into a6, and then branches to a different part of the routine.
- `scan_four_objects_for_zero_state`: Pushes d0, walks up to four 88-byte records from the absolute base 93576, and returns with carry clear if a record word is zero, carry set otherwise.
- `check_a6_object_inside_screen_bounds`: Pushes d0, reads the words at offsets 6 and 2 of a6, and returns carry clear only if both lie within the screen limits, carry set otherwise.
- `init_two_sprite_block_arrays`: Saves d7 and a6, then calls a per-block setup routine four times over one array and six times over a second array, stepping 88 bytes each time, before restoring registers.
- `clear_channel_flag_and_clear_bit7`: If the word at a6 is nonzero, zeroes it, loads the pointer at offset 42 into a0, and if non-null clears bit 7 of the byte it points to.
- `clear_channel_flag_unless_bit6_set`: If the word at a6 is nonzero, zeroes it, loads the pointer at offset 42 into a0, and clears bit 7 of its byte only when bit 6 of byte 3 is clear.
- `process_object_collision_flags`: Iterates over up to three sub-entries of the object at a0, testing flag bits in each entry's byte 3 and calling collision checks against the player and object tables, clearing bit 7 on failure and returning carry set on collision.
- `check_player_against_screen_bounds`: Returns carry clear if the player position in d0/d1 plus d2/d3 extents lies within the screen bounds at the global coordinate words, otherwise returns carry set; skips check when the pause flag is unable to be checked.
- `check_player_hits_active_enemy`: Saves d6 and d7, then if a global word is non-zero, loads two global words into d6 and d7 and calls an overlap test, returning carry set if the point is inside the rectangle, otherwise it returns with carry clear.
- `check_two_flags_then_point_in_rect`: Saves d6 and d7, then if two global words are non-zero, loads two global words into d6 and d7 and calls a point-in-rectangle test, returning carry set if the point is inside the rectangle, otherwise it returns with carry clear.
- `check_player_hits_active_enemy_2`: Checks if a global flag is non-zero, then loads two words from a global pointer into d6 and d7 and calls a rectangle-point-test routine, returning carry set if the test passes and carry clear otherwise.
- `point_in_rectangle_test`: Saves d6-d7, subtracts d0 from d6 and d1 from d7, compares against d2 and d3 bounds, and returns carry set if the point lies inside the rectangle.
- `check_entity_against_expanded_bounds`: If the word at offset 78 of the a2 structure is zero, tests d0 and d1 against bounds built from words at offsets 2 and 6 plus d2 and d3, returning carry set when inside and clear when outside.
- `check_entity_against_stored_extent_bounds`: Tests d0 and d1 against bounds formed from the words at offsets 2 and 6 of the a2 structure plus the extents at offsets 38 and 40, returning carry set when inside and clear when outside.
- `iterate_six_entity_slots_and_dispatch`: Returns immediately if the global word is nonzero; otherwise walks six 88-byte slots from the entity table, calling the sub-routine for each slot whose first word is nonzero.
- `update_player_state_and_position`: Reads the player object at a6, tests input bits and flags, updates position, velocity and animation pointers, and calls collision and sound routines before returning.
- `iterate_four_player_slots`: Loops four times over player slot structures spaced 88 bytes apart starting at a fixed address, calling the player update routine only when the slot's first word is nonzero.
- `update_player_physics_and_input`: Updates a player slot's velocity, position, collision flags, and animation state from joystick bits and global tables, calling several helper routines and returning when done.
- `check_entity_type_and_flag_bits`: Saves d5, reads a word from a pointer in a6, masks its low nibble, and if it equals 4, calls another routine and tests the low two bits of a global byte, returning carry set if they are zero and clear otherwise.
- `scan_four_slots_for_free_entry`: Saves d7 and a2, walks four 88-byte slots starting at a global table, and for each active slot with two zero words, calls a test routine, setting word 76 of the structure to minus one when the test fails.
- `check_state_one_and_probe_region`: Returns immediately unless the global word at the state address equals one, then saves d4-d7, loads coordinates and sizes into d4-d7, calls a region test, and sets a global word to minus one if it fails.
- `fall_through_entry_13`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_14`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `clear_global_word`: Sets a global word to zero and returns immediately.
- `start_timer_and_play_sound`: Tests if a certain global word is zero; if so, sets it to -1, stores 8192 and 25 in certain global locations, and calls the sound routine with d0=17 and d1=0.
- `stop_timer_and_clear_sound_bit`: If a certain global word is non-zero, clears it, calls a conversion routine, calls the sound routine with d0=1 and d1=0, then sets bit 7 of a byte at a pointer in a0.
- `tick_timer_and_bcd_countdown`: If a timer is active, decrements the countdown word; when it reaches zero, it reloads 25, performs a BCD subtraction using a pointer-based calculation, and clears the active flag if the timer value is zero.
- `fall_through_entry_15`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `init_five_sprite_slots`: Saves d7, a0, and a6, writes 20 to a global variable, then fills five 88-byte sprite slots from a source table, calling a setup routine per slot, and clears several state words before returning.
- `update_player_and_check_collisions`: Tests two state words, moves five sprite slots by deltas, then runs bounding-box collision checks against obstacles and triggers death or respawn handling before returning.
- `init_shot_velocity_from_player_and_target`: Decrements a cooldown word and returns if nonzero; otherwise calls a sound routine, sets timer and sprite fields, computes velocity deltas from player and target positions, halves them until within four pixels, and stores the result.
- `move_shot_and_check_collision`: Decrements a timer, adds velocity to the shot position, writes the position to a sprite, checks tile collision and a flag bit, and on hit sets a hit marker and clears the active flag.
- `reset_five_sprite_objects_and_report`: Saves d7 and d0-d1, calls the sound routine, then loops five times setting the active word of each 88-byte sprite object to one, decrements a shared counter, and returns with carry set if it reached zero.
- `fall_through_entry_16`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_17`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_18`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_19`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `check_tile_attributes_and_update`: Reads tile attribute bytes from a tilemap or a list of pointers, ORs certain bits together, and updates a global variable with the result.
- `scan_tilemap_and_update_collision_state`: Saves registers, sets scroll or tilemap globals, ORs tile attribute bytes from a table via a pointer, then walks an object list testing bounding boxes and updating collision or flag fields in a global structure.
- `fall_through_entry_20`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `rectangles_overlap_test`: Saves all data registers, adds d4 to d6 and d5 to d7, compares d0 and d1 against those sums and d2/d3 against d4/d5, then restores registers and returns carry set if the two rectangles overlap, clear otherwise.
- `scan_objects_for_collision_flag`: Reads a byte from a list using an index, checks if a bit is already set in a global flag, and if not, iterates through a list of 44-byte object records to check for a point-in-rectangle collision, setting bit 1 of the global flag if a collision is detected.
- `scan_terrain_tiles_for_player_collision`: Builds a bitmask from tilemap bytes, then iterates through a list of objects to check if their bounding boxes overlap with the player's position, updating contact flags and stored coordinates in a global structure if a collision or certain boundary condition is met.
- `compute_tilemap_row_pointer`: Loads the tilemap base at 65300, adds the row offset from 160ae shifted left twice and the column offset from 160ac divided by eight, returning the tile pointer in a1.
- `blit_tilemap_column_strip`: Saves registers, computes a tilemap pointer from a global variable and a base address, stores it and a remainder count, then calls the column blit and fill routines ten times to draw a vertical strip.
- `copy_tilemap_block_and_advance_column`: Saves registers, copies 39 rows of 32 bytes from a source buffer to a destination buffer, then calls the tile blitter for the current column and advances the column index and source pointer.
- `scroll_background_tile_columns`: Saves registers, copies thirty-nine rows of eight longs from one screen buffer backwards to the next, decrements a column counter modulo four and shifts the source pointer when it wraps.
- `blit_tile_column_from_tileset`: Saves registers, selects a tileset plane from the index in d0, then copies eight tile rows of four bytes each from the tilesset into the destination buffer, advancing the destination by 32 bytes per row.
- `copy_eight_sprite_blocks_from_table`: For eight entries, reads a byte from (a0), scales it by sixteen, and copies four longwords from the table at a fixed address into (a1), 2and 60(a1) and 92(a1), advancing a1 by one longword each time.
- `fall_through_entry_21`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `reset_counter_and_call_two_routines`: Clears the word at a fixed address, copies the word at a fixed address masked with seven into a fixed address, then calls the routines at a fixed address and a fixed address before returning.
- `check_player_x_bounds_and_step`: Tests the pause flag; if clear, reads the player x position, masks it to 8 pixels, compares against a limit and calls the left or right movement routine.
- `move_player_right_clamped`: Adds the horizontal velocity to the player x position, clamps it to the right limit, stores the new position and calls the scroll and animation routines when the pixel column changes.
- `move_player_left_clamped`: Adds the horizontal velocity to the player x position, clamps it to the left limit, stores the new position and calls the scroll and animation routines when the pixel column changes.
- `advance_byte_counter_mod`: Saves d0, loads the word at the counter address, adds the word at the increment address, masks the result to a byte, stores it back to the counter, restores d0 and returns.
- `clear_entity_records_until_negative`: Walks a6 through records of 44 bytes, clearing the first word and several later word and long fields, stopping when the first word is negative, then returns.
- `reset_sound_state_via_pointer`: Saves a1 and a6, loads a1 from the global pointer, points a6 at the sound state area, calls the reset routine, restores a1 and a6 and returns.
- `load_fixed_68000_source_into_a1`: Saves a1 and a6 on the stack, loads the fixed address 68000 into a1 and 16940 into a6, calls the shared copy routine, then restores a1 and a6 and returns.
- `reset_sound_state_via_pointer_v2`: Saves a1 and a6, loads a1 from the global pointer, points a6 at the sound state area, calls the reset routine, restores a1 and a6 and returns.
- `call_two_subroutines`: Calls two different routines and then returns.
- `activate_idle_objects_in_list`: Walks a linked list of object records at a6, and for each active record whose field at offset 20 is zero, calls the object activation routine, advancing by 44 bytes per record.
- `deactivate_active_objects_in_list`: Walks a linked list of object records at a6, and for each active record whose field at offset 20 is nonzero, clears that field to zero and calls the object update routine, advancing by 44 bytes per record.
- `draw_object_sprite_by_type`: Saves registers, selects a sprite base table from the object type at offset 14, computes the sprite address, then draws the object at its x and y coordinates via one of two draw routines depending on the flag at offset 18.
- `add_delta_to_active_objects`: Walks a table of 44-byte object records starting at a fixed address, skipping records whose first word is negative or zero, and adds the incoming d0 word to the word at offset 6 of each active record.
- `advance_script_stream_pointer`: Steps the script pointer at offset 30 of a6 through command words, handling jump, delay and call opcodes, updating the current command word at offset 1and returning carry set when a command was consumed.
- `read_next_script_pair`: Reads the next byte pair from the script pointer at offset 22 of a6, decrementing the repeat counter at offset 26, and returns the two sign-extended bytes in d0 and d1 with carry set when data was read.
- `advance_script_stream_pointer_2`: Reads the script pointer at offset 22 of a6, skips zero and marker words, optionally calls a routine for marker entries, loads a count and two signed bytes, advances the pointer and returns with carry set or clear.
- `clear_global_string_buffer`: Sets a global byte to zero and then loops through a buffer starting at a fixed address, setting every byte to zero until a null terminator is reached.
- `reset_script_slot_table`: Saves a6, clears the slot count word, stores the table base pointer, then walks fourteen-byte records clearing each first byte until a negative byte ends the loop, restoring a6 and returning.
- `update_slot_and_increment_count`: Checks a global counter, if not exceeding a limit, saves registers, clears a text field, copies a byte from a source, calls a routine, and increments the global counter.
- `advance_effect_positions_forward`: Checks a global counter, if not zero, saves registers, walks the fourteen-byte effect slots, adding thirty-two to each active slot's position field and freeing slots whose position passes the limit, decrementing the effect count.
- `retreat_effect_positions_backward`: Checks a global counter, if not zero, saves registers, walks the fourteen-byte effect slots, subtracting thirty-two from each active slot's position field and freeing slots whose position falls below the limit, decrementing the effect count.
- `fall_through_entry_22`: Entry point part-way through straight-line code: runs three instructions (ori.b, ori.b, ori.b) and then falls through into the code that follows, with no return of its own.
- `reset_sound_channel_state_block`: Writes zero to three bytes and a longword, zero to two words, then sets two words to minus one and one word to six in the sound state block, and returns.
- `reset_two_sound_channel_pairs`: Writes six and minus one to two pairs of words in the sound state block, setting two channel periods and volumes, then returns.
- `increment_score_display_counter`: Loads a word from a global variable, increments it, clamps it to a maximum of 6, stores it back, sets another global variable to all ones, and returns.
- `clear_three_score_entry_slots`: Saves registers, optionally clears a four-byte buffer, then for three slots blanks the six-byte text field, copies a digit byte into it, calls the text routine, zeroes the slot and advances by fourteen bytes.
- `update_bcd_score_and_split_digits`: If a certain global word is zero, it saves registers, adds a 32-bit value from a register to a global variable using decimal-adjusted addition, and then splits the resulting nibble-digits of that value into a table in memory before setting the global word to all ones; otherwise, it sets a global word to all ones and returns.
- `fall_through_entry_23`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `run_three_delay_loops_by_counter`: Loads three separate word counters from fixed memory, and for each counter repeatedly calls the delay routine with different immediate values until the counter expires, then returns.
- `toggle_flag_and_run_four_step_sequence`: Clears a 16-bit word in a global variable and sets it to all ones if a specific bit in a status byte is set, then if a certain global word is zero, executes a sequence of four subroutines (each potentially followed by a carry check) and finally stores all ones to a global word.
- `draw_sprite_via_7afa`: Saves a0-a1, calls the setup routine, loads a1 from a global pointer and a0 from a fixed table, then calls the sprite blitter and the wait routine before restoring a0-a1 and returning.
- `process_string_list`: Iterates through a list of pointers, and if a pointer or a value at a pointer-offset is negative, it stops; otherwise, it copies a byte from a source buffer to a destination buffer and increments the source and destination pointers.
- `menu_cursor_input_loop`: Initialises menu selection words, checks a flag and a count, draws the menu, then polls joystick bits to move the cursor up or down and stores the chosen index before returning.
- `draw_status_bar_if_active`: Returns immediately when the display flag is zero; otherwise saves registers, loads dimensions twelve by twenty-one and a graphic pointer, selects an alternate pointer when the second flag is unable to be used and calls the blitter routine.
- `init_sound_and_set_mode_two`: Returns early if the word at 4bd34 is nonzero; otherwise loads d0 with 9 and d1 with 0, calls the sound setup routine, then calls the mode routine with d0 set to 2 and returns.
- `set_mode_one_if_idle`: Tests the word at a global variable and returns immediately when it is nonzero; otherwise loads d0 with 1, calls the mode routine and returns.
- `init_level_and_wait_for_input`: Calls setup routines, sets four word variables to minus one, draws a screen region, then polls the input flag and a hardware bit until a key is pressed before calling the exit routine.
- `poll_input_until_key_or_timeout`: Saves d0, d1 and d7, loops up to 175 times calling the input routine and checking the input flag, clearing a word variable, and returns carry set on keypress or clear on timeout.
- `toggle_screen_buffer_on_key`: If the byte at the input location equals 64, calls the setup routine and switches the display pointer between two screen buffers before calling the restore routine.
- `flash_screen_rows_decrement_increment`: Saves d0-d1/a0-a1, calls four subroutines, decrements each byte of eight rows of ten bytes at the screen buffer, calls two more subroutines, then increments the same bytes back and restores registers.
- `fall_through_entry_24`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_25`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_26`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `update_high_score_table_and_input_name`: Compares a new score against a table of existing scores, shifts existing entries down if a new high score is detected, and then executes a loop that reads keyboard input to update a name buffer and redraws the table or name entry area.
- `redraw_cursor_and_name_buffer`: Converts the grid cursor position d2/d3 and the name-entry index d7 into screen coordinates and calls the draw routine three times to redraw the cursor, the name buffer and the current character.
- `fall_through_entry_27`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `run_level_script_dispatch_loop`: Checks a global flag and returns immediately if set; otherwise, it clears several global words, initializes a set of global pointers/counters, and then enters a loop that reads a script index from a table, calculates a function pointer, and calls that routine until a zero-value or a stop-flag-related condition is met.
- `initialise_script_entry_record`: Calls a setup routine, then writes a type word of one and copies two words from the pointed record into the a2 structure, zeroing its remaining fields.
- `clear_word_at_pointer`: Stores a zero word into a location pointed to by a register.
- `init_object_velocity_and_advance_source`: Reads a word from offset two of a0 into d2, calls the shared setup routine, clears the long at offset twenty-two of a2, copies two words from a0 into a2 offsets ten and twelve, then advances a0 by eight.
- `set_object_pointer_from_offset_table`: Reads a word from offset two of a0 into d2, calls the shared setup routine, adds the word at offset four of a0 to the base at 55400, stores the result at offset thirty of a2, clears the word at offset thirty-four and advances a0 by six.
- `advance_a0_by_script_entry_size`: Tests the word at offset 2 of a0; if zero it calls the routine at a fixed address then advances a0 by 4, otherwise it indexes the table at a fixed address by that word and jumps to the routine at 36982 before advancing a0 by 16.
- `search_for_sentinel_byte`: Iterates through a memory block starting from an offset until a negative byte value is encountered.
- `increment_pointer_offset`: Increments the pointer in a0 by 4 and returns.
- `update_entity_positions_and_effects`: Loops over a list of entity records, updating their x and y coordinates using either a direct velocity or a called routine, then executes conditional sound and effect updates based on a game mode or a counter in a global variable, and finally decrements a loop counter or returns if the counter reaches zero.
- `update_structure_offset`: Stores a calculated long value and two zero words into a structure at different offsets than another routine.
- `increment_pointer_offset_large`: Increments the pointer in a0 by 16 and returns.
- `index_58_byte_record_table`: Multiplies d2 by 88, loads the base address of a table into a2, and adds the product to a2 before returning.
- `initialize_global_values`: Stores two specific longword values into two consecutive global variables and returns.
- `update_global_values`: Swaps two 32-bit values from global variables, performs a bitwise rotation and subtraction on one, and stores the results back into the original global variables.
- `fall_through_entry_28`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `fall_through_entry_29`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `copy_three_buffers_twenty_six_times`: Saves d0-d2 and a0-a4, then calls a copy routine on three buffer pairs once, then repeats it twenty-six times using d2 as the counter before restoring registers and returning.
- `blit_tile_column_pair_to_bitplanes`: Reads two tile indices from a stream in a0, looks up tile data in a table, and copies 40 bytes per tile into interleaved bitplanes and a mask plane at a1 and a2, repeating for up to sixteen rows or until an end-marker byte is encountered.
- `scroll_playfield_rows_by_offset`: Reads the vertical scroll offset, computes wrapped source and destination pointers into the playfield buffer at a1, and calls the blitter copy routine to shift rows of the screen buffer.
- `blit_four_bitplane_block_with_mask`: Returns immediately if d0 is zero; otherwise, it programs the blitter control and certain registers, then performs up to four blit operations using 8192-byte offsets and waits for the blitter busy bit between each.
- `blit_tile_row_offset`: Saves registers, computes a tile source offset from the word at the tile index variable, subtracts 8, scales it, sets a0/a1/a2 to tile, bitmap and mask bases, then calls the blitter routine.
- `blit_tile_row_offset_2`: Saves registers, computes a tile source offset from the tile index variable, adds 208, scales it, sets a0/a1/a2 to tile, bitmap and mask bases, then calls the blitter routine.
- `render_level_tilemap_to_bitplanes`: Clears a tile index variable, then copies 24 rows of 32 tile bytes from the tile table into four bitplanes and a mask buffer, honoring the scroll offset in the scroll variable.
- `run_forward_animation_sequence`: Calls setup routines, clears and sets several global words, then loops sixteen times stepping a0 backward by two bytes while calling two subroutines, ending with a final call.
- `draw_two_shifted_sprite_frames`: Saves a0-a2, loads two pointers from globals, offsets each by 324 bytes, calls two drawing routines with a2 advanced by two, then restores registers and returns.
- `run_backward_animation_sequence`: Calls setup routines, clears and sets several global words, then loops sixteen times stepping a0 forward by two bytes while calling two subroutines, ending with a final call.
- `prepare_and_execute_blits`: Saves a0-a2, loads two pointers from global memory, applies offsets of 326 and 324 to them, calls a routine to perform a wide blit, then calls a routine to perform a narrow blit, and restores a0-a2 before returning.
- `blit_four_wide_blocks`: Sets both blit modulo registers to 10, then four times waits for blitter idle and starts a 12303-sized blit advancing source and destination by 8192 bytes each.
- `blit_four_narrow_blocks`: Sets both blit modulo registers to 38, then four times waits for blitter idle and starts a 12289-sized blit advancing source and destination by 8192 bytes each.
- `blit_four_8k_planes_to_screen`: Saves a0/a2, sets blitter control and modulo registers, then four times waits for blitter idle and copies 12288-sized blit advancing source and destination by 8192 bytes each.
- `draw_tile_block_from_tilemap`: Computes a destination bitmap address and a source tilemap address from a descriptor in a0, then uses nested loops to copy 32-byte chunks of tile graphics (distributed across four bitplanes) from a source table into the destination bitmap.
- `fall_through_entry_30`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `update_sprite_animation_tiles`: Iterates through a list of eight sprite or tile descriptors, updates an animation frame index in each descriptor, and copies 40-byte tiles from chip memory into interleaved bitplanes and a mask plane at the pointers stored in the descriptors.
- `copy_two_longwords`: Copies a longword from one global variable to another, then copies another longword from a different global variable to a third global variable, then returns.
- `disable_dma_interrupts_and_clear_buffer`: Writes all bits to DMACON and INTENA to disable DMA and interrupts, then zeroes a large block of memory starting at the buffer address before returning.
- `start_copper_list`: Points the copper at a new copper list by writing its address to COP1LC and strobing COPJMP1 so it starts at once, then returns.
- `initialize_display_registers`: Writes constant values to the bitplane modulo, control, data-fetch and display-window hardware registers to configure the display.
- `set_boot_pointers`: Stores a longword containing a specific address into a global variable and writes a word containing the value two into another global variable, then returns.
- `update_input_and_sound`: Saves registers, increments a global word, calls a routine, calls another routine, restores registers, and sets a hardware register to a constant value.
- `tiny_rte`: Small routine of 1 instruction that returns from the exception.
- `enable_dma_and_interrupts_set_supervisor`: Writes the DMA and interrupt enable masks to the custom chip registers, sets the supervisor bit in the status register and clears all interrupt requests before returning.
- `initialize_blitter_registers`: Writes constant values to the blitter control and first/last word mask registers and enables DMA channels via the DMA control register.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `call_with_saved_a0`: Pushes a0 to the stack, loads a specific address into a0, calls another routine, restores a0 from the stack and returns.
- `copy_16_colors_shifted_to_color_registers`: Saves d0-d1/a1, stores a0 to a global pointer, then copies 16 words from (a0) into COLOR00 registers, each value shifted left by one, and restores registers.
- `fade_palette_up_by_bit_planes`: Over 8 passes, if a color component exceeds a threshold, it increments a pointer and updates a color buffer, then writes doubled values to the color registers.
- `fade_palette_down_by_bit_planes`: Copies 16 colors to a buffer, then over 8 passes, if a color component is non-zero, it decrements a pointer and updates a color buffer, then writes doubled values to the color registers.
- `fill_zero_bytes`: Performs eight or-with-zero operations on d0.
- `reset_counter_if_at_end`: Saves d0-d1, loads two counters from absolute memory, and clears the first counter when the second minus one is not greater and the values match.
- `call_reset_when_counter_at_end`: Saves d0, loads a value from a memory location, compares it with another memory location minus one, calls a reset routine if the comparison fails, then restores d0 and returns.
- `toggle_six_playfield_bytes`: Flips bit 7 of six longword-addressed bytes in memory, toggling each byte's high bit, then returns to the caller.
- `run_command_list_until_negative`: Saves d0-d1 and a0-a1, walks a word list at a0 reading opcode words, and for each non-negative word reads a parameter and pointer then calls the dispatcher; restores registers and returns.
- `blit_tile_column_to_bitplanes`: Computes a destination address in chip RAM from a tile index and a row offset, then loops through a list of tile indices, copying or updating bitplane-interleaved pixel data or tile-row patterns into the bitplane buffers or a destination buffer, otherwise it returns.
- `blit_tile_column_to_bitplanes_2`: Reads tile indices from a byte stream in a0 until a 255 terminator is encountered; for each index, it copies 32 bytes of tile data from a tile table into four interleaved bitplanes at a1, advancing a1 by one byte per tile.
- `clear_both_playfield_buffers`: Saves a0, loads two playfield buffer pointers from fixed memory, calls a clear routine on each, then restores a0 and returns.
- `clear_playfield_buffer_32_rows`: Saves registers, then writes 32 rows of 32 zeroed-out longwords starting at the address in a0 using a dbra loop over d7, restoring registers and returning.
- `blit_tile_column_to_two_bitplanes`: Copies a 24-row by 16-word tile from the tile data at an offset determined by a register into two destination buffers, writing each word to both the first and second bitplane areas 8192 bytes apart; if a certain value is detected, it uses an alternative source or offset.
- `draw_masked_tile_to_four_bitplanes`: Clips a tile or a partial tile (defined by a horizontal or vertical range) against the screen boundaries, then ORs or masked-ORs the tile's bitplanes into the bitplane buffers at a certain offset from a pointer in a1, advancing 40 bytes per row.
- `draw_masked_tile_with_terrain_mask`: Blits a source tile or block of pixels to a destination, where each word or halfword is masked by a value fetched from a terrain buffer that advances 40 bytes per row and wraps after 256 rows.
- `fall_through_entry_31`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `set_cia_interrupt_and_vector`: Stores a long immediate value at an absolute address, then writes 127 and 137 to the CIA interrupt control register to acknowledge and enable interrupts, and returns.
- `handle_keyboard_interrupt`: Saves d0, reads the keyboard serial data register, and if bit 3 is set, rotates and inverts the byte before storing it in a global variable, then updates several global bytes and certain hardware registers before returning from exception; otherwise, it updates certain global bytes and hardware registers before returning from exception.
- `process_input_state`: Clears a word in a global variable, reads a byte from a global variable, and evaluates bits within hardware input registers to calculate and store new values into a routine address and a global byte.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `fall_through_entry_32`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `zero_filled_data_area`: Contains a sequence of instructions that clear d0, acting as a zero-filled data area.
- `check_global_flag`: Checks if a certain global word is non-zero and otherwise returns immediately.
- `select_and_register_sound_slot`: Saves all registers, indexes a table at 1c2f0 by d0, and either starts a pending sound via d1 or picks the least-used of four sound slots and records d0 there before calling the sound start routine.
- `update_sound_state_and_dispatch`: Saves registers, calls the sound tick routine, then depending on the pending-sound and active-sound flags either starts a queued sound with d0 and d1 or calls the sound cleanup routine.
- `reset_sound_channels_and_counters`: Saves all registers, clears four channel bytes at offsets 22, 90, 158 and 226 from a base table, calls a sound reset routine, then zeroes several word counters and restores registers.
- `update_audio_parameters`: Updates a structure containing audio or DMA parameters by calculating a new volume or frequency-related value using a multiplication-and-shift method, and updates a global pointer or counter.
- `clear_memory_block_2`: Clears a block of memory by setting a range of bytes to zero and a range of words to zero, while also initializing several global variables to zero, one, or certain constant values.
- `clear_memory_and_initialize_pointers`: Clears a byte at the memory location pointed to by a0 and then clears a series of global variables or structure members to zero or certain constant values, while also performing a loop that writes zeroed bytes into a block of memory.
- `clear_d0_register_bytes`: Clears the lower byte of register d0 eight times using or instructions.
- `init_four_player_sprite_slots`: Saves d0, calls a setup routine, writes certain values to custom chip registers, and fills four sprite slot structures with frame pointers, flags, and zeroed fields before returning.
- `parse_command_stream`: Parses a stream of command bytes, updating a pointer and a counter, while handling specific command opcodes that can modify a global pointer, update a table, or call other routines.
- `process_command_stream`: Checks a flag in a structure, then calls two other routines and enters a loop that parses a stream of command bytes.
- `execute_command_dispatcher`: Reads a byte from a source, checks if it is a valid command, and if so, updates a structure or pointer or clears a field based on the command type.
- `update_sprite_pointer_and_flags_2`: Checks a bit in a byte at an offset from a pointer, and if clear, decrements a byte in a structure, and if that byte is not zero, updates a pointer and calls an audio reset routine.
- `update_sprite_pointer_and_flags`: Checks a bit in a byte at an offset from a pointer, and if clear, decrements a byte in a structure, and if that byte is not zero, updates a pointer with a new address derived from a byte in a table and certain bit-masked or shifted values.
- `check_sprite_flag_and_advance`: Calculates an address by adding a scaled byte from a pointer to a base, checks a bit in a byte at that address, and if bit 6 is clear, sets a byte at an offset from a pointer to 1.
- `clear_sprite_flags_and_advance_pointer`: Clears a bit in a byte at an offset from a pointer, writes two bytes derived from a byte register and a byte pointed to by a pointer, advances that pointer by one and stores it back, then returns twice the value of a signed byte at another offset from the pointer.
- `update_timer_counter`: Updates a 32-bit timer value stored in a global variable using a 16-bit delta from a pointer, then decrements a signed byte counter and clears its sign bit.
- `configure_audio_channel_playback`: Checks if certain counters in a structure are active, decrements them, and if a reload counter reaches zero, programs a custom chip audio channel with a new sample pointer, period, volume, and DMA length from a sound descriptor, then installs a playback interrupt routine.
- `reset_audio_and_dma_registers`: Writes constant values to several audio-related registers and clears several other memory-mapped registers and values in a table.
- `setup_audio_slots`: Calls three different routines, each passing a pointer-relative address in a3, and sets a pointer to a memory location.
- `update_sound_slot_playback`: Checks a sound slot's active flag, decrements its counter, calls a helper and a slot callback, then sets period and volume in the audio hardware or clears the slot.
- `check_pointer_and_set_address`: Checks if a word at an offset from a pointer is positive, and if a certain value is not one, loads a PC-relative address into a0 and stores it at an offset in the structure pointed to by a3, then returns.
- `reset_audio_registers`: Loads a value from a pointer, or-sets bit 15, and writes it to a memory-mapped register, then decrements a counter in a structure and branches back to the start if not zero.
- `set_flag_and_accumulate_short`: Writes a constant word to an offset in a structure pointed to by an address register and then adds a short value from another structure pointed to by a different address register to a short field in the first structure, storing the sum back in that field.
- `update_audio_state`: Checks a counter, and if it is not one, decrements it and potentially writes or or-sets values to a memory-mapped register or a table, or otherwise clears a counter and writes or-set values to a table.
- `decrement_counters_and_flip_sign`: Decrements two short counters stored in a structure pointed to by an argument register and accumulates values from other fields of the same structure into a running total.
- `disable_dma_and_interrupts`: Writes to DMACON to disable DMA channels and writes to INTENA to disable interrupts, then writes to INTREQ and INTREQR to clear interrupt requests.
- `fall_through_entry_33`: Entry point part-way through straight-line code: runs one instruction (ori.b) and then falls through into the code that follows, with no return of its own.
- `decrement_and_branch_on_table`: Loads a word from a table, and if not zero, decrements a value at a pointer and branches back to the start if not zero, otherwise it clears a byte at a pointer and branches to another routine.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `call_stub`: Call stub: calls a routine at a fixed address outside this code unit, then returns to its own caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `update_pointer`: Updates a pointer in a0 by adding 4 to it and then returns immediately.
- `function_epilogue_2`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `load_and_return_stack_value`: Calls another routine and then returns the value that was previously stored on the stack.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `function_epilogue_3`: Function exit sequence shared or jumped to by a routine: restores saved registers or drops stack space, then returns to the caller.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `tiny_move_rte`: Small routine of 2 instructions that copies (a7)+ to d0, then returns from the exception.
