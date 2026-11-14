# Game 1051 (chaos engine, the (1993)(renegade)[b]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01051). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1802304 top-level bytes in 15 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (1 by rule, 1 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 1 of 1 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 156 of 156 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| code | 526 |
| data | 851506 |
| unused | 950272 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Chaos Engine, The (1993)(Renegade)[b](Disk 1 of 2).adf:bootblock | 1 | 145 | 4 |
| Chaos Engine, The (1993)(Renegade)[b](Disk 2 of 2).adf:bootblock | 1 | 10 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |

## Cases handled

- chaos engine, the (1993)(renegade)[b](disk 1 of 2)/Chaos Engine, The (1993)(Renegade)[b](Disk 1 of 2).adf: custom-format (non-DOS) disk, 1658 non-empty blocks: matched against track-loaded segments
- extended reference run ignored: it does not re-execute 1 of the 1 PCs the canonical run logged (121382 PCs logged; the harness no longer reproduces the game), so its PCs and RAM are not used
- reference run failed (Karl 4 Nov 2026): run ext slid through memory (an address: 6 64 KB pages each with >= 29491 distinct PCs executed, 98304 of its 121382 PCs), a runaway, not game code; the run is not used (re-run it: pipeline.py extend regenerates ext when its RAM is moved aside)
- runtime segment an address (play-lgpwide): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- chaos engine, the (1993)(renegade)[b](disk 1 of 2)/Chaos Engine, The (1993)(Renegade)[b](Disk 1 of 2).adf: blocks 2-7 (bytes 1024-4096) outside the filesystem hold data, not code (opcode density 0.027, entropy 2.86)
- chaos engine, the (1993)(renegade)[b](disk 1 of 2)/Chaos Engine, The (1993)(Renegade)[b](Disk 1 of 2).adf: blocks 22-22 (bytes 11264-11776) outside the filesystem hold data, not code (opcode density 0.035, entropy 2.71)
- chaos engine, the (1993)(renegade)[b](disk 1 of 2)/Chaos Engine, The (1993)(Renegade)[b](Disk 1 of 2).adf:blocks 26-867 (bytes 13312-444416)+140288#unpacked: high-entropy data file (7.87 bits/byte), classified as packed data
- chaos engine, the (1993)(renegade)[b](disk 1 of 2)/Chaos Engine, The (1993)(Renegade)[b](Disk 1 of 2).adf:blocks 26-867 (bytes 13312-444416)+355840#unpacked: high-entropy data file (7.76 bits/byte), classified as packed data
- chaos engine, the (1993)(renegade)[b](disk 1 of 2)/Chaos Engine, The (1993)(Renegade)[b](Disk 1 of 2).adf: blocks 26-867 (bytes 13312-444416) hold 9 packed streams (RNC1: Rob Northen RNC1 Compressor, RNC2: Rob Northen RNC2 Compressor) found by scanning, 99% of the bytes
- chaos engine, the (1993)(renegade)[b](disk 1 of 2)/Chaos Engine, The (1993)(Renegade)[b](Disk 1 of 2).adf: blocks 881-1689 (bytes 451072-865280) hold 3 packed streams (RNC1: Rob Northen RNC1 Compressor, RNC2: Rob Northen RNC2 Compressor) found by scanning, 99% of the bytes

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Chaos Engine, The (1993)(Renegade)[b](Disk 1 of 2).adf:bootblock

- `relocate_and_start_loader`: Boot-block entry that walks a list of memory-region records to pick a load mode, sets up sprite and interrupt registers through exec calls, copies its own body into low memory and jumps there.
- `reset_hardware_and_relaunch`: Boot-level routine that allocates a block through exec, disables interrupts and DMA, clears the sprite and colour registers and copies a short body to low memory before jumping to it.
- `copy_memory_block_and_dispatch`: Copies a block of memory word by word from source to destination, copying backward when the regions overlap, then records the new free position and jumps through a handler pointer stored in the structure.
- `copy_longs_then_invoke_callback`: Copies a run of longwords from a source to a destination, writes a terminating zero at the current offset in the structure and calls the function pointer stored there.

### Chaos Engine, The (1993)(Renegade)[b](Disk 2 of 2).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `branch_to_offset_eight`: Performs a short relative branch to an instruction eight bytes ahead.
