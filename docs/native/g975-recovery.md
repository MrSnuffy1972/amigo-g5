# Game 975 (g-loc - r360 air battle (1992)(u. s. gold)[cr fsn]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00975). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (902688 top-level bytes in 24 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 9 of 9 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 29 of 29 named (0 author symbols), 29 of 29 described (5 by rule, 10 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 538 of 538 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 779454 bytes, 2 functions |
| Independent decoder cross-check (Ghidra against capstone) | 675 of 675 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 149891 |
| code | 2036 |
| data | 426509 |
| padding | 4252 |
| unused | 320000 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:PHIL | 1 | 28 | 1 |
| G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:FUSION | 2 | 267 | 8 |
| G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:PATCH | 2 | 86 | 4 |
| G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:GLOC | 1 | 15 | 3 |
| G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:bootblock | 1 | 11 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 266 | 10 |
| G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:FORWARD.HOF (raw code file, load address unknown) | 1 | 0 | 0 |
| G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:FRONT.CMP#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |
| G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:GLOC.CMP#unpacked (raw code file, load address unknown) | 1 | 1 | 1 |

## Cases handled

- g-loc - r360 air battle (1992)(u. s. gold)[cr fsn]/G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:CHEER.HOF: high-entropy data file (7.67 bits/byte), classified as packed data
- g-loc - r360 air battle (1992)(u. s. gold)[cr fsn]/G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:FRONT.CMP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (118797 -> 371616 bytes)
- g-loc - r360 air battle (1992)(u. s. gold)[cr fsn]/G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:title32.cmp: high-entropy data file (7.94 bits/byte), classified as packed data
- g-loc - r360 air battle (1992)(u. s. gold)[cr fsn]/G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:OPT_BIT.CMP: high-entropy data file (7.83 bits/byte), classified as packed data
- g-loc - r360 air battle (1992)(u. s. gold)[cr fsn]/G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:si_bit.cmp: high-entropy data file (7.86 bits/byte), classified as packed data
- g-loc - r360 air battle (1992)(u. s. gold)[cr fsn]/G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:GLOC.CMP: RNC2: Rob Northen RNC2 Compressor-packed, unpacked with ancient (172448 -> 397874 bytes)
- runtime segment an address: 248 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- g-loc - r360 air battle (1992)(u. s. gold)[cr fsn]/G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:FORWARD.HOF: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- g-loc - r360 air battle (1992)(u. s. gold)[cr fsn]/G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:FRONT.CMP#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- g-loc - r360 air battle (1992)(u. s. gold)[cr fsn]/G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:GLOC.CMP#unpacked: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 3 never-loaded region(s), 779454 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `setup_copper_and_restore_state` (runtime segment an address): Saves several hardware registers, sets interrupt and DMA masks, points the copper at a list, waits on a joystick port line for a short count, then restores the saved DMA, interrupt and disk values and returns. Kind: disk hardware/sync check. Pass path: the branch following the loop exit (u5__l0_02B4) which restores hardware registers and returns.

## Functions


### G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:PHIL

- `patch_low_memory_vectors`: Program entry that writes a block of machine instruction words into low memory, rewrites a second block when a flag bit is clear, then sets the interrupt enable register.

### G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:FUSION

- `load_hunk_executable_relocations`: Allocates memory for each hunk of an AmigaDOS executable and walks its records, applying 32-bit relocation entries until the end-of-hunk marker.
- `delay_countdown_loop`: Runs a countdown loop as a delay; the recorded facts also list two library calls and one unnamed routine call that the shown body does not contain.
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `unpack_bit_coded_runs_backward`: Decodes a bit-coded stream into a buffer working backward from the end, emitting single bytes and back-referenced runs until the output pointer passes the start.
- `check_shifted_flag`: The routine tests whether a shifted version of a global variable is non-zero and returns early if it is; otherwise it returns.
- `tiny_sub`: Small routine of 1 instruction that subtracts d4 from d0.
- `shift_multiword_bit_register`: Shifts a multi-word bit register one bit at a time for a count in a data register, pulling a fresh longword from memory when the lower word empties and carrying the bit that falls out into the extend flag.
- `trigger_trap_exception`: Executes a software trap instruction to trigger a processor exception.

### G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:PATCH

- `unpack_bitstream_into_buffer`: Unpacks a compressed stream into memory by reading bit groups through the depacker bit reader, writing literal bytes or backward copies.
- `sweep_word_counter_minus_one`: Writes each 16-bit count value to a register in turn over one full wrap of the counter, then returns minus one.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.

### G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:GLOC

- `forward_single_call_stub`: A one-instruction program entry that calls one unnamed routine.
- `stub_undecompiled_routine`: No decompiled body was recovered for this four-instruction routine, which calls one unnamed routine and is called by two others, so nothing about its behaviour is claimed.
- `control_dma_and_interrupts`: Writes the DMA and interrupt control registers and calls one unnamed routine; the decompiled body was unavailable, so nothing further is claimed.

### G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `return_constant`: Returns the constant -1 in d0 to the caller without any other effect.
- `setup_copper_and_restore_state`: Saves several hardware registers, sets interrupt and DMA masks, points the copper at a list, waits on a joystick port line for a short count, then restores the saved DMA, interrupt and disk values and returns.
- `copy_table_with_word_stride`: Writes four control words into a block, then copies a table of 142 words into the block, stepping the destination by eight words each time.
- `fill_strided_rows_with_words`: For thirty-two rows, waits on a status bit, then stores 143 words returned by another routine into a destination at a stride of eight words, choosing one of two bases from a flag byte.
- `fill_strided_rows_with_words_2`: Iterates thirty-two times to wait for a hardware status bit, then populates a destination buffer with 143 words from another routine using a stride of eight words and a conditional base address.
- `interpolate_rgb_colour`: Blends two twelve-bit colour values channel by channel, each split into three 4-bit parts, using a signed fraction, and returns the blended colour packed back into twelve bits.
- `init_object_control_fields`: Stores a table pointer chosen by a flag byte and writes six fixed control words into the object record.
- `reset_object_and_toggle_flag`: Calls a sequence of routines, stores three pointer fields and several control words in an object record, then flips a flag byte.
- `lay_out_text_characters`: Sums a width per character from a table to measure a string, then writes a glyph pointer and position words for each character into the object record, advancing across the line.
- `zero_data_block`: Contains a series of instructions that perform logical OR operations with zero on register d0.

### G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:FRONT.CMP#unpacked (raw code file, load address unknown)

- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### G-LOC - R360 Air Battle (1992)(U. S. Gold)[cr FSN].adf:GLOC.CMP#unpacked (raw code file, load address unknown)

- `stub_undecompiled_routine_3`: A single instruction at the start of a raw code region with no decompiled body available, so nothing about its behaviour is claimed.
