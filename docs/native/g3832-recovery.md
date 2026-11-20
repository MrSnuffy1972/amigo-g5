# Game 3832 (Goldrunner2 v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03832). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1335682 top-level bytes in 10 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 2 raw-word lines |
| Functions named and described | pass, 13 of 13 named (0 author symbols), 13 of 13 described (4 by rule, 7 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 27904 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 141 of 141 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 10601 |
| code | 594 |
| data | 1307564 |
| padding | 27 |
| unused | 16896 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 141 | 13 |
| Disk.1 bytes 0-512 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 1024-28416 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/Disk.1: raw disk image (112640 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.2: raw disk image (131072 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: raw disk image (340992 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Goldrunner2_v1.0/Goldrunner2_v1.0.hdf: hard-disk image (AmigaDOS OFS, 1440 blocks): its filesystem parses and all 8 files in it are byte-identical to the extracted tree Goldrunner2_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/Disk.1: bytes 0-512 look like code no run loaded: recovered position-independently (load address unknown)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/Disk.1: bytes 1024-28416 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 1 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 1 immediate
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/Disk.1: 2 logged loads cover 27904 bytes of the image
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/Disk.1: chunks 113-439 (bytes 28928-112640) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.92)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.2: chunks 0-124 (bytes 0-32000) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.53)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.2: chunks 128-157 (bytes 32768-40448) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.91)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.2: chunks 160-362 (bytes 40960-92928) outside the filesystem hold data, not code (opcode density 0.009, entropy 4.46)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.2: chunks 368-493 (bytes 94208-126464) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.13)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.2: chunks 496-511 (bytes 126976-131072) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 0-13 (bytes 0-3584) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.57)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 16-31 (bytes 4096-8192) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.78)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 34-49 (bytes 8704-12800) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.66)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 52-67 (bytes 13312-17408) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.65)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 70-85 (bytes 17920-22016) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.63)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 88-103 (bytes 22528-26624) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.21)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 106-121 (bytes 27136-31232) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.74)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 124-139 (bytes 31744-35840) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.71)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 142-157 (bytes 36352-40448) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.81)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 160-175 (bytes 40960-45056) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.72)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 178-193 (bytes 45568-49664) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.76)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 196-211 (bytes 50176-54272) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.04)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 214-229 (bytes 54784-58880) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.10)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 232-247 (bytes 59392-63488) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.68)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 250-265 (bytes 64000-68096) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.81)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 268-283 (bytes 68608-72704) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.65)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 286-1156 (bytes 73216-296192) outside the filesystem hold data, not code (opcode density 0.005, entropy 5.86)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 1168-1322 (bytes 299008-338688) outside the filesystem hold data, not code (opcode density 0.005, entropy 3.84)
- Goldrunner2_v1.0/Goldrunner2_v1.0_hdf/Goldrunner2/disk.3: chunks 1330-1331 (bytes 340480-340992) outside the filesystem hold data, not code (opcode density 0.000, entropy -0.00)
- 2 never-loaded region(s), 27904 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `setup_jump_table_entries`: Initializes jump table entries in memory by writing jump instructions and their corresponding addresses, then calls a routine to set up additional parameters.
- `setup_render_call`: Saves all general-purpose and address registers on the stack, sets up parameters for a graphics or rendering call, and then restores the registers before returning.
- `initialize_hardware_registers`: Writes fixed 16-bit values to several hardware register locations and sets up jump table entries for later execution.
- `load_vhposr_value`: Reads the vertical position register value and stores it in a global variable, then performs a conditional jump based on the result.
- `frag_cmpi_bne_move_pea_rts`: Short code fragment that compares a fixed memory address with an address, then branches if the last result was not zero, then copies 20153 to a fixed memory address, then pushes the address of a location in the code onto the stack, then copies the top of the stack to a fixed memory address, then returns.
- `copy_word_to_masked_addresses`: Reads a 16-bit value from one memory location and writes it to two different memory locations after applying a bitmask operation to their addresses.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
- `frag_movem_lea_movea_jsr_rts`: Short code fragment that saves registers on the stack, then loads the address of a location in the code into a0, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `conditional_render_setup`: Calls another routine, checks a condition, and if the condition is met, prepares parameters for a rendering operation before returning.
- `setup_and_execute_render`: Prepares for a rendering operation by setting up source and destination pointers, configures parameters, and calls a graphics routine with specific values.
