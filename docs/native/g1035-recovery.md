# Game 1035 (stardust (1993)(bloodhouse)[cr cls - znt]): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/01035). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (2703584 top-level bytes in 4 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 6 of 6 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 8 of 8 named (0 author symbols), 8 of 8 described (1 by rule, 5 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 18 of 18 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 147 of 147 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 20257 |
| code | 522 |
| data | 2644205 |
| padding | 712 |
| unused | 37888 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 1 of 3).adf:bootblock | 1 | 71 | 2 |
| Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 2 of 3).adf:bootblock | 1 | 10 | 1 |
| Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 3 of 3).adf:bootblock | 1 | 10 | 1 |
| logged disk load from Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 1 of 3).adf boot-block load | 1 | 0 | 0 |
| logged disk load from Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 1 of 3).adf boot-block load | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 56 | 4 |

## Cases handled

- stardust (1993)(bloodhouse)[cr cls - znt](disk 1 of 3)/Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 1 of 3).adf: custom-format (non-DOS) disk, 1730 non-empty blocks: matched against track-loaded segments
- stardust (1993)(bloodhouse)[cr cls - znt](disk 2 of 3)/Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 2 of 3).adf: custom-format (non-DOS) disk, 1720 non-empty blocks: matched against track-loaded segments
- stardust (1993)(bloodhouse)[cr cls - znt](disk 3 of 3)/Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 3 of 3).adf: custom-format (non-DOS) disk, 1750 non-empty blocks: matched against track-loaded segments
- stardust (1993)(bloodhouse)[cr cls - znt](disk 1 of 3)/Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 1 of 3).adf: the boot block reads 15872 bytes from disk offset 875520 to an address (found statically); recovered as a unit
- stardust (1993)(bloodhouse)[cr cls - znt](disk 1 of 3)/Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 1 of 3).adf: the boot block reads 5632 bytes from disk offset 5632 to an address (found statically); recovered as a unit
- runtime segment an address: 18 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- stardust (1993)(bloodhouse)[cr cls - znt](disk 1 of 3)/Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 1 of 3).adf: 2 logged loads cover 21504 bytes of the image
- stardust (1993)(bloodhouse)[cr cls - znt](disk 1 of 3)/Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 1 of 3).adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.04)
- stardust (1993)(bloodhouse)[cr cls - znt](disk 2 of 3)/Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 2 of 3).adf: blocks 2-3 (bytes 1024-2048) outside the filesystem hold data, not code (opcode density 0.027, entropy 3.25)
- stardust (1993)(bloodhouse)[cr cls - znt](disk 2 of 3)/Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 2 of 3).adf: blocks 22-1739 (bytes 11264-890880) outside the filesystem hold data, not code (opcode density 0.005, entropy 6.90)
- stardust (1993)(bloodhouse)[cr cls - znt](disk 3 of 3)/Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 3 of 3).adf: blocks 2-2 (bytes 1024-1536) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.06)
- 2 packed track region(s), 1759232 bytes, recorded as PACKED DATA (UNVERIFIED): packer unknown, code recovered from the unpacked RAM image (Karl 5 Nov 2026)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 1 of 3).adf:bootblock

- `check_memory_and_launch`: Boot-block entry that issues an exec I/O request to load a block, checks the reported memory size, and on success disables interrupts and DMA, clears the video plane registers and jumps into the loaded code; otherwise it spins forever.
- `execute_indirect_branch`: calculates a new target address by adding an offset to a register and branches to it.

### Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 2 of 3).adf:bootblock

- `boot_entry`: Disk boot block code run by Kickstart at boot: finds the dos.library resident module and returns its entry so the system boots AmigaDOS from this disk.

### Stardust (1993)(Bloodhouse)[cr CLS - ZNT](Disk 3 of 3).adf:bootblock

- `check_dos_library_opened`: Calls a library function through the library vector table, after the dos library name is referenced, and returns minus one when the call gives zero, otherwise zero.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `mask_interrupt_requests`: Writes 112 to the interrupt request register twice and returns from an interrupt.
- `mask_interrupt_requests_large`: Writes 1952 to the interrupt request register twice and returns from an interrupt.
- `mask_interrupt_requests_extra_large`: Writes 6144 to the interrupt request register twice and returns from an interrupt.
- `update_memory_structure`: Checks a byte at a specific address, writes 8192 to the interrupt request register twice, and then enters a loop that processes a list of pointers to copy and manipulate data between memory locations using several nested loops.
