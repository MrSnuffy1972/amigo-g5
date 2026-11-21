# Game 2858 (Belial v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02858). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (1818644 top-level bytes in 7 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 2 of 2 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 10 of 10 named (0 author symbols), 10 of 10 described (6 by rule, 3 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 15872 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 74 of 74 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 5868 |
| code | 340 |
| data | 1680340 |
| unused | 132096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 74 | 10 |
| Disk.1 bytes 531200-547072 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: raw disk image (811008 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- Belial_v1.0/Belial_v1.0.hdf: hard-disk image (AmigaDOS OFS, 1952 blocks): its filesystem parses and all 5 files in it are byte-identical to the extracted tree Belial_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: bytes 531200-547072 look like code no run loaded: recovered position-independently (load address unknown)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: 1 logged loads cover 15872 bytes of the image
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 0-21 (bytes 0-5632) hold ASCII text
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 44-49 (bytes 11264-12800) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 51-56 (bytes 13056-14592) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.13)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 58-134 (bytes 14848-34560) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.59)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 156-184 (bytes 39936-47360) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 188-221 (bytes 48128-56832) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.65)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 231-246 (bytes 59136-63232) outside the filesystem hold data, not code (opcode density 0.042, entropy 5.32)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 289-334 (bytes 73984-85760) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.38)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 337-345 (bytes 86272-88576) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.59)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 347-359 (bytes 88832-92160) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.81)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 397-409 (bytes 101632-104960) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.19)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 413-418 (bytes 105728-107264) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.15)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 420-421 (bytes 107520-108032) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.21)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 423-424 (bytes 108288-108800) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.21)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 426-427 (bytes 109056-109568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.21)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 429-446 (bytes 109824-114432) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.60)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 460-471 (bytes 117760-120832) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.25)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 484-845 (bytes 123904-216576) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.32)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 859-933 (bytes 219904-239104) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.05)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 946-970 (bytes 242176-248576) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.19)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 984-1035 (bytes 251904-265216) outside the filesystem hold data, not code (opcode density 0.012, entropy 1.95)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 1037-1078 (bytes 265472-276224) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.27)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 1096-1190 (bytes 280576-304896) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.38)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 1192-1195 (bytes 305152-306176) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.86)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 1208-1302 (bytes 309248-333568) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.07)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 1321-1322 (bytes 338176-338688) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.26)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 1326-1334 (bytes 339456-341760) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.25)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 1337-1348 (bytes 342272-345344) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.36)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 1350-1395 (bytes 345600-357376) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.36)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 1408-1417 (bytes 360448-363008) outside the filesystem hold data, not code (opcode density 0.001, entropy 1.97)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 1424-2072 (bytes 364544-530688) hold ASCII text
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2140-2176 (bytes 547840-557312) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.40)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2179-2457 (bytes 557824-629248) outside the filesystem hold data, not code (opcode density 0.001, entropy 6.69)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2459-2555 (bytes 629504-654336) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.54)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2560-2575 (bytes 655360-659456) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.43)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2587-2644 (bytes 662272-677120) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.22)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2651-2656 (bytes 678656-680192) outside the filesystem hold data, not code (opcode density 0.034, entropy 3.33)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2693-2695 (bytes 689408-690176) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.94)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2718-2723 (bytes 695808-697344) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.10)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2725-2730 (bytes 697600-699136) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.13)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2732-2808 (bytes 699392-719104) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.59)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2830-2858 (bytes 724480-731904) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2862-2895 (bytes 732672-741376) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.65)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2905-2920 (bytes 743680-747776) outside the filesystem hold data, not code (opcode density 0.042, entropy 5.32)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 2963-3008 (bytes 758528-770304) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.38)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 3011-3019 (bytes 770816-773120) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.59)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 3021-3033 (bytes 773376-776704) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.81)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 3071-3083 (bytes 786176-789504) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.19)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 3087-3092 (bytes 790272-791808) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.15)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 3094-3095 (bytes 792064-792576) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.21)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 3097-3098 (bytes 792832-793344) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.21)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 3100-3101 (bytes 793600-794112) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.21)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 3103-3120 (bytes 794368-798976) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.60)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 3134-3145 (bytes 802304-805376) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.25)
- Belial_v1.0/Belial_v1.0_hdf/Belial/Disk.1: chunks 3156-3167 (bytes 807936-811008) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.25)
- 1 never-loaded region(s), 15872 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_move_moveq_lea_bsr`: Short code fragment that copies 1024 to d1, then loads 1 into d2, then loads the address of a fixed memory address into a0, then calls another routine, then loads the address of a location in the code into a0, then copies a0 to a fixed memory address.
- `initialize_hardware_and_jump`: Configures several hardware registers including DMA control and interrupt settings, performs conditional subroutine calls based on comparison results, and executes a jump to a memory location.
- `setup_cia_interrupt`: Checks a memory location for a specific value and updates it with a new word value if the condition is met, then returns.
- `frag_move_movem_not_ror_cmpi_bne_rts`: Short code fragment that copies a fixed memory address to d1, then saves registers on the stack, then inverts d1, then rotates d1 right by 1, then compares d1 with 95, then branches if the last result was not zero, then copies 240 to a fixed memory address, then copies a fixed memory address to a fixed memory address, then restores saved registers from the stack, then returns.
- `load_and_execute`: Loads data from a table using an index from register d7, performs arithmetic on the index, calls a routine through an indirect jump, and adjusts a pointer before returning.
- `frag_movem_movea_jsr_rts`: Short code fragment that saves registers on the stack, then copies a location in the code to a2, then calls another routine, then restores saved registers from the stack, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
- `frag_movea_jmp`: Short code fragment that copies a location in the code to a2, then jumps to another point in the code.
