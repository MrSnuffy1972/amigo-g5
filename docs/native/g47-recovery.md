# Game 47 (Speedball v2.0 0086): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/00047). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (992262 top-level bytes in 11 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 61 of 61 units identical (bytes and relocations), 77 raw-word lines |
| Functions named and described | pass, 380 of 380 named (0 author symbols), 380 of 380 described (26 by rule, 2 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 4016 of 4016 (100.00%) |
| Recovered without load address (counted in proof 1, flagged) | 12288 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7604 of 7604 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 562441 |
| code | 25594 |
| data | 404153 |
| padding | 74 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| Speedball.slave | 1 | 258 | 20 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 2549 | 148 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 676 | 26 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| logged disk load from Disk.1 | 1 | 0 | 0 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 4112 | 178 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 1 | 1 |
| run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM) | 1 | 8 | 7 |
| Disk.1 bytes 17664-17920 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 29696-29952 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 374784-375040 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 416512-416768 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 422400-422656 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 434432-434688 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 440320-440576 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 446208-446464 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 458240-458496 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 464128-464384 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 470016-470272 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 482048-482304 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 487936-488192 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 493824-494080 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 505856-506112 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 511744-512000 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 517632-517888 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 529664-529920 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 535552-535808 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 541440-541696 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 553472-553728 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 559360-559616 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 565248-565504 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 577280-577536 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 583168-583424 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 660480-660736 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 767744-768000 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 773632-773888 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 779520-779776 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 791552-791808 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 797440-797696 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 803328-803584 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 815360-815616 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 821248-821504 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 827136-827392 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 839168-839424 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 845056-845312 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 850944-851200 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 862976-863232 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 868864-869120 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 874752-875008 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 886784-887040 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 892672-892928 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 898560-898816 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 910592-910848 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 916480-916736 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 922368-922624 (code never loaded by a reference run) | 1 | 0 | 0 |
| Disk.1 bytes 934400-934656 (code never loaded by a reference run) | 1 | 0 | 0 |

## Cases handled

