# Game 860 (alianator (1991)(p.s.)(pd)): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00860). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (901120 top-level bytes in 6 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 4 of 4 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 12 of 12 named (0 author symbols), 12 of 12 described (3 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 60 of 60 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 360085 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 621 of 866 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 232 |
| code | 3060 |
| data | 389761 |
| padding | 1187 |
| unused | 506880 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Disk-Validator | 1 | 386 | 1 |
| Alianator (1991)(P.S.)(PD).adf:Alianator | 1 | 376 | 7 |
| Alianator (1991)(P.S.)(PD).adf:bootblock | 1 | 104 | 4 |
| Alianator (1991)(P.S.)(PD).adf:mainpart (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- alianator (1991)(p.s.)(pd)/Alianator (1991)(P.S.)(PD).adf:mainpart: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- 1 never-loaded region(s), 360085 bytes, recorded as UNVERIFIED DATA (under 1% of bytes decode as instruction flows; Karl 4 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Disk-Validator

- `scan_block_bitmap_and_count`: Writes a two-digit count into a text buffer with a blank in place of a leading zero, then resets a table of slots and walks a bitmap range, counting bit-state changes.

### Alianator (1991)(P.S.)(PD).adf:Alianator

- `open_dos_and_read_mainpart`: Opens the DOS library, opens the file named mainpart on the df0 disk and reads it into memory allocated through exec.
- `tiny_ori`: Small routine of 1 instruction that merges 0 into d0.
- `set_interrupt_and_dma_registers`: Writes the interrupt enable and DMA control registers using the set-when-bit-15 convention and reads the vertical position register.
- `set_background_colour_register`: Writes the background colour register and calls one further routine.
- `forward_call_from_two_sites`: Calls one further routine and is itself called from two places.
- `jump_through_vector_entry`: Calls one further routine through a jump-table entry that passes control straight on to a fixed address.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### Alianator (1991)(P.S.)(PD).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
- `unpack_stream_using_bit_reader`: Calls the depacker's bit-refill and bit-read routines to pull variable-width fields from a packed stream, and is entered from the boot entry.
- `bk_refill_bit_buffer`: Depacker bit reader refill: fetches the previous longword of the packed stream, folds it into the checksum and shifts out its lowest bit with a marker bit entering at the top.
- `bk_read_bits`: Depacker bit reader: collects the number of bits given in D1 from the bit buffer into D2, most significant first, refilling from the packed stream whenever the buffer runs empty.
