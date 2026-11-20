# Game 2903 (BlackLamp v1.0 0826): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02903). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1008692 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 6 of 6 named (0 author symbols), 6 of 6 described (3 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 15104 bytes, 1 functions |
| Independent decoder cross-check (Ghidra against capstone) | 175 of 175 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 12295 |
| code | 748 |
| data | 973633 |
| unused | 22016 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 130 | 5 |
| Disk.1 bytes 19456-34560 (code never loaded by a reference run) | 1 | 45 | 1 |

## Cases handled

- BlackLamp_v1.0_0826/BlackLamp_v1.0_0826_hdf/BlackLamp/Disk.1: raw disk image (417280 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- BlackLamp_v1.0_0826/BlackLamp_v1.0_0826.hdf: hard-disk image (AmigaDOS OFS, 1088 blocks): its filesystem parses and all 8 files in it are byte-identical to the extracted tree BlackLamp_v1.0_0826_hdf, which is recovered file by file; no block outside the filesystem holds data
- BlackLamp_v1.0_0826/BlackLamp_v1.0_0826_hdf/BlackLamp/Disk.1: bytes 19456-34560 look like code no run loaded: recovered position-independently (load address unknown)
- BlackLamp_v1.0_0826/BlackLamp_v1.0_0826_hdf/BlackLamp/Disk.1: 1 logged loads cover 15104 bytes of the image
- BlackLamp_v1.0_0826/BlackLamp_v1.0_0826_hdf/BlackLamp/Disk.1: chunks 0-3 (bytes 0-1024) outside the filesystem hold data, not code (opcode density 0.016, entropy 4.11)
- BlackLamp_v1.0_0826/BlackLamp_v1.0_0826_hdf/BlackLamp/Disk.1: chunks 17-66 (bytes 4352-17152) outside the filesystem hold data, not code (opcode density 0.057, entropy 5.95)
- BlackLamp_v1.0_0826/BlackLamp_v1.0_0826_hdf/BlackLamp/Disk.1: chunks 136-136 (bytes 34816-35072) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.13)
- BlackLamp_v1.0_0826/BlackLamp_v1.0_0826_hdf/BlackLamp/Disk.1: chunks 146-270 (bytes 37376-69376) outside the filesystem hold data, not code (opcode density 0.020, entropy 6.32)
- BlackLamp_v1.0_0826/BlackLamp_v1.0_0826_hdf/BlackLamp/Disk.1: chunks 284-1398 (bytes 72704-358144) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.06)
- BlackLamp_v1.0_0826/BlackLamp_v1.0_0826_hdf/BlackLamp/Disk.1: chunks 1404-1592 (bytes 359424-407808) outside the filesystem hold data, not code (opcode density 0.003, entropy 5.43)
- BlackLamp_v1.0_0826/BlackLamp_v1.0_0826_hdf/BlackLamp/Disk.1: chunks 1606-1606 (bytes 411136-411392) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.35)
- 1 never-loaded region(s), 15104 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `process_input_data`: Loads several values from memory locations into registers, performs conditional checks on some of them, and calls another routine with specific parameters.
- `initialize_hardware`: Sets up multiple hardware registers and memory locations with specific values, including writing jump instructions to certain addresses in the custom chip registers.
- `frag_movem_lea_move_clr_movea_jsr_rts`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then copies 134 to d0, then clears d1, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_movem_lea_move_clr_movea_jsr_rts_2`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a0, then loads the address of a fixed memory address into a1, then copies 134 to d0, then clears d1, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then copies an address to a1, then returns.

### Disk.1 bytes 19456-34560 (code never loaded by a reference run)

- `clear_accumulator`: Repeatedly clears the low-order bits of a data register using ori.b instructions, then loads a large constant value into the same register and branches unconditionally to a later location in the code.