- Speedball_v2.0_0086/Speedball/Disk.1: raw disk image (940416 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- 9 logged WHDLoad disk loads recovered as units built from the disk image bytes
- runtime segment an address: 3233 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address (play-rs-m1500-unpk00): 1 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- runtime segment an address: 8 executed PCs in code the game unpacked, copied or loaded at run time, carved from reference-run RAM and recovered
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 17664-17920 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 29696-29952 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 374784-375040 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 416512-416768 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 422400-422656 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 434432-434688 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 440320-440576 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 446208-446464 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 458240-458496 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 464128-464384 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 470016-470272 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 482048-482304 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 487936-488192 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 493824-494080 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 505856-506112 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 511744-512000 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 517632-517888 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 529664-529920 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 535552-535808 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 541440-541696 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 553472-553728 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 559360-559616 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 565248-565504 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 577280-577536 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 583168-583424 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 660480-660736 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 767744-768000 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 773632-773888 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 779520-779776 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 791552-791808 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 797440-797696 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 803328-803584 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 815360-815616 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 821248-821504 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 827136-827392 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 839168-839424 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 845056-845312 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 850944-851200 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 862976-863232 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 868864-869120 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 874752-875008 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 886784-887040 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 892672-892928 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 898560-898816 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 910592-910848 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 916480-916736 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 922368-922624 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: bytes 934400-934656 look like code no run loaded: recovered position-independently (load address unknown)
- Speedball_v2.0_0086/Speedball/Disk.1: 57 logged loads cover 588672 bytes of the image
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 70-115 (bytes 17920-29696) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 117-138 (bytes 29952-35584) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1628-1649 (bytes 416768-422400) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1651-1696 (bytes 422656-434432) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1698-1719 (bytes 434688-440320) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1721-1742 (bytes 440576-446208) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1744-1789 (bytes 446464-458240) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1791-1812 (bytes 458496-464128) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1814-1835 (bytes 464384-470016) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1837-1882 (bytes 470272-482048) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1884-1905 (bytes 482304-487936) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1907-1928 (bytes 488192-493824) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1930-1975 (bytes 494080-505856) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 1977-1998 (bytes 506112-511744) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 2000-2021 (bytes 512000-517632) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 2023-2068 (bytes 517888-529664) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 2070-2091 (bytes 529920-535552) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 2093-2114 (bytes 535808-541440) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 2116-2161 (bytes 541696-553472) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 2163-2184 (bytes 553728-559360) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 2186-2207 (bytes 559616-565248) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 2209-2254 (bytes 565504-577280) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 2256-2277 (bytes 577536-583168) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 2581-2603 (bytes 660736-666624) outside the filesystem hold data, not code (opcode density 0.030, entropy 6.20)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3000-3021 (bytes 768000-773632) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3023-3044 (bytes 773888-779520) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3046-3091 (bytes 779776-791552) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3093-3114 (bytes 791808-797440) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3116-3137 (bytes 797696-803328) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3139-3184 (bytes 803584-815360) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3186-3207 (bytes 815616-821248) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3209-3230 (bytes 821504-827136) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3232-3277 (bytes 827392-839168) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3279-3300 (bytes 839424-845056) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3302-3323 (bytes 845312-850944) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3325-3370 (bytes 851200-862976) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3372-3393 (bytes 863232-868864) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3395-3416 (bytes 869120-874752) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3418-3463 (bytes 875008-886784) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3465-3486 (bytes 887040-892672) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3488-3509 (bytes 892928-898560) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3511-3556 (bytes 898816-910592) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3558-3579 (bytes 910848-916480) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3581-3602 (bytes 916736-922368) outside the filesystem hold data, not code (opcode density 0.029, entropy 6.23)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3604-3649 (bytes 922624-934400) outside the filesystem hold data, not code (opcode density 0.033, entropy 6.21)
- Speedball_v2.0_0086/Speedball/Disk.1: chunks 3651-3673 (bytes 934656-940416) outside the filesystem hold data, not code (opcode density 0.028, entropy 6.18)
- 48 never-loaded region(s), 12288 bytes, recorded as UNVERIFIED DATA (under 5% of bytes decode as instruction flows; Karl 4 Nov 2026, 5% from 5 Nov)

## Cracktros (kept, not protection)

- none

## Copy protection found

- `enable_audio_dma_if_flag_set` (Disk.1 +an address loaded at an address (77376 bytes)): If the byte flag at offset 5449 from the program base is set, writes 255 to the audio interrupt enable register and enables DMA channels 0 through 3 via DMACON, then sets a done flag and returns. Kind: hardware register manipulation/timing trap. Pass path: the branch where cRam0100df0d is non-zero (the 'if' block executes).
- `save_custom_chip_state_and_enable_dma` (runtime segment an address): Stores a flag at address zero, then copies ADKCONR, DMACONR, INTENAR and INTREQR into low memory and writes new ADKCON, INTENA, INTREQ and DMACON values to the custom chip registers before setting CIAB port A bits. Kind: disk hardware/DMA state synchronization. Pass path: the instruction following the JSR an address.l (the code continues execution normally after the routine returns).
- `enable_dma_interrupts_and_clear_buffer` (runtime segment an address): Enables interrupt and DMA bits in the custom chip registers, calls a subroutine, then fills a large buffer at offset 4270 of a5 with zeros using a counted loop. Kind: disk hardware/DMA initialization and buffer clearing. Pass path: the loop completes and execution continues to the next instruction after the function call.
- `pulse_ciab_disk_select_and_return` (runtime segment an address): Loads d0 with a word immediate, calls the system vector at address 128, sets then clears bit 7 and bit 3 of the CIAB port B disk control register, restores d0 from the stack and returns. Kind: disk hardware status check/trap. Pass path: the instruction immediately following the call (the code continues to the next line if the bitmask operation succeeds).
- `toggle_ciab_disk_bit_with_delay_loop` (runtime segment an address): Clears bit 3 of the CIAB port B disk control register, loops 100 times using dbra, then calls a nearby routine and returns; the leading ori and addx bytes are stray data. Kind: disk hardware bit manipulation (CIAB_PRB). Pass path: not determined.

## Functions


### Speedball.slave

- `return_minus_one_constant`: Loads the immediate value minus one into d0 and returns immediately, providing a constant result to the caller.
- `slave_startup_hunk_dispatch`: Saves the resident library base, sets up the stack, clears memory, calls the loader, and branches on hunk type codes to relocate or launch the game.
- `disable_dma_and_interrupts_restore`: Clears a low memory word, copies a small routine, calls the library, then disables DMA and all interrupts via the custom chip registers before returning.
- `shutdown_dma_interrupts_and_chain`: Clears a word at address 36, calls a copy routine, invokes an offset of the library base in a2, then writes DMACON, INTENA and INTREQ to disable DMA and interrupts before returning.
- `call_library_offset_32_preserving_a2`: Saves a2 on the stack, loads the library base from a pc-relative longword into a2, calls the routine at offset 32 of that base, restores a2 and returns.
- `scale_d6_d7_by_5632_and_call`: Saves d1, d6 and d7, multiplies d6 and d7 by 5632, moves the products into d1 and d0, calls a subroutine, restores the registers, sets d0 to zero and returns.
- `scale_row_col_by_5952_and_call`: Saves d1 and d6-d7, masks d6 and d7 to 16 bits, decrements d7, multiplies both by 5952, moves results into d1 and d0, calls a subroutine, restores registers and returns zero in d0.
- `set_bit_and_compare_rotated_not`: Sets bit 0 at offset 3584 from a0, pushes d0, inverts and rotates d0 right by one, compares it with a byte at a fixed program counter offset, branches if equal, else pops d0 and returns.
- `store_d7_to_global_and_jump`: Writes d7 into the absolute word address 1082, then branches unconditionally to another routine without returning to the caller.
- `store_d7_to_2450_then_jump`: Writes the longword in d7 to absolute memory address 2450 and branches to the shared continuation at 404.
- `store_d7_to_238a_then_jump`: Writes the longword in d7 to absolute memory address 238a and branches to the shared continuation at 404.
- `dispatch_league_save_by_d7_code`: Compares d7 against three byte codes and branches to matching handlers; otherwise executes an illegal instruction, then saves registers and calls library routines with the league.sav name.
- `load_knockout_save_into_buffer`: Pushes d0-d1/a0-a2, calls the library open routine with the knockout save name and a 5952-byte buffer, then returns zero in d0.
- `load_twoplay_save_into_buffer`: Pushes d0-d1/a0-a2, calls the library open routine with the twoplay save name and a 5952-byte buffer, then returns zero in d0.
- `zero_filled_data_area`: Single zero word that decodes as ori.b #0,d0; this is a zero-filled data area referenced by address, not executable code.
- `divide_d0_by_40_keep_high_word`: Swaps d0 to divide its high word by 40, clears the low word, swaps back, then branches to the delay routine.
- `save_d0_delay_by_d7_restore`: Pushes d0, loads d7 into d0, calls the vertical-position delay loop, then pops d0 and returns.
- `wait_for_vertical_position_change`: Saves d0, polls the VHPOSR register until its byte changes, then restores d0 and loops dbra times before returning.
- `replace_value_in_range`: Scans the memory range from a0 up to a1 in two-byte steps, replacing each long word equal to d0 with d1, then restores registers and returns.
- `call_library_offset_28_with_d2_one`: Saves registers, sets d2 to one, loads the library base from a pc-relative location into a2, calls the function at offset 28 of that base, restores registers and returns.

### logged disk load from Disk.1

- `hunk_relocator_apply_deltas`: Parses AmigaDOS hunk data, allocates stack space for hunks, applies relocation deltas to memory, sets COLOR00 to indicate success or failure, and returns.
- `index_a3_table_to_a2_and_d1`: Scales d6 by four, loads the longword at that offset from the table pointed to by a3 into a2, copies a2 into d1 and returns.
- `data_bytes_then_set_trap_vector`: The leading bytes decode as immediate ori instructions and are data, not code; the tail sets d1 to zero, loads address 14 into a0, stores a0 at absolute address 20 and branches away.
- `fill_words_with_color_cycle`: Walks a1 from address 8 to 128 in steps of four, storing the value 114 at each slot and writing an incrementing d1 word to the COLOR00 register, then pops a word and returns.
- `install_trap_and_switch_stack`: Clears d0, triggers a divide-by-zero exception, writes a trap instruction and stack pointer into low memory, then saves the old stack pointer and branches to the next routine.
- `set_color00_and_loop_forever`: Writes the color value to the COLOR00 hardware register and jumps back to itself, repeating the write indefinitely.
- `speedball_level_setup_and_load`: Sets color and interrupt registers, clears memory, initializes game pointers and structures, loads graphics and level data via library calls, and polls the vertical position register while setting up the playfield.
- `load_and_relocate_speedball_assets`: Calls setup routines, allocates chip memory via the graphics library, computes pointers into the loaded data block at offsets, then copies a large block of asset data into the destination buffer.
- `setup_playfield_pointers_and_start`: Copies a text string, sets up three pointers into the level data at fixed offsets, calls the level init routines, patches a vector, and jumps to the main game loop.
- `format_highscore_name_entry_buffer`: Copies a name string into a fixed buffer, inserting a carriage return every 34 characters and replacing non-alphanumeric bytes with spaces, stopping at a plus sign.
- `fill_d7_plus_one_bytes_with_d0_then_zero`: Loads d0 with 13 and d1 with 0, writes d0 to successive bytes at a1 while decrementing d7, then writes a zero byte after the loop and restores registers d0-d7/a0-a4/a6 from the stack.
- `disable_all_interrupts_and_restore_intena`: Saves d0, writes all bits to INTENA to disable interrupts, copies the saved interrupt mask from memory into d0, sets the master enable bit and writes it back to INTENA, then restores d0.
- `save_a5_disable_interrupts_and_call`: Saves all registers, stores a5 at offset 1654, reads INTENAR into d7, saves it at offset 1640, writes 16383 to INTENA to disable interrupts, calls a subroutine, then re-enables interrupts with bits 15 and 3 set and restores registers.
- `install_cia_timer_interrupt_vector`: Writes zero to CIAA control register and 136 to CIAA data direction, then stores the address of code at 1514 into the level-1 autovector at 104, chaining the old vector at 1642.
- `clear_state_and_dispatch_on_byte_change`: Clears word at offset 746 from a5, compares d0 with byte at 1130(a5), and if changed stores d0 there and jumps through pointer at 730(a5).
- `empty_stub`: Empty routine: returns to the caller immediately without doing anything.
- `clear_longwords_from_694_forever`: Loads address 1684 into a0 and repeatedly clears longwords while incrementing a0, looping forever without returning.
- `clamp_timer_to_300_or_decrement`: Two no-op ori.b instructions precede reading word at offset 952 from a5 into d1; compares d1 with 300, branching to skip or subtract 2 when above, then compares again and branches.
- `advance_timer_and_clamp_to_300`: Adds 2 to d1, clamps it to 300 if it exceeds that, stores it at offset 952 from a5, subtracts the word at 994, and clears the word at 886 if the subtraction borrows.
- `clamp_d1_to_limit_store_at_376`: Compares d1 with the word at offset 996 from a5, replaces d1 with that limit if d1 is not less, then stores d1 at offset 886 from a5 and returns.
- `select_player_one_pointer_and_palette`: Checks a direction flag, optionally negates two velocity words and calls a helper, then reads the current player object's byte at offset 63, swaps its byte 22 with the stored player-one object, clears three flags and stores a palette index from a table indexed by the object's first word shifted right four.
- `select_player_two_pointer_and_palette`: Checks a direction flag, optionally negates two velocity words and calls a helper, then reads the current player object's byte at offset 63, swaps its byte 22 with the stored player-two object, sets a flag to all ones, clears two flags and stores a palette index from a table indexed by the object's first word shifted right four.
- `blit_playfield_rows_to_bitplanes`: Saves registers, picks source and destination pointers from the a5 structure swapping them by a flag bit, programs the blitter control, modulo and pointer registers, then issues four blits of height 200 with a fixed stride, waiting on the blitter busy bit before each.
- `clear_word_at_a5_offset_and_call`: Saves registers d0-d7 and a0-a4/a6, clears the word at offset 376 from a5, loads d0/d2/d4 with zero and d1/d3 with 320 and 300, then calls a routine and restores registers.
- `fill_three_playfield_bands`: Saves registers, computes a screen address from d0 and d2 plus a base at offset 442 from a5, then calls the band filler three times advancing a0 by 23680 bytes and halving d3 each time.
- `fill_band_bytes_with_d3_bit`: Saves registers, aligns d0 to a multiple of eight, fills a run of bytes at a0 with all ones or zeros depending on bit zero of d3, then calls a follow-up routine and restores registers.
- `set_pixel_bit_in_plane`: Computes bit position from d0 low bits into d7, tests bit 0 of d3 and sets that bit in the byte at (a0) if set, then branches to the next plane step.
- `clear_pixel_bit_and_advance_plane`: Clears the bit in d7 at (a0), increments d0, compares it with d1 and branches back when d0 exceeds d1, otherwise advances a0 by one byte and returns with d6 set to 255.
- `return_constant`: Returns the constant an address in d0 to the caller without any other effect.
- `plot_pixel_across_four_bitplanes`: Saves registers, computes a plane byte address from d0 and d1 plus the bitmap base at offset 442 of a5, then for each row up to d2 clears and conditionally sets the pixel bit in four planes spaced 23680 bytes apart.
- `save_d6_and_376_call_1372`: Pushes d6 and the word at offset 376 of a5, clears that word, sets byte at offset 415 of a5 to 4, sets d6 to 1, calls the routine at 1372, then restores the word and d6.
- `clear_376_set_415_call_1372`: Saves the word at offset 376 of a5 on the stack, clears it, sets byte at offset 415 of a5 to 4, calls the routine at 1372, then restores the saved word.
- `swap_1ba_1ce_1d2_call_1372`: Sets byte at offset 415 of a5 to 255, saves the long at offset 466 of a5, copies the long at offset 462 into it, calls 1372, copies the long at offset 466 into it, calls 1372 again, then restores the saved long.
- `clear_sprite_and_clamp_d2_to_limit`: Clears the word at address 16, saves registers d0-d7/a0-a4/a6 on the stack, copies d1 to d2, loads the word at offset 886 from a5 into d5, and compares d2 against it, adding d7 and branching to clamp or accept.
- `compute_sprite_tile_address_and_flags`: Adjusts d5 by the words at offsets 884 and 882 from a5, compares against d2, calls the tile lookup, clears bit 0 at offset 1026 from a5, and computes a sprite tile address from d1, d3, d0 and d7 into a1.
- `build_sprite_descriptor_entries`: Clears bit 2 at offset 1045 from a5, allocates a descriptor via the routine at 16D4, advances the pointer at offset 438 from a5 by 780, and fills sprite descriptor fields at offsets from a3 with tile, size and pointer values.
- `write_three_sprite_entries_wait_blitter`: Stores a0, a2, a1 and d7 into fields at offsets 76, 80, 72, 84 and 88 of a3, advances a1 by d6 and a0 by d4, waits while bit 6 of a3+2 is set, and repeats this three times before restoring registers and returning.
- `wait_blitter_and_setup_sprite_entries`: Waits while bit 6 of the word at offset 2 of a3 is set, then fills sprite control words at offsets 64 to 70 and 96 to 100 of a3 from d0 and d2, tests a flag at offset 1045 of a5 and branches back to the caller's loop.
- `restore_registers_and_return`: Pops d0 through d7, a0 through a4 and a6 from the stack and returns immediately to the caller.
- `compute_a2_from_d7_and_a1`: Tests the word at absolute address 16; if zero branches away, if negative skips, otherwise clears a2 then builds a2 from d7 shifted left four times plus a1.
- `test_a5_flag_byte_branch`: Tests the byte at offset 415 from a5 and branches to one of two targets depending on its sign bit, without touching other registers.
- `load_a2_a3_for_custom_chip`: Loads a2 with the address at offset 892 from a5 and a3 with the custom chip base, then branches onward to shared code.
- `fill_blitter_line_buffer_with_ones`: When d6 exceeds two, saves d0-d6, points a2 at the blitter line buffer plus offset, writes all-ones words repeatedly for six passes, restores registers and jumps to the shared exit.
- `setup_blitter_line_and_store_pointer`: Builds a blitter control word from d6 and d7, programs blitter pointers, modulos and control registers, waits on DMACONR, then stores a computed line pointer at offset 562 of a5.
- `queue_blitter_line_command`: Saves registers, optionally advances d4 and d6, writes a command word and blitter register values into the a2 queue, programs blitter pointers and size, then restores registers and returns.
- `set_timer_and_jump_to_init`: Stores the immediate value 288 into d7 and branches unconditionally to the routine at the fixed target, setting up a timer constant before continuing.
- `init_counters_from_d4_and_d7`: Writes 153 to a5 offset 1120, saves d7 to a5 offset 1002, loads a byte from a5 offset 1134 or 1090 depending on d4 sign, stores it and its tenth to a5 offsets, then branches.
- `scan_five_entities_for_target`: Checks a5 flags, then loops up to five times over 64-byte entity records in a0, skipping entries matching d3 or with bit 7 set, comparing type and position fields, and returns when none match.
- `clear_slot_and_maybe_play_sound`: Clears byte at offset 53 of a0, tests byte at 22 of a0 and returns if zero, otherwise clears byte at 1041 of a5, tests d4 sign and branches to the sound call or back to the loop start.
- `play_sound_then_loop_back`: Calls the sound routine at a fixed offset, then branches back to the start of the surrounding loop in the caller.
- `update_ball_velocity_and_sound`: Checks two a5 flags, optionally calls a sound routine via a0, then uses d4 sign and a0 byte to scale velocity from a table into a5 fields and resets a0 byte.
- `set_player_speed_from_table_and_state`: If the byte at offset 1060 of a5 is zero, multiplies the word at offset 958 of a5 by a table value indexed by the doubled byte at offset 8 of a0, stores results at offsets 954 and 956 of a5, sets state bytes and calls a subroutine.
- `build_sprite_frame_and_position_table`: Writes four consecutive d7 values spaced by 23680 into a0, copies 82 longs from a pc-relative table into a1, copies ten words from a0 into a1 offset 170, and writes four screen positions derived from a5 offset 602 into a1 offset 262.
- `select_vblank_or_custom_chip_path`: Loads the custom chip base address into a3, tests the longword at a5 offset 132 and branches to one of two continuation addresses depending on whether it is zero.
- `set_d0_one_and_branch_to_toggle`: Loads the immediate value 1 into d0 and branches unconditionally to the shared toggle routine, which selects the alternate pointer and flag state.
- `toggle_pointer_based_on_d0`: Tests the low byte of d0; if zero it branches to the clear path, otherwise it copies the longword at offset 124 of a5 to offset 424, calls the routine at 7780 and branches onward.
- `select_first_pointer_and_clear_flag`: Copies the longword at offset 124 of a5 into offset 424, clears the longword at offset 132 and the byte at offset 420, then returns to the caller.
- `conditional_audio_tick_or_skip`: Tests byte in d0; if zero branches past the update, otherwise copies the longword at offset 128 of a5 to offset 424, calls the routine at absolute address 7780, then branches to the end.
- `reset_audio_state_and_flag`: Copies the longword at offset 128 of a5 to offset 424, stores 1 at offset 132 of a5 and byte 1 at offset 420 of a5, then returns.
- `install_vertical_interrupt_vector`: Fills an interrupt structure at offset 136 of a5 with handler and data pointers, sets interrupt level bytes, and if d5 is nonzero chains the old level-2 vector and enables INTENA bit 15.
- `call_library_vector_168`: Library stub: loads a library base from a global variable and calls the library function at offset minus 168 of that base, returning its result to the caller.
- `copy_32_words_to_two_buffers`: Saves registers, calls a subroutine, then copies 32 words from the a5 structure at offset 8 into two buffers pointed to by offsets 124 and 128, advancing each by 4 bytes per word.
- `clear_32_words_then_copy`: Saves d0 and a0, zeroes 32 words starting at offset 8 of the a5 structure, then calls the copy routine to duplicate that buffer into the two destination buffers.
- `clear_16_words_then_jump_copy`: Saves d0 and a0, sets d0 to 15 and a0 to the a5 structure offset 8, then branches into the shared clear loop to zero 16 words before the copy routine runs.
- `decrement_matching_nibbles_in_grid`: Loops over 32 words at offset 8 from a5, decrementing the low, middle and high nibbles that equal d6, then calls two routines and repeats for each value from 15 down to 1.
- `call_clear_grid_and_advance_counter`: Loads the address of the grid at offset 8 from a5 into a1, calls the clearing routine, calls two library routines, decrements d6 and branches back into the outer loop.
- `call_305e_then_restore_regs_and_return`: Calls the routine at the given offset, then pops registers d0 through d7 and a0 through a4 plus a6 from the stack, releases the a6 frame and returns.
- `count_team_attribute_thresholds`: Calls a setup routine, then for each of 32 entries reads a word from the passed table and increments counters at offsets in a5 for each nibble value at or above the threshold in d6.
- `decrement_threshold_and_loop_back`: Loads a1 with the counter block at offset 8 of a5, calls a routine, calls two library functions, decrements d6 and branches back into the outer counting loop.
- `restore_registers_and_return_2`: Loads a1 with the counter block at offset 8 of a5, calls a routine, then pops d0-d7/a0-a4/a6 from the stack, releases the a6 frame and returns.
- `copy_32_words_to_a5_buffer`: Copies 32 words from the address in a6+8 into the buffer at a5+8, then calls a subroutine and restores d0/a0-a1 before returning.
- `init_player_state_and_sprite_table`: Saves registers, calls a setup routine, clears the word at a5+886 and a global word, selects one of two state blocks based on a5+420 bit 0, then fills four sprite entries and calls another routine.
- `build_sprite_entries_from_player_index`: Computes an offset from the word at a5+886, selects one of two sprite tables based on a5+420 bit 0, stores table pointers, calls a setup routine, writes four sprite coordinate pairs and jumps to a routine.
- `init_sound_chip_registers`: Tests the word at absolute address 3320; if zero, writes mixer and period values to offsets in the a0 hardware block and returns.
- `clear_flag_and_decrement_counter`: Saves d0-d1, clears byte at offset 373 of a5, reads byte at 3b4 of a5 and if it equals 25 decrements the byte counter at 372 of a5 when above 5.
- `increment_counter_on_key_40`: Compares d0 with 40 and, when equal, adds one to the byte counter at offset 372 of a5 before branching onward.
- `handle_key_50_and_10_adjust_speed`: Compares d0 with 80 and 16, clears byte at a5+948, calls the routine at 7780, clears two a5 flag bytes when bit 7 is set, and decrements the word at a5+884 for key 16.
- `handle_space_increment_speed_counter`: When d0 is 32, increments the word at a5+884 up to 304, clears byte at a5+948, copies byte at a5+882 into a0+208 and a0+134, then writes sprite control longs and bytes based on the counter.
- `write_display_control_and_loop_back`: Stores a long constant and a byte from d0 into offsets of a0, increments d0, and on carry writes a long constant to another a0 offset before branching back.
- `write_blitter_control_and_advance_d0`: Writes a blitter control longword to offset 300 of a0, stores d0 at offset 304, adds 7 to d0 and, on carry, writes a second control longword to offset 312 and d0 to offset 316 before branching on.
- `write_second_blitter_control_word`: Writes a blitter control longword to offset 312 of a0 and stores d0 at offset 316, then branches to the shared tail that computes the two playfield widths.
- `compute_playfield_half_and_remainder`: Subtracts the byte at offset 882 of a5 from the word at offset 884, stores half the difference at offset 994 and the value 592 minus the difference at offset 996, then restores d0-d1 and returns.
- `set_object_frame_from_direction_table`: If byte 16 of a0 is zero and byte 52 of a0 is nonzero, calls a direction routine, indexes the table at 17900 by direction and object byte 8, stores the result in byte 15 of a0 and sets byte 16 to 5, then optionally calls another routine.
- `resolve_adjacent_object_collision`: Calls a direction routine, indexes the table at 17972 using both objects' byte 8 values, and branches on the table byte to push objects apart, copy positions and set flag bit 3 in byte 11 of a0.
- `resolve_object_overlap_by_state`: Checks bytes 19 of a0 and a1, swaps the objects, copies words 48 and 50 into offsets 0 and 2, calls the frame routine, adjusts byte 56 of a0 by byte 55 of a1 and sets flag bit 3 in byte 11.
- `empty_stub_2`: Empty routine: returns to the caller immediately without doing anything.
- `set_a6_to_41b_a5_and_call_5886`: Loads a6 with the address a5 plus 1051, calls the routine at 5886, then returns to its caller.
- `store_d0_and_process_object_flags`: Saves d0 into a5+1057, tests d7 bit 7 and object flag bits in a0, compares counters in a5, and branches to other routines based on the results.
- `set_426_flag_if_41d_nonzero`: Tests the byte at a5+1053; if nonzero it writes 1 to a5+1062, then branches to the routine at 59D0.
- `scale_sprite_velocity_by_speed`: Reads a sprite index from a0, looks up two words in a table at a1, scales both by a speed value from a5 and stores the results plus the object position into a5 fields.
- `spawn_effect_and_advance_frame_counter`: Builds an effect record at a5, calls three subroutines, optionally invokes a library vector, then increments the byte at a6 and resets it with new state bytes when it reaches ten.
- `clear_animation_if_flag_set`: Tests the byte at offset 1053 in a5; if set, clears the byte at a6, clears bit three of a byte at a0, and calls a routine when d7 is zero.
- `check_player_flag_and_branch`: Tests byte at offset 1089 from a5, branches if zero or negative, then tests byte at offset 1057 from a5 and branches if negative, otherwise falls through.
- `clamp_and_store_sprite_position`: Checks a5 flag and a0 byte, calls two subroutines to get d0/d1, clamps them to screen bounds, saves old position at offsets 48 and 50, stores new d0/d1 at a0 and a0+2, and writes d7 to offset 52.
- `restore_x_position_and_clear_flag`: Loads word at offset 48 from a0 into d0, clears byte at offset 53 from a0, masks d7 with 131, then branches to shared continuation.
- `load_word_clear_byte_and_branch`: Loads a word from offset 50 of the structure pointed to by a0 into d1, clears the byte at offset 53, masks d7 with 140, then branches to another routine.
- `select_frame_and_advance_animation`: Chooses a frame index from a table or the object's byte field, reads a byte from the object's data pointer, stores it at offset 20, then updates animation counters and frame offsets in the object.
- `compute_sprite_offset_and_advance_frame`: Adjusts d0 by game state and object fields, looks up a word offset in a table indexed by d2 and d0, adds the object's position, then advances the animation frame counter and calls a routine when the frame wraps.
- `load_two_words_from_a0`: Reads the word at offset 0 of a0 into d0 and the word at offset 2 of a0 into d1, then branches to another routine.
- `push_d2_d3_and_abs_differences`: Saves d2 and d3 on the stack, then replaces d2 with d2 minus d0 and d3 with d3 minus d1, negating each result if negative, before branching onward.
- `restore_d3_d2_after_ratio_estimate`: Compares d2 and d3, swaps them if needed, computes a scaled sum and difference using shifts and adds, then pops d3 and d2 from the stack and returns.
- `halve_d2_subtract_from_d0_restore`: Shifts d2 right by one bit, subtracts the result from d0, then pops d3 and d2 from the stack and returns.
- `init_object_from_table_if_flags_clear`: Checks two a5 flags, optionally calls a jump-table routine, then fills fields of the object at a0 from a table indexed by d0 and branches onward.
- `test_bit4_of_object_byte_b`: Tests bit 4 of the byte at offset 11 from a0 and branches away when it is set, otherwise falls through to the following code.
- `load_sprite_frame_from_table`: Copies a sprite frame descriptor from the table at a fixed base plus index times eight into the object at a0, storing frame pointer, dimensions, flags and a computed pixel offset.
- `refresh_if_not_state_two`: Checks byte 19 of the object at a0; if it is not 2, saves d0, sets d0 to 2 and calls the refresh routine before restoring d0 and returning.
- `compute_five_tile_offsets`: Loops five times over 64-byte records at a0, reading x and y words and calling the offset routine to store the result at offset 12 of each record.
- `classify_relative_octant`: Reads two word pairs from a0 and a1, subtracts the y and x differences and returns a code from 0 to 7 in d0 classifying the direction between the two points.
- `classify_relative_octant_swapped`: Reads word pairs from a1 and a0, computes the y and x differences with halved comparisons and returns a direction code from 0 to 7 in d0.
- `negate_dx_compare_half_dy`: Negates d2, copies it to d4, halves d4 and d0, compares the halves against d0 and d2 and returns 5 in d0 when the branches fall through.
- `halve_d0_until_within_d2`: Negates d0, subtracts d3 from d2, then repeatedly halves d4 and d0 comparing them against d2, returning 1 in d0 when d0 no longer exceeds d2.
- `halve_d0_until_within_negated_d2`: Negates d2, copies it to d4 and halves d4 and d0 while comparing against d2, returning 7 in d0 when d0 no longer exceeds d2.
- `return_constant_2`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_3`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_4`: Returns the constant an address in d0 to the caller without any other effect.
- `return_constant_5`: Returns the constant an address in d0 to the caller without any other effect.
- `fill_row_span_with_dbra`: Saves d1-d4, swaps d3 and d4, decrements d4, adds d0 to d1 and decrements it, calls a fill routine, increments d2 and repeats via dbra until d4 expires, then restores registers.
- `save_d0_d1_check_flag_set_delay`: Pushes d0 and d1 onto the stack, tests the byte flag at offset 1103 of a5, and if set loads 250 into d1 before branching onward.
- `poll_input_flags_until_change`: Calls a routine, masks bytes at a5 offsets with 143, loops while d1 counts down, stores the result at a5 offset 1122 and calls another routine before restoring d0 and d1.
- `wait_for_input_flags_clear`: Saves d0, then loops while the masked byte at a5 offset 941 is nonzero and while offset 1060 is zero and offset 933 masked is nonzero, then restores d0 and returns.
- `load_sprite_frame_words_to_a0`: Reads two bytes from a5 offsets, combines them with a base table pointer, and copies two words into offsets 24 and 26 of the structure at a0.
- `draw_filled_rect_four_edges`: Saves all data registers, then calls edge-drawing routines four times using d0-d4 bounds to outline a rectangle, restoring registers before returning.
- `format_and_print_message_via_library`: Saves registers, selects one of two a5 message buffers based on a flag bit, pushes arguments and calls the library routine at the fixed vector, then restores registers.
- `init_text_render_state`: Writes 5 to two a5-relative words, sets the a5 pointer at offset 678 plus 6608 into offset 686, writes 2 to another word, then branches onward.
- `render_text_with_control_codes`: Sets three a5 words, copies a pointer, saves registers, then scans bytes from a0 handling control codes and advancing a5 counters before branching to subroutines.
- `handle_carriage_return_code`: Compares d7 with 13, and if equal copies d2 to d0 and adds the a5 words at offsets 1000 and 1006 to d1 before branching back.
- `render_text_glyph_run`: Walks a string in a1, handling control bytes for position and color, and draws each character via the glyph and blit routines using the a5 globals for width and row stride.
- `restore_registers_and_return_3`: Pops d0-d2, a0 and a6 from the stack and returns to the caller, restoring the saved register set.
- `set_flag_and_jump_to_state`: Writes minus one to the longword at absolute address 992a and branches to the state routine at 8322.
- `init_player_select_and_draw_icons`: Stores pointers to two a5-relative buffers, sets a constant, calls a system routine, then loops three times reading coordinates from a table and drawing icons before checking flags and branching.
- `clear_selection_flag_and_continue`: Clears the byte at offset 1114 from a5 to zero and branches unconditionally to the shared continuation code.
- `wait_for_joystick_input_loop`: Loads 1500 into d0, saves registers, calls a routine, reads a byte from a4, masks bits, and loops via dbra until input or timeout, then jumps away.
- `poll_input_until_joystick_event`: Reads a byte from (a4), masks it with 143, and loops until a bit is set; then tests d7 bits 2 and 3 to branch to other input handling.
- `advance_selection_forward_wrap`: If d7 bit 3 is set, copies byte at offset 1114(a5) to 1115(a5), increments it modulo 3, skips the value at 1116(a5), and stores it back at 1114(a5).
- `apply_selection_and_redraw_panels`: If d7 bit 7 is clear, indexes two 12-byte records from 738(a5) using bytes at 1115(a5) and 1114(a5), reads two words from each, calls three subroutines, then jumps to 8256.
- `write_byte_to_pointer_and_read_back`: Loads a pointer from offset 702 of a5, stores the byte at offset 1114 of a5 through it, reads that byte back into offset 1116 of a5, then calls a subroutine and returns.
- `copy_two_indexed_bytes_to_buffers`: Copies the byte at offset 1106 of a5 into buffer at 1800 of a5, and if the byte at offset 1060 of a5 is zero also copies the byte at 1110 of a5 into buffer at 1832 of a5.
- `copy_bytes_until_terminator_thirteen`: Uses the byte at offset 1106 of a5 as an index into a table of longword pointers at 738 of a5, then copies bytes from the pointed string to a1 until a byte equal to 13 is found.
- `clear_byte_at_a1`: Writes zero to the byte pointed to by address register a1 and returns immediately to the caller.
- `clear_sprite_flag_and_chain`: Calls a subroutine, clears the byte at offset 1178 from a5, then branches to another routine after copying that zero through a pointer at offset 702.
- `render_sprite_from_table_entry`: Saves registers, indexes a sprite table by the byte at offset 1114 from a5, reads position and size fields, waits in a delay loop, calls drawing routines and repeats until a negative table byte.
- `call_7780_then_restore_registers`: Calls the routine at address 7780, then pops registers d0-d7 and a0-a4/a6 from the stack and returns to the caller.
- `draw_sprite_with_palette_offset`: Computes a sprite base from d2 times 4608 added to a5 offset 698, draws it, then reads a 12-byte table entry at 555c to offset coordinates and a second sprite pointer before drawing again.
- `draw_sprite_with_96_size`: Saves registers, copies d1 into d2, sets d1 and d3 to 96, restores d0-d1 and calls the drawing routine at 8ac6 before restoring all registers and returning.
- `push_regs_and_load_sprite_source`: Saves d0-d5 and a0 on the stack, loads the address of a sprite data table into a0, then branches to the shared drawing code.
- `draw_sixteen_sprite_rows`: Saves registers, loads a sprite table into a0, sets up width, height and a counter of one, then loops sixteen times reading bytes and calling the blitter routine before restoring registers.
- `conditional_screen_update_sequence`: Saves a0, tests a flag in the a5 structure, and if clear calls two setup routines, then loads a pointer from a5 and calls two more routines before restoring a0.
- `reset_display_flag_and_clear_pending`: Calls two subroutines, sets byte at offset 748 in a5 to 255 if zero and passes address 750 in a5, then clears word at 992a and calls a routine if it was set.

### logged disk load from Disk.1

- `save_regs_call_54_restore`: Pushes d0-d3, d7 and a0-a6 onto the stack, calls the subroutine at offset 84, restores those registers and returns.
- `save_regs_call_19a_restore`: Pushes d0-d3, d7 and a0-a6 onto the stack, calls the subroutine at offset 410, restores those registers and returns.
- `save_a3_call_f2_restore_a3`: Pushes a3 onto the stack, calls the routine at offset f2, restores a3 from the stack and returns to the caller.
- `save_d1_d7_a3_call_16a_restore`: Pushes d1, d7 and a3 onto the stack, calls the routine at offset 16a, restores those registers and returns.
- `save_all_but_d4_d5_d6_call_10f0`: Pushes d0-d3, d7 and a0-a6 onto the stack, calls the routine at offset 10f0, restores those registers and returns.
- `call_sub14c_preserve_a3`: Saves a3 on the stack, calls the routine at offset 332, restores a3 and returns to the caller.
- `init_four_channel_audio_structures`: Sets the CIAA fire bit, then fills four 48-byte audio channel structures with volume, period, pointer and enable fields, and sets sound state flags.
- `silence_audio_and_clear_sound_flags`: Clears sound enable flags and four channel counters, then writes ADKCON, AUD0-AUD3 volume and DMACON to disable audio DMA and mute all channels.
- `set_four_player_flags_to_one`: Loads the program base into a3 and writes byte value one to four player state flags at offsets 5322, 5358, 5394 and 5430 from that base, then returns.
- `enable_audio_dma_if_flag_set`: If the byte flag at offset 5449 from the program base is set, writes 255 to the audio interrupt enable register and enables DMA channels 0 through 3 via DMACON, then sets a done flag and returns.
- `per_frame_sound_and_sprite_update`: Checks the enable flag, decrements a frame counter, updates a sprite pointer, then loops over four channels writing sprite pointers and audio period and volume registers in custom chip space before returning.
- `advance_sprite_frame_pointer`: Reads word at offset ten of a0 into d0, adds it to the word at offset eight to form an a2 index, increments d0 by two, tests the word at a3 plus a2; if zero it resets a2 and d0 and loads a1 from that table entry, then stores d0 at offset ten of a0.
- `load_two_bytes_and_set_flag`: Clears the word at offset thirty-two of a0, copies two consecutive bytes from a1 into offsets twenty and twenty-one of a0, sets bit one of the byte at offset zero of a0, then branches away.
- `copy_timer_and_set_channel_registers`: Copies the word at offset twenty-eight of a0 to offset thirty, stores a1 at offset four of a0, writes a constant long to a4 and a word to offset four of a4, tests a byte and conditionally writes to a6 offsets before branching.
- `reset_sound_channels_and_dma`: Clears several sound state bytes and words at offsets from a3, then writes 15 to DMACON and 255 to ADKCON to reset the audio hardware before returning.
- `store_byte_from_a1_to_a3_offset`: Copies one byte from the address in a1 into offset 1552 from a3, increments a1, then branches away to another routine.
- `init_sound_channel_from_a1_bytes`: Sets a flag byte at offset 1 from a0, copies two bytes from a1 into offsets 44 and 46 from a0, clears offset 45, then branches to another routine.
- `clear_byte_at_a0_offset_one`: Stores a zero byte at offset one from a0, then branches to a shared continuation elsewhere.
- `copy_byte_from_a1_to_a0_offset_three`: Reads one byte from the address in a1, increments a1, stores the byte at offset three from a0, then branches to a shared continuation.
- `copy_two_bytes_and_clear_word_at_a0`: Copies two bytes from a1 into offsets eight and nine of a0, increments a1 twice, clears the word at offset ten of a0, then branches to a shared continuation.
- `copy_byte_to_a3_offset_and_branch`: Copies the byte pointed to by a1 into offset 1547 of a3, then copies that same byte from program counter relative memory into offset 1548 of a3, and branches away.
- `init_seven_sprite_descriptors_and_masks`: If flag at offset 1550 of a3 is clear, builds seven sprite descriptors from a table, stores frame sizes and timing divisors, sets pointers at offsets 1556 and 24578, fills two eight-word masks with alternating bit patterns, and sets counters.
- `init_ten_sprite_descriptors_with_scale`: If flag at offset 4334 of a3 is clear, builds ten sprite descriptors from a table, computes a scaled byte from width and height, stores frame sizes and timing divisors, and sets the flag.
- `start_audio_channel_playback`: Enables CIA fire bit, silences audio, masks channel index to ten bits, clears per-channel active flags, copies a 24-byte sample descriptor into the channel structure, sets the audio pointer and length registers, waits, then marks the channel active.
- `wait_then_stop_all_audio_channels`: Spins a delay loop of 513 iterations, then for each of four audio channel structures checks the active flag and calls the channel stop routine with the matching custom chip base and DMA bit.
- `update_audio_channel_playback`: Performs per-channel audio update: handles volume fade, pitch slide, sample pointer advance and period reload, writes AUDxVOL, AUDxPER and AUDxPTR, and disables the channel DMA when the sample ends.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `check_entity_x_and_state_then_dispatch`: Loads d7 with 4, masks d7 with 12, reads word at offset 0 of a0 into d0, compares it with 96 and checks byte at offset 19 of a0 for 11, then calls two routines and branches away.
- `compare_entity_x_and_state_byte`: Reads the word at offset 0 of a0 and the byte at offset 19 of a0, comparing them against 225 and 11, then branches into other code paths based on those comparisons.
- `check_entity_bounds_and_set_state`: Tests byte at offset 53 of a0, then checks d0 between 96 and 224 and d1 against 56, sets byte 4 at offset 8 of a0, calls a routine and branches away.
- `init_entity_timer_and_position`: Tests byte at offset 53 of a0, and if clear sets it to one, stores 144 and 40 into words at offsets 24 and 26 of a0, loads them into d2 and d3, tests bit 1 of byte at offset 11 of a0 and clears it if set, then clears the offset 53 byte.
- `undecodable_bytes`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `undecodable_bytes_2`: Undecodable bytes (unverified code): the bytes at this entry do not decode as valid 68000 code, so the decompiler stopped here and no behaviour is claimed.
- `compare_d5_and_d1_against_208`: Compares the word in d5 with 208 and the word in d1 with 208, branching to bad instruction data when d5 is above 208 and d1 is below 208; the decoded bytes are not valid executable code.
- `compare_d5_and_d1_against_296`: Compares the word in d5 with 296 and the word in d1 with 296, branching to bad instruction data in several cases; the decoded bytes are not valid executable code.
- `scan_entity_slots_and_clear_offsets`: Walks entity records at a5+1160 in 32-byte steps, checks each record's byte at offset 23, and copies words 24 and 25 into the record when a counter matches a5+1036 or a5+1037.
- `check_two_flags_then_save_regs`: Tests byte 16 of a0 and byte 52 of a0, and if the first is zero and the second nonzero saves d1-d2/d4/a1 on the stack before branching onward.
- `advance_entity_frame_from_table`: If a0 byte 16 is zero and byte 52 nonzero, saves registers, indexes a table by d1 and a0 byte 8, stores the byte at a0+15, sets a0+16 to 5, and may call a helper when a0 matches a5+570.
- `resolve_player_collision_and_swap_ball`: Checks player state bytes and flags, compares scores against thresholds, then updates player position and state bytes and may call scoring routines when a collision condition matches.
- `update_ball_owner_and_switch_players`: Decrements a timer, picks a new ball carrier based on ball position, swaps player sprite pointers and direction bytes, then calls several update routines.
- `set_flag_and_call_41c_helper`: Writes 1 to byte at offset 421 of a5, loads address 41c of a5 into a6 and calls the routine at 195e before returning.
- `set_flag_and_init_41b_struct`: Writes byte 255 to offset 1065 from a5, loads address 1051 from a5 into a6, then calls the routine at 195e and returns.
- `init_41c_struct_via_17e6`: Loads address 1052 from a5 into a6, calls the routine at 17e6, and returns to its caller.
- `init_41b_struct_via_17e6`: Loads address 1051 from a5 into a6, calls the routine at 17e6, and returns to its caller.
- `player_shot_fire_and_animation_update`: Stores d0 in a5+1057, checks d7 bit 7 and object flags at a0+11, scales sprite offsets from table at 17708 by a5+958, writes position and velocity to a5+950..956, triggers sound and animation, and advances the a6 counter.
- `clamp_and_store_sprite_position_2`: Calls a helper to read coordinates, clamps x to 40-280 and y to 34-552, then saves the old position and writes the new x and y into the object at a0.
- `compute_object_sprite_frame_position`: Reads a sprite index from table at 17696 or a0+8, fetches a frame byte from a0+4 plus a0+10, looks up a scaled offset in table at 17708, adds a0+0/2, and decrements the animation counter at a0+20.
- `toggle_screen_and_check_player_state`: Calls a routine, clears then sets the byte at offset 1045 of a5, calls again, then compares two player state bytes and a direction value to decide whether to call another routine.
- `switch_two_sprite_banks_with_flag`: Sets the byte at offset 1045 of a5 to zero, sets the flag at 1142, calls a routine with a1 pointing at 1160 of a5, then repeats with a1 at 1480 and clears the flag.
- `update_four_entity_animation_frames`: Loops four times over 64-byte entity records starting at a1, and when the direction byte at 19 matches the flag at 1142 of a5, copies two words from a table indexed by byte 14 into offsets 58 and 60.
- `signed_abs_diff_d2_d3`: Pushes d2 and d3, computes d2 minus d0 and d3 minus d1, negates each result if negative, then branches to the shared return path.
- `scaled_ratio_of_d2_d3`: Pushes d2 and d3, orders them, then computes a scaled difference using shifts and additions, returning the result in d0 and restoring d2 and d3.
- `decrement_counter_and_call_update`: If a global counter is nonzero, loops over a table at a5-relative offsets calling a routine per entry, decrements the counter, then calls another routine when a flag is zero.
- `spawn_entity_from_type_table`: If two flags at a5 offsets are clear, calls a library vector, then copies type-table fields into the object at a0 and sets its state byte.
- `send_sound_command_sequence_109`: Checks the same two a5 flags; if clear, saves d0/a0, loads the sound driver pointer from 730(a5), sends command 265 then 781 with bit 9 set through the driver's offset 56 routine, then restores d0/a0 and returns.
- `test_bit4_and_branch_if_set`: Tests bit 4 of the byte at offset 11 from a0 and branches to the code at 1DC2 when that bit is set; otherwise falls through to the following instruction.
- `init_entity_from_type_table`: Stores d0 in byte 19 of a0, indexes a table by d0 times eight to set the pointer at offset 4 and copies type bytes into offsets 17, 18, 20, 21 and 54, then derives offset 14 from the table.
- `reset_entity_if_not_type_two`: Checks byte 19 of a0; if it is not 2, saves d0, loads 2 into d0, calls the reset routine and restores d0 before returning.
- `spawn_entity_from_sprite_table`: Clears flags in a0, stores d0 in byte 8, calls the reset routine, then loads two words from a table indexed by byte 14 and adds a1 offsets to set positions at offsets 0, 2, 48, 50, 58 and 60, zeroing counters and a ten-word block.
- `fill_two_columns_of_five_rows`: Writes byte pairs from a5 offsets into two runs of five 64-byte-spaced records starting at a5+1160, then returns.
- `sort_five_entries_and_assign_ranks`: Calls a setup routine, clears a byte in five 64-byte records, sorts their word fields, and writes rank bytes 0,2,3,1,4 into the matching records.
- `advance_pseudo_random_and_scale_d1`: Updates the word at a5+458 with a linear congruential step, then divides and adds to d1, returning the scaled value in d0.
- `advance_packed_counter_at_a5_offset`: Saves d1, reads the word counter at offset 458 from a5, computes counter times 4 minus counter times 3 plus 1, stores the result back and restores d1.
- `find_min_word_from_a0_offset_30`: Scans four words starting at offset 30 from a0, keeps the smallest in d0 and its index in d1, then returns.
- `find_min_word_skipping_disabled_index`: Scans four words starting at offset 40 from a0, tracks the smallest in d0 and its index in d1 while skipping the index derived from the byte at offset 1037 from a5.
- `compute_heading_for_eight_objects`: Saves a0, loads screen offsets from a5 into d2 and d3 plus eight, then for eight objects calls the angle routine on each pair of words and stores the result at offset twelve, advancing a0 by 64 bytes.
- `find_nearest_visible_object`: Scans four objects starting at a0 plus 64 bytes, skipping those with bit two set at offset eleven, and tracks the smallest word at offset twelve in d6 with its index in d5 and pointer in a1.
- `classify_relative_object_direction`: Compares the x and y words of objects at a0 and a1 and returns in d0 a direction code from zero to seven based on the sign and magnitude of the differences.
- `copy_two_word_pairs_from_a1_a0`: Loads the word at offset 2 of a1 into d0, the word at offset 0 of a1 into d2, the word at offset 2 of a0 into d1 and the word at offset 0 of a0 into d3, then falls through to the next routine.
- `classify_delta_quadrant_into_d0`: Subtracts d1 from d0 and d3 from d2, compares the absolute deltas and their halves, and returns in d0 a direction code from 0 to 7.
- `classify_relative_position_direction`: Pushes d0 and d1, subtracts the words at offsets 58 and 60 of a0 plus d2 and d3, compares the results against small thresholds and returns a direction code in d7 while restoring d0 and d1.
- `find_player_by_team_byte`: Scans up to four 64-byte records at offset 1480 from a5 comparing byte 22 with d0, then returns the word at the matched record start in d0 and the next word in d1.
- `match_opponent_slot_by_threshold`: Walks five 64-byte records at offset 1160 from a5, skipping entries whose word is below d5, calls a comparison routine, and returns d7 as 1 on a byte match at offset 8 of a0 or 0 otherwise.
- `scan_opponent_list_for_byte_match`: Reads a count from the pointer at 554(a5), iterates entries at 2184(a5) skipping words below d5, calls a comparison routine, and returns d6 as 1 when the result byte matches offset 8 of a0, else 0.
- `empty_stub_3`: Empty routine: returns to the caller immediately without doing anything.
- `toggle_player_control_flags`: Reads signed byte at offset 1054 from a5; if zero toggles flags at 1078 and 1079 calling two routines, if negative cycles counter at 1077 to seven toggling flag 1079, else cycles counter toggling flag 1078.
- `draw_two_digit_counters`: Clears d0-d2, loads x 60 and y 193 with byte at 1033 from a5, calls digit drawer, then repeats with x 246 and byte at 1034.
- `draw_two_digit_number`: Saves d0-d2, divides d2 by ten if at least ten, draws tens digit via helper, restores registers, computes remainder and draws ones digit at d0 plus eight.
- `setup_sprite_params_and_call`: Saves d0-d2, copies d1 to d2, sets d1 to 6, d3 to 7, d4 to 0, restores d0-d2, computes a0 as base plus d2 times 56, sets d7 to 7 and calls a routine.
- `loop_call_with_register_adjust`: Saves d1-d4, swaps d3 and d4, decrements d4, adds d0 to d1 and decrements d1, calls a routine, increments d2, loops on d4, then restores d1-d4 and returns.
- `call_same_routine_twice_preserving`: Saves d0, d1 and a0, calls a routine, restores them, calls the same routine again and returns.
- `format_word_as_decimal_string`: Converts the signed word in d0 to a decimal ASCII string at (a0), using d7 as a leading-zero count and d6 as a divisor, then writes a null terminator and restores d2/d7/a0.
- `poll_input_until_button_or_timeout`: Calls a delay routine repeatedly while checking two a5 input bytes masked with 143, storing the result at a5 offset 1122 and calling the follow-up routine.
- `wait_for_fire_button_release`: Loops reading the joystick and fire button bytes at offsets in a5, returning only once the fire bits are clear and the secondary flag is set.
- `check_player_input_and_call_246e`: Saves d0, tests controller and flag bytes at offsets in a5, and if the conditions hold calls the routine at 246E with d1 set to minus one or zero, then restores d0 and returns.
- `load_sprite_frame_words_to_object`: Reads two bytes from the a5 context, combines them into an offset, adds it to a sprite table base, and copies two words into offsets 24 and 26 of the object pointed to by a0.
- `call_four_audio_subroutines`: Calls four external subroutines in sequence, two at one address and two at another, then returns with d0 and d1 unchanged.
- `decrement_timer_and_reload_from_period`: If the byte flag at offset 1091 of the a5-relative structure is set, calls a routine, then decrements the countdown byte at 1119 and, when it reaches zero, reloads it from 1118 and decrements the byte at 1120 unless that byte equals -112.
- `increment_counter_and_start_copper_list`: Increments the byte at offset 1028 of the a1 structure, and if the word at 746 is non-negative, writes the pointer at 424 to the copper list registers and the word to the copper jump register, then calls through the pointer at 730.
- `per_frame_input_and_copper_update`: Adds 257 to a global counter, optionally reloads the copper list, reads fire buttons and joystick ports into the bytes at 933 and 941, decrements the frame counter at 1028, and calls the routine pointed to by 730 when its word is non-negative.
- `build_parity_flags_from_d0`: Tests bits 1, 9, 0 and 8 of d0 and sets bits 3, 2, 1 and 0 of d1 accordingly, using d2 and d3 to compute a shifted xor parity bit, then returns.
- `empty_stub_4`: Empty routine: returns to the caller immediately without doing anything.
- `dispatch_joystick_or_keyboard_input`: Saves registers, selects a buffer at offset 96 or 72 of a5 based on bit 0 of a5+420, pushes arguments and calls the routine at 367c, then restores registers and returns.
- `wait_for_byte_change_at_a5_404`: Saves d0, reads the byte at a5+404 into d0 and compares it with itself, branching back while equal, then restores d0 and returns.
- `repeat_call_d0_times`: Saves d0-d2/d7/a0-a2, copies d0 into d7, calls the routine at 14336 once per count in d7 using dbra, then restores the registers and returns.
- `read_cia_fire_buttons_to_a5`: Saves d0 and a3, reads CIAA port A at a fixed address offset 2049, shifts it left by 8, reads CIAB port A at a fixed address offset 2048 into d0, stores the word at offset 458 of a5, restores registers and returns.
- `init_sound_state_and_jump`: Writes 5 to a5 offsets 1000 and 998, loads the pointer at a5 offset 678, adds 6608 to it, stores it at a5 offset 686, writes 2 to a5 offset 1006, then branches to 14506.
- `init_text_render_state_and_jump`: Writes 7, 6 and 2 to three words in the a5 structure, loads the pointer at offset 678 into a1, adds 3304 to it, stores it at offset 686, then branches to another routine.
- `set_render_mode_five_and_branch`: Stores the value 5 into the word at offset 1006 of the a5 structure and then branches to a following routine.
- `render_text_stream_with_control_codes`: Stores 2, 7 and 6 into a5 structure words, copies the pointer at offset 678 to offset 686, then walks the a0 byte stream handling control bytes 1 and 13, calling a drawing routine per character until a zero byte ends it.
- `push_regs_set_a6_3892_and_branch`: Saves d2, d7, a0 and a6 on the stack, loads the address of the code at 3892 relative to pc into a6, then branches forward to the shared continuation.
- `push_regs_set_a6_3868_and_branch`: Saves d2, d7, a0 and a6 on the stack, loads the address of the code at 3868 relative to pc into a6, then branches forward to the shared continuation.
- `mask_args_call_23e0_then_a6`: Saves d2, d7, a0 and a6, loads a6 with the routine at 3846, masks d0, d1 and d2 to 16 bits, pushes d0, calls the routine at 23E0 with a0 pointing at 4250(a5), restores d0, jumps through a6 and restores registers.
- `animate_six_rounds_then_fade_palette`: Runs six timed animation rounds calling several subroutines, then loops a counter down from forty-eight while adjusting colour values and reading palette words near a5 offsets.
- `repeat_fifteen_times_call_sub`: Loads fifteen into d5, calls the shared subroutine once, then decrements d5 and repeats until it wraps, returning to the caller.
- `repeat_sixteen_times_call_sub`: Starts d5 at zero, calls the shared subroutine, increments d5, compares it with sixteen and loops until equal, then returns.
- `toggle_byte_and_call_sound`: Calls several subroutines, then toggles the byte at offset 749 from a5 between zero and minus one, calling a sound routine when it becomes minus one.
- `init_two_sprite_blocks`: Loads d4 with zero and four, then calls the same setup routine twice with a0 pointing at offsets 1160 and 1480 from a5 and a2 at two fixed data blocks.
- `update_five_entity_blocks`: Loops five times over 64-byte blocks at a0, reading positions and deltas from a2, calling a collision check, and updating position and flags when d7 is zero.
- `load_two_sprite_blocks_via_bsr`: Sets d4 to zero, loads a0 from offset 1160 of a5 and a2 from a fixed long address, calls a routine, then repeats with d4 set to four, a0 from offset 1480 of a5 and a2 from another fixed address, and returns.
- `misaligned_data_bytes_not_code`: The four words are data bytes decoded as instructions: two ori.b immediates, a conditional branch and an unconditional branch, not executable code.
- `continue_game_prompt_input_loop`: Clears state bytes in the a5 frame, calls sixteen routines, loops reading a key code in d5 and branching to set flags at offsets 1060, 1133, 1140 and 1116 of a5, showing the cannot-continue message path.
- `toggle_global_flag_and_call_draw`: If the byte at offset 1103 of a5 is zero, calls a function through a5 offset 730, saves and sets a global byte flag, calls another routine with a0 pointing to a table, increments d5, stores it at a5 offset 1117, then restores the flag.
- `run_menu_loop_with_input_polling`: Sets up menu state at offsets in a5, calls several subroutines, then loops polling byte at 941(a5) for joystick input, updating selection index d5 and calling subroutines until the fire button is pressed.
- `select_and_draw_menu_cursor_rows`: Computes two screen coordinates from a5 offsets, polls joystick bits at a5 offset 941 to move a selection index d5 within a limit, draws two rows via subroutines using the coordinates and index, and updates d7.
- `set_d0_11_d1_37_branch_403c`: Loads the immediate value 11 into d0 and 37 into d1, then branches unconditionally to the shared code at 403c.
- `set_d0_112_d1_37_branch_403c`: Loads the immediate value 112 into d0 and 37 into d1, then branches unconditionally to the shared code at 403c.
- `read_input_stream_and_dispatch_commands`: Reads bytes from the input pointer at offset 718 of a5, handles the 127 command by toggling a flag and swapping a pointer, and calls the sound driver through offset 56 of the structure at 730 of a5.
- `init_sound_and_playfield_then_branch`: Calls two setup routines, stores 255 at offset 1116 of a5, and if byte 1060 of a5 is zero sets pointers at 706 and 702 of a5, loads a sample pointer, calls the sound driver, writes hardware registers at 36 of a5 and a global word, then branches to the shared exit.
- `set_sample_pointers_and_trigger_sound`: Stores pointers to a5 offsets 941 and 1106 into a5 offsets 706 and 702, sets a sample pointer at 710 of a5, saves d0 and a0, calls the sound driver with command 4, restores them and calls the sample setup routine.
- `team_select_menu_loop`: Reads team data via a6, draws three team entries with text, polls input with a delay loop, moves the selection cursor up or down skipping the excluded team, updates the chosen team byte and redraws the highlighted entry until fire is pressed.
- `copy_carriage_return_terminated_string`: Uses d0 as an index into a table of string pointers at offset 738 of a5, then copies bytes from that string into the buffer at a1 until a carriage return byte, writing a null terminator.
- `draw_sprite_sequence_from_table`: Reads a sprite entry indexed by byte at offset 45a of a5, computes position from table offsets, then loops over bytes drawing each sprite with delays until a negative byte ends the sequence.
- `compare_byte_and_call_if_less`: Loads byte at offset 40a of a5 into d0, compares it with byte at offset 409, and if d0 is less, saves registers, calls a routine with d0 set to 2, restores registers and branches on.
- `save_d0_a0_call_6f2_then_4c4a`: Pushes d0 and a0 onto the stack, sets d0 to 3, calls the routine at 6f2, restores d0 and a0, then branches to the routine at 4c4a.
- `draw_eleven_rows_then_poll_input`: Copies two tables via a0, loops eleven times over 32-byte rows at a6 calling 4444, calls 5032 and the 7800 routine, increments the byte at 1050(a5), then tests input bytes and draws sprite entries from 1600.
- `call_4444_5032_then_7800_and_loop`: Calls the routines at 4444 and 5032, saves d0-d7/a0-a4/a6, calls the routine at 7800, restores those registers and branches back to the code at 43ca.
- `call_stub`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `draw_player_status_panel_2`: Reads player fields from a6, computes a score offset, draws several text and number fields via subroutines, then sets a flag byte at a5+4250 based on comparing byte 23 with byte 13.
- `write_blank_status_flag`: Stores byte 62 into the buffer at a0, clears the following byte, sets d0 to 242 and calls the draw routine, then returns.
- `toggle_flag_and_wait_for_zero`: Calls the routine at 6f2 with d0 set to 1, sets byte 46c(a5) to 1, then toggles it and calls 458c until the result is zero.
- `toggle_status_byte`: Calls a routine with argument 1, then checks a byte at offset 1141 of the structure in a5.
- `draw_player_status_panel`: Loads byte 44d(a5) into d0, sets d7 to 3 and a0 to the table at 6edf, calls the draw routine, then sets a0 to 6eca and calls three more routines before jumping to 4620.
- `draw_score_and_store_next_value`: Reads byte 438(a5) into d0, draws with d7=3 and a0=6ec7, then increments that byte into 455(a5), sets a0 to 6eb3, calls the draw routine and branches to 45e6.
- `draw_win_or_draw_result_screen`: Copies a status byte, branches on match result flags, then repeatedly calls drawing routines to render the win-or-draw message and team text before returning a result code.
- `draw_team_names_and_scores`: Draws the PERSEUS and QUASTAR team name strings and the score digits at fixed screen coordinates, selecting text variants from the match state bytes in the a5 structure.
- `draw_team_label_and_score_panel`: Draws the TEAM label and the current score value at fixed screen positions, then either draws a status string or the team name pointer from the a5 table.
- `draw_team_stats_panel`: Saves a0, draws a TEAM label via the text routine, then draws six fields from offsets 24, 25, 26, 8, 10 and 20 of the team structure at seven-pixel line spacing.
- `draw_team_name_if_present`: Draws a fixed label at position d0/d1, then if the byte at offset 1132 of a5 is nonzero, loads the team name pointer and calls the team stats drawing routine.
- `draw_selected_team_name`: Draws a TEAM label, then indexes the team pointer table at offset 738 of a5 by d2 times twelve, dereferences it and draws that team's name.
- `call_twice_and_return_d0_d1`: Calls one unnamed routine, then calls another unnamed routine, and returns with d0 and d1 unchanged as the result pair.
- `save_registers_and_jump_via_a5_offset`: Pushes d0 through d7 and a0 through a4 plus a6 onto the stack, loads a0 from the longword at offset 726 from a5, then branches onward.
- `call_two_routines_and_return_d0_d1`: Calls one unnamed routine, then calls a second unnamed routine, and returns with d0 and d1 unchanged as the result pair.
- `draw_sprite_at_slot_offset`: Multiplies the low word of d2 by 4608, adds the base pointer at offset 698 from a5, then calls a drawing routine with that computed address and the incoming d0/d1 values.
- `save_regs_set_96_and_call`: Pushes d0-d7, a0-a4 and a6, saves d0-d1, copies d1 to d2, sets d1 and d3 to 96, restores d0-d1, calls the routine at 4a26, then restores all registers and returns.
- `push_regs_load_table_and_branch`: Saves d0-d5 and a0 on the stack, loads the address 21788 into a0, then branches forward to the shared routine at 4a30.
- `blit_96_rows_from_table`: Saves d0-d5/a0, loads a table pointer, sets blit width and height to 96 with step 2, then calls the blit routine 96 times reading bytes from the table.
- `blit_64_rows_from_551a`: Saves d0-d5/a0, loads the table at 551a into a0, sets blit width and height to 64, then branches into the shared blit loop.
- `advance_two_positions_by_velocity`: Saves d0-d1/d4, reads a byte and two words from the a5 structure, calls a helper, then adds two velocity words to the stored positions and writes them back.
- `save_regs_zero_d4_jump`: Pushes d0-d7, a0-a4 and a6 onto the stack, sets d4 to zero, then branches into another routine.
- `save_regs_zero_d4_set_d7_jump`: Pushes d0-d7, a0-a4 and a6 onto the stack, sets d4 to zero and d7 to four, then branches into another routine.
- `measure_text_width_and_draw`: Scans a null-terminated string in a0 counting characters and line breaks, stores the widest line width and line count in a5 offsets, then calls drawing routines with adjusted d0, d1, d2 and d3.
- `save_regs_swap_d1_d2_call_and_jump`: Pushes d0-d5/a0 then d0-d3 onto the stack, exchanges d1 and d2, calls the routine at 23b2, pops d0-d3, loads a0 with the long address 551a and branches to 4a34.
- `advance_shot_phase_counters`: Compares bytes at offsets 1033 and 1034 of a5, increments counters at 1111, 1112 and 1080 of a5, calls two subroutines when thresholds are reached, and returns a word flag on the stack.
- `accumulate_team_stats_from_score_bytes`: Loads a0 with 1800 of a5 and a1 with a0 plus 32, adds the bytes at 1033 and 1034 of a5 into word fields at offsets 8 and 10 of both structures, increments counters and branches to 4C00, 4BEA or 4C0E.
- `advance_two_entity_offsets`: Adds 20 to the word at offset 20 of both a0 and a1, increments the byte at offset 26 of each, then branches to another routine.
- `update_two_entities_and_cycle_counter`: Adds 100 to a1 offset 20, bumps bytes at a0 offset 25 and a1 offset 24, adds d0 and d1 times 5 to the offset-20 words, advances a5 counter at 1101 wrapping at 11, and returns 1 if the cycle byte matches 1117 else 0.
- `call_sub_and_return_one`: Calls the routine at the given target, then sets d0 to 1 and returns to the caller.
- `assign_team_slots_by_score`: Copies each of eleven 32-byte player records' byte 13 into byte 23, clears byte 13, then repeatedly picks the highest word at offset 20 among unassigned records and writes an increasing slot number into its byte 13.
- `draw_team_names_and_advance_round`: Draws the team name table at a5+1800 using the DRACO or MACHOV font, draws a two-digit round counter, and sets the a5+1101 and a5+1102 flags when the counter reaches eleven.
- `draw_all_team_names_set_round_two`: Draws the eleven team name entries starting at a5+1800 with the DRACO or MACHOV font, then sets the a5+1101 flag and stores two in a5+1102.
- `copy_entity_template_to_record`: Copies eight bytes from the source pointed to by a3 into the record at a2, then fills fields at offsets 14 through 22 from a3, stores d0 at offset 27 and zeroes words at 8, 10, 20 and bytes at 12, 24, 25, 26.
- `load_player_stats_from_table`: Multiplies d0 by 32, indexes the table at a5 offset 1800, and copies bytes from offsets 14, 15, 16, 17, 18, 19 and 22 into the a5 fields at 1066, 1100, 1067, 1071, 1056, 1068, 1069 and 1086.
- `call_stub_2`: Call stub: calls one other routine as a subroutine and returns to its own caller when that routine returns.
- `call_two_setup_routines`: Calls the routine at the next address, then calls the routine further ahead, and returns to its caller after both have run.
- `load_team_colour_triplet_from_table`: Reads byte at offset 1106 from a5, multiplies it by four, indexes the table at 21967 and copies three bytes into a5 offsets 1099, 1065 and 1097.
- `load_opponent_colour_triplet_from_table`: Reads byte at offset 1110 from a5, multiplies it by four, indexes the table at 21967 and copies three bytes into a5 offsets 1100, 1066 and 1086.
- `refresh_display_and_redraw_playfield`: Saves a0, conditionally calls a setup routine when a5 offset 748 is zero, calls another routine, reloads a0 from a5 offset 682, draws via two calls and restores a0.
- `reset_sound_and_clear_global_flag`: Calls two external routines, sets the byte at offset 748 from a5 to 255 and calls another routine with the address 750 from a5, then clears the global flag after calling a routine.
- `call_two_external_routines`: Calls one routine via bsr and another via jsr to absolute address, then returns to the caller.
- `draw_six_by_six_tile_block`: Saves registers, computes a tile count from a5 offsets, then loops drawing a six by six block of tiles via jsr, advancing coordinates and wrapping the tile index at 52.
- `toggle_player_side_and_tick_timer`: Clears a flag at offset 1145 in a5, calls the side-switch routine, sets the flag and calls it again if the byte at offset 1060 is zero, then decrements the byte at offset 1129 if nonzero.
- `team_select_menu_loop_2`: Clears an eleven-byte selection table, draws the team-select screen, reads joystick bits to move a cursor and fire, updates player colours, and loops until a team is confirmed.
- `draw_cant_select_message`: Saves registers, then draws the text at the given address with colour and position values in d0 and d1 via the text routine, restores registers and returns.
- `draw_player_team_status_rows`: Saves registers, selects a status table based on two flags at offsets 1063 and 1113 of a5, calls a setup routine, then loops eleven times over player slots, invoking per-slot handlers and drawing text via another routine before restoring registers and returning.
- `dispatch_player_one_sprite_update`: Chooses a sprite table based on the flag at offset 1140 of a5, indexes it with the byte at 1106, loads three bytes from offsets 1099, 1065 and 1097 into d0, d1 and d2, calls the shared update routine and returns.
- `dispatch_player_two_sprite_update`: Chooses a sprite table based on the flag at offset 1140 of a5, indexes it with the byte at 1110, loads three bytes from offsets 1100, 1066 and 1086 into d0, d1 and d2, calls the shared update routine and returns.
- `draw_three_team_score_digits`: Loads three score label strings, choosing alternate text when the byte at offset 1140 of a5 is set, reads three bytes from a data pointer into d0, d1 and d2, and calls the digit drawing routine for each.
- `draw_score_pair_with_separator`: Calls the text drawing routine with d7 set to 2, draws d1 times 100, writes a carriage return byte after advancing a0 by four, then draws 256 minus d2 and another carriage return.
- `show_you_won_the_blood_bowl`: Loads the address of the YOU WON THE BLOOD-BOWL text into a0, calls the display routine, then loads another text pointer and branches onward with d0 set to 78.
- `setup_knockout_loss_message`: Loads message id 152 into d1 and calls the text setup, then sets d0 to 112, d1 to 44, reads byte at offset 1106 from a5 into d2 and calls the text, sound and display routines before returning.
- `show_knockout_won_message`: Points a0 at the won-knockout text, calls the text setup, loads message id 143 into d1, sets d0 to 112, reads byte at offset 1106 from a5 into d2 and calls the text, sound and display routines.
- `show_end_of_round_message`: Displays the end-of-round text, sets flag at offset 1132 from a5, calls two setup routines, then compares word at offset 20 with offset 52 of the structure at 1800(a5) to pick a byte before calling text, sound and display routines.
- `save_game_disk_prompt_flow`: Checks the byte at offset 1060 in a5; if not minus 128, it calls the disk routine, prints save prompts, picks a message based on that byte, draws text, and calls the save routine.
- `load_knockout_save_prompt`: Loads the address of a text string into a0 and the value 151 into a1, then branches to shared prompt code.
- `load_league_save_prompt`: Loads the address of a text string into a0 and the value 150 into a1, then branches to shared prompt code.
- `load_game_with_disk_prompt`: Calls the system routine with the disk prompt text, checks the loaded game, displays the remove-disk and press-return messages, and clears the load flag at the global byte.
- `check_and_clear_key_d_or_e`: Reads the byte at offset 948 of a5, clears it, and returns 1 if it was 'E' or 0 if it was 'D'.
- `copy_save_data_with_header`: Writes the header words 'save' and 'data' to a0, then copies 327 dwords from offset 880 of a5 into a0, preserving registers.
- `copy_save_data_after_magic_check`: Saves registers, checks for the magic words save and data at a0, then copies 327 dwords from a0 into the buffer at offset 880 from a5 and returns zero in d0.
- `show_failed_to_qualify_message`: Displays the text YOU FAILED TO QUALIFY using the text and screen routines, passing a byte from offset 1106 of a5, then calls further display and update routines before returning.
- `run_game_setup_sequence`: Saves registers and stack pointer, stores a buffer pointer, initializes display and sound hardware, patches a word in memory, then calls setup, render and update routines before restoring registers and returning.
- `save_custom_chip_state_and_enable_dma`: Stores a flag at address zero, then copies ADKCONR, DMACONR, INTENAR and INTREQR into low memory and writes new ADKCON, INTENA, INTREQ and DMACON values to the custom chip registers before setting CIAB port A bits.
- `restore_stack_pointer_and_set_d0_ff`: Loads the stack pointer from the longword at address 28 and sets d0 to 255, then falls through into the following code.
- `enable_dma_interrupts_and_clear_buffer`: Enables interrupt and DMA bits in the custom chip registers, calls a subroutine, then fills a large buffer at offset 4270 of a5 with zeros using a counted loop.
- `empty_stub_5`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_6`: Empty routine: returns to the caller immediately without doing anything.
- `pulse_ciab_disk_select_and_return`: Loads d0 with a word immediate, calls the system vector at address 128, sets then clears bit 7 and bit 3 of the CIAB port B disk control register, restores d0 from the stack and returns.
- `empty_stub_7`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_8`: Empty routine: returns to the caller immediately without doing anything.
- `toggle_ciab_disk_bit_with_delay_loop`: Clears bit 3 of the CIAB port B disk control register, loops 100 times using dbra, then calls a nearby routine and returns; the leading ori and addx bytes are stray data.
- `wait_for_fire_button_release_polling_ciaa`: Calls the system vector at address 128 with a count, then repeatedly tests bit 5 of the CIAA port A fire button line with dbeq timeouts, branching away if still pressed, otherwise returning.
- `jump_vector_entry`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.
- `execute_indirect_jump`: Calls a routine at a long address, sets d0 to zero, and performs an indirect jump to a long address.
- `write_4e75_to_a3_and_clear_absolute_zero`: Loads a3 with 24, computes 4454 plus 2585 into d1, stores d1 at (a3), sets d0 to zero and writes it to absolute address zero, then returns.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `jump_vector_entry_2`: Jump-table entry: a single absolute jump that passes control straight on to a fixed address elsewhere in memory.

### run-time segment (code the game unpacked, copied or loaded; carved from reference-run RAM)

- `empty_stub_9`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_10`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_11`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_12`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_13`: Empty routine: returns to the caller immediately without doing anything.
- `empty_stub_14`: Empty routine: returns to the caller immediately without doing anything.
- `empty_rts_with_unreachable_branch`: Returns immediately to the caller; the following branch instruction is unreachable and never executes.
