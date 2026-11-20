# Game 3216 (CosmicPirate v1.1): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03216). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2081410 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 5 of 5 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (4 by rule, 6 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 20928 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 259 of 259 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 82945 |
| code | 970 |
| data | 1997356 |
| padding | 139 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 2 | 1 |
| Disk.1:bootblock | 1 | 103 | 2 |
| logged disk load from Disk.1 boot-block load | 1 | 154 | 7 |
| logged disk load from Disk.1 boot-block load | 1 | 0 | 0 |
| OSEmu.400 (raw code file, load address unknown) | 1 | 0 | 0 |

## Cases handled

- CosmicPirate_v1.1/CosmicPirate_v1.1_hdf/CosmicPirate/Disk.1: custom-format (non-DOS) disk, 1758 non-empty blocks: matched against track-loaded segments
- CosmicPirate_v1.1/CosmicPirate_v1.1.hdf: hard-disk image (AmigaDOS OFS, 2240 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree CosmicPirate_v1.1_hdf, which is recovered file by file; no block outside the filesystem holds data
- CosmicPirate_v1.1/CosmicPirate_v1.1_hdf/CosmicPirate/Disk.1: the boot block reads 1024 bytes from disk offset 11264 to an address (found statically); recovered as a unit
- CosmicPirate_v1.1/CosmicPirate_v1.1_hdf/CosmicPirate/Disk.1: the boot block reads 71680 bytes from disk offset 15360 to an address (found statically); recovered as a unit
- CosmicPirate_v1.1/CosmicPirate_v1.1_hdf/CosmicPirate/OSEmu.400: raw code file no run loaded verbatim: recovered position-independently (load address unknown)
- CosmicPirate_v1.1/CosmicPirate_v1.1_hdf/CosmicPirate/Disk.1: 2 logged loads cover 72704 bytes of the image
- CosmicPirate_v1.1/CosmicPirate_v1.1_hdf/CosmicPirate/Disk.1: blocks 2-21 (bytes 1024-11264) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.62)
- CosmicPirate_v1.1/CosmicPirate_v1.1_hdf/CosmicPirate/Disk.1: blocks 24-29 (bytes 12288-15360) outside the filesystem hold data, not code (opcode density 0.012, entropy 5.25)
- CosmicPirate_v1.1/CosmicPirate_v1.1_hdf/CosmicPirate/Disk.1: blocks 170-1759 (bytes 87040-901120) outside the filesystem hold data, not code (opcode density 0.003, entropy 6.32)
- 1 never-loaded region(s), 20928 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.

### Disk.1:bootblock

- `initialize_coprocessor`: Initializes coprocessor registers by copying data from multiple tables, setting up interrupt handlers, and configuring hardware structures.
- `update_timer_value`: Reads a hardware interrupt status register, checks bit 5, and if set updates a global timer value by adding 10 to it.

### logged disk load from Disk.1 boot-block load

- `process_io_request`: Sets up an I/O request structure with specific values for command, buffer address, and transfer length, then executes the request.
- `process_bit_flags`: Processes a series of bit flags by shifting and masking values, updating a global array based on test conditions, and copying data between memory locations using a loop that decrements a counter until it reaches zero.
- `frag_move_cmpa_bcs_movea`: Short code fragment that copies the memory at a3 (advancing a3) to d1, then compares a3 with 59912, then branches if the carry flag is set, then copies 59400 to a3.
- `execute_io_request`: Sets up an I/O request structure with specific values for command, buffer address, and transfer length, then calls the operating system's DoIO function to execute the request, checking the result for errors.
- `frag_movea_clr_move_add_dbra_rts`: Short code fragment that copies 58376 to a0, then clears d0, then copies 127 to d1, then adds the memory at a0 (advancing a0) to d0 (2 times in a row), then loops back while counting down d1, then returns.
- `frag_bset_ori_movea_jsr_move`: Short code fragment that sets bit d1 of an indexed entry from a2, then merges 240 into d7, then copies a fixed memory address to a6, then calls another routine, then copies an address to a7, then copies 60712 to a0, then copies 17919 to d0.
- `rotate_and_xor_data`: Performs a complex bit manipulation operation on data words in memory, rotating and XORing values based on loop counters, and storing the results back to the same memory locations.
