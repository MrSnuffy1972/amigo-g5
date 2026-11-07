# Game 1185 (gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01185). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (906032 top-level bytes in 65 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 20 of 20 units identical (bytes and relocations), 11 raw-word lines |
| Functions named and described | pass, 115 of 115 named (0 author symbols), 115 of 115 described (5 by rule, 42 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1226 of 1226 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 244590 bytes, 52 functions |
| Independent decoder cross-check (Ghidra against capstone) | 3534 of 3592 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 159106 |
| code | 12746 |
| data | 595093 |
| padding | 16207 |
| unused | 122880 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:tr | 1 | 89 | 3 |
| setmap | 2 | 229 | 8 |
| Off | 7 | 29 | 2 |
| x | 6 | 34 | 3 |
| cls | 2 | 20 | 1 |
| border | 2 | 233 | 8 |
| Disk-Validator | 1 | 386 | 1 |
| Astro | 1 | 13 | 1 |
| type | 2 | 117 | 2 |
| boot | 2 | 15 | 2 |
| intro | 1 | 76 | 5 |
| Astro | 1 | 12 | 1 |
| Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1380 | 25 |
| Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:MUS2.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:MUS1.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:GAUNTLET.COD (raw code file, load address unknown) | 1 | 45 | 1 |
| Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:BOOT (raw code file, load address unknown) | 1 | 386 | 24 |
| Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:MUS0.DAT (raw code file, load address unknown) | 1 | 0 | 0 |
| Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:COMBOOT (raw code file, load address unknown) | 1 | 518 | 27 |

## Cases handled

- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf: file COMSPEC (header 4000 bytes, blocks hold 0): its blocks are judged as raw blocks outside the filesystem
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP16.COM: high-entropy data file (7.89 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP06.COM: high-entropy data file (7.88 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP21.COM: high-entropy data file (7.75 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP04.COM: high-entropy data file (7.84 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:PANF.COM: high-entropy data file (7.84 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP12.COM: high-entropy data file (7.83 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:L1AZ.COM: high-entropy data file (7.92 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP10.COM: high-entropy data file (7.85 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP01.COM: high-entropy data file (7.79 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP18.COM: high-entropy data file (7.90 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP09.COM: high-entropy data file (7.83 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:DOR0.COM: high-entropy data file (7.62 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP07.COM: high-entropy data file (7.91 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP15.COM: high-entropy data file (7.87 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP22.COM: high-entropy data file (7.72 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP13.COM: high-entropy data file (7.82 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP03.COM: high-entropy data file (7.87 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP19.COM: high-entropy data file (7.85 bits/byte), classified as packed data
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:SP00.COM: high-entropy data file (7.84 bits/byte), classified as packed data
- runtime segment an address: 1137 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:MUS2.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:MUS1.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:GAUNTLET.COD: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:BOOT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:MUS0.DAT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:COMBOOT: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- gauntlet iii (1991)(u. s. gold)[cr skr](disk 1 of 3)[h astronut - wise man][t metallians +3]/Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf: blocks 2-10 (bytes 1024-5632) outside the filesystem hold data, not code (opcode density 0.043, entropy 3.44)
- 4 never-loaded region(s), 239666 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- intro

## Copy protection found

- `examine_undecompiled_hardware_region` (GAUNTLET.COD (raw code file, load address unknown)): The decompiler timed out on this raw region; the facts show writes to interrupt, DMA, copper and CIA registers, but the body is not visible. Kind: disk hardware/integrity check. Pass path: The branch not taken by the error/failure condition (typically the instruction immediately following the call if the routine returns a success flag in D0 or A0, or the continuation of the main loading loop)..
- `setup_disk_read_registers` (BOOT (raw code file, load address unknown)): Writes fixed values into the disk pointer, sync, length and control registers, and sets a global status to two. Kind: disk hardware register initialization/sync check. Pass path: the code following the call (the instructions provided in the hunk).
- `step_drive_head_to_track` (BOOT (raw code file, load address unknown)): Rejects targets above seventy-nine, then steps the drive head one track at a time toward the requested position by changing the drive control bits, returning zero on success. Kind: track/head movement validation. Pass path: bmi.w an address.
- `mask_control_byte_by_index` (BOOT (raw code file, load address unknown)): Combines an input byte with a constant mask and clears a bit selected by a counter, then stores the result in a global. Kind: disk hardware/control byte check. Pass path: the instruction following the btst.b #an address, an address.l (the branch taken if bit 4 is set).
- `update_drive_control_bits` (BOOT (raw code file, load address unknown)): Clears one bit selected by a counter from a drive control byte, then sets or keeps flag bits depending on whether the counter is zero. Kind: disk hardware/drive configuration check. Pass path: beq.w an address.
- `clear_global_mask_and_spin` (BOOT (raw code file, load address unknown)): Clears one bit selected by a counter in a drive control byte, spins through a countdown loop, and returns zero. Kind: checksum/integrity trap. Pass path: the code continues to the next instruction after the call (bsr.w an address, an address, or an address).
- `configure_dma_and_disk` (COMBOOT (raw code file, load address unknown)): Sets several values in memory and writes to disk and DMA control registers. Kind: disk hardware/sync validation. Pass path: the code following the call (sub_04BC) which performs bit manipulation on d0, likely preparing a status check or command register.
- `iterate_and_modify_global_variable` (COMBOOT (raw code file, load address unknown)): Checks if a global variable is within a specific range and then enters a loop that performs bitwise masking and bit setting on that variable while calling another routine. Kind: disk hardware/track sync check. Pass path: the loop completes and the function returns 0.
- `mask_global_variable` (COMBOOT (raw code file, load address unknown)): The routine reads a byte from the input register, combines it with a constant mask and clears a bit selected by a value taken from an internal table, then writes the result to a global variable. Kind: disk hardware/track validation check. Pass path: btst.b #an address, an address.l (bit 2 of the status register/hardware register being checked).
- `modify_global_variable_with_mask` (COMBOOT (raw code file, load address unknown)): Calculates a bitmask based on a global variable and performs bitwise operations to update that global variable. Kind: disk hardware/track validation check. Pass path: tst.l d0; beq.w an address.
- `modify_bit_mask` (COMBOOT (raw code file, load address unknown)): Calculates a bitmask using a global variable and performs a loop that decrements a local counter. Kind: checksum/bitmask trap. Pass path: sub_002E continues execution (the bsr calls are likely subsequent initialization or data loading steps).

## Functions


### Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:tr

- `decode_bitstream_back_references`: Reads a bit stream in which each flag selects either a literal copy or a length-and-offset copy from earlier output, writing the decoded bytes into a buffer.
- `shift_bit_reader_word`: Stores the low byte of the longword just before the input pointer into a destination with a fixed high byte, and returns that longword shifted right by one with its top bit set, as one bit-reader step.
- `consume_bits_with_refill`: Shifts a bit buffer right by a count, fetching a fresh word from a helper whenever the buffer empties during the shift.

### setmap

- `load_hunk_executable_image`: Parses AmigaDOS hunk records from a loaded file, allocates memory for each section with the system allocator, and applies the relocation records to the sections.
- `spin_delay_count`: Spins in a countdown loop on a 16-bit count until it passes zero, providing a timed delay.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_bitstream_runs_backward`: Decodes a bit-coded stream of literal bytes and run-length tokens, writing the output backward from a destination pointer until the start of the buffer is reached.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `rotate_bits_through_extend`: Loops a count of times shifting a bit buffer through the extend flag, refilling the buffer from the input stream when it runs empty.
- `trigger_software_interrupt`: Executes a software interrupt trap.

### Off

- `free_blocks_and_clear_field`: Runs a short loop that releases memory blocks through the system FreeMem entry, then zeroes a longword field in the caller's structure.
- `decrement_word_until_negative`: Decrements a 16-bit counter in a register until it reaches minus one, then returns.

### x

- `execute_system_initialization`: Calls several functions via pointers stored in a global structure, including a call to forbid the system.
- `permit_and_release_resources`: Releases memory blocks through the system FreeMem entry in a loop, writes a fixed value, and ends with a Permit call that re-enables multitasking.
- `decrement_until_minus_one`: Decrements a local variable until it reaches negative one and then returns.

### cls

- `open_dos_library`: Opens the DOS library through a function pointer stored in a global and calls a library entry on the result, returning the outcome of a second call or a fixed code of twenty when a flag is set.

### border

- `relocate_hunk_sections`: Walks AmigaDOS hunk records, allocating section memory and patching relocation entries, and reads a field from a process-related structure to choose the starting block.
- `decrement_register_to_zero_2`: Decrements a 16-bit value in a register until it reaches zero.
- `stub_empty_return`: Does nothing and returns immediately; it is a stub with no body.
- `shift_bit_buffer`: Shifts a bit buffer through the extend flag and performs memory copies using variable offsets and counts.
- `check_shifted_flag_2`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `rotate_bits_through_extension`: Performs a bitwise rotation on a sequence of data elements by shifting bits across word boundaries.
- `rotate_bits_through_buffer`: Shifts bits from a source buffer into a destination buffer using the extend flag, refilling the source from a stream when empty.
- `trigger_software_interrupt_2`: Executes a software interrupt trap.

### Disk-Validator

- `write_counter_digits_to_text`: Writes a single character and a digit derived from a count into a text buffer at offsets computed from a loop index, then sets up counters for a following loop.

### Astro

- `initialize_program_execution`: This is the initial entry point of the executable.

### type

- `find_max_and_prepare_buffer`: The routine walks a linked list of entries to find the maximum value stored in a field and then, if a condition is met, writes a small header, fills a buffer with sequential words, and sets up a stack frame for a call.
- `copy_fifteen_longs_then_call`: Copies fifteen longwords from a fixed source block into a destination when one is supplied, then calls a routine through a pointer.

### boot

- `enter_supervisor_copy_and_jump`: Enters supervisor state, copies a block of code into a fixed low-memory address, and jumps to a routine selected through an indirect table.
- `execute_short_routine`: A four-instruction routine called by a hunk loader.

### intro

- `select_indirect_table_entry`: Calls two helpers, looks up a library entry, stores a fixed value in a table slot and then calls through an indirect table.
- `process_bitstream_references`: performs bitstream decompression by copying or shifting data from a source buffer to a destination based on control bits.
- `test_upper_bits_no_effect`: Tests whether a register has bits above its lowest bit and returns in both cases, with no observable effect.
- `tiny_subq`: Small routine of 1 instruction that subtracts 1 from d0.
- `shift_bit_buffer_right_refill`: Shifts a bit buffer right by a count, refilling it from a byte stream when it empties and carrying the shifted-out bit into the top.

### Astro

- `open_dos_library_old_style`: Opens the DOS library through the older open call, invokes two library entries on the result and returns zero.

### Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `enter_supervisor_then_jump_table`: Opens the DOS library, enters supervisor state, stores a fixed value in a table slot and jumps to a routine through an indirect table.
- `poll_input_and_setup_display`: Sets up the copper, DMA and interrupt state, then polls the keyboard serial register and the mouse and button registers in a loop with a timeout counter.
- `service_flag_then_call_helpers`: When a status bit is set, writes a fixed value to a global and calls three helpers; otherwise it returns without effect.
- `copy_six_strided_bytes`: Copies six bytes from a source using one row stride into a destination using another row stride, with the start offset computed from an index.
- `start_blitter_rect_copy`: Waits for the blitter to be free, programs its control, modulo, pointer and size registers, and starts a rectangle copy, using a masked path when a condition holds.
- `clear_and_or_pattern_rows`: Clears a block of fifty-six words, ORs a fourteen-word pattern into a row chosen by a counter, advances that counter with wraparound at 144, and then ORs a second pattern.
- `fill_pattern_table_longs`: Writes fixed longword patterns into a table in five blocks and sets two offset fields in the caller's structure from a computed value.
- `store_split_long_fields`: Writes the low and high words of a register value into two word fields of the structure addressed by a base register.
- `store_word_split`: stores a word into a memory location and stores the upper bits of a register into a different location relative to the same pointer.
- `advance_bounce_counters_and_positions`: For each of three entries, advances a bouncing counter and a wrapping counter, then writes the computed positions as split word fields in a table.
- `step_nibble_fields_toward_zero`: Decrements the nibble fields of each word toward zero, raising a flag while any field remains nonzero, and then waits on a button state and blitter status.
- `fade_palette_components_toward_target`: Every fourth call, steps the red, green and blue nibbles of each palette entry one unit toward a target entry, leaving other bits unchanged.
- `write_stepped_index_bytes`: Writes five consecutive byte values taken from a counter at a fixed row stride, and adjusts two boundary bytes from the same counter.
- `copy_table_rows_with_wrap`: Copies a table of words into a buffer in five rows of forty-four words each, advancing the source offset and wrapping it at a fixed limit.
- `decode_keyboard_key_code`: Inverts the keyboard serial byte and returns it when it is not one of three reserved codes and the status bits show a pressed key; otherwise it returns a fixed idle value.
- `build_row_pointer_table`: Computes a base offset from a flag and the largest entry in a table, then fills fourteen pointer slots with offsets that grow by each entry's value.
- `update_audio_channel_registers`: Loops over three channels, advancing each channel's period-like word by an increment and writing it with its pointer fields to the audio channel registers, with a DMA control write.
- `update_four_channel_records`: Resets a state byte and walks four channel records, updating each one's period-related fields from a table while checking the fire-button register.
- `load_channel_block_into_hardware`: Copies a channel's eight longwords into hardware registers when a pointer is set, clears a status bit, and resets the channel's fields when a table entry is minus one.
- `store_channel_period_from_table`: Looks up a 16-bit value in a period table using a byte index and stores it in a field of the channel block.
- `apply_active_channel_updates`: Walks four audio channel blocks and calls a routine for each block whose active flag is set.
- `step_channel_volume_envelope`: Decrements a channel's step counter and, when it reaches zero, loads the next envelope step from a table, scales the volume by a factor and advances the envelope index with wraparound.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `build_row_table_helper`: an eight instruction routine called by a routine that builds a row pointer table.
- `build_row_table_helper_2`: a six instruction routine called by a routine that builds a row pointer table.

### Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:GAUNTLET.COD (raw code file, load address unknown)

- `examine_undecompiled_hardware_region`: The decompiler timed out on this raw region; the facts show writes to interrupt, DMA, copper and CIA registers, but the body is not visible.

### Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:BOOT (raw code file, load address unknown)

- `copy_code_block`: Copies a block of code to a high memory address and then calls a routine via a pointer from a global variable.
- `decode_bitstream_references`: an instruction sequence that serves as a dispatcher for various bitstream decoding and record copying operations.
- `copy_records_until_terminator`: Walks a table of records until a minus-one marker, copying record data into an output buffer for each entry that passes a status check, with a countdown global controlling the passes.
- `verify_table_checksum`: Negates and sums 128 longwords from a fixed table and returns minus one when the total differs from a stored reference value, zero otherwise.
- `copy_nonzero_entries_until_end`: Repeatedly calls a routine, then copies nonzero longwords from a record's table into an output list in reverse order while a chained value stays nonzero, and writes a minus-one terminator.
- `find_case_insensitive_name_entry`: Scans a table of six-byte entries for one whose four-character key matches a fixed text ignoring case, storing the entry's word and returning zero on a match or minus one at the end of the table.
- `build_tagged_record_list`: When a flag is clear, walks a table until a terminator, writing each entry as a record with a three-character tag copied from fixed text and a value returned by a helper.
- `copy_nonzero_words_into_list`: Once, copies the nonzero words from a source table into an output list, skipping zeros, and ends the list with minus one.
- `copy_longword_block_after_check`: When not already done and a helper check passes, copies 128 longwords from a source pointer to a destination pointer.
- `update_global_status`: Stores a status value returned by a helper in a global and, when that value is non-negative, updates the global again from a second helper result if it is nonzero.
- `split_byte_by_twenty_two`: Stores the input word, splits the input byte by twenty-two into a quotient and a remainder, subtracts eleven from remainders above ten, and records whether that adjustment was made.
- `setup_disk_read_registers`: Writes fixed values into the disk pointer, sync, length and control registers, and sets a global status to two.
- `wait_for_disk_read_completion`: Writes a fixed value to the disk length register, then waits on a flag with a countdown timeout before clearing a global and returning a status value.
- `sequence_two_helper_calls`: Calls two helper routines in sequence and returns.
- `scan_table_until_routine_succeeds`: Repeatedly finds the next record whose first word matches a counter, calls a routine with it and increments the counter, stopping when that routine returns one and then returning zero.
- `interleave_bits_from_two_tables`: Reads 128 values from a source table and combines each with the value 128 positions ahead using even-bit masks, writing one combined longword per iteration.
- `pack_bit_interleave_table_entries`: Calls a helper, then for ten table entries interleaves the bits of two longword fields and writes the high and low bytes of the low sixteen bits into a per-entry table, advancing by sixteen bytes each time.
- `locate_sync_words_in_buffer`: Scans a data buffer for a fixed sync word and records up to ten positions in a table, backing up one word when the match is not exact; it stops at the end of the buffer.
- `step_drive_head_to_track`: Rejects targets above seventy-nine, then steps the drive head one track at a time toward the requested position by changing the drive control bits, returning zero on success.
- `poll_input_and_clear_flag`: Repeatedly calls two helpers while a status bit is set and a count is nonzero, then clears the global flag.
- `spin_delay_count_2`: Spins decrementing a count in a register until it reaches zero.
- `mask_control_byte_by_index`: Combines an input byte with a constant mask and clears a bit selected by a counter, then stores the result in a global.
- `update_drive_control_bits`: Clears one bit selected by a counter from a drive control byte, then sets or keeps flag bits depending on whether the counter is zero.
- `clear_global_mask_and_spin`: Clears one bit selected by a counter in a drive control byte, spins through a countdown loop, and returns zero.

### Gauntlet III (1991)(U. S. Gold)[cr SKR](Disk 1 of 3)[h Astronut - Wise Man][t Metallians +3].adf:COMBOOT (raw code file, load address unknown)

- `initialize_system_environment`: Sets a value in a memory location, calls an unnamed routine, and copies a block of code to a new destination.
- `update_hardware_and_copy_data`: Modifies CIAA_PRA, COLOR00, COP1LCH, and COP2LCH registers while executing various data movement and comparison routines.
- `copy_table_entries`: Iterates through a table and copies a specified number of bytes from one memory location to another.
- `verify_table_checksum_2`: negates and sums 128 longwords from a fixed table, then returns zero when the total equals a stored reference value and minus one otherwise.
- `copy_nonzero_entries_until_end_2`: Repeatedly calls a routine, then copies nonzero longwords from a record's table into an output list in reverse order while a chained value stays nonzero, and finally writes a minus-one terminator.
- `compare_case_insensitive_strings`: iterates through a table of records to find a case-insensitive match between two strings, returning zero on success.
- `populate_data_list`: Iterates through a table to copy values into a destination list and performs conditional sign inversion on a set of bytes.
- `copy_nonzero_words_into_list_2`: once, copies the nonzero words from a source table into an output list, skipping zeros, and ends the list with minus one.
- `copy_longword_block_after_check_2`: When not already done and a status check after a routine call passes, copies 128 longwords from one pointer to another.
- `update_global_status_2`: Calls another routine to check a value and updates a global variable if the returned value is non-negative.
- `decode_input_bytes`: the routine stores the input word in a global variable, divides the input byte by 22 and stores the quotient in a global variable, stores the remainder of that division in a global variable, subtracts 11 from the remainder if it is greater than 10, and stores a boolean flag indicating the remainder was greater than 10 in a global variable.
- `configure_dma_and_disk`: Sets several values in memory and writes to disk and DMA control registers.
- `wait_for_disk_status`: Polls a bit in a global variable within a loop and writes a constant value to a memory location.
- `execute_empty_routine`: The routine is empty and performs no operations before returning.
- `scan_table_until_routine_succeeds_2`: Repeatedly finds the next record in a fixed table whose first word matches a counter, calls a routine with it, and increments the counter.
- `interleave_bytes_from_two_tables`: Reads 128 bytes from a source table via a pointer, interleaves each byte with the byte 128 positions ahead in the same table, and writes the combined 128 bytes to a destination table via a second pointer.
- `pack_bit_interleave_table_entries_2`: Calls a routine, then for ten table entries combines two longword fields by interleaving their bits.
- `locate_sync_words_in_buffer_2`: Scans a data buffer for the sync word and records up to ten positions in a table, backing up one word when the match is not exact.
- `iterate_and_modify_global_variable`: Checks if a global variable is within a specific range and then enters a loop that performs bitwise masking and bit setting on that variable while calling another routine.
- `poll_input_and_clear_flag_2`: The routine checks a global variable and, if a condition involving it holds, repeatedly calls another routine twice while testing the variable and a short value in D1.
- `decrement_register_to_zero`: Decrements a value in a register until it reaches zero.
- `mask_global_variable`: The routine reads a byte from the input register, combines it with a constant mask and clears a bit selected by a value taken from an internal table, then writes the result to a global variable.
- `modify_global_variable_with_mask`: Calculates a bitmask based on a global variable and performs bitwise operations to update that global variable.
- `modify_bit_mask`: Calculates a bitmask using a global variable and performs a loop that decrements a local counter.
- `assemble_long_from_four_bytes`: Calls a helper four times, shifting each returned byte into a longword global, so the global ends up holding four bytes in order.
- `write_self_modifying_code`: Writes a sequence of bytes to memory using a loop and then executes the resulting code at an indirect address.
- `toggle_color_and_clear_interrupt`: Inverts a global variable and clears a CIA interrupt request before writing a value to the first color register.
