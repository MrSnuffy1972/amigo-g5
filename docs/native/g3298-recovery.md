# Game 3298 (Cytron v1.02 0110): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/03298). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (4104512 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 3 of 3 units identical (bytes and relocations), 1 raw-word lines |
| Functions named and described | pass, 16 of 16 named (0 author symbols), 16 of 16 described (8 by rule, 8 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 256 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 160 of 160 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 7903 |
| code | 678 |
| data | 3994299 |
| unused | 101632 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 150 | 15 |
| disk.2:bootblock | 1 | 10 | 1 |
| disk.1 bytes 0-256 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Cytron_v1.02_0110/Cytron_v1.02_0110_hdf/Cytron/disk.1: raw disk image (551936 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Cytron_v1.02_0110/Cytron_v1.02_0110_hdf/Cytron/disk.2: custom-format (non-DOS) disk, 1650 non-empty blocks: matched against track-loaded segments
- Cytron_v1.02_0110/Cytron_v1.02_0110_hdf/Cytron/disk.3: raw disk image (380928 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Cytron_v1.02_0110/Cytron_v1.02_0110.hdf: hard-disk image (AmigaDOS OFS, 4416 blocks): its filesystem parses and all 7 files in it are byte-identical to the extracted tree Cytron_v1.02_0110_hdf, which is recovered file by file; no block outside the filesystem holds data
- Cytron_v1.02_0110/Cytron_v1.02_0110_hdf/Cytron/disk.1: bytes 0-256 look like code no run loaded: recovered position-independently (load address unknown)
- self-modifying operands (static SMC idiom recogniser, 12 Nov 2026): 2 instruction(s) whose operand field other code writes or takes the address of, to be lifted to variables (work/smc.json): 2 immediate
- Cytron_v1.02_0110/Cytron_v1.02_0110_hdf/Cytron/disk.1: 1 logged loads cover 256 bytes of the image
- Cytron_v1.02_0110/Cytron_v1.02_0110_hdf/Cytron/disk.1: chunks 46-2019 (bytes 11776-517120) hold a RNC1: Rob Northen RNC1 Compressor stream (505344 -> 111804 bytes)
- Cytron_v1.02_0110/Cytron_v1.02_0110_hdf/Cytron/disk.2: blocks 22-1671 (bytes 11264-856064) hold a RNC1: Rob Northen RNC1 Compressor stream (844800 -> 113600 bytes)
- Cytron_v1.02_0110/Cytron_v1.02_0110_hdf/Cytron/disk.3: chunks 0-1487 (bytes 0-380928) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.88)
- 1 never-loaded region(s), 256 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `compare_and_branch`: Calls functions with different parameters, compares results against specific values, and branches to another routine based on those comparisons.
- `setup_pointers`: Sets up pointers to several routines in memory, writes specific values to fixed memory locations, and performs conditional jumps based on a test result.
- `frag_move_jmp`: Short code fragment that copies 8316 to a fixed memory address, then copies the memory at a0 to a fixed memory address, then copies an address to a fixed memory address, then copies 24654 to a fixed memory address, then copies 9984 to the status register, then jumps to another point in the code.
- `setup_jump_table`: Writes jump table entries to fixed memory addresses and then executes a function through an indirect jump, modifying several memory locations before returning.
- `frag_pea_move_jmp`: Short code fragment that pushes the address of a location in the code onto the stack, then copies the top of the stack to a fixed memory address, then jumps to another point in the code.
- `initialize_hardware`: Initializes various hardware registers with specific values and sets up jump table entries for different routines, then jumps to a fixed address.
- `scale_coordinates`: Multiplies two 16-bit values by 512 and stores the results in registers, then calls a function through a pointer table and returns a zero value.
- `process_input_loop`: Checks if a condition is met, and if not, performs a multiplication operation on coordinates before calling a function.
- `function_epilogue`: Function exit sequence shared or jumped to by a routine: puts the result in d0, restores saved registers or drops stack space, then returns to the caller.
- `copy_byte_block`: Copies bytes from one memory location to another in a loop, using a counter and incrementing pointers, then returns.
- `fall_through_entry`: Entry point part-way through straight-line code: runs one instruction (btst.l) and then falls through into the code that follows, with no return of its own.
- `update_memory_values`: Updates specific memory locations with 16-bit values based on the input byte, handling four different cases for different memory offsets.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_pea_move_addi_rts`: Short code fragment that pushes the address of a fixed memory address onto the stack, then copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.

### disk.2:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.
