# Game 2990 (BubbleGun v1.0): code recovery summary

Step 1 of the native-port process: every byte of the game recovered into named, explained, reassemblable source on Ryzen (/srv/fast/amigo/recovery/02990). Source, disassembly, data and assets stay on Ryzen; this page has numbers and names only.

## Proofs

| proof | result |
|---|---|
| Bytes classified (3244346 top-level bytes in 8 containers) | pass, 100.000%, 0 unknown |
| Byte-identical reassembly with vasm | pass, 1 of 1 units identical (bytes and relocations), 0 raw-word lines |
| Functions named and described | pass, 3 of 3 named (0 author symbols), 3 of 3 described (3 by rule, 0 by the local model) |
| Executed addresses (reference run) in recovered, described code | pass, 0 of 0 (0.00%) |
| Recovered without load address (counted in proof 1, flagged) | 0 bytes, 0 functions |
| Independent decoder cross-check (Ghidra against capstone) | 7 of 7 instructions identical in start and length, 0 mismatches |

## Bytes by class (top level)

| class | bytes |
|---|---|
| asset | 18005 |
| code | 18 |
| data | 3094227 |
| unused | 132096 |

## Code units

| file | hunks | instructions | functions |
|---|---|---|---|
| game.slave | 1 | 7 | 3 |

## Cases handled

- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: raw disk image (828416 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: raw disk image (609280 bytes, no filesystem): matched against track-loaded segments in 256-byte chunks
- BubbleGun_v1.0/BubbleGun_v1.0.hdf: hard-disk image (AmigaDOS OFS, 3488 blocks): its filesystem parses and all 6 files in it are byte-identical to the extracted tree BubbleGun_v1.0_hdf, which is recovered file by file; no block outside the filesystem holds data
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 0-14 (bytes 0-3840) outside the filesystem hold data, not code (opcode density 0.051, entropy 4.07)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 16-30 (bytes 4096-7936) outside the filesystem hold data, not code (opcode density 0.038, entropy 5.12)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 41-218 (bytes 10496-56064) outside the filesystem hold data, not code (opcode density 0.058, entropy 5.79)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 221-221 (bytes 56576-56832) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.64)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 227-227 (bytes 58112-58368) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.55)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 259-389 (bytes 66304-99840) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.31)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 391-394 (bytes 100096-101120) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.89)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 396-399 (bytes 101376-102400) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.12)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 401-434 (bytes 102656-111360) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.05)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 443-462 (bytes 113408-118528) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.77)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 464-469 (bytes 118784-120320) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.37)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 472-482 (bytes 120832-123648) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.44)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 484-486 (bytes 123904-124672) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.65)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 618-824 (bytes 158208-211200) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.02)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 826-872 (bytes 211456-223488) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.52)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 874-1336 (bytes 223744-342272) outside the filesystem hold data, not code (opcode density 0.004, entropy 6.49)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 1347-1364 (bytes 344832-349440) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.60)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 1366-1366 (bytes 349696-349952) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.85)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 1420-1420 (bytes 363520-363776) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.07)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 1422-1644 (bytes 364032-421120) outside the filesystem hold data, not code (opcode density 0.006, entropy 3.23)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 1647-1843 (bytes 421632-472064) outside the filesystem hold data, not code (opcode density 0.002, entropy 6.60)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 1845-1980 (bytes 472320-507136) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.64)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2002-2043 (bytes 512512-523264) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.30)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2062-2073 (bytes 527872-530944) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.18)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2092-2102 (bytes 535552-538368) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.38)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2122-2178 (bytes 543232-557824) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.33)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2180-2196 (bytes 558080-562432) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.72)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2198-2198 (bytes 562688-562944) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.85)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2200-2216 (bytes 563200-567552) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.10)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2218-2218 (bytes 567808-568064) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.85)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2220-2236 (bytes 568320-572672) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.04)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2238-2238 (bytes 572928-573184) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.85)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2240-2256 (bytes 573440-577792) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.87)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2258-2258 (bytes 578048-578304) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.85)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2260-2276 (bytes 578560-582912) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.41)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2278-2278 (bytes 583168-583424) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.85)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2281-2380 (bytes 583936-609536) outside the filesystem hold data, not code (opcode density 0.002, entropy 4.44)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2406-2592 (bytes 615936-663808) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.97)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2594-2599 (bytes 664064-665600) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.37)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2602-2612 (bytes 666112-668928) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.92)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2614-2630 (bytes 669184-673536) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.65)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2632-2632 (bytes 673792-674048) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.85)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2634-2652 (bytes 674304-679168) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.82)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2654-2672 (bytes 679424-684288) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.73)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2674-2690 (bytes 684544-688896) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.71)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2692-2709 (bytes 689152-693760) outside the filesystem hold data, not code (opcode density 0.000, entropy 1.94)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2711-2712 (bytes 694016-694528) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2714-2730 (bytes 694784-699136) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.59)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2732-2749 (bytes 699392-704000) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.65)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2751-2752 (bytes 704256-704768) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.49)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2754-2782 (bytes 705024-712448) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.66)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2784-2852 (bytes 712704-730368) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.98)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 2854-3086 (bytes 730624-790272) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.12)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 3088-3104 (bytes 790528-794880) outside the filesystem hold data, not code (opcode density 0.004, entropy 3.12)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 3106-3106 (bytes 795136-795392) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.87)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 3108-3146 (bytes 795648-805632) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.94)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 3148-3164 (bytes 805888-810240) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.11)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 3166-3184 (bytes 810496-815360) outside the filesystem hold data, not code (opcode density 0.003, entropy 2.78)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 3186-3204 (bytes 815616-820480) outside the filesystem hold data, not code (opcode density 0.004, entropy 2.99)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 3206-3223 (bytes 820736-825344) outside the filesystem hold data, not code (opcode density 0.003, entropy 2.47)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 3225-3226 (bytes 825600-826112) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.81)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.1: chunks 3228-3235 (bytes 826368-828416) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.86)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 0-0 (bytes 0-256) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.27)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 3-100 (bytes 768-25856) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.65)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 102-228 (bytes 26112-58624) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.65)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 233-289 (bytes 59648-74240) outside the filesystem hold data, not code (opcode density 0.000, entropy 3.17)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 314-334 (bytes 80384-85760) outside the filesystem hold data, not code (opcode density 0.000, entropy 4.56)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 336-352 (bytes 86016-90368) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.74)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 354-371 (bytes 90624-95232) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.31)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 373-374 (bytes 95488-96000) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.36)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 376-392 (bytes 96256-100608) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.87)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 394-412 (bytes 100864-105728) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.76)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 414-432 (bytes 105984-110848) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.64)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 434-452 (bytes 111104-115968) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.57)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 454-562 (bytes 116224-144128) outside the filesystem hold data, not code (opcode density 0.004, entropy 4.06)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 564-788 (bytes 144384-201984) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.74)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 790-806 (bytes 202240-206592) outside the filesystem hold data, not code (opcode density 0.007, entropy 3.83)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 808-828 (bytes 206848-212224) outside the filesystem hold data, not code (opcode density 0.005, entropy 3.10)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 830-846 (bytes 212480-216832) outside the filesystem hold data, not code (opcode density 0.018, entropy 3.56)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 848-866 (bytes 217088-221952) outside the filesystem hold data, not code (opcode density 0.009, entropy 3.43)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 868-885 (bytes 222208-226816) outside the filesystem hold data, not code (opcode density 0.019, entropy 3.27)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 887-888 (bytes 227072-227584) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.71)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 890-906 (bytes 227840-232192) outside the filesystem hold data, not code (opcode density 0.006, entropy 3.81)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 908-928 (bytes 232448-237824) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.89)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 930-946 (bytes 238080-242432) outside the filesystem hold data, not code (opcode density 0.018, entropy 3.63)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 948-949 (bytes 242688-243200) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.82)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 951-1048 (bytes 243456-268544) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.82)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1050-1282 (bytes 268800-328448) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.71)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1284-1300 (bytes 328704-333056) outside the filesystem hold data, not code (opcode density 0.003, entropy 2.23)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1302-1302 (bytes 333312-333568) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.40)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1304-1320 (bytes 333824-338176) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.44)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1322-1340 (bytes 338432-343296) outside the filesystem hold data, not code (opcode density 0.005, entropy 2.41)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1342-1360 (bytes 343552-348416) outside the filesystem hold data, not code (opcode density 0.006, entropy 2.35)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1362-1379 (bytes 348672-353280) outside the filesystem hold data, not code (opcode density 0.006, entropy 2.32)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1381-1382 (bytes 353536-354048) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.34)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1384-1400 (bytes 354304-358656) outside the filesystem hold data, not code (opcode density 0.006, entropy 2.56)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1402-1419 (bytes 358912-363520) outside the filesystem hold data, not code (opcode density 0.000, entropy 2.16)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1421-1422 (bytes 363776-364288) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.33)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1425-1522 (bytes 364800-389888) outside the filesystem hold data, not code (opcode density 0.003, entropy 4.61)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1524-1756 (bytes 390144-449792) outside the filesystem hold data, not code (opcode density 0.003, entropy 3.83)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1758-1774 (bytes 450048-454400) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.51)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1776-1794 (bytes 454656-459520) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.33)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1796-1814 (bytes 459776-464640) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.67)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1816-1833 (bytes 464896-469504) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.32)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1835-1836 (bytes 469760-470272) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.23)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1838-1854 (bytes 470528-474880) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.60)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1856-1874 (bytes 475136-480000) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.34)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1876-1893 (bytes 480256-484864) outside the filesystem hold data, not code (opcode density 0.003, entropy 1.87)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1895-1896 (bytes 485120-485632) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.71)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1899-1996 (bytes 486144-511232) outside the filesystem hold data, not code (opcode density 0.001, entropy 4.47)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 1998-2147 (bytes 511488-549888) outside the filesystem hold data, not code (opcode density 0.002, entropy 3.52)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 2174-2230 (bytes 556544-571136) outside the filesystem hold data, not code (opcode density 0.001, entropy 3.70)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 2232-2248 (bytes 571392-575744) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.24)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 2250-2267 (bytes 576000-580608) outside the filesystem hold data, not code (opcode density 0.002, entropy 2.32)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 2269-2270 (bytes 580864-581376) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.81)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 2272-2288 (bytes 581632-585984) outside the filesystem hold data, not code (opcode density 0.007, entropy 2.65)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 2290-2308 (bytes 586240-591104) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.59)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 2310-2328 (bytes 591360-596224) outside the filesystem hold data, not code (opcode density 0.001, entropy 2.34)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 2330-2348 (bytes 596480-601344) outside the filesystem hold data, not code (opcode density 0.007, entropy 2.39)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 2350-2367 (bytes 601600-606208) outside the filesystem hold data, not code (opcode density 0.007, entropy 2.31)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 2369-2370 (bytes 606464-606976) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.60)
- BubbleGun_v1.0/BubbleGun_v1.0_hdf/BubbleGun/Disk.2: chunks 2372-2379 (bytes 607232-609280) outside the filesystem hold data, not code (opcode density 0.000, entropy 0.86)

## Cracktros (kept, not protection)

- none

## Copy protection found

- none found

## Functions


### game.slave

- `tiny_moveq_rts`: Small routine of 2 instructions that loads 255 into d0, then returns.
- `frag_pea_bra`: Short code fragment that pushes the address of a fixed memory address onto the stack, then jumps to another point in the code.
- `frag_move_addq_rts`: Short code fragment that copies a location in the code to the stack, then adds 4 to the top of the stack, then returns.
